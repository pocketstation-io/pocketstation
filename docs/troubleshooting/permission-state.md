# Permission state is denied or unobservable

<!-- claims: CLM-TRBL-004-CAP-001,CLM-TRBL-004-CAP-002,CLM-TRBL-004-CAP-003,CLM-TRBL-004-CAP-004,CLM-TRBL-004-CAP-005,CLM-TRBL-004-SOURCE-001 -->

## Symptom

Permission preflight reports denied or cannot produce a reliable state.

## Evidenced causes

- The platform reports a real denial.
- The target exposes no non-prompting observation and returns `NotObservable`.
- Authorization changed after a previous observation.

## Distinguish the causes

Treat denied and unobservable as different values. Compare the permission epoch and use the next source-open outcome as authority.

## Diagnostic signals

- `pocketstation::capture::authorization::CaptureError` / `PermissionDenied` (`error-d902cf4c11a93cbcb084`)
- `pocketstation::capture::authorization::CaptureError` (`error-7905cc933b9eb45fe4ef`)
- `pocketstation::capture::authorization::CaptureError` / `BackendInit` (`error-ffea5e00d982c5213eba`)
- `pocketstation::capture::authorization::CaptureError` / `BackendSetupRequired` (`error-6e8f9f8ca8efa76ded69`)
- `pocketstation::capture::authorization::CaptureError` / `BackendStatus` (`error-533b29bac30886d8c79c`)
- `pocketstation::capture::authorization::CaptureError` / `CaptureWorkerPanicked` (`error-01c4b3cce2fa1669ee13`)
- `pocketstation::capture::authorization::CaptureError` / `InvalidRuntimeEventCapacity` (`error-c683702117e27ad45f33`)
- `pocketstation::capture::authorization::CaptureError` / `InvalidStreamCapacity` (`error-6167103023ec8fded812`)
- `pocketstation::capture::authorization::CaptureError` / `ModeUnsupported` (`error-786199dd7e94542436f2`)
- `pocketstation::capture::authorization::CaptureError` / `NotSupported` (`error-0f2fd6c6275925740175`)
- `pocketstation::capture::authorization::CaptureError` / `SourceUnavailable` (`error-61051d668a17eec6c3ac`)

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
- `given_active_capture_when_owner_is_dropped_then_backend_is_reclaimed` exercises given active capture when owner is dropped then backend is reclaimed under its recorded setup (`test-c55d7a75628c1be024f1`).
- `given_active_capture_when_stopped_then_backend_is_joined` exercises given active capture when stopped then backend is joined under its recorded setup (`test-4f65c4d2e20b5226cd4f`).
- `given_backend_frame_when_source_differs_from_open_identity_then_lineage_fails_closed` exercises given backend frame when source differs from open identity then lineage fails closed under its recorded setup (`test-a8dbef4f3b61c752ce0e`).
- `given_panicking_capture_worker_when_joined_then_typed_failure_is_returned` exercises given panicking capture worker when joined then typed failure is returned under its recorded setup (`test-889c6cfb54cc924fc2b4`).
- `given_prepared_capture_when_opened_then_bounded_delivery_is_owned` exercises given prepared capture when opened then bounded delivery is owned under its recorded setup (`test-8de0974346f9110044c2`).
- `given_zero_frame_capacity_when_preparing_then_backend_is_not_prepared` exercises given zero frame capacity when preparing then backend is not prepared under its recorded setup (`test-f42d54d3bd1632c2ccfa`).

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

The claims on **Permission state is denied or unobservable** are anchored to Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `src/capture/authorization.rs:1-318` (`DIRECT`)

For **Permission state is denied or unobservable**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.
