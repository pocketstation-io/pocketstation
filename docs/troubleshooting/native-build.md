# A native-capture build fails

<!-- claims: CLM-TRBL-015-CAP-001,CLM-TRBL-015-CAP-002,CLM-TRBL-015-CAP-003,CLM-TRBL-015-CAP-004,CLM-TRBL-015-SOURCE-001 -->

## Symptom

A build with the default native-capture feature fails for the selected target.

## Evidenced causes

- A target-specific native dependency or development library is unavailable.
- The build script selects a native shim that cannot compile.
- The toolchain or target does not meet package metadata and dependency requirements.

## Distinguish the causes

First run a contracts-only check with default features disabled. Then restore native capture and inspect the failing target dependency or build-script stage.

## Diagnostic signals

- `pocketstation::capture::authorization::CaptureError` (`error-8a6cfaf6313c49f3d002`)
- `pocketstation::capture::authorization::CaptureError` / `BackendInit` (`error-e16ac3af9c00b5a9e1ef`)
- `pocketstation::capture::authorization::CaptureError` / `BackendSetupRequired` (`error-49a3487734f77997ff1d`)
- `pocketstation::capture::authorization::CaptureError` / `BackendStatus` (`error-433a8f64b39d41fe58e4`)
- `pocketstation::capture::authorization::CaptureError` / `CaptureWorkerPanicked` (`error-6a1ddaf64fd582202ee9`)
- `pocketstation::capture::authorization::CaptureError` / `InvalidRuntimeEventCapacity` (`error-c838e8f36c42c18a2a83`)
- `pocketstation::capture::authorization::CaptureError` / `InvalidStreamCapacity` (`error-abbc7e6ad159c238bf74`)
- `pocketstation::capture::authorization::CaptureError` / `ModeUnsupported` (`error-4a58ec0f52d2f2ee5a44`)
- `pocketstation::capture::authorization::CaptureError` / `NotSupported` (`error-fc10abae73bd96954b49`)
- `pocketstation::capture::authorization::CaptureError` / `PermissionDenied` (`error-54d94f02abd4884ade73`)
- `pocketstation::capture::authorization::CaptureError` / `SourceUnavailable` (`error-fb207c871b52ba476b04`)
- `pocketstation::capture::events::CaptureRuntimeFailure` (`error-27b8f339dd8b80dbf899`)

## Executable evidence

- `given_negotiated_format_when_channel_count_changes_then_capture_fails_closed` exercises given negotiated format when channel count changes then capture fails closed under its recorded setup (`test-0f6c4f31518ab5e8ffd8`).
- `given_denied_permission_when_opening_input_then_capture_fails_closed` exercises given denied permission when opening input then capture fails closed under its recorded setup (`test-93f56a3510497f49f523`).
- `given_native_source_overlap_when_advanced_then_timeline_fails_closed` exercises given native source overlap when advanced then timeline fails closed under its recorded setup (`test-25a09fb5b40411afaa30`).
- `given_active_capture_when_owner_is_dropped_then_backend_is_reclaimed` exercises given active capture when owner is dropped then backend is reclaimed under its recorded setup (`test-fa34e5723160d56f560f`).
- `given_active_capture_when_stopped_then_backend_is_joined` exercises given active capture when stopped then backend is joined under its recorded setup (`test-dd4aaaf6b93ddb500769`).
- `given_backend_frame_when_source_differs_from_open_identity_then_lineage_fails_closed` exercises given backend frame when source differs from open identity then lineage fails closed under its recorded setup (`test-805d755d4acd2257ba9b`).
- `given_panicking_capture_worker_when_joined_then_typed_failure_is_returned` exercises given panicking capture worker when joined then typed failure is returned under its recorded setup (`test-2d873f94835a177ce436`).
- `given_prepared_capture_when_opened_then_bounded_delivery_is_owned` exercises given prepared capture when opened then bounded delivery is owned under its recorded setup (`test-a3a0d044f02b7f664bb9`).
- `given_capture_mode_when_channels_selected_then_microphone_is_mono_and_output_is_stereo` exercises given capture mode when channels selected then microphone is mono and output is stereo under its recorded setup (`test-df5c7fa69c2c79a8f2a1`).
- `given_exact_application_selector_when_identity_is_transient_then_selection_fails_closed` exercises given exact application selector when identity is transient then selection fails closed under its recorded setup (`test-1e40dd4ec9e96cd35eb7`).
- `given_exhausted_capture_pool_when_acquiring_then_failure_is_observed_once` exercises given exhausted capture pool when acquiring then failure is observed once under its recorded setup (`test-3002ec0fb883ffa835f6`).
- `given_pipewire_properties_when_native_format_is_reported_then_unknown_is_not_fabricated` exercises given pipewire properties when native format is reported then unknown is not fabricated under its recorded setup (`test-fe53d6769213e72fdd3a`).
- `given_capture_before_callback_when_mapped_then_process_timestamp_preserves_delay` exercises given capture before callback when mapped then process timestamp preserves delay under its recorded setup (`test-de7d536ac9b0edc1d4da`).
- `given_capture_before_process_epoch_when_mapped_then_timestamp_is_earliest_representable` exercises given capture before process epoch when mapped then timestamp is earliest representable under its recorded setup (`test-dc164b0e06605b749d99`).
- `given_promptable_or_observable_permission_when_opening_input_then_native_open_decides` exercises given promptable or observable permission when opening input then native open decides under its recorded setup (`test-136298dd50a44f77d3ac`).

## Corrective action

Install the target prerequisite or choose contracts-only mode when you do not need native capture.

## Retry and incomplete state

Rebuilding after environment correction is safe; it does not prove physical capture qualification. No runtime data exists from a failed build.

## Related reference

- [Cargo Features](/docs/concepts/cargo-features.md)
- [Compatibility](/docs/platform/compatibility.md)

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

The claims on **A native-capture build fails** are anchored to Git snapshot `136e74888962558aa846d3143a19136a70936f45` and these primary files:

- `Cargo.toml:1-180` (`DIRECT`)
- `build.rs:1-118` (`DIRECT`)

For **A native-capture build fails**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.
