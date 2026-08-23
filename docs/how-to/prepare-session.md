# Prepare resources before start

<!-- claims: CLM-GUIDE-030-CAP-001,CLM-GUIDE-030-CAP-002,CLM-GUIDE-030-CAP-003,CLM-GUIDE-030-SOURCE-001 -->

## Scope

- **Compile Session declarations.** Validate declarations, resolve bindings, and lower a Session specification into an executable plan.
- **Prepare runtime resources.** Prepare source and endpoint runtimes while preserving the mapping back to declaration identities.
- **Start, cancel, stop, and finalize a Session.** Coordinate Session startup, rollback, cancellation, steady-state ownership, stop, and terminal outcomes.

The scope of **Prepare resources before start** ends at the native contracts and executable conditions cited below. Platform qualification, performance, retry, and delivery require their own explicit evidence.

## Prerequisites

A frozen Session declaration that has already passed structural compilation.

## Procedure

1. Compile the immutable Session declaration.
2. Prepare resources and retain identity mappings.
3. Handle source and endpoint preparation errors.
4. Start with the intended cancellation option.
5. Preserve rollback failures alongside a primary start failure.

## Important consequence

Preparation can perform I/O and resource acquisition; do not treat compilation success as start success.

## Verify the outcome

Every resource mapping is retained, start reaches running state, or a staged failure reports primary and rollback evidence.

Executable evidence selected for **Prepare resources before start** is limited to each test's recorded setup and assertions:

- `given_endpoint_prepare_failure_when_started_then_every_prior_owner_rolls_back` — given endpoint prepare failure when started then every prior owner rolls back (`src/session/lifecycle/tests/running.rs:2248`; `test-0d0c674c4d7598eff201`).
- `given_operator_prepare_failure_when_started_then_all_prior_owners_roll_back` — given operator prepare failure when started then all prior owners roll back (`src/session/lifecycle/tests/running.rs:1218`; `test-5b0700e73704ac58624e`).
- `given_receive_before_enqueue_when_observed_then_latency_sample_is_rejected` — given receive before enqueue when observed then latency sample is rejected (`src/runtime/audio/router.rs:1200`; `test-904f86f2ad1e1369647c`).
- `given_payload_above_branch_limit_when_published_then_all_branches_reject_before_fanout` — given payload above branch limit when published then all branches reject before fanout (`src/runtime/signal/edge.rs:536`; `test-5f2124d7ed0e92c73799`).
- `given_every_nonaudio_signal_class_when_worker_prepares_then_exact_signal_context_is_received` — given every nonaudio signal class when worker prepares then exact signal context is received (`src/runtime/signal/operator.rs:1752`; `test-8298f7b73ae7319aa84e`).
- `given_idle_worker_when_cancelled_then_cancel_hook_runs_before_close` — given idle worker when cancelled then cancel hook runs before close (`src/runtime/signal/operator.rs:2343`; `test-f5115a0f815503398dd5`).
- `given_prepare_context_capacity_disagrees_with_runtime_edge_when_spawned_then_prepare_fails_closed` — given prepare context capacity disagrees with runtime edge when spawned then prepare fails closed (`src/runtime/signal/operator.rs:1808`; `test-ef78893c6bb92b613da0`).
- `given_prepare_failure_when_readiness_is_awaited_then_waiter_returns_false` — given prepare failure when readiness is awaited then waiter returns false (`src/runtime/signal/operator.rs:2179`; `test-f4209d45de1b92221721`).
- `given_compiled_derived_route_when_runtime_prepared_then_compiled_topology_is_preserved` — given compiled derived route when runtime prepared then compiled topology is preserved (`src/session/compile/tests.rs:659`; `test-f38493cc0593f603aece`).
- `given_two_derived_destinations_when_prepared_then_independent_branch_plans_are_preserved` — given two derived destinations when prepared then independent branch plans are preserved (`src/session/compile/tests.rs:685`; `test-d6762b694308bbfc1e5c`).
- `given_foreign_input_handle_when_connected_then_declaration_fails_before_freeze` — given foreign input handle when connected then declaration fails before freeze (`src/session/declaration/tests/operator_connections.rs:133`; `test-766194f5939b3ddb896d`).
- `given_start_and_capture_failures_when_mapped_then_specific_classes_are_preserved` — given start and capture failures when mapped then specific classes are preserved (`src/session/error_code.rs:470`; `test-e5e2a976b704c1bcb17d`).

## Failure signals

- `pocketstation::session::lifecycle::start_contract::SessionStartError` / `CapturePrepare` — `error-8e2ce672937c8251c7d6`
- `pocketstation::session::lifecycle::start_contract::SessionStartError` / `EndpointPrepare` — `error-a45368a6965533fd2ebc`
- `pocketstation::session::lifecycle::start_contract::SessionStartError` / `ExternalSourcePrepare` — `error-bbc9e8298f41cb00dbbf`
- `pocketstation::session::lifecycle::start_contract::SessionStartError` / `OperatorPrepare` — `error-5982a002389727245e54`
- `pocketstation::session::lifecycle::start_contract::SessionStartError` — `error-504eed2dbff73ab46e96`
- `pocketstation::session::lifecycle::start_contract::SessionStartError` / `Cancelled` — `error-e85546ef287c5d8b1a10`
- `pocketstation::session::lifecycle::start_contract::SessionStartError` / `CaptureOpen` — `error-f06eab3e8f6ceeae37d2`
- `pocketstation::session::lifecycle::start_contract::SessionStartError` / `EndpointStart` — `error-8dfbde013f90f254043c`
- `pocketstation::session::lifecycle::start_contract::SessionStartError` / `ExternalAudioBridge` — `error-87902c069db58b4b0049`
- `pocketstation::session::lifecycle::start_contract::SessionStartError` / `ExternalSourceStart` — `error-dd0f49f52c790e0f32f9`

## API reference

- [Runtime Preparation](/docs/concepts/runtime-preparation.md)
- [Build Prepare Start](/docs/lifecycle/build-prepare-start.md)

| Public declaration | Kind | Declared purpose | Source |
|---|---|---|---|
| `pocketstation::session::lifecycle::running::start_prepared_session` | function | Starts prepared session for `running`. | `src/session/lifecycle/running.rs:615` |
| `pocketstation::session::lifecycle::running::start_prepared_session_cancellable` | function | Starts prepared session cancellable for `running`. | `src/session/lifecycle/running.rs:631` |
| `pocketstation::session::lifecycle::engine::SessionEngineStartError::Prepare` | variant | Reported when the owning operation encounters prepare. | `src/session/lifecycle/engine.rs:321` |
| `pocketstation::session::lifecycle::start_contract::SessionStartError::CapturePrepare` | variant | Reported when the owning operation encounters capture prepare. | `src/session/lifecycle/start_contract.rs:158` |
| `pocketstation::session::lifecycle::start_contract::SessionStartError::EndpointPrepare` | variant | Reported when the owning operation encounters endpoint prepare. | `src/session/lifecycle/start_contract.rs:152` |
| `pocketstation::session::lifecycle::start_contract::SessionStartError::ExternalSourcePrepare` | variant | Reported when the owning operation encounters external source prepare. | `src/session/lifecycle/start_contract.rs:119` |
| `pocketstation::session::lifecycle::start_contract::SessionStartError::OperatorPrepare` | variant | Reported when the owning operation encounters operator prepare. | `src/session/lifecycle/start_contract.rs:144` |
| `SessionStartError::CapturePrepare::rollback_failures_total` | struct_field | Counts the total number of rollback failures observed by `CapturePrepare`. | `src/session/lifecycle/start_contract.rs:162` |

## Related documentation

- [Build, prepare, and start](/docs/lifecycle/build-prepare-start.md)
- [Glossary](/docs/glossary.md)
- [PocketStation](/README.md)
- [Behavior evidence index](/docs/reference/behavior-evidence.md)
- [Rust API reference](/docs/reference/rust-api.md)
- [Session API](/docs/reference/session.md)
- [Session fails before start](/docs/troubleshooting/session-start.md)
- [Session failures](/docs/errors/session.md)

## Evidence boundary

The claims on **Prepare resources before start** are anchored to Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `src/session/prepare/mod.rs:1-1290` (`DIRECT`)
- `src/session/lifecycle/start_contract.rs:1-362` (`DIRECT`)

For **Prepare resources before start**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.
