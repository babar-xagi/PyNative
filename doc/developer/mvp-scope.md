# MVP Scope

The first MVP should be small enough to build but impressive enough to prove the idea.

## MVP Features

- Python declarative API with `App`, `Window`, `Column`, `Row`, `Text`, `Button`, `Input`, and `Image`.
- State object with automatic UI refresh.
- Rust event loop and bridge to Python callbacks.
- Windows desktop run command.
- Android proof of concept with a basic screen and button callback.
- CLI commands: `new`, `run desktop`, `run android`, `build windows`, `build apk`.
- Documentation with examples.

## First Demo Apps

| Demo App | Why It Matters |
| --- | --- |
| Counter app | Shows state and callbacks |
| Login form | Shows input, validation, and layout |
| AI chatbot UI | Matches AI framework positioning |
| Data table app | Shows business/data use case |
| File analyzer app | Shows backend mode and progress jobs |

## Out Of Scope For First MVP

- Full iOS support.
- Custom GPU renderer.
- Full scientific Python package support on mobile.
- Advanced animation system.
- Public plugin marketplace.
- Visual UI designer.

## Definition Of MVP Done

- A new app can be generated from the CLI.
- A Windows app can run from Python source.
- At least five widgets render correctly.
- Button callbacks can call Python.
- State updates refresh visible UI.
- Android proof of concept can render a screen and handle one callback.
- Docs include install, first app, compatibility model, and troubleshooting.
