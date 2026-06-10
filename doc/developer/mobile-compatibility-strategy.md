# Mobile Compatibility Strategy

Mobile support is the hardest part of PyNative UI. The project should avoid promising that every Python package will run on every phone.

## Compatibility Levels

| Level | Name | What It Supports | Examples |
| --- | --- | --- | --- |
| 1 | Pure Python on-device | Packages without heavy native compiled dependencies | `requests`, `pydantic`, utility packages, custom business logic |
| 2 | Curated native packages | Selected binary packages compiled and tested for mobile | `Pillow`, SQLite, NumPy-lite, ONNX Runtime Mobile, OpenCV-lite |
| 3 | Remote/edge full ecosystem | Heavy Python ecosystem outside the phone, controlled through the app | PyTorch, TensorFlow, Pandas heavy jobs, LangChain, large LLM inference |

## Android Strategy

- Use Android/Kotlin shell for lifecycle, permissions, and native services.
- Use Rust core for shared runtime and UI coordination.
- Embed Python runtime for pure Python logic and selected packages.
- Study Chaquopy-style integration patterns for Gradle and Kotlin/Python communication.
- Publish a supported package list instead of implying unlimited package support.

## iOS Strategy

- Do not start with iOS.
- Add iOS after Windows and Android are stable.
- Use Swift shell for iOS lifecycle and native APIs.
- Keep Python package support more limited on iOS.
- Prefer backend mode for heavy AI/data workloads.

## Heavy AI/Data Workloads

```text
Mobile App UI
    |
    | sends task / file / prompt
    v
Python Backend Service
    |-- PyTorch / TensorFlow
    |-- Pandas / NumPy
    |-- LangChain / LlamaIndex
    |-- Vector databases
    |-- Large file processing
    v
Result returned to mobile app
```

## Required Enhancement

Maintain a public compatibility matrix:

- Package name.
- Supported platforms.
- On-device or backend-only.
- Minimum PyNative UI version.
- Known limitations.
- Example project link.
