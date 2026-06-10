# Risks and Mitigation

| Risk | Impact | Mitigation |
| --- | --- | --- |
| Trying to support all platforms from day one | Project becomes too large and slow | Start with Windows and Android only |
| Promising all Python packages on mobile | Users will be disappointed | Use the three-level compatibility model |
| Building a custom renderer too early | Massive complexity | Use native widgets first, canvas later |
| Python callbacks blocking UI | Bad performance and frozen app | Rust event loop, async task queue, background jobs |
| Large app size | Poor mobile experience | Runtime trimming, profiles, backend mode |
| Lack of developer trust | Low adoption | Clear docs, examples, honest compatibility table |
| Weak packaging story | Developers abandon the tool | Treat packaging as a core feature from Phase 1 |
| Security gaps in backend mode | User data risk | Auth, transport security, upload limits, log redaction |
| Platform-specific behavior drift | Inconsistent developer experience | Platform capability matrix and CI |
| Poor accessibility | Limited professional adoption | Keyboard navigation, labels, focus, screen reader planning |
| Hard contributor onboarding | Slow open-source growth | ADRs, architecture docs, examples, issue templates |

## Highest-Risk Area

Mobile Python package compatibility is the highest-risk technical and expectation-management area. The project should always describe support in levels:

- Pure Python on-device.
- Curated native packages.
- Backend or edge full ecosystem.

## Release Readiness Checklist

- Current platform support is documented.
- Known package limitations are documented.
- Demo apps are tested.
- CLI errors explain what to fix.
- App size is measured.
- Startup time is measured.
- Backend mode has a security checklist.
- Accessibility basics are present in core widgets.
