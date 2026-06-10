from pynative import App, Button, Column, Image, Row, State, Text, Window

photos = [
    ("assets/mountain.png", "Mountain sunrise"),
    ("assets/city.png", "City evening"),
    ("assets/workspace.png", "Developer workspace"),
]
index = State(0)


def next_photo() -> None:
    index.set((index.value + 1) % len(photos))


def previous_photo() -> None:
    index.set((index.value - 1) % len(photos))


def caption() -> str:
    return f"{index.value + 1}/{len(photos)}: {photos[index.value][1]}"


app = App(
    Window(
        title="Gallery",
        child=Column(
            [
                Text("Gallery"),
                Image(src=photos[0][0], alt=photos[0][1]),
                Text(caption),
                Row(
                    [
                        Button("Previous", on_click=previous_photo),
                        Button("Next", on_click=next_photo),
                    ]
                ),
            ]
        ),
    )
)


if __name__ == "__main__":
    app.run()
