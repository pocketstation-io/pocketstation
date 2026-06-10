# AUDIO-005-relay-listener-slice — Relay Listener Slice Model

## Status
Accepted. Phase 0 and Phase 1 complete; no reversal triggered. Active for Phase 2.

## Context
PocketStation v2.3 requires this ADR before implementation lands. See `docs/architecture/pocketstation-v2.3.md`.

## Decision
Phase 1 may use RWMutex around the listener slice. Phase 2 migrates to copy-on-write atomic pointer to avoid per-packet lock contention.

## Options considered

See v2.3 §26 for the complete option list.

## Consequences

- Agents must follow this decision until a new ADR supersedes it.
- Tests/benchmarks must verify the decision in the relevant phase.

## Test / measurement plan

- Add unit tests for correctness.
- Add benchmark where performance matters.
- Add soak/load tests where reliability matters.

## Reversal trigger

Measured Phase 0/1 data shows this decision breaks latency, reliability, safety, or developer usability targets.
