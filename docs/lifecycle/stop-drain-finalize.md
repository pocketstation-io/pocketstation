# Stop, drain, and finalization

<!-- claims: CLM-DOC-041-CAP-001,CLM-DOC-041-CAP-002,CLM-DOC-041-CAP-003,CLM-DOC-041-CAP-004,CLM-DOC-041-SOURCE-001 -->

## Scope

- **Start, cancel, stop, and finalize a Session.** Coordinate Session startup, rollback, cancellation, steady-state ownership, stop, and terminal outcomes.
- **Record aligned multistem output.** Configure stems and finalize per-stem outcomes through the recording endpoint lifecycle.
- **Implement endpoint drivers.** Prepare, start, receive, cancel, and finalize destinations behind the endpoint driver contract.
- **Run connector workers.** Supervise connector delivery, acknowledgement, readiness, cancellation, drain, and abort while reporting retry attempts and typed retryability.

The scope of **Stop, drain, and finalization** ends at the native contracts and executable conditions cited below. Platform qualification, performance, retry, and delivery require their own explicit evidence.

## Ownership transition

For **Stop, drain, and finalization**, PocketStation keeps the declaration, compilation, preparation, running, cancellation, rollback, stop, and terminal-result types exposed by the source distinct. Do not collapse its stop outcome into a Boolean assumption: component and finalization failures remain structured data.

## Extracted lifecycle operations

| Operation | Trigger | Source state | Destination state | Evidence record |
|---|---|---|---|---|
| `EndpointDriverFactory::prepare` | `prepare` | declared_or_compiled | prepared_or_prepare_failed | `life-016a48605128b4d9d1b5` |
| `cancel_and_reap` | `cancel_and_reap` | preparing_or_running | cancellation_requested_or_cancelled | `life-033b98da937709d4b2b0` |
| `join` | `join` | stopping_or_completed | terminal | `life-05d0b22aacaf35a3ff7f` |
| `close` | `close` | owned_or_running | closed_or_released | `life-072d37f373f0c2a2db44` |
| `pocketstation::session::lifecycle::running::start_prepared_session` | `start_prepared_session` | declared_or_compiled | prepared_or_prepare_failed | `life-0d33bc06c15c8f36db67` |
| `pocketstation::runtime::lifecycle::sidecar_protocol::SidecarMessageKind::Cancel` | `Cancel` | preparing_or_running | cancellation_requested_or_cancelled | `life-10551e3b62fb7f772613` |
| `pocketstation::connector::error::ConnectorErrorStage::Prepare` | `Prepare` | declared_or_compiled | prepared_or_prepare_failed | `life-146b0fb4adede8184b3b` |
| `start` | `start` | prepared | running_or_start_failed | `life-1695d590bdb58923755e` |
| `prepare_session` | `prepare_session` | declared_or_compiled | prepared_or_prepare_failed | `life-17262c72ad57bde7091d` |
| `drop` | `drop` | owned_or_running | closed_or_released | `life-1c7816652dd061fb1141` |
| `drop` | `drop` | owned_or_running | closed_or_released | `life-1cfe1c82728cfedf39b3` |
| `close_and_reap` | `close_and_reap` | owned_or_running | closed_or_released | `life-2a340a7ae69b2c18f4f3` |
| `pocketstation::runtime::lifecycle::sidecar_host::SidecarState::Running` | `Running` | prepared | running_or_terminal | `life-2d4ebaa478c36896b51d` |
| `ConnectorFactory::prepare` | `prepare` | declared_or_compiled | prepared_or_prepare_failed | `life-2f13aa6cf2bb8190d052` |
| `pocketstation::session::lifecycle::engine::SessionEngineStartError::Prepare` | `Prepare` | declared_or_compiled | prepared_or_prepare_failed | `life-3171793f782cee3c5b77` |
| `start_failure` | `start_failure` | prepared | running_or_start_failed | `life-325781a0a47d85d28f37` |
| `pocketstation::endpoint::runtime::EndpointFailureStage::Prepare` | `Prepare` | declared_or_compiled | prepared_or_prepare_failed | `life-326023ebed5614e93105` |
| `prepare` | `prepare` | declared_or_compiled | prepared_or_prepare_failed | `life-3691d2e1f8efde5ac0a0` |
| `cancel` | `cancel` | preparing_or_running | cancellation_requested_or_cancelled | `life-37895c4b927a1e2c20bd` |
| `cancel_and_join` | `cancel_and_join` | preparing_or_running | cancellation_requested_or_cancelled | `life-3acf2ecfada4977ce7df` |
| `drop_observations` | `drop_observations` | owned_or_running | closed_or_released | `life-3c1a60a96f386b0c2233` |
| `ConnectorWorker::run` | `run` | prepared | running_or_terminal | `life-3cf82f14da5d97113a16` |
| `pocketstation::connector::error::ConnectorErrorStage::Shutdown` | `Shutdown` | state_declared_by_owning_type | state_returned_by_owning_operation | `life-403d696bc539b89298c0` |
| `PreparedEndpointDriver::start` | `start` | declared_or_compiled | prepared_or_prepare_failed | `life-427bb7396c5941e2f668` |
| `drop` | `drop` | owned_or_running | closed_or_released | `life-42da2b93d0ac2fe12e3b` |
| `drop_rate_pct` | `drop_rate_pct` | owned_or_running | closed_or_released | `life-449ed8a22cf03f904d39` |
| `drop` | `drop` | owned_or_running | closed_or_released | `life-44a40cbf57416c570e19` |
| `drop` | `drop` | owned_or_running | closed_or_released | `life-44d8a960be8c4f4eba48` |
| `pocketstation::session::prepare::prepare_session_runtime` | `prepare_session_runtime` | declared_or_compiled | prepared_or_prepare_failed | `life-4788465795aa5b459d72` |
| `shutdown_mode` | `shutdown_mode` | state_declared_by_owning_type | state_returned_by_owning_operation | `life-486be4cc86f74661f105` |
| `drop` | `drop` | owned_or_running | closed_or_released | `life-55d2a5f2ad91927f2c84` |
| `shutdown` | `shutdown` | state_declared_by_owning_type | state_returned_by_owning_operation | `life-57c87f8d848ba4083d83` |
| `pocketstation::runtime::lifecycle::async_host::AsyncRuntimeHostError::Start` | `Start` | prepared | running_or_start_failed | `life-58786e40f7899fab0a36` |
| `prepare_and_spawn_from_plan_edge` | `prepare_and_spawn_from_plan_edge` | declared_or_compiled | prepared_or_prepare_failed | `life-5aa27838dcaa21b67918` |
| `pocketstation::connector::error::ConnectorErrorStage::Join` | `Join` | stopping_or_completed | terminal | `life-5b3d19ddd8f51b4fbb52` |
| `ConnectorWorker::cancel_preparation` | `cancel_preparation` | preparing_or_running | cancellation_requested_or_cancelled | `life-5f5806c43bb0ffb52c32` |
| `start_compiled` | `start_compiled` | prepared | running_or_start_failed | `life-68e51aae7a58503e68ec` |
| `pocketstation::session::lifecycle::events::SessionLifecycleState::Stopped` | `Stopped` | state_declared_by_owning_type | state_returned_by_owning_operation | `life-69a6aff4df2be3f84131` |
| `drop` | `drop` | owned_or_running | closed_or_released | `life-69ac4f312552d661aca2` |
| `pocketstation::session::lifecycle::events::SessionTerminalState::Stopped` | `Stopped` | state_declared_by_owning_type | state_returned_by_owning_operation | `life-6a218d0b344e24bcf2b3` |
| `ConnectorDriver::cancel_preparation` | `cancel_preparation` | preparing_or_running | cancellation_requested_or_cancelled | `life-6aa960ddfdc252c92b3b` |
| `start` | `start` | prepared | running_or_start_failed | `life-6dfd4a8b6248d8918571` |
| `pocketstation::endpoint::runtime::EndpointFailureStage::Start` | `Start` | prepared | running_or_start_failed | `life-6e2140069e12be4278b9` |
| `SourceDriver::close` | `close` | owned_or_running | closed_or_released | `life-78c6b2ef32d95e6ad82c` |
| `start_compiled_cancellable` | `start_compiled_cancellable` | prepared | running_or_start_failed | `life-7b1469dcc46309d2d629` |
| `cancel_and_join` | `cancel_and_join` | preparing_or_running | cancellation_requested_or_cancelled | `life-7e3c74efe4ca679fb089` |
| `prepare` | `prepare` | declared_or_compiled | prepared_or_prepare_failed | `life-7e79b0cec2da142d0cf8` |
| `drop` | `drop` | owned_or_running | closed_or_released | `life-7f4be68921e6d8b9cb92` |
| `prepare` | `prepare` | declared_or_compiled | prepared_or_prepare_failed | `life-8290bb198180554201c6` |
| `ConnectorDriver::start` | `start` | prepared | running_or_start_failed | `life-82c5f212b833b7f793d3` |
| `cancel` | `cancel` | preparing_or_running | cancellation_requested_or_cancelled | `life-84814636a2bc2fd27250` |
| `drop` | `drop` | owned_or_running | closed_or_released | `life-872f31ae0091d843e07d` |
| `pocketstation::runtime::lifecycle::sidecar_protocol::SidecarMessageKind::Close` | `Close` | owned_or_running | closed_or_released | `life-94fdf744a545aa18f84d` |
| `close` | `close` | owned_or_running | closed_or_released | `life-990eedf83476e0e861be` |
| `start_compiled_cancellable` | `start_compiled_cancellable` | prepared | running_or_start_failed | `life-997609314cf808402756` |
| `start` | `start` | prepared | running_or_start_failed | `life-9e8c247c9bcac897aa32` |
| `prepare` | `prepare` | declared_or_compiled | prepared_or_prepare_failed | `life-a9a328eb15d8070dfd7c` |
| `pocketstation::runtime::signal::error::AsyncOperatorWorkerError::Cancel` | `Cancel` | preparing_or_running | cancellation_requested_or_cancelled | `life-afba705f6d4e82a18998` |
| `pocketstation::runtime::audio::runner::PlanSourceSendError::Cancelled` | `Cancelled` | preparing_or_running | cancellation_requested_or_cancelled | `life-b2a66fd10eb50f205b98` |
| `pocketstation::session::extensions::audio_input::buffer::AudioInputBufferAcquireError::Cancelled` | `Cancelled` | preparing_or_running | cancellation_requested_or_cancelled | `life-b7aa4ebc1860ffa7ee22` |
| `start_compiled` | `start_compiled` | prepared | running_or_start_failed | `life-bb6b635326c31b017c27` |
| `pocketstation::session::extensions::audio_input::buffer::AudioInputWriteErrorKind::Cancelled` | `Cancelled` | preparing_or_running | cancellation_requested_or_cancelled | `life-bce2897a917246800a2c` |
| `pocketstation::endpoint::runtime::EndpointShutdownMode::Drain` | `Drain` | stopping | drained_or_drain_failed | `life-bf381a8021cf67f9acb7` |
| `pocketstation::runtime::signal::error::AsyncOperatorWorkerError::Prepare` | `Prepare` | declared_or_compiled | prepared_or_prepare_failed | `life-bfb028fd80ee0267a457` |
| `pocketstation::session::lifecycle::events::SessionLifecycleState::Running` | `Running` | prepared | running_or_terminal | `life-c25444e4f9cd62e3ad01` |
| `pocketstation::runtime::signal::error::AsyncOperatorWorkerError::Join` | `Join` | stopping_or_completed | terminal | `life-c28e3e6c28087dff573a` |
| `pocketstation::runtime::signal::error::AsyncOperatorWorkerError::Close` | `Close` | owned_or_running | closed_or_released | `life-c5eee2ecf1a79f8d55cf` |
| `pocketstation::endpoint::runtime::EndpointShutdownMode::Abort` | `Abort` | owned_or_running | closed_or_released | `life-c6ec15104c7fd3af85a6` |
| `ConnectorDriverFactory::prepare` | `prepare` | declared_or_compiled | prepared_or_prepare_failed | `life-c7058df32bd15fcc307f` |
| `pocketstation::session::lifecycle::running::start_prepared_session_cancellable` | `start_prepared_session_cancellable` | declared_or_compiled | prepared_or_prepare_failed | `life-cad3bede2bc1d9dfdadd` |
| `start` | `start` | prepared | running_or_start_failed | `life-ce1437749f3c39aea285` |
| `prepare_context` | `prepare_context` | declared_or_compiled | prepared_or_prepare_failed | `life-d11d5649023858ebf3d2` |
| `stop` | `stop` | state_declared_by_owning_type | state_returned_by_owning_operation | `life-d6ffb4e55b1f64a3a73d` |
| `pocketstation::session::error_code::SessionStopCode::Stopped` | `Stopped` | state_declared_by_owning_type | state_returned_by_owning_operation | `life-e0bd45ab7651528422f4` |
| `drop` | `drop` | owned_or_running | closed_or_released | `life-e48ae7dd511c2ee65dec` |
| `PreparedEndpointDriver::cancel_preparation` | `cancel_preparation` | declared_or_compiled | prepared_or_prepare_failed | `life-e7b5f8b7d1c5833792cb` |
| `pocketstation::session::lifecycle::start_contract::SessionStartError::Cancelled` | `Cancelled` | prepared | running_or_start_failed | `life-e85546ef287c5d8b1a10` |
| `SourceDriver::prepare` | `prepare` | declared_or_compiled | prepared_or_prepare_failed | `life-e9b55238f265a7f63da2` |
| `RunningEndpointDriver::join_and_finalize` | `join_and_finalize` | stopping_or_completed | terminal | `life-eac281c039a988801809` |
| `pocketstation::session::lifecycle::engine::SessionEngineStartError::Start` | `Start` | prepared | running_or_start_failed | `life-f81b9d7345fea4f92fd6` |
| `ConnectorDriver::shutdown` | `shutdown` | state_declared_by_owning_type | state_returned_by_owning_operation | `life-f901db771cf9230c62f9` |
| `pocketstation::session::lifecycle::observations::EndpointObservationStage::Finalized` | `Finalized` | stopping_or_completed | terminal | `life-f98c28a0a56c89e4ba80` |
| `drop` | `drop` | owned_or_running | closed_or_released | `life-fc32baa1cb75a0aa584e` |
| `drop_rate_pct` | `drop_rate_pct` | owned_or_running | closed_or_released | `life-fe3ec5c72e581c7d44d5` |

## Failure handling

Within **Stop, drain, and finalization**, a transition whose guard, idempotence, or recovery is recorded as not declared has no published guarantee. Preserve its returned error or terminal outcome, inspect component and stage fields, and consult the error reference before retrying.

## Executable evidence

Executable evidence selected for **Stop, drain, and finalization** is limited to each test's recorded setup and assertions:

- `given_process_instance_selector_when_capture_mode_built_then_exact_identity_is_preserved` — given process instance selector when capture mode built then exact identity is preserved (`src/session/lifecycle/running.rs:2589`; `test-dac823b98be9f727652f`).
- `given_worker_failure_or_panic_when_session_stops_then_endpoint_finalization_is_terminal` — given worker failure or panic when session stops then endpoint finalization is terminal (`tests/connector_contract.rs:803`; `test-40ed6b8bb8e60b0fd01a`).
- `given_drain_then_abort_when_requested_then_shutdown_intent_upgrades_monotonically` — given drain then abort when requested then shutdown intent upgrades monotonically (`src/connector/worker/coordination.rs:216`; `test-50a5f1631531f3816b13`).
- `given_stop_and_join_failures_when_finalized_then_both_failures_and_observations_remain_true` — given stop and join failures when finalized then both failures and observations remain true (`src/endpoint/registry/tests.rs:294`; `test-20a4c27d70a60c9bc881`).
- `given_queued_sources_when_cancelled_with_budget_then_drain_is_bounded_and_rest_discards` — given queued sources when cancelled with budget then drain is bounded and rest discards (`src/runtime/audio/runner.rs:636`; `test-31632b8eb3f0b3c90934`).
- `given_no_failures_when_terminal_then_state_is_stopped` — given no failures when terminal then state is stopped (`src/session/lifecycle/events.rs:721`; `test-6dd97c870cc349f825f9`).
- `given_host_owned_backends_when_started_then_polled_audio_and_stop_are_real` — given host owned backends when started then polled audio and stop are real (`src/session/lifecycle/host.rs:724`; `test-1e7766766c491d2b7101`).
- `given_two_sources_when_started_then_gate_lineage_and_repeated_stop_are_truthful` — given two sources when started then gate lineage and repeated stop are truthful (`src/session/lifecycle/tests/running.rs:1309`; `test-39a6ab1a3e3e6782af3a`).
- `given_typed_operator_routes_when_stopped_then_final_state_and_metrics_are_truthful` — given typed operator routes when stopped then final state and metrics are truthful (`src/session/lifecycle/tests/running.rs:1114`; `test-4a96ceb3ecb843502e07`).
- `given_connector_driver_when_two_stems_run_then_core_owns_typed_delivery_and_drain` — given connector driver when two stems run then core owns typed delivery and drain (`tests/connector_contract.rs:580`; `test-c7deea8505c28b2f4d0d`).
- `given_grouped_connector_when_session_stops_then_one_worker_is_joined_and_observed` — given grouped connector when session stops then one worker is joined and observed (`tests/connector_contract.rs:677`; `test-92f5704ec6ee88e59fd8`).
- `given_stopped_public_session_when_new_session_starts_then_capture_restarts_cleanly` — given stopped public session when new session starts then capture restarts cleanly (`tests/session_facade.rs:122`; `test-6a5d630191363f7e4442`).

## Related documentation

- [Glossary](/docs/glossary.md)
- [PocketStation](/README.md)
- [Behavior evidence index](/docs/reference/behavior-evidence.md)
- [Rust API reference](/docs/reference/rust-api.md)
- [Session stop reports component failures](/docs/troubleshooting/session-stop.md)
- [Treat stop outcomes as data](/docs/best-practices/terminal-outcomes.md)
- [Stop a Session and inspect failures](/docs/how-to/stop-session.md)
- [Lifecycle evidence index](/docs/reference/lifecycle-evidence.md)

## Evidence boundary

The claims on **Stop, drain, and finalization** are anchored to Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `src/session/lifecycle/running.rs:1-2612` (`DIRECT`)
- `src/endpoint/runtime.rs:1-531` (`DIRECT`)

For **Stop, drain, and finalization**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.
