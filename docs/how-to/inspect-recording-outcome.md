# Inspect recording outcomes

<!-- claims: CLM-GUIDE-010-CAP-001,CLM-GUIDE-010-CAP-002,CLM-GUIDE-010-SOURCE-001 -->

## Scope

- **Record aligned multistem output.** Configure stems and finalize per-stem outcomes through the recording endpoint lifecycle.
- **Classify public failures.** Expose stable typed errors and cross-boundary error codes without inferring retry or recovery guarantees.

These statements describe repository contracts at the documented snapshot. They do not extend platform qualification, performance, retry, or delivery guarantees beyond the native API contracts and executable evidence.

## Prerequisites

Read the linked concept and confirm that target platform, Cargo features, source or provider dependencies, and application-owned permission work match this task. Keep returned typed errors and outcomes available for verification.

## Procedure

1. Retain RunningSession until stop returns.
2. Preserve SessionStopOutcome.
3. Read recording_outcome after stop.
4. Check overall state plus completed and failed stem counts.
5. Use error codes and per-stem results to diagnose partial finalization.

## APIs used

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

## Verify the outcome

The following test bodies are evidence only for their recorded setup:

- `given_derived_permission_epoch_when_later_frame_changes_it_then_recording_fails_closed` — given derived permission epoch when later frame changes it then recording fails closed (`src/recording/endpoint/tests.rs:287`; `test-5c3ff5e741df683ae4d8`).
- `given_recording_codes_when_serialized_then_values_are_exact_and_unique` — given recording codes when serialized then values are exact and unique (`src/recording/error_code.rs:95`; `test-bb7e1e95ee2acd51bc20`).
- `given_queued_audio_when_recording_cancelled_then_wav_header_is_playable_and_manifest_incomplete` — given queued audio when recording cancelled then wav header is playable and manifest incomplete (`src/recording/writer/tests.rs:215`; `test-68bd764f7d45a4b8fbe7`).
- `native_with_multistem_recording` — native with multistem recording (`src/session/lifecycle/host.rs:46`; `test-32fc9268bda35308f357`).
- `recording_receipts_total` — recording receipts total (`src/session/lifecycle/host.rs:110`; `test-a72ac96f1af1506d4bbd`).
- `group_id` — group id (`src/recording/endpoint.rs:67`; `test-d43865384a3daad3b5b2`).
- `output_root` — output root (`src/recording/endpoint.rs:62`; `test-ef2731050600f4c1f575`).
- `given_session_context_and_two_first_frames_when_recorded_then_manifest_derives_capture_lineage_and_common_origin` — given session context and two first frames when recorded then manifest derives capture lineage and common origin (`src/recording/endpoint/tests.rs:187`; `test-9352f7a742c1f649857a`).
- `given_session_recorder_input_without_audio_stem_origin_when_prepared_then_it_is_rejected` — given session recorder input without audio stem origin when prepared then it is rejected (`src/recording/endpoint/tests.rs:262`; `test-497452363244c581f9e6`).
- `given_terminal_failure_when_projected_then_code_is_typed` — given terminal failure when projected then code is typed (`src/recording/error_code.rs:158`; `test-0a50247a3c74a66f107d`).
- `cancel` — cancel (`src/recording/writer.rs:280`; `test-7938d06e3aca3a5cb043`).
- `session_dir` — session dir (`src/recording/writer.rs:243`; `test-6d6ced88f99690c75bed`).

## Failure signals

- `pocketstation::session::declaration::typed_stream::TypedStreamError` / `OutputSignalMismatch` — `error-00e5716261eba0f8cf3d`
- `pocketstation::session::error::SessionError` / `UnknownStem` — `error-00f6e798d158df66c847`
- `pocketstation::session::error_code::SessionStartErrorCode` / `StartCancelled` — `error-01d3fc855e2a00319076`
- `pocketstation::session::lifecycle::start_contract::SessionStartError` / `OperatorPrepare` — `error-023d6ab0b23a50a614ff`
- `pocketstation::session::error_code::SessionStartErrorCode` / `TraceRecorderSetupFailed` — `error-0279b2b6b0cb3b5801bc`
- `pocketstation::session::prepare::error::SessionPrepareError` / `MissingOperatorSignalInput` — `error-037ddc3e193da74177f8`
- `pocketstation::recording::error_code::RecordingErrorCode` / `PermissionDenied` — `error-059bf10da1dcb4446e68`
- `pocketstation::session::lifecycle::trace::SessionTraceValidationError` / `InvalidLayout` — `error-05c60389efcb84311921`
- `pocketstation::session::prepare::error::SessionPrepareError` — `error-085082b521c14e5ecd1e`
- `pocketstation::session::extensions::audio_input::buffer::AudioInputWriteErrorKind` / `Closed` — `error-08a7536094bfb2242b17`
- `pocketstation::session::lifecycle::host::SessionEngineHostBuildError` / `EndpointExtensionRegistration` — `error-09837185c7fca0f70618`
- `pocketstation::session::lifecycle::start_contract::SessionStartError` / `MissingEndpointDeclaration` — `error-0bc2f7c0b9f9dbf8ddd7`

Retry only when the relevant API or error contract explicitly permits it. An error name, a transient-looking message, or a successful prior run is not retry evidence.

## Related documentation

- [Glossary](/docs/glossary.md)
- [PocketStation](/README.md)
- [Stop a Session and inspect failures](/docs/how-to/stop-session.md)
- [Behavior evidence index](/docs/reference/behavior-evidence.md)
- [Rust API reference](/docs/reference/rust-api.md)
- [A recording is incomplete](/docs/troubleshooting/recording-incomplete.md)
- [Recording failures](/docs/errors/recording.md)
- [Session stop reports component failures](/docs/troubleshooting/session-stop.md)

## Evidence boundary

This page was verified against Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `src/recording/writer.rs:1-1248` (`DIRECT`)
- `src/session/extensions/recording.rs:1-121` (`DIRECT`)

A file's presence proves implementation or declaration at this snapshot. It does not by itself prove physical-device qualification, operational performance, retry safety, or behavior outside the recorded test conditions.
