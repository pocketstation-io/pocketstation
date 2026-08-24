# No application audio arrives

<!-- claims: CLM-TRBL-002-SCOPE-001,CLM-TRBL-002-TEXT-001,CLM-TRBL-002-TEXT-002,CLM-TRBL-002-TEXT-003,CLM-TRBL-002-TEXT-004,CLM-TRBL-002-TEXT-005,CLM-TRBL-002-TEXT-006,CLM-TRBL-002-SOURCE-001 -->

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

- `pocketstation::capture::events::CaptureRuntimeFailure` (`error-27b8f339dd8b80dbf899`)
- `pocketstation::capture::events::CaptureRuntimeFailureClass` (`error-3169b6fff966018c5c73`)
- `pocketstation::capture::events::CaptureRuntimeFailureClass` / `BackendClass` (`error-29a5a1b804166e5e41ea`)
- `pocketstation::capture::events::CaptureRuntimeFailureClass` / `PlatformStatus` (`error-476e2ddf8c1ad7099081`)
- `pocketstation::capture::events::CaptureRuntimeFailureClass` / `SourceInstanceExited` (`error-677a63665bbdf8a0715a`)
- `pocketstation::capture::authorization::CaptureError` / `NotSupported` (`error-fc10abae73bd96954b49`)
- `pocketstation::session::compile::error::SessionCompileError` / `AudioBridgeOutputNotExclusive` (`error-455e5bda7c2d08e2f47d`)
- `pocketstation::session::compile::error::SessionCompileError` / `InvalidAudioBridgeOutput` (`error-05658b1653627f20c5c4`)
- `pocketstation::session::declaration::typed_stream::TypedStreamError` / `StemRequiresPcmAudio` (`error-bce02a098e0e662db3ff`)
- `pocketstation::session::error_code::PolledAudioPollErrorCode` (`error-85c6a49bf2630a26eca9`)
- `pocketstation::session::error_code::PolledAudioPollErrorCode` / `Empty` (`error-fd0478f7ceb6192b049b`)
- `pocketstation::session::error_code::PolledAudioPollErrorCode` / `InternalStateUnavailable` (`error-d97a81cbcae2ff4fae22`)

## Executable evidence

- `given_application_owned_audio_when_written_through_facade_then_session_delivers_its_lineage` exercises given application owned audio when written through facade then session delivers its lineage under its recorded setup (`test-f1139be85b9372ec989b`).
- `given_sender_callback_when_frame_arrives_then_stream_receives_it` exercises given sender callback when frame arrives then stream receives it under its recorded setup (`test-698bc05f28228eb21d82`).
- `given_application_mode_when_pipewire_unavailable_then_mode_unsupported_not_system_mix` exercises given application mode when pipewire unavailable then mode unsupported not system mix under its recorded setup (`test-3935b20953f69bd82dab`).
- `given_exact_application_selector_when_identity_is_transient_then_selection_fails_closed` exercises given exact application selector when identity is transient then selection fails closed under its recorded setup (`test-1e40dd4ec9e96cd35eb7`).
- `given_exact_application_selector_when_multiple_nodes_match_then_selection_is_ambiguous` exercises given exact application selector when multiple nodes match then selection is ambiguous under its recorded setup (`test-2843e96f914d98065a94`).
- `given_exact_application_selector_when_one_live_node_matches_then_current_target_is_selected` exercises given exact application selector when one live node matches then current target is selected under its recorded setup (`test-15388b47d24aa21999f6`).
- `given_exact_stable_application_when_pipewire_unavailable_then_mode_is_not_weakened` exercises given exact stable application when pipewire unavailable then mode is not weakened under its recorded setup (`test-51cbb8d765eada41b0c9`).
- `given_pipewire_application_metadata_when_identity_is_derived_then_persistent_fields_win` exercises given pipewire application metadata when identity is derived then persistent fields win under its recorded setup (`test-964084de3faa5b449071`).
- `given_core_audio_permission_status_when_mapped_then_denial_remains_typed` exercises given core audio permission status when mapped then denial remains typed under its recorded setup (`test-052dbf5299c7bb5e6456`).
- `given_exact_application_target_when_framed_then_stable_identity_is_preserved` exercises given exact application target when framed then stable identity is preserved under its recorded setup (`test-2a85422b1f2de56f1698`).
- `given_other_core_audio_status_when_mapped_then_raw_status_is_preserved` exercises given other core audio status when mapped then raw status is preserved under its recorded setup (`test-08a6379bc20593cb04fd`).
- `given_reused_pid_with_different_application_when_verified_then_target_is_rejected` exercises given reused pid with different application when verified then target is rejected under its recorded setup (`test-741e0f83602c03a90e18`).
- `given_same_pid_and_application_when_verified_then_target_is_retained` exercises given same pid and application when verified then target is retained under its recorded setup (`test-610274c97d459763d74c`).
- `given_same_pid_and_application_with_new_creation_when_audited_then_reuse_is_detected` exercises given same pid and application with new creation when audited then reuse is detected under its recorded setup (`test-cd697175b87f09605ed4`).
- `given_exact_application_after_open_when_authorization_snapshotted_then_scope_stays_exact` exercises given exact application after open when authorization snapshotted then scope stays exact under its recorded setup (`test-8bcd43369d80f417a7c8`).

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

The claims on **No application audio arrives** are anchored to Git snapshot `136e74888962558aa846d3143a19136a70936f45` and these primary files:

- `src/capture/events.rs:1-1` (`DECLARED`)
- `src/capture/observations.rs:1-1` (`DECLARED`)

For **No application audio arrives**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.
