# Permission state is denied or unobservable

<!-- claims: CLM-TRBL-004-SCOPE-001,CLM-TRBL-004-TEXT-001,CLM-TRBL-004-TEXT-002,CLM-TRBL-004-TEXT-003,CLM-TRBL-004-TEXT-004,CLM-TRBL-004-TEXT-005,CLM-TRBL-004-TEXT-006,CLM-TRBL-004-SOURCE-001 -->

## Symptom

Permission preflight reports denied or cannot produce a reliable state.

## Evidenced causes

- The platform reports a real denial.
- The target exposes no non-prompting observation and returns `NotObservable`.
- Authorization changed after a previous observation.

## Distinguish the causes

Treat denied and unobservable as different values. Compare the permission epoch and use the next source-open outcome as authority.

## Diagnostic signals

- `pocketstation::capture::authorization::CaptureError` / `PermissionDenied` (`error-54d94f02abd4884ade73`)
- `pocketstation::capture::authorization::CaptureError` (`error-8a6cfaf6313c49f3d002`)
- `pocketstation::capture::authorization::CaptureError` / `BackendInit` (`error-e16ac3af9c00b5a9e1ef`)
- `pocketstation::capture::authorization::CaptureError` / `BackendSetupRequired` (`error-49a3487734f77997ff1d`)
- `pocketstation::capture::authorization::CaptureError` / `BackendStatus` (`error-433a8f64b39d41fe58e4`)
- `pocketstation::capture::authorization::CaptureError` / `CaptureWorkerPanicked` (`error-6a1ddaf64fd582202ee9`)
- `pocketstation::capture::authorization::CaptureError` / `InvalidRuntimeEventCapacity` (`error-c838e8f36c42c18a2a83`)
- `pocketstation::capture::authorization::CaptureError` / `InvalidStreamCapacity` (`error-abbc7e6ad159c238bf74`)
- `pocketstation::capture::authorization::CaptureError` / `ModeUnsupported` (`error-4a58ec0f52d2f2ee5a44`)
- `pocketstation::capture::authorization::CaptureError` / `NotSupported` (`error-fc10abae73bd96954b49`)
- `pocketstation::capture::authorization::CaptureError` / `SourceUnavailable` (`error-fb207c871b52ba476b04`)

## Executable evidence

- `given_denied_permission_when_opening_input_then_capture_fails_closed` exercises given denied permission when opening input then capture fails closed under its recorded setup (`test-93f56a3510497f49f523`).
- `given_authoritative_permission_when_snapshotted_then_platform_state_is_preserved` exercises given authoritative permission when snapshotted then platform state is preserved under its recorded setup (`test-faaf69147963e5e88acc`).
- `given_authorization_values_when_mapped_then_every_state_remains_distinct` exercises given authorization values when mapped then every state remains distinct under its recorded setup (`test-29e0dda088edb57b37e0`).
- `given_promptable_or_observable_permission_when_opening_input_then_native_open_decides` exercises given promptable or observable permission when opening input then native open decides under its recorded setup (`test-136298dd50a44f77d3ac`).
- `given_core_audio_permission_status_when_mapped_then_denial_remains_typed` exercises given core audio permission status when mapped then denial remains typed under its recorded setup (`test-052dbf5299c7bb5e6456`).
- `given_permission_lifecycle_when_authorization_changes_then_epoch_and_kind_are_canonical` exercises given permission lifecycle when authorization changes then epoch and kind are canonical under its recorded setup (`test-e62afd814c0b32723785`).
- `given_process_only_application_when_identity_inspected_then_strength_is_not_overstated` exercises given process only application when identity inspected then strength is not overstated under its recorded setup (`test-9b3f553dcb5719207ea0`).
- `given_revoked_permission_when_snapshotted_then_revocation_and_new_epoch_are_preserved` exercises given revoked permission when snapshotted then revocation and new epoch are preserved under its recorded setup (`test-a9baea9d903dfd343a6e`).
- `given_unclassified_backend_failure_when_snapshotted_then_permission_is_not_guessed` exercises given unclassified backend failure when snapshotted then permission is not guessed under its recorded setup (`test-8540d7d4e458158fb98e`).
- `given_active_capture_when_owner_is_dropped_then_backend_is_reclaimed` exercises given active capture when owner is dropped then backend is reclaimed under its recorded setup (`test-fa34e5723160d56f560f`).
- `given_active_capture_when_stopped_then_backend_is_joined` exercises given active capture when stopped then backend is joined under its recorded setup (`test-dd4aaaf6b93ddb500769`).
- `given_backend_frame_when_source_differs_from_open_identity_then_lineage_fails_closed` exercises given backend frame when source differs from open identity then lineage fails closed under its recorded setup (`test-805d755d4acd2257ba9b`).
- `given_panicking_capture_worker_when_joined_then_typed_failure_is_returned` exercises given panicking capture worker when joined then typed failure is returned under its recorded setup (`test-2d873f94835a177ce436`).
- `given_prepared_capture_when_opened_then_bounded_delivery_is_owned` exercises given prepared capture when opened then bounded delivery is owned under its recorded setup (`test-a3a0d044f02b7f664bb9`).
- `given_zero_frame_capacity_when_preparing_then_backend_is_not_prepared` exercises given zero frame capacity when preparing then backend is not prepared under its recorded setup (`test-0afbec4242ea2fad4582`).

## Corrective action

For denial, guide the developer through the host's platform UI. For unobservable state, continue only through an explicit user action and handle the open result.

## Retry and incomplete state

A denied state is not safe to hammer with retries; an unobservable state is not success. No source frames exist until opening succeeds.

## Related reference

- [Permissions And Source Lifecycle](/docs/concepts/permissions-and-source-lifecycle.md)
- [Capture](/docs/reference/capture.md)

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

The claims on **Permission state is denied or unobservable** are anchored to Git snapshot `136e74888962558aa846d3143a19136a70936f45` and these primary files:

- `src/capture/authorization.rs:1-1` (`DECLARED`)

For **Permission state is denied or unobservable**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.
