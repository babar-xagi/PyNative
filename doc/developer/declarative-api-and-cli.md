# Declarative API and CLI

## API Direction

The developer API should be simple, tree-based, and close to Flutter concepts while staying natural in Python.

```python
from pynative import App, Window, Column, Text, Button, State

count = State(0)

def increment():
    count.set(count.value + 1)

App(
    Window(
        title="Counter App",
        child=Column([
            Text(lambda: f"Count: {count.value}"),
            Button("Increase", on_click=increment),
        ])
    )
).run()
```

## MVP Widgets

| Widget | Purpose |
| --- | --- |
| `App` | Root runtime object |
| `Window` / `Screen` | Top-level app container |
| `Column` / `Row` | Basic layouts |
| `Text` | Display text |
| `Button` | Actions and events |
| `Input` | Text entry |
| `Image` | Display image/assets |
| `ListView` | Scrollable list |
| `Table` | Data display for AI/data/business apps |
| `Progress` | Long-running task and AI job status |

## State Model

```python
from pynative import Window, Column, Input, Text, State

name = State("")

Window(
    child=Column([
        Input(value=name, placeholder="Enter name"),
        Text(lambda: f"Hello {name.value}"),
    ])
)
```

State must be easy. Python callbacks update state, and Rust re-renders only the affected UI tree when possible.

## CLI Commands

```text
pynative new my_app
cd my_app
pynative run desktop
pynative run android
pynative build windows
pynative build apk
```

## Project Structure

```text
my_app/
|-- app.py
|-- pynative.toml
|-- requirements.txt
|-- assets/
|-- screens/
|   |-- home.py
|   |-- settings.py
|-- services/
|   |-- api.py
|   |-- ai.py
|-- backend/
|   |-- server.py
|-- build/
```

## Example Configuration

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
