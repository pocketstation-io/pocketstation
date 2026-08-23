# Implement an endpoint driver

<!-- claims: CLM-GUIDE-014-CAP-001,CLM-GUIDE-014-SOURCE-001 -->

## Scope

- **Implement endpoint drivers.** Prepare, start, receive, cancel, and finalize destinations behind the endpoint driver contract.

These statements describe repository contracts at the documented snapshot. They do not extend platform qualification, performance, retry, or delivery guarantees beyond the native API contracts and executable evidence.

## Prerequisites

Read the linked concept and confirm that target platform, Cargo features, source or provider dependencies, and application-owned permission work match this task. Keep returned typed errors and outcomes available for verification.

## Procedure

1. Implement EndpointDriverFactory preparation.
2. Return a prepared driver with its start gate.
3. Consume matching audio or signal inputs.
4. Honor cancellation and shutdown mode.
5. Return finalization observations and staged failures.

## APIs used

| Public declaration | Kind | Declared purpose | Source |
|---|---|---|---|
| `pocketstation::endpoint::runtime::PreparedEndpointDriver` | trait | Prepared endpoint resources that have not started consuming their edge. | `src/endpoint/runtime.rs:318` |
| `pocketstation::endpoint::runtime::RunningEndpointDriver` | trait | Active endpoint resources owned until finalization. | `src/endpoint/runtime.rs:336` |
| `PreparedEndpointDriver::start` | function | Makes endpoint resources ready behind the supplied closed gate. | `src/endpoint/runtime.rs:323` |
| `pocketstation::endpoint::contract::EndpointDriverFactory` | trait | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/endpoint/contract.rs:232` |
| `pocketstation::endpoint::polled_audio_driver::PolledAudioBatchLease` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/endpoint/polled_audio_driver.rs:172` |
| `pocketstation::endpoint::polled_audio_driver::PolledAudioFrame` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/endpoint/polled_audio_driver.rs:210` |
| `pocketstation::endpoint::polled_audio_driver::PolledAudioObservations` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/endpoint/polled_audio_driver.rs:56` |
| `pocketstation::endpoint::runtime::EndpointDriverFinalization` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/endpoint/runtime.rs:295` |
| `pocketstation::endpoint::runtime::EndpointDriverObservations` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/endpoint/runtime.rs:228` |
| `pocketstation::endpoint::polled_audio_driver::PolledAudioEndpointConfigError` | enum | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/endpoint/polled_audio_driver.rs:40` |
| `pocketstation::endpoint::polled_audio_driver::PolledAudioPollError` | enum | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/endpoint/polled_audio_driver.rs:74` |
| `pocketstation::endpoint::registry::EndpointDriverRegistryError` | enum | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/endpoint/registry.rs:16` |
| `EndpointDriverFactory::preparation_group` | function | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/endpoint/contract.rs:233` |
| `EndpointDriverFactory::prepare` | function | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/endpoint/contract.rs:241` |
| `PreparedEndpointDriver::cancel_preparation` | function | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/endpoint/runtime.rs:328` |
| `RunningEndpointDriver::join_and_finalize` | function | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/endpoint/runtime.rs:346` |

## Verify the outcome

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

## Failure signals

- `pocketstation::session::declaration::typed_stream::TypedStreamError` / `OutputSignalMismatch` — `error-00e5716261eba0f8cf3d`
- `pocketstation::session::error::SessionError` / `UnknownStem` — `error-00f6e798d158df66c847`
- `pocketstation::session::error_code::SessionStartErrorCode` / `StartCancelled` — `error-01d3fc855e2a00319076`
- `pocketstation::session::lifecycle::start_contract::SessionStartError` / `OperatorPrepare` — `error-023d6ab0b23a50a614ff`
- `pocketstation::endpoint::runtime::EndpointFailureStage` / `CancelPreparation` — `error-0265bb447764629fa47b`
- `pocketstation::session::error_code::SessionStartErrorCode` / `TraceRecorderSetupFailed` — `error-0279b2b6b0cb3b5801bc`
- `pocketstation::endpoint::polled_audio_driver::PolledAudioEndpointConfigError` / `ZeroLeaseCapacity` — `error-0370b7ecbdf2b9d6fbdb`
- `pocketstation::session::prepare::error::SessionPrepareError` / `MissingOperatorSignalInput` — `error-037ddc3e193da74177f8`
- `pocketstation::session::lifecycle::trace::SessionTraceValidationError` / `InvalidLayout` — `error-05c60389efcb84311921`
- `pocketstation::session::prepare::error::SessionPrepareError` — `error-085082b521c14e5ecd1e`
- `pocketstation::session::extensions::audio_input::buffer::AudioInputWriteErrorKind` / `Closed` — `error-08a7536094bfb2242b17`
- `pocketstation::session::lifecycle::host::SessionEngineHostBuildError` / `EndpointExtensionRegistration` — `error-09837185c7fca0f70618`

Retry only when the relevant API or error contract explicitly permits it. An error name, a transient-looking message, or a successful prior run is not retry evidence.

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

This page was verified against Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `src/endpoint/contract.rs:1-246` (`DIRECT`)

A file's presence proves implementation or declaration at this snapshot. It does not by itself prove physical-device qualification, operational performance, retry safety, or behavior outside the recorded test conditions.
