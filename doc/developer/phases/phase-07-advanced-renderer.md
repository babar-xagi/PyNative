# Phase 7 - Advanced Renderer

## Goal

Add an optional custom canvas/wgpu renderer for Flutter-like UI, animations, and highly consistent cross-platform visuals where native widgets are not enough.

## Important Constraint

Do not build the custom renderer too early. Native widgets should remain the first renderer because they reduce early complexity, app size, and platform risk.

## Deliverables

- Renderer abstraction mature enough to support multiple backends.
- Prototype custom canvas or wgpu renderer.
- Text rendering strategy.
- Input, focus, accessibility, and hit testing model.
- Animation primitives.
- Renderer selection in configuration.
- Performance benchmarks against native widget mode.

## Workstreams

- Renderer trait/interface in Rust.
- Scene graph design.
- Layout compatibility with existing widget model.
- Text shaping and font fallback.
- GPU resource lifecycle.
- Accessibility mapping.
- Animation scheduler.

## Acceptance Criteria

- The same simple app can render through native mode and custom renderer mode.
- Custom renderer supports basic text, button, input, layout, and hit testing.
- Performance and app size costs are measured.
- Docs explain when to choose native widgets versus custom renderer.

## Exit Gate

Phase 7 is complete when the custom renderer proves clear value without weakening the original native-widget path.

## Enhancement Added

Treat the advanced renderer as an optional premium capability, not the identity of the whole framework.
