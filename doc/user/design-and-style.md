# Design And Style

PyNative widgets accept an optional `style` argument. Use `Style(...)` for a typed style object, or pass a plain dictionary when that is more convenient.

## Supported Style Fields

| Field | Purpose | Example |
| --- | --- | --- |
| `color` | Text color | `"#0F172A"` |
| `background_color` | Background color | `"#F8FAFC"` |
| `font_size` | Text size | `18` |
| `font_weight` | Text weight | `"bold"` |
| `width` | Control width | `280` |
| `height` | Control height | `40` |
| `padding` | Inner spacing | `12` |
| `margin` | Outer spacing | `8` |
| `gap` | Spacing between row/column children | `14` |
| `align` | Horizontal alignment | `"start"`, `"center"`, or `"end"` |

Colors should use hex strings like `#2563EB`.

## Example

```python
from pynative import App, Button, Column, State, Style, Text, Window

count = State(0)


def increment() -> None:
    count.set(count.value + 1)


app = App(
    Window(
        title="Styled Counter",
        style=Style(background_color="#F8FAFC", padding=28),
        child=Column(
            [
                Text(
                    lambda: f"Count: {count.value}",
                    style=Style(
                        color="#0F172A",
                        font_size=24,
                        font_weight="bold",
                        align="center",
                    ),
                ),
                Button(
                    "Increase",
                    on_click=increment,
                    style=Style(width=180, height=40),
                ),
            ],
            style=Style(gap=16),
        ),
    )
)
```

## Run The Showcase

```powershell
uv run pynative run desktop examples\design_showcase\app.py
uv run pynative build apk examples\design_showcase\app.py
```

## Current Limits

- Windows supports the core design fields, but classic native buttons have limited custom background rendering.
- Android currently renders a build-time export of the widget tree and style data.
- Images still render as placeholders until native image rendering is added.
