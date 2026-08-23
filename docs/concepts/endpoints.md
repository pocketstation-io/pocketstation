# Endpoint lifecycle

<!-- claims: CLM-DOC-024-CAP-001,CLM-DOC-024-SOURCE-001 -->

## What it is

The endpoint lifecycle prepares a destination driver, waits for its start gate, delivers declared inputs, requests cancellation or stop, joins its worker, and returns finalization observations.

## Why it exists

Destinations can fail at different stages. A staged contract keeps preparation, start, delivery, cancellation, join, and finalization failures attributable.

## Relationships

- An endpoint manifest declares its inputs and preparation group.
- `EndpointDriverFactory` creates a prepared driver.
- Session stop collects endpoint failures without discarding other component outcomes.

## Invariants and guarantees

- Delivered media must match the endpoint input contract.
- The driver reports readiness through its start gate.
- Finalization is part of terminal outcome handling, not an assumed side effect.

## When you encounter it

- **Author a connector** — Declare a connector manifest and run its endpoint worker under finite delivery and shutdown policy.

## Use it

- [Implement an endpoint driver](/docs/how-to/implement-endpoint.md)
- [Stop, drain, and finalization](/docs/lifecycle/stop-drain-finalize.md)
- [Session stop reports component failures](/docs/troubleshooting/session-stop.md)

## Scope

- **Implement endpoint drivers.** Prepare, start, receive, cancel, and finalize destinations behind the endpoint driver contract.

The scope of **Endpoint lifecycle** ends at the native contracts and executable conditions cited below. Platform qualification, performance, retry, and delivery require their own explicit evidence.

## Key API

| Public declaration | Kind | Declared purpose | Source |
|---|---|---|---|
| `pocketstation::endpoint::contract::EndpointDriverFactory` | trait | Implement this trait to provide endpoint behavior to PocketStation; its methods define the preparation and runtime contract. | `src/endpoint/contract.rs:232` |
| `pocketstation::endpoint::runtime::PreparedEndpointDriver` | trait | Prepared endpoint resources that have not started consuming their edge. | `src/endpoint/runtime.rs:318` |
| `pocketstation::endpoint::runtime::RunningEndpointDriver` | trait | Active endpoint resources owned until finalization. | `src/endpoint/runtime.rs:336` |
| `pocketstation::endpoint::contract::EndpointAudioFrame` | struct | Read-only audio frame delivered to an external endpoint. | `src/endpoint/contract.rs:18` |
| `pocketstation::endpoint::contract::EndpointAudioReceiver` | struct | Exclusive consumer for one bounded realtime-audio endpoint edge. | `src/endpoint/contract.rs:68` |
| `pocketstation::endpoint::contract::EndpointPortInput` | struct | Carries typed input for endpoint port. | `src/endpoint/contract.rs:153` |
| `pocketstation::endpoint::contract::EndpointSignalReceiver` | struct | Exclusive consumer for one bounded asynchronous signal endpoint edge. | `src/endpoint/contract.rs:123` |
| `pocketstation::endpoint::registry::EndpointDriverRegistry` | struct | Indexes registered endpoint driver implementations by their stable identities. | `src/endpoint/registry.rs:54` |
| `pocketstation::endpoint::runtime::EndpointCancellationOutcome` | struct | Reports the structured endpoint cancellation outcome. | `src/endpoint/runtime.rs:289` |
| `pocketstation::endpoint::runtime::EndpointDriverFinalization` | struct | Reports an endpoint driver's terminal observations and any finalization failure. | `src/endpoint/runtime.rs:295` |

## Executable evidence

Executable evidence selected for **Endpoint lifecycle** is limited to each test's recorded setup and assertions:

- `given_already_open_start_gate_when_endpoint_start_requested_then_start_fails_recoverably` — given already open start gate when endpoint start requested then start fails recoverably (`src/endpoint/registry/tests.rs:272`; `test-e8c1d06b58a459a61c14`).
- `given_closed_start_gate_when_endpoint_starts_then_delivery_waits_until_session_opens_gate` — given closed start gate when endpoint starts then delivery waits until session opens gate (`src/endpoint/registry/tests.rs:250`; `test-cd73c609f0b99f88ac58`).
- `given_prior_preparation_when_next_endpoint_fails_then_prior_endpoint_rolls_back_explicitly` — given prior preparation when next endpoint fails then prior endpoint rolls back explicitly (`src/endpoint/registry/tests.rs:225`; `test-311a13acdbeb4fafa4a7`).
- `given_endpoint_context_when_constructed_then_route_and_timeline_are_required` — given endpoint context when constructed then route and timeline are required (`src/endpoint/runtime/tests.rs:4`; `test-519ee5868705e874dba9`).
- `given_text_operator_sent_to_audio_endpoint_when_compiled_then_signal_mismatch_is_typed` — given text operator sent to audio endpoint when compiled then signal mismatch is typed (`src/session/compile/tests.rs:709`; `test-3c9cfc84af12d388d72a`).
- `given_connector_endpoint_when_declared_then_allocated_identity_is_exposed` — given connector endpoint when declared then allocated identity is exposed (`src/session/declaration/draft.rs:1166`; `test-161b990db51190f37641`).
- `given_empty_operator_id_when_endpoint_declared_then_descriptor_is_rejected` — given empty operator id when endpoint declared then descriptor is rejected (`src/session/declaration/draft.rs:1294`; `test-3094497c2340606ba13f`).
- `given_foreign_endpoint_when_derived_route_declared_then_error_is_immediate` — given foreign endpoint when derived route declared then error is immediate (`src/session/declaration/draft.rs:1390`; `test-cdff89219c2eef8f0d45`).
- `given_foreign_endpoint_when_route_declared_then_error_is_immediate` — given foreign endpoint when route declared then error is immediate (`src/session/declaration/draft.rs:1247`; `test-c03c1e7f3591b6b9a633`).
- `given_two_stems_when_sent_to_one_endpoint_then_routes_are_distinct` — given two stems when sent to one endpoint then routes are distinct (`src/session/declaration/draft.rs:1156`; `test-8ad7b42874649ad3c238`).
- `given_endpoint_operator_id_when_imported_from_session_then_endpoint_contract_type_is_reexported` — given endpoint operator id when imported from session then endpoint contract type is reexported (`src/session/declaration/endpoint.rs:174`; `test-c1047cbdeb5a7bf9bc3b`).
- `given_provider_owned_endpoint_key_when_validated_then_core_keeps_it_open` — given provider owned endpoint key when validated then core keeps it open (`src/session/declaration/endpoint.rs:183`; `test-fa9b9109ca59e684b0da`).

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

The claims on **Endpoint lifecycle** are anchored to Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `src/endpoint/contract.rs:1-246` (`DIRECT`)
- `src/endpoint/runtime.rs:1-531` (`DIRECT`)

For **Endpoint lifecycle**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.
