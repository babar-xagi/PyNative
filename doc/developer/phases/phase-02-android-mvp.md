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

The build packages runtime assets under `assets/pynative`:

- `app.py`
- `widget_tree.json`
- `app_spec.json`
- `runtime.json`

Runtime assets use stable IDs:

```text
node:/0/0/1
event:/0/0/1
state:/0/0/2
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
- Python app source and widget-tree assets are packaged into the APK.
- Android button events are sent to Rust as JSON and receive JSON responses.
- Rust initializes a runtime session from packaged `runtime.json`, `app.py`, and `widget_tree.json`.
- Rust event responses include an updated widget-tree preview for the current Android UI refresh loop.
- Runtime assets and events use stable node/event/state IDs instead of Python memory addresses.
- Android log diagnostics through `Log.i("PyNative", ...)`.

## Workstreams

- Android Gradle project template.
- Kotlin to Rust integration.
- Rust to Python bridge strategy on Android.
- Asset and Python source packaging.
- Device/emulator deployment.
- Mobile logging and diagnostics.
- Compatibility notes for pure Python packages.
 
## Current Event Protocol

Android sends:

```json
{"kind":"button_click","event_id":"event:/0/0/1","node_id":"node:/0/0/1","label":"Increase","ui_count":1}
```

Rust returns:

```json
{"ok":true,"protocol":"pynative.android.event.v1","native_events":1,"python_runtime":"not_embedded"}
```

Before dispatching events, Android initializes the Rust runtime session:

```json
{"protocol":"pynative.android.runtime.v1","runtime_loaded":true,"python_runtime":"not_embedded"}
```

The event response can include:

```json
{"updated_by":"rust_preview","updated_text":"Count: 1","updated_widget_tree":{}}
```

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
