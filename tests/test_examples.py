from pathlib import Path

from pynative.android import android_spec_from_app
from pynative.project import load_app


EXAMPLE_APPS = sorted(Path("examples").glob("*/app.py"))


def test_all_examples_load_and_summarize():
    assert EXAMPLE_APPS

    for app_path in EXAMPLE_APPS:
        app = load_app(app_path)
        summary = app.summarize()

        assert summary["root"] == "App", app_path
        assert summary["node_count"] >= 4, app_path
        assert summary["max_depth"] >= 3, app_path


def test_all_examples_export_android_specs():
    for app_path in EXAMPLE_APPS:
        app = load_app(app_path)
        spec = android_spec_from_app(app, source_path=app_path)

        assert spec["title"], app_path
        assert spec["node_count"] >= 4, app_path
        assert spec["max_depth"] >= 3, app_path
        assert (
            spec["texts"]
            or spec["buttons"]
            or spec["inputs"]
            or spec["images"]
        ), app_path
