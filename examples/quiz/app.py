from pynative import App, Button, Column, Row, State, Text, Window

questions = [
    ("Which language powers the PyNative native core?", ["Rust", "Ruby", "Go"], 0),
    ("Which tool builds the Python extension?", ["maturin", "pipx", "poetry"], 0),
    ("Which command builds an Android APK?", ["build apk", "ship apk", "make phone"], 0),
]

question_index = State(0)
score = State(0)
feedback = State("Choose an answer.")


def current_question() -> tuple[str, list[str], int]:
    return questions[question_index.value]


def answer(choice: int) -> None:
    _, _, correct = current_question()
    if choice == correct:
        score.set(score.value + 1)
        feedback.set("Correct.")
    else:
        feedback.set("Not quite.")


def next_question() -> None:
    question_index.set((question_index.value + 1) % len(questions))
    feedback.set("Choose an answer.")


app = App(
    Window(
        title="Quiz",
        child=Column(
            [
                Text(lambda: f"Question {question_index.value + 1}: {current_question()[0]}"),
                Row(
                    [
                        Button("A", on_click=lambda: answer(0)),
                        Button("B", on_click=lambda: answer(1)),
                        Button("C", on_click=lambda: answer(2)),
                    ]
                ),
                Text(lambda: " | ".join(current_question()[1])),
                Text(lambda: f"Score: {score.value}"),
                Text(lambda: feedback.value),
                Button("Next", on_click=next_question),
            ]
        ),
    )
)


if __name__ == "__main__":
    app.run()
