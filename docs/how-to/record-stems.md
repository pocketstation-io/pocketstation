# Record independent stems

<!-- claims: CLM-GUIDE-009-SCOPE-001,CLM-GUIDE-009-TEXT-001,CLM-GUIDE-009-TEXT-002,CLM-GUIDE-009-TEXT-003,CLM-GUIDE-009-TEXT-004,CLM-GUIDE-009-TEXT-005,CLM-GUIDE-009-TEXT-006,CLM-GUIDE-009-SOURCE-001 -->

## Scope

- **Record aligned multistem output.** Configure stems and finalize per-stem outcomes through the recording endpoint lifecycle.

The scope of **Record independent stems** ends at the native contracts and executable conditions cited below. Platform qualification, performance, retry, and delivery require their own explicit evidence.

## Prerequisites

A writable recording root and stable labels for every source stem you intend to preserve.

## Procedure

1. Set the recording root on SessionBuilder.
2. Call record with a label for each stem.
3. Start and run the Session.
4. Stop to trigger endpoint finalization.
5. Inspect overall and per-stem recording outcomes.

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

File finalization happens during stop; do not declare success from captured-frame counts alone.

## Verify the outcome

Stop returns a recording outcome whose completed stems match the declared labels and whose failed-stem list is empty.

Executable evidence selected for **Record independent stems** is limited to each test's recorded setup and assertions:

- `given_derived_permission_epoch_when_later_frame_changes_it_then_recording_fails_closed` — given derived permission epoch when later frame changes it then recording fails closed (`src/recording/endpoint/tests.rs:287`; `test-8c7b0f326da2b4760c28`).
- `given_session_context_and_two_first_frames_when_recorded_then_manifest_derives_capture_lineage_and_common_origin` — given session context and two first frames when recorded then manifest derives capture lineage and common origin (`src/recording/endpoint/tests.rs:187`; `test-1d7c657b57a9c71d6591`).
- `given_session_recorder_input_without_audio_stem_origin_when_prepared_then_it_is_rejected` — given session recorder input without audio stem origin when prepared then it is rejected (`src/recording/endpoint/tests.rs:262`; `test-a2e8d174434f9a88bf9e`).
- `given_recording_codes_when_serialized_then_values_are_exact_and_unique` — given recording codes when serialized then values are exact and unique (`src/recording/error_code.rs:95`; `test-921c1c5c1fdb60c7bf78`).
- `given_failed_recorder_branch_when_more_frames_dispatched_then_healthy_branch_continues` — given failed recorder branch when more frames dispatched then healthy branch continues (`src/recording/writer/tests.rs:253`; `test-668e2a246f514118dc91`).
- `given_queued_audio_when_recording_cancelled_then_wav_header_is_playable_and_manifest_incomplete` — given queued audio when recording cancelled then wav header is playable and manifest incomplete (`src/recording/writer/tests.rs:221`; `test-7f4ab2688e1c16ab56c2`).
- `given_two_clock_mapped_stems_when_finished_then_two_aligned_playable_wavs_are_written` — given two clock mapped stems when finished then two aligned playable wavs are written (`src/recording/writer/tests.rs:107`; `test-13b59c3a2ed9350468eb`).
- `given_product_spec_when_compiled_then_six_independent_edges_are_planned` — given product spec when compiled then six independent edges are planned (`src/session/compile/tests.rs:497`; `test-1d573ffc27e7c57617ac`).
- `given_two_derived_destinations_when_prepared_then_independent_branch_plans_are_preserved` — given two derived destinations when prepared then independent branch plans are preserved (`src/session/compile/tests.rs:685`; `test-b08c2cf433f7a97a1e94`).
- `given_two_record_declarations_when_frozen_then_default_group_identity_is_explicit_and_stable` — given two record declarations when frozen then default group identity is explicit and stable (`src/session/declaration/draft.rs:1212`; `test-79533e9048ae8212983b`).
- `given_two_stems_when_sent_to_one_endpoint_then_routes_are_distinct` — given two stems when sent to one endpoint then routes are distinct (`src/session/declaration/draft.rs:1144`; `test-3597ca95a8f2aa93709b`).
- `given_current_schema_when_version_read_then_derived_route_extension_is_recorded` — given current schema when version read then derived route extension is recorded (`src/session/declaration/spec.rs:843`; `test-a644dc428589d709659c`).

## Failure signals

- `pocketstation::recording::error_code::RecordingErrorCode` — `error-201ac2d01184e6098899`
- `pocketstation::recording::error_code::RecordingErrorCode` / `DuplicateStemLabel` — `error-f62c6dcf93e8e1fe2d15`
- `pocketstation::recording::error_code::RecordingErrorCode` / `FrameSpecMismatch` — `error-261ee409b627f2010e70`
- `pocketstation::recording::error_code::RecordingErrorCode` / `GapTooLarge` — `error-0c7b158ad325febfe411`
- `pocketstation::recording::error_code::RecordingErrorCode` / `Incomplete` — `error-1eb947da974293657444`
- `pocketstation::recording::error_code::RecordingErrorCode` / `InvalidSampleSpec` — `error-59fd53e59aecb01322e9`
- `pocketstation::recording::error_code::RecordingErrorCode` / `InvalidStemLabel` — `error-303563e54893f3cb8d02`
- `pocketstation::recording::error_code::RecordingErrorCode` / `IoFailed` — `error-5d924a6d7cf9a55a94a0`
- `pocketstation::recording::error_code::RecordingErrorCode` / `JsonFailed` — `error-5fd576960628ed5236fe`
- `pocketstation::recording::error_code::RecordingErrorCode` / `LineageMismatch` — `error-2424096aeb94c3c1b34e`

## API reference

- [Multistem Recording](/docs/concepts/multistem-recording.md)
- [Recording](/docs/reference/recording.md)

| Public declaration | Kind | Declared purpose | Source |
|---|---|---|---|
| `RecordingOutcome::completed_stems` | struct_field | Contains the completed stems owned or reported by `RecordingOutcome`. | `src/recording/writer.rs:115` |
| `RecordingOutcome::failed_stems` | struct_field | Contains the failed stems owned or reported by `RecordingOutcome`. | `src/recording/writer.rs:116` |
| `RecordingOutcome::stems` | struct_field | Contains the stems owned or reported by `RecordingOutcome`. | `src/recording/writer.rs:117` |
| `pocketstation::recording::config::RecorderStemConfig` | struct | Configures recorder stem behavior at its owning API boundary. | `src/recording/config.rs:55` |
| `pocketstation::recording::config::StemLabel` | struct | Stores the validated human-readable label used for one recording stem. | `src/recording/config.rs:20` |
| `pocketstation::recording::endpoint::MultistemRecordingReceipt` | struct | Retains the identity and observation access returned for multistem recording. | `src/recording/endpoint.rs:28` |
| `pocketstation::recording::endpoint::SessionMultistemEndpointCoordinator` | struct | Canonical Session-owned multistem recorder declaration. | `src/recording/endpoint.rs:49` |
| `pocketstation::recording::writer::DiscontinuityRecord` | struct | Records one immutable discontinuity observation. | `src/recording/writer.rs:93` |

## Related documentation

- [Glossary](/docs/glossary.md)
- [Multistem recording](/docs/concepts/multistem-recording.md)
- [PocketStation](/README.md)
- [Rust quickstart](/docs/getting-started/rust-quickstart.md)
- [Stop, drain, and finalization](/docs/lifecycle/stop-drain-finalize.md)
- [Inspect recording outcomes](/docs/how-to/inspect-recording-outcome.md)
- [Stop a Session and inspect failures](/docs/how-to/stop-session.md)
- [Behavior evidence index](/docs/reference/behavior-evidence.md)

## Evidence boundary

The claims on **Record independent stems** are anchored to Git snapshot `136e74888962558aa846d3143a19136a70936f45` and these primary files:

- `examples/product_quickstart.rs:1-21` (`DIRECT`)

For **Record independent stems**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.
