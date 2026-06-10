from pynative import App, Button, Column, Input, Row, State, Text, Window

draft = State("")
tasks = State(["Review PyNative roadmap", "Build Android APK"])
status = State("2 tasks ready.")


def add_task() -> None:
    task = draft.value.strip()
    if not task:
        status.set("Type a task before adding it.")
        return

    updated = [*tasks.value, task]
    tasks.set(updated)
    draft.set("")
    status.set(f"Added task. Total: {len(updated)}.")


def complete_first() -> None:
    if not tasks.value:
        status.set("No tasks to complete.")
        return

    completed = tasks.value[0]
    updated = tasks.value[1:]
    tasks.set(updated)
    status.set(f"Completed: {completed}")


def clear_tasks() -> None:
    tasks.set([])
    status.set("All tasks cleared.")


def task_list() -> str:
    if not tasks.value:
        return "No tasks yet."
    return " | ".join(f"{index + 1}. {task}" for index, task in enumerate(tasks.value))


app = App(
    Window(
        title="Todo List",
        child=Column(
            [
                Text("Todo List"),
                Input(value=draft, placeholder="New task"),
                Row(
                    [
                        Button("Add", on_click=add_task),
                        Button("Complete First", on_click=complete_first),
                        Button("Clear", on_click=clear_tasks),
                    ]
                ),
                Text(lambda: status.value),
                Text(task_list),
            ]
        ),
    )
)


if __name__ == "__main__":
    app.run()
