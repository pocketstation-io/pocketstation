# Implement an endpoint driver

<!-- claims: CLM-GUIDE-014-CAP-001,CLM-GUIDE-014-SOURCE-001 -->

## Scope

- **Implement endpoint drivers.** Prepare, start, receive, cancel, and finalize destinations behind the endpoint driver contract.

The scope of **Implement an endpoint driver** ends at the native contracts and executable conditions cited below. Platform qualification, performance, retry, and delivery require their own explicit evidence.

## Prerequisites

An endpoint manifest with declared inputs and a factory that can prepare resources without starting delivery early.

## Procedure

1. Implement EndpointDriverFactory preparation.
2. Return a prepared driver with its start gate.
3. Consume matching audio or signal inputs.
4. Honor cancellation and shutdown mode.
5. Return finalization observations and staged failures.

## Important consequence

Preserve the exact endpoint failure stage and retryability classification.

## Verify the outcome

The start gate reports readiness, declared inputs are consumed, and finalization returns observations or a staged failure.

Executable evidence selected for **Implement an endpoint driver** is limited to each test's recorded setup and assertions:

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

## Failure signals

- `pocketstation::endpoint::polled_audio_driver::PolledAudioEndpointConfigError` — `error-aec9407f95faa6af7f1b`
- `pocketstation::endpoint::polled_audio_driver::PolledAudioEndpointConfigError` / `BatchCapacityTooLarge` — `error-3cf47be95ecbd48c4258`
- `pocketstation::endpoint::polled_audio_driver::PolledAudioEndpointConfigError` / `LeaseCapacityTooLarge` — `error-a8e01e3a55a9228d8a02`
- `pocketstation::endpoint::polled_audio_driver::PolledAudioEndpointConfigError` / `QueueCapacityTooLarge` — `error-c03edc8fd65e68ee052b`
- `pocketstation::endpoint::polled_audio_driver::PolledAudioEndpointConfigError` / `ZeroBatchCapacity` — `error-76dfb07982404f1eb108`
- `pocketstation::endpoint::polled_audio_driver::PolledAudioEndpointConfigError` / `ZeroLeaseCapacity` — `error-7b7d417980a04711815d`
- `pocketstation::endpoint::polled_audio_driver::PolledAudioEndpointConfigError` / `ZeroQueueCapacity` — `error-f1dc5f67b39cdfcdbe0f`
- `pocketstation::endpoint::polled_audio_driver::PolledAudioPollError` — `error-d0a72409314739fa830e`
- `pocketstation::endpoint::polled_audio_driver::PolledAudioPollError` / `Empty` — `error-bc1c2a632fd4521e98a0`
- `pocketstation::endpoint::polled_audio_driver::PolledAudioPollError` / `LeaseCapacityExhausted` — `error-aefa74a1ce402362646b`

## API reference

- [Endpoints](/docs/concepts/endpoints.md)
- [Endpoints](/docs/reference/endpoints.md)

| Public declaration | Kind | Declared purpose | Source |
|---|---|---|---|
| `pocketstation::endpoint::contract::EndpointDriverFactory` | trait | Implement this trait to provide endpoint behavior to PocketStation; its methods define the preparation and runtime contract. | `src/endpoint/contract.rs:262` |
| `pocketstation::endpoint::runtime::PreparedEndpointDriver` | trait | Prepared endpoint resources that have not started consuming their edge. | `src/endpoint/runtime.rs:318` |
| `pocketstation::endpoint::runtime::RunningEndpointDriver` | trait | Active endpoint resources owned until finalization. | `src/endpoint/runtime.rs:336` |
| `pocketstation::endpoint::registry::EndpointDriverRegistry` | struct | Indexes registered endpoint driver implementations by their stable identities. | `src/endpoint/registry.rs:54` |
| `pocketstation::endpoint::runtime::EndpointDriverFinalization` | struct | Reports an endpoint driver's terminal observations and any finalization failure. | `src/endpoint/runtime.rs:295` |
| `pocketstation::endpoint::runtime::EndpointDriverObservations` | struct | Reports the endpoint driver observations collected at an observation boundary. | `src/endpoint/runtime.rs:228` |
| `pocketstation::endpoint::registry::EndpointDriverRegistryError` | enum | Classifies failures reported as endpoint driver registry error. | `src/endpoint/registry.rs:16` |
| `EndpointDriverFactory::preparation_group` | function | Returns the preparation group associated with `EndpointDriverFactory`. | `src/endpoint/contract.rs:263` |

## Related documentation

- [Endpoint lifecycle](/docs/concepts/endpoints.md)
- [Glossary](/docs/glossary.md)
- [PocketStation](/README.md)
- [Stop, drain, and finalization](/docs/lifecycle/stop-drain-finalize.md)
- [Stop a Session and inspect failures](/docs/how-to/stop-session.md)
- [Behavior evidence index](/docs/reference/behavior-evidence.md)
- [Endpoint API](/docs/reference/endpoints.md)
- [Lifecycle evidence index](/docs/reference/lifecycle-evidence.md)

## Evidence boundary

The claims on **Implement an endpoint driver** are anchored to Git snapshot `136e74888962558aa846d3143a19136a70936f45` and these primary files:

- `src/endpoint/contract.rs:1-276` (`DIRECT`)

For **Implement an endpoint driver**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.
