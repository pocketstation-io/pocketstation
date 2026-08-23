# Instrument a Session

<!-- claims: CLM-GUIDE-021-CAP-001,CLM-GUIDE-021-CAP-002,CLM-GUIDE-021-SOURCE-001 -->

## Scope

- **Observe Session metrics and events.** Read route, source, operator, sidecar, endpoint, drop, latency, queue, and terminal observations.
- **Record and validate Session traces.** Persist lifecycle trace records and validate their structural and terminal consistency.

These statements describe repository contracts at the documented snapshot. They do not extend platform qualification, performance, retry, or delivery guarantees beyond the native API contracts and executable evidence.

## Prerequisites

Read the linked concept and confirm that target platform, Cargo features, source or provider dependencies, and application-owned permission work match this task. Keep returned typed errors and outcomes available for verification.

## Procedure

1. Acquire observation handles before the period you need to inspect.
2. Snapshot metrics by stable component IDs.
3. Record a trace for durable lifecycle evidence.
4. Stop and include the terminal outcome.
5. Validate trace structure before diagnosis.

## APIs used

| Public declaration | Kind | Declared purpose | Source |
|---|---|---|---|
| `pocketstation::session::declaration::typed_stream::StreamSignal` | trait | Compile-time marker supplied by an SDK or external package. | `src/session/declaration/typed_stream.rs:15` |
| `pocketstation::session::declaration::spec::ConnectionSpec` | struct | The single Session connection declaration used for every stream origin and every operator/endpoint destination. | `src/session/declaration/spec.rs:238` |
| `pocketstation::session::declaration::typed_stream::Stream` | struct | Typed Rust declaration façade compiled into stable dynamic signal, schema, port, and edge contracts. This wrapper carries no frames and is not a generic runtime queue. | `src/session/declaration/typed_stream.rs:96` |
| `pocketstation::session::extensions::audio_input::AudioInput` | struct | Intent-first façade for feeding audio already owned by the embedding application into a Session. | `src/session/extensions/audio_input/mod.rs:94` |
| `pocketstation::session::extensions::audio_input::source::PcmSource` | struct | Low-level PCM source ownership for integrations that separately retain the Session handles and producer writer. | `src/session/extensions/audio_input/source.rs:33` |
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

## Verify the outcome

The following test bodies are evidence only for their recorded setup:

- `given_compiled_lineaged_edge_when_worker_runs_then_exact_session_stem_is_preserved` — given compiled lineaged edge when worker runs then exact session stem is preserved (`src/runtime/signal/operator.rs:2208`; `test-6615dcd3b3105010af0b`).
- `given_cloned_stem_when_session_frozen_then_mutation_is_rejected` — given cloned stem when session frozen then mutation is rejected (`src/session/declaration/draft.rs:1263`; `test-1682e00b3166c4846a92`).
- `given_derived_stream_when_through_called_again_then_chain_is_preserved_in_session_spec` — given derived stream when through called again then chain is preserved in session spec (`src/session/declaration/draft.rs:1179`; `test-837dd73be7d5c552ef15`).
- `given_operator_when_declared_then_session_scoped_instance_and_routes_are_preserved` — given operator when declared then session scoped instance and routes are preserved (`src/session/declaration/draft.rs:1317`; `test-69203660038a41959c14`).
- `given_unrouted_stem_when_session_frozen_then_validation_fails_closed` — given unrouted stem when session frozen then validation fails closed (`src/session/declaration/draft.rs:1305`; `test-1633b6167eec91db04e2`).
- `given_endpoint_operator_id_when_imported_from_session_then_endpoint_contract_type_is_reexported` — given endpoint operator id when imported from session then endpoint contract type is reexported (`src/session/declaration/endpoint.rs:174`; `test-c1047cbdeb5a7bf9bc3b`).
- `given_duplicate_type_when_session_nodes_registered_then_registry_is_unchanged` — given duplicate type when session nodes registered then registry is unchanged (`src/session/extensions/builtins.rs:565`; `test-417c35b0251dee5fe0b7`).
- `given_structural_ingress_when_validated_then_session_metadata_is_not_required` — given structural ingress when validated then session metadata is not required (`src/session/extensions/builtins.rs:636`; `test-7406cc23117530680012`).
- `register_session_graph_nodes` — register session graph nodes (`src/session/extensions/builtins.rs:33`; `test-3d5f82ddbefbd9cd1a57`).
- `given_custom_source_output_when_compiled_then_session_identity_and_typed_plan_are_preserved` — given custom source output when compiled then session identity and typed plan are preserved (`src/session/extensions/tests/composition.rs:300`; `test-4e5aeb90e44be738cb79`).
- `given_external_pcm_source_when_session_runs_then_audio_uses_bounded_ingress_with_source_identity` — given external pcm source when session runs then audio uses bounded ingress with source identity (`src/session/extensions/tests/runtime.rs:823`; `test-1d9f4de1e64929bbc714`).
- `given_one_external_source_failure_when_session_runs_then_unrelated_source_completes` — given one external source failure when session runs then unrelated source completes (`src/session/extensions/tests/runtime.rs:734`; `test-bdedfb8ca6cbd5442810`).

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

- [Glossary](/docs/glossary.md)
- [PocketStation](/README.md)
- [Behavior evidence index](/docs/reference/behavior-evidence.md)
- [Observation API](/docs/reference/observations.md)
- [Rust API reference](/docs/reference/rust-api.md)
- [Session trace validation fails](/docs/troubleshooting/session-trace.md)
- [Observations and metrics](/docs/concepts/observability.md)
- [Running ownership](/docs/lifecycle/running.md)

## Evidence boundary

This page was verified against Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `src/session/lifecycle/observations.rs:1-636` (`DIRECT`)
- `src/session/lifecycle/trace.rs:1-1179` (`DIRECT`)

A file's presence proves implementation or declaration at this snapshot. It does not by itself prove physical-device qualification, operational performance, retry safety, or behavior outside the recorded test conditions.
