# Phase 2 - Android MVP

## Goal

Create an Android proof of concept that uses a Kotlin shell, Rust core integration, embedded Python strategy, APK build, and core widgets.

## Current Android Experiment

```powershell
uv run pynative run android --build-only
```

This builds a tiny native Android APK through the installed Android SDK tools directly:

- `aapt2`
- `javac`
- `d8`
- `zipalign`
- `apksigner`

It does not use Gradle yet and does not embed Python yet. It proves the local Android build/install path before the real Android runtime is added.

The experiment can also build from a PyNative app target:

```powershell
uv run pynative run android examples\counter\app.py --build-only
uv run pynative build apk examples\counter\app.py
```

This exports a build-time Android screen spec from the Python widget tree and generates a Java `GeneratedApp` class for the Activity.

The APK also packages a Rust JNI bridge. The default ABI is `arm64-v8a` for real phones:

```powershell
uv run pynative build apk examples\counter\app.py
```

For an x86_64 emulator:

```powershell
uv run pynative build apk examples\counter\app.py --android-abi x86_64
```

## Deliverables

- Android/Kotlin shell for lifecycle and permissions.
- Rust core integration for shared runtime behavior.
- Embedded Python proof of concept for pure Python logic.
- Basic Android screen with text and button.
- Python callback from Android button click.
- CLI command: `pynative run android`.
- CLI command: `pynative build apk`.

## Delivered In Current Experiment

- Java Android shell for lifecycle and native controls.
- Build-only and install/launch flows through `pynative run android`.
- `pynative build apk` command.
- Build-time widget export for `Text`, `Button`, `Input`, and `Image` placeholders.
- Basic Android button state update inside the generated screen shell.
- Rust JNI bridge crate packaged as `libpynative_android_bridge.so`.
- Android button events are forwarded into Rust and counted by the native bridge.
- Android log diagnostics through `Log.i("PyNative", ...)`.

## Workstreams

- Android Gradle project template.
- Kotlin to Rust integration.
- Rust to Python bridge strategy on Android.
- Asset and Python source packaging.
- Device/emulator deployment.
- Mobile logging and diagnostics.
- Compatibility notes for pure Python packages.

## Acceptance Criteria

- A generated app can build as an APK.
- The APK launches on an emulator or device.
- A screen renders using the shared widget model.
- A button callback reaches Python.
- Android logs show useful framework diagnostics.
- The docs explain what Python package types are supported.

## Exit Gate

Phase 2 is complete when Android can run a small app with at least text, layout, button callback, and simple state update.

## Enhancement Added

Create an Android compatibility page during this phase, even if support is tiny. Honest limits are better than vague promises.
