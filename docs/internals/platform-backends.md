# Platform backend boundary

<!-- claims: CLM-DOC-053-SCOPE-001,CLM-DOC-053-TEXT-001,CLM-DOC-053-TEXT-002,CLM-DOC-053-TEXT-003,CLM-DOC-053-TEXT-004,CLM-DOC-053-SOURCE-001 -->

## Scope

- **Capture application audio.** Prepare application-scoped capture through the platform backend selected for the current target.
- **Capture microphone audio.** Select the default or identified input device and open native microphone capture.
- **Capture system audio.** Represent and open platform system-loopback capture where the selected backend implements it.

The scope of **Platform backend boundary** ends at the native contracts and executable conditions cited below. Platform qualification, performance, retry, and delivery require their own explicit evidence.

## Ownership map

- `src/capture/platform/mod.rs` owns part of this boundary.

## Compiler-visible surface

| Public declaration | Kind | Declared purpose | Source |
|---|---|---|---|
| `pocketstation::capture::platform::macos::session_backend::DesktopCaptureBackend` | struct | macOS adapter from the platform-neutral Session capture contract to the existing CoreAudio/input RAII owner. | `src/capture/platform/macos/session_backend.rs:11` |
| `pocketstation::capture::capture_owner::ActiveCaptureBackend` | trait | Native capture resources owned for exactly one active capture. | `src/capture/capture_owner.rs:100` |
| `pocketstation::capture::capture_owner::CallbackCaptureBackend` | trait | Platform-neutral prepare/open boundary for callback-oriented capture. | `src/capture/capture_owner.rs:83` |
| `pocketstation::capture::capture_owner::PreparedCaptureBackend` | trait | Backend state that has passed validation but has not started delivery. | `src/capture/capture_owner.rs:88` |
| `pocketstation::capture::platform::macos::input::MacosInputSource` | struct | Owns production of macos input values and its lifecycle state. | `src/capture/platform/macos/input.rs:65` |
| `pocketstation::capture::platform::macos::loopback::SystemLoopbackSource` | struct | Manages a macOS loopback capture session. | `src/capture/platform/macos/loopback.rs:53` |
| `ActiveCaptureBackend::observation_handle` | function | Returns a handle for reading observations from `ActiveCaptureBackend`. | `src/capture/capture_owner.rs:107` |
| `ActiveCaptureBackend::observations` | function | Returns the observations exposed by `ActiveCaptureBackend`. | `src/capture/capture_owner.rs:109` |
| `ActiveCaptureBackend::source_id` | function | Resolved native source identity for every frame emitted by this open. | `src/capture/capture_owner.rs:105` |
| `ActiveCaptureBackend::stop_and_join` | function | Stops `ActiveCaptureBackend`, joins its worker, and returns the terminal result. | `src/capture/capture_owner.rs:111` |
| `CallbackCaptureBackend::prepare` | function | Prepares resources required by `CallbackCaptureBackend`. | `src/capture/capture_owner.rs:84` |
| `PreparedCaptureBackend::open` | function | Opens the resource represented by `PreparedCaptureBackend`. | `src/capture/capture_owner.rs:89` |
| `pocketstation::capture::platform::macos::input::discover_input_sources_native` | function | Discovers microphone input sources through the native macOS backend. | `src/capture/platform/macos/input.rs:263` |
| `CaptureOwnerObservations::backend` | struct_field | Stores the backend as a `CaptureObservations` value in `CaptureOwnerObservations`. | `src/capture/capture_owner.rs:161` |
| `query` | module | Control-plane source discovery queries used by the first-party CLI. | `src/capture/query.rs:1` |
| `pocketstation::capture::query::SourceProvider` | trait | Implement this trait to provide source behavior to PocketStation; its methods define the preparation and runtime contract. | `src/capture/query.rs:48` |
| `pocketstation::capture::capture_owner::CaptureDelivery` | struct | Callback delivery endpoints transferred to a prepared native backend. | `src/capture/capture_owner.rs:73` |
| `pocketstation::capture::capture_owner::CaptureLineageSeed` | struct | Stable session and stem identity assigned before an exact source is opened. | `src/capture/capture_owner.rs:25` |
| `pocketstation::capture::capture_owner::CaptureObservationReceipt` | struct | Retains the identity and observation access returned for capture observation. | `src/capture/capture_owner.rs:167` |
| `pocketstation::capture::capture_owner::CaptureOpenMetadata` | struct | Authoritative lineage state established only after native capture opens. | `src/capture/capture_owner.rs:49` |
| `pocketstation::capture::capture_owner::CaptureOwner` | struct | RAII owner for native capture, its bounded frame stream, and runtime events. | `src/capture/capture_owner.rs:194` |
| `pocketstation::capture::capture_owner::CaptureOwnerObservations` | struct | Aggregate observations from one active capture ownership boundary. | `src/capture/capture_owner.rs:160` |
| `pocketstation::capture::capture_owner::CapturePrepareRequest` | struct | Setup-time request for one bounded callback-oriented capture owner. | `src/capture/capture_owner.rs:61` |
| `pocketstation::capture::capture_owner::CaptureStopOutcome` | struct | Final observations returned only after backend stop and join complete. | `src/capture/capture_owner.rs:185` |

## Observed implementation patterns

- `typed_error` — `src/capture/platform/macos/session_backend.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `buffer_pool` — `tests/capture_callback_source_contract.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `bounded_queue` — `src/capture/platform/windows/windows.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `clock_correlation` — `src/capture/capture_owner.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `buffer_pool` — `src/capture/frame_stream.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `typed_error` — `src/capture/platform/macos/loopback.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `buffer_pool` — `benches/capture_to_stream.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `bounded_queue` — `src/capture/events.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `typed_error` — `src/capture/authorization.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `buffer_pool` — `src/capture/platform/macos/input.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `typed_error` — `src/capture/platform/macos/input.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `buffer_pool` — `tests/capture_hot_path_alloc.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `typed_error` — `src/capture/capture_owner.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `buffer_pool` — `src/capture/platform/macos/macos_tap.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `bounded_queue` — `src/capture/platform/macos/loopback.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `bounded_queue` — `src/capture/platform/linux/pipewire.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `buffer_pool` — `src/capture/platform/linux/pipewire.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `clock_correlation` — `native/macos/asp/Plugin.cpp` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `typed_error` — `src/capture/platform/windows/windows.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `typed_error` — `src/capture/platform/windows/open_lifecycle.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `typed_error` — `src/capture/frame_stream.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `buffer_pool` — `src/capture/capture_owner.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `typed_error` — `src/capture/platform/linux/session_backend.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `typed_error` — `tests/external_source.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `clock_correlation` — `src/capture/mod.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `buffer_pool` — `benches/captured_frame_stream.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `typed_error` — `src/capture/platform/macos/mod.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `typed_error` — `tests/audio_input.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `buffer_pool` — `src/capture/platform/macos/loopback.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `typed_error` — `src/capture/platform/windows/session_backend.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).

## Behavioral evidence

Executable evidence selected for **Platform backend boundary** is limited to each test's recorded setup and assertions:

- `given_active_capture_when_owner_is_dropped_then_backend_is_reclaimed` — given active capture when owner is dropped then backend is reclaimed (`src/capture/capture_owner.rs:567`; `test-fa34e5723160d56f560f`).
- `given_active_capture_when_stopped_then_backend_is_joined` — given active capture when stopped then backend is joined (`src/capture/capture_owner.rs:540`; `test-dd4aaaf6b93ddb500769`).
- `given_backend_frame_when_source_differs_from_open_identity_then_lineage_fails_closed` — given backend frame when source differs from open identity then lineage fails closed (`src/capture/capture_owner.rs:511`; `test-805d755d4acd2257ba9b`).
- `given_zero_frame_capacity_when_preparing_then_backend_is_not_prepared` — given zero frame capacity when preparing then backend is not prepared (`src/capture/capture_owner.rs:590`; `test-0afbec4242ea2fad4582`).
- `given_process_mode_when_node_not_found_then_backend_init_error_not_system_mix` — given process mode when node not found then backend init error not system mix (`src/capture/platform/linux/pipewire.rs:2131`; `test-b704602af68d2c7a0b53`).
- `given_native_host_time_when_normalized_then_process_clock_boundary_is_comparable` — given native host time when normalized then process clock boundary is comparable (`src/capture/platform/macos/macos_tap.rs:721`; `test-0426336f8a6e2c5cc82f`).
- `given_macos_backend_when_type_checked_then_callback_contract_is_implemented` — given macos backend when type checked then callback contract is implemented (`src/capture/platform/macos/session_backend.rs:65`; `test-deb69184cf289583eed0`).
- `given_generic_hresult_when_classified_then_exact_failure_remains_backend_failure` — given generic hresult when classified then exact failure remains backend failure (`src/capture/platform/windows/runtime_lifecycle.rs:44`; `test-10921e0fdc49f47b04b5`).
- `given_authoritative_permission_when_snapshotted_then_platform_state_is_preserved` — given authoritative permission when snapshotted then platform state is preserved (`src/capture/tests.rs:437`; `test-faaf69147963e5e88acc`).
- `given_backend_failure_publisher_when_owner_ends_then_event_and_closure_are_observable` — given backend failure publisher when owner ends then event and closure are observable (`src/capture/tests.rs:114`; `test-a12e485cf3da74e463dd`).
- `given_capture_events_when_observed_then_snapshot_preserves_each_boundary` — given capture events when observed then snapshot preserves each boundary (`src/capture/tests.rs:147`; `test-1c6d0316ef2c3a9ff8c9`).
- `given_runtime_event_when_sent_then_exact_identity_and_platform_status_are_retained` — given runtime event when sent then exact identity and platform status are retained (`src/capture/tests.rs:27`; `test-fb6a99eab03a46e120cd`).

## Stability boundary

**Platform backend boundary** describes internal ownership. Its private module layout is not a compatibility promise; compatibility comes from exported Rust declarations, the C header, manifests, error codes, and explicit compatibility artifacts.

## Related documentation

- [Glossary](/docs/glossary.md)
- [Linux capture](/docs/platform/linux.md)
- [Platform prerequisites](/docs/getting-started/platform-prerequisites.md)
- [Platform support and evidence](/docs/platform/compatibility.md)
- [PocketStation](/README.md)
- [Windows capture](/docs/platform/windows.md)
- [macOS capture](/docs/platform/macos.md)
- [Behavior evidence index](/docs/reference/behavior-evidence.md)

## Evidence boundary

The claims on **Platform backend boundary** are anchored to Git snapshot `136e74888962558aa846d3143a19136a70936f45` and these primary files:

- `src/capture/platform/mod.rs:2-6` (`DIRECT`)

For **Platform backend boundary**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.
