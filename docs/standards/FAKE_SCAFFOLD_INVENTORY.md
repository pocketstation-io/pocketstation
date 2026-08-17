# PocketStation Scaffold Inventory

This repository now contains one Cargo package. The workspace-wide inventory
at `../../../../docs/standards/FAKE_SCAFFOLD_INVENTORY.md` is authoritative for
cross-repository status. This file records only live central-package limits and
intentional test doubles.

Use only the project status labels defined by the workspace `AGENTS.md`.

## Active limits

| Component | Status | File | Exact boundary |
|---|---|---|---|
| macOS Audio Server Plug-in fallback | `PARTIAL` | `native/macos/asp/Plugin.cpp`, `src/capture/platform/macos/loopback.rs` | The real direct ASP and versioned shared-memory reader compile; SDK installation/restart is deliberately excluded and final physical qualification remains required. |
| Linux capture | `PARTIAL` | `src/capture/platform/linux/` | PipeWire/ALSA paths are implemented and VM-proven; physical-device and full lifecycle requalification belongs to W13. |
| Windows capture | `PARTIAL` | `src/capture/platform/windows/` | WASAPI paths are implemented and VM-proven; physical-device and full lifecycle requalification belongs to W13. |
| Native extension authoring | `REAL` | `src/abi/extension.rs`, `src/abi/executable_extension.rs`, `src/native_extension/`, `src/session/extensions/native_library.rs` | Extension ABI 1.2 executes bounded source/operator/endpoint callbacks through the existing Session engine and adds absolute-path packaged-library import with transactional registration and retained code lifetime. ABI 1.1 layouts/symbols remain the compatibility floor. Foreign calls remain unwind-contained and off audio callbacks. The compiled test plugin is a conformance fixture, not a product mock or deployment claim. |
| Sidecar execution | `REAL` | `src/runtime/lifecycle/sidecar_protocol.rs`, `src/runtime/lifecycle/sidecar_host.rs` | Public Session registration owns bounded data and reserved control queues, negotiated startup, typed transport, deadlines, close/cancel, observations and kill/wait/reap; external Python crash/hang/malformed/saturation fixtures pass. |
| Unsupported operating systems | `STUB` | `src/capture/platform/mod.rs` | Unsupported targets fail with a typed not-supported error; they never fabricate media. |

## Intentional test-only paths

| Component | Status | File | Purpose |
|---|---|---|---|
| Public Session conformance capture | `MOCKED`, `LOOPBACK-ONLY` | `src/conformance.rs`, `src/connector/conformance.rs` | Feature-gated deterministic capture for external-consumer lifecycle, connector rollback/start-gate/saturation/finalization faults, lineage, bounded-route, cancellation, recording, and replay tests. Disabled by default. |
| Native ABI conformance capture | `MOCKED`, `LOOPBACK-ONLY` | `src/abi/session/conformance_fixture.rs` | Feature-gated deterministic capture for C/C++ ABI lifecycle, lease, bounds, lineage, and panic-containment tests. Disabled by default. |

None of these paths may be presented as physical-device evidence. W10 retains
the accepted macOS proof; W13 owns fresh operational requalification of the
single-package candidate.

## Removed from the shipping package

The former simplified AEC, noise suppression, VAD, and watermark modules were
not consumed by the public Session path and did not meet a production algorithm
claim. They were removed during the Core 1.0 boundary correction instead of
remaining compiled scaffolds. Future implementations belong in external
Operator packages with their own quality evidence.

The deleted `src/runtime/nodes/` shipping path and empty `src/dsp/` directory
were removed during capability recovery. No source directory is currently
empty and no documentation may name DSP experiments as compiled Core owners.
