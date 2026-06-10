# PyNative UI

PyNative UI is a Python-first declarative UI framework powered by Rust. The first build target is a Phase 0 prototype: prove the Rust/Python bridge, define the widget tree model, and prepare the project for Windows desktop and Android work.

## Current Status

Phase 0 scaffold is in progress.

- Python package source lives in `python/pynative`.
- PyO3 bridge lives in `src`.
- Shared Rust runtime logic lives in `crates/pynative_core`.
- Future Rust CLI code lives in `crates/pynative_cli`.
- Example apps live in `examples`.
- Documentation lives in `doc/user` and `doc/developer`.

## Tooling

This project is configured for:

- Rust and Cargo.
- Python.
- uv.
- PyO3.
- maturin.

`maturin` does not need to be installed globally. Use it through uv:

```powershell
uv run maturin develop
```

## First Local Checks

```powershell
cargo check --workspace
python -c "import sys; sys.path.insert(0, 'python'); import pynative; print(pynative.__version__)"
```

After the native module is built with maturin:

```powershell
uv run maturin develop
uv run pynative doctor
python examples/counter/app.py
```

## Phase 1 Desktop Renderer

Create a new app:

```powershell
uv run pynative new my_app
uv run pynative run desktop my_app
```

Run the counter example as a native Windows app:

```powershell
uv run python examples\counter\app.py
```

Click `Increase` and the native button event calls Python, updates `State`, and re-renders the visible `Text` widget.

Run the login form example:

```powershell
uv run pynative run desktop examples\login\app.py
```

Typing into native `Input` controls updates Python `State`; clicking `Sign in` reads that state and refreshes the message text.

For a non-GUI smoke check:

```powershell
uv run python -c "from examples.counter.app import app; app.run('summary')"
uv run pynative run desktop examples\counter\app.py --summary
```

## Phase 0.1 Native Window Prototype

After running `uv run maturin develop`, open the first native Windows prototype:

```powershell
uv run pynative hello-window
```

Click the button a few times, then close the window. The command prints a JSON result with the native click count.

To prove a live native-to-Python callback:

```powershell
uv run pynative callback-window
```

Each button click prints from Python immediately while the native window is still open.

## Android Experiment

Build the first Android APK experiment:

```powershell
uv run pynative run android --build-only
```

Connect a device or start an emulator, then install and launch:

```powershell
uv run pynative run android
```

The APK is written to:

```text
build\android-experiment\pynative-android-debug.apk
```
