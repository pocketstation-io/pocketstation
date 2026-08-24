# Endpoint lifecycle

<!-- claims: CLM-DOC-024-SCOPE-001,CLM-DOC-024-TEXT-001,CLM-DOC-024-TEXT-002,CLM-DOC-024-TEXT-003,CLM-DOC-024-TEXT-004,CLM-DOC-024-TEXT-005,CLM-DOC-024-TEXT-006,CLM-DOC-024-SOURCE-001 -->

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
| `pocketstation::endpoint::contract::EndpointDriverFactory` | trait | Implement this trait to provide endpoint behavior to PocketStation; its methods define the preparation and runtime contract. | `src/endpoint/contract.rs:262` |
| `pocketstation::endpoint::runtime::PreparedEndpointDriver` | trait | Prepared endpoint resources that have not started consuming their edge. | `src/endpoint/runtime.rs:318` |
| `pocketstation::endpoint::runtime::RunningEndpointDriver` | trait | Active endpoint resources owned until finalization. | `src/endpoint/runtime.rs:336` |
| `pocketstation::endpoint::contract::EndpointAudioFrame` | struct | Read-only audio frame delivered to an external endpoint. | `src/endpoint/contract.rs:18` |
| `pocketstation::endpoint::contract::EndpointAudioReceiver` | struct | Exclusive consumer for one bounded realtime-audio endpoint edge. | `src/endpoint/contract.rs:92` |
| `pocketstation::endpoint::contract::EndpointPortInput` | struct | Carries typed input for endpoint port. | `src/endpoint/contract.rs:183` |
| `pocketstation::endpoint::contract::EndpointSignalReceiver` | struct | Exclusive consumer for one bounded asynchronous signal endpoint edge. | `src/endpoint/contract.rs:153` |
| `pocketstation::endpoint::registry::EndpointDriverRegistry` | struct | Indexes registered endpoint driver implementations by their stable identities. | `src/endpoint/registry.rs:54` |
| `pocketstation::endpoint::runtime::EndpointCancellationOutcome` | struct | Reports the structured endpoint cancellation outcome. | `src/endpoint/runtime.rs:289` |
| `pocketstation::endpoint::runtime::EndpointDriverFinalization` | struct | Reports an endpoint driver's terminal observations and any finalization failure. | `src/endpoint/runtime.rs:295` |

## Executable evidence

Executable evidence selected for **Endpoint lifecycle** is limited to each test's recorded setup and assertions:

- `given_already_open_start_gate_when_endpoint_start_requested_then_start_fails_recoverably` — given already open start gate when endpoint start requested then start fails recoverably (`src/endpoint/registry/tests.rs:272`; `test-70431fbc7f2633c86453`).
- `given_closed_start_gate_when_endpoint_starts_then_delivery_waits_until_session_opens_gate` — given closed start gate when endpoint starts then delivery waits until session opens gate (`src/endpoint/registry/tests.rs:250`; `test-20a233e58338864c5c2f`).
- `given_prior_preparation_when_next_endpoint_fails_then_prior_endpoint_rolls_back_explicitly` — given prior preparation when next endpoint fails then prior endpoint rolls back explicitly (`src/endpoint/registry/tests.rs:225`; `test-32d4cc9ba109d39a6287`).
- `given_endpoint_context_when_constructed_then_route_and_timeline_are_required` — given endpoint context when constructed then route and timeline are required (`src/endpoint/runtime/tests.rs:4`; `test-fe33c3a3e33431f70a5e`).
- `given_text_operator_sent_to_audio_endpoint_when_compiled_then_signal_mismatch_is_typed` — given text operator sent to audio endpoint when compiled then signal mismatch is typed (`src/session/compile/tests.rs:709`; `test-76c3eb4c4fd13e959a1c`).
- `given_connector_endpoint_when_declared_then_allocated_identity_is_exposed` — given connector endpoint when declared then allocated identity is exposed (`src/session/declaration/draft.rs:1154`; `test-9f5f29ec97239eab31dd`).
- `given_empty_operator_id_when_endpoint_declared_then_descriptor_is_rejected` — given empty operator id when endpoint declared then descriptor is rejected (`src/session/declaration/draft.rs:1282`; `test-acb91c715c26103d2747`).
- `given_foreign_endpoint_when_derived_route_declared_then_error_is_immediate` — given foreign endpoint when derived route declared then error is immediate (`src/session/declaration/draft.rs:1378`; `test-c3e77a402eefae2294d5`).
- `given_foreign_endpoint_when_route_declared_then_error_is_immediate` — given foreign endpoint when route declared then error is immediate (`src/session/declaration/draft.rs:1235`; `test-de5b0287b18425cccf03`).
- `given_two_stems_when_sent_to_one_endpoint_then_routes_are_distinct` — given two stems when sent to one endpoint then routes are distinct (`src/session/declaration/draft.rs:1144`; `test-3597ca95a8f2aa93709b`).
- `given_endpoint_operator_id_when_imported_from_session_then_endpoint_contract_type_is_reexported` — given endpoint operator id when imported from session then endpoint contract type is reexported (`src/session/declaration/endpoint.rs:174`; `test-0b2dadbe3265dde022e4`).
- `given_provider_owned_endpoint_key_when_validated_then_core_keeps_it_open` — given provider owned endpoint key when validated then core keeps it open (`src/session/declaration/endpoint.rs:183`; `test-ea23688d79c3f65ded44`).

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

The claims on **Endpoint lifecycle** are anchored to Git snapshot `136e74888962558aa846d3143a19136a70936f45` and these primary files:

- `src/endpoint/contract.rs:18-22` (`DIRECT`)
- `src/endpoint/contract.rs:19-19` (`DIRECT`)
- `src/endpoint/contract.rs:20-20` (`DIRECT`)
- `src/endpoint/contract.rs:21-21` (`DIRECT`)
- `src/endpoint/contract.rs:25-35` (`DIRECT`)
- `src/endpoint/contract.rs:37-39` (`DIRECT`)
- `src/endpoint/contract.rs:41-43` (`DIRECT`)
- `src/endpoint/contract.rs:45-47` (`DIRECT`)
- `src/endpoint/contract.rs:49-51` (`DIRECT`)
- `src/endpoint/contract.rs:53-55` (`DIRECT`)
- `src/endpoint/contract.rs:57-59` (`DIRECT`)
- `src/endpoint/contract.rs:61-63` (`DIRECT`)
- `src/endpoint/contract.rs:65-67` (`DIRECT`)
- `src/endpoint/contract.rs:69-71` (`DIRECT`)
- `src/endpoint/contract.rs:73-75` (`DIRECT`)
- `src/endpoint/contract.rs:78-80` (`DIRECT`)
- `src/endpoint/contract.rs:83-85` (`DIRECT`)
- `src/endpoint/contract.rs:92-94` (`DIRECT`)
- `src/endpoint/contract.rs:93-93` (`DIRECT`)
- `src/endpoint/contract.rs:97-99` (`DIRECT`)
- `src/endpoint/contract.rs:101-103` (`DIRECT`)
- `src/endpoint/contract.rs:106-108` (`DIRECT`)
- `src/endpoint/contract.rs:110-120` (`DIRECT`)
- `src/endpoint/contract.rs:122-124` (`DIRECT`)
- `src/endpoint/runtime.rs:12-12` (`DIRECT`)
- `src/endpoint/runtime.rs:12-12` (`DIRECT`)
- `src/endpoint/runtime.rs:12-12` (`DIRECT`)
- `src/endpoint/runtime.rs:13-15` (`DIRECT`)
- `src/endpoint/runtime.rs:14-14` (`DIRECT`)
- `src/endpoint/runtime.rs:18-22` (`DIRECT`)
- `src/endpoint/runtime.rs:24-26` (`DIRECT`)
- `src/endpoint/runtime.rs:30-30` (`DIRECT`)
- `src/endpoint/runtime.rs:30-30` (`DIRECT`)
- `src/endpoint/runtime.rs:30-30` (`DIRECT`)
- `src/endpoint/runtime.rs:31-40` (`DIRECT`)
- `src/endpoint/runtime.rs:32-32` (`DIRECT`)
- `src/endpoint/runtime.rs:32-32` (`DIRECT`)
- `src/endpoint/runtime.rs:34-34` (`DIRECT`)
- `src/endpoint/runtime.rs:35-39` (`DIRECT`)
- `src/endpoint/runtime.rs:36-36` (`DIRECT`)
- `src/endpoint/runtime.rs:37-37` (`DIRECT`)
- `src/endpoint/runtime.rs:38-38` (`DIRECT`)
- `src/endpoint/runtime.rs:43-43` (`DIRECT`)
- `src/endpoint/runtime.rs:43-43` (`DIRECT`)
- `src/endpoint/runtime.rs:43-43` (`DIRECT`)
- `src/endpoint/runtime.rs:44-47` (`DIRECT`)
- `src/endpoint/runtime.rs:45-45` (`DIRECT`)
- `src/endpoint/runtime.rs:46-46` (`DIRECT`)

For **Endpoint lifecycle**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.
