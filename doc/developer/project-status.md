# Project Status

Snapshot date: 2026-06-13

## Current Phase

PyNative UI is in Phase 1 Desktop MVP, with Phase 2 Android exploration started.

The project now has a working Python package, Rust/PyO3 bridge, basic Windows native renderer, CLI workflow, examples, tests, documentation, style support, and an Android APK experiment with a Rust JNI bridge.

## Done

### Foundation

- Read the original project blueprint and converted it into structured user and developer docs.
- Created a clean Rust workspace and Python package directory.
- Configured uv, maturin, PyO3, and Cargo for a mixed Python/Rust project.
- Added a shared Rust core crate for widget-tree summary behavior.
- Added Python package metadata, type marker, examples, tests, and root documentation.

### Desktop Prototype

- Built the first native Windows hello-window prototype.
- Built a native Windows callback prototype where a Win32 button calls Python while the window is open.
- Added native rendering for `Window`, `Column`, `Row`, `Text`, `Button`, and `Input`.
- Added Python `State` support for counters, text values, and simple forms.
- Added button event handling and tree re-rendering.
- Fixed the input crash that occurred when typing into the login form.

### CLI And Examples

- Added `pynative doctor`.
- Added `pynative new my_app` project generation.
- Added `pynative run desktop` with app-file and project-folder support.
- Added `--summary` mode for non-GUI smoke checks.
- Added counter, login, todo, calculator, contact form, gallery, settings panel, quiz, and dashboard examples.
- Added tests that load every example and export Android screen specs.
- Added a first-pass design/style API with Windows and Android rendering support.

### Android Experiment

- Added a minimal Android experiment project.
- Added a PowerShell build helper using the installed Android SDK and Android Studio JBR.
- Added `pynative run android --build-only`.
- Added `pynative run android <target> --build-only` to export a PyNative app into the Android shell.
- Added `pynative build apk`.
- Added install/launch flow for connected devices or emulators.
- Added generated Android screen metadata and native button state handling.
- Added a Rust JNI bridge crate for Android.
- Packaged `libpynative_android_bridge.so` into Android APK builds.
- Forwarded Android button taps into Rust and reported native event counts.
- Documented current Android requirements and limitations.

### Repository

- Initialized Git, committed the first scaffold, added the GitHub remote, and pushed `main`.

## Pending

### Phase 1 Desktop MVP

- Improve renderer layout calculations and control reuse.
- Preserve focus and cursor position more robustly during updates.
- Add native image rendering.
- Add window sizing, title, and layout options from app/project configuration.
- Add better error messages for app loading and callback failures.
- Add more tests around widget serialization, CLI execution, and state changes.

### Phase 2 Android MVP

- Replace the generated Java experiment with a proper Android runtime shell.
- Decide what Android event/runtime logic belongs in Rust versus Python now that Android can call Rust through JNI.
- Choose and document the Python-on-Android runtime approach.
- Move from build-time widget export to a live Python widget tree on Android.
- Render basic `Text`, `Button`, and `Input` controls on Android.
- Add device/emulator verification steps.

### Packaging

- Add desktop packaging commands.
- Define release artifact names and output folders.
- Add dependency compatibility checks for Python packages.
- Add docs for pure-Python dependencies and packages with native extensions.

### Quality

- Add CI for `cargo fmt`, `cargo check`, `cargo test`, `pytest`, and docs link checks.
- Add a compatibility matrix for Windows, Python, Rust, uv, maturin, Android SDK, and NDK versions.
- Add issue templates or a simple contribution guide before external collaboration grows.

## Recommended Next Build Order

1. Finish Phase 1 desktop polish enough for a reliable counter and login demo.
2. Add image rendering and basic configuration loading.
3. Add tests around event/state behavior.
4. Decide and prototype the Python runtime strategy on Android.
5. Move Android from build-time widget export to live runtime state updates.
6. Add packaging and CI once the desktop and Android paths are stable enough to repeat.

## Active Risks

- Android Python packaging can become the hardest part of the project if native dependencies are allowed too early.
- Re-rendering native desktop controls is simple but can cause focus and input bugs unless widget identity and control reuse are improved.
- A custom renderer may be needed later for advanced UI, but native controls are faster for the current MVP.
