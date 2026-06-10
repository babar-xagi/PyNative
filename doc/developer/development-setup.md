# Development Setup

## Required Tools

- Rust and Cargo.
- Python 3.12 or newer.
- uv.
- maturin through uv.

## Verify Toolchain

```powershell
rustc --version
cargo --version
python --version
uv --version
```

## Build Native Extension

```powershell
uv run maturin develop
```

The project uses maturin with:

- Python source in `python`.
- PyO3 module name `pynative._native`.
- Rust bridge in the root `src` folder.
- Core runtime crate in `crates/pynative_core`.

## Rust Checks

```powershell
cargo fmt --all
cargo check --workspace
cargo test --workspace
```

## Python Checks

```powershell
uv run pytest
uv run pynative doctor
uv run python -c "from examples.counter.app import app; app.run('summary')"
uv run pynative run desktop examples\counter\app.py --summary
uv run pynative run desktop examples\login\app.py --summary
```

## Phase 0.1 Windows Prototype

```powershell
uv run maturin develop
uv run pynative hello-window
uv run pynative callback-window
```

`hello-window` opens a native Win32 window and returns JSON after close. `callback-window` proves that the native button event can call a Python function immediately.

## Phase 0 Build Target

Phase 0 does not need a native window yet. It must prove:

- Python can define a widget tree.
- Rust can parse and summarize that tree.
- PyO3 can expose native functions to Python.
- A native Windows button can call into Python.
- The repository has a clean foundation for Phase 1 desktop MVP work.

## Phase 1 Desktop Renderer Check

```powershell
uv run maturin develop
uv run python examples\counter\app.py
uv run pynative run desktop examples\login\app.py
uv run pynative new sample_app
uv run pynative run desktop sample_app --summary
```

This opens the first widget-tree-rendered Windows apps. Button clicks route native events to Python and re-render the window. Input changes update Python `State` without rebuilding the focused edit control on every keystroke.

## Phase 2 Android Experiment Check

```powershell
uv run pynative run android --build-only
```

This uses the installed Android SDK directly, without Gradle, to build:

```text
build\android-experiment\pynative-android-debug.apk
```

With a connected device or running emulator:

```powershell
uv run pynative run android
```
