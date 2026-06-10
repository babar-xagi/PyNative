from __future__ import annotations

import re
import runpy
import sys
from dataclasses import dataclass
from pathlib import Path
from typing import Any


@dataclass(frozen=True)
class CreatedProject:
    name: str
    path: Path
    files: tuple[Path, ...]


def create_project(name: str, base_dir: Path | None = None) -> CreatedProject:
    project_name = normalize_project_name(name)
    root = (base_dir or Path.cwd()) / project_name

    if root.exists() and any(root.iterdir()):
        raise FileExistsError(f"Project directory is not empty: {root}")

    directories = [
        root,
        root / "assets",
        root / "screens",
        root / "services",
        root / "backend",
    ]

    for directory in directories:
        directory.mkdir(parents=True, exist_ok=True)

    files = {
        root / "app.py": app_template(project_name),
        root / "pynative.toml": config_template(project_name),
        root / "requirements.txt": "",
        root / "README.md": readme_template(project_name),
        root / ".gitignore": ".venv/\n__pycache__/\nbuild/\ndist/\n",
        root / "screens" / "__init__.py": "",
        root / "services" / "__init__.py": "",
    }

    for path, content in files.items():
        path.write_text(content, encoding="utf-8")

    return CreatedProject(
        name=project_name,
        path=root,
        files=tuple(files.keys()),
    )


def normalize_project_name(name: str) -> str:
    normalized = re.sub(r"[^A-Za-z0-9_-]+", "-", name.strip()).strip("-_")
    if not normalized:
        raise ValueError("Project name cannot be empty")
    return normalized


def resolve_app_path(target: str | Path | None = None) -> Path:
    path = Path(target) if target is not None else Path.cwd()
    if path.is_dir():
        path = path / "app.py"

    if not path.exists():
        raise FileNotFoundError(f"Could not find app file: {path}")

    if path.suffix != ".py":
        raise ValueError(f"App file must be a Python file: {path}")

    return path.resolve()


def load_app(app_path: str | Path | None = None) -> Any:
    path = resolve_app_path(app_path)
    project_root = str(path.parent)
    old_path = list(sys.path)

    try:
        if project_root not in sys.path:
            sys.path.insert(0, project_root)
        namespace = runpy.run_path(str(path), run_name="__pynative_app__")
    finally:
        sys.path[:] = old_path

    app = namespace.get("app")
    if app is None:
        raise RuntimeError(f"{path} must define an `app` variable")

    if not hasattr(app, "run"):
        raise TypeError(f"`app` in {path} does not look like a PyNative App")

    return app


def run_desktop_app(target: str | Path | None = None, *, summary: bool = False) -> None:
    app = load_app(target)
    app.run("summary" if summary else "desktop")


def app_template(project_name: str) -> str:
    title = title_from_name(project_name)
    return f'''from pynative import App, Button, Column, State, Text, Window

count = State(0)


def increment() -> None:
    count.set(count.value + 1)


app = App(
    Window(
        title="{title}",
        child=Column(
            [
                Text(lambda: f"Count: {{count.value}}"),
                Button("Increase", on_click=increment),
            ]
        ),
    )
)


if __name__ == "__main__":
    app.run()
'''


def config_template(project_name: str) -> str:
    package = package_from_name(project_name)
    title = title_from_name(project_name)
    return f'''[app]
name = "{title}"
package = "{package}"
version = "0.1.0"

[python]
mode = "on_device"
version = "3.12"
requirements = "requirements.txt"

[platforms]
windows = true
android = false
linux = false
macos = false
ios = false

[backend]
enabled = false
mode = "remote"
'''


def readme_template(project_name: str) -> str:
    title = title_from_name(project_name)
    return f'''# {title}

Run the app:

```powershell
pynative run desktop
```

Run a non-GUI summary check:

```powershell
pynative run desktop --summary
```
'''


def title_from_name(name: str) -> str:
    return name.replace("-", " ").replace("_", " ").title()


def package_from_name(name: str) -> str:
    safe = re.sub(r"[^a-z0-9]+", "", name.lower())
    return f"com.pynative.{safe or 'app'}"
