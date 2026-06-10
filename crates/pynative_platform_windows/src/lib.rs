use serde::Serialize;

pub use pynative_core::WidgetNode;

#[derive(Debug, Clone)]
pub struct HelloWindowConfig {
    pub title: String,
    pub message: String,
    pub button_label: String,
}

impl Default for HelloWindowConfig {
    fn default() -> Self {
        Self {
            title: "PyNative UI Phase 0.1".to_string(),
            message: "Rust opened a native Windows window.".to_string(),
            button_label: "Click native button".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct HelloWindowResult {
    pub platform: &'static str,
    pub clicked: usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct WidgetWindowResult {
    pub platform: &'static str,
    pub events: usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct WidgetEvent {
    pub kind: &'static str,
    pub callback_id: u64,
    pub value: Option<String>,
}

#[cfg(windows)]
pub fn run_hello_window(config: HelloWindowConfig) -> Result<HelloWindowResult, String> {
    windows_impl::run_hello_window(config, None)
}

#[cfg(windows)]
pub fn run_hello_window_with_callback<F>(
    config: HelloWindowConfig,
    on_click: F,
) -> Result<HelloWindowResult, String>
where
    F: FnMut(usize) -> Result<(), String> + 'static,
{
    windows_impl::run_hello_window(config, Some(Box::new(on_click)))
}

#[cfg(not(windows))]
pub fn run_hello_window(_config: HelloWindowConfig) -> Result<HelloWindowResult, String> {
    Err("The hello window prototype is only available on Windows.".to_string())
}

#[cfg(not(windows))]
pub fn run_hello_window_with_callback<F>(
    _config: HelloWindowConfig,
    _on_click: F,
) -> Result<HelloWindowResult, String>
where
    F: FnMut(usize) -> Result<(), String> + 'static,
{
    Err("The hello window prototype is only available on Windows.".to_string())
}

#[cfg(windows)]
pub fn run_widget_tree_window(root: WidgetNode) -> Result<WidgetWindowResult, String> {
    windows_impl::run_widget_tree_window(root, None)
}

#[cfg(windows)]
pub fn run_widget_tree_window_with_events<F>(
    root: WidgetNode,
    on_event: F,
) -> Result<WidgetWindowResult, String>
where
    F: FnMut(WidgetEvent) -> Result<Option<WidgetNode>, String> + 'static,
{
    windows_impl::run_widget_tree_window(root, Some(Box::new(on_event)))
}

#[cfg(not(windows))]
pub fn run_widget_tree_window(_root: WidgetNode) -> Result<WidgetWindowResult, String> {
    Err("The widget tree window renderer is only available on Windows.".to_string())
}

#[cfg(not(windows))]
pub fn run_widget_tree_window_with_events<F>(
    _root: WidgetNode,
    _on_event: F,
) -> Result<WidgetWindowResult, String>
where
    F: FnMut(WidgetEvent) -> Result<Option<WidgetNode>, String> + 'static,
{
    Err("The widget tree window renderer is only available on Windows.".to_string())
}

#[cfg(windows)]
mod windows_impl {
    use std::cell::RefCell;
    use std::collections::HashMap;
    use std::ptr::null_mut;
    use std::sync::atomic::{AtomicIsize, AtomicUsize, Ordering};

    use super::{
        HelloWindowConfig, HelloWindowResult, WidgetEvent, WidgetNode, WidgetWindowResult,
    };
    use windows_sys::Win32::Foundation::{
        ERROR_CLASS_ALREADY_EXISTS, GetLastError, HWND, LPARAM, LRESULT, WPARAM,
    };
    use windows_sys::Win32::Graphics::Gdi::{DEFAULT_GUI_FONT, GetStockObject};
    use windows_sys::Win32::System::LibraryLoader::GetModuleHandleW;
    use windows_sys::Win32::UI::WindowsAndMessaging::{
        CS_HREDRAW, CS_VREDRAW, CW_USEDEFAULT, CreateWindowExW, DefWindowProcW, DestroyWindow,
        DispatchMessageW, EN_CHANGE, ES_AUTOHSCROLL, GetMessageW, GetWindowTextLengthW,
        GetWindowTextW, HMENU, IDC_ARROW, LoadCursorW, MSG, PostQuitMessage, RegisterClassW,
        SendMessageW, SetWindowTextW, TranslateMessage, WINDOW_EX_STYLE, WM_COMMAND, WM_CREATE,
        WM_DESTROY, WM_SETFONT, WNDCLASSW, WS_BORDER, WS_CHILD, WS_OVERLAPPEDWINDOW, WS_VISIBLE,
    };

    const BUTTON_ID: usize = 1001;
    const LABEL_ID: usize = 1002;
    const FIRST_WIDGET_CONTROL_ID: usize = 2000;
    const WINDOW_WIDTH: i32 = 680;
    const WINDOW_HEIGHT: i32 = 480;
    const CONTENT_X: i32 = 24;
    const CONTENT_Y: i32 = 24;
    const CONTENT_WIDTH: i32 = 600;
    const ROW_GAP: i32 = 12;

    static CLICK_COUNT: AtomicUsize = AtomicUsize::new(0);
    static WIDGET_EVENT_COUNT: AtomicUsize = AtomicUsize::new(0);
    static LABEL_HWND: AtomicIsize = AtomicIsize::new(0);

    type ClickCallback = Box<dyn FnMut(usize) -> Result<(), String>>;
    type WidgetEventCallback = Box<dyn FnMut(WidgetEvent) -> Result<Option<WidgetNode>, String>>;

    #[derive(Default)]
    struct WidgetRenderState {
        root: Option<WidgetNode>,
        controls: Vec<HWND>,
        button_callbacks: HashMap<usize, u64>,
        input_callbacks: HashMap<usize, u64>,
        next_control_id: usize,
        root_hwnd: HWND,
        hinstance: *mut core::ffi::c_void,
    }

    thread_local! {
        static CLICK_CALLBACK: RefCell<Option<ClickCallback>> = RefCell::new(None);
        static WIDGET_EVENT_CALLBACK: RefCell<Option<WidgetEventCallback>> = RefCell::new(None);
        static WIDGET_RENDER_STATE: RefCell<WidgetRenderState> = RefCell::new(WidgetRenderState {
            root: None,
            controls: Vec::new(),
            button_callbacks: HashMap::new(),
            input_callbacks: HashMap::new(),
            next_control_id: FIRST_WIDGET_CONTROL_ID,
            root_hwnd: null_mut(),
            hinstance: null_mut(),
        });
        static CALLBACK_ERROR: RefCell<Option<String>> = const { RefCell::new(None) };
    }

    struct CallbackGuard;

    impl Drop for CallbackGuard {
        fn drop(&mut self) {
            CLICK_CALLBACK.with(|callback| {
                callback.borrow_mut().take();
            });
            CALLBACK_ERROR.with(|error| {
                error.borrow_mut().take();
            });
            WIDGET_EVENT_CALLBACK.with(|callback| {
                callback.borrow_mut().take();
            });
            WIDGET_RENDER_STATE.with(|state| {
                state.borrow_mut().clear();
            });
        }
    }

    pub fn run_hello_window(
        config: HelloWindowConfig,
        on_click: Option<ClickCallback>,
    ) -> Result<HelloWindowResult, String> {
        CLICK_COUNT.store(0, Ordering::SeqCst);
        LABEL_HWND.store(0, Ordering::SeqCst);
        CALLBACK_ERROR.with(|error| {
            error.borrow_mut().take();
        });
        CLICK_CALLBACK.with(|callback| {
            *callback.borrow_mut() = on_click;
        });
        let _callback_guard = CallbackGuard;

        unsafe {
            let hinstance = GetModuleHandleW(null_mut());
            if hinstance.is_null() {
                return Err(format_last_error("GetModuleHandleW failed"));
            }

            let class_name = wide("PyNativePhase01Window");
            let cursor = LoadCursorW(null_mut(), IDC_ARROW);
            let window_class = WNDCLASSW {
                style: CS_HREDRAW | CS_VREDRAW,
                lpfnWndProc: Some(window_proc),
                hInstance: hinstance,
                hCursor: cursor,
                lpszClassName: class_name.as_ptr(),
                ..Default::default()
            };

            let class_atom = RegisterClassW(&window_class);
            if class_atom == 0 {
                let error = GetLastError();
                if error != ERROR_CLASS_ALREADY_EXISTS {
                    return Err(format!("RegisterClassW failed with Windows error {error}"));
                }
            }

            let title = wide(&config.title);
            let hwnd = CreateWindowExW(
                WINDOW_EX_STYLE::default(),
                class_name.as_ptr(),
                title.as_ptr(),
                WS_OVERLAPPEDWINDOW | WS_VISIBLE,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                520,
                240,
                null_mut(),
                null_mut(),
                hinstance,
                null_mut(),
            );

            if hwnd.is_null() {
                return Err(format_last_error("CreateWindowExW failed"));
            }

            create_child_controls(hwnd, hinstance, &config.message, &config.button_label)?;
            message_loop();
        }

        if let Some(error) = CALLBACK_ERROR.with(|error| error.borrow_mut().take()) {
            return Err(error);
        }

        Ok(HelloWindowResult {
            platform: "windows",
            clicked: CLICK_COUNT.load(Ordering::SeqCst),
        })
    }

    pub fn run_widget_tree_window(
        root: WidgetNode,
        on_event: Option<WidgetEventCallback>,
    ) -> Result<WidgetWindowResult, String> {
        WIDGET_EVENT_COUNT.store(0, Ordering::SeqCst);
        CALLBACK_ERROR.with(|error| {
            error.borrow_mut().take();
        });
        WIDGET_EVENT_CALLBACK.with(|callback| {
            *callback.borrow_mut() = on_event;
        });
        let _callback_guard = CallbackGuard;

        unsafe {
            let hinstance = GetModuleHandleW(null_mut());
            if hinstance.is_null() {
                return Err(format_last_error("GetModuleHandleW failed"));
            }

            let class_name = wide("PyNativeWidgetTreeWindow");
            let cursor = LoadCursorW(null_mut(), IDC_ARROW);
            let window_class = WNDCLASSW {
                style: CS_HREDRAW | CS_VREDRAW,
                lpfnWndProc: Some(window_proc),
                hInstance: hinstance,
                hCursor: cursor,
                lpszClassName: class_name.as_ptr(),
                ..Default::default()
            };

            let class_atom = RegisterClassW(&window_class);
            if class_atom == 0 {
                let error = GetLastError();
                if error != ERROR_CLASS_ALREADY_EXISTS {
                    return Err(format!("RegisterClassW failed with Windows error {error}"));
                }
            }

            let title = widget_window_title(&root);
            let title = wide(&title);
            let hwnd = CreateWindowExW(
                WINDOW_EX_STYLE::default(),
                class_name.as_ptr(),
                title.as_ptr(),
                WS_OVERLAPPEDWINDOW | WS_VISIBLE,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                WINDOW_WIDTH,
                WINDOW_HEIGHT,
                null_mut(),
                null_mut(),
                hinstance,
                null_mut(),
            );

            if hwnd.is_null() {
                return Err(format_last_error("CreateWindowExW failed"));
            }

            WIDGET_RENDER_STATE.with(|state| {
                let mut state = state.borrow_mut();
                state.root = Some(root);
                state.root_hwnd = hwnd;
                state.hinstance = hinstance;
            });

            render_current_widget_tree()?;
            message_loop();
        }

        if let Some(error) = CALLBACK_ERROR.with(|error| error.borrow_mut().take()) {
            return Err(error);
        }

        Ok(WidgetWindowResult {
            platform: "windows",
            events: WIDGET_EVENT_COUNT.load(Ordering::SeqCst),
        })
    }

    unsafe extern "system" fn window_proc(
        hwnd: HWND,
        message: u32,
        wparam: WPARAM,
        lparam: LPARAM,
    ) -> LRESULT {
        match message {
            WM_CREATE => 0,
            WM_COMMAND => {
                let control_id = wparam & 0xffff;
                let notification_code = (wparam >> 16) & 0xffff;
                if control_id == BUTTON_ID {
                    let clicked = CLICK_COUNT.fetch_add(1, Ordering::SeqCst) + 1;
                    let label_hwnd = LABEL_HWND.load(Ordering::SeqCst) as HWND;
                    if !label_hwnd.is_null() {
                        let text = wide(&format!(
                            "Native Windows button clicked {clicked} time(s). Close the window to return to Python."
                        ));
                        unsafe {
                            SetWindowTextW(label_hwnd, text.as_ptr());
                        }
                    }
                    run_click_callback(clicked);
                    0
                } else if run_widget_button_event(control_id, hwnd) {
                    0
                } else if notification_code == EN_CHANGE as usize
                    && run_widget_input_event(control_id, lparam as HWND, hwnd)
                {
                    0
                } else {
                    unsafe { DefWindowProcW(hwnd, message, wparam, lparam) }
                }
            }
            WM_DESTROY => {
                unsafe {
                    PostQuitMessage(0);
                }
                0
            }
            _ => unsafe { DefWindowProcW(hwnd, message, wparam, lparam) },
        }
    }

    fn create_child_controls(
        hwnd: HWND,
        hinstance: *mut core::ffi::c_void,
        message: &str,
        button_label: &str,
    ) -> Result<(), String> {
        let static_class = wide("STATIC");
        let label_text = wide(message);
        let label_hwnd = unsafe {
            CreateWindowExW(
                WINDOW_EX_STYLE::default(),
                static_class.as_ptr(),
                label_text.as_ptr(),
                WS_CHILD | WS_VISIBLE,
                24,
                24,
                460,
                48,
                hwnd,
                LABEL_ID as HMENU,
                hinstance,
                null_mut(),
            )
        };

        if label_hwnd.is_null() {
            return Err(format_last_error("CreateWindowExW STATIC failed"));
        }

        LABEL_HWND.store(label_hwnd as isize, Ordering::SeqCst);

        let button_class = wide("BUTTON");
        let button_text = wide(button_label);
        let button_hwnd = unsafe {
            CreateWindowExW(
                WINDOW_EX_STYLE::default(),
                button_class.as_ptr(),
                button_text.as_ptr(),
                WS_CHILD | WS_VISIBLE,
                24,
                96,
                200,
                36,
                hwnd,
                BUTTON_ID as HMENU,
                hinstance,
                null_mut(),
            )
        };

        if button_hwnd.is_null() {
            return Err(format_last_error("CreateWindowExW BUTTON failed"));
        }

        let font = unsafe { GetStockObject(DEFAULT_GUI_FONT) };
        if !font.is_null() {
            unsafe {
                SendMessageW(label_hwnd, WM_SETFONT, font as WPARAM, 1);
                SendMessageW(button_hwnd, WM_SETFONT, font as WPARAM, 1);
            }
        }

        Ok(())
    }

    fn message_loop() {
        let mut message = MSG::default();
        while unsafe { GetMessageW(&mut message, null_mut(), 0, 0) } > 0 {
            unsafe {
                TranslateMessage(&message);
                DispatchMessageW(&message);
            }
        }
    }

    fn run_click_callback(clicked: usize) {
        let result = CLICK_CALLBACK.with(|callback| {
            let mut callback = callback.borrow_mut();
            callback.as_mut().map(|callback| callback(clicked))
        });

        if let Some(Err(error)) = result {
            CALLBACK_ERROR.with(|callback_error| {
                *callback_error.borrow_mut() = Some(error);
            });
        }
    }

    fn run_widget_button_event(control_id: usize, hwnd: HWND) -> bool {
        let callback_id = WIDGET_RENDER_STATE
            .with(|state| state.borrow().button_callbacks.get(&control_id).copied());

        let Some(callback_id) = callback_id else {
            return false;
        };

        let event = WidgetEvent {
            kind: "button_click",
            callback_id,
            value: None,
        };
        run_widget_event_callback(event, hwnd);
        true
    }

    fn run_widget_input_event(control_id: usize, input_hwnd: HWND, root_hwnd: HWND) -> bool {
        let callback_id = WIDGET_RENDER_STATE
            .with(|state| state.borrow().input_callbacks.get(&control_id).copied());

        let Some(callback_id) = callback_id else {
            return false;
        };

        let value = window_text(input_hwnd).unwrap_or_default();
        let event = WidgetEvent {
            kind: "input_change",
            callback_id,
            value: Some(value),
        };
        run_widget_event_callback(event, root_hwnd);
        true
    }

    fn run_widget_event_callback(event: WidgetEvent, hwnd: HWND) {
        let should_render = event.kind != "input_change";
        WIDGET_EVENT_COUNT.fetch_add(1, Ordering::SeqCst);
        let result = WIDGET_EVENT_CALLBACK.with(|callback| {
            let mut callback = callback.borrow_mut();
            callback
                .as_mut()
                .map(|callback| callback(event))
                .unwrap_or(Ok(None))
        });

        match result {
            Ok(Some(updated_root)) => {
                WIDGET_RENDER_STATE.with(|state| {
                    state.borrow_mut().root = Some(updated_root);
                });
                if should_render {
                    if let Err(error) = render_current_widget_tree() {
                        CALLBACK_ERROR.with(|callback_error| {
                            *callback_error.borrow_mut() = Some(error);
                        });
                        unsafe {
                            DestroyWindow(hwnd);
                        }
                    }
                }
            }
            Ok(None) => {}
            Err(error) => {
                CALLBACK_ERROR.with(|callback_error| {
                    *callback_error.borrow_mut() = Some(error);
                });
                unsafe {
                    DestroyWindow(hwnd);
                }
            }
        }
    }

    fn render_current_widget_tree() -> Result<(), String> {
        let (root, hwnd, hinstance) = WIDGET_RENDER_STATE.with(|state| {
            let state = state.borrow();
            (state.root.clone(), state.root_hwnd, state.hinstance)
        });

        let Some(root) = root else {
            return Ok(());
        };

        clear_widget_controls();
        let window = visible_window_node(&root);
        let title = widget_window_title(&root);
        let title = wide(&title);
        unsafe {
            SetWindowTextW(hwnd, title.as_ptr());
        }

        let content = window
            .map(|window| window.children.as_slice())
            .unwrap_or(&root.children);
        let mut y = CONTENT_Y;
        for child in content {
            y += render_node(hwnd, hinstance, child, CONTENT_X, y, CONTENT_WIDTH)?;
            y += ROW_GAP;
        }

        Ok(())
    }

    fn clear_widget_controls() {
        let controls = WIDGET_RENDER_STATE.with(|state| {
            let mut state = state.borrow_mut();
            let controls = std::mem::take(&mut state.controls);
            state.button_callbacks.clear();
            state.input_callbacks.clear();
            state.next_control_id = FIRST_WIDGET_CONTROL_ID;
            controls
        });

        for hwnd in controls {
            unsafe {
                DestroyWindow(hwnd);
            }
        }
    }

    fn render_node(
        parent: HWND,
        hinstance: *mut core::ffi::c_void,
        node: &WidgetNode,
        x: i32,
        y: i32,
        width: i32,
    ) -> Result<i32, String> {
        match node.kind.as_str() {
            "App" | "Window" | "Column" => {
                render_column(parent, hinstance, &node.children, x, y, width)
            }
            "Row" => render_row(parent, hinstance, &node.children, x, y, width),
            "Text" => create_static(
                parent,
                hinstance,
                x,
                y,
                width,
                &prop_string(node, "value", ""),
            ),
            "Button" => create_button(parent, hinstance, x, y, width, node),
            "Input" => create_input(parent, hinstance, x, y, width, node),
            other => create_static(
                parent,
                hinstance,
                x,
                y,
                width,
                &format!("Unsupported widget: {other}"),
            ),
        }
    }

    fn render_column(
        parent: HWND,
        hinstance: *mut core::ffi::c_void,
        children: &[WidgetNode],
        x: i32,
        mut y: i32,
        width: i32,
    ) -> Result<i32, String> {
        let start_y = y;
        for child in children {
            y += render_node(parent, hinstance, child, x, y, width)?;
            y += ROW_GAP;
        }

        Ok((y - start_y).max(1))
    }

    fn render_row(
        parent: HWND,
        hinstance: *mut core::ffi::c_void,
        children: &[WidgetNode],
        x: i32,
        y: i32,
        width: i32,
    ) -> Result<i32, String> {
        if children.is_empty() {
            return Ok(1);
        }

        let gap = ROW_GAP;
        let cell_width =
            ((width - gap * (children.len() as i32 - 1)) / children.len() as i32).max(80);
        let mut max_height = 1;

        for (index, child) in children.iter().enumerate() {
            let child_x = x + index as i32 * (cell_width + gap);
            let height = render_node(parent, hinstance, child, child_x, y, cell_width)?;
            max_height = max_height.max(height);
        }

        Ok(max_height)
    }

    fn create_static(
        parent: HWND,
        hinstance: *mut core::ffi::c_void,
        x: i32,
        y: i32,
        width: i32,
        text: &str,
    ) -> Result<i32, String> {
        let hwnd = create_control(
            "STATIC",
            text,
            WS_CHILD | WS_VISIBLE,
            parent,
            hinstance,
            0,
            x,
            y,
            width,
            28,
        )?;
        set_default_font(hwnd);
        remember_control(hwnd);
        Ok(28)
    }

    fn create_button(
        parent: HWND,
        hinstance: *mut core::ffi::c_void,
        x: i32,
        y: i32,
        width: i32,
        node: &WidgetNode,
    ) -> Result<i32, String> {
        let control_id = next_widget_control_id();
        let label = prop_string(node, "label", "Button");
        let hwnd = create_control(
            "BUTTON",
            &label,
            WS_CHILD | WS_VISIBLE,
            parent,
            hinstance,
            control_id,
            x,
            y,
            width.min(220),
            36,
        )?;
        set_default_font(hwnd);
        remember_control(hwnd);

        if let Some(callback_id) = prop_u64(node, "callback_id") {
            WIDGET_RENDER_STATE.with(|state| {
                state
                    .borrow_mut()
                    .button_callbacks
                    .insert(control_id, callback_id);
            });
        }

        Ok(36)
    }

    fn create_input(
        parent: HWND,
        hinstance: *mut core::ffi::c_void,
        x: i32,
        y: i32,
        width: i32,
        node: &WidgetNode,
    ) -> Result<i32, String> {
        let value = prop_string(node, "value", "");
        let placeholder = prop_string(node, "placeholder", "");
        let text = if value.is_empty() { placeholder } else { value };
        let control_id = next_widget_control_id();
        let hwnd = create_control(
            "EDIT",
            &text,
            WS_CHILD | WS_VISIBLE | WS_BORDER | ES_AUTOHSCROLL as u32,
            parent,
            hinstance,
            control_id,
            x,
            y,
            width.min(320),
            30,
        )?;
        set_default_font(hwnd);
        remember_control(hwnd);

        if let Some(callback_id) = prop_u64(node, "state_id") {
            WIDGET_RENDER_STATE.with(|state| {
                state
                    .borrow_mut()
                    .input_callbacks
                    .insert(control_id, callback_id);
            });
        }

        Ok(30)
    }

    #[allow(clippy::too_many_arguments)]
    fn create_control(
        class_name: &str,
        text: &str,
        style: u32,
        parent: HWND,
        hinstance: *mut core::ffi::c_void,
        control_id: usize,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ) -> Result<HWND, String> {
        let class_name = wide(class_name);
        let text = wide(text);
        let hwnd = unsafe {
            CreateWindowExW(
                WINDOW_EX_STYLE::default(),
                class_name.as_ptr(),
                text.as_ptr(),
                style,
                x,
                y,
                width,
                height,
                parent,
                control_id as HMENU,
                hinstance,
                null_mut(),
            )
        };

        if hwnd.is_null() {
            return Err(format_last_error("CreateWindowExW control failed"));
        }

        Ok(hwnd)
    }

    fn remember_control(hwnd: HWND) {
        WIDGET_RENDER_STATE.with(|state| {
            state.borrow_mut().controls.push(hwnd);
        });
    }

    fn next_widget_control_id() -> usize {
        WIDGET_RENDER_STATE.with(|state| {
            let mut state = state.borrow_mut();
            let control_id = state.next_control_id;
            state.next_control_id += 1;
            control_id
        })
    }

    fn set_default_font(hwnd: HWND) {
        let font = unsafe { GetStockObject(DEFAULT_GUI_FONT) };
        if !font.is_null() {
            unsafe {
                SendMessageW(hwnd, WM_SETFONT, font as WPARAM, 1);
            }
        }
    }

    impl WidgetRenderState {
        fn clear(&mut self) {
            self.root = None;
            self.controls.clear();
            self.button_callbacks.clear();
            self.input_callbacks.clear();
            self.next_control_id = FIRST_WIDGET_CONTROL_ID;
            self.root_hwnd = null_mut();
            self.hinstance = null_mut();
        }
    }

    fn visible_window_node(root: &WidgetNode) -> Option<&WidgetNode> {
        if root.kind == "Window" {
            Some(root)
        } else {
            root.children.iter().find_map(visible_window_node)
        }
    }

    fn widget_window_title(root: &WidgetNode) -> String {
        visible_window_node(root)
            .map(|window| prop_string(window, "title", "PyNative UI"))
            .unwrap_or_else(|| "PyNative UI".to_string())
    }

    fn prop_string(node: &WidgetNode, key: &str, default: &str) -> String {
        node.props
            .get(key)
            .and_then(|value| value.as_str())
            .unwrap_or(default)
            .to_string()
    }

    fn prop_u64(node: &WidgetNode, key: &str) -> Option<u64> {
        node.props.get(key).and_then(|value| value.as_u64())
    }

    fn window_text(hwnd: HWND) -> Result<String, String> {
        let length = unsafe { GetWindowTextLengthW(hwnd) };
        if length < 0 {
            return Err(format_last_error("GetWindowTextLengthW failed"));
        }

        let mut buffer = vec![0u16; length as usize + 1];
        let copied = unsafe { GetWindowTextW(hwnd, buffer.as_mut_ptr(), buffer.len() as i32) };
        if copied < 0 {
            return Err(format_last_error("GetWindowTextW failed"));
        }

        Ok(String::from_utf16_lossy(&buffer[..copied as usize]))
    }

    fn format_last_error(context: &str) -> String {
        let error = unsafe { GetLastError() };
        format!("{context} with Windows error {error}")
    }

    fn wide(value: &str) -> Vec<u16> {
        value.encode_utf16().chain(Some(0)).collect()
    }
}
