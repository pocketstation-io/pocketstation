# Cancellation and rollback

<!-- claims: CLM-DOC-040-CAP-001,CLM-DOC-040-SOURCE-001 -->

## Scope

- **Start, cancel, stop, and finalize a Session.** Coordinate Session startup, rollback, cancellation, steady-state ownership, stop, and terminal outcomes.

The scope of **Cancellation and rollback** ends at the native contracts and executable conditions cited below. Platform qualification, performance, retry, and delivery require their own explicit evidence.

## Ownership transition

For **Cancellation and rollback**, PocketStation keeps the declaration, compilation, preparation, running, cancellation, rollback, stop, and terminal-result types exposed by the source distinct. Do not collapse its stop outcome into a Boolean assumption: component and finalization failures remain structured data.

## Extracted lifecycle operations

| Operation | Trigger | Source state | Destination state | Evidence record |
|---|---|---|---|---|
| `cancel_and_reap` | `cancel_and_reap` | preparing_or_running | cancellation_requested_or_cancelled | `life-033b98da937709d4b2b0` |
| `join` | `join` | stopping_or_completed | terminal | `life-05d0b22aacaf35a3ff7f` |
| `close` | `close` | owned_or_running | closed_or_released | `life-072d37f373f0c2a2db44` |
| `pocketstation::session::lifecycle::running::start_prepared_session` | `start_prepared_session` | declared_or_compiled | prepared_or_prepare_failed | `life-0d33bc06c15c8f36db67` |
| `pocketstation::runtime::lifecycle::sidecar_protocol::SidecarMessageKind::Cancel` | `Cancel` | preparing_or_running | cancellation_requested_or_cancelled | `life-10551e3b62fb7f772613` |
| `start` | `start` | prepared | running_or_start_failed | `life-1695d590bdb58923755e` |
| `prepare_session` | `prepare_session` | declared_or_compiled | prepared_or_prepare_failed | `life-17262c72ad57bde7091d` |
| `drop` | `drop` | owned_or_running | closed_or_released | `life-1c7816652dd061fb1141` |
| `close_and_reap` | `close_and_reap` | owned_or_running | closed_or_released | `life-2a340a7ae69b2c18f4f3` |
| `pocketstation::runtime::lifecycle::sidecar_host::SidecarState::Running` | `Running` | prepared | running_or_terminal | `life-2d4ebaa478c36896b51d` |
| `pocketstation::session::lifecycle::engine::SessionEngineStartError::Prepare` | `Prepare` | declared_or_compiled | prepared_or_prepare_failed | `life-3171793f782cee3c5b77` |
| `start_failure` | `start_failure` | prepared | running_or_start_failed | `life-325781a0a47d85d28f37` |
| `prepare` | `prepare` | declared_or_compiled | prepared_or_prepare_failed | `life-3691d2e1f8efde5ac0a0` |
| `cancel` | `cancel` | preparing_or_running | cancellation_requested_or_cancelled | `life-37895c4b927a1e2c20bd` |
| `cancel_and_join` | `cancel_and_join` | preparing_or_running | cancellation_requested_or_cancelled | `life-3acf2ecfada4977ce7df` |
| `drop_observations` | `drop_observations` | owned_or_running | closed_or_released | `life-3c1a60a96f386b0c2233` |
| `drop_rate_pct` | `drop_rate_pct` | owned_or_running | closed_or_released | `life-449ed8a22cf03f904d39` |
| `drop` | `drop` | owned_or_running | closed_or_released | `life-44a40cbf57416c570e19` |
| `drop` | `drop` | owned_or_running | closed_or_released | `life-44d8a960be8c4f4eba48` |
| `pocketstation::session::prepare::prepare_session_runtime` | `prepare_session_runtime` | declared_or_compiled | prepared_or_prepare_failed | `life-4788465795aa5b459d72` |
| `drop` | `drop` | owned_or_running | closed_or_released | `life-55d2a5f2ad91927f2c84` |
| `shutdown` | `shutdown` | state_declared_by_owning_type | state_returned_by_owning_operation | `life-57c87f8d848ba4083d83` |
| `pocketstation::runtime::lifecycle::async_host::AsyncRuntimeHostError::Start` | `Start` | prepared | running_or_start_failed | `life-58786e40f7899fab0a36` |
| `prepare_and_spawn_from_plan_edge` | `prepare_and_spawn_from_plan_edge` | declared_or_compiled | prepared_or_prepare_failed | `life-5aa27838dcaa21b67918` |
| `start_compiled` | `start_compiled` | prepared | running_or_start_failed | `life-68e51aae7a58503e68ec` |
| `pocketstation::session::lifecycle::events::SessionLifecycleState::Stopped` | `Stopped` | state_declared_by_owning_type | state_returned_by_owning_operation | `life-69a6aff4df2be3f84131` |
| `pocketstation::session::lifecycle::events::SessionTerminalState::Stopped` | `Stopped` | state_declared_by_owning_type | state_returned_by_owning_operation | `life-6a218d0b344e24bcf2b3` |
| `start` | `start` | prepared | running_or_start_failed | `life-6dfd4a8b6248d8918571` |
| `SourceDriver::close` | `close` | owned_or_running | closed_or_released | `life-78c6b2ef32d95e6ad82c` |
| `start_compiled_cancellable` | `start_compiled_cancellable` | prepared | running_or_start_failed | `life-7b1469dcc46309d2d629` |
| `cancel_and_join` | `cancel_and_join` | preparing_or_running | cancellation_requested_or_cancelled | `life-7e3c74efe4ca679fb089` |
| `drop` | `drop` | owned_or_running | closed_or_released | `life-7f4be68921e6d8b9cb92` |
| `prepare` | `prepare` | declared_or_compiled | prepared_or_prepare_failed | `life-8290bb198180554201c6` |
| `cancel` | `cancel` | preparing_or_running | cancellation_requested_or_cancelled | `life-84814636a2bc2fd27250` |
| `drop` | `drop` | owned_or_running | closed_or_released | `life-872f31ae0091d843e07d` |
| `pocketstation::runtime::lifecycle::sidecar_protocol::SidecarMessageKind::Close` | `Close` | owned_or_running | closed_or_released | `life-94fdf744a545aa18f84d` |
| `close` | `close` | owned_or_running | closed_or_released | `life-990eedf83476e0e861be` |
| `start_compiled_cancellable` | `start_compiled_cancellable` | prepared | running_or_start_failed | `life-997609314cf808402756` |
| `start` | `start` | prepared | running_or_start_failed | `life-9e8c247c9bcac897aa32` |
| `pocketstation::runtime::signal::error::AsyncOperatorWorkerError::Cancel` | `Cancel` | preparing_or_running | cancellation_requested_or_cancelled | `life-afba705f6d4e82a18998` |
| `pocketstation::runtime::audio::runner::PlanSourceSendError::Cancelled` | `Cancelled` | preparing_or_running | cancellation_requested_or_cancelled | `life-b2a66fd10eb50f205b98` |
| `pocketstation::session::extensions::audio_input::buffer::AudioInputBufferAcquireError::Cancelled` | `Cancelled` | preparing_or_running | cancellation_requested_or_cancelled | `life-b7aa4ebc1860ffa7ee22` |
| `start_compiled` | `start_compiled` | prepared | running_or_start_failed | `life-bb6b635326c31b017c27` |
| `pocketstation::session::extensions::audio_input::buffer::AudioInputWriteErrorKind::Cancelled` | `Cancelled` | preparing_or_running | cancellation_requested_or_cancelled | `life-bce2897a917246800a2c` |
| `pocketstation::runtime::signal::error::AsyncOperatorWorkerError::Prepare` | `Prepare` | declared_or_compiled | prepared_or_prepare_failed | `life-bfb028fd80ee0267a457` |
| `pocketstation::session::lifecycle::events::SessionLifecycleState::Running` | `Running` | prepared | running_or_terminal | `life-c25444e4f9cd62e3ad01` |
| `pocketstation::runtime::signal::error::AsyncOperatorWorkerError::Join` | `Join` | stopping_or_completed | terminal | `life-c28e3e6c28087dff573a` |
| `pocketstation::runtime::signal::error::AsyncOperatorWorkerError::Close` | `Close` | owned_or_running | closed_or_released | `life-c5eee2ecf1a79f8d55cf` |
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
| `drop` | `drop` | owned_or_running | closed_or_released | `life-fc32baa1cb75a0aa584e` |
| `drop_rate_pct` | `drop_rate_pct` | owned_or_running | closed_or_released | `life-fe3ec5c72e581c7d44d5` |

## Failure handling

Within **Cancellation and rollback**, a transition whose guard, idempotence, or recovery is recorded as not declared has no published guarantee. Preserve its returned error or terminal outcome, inspect component and stage fields, and consult the error reference before retrying.

## Executable evidence

Executable evidence selected for **Cancellation and rollback** is limited to each test's recorded setup and assertions:

- `given_cancellation_when_operator_has_pending_state_then_no_final_is_fabricated` — given cancellation when operator has pending state then no final is fabricated (`src/runtime/signal/operator.rs:2308`; `test-8f495e7bcc9df9f06f5a`).
- `given_blocked_operator_when_cancelled_then_session_cancellation_is_bounded_and_observed` — given blocked operator when cancelled then session cancellation is bounded and observed (`src/session/lifecycle/tests/running.rs:1178`; `test-5c8c016269be4635fd2f`).
- `given_connected_gain_plan_when_executed_then_only_connected_nodes_run_and_worker_receives_output` — given connected gain plan when executed then only connected nodes run and worker receives output (`src/runtime/audio/executor.rs:331`; `test-cd64bb966db1f193ea6f`).
- `given_lineaged_frame_when_realtime_operator_executes_then_output_keeps_capture_epochs` — given lineaged frame when realtime operator executes then output keeps capture epochs (`src/runtime/audio/executor.rs:411`; `test-14905efc2e19f82a8cb2`).
- `given_realtime_fan_out_when_executed_then_each_mutating_branch_gets_independent_copy` — given realtime fan out when executed then each mutating branch gets independent copy (`src/runtime/audio/executor.rs:369`; `test-c0c81ff42570a02c1eb9`).
- `given_compiled_text_edge_when_router_builds_then_only_audio_edge_gets_audio_receiver` — given compiled text edge when router builds then only audio edge gets audio receiver (`src/runtime/audio/router.rs:983`; `test-c5f24b62056cfa546c3a`).
- `given_enqueued_and_dropped_frames_when_observed_then_drop_rate_uses_all_attempts` — given enqueued and dropped frames when observed then drop rate uses all attempts (`src/runtime/audio/router.rs:1241`; `test-9a0bb689d2371b66a92f`).
- `given_failed_branch_when_receiver_drops_then_unrelated_branch_continues` — given failed branch when receiver drops then unrelated branch continues (`src/runtime/audio/router.rs:1518`; `test-b5854f13d50d15dfdbe3`).
- `given_foreign_clock_timestamp_when_delivered_then_source_latency_is_not_fabricated` — given foreign clock timestamp when delivered then source latency is not fabricated (`src/runtime/audio/router.rs:1182`; `test-133d3a4b4c11520b3884`).
- `given_lineage_discontinuity_epoch_change_when_received_then_declared_discontinuity_is_counted` — given lineage discontinuity epoch change when received then declared discontinuity is counted (`src/runtime/audio/router.rs:1117`; `test-fceb86228ea42976addb`).
- `given_lineaged_source_fan_out_when_branch_frames_are_copied_then_exact_lineage_is_preserved` — given lineaged source fan out when branch frames are copied then exact lineage is preserved (`src/runtime/audio/router.rs:1076`; `test-d798548d6c8b059ba1a8`).
- `given_observation_handle_when_consumer_detects_gap_then_live_discontinuity_is_visible` — given observation handle when consumer detects gap then live discontinuity is visible (`src/runtime/audio/router.rs:1410`; `test-225f3db0b8f734fb6907`).

## Related documentation

- [Architecture overview](/docs/architecture/overview.md)
- [Build, prepare, and start](/docs/lifecycle/build-prepare-start.md)
- [Glossary](/docs/glossary.md)
- [PocketStation](/README.md)
- [Running ownership](/docs/lifecycle/running.md)
- [Session lifecycle](/docs/concepts/session-lifecycle.md)
- [Stop, drain, and finalization](/docs/lifecycle/stop-drain-finalize.md)
- [Terminal outcomes](/docs/lifecycle/terminal-outcomes.md)

## Evidence boundary

The claims on **Cancellation and rollback** are anchored to Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `src/session/lifecycle/rollback.rs:1-19` (`DIRECT`)

For **Cancellation and rollback**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.
