# Build, prepare, and start

<!-- claims: CLM-DOC-038-CAP-001,CLM-DOC-038-CAP-002,CLM-DOC-038-CAP-003,CLM-DOC-038-CAP-004,CLM-DOC-038-SOURCE-001 -->

## Scope

- **Declare a Session.** Describe sources, operators, endpoints, streams, and recording routes before runtime preparation.
- **Compile Session declarations.** Validate declarations, resolve bindings, and lower a Session specification into an executable plan.
- **Prepare runtime resources.** Prepare source and endpoint runtimes while preserving the mapping back to declaration identities.
- **Start, cancel, stop, and finalize a Session.** Coordinate Session startup, rollback, cancellation, steady-state ownership, stop, and terminal outcomes.

The scope of **Build, prepare, and start** ends at the native contracts and executable conditions cited below. Platform qualification, performance, retry, and delivery require their own explicit evidence.

## Ownership transition

For **Build, prepare, and start**, PocketStation keeps the declaration, compilation, preparation, running, cancellation, rollback, stop, and terminal-result types exposed by the source distinct. Do not collapse its stop outcome into a Boolean assumption: component and finalization failures remain structured data.

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

Within **Build, prepare, and start**, a transition whose guard, idempotence, or recovery is recorded as not declared has no published guarantee. Preserve its returned error or terminal outcome, inspect component and stage fields, and consult the error reference before retrying.

## Executable evidence

Executable evidence selected for **Build, prepare, and start** is limited to each test's recorded setup and assertions:

- `given_session_without_source_when_validated_then_topology_is_rejected` — given session without source when validated then topology is rejected (`src/session/lifecycle/control.rs:218`; `test-3ad011ae6ea2c1d8804b`).
- `given_supported_source_compositions_when_validated_then_each_is_accepted` — given supported source compositions when validated then each is accepted (`src/session/lifecycle/control.rs:207`; `test-430adf035ce3d16bc420`).
- `given_endpoint_prepare_failure_when_started_then_every_prior_owner_rolls_back` — given endpoint prepare failure when started then every prior owner rolls back (`src/session/lifecycle/tests/running.rs:2251`; `test-c924643ab2f4f9cf21d1`).
- `given_operator_prepare_failure_when_started_then_all_prior_owners_roll_back` — given operator prepare failure when started then all prior owners roll back (`src/session/lifecycle/tests/running.rs:1218`; `test-175f3b0edc08fa4b02f4`).
- `given_compiled_text_edge_when_router_builds_then_only_audio_edge_gets_audio_receiver` — given compiled text edge when router builds then only audio edge gets audio receiver (`src/runtime/audio/router.rs:1012`; `test-687c08c4ebc7699d891b`).
- `given_every_nonaudio_signal_class_when_worker_prepares_then_exact_signal_context_is_received` — given every nonaudio signal class when worker prepares then exact signal context is received (`src/runtime/signal/operator.rs:1752`; `test-0769819bf6f85fc4186c`).
- `given_prepare_context_capacity_disagrees_with_runtime_edge_when_spawned_then_prepare_fails_closed` — given prepare context capacity disagrees with runtime edge when spawned then prepare fails closed (`src/runtime/signal/operator.rs:1808`; `test-94655e5366915899c2bd`).
- `given_prepare_failure_when_readiness_is_awaited_then_waiter_returns_false` — given prepare failure when readiness is awaited then waiter returns false (`src/runtime/signal/operator.rs:2179`; `test-b865f3f1c4ba6e60ac49`).
- `given_compiled_derived_route_when_runtime_prepared_then_compiled_topology_is_preserved` — given compiled derived route when runtime prepared then compiled topology is preserved (`src/session/compile/tests.rs:659`; `test-21f8c08b6457bb762def`).
- `given_graph_mismatch_when_start_fails_then_diagnostic_is_retained` — given graph mismatch when start fails then diagnostic is retained (`src/session/compile/tests.rs:867`; `test-604c0e001a7dcb5f87ae`).
- `given_two_derived_destinations_when_prepared_then_independent_branch_plans_are_preserved` — given two derived destinations when prepared then independent branch plans are preserved (`src/session/compile/tests.rs:685`; `test-b08c2cf433f7a97a1e94`).
- `given_start_and_capture_failures_when_mapped_then_specific_classes_are_preserved` — given start and capture failures when mapped then specific classes are preserved (`src/session/error_code.rs:470`; `test-0a618ca8e650912f01ce`).

## Related documentation

- [Glossary](/docs/glossary.md)
- [PocketStation](/README.md)
- [Behavior evidence index](/docs/reference/behavior-evidence.md)
- [Rust API reference](/docs/reference/rust-api.md)
- [Session API](/docs/reference/session.md)
- [Session fails before start](/docs/troubleshooting/session-start.md)
- [Prepare resources before start](/docs/how-to/prepare-session.md)
- [Session failures](/docs/errors/session.md)

## Evidence boundary

The claims on **Build, prepare, and start** are anchored to Git snapshot `136e74888962558aa846d3143a19136a70936f45` and these primary files:

- `src/session/lifecycle/control.rs:1-391` (`DIRECT`)

For **Build, prepare, and start**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.
