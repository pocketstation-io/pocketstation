# Running ownership

<!-- claims: CLM-DOC-039-CAP-001,CLM-DOC-039-CAP-002,CLM-DOC-039-SOURCE-001 -->

## Scope

- **Start, cancel, stop, and finalize a Session.** Coordinate Session startup, rollback, cancellation, steady-state ownership, stop, and terminal outcomes.
- **Observe Session metrics and events.** Read route, source, operator, sidecar, endpoint, drop, latency, queue, and terminal observations.

The scope of **Running ownership** ends at the native contracts and executable conditions cited below. Platform qualification, performance, retry, and delivery require their own explicit evidence.

## Ownership transition

For **Running ownership**, PocketStation keeps the declaration, compilation, preparation, running, cancellation, rollback, stop, and terminal-result types exposed by the source distinct. Do not collapse its stop outcome into a Boolean assumption: component and finalization failures remain structured data.

## Extracted lifecycle operations

| Operation | Trigger | Source state | Destination state | Evidence record |
|---|---|---|---|---|
| `pocketstation::session::lifecycle::observations::EndpointObservationStage::Finalized` | `Finalized` | stopping_or_completed | terminal | `life-079ba79743246d62b02e` |
| `pocketstation::session::lifecycle::events::SessionLifecycleState::Stopped` | `Stopped` | state_declared_by_owning_type | state_returned_by_owning_operation | `life-0c83bede0af1826f5b6a` |
| `drop` | `drop` | owned_or_running | closed_or_released | `life-0eb9c8d45523705c071c` |
| `pocketstation::session::lifecycle::control::SessionStartError::Cancelled` | `Cancelled` | prepared | running_or_start_failed | `life-14367931dacc2ea6803e` |
| `drop_rate_pct` | `drop_rate_pct` | owned_or_running | closed_or_released | `life-1a59542c3bc90997a7e9` |
| `cancel_and_join` | `cancel_and_join` | preparing_or_running | cancellation_requested_or_cancelled | `life-1b6e541aba4cc6f4cdfb` |
| `pocketstation::session::prepare::prepare_session_runtime` | `prepare_session_runtime` | declared_or_compiled | prepared_or_prepare_failed | `life-1ff9bd824b35e4bd86a6` |
| `close_and_reap` | `close_and_reap` | owned_or_running | closed_or_released | `life-227c49c48f136eb6305f` |
| `SourceDriver::close` | `close` | owned_or_running | closed_or_released | `life-24ac3bb4851827ee31fe` |
| `start_compiled_cancellable` | `start_compiled_cancellable` | prepared | running_or_start_failed | `life-26d46ffab4e997084bfb` |
| `pocketstation::session::extensions::audio_input::buffer::AudioInputBufferAcquireError::Cancelled` | `Cancelled` | preparing_or_running | cancellation_requested_or_cancelled | `life-277b4455fde06fca7361` |
| `start` | `start` | prepared | running_or_start_failed | `life-27d61a4d665450e563d4` |
| `pocketstation::runtime::lifecycle::async_host::AsyncRuntimeHostError::Start` | `Start` | prepared | running_or_start_failed | `life-28b94670f06967d2fccc` |
| `pocketstation::session::lifecycle::running::start_prepared_session` | `start_prepared_session` | declared_or_compiled | prepared_or_prepare_failed | `life-2ad92f34c9423181c5be` |
| `close` | `close` | owned_or_running | closed_or_released | `life-31fcaf8e4ff4dc364909` |
| `pocketstation::runtime::lifecycle::sidecar_protocol::SidecarMessageKind::Cancel` | `Cancel` | preparing_or_running | cancellation_requested_or_cancelled | `life-361d2b8d8a09134c7799` |
| `drop` | `drop` | owned_or_running | closed_or_released | `life-45188ad8c6a8dde207aa` |
| `prepare` | `prepare` | declared_or_compiled | prepared_or_prepare_failed | `life-456533bd456a4ee124a1` |
| `drop` | `drop` | owned_or_running | closed_or_released | `life-4b2edb4fb23deb5bd0b4` |
| `start_failure` | `start_failure` | prepared | running_or_start_failed | `life-4c39dbc19bad333c2912` |
| `drop` | `drop` | owned_or_running | closed_or_released | `life-53a5c0c438b0534a2964` |
| `drop_rate_pct` | `drop_rate_pct` | owned_or_running | closed_or_released | `life-572fd1fca97f08fe449b` |
| `cancel` | `cancel` | preparing_or_running | cancellation_requested_or_cancelled | `life-5a4d79d60befb5c0de67` |
| `pocketstation::session::lifecycle::engine::SessionEngineStartError::Prepare` | `Prepare` | declared_or_compiled | prepared_or_prepare_failed | `life-5cdc4329bb39da70e765` |
| `drop` | `drop` | owned_or_running | closed_or_released | `life-5fb7a5014d6ffc5d4352` |
| `prepare_and_spawn_from_plan_edge` | `prepare_and_spawn_from_plan_edge` | declared_or_compiled | prepared_or_prepare_failed | `life-62a217a84804d5de5e4c` |
| `pocketstation::session::lifecycle::events::SessionLifecycleState::Running` | `Running` | prepared | running_or_terminal | `life-68c28ff7b438b8f11879` |
| `SourceDriver::prepare` | `prepare` | declared_or_compiled | prepared_or_prepare_failed | `life-6a6429930d687e6fac25` |
| `start_compiled` | `start_compiled` | prepared | running_or_start_failed | `life-6e9224a91062bb5ec713` |
| `pocketstation::runtime::lifecycle::sidecar_protocol::SidecarMessageKind::Close` | `Close` | owned_or_running | closed_or_released | `life-720bc3672572da603047` |
| `pocketstation::runtime::audio::runner::PlanSourceSendError::Cancelled` | `Cancelled` | preparing_or_running | cancellation_requested_or_cancelled | `life-78031521ad3200204b7a` |
| `start_compiled_cancellable` | `start_compiled_cancellable` | prepared | running_or_start_failed | `life-7e2e74712c52616d3c2c` |
| `pocketstation::session::error_code::SessionStopCode::Stopped` | `Stopped` | state_declared_by_owning_type | state_returned_by_owning_operation | `life-80e6f0ced6a8a0e4d47b` |
| `drop_observations` | `drop_observations` | owned_or_running | closed_or_released | `life-87afb6dffef4ad5765d6` |
| `prepare_session` | `prepare_session` | declared_or_compiled | prepared_or_prepare_failed | `life-8871b96155ed0020e00c` |
| `start_compiled` | `start_compiled` | prepared | running_or_start_failed | `life-8b50910aa645c7069ac3` |
| `stop` | `stop` | state_declared_by_owning_type | state_returned_by_owning_operation | `life-8bb2a23671da67e30c1b` |
| `pocketstation::runtime::signal::error::AsyncOperatorWorkerError::Join` | `Join` | stopping_or_completed | terminal | `life-93b4a56b42d1f44497bf` |
| `pocketstation::runtime::signal::error::AsyncOperatorWorkerError::Cancel` | `Cancel` | preparing_or_running | cancellation_requested_or_cancelled | `life-9472a597b8db5b6c3d01` |
| `start` | `start` | prepared | running_or_start_failed | `life-96cd11312a6f1461a2ca` |
| `cancel` | `cancel` | preparing_or_running | cancellation_requested_or_cancelled | `life-977aafa66864957cfcb4` |
| `shutdown` | `shutdown` | state_declared_by_owning_type | state_returned_by_owning_operation | `life-9be9144874aff9e686bd` |
| `pocketstation::runtime::signal::error::AsyncOperatorWorkerError::Prepare` | `Prepare` | declared_or_compiled | prepared_or_prepare_failed | `life-a08adc009d9bab5b3fcf` |
| `drop` | `drop` | owned_or_running | closed_or_released | `life-a66c19cea3372c7a33d8` |
| `pocketstation::session::lifecycle::running::start_prepared_session_cancellable` | `start_prepared_session_cancellable` | declared_or_compiled | prepared_or_prepare_failed | `life-a8706f56832ee13271c9` |
| `cancel_and_join` | `cancel_and_join` | preparing_or_running | cancellation_requested_or_cancelled | `life-ab1d9f74cf7c8203e8b4` |
| `pocketstation::session::lifecycle::events::SessionTerminalState::Stopped` | `Stopped` | state_declared_by_owning_type | state_returned_by_owning_operation | `life-b5ed04fa91e839c293be` |
| `join` | `join` | stopping_or_completed | terminal | `life-b5f9ab974b55c8f4b6c2` |
| `pocketstation::session::extensions::audio_input::buffer::AudioInputWriteErrorKind::Cancelled` | `Cancelled` | preparing_or_running | cancellation_requested_or_cancelled | `life-b65b53dd882678e7685b` |
| `start` | `start` | prepared | running_or_start_failed | `life-b76642f17aa575a0757e` |
| `close` | `close` | owned_or_running | closed_or_released | `life-b9b46198766a5efb68d8` |
| `drop` | `drop` | owned_or_running | closed_or_released | `life-b9c3e073905015b85207` |
| `pocketstation::session::lifecycle::engine::SessionEngineStartError::Start` | `Start` | prepared | running_or_start_failed | `life-cb5a740e84f51401ac95` |
| `cancel_and_reap` | `cancel_and_reap` | preparing_or_running | cancellation_requested_or_cancelled | `life-e16c2f3cc3e18a861647` |
| `pocketstation::runtime::lifecycle::sidecar_host::SidecarState::Running` | `Running` | prepared | running_or_terminal | `life-e3721811066c7fdb3dd7` |
| `start` | `start` | prepared | running_or_start_failed | `life-e4d2a447d0f97df30948` |
| `pocketstation::runtime::signal::error::AsyncOperatorWorkerError::Close` | `Close` | owned_or_running | closed_or_released | `life-ea7544a0b40b7b99d4c0` |
| `prepare` | `prepare` | declared_or_compiled | prepared_or_prepare_failed | `life-f2d6afa474fb10373eb8` |
| `drop` | `drop` | owned_or_running | closed_or_released | `life-f39c0d7ab0cd31705e26` |
| `prepare_context` | `prepare_context` | declared_or_compiled | prepared_or_prepare_failed | `life-f4c3e7122fc0dc984668` |

## Failure handling

Within **Running ownership**, a transition whose guard, idempotence, or recovery is recorded as not declared has no published guarantee. Preserve its returned error or terminal outcome, inspect component and stage fields, and consult the error reference before retrying.

## Executable evidence

Executable evidence selected for **Running ownership** is limited to each test's recorded setup and assertions:

- `given_process_instance_selector_when_capture_mode_built_then_exact_identity_is_preserved` — given process instance selector when capture mode built then exact identity is preserved (`src/session/lifecycle/running.rs:2602`; `test-284127121760cbb5874f`).
- `given_one_branch_when_signal_published_then_receiver_has_exclusive_ownership` — given one branch when signal published then receiver has exclusive ownership (`src/runtime/signal/edge.rs:626`; `test-ff5044918a12088e3cc1`).
- `given_connected_gain_plan_when_executed_then_only_connected_nodes_run_and_worker_receives_output` — given connected gain plan when executed then only connected nodes run and worker receives output (`src/runtime/audio/executor.rs:331`; `test-3f9281677e5af26dc9ad`).
- `given_lineaged_frame_when_realtime_operator_executes_then_output_keeps_capture_epochs` — given lineaged frame when realtime operator executes then output keeps capture epochs (`src/runtime/audio/executor.rs:411`; `test-aee462488aef78361374`).
- `given_realtime_fan_out_when_executed_then_each_mutating_branch_gets_independent_copy` — given realtime fan out when executed then each mutating branch gets independent copy (`src/runtime/audio/executor.rs:369`; `test-8b303620bdafeb3aa260`).
- `given_compiled_text_edge_when_router_builds_then_only_audio_edge_gets_audio_receiver` — given compiled text edge when router builds then only audio edge gets audio receiver (`src/runtime/audio/router.rs:1012`; `test-687c08c4ebc7699d891b`).
- `given_enqueued_and_dropped_frames_when_observed_then_drop_rate_uses_all_attempts` — given enqueued and dropped frames when observed then drop rate uses all attempts (`src/runtime/audio/router.rs:1272`; `test-81f2a37c65fc1321fb4b`).
- `given_failed_branch_when_receiver_drops_then_unrelated_branch_continues` — given failed branch when receiver drops then unrelated branch continues (`src/runtime/audio/router.rs:1549`; `test-e79727ff2a1d9faecc74`).
- `given_foreign_clock_timestamp_when_delivered_then_source_latency_is_not_fabricated` — given foreign clock timestamp when delivered then source latency is not fabricated (`src/runtime/audio/router.rs:1211`; `test-2dfdf77222ba4754d494`).
- `given_lineage_discontinuity_epoch_change_when_received_then_declared_discontinuity_is_counted` — given lineage discontinuity epoch change when received then declared discontinuity is counted (`src/runtime/audio/router.rs:1146`; `test-6161d7a8c36359a8e55e`).
- `given_lineaged_source_fan_out_when_branch_frames_are_copied_then_exact_lineage_is_preserved` — given lineaged source fan out when branch frames are copied then exact lineage is preserved (`src/runtime/audio/router.rs:1105`; `test-8407a37a5b957f010ddf`).
- `given_observation_handle_when_consumer_detects_gap_then_live_discontinuity_is_visible` — given observation handle when consumer detects gap then live discontinuity is visible (`src/runtime/audio/router.rs:1441`; `test-b11b6230db89e523d9d4`).

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

The claims on **Running ownership** are anchored to Git snapshot `136e74888962558aa846d3143a19136a70936f45` and these primary files:

- `src/session/lifecycle/running.rs:1-2625` (`DIRECT`)

For **Running ownership**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.
