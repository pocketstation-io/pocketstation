# No microphone audio arrives

<!-- claims: CLM-TRBL-003-CAP-001,CLM-TRBL-003-CAP-002,CLM-TRBL-003-CAP-003,CLM-TRBL-003-CAP-004,CLM-TRBL-003-SOURCE-001 -->

Use this page when you observe **no microphone audio arrives**. Diagnose the reported stage and identity before changing route, source, connector, or lifecycle policy.

## Distinguish the cause

Check non-prompting permission observation, the host-owned prompt result, selected input identity, and source-open outcome in that order.

## Diagnostic signals

- `pocketstation::session::declaration::typed_stream::TypedStreamError` / `OutputSignalMismatch` (`error-00e5716261eba0f8cf3d`)
- `pocketstation::session::error::SessionError` / `UnknownStem` (`error-00f6e798d158df66c847`)
- `pocketstation::session::error_code::SessionStartErrorCode` / `StartCancelled` (`error-01d3fc855e2a00319076`)
- `pocketstation::session::lifecycle::start_contract::SessionStartError` / `OperatorPrepare` (`error-023d6ab0b23a50a614ff`)
- `pocketstation::session::error_code::SessionStartErrorCode` / `TraceRecorderSetupFailed` (`error-0279b2b6b0cb3b5801bc`)
- `pocketstation::session::prepare::error::SessionPrepareError` / `MissingOperatorSignalInput` (`error-037ddc3e193da74177f8`)
- `pocketstation::session::lifecycle::trace::SessionTraceValidationError` / `InvalidLayout` (`error-05c60389efcb84311921`)
- `pocketstation::session::prepare::error::SessionPrepareError` (`error-085082b521c14e5ecd1e`)
- `pocketstation::session::extensions::audio_input::buffer::AudioInputWriteErrorKind` / `Closed` (`error-08a7536094bfb2242b17`)
- `pocketstation::session::lifecycle::host::SessionEngineHostBuildError` / `EndpointExtensionRegistration` (`error-09837185c7fca0f70618`)
- `pocketstation::session::lifecycle::start_contract::SessionStartError` / `MissingEndpointDeclaration` (`error-0bc2f7c0b9f9dbf8ddd7`)
- `pocketstation::session::lifecycle::trace::SessionTraceRecorderStartError` / `ZeroCapacity` (`error-0bd6f58be40ade9a01fe`)
- `pocketstation::session::lifecycle::trace::SessionTraceValidationError` / `SequenceGap` (`error-0c04a3eedb823da29323`)
- `pocketstation::session::prepare::error::SessionPrepareError` / `MissingExternalAudioIngress` (`error-0cc0ae8a8cc4f1e05996`)
- `pocketstation::session::lifecycle::engine::SessionEngineBuildError` / `DuplicateSidecarId` (`error-0ce1015c73b65576cbeb`)
- `pocketstation::session::lifecycle::trace::SessionTraceValidationError` / `TimestampRegression` (`error-0d567cf627daa0adfee1`)
- `pocketstation::session::error_code::SessionStartErrorCode` / `MissingEndpointDeclaration` (`error-0e46a3d13215bfc3898f`)
- `pocketstation::session::extensions::audio_input::AudioInputConfigError` (`error-108ece57ea443c789d81`)
- `pocketstation::session::extensions::audio_input::source::AudioInputError` / `Manifest` (`error-11863b3a293345b0bb2d`)
- `pocketstation::capture::events::CaptureRuntimeFailure` (`error-11b972ad42d5de880e06`)
- `pocketstation::session::compile::error::SessionCompileError` / `UnknownEndpointInputPort` (`error-1281b697f9f4d62194b1`)
- `pocketstation::session::prepare::error::SessionPrepareError` / `MissingTypedEdgePlan` (`error-12fef698a1fbec823e7e`)
- `pocketstation::session::prepare::error::SessionPrepareError` / `MissingAsyncOperatorFactory` (`error-1310461ef521d30d4686`)
- `pocketstation::session::error_code::SessionStartErrorCode` / `MissingEventReceiver` (`error-13dd584b4e2e8eaa490c`)
- `pocketstation::session::lifecycle::trace::SessionTraceValidationError` / `RecordAfterTerminal` (`error-16e269f1786471c2db63`)
- `pocketstation::session::error::SessionError` / `UnknownSourceOutput` (`error-16edb8f15b75c471db64`)
- `pocketstation::session::compile::error::SessionCompileError` / `AmbiguousEndpointInput` (`error-17674f66426c713d90a2`)
- `pocketstation::session::lifecycle::events::SessionRollbackFailure` (`error-1955a522796dc25c325d`)
- `pocketstation::session::declaration::typed_stream::TypedStreamError` / `MissingPort` (`error-1bd7ae7942029f778071`)
- `pocketstation::session::error_code::SessionStartErrorCode` (`error-1c7816652dd061fb1141`)

## Executable evidence

- `given_sender_callback_when_frame_arrives_then_stream_receives_it` exercises given sender callback when frame arrives then stream receives it under its recorded setup (`test-136e5806a4cea77a2f5a`).
- `given_capture_mode_when_channels_selected_then_microphone_is_mono_and_output_is_stereo` exercises given capture mode when channels selected then microphone is mono and output is stereo under its recorded setup (`test-c28f1242d8a2b60457db`).
- `given_core_audio_permission_status_when_mapped_then_denial_remains_typed` exercises given core audio permission status when mapped then denial remains typed under its recorded setup (`test-afb8b00e7e6a55f6d16d`).
- `given_other_core_audio_status_when_mapped_then_raw_status_is_preserved` exercises given other core audio status when mapped then raw status is preserved under its recorded setup (`test-02f8243c416d85f0ef0c`).
- `given_default_and_exact_microphones_when_contract_inspected_then_lifetimes_differ` exercises given default and exact microphones when contract inspected then lifetimes differ under its recorded setup (`test-4af1790482adc3c295e6`).
- `given_exact_microphone_after_open_when_authorization_snapshotted_then_device_uid_is_retained` exercises given exact microphone after open when authorization snapshotted then device uid is retained under its recorded setup (`test-ee0390baad0b6dfd081d`).
- `planned_audio_edge_count` exercises planned audio edge count under its recorded setup (`test-df0021f5a7204cec4764`).
- `given_text_operator_sent_to_audio_endpoint_when_compiled_then_signal_mismatch_is_typed` exercises given text operator sent to audio endpoint when compiled then signal mismatch is typed under its recorded setup (`test-3c9cfc84af12d388d72a`).
- `given_polled_audio_failures_when_mapped_then_every_status_is_preserved` exercises given polled audio failures when mapped then every status is preserved under its recorded setup (`test-36b002bb02a82e639fa5`).
- `given_audio_endpoint_extension_when_requested_then_definition_is_not_boot_registered` exercises given audio endpoint extension when requested then definition is not boot registered under its recorded setup (`test-85b9176c8aff351621b9`).
- `given_builtin_microphone_when_compiled_after_extension_then_existing_stem_path_is_unchanged` exercises given builtin microphone when compiled after extension then existing stem path is unchanged under its recorded setup (`test-4ee45ca045d2ee40a2a5`).
- `given_external_pcm_output_when_compiled_then_bounded_audio_edge_is_planned` exercises given external pcm output when compiled then bounded audio edge is planned under its recorded setup (`test-477958c0b22fe8487982`).
- `given_external_pcm_source_when_session_runs_then_audio_uses_bounded_ingress_with_source_identity` exercises given external pcm source when session runs then audio uses bounded ingress with source identity under its recorded setup (`test-1d9f4de1e64929bbc714`).
- `given_host_owned_backends_when_started_then_polled_audio_and_stop_are_real` exercises given host owned backends when started then polled audio and stop are real under its recorded setup (`test-1e7766766c491d2b7101`).
- `polled_audio_receipts_total` exercises polled audio receipts total under its recorded setup (`test-d0262b09f7e1a858e09e`).

## Corrective action and retry

Apply only the action implied by the typed failure or violated precondition. Retry is not safe merely because a failure appears transient. When retryability or recovery is unknown, preserve the failure for application policy or maintainer review.

## Data and state

Treat frames, signals, files, acknowledgements, and finalization results produced before failure as potentially partial unless the terminal contract says otherwise. Inspect per-route, per-stem, and per-component outcomes.

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

This page was verified against Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `src/capture/authorization.rs:1-318` (`DIRECT`)
- `src/capture/observations.rs:1-130` (`DIRECT`)

A file's presence proves implementation or declaration at this snapshot. It does not by itself prove physical-device qualification, operational performance, retry safety, or behavior outside the recorded test conditions.
