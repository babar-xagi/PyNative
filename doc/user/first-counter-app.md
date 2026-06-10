# First Counter App

The first PyNative UI app uses a declarative widget tree and simple state.

```python
from pynative import App, Button, Column, State, Text, Window

count = State(0)

def increment() -> None:
    count.set(count.value + 1)

app = App(
    Window(
        title="Counter App",
        child=Column([
            Text(lambda: f"Count: {count.value}"),
            Button("Increase", on_click=increment),
        ]),
    )
)

app.run()
```

## Run

```powershell
uv run pynative run desktop
```

During Phase 1, this opens a native Windows window. The `Increase` button calls Python, updates `State`, and refreshes the displayed count.

The same event path is used by `Input` widgets: native text changes update Python `State`, and action events such as button clicks can then render UI that reads the latest input state.
