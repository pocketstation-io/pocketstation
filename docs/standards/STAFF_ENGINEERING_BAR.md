# Staff Engineering Bar — PocketStation

This document defines the minimum engineering bar for all PocketStation code.

The goal is not clever code. The goal is boring, correct, maintainable, observable systems code that survives real production use.

This file ships into every PocketStation repo at `docs/standards/STAFF_ENGINEERING_BAR.md`. Agents must read it before any non-trivial code change.

---

## 1. General Principles

Every change must optimize, in this order:

1. Correctness
2. Simplicity
3. Maintainability
4. Testability
5. Observability
6. Performance — only where required
7. Explicit tradeoffs

Rules:

- Do not write clever code when clear code is enough.
- Do not introduce abstractions before at least two real use cases or an ADR.
- Do not change public APIs casually.
- Do not hide uncertainty. If a decision is provisional, mark it clearly.

## 2. Staff-Level Code Requirements

A PR or branch is not acceptable unless it answers:

- What problem does this solve?
- Why is this the smallest correct design?
- What invariants does this code rely on?
- How is it tested?
- What can go wrong?
- What is intentionally not solved?
- What future phase does this unblock?

These answers live in `PHASE<N>_PROGRESS.md` for the current phase, not in commit messages or PR descriptions alone.

## 3. Rust Real-Time Audio Rules

Hot-path code (anything that runs on or near an audio callback) must not:

- allocate after initialization
- lock
- block
- log
- panic in release builds
- call `async`/`await`
- call FFI per frame
- perform ML inference
- depend on unbounded queues

All callback-path code must be:

- bounded
- predictable
- measurable via metrics
- explicit about ownership and lifetimes

## 4. Unsafe Code

`unsafe` is permitted at FFI boundaries (cbindgen-generated bridges, JNI handoffs, raw pointers from platform callbacks like `ByteBuffer.allocateDirect()` or `AVAudioPCMBuffer`) when there is no safe alternative.

Every `unsafe` block must have a `SAFETY:` comment that documents:

- the invariants the caller must uphold
- why the operation is sound under those invariants
- how the surrounding code maintains the invariants

Outside FFI, prefer safe Rust. New `unsafe` outside FFI requires an ADR.

## 5. API Design Rules

Rust APIs should follow the Rust API Guidelines where practical:

- meaningful types, not naked booleans
- explicit ownership and lifetimes
- no surprising side effects
- clear error types, not stringly-typed errors
- prefer small focused traits
- document invariants in rustdoc
- make invalid states hard to represent

Public APIs are a long-term contract. Before exposing something `pub`, ask: does another crate need this now, or could it stay private until the consumer materializes?

## 6. Testing Bar

Every implementation task must include at least one of:

- unit tests
- integration tests
- invariant / property tests
- regression tests for a fixed bug
- benchmark or smoke test
- explicit explanation in the progress file why testing is not practical

Minimum checks for `pocketstation` (Phase 0):

```
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo run -p pocketstation-audio --example sine_to_wav
```

Minimum checks for Go services (Phase 1+):

```
go fmt ./...
go vet ./...
go test ./...
go test -race ./...
```

## 7. Review Bar

Reviewers must block changes for:

- architecture drift not covered by an ADR
- missing tests
- unclear ownership
- unbounded queues
- hidden allocation on hot paths
- panic paths in release builds
- premature or unjustified abstractions
- dependency creep
- public API drift
- fake completion claims in progress files

A change that merely compiles and passes basic tests is not enough. The review asks: would a senior engineer joining the project next month understand this code, trust its invariants, and be able to change it safely?

## 8. Commit Bar

Each commit completes one logical step.

Format:

```
type(scope): short imperative summary
```

Examples:

```
fix(frame): prevent double-release corruption
test(bus): add drop-newest invariant coverage
docs(standards): add structure naming style rules
feat(metrics): add atomic frame counters
```

Avoid:

```
update
fix
work
stuff
changes
WIP
```

WIP commits are fine on a working branch but must be squashed before merge.

## 9. Architecture Document Rules

The architecture document at `docs/architecture/PocketStation-v2.3.md` (and any successor version) is treated as the source of truth for the project's shape.

Agents must not edit the architecture document without an ADR documenting why the change is needed.

AUDIO-008 through AUDIO-013 in v2.3 are open questions; resolving them may legitimately require architecture-doc changes. When that happens:

1. Write the ADR first.
2. Get human approval on the ADR.
3. Then edit the architecture doc to reflect the resolved decision.
4. Bump the architecture doc version (v2.3 → v2.4 etc.).

Not: edit the doc speculatively, then write an ADR to justify it.

## 10. Forbidden Agent Behavior

Agents must not:

- edit `docs/architecture/PocketStation-v2.X.md` without an ADR
- add dependencies without explicit human approval
- create new phases
- implement future-phase features early
- weaken or skip tests to make CI pass
- hide failing commands or suppress error output
- claim production readiness without evidence (benchmarks, tests, deployment history)
- create empty placeholder repos to claim "Phase N started"

## 11. Self-Check Before Every Commit

Before every commit, the agent writes this in the relevant `PHASE<N>_PROGRESS.md`:

```md
### Staff Bar Self-Check — <task name>

- Smallest correct design: yes / no / explain
- Tests added or updated: yes / no / explain
- Hot-path safe: yes / no / not applicable
- Public API changed: yes / no
- New dependency: yes / no
- Phase scope respected: yes / no
- Unsafe added: yes / no — if yes, SAFETY comment present
- Remaining risk:
```

If any answer is unclear or "no" without justification, do not commit. Stop and ask for human input.