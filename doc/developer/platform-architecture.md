# Platform Architecture

## High-Level Architecture

```text
Python App Code
    |
    v
Python Declarative API
    |
    v
Python-Rust Bridge
    |
    v
Rust Core Runtime
    |-- State engine
    |-- Layout engine
    |-- Event queue
    |-- Widget registry
    |-- Renderer adapter
    |-- Platform services
    |
    v
Platform Shells
    |-- Windows shell
    |-- Android Kotlin shell
    |-- Linux shell
    |-- macOS shell
    |-- iOS Swift shell
```

## Layer Responsibilities

| Layer | Responsibilities |
| --- | --- |
| Python API | Widgets, state, routing, callbacks, developer-facing app code |
| Bridge | Convert Python widget tree into Rust data model, return events to Python |
| Rust core | Layout, event loop, scheduling, widget registry, platform abstraction |
| Renderer adapter | Native widgets first, custom renderer/canvas later |
| Platform shell | App lifecycle, permissions, file system, camera, notifications, packaging |
| Backend mode | Full Python AI/data stack on local, LAN, cloud, or edge backend |

## Core Runtime Modules

- `pynative_api`: Python package with public widgets and state primitives.
- `pynative_bridge`: PyO3 bridge for desktop MVP and shared data conversion.
- `pynative_core`: Rust runtime, state graph, event loop, layout, and scheduler.
- `pynative_widgets`: Shared widget definitions and native widget registry.
- `pynative_platform_windows`: Windows platform shell.
- `pynative_platform_android`: Android/Kotlin shell plus Rust integration.
- `pynative_cli`: Project creation, run, build, packaging, and diagnostics.

## Architecture Decisions To Record

Create architecture decision records in a future `doc/adr` folder for:

- Python bridge approach for desktop.
- Windowing/native widget toolkit.
- Widget tree serialization model.
- State update and diffing strategy.
- Android embedded Python strategy.
- Packaging and dependency isolation strategy.
- Backend transport protocol.
