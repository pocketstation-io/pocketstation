# Run the transcription example

<!-- claims: CLM-GUIDE-026-CAP-001,CLM-GUIDE-026-CAP-002,CLM-GUIDE-026-CAP-003,CLM-GUIDE-026-SOURCE-001 -->

## Scope

- **Integrate transcription processing.** Use the repository-owned Whisper example to send captured stems to an external transcription process with evidence output.
- **Capture application audio.** Prepare application-scoped capture through the platform backend selected for the current target.
- **Capture microphone audio.** Select the default or identified input device and open native microphone capture.

These statements describe repository contracts at the documented snapshot. They do not extend platform qualification, performance, retry, or delivery guarantees beyond the native API contracts and executable evidence.

## Prerequisites

Read the linked concept and confirm that target platform, Cargo features, source or provider dependencies, and application-owned permission work match this task. Keep returned typed errors and outcomes available for verification.

## Procedure

1. Read the nested example prerequisites.
2. Build its Cargo manifest.
3. Choose required capture sources.
4. Run the external process integration.
5. Preserve and validate process evidence.

## APIs used

| Public declaration | Kind | Declared purpose | Source |
|---|---|---|---|
| `authorization` | module | Explicit capture authorization evidence and open outcomes. | `src/capture/authorization.rs:1` |
| `identity` | module | Stable source identity and source discovery state. | `src/capture/identity.rs:1` |
| `query` | module | Control-plane source discovery queries used by the first-party CLI. | `src/capture/query.rs:1` |
| `selection` | module | Capture selection semantics; control-plane only. | `src/capture/selection.rs:1` |
| `pocketstation::capture::capture_owner::ActiveCaptureBackend` | trait | Native capture resources owned for exactly one active capture. | `src/capture/capture_owner.rs:100` |
| `pocketstation::capture::capture_owner::CallbackCaptureBackend` | trait | Platform-neutral prepare/open boundary for callback-oriented capture. | `src/capture/capture_owner.rs:83` |
| `pocketstation::capture::capture_owner::PreparedCaptureBackend` | trait | Backend state that has passed validation but has not started delivery. | `src/capture/capture_owner.rs:88` |
| `pocketstation::capture::authorization::CaptureAuthorizationSnapshot` | struct | Point-in-time authorization evidence for opening one exact capture source. | `src/capture/authorization.rs:17` |
| `pocketstation::capture::authorization::CapturePermissionLifecycle` | struct | Control-plane owner for one source's observed authorization epoch. | `src/capture/authorization.rs:183` |
| `pocketstation::capture::authorization::CapturePermissionTransition` | struct | One authoritative authorization-state transition observed by the host. | `src/capture/authorization.rs:168` |
| `pocketstation::capture::capture_owner::CaptureDelivery` | struct | Callback delivery endpoints transferred to a prepared native backend. | `src/capture/capture_owner.rs:73` |
| `pocketstation::capture::capture_owner::CaptureOwnerObservations` | struct | Aggregate observations from one active capture ownership boundary. | `src/capture/capture_owner.rs:160` |
| `pocketstation::capture::frame_stream::CapturedFrameSender` | struct | Single-producer endpoint passed into a platform capture callback. | `src/capture/frame_stream.rs:102` |
| `ActiveCaptureBackend::source_id` | function | Resolved native source identity for every frame emitted by this open. | `src/capture/capture_owner.rs:105` |
| `after_failed_open` | function | Records a backend open failure without guessing that permission was denied. | `src/capture/authorization.rs:45` |
| `after_successful_open` | function | Records the evidence available after an explicitly selected source opens. | `src/capture/authorization.rs:31` |

## Verify the outcome

The following test bodies are evidence only for their recorded setup:

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

- `pocketstation::capture::events::CaptureRuntimeFailure` — `error-11b972ad42d5de880e06`
- `pocketstation::capture::events::CaptureRuntimeFailureClass` / `BackendClass` — `error-29e952ae7432566a9e95`
- `pocketstation::capture::authorization::CaptureError` / `CaptureWorkerPanicked` — `error-365f9b6fbda74eb0d631`
- `pocketstation::capture::authorization::CaptureError` / `PermissionDenied` — `error-38030156125346a8e892`
- `pocketstation::capture::authorization::CaptureError` / `NotSupported` — `error-3b4b5393164d9f6f12a5`
- `pocketstation::capture::events::CaptureRuntimeFailureClass` / `PlatformStatus` — `error-3c6fcc22deb2f54788ba`
- `pocketstation::capture::authorization::CaptureError` / `SourceUnavailable` — `error-71c87f975acc9e22a402`
- `pocketstation::capture::authorization::CaptureError` / `BackendSetupRequired` — `error-8db0fec69a9c7158ffdf`
- `pocketstation::capture::authorization::CaptureError` — `error-96ffe4bc4254583d1e17`
- `pocketstation::capture::events::CaptureRuntimeFailureClass` / `SourceInstanceExited` — `error-a9c0f7dfff744e9ba6b7`
- `pocketstation::capture::authorization::CaptureError` / `BackendInit` — `error-b320ea1cba2b3c8dc4c7`
- `pocketstation::capture::authorization::CaptureError` / `InvalidStreamCapacity` — `error-bcf5d4d897b6bd0784bf`

Retry only when the relevant API or error contract explicitly permits it. An error name, a transient-looking message, or a successful prior run is not retry evidence.

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

This page was verified against Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `examples/whisper-transcribe/README.md:1-21` (`DIRECT`)
- `examples/whisper-transcribe/src/main.rs:1-75` (`DIRECT`)

A file's presence proves implementation or declaration at this snapshot. It does not by itself prove physical-device qualification, operational performance, retry safety, or behavior outside the recorded test conditions.
