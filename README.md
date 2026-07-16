# PocketStation

PocketStation is the central open-source Rust runtime for turning permitted
desktop application and microphone audio into independent, source-aware live
stems that can be processed, fanned out, transported, and recorded.

This workspace was formerly named `pocketstation`. It is now the product center;
the individual `pks-*` crates keep narrow technical responsibilities. The
current product proof is intentionally smaller than the long-term runtime:

```text
one desktop application + one microphone
                ↓
    two independent timed stems
                ↓
example connector + browser/remote receiver + multistem recording
```

The binding deadline, acceptance gate, and scope cuts are in the factory root
`PRODUCT_OPERATING_CONTRACT.md`. Repository and crate ownership are in
`V3_REPO_OWNERSHIP_STATE.md` and [CRATE_OWNERSHIP.md](docs/architecture/CRATE_OWNERSHIP.md).

## Workspace

| Crate | Responsibility |
|---|---|
| `pks-frame` | Audio buffers, source/stem identity, timestamps, sequence and lineage |
| `pks-timing` | Clock drift/correction and compiled experimental SegmentGate storage |
| `pks-caps` | Capability, permission, media-edge and backpressure requirements |
| `pks-codec` | Opus encode/decode and codec configuration |
| `pks-metrics` | Runtime counters and bounded latency/drop observations |
| `pks-graph` | Open signal/operator/endpoint contracts, compiler and plan |
| `pks-runtime` | Realtime/async scheduling, bounded Bridges and fan-out |
| `pks-capture` | Platform-neutral permitted capture contract |
| `pks-capture-macos` | macOS application/system/device capture adapter |
| `pks-capture-windows` | Windows capture adapter (partial) |
| `pks-capture-linux` | Linux capture adapter (partial) |
| `pks-nodes` | First-party audio, transport-adjacent and recording operators |
| `pks-ml` | Bounded local audio inference primitives |
| `pks-pipeline` | Composition helpers |
| `pks-audio` | Public facade and integration examples |

Provider-specific model connectors belong in `examples/` or external/community
packages. They do not belong in first-party `pks-ai-*` crates.

## Timing Boundary

`pks-timing` owns runtime clock-domain estimation and correction. The relay
owns live RTP sequence/timestamp continuity, pacing, repair, and RTCP clock
lineage. WebRTC receivers own network jitter buffering and playout. The retired
`media-clock` workspace is tagged history and is not a dependency of this
workspace, the CLI, or the neutral benchmark.

`pks_timing::experimental::SegmentGate` remains compiled and tested. It is
preserved code for a future generated-audio endpoint, not a current product
claim and not merely Git history.

There is no accepted `pks-playout` crate. Add one only when a real native
receiver needs decoder-side jitter buffering, FEC/PLC scheduling, and device
playout outside WebRTC.

## Development

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo run -p pks-audio --example holy_shit_demo
```

The narrow product is not complete merely because the workspace tests pass.
Completion requires the external app+mic, three-destination, 60-minute and
clean-checkout evidence defined in the product operating contract.

## Current Status

- Frame, graph, runtime, codec, core node and macOS capture foundations: REAL or
  PARTIAL as recorded in `PHASE2_PROGRESS.md`.
- Realtime-to-async partition execution, complete bounded multi-destination
  fan-out, and multistem product workflow: current Phase 2 critical path.
- Windows/Linux production capture, virtual devices, consumer apps, audio
  memory and broad provider catalogs: deferred from the 2026-08-15 proof.
