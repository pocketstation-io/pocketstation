# macOS capture

<!-- claims: CLM-DOC-044-CAP-001,CLM-DOC-044-CAP-002,CLM-DOC-044-CAP-003,CLM-DOC-044-CAP-004,CLM-DOC-044-SOURCE-001 -->

## Scope

- **Capture application audio.** Prepare application-scoped capture through the platform backend selected for the current target.
- **Capture microphone audio.** Select the default or identified input device and open native microphone capture.
- **Capture system audio.** Represent and open platform system-loopback capture where the selected backend implements it.
- **Observe permission and source lifecycle.** Query non-prompting authorization where observable and receive source-generation, loss, and permission-epoch changes.

These statements describe repository contracts at the documented snapshot. They do not extend platform qualification, performance, retry, or delivery guarantees beyond the native API contracts and executable evidence.

## Implemented boundary

| Public declaration | Kind | Declared purpose | Source |
|---|---|---|---|
| `pocketstation::capture::capture_owner::ActiveCaptureBackend` | trait | Native capture resources owned for exactly one active capture. | `src/capture/capture_owner.rs:100` |
| `pocketstation::capture::capture_owner::CallbackCaptureBackend` | trait | Platform-neutral prepare/open boundary for callback-oriented capture. | `src/capture/capture_owner.rs:83` |
| `pocketstation::capture::capture_owner::PreparedCaptureBackend` | trait | Backend state that has passed validation but has not started delivery. | `src/capture/capture_owner.rs:88` |
| `pocketstation::capture::authorization::CaptureAuthorizationSnapshot` | struct | Point-in-time authorization evidence for opening one exact capture source. | `src/capture/authorization.rs:17` |
| `pocketstation::capture::authorization::CapturePermissionLifecycle` | struct | Control-plane owner for one source's observed authorization epoch. | `src/capture/authorization.rs:183` |
| `pocketstation::capture::authorization::CapturePermissionTransition` | struct | One authoritative authorization-state transition observed by the host. | `src/capture/authorization.rs:168` |
| `pocketstation::capture::capture_owner::CaptureDelivery` | struct | Callback delivery endpoints transferred to a prepared native backend. | `src/capture/capture_owner.rs:73` |
| `pocketstation::capture::capture_owner::CaptureOwnerObservations` | struct | Aggregate observations from one active capture ownership boundary. | `src/capture/capture_owner.rs:160` |
| `pocketstation::capture::frame_stream::CapturedFrameSender` | struct | Single-producer endpoint passed into a platform capture callback. | `src/capture/frame_stream.rs:102` |
| `ActiveCaptureBackend::source_id` | function | Resolved native source identity for every frame emitted by this open. | `src/capture/capture_owner.rs:105` |
| `pocketstation::capture::query::application_capture_available` | function | Reports whether this host exposes the native application-capture facility. | `src/capture/query.rs:64` |
| `pocketstation::capture::query::SourceProvider` | trait | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/capture/query.rs:48` |
| `pocketstation::capture::authorization::PermissionEpoch` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/capture/authorization.rs:267` |
| `pocketstation::capture::events::CaptureRuntimeFailure` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/capture/events.rs:47` |
| `pocketstation::capture::events::SourceGeneration` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/capture/events.rs:12` |
| `pocketstation::capture::events::SourceRuntimeEventObservationHandle` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/capture/events.rs:200` |
| `pocketstation::capture::events::SourceRuntimeEventObservations` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/capture/events.rs:111` |
| `pocketstation::capture::events::SourceRuntimeEventSender` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/capture/events.rs:224` |
| `pocketstation::capture::frame_stream::CapturedFrameObservationHandle` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/capture/frame_stream.rs:31` |
| `pocketstation::capture::frame_stream::CapturedFrameStreamStats` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/capture/frame_stream.rs:17` |

## Permission and source opening

Permission observation and source opening are separate. The host application owns prompts and selection UI. A non-prompting observation is advisory where implemented; preparation or open returns the authoritative result.

## Qualification boundary

Target-specific files, Cargo dependencies, or CI establish implementation or build evidence only. They do not establish that every device, operating-system revision, packaging context, permission state, or physical path was qualified.

## Executable evidence

The following test bodies are evidence only for their recorded setup:

- `given_active_capture_when_owner_is_dropped_then_backend_is_reclaimed` — given active capture when owner is dropped then backend is reclaimed (`src/capture/capture_owner.rs:567`; `test-c55d7a75628c1be024f1`).
- `given_active_capture_when_stopped_then_backend_is_joined` — given active capture when stopped then backend is joined (`src/capture/capture_owner.rs:540`; `test-4f65c4d2e20b5226cd4f`).
- `given_panicking_capture_worker_when_joined_then_typed_failure_is_returned` — given panicking capture worker when joined then typed failure is returned (`src/capture/capture_owner.rs:610`; `test-889c6cfb54cc924fc2b4`).
- `given_prepared_capture_when_opened_then_bounded_delivery_is_owned` — given prepared capture when opened then bounded delivery is owned (`src/capture/capture_owner.rs:463`; `test-8de0974346f9110044c2`).
- `join_capture_worker` — join capture worker (`src/capture/capture_owner.rs:332`; `test-89b10abefa1f5c9a47e2`).
- `prepare_capture` — prepare capture (`src/capture/capture_owner.rs:296`; `test-59d7e50bbae31896948a`).
- `captured_frame_stream` — captured frame stream (`src/capture/frame_stream.rs:188`; `test-0e40457259bf43cdd2a7`).
- `given_capture_mode_when_channels_selected_then_microphone_is_mono_and_output_is_stereo` — given capture mode when channels selected then microphone is mono and output is stereo (`src/capture/platform/linux/pipewire.rs:1837`; `test-c28f1242d8a2b60457db`).
- `given_exhausted_capture_pool_when_acquiring_then_failure_is_observed_once` — given exhausted capture pool when acquiring then failure is observed once (`src/capture/platform/linux/pipewire.rs:1855`; `test-bcfd12a436362de05085`).
- `given_negotiated_format_when_channel_count_changes_then_capture_fails_closed` — given negotiated format when channel count changes then capture fails closed (`src/capture/platform/linux/pipewire.rs:2066`; `test-1993ee9e15230d1f6226`).
- `given_capture_before_callback_when_mapped_then_process_timestamp_preserves_delay` — given capture before callback when mapped then process timestamp preserves delay (`src/capture/platform/macos/input.rs:351`; `test-8a2ea38f6f2c1b3ffa2f`).
- `given_capture_before_process_epoch_when_mapped_then_timestamp_is_earliest_representable` — given capture before process epoch when mapped then timestamp is earliest representable (`src/capture/platform/macos/input.rs:364`; `test-9519b3f93a4a0e689bcc`).

## Related documentation

- [Glossary](/docs/glossary.md)
- [Linux capture](/docs/platform/linux.md)
- [Platform prerequisites](/docs/getting-started/platform-prerequisites.md)
- [Platform support and evidence](/docs/platform/compatibility.md)
- [PocketStation](/README.md)
- [Windows capture](/docs/platform/windows.md)
- [Behavior evidence index](/docs/reference/behavior-evidence.md)
- [Capture API](/docs/reference/capture.md)

## Evidence boundary

This page was verified against Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `src/capture/platform/macos/mod.rs:1-112` (`DIRECT`)

A file's presence proves implementation or declaration at this snapshot. It does not by itself prove physical-device qualification, operational performance, retry safety, or behavior outside the recorded test conditions.
