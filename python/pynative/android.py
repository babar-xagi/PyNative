from __future__ import annotations

import json
import os
import subprocess
from dataclasses import dataclass
from pathlib import Path
from typing import Any


@dataclass(frozen=True)
class AndroidEnvironment:
    sdk: Path | None
    adb: Path | None
    android_studio_java: Path | None
    build_script: Path


@dataclass(frozen=True)
class AndroidBuildResult:
    returncode: int
    apk: Path
    spec: Path | None
    target: Path | None


def android_environment() -> AndroidEnvironment:
    root = repo_root()
    sdk = find_android_sdk()
    adb = sdk / "platform-tools" / "adb.exe" if sdk else None
    java = Path("C:/Program Files/Android/Android Studio/jbr/bin/java.exe")

    return AndroidEnvironment(
        sdk=sdk,
        adb=adb if adb and adb.exists() else None,
        android_studio_java=java if java.exists() else None,
        build_script=root / "tools" / "android" / "build_experiment.ps1",
    )


def run_android_experiment(
    target: str | Path | None = None,
    *,
    build_only: bool = False,
    android_abi: str = "arm64-v8a",
) -> int:
    result = build_android_app(
        target=target,
        install=not build_only,
        launch=not build_only,
        android_abi=android_abi,
    )
    return result.returncode


def build_android_app(
    target: str | Path | None = None,
    *,
    install: bool = False,
    launch: bool = False,
    android_abi: str = "arm64-v8a",
) -> AndroidBuildResult:
    env = android_environment()
    if not env.build_script.exists():
        raise FileNotFoundError(f"Android build script not found: {env.build_script}")

    root = repo_root()
    target_path = resolve_optional_app_target(target)
    spec_path = None

    command = [
        "powershell",
        "-ExecutionPolicy",
        "Bypass",
        "-File",
        str(env.build_script),
        "-AndroidAbi",
        android_abi,
    ]

    if target_path is not None:
        from pynative.project import load_app

        app = load_app(target_path)
        spec = android_spec_from_app(app, source_path=target_path)
        spec_path = write_android_app_spec(spec)
        command.extend(["-AppSpec", str(spec_path)])

    if install or launch:
        command.append("-Install")
    if launch:
        command.append("-Launch")

    completed = subprocess.run(command, check=False)
    return AndroidBuildResult(
        returncode=completed.returncode,
        apk=root / "build" / "android-experiment" / "pynative-android-debug.apk",
        spec=spec_path,
        target=target_path,
    )


def resolve_optional_app_target(target: str | Path | None = None) -> Path | None:
    from pynative.project import resolve_app_path

    if target is not None:
        return resolve_app_path(target)

    current_app = Path.cwd() / "app.py"
    if current_app.exists():
        return resolve_app_path(Path.cwd())

    return None


def android_spec_from_app(app: Any, *, source_path: str | Path | None = None) -> dict[str, Any]:
    tree = app.to_dict()
    title = find_window_title(tree) or "PyNative Android"
    widgets = collect_android_widgets(tree)
    root_style = find_window_style(tree)

    return {
        "title": title,
        "source_path": str(source_path) if source_path else "built-in experiment",
        "root_style": root_style,
        "elements": collect_android_elements(tree),
        "texts": widgets["texts"],
        "buttons": widgets["buttons"],
        "inputs": widgets["inputs"],
        "images": widgets["images"],
        "has_python_callbacks": widgets["has_python_callbacks"],
        "node_count": count_nodes(tree),
        "max_depth": max_depth(tree),
    }


def write_android_app_spec(spec: dict[str, Any]) -> Path:
    output = repo_root() / "build" / "android-app-spec.json"
    output.parent.mkdir(parents=True, exist_ok=True)
    output.write_text(json.dumps(spec, indent=2), encoding="utf-8")
    return output


def find_window_title(node: dict[str, Any]) -> str | None:
    if node.get("kind") == "Window":
        title = node.get("props", {}).get("title")
        return str(title) if title else None

    for child in node.get("children", []):
        title = find_window_title(child)
        if title:
            return title

    return None


def find_window_style(node: dict[str, Any]) -> dict[str, Any]:
    if node.get("kind") == "Window":
        return style_from_props(node.get("props", {}))

    for child in node.get("children", []):
        style = find_window_style(child)
        if style:
            return style

    return {}


def collect_android_widgets(node: dict[str, Any]) -> dict[str, Any]:
    widgets: dict[str, Any] = {
        "texts": [],
        "buttons": [],
        "inputs": [],
        "images": [],
        "has_python_callbacks": False,
    }

    def visit(current: dict[str, Any]) -> None:
        kind = current.get("kind")
        props = current.get("props", {})

        if kind == "Text":
            widgets["texts"].append(str(props.get("value", "")))
        elif kind == "Button":
            widgets["buttons"].append(str(props.get("label", "Button")))
            if props.get("callback_id") is not None:
                widgets["has_python_callbacks"] = True
        elif kind == "Input":
            placeholder = props.get("placeholder") or "Input"
            widgets["inputs"].append(str(placeholder))
        elif kind == "Image":
            image_text = props.get("alt") or props.get("src") or "Image"
            widgets["images"].append(str(image_text))

        for child in current.get("children", []):
            visit(child)

    visit(node)
    return widgets


def collect_android_elements(node: dict[str, Any]) -> list[dict[str, Any]]:
    elements: list[dict[str, Any]] = []

    def visit(current: dict[str, Any]) -> None:
        kind = current.get("kind")
        props = current.get("props", {})
        style = style_from_props(props)

        if kind == "Text":
            elements.append(
                {
                    "kind": "Text",
                    "value": str(props.get("value", "")),
                    "style": style,
                }
            )
        elif kind == "Button":
            elements.append(
                {
                    "kind": "Button",
                    "value": str(props.get("label", "Button")),
                    "style": style,
                    "has_callback": props.get("callback_id") is not None,
                }
            )
        elif kind == "Input":
            elements.append(
                {
                    "kind": "Input",
                    "value": str(props.get("placeholder") or "Input"),
                    "style": style,
                }
            )
        elif kind == "Image":
            elements.append(
                {
                    "kind": "Image",
                    "value": str(props.get("alt") or props.get("src") or "Image"),
                    "style": style,
                }
            )

        for child in current.get("children", []):
            visit(child)

    visit(node)
    return elements


def style_from_props(props: dict[str, Any]) -> dict[str, Any]:
    style = props.get("style", {})
    if isinstance(style, dict):
        return style
    return {}


def count_nodes(node: dict[str, Any]) -> int:
    return 1 + sum(count_nodes(child) for child in node.get("children", []))


def max_depth(node: dict[str, Any]) -> int:
    children = node.get("children", [])
    if not children:
        return 1
    return 1 + max(max_depth(child) for child in children)


def repo_root() -> Path:
    return Path(__file__).resolve().parents[2]


def find_android_sdk() -> Path | None:
    for key in ("ANDROID_HOME", "ANDROID_SDK_ROOT"):
        value = os.environ.get(key)
        if value and Path(value).exists():
            return Path(value)

    default = Path(os.environ.get("LOCALAPPDATA", "")) / "Android" / "Sdk"
    if default.exists():
        return default

    return None
