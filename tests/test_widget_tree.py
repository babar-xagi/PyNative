from pynative import App, Button, Column, Input, State, Style, Text, Window


def test_app_serializes_widget_tree():
    count = State(0)
    app = App(
        Window(
            title="Counter",
            child=Column(
                [
                    Text(lambda: f"Count: {count.value}"),
                    Button("Increase"),
                ]
            ),
        )
    )

    tree = app.to_dict()

    assert tree["kind"] == "App"
    assert tree["children"][0]["kind"] == "Window"
    assert tree["children"][0]["children"][0]["kind"] == "Column"


def test_state_notifies_subscribers():
    count = State(0)
    seen = []

    count.subscribe(seen.append)
    count.set(1)

    assert seen == [1]


def test_widgets_serialize_style_props():
    app = App(
        Window(
            title="Styled",
            style=Style(background_color="#F8FAFC", padding=24),
            child=Column(
                [
                    Text(
                        "Hello",
                        style=Style(
                            color="#0F172A",
                            font_size=22,
                            font_weight="bold",
                        ),
                    ),
                    Button(
                        "Continue",
                        style={
                            "background_color": "#2563EB",
                            "color": "#FFFFFF",
                            "width": 180,
                        },
                    ),
                ],
                style=Style(gap=16),
            ),
        )
    )

    tree = app.to_dict()
    window = tree["children"][0]
    column = window["children"][0]
    text = column["children"][0]
    button = column["children"][1]

    assert window["props"]["style"]["background_color"] == "#F8FAFC"
    assert window["props"]["style"]["padding"] == 24
    assert column["props"]["style"]["gap"] == 16
    assert text["props"]["style"]["font_weight"] == "bold"
    assert button["props"]["style"]["width"] == 180


def test_app_dispatches_registered_button_callback():
    count = State(0)

    def increment():
        count.set(count.value + 1)

    app = App(
        Window(
            child=Column(
                [
                    Text(lambda: f"Count: {count.value}"),
                    Button("Increase", on_click=increment),
                ]
            )
        )
    )

    callbacks = app._callback_registry()
    callback_id = id(increment)
    updated_tree_json = app._dispatch_native_event(
        {
            "kind": "button_click",
            "callback_id": callback_id,
            "value": None,
        },
        callbacks,
        {},
    )

    assert count.value == 1
    assert updated_tree_json is not None
    assert "Count: 1" in updated_tree_json


def test_app_dispatches_input_change_to_state():
    name = State("")
    app = App(
        Window(
            child=Column(
                [
                    Input(value=name, placeholder="Name"),
                    Text(lambda: f"Hello {name.value}"),
                ]
            )
        )
    )

    states = app._state_registry()
    updated_tree_json = app._dispatch_native_event(
        {
            "kind": "input_change",
            "callback_id": id(name),
            "value": "Babar",
        },
        {},
        states,
    )

    assert name.value == "Babar"
    assert updated_tree_json is not None
    assert "Hello Babar" in updated_tree_json
