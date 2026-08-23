# Platform support and evidence

<!-- claims: CLM-DOC-043-CAP-001,CLM-DOC-043-CAP-002,CLM-DOC-043-CAP-003,CLM-DOC-043-CAP-004,CLM-DOC-043-CAP-005,CLM-DOC-043-SOURCE-001 -->

## Scope

- **Capture application audio.** Prepare application-scoped capture through the platform backend selected for the current target.
- **Capture microphone audio.** Select the default or identified input device and open native microphone capture.
- **Capture system audio.** Represent and open platform system-loopback capture where the selected backend implements it.
- **Observe permission and source lifecycle.** Query non-prompting authorization where observable and receive source-generation, loss, and permission-epoch changes.
- **Validate protocol and conformance boundaries.** Check ABI layout, cross-language behavior, connector vectors, and protocol compatibility against versioned fixtures.

These statements describe repository contracts at the documented snapshot. They do not extend platform qualification, performance, retry, or delivery guarantees beyond the native API contracts and executable evidence.

## Implemented boundary

| Public declaration | Kind | Declared purpose | Source |
|---|---|---|---|
| `pocketstation::capture::authorization::CaptureCapabilityState::Unsupported` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/capture/authorization.rs:148` |
| `pocketstation::capture::authorization::CaptureError::ModeUnsupported` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/capture/authorization.rs:310` |
| `pocketstation::capture::authorization::CaptureError::NotSupported` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/capture/authorization.rs:292` |
| `pocketstation::capture::authorization::SourceIdentityStrength::PlatformStableId` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/capture/authorization.rs:262` |
| `pocketstation::capture::events::CaptureRuntimeFailureClass::PlatformStatus` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/capture/events.rs:42` |
| `pocketstation::capture::selection::SelectorPersistenceScope::PlatformIdentity` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/capture/selection.rs:78` |
| `events::CaptureRuntimeFailureClass::PlatformStatus::status_code` | struct_field | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/capture/events.rs:42` |
| `identity::StableSourceId::platform` | struct_field | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/capture/identity.rs:27` |
| `authorization` | module | Explicit capture authorization evidence and open outcomes. | `src/capture/authorization.rs:1` |
| `identity` | module | Stable source identity and source discovery state. | `src/capture/identity.rs:1` |
| `query` | module | Control-plane source discovery queries used by the first-party CLI. | `src/capture/query.rs:1` |
| `selection` | module | Capture selection semantics; control-plane only. | `src/capture/selection.rs:1` |
| `pocketstation::capture::capture_owner::ActiveCaptureBackend` | trait | Native capture resources owned for exactly one active capture. | `src/capture/capture_owner.rs:100` |
| `pocketstation::capture::capture_owner::CallbackCaptureBackend` | trait | Platform-neutral prepare/open boundary for callback-oriented capture. | `src/capture/capture_owner.rs:83` |
| `pocketstation::capture::capture_owner::PreparedCaptureBackend` | trait | Backend state that has passed validation but has not started delivery. | `src/capture/capture_owner.rs:88` |
| `pocketstation::capture::authorization::CaptureAuthorizationSnapshot` | struct | Point-in-time authorization evidence for opening one exact capture source. | `src/capture/authorization.rs:17` |
| `pocketstation::capture::authorization::CapturePermissionLifecycle` | struct | Control-plane owner for one source's observed authorization epoch. | `src/capture/authorization.rs:183` |
| `pocketstation::capture::authorization::CapturePermissionTransition` | struct | One authoritative authorization-state transition observed by the host. | `src/capture/authorization.rs:168` |
| `pocketstation::capture::capture_owner::CaptureDelivery` | struct | Callback delivery endpoints transferred to a prepared native backend. | `src/capture/capture_owner.rs:73` |
| `pocketstation::capture::capture_owner::CaptureOwnerObservations` | struct | Aggregate observations from one active capture ownership boundary. | `src/capture/capture_owner.rs:160` |

## Permission and source opening

Permission observation and source opening are separate. The host application owns prompts and selection UI. A non-prompting observation is advisory where implemented; preparation or open returns the authoritative result.

## Qualification boundary

Target-specific files, Cargo dependencies, or CI establish implementation or build evidence only. They do not establish that every device, operating-system revision, packaging context, permission state, or physical path was qualified.

## Executable evidence

The following test bodies are evidence only for their recorded setup:

- `given_application_mode_when_pipewire_unavailable_then_mode_unsupported_not_system_mix` — given application mode when pipewire unavailable then mode unsupported not system mix (`src/capture/platform/linux/pipewire.rs:2097`; `test-a19b8c36cc500e40f220`).
- `given_process_mode_when_pipewire_unavailable_then_mode_unsupported_not_system_mix` — given process mode when pipewire unavailable then mode unsupported not system mix (`src/capture/platform/linux/pipewire.rs:2083`; `test-06acab2e5df43578641f`).
- `given_authoritative_permission_when_snapshotted_then_platform_state_is_preserved` — given authoritative permission when snapshotted then platform state is preserved (`src/capture/tests.rs:428`; `test-61d68aba989969d649b0`).
- `given_capture_error_not_supported_when_displayed_then_contains_not_supported` — given capture error not supported when displayed then contains not supported (`src/capture/tests.rs:193`; `test-676aa99d320d214bccad`).
- `given_mode_unsupported_error_when_displayed_then_contains_not_supported` — given mode unsupported error when displayed then contains not supported (`src/capture/tests.rs:263`; `test-ef57017ec533dcedac64`).
- `given_runtime_event_when_sent_then_exact_identity_and_platform_status_are_retained` — given runtime event when sent then exact identity and platform status are retained (`src/capture/tests.rs:18`; `test-11b326c09cc37ec133a0`).
- `frame_stream_closed` — frame stream closed (`src/capture/capture_owner.rs:248`; `test-3ab763bff0cd08d4b4e1`).
- `given_active_capture_when_owner_is_dropped_then_backend_is_reclaimed` — given active capture when owner is dropped then backend is reclaimed (`src/capture/capture_owner.rs:567`; `test-c55d7a75628c1be024f1`).
- `given_active_capture_when_stopped_then_backend_is_joined` — given active capture when stopped then backend is joined (`src/capture/capture_owner.rs:540`; `test-4f65c4d2e20b5226cd4f`).
- `given_backend_frame_when_source_differs_from_open_identity_then_lineage_fails_closed` — given backend frame when source differs from open identity then lineage fails closed (`src/capture/capture_owner.rs:511`; `test-a8dbef4f3b61c752ce0e`).
- `given_panicking_capture_worker_when_joined_then_typed_failure_is_returned` — given panicking capture worker when joined then typed failure is returned (`src/capture/capture_owner.rs:610`; `test-889c6cfb54cc924fc2b4`).
- `given_prepared_capture_when_opened_then_bounded_delivery_is_owned` — given prepared capture when opened then bounded delivery is owned (`src/capture/capture_owner.rs:463`; `test-8de0974346f9110044c2`).

## Related documentation

- [Glossary](/docs/glossary.md)
- [PocketStation](/README.md)
- [Behavior evidence index](/docs/reference/behavior-evidence.md)
- [Rust API reference](/docs/reference/rust-api.md)
- [Linux capture](/docs/platform/linux.md)
- [Platform prerequisites](/docs/getting-started/platform-prerequisites.md)
- [Windows capture](/docs/platform/windows.md)
- [macOS capture](/docs/platform/macos.md)

## Evidence boundary

This page was verified against Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `src/capture/platform/mod.rs:1-7` (`DIRECT`)
- `.github/workflows/ci.yml:1-63` (`DIRECT`)

A file's presence proves implementation or declaration at this snapshot. It does not by itself prove physical-device qualification, operational performance, retry safety, or behavior outside the recorded test conditions.
