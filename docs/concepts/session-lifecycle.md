# Session lifecycle

<!-- claims: CLM-DOC-016-CAP-001,CLM-DOC-016-SOURCE-001 -->

## What it is

Preparation converts a compiled plan into owned source, endpoint, operator, bridge, and mapping resources without yet transferring the Session to steady-state running ownership.

## Why it exists

Resource acquisition can fail after structural validation. A distinct phase keeps those failures attributable and gives rollback a defined set of already-prepared components.

## Relationships

- Compilation produces the plan and declaration identities.
- Preparation creates runtime mappings back to those identities.
- Start consumes prepared resources or returns a staged failure with rollback evidence.

## Invariants and guarantees

- Prepared resources preserve their declaration IDs.
- A primary preparation failure is not replaced by a rollback failure.
- A prepared Session is not yet evidence that callbacks or workers reached running state.

## When you encounter it

- **Reach first captured frames** — Build a Session that captures an application and microphone and polls their independent frames.
- **Record separate stems** — Record independent source stems and inspect finalization outcomes after Session stop.
- **Diagnose a running Session** — Correlate events, metrics, trace records, stable error codes, and terminal outcomes.

## Use it

- [Prepare resources before start](/docs/how-to/prepare-session.md)
- [Build, prepare, and start lifecycle](/docs/lifecycle/build-prepare-start.md)
- [Session fails before start](/docs/troubleshooting/session-start.md)

## Scope

- **Start, cancel, stop, and finalize a Session.** Coordinate Session startup, rollback, cancellation, steady-state ownership, stop, and terminal outcomes.

The scope of **Session lifecycle** ends at the native contracts and executable conditions cited below. Platform qualification, performance, retry, and delivery require their own explicit evidence.

## Key API

| Public declaration | Kind | Declared purpose | Source |
|---|---|---|---|
| `pocketstation::session::lifecycle::engine::SessionEngine` | struct | Canonical production composition path for one safe Rust Session engine. | `src/session/lifecycle/engine.rs:202` |
| `pocketstation::session::lifecycle::engine::SessionEngineBuilder` | struct | Setup-time builder for one canonical Session composition environment. | `src/session/lifecycle/engine.rs:30` |
| `pocketstation::session::lifecycle::running::RunningSession` | struct | Owns a started Session together with event, polling, recording, trace, and stop resources. | `src/session/lifecycle/running.rs:173` |
| `pocketstation::session::lifecycle::engine::EndpointExtensionRegistrationError` | enum | Classifies failures reported as endpoint extension registration error. | `src/session/lifecycle/engine.rs:305` |
| `pocketstation::session::lifecycle::engine::SessionEngineBuildError` | enum | Classifies failures reported as session engine build error. | `src/session/lifecycle/engine.rs:295` |
| `pocketstation::session::lifecycle::engine::SessionEngineStartError` | enum | Classifies failures reported as session engine start error. | `src/session/lifecycle/engine.rs:315` |
| `pocketstation::session::lifecycle::running::start_prepared_session` | function | Starts prepared session for `running`. | `src/session/lifecycle/running.rs:615` |
| `pocketstation::session::lifecycle::running::start_prepared_session_cancellable` | function | Starts prepared session cancellable for `running`. | `src/session/lifecycle/running.rs:631` |
| `pocketstation::session::lifecycle::engine::EndpointExtensionRegistrationError::ConflictingDefinition` | variant | Reported when the owning operation encounters conflicting definition. | `src/session/lifecycle/engine.rs:311` |
| `pocketstation::session::lifecycle::engine::EndpointExtensionRegistrationError::Definition` | variant | Reported when the owning operation encounters definition. | `src/session/lifecycle/engine.rs:307` |

## Executable evidence

Executable evidence selected for **Session lifecycle** is limited to each test's recorded setup and assertions:

- `given_process_instance_selector_when_capture_mode_built_then_exact_identity_is_preserved` — given process instance selector when capture mode built then exact identity is preserved (`src/session/lifecycle/running.rs:2589`; `test-dac823b98be9f727652f`).
- `given_public_session_pcm_output_when_reentered_then_audio_lane_and_lifecycle_are_observed` — given public session pcm output when reentered then audio lane and lifecycle are observed (`src/session/lifecycle/tests/running.rs:2133`; `test-ae4fbbb0bfe0bcb0aff0`).
- `given_public_facade_when_session_trace_enabled_then_trace_replays_complete_lifecycle` — given public facade when session trace enabled then trace replays complete lifecycle (`tests/session_facade.rs:69`; `test-17d9667bcb3d339c7157`).
- `given_tokio_caller_when_sync_lifecycle_executes_then_no_nested_runtime_panics` — given tokio caller when sync lifecycle executes then no nested runtime panics (`src/runtime/lifecycle/async_host.rs:129`; `test-905856446bbeb46b221d`).
- `given_compiled_lineaged_edge_when_worker_runs_then_exact_session_stem_is_preserved` — given compiled lineaged edge when worker runs then exact session stem is preserved (`src/runtime/signal/operator.rs:2208`; `test-6615dcd3b3105010af0b`).
- `given_cloned_stem_when_session_frozen_then_mutation_is_rejected` — given cloned stem when session frozen then mutation is rejected (`src/session/declaration/draft.rs:1263`; `test-1682e00b3166c4846a92`).
- `given_derived_stream_when_through_called_again_then_chain_is_preserved_in_session_spec` — given derived stream when through called again then chain is preserved in session spec (`src/session/declaration/draft.rs:1179`; `test-837dd73be7d5c552ef15`).
- `given_operator_when_declared_then_session_scoped_instance_and_routes_are_preserved` — given operator when declared then session scoped instance and routes are preserved (`src/session/declaration/draft.rs:1317`; `test-69203660038a41959c14`).
- `given_unrouted_stem_when_session_frozen_then_validation_fails_closed` — given unrouted stem when session frozen then validation fails closed (`src/session/declaration/draft.rs:1305`; `test-1633b6167eec91db04e2`).
- `given_endpoint_operator_id_when_imported_from_session_then_endpoint_contract_type_is_reexported` — given endpoint operator id when imported from session then endpoint contract type is reexported (`src/session/declaration/endpoint.rs:174`; `test-c1047cbdeb5a7bf9bc3b`).
- `given_duplicate_type_when_session_nodes_registered_then_registry_is_unchanged` — given duplicate type when session nodes registered then registry is unchanged (`src/session/extensions/builtins.rs:565`; `test-417c35b0251dee5fe0b7`).
- `given_structural_ingress_when_validated_then_session_metadata_is_not_required` — given structural ingress when validated then session metadata is not required (`src/session/extensions/builtins.rs:636`; `test-7406cc23117530680012`).

## Related documentation

- [Architecture overview](/docs/architecture/overview.md)
- [Build, prepare, and start](/docs/lifecycle/build-prepare-start.md)
- [Cancellation and rollback](/docs/lifecycle/cancellation-and-rollback.md)
- [Glossary](/docs/glossary.md)
- [PocketStation](/README.md)
- [Running ownership](/docs/lifecycle/running.md)
- [Stop, drain, and finalization](/docs/lifecycle/stop-drain-finalize.md)
- [Terminal outcomes](/docs/lifecycle/terminal-outcomes.md)

## Evidence boundary

The claims on **Session lifecycle** are anchored to Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `src/session/lifecycle/engine.rs:1-396` (`DIRECT`)
- `src/session/lifecycle/running.rs:1-2612` (`DIRECT`)

For **Session lifecycle**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.
