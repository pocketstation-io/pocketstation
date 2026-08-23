# Stop, drain, and finalization

<!-- claims: CLM-DOC-041-CAP-001,CLM-DOC-041-CAP-002,CLM-DOC-041-CAP-003,CLM-DOC-041-CAP-004,CLM-DOC-041-SOURCE-001 -->

## Scope

- **Start, cancel, stop, and finalize a Session.** Coordinate Session startup, rollback, cancellation, steady-state ownership, stop, and terminal outcomes.
- **Record aligned multistem output.** Configure stems and finalize per-stem outcomes through the recording endpoint lifecycle.
- **Implement endpoint drivers.** Prepare, start, receive, cancel, and finalize destinations behind the endpoint driver contract.
- **Run connector workers.** Supervise connector delivery, acknowledgement, retry budgets, readiness, cancellation, drain, and abort.

These statements describe repository contracts at the documented snapshot. They do not extend platform qualification, performance, retry, or delivery guarantees beyond the native API contracts and executable evidence.

## Ownership transition

PocketStation uses distinct declaration, compilation, preparation, running, cancellation, rollback, stop, and terminal-result types where the source exposes them. Do not collapse a stop outcome into a Boolean assumption: component and finalization failures remain structured data.

## Extracted lifecycle operations

| Operation | Trigger | Source state | Destination state | Evidence record |
|---|---|---|---|---|
| `ConnectorWorker::cancel_preparation` | `cancel_preparation` | unknown | unknown | `life-01cc5d2b80f8e24f1fdc` |
| `drop_observations` | `drop_observations` | unknown | unknown | `life-033f82e656e133a23c4c` |
| `pocketstation::runtime::lifecycle::sidecar_protocol::SidecarMessageKind::Cancel` | `Cancel` | unknown | unknown | `life-05e66cfb59672fcf3e47` |
| `pocketstation::session::lifecycle::engine::SessionEngineStartError::Prepare` | `Prepare` | unknown | unknown | `life-1e6e9f452ca83bcd4874` |
| `pocketstation::session::extensions::audio_input::buffer::AudioInputBufferAcquireError::Cancelled` | `Cancelled` | unknown | unknown | `life-23ee10c2333375238483` |
| `drop` | `drop` | unknown | unknown | `life-2c74a6091c9cde4045cb` |
| `pocketstation::connector::error::ConnectorErrorStage::Join` | `Join` | unknown | unknown | `life-326d10a69e8bf7fdb781` |
| `pocketstation::endpoint::runtime::EndpointShutdownMode::Drain` | `Drain` | unknown | unknown | `life-37b462ed94c0e7d7bcaa` |
| `pocketstation::session::lifecycle::start_contract::SessionStartError::Cancelled` | `Cancelled` | unknown | unknown | `life-396be01d86a31314ead0` |
| `pocketstation::runtime::lifecycle::sidecar_host::SidecarState::Running` | `Running` | unknown | unknown | `life-3ac69529ebf1ff2919b5` |
| `shutdown_mode` | `shutdown_mode` | unknown | unknown | `life-3bd045de883d59fb612b` |
| `drop_rate_pct` | `drop_rate_pct` | unknown | unknown | `life-4ca80c498146346ca2ad` |
| `SourceDriver::close` | `close` | unknown | unknown | `life-51677582d6bfbf19bf36` |
| `EndpointDriverFactory::prepare` | `prepare` | unknown | unknown | `life-591e104c350f112363df` |
| `pocketstation::connector::error::ConnectorErrorStage::Prepare` | `Prepare` | unknown | unknown | `life-5b07b94601e40546dc60` |
| `drop_rate_pct` | `drop_rate_pct` | unknown | unknown | `life-6127631a15d75622b3a3` |
| `pocketstation::session::lifecycle::events::SessionTerminalState::Stopped` | `Stopped` | unknown | unknown | `life-6838c7db0d54daff94be` |
| `ConnectorFactory::prepare` | `prepare` | unknown | unknown | `life-6b5d8c54e0f2147b52d5` |
| `close` | `close` | unknown | unknown | `life-6f6114d034e4edd4755a` |
| `prepare` | `prepare` | unknown | unknown | `life-7030b8447dc3db092c92` |
| `start` | `start` | unknown | unknown | `life-7a6ae3567d5c4cae3ab8` |
| `pocketstation::endpoint::runtime::EndpointFailureStage::Start` | `Start` | unknown | unknown | `life-8389e526c053c0f4878c` |
| `ConnectorWorker::run` | `run` | unknown | unknown | `life-88f209a3c7bc2ba137fb` |
| `SourceDriver::prepare` | `prepare` | unknown | unknown | `life-8ab6ff40055ef9e6b1e4` |
| `pocketstation::endpoint::runtime::EndpointShutdownMode::Abort` | `Abort` | unknown | unknown | `life-9cf382d0e5a6816d4c71` |
| `pocketstation::session::extensions::audio_input::buffer::AudioInputWriteErrorKind::Cancelled` | `Cancelled` | unknown | unknown | `life-9dedf8edf94ac1e55756` |
| `pocketstation::session::lifecycle::events::SessionLifecycleState::Stopped` | `Stopped` | unknown | unknown | `life-a103fa50f0a44e41e441` |
| `pocketstation::session::lifecycle::observations::EndpointObservationStage::Finalized` | `Finalized` | unknown | unknown | `life-a125b7d6bdf384df6d6f` |
| `RunningEndpointDriver::join_and_finalize` | `join_and_finalize` | unknown | unknown | `life-a41d9c931f7f68bd3076` |
| `PreparedEndpointDriver::start` | `start` | unknown | unknown | `life-a77333ff603f041c3363` |
| `ConnectorDriverFactory::prepare` | `prepare` | unknown | unknown | `life-a77494b6f212277d3533` |
| `ConnectorDriver::cancel_preparation` | `cancel_preparation` | unknown | unknown | `life-a7cf93ad46ee7edbf114` |
| `pocketstation::endpoint::runtime::EndpointFailureStage::Prepare` | `Prepare` | unknown | unknown | `life-b190ec77abf54ff75844` |
| `pocketstation::session::error_code::SessionStopCode::Stopped` | `Stopped` | unknown | unknown | `life-b56488eda9c3ea6b7b3e` |
| `close` | `close` | unknown | unknown | `life-b86a51a255f9938c1308` |
| `ConnectorDriver::shutdown` | `shutdown` | unknown | unknown | `life-c2ba2f7ef3b906bb0d57` |
| `ConnectorDriver::start` | `start` | unknown | unknown | `life-c85ae5780d9d74272fa7` |
| `drop` | `drop` | unknown | unknown | `life-ca5172d134b2b5db799e` |
| `pocketstation::session::lifecycle::events::SessionLifecycleState::Running` | `Running` | unknown | unknown | `life-ce971b431224b00409e2` |
| `pocketstation::session::lifecycle::engine::SessionEngineStartError::Start` | `Start` | unknown | unknown | `life-cfbf77d2f976c5aed1ae` |
| `pocketstation::runtime::lifecycle::sidecar_protocol::SidecarMessageKind::Close` | `Close` | unknown | unknown | `life-d2e8d472ee35c4189976` |
| `PreparedEndpointDriver::cancel_preparation` | `cancel_preparation` | unknown | unknown | `life-d8c08562ef859bb25c31` |
| `drop` | `drop` | unknown | unknown | `life-ec902207cd2a525b3786` |
| `pocketstation::connector::error::ConnectorErrorStage::Shutdown` | `Shutdown` | unknown | unknown | `life-f648d9f62ff6bcc1ebe5` |

## Failure handling

A transition whose guard, idempotence, or recovery is recorded as unknown has no published guarantee here. Preserve the returned error or terminal outcome, inspect component and stage fields, and consult the error reference before retrying.

## Executable evidence

The following test bodies are evidence only for their recorded setup:

- `given_worker_failure_or_panic_when_session_stops_then_endpoint_finalization_is_terminal` — given worker failure or panic when session stops then endpoint finalization is terminal (`tests/connector_contract.rs:805`; `test-e56e88c9e99290ea720a`).
- `given_drain_then_abort_when_requested_then_shutdown_intent_upgrades_monotonically` — given drain then abort when requested then shutdown intent upgrades monotonically (`src/connector/worker/coordination.rs:216`; `test-50a5f1631531f3816b13`).
- `given_stop_and_join_failures_when_finalized_then_both_failures_and_observations_remain_true` — given stop and join failures when finalized then both failures and observations remain true (`src/endpoint/registry/tests.rs:294`; `test-20a4c27d70a60c9bc881`).
- `given_queued_sources_when_cancelled_with_budget_then_drain_is_bounded_and_rest_discards` — given queued sources when cancelled with budget then drain is bounded and rest discards (`src/runtime/audio/runner.rs:636`; `test-31632b8eb3f0b3c90934`).
- `given_no_failures_when_terminal_then_state_is_stopped` — given no failures when terminal then state is stopped (`src/session/lifecycle/events.rs:721`; `test-6dd97c870cc349f825f9`).
- `given_host_owned_backends_when_started_then_polled_audio_and_stop_are_real` — given host owned backends when started then polled audio and stop are real (`src/session/lifecycle/host.rs:724`; `test-1e7766766c491d2b7101`).
- `given_two_sources_when_started_then_gate_lineage_and_repeated_stop_are_truthful` — given two sources when started then gate lineage and repeated stop are truthful (`src/session/lifecycle/tests/running.rs:1309`; `test-39a6ab1a3e3e6782af3a`).
- `given_typed_operator_routes_when_stopped_then_final_state_and_metrics_are_truthful` — given typed operator routes when stopped then final state and metrics are truthful (`src/session/lifecycle/tests/running.rs:1114`; `test-4a96ceb3ecb843502e07`).
- `given_connector_driver_when_two_stems_run_then_core_owns_typed_delivery_and_drain` — given connector driver when two stems run then core owns typed delivery and drain (`tests/connector_contract.rs:582`; `test-eefa0d157754becdb1a2`).
- `given_grouped_connector_when_session_stops_then_one_worker_is_joined_and_observed` — given grouped connector when session stops then one worker is joined and observed (`tests/connector_contract.rs:679`; `test-aa2345c7b9339f742b48`).
- `given_stopped_public_session_when_new_session_starts_then_capture_restarts_cleanly` — given stopped public session when new session starts then capture restarts cleanly (`tests/session_facade.rs:124`; `test-cbee0768bffa592adde2`).
- `given_provider_owned_field_name_when_resolved_then_core_preserves_it_opaquely` — given provider owned field name when resolved then core preserves it opaquely (`src/connector/configuration.rs:642`; `test-d9078fd01d0271720b30`).

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

This page was verified against Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `src/session/lifecycle/running.rs:1-2612` (`DIRECT`)
- `src/endpoint/runtime.rs:1-531` (`DIRECT`)

A file's presence proves implementation or declaration at this snapshot. It does not by itself prove physical-device qualification, operational performance, retry safety, or behavior outside the recorded test conditions.
