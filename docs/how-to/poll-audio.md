# Poll audio without unbounded buffering

<!-- claims: CLM-GUIDE-006-SCOPE-001,CLM-GUIDE-006-TEXT-001,CLM-GUIDE-006-TEXT-002,CLM-GUIDE-006-TEXT-003,CLM-GUIDE-006-TEXT-004,CLM-GUIDE-006-TEXT-005,CLM-GUIDE-006-TEXT-006,CLM-GUIDE-006-SOURCE-001 -->

## Scope

- **Poll bounded audio batches.** Consume routed audio from the built-in polled-audio endpoint through bounded batch leases and receipts.
- **Route realtime audio.** Deliver pooled audio frames through independent fixed-capacity routes governed by explicit edge policy.

The scope of **Poll audio without unbounded buffering** ends at the native contracts and executable conditions cited below. Platform qualification, performance, retry, and delivery require their own explicit evidence.

## Prerequisites

A declared source or stream and a finite `PolledAudioEndpointConfig` appropriate for non-realtime polling.

## Procedure

1. Declare a separate polled_audio endpoint for each independent route.
2. Send the source or stream to that endpoint.
3. Call try_poll_audio for an immediate check or wait_poll with a finite timeout from non-realtime application code.
4. Iterate only indices below the returned batch length and retain the route, endpoint, and poll observation timestamps needed for diagnosis.
5. Release the lease promptly and inspect polling observations.

## Concrete repository example

This is the frozen, repository-owned example `example-64188d831f3c13af50ff` at `examples/product_quickstart.rs`. It is validated by the examples checkpoint.

```rust
use std::collections::BTreeMap;
use std::error::Error;
use std::time::{Duration, Instant};

use pocketstation as pks;

fn main() -> Result<(), Box<dyn Error>> {
    let session = pks::Session::builder()
        .recording_root("pocketstation-recordings")
        .build();
    let app = session.capture(pks::Source::application(pks::ApplicationSelector::name(
        "PocketStation Demo",
    )))?;
    let mic = session.capture(pks::Source::microphone_default())?;
    let app_audio = session.polled_audio()?;
    let mic_audio = session.polled_audio()?;

    app.send(app_audio)?;
    mic.send(mic_audio)?;
    app.record("application")?;
    mic.record("microphone")?;

    let mut running = session.start()?;
    let deadline = Instant::now() + Duration::from_secs(10);
    let mut frames_by_stem = BTreeMap::<u64, usize>::new();
    while Instant::now() < deadline
        && frames_by_stem.values().filter(|count| **count >= 2).count() < 2
    {
        if let Ok(batch) = running.try_poll_audio() {
            for index in 0..batch.len() {
                let frame = batch
                    .frame(index)
                    .ok_or("bounded audio batch returned an invalid frame index")?;
                let count = frames_by_stem
                    .entry(frame.lineage().stem_id().get())
                    .or_default();
                *count = count.saturating_add(1);
            }
        }
        std::thread::sleep(Duration::from_millis(1));
    }
    if frames_by_stem.values().filter(|count| **count >= 2).count() != 2 {
        return Err("application and microphone media were not both observed".into());
    }

    let outcome = running.stop();
    if !outcome.is_success() {
        return Err("PocketStation Session did not finalize cleanly".into());
    }
    let recording = running
        .recording_outcome()
        .ok_or("PocketStation Session did not expose a recording outcome")?;
    if recording.state != pks::SessionRecordingState::Complete
        || recording.completed_stems != 2
        || recording.failed_stems != 0
    {
        return Err("PocketStation multistem recording did not complete".into());
    }
    Ok(())
}
```

## Important consequence

Distinguish an empty queue from exhausted outstanding-lease capacity and route drops.

## Verify the outcome

`try_poll_audio` returns a bounded lease; observed frame counts advance and the lease is released after consumption.

Executable evidence selected for **Poll audio without unbounded buffering** is limited to each test's recorded setup and assertions:

- `given_polled_audio_failures_when_mapped_then_every_status_is_preserved` — given polled audio failures when mapped then every status is preserved (`src/session/error_code.rs:524`; `test-d8f211a56e8b18b3cbd6`).
- `given_host_owned_backends_when_started_then_polled_audio_and_stop_are_real` — given host owned backends when started then polled audio and stop are real (`src/session/lifecycle/host.rs:724`; `test-1e1a4d3810caea030f74`).
- `given_concurrent_publish_and_poll_when_observed_then_depth_stays_bounded_and_returns_to_zero` — given concurrent publish and poll when observed then depth stays bounded and returns to zero (`src/endpoint/polled_audio_driver/tests.rs:204`; `test-a0ab3d16a966eb5f5862`).
- `given_held_batch_when_polled_then_samples_stay_stable_and_lease_exhaustion_is_counted` — given held batch when polled then samples stay stable and lease exhaustion is counted (`src/endpoint/polled_audio_driver/tests.rs:90`; `test-cb5950f6730dd555dc93`).
- `given_text_operator_sent_to_audio_endpoint_when_compiled_then_signal_mismatch_is_typed` — given text operator sent to audio endpoint when compiled then signal mismatch is typed (`src/session/compile/tests.rs:709`; `test-76c3eb4c4fd13e959a1c`).
- `given_derived_stream_without_destination_when_frozen_then_validation_fails_closed` — given derived stream without destination when frozen then validation fails closed (`src/session/declaration/draft.rs:1357`; `test-17c702ffaf38dad01e0a`).
- `given_audio_endpoint_extension_when_requested_then_definition_is_not_boot_registered` — given audio endpoint extension when requested then definition is not boot registered (`src/session/extensions/builtins.rs:616`; `test-e96f8506522ac5a30e20`).
- `given_external_pcm_output_when_compiled_then_bounded_audio_edge_is_planned` — given external pcm output when compiled then bounded audio edge is planned (`src/session/extensions/tests/composition.rs:333`; `test-72f08a54e97cf69789ac`).
- `given_external_pcm_source_when_session_runs_then_audio_uses_bounded_ingress_with_source_identity` — given external pcm source when session runs then audio uses bounded ingress with source identity (`src/session/extensions/tests/runtime.rs:823`; `test-4d0f3e5a95ea9490a090`).
- `given_session_without_source_when_validated_then_topology_is_rejected` — given session without source when validated then topology is rejected (`src/session/lifecycle/control.rs:218`; `test-3ad011ae6ea2c1d8804b`).
- `given_all_senders_dropped_when_polled_then_receiver_reports_closed` — given all senders dropped when polled then receiver reports closed (`src/session/lifecycle/events.rs:656`; `test-869cc16c477444c9b6fd`).
- `given_events_when_polled_then_fifo_order_and_depth_are_preserved` — given events when polled then fifo order and depth are preserved (`src/session/lifecycle/events.rs:632`; `test-5346edce6d9a6da2069d`).

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

- [Polled Audio](/docs/concepts/polled-audio.md)
- [Endpoints](/docs/reference/endpoints.md)

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

The claims on **Poll audio without unbounded buffering** are anchored to Git snapshot `136e74888962558aa846d3143a19136a70936f45` and these primary files:

- `examples/product_quickstart.rs:1-21` (`DIRECT`)
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

For **Poll audio without unbounded buffering**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.
