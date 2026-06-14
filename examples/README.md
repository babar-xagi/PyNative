# Example Apps

These examples are small PyNative apps used to test the current desktop and Android paths.

## Run On Windows Desktop

Use `--summary` for automated smoke checks:

```powershell
uv run pynative run desktop examples\counter\app.py --summary
```

Open a native Windows window interactively:

```powershell
uv run pynative run desktop examples\counter\app.py
```

## Build For Android

Build an APK from any example:

```powershell
uv run pynative build apk examples\counter\app.py
```

The default ABI is `arm64-v8a` for real phones. For an x86_64 emulator:

```powershell
uv run pynative build apk examples\counter\app.py --android-abi x86_64
```

Install and launch when a device or emulator is connected:

```powershell
uv run pynative run android examples\counter\app.py
```

The APK packages the selected example under `assets/pynative` so the next Android runtime phase can load it on-device.

## Available Examples

- `counter`: basic state and button callback.
- `login`: input fields and form validation.
- `todo`: task entry, list state, and clearing.
- `calculator`: keypad layout and arithmetic state.
- `contact_form`: form validation and reset flow.
- `design_showcase`: colors, background, typography, spacing, sizing, and alignment.
- `gallery`: image placeholder, caption state, and navigation.
- `settings_panel`: toggle-style settings with state.
- `quiz`: question flow, answer feedback, and score.
- `dashboard`: dashboard-style metrics and refresh actions.
