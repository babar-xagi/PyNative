from pynative import App, Button, Column, Input, State, Text, Window

username = State("")
password = State("")
message = State("Enter your details.")


def sign_in() -> None:
    if username.value.strip() and password.value.strip():
        message.set(f"Welcome, {username.value.strip()}!")
    else:
        message.set("Username and password are required.")


app = App(
    Window(
        title="Login Form",
        child=Column(
            [
                Text("Login"),
                Input(value=username, placeholder="Username"),
                Input(value=password, placeholder="Password"),
                Button("Sign in", on_click=sign_in),
                Text(lambda: message.value),
            ]
        ),
    )
)


if __name__ == "__main__":
    app.run()
