from pynative import App, Button, Column, Input, Row, State, Text, Window

name = State("")
email = State("")
message = State("")
status = State("Ready to send.")


def send_message() -> None:
    clean_name = name.value.strip()
    clean_email = email.value.strip()
    clean_message = message.value.strip()

    if not clean_name:
        status.set("Name is required.")
    elif "@" not in clean_email:
        status.set("Enter a valid email address.")
    elif not clean_message:
        status.set("Message is required.")
    else:
        status.set(f"Message queued for {clean_name}.")


def reset_form() -> None:
    name.set("")
    email.set("")
    message.set("")
    status.set("Form reset.")


app = App(
    Window(
        title="Contact Form",
        child=Column(
            [
                Text("Contact Form"),
                Input(value=name, placeholder="Name"),
                Input(value=email, placeholder="Email"),
                Input(value=message, placeholder="Message"),
                Row(
                    [
                        Button("Send", on_click=send_message),
                        Button("Reset", on_click=reset_form),
                    ]
                ),
                Text(lambda: status.value),
            ]
        ),
    )
)


if __name__ == "__main__":
    app.run()
