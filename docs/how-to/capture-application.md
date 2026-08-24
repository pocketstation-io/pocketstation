# Capture a desktop application

<!-- claims: CLM-GUIDE-001-SCOPE-001,CLM-GUIDE-001-TEXT-001,CLM-GUIDE-001-TEXT-002,CLM-GUIDE-001-TEXT-003,CLM-GUIDE-001-TEXT-004,CLM-GUIDE-001-TEXT-005,CLM-GUIDE-001-TEXT-006,CLM-GUIDE-001-SOURCE-001 -->

## Scope

- **Select and resolve capture sources.** Discover capture candidates and resolve application, process, device, and system queries to stable source identities.
- **Capture application audio.** Prepare application-scoped capture through the platform backend selected for the current target.
- **Observe permission and source lifecycle.** Query non-prompting authorization where observable and receive source-generation, loss, and permission-epoch changes.

The scope of **Capture a desktop application** ends at the native contracts and executable conditions cited below. Platform qualification, performance, retry, and delivery require their own explicit evidence.

## Prerequisites

A target with application capture support, host-owned capture permission, and an application selector that can resolve the intended process.

## Procedure

1. Create a Session declaration.
2. Build an ApplicationSelector whose evidence matches your selection need.
3. Declare the application Source and attach a consumer route.
4. Start the Session and retain RunningSession.
5. Observe frames or typed capture failures, then stop and inspect the outcome.

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

Separate selector resolution, permission, source opening, and route delivery; each can fail independently.

## Verify the outcome

Frames from the selected application arrive on its polled route, and stop returns without a source or route failure.

Executable evidence selected for **Capture a desktop application** is limited to each test's recorded setup and assertions:

- `given_active_capture_when_owner_is_dropped_then_backend_is_reclaimed` — given active capture when owner is dropped then backend is reclaimed (`src/capture/capture_owner.rs:567`; `test-fa34e5723160d56f560f`).
- `given_active_capture_when_stopped_then_backend_is_joined` — given active capture when stopped then backend is joined (`src/capture/capture_owner.rs:540`; `test-dd4aaaf6b93ddb500769`).
- `given_panicking_capture_worker_when_joined_then_typed_failure_is_returned` — given panicking capture worker when joined then typed failure is returned (`src/capture/capture_owner.rs:610`; `test-2d873f94835a177ce436`).
- `given_prepared_capture_when_opened_then_bounded_delivery_is_owned` — given prepared capture when opened then bounded delivery is owned (`src/capture/capture_owner.rs:463`; `test-a3a0d044f02b7f664bb9`).
- `given_application_mode_when_pipewire_unavailable_then_mode_unsupported_not_system_mix` — given application mode when pipewire unavailable then mode unsupported not system mix (`src/capture/platform/linux/pipewire.rs:2097`; `test-3935b20953f69bd82dab`).
- `given_capture_mode_when_channels_selected_then_microphone_is_mono_and_output_is_stereo` — given capture mode when channels selected then microphone is mono and output is stereo (`src/capture/platform/linux/pipewire.rs:1837`; `test-df5c7fa69c2c79a8f2a1`).
- `given_exact_application_selector_when_identity_is_transient_then_selection_fails_closed` — given exact application selector when identity is transient then selection fails closed (`src/capture/platform/linux/pipewire.rs:1962`; `test-1e40dd4ec9e96cd35eb7`).
- `given_exact_application_selector_when_multiple_nodes_match_then_selection_is_ambiguous` — given exact application selector when multiple nodes match then selection is ambiguous (`src/capture/platform/linux/pipewire.rs:1998`; `test-2843e96f914d98065a94`).
- `given_exact_application_selector_when_one_live_node_matches_then_current_target_is_selected` — given exact application selector when one live node matches then current target is selected (`src/capture/platform/linux/pipewire.rs:1932`; `test-15388b47d24aa21999f6`).
- `given_exact_stable_application_when_pipewire_unavailable_then_mode_is_not_weakened` — given exact stable application when pipewire unavailable then mode is not weakened (`src/capture/platform/linux/pipewire.rs:2111`; `test-51cbb8d765eada41b0c9`).
- `given_exhausted_capture_pool_when_acquiring_then_failure_is_observed_once` — given exhausted capture pool when acquiring then failure is observed once (`src/capture/platform/linux/pipewire.rs:1855`; `test-3002ec0fb883ffa835f6`).
- `given_negotiated_format_when_channel_count_changes_then_capture_fails_closed` — given negotiated format when channel count changes then capture fails closed (`src/capture/platform/linux/pipewire.rs:2066`; `test-0f6c4f31518ab5e8ffd8`).

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

- [Application Capture](/docs/concepts/application-capture.md)
- [Capture](/docs/reference/capture.md)

| Public declaration | Kind | Declared purpose | Source |
|---|---|---|---|
| `pocketstation::capture::platform::macos::session_backend::DesktopCaptureBackend` | struct | macOS adapter from the platform-neutral Session capture contract to the existing CoreAudio/input RAII owner. | `src/capture/platform/macos/session_backend.rs:11` |
| `pocketstation::capture::authorization::ApplicationPolicyObservation` | enum | Classifies the observable application policy observation. | `src/capture/authorization.rs:231` |
| `pocketstation::capture::query::application_capture_available` | function | Reports whether this host exposes the native application-capture facility. | `src/capture/query.rs:64` |
| `pocketstation::capture::authorization::ApplicationPolicyObservation::Allowed` | variant | Reports the observed application policy as allowed. | `src/capture/authorization.rs:232` |
| `pocketstation::capture::authorization::ApplicationPolicyObservation::Denied` | variant | Reports the observed application policy as denied. | `src/capture/authorization.rs:233` |
| `pocketstation::capture::authorization::ApplicationPolicyObservation::NotApplicable` | variant | Reports the observed application policy as not applicable. | `src/capture/authorization.rs:235` |
| `pocketstation::capture::authorization::ApplicationPolicyObservation::NotObservable` | variant | Reports the observed application policy as not observable. | `src/capture/authorization.rs:234` |
| `pocketstation::capture::authorization::CaptureScope::ExactApplication` | variant | Limits capture authorization to exact application. | `src/capture/authorization.rs:249` |

## Related documentation

- [Glossary](/docs/glossary.md)
- [PocketStation](/README.md)
- [Behavior evidence index](/docs/reference/behavior-evidence.md)
- [Capture API](/docs/reference/capture.md)
- [Rust API reference](/docs/reference/rust-api.md)
- [Capture failures](/docs/errors/capture.md)
- [No application audio arrives](/docs/troubleshooting/no-application-audio.md)
- [Linux capture](/docs/platform/linux.md)

## Evidence boundary

The claims on **Capture a desktop application** are anchored to Git snapshot `136e74888962558aa846d3143a19136a70936f45` and these primary files:

- `examples/product_quickstart.rs:1-21` (`DIRECT`)

For **Capture a desktop application**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.
