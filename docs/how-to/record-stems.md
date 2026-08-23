# Record independent stems

<!-- claims: CLM-GUIDE-009-CAP-001,CLM-GUIDE-009-SOURCE-001 -->

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

## Important consequence

File finalization happens during stop; do not declare success from captured-frame counts alone.

## Verify the outcome

Stop returns a recording outcome whose completed stems match the declared labels and whose failed-stem list is empty.

Executable evidence selected for **Record independent stems** is limited to each test's recorded setup and assertions:

- `given_derived_permission_epoch_when_later_frame_changes_it_then_recording_fails_closed` — given derived permission epoch when later frame changes it then recording fails closed (`src/recording/endpoint/tests.rs:287`; `test-5c3ff5e741df683ae4d8`).
- `given_session_context_and_two_first_frames_when_recorded_then_manifest_derives_capture_lineage_and_common_origin` — given session context and two first frames when recorded then manifest derives capture lineage and common origin (`src/recording/endpoint/tests.rs:187`; `test-9352f7a742c1f649857a`).
- `given_session_recorder_input_without_audio_stem_origin_when_prepared_then_it_is_rejected` — given session recorder input without audio stem origin when prepared then it is rejected (`src/recording/endpoint/tests.rs:262`; `test-497452363244c581f9e6`).
- `given_recording_codes_when_serialized_then_values_are_exact_and_unique` — given recording codes when serialized then values are exact and unique (`src/recording/error_code.rs:95`; `test-bb7e1e95ee2acd51bc20`).
- `given_failed_recorder_branch_when_more_frames_dispatched_then_healthy_branch_continues` — given failed recorder branch when more frames dispatched then healthy branch continues (`src/recording/writer/tests.rs:246`; `test-a11ec53516f0e2c9bed1`).
- `given_queued_audio_when_recording_cancelled_then_wav_header_is_playable_and_manifest_incomplete` — given queued audio when recording cancelled then wav header is playable and manifest incomplete (`src/recording/writer/tests.rs:215`; `test-68bd764f7d45a4b8fbe7`).
- `given_two_clock_mapped_stems_when_finished_then_two_aligned_playable_wavs_are_written` — given two clock mapped stems when finished then two aligned playable wavs are written (`src/recording/writer/tests.rs:107`; `test-8457eec8d2b39e34dee4`).
- `given_product_spec_when_compiled_then_six_independent_edges_are_planned` — given product spec when compiled then six independent edges are planned (`src/session/compile/tests.rs:497`; `test-39097f0adf1fe30dc87a`).
- `given_two_derived_destinations_when_prepared_then_independent_branch_plans_are_preserved` — given two derived destinations when prepared then independent branch plans are preserved (`src/session/compile/tests.rs:685`; `test-d6762b694308bbfc1e5c`).
- `given_two_record_declarations_when_frozen_then_default_group_identity_is_explicit_and_stable` — given two record declarations when frozen then default group identity is explicit and stable (`src/session/declaration/draft.rs:1224`; `test-ad12556b25e1d517daba`).
- `given_two_stems_when_sent_to_one_endpoint_then_routes_are_distinct` — given two stems when sent to one endpoint then routes are distinct (`src/session/declaration/draft.rs:1156`; `test-8ad7b42874649ad3c238`).
- `given_current_schema_when_version_read_then_derived_route_extension_is_recorded` — given current schema when version read then derived route extension is recorded (`src/session/declaration/spec.rs:853`; `test-38ca4207242ff3340ec8`).

## Failure signals

- `pocketstation::recording::error_code::RecordingErrorCode` — `error-68ad0dee12406c0e6362`
- `pocketstation::recording::error_code::RecordingErrorCode` / `DuplicateStemLabel` — `error-dce6c6ee7c6a5e364db2`
- `pocketstation::recording::error_code::RecordingErrorCode` / `FrameSpecMismatch` — `error-12c636b42e1cdd1deab7`
- `pocketstation::recording::error_code::RecordingErrorCode` / `GapTooLarge` — `error-38b0ea1c2434c574c637`
- `pocketstation::recording::error_code::RecordingErrorCode` / `Incomplete` — `error-590cdd41a3dd3ec6ffcd`
- `pocketstation::recording::error_code::RecordingErrorCode` / `InvalidSampleSpec` — `error-9a35932efb9b3e31e85a`
- `pocketstation::recording::error_code::RecordingErrorCode` / `InvalidStemLabel` — `error-b6f7bbf887c2eb4077ad`
- `pocketstation::recording::error_code::RecordingErrorCode` / `IoFailed` — `error-b17153cc6b42bd71e6c3`
- `pocketstation::recording::error_code::RecordingErrorCode` / `JsonFailed` — `error-841ee8b8be291cf14dc3`
- `pocketstation::recording::error_code::RecordingErrorCode` / `LineageMismatch` — `error-cd7f8f4c84e98696daec`

## API reference

- [Multistem Recording](/docs/concepts/multistem-recording.md)
- [Recording](/docs/reference/recording.md)

| Public declaration | Kind | Declared purpose | Source |
|---|---|---|---|
| `RecordingOutcome::completed_stems` | struct_field | Stores the completed stems used by `RecordingOutcome`. | `src/recording/writer.rs:114` |
| `RecordingOutcome::failed_stems` | struct_field | Stores the failed stems used by `RecordingOutcome`. | `src/recording/writer.rs:115` |
| `RecordingOutcome::stems` | struct_field | Stores the stems used by `RecordingOutcome`. | `src/recording/writer.rs:116` |
| `pocketstation::recording::config::RecorderStemConfig` | struct | Configures recorder stem behavior at its owning API boundary. | `src/recording/config.rs:55` |
| `pocketstation::recording::config::StemLabel` | struct | Stores the validated human-readable label used for one recording stem. | `src/recording/config.rs:20` |
| `pocketstation::recording::endpoint::MultistemRecordingReceipt` | struct | Retains the identity and observation access returned for multistem recording. | `src/recording/endpoint.rs:28` |
| `pocketstation::recording::endpoint::SessionMultistemEndpointCoordinator` | struct | Canonical Session-owned multistem recorder declaration. | `src/recording/endpoint.rs:49` |
| `pocketstation::recording::writer::DiscontinuityRecord` | struct | Records one immutable discontinuity observation. | `src/recording/writer.rs:92` |

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

The claims on **Record independent stems** are anchored to Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `examples/product_quickstart.rs:1-61` (`DIRECT`)

For **Record independent stems**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.
