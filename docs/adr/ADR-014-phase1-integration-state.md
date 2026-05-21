# ADR-014 — Phase 1 Integration State

## Status

Accepted 2026-05-20.

## Context

audio-core Phase 0 is complete as of 2026-05-20. 26 tests pass across 6 crates;
race detector clean; all exit criteria met per v2.3 §15.

relay Phase 1 is complete as of 2026-05-20 (P1-PROD-001 through P1-PROD-010).
The relay's `fake-source` binary (`relay/cmd/fake-source`) publishes synthetic
0xAB RTP packets as a development and integration-testing tool.

audio-core does not connect directly to the relay in Phase 1. The fake-source
binary uses synthetic bytes, not output from `pocketstation-codec`. Real
audio-core output enters the relay pipeline only when sdk-ios (Phase 2) wraps
audio-core via FFI and establishes a WebRTC PUBLISH connection. Until then,
there is no runtime path between audio-core and the relay.

The relay's fake-source is therefore a development tool, not an audio-core
integration. It must not be treated as evidence that audio-core is relay-capable
in Phase 1.

## Decision

audio-core's Phase 2 gate consists of four requirements:

1. **crates.io publish of `pocketstation-audio` v0.1.0** — per v2.3 §14.5 and
   §15. Publish happens after the Phase 1 relay demo validates the API surface.
   Use `cargo-release --workspace` with the dependency order specified in ADR-008.
   The workspace version across all crates must be identical at publish time.

2. **FFI headers generated via cbindgen** — headers must be generated under
   `ffi/` and consumed by sdk-ios. The `AudioBufferPool`, `AudioFrame`,
   `FrameBus`, and core session lifecycle types must be exported. No Rust panic
   may cross the FFI boundary (v2.3 §6, ADR-001).

3. **Opus real bindings replacing the MOCK** — the `real-opus` feature flag
   activates `libopus-sys`. The MOCK row in
   `docs/standards/FAKE_SCAFFOLD_INVENTORY.md` is burned in the same PR that
   validates the real codec path in CI with libopus installed.

4. **ClockSync PI controller** — the dual-stage anti-windup PI controller
   specified in ADR-006 replaces the Phase 0 exponential-smoother placeholder.
   The PARTIAL row for ClockSync in the inventory is burned when the PI
   controller passes its convergence and sign-direction tests.

sdk-ios depends on requirement 1 (the crates.io publish) before it can be
created. The Phase 2 sequence is therefore: audio-core publish first, then
sdk-ios XCFramework integration, then the Phase 2 relay hardening.

## Options considered

**Option A — publish audio-core at Phase 0 exit**
Rejected. v2.3 §15 explicitly defers the actual publish to Phase 1 exit,
after the demo validates the API surface. Publishing at Phase 0 would lock
names and create SemVer pressure before the API has met one real route.

**Option B — integrate audio-core directly with relay in Phase 1**
Rejected. There is no sdk-ios or FFI boundary yet. Building a direct
Rust-to-Go integration path for Phase 1 would invent architecture not
specified in v2.3 and would be discarded when sdk-ios ships.

**Option C (this decision) — relay uses fake-source in Phase 1; audio-core
integrates via sdk-ios in Phase 2**
Accepted. Matches v2.3 §14.6 phase sequence. Keeps relay and audio-core
independently releasable without coupling their Phase 1 timelines.

## Consequences

- sdk-ios cannot be created until audio-core v0.1.0 is published.
- The fake-source binary remains a development-only tool for relay E2E tests.
  It does not represent a production audio path.
- DHAT CI integration (Phase 0 gap) and Criterion benchmarks are Phase 2
  prerequisites alongside the crates.io publish. They must land in the same
  Phase 2 milestone, not after.
- JitterBuffer PARTIAL and ClockSync PARTIAL rows in the scaffold inventory
  remain open until Phase 2 burns them. They are not Phase 1 blockers.

## Test / measurement plan

- `cargo test --workspace` covers Phase 0 exit and must remain green throughout
  Phase 2 development.
- Phase 2 adds: Criterion benchmarks under `benches/`, DHAT CI gate proving zero
  per-frame heap allocation, and a CI job that builds with the `real-opus`
  feature enabled and libopus installed.
- FFI correctness: sdk-ios integration tests exercise the cbindgen-generated
  headers. Any Rust panic crossing the FFI boundary fails the test suite.
- crates.io publish: `cargo publish --dry-run --workspace` must pass in CI
  before the actual publish tag is created. Dependency publish order follows
  ADR-008.

## Reversal trigger

Phase 1 demo latency measurements show the audio-core API surface is
structurally wrong for the relay integration use case (e.g., frame size,
transport abstraction, or FFI shape requires a breaking change). If this
occurs, this ADR is superseded by a revised ADR that documents the required
API changes before any Phase 2 code lands.
