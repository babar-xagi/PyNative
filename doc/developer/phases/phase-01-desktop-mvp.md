# Phase 1 - Desktop MVP

## Goal

Build the first usable Windows desktop MVP with core widgets, state, events, and a CLI run command.

## Current Phase 1 Slice

```powershell
uv run maturin develop
uv run python examples\counter\app.py
uv run pynative run desktop examples\login\app.py
uv run pynative new sample_app
uv run pynative run desktop sample_app --summary
```

The counter and login examples now render Python widget trees into native Windows windows. Button clicks route back to Python, update `State`, and refresh the rendered tree. Input text changes update Python `State` without rebuilding the focused edit control on every keystroke. The CLI can also create a new app template and run it through the desktop command.

## Deliverables

- Python package exposing the first public API.
- Rust runtime with event loop, widget registry, and state refresh.
- Windows desktop shell.
- Core widgets: `App`, `Window`, `Column`, `Row`, `Text`, `Button`, `Input`, `Image`.
- CLI command: `pynative new` and `pynative run desktop`.
- Example apps: counter, login form, and basic data table.

## Workstreams

- Public Python API design.
- Widget tree serialization from Python to Rust.
- Event dispatch from native UI to Python callbacks.
- State object implementation.
- Basic layout engine.
- Asset loading for images.
- CLI project templates.
- Developer documentation.

## Acceptance Criteria

- A new project can be created with `pynative new`.
- The generated app runs on Windows with `pynative run desktop`.
- Text, button, input, row, column, and image widgets render.
- Button and input callbacks reach Python.
- State updates refresh visible UI.
- Counter and login demos work from a clean project.

## Exit Gate

Phase 1 is complete when a Python developer can build and run a small Windows app without manually touching Rust build commands.

## Enhancement Added

Add desktop smoke tests that launch a sample app and verify the event loop starts. GUI frameworks need simple automated confidence from the beginning.
