# Observe permission without prompting

<!-- claims: CLM-GUIDE-005-CAP-001,CLM-GUIDE-005-SOURCE-001 -->

## Scope

- **Observe permission and source lifecycle.** Query non-prompting authorization where observable and receive source-generation, loss, and permission-epoch changes.

These statements describe repository contracts at the documented snapshot. They do not extend platform qualification, performance, retry, or delivery guarantees beyond the native API contracts and executable evidence.

## Prerequisites

Read the linked concept and confirm that target platform, Cargo features, source or provider dependencies, and application-owned permission work match this task. Keep returned typed errors and outcomes available for verification.

## Procedure

1. Call microphone_permission_observation before opening a source when preflight information helps the UI.
2. Interpret NotObservable as neither allowed nor denied.
3. Request permission only through the host application's platform UI.
4. Prepare or start the selected source.
5. Use the open result as the authoritative decision.

## APIs used

| Public declaration | Kind | Declared purpose | Source |
|---|---|---|---|
| `pocketstation::capture::authorization::CapturePermissionLifecycle` | struct | Control-plane owner for one source's observed authorization epoch. | `src/capture/authorization.rs:183` |
| `pocketstation::capture::authorization::CapturePermissionTransition` | struct | One authoritative authorization-state transition observed by the host. | `src/capture/authorization.rs:168` |
| `pocketstation::capture::authorization::PermissionEpoch` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/capture/authorization.rs:267` |
| `pocketstation::capture::authorization::PermissionObservation` | enum | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/capture/authorization.rs:153` |
| `observe` | function | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/capture/authorization.rs:204` |
| `permission_epoch` | function | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/capture/authorization.rs:200` |
| `pocketstation::capture::authorization::CaptureError::PermissionDenied` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/capture/authorization.rs:301` |
| `pocketstation::capture::authorization::CaptureOpenOutcome::PermissionDenied` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/capture/authorization.rs:284` |
| `pocketstation::capture::authorization::PermissionObservation::Allowed` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/capture/authorization.rs:154` |
| `pocketstation::capture::authorization::PermissionObservation::Denied` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/capture/authorization.rs:155` |
| `pocketstation::capture::authorization::PermissionObservation::NotApplicable` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/capture/authorization.rs:160` |
| `pocketstation::capture::authorization::PermissionObservation::NotDetermined` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/capture/authorization.rs:157` |
| `pocketstation::capture::authorization::PermissionObservation::NotObservable` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/capture/authorization.rs:159` |
| `pocketstation::capture::authorization::PermissionObservation::Restricted` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/capture/authorization.rs:156` |
| `pocketstation::capture::authorization::PermissionObservation::Revoked` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/capture/authorization.rs:158` |
| `pocketstation::capture::events::SourceLifecycleEventKind::PermissionChanged` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/capture/events.rs:28` |

## Verify the outcome

The following test bodies are evidence only for their recorded setup:

- `observe_invalid_buffer` — observe invalid buffer (`src/capture/observations.rs:80`; `test-ab5942234c3e835ede9b`).
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

## Failure signals

- `pocketstation::capture::events::CaptureRuntimeFailure` — `error-11b972ad42d5de880e06`
- `pocketstation::capture::events::CaptureRuntimeFailureClass` / `BackendClass` — `error-29e952ae7432566a9e95`
- `pocketstation::capture::authorization::CaptureError` / `CaptureWorkerPanicked` — `error-365f9b6fbda74eb0d631`
- `pocketstation::capture::authorization::CaptureError` / `PermissionDenied` — `error-38030156125346a8e892`
- `pocketstation::capture::authorization::CaptureError` / `NotSupported` — `error-3b4b5393164d9f6f12a5`
- `pocketstation::capture::events::CaptureRuntimeFailureClass` / `PlatformStatus` — `error-3c6fcc22deb2f54788ba`
- `pocketstation::capture::authorization::CaptureError` / `SourceUnavailable` — `error-71c87f975acc9e22a402`
- `pocketstation::capture::authorization::CaptureError` / `BackendSetupRequired` — `error-8db0fec69a9c7158ffdf`
- `pocketstation::capture::authorization::CaptureError` — `error-96ffe4bc4254583d1e17`
- `pocketstation::capture::events::CaptureRuntimeFailureClass` / `SourceInstanceExited` — `error-a9c0f7dfff744e9ba6b7`
- `pocketstation::capture::authorization::CaptureError` / `BackendInit` — `error-b320ea1cba2b3c8dc4c7`
- `pocketstation::capture::authorization::CaptureError` / `InvalidStreamCapacity` — `error-bcf5d4d897b6bd0784bf`

Retry only when the relevant API or error contract explicitly permits it. An error name, a transient-looking message, or a successful prior run is not retry evidence.

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

This page was verified against Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `src/capture/authorization.rs:1-318` (`DIRECT`)
- `src/lib.rs:1-1129` (`DIRECT`)

A file's presence proves implementation or declaration at this snapshot. It does not by itself prove physical-device qualification, operational performance, retry safety, or behavior outside the recorded test conditions.
