# Record independent stems

<!-- claims: CLM-GUIDE-009-CAP-001,CLM-GUIDE-009-SOURCE-001 -->

## Scope

- **Record aligned multistem output.** Configure stems and finalize per-stem outcomes through the recording endpoint lifecycle.

These statements describe repository contracts at the documented snapshot. They do not extend platform qualification, performance, retry, or delivery guarantees beyond the native API contracts and executable evidence.

## Prerequisites

Read the linked concept and confirm that target platform, Cargo features, source or provider dependencies, and application-owned permission work match this task. Keep returned typed errors and outcomes available for verification.

## Procedure

1. Set the recording root on SessionBuilder.
2. Call record with a label for each stem.
3. Start and run the Session.
4. Stop to trigger endpoint finalization.
5. Inspect overall and per-stem recording outcomes.

## APIs used

| Public declaration | Kind | Declared purpose | Source |
|---|---|---|---|
| `RecordingOutcome::completed_stems` | struct_field | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/recording/writer.rs:114` |
| `RecordingOutcome::failed_stems` | struct_field | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/recording/writer.rs:115` |
| `RecordingOutcome::stems` | struct_field | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/recording/writer.rs:116` |
| `pocketstation::recording::error_code::RecordingErrorCode` | enum | Stable language-neutral code for a recording failure. | `src/recording/error_code.rs:9` |
| `pocketstation::recording::writer::DiscontinuityRecord` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/recording/writer.rs:92` |
| `pocketstation::recording::writer::RecordingObservations` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/recording/writer.rs:130` |
| `pocketstation::recording::writer::RecordingOutcome` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/recording/writer.rs:111` |
| `pocketstation::recording::writer::RecordingStemOutcome` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/recording/writer.rs:120` |
| `pocketstation::session::lifecycle::trace::SessionTraceRecord` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/session/lifecycle/trace.rs:55` |
| `pocketstation::session::lifecycle::trace::SessionTraceRecorder` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/session/lifecycle/trace.rs:152` |
| `pocketstation::session::lifecycle::trace::SessionTraceRecorderHandle` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/session/lifecycle/trace.rs:108` |
| `pocketstation::session::lifecycle::trace::SessionTraceRecorderOutcome` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/session/lifecycle/trace.rs:70` |
| `pocketstation::recording::writer::DiscontinuityKind` | enum | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/recording/writer.rs:104` |
| `pocketstation::recording::writer::RecordingState` | enum | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/recording/writer.rs:85` |
| `pocketstation::session::lifecycle::trace::SessionTraceRecordKind` | enum | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/session/lifecycle/trace.rs:27` |
| `pocketstation::session::lifecycle::trace::SessionTraceRecorderFinishError` | enum | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/session/lifecycle/trace.rs:98` |

## Verify the outcome

The following test bodies are evidence only for their recorded setup:

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
- [Multistem recording](/docs/concepts/multistem-recording.md)
- [PocketStation](/README.md)
- [Rust quickstart](/docs/getting-started/rust-quickstart.md)
- [Stop, drain, and finalization](/docs/lifecycle/stop-drain-finalize.md)
- [Inspect recording outcomes](/docs/how-to/inspect-recording-outcome.md)
- [Stop a Session and inspect failures](/docs/how-to/stop-session.md)
- [Behavior evidence index](/docs/reference/behavior-evidence.md)

## Evidence boundary

This page was verified against Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `examples/product_quickstart.rs:1-61` (`DIRECT`)

A file's presence proves implementation or declaration at this snapshot. It does not by itself prove physical-device qualification, operational performance, retry safety, or behavior outside the recorded test conditions.
