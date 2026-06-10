# Roadmap

| Phase | Goal | Main Deliverables |
| --- | --- | --- |
| [Phase 0](phases/phase-00-research-and-prototype.md) | Research and prototype | Architecture choices, Rust/Python bridge proof, hello window |
| [Phase 1](phases/phase-01-desktop-mvp.md) | Desktop MVP | Windows app, basic widgets, state, events, CLI run command |
| [Phase 2](phases/phase-02-android-mvp.md) | Android MVP | Kotlin shell, embedded Python, APK build, core widgets |
| [Phase 3](phases/phase-03-package-system.md) | Package system | `requirements.txt` support, pure Python packages, curated native packages |
| [Phase 4](phases/phase-04-ai-data-widgets.md) | AI/data widgets | Table, chart, chat UI, file picker, progress, background jobs |
| [Phase 5](phases/phase-05-backend-mode.md) | Backend mode | Remote functions, API transport, auth, file upload, job status |
| [Phase 6](phases/phase-06-more-platforms.md) | More platforms | Linux, macOS, then iOS |
| [Phase 7](phases/phase-07-advanced-renderer.md) | Advanced renderer | Custom canvas/wgpu renderer for Flutter-like UI where needed |

## Roadmap Rule

Do not move to a new phase until the current phase has a working demo, documented limitations, and a clear exit gate.
