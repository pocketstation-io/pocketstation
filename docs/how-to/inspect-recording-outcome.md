# Inspect recording outcomes

<!-- claims: CLM-GUIDE-010-CAP-001,CLM-GUIDE-010-CAP-002,CLM-GUIDE-010-SOURCE-001 -->

## Scope

- **Record aligned multistem output.** Configure stems and finalize per-stem outcomes through the recording endpoint lifecycle.
- **Classify public failures.** Expose stable typed errors and cross-boundary error codes without inferring retry or recovery guarantees.

The scope of **Inspect recording outcomes** ends at the native contracts and executable conditions cited below. Platform qualification, performance, retry, and delivery require their own explicit evidence.

## Prerequisites

A stopped Session with its structured `SessionStopOutcome` still available.

## Procedure

1. Retain RunningSession until stop returns.
2. Preserve SessionStopOutcome.
3. Read recording_outcome after stop.
4. Check overall state plus completed and failed stem counts.
5. Use error codes and per-stem results to diagnose partial finalization.

## Important consequence

A partial recording can coexist with useful completed stems; preserve both sides of the outcome.

## Verify the outcome

Overall state, completed stems, failed stems, and per-stem errors have all been inspected before the outcome is discarded.

Executable evidence selected for **Inspect recording outcomes** is limited to each test's recorded setup and assertions:

- `given_derived_permission_epoch_when_later_frame_changes_it_then_recording_fails_closed` — given derived permission epoch when later frame changes it then recording fails closed (`src/recording/endpoint/tests.rs:287`; `test-5c3ff5e741df683ae4d8`).
- `given_recording_codes_when_serialized_then_values_are_exact_and_unique` — given recording codes when serialized then values are exact and unique (`src/recording/error_code.rs:95`; `test-bb7e1e95ee2acd51bc20`).
- `given_queued_audio_when_recording_cancelled_then_wav_header_is_playable_and_manifest_incomplete` — given queued audio when recording cancelled then wav header is playable and manifest incomplete (`src/recording/writer/tests.rs:215`; `test-68bd764f7d45a4b8fbe7`).
- `given_session_context_and_two_first_frames_when_recorded_then_manifest_derives_capture_lineage_and_common_origin` — given session context and two first frames when recorded then manifest derives capture lineage and common origin (`src/recording/endpoint/tests.rs:187`; `test-9352f7a742c1f649857a`).
- `given_session_recorder_input_without_audio_stem_origin_when_prepared_then_it_is_rejected` — given session recorder input without audio stem origin when prepared then it is rejected (`src/recording/endpoint/tests.rs:262`; `test-497452363244c581f9e6`).
- `given_terminal_failure_when_projected_then_code_is_typed` — given terminal failure when projected then code is typed (`src/recording/error_code.rs:158`; `test-0a50247a3c74a66f107d`).
- `given_failed_recorder_branch_when_more_frames_dispatched_then_healthy_branch_continues` — given failed recorder branch when more frames dispatched then healthy branch continues (`src/recording/writer/tests.rs:246`; `test-a11ec53516f0e2c9bed1`).
- `given_fractional_stereo_gap_when_silence_is_sized_then_channels_remain_aligned` — given fractional stereo gap when silence is sized then channels remain aligned (`src/recording/writer/tests.rs:99`; `test-19f291f86bfba30549dc`).
- `given_timestamp_and_sequence_gap_when_finished_then_silence_and_events_preserve_time` — given timestamp and sequence gap when finished then silence and events preserve time (`src/recording/writer/tests.rs:169`; `test-186865304c35cff29002`).
- `given_two_clock_mapped_stems_when_finished_then_two_aligned_playable_wavs_are_written` — given two clock mapped stems when finished then two aligned playable wavs are written (`src/recording/writer/tests.rs:107`; `test-8457eec8d2b39e34dee4`).
- `given_compiled_derived_route_when_runtime_prepared_then_compiled_topology_is_preserved` — given compiled derived route when runtime prepared then compiled topology is preserved (`src/session/compile/tests.rs:659`; `test-f38493cc0593f603aece`).
- `given_derived_stream_chain_when_compiled_then_operator_output_feeds_next_named_input` — given derived stream chain when compiled then operator output feeds next named input (`src/session/compile/tests.rs:587`; `test-e9a24a392741b4dbe6e7`).

## Failure signals

- `pocketstation::recording::writer::RecorderError` — `error-d2e8d472ee35c4189976`
- `pocketstation::recording::writer::RecorderError` / `DuplicateStemLabel` — `error-1c6e40d1640c72bd48b2`
- `pocketstation::recording::writer::RecorderError` / `FrameSpecMismatch` — `error-7115a5b83125ed6548ac`
- `pocketstation::recording::writer::RecorderError` / `GapTooLarge` — `error-60461287a4b163819e59`
- `pocketstation::recording::writer::RecorderError` / `InvalidSampleSpec` — `error-4c6be945556fe930a69b`
- `pocketstation::recording::writer::RecorderError` / `InvalidStemLabel` — `error-59d9d4bcfbf0e21bf345`
- `pocketstation::recording::writer::RecorderError` / `Io` — `error-d60c0b0b91ebef260440`
- `pocketstation::recording::writer::RecorderError` / `Json` — `error-616039d9e7a8149973fa`
- `pocketstation::recording::writer::RecorderError` / `LineageMismatch` — `error-1f865658e279f391d815`
- `pocketstation::recording::writer::RecorderError` / `OutputExists` — `error-7b7a76cf6a542a0cb7eb`

## API reference

- [Multistem Recording](/docs/concepts/multistem-recording.md)
- [Recording](/docs/errors/recording.md)

| Public declaration | Kind | Declared purpose | Source |
|---|---|---|---|
| `pocketstation::recording::config::RecorderStemConfig` | struct | Configures recorder stem behavior at its owning API boundary. | `src/recording/config.rs:55` |
| `pocketstation::recording::config::StemLabel` | struct | Stores the validated human-readable label used for one recording stem. | `src/recording/config.rs:20` |
| `pocketstation::recording::endpoint::MultistemRecordingReceipt` | struct | Retains the identity and observation access returned for multistem recording. | `src/recording/endpoint.rs:28` |
| `pocketstation::recording::endpoint::SessionMultistemEndpointCoordinator` | struct | Canonical Session-owned multistem recorder declaration. | `src/recording/endpoint.rs:49` |
| `pocketstation::recording::writer::DiscontinuityRecord` | struct | Records one immutable discontinuity observation. | `src/recording/writer.rs:92` |
| `pocketstation::recording::writer::MultistemRecording` | struct | Owns the per-stem recording workers and coordinates their terminal finalization outcome. | `src/recording/writer.rs:138` |
| `pocketstation::recording::writer::RecordingObservations` | struct | Reports the recording observations collected at an observation boundary. | `src/recording/writer.rs:130` |
| `pocketstation::recording::writer::RecordingOutcome` | struct | Reports the structured recording outcome. | `src/recording/writer.rs:111` |

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

The claims on **Inspect recording outcomes** are anchored to Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `src/recording/writer.rs:1-1248` (`DIRECT`)
- `src/session/extensions/recording.rs:1-121` (`DIRECT`)

For **Inspect recording outcomes**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.
