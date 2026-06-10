use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;

fn to_py_runtime_error(error: impl std::fmt::Display) -> PyErr {
    PyRuntimeError::new_err(error.to_string())
}

#[pyfunction]
fn runtime_info() -> PyResult<String> {
    serde_json::to_string(&pynative_core::runtime_info()).map_err(to_py_runtime_error)
}

#[pyfunction]
fn summarize_widget_tree_json(tree_json: &str) -> PyResult<String> {
    let summary =
        pynative_core::summarize_widget_tree_json(tree_json).map_err(to_py_runtime_error)?;
    serde_json::to_string(&summary).map_err(to_py_runtime_error)
}

fn parse_widget_tree_json(tree_json: &str) -> PyResult<pynative_core::WidgetNode> {
    pynative_core::widget_tree_from_json(tree_json).map_err(to_py_runtime_error)
}

#[pyfunction]
fn echo_event(event_name: &str) -> PyResult<String> {
    Ok(format!("event:{event_name}"))
}

#[pyfunction]
#[pyo3(signature = (title = "PyNative UI Phase 0.1", message = "Rust opened a native Windows window.", button_label = "Click native button"))]
fn run_windows_hello_window(title: &str, message: &str, button_label: &str) -> PyResult<String> {
    let result =
        pynative_platform_windows::run_hello_window(pynative_platform_windows::HelloWindowConfig {
            title: title.to_string(),
            message: message.to_string(),
            button_label: button_label.to_string(),
        })
        .map_err(to_py_runtime_error)?;

    serde_json::to_string(&result).map_err(to_py_runtime_error)
}

#[pyfunction]
#[pyo3(signature = (callback, title = "PyNative UI Phase 0.2", message = "Native button will call Python live.", button_label = "Call Python"))]
fn run_windows_callback_window(
    callback: Py<PyAny>,
    title: &str,
    message: &str,
    button_label: &str,
) -> PyResult<String> {
    let result = pynative_platform_windows::run_hello_window_with_callback(
        pynative_platform_windows::HelloWindowConfig {
            title: title.to_string(),
            message: message.to_string(),
            button_label: button_label.to_string(),
        },
        move |clicked| {
            Python::attach(|py| {
                callback
                    .bind(py)
                    .call1((clicked,))
                    .map(|_| ())
                    .map_err(|error| error.to_string())
            })
        },
    )
    .map_err(to_py_runtime_error)?;

    serde_json::to_string(&result).map_err(to_py_runtime_error)
}

#[pyfunction]
fn run_windows_widget_tree_json(tree_json: &str) -> PyResult<String> {
    let root = parse_widget_tree_json(tree_json)?;
    let result =
        pynative_platform_windows::run_widget_tree_window(root).map_err(to_py_runtime_error)?;
    serde_json::to_string(&result).map_err(to_py_runtime_error)
}

#[pyfunction]
fn run_windows_widget_tree_json_with_events(
    tree_json: &str,
    callback: Py<PyAny>,
) -> PyResult<String> {
    let root = parse_widget_tree_json(tree_json)?;
    let result =
        pynative_platform_windows::run_widget_tree_window_with_events(root, move |event| {
            let event_json = serde_json::to_string(&event).map_err(|error| error.to_string())?;
            Python::attach(|py| {
                let result = callback
                    .bind(py)
                    .call1((event_json,))
                    .map_err(|error| error.to_string())?;

                if result.is_none() {
                    return Ok(None);
                }

                let updated_tree_json = result
                    .extract::<String>()
                    .map_err(|error| error.to_string())?;
                let updated_tree = pynative_core::widget_tree_from_json(&updated_tree_json)
                    .map_err(|error| error.to_string())?;

                Ok(Some(updated_tree))
            })
        })
        .map_err(to_py_runtime_error)?;

    serde_json::to_string(&result).map_err(to_py_runtime_error)
}

#[pymodule]
#[pyo3(name = "_native")]
fn pynative_native(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(runtime_info, m)?)?;
    m.add_function(wrap_pyfunction!(summarize_widget_tree_json, m)?)?;
    m.add_function(wrap_pyfunction!(echo_event, m)?)?;
    m.add_function(wrap_pyfunction!(run_windows_hello_window, m)?)?;
    m.add_function(wrap_pyfunction!(run_windows_callback_window, m)?)?;
    m.add_function(wrap_pyfunction!(run_windows_widget_tree_json, m)?)?;
    m.add_function(wrap_pyfunction!(
        run_windows_widget_tree_json_with_events,
        m
    )?)?;
    Ok(())
}
