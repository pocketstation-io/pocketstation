# Polled audio

<!-- claims: CLM-DOC-018-CAP-001,CLM-DOC-018-SOURCE-001 -->

Consume routed audio from the built-in polled-audio endpoint through bounded batch leases and receipts.

## Scope

- **Poll bounded audio batches.** Consume routed audio from the built-in polled-audio endpoint through bounded batch leases and receipts.

These statements describe repository contracts at the documented snapshot. They do not extend platform qualification, performance, retry, or delivery guarantees beyond the native API contracts and executable evidence.

## Contract surface

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
| `pocketstation::endpoint::polled_audio_driver::PolledAudioPollError::StatePoisoned` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/endpoint/polled_audio_driver.rs:80` |
| `pocketstation::session::error_code::PolledAudioPollErrorCode::Empty` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/session/error_code.rs:132` |
| `pocketstation::session::error_code::PolledAudioPollErrorCode::InternalStateUnavailable` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/session/error_code.rs:134` |
| `pocketstation::session::error_code::PolledAudioPollErrorCode::LeaseCapacityExhausted` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/session/error_code.rs:133` |

## Where you encounter it

- **Reach first captured frames** — Build a Session that captures an application and microphone and polls their independent frames.
- **Transcribe captured stems** — Run the repository transcription example and preserve process evidence for its external boundary.

## Behavior established by tests

The following test bodies are evidence only for their recorded setup:

- `given_polled_audio_failures_when_mapped_then_every_status_is_preserved` — given polled audio failures when mapped then every status is preserved (`src/session/error_code.rs:524`; `test-36b002bb02a82e639fa5`).
- `given_host_owned_backends_when_started_then_polled_audio_and_stop_are_real` — given host owned backends when started then polled audio and stop are real (`src/session/lifecycle/host.rs:724`; `test-1e7766766c491d2b7101`).
- `polled_audio_receipts_total` — polled audio receipts total (`src/session/lifecycle/host.rs:101`; `test-d0262b09f7e1a858e09e`).
- `given_held_batch_when_polled_then_samples_stay_stable_and_lease_exhaustion_is_counted` — given held batch when polled then samples stay stable and lease exhaustion is counted (`src/endpoint/polled_audio_driver/tests.rs:82`; `test-1c0e175d584cd2910a77`).
- `planned_audio_edge_count` — planned audio edge count (`src/session/compile/compiled.rs:69`; `test-df0021f5a7204cec4764`).
- `given_text_operator_sent_to_audio_endpoint_when_compiled_then_signal_mismatch_is_typed` — given text operator sent to audio endpoint when compiled then signal mismatch is typed (`src/session/compile/tests.rs:709`; `test-3c9cfc84af12d388d72a`).
- `given_audio_endpoint_extension_when_requested_then_definition_is_not_boot_registered` — given audio endpoint extension when requested then definition is not boot registered (`src/session/extensions/builtins.rs:616`; `test-85b9176c8aff351621b9`).
- `given_external_pcm_output_when_compiled_then_bounded_audio_edge_is_planned` — given external pcm output when compiled then bounded audio edge is planned (`src/session/extensions/tests/composition.rs:333`; `test-477958c0b22fe8487982`).
- `given_external_pcm_source_when_session_runs_then_audio_uses_bounded_ingress_with_source_identity` — given external pcm source when session runs then audio uses bounded ingress with source identity (`src/session/extensions/tests/runtime.rs:823`; `test-1d9f4de1e64929bbc714`).
- `given_all_senders_dropped_when_polled_then_receiver_reports_closed` — given all senders dropped when polled then receiver reports closed (`src/session/lifecycle/events.rs:656`; `test-ed28bb41869db2c16ec2`).
- `given_events_when_polled_then_fifo_order_and_depth_are_preserved` — given events when polled then fifo order and depth are preserved (`src/session/lifecycle/events.rs:632`; `test-ef7a999d2659ba14b610`).
- `given_registered_polled_endpoint_when_host_built_then_receipt_is_retained` — given registered polled endpoint when host built then receipt is retained (`src/session/lifecycle/host.rs:677`; `test-c085fadc90c76bf5e3d9`).

## Boundaries

The compiler inventory establishes names, kinds, visibility, and signatures. Tests establish only their exercised conditions. Where retryability, ordering, cancellation, physical qualification, or recovery is not declared, this page leaves it unspecified.

## Related documentation

- [Glossary](/docs/glossary.md)
- [PocketStation](/README.md)
- [Rust quickstart](/docs/getting-started/rust-quickstart.md)
- [Poll audio without unbounded buffering](/docs/how-to/poll-audio.md)
- [Behavior evidence index](/docs/reference/behavior-evidence.md)
- [Endpoint API](/docs/reference/endpoints.md)
- [Rust API reference](/docs/reference/rust-api.md)
- [Endpoint failures](/docs/errors/endpoints.md)

## Evidence boundary

This page was verified against Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `src/endpoint/polled_audio_driver.rs:1-773` (`DIRECT`)

A file's presence proves implementation or declaration at this snapshot. It does not by itself prove physical-device qualification, operational performance, retry safety, or behavior outside the recorded test conditions.
