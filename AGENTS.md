# AGENTS.md — pocketstation-io/pocketstation

This repository ships one Rust package and one native library product named
`pocketstation`. Internal ownership is enforced with Rust modules, visibility,
tests, and architecture checks—not separately versioned `pks-*` packages.

The factory-root `AGENTS.md`, `PRODUCT_OPERATING_CONTRACT.md`, and
`PROJECT_STATE.md` are binding. By 2026-08-15 the implementation must prove one
desktop application plus one microphone as independent stems fanning out to an
example connector, browser/remote receiver, and multistem recording.

## Before Editing

Read, in order:

1. Factory-root `AGENTS.md`
2. Factory-root `PRODUCT_OPERATING_CONTRACT.md`
3. Factory-root `docs/standards/CODE_PROTOCOL.md`
4. Factory-root `PROJECT_STATE.md`
5. `docs/architecture/pocketstation-v3.0.md`
6. Factory-root `docs/PocketStation-BuildGuide.md`
7. Relevant ADRs
8. `PHASE2_PROGRESS.md`
9. Factory-root `docs/standards/FAKE_SCAFFOLD_INVENTORY.md`

Before code, state the current task, owning module, product outcome, deadline
milestone, next three acceptance commands, and any scaffold/mock/loopback path.

## Active Product

The narrow Phase 2 product slice is:

- independent application and microphone stems;
- source, clock, timestamp, sequence, lineage, and discontinuity truth;
- bounded one-to-many fan-out with explicit backpressure;
- application polling, example connector, browser/remote delivery, and
  multistem recording;
- permission lifecycle and operational observations.

New platforms, virtual drivers, consumer apps, provider catalogs, and memory
work remain deferred unless the product operating contract changes.

## Package and Module Ownership

There is exactly one central Cargo package: `pocketstation`.

- `src/session`: public declaration and lifecycle semantics, orchestration,
  cancellation, bounded polling, and final outcomes.
- `src/frame`: buffers, source/stem identity, timestamps, sequence, and lineage.
- `src/timing`: clock-domain estimation and correction.
- `src/graph`: open manifests, signals, edges, compiler, and execution plan.
- `src/runtime`: plan execution, bounded Bridges, backpressure, fan-out, and
  runtime observations. Metrics are runtime observations, not a product or a
  separately versioned package.
- `src/capture`: platform-neutral capture contracts and target-selected native
  implementations under `src/capture/platform`.
- `src/endpoint`: open endpoint registration and lifecycle contracts.
- `src/recording`: concrete multistem recording and finalization.
- `src/codec`: codec implementation and compatibility symbols.
- `src/dsp`: bounded local signal processing.
- `src/abi`: the C projection of the same Session and codec implementation.
- `examples`: provider-specific connectors and customer workflows.

The public Rust API is rooted at `pocketstation::Session`. Internal modules are
private unless a deliberately documented extension surface requires exposure.
The `internal-testing` feature exists only for repository-owned fixtures,
benchmarks, the CLI, and migration tests; it is not a second SDK.

The native deliverable is `libpocketstation` with `pocketstation.h`. There is no
separate PocketStation C product. Retained `pks_*` C symbols are temporary ABI
compatibility, not package or product names.

A new package is blocked unless it has an independent consumer, versioning
contract, shipped artifact, security/process boundary, or unavoidable native
toolchain boundary that modules and target-specific Cargo dependencies cannot
provide.

RTP/RTCP pacing and continuity belong to the relay. Receiver jitter buffering
and playout belong to the receiver. Provider implementations remain outside
core.

## Required Quality

- Audio callback/realtime paths are allocation-free, lock-free, blocking-free,
  async-free, log-free, and panic-free.
- Queues are bounded and expose overflow policy and counters.
- Provider integrations do not enter first-party core modules.
- No closed model/policy/provider enums enter graph contracts.
- Update `PHASE2_PROGRESS.md` before a completed code step.
- Update the fake-scaffold inventory when a scaffold is added or burned.
- Component tests never justify a real-device product claim.

## Acceptance

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features --locked -- -D warnings
cargo test --all-targets --all-features --locked
cargo build --release --example product_quickstart --locked
bash scripts/check_protocol.sh
```
