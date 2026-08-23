# Terminal outcomes

<!-- claims: CLM-DOC-042-CAP-001,CLM-DOC-042-CAP-002,CLM-DOC-042-SOURCE-001 -->

## Scope

- **Start, cancel, stop, and finalize a Session.** Coordinate Session startup, rollback, cancellation, steady-state ownership, stop, and terminal outcomes.
- **Classify public failures.** Expose stable typed errors and cross-boundary error codes without inferring retry or recovery guarantees.

The scope of **Terminal outcomes** ends at the native contracts and executable conditions cited below. Platform qualification, performance, retry, and delivery require their own explicit evidence.

## Ownership transition

For **Terminal outcomes**, PocketStation keeps the declaration, compilation, preparation, running, cancellation, rollback, stop, and terminal-result types exposed by the source distinct. Do not collapse its stop outcome into a Boolean assumption: component and finalization failures remain structured data.

## Extracted lifecycle operations

| Operation | Trigger | Source state | Destination state | Evidence record |
|---|---|---|---|---|
| `join` | `join` | stopping_or_completed | terminal | `life-05d0b22aacaf35a3ff7f` |
| `close` | `close` | owned_or_running | closed_or_released | `life-072d37f373f0c2a2db44` |
| `pocketstation::session::lifecycle::running::start_prepared_session` | `start_prepared_session` | declared_or_compiled | prepared_or_prepare_failed | `life-0d33bc06c15c8f36db67` |
| `start` | `start` | prepared | running_or_start_failed | `life-1695d590bdb58923755e` |
| `prepare_session` | `prepare_session` | declared_or_compiled | prepared_or_prepare_failed | `life-17262c72ad57bde7091d` |
| `pocketstation::session::lifecycle::engine::SessionEngineStartError::Prepare` | `Prepare` | declared_or_compiled | prepared_or_prepare_failed | `life-3171793f782cee3c5b77` |
| `start_failure` | `start_failure` | prepared | running_or_start_failed | `life-325781a0a47d85d28f37` |
| `cancel` | `cancel` | preparing_or_running | cancellation_requested_or_cancelled | `life-37895c4b927a1e2c20bd` |
| `drop_observations` | `drop_observations` | owned_or_running | closed_or_released | `life-3c1a60a96f386b0c2233` |
| `drop` | `drop` | owned_or_running | closed_or_released | `life-44a40cbf57416c570e19` |
| `pocketstation::session::prepare::prepare_session_runtime` | `prepare_session_runtime` | declared_or_compiled | prepared_or_prepare_failed | `life-4788465795aa5b459d72` |
| `drop` | `drop` | owned_or_running | closed_or_released | `life-55d2a5f2ad91927f2c84` |
| `start_compiled` | `start_compiled` | prepared | running_or_start_failed | `life-68e51aae7a58503e68ec` |
| `pocketstation::session::lifecycle::events::SessionLifecycleState::Stopped` | `Stopped` | state_declared_by_owning_type | state_returned_by_owning_operation | `life-69a6aff4df2be3f84131` |
| `pocketstation::session::lifecycle::events::SessionTerminalState::Stopped` | `Stopped` | state_declared_by_owning_type | state_returned_by_owning_operation | `life-6a218d0b344e24bcf2b3` |
| `start` | `start` | prepared | running_or_start_failed | `life-6dfd4a8b6248d8918571` |
| `SourceDriver::close` | `close` | owned_or_running | closed_or_released | `life-78c6b2ef32d95e6ad82c` |
| `start_compiled_cancellable` | `start_compiled_cancellable` | prepared | running_or_start_failed | `life-7b1469dcc46309d2d629` |
| `prepare` | `prepare` | declared_or_compiled | prepared_or_prepare_failed | `life-8290bb198180554201c6` |
| `cancel` | `cancel` | preparing_or_running | cancellation_requested_or_cancelled | `life-84814636a2bc2fd27250` |
| `drop` | `drop` | owned_or_running | closed_or_released | `life-872f31ae0091d843e07d` |
| `close` | `close` | owned_or_running | closed_or_released | `life-990eedf83476e0e861be` |
| `start_compiled_cancellable` | `start_compiled_cancellable` | prepared | running_or_start_failed | `life-997609314cf808402756` |
| `start` | `start` | prepared | running_or_start_failed | `life-9e8c247c9bcac897aa32` |
| `pocketstation::session::extensions::audio_input::buffer::AudioInputBufferAcquireError::Cancelled` | `Cancelled` | preparing_or_running | cancellation_requested_or_cancelled | `life-b7aa4ebc1860ffa7ee22` |
| `start_compiled` | `start_compiled` | prepared | running_or_start_failed | `life-bb6b635326c31b017c27` |
| `pocketstation::session::extensions::audio_input::buffer::AudioInputWriteErrorKind::Cancelled` | `Cancelled` | preparing_or_running | cancellation_requested_or_cancelled | `life-bce2897a917246800a2c` |
| `pocketstation::session::lifecycle::events::SessionLifecycleState::Running` | `Running` | prepared | running_or_terminal | `life-c25444e4f9cd62e3ad01` |
| `pocketstation::session::lifecycle::running::start_prepared_session_cancellable` | `start_prepared_session_cancellable` | declared_or_compiled | prepared_or_prepare_failed | `life-cad3bede2bc1d9dfdadd` |
| `start` | `start` | prepared | running_or_start_failed | `life-ce1437749f3c39aea285` |
| `prepare_context` | `prepare_context` | declared_or_compiled | prepared_or_prepare_failed | `life-d11d5649023858ebf3d2` |
| `stop` | `stop` | state_declared_by_owning_type | state_returned_by_owning_operation | `life-d6ffb4e55b1f64a3a73d` |
| `pocketstation::session::error_code::SessionStopCode::Stopped` | `Stopped` | state_declared_by_owning_type | state_returned_by_owning_operation | `life-e0bd45ab7651528422f4` |
| `drop` | `drop` | owned_or_running | closed_or_released | `life-e48ae7dd511c2ee65dec` |
| `pocketstation::session::lifecycle::start_contract::SessionStartError::Cancelled` | `Cancelled` | prepared | running_or_start_failed | `life-e85546ef287c5d8b1a10` |
| `SourceDriver::prepare` | `prepare` | declared_or_compiled | prepared_or_prepare_failed | `life-e9b55238f265a7f63da2` |
| `pocketstation::session::lifecycle::engine::SessionEngineStartError::Start` | `Start` | prepared | running_or_start_failed | `life-f81b9d7345fea4f92fd6` |
| `pocketstation::session::lifecycle::observations::EndpointObservationStage::Finalized` | `Finalized` | stopping_or_completed | terminal | `life-f98c28a0a56c89e4ba80` |
| `drop_rate_pct` | `drop_rate_pct` | owned_or_running | closed_or_released | `life-fe3ec5c72e581c7d44d5` |

## Failure handling

Within **Terminal outcomes**, a transition whose guard, idempotence, or recovery is recorded as not declared has no published guarantee. Preserve its returned error or terminal outcome, inspect component and stage fields, and consult the error reference before retrying.

## Executable evidence

Executable evidence selected for **Terminal outcomes** is limited to each test's recorded setup and assertions:

- `given_all_failure_classes_when_terminal_then_each_class_is_preserved` — given all failure classes when terminal then each class is preserved (`src/session/lifecycle/events.rs:679`; `test-070a5fc90aa90f7d986a`).
- `given_no_failures_when_terminal_then_state_is_stopped` — given no failures when terminal then state is stopped (`src/session/lifecycle/events.rs:721`; `test-6dd97c870cc349f825f9`).
- `given_all_senders_dropped_when_polled_then_receiver_reports_closed` — given all senders dropped when polled then receiver reports closed (`src/session/lifecycle/events.rs:656`; `test-ed28bb41869db2c16ec2`).
- `given_closed_receiver_when_publishing_then_drop_and_closure_are_counted` — given closed receiver when publishing then drop and closure are counted (`src/session/lifecycle/events.rs:664`; `test-14b780bec81ca78fbbe6`).
- `given_events_when_polled_then_fifo_order_and_depth_are_preserved` — given events when polled then fifo order and depth are preserved (`src/session/lifecycle/events.rs:632`; `test-ef7a999d2659ba14b610`).
- `given_full_queue_when_publishing_then_newest_event_is_dropped_and_counted` — given full queue when publishing then newest event is dropped and counted (`src/session/lifecycle/events.rs:572`; `test-c6f9da58b0231d87b0b9`).
- `given_oversized_session_event_when_published_then_queue_owned_memory_stays_bounded` — given oversized session event when published then queue owned memory stays bounded (`src/session/lifecycle/events.rs:608`; `test-681a72f40c41938b9b0d`).
- `given_complete_trace_when_validated_then_lifecycle_and_terminal_match` — given complete trace when validated then lifecycle and terminal match (`src/session/lifecycle/trace.rs:988`; `test-9dd947375f94d2a4f21f`).
- `given_record_after_terminal_when_validated_then_trace_is_rejected` — given record after terminal when validated then trace is rejected (`src/session/lifecycle/trace.rs:1160`; `test-13dde755fb2a02648705`).
- `given_compiled_derived_route_when_runtime_prepared_then_compiled_topology_is_preserved` — given compiled derived route when runtime prepared then compiled topology is preserved (`src/session/compile/tests.rs:659`; `test-f38493cc0593f603aece`).
- `given_derived_stream_chain_when_compiled_then_operator_output_feeds_next_named_input` — given derived stream chain when compiled then operator output feeds next named input (`src/session/compile/tests.rs:587`; `test-e9a24a392741b4dbe6e7`).
- `given_exact_process_instance_when_lowered_then_typed_declaration_remains_authoritative` — given exact process instance when lowered then typed declaration remains authoritative (`src/session/compile/tests.rs:803`; `test-43d80b8d3e727ac91d90`).

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

The claims on **Terminal outcomes** are anchored to Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `src/session/lifecycle/events.rs:1-736` (`DIRECT`)

For **Terminal outcomes**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.
