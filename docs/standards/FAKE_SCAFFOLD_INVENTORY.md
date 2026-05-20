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
| _example row — delete when first real row lands_ | _SCAFFOLD_ | _audio-core/crates/pocketstation-codec/src/opus_mock.rs_ | _Real libopus binding_ | _Phase 0 task 7_ | _libopus-sys dependency approval_ |

---

## Phase 0 starter rows

These are typical scaffolds expected at Phase 0 exit. Replace this section with the actual state when Phase 0 starts.

| Component | Status | Repo / File | What's missing | Replace by | Blocked on |
|---|---|---|---|---|---|
| Opus encoder/decoder | MOCK | audio-core / pocketstation-codec | Real libopus bindings; current mock copies bytes | Phase 1 | ADR-013 sample format finalized, libopus-sys dep approval |
| JitterBuffer | PARTIAL | audio-core / pocketstation-codec | NetEQ-class adaptive algorithm; current scaffold is fixed-delay | Phase 5 | ADR-010 algorithm choice |
| ClockSync | PARTIAL | audio-core / pocketstation-bus | PI controller per ADR-006; current scaffold is fixed-rate | Phase 1 | ADR-006 resolution |
| DHAT allocation check | DEFERRED | audio-core / tools/pocketstation-alloccheck | Real DHAT integration; current is cargo-bloat placeholder | Phase 1 | DHAT setup in CI |

## Phase 1 expected additions

| Component | Status | Repo / File | What's missing | Replace by | Blocked on |
|---|---|---|---|---|---|
| Fake-source publisher | _to add_ | relay / cmd/fake-source | Real WebRTC publisher; needed for E2E smoke | Phase 1 exit | P1-PROD-003 |
| Token authority | _to add_ | api-server + relay | api-server JWTs accepted by relay (or relay owns issuance) | Phase 1 exit | P1-PROD-002 |
| Browser metrics | PARTIAL | app-web-receiver | Real RTCStats.getStats() values; current returns null | Phase 1 exit | P1-PROD-006 |
| TURN configuration | DEFERRED | relay | Production TURN credentials; STUN-only works on most networks | Phase 2 | TURN provider decision |
| SFrame E2EE | DEFERRED | relay + SDKs | Frame-layer encryption per RFC 9605 | Phase 3 | ADR for per-platform insertion point |

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