# A recording is incomplete

<!-- claims: CLM-TRBL-007-CAP-001,CLM-TRBL-007-CAP-002,CLM-TRBL-007-CAP-003,CLM-TRBL-007-SOURCE-001 -->

Use this page when you observe **a recording is incomplete**. Diagnose the reported stage and identity before changing route, source, connector, or lifecycle policy.

## Distinguish the cause

Inspect overall recording state and every stem outcome after Session stop. A successful source run does not imply every file finalized.

## Diagnostic signals

- `pocketstation::session::declaration::typed_stream::TypedStreamError` / `OutputSignalMismatch` (`error-00e5716261eba0f8cf3d`)
- `pocketstation::session::error::SessionError` / `UnknownStem` (`error-00f6e798d158df66c847`)
- `pocketstation::session::error_code::SessionStartErrorCode` / `StartCancelled` (`error-01d3fc855e2a00319076`)
- `pocketstation::session::lifecycle::start_contract::SessionStartError` / `OperatorPrepare` (`error-023d6ab0b23a50a614ff`)
- `pocketstation::session::error_code::SessionStartErrorCode` / `TraceRecorderSetupFailed` (`error-0279b2b6b0cb3b5801bc`)
- `pocketstation::session::prepare::error::SessionPrepareError` / `MissingOperatorSignalInput` (`error-037ddc3e193da74177f8`)
- `pocketstation::recording::error_code::RecordingErrorCode` / `PermissionDenied` (`error-059bf10da1dcb4446e68`)
- `pocketstation::session::lifecycle::trace::SessionTraceValidationError` / `InvalidLayout` (`error-05c60389efcb84311921`)
- `pocketstation::session::prepare::error::SessionPrepareError` (`error-085082b521c14e5ecd1e`)
- `pocketstation::session::extensions::audio_input::buffer::AudioInputWriteErrorKind` / `Closed` (`error-08a7536094bfb2242b17`)
- `pocketstation::session::lifecycle::host::SessionEngineHostBuildError` / `EndpointExtensionRegistration` (`error-09837185c7fca0f70618`)
- `pocketstation::session::lifecycle::start_contract::SessionStartError` / `MissingEndpointDeclaration` (`error-0bc2f7c0b9f9dbf8ddd7`)
- `pocketstation::session::lifecycle::trace::SessionTraceRecorderStartError` / `ZeroCapacity` (`error-0bd6f58be40ade9a01fe`)
- `pocketstation::session::lifecycle::trace::SessionTraceValidationError` / `SequenceGap` (`error-0c04a3eedb823da29323`)
- `pocketstation::session::prepare::error::SessionPrepareError` / `MissingExternalAudioIngress` (`error-0cc0ae8a8cc4f1e05996`)
- `pocketstation::session::lifecycle::engine::SessionEngineBuildError` / `DuplicateSidecarId` (`error-0ce1015c73b65576cbeb`)
- `pocketstation::session::lifecycle::trace::SessionTraceValidationError` / `TimestampRegression` (`error-0d567cf627daa0adfee1`)
- `pocketstation::session::error_code::SessionStartErrorCode` / `MissingEndpointDeclaration` (`error-0e46a3d13215bfc3898f`)
- `pocketstation::session::extensions::audio_input::AudioInputConfigError` (`error-108ece57ea443c789d81`)
- `pocketstation::session::extensions::audio_input::source::AudioInputError` / `Manifest` (`error-11863b3a293345b0bb2d`)
- `pocketstation::session::compile::error::SessionCompileError` / `UnknownEndpointInputPort` (`error-1281b697f9f4d62194b1`)
- `pocketstation::session::prepare::error::SessionPrepareError` / `MissingTypedEdgePlan` (`error-12fef698a1fbec823e7e`)
- `pocketstation::session::prepare::error::SessionPrepareError` / `MissingAsyncOperatorFactory` (`error-1310461ef521d30d4686`)
- `pocketstation::session::error_code::SessionStartErrorCode` / `MissingEventReceiver` (`error-13dd584b4e2e8eaa490c`)
- `pocketstation::session::lifecycle::trace::SessionTraceValidationError` / `RecordAfterTerminal` (`error-16e269f1786471c2db63`)
- `pocketstation::session::error::SessionError` / `UnknownSourceOutput` (`error-16edb8f15b75c471db64`)
- `pocketstation::session::compile::error::SessionCompileError` / `AmbiguousEndpointInput` (`error-17674f66426c713d90a2`)
- `pocketstation::session::lifecycle::events::SessionRollbackFailure` (`error-1955a522796dc25c325d`)
- `pocketstation::session::declaration::typed_stream::TypedStreamError` / `MissingPort` (`error-1bd7ae7942029f778071`)
- `pocketstation::session::error_code::SessionStartErrorCode` (`error-1c7816652dd061fb1141`)

## Executable evidence

- `given_queued_audio_when_recording_cancelled_then_wav_header_is_playable_and_manifest_incomplete` exercises given queued audio when recording cancelled then wav header is playable and manifest incomplete under its recorded setup (`test-68bd764f7d45a4b8fbe7`).
- `given_derived_permission_epoch_when_later_frame_changes_it_then_recording_fails_closed` exercises given derived permission epoch when later frame changes it then recording fails closed under its recorded setup (`test-5c3ff5e741df683ae4d8`).
- `given_recording_codes_when_serialized_then_values_are_exact_and_unique` exercises given recording codes when serialized then values are exact and unique under its recorded setup (`test-bb7e1e95ee2acd51bc20`).
- `native_with_multistem_recording` exercises native with multistem recording under its recorded setup (`test-32fc9268bda35308f357`).
- `recording_receipts_total` exercises recording receipts total under its recorded setup (`test-a72ac96f1af1506d4bbd`).
- `given_dropped_records_when_validated_then_trace_is_incomplete` exercises given dropped records when validated then trace is incomplete under its recorded setup (`test-7647b415f36fc0ddd5a0`).
- `group_id` exercises group id under its recorded setup (`test-d43865384a3daad3b5b2`).
- `output_root` exercises output root under its recorded setup (`test-ef2731050600f4c1f575`).
- `given_session_context_and_two_first_frames_when_recorded_then_manifest_derives_capture_lineage_and_common_origin` exercises given session context and two first frames when recorded then manifest derives capture lineage and common origin under its recorded setup (`test-9352f7a742c1f649857a`).
- `given_session_recorder_input_without_audio_stem_origin_when_prepared_then_it_is_rejected` exercises given session recorder input without audio stem origin when prepared then it is rejected under its recorded setup (`test-497452363244c581f9e6`).
- `given_terminal_failure_when_projected_then_code_is_typed` exercises given terminal failure when projected then code is typed under its recorded setup (`test-0a50247a3c74a66f107d`).
- `cancel` exercises cancel under its recorded setup (`test-7938d06e3aca3a5cb043`).
- `session_dir` exercises session dir under its recorded setup (`test-6d6ced88f99690c75bed`).
- `given_failed_recorder_branch_when_more_frames_dispatched_then_healthy_branch_continues` exercises given failed recorder branch when more frames dispatched then healthy branch continues under its recorded setup (`test-a11ec53516f0e2c9bed1`).
- `given_fractional_stereo_gap_when_silence_is_sized_then_channels_remain_aligned` exercises given fractional stereo gap when silence is sized then channels remain aligned under its recorded setup (`test-19f291f86bfba30549dc`).

## Corrective action and retry

Apply only the action implied by the typed failure or violated precondition. Retry is not safe merely because a failure appears transient. When retryability or recovery is unknown, preserve the failure for application policy or maintainer review.

## Data and state

Treat frames, signals, files, acknowledgements, and finalization results produced before failure as potentially partial unless the terminal contract says otherwise. Inspect per-route, per-stem, and per-component outcomes.

## Related documentation

- [Glossary](/docs/glossary.md)
- [PocketStation](/README.md)
- [Stop a Session and inspect failures](/docs/how-to/stop-session.md)
- [Behavior evidence index](/docs/reference/behavior-evidence.md)
- [Rust API reference](/docs/reference/rust-api.md)
- [Session stop reports component failures](/docs/troubleshooting/session-stop.md)
- [Treat stop outcomes as data](/docs/best-practices/terminal-outcomes.md)
- [Stop, drain, and finalization](/docs/lifecycle/stop-drain-finalize.md)

## Evidence boundary

This page was verified against Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `src/recording/writer.rs:1-1248` (`DIRECT`)

A file's presence proves implementation or declaration at this snapshot. It does not by itself prove physical-device qualification, operational performance, retry safety, or behavior outside the recorded test conditions.
