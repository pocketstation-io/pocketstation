# Build, prepare, and start

<!-- claims: CLM-DOC-038-CAP-001,CLM-DOC-038-CAP-002,CLM-DOC-038-CAP-003,CLM-DOC-038-CAP-004,CLM-DOC-038-SOURCE-001 -->

## Scope

- **Declare a Session.** Describe sources, operators, endpoints, streams, and recording routes before runtime preparation.
- **Compile Session declarations.** Validate declarations, resolve bindings, and lower a Session specification into an executable plan.
- **Prepare runtime resources.** Prepare source and endpoint runtimes while preserving the mapping back to declaration identities.
- **Start, cancel, stop, and finalize a Session.** Coordinate Session startup, rollback, cancellation, steady-state ownership, stop, and terminal outcomes.

These statements describe repository contracts at the documented snapshot. They do not extend platform qualification, performance, retry, or delivery guarantees beyond the native API contracts and executable evidence.

## Ownership transition

PocketStation uses distinct declaration, compilation, preparation, running, cancellation, rollback, stop, and terminal-result types where the source exposes them. Do not collapse a stop outcome into a Boolean assumption: component and finalization failures remain structured data.

## Extracted lifecycle operations

| Operation | Trigger | Source state | Destination state | Evidence record |
|---|---|---|---|---|
| `drop_observations` | `drop_observations` | unknown | unknown | `life-033f82e656e133a23c4c` |
| `pocketstation::runtime::lifecycle::sidecar_protocol::SidecarMessageKind::Cancel` | `Cancel` | unknown | unknown | `life-05e66cfb59672fcf3e47` |
| `pocketstation::session::lifecycle::engine::SessionEngineStartError::Prepare` | `Prepare` | unknown | unknown | `life-1e6e9f452ca83bcd4874` |
| `pocketstation::session::extensions::audio_input::buffer::AudioInputBufferAcquireError::Cancelled` | `Cancelled` | unknown | unknown | `life-23ee10c2333375238483` |
| `drop` | `drop` | unknown | unknown | `life-2c74a6091c9cde4045cb` |
| `pocketstation::session::lifecycle::start_contract::SessionStartError::Cancelled` | `Cancelled` | unknown | unknown | `life-396be01d86a31314ead0` |
| `pocketstation::runtime::lifecycle::sidecar_host::SidecarState::Running` | `Running` | unknown | unknown | `life-3ac69529ebf1ff2919b5` |
| `drop_rate_pct` | `drop_rate_pct` | unknown | unknown | `life-4ca80c498146346ca2ad` |
| `SourceDriver::close` | `close` | unknown | unknown | `life-51677582d6bfbf19bf36` |
| `drop_rate_pct` | `drop_rate_pct` | unknown | unknown | `life-6127631a15d75622b3a3` |
| `pocketstation::session::lifecycle::events::SessionTerminalState::Stopped` | `Stopped` | unknown | unknown | `life-6838c7db0d54daff94be` |
| `close` | `close` | unknown | unknown | `life-6f6114d034e4edd4755a` |
| `start` | `start` | unknown | unknown | `life-7a6ae3567d5c4cae3ab8` |
| `SourceDriver::prepare` | `prepare` | unknown | unknown | `life-8ab6ff40055ef9e6b1e4` |
| `pocketstation::session::extensions::audio_input::buffer::AudioInputWriteErrorKind::Cancelled` | `Cancelled` | unknown | unknown | `life-9dedf8edf94ac1e55756` |
| `pocketstation::session::lifecycle::events::SessionLifecycleState::Stopped` | `Stopped` | unknown | unknown | `life-a103fa50f0a44e41e441` |
| `pocketstation::session::lifecycle::observations::EndpointObservationStage::Finalized` | `Finalized` | unknown | unknown | `life-a125b7d6bdf384df6d6f` |
| `pocketstation::session::error_code::SessionStopCode::Stopped` | `Stopped` | unknown | unknown | `life-b56488eda9c3ea6b7b3e` |
| `close` | `close` | unknown | unknown | `life-b86a51a255f9938c1308` |
| `pocketstation::session::lifecycle::events::SessionLifecycleState::Running` | `Running` | unknown | unknown | `life-ce971b431224b00409e2` |
| `pocketstation::session::lifecycle::engine::SessionEngineStartError::Start` | `Start` | unknown | unknown | `life-cfbf77d2f976c5aed1ae` |
| `pocketstation::runtime::lifecycle::sidecar_protocol::SidecarMessageKind::Close` | `Close` | unknown | unknown | `life-d2e8d472ee35c4189976` |

## Failure handling

A transition whose guard, idempotence, or recovery is recorded as unknown has no published guarantee here. Preserve the returned error or terminal outcome, inspect component and stage fields, and consult the error reference before retrying.

## Executable evidence

The following test bodies are evidence only for their recorded setup:

- `start_prepared_session` — start prepared session (`src/session/lifecycle/running.rs:612`; `test-3c2e4e6902c7cc6b36a4`).
- `start_prepared_session_cancellable` — start prepared session cancellable (`src/session/lifecycle/running.rs:628`; `test-8ecdda012155cc965789`).
- `given_endpoint_prepare_failure_when_started_then_every_prior_owner_rolls_back` — given endpoint prepare failure when started then every prior owner rolls back (`src/session/lifecycle/tests/running.rs:2248`; `test-0d0c674c4d7598eff201`).
- `given_operator_prepare_failure_when_started_then_all_prior_owners_roll_back` — given operator prepare failure when started then all prior owners roll back (`src/session/lifecycle/tests/running.rs:1218`; `test-5b0700e73704ac58624e`).
- `given_compiled_text_edge_when_router_builds_then_only_audio_edge_gets_audio_receiver` — given compiled text edge when router builds then only audio edge gets audio receiver (`src/runtime/audio/router.rs:983`; `test-c5f24b62056cfa546c3a`).
- `build_output_branches` — build output branches (`src/runtime/signal/operator.rs:257`; `test-e082228209de99d5c6d6`).
- `composed_prepare_context` — composed prepare context (`src/runtime/signal/operator.rs:418`; `test-3a5d74a9beafd694df52`).
- `given_every_nonaudio_signal_class_when_worker_prepares_then_exact_signal_context_is_received` — given every nonaudio signal class when worker prepares then exact signal context is received (`src/runtime/signal/operator.rs:1752`; `test-8298f7b73ae7319aa84e`).
- `given_prepare_context_capacity_disagrees_with_runtime_edge_when_spawned_then_prepare_fails_closed` — given prepare context capacity disagrees with runtime edge when spawned then prepare fails closed (`src/runtime/signal/operator.rs:1808`; `test-ef78893c6bb92b613da0`).
- `given_prepare_failure_when_readiness_is_awaited_then_waiter_returns_false` — given prepare failure when readiness is awaited then waiter returns false (`src/runtime/signal/operator.rs:2179`; `test-f4209d45de1b92221721`).
- `prepare_and_spawn_from_plan_edge` — prepare and spawn from plan edge (`src/runtime/signal/operator.rs:761`; `test-3efdb0a0e7d9d177f679`).
- `simple_prepare_context` — simple prepare context (`src/runtime/signal/operator.rs:356`; `test-020bbea7e7822742e388`).

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

This page was verified against Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `src/session/lifecycle/start_contract.rs:1-362` (`DIRECT`)

A file's presence proves implementation or declaration at this snapshot. It does not by itself prove physical-device qualification, operational performance, retry safety, or behavior outside the recorded test conditions.
