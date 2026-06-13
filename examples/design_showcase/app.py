from pynative import App, Button, Column, Input, Row, State, Style, Text, Window

name = State("")
accent = State("#2563EB")
message = State("Design system ready.")


def use_blue() -> None:
    accent.set("#2563EB")
    message.set("Blue accent selected.")


def use_teal() -> None:
    accent.set("#0F766E")
    message.set("Teal accent selected.")


def greet() -> None:
    clean_name = name.value.strip()
    if clean_name:
        message.set(f"Hello, {clean_name}.")
    else:
        message.set("Type a name first.")


app = App(
    Window(
        title="Design Showcase",
        style=Style(background_color="#F8FAFC", padding=28),
        child=Column(
            [
                Text(
                    "Design Showcase",
                    style=Style(
                        color="#0F172A",
                        font_size=26,
                        font_weight="bold",
                        align="center",
                    ),
                ),
                Text(
                    "Text color, background color, font size, spacing, sizing, and alignment.",
                    style=Style(color="#475569", font_size=16, align="center"),
                ),
                Text(
                    lambda: message.value,
                    style=Style(
                        color="#FFFFFF",
                        background_color=accent.value,
                        font_size=18,
                        padding=12,
                        margin=4,
                        align="center",
                    ),
                ),
                Input(
                    value=name,
                    placeholder="Your name",
                    style=Style(width=280, height=36, font_size=16, align="center"),
                ),
                Row(
                    [
                        Button(
                            "Greet",
                            on_click=greet,
                            style=Style(width=140, height=40, font_weight="bold"),
                        ),
                        Button(
                            "Blue",
                            on_click=use_blue,
                            style=Style(width=120, height=40),
                        ),
                        Button(
                            "Teal",
                            on_click=use_teal,
                            style=Style(width=120, height=40),
                        ),
                    ],
                    style=Style(gap=10, align="center"),
                ),
                Text(
                    "This example is the visual smoke test for the design API.",
                    style=Style(color="#64748B", font_size=14),
                ),
            ],
            style=Style(gap=14),
        ),
    )
)


if __name__ == "__main__":
    app.run()
