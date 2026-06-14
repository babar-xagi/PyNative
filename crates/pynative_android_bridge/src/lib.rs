use std::ffi::c_void;
use std::sync::atomic::{AtomicI32, Ordering};

type JNIEnv = *mut c_void;
type JClass = *mut c_void;
type JString = *mut c_void;
type JBoolean = u8;
type JInt = i32;

static BUTTON_EVENT_COUNT: AtomicI32 = AtomicI32::new(0);

pub fn runtime_phase() -> JInt {
    2
}

pub fn record_button_event() -> JInt {
    BUTTON_EVENT_COUNT.fetch_add(1, Ordering::SeqCst) + 1
}

pub fn last_button_event_count() -> JInt {
    BUTTON_EVENT_COUNT.load(Ordering::SeqCst)
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
}
