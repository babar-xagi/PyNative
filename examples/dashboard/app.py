from pynative import App, Button, Column, Row, State, Text, Window

sales = State(12840)
tickets = State(18)
uptime = State("99.95%")
status = State("Dashboard loaded.")


def refresh_metrics() -> None:
    sales.set(sales.value + 420)
    tickets.set(max(0, tickets.value - 1))
    status.set("Metrics refreshed.")


def simulate_alert() -> None:
    tickets.set(tickets.value + 3)
    uptime.set("99.70%")
    status.set("Alert simulation added support load.")


app = App(
    Window(
        title="Operations Dashboard",
        child=Column(
            [
                Text("Operations Dashboard"),
                Row(
                    [
                        Text(lambda: f"Sales: ${sales.value}"),
                        Text(lambda: f"Tickets: {tickets.value}"),
                        Text(lambda: f"Uptime: {uptime.value}"),
                    ]
                ),
                Row(
                    [
                        Button("Refresh", on_click=refresh_metrics),
                        Button("Simulate Alert", on_click=simulate_alert),
                    ]
                ),
                Text(lambda: status.value),
            ]
        ),
    )
)


if __name__ == "__main__":
    app.run()
