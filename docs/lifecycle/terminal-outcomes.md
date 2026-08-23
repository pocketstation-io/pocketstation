# Terminal outcomes

<!-- claims: CLM-DOC-042-CAP-001,CLM-DOC-042-CAP-002,CLM-DOC-042-SOURCE-001 -->

## Scope

- **Start, cancel, stop, and finalize a Session.** Coordinate Session startup, rollback, cancellation, steady-state ownership, stop, and terminal outcomes.
- **Classify public failures.** Expose stable typed errors and cross-boundary error codes without inferring retry or recovery guarantees.

These statements describe repository contracts at the documented snapshot. They do not extend platform qualification, performance, retry, or delivery guarantees beyond the native API contracts and executable evidence.

## Ownership transition

PocketStation uses distinct declaration, compilation, preparation, running, cancellation, rollback, stop, and terminal-result types where the source exposes them. Do not collapse a stop outcome into a Boolean assumption: component and finalization failures remain structured data.

## Extracted lifecycle operations

| Operation | Trigger | Source state | Destination state | Evidence record |
|---|---|---|---|---|
| `drop_observations` | `drop_observations` | unknown | unknown | `life-033f82e656e133a23c4c` |
| `pocketstation::session::lifecycle::engine::SessionEngineStartError::Prepare` | `Prepare` | unknown | unknown | `life-1e6e9f452ca83bcd4874` |
| `pocketstation::session::extensions::audio_input::buffer::AudioInputBufferAcquireError::Cancelled` | `Cancelled` | unknown | unknown | `life-23ee10c2333375238483` |
| `drop` | `drop` | unknown | unknown | `life-2c74a6091c9cde4045cb` |
| `pocketstation::session::lifecycle::start_contract::SessionStartError::Cancelled` | `Cancelled` | unknown | unknown | `life-396be01d86a31314ead0` |
| `drop_rate_pct` | `drop_rate_pct` | unknown | unknown | `life-4ca80c498146346ca2ad` |
| `SourceDriver::close` | `close` | unknown | unknown | `life-51677582d6bfbf19bf36` |
| `pocketstation::session::lifecycle::events::SessionTerminalState::Stopped` | `Stopped` | unknown | unknown | `life-6838c7db0d54daff94be` |
| `close` | `close` | unknown | unknown | `life-6f6114d034e4edd4755a` |
| `start` | `start` | unknown | unknown | `life-7a6ae3567d5c4cae3ab8` |
| `SourceDriver::prepare` | `prepare` | unknown | unknown | `life-8ab6ff40055ef9e6b1e4` |
| `pocketstation::session::extensions::audio_input::buffer::AudioInputWriteErrorKind::Cancelled` | `Cancelled` | unknown | unknown | `life-9dedf8edf94ac1e55756` |
| `pocketstation::session::lifecycle::events::SessionLifecycleState::Stopped` | `Stopped` | unknown | unknown | `life-a103fa50f0a44e41e441` |
| `pocketstation::session::lifecycle::observations::EndpointObservationStage::Finalized` | `Finalized` | unknown | unknown | `life-a125b7d6bdf384df6d6f` |
| `pocketstation::session::error_code::SessionStopCode::Stopped` | `Stopped` | unknown | unknown | `life-b56488eda9c3ea6b7b3e` |
| `close` | `close` | unknown | unknown | `life-b86a51a255f9938c1308` |
| `pocketstation::session::lifecycle::events::SessionLifecycleState::Running` | `Running` | unknown | unknown | `life-ce971b431224b00409e2` |
| `pocketstation::session::lifecycle::engine::SessionEngineStartError::Start` | `Start` | unknown | unknown | `life-cfbf77d2f976c5aed1ae` |

## Failure handling

A transition whose guard, idempotence, or recovery is recorded as unknown has no published guarantee here. Preserve the returned error or terminal outcome, inspect component and stage fields, and consult the error reference before retrying.

## Executable evidence

The following test bodies are evidence only for their recorded setup:

- `given_all_failure_classes_when_terminal_then_each_class_is_preserved` — given all failure classes when terminal then each class is preserved (`src/session/lifecycle/events.rs:679`; `test-070a5fc90aa90f7d986a`).
- `given_no_failures_when_terminal_then_state_is_stopped` — given no failures when terminal then state is stopped (`src/session/lifecycle/events.rs:721`; `test-6dd97c870cc349f825f9`).
- `given_complete_trace_when_validated_then_lifecycle_and_terminal_match` — given complete trace when validated then lifecycle and terminal match (`src/session/lifecycle/trace.rs:988`; `test-9dd947375f94d2a4f21f`).
- `given_record_after_terminal_when_validated_then_trace_is_rejected` — given record after terminal when validated then trace is rejected (`src/session/lifecycle/trace.rs:1160`; `test-13dde755fb2a02648705`).
- `node_mut` — node mut (`src/session/compile/bindings.rs:59`; `test-ef9a92ed792fca62265a`).
- `bindings` — bindings (`src/session/compile/compiled.rs:95`; `test-2bbea620430ca871473a`).
- `bindings_mut` — bindings mut (`src/session/compile/compiled.rs:100`; `test-343b0db015a203ab2c76`).
- `edge_count` — edge count (`src/session/compile/compiled.rs:49`; `test-a6cb0cb0999ae28e13cf`).
- `endpoint_declarations` — endpoint declarations (`src/session/compile/compiled.rs:39`; `test-45244638cf19287785e6`).
- `external_source_declarations` — external source declarations (`src/session/compile/compiled.rs:34`; `test-067a9c1179f9fa65bb67`).
- `graph_ir` — graph ir (`src/session/compile/compiled.rs:85`; `test-40c21265dcd984747398`).
- `graph_ir_mut` — graph ir mut (`src/session/compile/compiled.rs:90`; `test-21b486e9ca30d96f276e`).

## Related documentation

- [Glossary](/docs/glossary.md)
- [PocketStation](/README.md)
- [Stop a Session and inspect failures](/docs/how-to/stop-session.md)
- [Behavior evidence index](/docs/reference/behavior-evidence.md)
- [Rust API reference](/docs/reference/rust-api.md)
- [A capture source disappears](/docs/troubleshooting/source-loss.md)
- [A recording is incomplete](/docs/troubleshooting/recording-incomplete.md)
- [Session fails before start](/docs/troubleshooting/session-start.md)

## Evidence boundary

This page was verified against Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `src/session/lifecycle/events.rs:1-736` (`DIRECT`)

A file's presence proves implementation or declaration at this snapshot. It does not by itself prove physical-device qualification, operational performance, retry safety, or behavior outside the recorded test conditions.
