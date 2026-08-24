# Platform prerequisites

<!-- claims: CLM-DOC-005-SCOPE-001,CLM-DOC-005-TEXT-001,CLM-DOC-005-TEXT-002,CLM-DOC-005-TEXT-003,CLM-DOC-005-SOURCE-001 -->

## Before you build

Cargo selects target dependencies and native implementation through target-specific tables and the native-capture feature. A successful compile proves that the selected source builds. It does not prove device presence, permission, or physical qualification.

## Before you run

The host application owns permission prompts and source-selection user experience. Use non-prompting observation where implemented, then use Session preparation or source opening as the authoritative typed outcome.

## Verify the environment

Start with a contracts-only Cargo check. Enable the feature set you intend to ship and run available target tests. Keep build, virtual-machine, conformance, and physical-device evidence separately labeled.

## Scope

- **Capture application audio.** Prepare application-scoped capture through the platform backend selected for the current target.
- **Capture microphone audio.** Select the default or identified input device and open native microphone capture.
- **Capture system audio.** Represent and open platform system-loopback capture where the selected backend implements it.
- **Observe permission and source lifecycle.** Query non-prompting authorization where observable and receive source-generation, loss, and permission-epoch changes.

The scope of **Platform prerequisites** ends at the native contracts and executable conditions cited below. Platform qualification, performance, retry, and delivery require their own explicit evidence.

## Public entry points

| Public declaration | Kind | Declared purpose | Source |
|---|---|---|---|
| `pocketstation::capture::platform::macos::input::MacosInputSource` | struct | Owns production of macos input values and its lifecycle state. | `src/capture/platform/macos/input.rs:65` |
| `pocketstation::capture::platform::macos::loopback::SystemLoopbackSource` | struct | Manages a macOS loopback capture session. | `src/capture/platform/macos/loopback.rs:53` |
| `pocketstation::capture::platform::macos::session_backend::DesktopCaptureBackend` | struct | macOS adapter from the platform-neutral Session capture contract to the existing CoreAudio/input RAII owner. | `src/capture/platform/macos/session_backend.rs:11` |
| `pocketstation::capture::platform::macos::input::discover_input_sources_native` | function | Discovers microphone input sources through the native macOS backend. | `src/capture/platform/macos/input.rs:263` |
| `pocketstation::capture::authorization::SourceIdentityStrength::PlatformStableId` | variant | Represents the platform stable identifier alternative defined by `SourceIdentityStrength`. | `src/capture/authorization.rs:262` |
| `pocketstation::capture::events::CaptureRuntimeFailureClass::PlatformStatus` | variant | Classifies a failure at the platform status stage or component of `CaptureRuntimeFailureClass`. | `src/capture/events.rs:42` |
| `events::CaptureRuntimeFailureClass::PlatformStatus::status_code` | struct_field | Preserves the platform or protocol status code reported by `PlatformStatus`. | `src/capture/events.rs:42` |
| `authorization` | module | Explicit capture authorization evidence and open outcomes. | `src/capture/authorization.rs:1` |
| `query` | module | Control-plane source discovery queries used by the first-party CLI. | `src/capture/query.rs:1` |
| `pocketstation::capture::capture_owner::ActiveCaptureBackend` | trait | Native capture resources owned for exactly one active capture. | `src/capture/capture_owner.rs:100` |
| `pocketstation::capture::capture_owner::CallbackCaptureBackend` | trait | Platform-neutral prepare/open boundary for callback-oriented capture. | `src/capture/capture_owner.rs:83` |
| `pocketstation::capture::capture_owner::PreparedCaptureBackend` | trait | Backend state that has passed validation but has not started delivery. | `src/capture/capture_owner.rs:88` |
| `pocketstation::capture::query::SourceProvider` | trait | Implement this trait to provide source behavior to PocketStation; its methods define the preparation and runtime contract. | `src/capture/query.rs:48` |
| `pocketstation::capture::authorization::CaptureAuthorizationSnapshot` | struct | Point-in-time authorization evidence for opening one exact capture source. | `src/capture/authorization.rs:17` |
| `pocketstation::capture::authorization::CapturePermissionLifecycle` | struct | Control-plane owner for one source's observed authorization epoch. | `src/capture/authorization.rs:183` |
| `pocketstation::capture::authorization::CapturePermissionTransition` | struct | One authoritative authorization-state transition observed by the host. | `src/capture/authorization.rs:168` |

## Executable evidence

Executable evidence selected for **Platform prerequisites** is limited to each test's recorded setup and assertions:

- `given_authoritative_permission_when_snapshotted_then_platform_state_is_preserved` — given authoritative permission when snapshotted then platform state is preserved (`src/capture/tests.rs:437`; `test-faaf69147963e5e88acc`).
- `given_runtime_event_when_sent_then_exact_identity_and_platform_status_are_retained` — given runtime event when sent then exact identity and platform status are retained (`src/capture/tests.rs:27`; `test-fb6a99eab03a46e120cd`).
- `given_active_capture_when_owner_is_dropped_then_backend_is_reclaimed` — given active capture when owner is dropped then backend is reclaimed (`src/capture/capture_owner.rs:567`; `test-fa34e5723160d56f560f`).
- `given_active_capture_when_stopped_then_backend_is_joined` — given active capture when stopped then backend is joined (`src/capture/capture_owner.rs:540`; `test-dd4aaaf6b93ddb500769`).
- `given_backend_frame_when_source_differs_from_open_identity_then_lineage_fails_closed` — given backend frame when source differs from open identity then lineage fails closed (`src/capture/capture_owner.rs:511`; `test-805d755d4acd2257ba9b`).
- `given_panicking_capture_worker_when_joined_then_typed_failure_is_returned` — given panicking capture worker when joined then typed failure is returned (`src/capture/capture_owner.rs:610`; `test-2d873f94835a177ce436`).
- `given_prepared_capture_when_opened_then_bounded_delivery_is_owned` — given prepared capture when opened then bounded delivery is owned (`src/capture/capture_owner.rs:463`; `test-a3a0d044f02b7f664bb9`).
- `given_zero_frame_capacity_when_preparing_then_backend_is_not_prepared` — given zero frame capacity when preparing then backend is not prepared (`src/capture/capture_owner.rs:590`; `test-0afbec4242ea2fad4582`).
- `given_available_capacity_when_frame_is_sent_then_stream_preserves_frame` — given available capacity when frame is sent then stream preserves frame (`src/capture/frame_stream.rs:234`; `test-8f4bb6c6c11e1d2947a7`).
- `given_closed_start_gate_when_frame_is_sent_then_frame_is_discarded_and_counted` — given closed start gate when frame is sent then frame is discarded and counted (`src/capture/frame_stream.rs:256`; `test-a59f34c85fd9d74e587e`).
- `given_full_stream_when_frame_is_sent_then_newest_is_dropped_and_counted` — given full stream when frame is sent then newest is dropped and counted (`src/capture/frame_stream.rs:277`; `test-bb4e6d290a21c545166a`).
- `given_sender_callback_when_frame_arrives_then_stream_receives_it` — given sender callback when frame arrives then stream receives it (`src/capture/frame_stream.rs:298`; `test-698bc05f28228eb21d82`).

## Related documentation

- [Glossary](/docs/glossary.md)
- [Linux capture](/docs/platform/linux.md)
- [Platform support and evidence](/docs/platform/compatibility.md)
- [PocketStation](/README.md)
- [Windows capture](/docs/platform/windows.md)
- [macOS capture](/docs/platform/macos.md)
- [Behavior evidence index](/docs/reference/behavior-evidence.md)
- [Capture API](/docs/reference/capture.md)

## Evidence boundary

The claims on **Platform prerequisites** are anchored to Git snapshot `136e74888962558aa846d3143a19136a70936f45` and these primary files:

- `Cargo.toml:1-21` (`DIRECT`)
- `src/capture/platform/mod.rs:2-6` (`DIRECT`)

For **Platform prerequisites**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.
