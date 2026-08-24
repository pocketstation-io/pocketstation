# Permission ownership

<!-- claims: CLM-DOC-047-SCOPE-001,CLM-DOC-047-TEXT-001,CLM-DOC-047-TEXT-002,CLM-DOC-047-SOURCE-001 -->

## Scope

- **Observe permission and source lifecycle.** Query non-prompting authorization where observable and receive source-generation, loss, and permission-epoch changes.

The scope of **Permission ownership** ends at the native contracts and executable conditions cited below. Platform qualification, performance, retry, and delivery require their own explicit evidence.

## Implemented boundary

| Public declaration | Kind | Declared purpose | Source |
|---|---|---|---|
| `pocketstation::capture::authorization::CapturePermissionLifecycle` | struct | Control-plane owner for one source's observed authorization epoch. | `src/capture/authorization.rs:183` |
| `pocketstation::capture::authorization::CapturePermissionTransition` | struct | One authoritative authorization-state transition observed by the host. | `src/capture/authorization.rs:168` |
| `pocketstation::capture::authorization::PermissionEpoch` | struct | Identifies the permission-observation generation attached to captured lineage. | `src/capture/authorization.rs:267` |
| `pocketstation::capture::authorization::PermissionObservation` | enum | Classifies the observable permission observation. | `src/capture/authorization.rs:153` |
| `permission_epoch` | function | Returns the permission epoch held by `CapturePermissionLifecycle`. | `src/capture/authorization.rs:200` |
| `pocketstation::capture::authorization::CaptureError::PermissionDenied` | variant | Reports that the required permission was denied. | `src/capture/authorization.rs:301` |
| `pocketstation::capture::authorization::CaptureOpenOutcome::PermissionDenied` | variant | Reports that the required permission was denied. | `src/capture/authorization.rs:284` |
| `pocketstation::capture::authorization::PermissionObservation::Allowed` | variant | Represents the allowed alternative defined by `PermissionObservation`. | `src/capture/authorization.rs:154` |
| `pocketstation::capture::authorization::PermissionObservation::Denied` | variant | Represents the denied alternative defined by `PermissionObservation`. | `src/capture/authorization.rs:155` |
| `pocketstation::capture::authorization::PermissionObservation::NotApplicable` | variant | Represents the not applicable alternative defined by `PermissionObservation`. | `src/capture/authorization.rs:160` |
| `pocketstation::capture::authorization::PermissionObservation::NotDetermined` | variant | Represents the not determined alternative defined by `PermissionObservation`. | `src/capture/authorization.rs:157` |
| `pocketstation::capture::authorization::PermissionObservation::NotObservable` | variant | Represents the not observable alternative defined by `PermissionObservation`. | `src/capture/authorization.rs:159` |
| `pocketstation::capture::authorization::PermissionObservation::Restricted` | variant | Represents the restricted alternative defined by `PermissionObservation`. | `src/capture/authorization.rs:156` |
| `pocketstation::capture::authorization::PermissionObservation::Revoked` | variant | Represents the revoked alternative defined by `PermissionObservation`. | `src/capture/authorization.rs:158` |
| `pocketstation::capture::events::SourceLifecycleEventKind::PermissionChanged` | variant | Identifies the permission changed state or stage represented by `SourceLifecycleEventKind`. | `src/capture/events.rs:28` |
| `pocketstation::capture::events::SourceLifecycleEventKind::PermissionRevoked` | variant | Identifies the permission revoked state or stage represented by `SourceLifecycleEventKind`. | `src/capture/events.rs:29` |
| `authorization::CaptureAuthorizationSnapshot::os_permission` | struct_field | Reports the operating-system permission state observed by `CaptureAuthorizationSnapshot`. | `src/capture/authorization.rs:19` |
| `authorization::CaptureAuthorizationSnapshot::permission_epoch` | struct_field | Identifies the permission-observation generation attached to `CaptureAuthorizationSnapshot`. | `src/capture/authorization.rs:24` |
| `authorization::CaptureError::PermissionDenied::operation` | struct_field | Names the operation that produced `PermissionDenied`. | `src/capture/authorization.rs:301` |
| `authorization::CapturePermissionTransition::current` | struct_field | Stores the current as a `PermissionObservation` value in `CapturePermissionTransition`. | `src/capture/authorization.rs:171` |

## Permission and source opening

For **Permission ownership**, permission observation and source opening remain separate. The host application owns prompts and selection UI. A non-prompting observation is advisory where implemented; preparation or open returns the authoritative result.

## Qualification boundary

The target-specific files, Cargo dependencies, and CI cited by **Permission ownership** establish implementation or build evidence only. They do not qualify every device, operating-system revision, packaging context, permission state, or physical path.

## Executable evidence

Executable evidence selected for **Permission ownership** is limited to each test's recorded setup and assertions:

- `given_denied_permission_when_opening_input_then_capture_fails_closed` — given denied permission when opening input then capture fails closed (`src/capture/platform/macos/input.rs:384`; `test-93f56a3510497f49f523`).
- `given_promptable_or_observable_permission_when_opening_input_then_native_open_decides` — given promptable or observable permission when opening input then native open decides (`src/capture/platform/macos/input.rs:400`; `test-136298dd50a44f77d3ac`).
- `given_core_audio_permission_status_when_mapped_then_denial_remains_typed` — given core audio permission status when mapped then denial remains typed (`src/capture/platform/macos/macos_tap.rs:699`; `test-052dbf5299c7bb5e6456`).
- `given_authoritative_permission_when_snapshotted_then_platform_state_is_preserved` — given authoritative permission when snapshotted then platform state is preserved (`src/capture/tests.rs:437`; `test-faaf69147963e5e88acc`).
- `given_permission_lifecycle_when_authorization_changes_then_epoch_and_kind_are_canonical` — given permission lifecycle when authorization changes then epoch and kind are canonical (`src/capture/tests.rs:479`; `test-e62afd814c0b32723785`).
- `given_revoked_permission_when_snapshotted_then_revocation_and_new_epoch_are_preserved` — given revoked permission when snapshotted then revocation and new epoch are preserved (`src/capture/tests.rs:456`; `test-a9baea9d903dfd343a6e`).
- `given_unclassified_backend_failure_when_snapshotted_then_permission_is_not_guessed` — given unclassified backend failure when snapshotted then permission is not guessed (`src/capture/tests.rs:421`; `test-8540d7d4e458158fb98e`).
- `given_bounded_audio_input_when_writes_are_invalid_or_saturated_then_ownership_is_explicit` — given bounded audio input when writes are invalid or saturated then ownership is explicit (`tests/audio_input.rs:73`; `test-5a764dda823599c553f3`).
- `given_hot_ownership_drops_when_source_changes_then_cleanup_remains_bounded_and_nonblocking` — given hot ownership drops when source changes then cleanup remains bounded and nonblocking (`tests/capture_callback_source_contract.rs:179`; `test-5dadae151a5633685820`).
- `given_active_capture_when_owner_is_dropped_then_backend_is_reclaimed` — given active capture when owner is dropped then backend is reclaimed (`src/capture/capture_owner.rs:567`; `test-fa34e5723160d56f560f`).
- `given_active_capture_when_stopped_then_backend_is_joined` — given active capture when stopped then backend is joined (`src/capture/capture_owner.rs:540`; `test-dd4aaaf6b93ddb500769`).
- `given_backend_frame_when_source_differs_from_open_identity_then_lineage_fails_closed` — given backend frame when source differs from open identity then lineage fails closed (`src/capture/capture_owner.rs:511`; `test-805d755d4acd2257ba9b`).

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

The claims on **Permission ownership** are anchored to Git snapshot `136e74888962558aa846d3143a19136a70936f45` and these primary files:

- `src/capture/authorization.rs:1-1` (`DECLARED`)

For **Permission ownership**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.
