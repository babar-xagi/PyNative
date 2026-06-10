# Phase 0 - Research and Prototype

## Goal

Select the architecture, prove the Rust-Python bridge, and run a simple native window.

## Primary Questions

- Can Rust open a native Windows window and communicate with embedded Python?
- Can a Python callback be triggered from a native button click?
- Which native widget/windowing stack should the desktop MVP use?
- What is the minimum widget tree data model?
- How will state updates flow from Python to Rust?

## Deliverables

- Rust + Python proof of concept.
- Hello window demo.
- Button click calling a Python callback.
- Minimal widget tree model.
- Initial CLI experiment or script to launch the demo.
- Architecture decision records for major choices.

## Current Phase 0.1 Command

```powershell
uv run maturin develop
uv run pynative hello-window
uv run pynative callback-window
```

This opens the first native Windows prototype. The current implementation proves that Rust can create a Win32 window and native button, then return the click count to Python after the window closes.

The callback command proves that the native button event can call a Python function immediately while the window is open.

## Workstreams

- Evaluate PyO3 embedding and extension-module approaches.
- Evaluate native Windows UI options.
- Design the first widget data model.
- Define the event queue and callback boundary.
- Create early benchmark measurements for startup time, callback latency, and app size.

## Acceptance Criteria

- A Windows window opens from the prototype.
- A native button renders.
- Clicking the button runs Python code.
- Python can update simple state.
- The selected bridge and windowing strategy are documented.

## Exit Gate

Phase 0 is complete when the project has a working proof of concept and enough technical confidence to build the desktop MVP.

## Enhancement Added

Start an ADR habit immediately. Early framework projects can drift quickly; short architecture records will help future contributors understand why decisions were made.
