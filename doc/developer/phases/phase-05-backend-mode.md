# Phase 5 - Backend Mode

## Goal

Support remote or edge Python execution for heavy AI/data workloads while the app acts as the native UI and control surface.

## Deliverables

- Backend function declaration API.
- API transport between app and backend.
- Local development backend server template.
- Auth strategy.
- File upload support.
- Job status and progress updates.
- Error handling and retry behavior.
- Example app using backend mode for file analysis or AI chat.

## Workstreams

- Backend configuration in `pynative.toml`.
- Request/response and streaming protocol.
- Background job model.
- File upload and download flow.
- Authentication and secret handling.
- Localhost, LAN, cloud, and edge deployment modes.
- Offline and degraded-network behavior.

## Acceptance Criteria

- A mobile or desktop app can call a Python backend function.
- The backend can run packages that are not bundled into the app.
- Long jobs report progress to the UI.
- Files can be sent to the backend and results returned.
- Auth and configuration are documented.

## Exit Gate

Phase 5 is complete when backend mode is reliable enough to be the recommended path for heavy AI/data workloads.

## Enhancement Added

Add a security checklist before release: secrets, transport security, file validation, upload limits, auth expiration, and log redaction.
