# Permission state is denied or unobservable

<!-- claims: CLM-TRBL-004-CAP-001,CLM-TRBL-004-CAP-002,CLM-TRBL-004-CAP-003,CLM-TRBL-004-CAP-004,CLM-TRBL-004-CAP-005,CLM-TRBL-004-SOURCE-001 -->

Use this page when you observe **permission state is denied or unobservable**. Diagnose the reported stage and identity before changing route, source, connector, or lifecycle policy.

## Distinguish the cause

Treat NotObservable as neither grant nor denial. The source-open result is authoritative where the platform has no single process-wide observation.

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

- `given_denied_permission_when_opening_input_then_capture_fails_closed` exercises given denied permission when opening input then capture fails closed under its recorded setup (`test-2b664c22fd511e3c2f45`).
- `given_authoritative_permission_when_snapshotted_then_platform_state_is_preserved` exercises given authoritative permission when snapshotted then platform state is preserved under its recorded setup (`test-61d68aba989969d649b0`).
- `given_authorization_values_when_mapped_then_every_state_remains_distinct` exercises given authorization values when mapped then every state remains distinct under its recorded setup (`test-6b528a3cf33716f63062`).
- `given_promptable_or_observable_permission_when_opening_input_then_native_open_decides` exercises given promptable or observable permission when opening input then native open decides under its recorded setup (`test-847d3fefe4665db8dd14`).
- `given_core_audio_permission_status_when_mapped_then_denial_remains_typed` exercises given core audio permission status when mapped then denial remains typed under its recorded setup (`test-afb8b00e7e6a55f6d16d`).
- `given_permission_lifecycle_when_authorization_changes_then_epoch_and_kind_are_canonical` exercises given permission lifecycle when authorization changes then epoch and kind are canonical under its recorded setup (`test-72b7390fb29e3b3a2756`).
- `given_process_only_application_when_identity_inspected_then_strength_is_not_overstated` exercises given process only application when identity inspected then strength is not overstated under its recorded setup (`test-d54d43b584bfdf5600ab`).
- `given_revoked_permission_when_snapshotted_then_revocation_and_new_epoch_are_preserved` exercises given revoked permission when snapshotted then revocation and new epoch are preserved under its recorded setup (`test-458fc5c9256649d9f55e`).
- `given_unclassified_backend_failure_when_snapshotted_then_permission_is_not_guessed` exercises given unclassified backend failure when snapshotted then permission is not guessed under its recorded setup (`test-109c3c453f0c382dcb45`).
- `frame_stream_closed` exercises frame stream closed under its recorded setup (`test-3ab763bff0cd08d4b4e1`).
- `given_active_capture_when_owner_is_dropped_then_backend_is_reclaimed` exercises given active capture when owner is dropped then backend is reclaimed under its recorded setup (`test-c55d7a75628c1be024f1`).
- `given_active_capture_when_stopped_then_backend_is_joined` exercises given active capture when stopped then backend is joined under its recorded setup (`test-4f65c4d2e20b5226cd4f`).
- `given_backend_frame_when_source_differs_from_open_identity_then_lineage_fails_closed` exercises given backend frame when source differs from open identity then lineage fails closed under its recorded setup (`test-a8dbef4f3b61c752ce0e`).
- `given_panicking_capture_worker_when_joined_then_typed_failure_is_returned` exercises given panicking capture worker when joined then typed failure is returned under its recorded setup (`test-889c6cfb54cc924fc2b4`).
- `given_prepared_capture_when_opened_then_bounded_delivery_is_owned` exercises given prepared capture when opened then bounded delivery is owned under its recorded setup (`test-8de0974346f9110044c2`).

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
- [Linux capture](/docs/platform/linux.md)
- [Platform prerequisites](/docs/getting-started/platform-prerequisites.md)
- [Platform support and evidence](/docs/platform/compatibility.md)

## Evidence boundary

This page was verified against Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `src/capture/authorization.rs:1-318` (`DIRECT`)

A file's presence proves implementation or declaration at this snapshot. It does not by itself prove physical-device qualification, operational performance, retry safety, or behavior outside the recorded test conditions.
