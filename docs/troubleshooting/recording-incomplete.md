# A recording is incomplete

<!-- claims: CLM-TRBL-007-CAP-001,CLM-TRBL-007-CAP-002,CLM-TRBL-007-CAP-003,CLM-TRBL-007-SOURCE-001 -->

## Symptom

Session stop returns an incomplete recording or one or more failed stems.

## Evidenced causes

- A writer rejected frame shape, ordering, or an excessive gap.
- A file operation failed while writing or finalizing one stem.
- Stop reached recording finalization after an earlier component failure.

## Distinguish the causes

Inspect overall recording state, each stem outcome, written-frame counts, discontinuity records, and the stable recording error code.

## Diagnostic signals

- `pocketstation::recording::writer::RecorderError` (`error-9339dedf4c84c38da0e2`)
- `pocketstation::recording::writer::RecorderError` / `DuplicateStemLabel` (`error-d49408f9053c2456c283`)
- `pocketstation::recording::writer::RecorderError` / `FrameSpecMismatch` (`error-578da524e06cea0e8e00`)
- `pocketstation::recording::writer::RecorderError` / `GapTooLarge` (`error-061391a6f6d35c982334`)
- `pocketstation::recording::writer::RecorderError` / `InvalidSampleSpec` (`error-d05cd71eb7b744b378a5`)
- `pocketstation::recording::writer::RecorderError` / `InvalidStemLabel` (`error-d435376c2427ddf5f115`)
- `pocketstation::recording::writer::RecorderError` / `Io` (`error-c553215c0c3b0de2f46e`)
- `pocketstation::recording::writer::RecorderError` / `Json` (`error-2d48aa8fde935c4cbdb5`)
- `pocketstation::recording::writer::RecorderError` / `LineageMismatch` (`error-88f9c83dd92f31e2cd0d`)
- `pocketstation::recording::writer::RecorderError` / `OutputExists` (`error-3e33d869c826f0e7c489`)
- `pocketstation::recording::writer::RecorderError` / `PermissionDenied` (`error-2536419eb04af7410a5d`)
- `pocketstation::recording::writer::RecorderError` / `SessionMismatch` (`error-a51db8cf4dd2648b8bc1`)

## Executable evidence

- `given_queued_audio_when_recording_cancelled_then_wav_header_is_playable_and_manifest_incomplete` exercises given queued audio when recording cancelled then wav header is playable and manifest incomplete under its recorded setup (`test-7f4ab2688e1c16ab56c2`).
- `given_derived_permission_epoch_when_later_frame_changes_it_then_recording_fails_closed` exercises given derived permission epoch when later frame changes it then recording fails closed under its recorded setup (`test-8c7b0f326da2b4760c28`).
- `given_recording_codes_when_serialized_then_values_are_exact_and_unique` exercises given recording codes when serialized then values are exact and unique under its recorded setup (`test-921c1c5c1fdb60c7bf78`).
- `given_dropped_records_when_validated_then_trace_is_incomplete` exercises given dropped records when validated then trace is incomplete under its recorded setup (`test-5ee80a5b5a1b2ce74da0`).
- `given_session_context_and_two_first_frames_when_recorded_then_manifest_derives_capture_lineage_and_common_origin` exercises given session context and two first frames when recorded then manifest derives capture lineage and common origin under its recorded setup (`test-1d7c657b57a9c71d6591`).
- `given_session_recorder_input_without_audio_stem_origin_when_prepared_then_it_is_rejected` exercises given session recorder input without audio stem origin when prepared then it is rejected under its recorded setup (`test-a2e8d174434f9a88bf9e`).
- `given_terminal_failure_when_projected_then_code_is_typed` exercises given terminal failure when projected then code is typed under its recorded setup (`test-41c72d3a0caba393eac7`).
- `given_failed_recorder_branch_when_more_frames_dispatched_then_healthy_branch_continues` exercises given failed recorder branch when more frames dispatched then healthy branch continues under its recorded setup (`test-668e2a246f514118dc91`).
- `given_fractional_stereo_gap_when_silence_is_sized_then_channels_remain_aligned` exercises given fractional stereo gap when silence is sized then channels remain aligned under its recorded setup (`test-991391bffbd7771b8674`).
- `given_timestamp_and_sequence_gap_when_finished_then_silence_and_events_preserve_time` exercises given timestamp and sequence gap when finished then silence and events preserve time under its recorded setup (`test-c0bc88b64402027ef6d4`).
- `given_two_clock_mapped_stems_when_finished_then_two_aligned_playable_wavs_are_written` exercises given two clock mapped stems when finished then two aligned playable wavs are written under its recorded setup (`test-13b59c3a2ed9350468eb`).
- `given_compiled_derived_route_when_runtime_prepared_then_compiled_topology_is_preserved` exercises given compiled derived route when runtime prepared then compiled topology is preserved under its recorded setup (`test-21f8c08b6457bb762def`).
- `given_derived_stream_chain_when_compiled_then_operator_output_feeds_next_named_input` exercises given derived stream chain when compiled then operator output feeds next named input under its recorded setup (`test-081f9254eabd3bfeaad1`).
- `given_exact_process_instance_when_lowered_then_typed_declaration_remains_authoritative` exercises given exact process instance when lowered then typed declaration remains authoritative under its recorded setup (`test-b731381bbb6154df587d`).
- `given_extension_key_matching_old_metadata_when_compiled_then_value_remains_opaque` exercises given extension key matching old metadata when compiled then value remains opaque under its recorded setup (`test-8a30a347c6b7a008fb97`).

## Corrective action

Preserve completed stems, correct the failing output path or frame contract, and start a new recording when needed.

## Retry and incomplete state

Do not append or retry finalization unless the writer contract permits it. Completed files may be usable while failed stems are partial or absent.

## Related reference

- [Multistem Recording](/docs/concepts/multistem-recording.md)
- [Recording](/docs/errors/recording.md)

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

The claims on **A recording is incomplete** are anchored to Git snapshot `136e74888962558aa846d3143a19136a70936f45` and these primary files:

- `src/recording/writer.rs:1-1262` (`DIRECT`)

For **A recording is incomplete**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.
