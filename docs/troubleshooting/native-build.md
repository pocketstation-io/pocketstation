# A native-capture build fails

<!-- claims: CLM-TRBL-015-CAP-001,CLM-TRBL-015-CAP-002,CLM-TRBL-015-CAP-003,CLM-TRBL-015-CAP-004,CLM-TRBL-015-SOURCE-001 -->

Use this page when you observe **a native-capture build fails**. Diagnose the reported stage and identity before changing route, source, connector, or lifecycle policy.

## Distinguish the cause

Reproduce without default features to separate Core contracts from native dependencies, then restore native capture and diagnose the selected target dependency.

## Diagnostic signals

- `pocketstation::capture::events::CaptureRuntimeFailure` (`error-11b972ad42d5de880e06`)
- `pocketstation::capture::events::CaptureRuntimeFailureClass` / `BackendClass` (`error-29e952ae7432566a9e95`)
- `pocketstation::capture::authorization::CaptureError` / `CaptureWorkerPanicked` (`error-365f9b6fbda74eb0d631`)
- `pocketstation::capture::authorization::CaptureError` / `PermissionDenied` (`error-38030156125346a8e892`)
- `pocketstation::capture::authorization::CaptureError` / `NotSupported` (`error-3b4b5393164d9f6f12a5`)
- `pocketstation::capture::events::CaptureRuntimeFailureClass` / `PlatformStatus` (`error-3c6fcc22deb2f54788ba`)
- `pocketstation::capture::authorization::CaptureError` / `SourceUnavailable` (`error-71c87f975acc9e22a402`)
- `pocketstation::capture::authorization::CaptureError` / `BackendSetupRequired` (`error-8db0fec69a9c7158ffdf`)
- `pocketstation::capture::authorization::CaptureError` (`error-96ffe4bc4254583d1e17`)
- `pocketstation::capture::events::CaptureRuntimeFailureClass` / `SourceInstanceExited` (`error-a9c0f7dfff744e9ba6b7`)
- `pocketstation::capture::authorization::CaptureError` / `BackendInit` (`error-b320ea1cba2b3c8dc4c7`)
- `pocketstation::capture::authorization::CaptureError` / `InvalidStreamCapacity` (`error-bcf5d4d897b6bd0784bf`)
- `pocketstation::capture::authorization::CaptureError` / `ModeUnsupported` (`error-bf1be2fb486df6136dc5`)
- `pocketstation::capture::authorization::CaptureError` / `InvalidRuntimeEventCapacity` (`error-ceedf8c06740748c9bd5`)
- `pocketstation::capture::authorization::CaptureError` / `BackendStatus` (`error-e8046b5b5989518ee482`)
- `pocketstation::capture::events::CaptureRuntimeFailureClass` (`error-ea2d5a94280522f41764`)

## Executable evidence

- `given_negotiated_format_when_channel_count_changes_then_capture_fails_closed` exercises given negotiated format when channel count changes then capture fails closed under its recorded setup (`test-1993ee9e15230d1f6226`).
- `given_denied_permission_when_opening_input_then_capture_fails_closed` exercises given denied permission when opening input then capture fails closed under its recorded setup (`test-2b664c22fd511e3c2f45`).
- `given_native_source_overlap_when_advanced_then_timeline_fails_closed` exercises given native source overlap when advanced then timeline fails closed under its recorded setup (`test-99083db5a93958229c27`).
- `given_active_capture_when_owner_is_dropped_then_backend_is_reclaimed` exercises given active capture when owner is dropped then backend is reclaimed under its recorded setup (`test-c55d7a75628c1be024f1`).
- `given_active_capture_when_stopped_then_backend_is_joined` exercises given active capture when stopped then backend is joined under its recorded setup (`test-4f65c4d2e20b5226cd4f`).
- `given_backend_frame_when_source_differs_from_open_identity_then_lineage_fails_closed` exercises given backend frame when source differs from open identity then lineage fails closed under its recorded setup (`test-a8dbef4f3b61c752ce0e`).
- `given_panicking_capture_worker_when_joined_then_typed_failure_is_returned` exercises given panicking capture worker when joined then typed failure is returned under its recorded setup (`test-889c6cfb54cc924fc2b4`).
- `given_prepared_capture_when_opened_then_bounded_delivery_is_owned` exercises given prepared capture when opened then bounded delivery is owned under its recorded setup (`test-8de0974346f9110044c2`).
- `join_capture_worker` exercises join capture worker under its recorded setup (`test-89b10abefa1f5c9a47e2`).
- `prepare_capture` exercises prepare capture under its recorded setup (`test-59d7e50bbae31896948a`).
- `captured_frame_stream` exercises captured frame stream under its recorded setup (`test-0e40457259bf43cdd2a7`).
- `given_capture_mode_when_channels_selected_then_microphone_is_mono_and_output_is_stereo` exercises given capture mode when channels selected then microphone is mono and output is stereo under its recorded setup (`test-c28f1242d8a2b60457db`).
- `given_exact_application_selector_when_identity_is_transient_then_selection_fails_closed` exercises given exact application selector when identity is transient then selection fails closed under its recorded setup (`test-1a09c0b9480a09c36429`).
- `given_exhausted_capture_pool_when_acquiring_then_failure_is_observed_once` exercises given exhausted capture pool when acquiring then failure is observed once under its recorded setup (`test-bcfd12a436362de05085`).
- `given_pipewire_properties_when_native_format_is_reported_then_unknown_is_not_fabricated` exercises given pipewire properties when native format is reported then unknown is not fabricated under its recorded setup (`test-03b9dd302a982e69d584`).

## Corrective action and retry

Apply only the action implied by the typed failure or violated precondition. Retry is not safe merely because a failure appears transient. When retryability or recovery is unknown, preserve the failure for application policy or maintainer review.

## Data and state

Treat frames, signals, files, acknowledgements, and finalization results produced before failure as potentially partial unless the terminal contract says otherwise. Inspect per-route, per-stem, and per-component outcomes.

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

This page was verified against Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `Cargo.toml:1-180` (`DIRECT`)
- `build.rs:1-118` (`DIRECT`)

A file's presence proves implementation or declaration at this snapshot. It does not by itself prove physical-device qualification, operational performance, retry safety, or behavior outside the recorded test conditions.
