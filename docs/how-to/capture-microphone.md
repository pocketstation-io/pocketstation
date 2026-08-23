# Capture the default microphone

<!-- claims: CLM-GUIDE-002-CAP-001,CLM-GUIDE-002-CAP-002,CLM-GUIDE-002-SOURCE-001 -->

## Scope

- **Capture microphone audio.** Select the default or identified input device and open native microphone capture.
- **Observe permission and source lifecycle.** Query non-prompting authorization where observable and receive source-generation, loss, and permission-epoch changes.

The scope of **Capture the default microphone** ends at the native contracts and executable conditions cited below. Platform qualification, performance, retry, and delivery require their own explicit evidence.

## Prerequisites

An available input device and a host application capable of owning the platform permission prompt.

## Procedure

1. Observe permission without prompting when the target exposes that operation.
2. Let the host application own any permission prompt.
3. Declare the default or identified microphone Source.
4. Attach a consumer before start.
5. Treat preparation or source opening as the authoritative result.

## Important consequence

Treat permission observation as advisory and source opening as authoritative.

## Verify the outcome

Microphone frames arrive with a distinct source and stem identity, and the source-open outcome is successful.

Executable evidence selected for **Capture the default microphone** is limited to each test's recorded setup and assertions:

- `given_capture_mode_when_channels_selected_then_microphone_is_mono_and_output_is_stereo` — given capture mode when channels selected then microphone is mono and output is stereo (`src/capture/platform/linux/pipewire.rs:1837`; `test-c28f1242d8a2b60457db`).
- `given_default_and_exact_microphones_when_contract_inspected_then_lifetimes_differ` — given default and exact microphones when contract inspected then lifetimes differ (`src/capture/tests.rs:240`; `test-4af1790482adc3c295e6`).
- `given_default_capture_mode_when_compared_then_is_system_mix` — given default capture mode when compared then is system mix (`src/capture/tests.rs:198`; `test-151f813bf569e94c809d`).
- `given_active_capture_when_owner_is_dropped_then_backend_is_reclaimed` — given active capture when owner is dropped then backend is reclaimed (`src/capture/capture_owner.rs:567`; `test-c55d7a75628c1be024f1`).
- `given_active_capture_when_stopped_then_backend_is_joined` — given active capture when stopped then backend is joined (`src/capture/capture_owner.rs:540`; `test-4f65c4d2e20b5226cd4f`).
- `given_panicking_capture_worker_when_joined_then_typed_failure_is_returned` — given panicking capture worker when joined then typed failure is returned (`src/capture/capture_owner.rs:610`; `test-889c6cfb54cc924fc2b4`).
- `given_prepared_capture_when_opened_then_bounded_delivery_is_owned` — given prepared capture when opened then bounded delivery is owned (`src/capture/capture_owner.rs:463`; `test-8de0974346f9110044c2`).
- `given_exhausted_capture_pool_when_acquiring_then_failure_is_observed_once` — given exhausted capture pool when acquiring then failure is observed once (`src/capture/platform/linux/pipewire.rs:1855`; `test-bcfd12a436362de05085`).
- `given_negotiated_format_when_channel_count_changes_then_capture_fails_closed` — given negotiated format when channel count changes then capture fails closed (`src/capture/platform/linux/pipewire.rs:2066`; `test-1993ee9e15230d1f6226`).
- `given_capture_before_callback_when_mapped_then_process_timestamp_preserves_delay` — given capture before callback when mapped then process timestamp preserves delay (`src/capture/platform/macos/input.rs:351`; `test-8a2ea38f6f2c1b3ffa2f`).
- `given_capture_before_process_epoch_when_mapped_then_timestamp_is_earliest_representable` — given capture before process epoch when mapped then timestamp is earliest representable (`src/capture/platform/macos/input.rs:364`; `test-9519b3f93a4a0e689bcc`).
- `given_denied_permission_when_opening_input_then_capture_fails_closed` — given denied permission when opening input then capture fails closed (`src/capture/platform/macos/input.rs:377`; `test-2b664c22fd511e3c2f45`).

## Failure signals

- `pocketstation::capture::authorization::CaptureError` — `error-7905cc933b9eb45fe4ef`
- `pocketstation::capture::authorization::CaptureError` / `BackendInit` — `error-ffea5e00d982c5213eba`
- `pocketstation::capture::authorization::CaptureError` / `BackendSetupRequired` — `error-6e8f9f8ca8efa76ded69`
- `pocketstation::capture::authorization::CaptureError` / `BackendStatus` — `error-533b29bac30886d8c79c`
- `pocketstation::capture::authorization::CaptureError` / `CaptureWorkerPanicked` — `error-01c4b3cce2fa1669ee13`
- `pocketstation::capture::authorization::CaptureError` / `InvalidRuntimeEventCapacity` — `error-c683702117e27ad45f33`
- `pocketstation::capture::authorization::CaptureError` / `InvalidStreamCapacity` — `error-6167103023ec8fded812`
- `pocketstation::capture::authorization::CaptureError` / `ModeUnsupported` — `error-786199dd7e94542436f2`
- `pocketstation::capture::authorization::CaptureError` / `NotSupported` — `error-0f2fd6c6275925740175`
- `pocketstation::capture::authorization::CaptureError` / `PermissionDenied` — `error-d902cf4c11a93cbcb084`

## API reference

- [Microphone Capture](/docs/concepts/microphone-capture.md)
- [Permissions](/docs/platform/permissions.md)

| Public declaration | Kind | Declared purpose | Source |
|---|---|---|---|
| `pocketstation::capture::query::SourceProvider` | trait | Implement this trait to provide source behavior to PocketStation; its methods define the preparation and runtime contract. | `src/capture/query.rs:48` |
| `pocketstation::capture::authorization::CaptureAuthorizationSnapshot` | struct | Point-in-time authorization evidence for opening one exact capture source. | `src/capture/authorization.rs:17` |
| `pocketstation::capture::authorization::CapturePermissionLifecycle` | struct | Control-plane owner for one source's observed authorization epoch. | `src/capture/authorization.rs:183` |
| `pocketstation::capture::authorization::CapturePermissionTransition` | struct | One authoritative authorization-state transition observed by the host. | `src/capture/authorization.rs:168` |
| `pocketstation::capture::authorization::PermissionEpoch` | struct | Identifies the permission-observation generation attached to captured lineage. | `src/capture/authorization.rs:267` |
| `pocketstation::capture::events::CaptureRuntimeFailure` | struct | Reports a capture runtime failure. | `src/capture/events.rs:47` |
| `pocketstation::capture::events::SourceGeneration` | struct | Identifies one appearance generation of a capture source across loss and reappearance. | `src/capture/events.rs:12` |
| `pocketstation::capture::events::SourceRuntimeEventObservationHandle` | struct | Owns bounded access to source runtime event observation. | `src/capture/events.rs:200` |

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

The claims on **Capture the default microphone** are anchored to Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `examples/product_quickstart.rs:1-61` (`DIRECT`)

For **Capture the default microphone**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.
