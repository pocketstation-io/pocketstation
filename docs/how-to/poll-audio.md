# Poll audio without unbounded buffering

<!-- claims: CLM-GUIDE-006-CAP-001,CLM-GUIDE-006-CAP-002,CLM-GUIDE-006-SOURCE-001 -->

## Scope

- **Poll bounded audio batches.** Consume routed audio from the built-in polled-audio endpoint through bounded batch leases and receipts.
- **Route realtime audio.** Deliver pooled audio frames through independent fixed-capacity routes governed by explicit edge policy.

These statements describe repository contracts at the documented snapshot. They do not extend platform qualification, performance, retry, or delivery guarantees beyond the native API contracts and executable evidence.

## Prerequisites

Read the linked concept and confirm that target platform, Cargo features, source or provider dependencies, and application-owned permission work match this task. Keep returned typed errors and outcomes available for verification.

## Procedure

1. Declare a separate polled_audio endpoint for each independent route.
2. Send the source or stream to that endpoint.
3. Call try_poll_audio from non-realtime application code.
4. Iterate only indices below the returned batch length.
5. Release the lease promptly and inspect polling observations.

## APIs used

| Public declaration | Kind | Declared purpose | Source |
|---|---|---|---|
| `pocketstation::session::error_code::PolledAudioPollErrorCode` | enum | Stable language-neutral code for bounded polled-audio status. | `src/session/error_code.rs:131` |
| `pocketstation::endpoint::polled_audio_driver::PolledAudioBatchLease` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/endpoint/polled_audio_driver.rs:172` |
| `pocketstation::endpoint::polled_audio_driver::PolledAudioFrame` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/endpoint/polled_audio_driver.rs:210` |
| `pocketstation::endpoint::polled_audio_driver::PolledAudioObservations` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/endpoint/polled_audio_driver.rs:56` |
| `pocketstation::endpoint::polled_audio_driver::PolledAudioEndpointConfigError` | enum | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/endpoint/polled_audio_driver.rs:40` |
| `pocketstation::endpoint::polled_audio_driver::PolledAudioPollError` | enum | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/endpoint/polled_audio_driver.rs:74` |
| `pocketstation::session::error_code::polled_audio_poll_error_code` | function | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/session/error_code.rs:255` |
| `polled_audio` | function | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/session/lifecycle/observations.rs:71` |
| `pocketstation::endpoint::polled_audio_driver::PolledAudioEndpointConfigError::BatchCapacityTooLarge` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/endpoint/polled_audio_driver.rs:50` |
| `pocketstation::endpoint::polled_audio_driver::PolledAudioEndpointConfigError::LeaseCapacityTooLarge` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/endpoint/polled_audio_driver.rs:52` |
| `pocketstation::endpoint::polled_audio_driver::PolledAudioEndpointConfigError::QueueCapacityTooLarge` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/endpoint/polled_audio_driver.rs:48` |
| `pocketstation::endpoint::polled_audio_driver::PolledAudioEndpointConfigError::ZeroBatchCapacity` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/endpoint/polled_audio_driver.rs:44` |
| `pocketstation::endpoint::polled_audio_driver::PolledAudioEndpointConfigError::ZeroLeaseCapacity` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/endpoint/polled_audio_driver.rs:46` |
| `pocketstation::endpoint::polled_audio_driver::PolledAudioEndpointConfigError::ZeroQueueCapacity` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/endpoint/polled_audio_driver.rs:42` |
| `pocketstation::endpoint::polled_audio_driver::PolledAudioPollError::Empty` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/endpoint/polled_audio_driver.rs:76` |
| `pocketstation::endpoint::polled_audio_driver::PolledAudioPollError::LeaseCapacityExhausted` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/endpoint/polled_audio_driver.rs:78` |

## Verify the outcome

The following test bodies are evidence only for their recorded setup:

- `given_polled_audio_failures_when_mapped_then_every_status_is_preserved` — given polled audio failures when mapped then every status is preserved (`src/session/error_code.rs:524`; `test-36b002bb02a82e639fa5`).
- `given_host_owned_backends_when_started_then_polled_audio_and_stop_are_real` — given host owned backends when started then polled audio and stop are real (`src/session/lifecycle/host.rs:724`; `test-1e7766766c491d2b7101`).
- `polled_audio_receipts_total` — polled audio receipts total (`src/session/lifecycle/host.rs:101`; `test-d0262b09f7e1a858e09e`).
- `given_concurrent_publish_and_poll_when_observed_then_depth_stays_bounded_and_returns_to_zero` — given concurrent publish and poll when observed then depth stays bounded and returns to zero (`src/endpoint/polled_audio_driver/tests.rs:138`; `test-53df4ef903e72a1de69c`).
- `given_held_batch_when_polled_then_samples_stay_stable_and_lease_exhaustion_is_counted` — given held batch when polled then samples stay stable and lease exhaustion is counted (`src/endpoint/polled_audio_driver/tests.rs:82`; `test-1c0e175d584cd2910a77`).
- `planned_audio_edge_count` — planned audio edge count (`src/session/compile/compiled.rs:69`; `test-df0021f5a7204cec4764`).
- `given_text_operator_sent_to_audio_endpoint_when_compiled_then_signal_mismatch_is_typed` — given text operator sent to audio endpoint when compiled then signal mismatch is typed (`src/session/compile/tests.rs:709`; `test-3c9cfc84af12d388d72a`).
- `given_derived_stream_without_destination_when_frozen_then_validation_fails_closed` — given derived stream without destination when frozen then validation fails closed (`src/session/declaration/draft.rs:1369`; `test-94e7fa143670693acd86`).
- `given_audio_endpoint_extension_when_requested_then_definition_is_not_boot_registered` — given audio endpoint extension when requested then definition is not boot registered (`src/session/extensions/builtins.rs:616`; `test-85b9176c8aff351621b9`).
- `given_external_pcm_output_when_compiled_then_bounded_audio_edge_is_planned` — given external pcm output when compiled then bounded audio edge is planned (`src/session/extensions/tests/composition.rs:333`; `test-477958c0b22fe8487982`).
- `given_external_pcm_source_when_session_runs_then_audio_uses_bounded_ingress_with_source_identity` — given external pcm source when session runs then audio uses bounded ingress with source identity (`src/session/extensions/tests/runtime.rs:823`; `test-1d9f4de1e64929bbc714`).
- `given_all_senders_dropped_when_polled_then_receiver_reports_closed` — given all senders dropped when polled then receiver reports closed (`src/session/lifecycle/events.rs:656`; `test-ed28bb41869db2c16ec2`).

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

- [Glossary](/docs/glossary.md)
- [PocketStation](/README.md)
- [Behavior evidence index](/docs/reference/behavior-evidence.md)
- [Rust API reference](/docs/reference/rust-api.md)
- [Frames or signals are dropped](/docs/troubleshooting/drops-and-saturation.md)
- [Architecture overview](/docs/architecture/overview.md)
- [Memory ownership and buffer pools](/docs/internals/memory-ownership.md)
- [Polled audio](/docs/concepts/polled-audio.md)

## Evidence boundary

This page was verified against Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `examples/product_quickstart.rs:1-61` (`DIRECT`)
- `src/endpoint/polled_audio_driver.rs:1-773` (`DIRECT`)

A file's presence proves implementation or declaration at this snapshot. It does not by itself prove physical-device qualification, operational performance, retry safety, or behavior outside the recorded test conditions.
