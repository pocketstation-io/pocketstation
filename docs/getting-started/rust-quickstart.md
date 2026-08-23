# Rust quickstart

<!-- claims: CLM-DOC-003-CAP-001,CLM-DOC-003-CAP-002,CLM-DOC-003-CAP-003,CLM-DOC-003-CAP-004,CLM-DOC-003-CAP-005,CLM-DOC-003-SOURCE-001 -->

## Audience

Use this quickstart if you are adding PocketStation to a Rust desktop application and want the first verified application-plus-microphone capture flow.

## Prerequisites

Choose a host with an application named `PocketStation Demo`, an available default microphone, permission to open both sources, and a writable recording path.

## Supported environment

The crate requires Rust 1.95 or newer. The program requires a target whose native-capture backend implements the selected sources; repository compilation is not physical-device qualification.

## Install

```toml
[dependencies]
pocketstation = "1.1.2"
```

## Program

This source is synchronized with `examples/product_quickstart.rs`:

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

## Run it

Run `cargo run --example product_quickstart` from the repository root on the prepared host.

## Success

The example accepts the run only after it observes two frames on each of two distinct stems, receives a successful Session stop outcome, and sees two completed recording stems with no failed stems.

## Common first-run failures

- The application selector cannot resolve `PocketStation Demo`.
- Permission or source opening fails for application or microphone capture.
- The recording root is not writable.
- A polled route returns no frames before the example's bounded observation condition.

Preserve the returned typed error or terminal outcome; do not translate a missing prerequisite into capture success.

## Next steps

- [Learn the Session mental model](/docs/concepts/session.md).
- [Select capture sources](/docs/concepts/source-selection.md).
- [Inspect recording outcomes](/docs/how-to/inspect-recording-outcome.md).
- [Diagnose a Session that fails before start](/docs/troubleshooting/session-start.md).

## Scope

- **Declare a Session.** Describe sources, operators, endpoints, streams, and recording routes before runtime preparation.
- **Capture application audio.** Prepare application-scoped capture through the platform backend selected for the current target.
- **Capture microphone audio.** Select the default or identified input device and open native microphone capture.
- **Poll bounded audio batches.** Consume routed audio from the built-in polled-audio endpoint through bounded batch leases and receipts.
- **Record aligned multistem output.** Configure stems and finalize per-stem outcomes through the recording endpoint lifecycle.

The scope of **Rust quickstart** ends at the native contracts and executable conditions cited below. Platform qualification, performance, retry, and delivery require their own explicit evidence.

## Public entry points

| Public declaration | Kind | Declared purpose | Source |
|---|---|---|---|
| `query` | module | Control-plane source discovery queries used by the first-party CLI. | `src/capture/query.rs:1` |
| `pocketstation::capture::capture_owner::ActiveCaptureBackend` | trait | Native capture resources owned for exactly one active capture. | `src/capture/capture_owner.rs:100` |
| `pocketstation::capture::capture_owner::CallbackCaptureBackend` | trait | Platform-neutral prepare/open boundary for callback-oriented capture. | `src/capture/capture_owner.rs:83` |
| `pocketstation::capture::capture_owner::PreparedCaptureBackend` | trait | Backend state that has passed validation but has not started delivery. | `src/capture/capture_owner.rs:88` |
| `pocketstation::capture::query::SourceProvider` | trait | Implement this trait to provide source behavior to PocketStation; its methods define the preparation and runtime contract. | `src/capture/query.rs:48` |
| `pocketstation::capture::capture_owner::CaptureDelivery` | struct | Callback delivery endpoints transferred to a prepared native backend. | `src/capture/capture_owner.rs:73` |
| `pocketstation::capture::capture_owner::CaptureLineageSeed` | struct | Stable session and stem identity assigned before an exact source is opened. | `src/capture/capture_owner.rs:25` |
| `pocketstation::capture::capture_owner::CaptureObservationReceipt` | struct | Retains the identity and observation access returned for capture observation. | `src/capture/capture_owner.rs:167` |
| `pocketstation::capture::capture_owner::CaptureOpenMetadata` | struct | Authoritative lineage state established only after native capture opens. | `src/capture/capture_owner.rs:49` |
| `pocketstation::capture::capture_owner::CaptureOwner` | struct | RAII owner for native capture, its bounded frame stream, and runtime events. | `src/capture/capture_owner.rs:194` |
| `pocketstation::capture::capture_owner::CaptureOwnerObservations` | struct | Aggregate observations from one active capture ownership boundary. | `src/capture/capture_owner.rs:160` |
| `pocketstation::capture::capture_owner::CapturePrepareRequest` | struct | Setup-time request for one bounded callback-oriented capture owner. | `src/capture/capture_owner.rs:61` |
| `pocketstation::capture::capture_owner::CaptureStopOutcome` | struct | Final observations returned only after backend stop and join complete. | `src/capture/capture_owner.rs:185` |
| `pocketstation::capture::capture_owner::PreparedCapture` | struct | Prepared capture plus its preallocated delivery endpoints. | `src/capture/capture_owner.rs:119` |
| `pocketstation::capture::platform::macos::input::MacosInputSource` | struct | Owns production of macos input values and its lifecycle state. | `src/capture/platform/macos/input.rs:65` |
| `pocketstation::capture::platform::macos::session_backend::DesktopCaptureBackend` | struct | macOS adapter from the platform-neutral Session capture contract to the existing CoreAudio/input RAII owner. | `src/capture/platform/macos/session_backend.rs:11` |

## Executable evidence

Executable evidence selected for **Rust quickstart** is limited to each test's recorded setup and assertions:

- `given_active_capture_when_owner_is_dropped_then_backend_is_reclaimed` — given active capture when owner is dropped then backend is reclaimed (`src/capture/capture_owner.rs:567`; `test-fa34e5723160d56f560f`).
- `given_active_capture_when_stopped_then_backend_is_joined` — given active capture when stopped then backend is joined (`src/capture/capture_owner.rs:540`; `test-dd4aaaf6b93ddb500769`).
- `given_backend_frame_when_source_differs_from_open_identity_then_lineage_fails_closed` — given backend frame when source differs from open identity then lineage fails closed (`src/capture/capture_owner.rs:511`; `test-805d755d4acd2257ba9b`).
- `given_panicking_capture_worker_when_joined_then_typed_failure_is_returned` — given panicking capture worker when joined then typed failure is returned (`src/capture/capture_owner.rs:610`; `test-2d873f94835a177ce436`).
- `given_prepared_capture_when_opened_then_bounded_delivery_is_owned` — given prepared capture when opened then bounded delivery is owned (`src/capture/capture_owner.rs:463`; `test-a3a0d044f02b7f664bb9`).
- `given_zero_frame_capacity_when_preparing_then_backend_is_not_prepared` — given zero frame capacity when preparing then backend is not prepared (`src/capture/capture_owner.rs:590`; `test-0afbec4242ea2fad4582`).
- `given_available_capacity_when_frame_is_sent_then_stream_preserves_frame` — given available capacity when frame is sent then stream preserves frame (`src/capture/frame_stream.rs:234`; `test-8f4bb6c6c11e1d2947a7`).
- `given_closed_start_gate_when_frame_is_sent_then_frame_is_discarded_and_counted` — given closed start gate when frame is sent then frame is discarded and counted (`src/capture/frame_stream.rs:256`; `test-a59f34c85fd9d74e587e`).
- `given_full_stream_when_frame_is_sent_then_newest_is_dropped_and_counted` — given full stream when frame is sent then newest is dropped and counted (`src/capture/frame_stream.rs:277`; `test-bb4e6d290a21c545166a`).
- `given_sender_callback_when_frame_arrives_then_stream_receives_it` — given sender callback when frame arrives then stream receives it (`src/capture/frame_stream.rs:298`; `test-698bc05f28228eb21d82`).
- `given_sender_dropped_when_stream_checked_then_closed_is_true` — given sender dropped when stream checked then closed is true (`src/capture/frame_stream.rs:309`; `test-013ff137ff3d32508a18`).
- `given_zero_capacity_when_stream_is_created_then_error_is_returned` — given zero capacity when stream is created then error is returned (`src/capture/frame_stream.rs:226`; `test-43b3af5a7d4a81817a36`).

## Related documentation

- [Glossary](/docs/glossary.md)
- [PocketStation](/README.md)
- [Behavior evidence index](/docs/reference/behavior-evidence.md)
- [Rust API reference](/docs/reference/rust-api.md)
- [Capture application and microphone stems](/docs/how-to/capture-app-and-mic.md)
- [Linux capture](/docs/platform/linux.md)
- [Platform backend boundary](/docs/internals/platform-backends.md)
- [Platform prerequisites](/docs/getting-started/platform-prerequisites.md)

## Evidence boundary

The claims on **Rust quickstart** are anchored to Git snapshot `136e74888962558aa846d3143a19136a70936f45` and these primary files:

- `examples/product_quickstart.rs:1-61` (`DIRECT`)

For **Rust quickstart**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.
