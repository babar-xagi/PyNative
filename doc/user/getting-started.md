# Getting Started

## Requirements

- Python 3.12 or newer.
- Rust and Cargo.
- uv.
- Android tools later, when Android MVP work starts.

## Install For Local Development

From the project root:

```powershell
uv run maturin develop
```

This builds the Rust PyO3 extension and installs the local Python package in the uv-managed environment.

## Check Your Setup

```powershell
uv run pynative doctor
```

The `doctor` command reports Python version, platform details, and whether the native extension is available.

## Run The Counter Example

```powershell
uv run python examples/counter/app.py
```

In Phase 1 this opens a native Windows window. Click `Increase` to send a native button event back into Python, update `State`, and refresh the visible counter text.

For a non-GUI check:

```powershell
uv run python -c "from examples.counter.app import app; app.run('summary')"
```

## Create A New App

```powershell
uv run pynative new my_app
uv run pynative run desktop my_app
```

`pynative run desktop my_app` loads `my_app/app.py` and opens the native Windows renderer.

Run without opening a window:

```powershell
uv run pynative run desktop my_app --summary
```

## Run The Login Example

```powershell
uv run pynative run desktop examples/login/app.py
```

This example uses native `Input` controls bound to Python `State`. Typing updates the state, and the `Sign in` button reads it.

## Build The Android Experiment

```powershell
uv run pynative run android --build-only
```

To install and launch, connect a device or start an emulator, then run:

```powershell
uv run pynative run android
```

## Run The Windows Hello Window

On Windows, after building with maturin:

```powershell
uv run pynative hello-window
```

Click the native button, then close the window. The command prints the number of button clicks after the window closes.

To test a live Python callback from the native button:

```powershell
uv run pynative callback-window
```

Each click prints a Python callback message immediately in the terminal.
