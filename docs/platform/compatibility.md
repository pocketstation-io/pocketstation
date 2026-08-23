# Platform support and evidence

<!-- claims: CLM-DOC-043-CAP-001,CLM-DOC-043-CAP-002,CLM-DOC-043-CAP-003,CLM-DOC-043-CAP-004,CLM-DOC-043-CAP-005,CLM-DOC-043-SOURCE-001 -->

## Scope

- **Capture application audio.** Prepare application-scoped capture through the platform backend selected for the current target.
- **Capture microphone audio.** Select the default or identified input device and open native microphone capture.
- **Capture system audio.** Represent and open platform system-loopback capture where the selected backend implements it.
- **Observe permission and source lifecycle.** Query non-prompting authorization where observable and receive source-generation, loss, and permission-epoch changes.
- **Validate protocol and conformance boundaries.** Check ABI layout, cross-language behavior, connector vectors, and protocol compatibility against versioned fixtures.

The scope of **Platform support and evidence** ends at the native contracts and executable conditions cited below. Platform qualification, performance, retry, and delivery require their own explicit evidence.

## Implemented boundary

| Public declaration | Kind | Declared purpose | Source |
|---|---|---|---|
| `pocketstation::capture::platform::macos::input::MacosInputSource` | struct | Owns production of macos input values and its lifecycle state. | `src/capture/platform/macos/input.rs:65` |
| `pocketstation::capture::platform::macos::loopback::SystemLoopbackSource` | struct | Manages a macOS loopback capture session. | `src/capture/platform/macos/loopback.rs:53` |
| `pocketstation::capture::platform::macos::session_backend::DesktopCaptureBackend` | struct | macOS adapter from the platform-neutral Session capture contract to the existing CoreAudio/input RAII owner. | `src/capture/platform/macos/session_backend.rs:11` |
| `pocketstation::capture::platform::macos::input::discover_input_sources_native` | function | Discovers microphone input sources through the native macOS backend. | `src/capture/platform/macos/input.rs:263` |
| `pocketstation::capture::authorization::CaptureCapabilityState::Unsupported` | variant | Reports that the requested operation is unsupported. | `src/capture/authorization.rs:148` |
| `pocketstation::capture::authorization::CaptureError::ModeUnsupported` | variant | Reported when the owning operation encounters mode unsupported. | `src/capture/authorization.rs:310` |
| `pocketstation::capture::authorization::CaptureError::NotSupported` | variant | Reported when the owning operation encounters not supported. | `src/capture/authorization.rs:292` |
| `pocketstation::capture::authorization::SourceIdentityStrength::PlatformStableId` | variant | Represents the platform stable identifier alternative defined by `SourceIdentityStrength`. | `src/capture/authorization.rs:262` |
| `pocketstation::capture::events::CaptureRuntimeFailureClass::PlatformStatus` | variant | Reported when the owning operation encounters platform status. | `src/capture/events.rs:42` |
| `events::CaptureRuntimeFailureClass::PlatformStatus::status_code` | struct_field | Stores the status code used by `PlatformStatus`. | `src/capture/events.rs:42` |
| `authorization` | module | Explicit capture authorization evidence and open outcomes. | `src/capture/authorization.rs:1` |
| `query` | module | Control-plane source discovery queries used by the first-party CLI. | `src/capture/query.rs:1` |
| `pocketstation::capture::capture_owner::ActiveCaptureBackend` | trait | Native capture resources owned for exactly one active capture. | `src/capture/capture_owner.rs:100` |
| `pocketstation::capture::capture_owner::CallbackCaptureBackend` | trait | Platform-neutral prepare/open boundary for callback-oriented capture. | `src/capture/capture_owner.rs:83` |
| `pocketstation::capture::capture_owner::PreparedCaptureBackend` | trait | Backend state that has passed validation but has not started delivery. | `src/capture/capture_owner.rs:88` |
| `pocketstation::capture::query::SourceProvider` | trait | Implement this trait to provide source behavior to PocketStation; its methods define the preparation and runtime contract. | `src/capture/query.rs:48` |
| `pocketstation::capture::authorization::CaptureAuthorizationSnapshot` | struct | Point-in-time authorization evidence for opening one exact capture source. | `src/capture/authorization.rs:17` |
| `pocketstation::capture::authorization::CapturePermissionLifecycle` | struct | Control-plane owner for one source's observed authorization epoch. | `src/capture/authorization.rs:183` |
| `pocketstation::capture::authorization::CapturePermissionTransition` | struct | One authoritative authorization-state transition observed by the host. | `src/capture/authorization.rs:168` |
| `pocketstation::capture::authorization::PermissionEpoch` | struct | Identifies the permission-observation generation attached to captured lineage. | `src/capture/authorization.rs:267` |

## Permission and source opening

For **Platform support and evidence**, permission observation and source opening remain separate. The host application owns prompts and selection UI. A non-prompting observation is advisory where implemented; preparation or open returns the authoritative result.

## Qualification boundary

The target-specific files, Cargo dependencies, and CI cited by **Platform support and evidence** establish implementation or build evidence only. They do not qualify every device, operating-system revision, packaging context, permission state, or physical path.

## Executable evidence

Executable evidence selected for **Platform support and evidence** is limited to each test's recorded setup and assertions:

- `given_application_mode_when_pipewire_unavailable_then_mode_unsupported_not_system_mix` — given application mode when pipewire unavailable then mode unsupported not system mix (`src/capture/platform/linux/pipewire.rs:2097`; `test-3935b20953f69bd82dab`).
- `given_process_mode_when_pipewire_unavailable_then_mode_unsupported_not_system_mix` — given process mode when pipewire unavailable then mode unsupported not system mix (`src/capture/platform/linux/pipewire.rs:2083`; `test-afbc5fb711b5d1e4c0fa`).
- `given_authoritative_permission_when_snapshotted_then_platform_state_is_preserved` — given authoritative permission when snapshotted then platform state is preserved (`src/capture/tests.rs:437`; `test-faaf69147963e5e88acc`).
- `given_capture_error_not_supported_when_displayed_then_contains_not_supported` — given capture error not supported when displayed then contains not supported (`src/capture/tests.rs:202`; `test-a9ee21553b930ba8710c`).
- `given_mode_unsupported_error_when_displayed_then_contains_not_supported` — given mode unsupported error when displayed then contains not supported (`src/capture/tests.rs:272`; `test-ac9ab01ee832591ddf63`).
- `given_runtime_event_when_sent_then_exact_identity_and_platform_status_are_retained` — given runtime event when sent then exact identity and platform status are retained (`src/capture/tests.rs:27`; `test-fb6a99eab03a46e120cd`).
- `given_active_capture_when_owner_is_dropped_then_backend_is_reclaimed` — given active capture when owner is dropped then backend is reclaimed (`src/capture/capture_owner.rs:567`; `test-fa34e5723160d56f560f`).
- `given_active_capture_when_stopped_then_backend_is_joined` — given active capture when stopped then backend is joined (`src/capture/capture_owner.rs:540`; `test-dd4aaaf6b93ddb500769`).
- `given_backend_frame_when_source_differs_from_open_identity_then_lineage_fails_closed` — given backend frame when source differs from open identity then lineage fails closed (`src/capture/capture_owner.rs:511`; `test-805d755d4acd2257ba9b`).
- `given_panicking_capture_worker_when_joined_then_typed_failure_is_returned` — given panicking capture worker when joined then typed failure is returned (`src/capture/capture_owner.rs:610`; `test-2d873f94835a177ce436`).
- `given_prepared_capture_when_opened_then_bounded_delivery_is_owned` — given prepared capture when opened then bounded delivery is owned (`src/capture/capture_owner.rs:463`; `test-a3a0d044f02b7f664bb9`).
- `given_zero_frame_capacity_when_preparing_then_backend_is_not_prepared` — given zero frame capacity when preparing then backend is not prepared (`src/capture/capture_owner.rs:590`; `test-0afbec4242ea2fad4582`).

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

The claims on **Platform support and evidence** are anchored to Git snapshot `136e74888962558aa846d3143a19136a70936f45` and these primary files:

- `src/capture/platform/mod.rs:1-7` (`DIRECT`)
- `.github/workflows/ci.yml:1-63` (`DIRECT`)

For **Platform support and evidence**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.
