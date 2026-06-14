# PyNative UI

PyNative UI is a Python-first declarative UI framework powered by Rust. The project is currently building the Phase 1 desktop MVP while exploring the Phase 2 Android path.

## Current Status

Phase 1 desktop MVP is active, and the first Android build experiment is available.

- Python package source lives in `python/pynative`.
- PyO3 bridge lives in `src`.
- Shared Rust runtime logic lives in `crates/pynative_core`.
- Rust CLI code lives in `crates/pynative_cli`.
- Example apps live in `examples`.
- Documentation lives in `doc/user` and `doc/developer`.
- Project history lives in [CHANGELOG.md](CHANGELOG.md).
- Current build status lives in [doc/developer/project-status.md](doc/developer/project-status.md).
- Example app catalog lives in [examples/README.md](examples/README.md).

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

## Example Apps

The `examples` folder includes counter, login, todo, calculator, contact form, design showcase, gallery, settings, quiz, and dashboard demos.

Run any example on desktop:

```powershell
uv run pynative run desktop examples\todo\app.py
```

Build any example as an Android APK:

```powershell
uv run pynative build apk examples\todo\app.py
```

Run the design showcase:

```powershell
uv run pynative run desktop examples\design_showcase\app.py
uv run pynative build apk examples\design_showcase\app.py
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

Build an APK from a PyNative app target:

```powershell
uv run pynative run android examples\counter\app.py --build-only
uv run pynative build apk examples\counter\app.py
```

Android builds now package a Rust JNI bridge. The default ABI is `arm64-v8a` for real phones. For an x86_64 emulator:

```powershell
uv run pynative build apk examples\counter\app.py --android-abi x86_64
```

The APK also includes early runtime assets:

```text
assets/pynative/app.py
assets/pynative/widget_tree.json
assets/pynative/runtime.json
```

On app launch, Android initializes a Rust runtime session from those assets. Button events return JSON responses with native event count and an updated widget-tree preview.
Runtime assets also include stable IDs like `node:/0/0/1` and `event:/0/0/1`, so Android events no longer depend on Python memory addresses from build time.

Connect a device or start an emulator, then install and launch:

```powershell
uv run pynative run android examples\counter\app.py
```

The APK is written to:

```text
build\android-experiment\pynative-android-debug.apk
```
