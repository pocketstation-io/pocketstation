# Recording API

<!-- claims: CLM-REF-008-CAP-001,CLM-REF-008-SOURCE-001 -->

## Scope

- **Record aligned multistem output.** Configure stems and finalize per-stem outcomes through the recording endpoint lifecycle.

These statements describe repository contracts at the documented snapshot. They do not extend platform qualification, performance, retry, or delivery guarantees beyond the native API contracts and executable evidence.

## Reference authority

The generated [docs.rs API](https://docs.rs/pocketstation/latest/pocketstation/) is the exhaustive symbol-level Rust reference. Human reference pages organize responsibilities and cross-boundary behavior; they do not duplicate every rustdoc signature.

## Public surface

| Declaration | Kind | Purpose | Source |
|---|---|---|---|
| `pocketstation::recording::error_code::RecordingErrorCode` | enum | Stable language-neutral code for a recording failure. | `src/recording/error_code.rs:9` |
| `pocketstation::recording::writer::DiscontinuityKind` | enum | Selects the discontinuity kind used by PocketStation. | `src/recording/writer.rs:104` |
| `pocketstation::recording::writer::RecordingState` | enum | Selects the recording state used by PocketStation. | `src/recording/writer.rs:85` |
| `as_str` | function | Returns the stable string representation of `RecordingErrorCode`. | `src/recording/error_code.rs:32` |
| `pocketstation::recording::error_code::recording_outcome_error_code` | function | Returns the recording outcome error code associated with `error_code`. | `src/recording/error_code.rs:82` |
| `pocketstation::recording::writer::DiscontinuityRecord` | struct | Represents discontinuity record in the PocketStation API. | `src/recording/writer.rs:92` |
| `pocketstation::recording::writer::RecordingObservations` | struct | Reports the recording observations collected at an observation boundary. | `src/recording/writer.rs:130` |
| `pocketstation::recording::writer::RecordingOutcome` | struct | Reports the structured recording outcome. | `src/recording/writer.rs:111` |
| `pocketstation::recording::writer::RecordingStemOutcome` | struct | Reports the structured recording stem outcome. | `src/recording/writer.rs:120` |
| `DiscontinuityRecord::kind` | struct_field | Stores the kind associated with `DiscontinuityRecord`. | `src/recording/writer.rs:95` |
| `DiscontinuityRecord::label` | struct_field | Stores the label associated with `DiscontinuityRecord`. | `src/recording/writer.rs:94` |
| `DiscontinuityRecord::sequence_end` | struct_field | Stores the sequence end associated with `DiscontinuityRecord`. | `src/recording/writer.rs:99` |
| `DiscontinuityRecord::sequence_start` | struct_field | Stores the sequence start associated with `DiscontinuityRecord`. | `src/recording/writer.rs:98` |
| `DiscontinuityRecord::stem_id` | struct_field | Identifies the stem associated with `DiscontinuityRecord`. | `src/recording/writer.rs:93` |
| `DiscontinuityRecord::timestamp_end_ns` | struct_field | Stores the timestamp end value for `DiscontinuityRecord`, in nanoseconds. | `src/recording/writer.rs:97` |
| `DiscontinuityRecord::timestamp_start_ns` | struct_field | Stores the timestamp start value for `DiscontinuityRecord`, in nanoseconds. | `src/recording/writer.rs:96` |
| `RecordingObservations::discontinuities_total` | struct_field | Counts the total number of discontinuities observed by `RecordingObservations`. | `src/recording/writer.rs:134` |
| `RecordingObservations::failures_total` | struct_field | Counts the total number of failures observed by `RecordingObservations`. | `src/recording/writer.rs:135` |
| `RecordingObservations::frames_received_total` | struct_field | Counts the total number of frames received observed by `RecordingObservations`. | `src/recording/writer.rs:131` |
| `RecordingObservations::frames_rejected_total` | struct_field | Counts the total number of frames rejected observed by `RecordingObservations`. | `src/recording/writer.rs:133` |
| `RecordingObservations::frames_written_total` | struct_field | Counts the total number of frames written observed by `RecordingObservations`. | `src/recording/writer.rs:132` |
| `RecordingOutcome::completed_stems` | struct_field | Stores the completed stems associated with `RecordingOutcome`. | `src/recording/writer.rs:114` |
| `RecordingOutcome::failed_stems` | struct_field | Stores the failed stems associated with `RecordingOutcome`. | `src/recording/writer.rs:115` |
| `RecordingOutcome::session_dir` | struct_field | Stores the session dir associated with `RecordingOutcome`. | `src/recording/writer.rs:112` |
| `RecordingOutcome::state` | struct_field | Stores the state associated with `RecordingOutcome`. | `src/recording/writer.rs:113` |
| `RecordingOutcome::stems` | struct_field | Stores the stems associated with `RecordingOutcome`. | `src/recording/writer.rs:116` |
| `RecordingStemOutcome::edge_observations` | struct_field | Stores the edge observations associated with `RecordingStemOutcome`. | `src/recording/writer.rs:126` |
| `RecordingStemOutcome::error` | struct_field | Stores the error associated with `RecordingStemOutcome`. | `src/recording/writer.rs:125` |
| `RecordingStemOutcome::gap_ranges` | struct_field | Stores the gap ranges associated with `RecordingStemOutcome`. | `src/recording/writer.rs:124` |
| `RecordingStemOutcome::label` | struct_field | Stores the label associated with `RecordingStemOutcome`. | `src/recording/writer.rs:121` |
| `RecordingStemOutcome::stale_frames` | struct_field | Stores the stale frames associated with `RecordingStemOutcome`. | `src/recording/writer.rs:123` |
| `RecordingStemOutcome::written_frames` | struct_field | Stores the written frames associated with `RecordingStemOutcome`. | `src/recording/writer.rs:122` |
| `pocketstation::recording::error_code::RecordingErrorCode::DuplicateStemLabel` | variant | Reports duplicate stem label. | `src/recording/error_code.rs:12` |
| `pocketstation::recording::error_code::RecordingErrorCode::FrameSpecMismatch` | variant | Reports frame spec mismatch. | `src/recording/error_code.rs:18` |
| `pocketstation::recording::error_code::RecordingErrorCode::GapTooLarge` | variant | Reports gap too large. | `src/recording/error_code.rs:21` |
| `pocketstation::recording::error_code::RecordingErrorCode::Incomplete` | variant | Reports incomplete. | `src/recording/error_code.rs:28` |
| `pocketstation::recording::error_code::RecordingErrorCode::InvalidSampleSpec` | variant | Reports invalid sample spec. | `src/recording/error_code.rs:15` |
| `pocketstation::recording::error_code::RecordingErrorCode::InvalidStemLabel` | variant | Reports invalid stem label. | `src/recording/error_code.rs:11` |
| `pocketstation::recording::error_code::RecordingErrorCode::IoFailed` | variant | Reports I/O failed. | `src/recording/error_code.rs:24` |
| `pocketstation::recording::error_code::RecordingErrorCode::JsonFailed` | variant | Reports json failed. | `src/recording/error_code.rs:26` |
| `pocketstation::recording::error_code::RecordingErrorCode::LineageMismatch` | variant | Reports lineage mismatch. | `src/recording/error_code.rs:17` |
| `pocketstation::recording::error_code::RecordingErrorCode::NotFinalized` | variant | Reports not finalized. | `src/recording/error_code.rs:27` |
| `pocketstation::recording::error_code::RecordingErrorCode::OutputExists` | variant | Reports output exists. | `src/recording/error_code.rs:10` |
| `pocketstation::recording::error_code::RecordingErrorCode::PermissionDenied` | variant | Reports that the required permission was denied. | `src/recording/error_code.rs:14` |
| `pocketstation::recording::error_code::RecordingErrorCode::SessionMismatch` | variant | Reports session mismatch. | `src/recording/error_code.rs:13` |
| `pocketstation::recording::error_code::RecordingErrorCode::SourceMismatch` | variant | Reports source mismatch. | `src/recording/error_code.rs:16` |
| `pocketstation::recording::error_code::RecordingErrorCode::TimestampOutOfRange` | variant | Reports timestamp out of range. | `src/recording/error_code.rs:20` |
| `pocketstation::recording::error_code::RecordingErrorCode::TooManyGaps` | variant | Reports too many gaps. | `src/recording/error_code.rs:22` |
| `pocketstation::recording::error_code::RecordingErrorCode::UnalignedSamples` | variant | Reports unaligned samples. | `src/recording/error_code.rs:19` |
| `pocketstation::recording::error_code::RecordingErrorCode::WavFailed` | variant | Reports wav failed. | `src/recording/error_code.rs:25` |
| `pocketstation::recording::error_code::RecordingErrorCode::WorkerPanicked` | variant | Reports worker panicked. | `src/recording/error_code.rs:23` |
| `pocketstation::recording::writer::DiscontinuityKind::OverlapRejected` | variant | Selects overlap rejected behavior for `DiscontinuityKind`. | `src/recording/writer.rs:107` |
| `pocketstation::recording::writer::DiscontinuityKind::SequenceGap` | variant | Selects sequence gap behavior for `DiscontinuityKind`. | `src/recording/writer.rs:106` |
| `pocketstation::recording::writer::DiscontinuityKind::TimestampGap` | variant | Selects timestamp gap behavior for `DiscontinuityKind`. | `src/recording/writer.rs:105` |
| `pocketstation::recording::writer::RecordingState::Complete` | variant | Indicates the complete state for `RecordingState`. | `src/recording/writer.rs:87` |
| `pocketstation::recording::writer::RecordingState::Incomplete` | variant | Indicates the incomplete state for `RecordingState`. | `src/recording/writer.rs:88` |
| `pocketstation::recording::writer::RecordingState::Recording` | variant | Indicates the recording state for `RecordingState`. | `src/recording/writer.rs:86` |

## Interpretation

An inventory row establishes that a declaration, test, lifecycle operation, configuration surface, or protocol element exists at the frozen snapshot. Fields shown as unknown remain deliberately unspecified. Consult the native API and error contract before relying on panic behavior, blocking, cancellation, ordering, limits, retry, or recovery.

## Related documentation

- [Glossary](/docs/glossary.md)
- [Multistem recording](/docs/concepts/multistem-recording.md)
- [PocketStation](/README.md)
- [Rust quickstart](/docs/getting-started/rust-quickstart.md)
- [Stop, drain, and finalization](/docs/lifecycle/stop-drain-finalize.md)
- [Inspect recording outcomes](/docs/how-to/inspect-recording-outcome.md)
- [Record independent stems](/docs/how-to/record-stems.md)
- [Stop a Session and inspect failures](/docs/how-to/stop-session.md)

## Evidence boundary

This page was verified against Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `src/recording/mod.rs:1-25` (`DIRECT`)

A file's presence proves implementation or declaration at this snapshot. It does not by itself prove physical-device qualification, operational performance, retry safety, or behavior outside the recorded test conditions.
