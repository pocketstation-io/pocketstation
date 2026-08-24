# Capture the default microphone

<!-- claims: CLM-GUIDE-002-SCOPE-001,CLM-GUIDE-002-TEXT-001,CLM-GUIDE-002-TEXT-002,CLM-GUIDE-002-TEXT-003,CLM-GUIDE-002-TEXT-004,CLM-GUIDE-002-TEXT-005,CLM-GUIDE-002-TEXT-006,CLM-GUIDE-002-SOURCE-001 -->

## Scope

- **Capture microphone audio.** Select the default or identified input device and open native microphone capture.
- **Observe permission and source lifecycle.** Query non-prompting authorization where observable and receive source-generation, loss, and permission-epoch changes.

The scope of **Capture the default microphone** ends at the native contracts and executable conditions cited below. Platform qualification, performance, retry, and delivery require their own explicit evidence.

## Prerequisites

An available input device and a host application capable of owning the platform permission prompt.

## Procedure

1. Observe permission without prompting when the target exposes that operation.
2. Let the host application own any permission prompt.
3. Declare the default or identified microphone Source.
4. Attach a consumer before start.
5. Treat preparation or source opening as the authoritative result.

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

Treat permission observation as advisory and source opening as authoritative.

## Verify the outcome

Microphone frames arrive with a distinct source and stem identity, and the source-open outcome is successful.

Executable evidence selected for **Capture the default microphone** is limited to each test's recorded setup and assertions:

- `given_capture_mode_when_channels_selected_then_microphone_is_mono_and_output_is_stereo` — given capture mode when channels selected then microphone is mono and output is stereo (`src/capture/platform/linux/pipewire.rs:1837`; `test-df5c7fa69c2c79a8f2a1`).
- `given_default_and_exact_microphones_when_contract_inspected_then_lifetimes_differ` — given default and exact microphones when contract inspected then lifetimes differ (`src/capture/tests.rs:249`; `test-2ac8da4006715acff504`).
- `given_default_capture_mode_when_compared_then_is_system_mix` — given default capture mode when compared then is system mix (`src/capture/tests.rs:207`; `test-f45d875b7b23fede26a0`).
- `given_active_capture_when_owner_is_dropped_then_backend_is_reclaimed` — given active capture when owner is dropped then backend is reclaimed (`src/capture/capture_owner.rs:567`; `test-fa34e5723160d56f560f`).
- `given_active_capture_when_stopped_then_backend_is_joined` — given active capture when stopped then backend is joined (`src/capture/capture_owner.rs:540`; `test-dd4aaaf6b93ddb500769`).
- `given_panicking_capture_worker_when_joined_then_typed_failure_is_returned` — given panicking capture worker when joined then typed failure is returned (`src/capture/capture_owner.rs:610`; `test-2d873f94835a177ce436`).
- `given_prepared_capture_when_opened_then_bounded_delivery_is_owned` — given prepared capture when opened then bounded delivery is owned (`src/capture/capture_owner.rs:463`; `test-a3a0d044f02b7f664bb9`).
- `given_exhausted_capture_pool_when_acquiring_then_failure_is_observed_once` — given exhausted capture pool when acquiring then failure is observed once (`src/capture/platform/linux/pipewire.rs:1855`; `test-3002ec0fb883ffa835f6`).
- `given_negotiated_format_when_channel_count_changes_then_capture_fails_closed` — given negotiated format when channel count changes then capture fails closed (`src/capture/platform/linux/pipewire.rs:2066`; `test-0f6c4f31518ab5e8ffd8`).
- `given_capture_before_callback_when_mapped_then_process_timestamp_preserves_delay` — given capture before callback when mapped then process timestamp preserves delay (`src/capture/platform/macos/input.rs:358`; `test-de7d536ac9b0edc1d4da`).
- `given_capture_before_process_epoch_when_mapped_then_timestamp_is_earliest_representable` — given capture before process epoch when mapped then timestamp is earliest representable (`src/capture/platform/macos/input.rs:371`; `test-dc164b0e06605b749d99`).
- `given_denied_permission_when_opening_input_then_capture_fails_closed` — given denied permission when opening input then capture fails closed (`src/capture/platform/macos/input.rs:384`; `test-93f56a3510497f49f523`).

## Failure signals

- `pocketstation::capture::authorization::CaptureError` — `error-8a6cfaf6313c49f3d002`
- `pocketstation::capture::authorization::CaptureError` / `BackendInit` — `error-e16ac3af9c00b5a9e1ef`
- `pocketstation::capture::authorization::CaptureError` / `BackendSetupRequired` — `error-49a3487734f77997ff1d`
- `pocketstation::capture::authorization::CaptureError` / `BackendStatus` — `error-433a8f64b39d41fe58e4`
- `pocketstation::capture::authorization::CaptureError` / `CaptureWorkerPanicked` — `error-6a1ddaf64fd582202ee9`
- `pocketstation::capture::authorization::CaptureError` / `InvalidRuntimeEventCapacity` — `error-c838e8f36c42c18a2a83`
- `pocketstation::capture::authorization::CaptureError` / `InvalidStreamCapacity` — `error-abbc7e6ad159c238bf74`
- `pocketstation::capture::authorization::CaptureError` / `ModeUnsupported` — `error-4a58ec0f52d2f2ee5a44`
- `pocketstation::capture::authorization::CaptureError` / `NotSupported` — `error-fc10abae73bd96954b49`
- `pocketstation::capture::authorization::CaptureError` / `PermissionDenied` — `error-54d94f02abd4884ade73`

## API reference

- [Microphone Capture](/docs/concepts/microphone-capture.md)
- [Permissions](/docs/platform/permissions.md)

| Public declaration | Kind | Declared purpose | Source |
|---|---|---|---|
| `pocketstation::capture::query::SourceProvider` | trait | Implement this trait to provide source behavior to PocketStation; its methods define the preparation and runtime contract. | `src/capture/query.rs:48` |
| `pocketstation::capture::authorization::CaptureAuthorizationSnapshot` | struct | Point-in-time authorization evidence for opening one exact capture source. | `src/capture/authorization.rs:17` |
| `pocketstation::capture::authorization::CapturePermissionLifecycle` | struct | Control-plane owner for one source's observed authorization epoch. | `src/capture/authorization.rs:183` |
| `pocketstation::capture::authorization::CapturePermissionTransition` | struct | One authoritative authorization-state transition observed by the host. | `src/capture/authorization.rs:168` |
| `pocketstation::capture::authorization::PermissionEpoch` | struct | Identifies the permission-observation generation attached to captured lineage. | `src/capture/authorization.rs:267` |
| `pocketstation::capture::events::CaptureRuntimeFailure` | struct | Reports a capture runtime failure. | `src/capture/events.rs:47` |
| `pocketstation::capture::events::SourceGeneration` | struct | Identifies one appearance generation of a capture source across loss and reappearance. | `src/capture/events.rs:12` |
| `pocketstation::capture::events::SourceRuntimeEventObservationHandle` | struct | Holds the ownership or bounded access represented by source runtime event observation handle. | `src/capture/events.rs:200` |

## Related documentation

- [Glossary](/docs/glossary.md)
- [Linux capture](/docs/platform/linux.md)
- [Platform prerequisites](/docs/getting-started/platform-prerequisites.md)
- [Platform support and evidence](/docs/platform/compatibility.md)
- [PocketStation](/README.md)
- [Windows capture](/docs/platform/windows.md)
- [macOS capture](/docs/platform/macos.md)
- [Behavior evidence index](/docs/reference/behavior-evidence.md)

## Evidence boundary

The claims on **Capture the default microphone** are anchored to Git snapshot `136e74888962558aa846d3143a19136a70936f45` and these primary files:

- `examples/product_quickstart.rs:1-21` (`DIRECT`)

For **Capture the default microphone**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.
