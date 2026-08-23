# A capture source disappears

<!-- claims: CLM-TRBL-005-CAP-001,CLM-TRBL-005-CAP-002,CLM-TRBL-005-CAP-003,CLM-TRBL-005-CAP-004,CLM-TRBL-005-CAP-005,CLM-TRBL-005-SOURCE-001 -->

Use this page when you observe **a capture source disappears**. Diagnose the reported stage and identity before changing route, source, connector, or lifecycle policy.

## Distinguish the cause

Inspect source lifecycle event kind, source generation, and permission epoch. Preserve lineage from frames received before the change.

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

- `given_canonical_capture_identity_when_derived_then_source_id_matches_stable_vector` exercises given canonical capture identity when derived then source id matches stable vector under its recorded setup (`test-39fa4a1bc5fb034e360f`).
- `given_active_capture_when_owner_is_dropped_then_backend_is_reclaimed` exercises given active capture when owner is dropped then backend is reclaimed under its recorded setup (`test-c55d7a75628c1be024f1`).
- `given_active_capture_when_stopped_then_backend_is_joined` exercises given active capture when stopped then backend is joined under its recorded setup (`test-4f65c4d2e20b5226cd4f`).
- `given_backend_frame_when_source_differs_from_open_identity_then_lineage_fails_closed` exercises given backend frame when source differs from open identity then lineage fails closed under its recorded setup (`test-a8dbef4f3b61c752ce0e`).
- `given_panicking_capture_worker_when_joined_then_typed_failure_is_returned` exercises given panicking capture worker when joined then typed failure is returned under its recorded setup (`test-889c6cfb54cc924fc2b4`).
- `given_prepared_capture_when_opened_then_bounded_delivery_is_owned` exercises given prepared capture when opened then bounded delivery is owned under its recorded setup (`test-8de0974346f9110044c2`).
- `join_capture_worker` exercises join capture worker under its recorded setup (`test-89b10abefa1f5c9a47e2`).
- `prepare_capture` exercises prepare capture under its recorded setup (`test-59d7e50bbae31896948a`).
- `captured_frame_stream` exercises captured frame stream under its recorded setup (`test-0e40457259bf43cdd2a7`).
- `given_capture_mode_when_channels_selected_then_microphone_is_mono_and_output_is_stereo` exercises given capture mode when channels selected then microphone is mono and output is stereo under its recorded setup (`test-c28f1242d8a2b60457db`).
- `given_exhausted_capture_pool_when_acquiring_then_failure_is_observed_once` exercises given exhausted capture pool when acquiring then failure is observed once under its recorded setup (`test-bcfd12a436362de05085`).
- `given_missing_exact_source_when_classified_then_stable_key_is_preserved` exercises given missing exact source when classified then stable key is preserved under its recorded setup (`test-50620fcc9117c7ad3cf6`).
- `given_negotiated_format_when_channel_count_changes_then_capture_fails_closed` exercises given negotiated format when channel count changes then capture fails closed under its recorded setup (`test-1993ee9e15230d1f6226`).
- `given_capture_before_callback_when_mapped_then_process_timestamp_preserves_delay` exercises given capture before callback when mapped then process timestamp preserves delay under its recorded setup (`test-8a2ea38f6f2c1b3ffa2f`).
- `given_capture_before_process_epoch_when_mapped_then_timestamp_is_earliest_representable` exercises given capture before process epoch when mapped then timestamp is earliest representable under its recorded setup (`test-9519b3f93a4a0e689bcc`).

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
- [Preserve source identity](/docs/best-practices/source-identity.md)
- [Terminal outcomes](/docs/lifecycle/terminal-outcomes.md)

## Evidence boundary

This page was verified against Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `src/capture/events.rs:1-344` (`DIRECT`)
- `src/capture/lifecycle_registry.rs:1-88` (`DIRECT`)

A file's presence proves implementation or declaration at this snapshot. It does not by itself prove physical-device qualification, operational performance, retry safety, or behavior outside the recorded test conditions.
