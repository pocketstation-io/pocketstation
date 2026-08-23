# Platform backend boundary

<!-- claims: CLM-DOC-053-CAP-001,CLM-DOC-053-CAP-002,CLM-DOC-053-CAP-003,CLM-DOC-053-SOURCE-001 -->

## Scope

- **Capture application audio.** Prepare application-scoped capture through the platform backend selected for the current target.
- **Capture microphone audio.** Select the default or identified input device and open native microphone capture.
- **Capture system audio.** Represent and open platform system-loopback capture where the selected backend implements it.

These statements describe repository contracts at the documented snapshot. They do not extend platform qualification, performance, retry, or delivery guarantees beyond the native API contracts and executable evidence.

## Ownership map

- `src/capture/platform/mod.rs` owns part of this boundary.

## Compiler-visible surface

| Public declaration | Kind | Declared purpose | Source |
|---|---|---|---|
| `pocketstation::capture::capture_owner::ActiveCaptureBackend` | trait | Native capture resources owned for exactly one active capture. | `src/capture/capture_owner.rs:100` |
| `pocketstation::capture::capture_owner::CallbackCaptureBackend` | trait | Platform-neutral prepare/open boundary for callback-oriented capture. | `src/capture/capture_owner.rs:83` |
| `pocketstation::capture::capture_owner::PreparedCaptureBackend` | trait | Backend state that has passed validation but has not started delivery. | `src/capture/capture_owner.rs:88` |
| `ActiveCaptureBackend::source_id` | function | Resolved native source identity for every frame emitted by this open. | `src/capture/capture_owner.rs:105` |
| `ActiveCaptureBackend::observation_handle` | function | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/capture/capture_owner.rs:107` |
| `ActiveCaptureBackend::observations` | function | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/capture/capture_owner.rs:109` |
| `ActiveCaptureBackend::stop_and_join` | function | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/capture/capture_owner.rs:111` |
| `CallbackCaptureBackend::prepare` | function | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/capture/capture_owner.rs:84` |
| `PreparedCaptureBackend::open` | function | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/capture/capture_owner.rs:89` |
| `pocketstation::capture::authorization::CaptureError::BackendInit` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/capture/authorization.rs:294` |
| `pocketstation::capture::authorization::CaptureError::BackendSetupRequired` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/capture/authorization.rs:296` |
| `pocketstation::capture::authorization::CaptureError::BackendStatus` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/capture/authorization.rs:303` |
| `pocketstation::capture::authorization::CaptureOpenOutcome::BackendFailed` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/capture/authorization.rs:286` |
| `pocketstation::capture::authorization::SourceIdentityStrength::PlatformStableId` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/capture/authorization.rs:262` |
| `pocketstation::capture::events::CaptureRuntimeFailureClass::BackendClass` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/capture/events.rs:43` |
| `pocketstation::capture::events::CaptureRuntimeFailureClass::PlatformStatus` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/capture/events.rs:42` |
| `pocketstation::capture::events::SourceRuntimeEvent::BackendFailure` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/capture/events.rs:60` |
| `pocketstation::capture::selection::SelectorPersistenceScope::PlatformIdentity` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/capture/selection.rs:78` |
| `CaptureOwnerObservations::backend` | struct_field | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/capture/capture_owner.rs:161` |
| `authorization::CaptureError::BackendSetupRequired::action` | struct_field | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/capture/authorization.rs:298` |
| `authorization::CaptureError::BackendSetupRequired::backend` | struct_field | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/capture/authorization.rs:297` |
| `authorization::CaptureError::BackendStatus::operation` | struct_field | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/capture/authorization.rs:304` |
| `authorization::CaptureError::BackendStatus::status_code` | struct_field | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/capture/authorization.rs:305` |
| `events::CaptureRuntimeFailureClass::BackendClass::class` | struct_field | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/capture/events.rs:43` |

## Observed implementation patterns

- `typed_error` — `src/capture/platform/macos/session_backend.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `clock_correlation` — `src/capture/platform/macos/macos_tap.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `bounded_queue` — `src/capture/platform/windows/windows.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `buffer_pool` — `src/capture/frame_stream.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `typed_error` — `src/capture/platform/macos/loopback.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `buffer_pool` — `benches/capture_to_stream.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `buffer_pool` — `src/capture/observations.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `bounded_queue` — `src/capture/events.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `typed_error` — `src/capture/authorization.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `buffer_pool` — `src/capture/platform/macos/input.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `typed_error` — `src/capture/platform/macos/input.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `buffer_pool` — `tests/capture_hot_path_alloc.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `typed_error` — `src/capture/capture_owner.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `clock_correlation` — `src/capture/timeline.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `clock_correlation` — `src/capture/tests.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `buffer_pool` — `src/capture/tests.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `bounded_queue` — `src/capture/mod.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `buffer_pool` — `src/capture/platform/macos/macos_tap.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `bounded_queue` — `src/capture/platform/macos/loopback.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `bounded_queue` — `src/capture/platform/linux/pipewire.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `buffer_pool` — `src/capture/platform/linux/pipewire.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `typed_error` — `src/capture/platform/windows/windows.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `typed_error` — `src/capture/platform/windows/open_lifecycle.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `buffer_pool` — `src/capture/capture_owner.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `typed_error` — `src/capture/platform/linux/session_backend.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `clock_correlation` — `src/capture/mod.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `buffer_pool` — `benches/captured_frame_stream.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `typed_error` — `tests/audio_input.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `bounded_queue` — `src/capture/capture_owner.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `buffer_pool` — `src/capture/platform/macos/loopback.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).

## Behavioral evidence

The following test bodies are evidence only for their recorded setup:

- `given_active_capture_when_owner_is_dropped_then_backend_is_reclaimed` — given active capture when owner is dropped then backend is reclaimed (`src/capture/capture_owner.rs:567`; `test-c55d7a75628c1be024f1`).
- `given_active_capture_when_stopped_then_backend_is_joined` — given active capture when stopped then backend is joined (`src/capture/capture_owner.rs:540`; `test-4f65c4d2e20b5226cd4f`).
- `given_backend_frame_when_source_differs_from_open_identity_then_lineage_fails_closed` — given backend frame when source differs from open identity then lineage fails closed (`src/capture/capture_owner.rs:511`; `test-a8dbef4f3b61c752ce0e`).
- `given_zero_frame_capacity_when_preparing_then_backend_is_not_prepared` — given zero frame capacity when preparing then backend is not prepared (`src/capture/capture_owner.rs:590`; `test-f42d54d3bd1632c2ccfa`).
- `publish_backend_failure` — publish backend failure (`src/capture/events.rs:278`; `test-d6ee1878cb3cf2d3f452`).
- `given_process_mode_when_node_not_found_then_backend_init_error_not_system_mix` — given process mode when node not found then backend init error not system mix (`src/capture/platform/linux/pipewire.rs:2131`; `test-d95b10aa2227cf4f9ffb`).
- `given_native_host_time_when_normalized_then_process_clock_boundary_is_comparable` — given native host time when normalized then process clock boundary is comparable (`src/capture/platform/macos/macos_tap.rs:721`; `test-e4f9e412f0b3f6d23f1b`).
- `given_macos_backend_when_type_checked_then_callback_contract_is_implemented` — given macos backend when type checked then callback contract is implemented (`src/capture/platform/macos/session_backend.rs:65`; `test-e20b48b4ecfc231f473f`).
- `given_generic_hresult_when_classified_then_exact_failure_remains_backend_failure` — given generic hresult when classified then exact failure remains backend failure (`src/capture/platform/windows/runtime_lifecycle.rs:44`; `test-b4ce76d406f5e96ede7a`).
- `given_authoritative_permission_when_snapshotted_then_platform_state_is_preserved` — given authoritative permission when snapshotted then platform state is preserved (`src/capture/tests.rs:428`; `test-61d68aba989969d649b0`).
- `given_backend_failure_publisher_when_owner_ends_then_event_and_closure_are_observable` — given backend failure publisher when owner ends then event and closure are observable (`src/capture/tests.rs:105`; `test-16dd12044eae17ce2455`).
- `given_capture_events_when_observed_then_snapshot_preserves_each_boundary` — given capture events when observed then snapshot preserves each boundary (`src/capture/tests.rs:138`; `test-f57c1920d9fffce76e67`).

## Stability boundary

This page explains internals. Public compatibility comes from exported Rust declarations, the C header, manifests, error codes, and explicit compatibility artifacts—not private module layout.

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

This page was verified against Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `src/capture/platform/mod.rs:1-7` (`DIRECT`)

A file's presence proves implementation or declaration at this snapshot. It does not by itself prove physical-device qualification, operational performance, retry safety, or behavior outside the recorded test conditions.
