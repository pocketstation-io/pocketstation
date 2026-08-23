# No application audio arrives

<!-- claims: CLM-TRBL-002-CAP-001,CLM-TRBL-002-CAP-002,CLM-TRBL-002-CAP-003,CLM-TRBL-002-CAP-004,CLM-TRBL-002-CAP-005,CLM-TRBL-002-SOURCE-001 -->

## Symptom

The Session is running, but the application route yields no frames.

## Evidenced causes

- The selector resolved no candidate or a different process instance.
- Permission or source opening did not succeed for application capture.
- The source disappeared, changed generation, or emits no frames.
- The intended route is absent, saturated, or polled through the wrong receipt.

## Distinguish the causes

Check resolution identity, source-open outcome, source lifecycle events, per-route observations, and the receipt's delivered-frame count in that order.

## Diagnostic signals

- `pocketstation::capture::events::CaptureRuntimeFailure` (`error-ee187ecbd20c3485593b`)
- `pocketstation::capture::events::CaptureRuntimeFailureClass` (`error-00ac112f8cac9b2976c5`)
- `pocketstation::capture::events::CaptureRuntimeFailureClass` / `BackendClass` (`error-ed79157bc39ce0d41fad`)
- `pocketstation::capture::events::CaptureRuntimeFailureClass` / `PlatformStatus` (`error-ae0e4e8e83a2cdd49e9e`)
- `pocketstation::capture::events::CaptureRuntimeFailureClass` / `SourceInstanceExited` (`error-35fe109728e80f2b126f`)
- `pocketstation::capture::authorization::CaptureError` / `NotSupported` (`error-0f2fd6c6275925740175`)
- `pocketstation::session::compile::error::SessionCompileError` / `AudioBridgeOutputNotExclusive` (`error-756808af75b7a76405bd`)
- `pocketstation::session::compile::error::SessionCompileError` / `InvalidAudioBridgeOutput` (`error-1e1c426aed779be65563`)
- `pocketstation::session::declaration::typed_stream::TypedStreamError` / `StemRequiresPcmAudio` (`error-1ced17a870f5f8b8182c`)
- `pocketstation::session::error_code::PolledAudioPollErrorCode` (`error-b11ef8f68fdd53959f54`)
- `pocketstation::session::error_code::PolledAudioPollErrorCode` / `Empty` (`error-8b50846ed001d249b043`)
- `pocketstation::session::error_code::PolledAudioPollErrorCode` / `InternalStateUnavailable` (`error-e5c943170627849a800a`)

## Executable evidence

- `given_application_owned_audio_when_written_through_facade_then_session_delivers_its_lineage` exercises given application owned audio when written through facade then session delivers its lineage under its recorded setup (`test-fdcedcc753e41fe3767e`).
- `given_sender_callback_when_frame_arrives_then_stream_receives_it` exercises given sender callback when frame arrives then stream receives it under its recorded setup (`test-136e5806a4cea77a2f5a`).
- `given_application_mode_when_pipewire_unavailable_then_mode_unsupported_not_system_mix` exercises given application mode when pipewire unavailable then mode unsupported not system mix under its recorded setup (`test-a19b8c36cc500e40f220`).
- `given_exact_application_selector_when_identity_is_transient_then_selection_fails_closed` exercises given exact application selector when identity is transient then selection fails closed under its recorded setup (`test-1a09c0b9480a09c36429`).
- `given_exact_application_selector_when_multiple_nodes_match_then_selection_is_ambiguous` exercises given exact application selector when multiple nodes match then selection is ambiguous under its recorded setup (`test-69d4e0c97753aed54953`).
- `given_exact_application_selector_when_one_live_node_matches_then_current_target_is_selected` exercises given exact application selector when one live node matches then current target is selected under its recorded setup (`test-7477ad1c961dad51886d`).
- `given_exact_stable_application_when_pipewire_unavailable_then_mode_is_not_weakened` exercises given exact stable application when pipewire unavailable then mode is not weakened under its recorded setup (`test-b6715ab214572748c3d2`).
- `given_pipewire_application_metadata_when_identity_is_derived_then_persistent_fields_win` exercises given pipewire application metadata when identity is derived then persistent fields win under its recorded setup (`test-e2539685a9c80008839f`).
- `given_core_audio_permission_status_when_mapped_then_denial_remains_typed` exercises given core audio permission status when mapped then denial remains typed under its recorded setup (`test-afb8b00e7e6a55f6d16d`).
- `given_exact_application_target_when_framed_then_stable_identity_is_preserved` exercises given exact application target when framed then stable identity is preserved under its recorded setup (`test-55d1d56a08dae220e1d4`).
- `given_other_core_audio_status_when_mapped_then_raw_status_is_preserved` exercises given other core audio status when mapped then raw status is preserved under its recorded setup (`test-02f8243c416d85f0ef0c`).
- `given_reused_pid_with_different_application_when_verified_then_target_is_rejected` exercises given reused pid with different application when verified then target is rejected under its recorded setup (`test-6f4ba1dfb2d103e389b4`).
- `given_same_pid_and_application_when_verified_then_target_is_retained` exercises given same pid and application when verified then target is retained under its recorded setup (`test-51b07027d04f4b13be09`).
- `given_same_pid_and_application_with_new_creation_when_audited_then_reuse_is_detected` exercises given same pid and application with new creation when audited then reuse is detected under its recorded setup (`test-bb302ee91ce25914320b`).
- `given_exact_application_after_open_when_authorization_snapshotted_then_scope_stays_exact` exercises given exact application after open when authorization snapshotted then scope stays exact under its recorded setup (`test-ca7f511d83273d705367`).

## Corrective action

Re-resolve the intended application, correct permission or routing, and recreate only the component whose typed state is invalid.

## Retry and incomplete state

Opening or routing failures do not establish safe automatic retry. Frames already delivered on other stems remain independent and may be partial.

## Related reference

- [Application Capture](/docs/concepts/application-capture.md)
- [Capture](/docs/reference/capture.md)

## Related documentation

- [Glossary](/docs/glossary.md)
- [PocketStation](/README.md)
- [Behavior evidence index](/docs/reference/behavior-evidence.md)
- [Rust API reference](/docs/reference/rust-api.md)
- [Capture failures](/docs/errors/capture.md)
- [Capture a desktop application](/docs/how-to/capture-application.md)
- [Capture API](/docs/reference/capture.md)
- [A capture source disappears](/docs/troubleshooting/source-loss.md)

## Evidence boundary

The claims on **No application audio arrives** are anchored to Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `src/capture/events.rs:1-344` (`DIRECT`)
- `src/capture/observations.rs:1-130` (`DIRECT`)

For **No application audio arrives**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.
