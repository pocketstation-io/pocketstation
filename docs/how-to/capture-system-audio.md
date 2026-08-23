# Capture system audio

<!-- claims: CLM-GUIDE-028-CAP-001,CLM-GUIDE-028-CAP-002,CLM-GUIDE-028-CAP-003,CLM-GUIDE-028-SOURCE-001 -->

## Scope

- **Select and resolve capture sources.** Discover capture candidates and resolve application, process, device, and system queries to stable source identities.
- **Capture system audio.** Represent and open platform system-loopback capture where the selected backend implements it.
- **Observe permission and source lifecycle.** Query non-prompting authorization where observable and receive source-generation, loss, and permission-epoch changes.

The scope of **Capture system audio** ends at the native contracts and executable conditions cited below. Platform qualification, performance, retry, and delivery require their own explicit evidence.

## Prerequisites

A target backend that implements system loopback and host permission needed by that platform path.

## Procedure

1. Build a supported system-capture query.
2. Resolve or prepare it through the source provider.
3. Attach a bounded consumer.
4. Start and observe the typed open result.
5. Keep implementation and qualification claims separate.

## Important consequence

Never fall back silently from unsupported system capture to application or microphone capture.

## Verify the outcome

The system query resolves, source preparation succeeds, and frames reach the bounded consumer with system-source lineage.

Executable evidence selected for **Capture system audio** is limited to each test's recorded setup and assertions:

- `given_default_capture_mode_when_compared_then_is_system_mix` — given default capture mode when compared then is system mix (`src/capture/tests.rs:198`; `test-151f813bf569e94c809d`).
- `given_active_capture_when_owner_is_dropped_then_backend_is_reclaimed` — given active capture when owner is dropped then backend is reclaimed (`src/capture/capture_owner.rs:567`; `test-c55d7a75628c1be024f1`).
- `given_active_capture_when_stopped_then_backend_is_joined` — given active capture when stopped then backend is joined (`src/capture/capture_owner.rs:540`; `test-4f65c4d2e20b5226cd4f`).
- `given_panicking_capture_worker_when_joined_then_typed_failure_is_returned` — given panicking capture worker when joined then typed failure is returned (`src/capture/capture_owner.rs:610`; `test-889c6cfb54cc924fc2b4`).
- `given_prepared_capture_when_opened_then_bounded_delivery_is_owned` — given prepared capture when opened then bounded delivery is owned (`src/capture/capture_owner.rs:463`; `test-8de0974346f9110044c2`).
- `given_application_mode_when_pipewire_unavailable_then_mode_unsupported_not_system_mix` — given application mode when pipewire unavailable then mode unsupported not system mix (`src/capture/platform/linux/pipewire.rs:2097`; `test-a19b8c36cc500e40f220`).
- `given_capture_mode_when_channels_selected_then_microphone_is_mono_and_output_is_stereo` — given capture mode when channels selected then microphone is mono and output is stereo (`src/capture/platform/linux/pipewire.rs:1837`; `test-c28f1242d8a2b60457db`).
- `given_exhausted_capture_pool_when_acquiring_then_failure_is_observed_once` — given exhausted capture pool when acquiring then failure is observed once (`src/capture/platform/linux/pipewire.rs:1855`; `test-bcfd12a436362de05085`).
- `given_negotiated_format_when_channel_count_changes_then_capture_fails_closed` — given negotiated format when channel count changes then capture fails closed (`src/capture/platform/linux/pipewire.rs:2066`; `test-1993ee9e15230d1f6226`).
- `given_process_mode_when_node_not_found_then_backend_init_error_not_system_mix` — given process mode when node not found then backend init error not system mix (`src/capture/platform/linux/pipewire.rs:2131`; `test-d95b10aa2227cf4f9ffb`).
- `given_process_mode_when_pipewire_unavailable_then_mode_unsupported_not_system_mix` — given process mode when pipewire unavailable then mode unsupported not system mix (`src/capture/platform/linux/pipewire.rs:2083`; `test-06acab2e5df43578641f`).
- `given_capture_before_callback_when_mapped_then_process_timestamp_preserves_delay` — given capture before callback when mapped then process timestamp preserves delay (`src/capture/platform/macos/input.rs:351`; `test-8a2ea38f6f2c1b3ffa2f`).

## Failure signals

- `pocketstation::capture::authorization::CaptureError` / `NotSupported` — `error-0f2fd6c6275925740175`
- `pocketstation::capture::authorization::CaptureError` — `error-7905cc933b9eb45fe4ef`
- `pocketstation::capture::authorization::CaptureError` / `BackendInit` — `error-ffea5e00d982c5213eba`
- `pocketstation::capture::authorization::CaptureError` / `BackendSetupRequired` — `error-6e8f9f8ca8efa76ded69`
- `pocketstation::capture::authorization::CaptureError` / `BackendStatus` — `error-533b29bac30886d8c79c`
- `pocketstation::capture::authorization::CaptureError` / `CaptureWorkerPanicked` — `error-01c4b3cce2fa1669ee13`
- `pocketstation::capture::authorization::CaptureError` / `InvalidRuntimeEventCapacity` — `error-c683702117e27ad45f33`
- `pocketstation::capture::authorization::CaptureError` / `InvalidStreamCapacity` — `error-6167103023ec8fded812`
- `pocketstation::capture::authorization::CaptureError` / `ModeUnsupported` — `error-786199dd7e94542436f2`
- `pocketstation::capture::authorization::CaptureError` / `PermissionDenied` — `error-d902cf4c11a93cbcb084`

## API reference

- [System Capture](/docs/concepts/system-capture.md)
- [Compatibility](/docs/platform/compatibility.md)

| Public declaration | Kind | Declared purpose | Source |
|---|---|---|---|
| `pocketstation::capture::platform::macos::loopback::SystemLoopbackSource` | struct | Manages a macOS loopback capture session. | `src/capture/platform/macos/loopback.rs:53` |
| `pocketstation::capture::authorization::CaptureScope::SystemMix` | variant | Selects system mix behavior for `CaptureScope`. | `src/capture/authorization.rs:252` |
| `pocketstation::capture::selection::CaptureMode::SystemMix` | variant | Selects system mix behavior for `CaptureMode`. | `src/capture/selection.rs:18` |
| `pocketstation::capture::query::SourceProvider` | trait | Implement this trait to provide source behavior to PocketStation; its methods define the preparation and runtime contract. | `src/capture/query.rs:48` |
| `pocketstation::capture::authorization::CaptureAuthorizationSnapshot` | struct | Point-in-time authorization evidence for opening one exact capture source. | `src/capture/authorization.rs:17` |
| `pocketstation::capture::authorization::CapturePermissionLifecycle` | struct | Control-plane owner for one source's observed authorization epoch. | `src/capture/authorization.rs:183` |
| `pocketstation::capture::authorization::CapturePermissionTransition` | struct | One authoritative authorization-state transition observed by the host. | `src/capture/authorization.rs:168` |
| `pocketstation::capture::authorization::PermissionEpoch` | struct | Identifies the permission-observation generation attached to captured lineage. | `src/capture/authorization.rs:267` |

## Related documentation

- [Glossary](/docs/glossary.md)
- [PocketStation](/README.md)
- [Behavior evidence index](/docs/reference/behavior-evidence.md)
- [Capture API](/docs/reference/capture.md)
- [Rust API reference](/docs/reference/rust-api.md)
- [Capture failures](/docs/errors/capture.md)
- [Linux capture](/docs/platform/linux.md)
- [Platform prerequisites](/docs/getting-started/platform-prerequisites.md)

## Evidence boundary

The claims on **Capture system audio** are anchored to Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `src/capture/query.rs:1-137` (`DIRECT`)
- `src/capture/platform/mod.rs:1-7` (`DIRECT`)

For **Capture system audio**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.
