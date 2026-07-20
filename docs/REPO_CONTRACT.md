# Repository Contract — pocketstation-io/pocketstation

- Boundary: central Rust runtime workspace
- Previous name: `pocketstation`
- Active phase: Phase 2 narrow product proof
- Release: coordinated SemVer for publishable `pks-*` crates after API and
  clean-checkout product proof
- Product deadline: 2026-08-15

This repository owns reusable source-aware audio frames, capture abstractions,
graph compilation, runtime scheduling, bounded Bridges/fan-out, DSP, codec,
recording primitives, runtime timing, metrics, and the public Rust facade.

It does not own WebRTC relay media-plane behavior, control-plane persistence,
CLI command UX, consumer apps, OS virtual-device drivers, or vendor-specific
model clients.

Dependency direction and individual crate ownership are binding in
`docs/architecture/CRATE_OWNERSHIP.md`. The factory-root
`PRODUCT_OPERATING_CONTRACT.md` and `PROJECT_STATE.md` take precedence over
historical v2.3/v3.0 repo maps.
