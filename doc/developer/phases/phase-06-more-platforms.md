# Phase 6 - More Platforms

## Goal

Expand from Windows and Android to Linux, macOS, and then iOS.

## Platform Order

1. Linux.
2. macOS.
3. iOS.

## Deliverables

- Linux shell and packaging target.
- macOS shell and packaging target.
- iOS shell proof of concept.
- Platform capability matrix.
- Cross-platform CI plan.
- Updated docs and examples.

## Workstreams

- Native shell integration per platform.
- Packaging targets: AppImage/deb, app/dmg, ipa.
- Platform service abstraction cleanup.
- File system, permissions, notifications, and camera differences.
- Python runtime packaging differences.
- CI runners and signing requirements.

## Acceptance Criteria

- Linux can run the core demo apps.
- macOS can run the core demo apps.
- iOS has a documented proof of concept and compatibility limits.
- Platform differences are visible in docs and CLI diagnostics.

## Exit Gate

Phase 6 is complete when each added platform has a working shell, clear packaging story, and documented limitations.

## Enhancement Added

Create a platform capability matrix before adding too many features. It will prevent APIs from silently becoming Windows-only or Android-only.
