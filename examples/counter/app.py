from pynative import App, Button, Column, State, Text, Window

count = State(0)


def increment() -> None:
    count.set(count.value + 1)


app = App(
    Window(
        title="Counter App",
        child=Column(
            [
                Text(lambda: f"Count: {count.value}"),
                Button("Increase", on_click=increment),
            ]
        ),
    )
)


if __name__ == "__main__":
    app.run()
