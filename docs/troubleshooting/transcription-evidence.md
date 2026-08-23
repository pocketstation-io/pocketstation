# Transcription process evidence is missing

<!-- claims: CLM-TRBL-014-CAP-001,CLM-TRBL-014-CAP-002,CLM-TRBL-014-SOURCE-001 -->

## Symptom

The transcription example produces no process-evidence artifact or the artifact does not match the run.

## Evidenced causes

- The external executable did not start or could not be found.
- Captured input never reached the process step.
- The child output was not persisted or belongs to another source revision.

## Distinguish the causes

Verify child start, exit status, captured output, evidence path, and recorded source revision independently from example compilation.

## Diagnostic signals

No error declaration is tied directly to transcription process evidence is missing; use the owning component's typed outcome and observations.

## Executable evidence

- `given_process_evidence_when_provider_succeeds_then_actual_invocation_is_persisted` exercises given process evidence when provider succeeds then actual invocation is persisted under its recorded setup (`test-004f9f3662355f6c02cc`).
- `given_process_evidence_when_provider_times_out_then_kill_and_reap_are_persisted` exercises given process evidence when provider times out then kill and reap are persisted under its recorded setup (`test-2ed5fdd4ba19977c8dc9`).
- `given_discontinuity_change_inside_window_when_processed_then_window_is_rejected` exercises given discontinuity change inside window when processed then window is rejected under its recorded setup (`test-2a5f4a8f1e04f5b220c4`).
- `given_missing_binary_when_prepare_runs_then_connector_fails_closed` exercises given missing binary when prepare runs then connector fails closed under its recorded setup (`test-bd5ed751c752083c7711`).
- `given_outer_cancellation_when_process_is_active_then_child_receipt_is_finalized` exercises given outer cancellation when process is active then child receipt is finalized under its recorded setup (`test-841a6b80171cfb0f55e8`).
- `given_permission_change_inside_window_when_processed_then_window_is_rejected` exercises given permission change inside window when processed then window is rejected under its recorded setup (`test-1601ba20883aee1ac630`).
- `given_source_change_inside_window_when_processed_then_window_is_rejected_and_reset` exercises given source change inside window when processed then window is rejected and reset under its recorded setup (`test-384d43cad3cb43576f09`).
- `given_hung_provider_when_deadline_expires_then_child_is_killed_and_reaped` exercises given hung provider when deadline expires then child is killed and reaped under its recorded setup (`test-aa794f0809b00c2d3394`).
- `given_instance_timeout_when_manifest_resolves_then_deadline_matches_configuration` exercises given instance timeout when manifest resolves then deadline matches configuration under its recorded setup (`test-899bb5750fda98d0832b`).
- `given_lineaged_window_when_transcribed_then_derived_range_covers_every_frame` exercises given lineaged window when transcribed then derived range covers every frame under its recorded setup (`test-5978528a8ea570fad70d`).
- `given_two_complete_windows_when_finished_then_partials_and_single_final_cover_stream` exercises given two complete windows when finished then partials and single final cover stream under its recorded setup (`test-46ae6f451f003a166202`).
- `given_typed_audio_when_window_fills_then_partial_precedes_one_final_transcript` exercises given typed audio when window fills then partial precedes one final transcript under its recorded setup (`test-3d1cc0ecef9a89cf23ff`).
- `given_wav_envelope_when_connector_runs_then_text_lineage_is_preserved` exercises given wav envelope when connector runs then text lineage is preserved under its recorded setup (`test-7a78788b7f5737646740`).

## Corrective action

Correct the external process prerequisite and rerun the example so it writes a fresh matching artifact.

## Retry and incomplete state

Do not treat a compile-only rerun as execution evidence. Capture or process output from an interrupted run can be incomplete.

## Related reference

- [Transcription Integration](/docs/concepts/transcription-integration.md)
- [Run Transcription](/docs/how-to/run-transcription.md)

## Related documentation

- [Glossary](/docs/glossary.md)
- [PocketStation](/README.md)
- [Behavior evidence index](/docs/reference/behavior-evidence.md)
- [Rust API reference](/docs/reference/rust-api.md)
- [Error and status model](/docs/concepts/error-model.md)
- [Run the examples](/docs/getting-started/examples.md)
- [Terminal outcomes](/docs/lifecycle/terminal-outcomes.md)
- [Transcription integration boundary](/docs/concepts/transcription-integration.md)

## Evidence boundary

The claims on **Transcription process evidence is missing** are anchored to Git snapshot `136e74888962558aa846d3143a19136a70936f45` and these primary files:

- `examples/whisper-transcribe/src/process_evidence.rs:1-211` (`DIRECT`)

For **Transcription process evidence is missing**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.
