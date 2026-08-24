# No microphone audio arrives

<!-- claims: CLM-TRBL-003-SCOPE-001,CLM-TRBL-003-TEXT-001,CLM-TRBL-003-TEXT-002,CLM-TRBL-003-TEXT-003,CLM-TRBL-003-TEXT-004,CLM-TRBL-003-TEXT-005,CLM-TRBL-003-TEXT-006,CLM-TRBL-003-SOURCE-001 -->

## Symptom

The Session is running, but the microphone route yields no frames.

## Evidenced causes

- The host has not granted input permission or the platform cannot observe it before open.
- The default input changed or the selected device is unavailable.
- Microphone preparation or opening failed.
- The microphone route is saturated or the wrong receipt is being polled.

## Distinguish the causes

Compare permission observation with the source-open result, then verify selected device identity and microphone route observations.

## Diagnostic signals

- `pocketstation::capture::authorization::CaptureError` / `NotSupported` (`error-fc10abae73bd96954b49`)
- `pocketstation::capture::authorization::CaptureError` (`error-8a6cfaf6313c49f3d002`)
- `pocketstation::capture::authorization::CaptureError` / `BackendInit` (`error-e16ac3af9c00b5a9e1ef`)
- `pocketstation::capture::authorization::CaptureError` / `BackendSetupRequired` (`error-49a3487734f77997ff1d`)
- `pocketstation::capture::authorization::CaptureError` / `BackendStatus` (`error-433a8f64b39d41fe58e4`)
- `pocketstation::capture::authorization::CaptureError` / `CaptureWorkerPanicked` (`error-6a1ddaf64fd582202ee9`)
- `pocketstation::capture::authorization::CaptureError` / `InvalidRuntimeEventCapacity` (`error-c838e8f36c42c18a2a83`)
- `pocketstation::capture::authorization::CaptureError` / `InvalidStreamCapacity` (`error-abbc7e6ad159c238bf74`)
- `pocketstation::capture::authorization::CaptureError` / `ModeUnsupported` (`error-4a58ec0f52d2f2ee5a44`)
- `pocketstation::capture::authorization::CaptureError` / `PermissionDenied` (`error-54d94f02abd4884ade73`)
- `pocketstation::capture::authorization::CaptureError` / `SourceUnavailable` (`error-fb207c871b52ba476b04`)
- `pocketstation::session::compile::error::SessionCompileError` / `AudioBridgeOutputNotExclusive` (`error-455e5bda7c2d08e2f47d`)

## Executable evidence

- `given_sender_callback_when_frame_arrives_then_stream_receives_it` exercises given sender callback when frame arrives then stream receives it under its recorded setup (`test-698bc05f28228eb21d82`).
- `given_capture_mode_when_channels_selected_then_microphone_is_mono_and_output_is_stereo` exercises given capture mode when channels selected then microphone is mono and output is stereo under its recorded setup (`test-df5c7fa69c2c79a8f2a1`).
- `given_core_audio_permission_status_when_mapped_then_denial_remains_typed` exercises given core audio permission status when mapped then denial remains typed under its recorded setup (`test-052dbf5299c7bb5e6456`).
- `given_other_core_audio_status_when_mapped_then_raw_status_is_preserved` exercises given other core audio status when mapped then raw status is preserved under its recorded setup (`test-08a6379bc20593cb04fd`).
- `given_default_and_exact_microphones_when_contract_inspected_then_lifetimes_differ` exercises given default and exact microphones when contract inspected then lifetimes differ under its recorded setup (`test-2ac8da4006715acff504`).
- `given_exact_microphone_after_open_when_authorization_snapshotted_then_device_uid_is_retained` exercises given exact microphone after open when authorization snapshotted then device uid is retained under its recorded setup (`test-17236e5de78c4babb327`).
- `given_text_operator_sent_to_audio_endpoint_when_compiled_then_signal_mismatch_is_typed` exercises given text operator sent to audio endpoint when compiled then signal mismatch is typed under its recorded setup (`test-76c3eb4c4fd13e959a1c`).
- `given_polled_audio_failures_when_mapped_then_every_status_is_preserved` exercises given polled audio failures when mapped then every status is preserved under its recorded setup (`test-d8f211a56e8b18b3cbd6`).
- `given_audio_endpoint_extension_when_requested_then_definition_is_not_boot_registered` exercises given audio endpoint extension when requested then definition is not boot registered under its recorded setup (`test-e96f8506522ac5a30e20`).
- `given_builtin_microphone_when_compiled_after_extension_then_existing_stem_path_is_unchanged` exercises given builtin microphone when compiled after extension then existing stem path is unchanged under its recorded setup (`test-bcc1f480b1bf9ae0efa7`).
- `given_external_pcm_output_when_compiled_then_bounded_audio_edge_is_planned` exercises given external pcm output when compiled then bounded audio edge is planned under its recorded setup (`test-72f08a54e97cf69789ac`).
- `given_external_pcm_source_when_session_runs_then_audio_uses_bounded_ingress_with_source_identity` exercises given external pcm source when session runs then audio uses bounded ingress with source identity under its recorded setup (`test-4d0f3e5a95ea9490a090`).
- `given_host_owned_backends_when_started_then_polled_audio_and_stop_are_real` exercises given host owned backends when started then polled audio and stop are real under its recorded setup (`test-1e1a4d3810caea030f74`).
- `given_public_session_pcm_output_when_reentered_then_audio_lane_and_lifecycle_are_observed` exercises given public session pcm output when reentered then audio lane and lifecycle are observed under its recorded setup (`test-ea5b06c730a73a1dc9ca`).
- `given_application_owned_audio_when_written_through_facade_then_session_delivers_its_lineage` exercises given application owned audio when written through facade then session delivers its lineage under its recorded setup (`test-f1139be85b9372ec989b`).

## Corrective action

Let the host own any permission prompt, select an available input, and rebuild the affected source route.

## Retry and incomplete state

Do not loop on `NotObservable`; retry opening only after a meaningful permission or device change. Other stems may still contain valid frames.

## Related reference

- [Microphone Capture](/docs/concepts/microphone-capture.md)
- [Permissions](/docs/platform/permissions.md)

## Related documentation

- [Glossary](/docs/glossary.md)
- [PocketStation](/README.md)
- [Behavior evidence index](/docs/reference/behavior-evidence.md)
- [Rust API reference](/docs/reference/rust-api.md)
- [Capture failures](/docs/errors/capture.md)
- [No application audio arrives](/docs/troubleshooting/no-application-audio.md)
- [Permission state is denied or unobservable](/docs/troubleshooting/permission-state.md)
- [Linux capture](/docs/platform/linux.md)

## Evidence boundary

The claims on **No microphone audio arrives** are anchored to Git snapshot `136e74888962558aa846d3143a19136a70936f45` and these primary files:

- `src/capture/authorization.rs:1-1` (`DECLARED`)
- `src/capture/observations.rs:1-1` (`DECLARED`)

For **No microphone audio arrives**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.
