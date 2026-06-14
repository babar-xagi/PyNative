package com.pynative.experiment;

import android.util.Log;

public final class PyNativeBridge {
    private static final String TAG = "PyNative";
    private static final boolean AVAILABLE;
    private static final String LOAD_ERROR;

    static {
        boolean available = false;
        String loadError = "";

        try {
            System.loadLibrary("pynative_android_bridge");
            available = true;
            Log.i(TAG, "Rust JNI bridge loaded.");
        } catch (UnsatisfiedLinkError error) {
            loadError = error.getMessage();
            Log.w(TAG, "Rust JNI bridge not loaded: " + loadError);
        }

        AVAILABLE = available;
        LOAD_ERROR = loadError;
    }

    private PyNativeBridge() {
    }

    public static boolean isAvailable() {
        return AVAILABLE;
    }

    public static String status() {
        if (AVAILABLE) {
            return "Rust JNI bridge ready. Phase " + runtimePhase() + ".";
        }

        return "Rust JNI bridge unavailable: " + LOAD_ERROR;
    }

    public static int runtimePhase() {
        if (!AVAILABLE) {
            return 0;
        }

        return nativeRuntimePhase();
    }

    public static int buttonEvent(String label, int uiCount, boolean hasPythonCallbacks) {
        if (!AVAILABLE) {
            return -1;
        }

        return nativeButtonEvent(label, uiCount, hasPythonCallbacks);
    }

    public static String initializeRuntimeJson(
            String runtimeJson,
            String appSource,
            String widgetTreeJson
    ) {
        if (!AVAILABLE) {
            return "{\"ok\":false,\"error\":\"Rust JNI bridge unavailable\",\"load_error\":\""
                    + jsonEscape(LOAD_ERROR)
                    + "\"}";
        }

        return nativeInitializeRuntimeJson(runtimeJson, appSource, widgetTreeJson);
    }

    public static String dispatchEventJson(String eventJson) {
        if (!AVAILABLE) {
            return "{\"ok\":false,\"error\":\"Rust JNI bridge unavailable\",\"load_error\":\""
                    + jsonEscape(LOAD_ERROR)
                    + "\"}";
        }

        return nativeDispatchEventJson(eventJson);
    }

    public static int lastButtonEventCount() {
        if (!AVAILABLE) {
            return 0;
        }

        return nativeLastButtonEventCount();
    }

    private static native int nativeRuntimePhase();

    private static native int nativeButtonEvent(
            String label,
            int uiCount,
            boolean hasPythonCallbacks
    );

    private static native String nativeInitializeRuntimeJson(
            String runtimeJson,
            String appSource,
            String widgetTreeJson
    );

    private static native String nativeDispatchEventJson(String eventJson);

    private static native int nativeLastButtonEventCount();

    private static String jsonEscape(String value) {
        if (value == null) {
            return "";
        }

        return value
                .replace("\\", "\\\\")
                .replace("\"", "\\\"")
                .replace("\r", "\\r")
                .replace("\n", "\\n")
                .replace("\t", "\\t");
    }
}
