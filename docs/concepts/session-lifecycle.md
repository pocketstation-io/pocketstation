# Session lifecycle

<!-- claims: CLM-DOC-016-CAP-001,CLM-DOC-016-SOURCE-001 -->

Coordinate Session startup, rollback, cancellation, steady-state ownership, stop, and terminal outcomes.

## Scope

- **Start, cancel, stop, and finalize a Session.** Coordinate Session startup, rollback, cancellation, steady-state ownership, stop, and terminal outcomes.

These statements describe repository contracts at the documented snapshot. They do not extend platform qualification, performance, retry, or delivery guarantees beyond the native API contracts and executable evidence.

## Contract surface

| Public declaration | Kind | Declared purpose | Source |
|---|---|---|---|
| `pocketstation::session::lifecycle::events::SessionControlFailure` | struct | Typed control-plane failure without exposing an implementation error type. | `src/session/lifecycle/events.rs:70` |
| `pocketstation::session::lifecycle::events::SessionEndpointFailure` | struct | Endpoint failure associated with one stable route and endpoint. | `src/session/lifecycle/events.rs:125` |
| `pocketstation::session::lifecycle::events::SessionEvent` | struct | Event emitted by the session lifecycle authority. | `src/session/lifecycle/events.rs:308` |
| `pocketstation::session::lifecycle::events::SessionEventReceiver` | struct | Sole consumer for a session's bounded control-event queue. | `src/session/lifecycle/events.rs:500` |
| `pocketstation::session::lifecycle::events::SessionFinalizationFailure` | struct | Failure observed while finalizing a stopping session. | `src/session/lifecycle/events.rs:186` |
| `pocketstation::session::lifecycle::events::SessionRollbackFailure` | struct | Failure observed while rolling back a partial session start. | `src/session/lifecycle/events.rs:165` |
| `pocketstation::session::lifecycle::events::SessionSourceFailure` | struct | Source failure associated with one stable session stem. | `src/session/lifecycle/events.rs:104` |
| `pocketstation::session::lifecycle::events::SessionTerminalOutcome` | struct | Complete terminal result. Failure categories remain separate for diagnosis. | `src/session/lifecycle/events.rs:217` |
| `pocketstation::session::lifecycle::observations::SessionAudioReentryMetrics` | struct | Exact boundedness and lifecycle accounting for one operator PCM output re-entering the Session audio lane. | `src/session/lifecycle/observations.rs:253` |
| `pocketstation::session::lifecycle::observations::SessionEventQueueObservations` | struct | Point-in-time observations for a session's bounded control-event queue. | `src/session/lifecycle/observations.rs:17` |
| `pocketstation::session::lifecycle::observations::SessionMetricsSnapshot` | struct | Authoritative point-in-time observations for the current Session boundary. | `src/session/lifecycle/observations.rs:36` |
| `pocketstation::session::lifecycle::observations::SessionRouteDropObservations` | struct | Explicit numerator, denominator, interval, and typed reasons for one route. | `src/session/lifecycle/observations.rs:157` |
| `pocketstation::session::lifecycle::observations::SessionRouteLatencyObservations` | struct | Common-clock source timestamp to route-receive latency in nanoseconds. | `src/session/lifecycle/observations.rs:182` |
| `pocketstation::session::lifecycle::observations::SessionSidecarMetrics` | struct | Exact bounded-queue and process-lifecycle accounting for one Session-owned language-neutral sidecar. | `src/session/lifecycle/observations.rs:133` |
| `pocketstation::session::lifecycle::start_contract::SessionStartCancellation` | struct | Thread-safe cancellation request for a Session that has not reached `Running` yet. | `src/session/lifecycle/start_contract.rs:98` |
| `pocketstation::session::lifecycle::events::SessionComponentId` | enum | Stable identity of the component that produced a session control failure. | `src/session/lifecycle/events.rs:51` |
| `pocketstation::session::lifecycle::events::SessionEventKind` | enum | Payload of one authoritative session event. | `src/session/lifecycle/events.rs:294` |
| `pocketstation::session::lifecycle::events::SessionEventReceive` | enum | Result of non-blocking event polling. | `src/session/lifecycle/events.rs:492` |
| `pocketstation::session::lifecycle::events::SessionFinalizationStage` | enum | The finalization operation that failed while stopping a session. | `src/session/lifecycle/events.rs:39` |
| `pocketstation::session::lifecycle::events::SessionLifecycleState` | enum | Public lifecycle states emitted by a running session. | `src/session/lifecycle/events.rs:19` |

## Where you encounter it

- **Reach first captured frames** — Build a Session that captures an application and microphone and polls their independent frames.
- **Record separate stems** — Record independent source stems and inspect finalization outcomes after Session stop.
- **Diagnose a running Session** — Correlate events, metrics, trace records, stable error codes, and terminal outcomes.

## Behavior established by tests

The following test bodies are evidence only for their recorded setup:

- `given_public_session_pcm_output_when_reentered_then_audio_lane_and_lifecycle_are_observed` — given public session pcm output when reentered then audio lane and lifecycle are observed (`src/session/lifecycle/tests/running.rs:2133`; `test-ae4fbbb0bfe0bcb0aff0`).
- `given_public_facade_when_session_trace_enabled_then_trace_replays_complete_lifecycle` — given public facade when session trace enabled then trace replays complete lifecycle (`tests/session_facade.rs:71`; `test-4f8b8179e33a1ceba291`).
- `given_tokio_caller_when_sync_lifecycle_executes_then_no_nested_runtime_panics` — given tokio caller when sync lifecycle executes then no nested runtime panics (`src/runtime/lifecycle/async_host.rs:129`; `test-905856446bbeb46b221d`).
- `given_compiled_lineaged_edge_when_worker_runs_then_exact_session_stem_is_preserved` — given compiled lineaged edge when worker runs then exact session stem is preserved (`src/runtime/signal/operator.rs:2208`; `test-6615dcd3b3105010af0b`).
- `given_cloned_stem_when_session_frozen_then_mutation_is_rejected` — given cloned stem when session frozen then mutation is rejected (`src/session/declaration/draft.rs:1263`; `test-1682e00b3166c4846a92`).
- `given_derived_stream_when_through_called_again_then_chain_is_preserved_in_session_spec` — given derived stream when through called again then chain is preserved in session spec (`src/session/declaration/draft.rs:1179`; `test-837dd73be7d5c552ef15`).
- `given_operator_when_declared_then_session_scoped_instance_and_routes_are_preserved` — given operator when declared then session scoped instance and routes are preserved (`src/session/declaration/draft.rs:1317`; `test-69203660038a41959c14`).
- `given_unrouted_stem_when_session_frozen_then_validation_fails_closed` — given unrouted stem when session frozen then validation fails closed (`src/session/declaration/draft.rs:1305`; `test-1633b6167eec91db04e2`).
- `given_endpoint_operator_id_when_imported_from_session_then_endpoint_contract_type_is_reexported` — given endpoint operator id when imported from session then endpoint contract type is reexported (`src/session/declaration/endpoint.rs:174`; `test-c1047cbdeb5a7bf9bc3b`).
- `given_duplicate_type_when_session_nodes_registered_then_registry_is_unchanged` — given duplicate type when session nodes registered then registry is unchanged (`src/session/extensions/builtins.rs:565`; `test-417c35b0251dee5fe0b7`).
- `given_structural_ingress_when_validated_then_session_metadata_is_not_required` — given structural ingress when validated then session metadata is not required (`src/session/extensions/builtins.rs:636`; `test-7406cc23117530680012`).
- `register_session_graph_nodes` — register session graph nodes (`src/session/extensions/builtins.rs:33`; `test-3d5f82ddbefbd9cd1a57`).

## Boundaries

The compiler inventory establishes names, kinds, visibility, and signatures. Tests establish only their exercised conditions. Where retryability, ordering, cancellation, physical qualification, or recovery is not declared, this page leaves it unspecified.

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

This page was verified against Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `src/session/lifecycle/engine.rs:1-396` (`DIRECT`)
- `src/session/lifecycle/running.rs:1-2612` (`DIRECT`)

A file's presence proves implementation or declaration at this snapshot. It does not by itself prove physical-device qualification, operational performance, retry safety, or behavior outside the recorded test conditions.
