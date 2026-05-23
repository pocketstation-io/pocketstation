# Fake / Scaffold Inventory

This file lists every component in this repo that is currently mocked, stubbed, hardcoded, deferred, or otherwise not production-grade.

**It is a living document.** Every PR that adds a scaffold appends a row. Every PR that replaces a scaffold burns the row down (delete the row in the same PR that replaces it).

**Rule:** if a component is fake but not in this file, the PR that added it failed the production bar. Reviewers block PRs that introduce un-inventoried fakes.

---

## Status column meaning

```
SCAFFOLD    Empty placeholder, returns Default::default() or similar
MOCK        Functional fake — tests pass against it, real impl absent
STUB        Throws unimplemented! or returns hardcoded value
PARTIAL     Real implementation, missing significant behavior
DEFERRED    Intentionally postponed; ADR or phase plan justifies it
```

---

## Active inventory

| Component | Status | Repo / File | What's missing | Replace by | Blocked on |
|---|---|---|---|---|---|
| JitterBuffer | PARTIAL | audio-core / pocketstation-codec | NetEQ-class adaptive algorithm; current scaffold is fixed-delay | Phase 5 | ADR-010 algorithm choice |
| DHAT allocation check | DEFERRED | audio-core / tools/pocketstation-alloccheck | Real DHAT integration; current is cargo-bloat placeholder | Phase 3 | DHAT setup in CI |
| TURN configuration | DEFERRED | relay | Production TURN credentials; STUN-only works on most networks | Phase 2 | TURN provider decision |
| SFrame E2EE | DEFERRED | relay + SDKs | Frame-layer encryption per RFC 9605 | Phase 3 | ADR for per-platform insertion point |

---

## Phase 3 burns (completed 2026-05-23)

| Component | Status | Repo / File | Resolution |
|---|---|---|---|
| Opus encoder/decoder | resolved | audio-core / pocketstation-codec | Real libopus bindings via `opus = "0.3"`. `OpusEncoder` / `OpusDecoder` wrap libopus at 48 kHz / mono / VOIP per ADR-012. `MockOpusEncoder` / `MockOpusDecoder` are retained as legacy aliases that delegate to the real types. |
| ClockSync | resolved | audio-core / pocketstation-bus | PI controller implemented per ADR-006. `kp = 0.1`, `ki = 0.001`. Output clamped to ±10 ms. Gains will be tuned in Phase 5 with real-world measurements. |

---

## Phase 1 burns (completed 2026-05-20)

These rows were resolved at Phase 1 exit. Work landed in the `relay` repo, not in `audio-core`.

| Component | Status | Repo / File | Resolution |
|---|---|---|---|
| Fake-source publisher | resolved | relay / cmd/fake-source | Implemented as P1-PROD-003. Binary publishes synthetic 0xAB RTP; it is a development tool, not an audio-core integration. |
| Token authority | resolved | relay (relay owns issuance) | Implemented as P1-PROD-002. relay issues and validates JWTs; api-server integration deferred to Phase 2. |
| Browser metrics | resolved | app-web-receiver | Completed as P1-PROD-006. Real RTCStats.getStats() values wired. |

## Permanent (intentional) scaffolds

These never become production — they exist for testing and development. They are listed here so they're not confused with production-track components.

| Component | Repo / File | Purpose |
|---|---|---|
| Sine wave source | audio-core / examples/sine_to_wav | Phase 0 smoke test, latency measurement |
| File output sink | audio-core / pocketstation-route | Test recording, offline verification |
| In-memory token store | api-server | Phase 1 only; Phase 2+ uses real persistence |

---

## How to use this file in a PR

When introducing a scaffold:
1. Add the row before the code lands.
2. Be specific about "what's missing" — "real implementation" is not enough.
3. Pick a "replace by" phase. If it's unknown, mark `DEFERRED` and link the ADR or issue tracking the decision.

When replacing a scaffold:
1. Delete the row in the same PR that lands the real implementation.
2. The PR description references the row being removed.

When reviewing:
1. Block any PR that introduces a fake component without adding to the table.
2. Block any PR that claims to "complete" a scaffold but doesn't burn down the row.
3. Block phase exit if the table has rows whose "replace by" matches the current phase.