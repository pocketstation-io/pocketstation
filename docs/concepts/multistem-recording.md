# Multistem recording

<!-- claims: CLM-DOC-019-CAP-001,CLM-DOC-019-SOURCE-001 -->

## What it is

Multistem recording writes independent labeled source stems under one Session recording root and reports per-stem finalization after stop.

## Why it exists

Keeping application and microphone output separate preserves source identity and makes partial file failure visible instead of hiding it behind one mixed recording result.

## Relationships

- Recording is declared on source stems before start.
- Frame lineage and timeline mapping align recorded output.
- The recording outcome is available after endpoint finalization during Session stop.

## Invariants and guarantees

- Each recorded stem has its own identity and outcome.
- A running Session does not imply that every output file finalized.
- Inspect completed and failed stems before treating the recording as successful.

## When you encounter it

- **Record separate stems** — Record independent source stems and inspect finalization outcomes after Session stop.

## Use it

- [Record independent stems](/docs/how-to/record-stems.md)
- [Inspect recording outcomes](/docs/how-to/inspect-recording-outcome.md)
- [A recording is incomplete](/docs/troubleshooting/recording-incomplete.md)

## Scope

- **Record aligned multistem output.** Configure stems and finalize per-stem outcomes through the recording endpoint lifecycle.

The scope of **Multistem recording** ends at the native contracts and executable conditions cited below. Platform qualification, performance, retry, and delivery require their own explicit evidence.

## Key API

| Public declaration | Kind | Declared purpose | Source |
|---|---|---|---|
| `pocketstation::recording::endpoint::MultistemRecordingReceipt` | struct | Retains the identity and observation access returned for multistem recording. | `src/recording/endpoint.rs:28` |
| `pocketstation::recording::endpoint::SessionMultistemEndpointCoordinator` | struct | Canonical Session-owned multistem recorder declaration. | `src/recording/endpoint.rs:49` |
| `pocketstation::recording::writer::MultistemRecording` | struct | Owns the per-stem recording workers and coordinates their terminal finalization outcome. | `src/recording/writer.rs:138` |
| `pocketstation::recording::endpoint::MULTISTEM_GROUP_CONFIGURATION_KEY` | constant | Defines the public multistem group configuration key value. | `src/recording/endpoint.rs:24` |
| `pocketstation::recording::endpoint::MULTISTEM_NAME_CONFIGURATION_KEY` | constant | Defines the public multistem name configuration key value. | `src/recording/endpoint.rs:25` |
| `pocketstation::recording::config::RecorderStemConfig` | struct | Configures recorder stem behavior at its owning API boundary. | `src/recording/config.rs:55` |
| `pocketstation::recording::config::StemLabel` | struct | Stores the validated human-readable label used for one recording stem. | `src/recording/config.rs:20` |
| `pocketstation::recording::writer::DiscontinuityRecord` | struct | Records one immutable discontinuity observation. | `src/recording/writer.rs:92` |
| `pocketstation::recording::writer::RecordingObservations` | struct | Reports the recording observations collected at an observation boundary. | `src/recording/writer.rs:130` |
| `pocketstation::recording::writer::RecordingOutcome` | struct | Reports the structured recording outcome. | `src/recording/writer.rs:111` |

## Executable evidence

Executable evidence selected for **Multistem recording** is limited to each test's recorded setup and assertions:

- `given_derived_permission_epoch_when_later_frame_changes_it_then_recording_fails_closed` — given derived permission epoch when later frame changes it then recording fails closed (`src/recording/endpoint/tests.rs:287`; `test-5c3ff5e741df683ae4d8`).
- `given_recording_codes_when_serialized_then_values_are_exact_and_unique` — given recording codes when serialized then values are exact and unique (`src/recording/error_code.rs:95`; `test-bb7e1e95ee2acd51bc20`).
- `given_queued_audio_when_recording_cancelled_then_wav_header_is_playable_and_manifest_incomplete` — given queued audio when recording cancelled then wav header is playable and manifest incomplete (`src/recording/writer/tests.rs:215`; `test-68bd764f7d45a4b8fbe7`).
- `given_registered_multistem_recorder_when_host_built_then_receipt_is_retained` — given registered multistem recorder when host built then receipt is retained (`src/session/lifecycle/host.rs:702`; `test-8415e09aa158a8386c8e`).
- `given_session_context_and_two_first_frames_when_recorded_then_manifest_derives_capture_lineage_and_common_origin` — given session context and two first frames when recorded then manifest derives capture lineage and common origin (`src/recording/endpoint/tests.rs:187`; `test-9352f7a742c1f649857a`).
- `given_session_recorder_input_without_audio_stem_origin_when_prepared_then_it_is_rejected` — given session recorder input without audio stem origin when prepared then it is rejected (`src/recording/endpoint/tests.rs:262`; `test-497452363244c581f9e6`).
- `given_terminal_failure_when_projected_then_code_is_typed` — given terminal failure when projected then code is typed (`src/recording/error_code.rs:158`; `test-0a50247a3c74a66f107d`).
- `given_failed_recorder_branch_when_more_frames_dispatched_then_healthy_branch_continues` — given failed recorder branch when more frames dispatched then healthy branch continues (`src/recording/writer/tests.rs:246`; `test-a11ec53516f0e2c9bed1`).
- `given_fractional_stereo_gap_when_silence_is_sized_then_channels_remain_aligned` — given fractional stereo gap when silence is sized then channels remain aligned (`src/recording/writer/tests.rs:99`; `test-19f291f86bfba30549dc`).
- `given_timestamp_and_sequence_gap_when_finished_then_silence_and_events_preserve_time` — given timestamp and sequence gap when finished then silence and events preserve time (`src/recording/writer/tests.rs:169`; `test-186865304c35cff29002`).
- `given_two_clock_mapped_stems_when_finished_then_two_aligned_playable_wavs_are_written` — given two clock mapped stems when finished then two aligned playable wavs are written (`src/recording/writer/tests.rs:107`; `test-8457eec8d2b39e34dee4`).
- `given_compiled_derived_route_when_runtime_prepared_then_compiled_topology_is_preserved` — given compiled derived route when runtime prepared then compiled topology is preserved (`src/session/compile/tests.rs:659`; `test-f38493cc0593f603aece`).

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

The claims on **Multistem recording** are anchored to Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `src/recording/endpoint.rs:1-740` (`DIRECT`)
- `src/recording/writer.rs:1-1248` (`DIRECT`)

For **Multistem recording**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.
