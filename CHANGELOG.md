# Changelog

All notable project changes are tracked here so each phase has a clear history.

## Unreleased

### Planned

- Improve the Windows renderer layout engine, focus handling, sizing rules, and widget reuse.
- Add native rendering for the `Image` widget.
- Load project settings from `pynative.toml` during CLI runs.
- Add a Windows package/build command for distributable desktop apps.
- Expand the Android experiment into a real Android runtime shell with a Rust JNI bridge.
- Decide the Android Python runtime strategy and document package compatibility limits.
- Add continuous integration for Rust, Python, formatting, tests, and docs checks.

### Pending Decisions

- Android runtime approach: embedded CPython, Chaquopy-style integration, or another packaged runtime.
- Rendering strategy for advanced widgets: native platform controls first, custom renderer later where needed.
- Plugin/package policy for Python dependencies with native wheels.
- Release packaging format and naming for early preview builds.

### Added

- Added Android app-target builds so `pynative run android examples\counter\app.py --build-only` exports a PyNative widget tree into the Android experiment.
- Added `pynative build apk` for the Phase 2 Android APK build flow.
- Added generated Android screen metadata for text, buttons, inputs, image placeholders, node count, and source path.

## 0.1.0 - 2026-06-10

### Added

- Created the initial Rust workspace, Python package layout, uv workflow, maturin build setup, and PyO3 native extension.
- Added the Python-first declarative API: `App`, `Window`, `Column`, `Row`, `Text`, `Button`, `Input`, `Image`, and `State`.
- Added shared Rust widget-tree summary logic in `pynative_core`.
- Added native Windows prototypes for a hello window and a native-to-Python callback window.
- Added the first Windows desktop renderer for basic widget trees.
- Added Python state updates, button callbacks, and input change handling for native desktop apps.
- Added CLI commands for `doctor`, `new`, `run desktop`, `run android`, `hello-window`, and `callback-window`.
- Added counter and login examples.
- Added project generator output for new PyNative apps.
- Added the first Android APK experiment using Android SDK tools without Gradle.
- Added Android user/developer documentation and a helper build script.
- Added user and developer docs organized under `doc/user` and `doc/developer`.
- Added focused Rust and Python tests for the current scaffold and CLI behavior.
- Published the initial repository to GitHub on the `main` branch.

### Changed

- Updated input handling so native edit controls update Python `State` without forcing a destructive re-render on every keystroke.
- Organized the project plan into phase-by-phase Markdown documentation.

### Fixed

- Fixed the Windows login/input crash caused by nested window destroy messages during renderer state mutation.

### Known Limitations

- The Android build is an experiment only; it opens a Java Activity and does not embed Python or Rust yet.
- The Windows renderer supports only a small widget set and simple layout behavior.
- `Image` exists in the Python API but is not rendered natively yet.
- Generated app projects are local templates and are not packaged as standalone distributions yet.
- Android install/launch requires a connected device or running emulator.
