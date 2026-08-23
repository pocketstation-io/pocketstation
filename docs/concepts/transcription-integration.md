# Transcription integration boundary

<!-- claims: CLM-DOC-037-CAP-001,CLM-DOC-037-SOURCE-001 -->

## What it is

The Whisper example is a separate Cargo package that captures PocketStation stems and crosses into an external transcription process while recording process evidence.

## Why it exists

Transcription behavior depends on an executable and environment outside the core crate. The example makes that integration boundary visible instead of claiming the SDK itself performs transcription.

## Relationships

- PocketStation capture and polling supply audio stems.
- The example owns conversion and child-process invocation.
- A process-evidence artifact records what external work actually ran.

## Invariants and guarantees

- Example compilation proves API compatibility, not process availability.
- A transcript without matching process evidence is not accepted as verified execution.
- The external process contract does not expand Core's guarantees.

## When you encounter it

- **Transcribe captured stems** — Run the repository transcription example and preserve process evidence for its external boundary.

## Use it

- [Run the transcription example](/docs/how-to/run-transcription.md)
- [Transcription process evidence is missing](/docs/troubleshooting/transcription-evidence.md)

## Scope

- **Integrate transcription processing.** Use the repository-owned Whisper example to send captured stems to an external transcription process with evidence output.

The scope of **Transcription integration boundary** ends at the native contracts and executable conditions cited below. Platform qualification, performance, retry, and delivery require their own explicit evidence.

## Key API

No intentionally public Rust declaration is owned directly by **Transcription integration boundary**. Its contract is expressed by the linked repository, protocol, or qualification evidence instead.

## Executable evidence

Executable evidence selected for **Transcription integration boundary** is limited to each test's recorded setup and assertions:

- `given_discontinuity_change_inside_window_when_processed_then_window_is_rejected` — given discontinuity change inside window when processed then window is rejected (`examples/whisper-transcribe/src/lib.rs:1400`; `test-2a5f4a8f1e04f5b220c4`).
- `given_hung_provider_when_deadline_expires_then_child_is_killed_and_reaped` — given hung provider when deadline expires then child is killed and reaped (`examples/whisper-transcribe/src/lib.rs:1108`; `test-aa794f0809b00c2d3394`).
- `given_instance_timeout_when_manifest_resolves_then_deadline_matches_configuration` — given instance timeout when manifest resolves then deadline matches configuration (`examples/whisper-transcribe/src/lib.rs:1055`; `test-899bb5750fda98d0832b`).
- `given_lineaged_window_when_transcribed_then_derived_range_covers_every_frame` — given lineaged window when transcribed then derived range covers every frame (`examples/whisper-transcribe/src/lib.rs:1311`; `test-5978528a8ea570fad70d`).
- `given_missing_binary_when_prepare_runs_then_connector_fails_closed` — given missing binary when prepare runs then connector fails closed (`examples/whisper-transcribe/src/lib.rs:1098`; `test-bd5ed751c752083c7711`).
- `given_outer_cancellation_when_process_is_active_then_child_receipt_is_finalized` — given outer cancellation when process is active then child receipt is finalized (`examples/whisper-transcribe/src/lib.rs:1220`; `test-841a6b80171cfb0f55e8`).
- `given_permission_change_inside_window_when_processed_then_window_is_rejected` — given permission change inside window when processed then window is rejected (`examples/whisper-transcribe/src/lib.rs:1419`; `test-1601ba20883aee1ac630`).
- `given_process_evidence_when_provider_succeeds_then_actual_invocation_is_persisted` — given process evidence when provider succeeds then actual invocation is persisted (`examples/whisper-transcribe/src/lib.rs:1129`; `test-004f9f3662355f6c02cc`).
- `given_process_evidence_when_provider_times_out_then_kill_and_reap_are_persisted` — given process evidence when provider times out then kill and reap are persisted (`examples/whisper-transcribe/src/lib.rs:1180`; `test-2ed5fdd4ba19977c8dc9`).
- `given_source_change_inside_window_when_processed_then_window_is_rejected_and_reset` — given source change inside window when processed then window is rejected and reset (`examples/whisper-transcribe/src/lib.rs:1379`; `test-384d43cad3cb43576f09`).
- `given_two_complete_windows_when_finished_then_partials_and_single_final_cover_stream` — given two complete windows when finished then partials and single final cover stream (`examples/whisper-transcribe/src/lib.rs:1338`; `test-46ae6f451f003a166202`).
- `given_typed_audio_when_window_fills_then_partial_precedes_one_final_transcript` — given typed audio when window fills then partial precedes one final transcript (`examples/whisper-transcribe/src/lib.rs:1263`; `test-3d1cc0ecef9a89cf23ff`).

## Related documentation

- [Glossary](/docs/glossary.md)
- [PocketStation](/README.md)
- [Run the examples](/docs/getting-started/examples.md)
- [Run the transcription example](/docs/how-to/run-transcription.md)
- [Behavior evidence index](/docs/reference/behavior-evidence.md)
- [Rust API reference](/docs/reference/rust-api.md)
- [Transcription process evidence is missing](/docs/troubleshooting/transcription-evidence.md)

## Evidence boundary

The claims on **Transcription integration boundary** are anchored to Git snapshot `136e74888962558aa846d3143a19136a70936f45` and these primary files:

- `examples/whisper-transcribe/src/lib.rs:1-1440` (`DIRECT`)

For **Transcription integration boundary**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.
