from pynative.cli import callback_window_command, doctor, hello_window_command
from pynative.android import android_environment, android_spec_from_app
from pynative.project import create_project, load_app, normalize_project_name, run_desktop_app


def test_doctor_reports_native_status():
    result = doctor()

    assert "python" in result
    assert "platform" in result
    assert "native_extension" in result
    assert "android" in result


def test_window_commands_are_importable():
    assert callable(hello_window_command)
    assert callable(callback_window_command)


def test_create_project_writes_expected_files(tmp_path):
    created = create_project("My App", base_dir=tmp_path)

    assert created.name == "My-App"
    assert (created.path / "app.py").exists()
    assert (created.path / "pynative.toml").exists()
    assert (created.path / "assets").is_dir()
    assert (created.path / "screens").is_dir()
    assert (created.path / "services").is_dir()


def test_load_generated_app(tmp_path):
    created = create_project("demo", base_dir=tmp_path)
    app = load_app(created.path)

    assert app.summarize()["root"] == "App"


def test_run_generated_app_summary(tmp_path, capsys):
    created = create_project("demo", base_dir=tmp_path)

    run_desktop_app(created.path, summary=True)

    output = capsys.readouterr().out
    assert '"root": "App"' in output


def test_normalize_project_name_rejects_empty_name():
    try:
        normalize_project_name("!!!")
    except ValueError as exc:
        assert "cannot be empty" in str(exc)
    else:
        raise AssertionError("Expected ValueError")


def test_android_environment_reports_build_script():
    env = android_environment()

    assert env.build_script.name == "build_experiment.ps1"


def test_android_spec_collects_widgets(tmp_path):
    created = create_project("android-demo", base_dir=tmp_path)
    app = load_app(created.path)

    spec = android_spec_from_app(app, source_path=created.path / "app.py")

    assert spec["title"] == "Android Demo"
    assert spec["texts"] == ["Count: 0"]
    assert spec["buttons"] == ["Increase"]
    assert spec["inputs"] == []
    assert spec["has_python_callbacks"] is True
    assert spec["node_count"] > 0
    assert spec["elements"][0]["kind"] == "Text"
    assert spec["elements"][1]["kind"] == "Button"


def test_android_spec_includes_style_metadata():
    from pynative import App, Column, Style, Text, Window

    app = App(
        Window(
            title="Styled Android",
            style=Style(background_color="#F8FAFC", padding=24),
            child=Column(
                [
                    Text(
                        "Styled",
                        style=Style(color="#2563EB", font_size=22, align="center"),
                    )
                ]
            ),
        )
    )

    spec = android_spec_from_app(app)

    assert spec["root_style"]["background_color"] == "#F8FAFC"
    assert spec["root_style"]["padding"] == 24
    assert spec["elements"][0]["style"]["color"] == "#2563EB"
    assert spec["elements"][0]["style"]["font_size"] == 22
