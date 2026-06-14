use std::ffi::{CStr, CString, c_char, c_void};
use std::ptr::null_mut;
use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::{Mutex, OnceLock};

type JNIEnv = *mut *const c_void;
type JClass = *mut c_void;
type JString = *mut c_void;
type JBoolean = u8;
type JInt = i32;

const JNI_NEW_STRING_UTF_INDEX: usize = 167;
const JNI_GET_STRING_UTF_CHARS_INDEX: usize = 169;
const JNI_RELEASE_STRING_UTF_CHARS_INDEX: usize = 170;

static BUTTON_EVENT_COUNT: AtomicI32 = AtomicI32::new(0);
static RUNTIME_SESSION: OnceLock<Mutex<RuntimeSession>> = OnceLock::new();

#[derive(Debug, Clone)]
struct RuntimeSession {
    initialized: bool,
    runtime: serde_json::Value,
    widget_tree: serde_json::Value,
    app_source_len: usize,
}

impl Default for RuntimeSession {
    fn default() -> Self {
        Self {
            initialized: false,
            runtime: serde_json::Value::Null,
            widget_tree: serde_json::Value::Null,
            app_source_len: 0,
        }
    }
}

pub fn runtime_phase() -> JInt {
    2
}

pub fn record_button_event() -> JInt {
    BUTTON_EVENT_COUNT.fetch_add(1, Ordering::SeqCst) + 1
}

pub fn last_button_event_count() -> JInt {
    BUTTON_EVENT_COUNT.load(Ordering::SeqCst)
}

pub fn initialize_runtime_json(
    runtime_json: &str,
    app_source: &str,
    widget_tree_json: &str,
) -> String {
    let runtime = parse_json_or_error(runtime_json);
    let widget_tree = parse_json_or_error(widget_tree_json);
    let title = runtime
        .get("title")
        .and_then(serde_json::Value::as_str)
        .unwrap_or("PyNative Android")
        .to_string();
    let node_count = runtime
        .get("node_count")
        .and_then(serde_json::Value::as_i64)
        .unwrap_or_else(|| count_nodes(&widget_tree) as i64);

    let session = RuntimeSession {
        initialized: true,
        runtime,
        widget_tree,
        app_source_len: app_source.len(),
    };

    if let Ok(mut current) = runtime_session().lock() {
        *current = session;
    }

    serde_json::json!({
        "ok": true,
        "protocol": "pynative.android.runtime.v1",
        "runtime_loaded": true,
        "python_runtime": "not_embedded",
        "title": title,
        "node_count": node_count,
        "app_source_len": app_source.len(),
    })
    .to_string()
}

pub fn dispatch_event_json(event_json: &str) -> String {
    let native_events = record_button_event();
    let event = serde_json::from_str::<serde_json::Value>(event_json).unwrap_or_else(|error| {
        serde_json::json!({
            "kind": "invalid",
            "parse_error": error.to_string(),
        })
    });
    let kind = event
        .get("kind")
        .and_then(serde_json::Value::as_str)
        .unwrap_or("unknown");
    let event_id = event
        .get("event_id")
        .and_then(serde_json::Value::as_str)
        .unwrap_or("");
    let node_id = event
        .get("node_id")
        .and_then(serde_json::Value::as_str)
        .unwrap_or("");
    let session = runtime_session()
        .lock()
        .map(|session| session.clone())
        .unwrap_or_default();
    let event_registered = session_event_registered(&session, event_id);
    let updated_widget_tree = if session.initialized {
        Some(updated_tree_for_event(
            session.widget_tree.clone(),
            &event,
            native_events,
        ))
    } else {
        None
    };
    let updated_text = updated_widget_tree
        .as_ref()
        .and_then(first_text_value)
        .unwrap_or_default();
    let runtime_title = session
        .runtime
        .get("title")
        .and_then(serde_json::Value::as_str)
        .unwrap_or("PyNative Android");

    serde_json::json!({
        "ok": true,
        "protocol": "pynative.android.event.v1",
        "kind": kind,
        "event_id": event_id,
        "node_id": node_id,
        "event_registered": event_registered,
        "native_events": native_events,
        "python_runtime": "not_embedded",
        "runtime_loaded": session.initialized,
        "runtime_title": runtime_title,
        "app_source_len": session.app_source_len,
        "updated_by": if session.initialized { "rust_preview" } else { "none" },
        "updated_text": updated_text,
        "updated_widget_tree": updated_widget_tree,
        "event": event,
    })
    .to_string()
}

fn session_event_registered(session: &RuntimeSession, event_id: &str) -> bool {
    if event_id.is_empty() {
        return false;
    }

    session
        .runtime
        .get("events")
        .and_then(serde_json::Value::as_array)
        .is_some_and(|events| {
            events.iter().any(|event| {
                event
                    .get("event_id")
                    .and_then(serde_json::Value::as_str)
                    .is_some_and(|registered| registered == event_id)
            })
        })
}

fn runtime_session() -> &'static Mutex<RuntimeSession> {
    RUNTIME_SESSION.get_or_init(|| Mutex::new(RuntimeSession::default()))
}

fn parse_json_or_error(input: &str) -> serde_json::Value {
    serde_json::from_str(input).unwrap_or_else(|error| {
        serde_json::json!({
            "parse_error": error.to_string(),
        })
    })
}

fn updated_tree_for_event(
    mut widget_tree: serde_json::Value,
    event: &serde_json::Value,
    native_events: JInt,
) -> serde_json::Value {
    let is_button = event
        .get("kind")
        .and_then(serde_json::Value::as_str)
        .is_some_and(|kind| kind == "button_click");

    if is_button {
        update_first_text_value(&mut widget_tree, format!("Count: {native_events}"));
    }

    widget_tree
}

fn update_first_text_value(node: &mut serde_json::Value, text: String) -> bool {
    if node
        .get("kind")
        .and_then(serde_json::Value::as_str)
        .is_some_and(|kind| kind == "Text")
    {
        if let Some(props) = node
            .get_mut("props")
            .and_then(serde_json::Value::as_object_mut)
        {
            props.insert("value".to_string(), serde_json::Value::String(text));
            return true;
        }
    }

    let Some(children) = node
        .get_mut("children")
        .and_then(serde_json::Value::as_array_mut)
    else {
        return false;
    };

    children
        .iter_mut()
        .any(|child| update_first_text_value(child, text.clone()))
}

fn first_text_value(node: &serde_json::Value) -> Option<String> {
    if node
        .get("kind")
        .and_then(serde_json::Value::as_str)
        .is_some_and(|kind| kind == "Text")
    {
        return node
            .get("props")
            .and_then(|props| props.get("value"))
            .and_then(serde_json::Value::as_str)
            .map(str::to_string);
    }

    node.get("children")
        .and_then(serde_json::Value::as_array)?
        .iter()
        .find_map(first_text_value)
}

fn count_nodes(node: &serde_json::Value) -> usize {
    1 + node
        .get("children")
        .and_then(serde_json::Value::as_array)
        .map(|children| children.iter().map(count_nodes).sum())
        .unwrap_or(0)
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_com_pynative_experiment_PyNativeBridge_nativeRuntimePhase(
    _env: JNIEnv,
    _class: JClass,
) -> JInt {
    runtime_phase()
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_com_pynative_experiment_PyNativeBridge_nativeButtonEvent(
    _env: JNIEnv,
    _class: JClass,
    _label: JString,
    _ui_count: JInt,
    _has_python_callbacks: JBoolean,
) -> JInt {
    record_button_event()
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_com_pynative_experiment_PyNativeBridge_nativeLastButtonEventCount(
    _env: JNIEnv,
    _class: JClass,
) -> JInt {
    last_button_event_count()
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_com_pynative_experiment_PyNativeBridge_nativeInitializeRuntimeJson(
    env: JNIEnv,
    _class: JClass,
    runtime_json: JString,
    app_source: JString,
    widget_tree_json: JString,
) -> JString {
    let runtime_json = unsafe { jstring_to_string(env, runtime_json) }.unwrap_or_default();
    let app_source = unsafe { jstring_to_string(env, app_source) }.unwrap_or_default();
    let widget_tree_json = unsafe { jstring_to_string(env, widget_tree_json) }.unwrap_or_default();
    let response = initialize_runtime_json(&runtime_json, &app_source, &widget_tree_json);

    unsafe { string_to_jstring(env, &response) }
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_com_pynative_experiment_PyNativeBridge_nativeDispatchEventJson(
    env: JNIEnv,
    _class: JClass,
    event_json: JString,
) -> JString {
    let event_json = unsafe { jstring_to_string(env, event_json) }.unwrap_or_else(|| {
        r#"{"kind":"invalid","error":"could not read Java string"}"#.to_string()
    });
    let response = dispatch_event_json(&event_json);

    unsafe { string_to_jstring(env, &response) }
}

type NewStringUtf = unsafe extern "system" fn(JNIEnv, *const c_char) -> JString;
type GetStringUtfChars = unsafe extern "system" fn(JNIEnv, JString, *mut JBoolean) -> *const c_char;
type ReleaseStringUtfChars = unsafe extern "system" fn(JNIEnv, JString, *const c_char);

unsafe fn jni_function<T: Copy>(env: JNIEnv, index: usize) -> Option<T> {
    if env.is_null() {
        return None;
    }

    let table = unsafe { *env } as *const *const c_void;
    if table.is_null() {
        return None;
    }

    let pointer = unsafe { *table.add(index) };
    if pointer.is_null() {
        return None;
    }

    Some(unsafe { std::mem::transmute_copy(&pointer) })
}

unsafe fn jstring_to_string(env: JNIEnv, value: JString) -> Option<String> {
    if value.is_null() {
        return None;
    }

    let get_chars: GetStringUtfChars =
        unsafe { jni_function(env, JNI_GET_STRING_UTF_CHARS_INDEX)? };
    let release_chars: ReleaseStringUtfChars =
        unsafe { jni_function(env, JNI_RELEASE_STRING_UTF_CHARS_INDEX)? };
    let chars = unsafe { get_chars(env, value, null_mut()) };
    if chars.is_null() {
        return None;
    }

    let output = unsafe { CStr::from_ptr(chars) }
        .to_string_lossy()
        .into_owned();
    unsafe {
        release_chars(env, value, chars);
    }

    Some(output)
}

unsafe fn string_to_jstring(env: JNIEnv, value: &str) -> JString {
    let Some(new_string_utf) =
        (unsafe { jni_function::<NewStringUtf>(env, JNI_NEW_STRING_UTF_INDEX) })
    else {
        return null_mut();
    };

    let safe_value = value.replace('\0', "\\u0000");
    let Ok(value) = CString::new(safe_value) else {
        return null_mut();
    };

    unsafe { new_string_utf(env, value.as_ptr()) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reports_android_runtime_phase() {
        assert_eq!(runtime_phase(), 2);
    }

    #[test]
    fn counts_button_events() {
        let before = last_button_event_count();
        let after = record_button_event();

        assert_eq!(after, before + 1);
        assert_eq!(last_button_event_count(), after);
    }

    #[test]
    fn dispatches_json_event() {
        initialize_runtime_json(
            r#"{"title":"Counter App","node_count":5,"events":[{"event_id":"event:/0"}]}"#,
            "from pynative import App",
            r#"{"kind":"App","props":{},"children":[{"kind":"Text","props":{"value":"Count: 0"},"children":[]}]}"#,
        );
        let response = dispatch_event_json(
            r#"{"kind":"button_click","event_id":"event:/0","node_id":"node:/0","label":"Increase","ui_count":1}"#,
        );
        let response: serde_json::Value = serde_json::from_str(&response).unwrap();

        assert_eq!(response["ok"], true);
        assert_eq!(response["protocol"], "pynative.android.event.v1");
        assert_eq!(response["kind"], "button_click");
        assert_eq!(response["event_id"], "event:/0");
        assert_eq!(response["node_id"], "node:/0");
        assert_eq!(response["event_registered"], true);
        assert!(response["native_events"].as_i64().unwrap() > 0);
        assert_eq!(response["python_runtime"], "not_embedded");
        assert_eq!(response["runtime_loaded"], true);
        assert_eq!(response["updated_by"], "rust_preview");
        assert!(
            response["updated_text"]
                .as_str()
                .unwrap()
                .starts_with("Count:")
        );
    }

    #[test]
    fn initializes_runtime_session() {
        let response = initialize_runtime_json(
            r#"{"title":"Counter App","node_count":5}"#,
            "print('hello')",
            r#"{"kind":"App","props":{},"children":[]}"#,
        );
        let response: serde_json::Value = serde_json::from_str(&response).unwrap();

        assert_eq!(response["ok"], true);
        assert_eq!(response["protocol"], "pynative.android.runtime.v1");
        assert_eq!(response["runtime_loaded"], true);
        assert_eq!(response["title"], "Counter App");
        assert_eq!(response["app_source_len"], 14);
    }
}
