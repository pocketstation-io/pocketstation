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

- `pocketstation::capture::authorization::CaptureError` (`error-7905cc933b9eb45fe4ef`)
- `pocketstation::capture::authorization::CaptureError` / `BackendInit` (`error-ffea5e00d982c5213eba`)
- `pocketstation::capture::authorization::CaptureError` / `BackendSetupRequired` (`error-6e8f9f8ca8efa76ded69`)
- `pocketstation::capture::authorization::CaptureError` / `BackendStatus` (`error-533b29bac30886d8c79c`)
- `pocketstation::capture::authorization::CaptureError` / `CaptureWorkerPanicked` (`error-01c4b3cce2fa1669ee13`)
- `pocketstation::capture::authorization::CaptureError` / `InvalidRuntimeEventCapacity` (`error-c683702117e27ad45f33`)
- `pocketstation::capture::authorization::CaptureError` / `InvalidStreamCapacity` (`error-6167103023ec8fded812`)
- `pocketstation::capture::authorization::CaptureError` / `ModeUnsupported` (`error-786199dd7e94542436f2`)
- `pocketstation::capture::authorization::CaptureError` / `NotSupported` (`error-0f2fd6c6275925740175`)
- `pocketstation::capture::authorization::CaptureError` / `PermissionDenied` (`error-d902cf4c11a93cbcb084`)
- `pocketstation::capture::authorization::CaptureError` / `SourceUnavailable` (`error-61051d668a17eec6c3ac`)
- `pocketstation::capture::events::CaptureRuntimeFailure` (`error-ee187ecbd20c3485593b`)

## Executable evidence

- `given_negotiated_format_when_channel_count_changes_then_capture_fails_closed` exercises given negotiated format when channel count changes then capture fails closed under its recorded setup (`test-1993ee9e15230d1f6226`).
- `given_denied_permission_when_opening_input_then_capture_fails_closed` exercises given denied permission when opening input then capture fails closed under its recorded setup (`test-2b664c22fd511e3c2f45`).
- `given_native_source_overlap_when_advanced_then_timeline_fails_closed` exercises given native source overlap when advanced then timeline fails closed under its recorded setup (`test-99083db5a93958229c27`).
- `given_active_capture_when_owner_is_dropped_then_backend_is_reclaimed` exercises given active capture when owner is dropped then backend is reclaimed under its recorded setup (`test-c55d7a75628c1be024f1`).
- `given_active_capture_when_stopped_then_backend_is_joined` exercises given active capture when stopped then backend is joined under its recorded setup (`test-4f65c4d2e20b5226cd4f`).
- `given_backend_frame_when_source_differs_from_open_identity_then_lineage_fails_closed` exercises given backend frame when source differs from open identity then lineage fails closed under its recorded setup (`test-a8dbef4f3b61c752ce0e`).
- `given_panicking_capture_worker_when_joined_then_typed_failure_is_returned` exercises given panicking capture worker when joined then typed failure is returned under its recorded setup (`test-889c6cfb54cc924fc2b4`).
- `given_prepared_capture_when_opened_then_bounded_delivery_is_owned` exercises given prepared capture when opened then bounded delivery is owned under its recorded setup (`test-8de0974346f9110044c2`).
- `given_capture_mode_when_channels_selected_then_microphone_is_mono_and_output_is_stereo` exercises given capture mode when channels selected then microphone is mono and output is stereo under its recorded setup (`test-c28f1242d8a2b60457db`).
- `given_exact_application_selector_when_identity_is_transient_then_selection_fails_closed` exercises given exact application selector when identity is transient then selection fails closed under its recorded setup (`test-1a09c0b9480a09c36429`).
- `given_exhausted_capture_pool_when_acquiring_then_failure_is_observed_once` exercises given exhausted capture pool when acquiring then failure is observed once under its recorded setup (`test-bcfd12a436362de05085`).
- `given_pipewire_properties_when_native_format_is_reported_then_unknown_is_not_fabricated` exercises given pipewire properties when native format is reported then unknown is not fabricated under its recorded setup (`test-03b9dd302a982e69d584`).
- `given_capture_before_callback_when_mapped_then_process_timestamp_preserves_delay` exercises given capture before callback when mapped then process timestamp preserves delay under its recorded setup (`test-8a2ea38f6f2c1b3ffa2f`).
- `given_capture_before_process_epoch_when_mapped_then_timestamp_is_earliest_representable` exercises given capture before process epoch when mapped then timestamp is earliest representable under its recorded setup (`test-9519b3f93a4a0e689bcc`).
- `given_promptable_or_observable_permission_when_opening_input_then_native_open_decides` exercises given promptable or observable permission when opening input then native open decides under its recorded setup (`test-847d3fefe4665db8dd14`).

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

The claims on **A native-capture build fails** are anchored to Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `Cargo.toml:1-180` (`DIRECT`)
- `build.rs:1-118` (`DIRECT`)

For **A native-capture build fails**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.
