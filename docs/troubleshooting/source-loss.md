# A capture source disappears

<!-- claims: CLM-TRBL-005-SCOPE-001,CLM-TRBL-005-TEXT-001,CLM-TRBL-005-TEXT-002,CLM-TRBL-005-TEXT-003,CLM-TRBL-005-TEXT-004,CLM-TRBL-005-TEXT-005,CLM-TRBL-005-TEXT-006,CLM-TRBL-005-TEXT-007,CLM-TRBL-005-SOURCE-001 -->

## Symptom

A previously selected capture source disappears or returns with a new identity generation.

## Evidenced causes

- The application exited or its exact process instance changed.
- The input device was removed or the default device changed.
- Permission or platform lifecycle state changed.

## Distinguish the causes

Inspect the source event kind, stable source ID, prior and current generation, and permission epoch. Correlate frames by lineage before and after the event.

## Diagnostic signals

- `pocketstation::capture::events::CaptureRuntimeFailureClass` / `SourceInstanceExited` (`error-677a63665bbdf8a0715a`)
- `pocketstation::capture::events::CaptureRuntimeFailure` (`error-27b8f339dd8b80dbf899`)
- `pocketstation::capture::events::CaptureRuntimeFailureClass` (`error-3169b6fff966018c5c73`)
- `pocketstation::capture::events::CaptureRuntimeFailureClass` / `BackendClass` (`error-29a5a1b804166e5e41ea`)
- `pocketstation::capture::events::CaptureRuntimeFailureClass` / `PlatformStatus` (`error-476e2ddf8c1ad7099081`)
- `pocketstation::capture::authorization::CaptureError` / `InvalidRuntimeEventCapacity` (`error-c838e8f36c42c18a2a83`)
- `pocketstation::capture::authorization::CaptureError` / `SourceUnavailable` (`error-fb207c871b52ba476b04`)
- `pocketstation::capture::timeline::CaptureSampleTimelineError` / `SourcePositionMovedBackward` (`error-012287cf4e78fb89426b`)
- `pocketstation::capture::timeline::CaptureSampleTimelineError` / `SourcePositionOverflow` (`error-c17db2e1686bee7e86be`)
- `pocketstation::session::error_code::SessionStartErrorCode` / `CaptureSourceUnavailable` (`error-847d3f82fd75ad64c8a0`)
- `pocketstation::session::lifecycle::control::SessionStartError` / `CaptureOpen` (`error-b4ace4191cf897863e82`)
- `pocketstation::session::lifecycle::control::SessionStartError` / `CapturePrepare` (`error-cd9c5fe2a7f48f75a63f`)

## Executable evidence

- `given_canonical_capture_identity_when_derived_then_source_id_matches_stable_vector` exercises given canonical capture identity when derived then source id matches stable vector under its recorded setup (`test-9c549d91f364bb436c12`).
- `given_active_capture_when_owner_is_dropped_then_backend_is_reclaimed` exercises given active capture when owner is dropped then backend is reclaimed under its recorded setup (`test-fa34e5723160d56f560f`).
- `given_active_capture_when_stopped_then_backend_is_joined` exercises given active capture when stopped then backend is joined under its recorded setup (`test-dd4aaaf6b93ddb500769`).
- `given_backend_frame_when_source_differs_from_open_identity_then_lineage_fails_closed` exercises given backend frame when source differs from open identity then lineage fails closed under its recorded setup (`test-805d755d4acd2257ba9b`).
- `given_panicking_capture_worker_when_joined_then_typed_failure_is_returned` exercises given panicking capture worker when joined then typed failure is returned under its recorded setup (`test-2d873f94835a177ce436`).
- `given_prepared_capture_when_opened_then_bounded_delivery_is_owned` exercises given prepared capture when opened then bounded delivery is owned under its recorded setup (`test-a3a0d044f02b7f664bb9`).
- `given_capture_mode_when_channels_selected_then_microphone_is_mono_and_output_is_stereo` exercises given capture mode when channels selected then microphone is mono and output is stereo under its recorded setup (`test-df5c7fa69c2c79a8f2a1`).
- `given_exhausted_capture_pool_when_acquiring_then_failure_is_observed_once` exercises given exhausted capture pool when acquiring then failure is observed once under its recorded setup (`test-3002ec0fb883ffa835f6`).
- `given_missing_exact_source_when_classified_then_stable_key_is_preserved` exercises given missing exact source when classified then stable key is preserved under its recorded setup (`test-d288558b68fc54333e50`).
- `given_negotiated_format_when_channel_count_changes_then_capture_fails_closed` exercises given negotiated format when channel count changes then capture fails closed under its recorded setup (`test-0f6c4f31518ab5e8ffd8`).
- `given_capture_before_callback_when_mapped_then_process_timestamp_preserves_delay` exercises given capture before callback when mapped then process timestamp preserves delay under its recorded setup (`test-de7d536ac9b0edc1d4da`).
- `given_capture_before_process_epoch_when_mapped_then_timestamp_is_earliest_representable` exercises given capture before process epoch when mapped then timestamp is earliest representable under its recorded setup (`test-dc164b0e06605b749d99`).
- `given_denied_permission_when_opening_input_then_capture_fails_closed` exercises given denied permission when opening input then capture fails closed under its recorded setup (`test-93f56a3510497f49f523`).
- `given_device_invalidated_hresult_when_classified_then_source_is_unavailable` exercises given device invalidated hresult when classified then source is unavailable under its recorded setup (`test-d191d2cb74b1f34f301b`).
- `given_resources_invalidated_hresult_when_classified_then_failure_is_not_guessed_as_disappearance` exercises given resources invalidated hresult when classified then failure is not guessed as disappearance under its recorded setup (`test-f7437f6b9062abefafe0`).

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

The claims on **A capture source disappears** are anchored to Git snapshot `136e74888962558aa846d3143a19136a70936f45` and these primary files:

- `src/capture/events.rs:1-1` (`DECLARED`)
- `src/capture/lifecycle_registry.rs:1-1` (`DECLARED`)

For **A capture source disappears**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.
