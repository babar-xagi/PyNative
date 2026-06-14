# Android Experiment

PyNative UI has an early Android experiment that builds a tiny native Android APK without Gradle.

This is not the final Android runtime yet. It proves that the local Android SDK can build, sign, install, launch, and render a build-time projection of a PyNative app.

## Build APK

```powershell
uv run pynative run android --build-only
```

Build from a PyNative app file:

```powershell
uv run pynative run android examples\counter\app.py --build-only
```

Or use the dedicated build command:

```powershell
uv run pynative build apk examples\counter\app.py
```

The default ABI is `arm64-v8a`, which is the right choice for most real phones. For an x86_64 emulator:

```powershell
uv run pynative build apk examples\counter\app.py --android-abi x86_64
uv run pynative run android examples\counter\app.py --android-abi x86_64
```

Output:

```text
build\android-experiment\pynative-android-debug.apk
```

The APK includes:

```text
assets/pynative/app.py
assets/pynative/widget_tree.json
assets/pynative/runtime.json
lib/arm64-v8a/libpynative_android_bridge.so
```

## Install And Launch

Connect an Android device with USB debugging enabled, or start an Android emulator, then run:

```powershell
uv run pynative run android examples\counter\app.py
```

This builds the APK, installs it with `adb install -r`, and launches:

```text
com.pynative.experiment/.MainActivity
```

## Current Limits

- Uses a Java Activity for the experiment.
- Does not embed Python yet.
- Loads a Rust JNI bridge on Android and sends button events into Rust.
- Packages the Python app source and widget tree as APK assets.
- Sends Android button events to Rust as JSON and receives a JSON response.
- Initializes a Rust runtime session from packaged APK assets.
- Returns a Rust-side updated widget tree preview in event responses.
- Uses stable runtime IDs for nodes, events, and state bindings.
- Renders a build-time export of the Python widget tree, not a live Python runtime.
- Android button taps update local Android state, but Python callbacks do not run on-device yet.

Those are the next Phase 2 steps.
