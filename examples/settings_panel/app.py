from pynative import App, Button, Column, Row, State, Text, Window

theme = State("Light")
notifications = State("On")
density = State("Comfortable")


def toggle_theme() -> None:
    theme.set("Dark" if theme.value == "Light" else "Light")


def toggle_notifications() -> None:
    notifications.set("Off" if notifications.value == "On" else "On")


def toggle_density() -> None:
    density.set("Compact" if density.value == "Comfortable" else "Comfortable")


app = App(
    Window(
        title="Settings Panel",
        child=Column(
            [
                Text("Settings Panel"),
                Text(lambda: f"Theme: {theme.value}"),
                Text(lambda: f"Notifications: {notifications.value}"),
                Text(lambda: f"Density: {density.value}"),
                Row(
                    [
                        Button("Theme", on_click=toggle_theme),
                        Button("Notifications", on_click=toggle_notifications),
                        Button("Density", on_click=toggle_density),
                    ]
                ),
            ]
        ),
    )
)


if __name__ == "__main__":
    app.run()
