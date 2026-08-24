# Session lifecycle

<!-- claims: CLM-DOC-016-SCOPE-001,CLM-DOC-016-TEXT-001,CLM-DOC-016-TEXT-002,CLM-DOC-016-TEXT-003,CLM-DOC-016-TEXT-004,CLM-DOC-016-TEXT-005,CLM-DOC-016-TEXT-006,CLM-DOC-016-SOURCE-001 -->

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
| `pocketstation::session::lifecycle::engine::SessionEngineBuilder` | struct | Registers the components and runtime configuration for one Session. | `src/session/lifecycle/engine.rs:30` |
| `pocketstation::session::lifecycle::running::RunningSession` | struct | Owns a started Session together with event, polling, recording, trace, and stop resources. | `src/session/lifecycle/running.rs:173` |
| `pocketstation::session::lifecycle::engine::EndpointExtensionRegistrationError` | enum | Classifies failures produced during endpoint extension registration. | `src/session/lifecycle/engine.rs:305` |
| `pocketstation::session::lifecycle::engine::SessionEngineBuildError` | enum | Classifies failures produced during session engine construction and input validation. | `src/session/lifecycle/engine.rs:295` |
| `pocketstation::session::lifecycle::engine::SessionEngineStartError` | enum | Classifies failures produced during session engine lifecycle start. | `src/session/lifecycle/engine.rs:315` |
| `pocketstation::session::lifecycle::running::start_prepared_session` | function | Starts prepared session for `running`. | `src/session/lifecycle/running.rs:627` |
| `pocketstation::session::lifecycle::running::start_prepared_session_cancellable` | function | Starts prepared session cancellable for `running`. | `src/session/lifecycle/running.rs:643` |
| `pocketstation::session::lifecycle::engine::EndpointExtensionRegistrationError::ConflictingDefinition` | variant | Reports that definition conflicts with an existing registration or declaration. | `src/session/lifecycle/engine.rs:311` |
| `pocketstation::session::lifecycle::engine::EndpointExtensionRegistrationError::Definition` | variant | Classifies a failure at the definition stage or component of `EndpointExtensionRegistrationError`. | `src/session/lifecycle/engine.rs:307` |

## Executable evidence

Executable evidence selected for **Session lifecycle** is limited to each test's recorded setup and assertions:

- `given_process_instance_selector_when_capture_mode_built_then_exact_identity_is_preserved` — given process instance selector when capture mode built then exact identity is preserved (`src/session/lifecycle/running.rs:2602`; `test-284127121760cbb5874f`).
- `given_public_session_pcm_output_when_reentered_then_audio_lane_and_lifecycle_are_observed` — given public session pcm output when reentered then audio lane and lifecycle are observed (`src/session/lifecycle/tests/running.rs:2136`; `test-ea5b06c730a73a1dc9ca`).
- `given_public_facade_when_session_trace_enabled_then_trace_replays_complete_lifecycle` — given public facade when session trace enabled then trace replays complete lifecycle (`tests/session_facade.rs:69`; `test-f5d1e9009c62e6cb57d5`).
- `given_tokio_caller_when_sync_lifecycle_executes_then_no_nested_runtime_panics` — given tokio caller when sync lifecycle executes then no nested runtime panics (`src/runtime/lifecycle/async_host.rs:129`; `test-11ac2e93d34a2efd0e98`).
- `given_compiled_lineaged_edge_when_worker_runs_then_exact_session_stem_is_preserved` — given compiled lineaged edge when worker runs then exact session stem is preserved (`src/runtime/signal/operator.rs:2208`; `test-9e1c8ad04d302a8bf88b`).
- `given_cloned_stem_when_session_frozen_then_mutation_is_rejected` — given cloned stem when session frozen then mutation is rejected (`src/session/declaration/draft.rs:1251`; `test-2cf3d98ffa38e0f5ee68`).
- `given_derived_stream_when_through_called_again_then_chain_is_preserved_in_session_spec` — given derived stream when through called again then chain is preserved in session spec (`src/session/declaration/draft.rs:1167`; `test-aec2c4ee7ff8efede00a`).
- `given_operator_when_declared_then_session_scoped_instance_and_routes_are_preserved` — given operator when declared then session scoped instance and routes are preserved (`src/session/declaration/draft.rs:1305`; `test-e84db4efcd6a7145550a`).
- `given_unrouted_stem_when_session_frozen_then_validation_fails_closed` — given unrouted stem when session frozen then validation fails closed (`src/session/declaration/draft.rs:1293`; `test-8e301580cdd23a244478`).
- `given_endpoint_operator_id_when_imported_from_session_then_endpoint_contract_type_is_reexported` — given endpoint operator id when imported from session then endpoint contract type is reexported (`src/session/declaration/endpoint.rs:174`; `test-0b2dadbe3265dde022e4`).
- `given_duplicate_type_when_session_nodes_registered_then_registry_is_unchanged` — given duplicate type when session nodes registered then registry is unchanged (`src/session/extensions/builtins.rs:565`; `test-df56c06567959a01bf75`).
- `given_structural_ingress_when_validated_then_session_metadata_is_not_required` — given structural ingress when validated then session metadata is not required (`src/session/extensions/builtins.rs:636`; `test-9a19e49a87f8cc918b10`).

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

The claims on **Session lifecycle** are anchored to Git snapshot `136e74888962558aa846d3143a19136a70936f45` and these primary files:

- `src/session/lifecycle/engine.rs:30-40` (`DIRECT`)
- `src/session/lifecycle/engine.rs:31-31` (`DIRECT`)
- `src/session/lifecycle/engine.rs:32-32` (`DIRECT`)
- `src/session/lifecycle/engine.rs:33-33` (`DIRECT`)
- `src/session/lifecycle/engine.rs:34-34` (`DIRECT`)
- `src/session/lifecycle/engine.rs:35-35` (`DIRECT`)
- `src/session/lifecycle/engine.rs:36-36` (`DIRECT`)
- `src/session/lifecycle/engine.rs:37-37` (`DIRECT`)
- `src/session/lifecycle/engine.rs:38-38` (`DIRECT`)
- `src/session/lifecycle/engine.rs:39-39` (`DIRECT`)
- `src/session/lifecycle/engine.rs:43-64` (`DIRECT`)
- `src/session/lifecycle/engine.rs:66-72` (`DIRECT`)
- `src/session/lifecycle/engine.rs:74-85` (`DIRECT`)
- `src/session/lifecycle/engine.rs:91-112` (`DIRECT`)
- `src/session/lifecycle/engine.rs:114-120` (`DIRECT`)
- `src/session/lifecycle/engine.rs:125-140` (`DIRECT`)
- `src/session/lifecycle/engine.rs:146-167` (`DIRECT`)
- `src/session/lifecycle/engine.rs:171-173` (`DIRECT`)
- `src/session/lifecycle/engine.rs:176-194` (`DIRECT`)
- `src/session/lifecycle/engine.rs:202-212` (`DIRECT`)
- `src/session/lifecycle/engine.rs:203-203` (`DIRECT`)
- `src/session/lifecycle/engine.rs:204-204` (`DIRECT`)
- `src/session/lifecycle/engine.rs:205-205` (`DIRECT`)
- `src/session/lifecycle/engine.rs:206-206` (`DIRECT`)
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

For **Session lifecycle**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.
