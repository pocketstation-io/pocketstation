# PocketStation Scaffold Inventory

This repository now contains one Cargo package. The workspace-wide inventory
at `../../../../docs/standards/FAKE_SCAFFOLD_INVENTORY.md` is authoritative for
cross-repository status. This file records only live central-package limits and
intentional test doubles.

Use only the project status labels defined by the workspace `AGENTS.md`.

## Active limits

| Component | Status | File | Exact boundary |
|---|---|---|---|
| Noise suppressor | `PARTIAL` | `src/dsp/denoise.rs` | Real bounded DSP implementation; it is not production RNNoise/WebRTC NS quality. |
| Echo canceller | `PARTIAL` | `src/dsp/aec.rs` | Real bounded NLMS implementation; delay estimation, double-talk handling, and production AEC3-class behavior remain absent. |
| macOS Audio Server Plug-in bridge | `STUB` | `native/macos/asp/bridge_stub.c`, `src/capture/platform/macos/loopback.rs` | The SDK detects externally provisioned plug-in support but does not install, restart, or silently emulate it. |
| Linux capture | `PARTIAL` | `src/capture/platform/linux/` | PipeWire/ALSA paths are implemented and VM-proven; physical-device and full lifecycle requalification belongs to W13. |
| Windows capture | `PARTIAL` | `src/capture/platform/windows/` | WASAPI paths are implemented and VM-proven; physical-device and full lifecycle requalification belongs to W13. |
| Unsupported operating systems | `STUB` | `src/capture/platform/mod.rs` | Unsupported targets fail with a typed not-supported error; they never fabricate media. |

## Intentional test-only paths

| Component | Status | File | Purpose |
|---|---|---|---|
| Public Session conformance capture | `MOCKED`, `LOOPBACK-ONLY` | `src/conformance.rs` | Feature-gated deterministic capture for external-consumer lifecycle, lineage, bounded-route, cancellation, recording, and replay tests. Disabled by default. |
| Native ABI conformance capture | `MOCKED`, `LOOPBACK-ONLY` | `src/abi/session/conformance_fixture.rs` | Feature-gated deterministic capture for C/C++ ABI lifecycle, lease, bounds, lineage, and panic-containment tests. Disabled by default. |
| Synthetic source operator | `MOCKED`, `LOOPBACK-ONLY` | `src/runtime/nodes/synthetic_source.rs` | Deterministic tone source used only by component and benchmark tests. |

None of these paths may be presented as physical-device evidence. W10 retains
the accepted macOS proof; W13 owns fresh operational requalification of the
single-package candidate.
