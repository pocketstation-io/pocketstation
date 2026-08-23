# No application audio arrives

<!-- claims: CLM-TRBL-002-CAP-001,CLM-TRBL-002-CAP-002,CLM-TRBL-002-CAP-003,CLM-TRBL-002-CAP-004,CLM-TRBL-002-CAP-005,CLM-TRBL-002-SOURCE-001 -->

Use this page when you observe **no application audio arrives**. Diagnose the reported stage and identity before changing route, source, connector, or lifecycle policy.

## Distinguish the cause

Confirm that selection resolves the intended source, then distinguish permission, open, lifecycle, and delivery observations. A running Session with no frames does not prove that capture opened the intended application.

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
- [Capture a desktop application](/docs/how-to/capture-application.md)
- [Capture API](/docs/reference/capture.md)
- [A capture source disappears](/docs/troubleshooting/source-loss.md)

## Evidence boundary

This page was verified against Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `src/capture/events.rs:1-344` (`DIRECT`)
- `src/capture/observations.rs:1-130` (`DIRECT`)

A file's presence proves implementation or declaration at this snapshot. It does not by itself prove physical-device qualification, operational performance, retry safety, or behavior outside the recorded test conditions.
