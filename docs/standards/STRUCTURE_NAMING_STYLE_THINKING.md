# Structure, Naming, Style, and Thinking — PocketStation

This document defines how PocketStation code, folders, names, tests, docs, comments, and implementation decisions must be structured.

The goal: every repo looks like it was written by one senior engineering team.

This file ships into every PocketStation repo at `docs/standards/STRUCTURE_NAMING_STYLE_THINKING.md`. Agents must read it before any non-trivial code change.

---

## 1. Core Principle

PocketStation code must be:

- boring
- explicit
- searchable
- reviewable
- testable
- phase-scoped
- architecture-aligned
- easy for a new engineer to understand in their first week

Do not optimize for cleverness, brevity, or agent-output speed. Optimize for long-term maintainability.

## 2. Folder Structure

Folders are organized by responsibility, not by random implementation detail.

Good:

```
crates/pocketstation-frame/
crates/pocketstation-bus/
docs/adr/
docs/standards/
docs/architecture/
```

Avoid these folder names — they're dumping grounds that absorb everything and reveal nothing:

```
utils/
helpers/
common/
misc/
stuff/
shared/        (acceptable only when scope is genuinely shared and named in README)
core/          (acceptable only as a top-level concept, not as a misc bucket)
```

If a folder is hard to name, the design is probably unclear. Fix the design.

## 3. Repo-Level Structure

Every repo must have:

```
AGENTS.md
README.md
docs/REPO_CONTRACT.md
docs/architecture/
docs/adr/
docs/standards/
.github/workflows/
.github/PULL_REQUEST_TEMPLATE.md
.github/CODEOWNERS
```

Every phase-active repo should have:

```
PHASE<N>_QUEUE.md
PHASE<N>_PROGRESS.md
```

Progress files must say:

- what is done and tested
- what is partial (and what would finish it)
- what is fake/mock/scaffold (and what would replace it)
- what is blocked (and on what)
- what needs human decision

## 4. Rust Crate Structure

Rust crates follow:

```
crates/<crate-name>/
  Cargo.toml
  src/
    lib.rs
  tests/
  benches/        (if applicable)
```

Submodules exist only when responsibilities are truly separate.

Good module names:

```
buffer_pool.rs
audio_frame.rs
clock_sync.rs
jitter_buffer.rs
processor_graph.rs
```

Avoid these as module names — they describe role-in-the-abstract, not what the code does:

```
manager.rs
handler.rs
processor.rs        (acceptable when paired with a domain, e.g. vad_processor.rs)
logic.rs
helpers.rs
```

If a module name ends in `manager` or `handler`, explain in a comment at the top why a more specific name was not possible.

## 5. Rust Naming

- Types, traits, enums: `UpperCamelCase`
- Functions, methods, modules, variables: `snake_case`
- Constants: `SCREAMING_SNAKE_CASE`
- Crates: `kebab-case`
- Features: `kebab-case`

Acceptable abbreviations (these are domain-standard):

```
pcm, rtp, sdp, ffi, jni, api, vad, aec, src, dsp, opus, sfu, ice, dtls, srtp
```

Avoid invented abbreviations:

```
buf_mgr, proc_hdl, aud_st, tmp_frm, snd_eng
```

Readable beats short.

## 6. Type Names — Pattern, Not Substring

These type-name patterns indicate the design hasn't found its concrete responsibility yet. Block them unless the issue explicitly approves:

- `Manager` (as the entire suffix: `AudioManager`, `RoomManager`) — what does it manage?
- `Handler` (as suffix without domain: `EventHandler` is fine, bare `Handler` is not)
- `System` (as suffix without domain: `RouteSystem` ok if `Route` is the domain; `AudioSystem` is too vague)
- `Magic`, `Stuff`, `Thing`, `Misc` — anywhere
- `Util`, `Utils`, `Helper`, `Helpers` — as type names or module names
- `UniversalX`, `GlobalX`, `SuperX` — as type names in concrete code (these may appear in vision docs)

Note: this is a **type-name pattern** rule, not a substring blacklist. `super::` in Rust, the word "global" in metric labels, "final" as a Java keyword, etc., are fine. The rule is about names you give to types you're defining.

Good abstractions (concrete responsibility, clear ownership):

```
AudioBufferPool
FrameBus
RoutePlan
SourceCapability
OutputTarget
AudioProcessorNode
```

## 7. Go Naming and Structure

Go packages are small, lowercase, single-word, responsibility-based.

Good:

```
room
signal
auth
relay
rtp
```

Avoid:

```
roomManager   (camelCase, wrong for Go)
room_manager  (snake_case, wrong for Go)
utils, common, misc
```

Every goroutine has a documented teardown path.
Every room/session lifecycle is explicit.
No long-running global state without a clear owner.

## 8. Swift Naming and Structure

Swift APIs optimize for clarity at the point of use.

Good:

```swift
try station.startMicrophoneStream()
try station.stopCurrentRoom()
try station.connect(to: roomToken)
```

Avoid:

```swift
try station.run()
try station.doIt()
try station.process(data)
```

Allowed iOS source type names:

```
MicrophoneSource
OwnAppAudioSource
AVAudioEngineTapSource
PluginHostSource
BroadcastExtensionSource
```

Avoid (until they're real, working capabilities — see v2.3 §5.1):

```
SystemAudioSource
GlobalAudioSource
UniversalCaptureSource
```

## 9. TypeScript Naming and Structure

Good names:

```
RoomClient
RelayConnection
SignalingMessage
ConnectionState
LatencyStats
```

Static web app file structure (no framework, see v2.3 §14.3 app-web-receiver):

```
src/
  main.ts
  signaling.ts
  webrtc.ts
  ui.ts
  metrics.ts
```

Avoid dumping grounds:

```
components/common/
lib/utils/
helpers/
```

## 10. Documentation Writing Style

Docs are direct, factual, and phase-aware.

Good:

> This is a Phase 1 relay scaffold. It supports room creation and signaling but does not yet implement production reconnect behavior.

Avoid:

> This relay is production-ready and scalable.

Never claim, in any doc, without evidence in the repo:

- production-ready
- secure
- low-latency
- zero-allocation
- end-to-end encrypted
- cross-platform
- universal capture

If the repo doesn't contain tests, benchmarks, or deployment history that prove the claim, don't make the claim.

## 11. Comment Style

Comments explain *why*, not *what*.

Good:

```rust
// Drop newest instead of oldest because audio callback freshness
// matters more than completeness — see ADR-004.
```

Avoid:

```rust
// Increment i by 1.
i += 1;
```

Every `unsafe` block has a `SAFETY:` comment (see `STAFF_ENGINEERING_BAR.md` §4).

Every `TODO` references a phase and an ADR or issue:

```rust
// TODO(Phase 1, ADR-009): measure WriteRTP allocation behavior
//                        before production relay.
```

Avoid:

```rust
// TODO fix later
// HACK
// XXX
```

## 12. Error Style

Errors are explicit and domain-specific.

Good:

```rust
pub enum BufferPoolError {
    Exhausted,
    InvalidSlot,
}
```

Avoid in hot-path primitives:

```rust
Err("failed")
Err(anyhow!("oops"))
```

`anyhow` and similar dynamic-error crates are fine in application code (CLI, tests, examples) but not in the public API of core crates.

## 13. Test Naming

Good:

```rust
#[test]
fn acquire_returns_none_when_pool_is_exhausted() {}

#[test]
fn dropping_handle_releases_slot() {}

#[test]
fn drop_newest_policy_preserves_existing_frames() {}
```

Avoid:

```rust
#[test]
fn test1() {}

#[test]
fn works() {}

#[test]
fn pool_test() {}
```

Test names are sentences that describe the contract.

## 14. Test Structure

Every test follows Given / When / Then:

```rust
#[test]
fn acquire_returns_none_when_pool_is_exhausted() {
    // Given
    let pool = AudioBufferPool::new_for_test();

    // When
    let handles: Vec<_> = (0..64).map(|_| pool.acquire().unwrap()).collect();
    let extra = pool.acquire();

    // Then
    assert!(extra.is_none());
    drop(handles);
}
```

For property tests (`proptest`, `quickcheck`), the property statement *is* the test name:

```rust
fn ring_buffer_is_fifo_under_arbitrary_push_pop_sequences(...) {}
```

## 15. Design Thinking

Before implementing non-trivial code, the agent thinks in this order:

1. What invariant must hold?
2. What is the smallest correct design?
3. What is the ownership model?
4. What can fail?
5. What must not happen on the hot path?
6. How will this be tested?
7. What phase does this belong to?
8. What is intentionally not implemented?

The wrong starting question: "What code can I generate fastest?"
The right starting question: "What invariant must never break?"

## 16. Abstraction Rules

Do not introduce an abstraction unless at least one of these is true:

- two real implementations exist
- an ADR requires it
- a phase boundary needs it
- it removes real duplication (not speculative duplication)
- it protects a dangerous invariant

A trait with one implementation is usually premature. Wait for the second concrete need.

## 17. File Size

Soft guideline: when a file passes ~500 lines, consider splitting by responsibility.

Test files have no upper limit — long test files with clear `mod` organization are fine.

Splitting a file just to satisfy a line count is anti-pattern. Splitting because two responsibilities have grown distinct is correct.

## 18. Public API

Before adding `pub`, ask:

- Does another crate or external consumer need this now?
- Is this part of the documented public contract?
- Can this remain `pub(crate)` or private until the consumer materializes?

Default to the most restrictive visibility that works.

## 19. Dependency Rules

No new dependency without answering:

- Why is this needed?
- Why can `std`/`core` not solve it?
- Is it safe for the hot path (no hidden allocation, locks, panics)?
- Does it affect build time materially?
- Does it affect mobile binary size?
- Is it actively maintained?
- Can it be behind a feature flag (optional)?

Dependency changes require explicit human approval. Document the approval in the relevant ADR or progress file.

## 20. Phase Scope

Every file change belongs to the current phase.

Phase 0 (`audio-core`) may touch:

```
audio-core code
docs/standards
docs/adr
tests
benches
ffi placeholders
```

Phase 0 must not implement:

```
relay
iOS app
Android app
protocol repo
SFrame E2EE
OS drivers
ML processing nodes
billing
public channels
social discovery
```

Same pattern for every phase: stay in scope, defer everything else.

## 21. Commit Format

```
type(scope): short imperative summary
```

Types:

```
feat:     new functionality
fix:      bug fix
test:     adding or updating tests
docs:     documentation only
refactor: no behavior change
perf:     performance change
chore:    build/tooling/dependency
```

Examples:

```
fix(frame): prevent double-release corruption
test(bus): add drop-newest invariant coverage
docs(standards): add structure naming style rules
feat(metrics): add atomic frame counters
refactor(graph): rename ProcessorGraph::process to step
```

## 22. Self-Check Before Commit

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

See `STAFF_ENGINEERING_BAR.md` §11 for the same checklist (it lives in both documents intentionally — this is the one that gets read most often).