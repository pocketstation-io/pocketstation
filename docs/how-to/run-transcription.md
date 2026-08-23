# Run the transcription example

<!-- claims: CLM-GUIDE-026-CAP-001,CLM-GUIDE-026-CAP-002,CLM-GUIDE-026-CAP-003,CLM-GUIDE-026-SOURCE-001 -->

## Scope

- **Integrate transcription processing.** Use the repository-owned Whisper example to send captured stems to an external transcription process with evidence output.
- **Capture application audio.** Prepare application-scoped capture through the platform backend selected for the current target.
- **Capture microphone audio.** Select the default or identified input device and open native microphone capture.

The scope of **Run the transcription example** ends at the native contracts and executable conditions cited below. Platform qualification, performance, retry, and delivery require their own explicit evidence.

## Prerequisites

The nested example's Rust and external-process prerequisites, capture access, and a location for process evidence.

## Procedure

1. Read the nested example prerequisites.
2. Build its Cargo manifest.
3. Choose required capture sources.
4. Run the external process integration.
5. Preserve and validate process evidence.

## Important consequence

Compilation alone does not prove an executable was found or a transcription request completed.

## Verify the outcome

The example package builds, the child process runs, and the evidence artifact matches this source revision.

Executable evidence selected for **Run the transcription example** is limited to each test's recorded setup and assertions:

- `given_discontinuity_change_inside_window_when_processed_then_window_is_rejected` — given discontinuity change inside window when processed then window is rejected (`examples/whisper-transcribe/src/lib.rs:1400`; `test-ecb60c6da5bff96b4580`).
- `given_hung_provider_when_deadline_expires_then_child_is_killed_and_reaped` — given hung provider when deadline expires then child is killed and reaped (`examples/whisper-transcribe/src/lib.rs:1108`; `test-d2c23e54192a869ee546`).
- `given_instance_timeout_when_manifest_resolves_then_deadline_matches_configuration` — given instance timeout when manifest resolves then deadline matches configuration (`examples/whisper-transcribe/src/lib.rs:1055`; `test-e3fecbbc626c7ca91545`).
- `given_lineaged_window_when_transcribed_then_derived_range_covers_every_frame` — given lineaged window when transcribed then derived range covers every frame (`examples/whisper-transcribe/src/lib.rs:1311`; `test-e2540be9a42100cc68c1`).
- `given_missing_binary_when_prepare_runs_then_connector_fails_closed` — given missing binary when prepare runs then connector fails closed (`examples/whisper-transcribe/src/lib.rs:1098`; `test-d05ebeb952bf0753b799`).
- `given_outer_cancellation_when_process_is_active_then_child_receipt_is_finalized` — given outer cancellation when process is active then child receipt is finalized (`examples/whisper-transcribe/src/lib.rs:1220`; `test-87f552f09cb152e83b10`).
- `given_permission_change_inside_window_when_processed_then_window_is_rejected` — given permission change inside window when processed then window is rejected (`examples/whisper-transcribe/src/lib.rs:1419`; `test-b8a974fb8cab9b036630`).
- `given_process_evidence_when_provider_succeeds_then_actual_invocation_is_persisted` — given process evidence when provider succeeds then actual invocation is persisted (`examples/whisper-transcribe/src/lib.rs:1129`; `test-461c6ec95bfefc8bb314`).
- `given_process_evidence_when_provider_times_out_then_kill_and_reap_are_persisted` — given process evidence when provider times out then kill and reap are persisted (`examples/whisper-transcribe/src/lib.rs:1180`; `test-96cab447b1d1ad9b61d9`).
- `given_source_change_inside_window_when_processed_then_window_is_rejected_and_reset` — given source change inside window when processed then window is rejected and reset (`examples/whisper-transcribe/src/lib.rs:1379`; `test-19a765a0dbacdd29aee0`).
- `given_two_complete_windows_when_finished_then_partials_and_single_final_cover_stream` — given two complete windows when finished then partials and single final cover stream (`examples/whisper-transcribe/src/lib.rs:1338`; `test-3ed49534bf02ce80cbcb`).
- `given_typed_audio_when_window_fills_then_partial_precedes_one_final_transcript` — given typed audio when window fills then partial precedes one final transcript (`examples/whisper-transcribe/src/lib.rs:1263`; `test-b79c368693e08eaa7d95`).

## Failure signals

No task-specific public error was resolved for run the transcription example; preserve the owning API's returned error.

## API reference

- [Transcription Integration](/docs/concepts/transcription-integration.md)
- [Transcription Evidence](/docs/troubleshooting/transcription-evidence.md)

| Public declaration | Kind | Declared purpose | Source |
|---|---|---|---|
| `query` | module | Control-plane source discovery queries used by the first-party CLI. | `src/capture/query.rs:1` |
| `pocketstation::capture::capture_owner::ActiveCaptureBackend` | trait | Native capture resources owned for exactly one active capture. | `src/capture/capture_owner.rs:100` |
| `pocketstation::capture::capture_owner::CallbackCaptureBackend` | trait | Platform-neutral prepare/open boundary for callback-oriented capture. | `src/capture/capture_owner.rs:83` |
| `pocketstation::capture::capture_owner::PreparedCaptureBackend` | trait | Backend state that has passed validation but has not started delivery. | `src/capture/capture_owner.rs:88` |
| `pocketstation::capture::query::SourceProvider` | trait | Implement this trait to provide source behavior to PocketStation; its methods define the preparation and runtime contract. | `src/capture/query.rs:48` |
| `pocketstation::capture::capture_owner::CaptureDelivery` | struct | Callback delivery endpoints transferred to a prepared native backend. | `src/capture/capture_owner.rs:73` |
| `pocketstation::capture::capture_owner::CaptureLineageSeed` | struct | Stable session and stem identity assigned before an exact source is opened. | `src/capture/capture_owner.rs:25` |
| `pocketstation::capture::capture_owner::CaptureObservationReceipt` | struct | Retains the identity and observation access returned for capture observation. | `src/capture/capture_owner.rs:167` |

## Related documentation

- [Glossary](/docs/glossary.md)
- [PocketStation](/README.md)
- [Behavior evidence index](/docs/reference/behavior-evidence.md)
- [Rust API reference](/docs/reference/rust-api.md)
- [Linux capture](/docs/platform/linux.md)
- [Platform backend boundary](/docs/internals/platform-backends.md)
- [Platform prerequisites](/docs/getting-started/platform-prerequisites.md)
- [Platform support and evidence](/docs/platform/compatibility.md)

## Evidence boundary

The claims on **Run the transcription example** are anchored to Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `examples/whisper-transcribe/README.md:1-21` (`DIRECT`)
- `examples/whisper-transcribe/src/main.rs:1-75` (`DIRECT`)

For **Run the transcription example**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.
