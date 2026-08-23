# Transcription process evidence is missing

<!-- claims: CLM-TRBL-014-CAP-001,CLM-TRBL-014-CAP-002,CLM-TRBL-014-SOURCE-001 -->

Use this page when you observe **transcription process evidence is missing**. Diagnose the reported stage and identity before changing route, source, connector, or lifecycle policy.

## Distinguish the cause

Verify that the child process ran, its output was captured, and the evidence artifact corresponds to this source revision.

## Diagnostic signals

No domain-specific error variant is assigned. Use the stable error-code index and terminal outcome.

## Executable evidence

- `given_process_evidence_when_provider_succeeds_then_actual_invocation_is_persisted` exercises given process evidence when provider succeeds then actual invocation is persisted under its recorded setup (`test-461c6ec95bfefc8bb314`).
- `given_process_evidence_when_provider_times_out_then_kill_and_reap_are_persisted` exercises given process evidence when provider times out then kill and reap are persisted under its recorded setup (`test-96cab447b1d1ad9b61d9`).
- `given_discontinuity_change_inside_window_when_processed_then_window_is_rejected` exercises given discontinuity change inside window when processed then window is rejected under its recorded setup (`test-ecb60c6da5bff96b4580`).
- `given_missing_binary_when_prepare_runs_then_connector_fails_closed` exercises given missing binary when prepare runs then connector fails closed under its recorded setup (`test-d05ebeb952bf0753b799`).
- `given_outer_cancellation_when_process_is_active_then_child_receipt_is_finalized` exercises given outer cancellation when process is active then child receipt is finalized under its recorded setup (`test-87f552f09cb152e83b10`).
- `given_permission_change_inside_window_when_processed_then_window_is_rejected` exercises given permission change inside window when processed then window is rejected under its recorded setup (`test-b8a974fb8cab9b036630`).
- `given_source_change_inside_window_when_processed_then_window_is_rejected_and_reset` exercises given source change inside window when processed then window is rejected and reset under its recorded setup (`test-19a765a0dbacdd29aee0`).
- `given_hung_provider_when_deadline_expires_then_child_is_killed_and_reaped` exercises given hung provider when deadline expires then child is killed and reaped under its recorded setup (`test-d2c23e54192a869ee546`).
- `given_instance_timeout_when_manifest_resolves_then_deadline_matches_configuration` exercises given instance timeout when manifest resolves then deadline matches configuration under its recorded setup (`test-e3fecbbc626c7ca91545`).
- `given_lineaged_window_when_transcribed_then_derived_range_covers_every_frame` exercises given lineaged window when transcribed then derived range covers every frame under its recorded setup (`test-e2540be9a42100cc68c1`).
- `given_two_complete_windows_when_finished_then_partials_and_single_final_cover_stream` exercises given two complete windows when finished then partials and single final cover stream under its recorded setup (`test-3ed49534bf02ce80cbcb`).
- `given_typed_audio_when_window_fills_then_partial_precedes_one_final_transcript` exercises given typed audio when window fills then partial precedes one final transcript under its recorded setup (`test-b79c368693e08eaa7d95`).
- `given_wav_envelope_when_connector_runs_then_text_lineage_is_preserved` exercises given wav envelope when connector runs then text lineage is preserved under its recorded setup (`test-21347646a3ff78175454`).

## Corrective action and retry

Apply only the action implied by the typed failure or violated precondition. Retry is not safe merely because a failure appears transient. When retryability or recovery is unknown, preserve the failure for application policy or maintainer review.

## Data and state

Treat frames, signals, files, acknowledgements, and finalization results produced before failure as potentially partial unless the terminal contract says otherwise. Inspect per-route, per-stem, and per-component outcomes.

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

This page was verified against Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `examples/whisper-transcribe/src/process_evidence.rs:1-211` (`DIRECT`)

A file's presence proves implementation or declaration at this snapshot. It does not by itself prove physical-device qualification, operational performance, retry safety, or behavior outside the recorded test conditions.
