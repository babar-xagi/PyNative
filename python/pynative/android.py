from __future__ import annotations

import os
import subprocess
from dataclasses import dataclass
from pathlib import Path


@dataclass(frozen=True)
class AndroidEnvironment:
    sdk: Path | None
    adb: Path | None
    android_studio_java: Path | None
    build_script: Path


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


def run_android_experiment(*, build_only: bool = False) -> int:
    env = android_environment()
    if not env.build_script.exists():
        raise FileNotFoundError(f"Android build script not found: {env.build_script}")

    command = [
        "powershell",
        "-ExecutionPolicy",
        "Bypass",
        "-File",
        str(env.build_script),
    ]

    if not build_only:
        command.extend(["-Install", "-Launch"])

    completed = subprocess.run(command, check=False)
    return completed.returncode


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
