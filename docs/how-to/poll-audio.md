# Poll audio without unbounded buffering

<!-- claims: CLM-GUIDE-006-CAP-001,CLM-GUIDE-006-CAP-002,CLM-GUIDE-006-SOURCE-001 -->

## Scope

- **Poll bounded audio batches.** Consume routed audio from the built-in polled-audio endpoint through bounded batch leases and receipts.
- **Route realtime audio.** Deliver pooled audio frames through independent fixed-capacity routes governed by explicit edge policy.

The scope of **Poll audio without unbounded buffering** ends at the native contracts and executable conditions cited below. Platform qualification, performance, retry, and delivery require their own explicit evidence.

## Prerequisites

A declared source or stream and a finite `PolledAudioEndpointConfig` appropriate for non-realtime polling.

## Procedure

1. Declare a separate polled_audio endpoint for each independent route.
2. Send the source or stream to that endpoint.
3. Call try_poll_audio from non-realtime application code.
4. Iterate only indices below the returned batch length.
5. Release the lease promptly and inspect polling observations.

## Important consequence

Distinguish an empty queue from exhausted outstanding-lease capacity and route drops.

## Verify the outcome

`try_poll_audio` returns a bounded lease; observed frame counts advance and the lease is released after consumption.

Executable evidence selected for **Poll audio without unbounded buffering** is limited to each test's recorded setup and assertions:

- `given_polled_audio_failures_when_mapped_then_every_status_is_preserved` — given polled audio failures when mapped then every status is preserved (`src/session/error_code.rs:524`; `test-36b002bb02a82e639fa5`).
- `given_host_owned_backends_when_started_then_polled_audio_and_stop_are_real` — given host owned backends when started then polled audio and stop are real (`src/session/lifecycle/host.rs:724`; `test-1e7766766c491d2b7101`).
- `given_concurrent_publish_and_poll_when_observed_then_depth_stays_bounded_and_returns_to_zero` — given concurrent publish and poll when observed then depth stays bounded and returns to zero (`src/endpoint/polled_audio_driver/tests.rs:138`; `test-53df4ef903e72a1de69c`).
- `given_held_batch_when_polled_then_samples_stay_stable_and_lease_exhaustion_is_counted` — given held batch when polled then samples stay stable and lease exhaustion is counted (`src/endpoint/polled_audio_driver/tests.rs:82`; `test-1c0e175d584cd2910a77`).
- `given_text_operator_sent_to_audio_endpoint_when_compiled_then_signal_mismatch_is_typed` — given text operator sent to audio endpoint when compiled then signal mismatch is typed (`src/session/compile/tests.rs:709`; `test-3c9cfc84af12d388d72a`).
- `given_derived_stream_without_destination_when_frozen_then_validation_fails_closed` — given derived stream without destination when frozen then validation fails closed (`src/session/declaration/draft.rs:1369`; `test-94e7fa143670693acd86`).
- `given_audio_endpoint_extension_when_requested_then_definition_is_not_boot_registered` — given audio endpoint extension when requested then definition is not boot registered (`src/session/extensions/builtins.rs:616`; `test-85b9176c8aff351621b9`).
- `given_external_pcm_output_when_compiled_then_bounded_audio_edge_is_planned` — given external pcm output when compiled then bounded audio edge is planned (`src/session/extensions/tests/composition.rs:333`; `test-477958c0b22fe8487982`).
- `given_external_pcm_source_when_session_runs_then_audio_uses_bounded_ingress_with_source_identity` — given external pcm source when session runs then audio uses bounded ingress with source identity (`src/session/extensions/tests/runtime.rs:823`; `test-1d9f4de1e64929bbc714`).
- `given_all_senders_dropped_when_polled_then_receiver_reports_closed` — given all senders dropped when polled then receiver reports closed (`src/session/lifecycle/events.rs:656`; `test-ed28bb41869db2c16ec2`).
- `given_events_when_polled_then_fifo_order_and_depth_are_preserved` — given events when polled then fifo order and depth are preserved (`src/session/lifecycle/events.rs:632`; `test-ef7a999d2659ba14b610`).
- `given_registered_polled_endpoint_when_host_built_then_receipt_is_retained` — given registered polled endpoint when host built then receipt is retained (`src/session/lifecycle/host.rs:677`; `test-c085fadc90c76bf5e3d9`).

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

- [Polled Audio](/docs/concepts/polled-audio.md)
- [Endpoints](/docs/reference/endpoints.md)

| Public declaration | Kind | Declared purpose | Source |
|---|---|---|---|
| `pocketstation::endpoint::polled_audio::PolledAudioEndpoint` | struct | Safe composition owner for application-polled audio. | `src/endpoint/polled_audio.rs:16` |
| `pocketstation::endpoint::polled_audio_driver::PolledAudioBatchLease` | struct | Owns bounded access to polled audio batch. | `src/endpoint/polled_audio_driver.rs:172` |
| `pocketstation::endpoint::polled_audio_driver::PolledAudioEndpointConfig` | struct | Configures polled audio endpoint behavior at its owning API boundary. | `src/endpoint/polled_audio_driver.rs:23` |
| `pocketstation::endpoint::polled_audio_driver::PolledAudioFrame` | struct | Carries one polled audio payload together with its declared metadata. | `src/endpoint/polled_audio_driver.rs:210` |
| `pocketstation::endpoint::polled_audio_driver::PolledAudioObservations` | struct | Reports the polled audio observations collected at an observation boundary. | `src/endpoint/polled_audio_driver.rs:56` |
| `pocketstation::endpoint::polled_audio_driver::PolledAudioReceipt` | struct | Retains the identity and observation access returned for polled audio. | `src/endpoint/polled_audio_driver.rs:105` |
| `pocketstation::endpoint::polled_audio_driver::PolledAudioEndpointConfigError` | enum | Classifies failures reported as polled audio endpoint config error. | `src/endpoint/polled_audio_driver.rs:40` |
| `pocketstation::endpoint::polled_audio_driver::PolledAudioPollError` | enum | Classifies failures reported as polled audio poll error. | `src/endpoint/polled_audio_driver.rs:74` |

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

The claims on **Poll audio without unbounded buffering** are anchored to Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `examples/product_quickstart.rs:1-61` (`DIRECT`)
- `src/endpoint/polled_audio_driver.rs:1-773` (`DIRECT`)

For **Poll audio without unbounded buffering**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.
