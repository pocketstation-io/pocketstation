# Running ownership

<!-- claims: CLM-DOC-039-CAP-001,CLM-DOC-039-CAP-002,CLM-DOC-039-SOURCE-001 -->

## Scope

- **Start, cancel, stop, and finalize a Session.** Coordinate Session startup, rollback, cancellation, steady-state ownership, stop, and terminal outcomes.
- **Observe Session metrics and events.** Read route, source, operator, sidecar, endpoint, drop, latency, queue, and terminal observations.

These statements describe repository contracts at the documented snapshot. They do not extend platform qualification, performance, retry, or delivery guarantees beyond the native API contracts and executable evidence.

## Ownership transition

PocketStation uses distinct declaration, compilation, preparation, running, cancellation, rollback, stop, and terminal-result types where the source exposes them. Do not collapse a stop outcome into a Boolean assumption: component and finalization failures remain structured data.

## Extracted lifecycle operations

| Operation | Trigger | Source state | Destination state | Evidence record |
|---|---|---|---|---|
| `drop_observations` | `drop_observations` | unknown | unknown | `life-033f82e656e133a23c4c` |
| `pocketstation::runtime::lifecycle::sidecar_protocol::SidecarMessageKind::Cancel` | `Cancel` | unknown | unknown | `life-05e66cfb59672fcf3e47` |
| `pocketstation::session::lifecycle::engine::SessionEngineStartError::Prepare` | `Prepare` | unknown | unknown | `life-1e6e9f452ca83bcd4874` |
| `pocketstation::session::extensions::audio_input::buffer::AudioInputBufferAcquireError::Cancelled` | `Cancelled` | unknown | unknown | `life-23ee10c2333375238483` |
| `drop` | `drop` | unknown | unknown | `life-2c74a6091c9cde4045cb` |
| `pocketstation::session::lifecycle::start_contract::SessionStartError::Cancelled` | `Cancelled` | unknown | unknown | `life-396be01d86a31314ead0` |
| `pocketstation::runtime::lifecycle::sidecar_host::SidecarState::Running` | `Running` | unknown | unknown | `life-3ac69529ebf1ff2919b5` |
| `drop_rate_pct` | `drop_rate_pct` | unknown | unknown | `life-4ca80c498146346ca2ad` |
| `SourceDriver::close` | `close` | unknown | unknown | `life-51677582d6bfbf19bf36` |
| `drop_rate_pct` | `drop_rate_pct` | unknown | unknown | `life-6127631a15d75622b3a3` |
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
| `pocketstation::runtime::lifecycle::sidecar_protocol::SidecarMessageKind::Close` | `Close` | unknown | unknown | `life-d2e8d472ee35c4189976` |

## Failure handling

A transition whose guard, idempotence, or recovery is recorded as unknown has no published guarantee here. Preserve the returned error or terminal outcome, inspect component and stage fields, and consult the error reference before retrying.

## Executable evidence

The following test bodies are evidence only for their recorded setup:

- `given_one_branch_when_signal_published_then_receiver_has_exclusive_ownership` — given one branch when signal published then receiver has exclusive ownership (`src/runtime/signal/edge.rs:626`; `test-9674b8f8edebf8590582`).
- `given_connected_gain_plan_when_executed_then_only_connected_nodes_run_and_worker_receives_output` — given connected gain plan when executed then only connected nodes run and worker receives output (`src/runtime/audio/executor.rs:331`; `test-cd64bb966db1f193ea6f`).
- `given_lineaged_frame_when_realtime_operator_executes_then_output_keeps_capture_epochs` — given lineaged frame when realtime operator executes then output keeps capture epochs (`src/runtime/audio/executor.rs:411`; `test-14905efc2e19f82a8cb2`).
- `given_realtime_fan_out_when_executed_then_each_mutating_branch_gets_independent_copy` — given realtime fan out when executed then each mutating branch gets independent copy (`src/runtime/audio/executor.rs:369`; `test-c0c81ff42570a02c1eb9`).
- `observations` — observations (`src/runtime/audio/executor.rs:185`; `test-8e5dda8471ef4129edb9`).
- `from` — from (`src/runtime/audio/router.rs:510`; `test-bd1711e374cc4ec84e26`).
- `given_compiled_text_edge_when_router_builds_then_only_audio_edge_gets_audio_receiver` — given compiled text edge when router builds then only audio edge gets audio receiver (`src/runtime/audio/router.rs:983`; `test-c5f24b62056cfa546c3a`).
- `given_enqueued_and_dropped_frames_when_observed_then_drop_rate_uses_all_attempts` — given enqueued and dropped frames when observed then drop rate uses all attempts (`src/runtime/audio/router.rs:1241`; `test-9a0bb689d2371b66a92f`).
- `given_failed_branch_when_receiver_drops_then_unrelated_branch_continues` — given failed branch when receiver drops then unrelated branch continues (`src/runtime/audio/router.rs:1518`; `test-b5854f13d50d15dfdbe3`).
- `given_foreign_clock_timestamp_when_delivered_then_source_latency_is_not_fabricated` — given foreign clock timestamp when delivered then source latency is not fabricated (`src/runtime/audio/router.rs:1182`; `test-133d3a4b4c11520b3884`).
- `given_lineage_discontinuity_epoch_change_when_received_then_declared_discontinuity_is_counted` — given lineage discontinuity epoch change when received then declared discontinuity is counted (`src/runtime/audio/router.rs:1117`; `test-fceb86228ea42976addb`).
- `given_lineaged_source_fan_out_when_branch_frames_are_copied_then_exact_lineage_is_preserved` — given lineaged source fan out when branch frames are copied then exact lineage is preserved (`src/runtime/audio/router.rs:1076`; `test-d798548d6c8b059ba1a8`).

## Related documentation

- [Glossary](/docs/glossary.md)
- [PocketStation](/README.md)
- [Behavior evidence index](/docs/reference/behavior-evidence.md)
- [Rust API reference](/docs/reference/rust-api.md)
- [Architecture overview](/docs/architecture/overview.md)
- [Build, prepare, and start](/docs/lifecycle/build-prepare-start.md)
- [Cancellation and rollback](/docs/lifecycle/cancellation-and-rollback.md)
- [Observations and metrics](/docs/concepts/observability.md)

## Evidence boundary

This page was verified against Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `src/session/lifecycle/running.rs:1-2612` (`DIRECT`)

A file's presence proves implementation or declaration at this snapshot. It does not by itself prove physical-device qualification, operational performance, retry safety, or behavior outside the recorded test conditions.
