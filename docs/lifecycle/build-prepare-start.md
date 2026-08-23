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

Within **Build, prepare, and start**, a transition whose guard, idempotence, or recovery is recorded as not declared has no published guarantee. Preserve its returned error or terminal outcome, inspect component and stage fields, and consult the error reference before retrying.

## Executable evidence

Executable evidence selected for **Build, prepare, and start** is limited to each test's recorded setup and assertions:

- `given_endpoint_prepare_failure_when_started_then_every_prior_owner_rolls_back` — given endpoint prepare failure when started then every prior owner rolls back (`src/session/lifecycle/tests/running.rs:2248`; `test-0d0c674c4d7598eff201`).
- `given_operator_prepare_failure_when_started_then_all_prior_owners_roll_back` — given operator prepare failure when started then all prior owners roll back (`src/session/lifecycle/tests/running.rs:1218`; `test-5b0700e73704ac58624e`).
- `given_compiled_text_edge_when_router_builds_then_only_audio_edge_gets_audio_receiver` — given compiled text edge when router builds then only audio edge gets audio receiver (`src/runtime/audio/router.rs:983`; `test-c5f24b62056cfa546c3a`).
- `given_every_nonaudio_signal_class_when_worker_prepares_then_exact_signal_context_is_received` — given every nonaudio signal class when worker prepares then exact signal context is received (`src/runtime/signal/operator.rs:1752`; `test-8298f7b73ae7319aa84e`).
- `given_prepare_context_capacity_disagrees_with_runtime_edge_when_spawned_then_prepare_fails_closed` — given prepare context capacity disagrees with runtime edge when spawned then prepare fails closed (`src/runtime/signal/operator.rs:1808`; `test-ef78893c6bb92b613da0`).
- `given_prepare_failure_when_readiness_is_awaited_then_waiter_returns_false` — given prepare failure when readiness is awaited then waiter returns false (`src/runtime/signal/operator.rs:2179`; `test-f4209d45de1b92221721`).
- `given_compiled_derived_route_when_runtime_prepared_then_compiled_topology_is_preserved` — given compiled derived route when runtime prepared then compiled topology is preserved (`src/session/compile/tests.rs:659`; `test-f38493cc0593f603aece`).
- `given_two_derived_destinations_when_prepared_then_independent_branch_plans_are_preserved` — given two derived destinations when prepared then independent branch plans are preserved (`src/session/compile/tests.rs:685`; `test-d6762b694308bbfc1e5c`).
- `given_start_and_capture_failures_when_mapped_then_specific_classes_are_preserved` — given start and capture failures when mapped then specific classes are preserved (`src/session/error_code.rs:470`; `test-e5e2a976b704c1bcb17d`).
- `given_host_owned_backend_failure_when_started_then_error_remains_typed` — given host owned backend failure when started then error remains typed (`src/session/lifecycle/host.rs:782`; `test-93f1bcd9901f0717e324`).
- `given_host_owned_backends_when_started_then_polled_audio_and_stop_are_real` — given host owned backends when started then polled audio and stop are real (`src/session/lifecycle/host.rs:724`; `test-1e7766766c491d2b7101`).
- `given_16khz_session_when_started_then_compiled_endpoint_contexts_preserve_declared_rate` — given 16khz session when started then compiled endpoint contexts preserve declared rate (`src/session/lifecycle/tests/engine.rs:365`; `test-036ad7eaba4f64ee56a4`).

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

The claims on **Build, prepare, and start** are anchored to Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `src/session/lifecycle/start_contract.rs:1-362` (`DIRECT`)

For **Build, prepare, and start**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.
