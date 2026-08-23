# Multistem recording

<!-- claims: CLM-DOC-019-CAP-001,CLM-DOC-019-SOURCE-001 -->

Configure stems and finalize per-stem outcomes through the recording endpoint lifecycle.

## Scope

- **Record aligned multistem output.** Configure stems and finalize per-stem outcomes through the recording endpoint lifecycle.

These statements describe repository contracts at the documented snapshot. They do not extend platform qualification, performance, retry, or delivery guarantees beyond the native API contracts and executable evidence.

## Contract surface

| Public declaration | Kind | Declared purpose | Source |
|---|---|---|---|
| `pocketstation::recording::error_code::RecordingErrorCode` | enum | Stable language-neutral code for a recording failure. | `src/recording/error_code.rs:9` |
| `pocketstation::recording::writer::DiscontinuityRecord` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/recording/writer.rs:92` |
| `pocketstation::recording::writer::RecordingObservations` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/recording/writer.rs:130` |
| `pocketstation::recording::writer::RecordingOutcome` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/recording/writer.rs:111` |
| `pocketstation::recording::writer::RecordingStemOutcome` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/recording/writer.rs:120` |
| `pocketstation::recording::writer::DiscontinuityKind` | enum | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/recording/writer.rs:104` |
| `pocketstation::recording::writer::RecordingState` | enum | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/recording/writer.rs:85` |
| `pocketstation::recording::error_code::recording_outcome_error_code` | function | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/recording/error_code.rs:82` |
| `pocketstation::recording::error_code::RecordingErrorCode::DuplicateStemLabel` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/recording/error_code.rs:12` |
| `pocketstation::recording::error_code::RecordingErrorCode::FrameSpecMismatch` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/recording/error_code.rs:18` |
| `pocketstation::recording::error_code::RecordingErrorCode::GapTooLarge` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/recording/error_code.rs:21` |
| `pocketstation::recording::error_code::RecordingErrorCode::Incomplete` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/recording/error_code.rs:28` |
| `pocketstation::recording::error_code::RecordingErrorCode::InvalidSampleSpec` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/recording/error_code.rs:15` |
| `pocketstation::recording::error_code::RecordingErrorCode::InvalidStemLabel` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/recording/error_code.rs:11` |
| `pocketstation::recording::error_code::RecordingErrorCode::IoFailed` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/recording/error_code.rs:24` |
| `pocketstation::recording::error_code::RecordingErrorCode::JsonFailed` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/recording/error_code.rs:26` |
| `pocketstation::recording::error_code::RecordingErrorCode::LineageMismatch` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/recording/error_code.rs:17` |
| `pocketstation::recording::error_code::RecordingErrorCode::NotFinalized` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/recording/error_code.rs:27` |
| `pocketstation::recording::error_code::RecordingErrorCode::OutputExists` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/recording/error_code.rs:10` |
| `pocketstation::recording::error_code::RecordingErrorCode::PermissionDenied` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/recording/error_code.rs:14` |

## Where you encounter it

- **Record separate stems** — Record independent source stems and inspect finalization outcomes after Session stop.

## Behavior established by tests

The following test bodies are evidence only for their recorded setup:

- `native_with_multistem_recording` — native with multistem recording (`src/session/lifecycle/host.rs:46`; `test-32fc9268bda35308f357`).
- `given_derived_permission_epoch_when_later_frame_changes_it_then_recording_fails_closed` — given derived permission epoch when later frame changes it then recording fails closed (`src/recording/endpoint/tests.rs:287`; `test-5c3ff5e741df683ae4d8`).
- `given_recording_codes_when_serialized_then_values_are_exact_and_unique` — given recording codes when serialized then values are exact and unique (`src/recording/error_code.rs:95`; `test-bb7e1e95ee2acd51bc20`).
- `given_queued_audio_when_recording_cancelled_then_wav_header_is_playable_and_manifest_incomplete` — given queued audio when recording cancelled then wav header is playable and manifest incomplete (`src/recording/writer/tests.rs:215`; `test-68bd764f7d45a4b8fbe7`).
- `given_registered_multistem_recorder_when_host_built_then_receipt_is_retained` — given registered multistem recorder when host built then receipt is retained (`src/session/lifecycle/host.rs:702`; `test-8415e09aa158a8386c8e`).
- `recording_receipts_total` — recording receipts total (`src/session/lifecycle/host.rs:110`; `test-a72ac96f1af1506d4bbd`).
- `group_id` — group id (`src/recording/endpoint.rs:67`; `test-d43865384a3daad3b5b2`).
- `output_root` — output root (`src/recording/endpoint.rs:62`; `test-ef2731050600f4c1f575`).
- `given_session_context_and_two_first_frames_when_recorded_then_manifest_derives_capture_lineage_and_common_origin` — given session context and two first frames when recorded then manifest derives capture lineage and common origin (`src/recording/endpoint/tests.rs:187`; `test-9352f7a742c1f649857a`).
- `given_session_recorder_input_without_audio_stem_origin_when_prepared_then_it_is_rejected` — given session recorder input without audio stem origin when prepared then it is rejected (`src/recording/endpoint/tests.rs:262`; `test-497452363244c581f9e6`).
- `given_terminal_failure_when_projected_then_code_is_typed` — given terminal failure when projected then code is typed (`src/recording/error_code.rs:158`; `test-0a50247a3c74a66f107d`).
- `cancel` — cancel (`src/recording/writer.rs:280`; `test-7938d06e3aca3a5cb043`).

## Boundaries

The compiler inventory establishes names, kinds, visibility, and signatures. Tests establish only their exercised conditions. Where retryability, ordering, cancellation, physical qualification, or recovery is not declared, this page leaves it unspecified.

## Related documentation

- [Glossary](/docs/glossary.md)
- [PocketStation](/README.md)
- [Rust quickstart](/docs/getting-started/rust-quickstart.md)
- [Stop, drain, and finalization](/docs/lifecycle/stop-drain-finalize.md)
- [Inspect recording outcomes](/docs/how-to/inspect-recording-outcome.md)
- [Record independent stems](/docs/how-to/record-stems.md)
- [Stop a Session and inspect failures](/docs/how-to/stop-session.md)
- [Behavior evidence index](/docs/reference/behavior-evidence.md)

## Evidence boundary

This page was verified against Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `src/recording/endpoint.rs:1-740` (`DIRECT`)
- `src/recording/writer.rs:1-1248` (`DIRECT`)

A file's presence proves implementation or declaration at this snapshot. It does not by itself prove physical-device qualification, operational performance, retry safety, or behavior outside the recorded test conditions.
