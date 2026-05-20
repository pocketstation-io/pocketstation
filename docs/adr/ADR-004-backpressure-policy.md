# ADR-004-backpressure-policy — Backpressure Policy on Pool / Ring Exhaustion

## Status
Accepted for v2.3 scaffold. Reversal requires Phase 0/1 measurement data.

## Context
PocketStation v2.3 requires this ADR before implementation lands. See `docs/architecture/pocketstation-v2.3.md`.

## Decision
Drop newest when the pool or SPSC ring is full. Stable latency is more important than preserving every frame. Blocking the producer is forbidden.

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
