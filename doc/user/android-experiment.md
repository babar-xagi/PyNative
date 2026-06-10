# Android Experiment

PyNative UI has an early Android experiment that builds a tiny native Android APK without Gradle.

This is not the final Android runtime yet. It proves that the local Android SDK can build, sign, install, and launch a simple native Android shell.

## Build APK

```powershell
uv run pynative run android --build-only
```

Output:

```text
build\android-experiment\pynative-android-debug.apk
```

## Install And Launch

Connect an Android device with USB debugging enabled, or start an Android emulator, then run:

```powershell
uv run pynative run android
```

This builds the APK, installs it with `adb install -r`, and launches:

```text
com.pynative.experiment/.MainActivity
```

## Current Limits

- Uses a Java Activity for the experiment.
- Does not embed Python yet.
- Does not use the Rust runtime on Android yet.
- Does not render the Python widget tree yet.

Those are the next Phase 2 steps.
