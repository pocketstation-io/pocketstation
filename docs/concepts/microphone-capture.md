# Microphone capture

<!-- claims: CLM-DOC-009-SCOPE-001,CLM-DOC-009-TEXT-001,CLM-DOC-009-TEXT-002,CLM-DOC-009-TEXT-003,CLM-DOC-009-TEXT-004,CLM-DOC-009-TEXT-005,CLM-DOC-009-TEXT-006,CLM-DOC-009-SOURCE-001 -->

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
| `pocketstation::capture::platform::macos::input::MacosInputSource` | struct | Owns production of macos input values and its lifecycle state. | `src/capture/platform/macos/input.rs:65` |
| `pocketstation::capture::query::LocalSourceProvider` | struct | Discovers and resolves capture sources through the target platform backend. | `src/capture/query.rs:52` |
| `pocketstation::capture::authorization::ApplicationPolicyObservation` | enum | Classifies the observable application policy observation. | `src/capture/authorization.rs:231` |
| `pocketstation::capture::authorization::CaptureCapabilityState` | enum | Selects the capture capability state used by PocketStation. | `src/capture/authorization.rs:145` |
| `pocketstation::capture::authorization::CaptureError` | enum | Classifies failures surfaced by capture operations. | `src/capture/authorization.rs:290` |

## Executable evidence

Executable evidence selected for **Microphone capture** is limited to each test's recorded setup and assertions:

- `given_capture_mode_when_channels_selected_then_microphone_is_mono_and_output_is_stereo` — given capture mode when channels selected then microphone is mono and output is stereo (`src/capture/platform/linux/pipewire.rs:1837`; `test-df5c7fa69c2c79a8f2a1`).
- `given_active_capture_when_owner_is_dropped_then_backend_is_reclaimed` — given active capture when owner is dropped then backend is reclaimed (`src/capture/capture_owner.rs:567`; `test-fa34e5723160d56f560f`).
- `given_active_capture_when_stopped_then_backend_is_joined` — given active capture when stopped then backend is joined (`src/capture/capture_owner.rs:540`; `test-dd4aaaf6b93ddb500769`).
- `given_panicking_capture_worker_when_joined_then_typed_failure_is_returned` — given panicking capture worker when joined then typed failure is returned (`src/capture/capture_owner.rs:610`; `test-2d873f94835a177ce436`).
- `given_prepared_capture_when_opened_then_bounded_delivery_is_owned` — given prepared capture when opened then bounded delivery is owned (`src/capture/capture_owner.rs:463`; `test-a3a0d044f02b7f664bb9`).
- `given_exhausted_capture_pool_when_acquiring_then_failure_is_observed_once` — given exhausted capture pool when acquiring then failure is observed once (`src/capture/platform/linux/pipewire.rs:1855`; `test-3002ec0fb883ffa835f6`).
- `given_negotiated_format_when_channel_count_changes_then_capture_fails_closed` — given negotiated format when channel count changes then capture fails closed (`src/capture/platform/linux/pipewire.rs:2066`; `test-0f6c4f31518ab5e8ffd8`).
- `given_capture_before_callback_when_mapped_then_process_timestamp_preserves_delay` — given capture before callback when mapped then process timestamp preserves delay (`src/capture/platform/macos/input.rs:358`; `test-de7d536ac9b0edc1d4da`).
- `given_capture_before_process_epoch_when_mapped_then_timestamp_is_earliest_representable` — given capture before process epoch when mapped then timestamp is earliest representable (`src/capture/platform/macos/input.rs:371`; `test-dc164b0e06605b749d99`).
- `given_denied_permission_when_opening_input_then_capture_fails_closed` — given denied permission when opening input then capture fails closed (`src/capture/platform/macos/input.rs:384`; `test-93f56a3510497f49f523`).
- `given_canonical_capture_identity_when_derived_then_source_id_matches_stable_vector` — given canonical capture identity when derived then source id matches stable vector (`src/capture/tests.rs:183`; `test-9c549d91f364bb436c12`).
- `given_capture_error_not_supported_when_displayed_then_contains_not_supported` — given capture error not supported when displayed then contains not supported (`src/capture/tests.rs:202`; `test-a9ee21553b930ba8710c`).

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

The claims on **Microphone capture** are anchored to Git snapshot `136e74888962558aa846d3143a19136a70936f45` and these primary files:

- `src/capture/authorization.rs:1-1` (`DECLARED`)

For **Microphone capture**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.
