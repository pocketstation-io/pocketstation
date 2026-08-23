# Microphone capture

<!-- claims: CLM-DOC-009-CAP-001,CLM-DOC-009-SOURCE-001 -->

## What it is

Microphone capture selects the default or an identified input device and opens it through the target's native capture backend.

## Why it exists

Microphone audio has independent permission, device identity, timing, and failure state. Modeling it as its own source prevents application audio and input-device behavior from being conflated.

## Relationships

- Permission observation can inform UI before preparation but is not the open result.
- The selected microphone becomes a source with its own stem and lineage.
- Polled audio and recording routes consume that stem independently.

## Invariants and guarantees

- `NotObservable` means no reliable preflight answer, not denial.
- The source-open outcome is authoritative.
- The default device may change; retain the identity returned for the active source.

## When you encounter it

- **Reach first captured frames** — Build a Session that captures an application and microphone and polls their independent frames.
- **Handle platform permission** — Perform non-prompting observation, own the prompt UX, and treat source opening as authoritative.
- **Transcribe captured stems** — Run the repository transcription example and preserve process evidence for its external boundary.

## Use it

- [Capture the default microphone](/docs/how-to/capture-microphone.md)
- [Observe permission without prompting](/docs/how-to/observe-permission.md)
- [No microphone audio arrives](/docs/troubleshooting/no-microphone-audio.md)

## Scope

- **Capture microphone audio.** Select the default or identified input device and open native microphone capture.

The scope of **Microphone capture** ends at the native contracts and executable conditions cited below. Platform qualification, performance, retry, and delivery require their own explicit evidence.

## Key API

| Public declaration | Kind | Declared purpose | Source |
|---|---|---|---|
| `pocketstation::capture::query::SourceProvider` | trait | Implement this trait to provide source behavior to PocketStation; its methods define the preparation and runtime contract. | `src/capture/query.rs:48` |
| `pocketstation::capture::authorization::CaptureAuthorizationSnapshot` | struct | Point-in-time authorization evidence for opening one exact capture source. | `src/capture/authorization.rs:17` |
| `pocketstation::capture::authorization::CapturePermissionLifecycle` | struct | Control-plane owner for one source's observed authorization epoch. | `src/capture/authorization.rs:183` |
| `pocketstation::capture::authorization::CapturePermissionTransition` | struct | One authoritative authorization-state transition observed by the host. | `src/capture/authorization.rs:168` |
| `pocketstation::capture::authorization::PermissionEpoch` | struct | Identifies the permission-observation generation attached to captured lineage. | `src/capture/authorization.rs:267` |
| `pocketstation::capture::platform::macos::input::MacosInputSource` | struct | Owns production of macos input values and its lifecycle state. | `src/capture/platform/macos/input.rs:68` |
| `pocketstation::capture::query::LocalSourceProvider` | struct | Discovers and resolves capture sources through the target platform backend. | `src/capture/query.rs:52` |
| `pocketstation::capture::authorization::ApplicationPolicyObservation` | enum | Classifies the observable application policy observation. | `src/capture/authorization.rs:231` |
| `pocketstation::capture::authorization::CaptureCapabilityState` | enum | Selects the capture capability state used by PocketStation. | `src/capture/authorization.rs:145` |
| `pocketstation::capture::authorization::CaptureError` | enum | Classifies failures reported as capture error. | `src/capture/authorization.rs:290` |

## Executable evidence

Executable evidence selected for **Microphone capture** is limited to each test's recorded setup and assertions:

- `given_capture_mode_when_channels_selected_then_microphone_is_mono_and_output_is_stereo` — given capture mode when channels selected then microphone is mono and output is stereo (`src/capture/platform/linux/pipewire.rs:1837`; `test-c28f1242d8a2b60457db`).
- `given_active_capture_when_owner_is_dropped_then_backend_is_reclaimed` — given active capture when owner is dropped then backend is reclaimed (`src/capture/capture_owner.rs:567`; `test-c55d7a75628c1be024f1`).
- `given_active_capture_when_stopped_then_backend_is_joined` — given active capture when stopped then backend is joined (`src/capture/capture_owner.rs:540`; `test-4f65c4d2e20b5226cd4f`).
- `given_panicking_capture_worker_when_joined_then_typed_failure_is_returned` — given panicking capture worker when joined then typed failure is returned (`src/capture/capture_owner.rs:610`; `test-889c6cfb54cc924fc2b4`).
- `given_prepared_capture_when_opened_then_bounded_delivery_is_owned` — given prepared capture when opened then bounded delivery is owned (`src/capture/capture_owner.rs:463`; `test-8de0974346f9110044c2`).
- `given_exhausted_capture_pool_when_acquiring_then_failure_is_observed_once` — given exhausted capture pool when acquiring then failure is observed once (`src/capture/platform/linux/pipewire.rs:1855`; `test-bcfd12a436362de05085`).
- `given_negotiated_format_when_channel_count_changes_then_capture_fails_closed` — given negotiated format when channel count changes then capture fails closed (`src/capture/platform/linux/pipewire.rs:2066`; `test-1993ee9e15230d1f6226`).
- `given_capture_before_callback_when_mapped_then_process_timestamp_preserves_delay` — given capture before callback when mapped then process timestamp preserves delay (`src/capture/platform/macos/input.rs:351`; `test-8a2ea38f6f2c1b3ffa2f`).
- `given_capture_before_process_epoch_when_mapped_then_timestamp_is_earliest_representable` — given capture before process epoch when mapped then timestamp is earliest representable (`src/capture/platform/macos/input.rs:364`; `test-9519b3f93a4a0e689bcc`).
- `given_denied_permission_when_opening_input_then_capture_fails_closed` — given denied permission when opening input then capture fails closed (`src/capture/platform/macos/input.rs:377`; `test-2b664c22fd511e3c2f45`).
- `given_canonical_capture_identity_when_derived_then_source_id_matches_stable_vector` — given canonical capture identity when derived then source id matches stable vector (`src/capture/tests.rs:174`; `test-39fa4a1bc5fb034e360f`).
- `given_capture_error_not_supported_when_displayed_then_contains_not_supported` — given capture error not supported when displayed then contains not supported (`src/capture/tests.rs:193`; `test-676aa99d320d214bccad`).

## Related documentation

- [Glossary](/docs/glossary.md)
- [Linux capture](/docs/platform/linux.md)
- [Platform backend boundary](/docs/internals/platform-backends.md)
- [Platform prerequisites](/docs/getting-started/platform-prerequisites.md)
- [Platform support and evidence](/docs/platform/compatibility.md)
- [PocketStation](/README.md)
- [Rust quickstart](/docs/getting-started/rust-quickstart.md)
- [Windows capture](/docs/platform/windows.md)

## Evidence boundary

The claims on **Microphone capture** are anchored to Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `src/capture/authorization.rs:1-318` (`DIRECT`)

For **Microphone capture**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.
