# Enhancement Recommendations

These additions strengthen the blueprint and make the project easier to build, explain, and maintain.

## Add Immediately

### Architecture Decision Records

Create short ADR files for major technical choices. This is especially important for bridge, renderer, state, packaging, and Android runtime decisions.

### Compatibility Matrix

Publish a matrix for Python packages and platform capabilities. This protects user trust and helps developers know when backend mode is required.

### `pynative doctor`

Add a diagnostic command that checks:

- Python version.
- Rust toolchain.
- Android SDK.
- Required build tools.
- `pynative.toml`.
- Package compatibility.
- Platform targets.

### Demo Gallery

Build examples that match the project niche:

- AI chat dashboard.
- Data table viewer.
- Medical billing automation panel.
- File analyzer with backend mode.
- Student/education app.

## Add Soon

### Test Strategy

Use several levels of testing:

- Unit tests for Rust core.
- Python API tests.
- Bridge contract tests.
- CLI template tests.
- Desktop smoke tests.
- Android emulator smoke tests.

### Performance Benchmarks

Track startup time, callback latency, state refresh time, table rendering performance, APK size, and Windows installer size.

### Security And Privacy Checklist

Backend mode should define clear rules for auth, secrets, file uploads, transport security, and log redaction.

### Accessibility Baseline

Core widgets should include focus states, keyboard navigation, labels, readable contrast, and screen-reader planning.

## Add Later

### Plugin And Widget Ecosystem

After the MVP is stable, support third-party widgets and integrations. Do not start with a marketplace before the core runtime is reliable.

### Visual Designer

A visual designer could help business users, but it should come after the framework API and renderer model are stable.

### Cloud Build Service

Long term, a cloud builder could simplify Android, macOS, and iOS packaging for users without local setup.

## Recommended First Build Order

1. Rust + Python proof of concept on Windows.
2. Five widgets: `Window`, `Text`, `Button`, `Column`, `Input`.
3. State object and minimal re-rendering.
4. CLI with `new` and `run desktop`.
5. Android proof of concept.
6. Honest mobile compatibility docs.
7. One impressive demo app.
