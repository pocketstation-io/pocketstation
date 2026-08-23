# A capture source disappears

<!-- claims: CLM-TRBL-005-CAP-001,CLM-TRBL-005-CAP-002,CLM-TRBL-005-CAP-003,CLM-TRBL-005-CAP-004,CLM-TRBL-005-CAP-005,CLM-TRBL-005-SOURCE-001 -->

## Symptom

A previously selected capture source disappears or returns with a new identity generation.

## Evidenced causes

- The application exited or its exact process instance changed.
- The input device was removed or the default device changed.
- Permission or platform lifecycle state changed.

## Distinguish the causes

Inspect the source event kind, stable source ID, prior and current generation, and permission epoch. Correlate frames by lineage before and after the event.

## Diagnostic signals

- `pocketstation::capture::events::CaptureRuntimeFailureClass` / `SourceInstanceExited` (`error-35fe109728e80f2b126f`)
- `pocketstation::capture::events::CaptureRuntimeFailure` (`error-ee187ecbd20c3485593b`)
- `pocketstation::capture::events::CaptureRuntimeFailureClass` (`error-00ac112f8cac9b2976c5`)
- `pocketstation::capture::events::CaptureRuntimeFailureClass` / `BackendClass` (`error-ed79157bc39ce0d41fad`)
- `pocketstation::capture::events::CaptureRuntimeFailureClass` / `PlatformStatus` (`error-ae0e4e8e83a2cdd49e9e`)
- `pocketstation::capture::authorization::CaptureError` / `InvalidRuntimeEventCapacity` (`error-c683702117e27ad45f33`)
- `pocketstation::capture::authorization::CaptureError` / `SourceUnavailable` (`error-61051d668a17eec6c3ac`)
- `pocketstation::capture::timeline::CaptureSampleTimelineError` / `SourcePositionMovedBackward` (`error-3812c0f164d9b3c8357b`)
- `pocketstation::capture::timeline::CaptureSampleTimelineError` / `SourcePositionOverflow` (`error-51de207a9656d496b4d1`)
- `pocketstation::session::error_code::SessionStartErrorCode` / `CaptureSourceUnavailable` (`error-543e76758f1868803745`)
- `pocketstation::session::lifecycle::start_contract::SessionStartError` / `CaptureOpen` (`error-f06eab3e8f6ceeae37d2`)
- `pocketstation::session::lifecycle::start_contract::SessionStartError` / `CapturePrepare` (`error-8e2ce672937c8251c7d6`)

## Executable evidence

- `given_canonical_capture_identity_when_derived_then_source_id_matches_stable_vector` exercises given canonical capture identity when derived then source id matches stable vector under its recorded setup (`test-39fa4a1bc5fb034e360f`).
- `given_active_capture_when_owner_is_dropped_then_backend_is_reclaimed` exercises given active capture when owner is dropped then backend is reclaimed under its recorded setup (`test-c55d7a75628c1be024f1`).
- `given_active_capture_when_stopped_then_backend_is_joined` exercises given active capture when stopped then backend is joined under its recorded setup (`test-4f65c4d2e20b5226cd4f`).
- `given_backend_frame_when_source_differs_from_open_identity_then_lineage_fails_closed` exercises given backend frame when source differs from open identity then lineage fails closed under its recorded setup (`test-a8dbef4f3b61c752ce0e`).
- `given_panicking_capture_worker_when_joined_then_typed_failure_is_returned` exercises given panicking capture worker when joined then typed failure is returned under its recorded setup (`test-889c6cfb54cc924fc2b4`).
- `given_prepared_capture_when_opened_then_bounded_delivery_is_owned` exercises given prepared capture when opened then bounded delivery is owned under its recorded setup (`test-8de0974346f9110044c2`).
- `given_capture_mode_when_channels_selected_then_microphone_is_mono_and_output_is_stereo` exercises given capture mode when channels selected then microphone is mono and output is stereo under its recorded setup (`test-c28f1242d8a2b60457db`).
- `given_exhausted_capture_pool_when_acquiring_then_failure_is_observed_once` exercises given exhausted capture pool when acquiring then failure is observed once under its recorded setup (`test-bcfd12a436362de05085`).
- `given_missing_exact_source_when_classified_then_stable_key_is_preserved` exercises given missing exact source when classified then stable key is preserved under its recorded setup (`test-50620fcc9117c7ad3cf6`).
- `given_negotiated_format_when_channel_count_changes_then_capture_fails_closed` exercises given negotiated format when channel count changes then capture fails closed under its recorded setup (`test-1993ee9e15230d1f6226`).
- `given_capture_before_callback_when_mapped_then_process_timestamp_preserves_delay` exercises given capture before callback when mapped then process timestamp preserves delay under its recorded setup (`test-8a2ea38f6f2c1b3ffa2f`).
- `given_capture_before_process_epoch_when_mapped_then_timestamp_is_earliest_representable` exercises given capture before process epoch when mapped then timestamp is earliest representable under its recorded setup (`test-9519b3f93a4a0e689bcc`).
- `given_denied_permission_when_opening_input_then_capture_fails_closed` exercises given denied permission when opening input then capture fails closed under its recorded setup (`test-2b664c22fd511e3c2f45`).
- `given_device_invalidated_hresult_when_classified_then_source_is_unavailable` exercises given device invalidated hresult when classified then source is unavailable under its recorded setup (`test-d2f761449f8212754ae7`).
- `given_resources_invalidated_hresult_when_classified_then_failure_is_not_guessed_as_disappearance` exercises given resources invalidated hresult when classified then failure is not guessed as disappearance under its recorded setup (`test-acc6963aea9a1e14e631`).

## Corrective action

Stop depending routes or resolve and prepare a new source generation. Do not relabel earlier frames as if they came from the replacement.

## Retry and incomplete state

A new generation requires a new source state; replay safety is not implied. Previously delivered frames remain valid only with their original lineage.

## Related reference

- [Permissions And Source Lifecycle](/docs/concepts/permissions-and-source-lifecycle.md)
- [Source Identity](/docs/best-practices/source-identity.md)

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

The claims on **A capture source disappears** are anchored to Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `src/capture/events.rs:1-344` (`DIRECT`)
- `src/capture/lifecycle_registry.rs:1-88` (`DIRECT`)

For **A capture source disappears**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.
