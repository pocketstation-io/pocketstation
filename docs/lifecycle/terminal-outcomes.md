# Terminal outcomes

<!-- claims: CLM-DOC-042-SCOPE-001,CLM-DOC-042-TEXT-001,CLM-DOC-042-TEXT-002,CLM-DOC-042-SOURCE-001 -->

## Scope

- **Start, cancel, stop, and finalize a Session.** Coordinate Session startup, rollback, cancellation, steady-state ownership, stop, and terminal outcomes.
- **Classify public failures.** Expose stable typed errors and cross-boundary error codes without inferring retry or recovery guarantees.

The scope of **Terminal outcomes** ends at the native contracts and executable conditions cited below. Platform qualification, performance, retry, and delivery require their own explicit evidence.

## Ownership transition

For **Terminal outcomes**, PocketStation keeps the declaration, compilation, preparation, running, cancellation, rollback, stop, and terminal-result types exposed by the source distinct. Do not collapse its stop outcome into a Boolean assumption: component and finalization failures remain structured data.

## Extracted lifecycle operations

| Operation | Trigger | Source state | Destination state | Evidence record |
|---|---|---|---|---|
| `drop` | `drop` | owned_or_running | closed_or_released | `life-0eb9c8d45523705c071c` |
| `drop_rate_pct` | `drop_rate_pct` | owned_or_running | closed_or_released | `life-1a59542c3bc90997a7e9` |
| `pocketstation::session::prepare::prepare_session_runtime` | `prepare_session_runtime` | constructed_before_preparation | prepared_or_prepare_failed | `life-1ff9bd824b35e4bd86a6` |
| `SourceDriver::close` | `close` | owned_or_running | closed_or_released | `life-24ac3bb4851827ee31fe` |
| `start_compiled_cancellable` | `start_compiled_cancellable` | prepared | running_or_start_failed | `life-26d46ffab4e997084bfb` |
| `start` | `start` | prepared | running_or_start_failed | `life-27d61a4d665450e563d4` |
| `pocketstation::session::lifecycle::running::start_prepared_session` | `start_prepared_session` | constructed_before_preparation | prepared_or_prepare_failed | `life-2ad92f34c9423181c5be` |
| `close` | `close` | owned_or_running | closed_or_released | `life-31fcaf8e4ff4dc364909` |
| `drop` | `drop` | owned_or_running | closed_or_released | `life-45188ad8c6a8dde207aa` |
| `start_failure` | `start_failure` | prepared | running_or_start_failed | `life-4c39dbc19bad333c2912` |
| `cancel` | `cancel` | preparing_or_running | cancellation_requested_or_cancelled | `life-5a4d79d60befb5c0de67` |
| `drop` | `drop` | owned_or_running | closed_or_released | `life-5fb7a5014d6ffc5d4352` |
| `SourceDriver::prepare` | `prepare` | constructed_before_preparation | prepared_or_prepare_failed | `life-6a6429930d687e6fac25` |
| `start_compiled` | `start_compiled` | prepared | running_or_start_failed | `life-6e9224a91062bb5ec713` |
| `start_compiled_cancellable` | `start_compiled_cancellable` | prepared | running_or_start_failed | `life-7e2e74712c52616d3c2c` |
| `drop_observations` | `drop_observations` | owned_or_running | closed_or_released | `life-87afb6dffef4ad5765d6` |
| `prepare_session` | `prepare_session` | constructed_before_preparation | prepared_or_prepare_failed | `life-8871b96155ed0020e00c` |
| `start_compiled` | `start_compiled` | prepared | running_or_start_failed | `life-8b50910aa645c7069ac3` |
| `stop` | `stop` | owning_state_before_operation | owning_state_after_returned_outcome | `life-8bb2a23671da67e30c1b` |
| `start` | `start` | prepared | running_or_start_failed | `life-96cd11312a6f1461a2ca` |
| `cancel` | `cancel` | preparing_or_running | cancellation_requested_or_cancelled | `life-977aafa66864957cfcb4` |
| `drop` | `drop` | owned_or_running | closed_or_released | `life-a66c19cea3372c7a33d8` |
| `pocketstation::session::lifecycle::running::start_prepared_session_cancellable` | `start_prepared_session_cancellable` | constructed_before_preparation | prepared_or_prepare_failed | `life-a8706f56832ee13271c9` |
| `join` | `join` | stopping_or_completed | terminal | `life-b5f9ab974b55c8f4b6c2` |
| `start` | `start` | prepared | running_or_start_failed | `life-b76642f17aa575a0757e` |
| `close` | `close` | owned_or_running | closed_or_released | `life-b9b46198766a5efb68d8` |
| `start` | `start` | prepared | running_or_start_failed | `life-e4d2a447d0f97df30948` |
| `prepare` | `prepare` | constructed_before_preparation | prepared_or_prepare_failed | `life-f2d6afa474fb10373eb8` |
| `prepare_context` | `prepare_context` | constructed_before_preparation | prepared_or_prepare_failed | `life-f4c3e7122fc0dc984668` |

## Failure handling

Within **Terminal outcomes**, a transition whose guard, idempotence, or recovery is recorded as not declared has no published guarantee. Preserve its returned error or terminal outcome, inspect component and stage fields, and consult the error reference before retrying.

## Executable evidence

Executable evidence selected for **Terminal outcomes** is limited to each test's recorded setup and assertions:

- `given_all_failure_classes_when_terminal_then_each_class_is_preserved` — given all failure classes when terminal then each class is preserved (`src/session/lifecycle/events.rs:679`; `test-391bc055ecec6559c5ee`).
- `given_no_failures_when_terminal_then_state_is_stopped` — given no failures when terminal then state is stopped (`src/session/lifecycle/events.rs:721`; `test-74456aeb5a4f8bda5b30`).
- `given_all_senders_dropped_when_polled_then_receiver_reports_closed` — given all senders dropped when polled then receiver reports closed (`src/session/lifecycle/events.rs:656`; `test-869cc16c477444c9b6fd`).
- `given_closed_receiver_when_publishing_then_drop_and_closure_are_counted` — given closed receiver when publishing then drop and closure are counted (`src/session/lifecycle/events.rs:664`; `test-a18e7b48053bd0288795`).
- `given_events_when_polled_then_fifo_order_and_depth_are_preserved` — given events when polled then fifo order and depth are preserved (`src/session/lifecycle/events.rs:632`; `test-5346edce6d9a6da2069d`).
- `given_full_queue_when_publishing_then_newest_event_is_dropped_and_counted` — given full queue when publishing then newest event is dropped and counted (`src/session/lifecycle/events.rs:572`; `test-50e2cd9cdbc18d041989`).
- `given_oversized_session_event_when_published_then_queue_owned_memory_stays_bounded` — given oversized session event when published then queue owned memory stays bounded (`src/session/lifecycle/events.rs:608`; `test-9e75be4a362fd68c5951`).
- `given_complete_trace_when_validated_then_lifecycle_and_terminal_match` — given complete trace when validated then lifecycle and terminal match (`src/session/lifecycle/trace.rs:988`; `test-20c537693a458a99f6f9`).
- `given_record_after_terminal_when_validated_then_trace_is_rejected` — given record after terminal when validated then trace is rejected (`src/session/lifecycle/trace.rs:1160`; `test-3d007d4a819d105d9058`).
- `given_compiled_derived_route_when_runtime_prepared_then_compiled_topology_is_preserved` — given compiled derived route when runtime prepared then compiled topology is preserved (`src/session/compile/tests.rs:659`; `test-21f8c08b6457bb762def`).
- `given_derived_stream_chain_when_compiled_then_operator_output_feeds_next_named_input` — given derived stream chain when compiled then operator output feeds next named input (`src/session/compile/tests.rs:587`; `test-081f9254eabd3bfeaad1`).
- `given_exact_process_instance_when_lowered_then_typed_declaration_remains_authoritative` — given exact process instance when lowered then typed declaration remains authoritative (`src/session/compile/tests.rs:803`; `test-b731381bbb6154df587d`).

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

The claims on **Terminal outcomes** are anchored to Git snapshot `136e74888962558aa846d3143a19136a70936f45` and these primary files:

- `src/session/lifecycle/events.rs:15-15` (`DIRECT`)
- `src/session/lifecycle/events.rs:18-18` (`DIRECT`)
- `src/session/lifecycle/events.rs:18-18` (`DIRECT`)
- `src/session/lifecycle/events.rs:18-18` (`DIRECT`)
- `src/session/lifecycle/events.rs:19-25` (`DIRECT`)
- `src/session/lifecycle/events.rs:20-20` (`DIRECT`)
- `src/session/lifecycle/events.rs:21-21` (`DIRECT`)
- `src/session/lifecycle/events.rs:22-22` (`DIRECT`)
- `src/session/lifecycle/events.rs:23-23` (`DIRECT`)
- `src/session/lifecycle/events.rs:24-24` (`DIRECT`)
- `src/session/lifecycle/events.rs:28-28` (`DIRECT`)
- `src/session/lifecycle/events.rs:28-28` (`DIRECT`)
- `src/session/lifecycle/events.rs:28-28` (`DIRECT`)
- `src/session/lifecycle/events.rs:29-35` (`DIRECT`)
- `src/session/lifecycle/events.rs:30-30` (`DIRECT`)
- `src/session/lifecycle/events.rs:31-31` (`DIRECT`)
- `src/session/lifecycle/events.rs:32-32` (`DIRECT`)
- `src/session/lifecycle/events.rs:33-33` (`DIRECT`)
- `src/session/lifecycle/events.rs:34-34` (`DIRECT`)
- `src/session/lifecycle/events.rs:38-38` (`DIRECT`)
- `src/session/lifecycle/events.rs:38-38` (`DIRECT`)
- `src/session/lifecycle/events.rs:38-38` (`DIRECT`)
- `src/session/lifecycle/events.rs:39-47` (`DIRECT`)
- `src/session/lifecycle/events.rs:40-40` (`DIRECT`)

For **Terminal outcomes**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.
