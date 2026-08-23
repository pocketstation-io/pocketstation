# No microphone audio arrives

<!-- claims: CLM-TRBL-003-CAP-001,CLM-TRBL-003-CAP-002,CLM-TRBL-003-CAP-003,CLM-TRBL-003-CAP-004,CLM-TRBL-003-SOURCE-001 -->

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

- `pocketstation::capture::authorization::CaptureError` / `NotSupported` (`error-0f2fd6c6275925740175`)
- `pocketstation::capture::authorization::CaptureError` (`error-7905cc933b9eb45fe4ef`)
- `pocketstation::capture::authorization::CaptureError` / `BackendInit` (`error-ffea5e00d982c5213eba`)
- `pocketstation::capture::authorization::CaptureError` / `BackendSetupRequired` (`error-6e8f9f8ca8efa76ded69`)
- `pocketstation::capture::authorization::CaptureError` / `BackendStatus` (`error-533b29bac30886d8c79c`)
- `pocketstation::capture::authorization::CaptureError` / `CaptureWorkerPanicked` (`error-01c4b3cce2fa1669ee13`)
- `pocketstation::capture::authorization::CaptureError` / `InvalidRuntimeEventCapacity` (`error-c683702117e27ad45f33`)
- `pocketstation::capture::authorization::CaptureError` / `InvalidStreamCapacity` (`error-6167103023ec8fded812`)
- `pocketstation::capture::authorization::CaptureError` / `ModeUnsupported` (`error-786199dd7e94542436f2`)
- `pocketstation::capture::authorization::CaptureError` / `PermissionDenied` (`error-d902cf4c11a93cbcb084`)
- `pocketstation::capture::authorization::CaptureError` / `SourceUnavailable` (`error-61051d668a17eec6c3ac`)
- `pocketstation::session::compile::error::SessionCompileError` / `AudioBridgeOutputNotExclusive` (`error-756808af75b7a76405bd`)

## Executable evidence

- `given_sender_callback_when_frame_arrives_then_stream_receives_it` exercises given sender callback when frame arrives then stream receives it under its recorded setup (`test-136e5806a4cea77a2f5a`).
- `given_capture_mode_when_channels_selected_then_microphone_is_mono_and_output_is_stereo` exercises given capture mode when channels selected then microphone is mono and output is stereo under its recorded setup (`test-c28f1242d8a2b60457db`).
- `given_core_audio_permission_status_when_mapped_then_denial_remains_typed` exercises given core audio permission status when mapped then denial remains typed under its recorded setup (`test-afb8b00e7e6a55f6d16d`).
- `given_other_core_audio_status_when_mapped_then_raw_status_is_preserved` exercises given other core audio status when mapped then raw status is preserved under its recorded setup (`test-02f8243c416d85f0ef0c`).
- `given_default_and_exact_microphones_when_contract_inspected_then_lifetimes_differ` exercises given default and exact microphones when contract inspected then lifetimes differ under its recorded setup (`test-4af1790482adc3c295e6`).
- `given_exact_microphone_after_open_when_authorization_snapshotted_then_device_uid_is_retained` exercises given exact microphone after open when authorization snapshotted then device uid is retained under its recorded setup (`test-ee0390baad0b6dfd081d`).
- `given_text_operator_sent_to_audio_endpoint_when_compiled_then_signal_mismatch_is_typed` exercises given text operator sent to audio endpoint when compiled then signal mismatch is typed under its recorded setup (`test-3c9cfc84af12d388d72a`).
- `given_polled_audio_failures_when_mapped_then_every_status_is_preserved` exercises given polled audio failures when mapped then every status is preserved under its recorded setup (`test-36b002bb02a82e639fa5`).
- `given_audio_endpoint_extension_when_requested_then_definition_is_not_boot_registered` exercises given audio endpoint extension when requested then definition is not boot registered under its recorded setup (`test-85b9176c8aff351621b9`).
- `given_builtin_microphone_when_compiled_after_extension_then_existing_stem_path_is_unchanged` exercises given builtin microphone when compiled after extension then existing stem path is unchanged under its recorded setup (`test-4ee45ca045d2ee40a2a5`).
- `given_external_pcm_output_when_compiled_then_bounded_audio_edge_is_planned` exercises given external pcm output when compiled then bounded audio edge is planned under its recorded setup (`test-477958c0b22fe8487982`).
- `given_external_pcm_source_when_session_runs_then_audio_uses_bounded_ingress_with_source_identity` exercises given external pcm source when session runs then audio uses bounded ingress with source identity under its recorded setup (`test-1d9f4de1e64929bbc714`).
- `given_host_owned_backends_when_started_then_polled_audio_and_stop_are_real` exercises given host owned backends when started then polled audio and stop are real under its recorded setup (`test-1e7766766c491d2b7101`).
- `given_public_session_pcm_output_when_reentered_then_audio_lane_and_lifecycle_are_observed` exercises given public session pcm output when reentered then audio lane and lifecycle are observed under its recorded setup (`test-ae4fbbb0bfe0bcb0aff0`).
- `given_application_owned_audio_when_written_through_facade_then_session_delivers_its_lineage` exercises given application owned audio when written through facade then session delivers its lineage under its recorded setup (`test-fdcedcc753e41fe3767e`).

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

The claims on **No microphone audio arrives** are anchored to Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `src/capture/authorization.rs:1-318` (`DIRECT`)
- `src/capture/observations.rs:1-130` (`DIRECT`)

For **No microphone audio arrives**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.
