# Stop, drain, and finalization

<!-- claims: CLM-DOC-041-SCOPE-001,CLM-DOC-041-TEXT-001,CLM-DOC-041-TEXT-002,CLM-DOC-041-SOURCE-001 -->

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
| `drop` | `drop` | owned_or_running | closed_or_released | `life-0eb9c8d45523705c071c` |
| `drop_rate_pct` | `drop_rate_pct` | owned_or_running | closed_or_released | `life-1a59542c3bc90997a7e9` |
| `cancel_and_join` | `cancel_and_join` | preparing_or_running | cancellation_requested_or_cancelled | `life-1b6e541aba4cc6f4cdfb` |
| `PreparedEndpointDriver::cancel_preparation` | `cancel_preparation` | constructed_before_preparation | prepared_or_prepare_failed | `life-1d26961a6a50c8cac4c0` |
| `ConnectorDriver::start` | `start` | prepared | running_or_start_failed | `life-1dbc57771bfe56e7548f` |
| `ConnectorFactory::prepare` | `prepare` | constructed_before_preparation | prepared_or_prepare_failed | `life-1ff88f416559bf11bdac` |
| `pocketstation::session::prepare::prepare_session_runtime` | `prepare_session_runtime` | constructed_before_preparation | prepared_or_prepare_failed | `life-1ff9bd824b35e4bd86a6` |
| `close_and_reap` | `close_and_reap` | owned_or_running | closed_or_released | `life-227c49c48f136eb6305f` |
| `SourceDriver::close` | `close` | owned_or_running | closed_or_released | `life-24ac3bb4851827ee31fe` |
| `start_compiled_cancellable` | `start_compiled_cancellable` | prepared | running_or_start_failed | `life-26d46ffab4e997084bfb` |
| `start` | `start` | prepared | running_or_start_failed | `life-27d61a4d665450e563d4` |
| `pocketstation::session::lifecycle::running::start_prepared_session` | `start_prepared_session` | constructed_before_preparation | prepared_or_prepare_failed | `life-2ad92f34c9423181c5be` |
| `close` | `close` | owned_or_running | closed_or_released | `life-31fcaf8e4ff4dc364909` |
| `PreparedEndpointDriver::start` | `start` | constructed_before_preparation | prepared_or_prepare_failed | `life-39ecdc10c277612b83d3` |
| `shutdown_mode` | `shutdown_mode` | owning_state_before_operation | owning_state_after_returned_outcome | `life-420e3f6a3744e7966769` |
| `drop` | `drop` | owned_or_running | closed_or_released | `life-45188ad8c6a8dde207aa` |
| `prepare` | `prepare` | constructed_before_preparation | prepared_or_prepare_failed | `life-456533bd456a4ee124a1` |
| `ConnectorDriver::shutdown` | `shutdown` | owning_state_before_operation | owning_state_after_returned_outcome | `life-47a5209dc72a75e14d0e` |
| `drop` | `drop` | owned_or_running | closed_or_released | `life-4b2edb4fb23deb5bd0b4` |
| `start_failure` | `start_failure` | prepared | running_or_start_failed | `life-4c39dbc19bad333c2912` |
| `drop` | `drop` | owned_or_running | closed_or_released | `life-53a5c0c438b0534a2964` |
| `drop_rate_pct` | `drop_rate_pct` | owned_or_running | closed_or_released | `life-572fd1fca97f08fe449b` |
| `cancel` | `cancel` | preparing_or_running | cancellation_requested_or_cancelled | `life-5a4d79d60befb5c0de67` |
| `drop` | `drop` | owned_or_running | closed_or_released | `life-5fb7a5014d6ffc5d4352` |
| `drop` | `drop` | owned_or_running | closed_or_released | `life-612b1ec3290de460d553` |
| `ConnectorDriverFactory::prepare` | `prepare` | constructed_before_preparation | prepared_or_prepare_failed | `life-61c13ae2e22de07eb8d2` |
| `prepare_and_spawn_from_plan_edge` | `prepare_and_spawn_from_plan_edge` | constructed_before_preparation | prepared_or_prepare_failed | `life-62a217a84804d5de5e4c` |
| `SourceDriver::prepare` | `prepare` | constructed_before_preparation | prepared_or_prepare_failed | `life-6a6429930d687e6fac25` |
| `start_compiled` | `start_compiled` | prepared | running_or_start_failed | `life-6e9224a91062bb5ec713` |
| `drop` | `drop` | owned_or_running | closed_or_released | `life-7ad869dbd8b68d3da850` |
| `start_compiled_cancellable` | `start_compiled_cancellable` | prepared | running_or_start_failed | `life-7e2e74712c52616d3c2c` |
| `drop_observations` | `drop_observations` | owned_or_running | closed_or_released | `life-87afb6dffef4ad5765d6` |
| `prepare_session` | `prepare_session` | constructed_before_preparation | prepared_or_prepare_failed | `life-8871b96155ed0020e00c` |
| `start_compiled` | `start_compiled` | prepared | running_or_start_failed | `life-8b50910aa645c7069ac3` |
| `stop` | `stop` | owning_state_before_operation | owning_state_after_returned_outcome | `life-8bb2a23671da67e30c1b` |
| `prepare` | `prepare` | constructed_before_preparation | prepared_or_prepare_failed | `life-8c21429f0258a9da8a51` |
| `EndpointDriverFactory::prepare` | `prepare` | constructed_before_preparation | prepared_or_prepare_failed | `life-8c8f0c911ad98635dd0a` |
| `RunningEndpointDriver::join_and_finalize` | `join_and_finalize` | stopping_or_completed | terminal | `life-8d739322439f0c50adb3` |
| `start` | `start` | prepared | running_or_start_failed | `life-96cd11312a6f1461a2ca` |
| `cancel` | `cancel` | preparing_or_running | cancellation_requested_or_cancelled | `life-977aafa66864957cfcb4` |
| `shutdown` | `shutdown` | owning_state_before_operation | owning_state_after_returned_outcome | `life-9be9144874aff9e686bd` |
| `ConnectorWorker::cancel_preparation` | `cancel_preparation` | preparing_or_running | cancellation_requested_or_cancelled | `life-a49e6e14d1ed4a8899e8` |
| `drop` | `drop` | owned_or_running | closed_or_released | `life-a66c19cea3372c7a33d8` |
| `pocketstation::session::lifecycle::running::start_prepared_session_cancellable` | `start_prepared_session_cancellable` | constructed_before_preparation | prepared_or_prepare_failed | `life-a8706f56832ee13271c9` |
| `prepare` | `prepare` | constructed_before_preparation | prepared_or_prepare_failed | `life-a8eaed56ee8a1b310d36` |
| `cancel_and_join` | `cancel_and_join` | preparing_or_running | cancellation_requested_or_cancelled | `life-ab1d9f74cf7c8203e8b4` |
| `join` | `join` | stopping_or_completed | terminal | `life-b5f9ab974b55c8f4b6c2` |
| `start` | `start` | prepared | running_or_start_failed | `life-b76642f17aa575a0757e` |
| `close` | `close` | owned_or_running | closed_or_released | `life-b9b46198766a5efb68d8` |
| `drop` | `drop` | owned_or_running | closed_or_released | `life-b9c3e073905015b85207` |
| `ConnectorWorker::run` | `run` | prepared | running_or_terminal | `life-ca3efc8111c30110b7af` |
| `cancel_and_reap` | `cancel_and_reap` | preparing_or_running | cancellation_requested_or_cancelled | `life-e16c2f3cc3e18a861647` |
| `ConnectorDriver::cancel_preparation` | `cancel_preparation` | preparing_or_running | cancellation_requested_or_cancelled | `life-e1ee69c2428e8bd186d6` |
| `start` | `start` | prepared | running_or_start_failed | `life-e4d2a447d0f97df30948` |
| `prepare` | `prepare` | constructed_before_preparation | prepared_or_prepare_failed | `life-f2d6afa474fb10373eb8` |
| `drop` | `drop` | owned_or_running | closed_or_released | `life-f39c0d7ab0cd31705e26` |
| `prepare_context` | `prepare_context` | constructed_before_preparation | prepared_or_prepare_failed | `life-f4c3e7122fc0dc984668` |
| `drop` | `drop` | owned_or_running | closed_or_released | `life-fb0f164096fe9b9fc25e` |

## Failure handling

Within **Stop, drain, and finalization**, a transition whose guard, idempotence, or recovery is recorded as not declared has no published guarantee. Preserve its returned error or terminal outcome, inspect component and stage fields, and consult the error reference before retrying.

## Executable evidence

Executable evidence selected for **Stop, drain, and finalization** is limited to each test's recorded setup and assertions:

- `given_process_instance_selector_when_capture_mode_built_then_exact_identity_is_preserved` — given process instance selector when capture mode built then exact identity is preserved (`src/session/lifecycle/running.rs:2602`; `test-284127121760cbb5874f`).
- `given_worker_failure_or_panic_when_session_stops_then_endpoint_finalization_is_terminal` — given worker failure or panic when session stops then endpoint finalization is terminal (`tests/connector_contract.rs:803`; `test-8ebafe380e15caf3c545`).
- `given_drain_then_abort_when_requested_then_shutdown_intent_upgrades_monotonically` — given drain then abort when requested then shutdown intent upgrades monotonically (`src/connector/worker/coordination.rs:216`; `test-01257fc4936a2d7e629a`).
- `given_stop_and_join_failures_when_finalized_then_both_failures_and_observations_remain_true` — given stop and join failures when finalized then both failures and observations remain true (`src/endpoint/registry/tests.rs:294`; `test-da6484ed83753b351441`).
- `given_queued_sources_when_cancelled_with_budget_then_drain_is_bounded_and_rest_discards` — given queued sources when cancelled with budget then drain is bounded and rest discards (`src/runtime/audio/runner.rs:636`; `test-f9e6bcf6752e622096af`).
- `given_no_failures_when_terminal_then_state_is_stopped` — given no failures when terminal then state is stopped (`src/session/lifecycle/events.rs:721`; `test-74456aeb5a4f8bda5b30`).
- `given_host_owned_backends_when_started_then_polled_audio_and_stop_are_real` — given host owned backends when started then polled audio and stop are real (`src/session/lifecycle/host.rs:724`; `test-1e1a4d3810caea030f74`).
- `given_two_sources_when_started_then_gate_lineage_and_repeated_stop_are_truthful` — given two sources when started then gate lineage and repeated stop are truthful (`src/session/lifecycle/tests/running.rs:1309`; `test-74fc5d0f0a325399dd1d`).
- `given_typed_operator_routes_when_stopped_then_final_state_and_metrics_are_truthful` — given typed operator routes when stopped then final state and metrics are truthful (`src/session/lifecycle/tests/running.rs:1114`; `test-89251e3206216dbcb480`).
- `given_connector_driver_when_two_stems_run_then_core_owns_typed_delivery_and_drain` — given connector driver when two stems run then core owns typed delivery and drain (`tests/connector_contract.rs:580`; `test-0226f46b368cc7dec827`).
- `given_grouped_connector_when_session_stops_then_one_worker_is_joined_and_observed` — given grouped connector when session stops then one worker is joined and observed (`tests/connector_contract.rs:677`; `test-2a1b6ff7d4015d418fc1`).
- `given_stopped_public_session_when_new_session_starts_then_capture_restarts_cleanly` — given stopped public session when new session starts then capture restarts cleanly (`tests/session_facade.rs:122`; `test-7d6c18ed486400271167`).

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

The claims on **Stop, drain, and finalization** are anchored to Git snapshot `136e74888962558aa846d3143a19136a70936f45` and these primary files:

- `src/session/lifecycle/running.rs:59-63` (`DIRECT`)
- `src/session/lifecycle/running.rs:60-60` (`DIRECT`)
- `src/session/lifecycle/running.rs:61-61` (`DIRECT`)
- `src/session/lifecycle/running.rs:62-62` (`DIRECT`)
- `src/session/lifecycle/running.rs:65-68` (`DIRECT`)
- `src/session/lifecycle/running.rs:66-66` (`DIRECT`)
- `src/session/lifecycle/running.rs:67-67` (`DIRECT`)
- `src/session/lifecycle/running.rs:70-76` (`DIRECT`)
- `src/session/lifecycle/running.rs:71-71` (`DIRECT`)
- `src/session/lifecycle/running.rs:72-72` (`DIRECT`)
- `src/session/lifecycle/running.rs:73-73` (`DIRECT`)
- `src/session/lifecycle/running.rs:74-74` (`DIRECT`)
- `src/session/lifecycle/running.rs:75-75` (`DIRECT`)
- `src/session/lifecycle/running.rs:78-78` (`DIRECT`)
- `src/session/lifecycle/running.rs:79-87` (`DIRECT`)
- `src/session/lifecycle/running.rs:80-80` (`DIRECT`)
- `src/session/lifecycle/running.rs:81-81` (`DIRECT`)
- `src/session/lifecycle/running.rs:82-82` (`DIRECT`)
- `src/session/lifecycle/running.rs:83-83` (`DIRECT`)
- `src/session/lifecycle/running.rs:84-84` (`DIRECT`)
- `src/session/lifecycle/running.rs:85-85` (`DIRECT`)
- `src/session/lifecycle/running.rs:86-86` (`DIRECT`)
- `src/session/lifecycle/running.rs:89-92` (`DIRECT`)
- `src/session/lifecycle/running.rs:90-90` (`DIRECT`)
- `src/endpoint/runtime.rs:12-12` (`DIRECT`)
- `src/endpoint/runtime.rs:12-12` (`DIRECT`)
- `src/endpoint/runtime.rs:12-12` (`DIRECT`)
- `src/endpoint/runtime.rs:13-15` (`DIRECT`)
- `src/endpoint/runtime.rs:14-14` (`DIRECT`)
- `src/endpoint/runtime.rs:18-22` (`DIRECT`)
- `src/endpoint/runtime.rs:24-26` (`DIRECT`)
- `src/endpoint/runtime.rs:30-30` (`DIRECT`)
- `src/endpoint/runtime.rs:30-30` (`DIRECT`)
- `src/endpoint/runtime.rs:30-30` (`DIRECT`)
- `src/endpoint/runtime.rs:31-40` (`DIRECT`)
- `src/endpoint/runtime.rs:32-32` (`DIRECT`)
- `src/endpoint/runtime.rs:32-32` (`DIRECT`)
- `src/endpoint/runtime.rs:34-34` (`DIRECT`)
- `src/endpoint/runtime.rs:35-39` (`DIRECT`)
- `src/endpoint/runtime.rs:36-36` (`DIRECT`)
- `src/endpoint/runtime.rs:37-37` (`DIRECT`)
- `src/endpoint/runtime.rs:38-38` (`DIRECT`)
- `src/endpoint/runtime.rs:43-43` (`DIRECT`)
- `src/endpoint/runtime.rs:43-43` (`DIRECT`)
- `src/endpoint/runtime.rs:43-43` (`DIRECT`)
- `src/endpoint/runtime.rs:44-47` (`DIRECT`)
- `src/endpoint/runtime.rs:45-45` (`DIRECT`)
- `src/endpoint/runtime.rs:46-46` (`DIRECT`)

For **Stop, drain, and finalization**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.
