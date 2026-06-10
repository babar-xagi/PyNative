from pynative import App, Button, Column, Row, State, Text, Window

display = State("0")
stored_value = State(None)
operator = State("")


def enter_digit(digit: str) -> None:
    if display.value == "0":
        display.set(digit)
    else:
        display.set(display.value + digit)


def set_operator(next_operator: str) -> None:
    stored_value.set(float(display.value))
    operator.set(next_operator)
    display.set("0")


def clear() -> None:
    display.set("0")
    stored_value.set(None)
    operator.set("")


def equals() -> None:
    if stored_value.value is None or not operator.value:
        return

    current = float(display.value)
    left = float(stored_value.value)

    if operator.value == "+":
        result = left + current
    elif operator.value == "-":
        result = left - current
    elif operator.value == "*":
        result = left * current
    elif current == 0:
        display.set("Cannot divide by zero")
        stored_value.set(None)
        operator.set("")
        return
    else:
        result = left / current

    if result.is_integer():
        display.set(str(int(result)))
    else:
        display.set(str(result))
    stored_value.set(None)
    operator.set("")


app = App(
    Window(
        title="Calculator",
        child=Column(
            [
                Text(lambda: f"Display: {display.value}"),
                Row(
                    [
                        Button("7", on_click=lambda: enter_digit("7")),
                        Button("8", on_click=lambda: enter_digit("8")),
                        Button("9", on_click=lambda: enter_digit("9")),
                        Button("/", on_click=lambda: set_operator("/")),
                    ]
                ),
                Row(
                    [
                        Button("4", on_click=lambda: enter_digit("4")),
                        Button("5", on_click=lambda: enter_digit("5")),
                        Button("6", on_click=lambda: enter_digit("6")),
                        Button("*", on_click=lambda: set_operator("*")),
                    ]
                ),
                Row(
                    [
                        Button("1", on_click=lambda: enter_digit("1")),
                        Button("2", on_click=lambda: enter_digit("2")),
                        Button("3", on_click=lambda: enter_digit("3")),
                        Button("-", on_click=lambda: set_operator("-")),
                    ]
                ),
                Row(
                    [
                        Button("0", on_click=lambda: enter_digit("0")),
                        Button("C", on_click=clear),
                        Button("=", on_click=equals),
                        Button("+", on_click=lambda: set_operator("+")),
                    ]
                ),
            ]
        ),
    )
)


if __name__ == "__main__":
    app.run()
