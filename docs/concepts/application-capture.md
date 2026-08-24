# Application capture

<!-- claims: CLM-DOC-008-SCOPE-001,CLM-DOC-008-TEXT-001,CLM-DOC-008-TEXT-002,CLM-DOC-008-TEXT-003,CLM-DOC-008-TEXT-004,CLM-DOC-008-TEXT-005,CLM-DOC-008-TEXT-006,CLM-DOC-008-SOURCE-001 -->

## What it is

Application capture opens audio associated with a selected desktop application through the backend compiled for the current target.

## Why it exists

Application-scoped capture keeps one application's audio distinct from microphone and system-loopback sources, so routes, observations, and recordings retain their source identity.

## Relationships

- Source selection identifies the application before preparation.
- `ApplicationPolicyObservation` projects allowed, denied, unobservable, or not-applicable policy state without conflating it with the source-open result.
- The platform backend owns native opening and callback delivery.
- Frame lineage carries application source identity into routes and recording stems.

## Invariants and guarantees

- Implementation availability and physical qualification are separate claims.
- A successful selection or allowed policy observation does not imply permission or source opening succeeded.
- The host application owns selection UI and permission prompts.

## When you encounter it

- **Reach first captured frames** — Build a Session that captures an application and microphone and polls their independent frames.
- **Handle platform permission** — Perform non-prompting observation, own the prompt UX, and treat source opening as authoritative.
- **Transcribe captured stems** — Run the repository transcription example and preserve process evidence for its external boundary.

## Use it

- [Capture a desktop application](/docs/how-to/capture-application.md)
- [Capture application and microphone stems](/docs/how-to/capture-app-and-mic.md)
- [No application audio arrives](/docs/troubleshooting/no-application-audio.md)

## Scope

- **Capture application audio.** Prepare application-scoped capture through the platform backend selected for the current target.

The scope of **Application capture** ends at the native contracts and executable conditions cited below. Platform qualification, performance, retry, and delivery require their own explicit evidence.

## Key API

| Public declaration | Kind | Declared purpose | Source |
|---|---|---|---|
| `pocketstation::capture::capture_owner::ActiveCaptureBackend` | trait | Native capture resources owned for exactly one active capture. | `src/capture/capture_owner.rs:100` |
| `pocketstation::capture::capture_owner::CallbackCaptureBackend` | trait | Platform-neutral prepare/open boundary for callback-oriented capture. | `src/capture/capture_owner.rs:83` |
| `pocketstation::capture::capture_owner::PreparedCaptureBackend` | trait | Backend state that has passed validation but has not started delivery. | `src/capture/capture_owner.rs:88` |
| `pocketstation::capture::capture_owner::CaptureDelivery` | struct | Callback delivery endpoints transferred to a prepared native backend. | `src/capture/capture_owner.rs:73` |
| `pocketstation::capture::capture_owner::CaptureLineageSeed` | struct | Stable session and stem identity assigned before an exact source is opened. | `src/capture/capture_owner.rs:25` |
| `pocketstation::capture::capture_owner::CaptureObservationReceipt` | struct | Retains the identity and observation access returned for capture observation. | `src/capture/capture_owner.rs:167` |
| `pocketstation::capture::capture_owner::CaptureOpenMetadata` | struct | Authoritative lineage state established only after native capture opens. | `src/capture/capture_owner.rs:49` |
| `pocketstation::capture::capture_owner::CaptureOwner` | struct | RAII owner for native capture, its bounded frame stream, and runtime events. | `src/capture/capture_owner.rs:194` |
| `pocketstation::capture::capture_owner::CaptureOwnerObservations` | struct | Aggregate observations from one active capture ownership boundary. | `src/capture/capture_owner.rs:160` |
| `pocketstation::capture::capture_owner::CapturePrepareRequest` | struct | Setup-time request for one bounded callback-oriented capture owner. | `src/capture/capture_owner.rs:61` |

## Executable evidence

Executable evidence selected for **Application capture** is limited to each test's recorded setup and assertions:

- `given_active_capture_when_owner_is_dropped_then_backend_is_reclaimed` — given active capture when owner is dropped then backend is reclaimed (`src/capture/capture_owner.rs:567`; `test-fa34e5723160d56f560f`).
- `given_active_capture_when_stopped_then_backend_is_joined` — given active capture when stopped then backend is joined (`src/capture/capture_owner.rs:540`; `test-dd4aaaf6b93ddb500769`).
- `given_panicking_capture_worker_when_joined_then_typed_failure_is_returned` — given panicking capture worker when joined then typed failure is returned (`src/capture/capture_owner.rs:610`; `test-2d873f94835a177ce436`).
- `given_prepared_capture_when_opened_then_bounded_delivery_is_owned` — given prepared capture when opened then bounded delivery is owned (`src/capture/capture_owner.rs:463`; `test-a3a0d044f02b7f664bb9`).
- `given_backend_frame_when_source_differs_from_open_identity_then_lineage_fails_closed` — given backend frame when source differs from open identity then lineage fails closed (`src/capture/capture_owner.rs:511`; `test-805d755d4acd2257ba9b`).
- `given_zero_frame_capacity_when_preparing_then_backend_is_not_prepared` — given zero frame capacity when preparing then backend is not prepared (`src/capture/capture_owner.rs:590`; `test-0afbec4242ea2fad4582`).
- `given_application_mode_when_pipewire_unavailable_then_mode_unsupported_not_system_mix` — given application mode when pipewire unavailable then mode unsupported not system mix (`src/capture/platform/linux/pipewire.rs:2097`; `test-3935b20953f69bd82dab`).
- `given_capture_mode_when_channels_selected_then_microphone_is_mono_and_output_is_stereo` — given capture mode when channels selected then microphone is mono and output is stereo (`src/capture/platform/linux/pipewire.rs:1837`; `test-df5c7fa69c2c79a8f2a1`).
- `given_exact_application_selector_when_identity_is_transient_then_selection_fails_closed` — given exact application selector when identity is transient then selection fails closed (`src/capture/platform/linux/pipewire.rs:1962`; `test-1e40dd4ec9e96cd35eb7`).
- `given_exact_application_selector_when_multiple_nodes_match_then_selection_is_ambiguous` — given exact application selector when multiple nodes match then selection is ambiguous (`src/capture/platform/linux/pipewire.rs:1998`; `test-2843e96f914d98065a94`).
- `given_exact_application_selector_when_one_live_node_matches_then_current_target_is_selected` — given exact application selector when one live node matches then current target is selected (`src/capture/platform/linux/pipewire.rs:1932`; `test-15388b47d24aa21999f6`).
- `given_exact_stable_application_when_pipewire_unavailable_then_mode_is_not_weakened` — given exact stable application when pipewire unavailable then mode is not weakened (`src/capture/platform/linux/pipewire.rs:2111`; `test-51cbb8d765eada41b0c9`).

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

The claims on **Application capture** are anchored to Git snapshot `136e74888962558aa846d3143a19136a70936f45` and these primary files:

- `src/capture/capture_owner.rs:20-21` (`DIRECT`)
- `src/capture/capture_owner.rs:24-24` (`DIRECT`)
- `src/capture/capture_owner.rs:24-24` (`DIRECT`)
- `src/capture/capture_owner.rs:24-24` (`DIRECT`)
- `src/capture/capture_owner.rs:25-28` (`DIRECT`)
- `src/capture/capture_owner.rs:26-26` (`DIRECT`)
- `src/capture/capture_owner.rs:27-27` (`DIRECT`)
- `src/capture/capture_owner.rs:31-36` (`DIRECT`)
- `src/capture/capture_owner.rs:38-40` (`DIRECT`)
- `src/capture/capture_owner.rs:42-44` (`DIRECT`)
- `src/capture/capture_owner.rs:48-48` (`DIRECT`)
- `src/capture/capture_owner.rs:48-48` (`DIRECT`)
- `src/capture/capture_owner.rs:48-48` (`DIRECT`)
- `src/capture/capture_owner.rs:49-57` (`DIRECT`)
- `src/capture/capture_owner.rs:50-50` (`DIRECT`)
- `src/capture/capture_owner.rs:51-51` (`DIRECT`)
- `src/capture/capture_owner.rs:52-52` (`DIRECT`)
- `src/capture/capture_owner.rs:53-53` (`DIRECT`)
- `src/capture/capture_owner.rs:54-54` (`DIRECT`)
- `src/capture/capture_owner.rs:55-55` (`DIRECT`)
- `src/capture/capture_owner.rs:56-56` (`DIRECT`)
- `src/capture/capture_owner.rs:60-60` (`DIRECT`)
- `src/capture/capture_owner.rs:60-60` (`DIRECT`)
- `src/capture/capture_owner.rs:60-60` (`DIRECT`)

For **Application capture**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.
