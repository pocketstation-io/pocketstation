# Prepare resources before start

<!-- claims: CLM-GUIDE-030-CAP-001,CLM-GUIDE-030-CAP-002,CLM-GUIDE-030-CAP-003,CLM-GUIDE-030-SOURCE-001 -->

## Scope

- **Compile Session declarations.** Validate declarations, resolve bindings, and lower a Session specification into an executable plan.
- **Prepare runtime resources.** Prepare source and endpoint runtimes while preserving the mapping back to declaration identities.
- **Start, cancel, stop, and finalize a Session.** Coordinate Session startup, rollback, cancellation, steady-state ownership, stop, and terminal outcomes.

These statements describe repository contracts at the documented snapshot. They do not extend platform qualification, performance, retry, or delivery guarantees beyond the native API contracts and executable evidence.

## Prerequisites

Read the linked concept and confirm that target platform, Cargo features, source or provider dependencies, and application-owned permission work match this task. Keep returned typed errors and outcomes available for verification.

## Procedure

1. Compile the immutable Session declaration.
2. Prepare resources and retain identity mappings.
3. Handle source and endpoint preparation errors.
4. Start with the intended cancellation option.
5. Preserve rollback failures alongside a primary start failure.

## APIs used

| Public declaration | Kind | Declared purpose | Source |
|---|---|---|---|
| `pocketstation::session::error_code::SessionStartErrorCode::EndpointPrepareFailed` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/session/error_code.rs:72` |
| `pocketstation::session::error_code::SessionStartErrorCode::RuntimePrepareFailed` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/session/error_code.rs:68` |
| `pocketstation::session::lifecycle::engine::SessionEngineStartError::Prepare` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/session/lifecycle/engine.rs:321` |
| `pocketstation::session::lifecycle::start_contract::SessionStartError::CapturePrepare` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/session/lifecycle/start_contract.rs:158` |
| `pocketstation::session::lifecycle::start_contract::SessionStartError::EndpointPrepare` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/session/lifecycle/start_contract.rs:152` |
| `pocketstation::session::lifecycle::start_contract::SessionStartError::ExternalSourcePrepare` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/session/lifecycle/start_contract.rs:119` |
| `pocketstation::session::lifecycle::start_contract::SessionStartError::OperatorPrepare` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/session/lifecycle/start_contract.rs:144` |
| `SessionStartError::CapturePrepare::rollback_failures_total` | struct_field | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/session/lifecycle/start_contract.rs:162` |
| `SessionStartError::CapturePrepare::source` | struct_field | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/session/lifecycle/start_contract.rs:161` |
| `SessionStartError::CapturePrepare::stem_id` | struct_field | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/session/lifecycle/start_contract.rs:159` |
| `SessionStartError::EndpointPrepare::rollback_failures_total` | struct_field | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/session/lifecycle/start_contract.rs:155` |
| `SessionStartError::EndpointPrepare::source` | struct_field | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/session/lifecycle/start_contract.rs:154` |
| `SessionStartError::ExternalSourcePrepare::message` | struct_field | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/session/lifecycle/start_contract.rs:120` |
| `SessionStartError::ExternalSourcePrepare::rollback_failures_total` | struct_field | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/session/lifecycle/start_contract.rs:121` |
| `SessionStartError::OperatorPrepare::message` | struct_field | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/session/lifecycle/start_contract.rs:146` |
| `SessionStartError::OperatorPrepare::operator_instance_id` | struct_field | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/session/lifecycle/start_contract.rs:145` |

## Verify the outcome

The following test bodies are evidence only for their recorded setup:

- `start_prepared_session` — start prepared session (`src/session/lifecycle/running.rs:612`; `test-3c2e4e6902c7cc6b36a4`).
- `start_prepared_session_cancellable` — start prepared session cancellable (`src/session/lifecycle/running.rs:628`; `test-8ecdda012155cc965789`).
- `given_endpoint_prepare_failure_when_started_then_every_prior_owner_rolls_back` — given endpoint prepare failure when started then every prior owner rolls back (`src/session/lifecycle/tests/running.rs:2248`; `test-0d0c674c4d7598eff201`).
- `given_operator_prepare_failure_when_started_then_all_prior_owners_roll_back` — given operator prepare failure when started then all prior owners roll back (`src/session/lifecycle/tests/running.rs:1218`; `test-5b0700e73704ac58624e`).
- `given_receive_before_enqueue_when_observed_then_latency_sample_is_rejected` — given receive before enqueue when observed then latency sample is rejected (`src/runtime/audio/router.rs:1200`; `test-904f86f2ad1e1369647c`).
- `given_payload_above_branch_limit_when_published_then_all_branches_reject_before_fanout` — given payload above branch limit when published then all branches reject before fanout (`src/runtime/signal/edge.rs:536`; `test-5f2124d7ed0e92c73799`).
- `composed_prepare_context` — composed prepare context (`src/runtime/signal/operator.rs:418`; `test-3a5d74a9beafd694df52`).
- `given_every_nonaudio_signal_class_when_worker_prepares_then_exact_signal_context_is_received` — given every nonaudio signal class when worker prepares then exact signal context is received (`src/runtime/signal/operator.rs:1752`; `test-8298f7b73ae7319aa84e`).
- `given_idle_worker_when_cancelled_then_cancel_hook_runs_before_close` — given idle worker when cancelled then cancel hook runs before close (`src/runtime/signal/operator.rs:2343`; `test-f5115a0f815503398dd5`).
- `given_prepare_context_capacity_disagrees_with_runtime_edge_when_spawned_then_prepare_fails_closed` — given prepare context capacity disagrees with runtime edge when spawned then prepare fails closed (`src/runtime/signal/operator.rs:1808`; `test-ef78893c6bb92b613da0`).
- `given_prepare_failure_when_readiness_is_awaited_then_waiter_returns_false` — given prepare failure when readiness is awaited then waiter returns false (`src/runtime/signal/operator.rs:2179`; `test-f4209d45de1b92221721`).
- `prepare_and_spawn_from_plan_edge` — prepare and spawn from plan edge (`src/runtime/signal/operator.rs:761`; `test-3efdb0a0e7d9d177f679`).

## Failure signals

- `pocketstation::session::declaration::typed_stream::TypedStreamError` / `OutputSignalMismatch` — `error-00e5716261eba0f8cf3d`
- `pocketstation::session::error::SessionError` / `UnknownStem` — `error-00f6e798d158df66c847`
- `pocketstation::session::error_code::SessionStartErrorCode` / `StartCancelled` — `error-01d3fc855e2a00319076`
- `pocketstation::session::lifecycle::start_contract::SessionStartError` / `OperatorPrepare` — `error-023d6ab0b23a50a614ff`
- `pocketstation::session::error_code::SessionStartErrorCode` / `TraceRecorderSetupFailed` — `error-0279b2b6b0cb3b5801bc`
- `pocketstation::session::prepare::error::SessionPrepareError` / `MissingOperatorSignalInput` — `error-037ddc3e193da74177f8`
- `pocketstation::session::lifecycle::trace::SessionTraceValidationError` / `InvalidLayout` — `error-05c60389efcb84311921`
- `pocketstation::session::prepare::error::SessionPrepareError` — `error-085082b521c14e5ecd1e`
- `pocketstation::session::extensions::audio_input::buffer::AudioInputWriteErrorKind` / `Closed` — `error-08a7536094bfb2242b17`
- `pocketstation::session::lifecycle::host::SessionEngineHostBuildError` / `EndpointExtensionRegistration` — `error-09837185c7fca0f70618`
- `pocketstation::session::lifecycle::start_contract::SessionStartError` / `MissingEndpointDeclaration` — `error-0bc2f7c0b9f9dbf8ddd7`
- `pocketstation::session::lifecycle::trace::SessionTraceRecorderStartError` / `ZeroCapacity` — `error-0bd6f58be40ade9a01fe`

Retry only when the relevant API or error contract explicitly permits it. An error name, a transient-looking message, or a successful prior run is not retry evidence.

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

This page was verified against Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `src/session/prepare/mod.rs:1-1290` (`DIRECT`)
- `src/session/lifecycle/start_contract.rs:1-362` (`DIRECT`)

A file's presence proves implementation or declaration at this snapshot. It does not by itself prove physical-device qualification, operational performance, retry safety, or behavior outside the recorded test conditions.
