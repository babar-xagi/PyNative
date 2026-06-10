from __future__ import annotations

import argparse
import json
import platform
import sys
from pathlib import Path

from pynative.android import android_environment, build_android_app, run_android_experiment
from pynative.project import create_project, run_desktop_app


def main() -> None:
    parser = argparse.ArgumentParser(prog="pynative")
    subcommands = parser.add_subparsers(dest="command", required=True)
    new_project = subcommands.add_parser("new", help="Create a new PyNative app project")
    new_project.add_argument("name", help="Project directory name")

    build = subcommands.add_parser("build", help="Build PyNative app artifacts")
    build_targets = build.add_subparsers(dest="artifact", required=True)
    apk = build_targets.add_parser("apk", help="Build an Android APK")
    apk.add_argument(
        "target",
        nargs="?",
        help="App file or project directory. Defaults to the current directory when it contains app.py.",
    )
    apk.add_argument("--install", action="store_true", help="Install the APK after building")
    apk.add_argument("--launch", action="store_true", help="Install and launch the APK after building")

    run_project = subcommands.add_parser("run", help="Run a PyNative app")
    run_project.add_argument("platform", choices=["desktop", "android"], help="Target platform")
    run_project.add_argument(
        "target",
        nargs="?",
        help="App file or project directory. Defaults to the current directory.",
    )
    run_project.add_argument(
        "--summary",
        action="store_true",
        help="Print a widget-tree summary instead of opening a native window",
    )
    run_project.add_argument(
        "--build-only",
        action="store_true",
        help="For Android, build the APK without installing or launching it",
    )
    subcommands.add_parser("doctor", help="Check the local PyNative UI development setup")
    hello_window = subcommands.add_parser(
        "hello-window",
        help="Open the Phase 0.1 native Windows prototype window",
    )
    hello_window.add_argument("--title", default="PyNative UI Phase 0.1")
    hello_window.add_argument(
        "--message",
        default="Rust opened a native Windows window.",
    )
    hello_window.add_argument("--button-label", default="Click native button")
    callback_window = subcommands.add_parser(
        "callback-window",
        help="Open a native Windows window that calls Python on each button click",
    )
    callback_window.add_argument("--title", default="PyNative UI Phase 0.2")
    callback_window.add_argument(
        "--message",
        default="Native button will call Python live.",
    )
    callback_window.add_argument("--button-label", default="Call Python")

    args = parser.parse_args()

    if args.command == "new":
        created = create_project(args.name)
        print(
            json.dumps(
                {
                    "name": created.name,
                    "path": str(created.path),
                    "files": [str(path.relative_to(created.path)) for path in created.files],
                },
                indent=2,
            )
        )
    elif args.command == "build":
        if args.artifact == "apk":
            result = build_android_app(
                target=Path(args.target) if args.target else None,
                install=args.install or args.launch,
                launch=args.launch,
            )
            raise SystemExit(result.returncode)
    elif args.command == "run":
        if args.platform == "desktop":
            run_desktop_app(Path(args.target) if args.target else None, summary=args.summary)
        elif args.platform == "android":
            if args.summary:
                raise SystemExit("--summary is only supported for desktop runs")
            raise SystemExit(
                run_android_experiment(
                    Path(args.target) if args.target else None,
                    build_only=args.build_only,
                )
            )
    elif args.command == "doctor":
        print(json.dumps(doctor(), indent=2))
    elif args.command == "hello-window":
        print(
            json.dumps(
                hello_window_command(args.title, args.message, args.button_label),
                indent=2,
            )
        )
    elif args.command == "callback-window":
        print(
            json.dumps(
                callback_window_command(args.title, args.message, args.button_label),
                indent=2,
            )
        )


def doctor() -> dict:
    native_available = True
    native_error = None

    try:
        from pynative import _native

        native_info = json.loads(_native.runtime_info())
    except Exception as exc:  # pragma: no cover - useful diagnostic output
        native_available = False
        native_error = str(exc)
        native_info = None

    return {
        "python": sys.version.split()[0],
        "platform": platform.platform(),
        "native_extension": native_available,
        "native_error": native_error,
        "runtime": native_info,
        "android": android_status(),
    }


def android_status() -> dict:
    env = android_environment()
    return {
        "sdk": str(env.sdk) if env.sdk else None,
        "adb": str(env.adb) if env.adb else None,
        "android_studio_java": str(env.android_studio_java)
        if env.android_studio_java
        else None,
        "build_script": str(env.build_script),
        "build_script_exists": env.build_script.exists(),
    }


def hello_window_command(title: str, message: str, button_label: str) -> dict:
    from pynative import _native

    return json.loads(
        _native.run_windows_hello_window(
            title=title,
            message=message,
            button_label=button_label,
        )
    )


def callback_window_command(title: str, message: str, button_label: str) -> dict:
    from pynative import _native

    def on_click(clicked: int) -> None:
        print(f"Python callback received native click #{clicked}")

    return json.loads(
        _native.run_windows_callback_window(
            on_click,
            title=title,
            message=message,
            button_label=button_label,
        )
    )


if __name__ == "__main__":
    main()
