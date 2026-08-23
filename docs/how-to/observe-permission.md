# Observe permission without prompting

<!-- claims: CLM-GUIDE-005-CAP-001,CLM-GUIDE-005-SOURCE-001 -->

## Scope

- **Observe permission and source lifecycle.** Query non-prompting authorization where observable and receive source-generation, loss, and permission-epoch changes.

The scope of **Observe permission without prompting** ends at the native contracts and executable conditions cited below. Platform qualification, performance, retry, and delivery require their own explicit evidence.

## Prerequisites

A host UI that can decide whether and when to present a permission prompt.

## Procedure

1. Call microphone_permission_observation before opening a source when preflight information helps the UI.
2. Interpret NotObservable as neither allowed nor denied.
3. Request permission only through the host application's platform UI.
4. Prepare or start the selected source.
5. Use the open result as the authoritative decision.

## Important consequence

Do not translate `NotObservable` into either grant or denial.

## Verify the outcome

The UI distinguishes granted, denied, and unobservable preflight state, then uses source opening as the final result.

Executable evidence selected for **Observe permission without prompting** is limited to each test's recorded setup and assertions:

- `given_exhausted_capture_pool_when_acquiring_then_failure_is_observed_once` — given exhausted capture pool when acquiring then failure is observed once (`src/capture/platform/linux/pipewire.rs:1855`; `test-bcfd12a436362de05085`).
- `given_full_dispatch_ring_when_producer_pushes_then_failure_is_observed_once` — given full dispatch ring when producer pushes then failure is observed once (`src/capture/platform/linux/pipewire.rs:1870`; `test-e32ec18234812bec720e`).
- `given_denied_permission_when_opening_input_then_capture_fails_closed` — given denied permission when opening input then capture fails closed (`src/capture/platform/macos/input.rs:377`; `test-2b664c22fd511e3c2f45`).
- `given_promptable_or_observable_permission_when_opening_input_then_native_open_decides` — given promptable or observable permission when opening input then native open decides (`src/capture/platform/macos/input.rs:393`; `test-847d3fefe4665db8dd14`).
- `given_core_audio_permission_status_when_mapped_then_denial_remains_typed` — given core audio permission status when mapped then denial remains typed (`src/capture/platform/macos/macos_tap.rs:699`; `test-afb8b00e7e6a55f6d16d`).
- `given_timeout_when_cancellation_cloned_then_late_worker_observes_it` — given timeout when cancellation cloned then late worker observes it (`src/capture/platform/windows/open_lifecycle.rs:107`; `test-ae57725d08a03ddb487e`).
- `given_authoritative_permission_when_snapshotted_then_platform_state_is_preserved` — given authoritative permission when snapshotted then platform state is preserved (`src/capture/tests.rs:428`; `test-61d68aba989969d649b0`).
- `given_capture_events_when_observed_then_snapshot_preserves_each_boundary` — given capture events when observed then snapshot preserves each boundary (`src/capture/tests.rs:138`; `test-f57c1920d9fffce76e67`).
- `given_permission_lifecycle_when_authorization_changes_then_epoch_and_kind_are_canonical` — given permission lifecycle when authorization changes then epoch and kind are canonical (`src/capture/tests.rs:470`; `test-72b7390fb29e3b3a2756`).
- `given_revoked_permission_when_snapshotted_then_revocation_and_new_epoch_are_preserved` — given revoked permission when snapshotted then revocation and new epoch are preserved (`src/capture/tests.rs:447`; `test-458fc5c9256649d9f55e`).
- `given_unclassified_backend_failure_when_snapshotted_then_permission_is_not_guessed` — given unclassified backend failure when snapshotted then permission is not guessed (`src/capture/tests.rs:412`; `test-109c3c453f0c382dcb45`).
- `given_active_capture_when_owner_is_dropped_then_backend_is_reclaimed` — given active capture when owner is dropped then backend is reclaimed (`src/capture/capture_owner.rs:567`; `test-c55d7a75628c1be024f1`).

## Failure signals

- `pocketstation::capture::authorization::CaptureError` / `PermissionDenied` — `error-d902cf4c11a93cbcb084`
- `pocketstation::capture::authorization::CaptureError` — `error-7905cc933b9eb45fe4ef`
- `pocketstation::capture::authorization::CaptureError` / `BackendInit` — `error-ffea5e00d982c5213eba`
- `pocketstation::capture::authorization::CaptureError` / `BackendSetupRequired` — `error-6e8f9f8ca8efa76ded69`
- `pocketstation::capture::authorization::CaptureError` / `BackendStatus` — `error-533b29bac30886d8c79c`
- `pocketstation::capture::authorization::CaptureError` / `CaptureWorkerPanicked` — `error-01c4b3cce2fa1669ee13`
- `pocketstation::capture::authorization::CaptureError` / `InvalidRuntimeEventCapacity` — `error-c683702117e27ad45f33`
- `pocketstation::capture::authorization::CaptureError` / `InvalidStreamCapacity` — `error-6167103023ec8fded812`
- `pocketstation::capture::authorization::CaptureError` / `ModeUnsupported` — `error-786199dd7e94542436f2`
- `pocketstation::capture::authorization::CaptureError` / `NotSupported` — `error-0f2fd6c6275925740175`

## API reference

- [Permissions And Source Lifecycle](/docs/concepts/permissions-and-source-lifecycle.md)
- [Permissions](/docs/platform/permissions.md)

| Public declaration | Kind | Declared purpose | Source |
|---|---|---|---|
| `pocketstation::capture::authorization::CapturePermissionLifecycle` | struct | Control-plane owner for one source's observed authorization epoch. | `src/capture/authorization.rs:183` |
| `pocketstation::capture::authorization::CapturePermissionTransition` | struct | One authoritative authorization-state transition observed by the host. | `src/capture/authorization.rs:168` |
| `pocketstation::capture::authorization::PermissionEpoch` | struct | Identifies the permission-observation generation attached to captured lineage. | `src/capture/authorization.rs:267` |
| `pocketstation::capture::authorization::PermissionObservation` | enum | Classifies the observable permission observation. | `src/capture/authorization.rs:153` |
| `observe` | function | Returns the current observation exposed by `CapturePermissionLifecycle`. | `src/capture/authorization.rs:204` |
| `observe_complete_snapshot` | function | Records an observation for complete snapshot for `SourceLifecycleRegistry`. | `src/capture/lifecycle_registry.rs:36` |
| `permission_epoch` | function | Returns the permission epoch held by `CapturePermissionLifecycle`. | `src/capture/authorization.rs:200` |
| `pocketstation::capture::authorization::CaptureError::PermissionDenied` | variant | Reports that the required permission was denied. | `src/capture/authorization.rs:301` |

## Related documentation

- [Glossary](/docs/glossary.md)
- [Linux capture](/docs/platform/linux.md)
- [Permission ownership](/docs/platform/permissions.md)
- [Permissions and source lifecycle](/docs/concepts/permissions-and-source-lifecycle.md)
- [Platform prerequisites](/docs/getting-started/platform-prerequisites.md)
- [Platform support and evidence](/docs/platform/compatibility.md)
- [PocketStation](/README.md)
- [Windows capture](/docs/platform/windows.md)

## Evidence boundary

The claims on **Observe permission without prompting** are anchored to Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `src/capture/authorization.rs:1-318` (`DIRECT`)
- `src/lib.rs:1-1129` (`DIRECT`)

For **Observe permission without prompting**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.
