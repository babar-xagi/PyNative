# Phase 3 - Package System

## Goal

Support Python dependencies through `requirements.txt`, pure Python package installation, curated native packages, and mobile package profiles.

## Deliverables

- `requirements.txt` support for desktop builds.
- Pure Python package support for Android where feasible.
- Curated mobile package list.
- Package profiles: `lite`, `data`, `vision`, `ai-remote`.
- Dependency diagnostics in the CLI.
- Clear error messages when packages are unsupported.

## Workstreams

- Dependency resolver integration.
- Package isolation and build cache design.
- Runtime trimming strategy.
- Mobile wheel compatibility research.
- Curated package build pipeline.
- Package compatibility matrix.

## Acceptance Criteria

- Desktop apps can install and bundle common pure Python dependencies.
- Android apps can include supported pure Python packages.
- Unsupported mobile packages fail with clear guidance.
- Package profiles change what runtime components are bundled.
- Compatibility docs list supported packages and limitations.

## Exit Gate

Phase 3 is complete when dependency support is predictable, documented, and testable across Windows and Android.

## Enhancement Added

Add `pynative doctor` to inspect Python versions, Rust toolchain, Android SDK, package support, and build configuration.
