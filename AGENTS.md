# AGENTS.md — pocketstation-io/pocketstation

This repository is the central PocketStation Rust runtime workspace. It was
formerly named `pocketstation`; current paths and new documentation must use
`pocketstation`.

The factory-root `AGENTS.md`, `PRODUCT_OPERATING_CONTRACT.md`, and
`PROJECT_STATE.md` are binding. By 2026-08-15 the runtime must help prove one
desktop application plus one microphone as independent stems fanning out to an
example connector, browser/remote receiver, and multistem recording.

## Before Editing

Read, in order:

1. Factory-root `AGENTS.md`
2. Factory-root `PRODUCT_OPERATING_CONTRACT.md`
3. Factory-root `PROJECT_STATE.md`
4. Factory-root `docs/standards/CODE_PROTOCOL.md`
5. `docs/architecture/pocketstation-v3.0.md`
6. `docs/architecture/CRATE_OWNERSHIP.md`
7. `PHASE2_PROGRESS.md`
8. `docs/standards/FAKE_SCAFFOLD_INVENTORY.md`
9. Relevant ADRs

State the product-proof outcome, demo line, deadline milestone, owning crate,
and next three acceptance commands before writing code.

## Active Phase

Phase 0 foundation and Phase 1 transport proof are complete under the recorded
exit reports. Active work is the narrow Phase 2 product slice:

- independent application and microphone stems;
- source/clock/timestamp lineage;
- realtime-to-async Bridges;
- bounded one-to-many fan-out and explicit backpressure;
- example connector, browser/remote sink, and multistem recording;
- permission, discontinuity, queue/drop, and latency observability.

New platforms, virtual drivers, consumer apps, provider catalogs, and memory
work are deferred unless the product operating contract explicitly changes.

## Ownership Locks

- `pks-frame`: frame data and lineage.
- `pks-timing`: drift/correction and compiled experimental SegmentGate storage.
- `pks-caps`: capability and permission truth.
- `pks-codec`: codec behavior only.
- `pks-metrics`: runtime observations.
- `pks-graph`: open manifests, signal/edge contracts, compiler and plan.
- `pks-runtime`: scheduling, bounded Bridges, backpressure and fan-out.
- `pks-capture-*`: permitted platform capture.
- `pks-nodes`: first-party audio/recording operators.
- `pks-ml`: bounded local audio inference, not cloud/provider clients.
- `examples/`: provider-specific connectors and customer workflows.

RTP/RTCP pacing, sequence/timestamp translation, repair and clock lineage belong
to the relay. WebRTC receiver jitter buffering/playout belongs to the receiver.
Do not recreate those as `pks-playout` without a real native receiver consumer.

The legacy `media-clock` workspace is retired historical source. The live
runtime, CLI, and benchmarks must not regain a dependency on it. The
experimental Gate must remain compiled and tested in `pks-timing` until an
explicit removal or promotion decision.

## Required Quality

- No allocation, blocking, mutex, I/O, or logging on audio callback/realtime
  paths.
- All queues are bounded and expose their overflow policy and counters.
- Provider integrations do not enter first-party core crates.
- No closed model/policy/provider enums in the graph contract.
- Update `PHASE2_PROGRESS.md` before a commit.
- Update the fake-scaffold inventory whenever a scaffold is added or burned.
- Component tests do not justify a product claim; record real-path artifacts.

## Acceptance

Use the smallest relevant subset first, then the full workspace before merge:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo run -p pks-audio --example holy_shit_demo
```
