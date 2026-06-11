# Production Engineering Bar — PocketStation

This document defines when a phase is *actually* done — not when its repos compile and unit tests pass, but when the product flow the phase promised works, is measured, and can survive failure.

This file ships into every PocketStation repo at `docs/standards/PRODUCTION_ENGINEERING_BAR.md`. It is the third standards doc, alongside `STAFF_ENGINEERING_BAR.md` (code quality bar) and `STRUCTURE_NAMING_STYLE_THINKING.md` (structure and naming).

---

## How This Bar Applies

This bar applies **at phase exit**, not retroactively to in-progress work.

Phase 0 produced scaffolds, types, and unit tests. That was correct for Phase 0. The bar below is what Phase 1, Phase 2, and every subsequent phase must clear *before being marked done*.

Code already shipped under earlier phases is not failed retroactively. It is audited against this bar at the next phase-exit checkpoint, and any gaps become tasks in the next phase's hardening pass.

The bar is not negotiable downward. It can be deferred (with an ADR) or split across phases (with explicit phase-exit criteria), but it cannot be silently lowered.

---

## 1. Product-Flow Rule

Every phase defines one real user-visible flow. The phase is not done until the flow works end-to-end against the actual code, not against mocks.

Phase 1 flow:

```
room created via control plane
→ token issued
→ fake-source publisher (or real iOS source) connects via WebRTC
→ browser subscriber connects
→ RTP packets reach the browser
→ audio is audible / measurable in the listener
→ stats panel reports non-null values
→ disconnect + reconnect behavior is exercised and known
```

A repo passing tests in isolation does not prove the phase. The conductor (root-level integration runner) must execute the flow and produce a report.

## 2. Test Pyramid Rule

PocketStation is a protocol project. Most bugs live at integration boundaries (FFI, signaling, WebRTC negotiation, RTP semantics, jitter buffer behavior under real network), not inside individual functions. The pyramid is therefore weighted toward integration:

```
~50% small/unit tests       allocation, ring buffer, pool, codec sanity, graph passthrough
~40% medium integration     cross-service signaling, room lifecycle, publisher↔relay,
                            relay↔listener, FFI boundary contracts on real devices
~10% large E2E              full source → relay → browser, 2-5 scenarios max
```

Standard pyramid guidance suggests ~70/20/10. PocketStation deliberately runs ~50/40/10 because the value lives in the integration tier. Don't overshoot E2E — 10% is a ceiling, not a target.

## 3. CI Honesty Rule

CI must not hide failures.

**Forbidden in correctness checks:**

```bash
go test -race ./... || true
pnpm test || true
cargo test || echo "tests failed"
```

unless the command is explicitly non-blocking (e.g. a lint check during early development that hasn't been triaged) and the override is documented in the CI file with a `# DELIBERATE NON-BLOCKING:` comment and an issue link.

**Required:**

```
Rust:    cargo fmt --check, cargo clippy -D warnings, cargo test, examples run,
         alloccheck/criterion benchmarks where relevant
Go:      gofmt, go vet, go test ./..., go test -race ./...
Web:     correct package manager, typecheck, build, Playwright smoke
Mobile:  simulator/emulator compile and test once SDK phases start
```

Status checks that lie are worse than status checks that don't exist — they produce false confidence.

## 4. Integration Contract Rule

Any cross-repo contract must be tested by a test that actually executes the contract.

Examples for Phase 1:

- api-server token signature must be accepted by relay (or relay must explicitly own room creation in Phase 1, documented in the relay's README)
- relay signaling message JSON must round-trip through the web receiver's TypeScript types
- fake-source publisher must complete the PUBLISH WebRTC flow against the real relay binary
- browser subscriber must complete the SUBSCRIBE WebRTC flow against the real relay binary

Contracts that aren't tested are documentation, not contracts.

## 5. Performance Rule

Phase 1 must measure three hot paths, no more, no less. The list is fixed so this gate cannot expand into its own multi-week project:

```
Phase 1 performance gate:
  1. AudioBufferPool acquire / release / drop (Criterion bench in audio-core)
  2. JWT verify rate (Go bench in relay/auth)
  3. Pion TrackLocalStaticRTP.WriteRTP allocation profile (per AUDIO-009)
```

Phase 2 hardening expands the gate to include:

```
  FrameBus push/pop rate
  Opus encode/decode per 20ms frame
  Relay RTP fanout at 1 / 10 / 50 / 200 listeners
  Room create/join/delete throughput
  WebSocket signaling latency
```

The full benchmark suite isn't a Phase 1 blocker. Three measurements are. Add the rest in Phase 2 hardening when the relay grows up.

Do not claim "low latency" anywhere in docs without numbers from a Criterion or Go benchmark in this repo.

## 6. Load and Soak Rule

Every phase from Phase 1 onward includes at least one local soak test.

```
Phase 1 minimum soak:
  - 1 fake-source publisher
  - 1 in-process or browser subscriber
  - 5-minute run
  - go test -race active
  - no goroutine leak (count before/after)
  - no unbounded memory growth (RSS sampled at start / 1min / 5min)
  - no race-detector failures

Phase 2 target soak:
  - 1 publisher
  - 50 listeners
  - 30-minute run
  - packets-forwarded, packets-dropped, listener errors all reported
  - p50/p95/p99 forward latency captured
```

Soak isn't load testing. Load testing is "what's the breaking point." Soak is "does it leak when nothing exciting happens." Both matter; Phase 1 focuses on soak.

## 7. Failure-Mode Rule

Every real flow must include tests for predictable failure paths.

Phase 1 failure-mode tests:

```
bad token → request rejected with structured error
expired token → request rejected with structured error
publisher disconnects → relay cleans source, notifies listeners
listener disconnects → relay cleans listener slot, source unaffected
ICE failure → both sides report a clean error (no silent hang)
room deleted while listeners present → graceful close
relay process receives SIGTERM → graceful drain, no deadlock
```

If a failure mode silently degrades to a hang, the test must catch it before merge.

## 8. Observability Rule

Every service exposes structured data sufficient to debug a live session without attaching a debugger.

Minimum fields per log/metric:

```
room_id
session_id
role           (source | listener)
connection_state
error_code     (enum, not free-text)
packets_forwarded
packets_dropped
listener_count
latency_estimate_ms   (where available)
```

This maps to the four golden signals (latency, traffic, errors, saturation). Each Phase 1 service should be able to answer "is the system healthy right now" from these counters alone.

## 9. Fake / Scaffold Inventory Rule

Every active repo maintains a top-level `FAKE_SCAFFOLD_INVENTORY.md` (template ships with this standards bundle).

The inventory lists every mock, stub, scaffold, or deferred component. Every PR that introduces a fake adds a row. Every PR that replaces a fake burns the row down (deletes it in the same PR that lands the real implementation).

A repo whose `FAKE_SCAFFOLD_INVENTORY.md` is missing or out of date fails the production bar at phase exit.

## 10. Phase Exit Rule

A phase cannot be marked PASS if any of these is true:

```
The main user flow does not work end-to-end against real code.
CI can pass while correctness checks fail.
Cross-repo contracts are incompatible or untested.
Progress files claim completion of items that don't work.
Docs claim behavior the code doesn't support (production-ready,
  low-latency, E2EE, etc.) without measurement.
The fake/scaffold inventory has rows whose "replace by" matches
  this phase and they're not burned down.
The three Phase 1 hot paths (or this phase's equivalent) are
  unmeasured.
A 5-minute soak has not run with race detection clean.
```

Phase exit requires:

1. Integration conductor report: PASS.
2. Production audit report (re-run of the Cursor reviewer with the production bar in scope): PASS or CONDITIONAL PASS with documented follow-up.
3. `FAKE_SCAFFOLD_INVENTORY.md` reviewed; no rows assigned to this phase remain.
4. Performance and soak artifacts checked into the repo (`benches/`, `soak/results/`, etc.).

If any of these fails, the phase is not done. Continue work in the same phase. Do not start the next phase.

---

## Self-Check Before Phase Exit

Append this block to `PHASE<N>_PROGRESS.md` at the end of the phase:

```md
### Production Bar Phase Exit Self-Check

- Product flow runs end-to-end against real code: yes / no
- Pyramid coverage (small / medium / large): _% / _% / _%
- CI honest (no `|| true` on correctness): yes / no
- Cross-repo contracts tested: list contract → test mapping
- Hot paths measured (this phase's required list): yes / partial / no
- Soak test run, race-clean, no leaks: yes / no
- Failure modes tested: list which ones
- Observability counters live: yes / partial / no
- FAKE_SCAFFOLD_INVENTORY.md up to date: yes / no
- Rows in inventory blocking this phase exit: list or "none"
- Remaining risk:
```

If any answer is unclear or "no" without an ADR or follow-up ticket, the phase is not done. Stop and produce the missing artifact.