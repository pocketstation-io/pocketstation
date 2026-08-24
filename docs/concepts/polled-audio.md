# Polled audio

<!-- claims: CLM-DOC-018-SCOPE-001,CLM-DOC-018-TEXT-001,CLM-DOC-018-TEXT-002,CLM-DOC-018-TEXT-003,CLM-DOC-018-TEXT-004,CLM-DOC-018-TEXT-005,CLM-DOC-018-TEXT-006,CLM-DOC-018-SOURCE-001 -->

## What it is

The polled-audio endpoint places routed frames in a fixed-capacity queue and returns bounded batch leases through a receipt owned by application code.

## Why it exists

Polling gives non-realtime application code a pull interface without turning the audio callback into an unbounded producer or copying every frame into unlimited storage.

## Relationships

- A Session route sends one source or stream to the endpoint.
- `try_poll` returns immediately, while `wait_poll` waits only until its caller-supplied finite timeout.
- Each leased frame exposes route enqueue, route receive, endpoint enqueue, and poll observation timestamps.
- The endpoint driver owns queue and lease observations.
- A batch lease bounds how many frames remain outstanding at once.

## Invariants and guarantees

- Queue, batch, and outstanding-lease capacities are non-zero and bounded.
- An empty queue, a finite wait timeout, and exhausted lease capacity are distinct outcomes.
- The wait does not create an additional audio queue.
- Release a lease promptly so capacity returns to the receipt.

## When you encounter it

- **Reach first captured frames** — Build a Session that captures an application and microphone and polls their independent frames.
- **Transcribe captured stems** — Run the repository transcription example and preserve process evidence for its external boundary.

## Use it

- [Poll audio without unbounded buffering](/docs/how-to/poll-audio.md)
- [Frames or signals are dropped](/docs/troubleshooting/drops-and-saturation.md)

## Scope

- **Poll bounded audio batches.** Consume routed audio from the built-in polled-audio endpoint through bounded batch leases and receipts.

The scope of **Polled audio** ends at the native contracts and executable conditions cited below. Platform qualification, performance, retry, and delivery require their own explicit evidence.

## Key API

| Public declaration | Kind | Declared purpose | Source |
|---|---|---|---|
| `pocketstation::endpoint::polled_audio::PolledAudioEndpoint` | struct | Declares application-polled audio and retains its bounded receipt. | `src/endpoint/polled_audio.rs:16` |
| `pocketstation::endpoint::polled_audio_driver::PolledAudioBatchLease` | struct | Holds the ownership or bounded access represented by polled audio batch lease. | `src/endpoint/polled_audio_driver.rs:218` |
| `pocketstation::endpoint::polled_audio_driver::PolledAudioEndpointConfig` | struct | Configures polled audio endpoint behavior at its owning API boundary. | `src/endpoint/polled_audio_driver.rs:23` |
| `pocketstation::endpoint::polled_audio_driver::PolledAudioFrame` | struct | Carries one polled audio payload together with its declared metadata. | `src/endpoint/polled_audio_driver.rs:256` |
| `pocketstation::endpoint::polled_audio_driver::PolledAudioObservations` | struct | Reports the polled audio observations collected at an observation boundary. | `src/endpoint/polled_audio_driver.rs:56` |
| `pocketstation::endpoint::polled_audio_driver::PolledAudioReceipt` | struct | Retains the identity and observation access returned for polled audio. | `src/endpoint/polled_audio_driver.rs:105` |
| `pocketstation::endpoint::polled_audio_driver::PolledAudioEndpointConfigError` | enum | Classifies failures surfaced by polled audio endpoint config operations. | `src/endpoint/polled_audio_driver.rs:40` |
| `pocketstation::endpoint::polled_audio_driver::PolledAudioPollError` | enum | Classifies failures surfaced by polled audio poll operations. | `src/endpoint/polled_audio_driver.rs:74` |
| `pocketstation::endpoint::polled_audio_driver::PolledAudioEndpointConfigError::BatchCapacityTooLarge` | variant | Reports that batch capacity exceeds the supported size limit. | `src/endpoint/polled_audio_driver.rs:50` |
| `pocketstation::endpoint::polled_audio_driver::PolledAudioEndpointConfigError::LeaseCapacityTooLarge` | variant | Reports that lease capacity exceeds the supported size limit. | `src/endpoint/polled_audio_driver.rs:52` |

## Executable evidence

Executable evidence selected for **Polled audio** is limited to each test's recorded setup and assertions:

- `given_polled_audio_failures_when_mapped_then_every_status_is_preserved` — given polled audio failures when mapped then every status is preserved (`src/session/error_code.rs:524`; `test-d8f211a56e8b18b3cbd6`).
- `given_host_owned_backends_when_started_then_polled_audio_and_stop_are_real` — given host owned backends when started then polled audio and stop are real (`src/session/lifecycle/host.rs:724`; `test-1e1a4d3810caea030f74`).
- `given_held_batch_when_polled_then_samples_stay_stable_and_lease_exhaustion_is_counted` — given held batch when polled then samples stay stable and lease exhaustion is counted (`src/endpoint/polled_audio_driver/tests.rs:90`; `test-cb5950f6730dd555dc93`).
- `given_text_operator_sent_to_audio_endpoint_when_compiled_then_signal_mismatch_is_typed` — given text operator sent to audio endpoint when compiled then signal mismatch is typed (`src/session/compile/tests.rs:709`; `test-76c3eb4c4fd13e959a1c`).
- `given_audio_endpoint_extension_when_requested_then_definition_is_not_boot_registered` — given audio endpoint extension when requested then definition is not boot registered (`src/session/extensions/builtins.rs:616`; `test-e96f8506522ac5a30e20`).
- `given_external_pcm_output_when_compiled_then_bounded_audio_edge_is_planned` — given external pcm output when compiled then bounded audio edge is planned (`src/session/extensions/tests/composition.rs:333`; `test-72f08a54e97cf69789ac`).
- `given_external_pcm_source_when_session_runs_then_audio_uses_bounded_ingress_with_source_identity` — given external pcm source when session runs then audio uses bounded ingress with source identity (`src/session/extensions/tests/runtime.rs:823`; `test-4d0f3e5a95ea9490a090`).
- `given_all_senders_dropped_when_polled_then_receiver_reports_closed` — given all senders dropped when polled then receiver reports closed (`src/session/lifecycle/events.rs:656`; `test-869cc16c477444c9b6fd`).
- `given_events_when_polled_then_fifo_order_and_depth_are_preserved` — given events when polled then fifo order and depth are preserved (`src/session/lifecycle/events.rs:632`; `test-5346edce6d9a6da2069d`).
- `given_registered_polled_endpoint_when_host_built_then_receipt_is_retained` — given registered polled endpoint when host built then receipt is retained (`src/session/lifecycle/host.rs:677`; `test-911427177e01f99153df`).
- `given_deterministic_capture_when_polled_then_real_runtime_branch_copy_and_lineage_are_exposed` — given deterministic capture when polled then real runtime branch copy and lineage are exposed (`src/session/lifecycle/tests/engine.rs:440`; `test-b1797fb7f4f0913afd38`).
- `given_public_session_pcm_output_when_reentered_then_audio_lane_and_lifecycle_are_observed` — given public session pcm output when reentered then audio lane and lifecycle are observed (`src/session/lifecycle/tests/running.rs:2136`; `test-ea5b06c730a73a1dc9ca`).

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

The claims on **Polled audio** are anchored to Git snapshot `136e74888962558aa846d3143a19136a70936f45` and these primary files:

- `src/endpoint/polled_audio_driver.rs:16-16` (`DIRECT`)
- `src/endpoint/polled_audio_driver.rs:17-17` (`DIRECT`)
- `src/endpoint/polled_audio_driver.rs:18-18` (`DIRECT`)
- `src/endpoint/polled_audio_driver.rs:19-19` (`DIRECT`)
- `src/endpoint/polled_audio_driver.rs:20-20` (`DIRECT`)
- `src/endpoint/polled_audio_driver.rs:22-22` (`DIRECT`)
- `src/endpoint/polled_audio_driver.rs:22-22` (`DIRECT`)
- `src/endpoint/polled_audio_driver.rs:22-22` (`DIRECT`)
- `src/endpoint/polled_audio_driver.rs:23-27` (`DIRECT`)
- `src/endpoint/polled_audio_driver.rs:24-24` (`DIRECT`)
- `src/endpoint/polled_audio_driver.rs:25-25` (`DIRECT`)
- `src/endpoint/polled_audio_driver.rs:26-26` (`DIRECT`)
- `src/endpoint/polled_audio_driver.rs:30-36` (`DIRECT`)
- `src/endpoint/polled_audio_driver.rs:39-39` (`DIRECT`)
- `src/endpoint/polled_audio_driver.rs:39-39` (`DIRECT`)
- `src/endpoint/polled_audio_driver.rs:39-39` (`DIRECT`)
- `src/endpoint/polled_audio_driver.rs:39-39` (`DIRECT`)
- `src/endpoint/polled_audio_driver.rs:40-53` (`DIRECT`)
- `src/endpoint/polled_audio_driver.rs:42-42` (`DIRECT`)
- `src/endpoint/polled_audio_driver.rs:44-44` (`DIRECT`)
- `src/endpoint/polled_audio_driver.rs:46-46` (`DIRECT`)
- `src/endpoint/polled_audio_driver.rs:48-48` (`DIRECT`)
- `src/endpoint/polled_audio_driver.rs:50-50` (`DIRECT`)
- `src/endpoint/polled_audio_driver.rs:52-52` (`DIRECT`)

For **Polled audio**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.
