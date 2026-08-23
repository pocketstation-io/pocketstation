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

## Failure signals

- `pocketstation::endpoint::polled_audio_driver::PolledAudioEndpointConfigError` — `error-66ac5d9011f97956ae49`
- `pocketstation::endpoint::polled_audio_driver::PolledAudioEndpointConfigError` / `BatchCapacityTooLarge` — `error-0c9244cc9373ac29c5c7`
- `pocketstation::endpoint::polled_audio_driver::PolledAudioEndpointConfigError` / `LeaseCapacityTooLarge` — `error-ecb39ec205969437feae`
- `pocketstation::endpoint::polled_audio_driver::PolledAudioEndpointConfigError` / `QueueCapacityTooLarge` — `error-b0234424a7dd69ec9068`
- `pocketstation::endpoint::polled_audio_driver::PolledAudioEndpointConfigError` / `ZeroBatchCapacity` — `error-8c0385abc6ec91e71a03`
- `pocketstation::endpoint::polled_audio_driver::PolledAudioEndpointConfigError` / `ZeroLeaseCapacity` — `error-3f8407385588d4a9a9f5`
- `pocketstation::endpoint::polled_audio_driver::PolledAudioEndpointConfigError` / `ZeroQueueCapacity` — `error-613c9d5cd2cfce29de24`
- `pocketstation::endpoint::polled_audio_driver::PolledAudioPollError` — `error-6046c03c1d5b093cca8f`
- `pocketstation::endpoint::polled_audio_driver::PolledAudioPollError` / `Empty` — `error-a85f11643a5af4bd024f`
- `pocketstation::endpoint::polled_audio_driver::PolledAudioPollError` / `LeaseCapacityExhausted` — `error-34d86ad1319d8eb0a9d0`

## API reference

- [Endpoints](/docs/concepts/endpoints.md)
- [Endpoints](/docs/reference/endpoints.md)

| Public declaration | Kind | Declared purpose | Source |
|---|---|---|---|
| `pocketstation::endpoint::contract::EndpointDriverFactory` | trait | Implement this trait to provide endpoint behavior to PocketStation; its methods define the preparation and runtime contract. | `src/endpoint/contract.rs:232` |
| `pocketstation::endpoint::runtime::PreparedEndpointDriver` | trait | Prepared endpoint resources that have not started consuming their edge. | `src/endpoint/runtime.rs:318` |
| `pocketstation::endpoint::runtime::RunningEndpointDriver` | trait | Active endpoint resources owned until finalization. | `src/endpoint/runtime.rs:336` |
| `pocketstation::endpoint::registry::EndpointDriverRegistry` | struct | Indexes registered endpoint driver implementations by their stable identities. | `src/endpoint/registry.rs:54` |
| `pocketstation::endpoint::runtime::EndpointDriverFinalization` | struct | Reports an endpoint driver's terminal observations and any finalization failure. | `src/endpoint/runtime.rs:295` |
| `pocketstation::endpoint::runtime::EndpointDriverObservations` | struct | Reports the endpoint driver observations collected at an observation boundary. | `src/endpoint/runtime.rs:228` |
| `pocketstation::endpoint::registry::EndpointDriverRegistryError` | enum | Classifies failures reported as endpoint driver registry error. | `src/endpoint/registry.rs:16` |
| `EndpointDriverFactory::preparation_group` | function | Returns the preparation group associated with `EndpointDriverFactory`. | `src/endpoint/contract.rs:233` |

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

The claims on **Implement an endpoint driver** are anchored to Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `src/endpoint/contract.rs:1-246` (`DIRECT`)

For **Implement an endpoint driver**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.
