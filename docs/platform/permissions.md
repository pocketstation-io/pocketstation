# Permission ownership

<!-- claims: CLM-DOC-047-CAP-001,CLM-DOC-047-SOURCE-001 -->

## Scope

- **Observe permission and source lifecycle.** Query non-prompting authorization where observable and receive source-generation, loss, and permission-epoch changes.

These statements describe repository contracts at the documented snapshot. They do not extend platform qualification, performance, retry, or delivery guarantees beyond the native API contracts and executable evidence.

## Implemented boundary

| Public declaration | Kind | Declared purpose | Source |
|---|---|---|---|
| `pocketstation::capture::authorization::CapturePermissionLifecycle` | struct | Control-plane owner for one source's observed authorization epoch. | `src/capture/authorization.rs:183` |
| `pocketstation::capture::authorization::CapturePermissionTransition` | struct | One authoritative authorization-state transition observed by the host. | `src/capture/authorization.rs:168` |
| `pocketstation::capture::authorization::PermissionEpoch` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/capture/authorization.rs:267` |
| `pocketstation::capture::authorization::PermissionObservation` | enum | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/capture/authorization.rs:153` |
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
| `pocketstation::capture::events::SourceLifecycleEventKind::PermissionRevoked` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/capture/events.rs:29` |
| `pocketstation::capture::identity::SourceState::PermissionBlocked` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/capture/identity.rs:22` |
| `authorization::CaptureAuthorizationSnapshot::os_permission` | struct_field | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/capture/authorization.rs:19` |
| `authorization::CaptureAuthorizationSnapshot::permission_epoch` | struct_field | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/capture/authorization.rs:24` |
| `authorization::CaptureError::PermissionDenied::operation` | struct_field | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/capture/authorization.rs:301` |

## Permission and source opening

Permission observation and source opening are separate. The host application owns prompts and selection UI. A non-prompting observation is advisory where implemented; preparation or open returns the authoritative result.

## Qualification boundary

Target-specific files, Cargo dependencies, or CI establish implementation or build evidence only. They do not establish that every device, operating-system revision, packaging context, permission state, or physical path was qualified.

## Executable evidence

The following test bodies are evidence only for their recorded setup:

- `given_denied_permission_when_opening_input_then_capture_fails_closed` — given denied permission when opening input then capture fails closed (`src/capture/platform/macos/input.rs:377`; `test-2b664c22fd511e3c2f45`).
- `given_promptable_or_observable_permission_when_opening_input_then_native_open_decides` — given promptable or observable permission when opening input then native open decides (`src/capture/platform/macos/input.rs:393`; `test-847d3fefe4665db8dd14`).
- `given_core_audio_permission_status_when_mapped_then_denial_remains_typed` — given core audio permission status when mapped then denial remains typed (`src/capture/platform/macos/macos_tap.rs:699`; `test-afb8b00e7e6a55f6d16d`).
- `given_authoritative_permission_when_snapshotted_then_platform_state_is_preserved` — given authoritative permission when snapshotted then platform state is preserved (`src/capture/tests.rs:428`; `test-61d68aba989969d649b0`).
- `given_permission_lifecycle_when_authorization_changes_then_epoch_and_kind_are_canonical` — given permission lifecycle when authorization changes then epoch and kind are canonical (`src/capture/tests.rs:470`; `test-72b7390fb29e3b3a2756`).
- `given_revoked_permission_when_snapshotted_then_revocation_and_new_epoch_are_preserved` — given revoked permission when snapshotted then revocation and new epoch are preserved (`src/capture/tests.rs:447`; `test-458fc5c9256649d9f55e`).
- `given_unclassified_backend_failure_when_snapshotted_then_permission_is_not_guessed` — given unclassified backend failure when snapshotted then permission is not guessed (`src/capture/tests.rs:412`; `test-109c3c453f0c382dcb45`).
- `given_bounded_audio_input_when_writes_are_invalid_or_saturated_then_ownership_is_explicit` — given bounded audio input when writes are invalid or saturated then ownership is explicit (`tests/audio_input.rs:73`; `test-795ca5a283c59dbf6066`).
- `given_hot_ownership_drops_when_source_changes_then_cleanup_remains_bounded_and_nonblocking` — given hot ownership drops when source changes then cleanup remains bounded and nonblocking (`tests/capture_callback_source_contract.rs:179`; `test-1edbc7a0f2144708b165`).
- `frame_stream_closed` — frame stream closed (`src/capture/capture_owner.rs:248`; `test-3ab763bff0cd08d4b4e1`).
- `given_active_capture_when_owner_is_dropped_then_backend_is_reclaimed` — given active capture when owner is dropped then backend is reclaimed (`src/capture/capture_owner.rs:567`; `test-c55d7a75628c1be024f1`).
- `given_active_capture_when_stopped_then_backend_is_joined` — given active capture when stopped then backend is joined (`src/capture/capture_owner.rs:540`; `test-4f65c4d2e20b5226cd4f`).

## Related documentation

- [Glossary](/docs/glossary.md)
- [Linux capture](/docs/platform/linux.md)
- [Permissions and source lifecycle](/docs/concepts/permissions-and-source-lifecycle.md)
- [Platform prerequisites](/docs/getting-started/platform-prerequisites.md)
- [Platform support and evidence](/docs/platform/compatibility.md)
- [PocketStation](/README.md)
- [Windows capture](/docs/platform/windows.md)
- [macOS capture](/docs/platform/macos.md)

## Evidence boundary

This page was verified against Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `src/capture/authorization.rs:1-318` (`DIRECT`)

A file's presence proves implementation or declaration at this snapshot. It does not by itself prove physical-device qualification, operational performance, retry safety, or behavior outside the recorded test conditions.
