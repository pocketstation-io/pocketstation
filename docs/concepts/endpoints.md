# Endpoint lifecycle

<!-- claims: CLM-DOC-024-CAP-001,CLM-DOC-024-SOURCE-001 -->

Prepare, start, receive, cancel, and finalize destinations behind the endpoint driver contract.

## Scope

- **Implement endpoint drivers.** Prepare, start, receive, cancel, and finalize destinations behind the endpoint driver contract.

These statements describe repository contracts at the documented snapshot. They do not extend platform qualification, performance, retry, or delivery guarantees beyond the native API contracts and executable evidence.

## Contract surface

| Public declaration | Kind | Declared purpose | Source |
|---|---|---|---|
| `pocketstation::session::lifecycle::events::SessionEndpointFailure` | struct | Endpoint failure associated with one stable route and endpoint. | `src/session/lifecycle/events.rs:125` |
| `pocketstation::session::lifecycle::engine::EndpointExtensionRegistrationError` | enum | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/session/lifecycle/engine.rs:305` |
| `pocketstation::session::lifecycle::observations::EndpointObservationStage` | enum | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/session/lifecycle/observations.rs:441` |
| `pocketstation::session::lifecycle::engine::EndpointExtensionRegistrationError::ConflictingDefinition` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/session/lifecycle/engine.rs:311` |
| `pocketstation::session::lifecycle::engine::EndpointExtensionRegistrationError::Definition` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/session/lifecycle/engine.rs:307` |
| `pocketstation::session::lifecycle::engine::EndpointExtensionRegistrationError::Driver` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/session/lifecycle/engine.rs:309` |
| `pocketstation::session::lifecycle::events::SessionComponentId::Endpoint` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/session/lifecycle/events.rs:55` |
| `pocketstation::session::lifecycle::events::SessionEventKind::Endpoint` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/session/lifecycle/events.rs:297` |
| `pocketstation::session::lifecycle::events::SessionFinalizationStage::FinalizeEndpoint` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/session/lifecycle/events.rs:45` |
| `pocketstation::session::lifecycle::events::SessionFinalizationStage::JoinEndpoint` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/session/lifecycle/events.rs:44` |
| `pocketstation::session::lifecycle::events::SessionFinalizationStage::RequestEndpointStop` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/session/lifecycle/events.rs:43` |
| `pocketstation::session::lifecycle::events::SessionRollbackStage::CancelEndpointPreparation` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/session/lifecycle/events.rs:31` |
| `pocketstation::session::lifecycle::events::SessionRollbackStage::FinalizeStartedEndpoint` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/session/lifecycle/events.rs:32` |
| `pocketstation::session::lifecycle::host::SessionEngineHostBuildError::EndpointExtensionRegistration` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/session/lifecycle/host.rs:368` |
| `pocketstation::session::lifecycle::host::SessionEngineHostBuildError::EndpointRegistration` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/session/lifecycle/host.rs:366` |
| `pocketstation::session::lifecycle::host::SessionEngineHostBuildError::PolledAudioEndpoint` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/session/lifecycle/host.rs:372` |
| `pocketstation::session::lifecycle::observations::EndpointObservationStage::Finalized` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/session/lifecycle/observations.rs:444` |
| `pocketstation::session::lifecycle::observations::EndpointObservationStage::Live` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/session/lifecycle/observations.rs:443` |
| `pocketstation::session::lifecycle::observations::EndpointObservationStage::Unavailable` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/session/lifecycle/observations.rs:442` |
| `pocketstation::session::lifecycle::start_contract::SessionStartError::EndpointPrepare` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/session/lifecycle/start_contract.rs:152` |

## Where you encounter it

- **Author a connector** — Declare a connector manifest and run its endpoint worker under finite delivery and shutdown policy.

## Behavior established by tests

The following test bodies are evidence only for their recorded setup:

- `given_already_open_start_gate_when_endpoint_start_requested_then_start_fails_recoverably` — given already open start gate when endpoint start requested then start fails recoverably (`src/endpoint/registry/tests.rs:272`; `test-e8c1d06b58a459a61c14`).
- `given_closed_start_gate_when_endpoint_starts_then_delivery_waits_until_session_opens_gate` — given closed start gate when endpoint starts then delivery waits until session opens gate (`src/endpoint/registry/tests.rs:250`; `test-cd73c609f0b99f88ac58`).
- `given_prior_preparation_when_next_endpoint_fails_then_prior_endpoint_rolls_back_explicitly` — given prior preparation when next endpoint fails then prior endpoint rolls back explicitly (`src/endpoint/registry/tests.rs:225`; `test-311a13acdbeb4fafa4a7`).
- `given_endpoint_context_when_constructed_then_route_and_timeline_are_required` — given endpoint context when constructed then route and timeline are required (`src/endpoint/runtime/tests.rs:4`; `test-519ee5868705e874dba9`).
- `endpoint_declarations` — endpoint declarations (`src/session/compile/compiled.rs:39`; `test-45244638cf19287785e6`).
- `given_text_operator_sent_to_audio_endpoint_when_compiled_then_signal_mismatch_is_typed` — given text operator sent to audio endpoint when compiled then signal mismatch is typed (`src/session/compile/tests.rs:709`; `test-3c9cfc84af12d388d72a`).
- `given_connector_endpoint_when_declared_then_allocated_identity_is_exposed` — given connector endpoint when declared then allocated identity is exposed (`src/session/declaration/draft.rs:1166`; `test-161b990db51190f37641`).
- `given_empty_operator_id_when_endpoint_declared_then_descriptor_is_rejected` — given empty operator id when endpoint declared then descriptor is rejected (`src/session/declaration/draft.rs:1294`; `test-3094497c2340606ba13f`).
- `given_foreign_endpoint_when_derived_route_declared_then_error_is_immediate` — given foreign endpoint when derived route declared then error is immediate (`src/session/declaration/draft.rs:1390`; `test-cdff89219c2eef8f0d45`).
- `given_foreign_endpoint_when_route_declared_then_error_is_immediate` — given foreign endpoint when route declared then error is immediate (`src/session/declaration/draft.rs:1247`; `test-c03c1e7f3591b6b9a633`).
- `given_two_stems_when_sent_to_one_endpoint_then_routes_are_distinct` — given two stems when sent to one endpoint then routes are distinct (`src/session/declaration/draft.rs:1156`; `test-8ad7b42874649ad3c238`).
- `given_endpoint_operator_id_when_imported_from_session_then_endpoint_contract_type_is_reexported` — given endpoint operator id when imported from session then endpoint contract type is reexported (`src/session/declaration/endpoint.rs:174`; `test-c1047cbdeb5a7bf9bc3b`).

## Boundaries

The compiler inventory establishes names, kinds, visibility, and signatures. Tests establish only their exercised conditions. Where retryability, ordering, cancellation, physical qualification, or recovery is not declared, this page leaves it unspecified.

## Related documentation

- [Glossary](/docs/glossary.md)
- [PocketStation](/README.md)
- [Stop, drain, and finalization](/docs/lifecycle/stop-drain-finalize.md)
- [Implement an endpoint driver](/docs/how-to/implement-endpoint.md)
- [Stop a Session and inspect failures](/docs/how-to/stop-session.md)
- [Behavior evidence index](/docs/reference/behavior-evidence.md)
- [Endpoint API](/docs/reference/endpoints.md)
- [Lifecycle evidence index](/docs/reference/lifecycle-evidence.md)

## Evidence boundary

This page was verified against Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `src/endpoint/contract.rs:1-246` (`DIRECT`)
- `src/endpoint/runtime.rs:1-531` (`DIRECT`)

A file's presence proves implementation or declaration at this snapshot. It does not by itself prove physical-device qualification, operational performance, retry safety, or behavior outside the recorded test conditions.
