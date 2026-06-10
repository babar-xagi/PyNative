# Project Configuration

PyNative UI apps will use `pynative.toml` for app metadata, Python mode, platform targets, and backend mode.

```toml
[app]
name = "My AI App"
package = "com.babar.myaiapp"
version = "0.1.0"

[python]
mode = "on_device"
version = "3.12"
requirements = "requirements.txt"

[platforms]
windows = true
android = true
linux = false
macos = false
ios = false

[backend]
enabled = false
mode = "remote"
```

## Planned Python Modes

- `on_device`: run supported Python code inside the app.
- `remote`: call a Python backend service.
- `hybrid`: use on-device Python for light work and backend mode for heavy AI/data jobs.
