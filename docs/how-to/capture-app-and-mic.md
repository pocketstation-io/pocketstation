# Capture application and microphone stems

<!-- claims: CLM-GUIDE-003-CAP-001,CLM-GUIDE-003-CAP-002,CLM-GUIDE-003-CAP-003,CLM-GUIDE-003-CAP-004,CLM-GUIDE-003-SOURCE-001 -->

## Scope

- **Declare a Session.** Describe sources, operators, endpoints, streams, and recording routes before runtime preparation.
- **Capture application audio.** Prepare application-scoped capture through the platform backend selected for the current target.
- **Capture microphone audio.** Select the default or identified input device and open native microphone capture.
- **Preserve frame identity and lineage.** Carry source, stream, stem, route, clock, sequence, generation, and derivation identity with audio frames.

The scope of **Capture application and microphone stems** ends at the native contracts and executable conditions cited below. Platform qualification, performance, retry, and delivery require their own explicit evidence.

## Prerequisites

Both sources can be selected and opened, and you have chosen independent destinations or recording labels.

## Procedure

1. Declare application and microphone sources in one Session.
2. Give each source an independent endpoint or route.
3. Retain stem and source identity from frame lineage.
4. Start once and consume both bounded routes.
5. Stop once and inspect Session plus recording outcomes.

## Important consequence

A healthy source does not hide failure or saturation on the other route.

## Verify the outcome

Both routes deliver frames, retain different stem identities, and report their own recording outcomes.

Executable evidence selected for **Capture application and microphone stems** is limited to each test's recorded setup and assertions:

- `given_capture_mode_when_channels_selected_then_microphone_is_mono_and_output_is_stereo` — given capture mode when channels selected then microphone is mono and output is stereo (`src/capture/platform/linux/pipewire.rs:1837`; `test-c28f1242d8a2b60457db`).
- `given_active_capture_when_owner_is_dropped_then_backend_is_reclaimed` — given active capture when owner is dropped then backend is reclaimed (`src/capture/capture_owner.rs:567`; `test-c55d7a75628c1be024f1`).
- `given_active_capture_when_stopped_then_backend_is_joined` — given active capture when stopped then backend is joined (`src/capture/capture_owner.rs:540`; `test-4f65c4d2e20b5226cd4f`).
- `given_panicking_capture_worker_when_joined_then_typed_failure_is_returned` — given panicking capture worker when joined then typed failure is returned (`src/capture/capture_owner.rs:610`; `test-889c6cfb54cc924fc2b4`).
- `given_prepared_capture_when_opened_then_bounded_delivery_is_owned` — given prepared capture when opened then bounded delivery is owned (`src/capture/capture_owner.rs:463`; `test-8de0974346f9110044c2`).
- `given_application_mode_when_pipewire_unavailable_then_mode_unsupported_not_system_mix` — given application mode when pipewire unavailable then mode unsupported not system mix (`src/capture/platform/linux/pipewire.rs:2097`; `test-a19b8c36cc500e40f220`).
- `given_exact_application_selector_when_identity_is_transient_then_selection_fails_closed` — given exact application selector when identity is transient then selection fails closed (`src/capture/platform/linux/pipewire.rs:1962`; `test-1a09c0b9480a09c36429`).
- `given_exact_application_selector_when_multiple_nodes_match_then_selection_is_ambiguous` — given exact application selector when multiple nodes match then selection is ambiguous (`src/capture/platform/linux/pipewire.rs:1998`; `test-69d4e0c97753aed54953`).
- `given_exact_application_selector_when_one_live_node_matches_then_current_target_is_selected` — given exact application selector when one live node matches then current target is selected (`src/capture/platform/linux/pipewire.rs:1932`; `test-7477ad1c961dad51886d`).
- `given_exact_stable_application_when_pipewire_unavailable_then_mode_is_not_weakened` — given exact stable application when pipewire unavailable then mode is not weakened (`src/capture/platform/linux/pipewire.rs:2111`; `test-b6715ab214572748c3d2`).
- `given_exhausted_capture_pool_when_acquiring_then_failure_is_observed_once` — given exhausted capture pool when acquiring then failure is observed once (`src/capture/platform/linux/pipewire.rs:1855`; `test-bcfd12a436362de05085`).
- `given_negotiated_format_when_channel_count_changes_then_capture_fails_closed` — given negotiated format when channel count changes then capture fails closed (`src/capture/platform/linux/pipewire.rs:2066`; `test-1993ee9e15230d1f6226`).

## Failure signals

- `pocketstation::session::lifecycle::host::SessionEngineHostBuildError` / `MissingApplicationBackend` — `error-cfaceeb6b969abc08c25`
- `pocketstation::session::lifecycle::host::SessionEngineHostBuildError` / `MissingMicrophoneBackend` — `error-b5df9cb8cf9ebc127bf9`
- `pocketstation::session::lifecycle::start_contract::SessionStartError` / `UnsupportedSourceTopology` — `error-ba1f04a6ba792a9367e0`
- `pocketstation::capture::authorization::CaptureError` — `error-7905cc933b9eb45fe4ef`
- `pocketstation::capture::authorization::CaptureError` / `BackendInit` — `error-ffea5e00d982c5213eba`
- `pocketstation::capture::authorization::CaptureError` / `BackendSetupRequired` — `error-6e8f9f8ca8efa76ded69`
- `pocketstation::capture::authorization::CaptureError` / `BackendStatus` — `error-533b29bac30886d8c79c`
- `pocketstation::capture::authorization::CaptureError` / `CaptureWorkerPanicked` — `error-01c4b3cce2fa1669ee13`
- `pocketstation::capture::authorization::CaptureError` / `InvalidRuntimeEventCapacity` — `error-c683702117e27ad45f33`
- `pocketstation::capture::authorization::CaptureError` / `InvalidStreamCapacity` — `error-6167103023ec8fded812`

## API reference

- [Session](/docs/concepts/session.md)
- [Multistem Recording](/docs/concepts/multistem-recording.md)

| Public declaration | Kind | Declared purpose | Source |
|---|---|---|---|
| `pocketstation::capture::query::application_capture_available` | function | Reports whether this host exposes the native application-capture facility. | `src/capture/query.rs:64` |
| `pocketstation::capture::capture_owner::ActiveCaptureBackend` | trait | Native capture resources owned for exactly one active capture. | `src/capture/capture_owner.rs:100` |
| `pocketstation::capture::capture_owner::CallbackCaptureBackend` | trait | Platform-neutral prepare/open boundary for callback-oriented capture. | `src/capture/capture_owner.rs:83` |
| `pocketstation::capture::capture_owner::PreparedCaptureBackend` | trait | Backend state that has passed validation but has not started delivery. | `src/capture/capture_owner.rs:88` |
| `pocketstation::capture::query::SourceProvider` | trait | Implement this trait to provide source behavior to PocketStation; its methods define the preparation and runtime contract. | `src/capture/query.rs:48` |
| `pocketstation::capture::capture_owner::CaptureDelivery` | struct | Callback delivery endpoints transferred to a prepared native backend. | `src/capture/capture_owner.rs:73` |
| `pocketstation::capture::capture_owner::CaptureLineageSeed` | struct | Stable session and stem identity assigned before an exact source is opened. | `src/capture/capture_owner.rs:25` |
| `pocketstation::capture::capture_owner::CaptureObservationReceipt` | struct | Retains the identity and observation access returned for capture observation. | `src/capture/capture_owner.rs:167` |

## Related documentation

- [Glossary](/docs/glossary.md)
- [PocketStation](/README.md)
- [Behavior evidence index](/docs/reference/behavior-evidence.md)
- [Rust API reference](/docs/reference/rust-api.md)
- [Rust quickstart](/docs/getting-started/rust-quickstart.md)
- [Linux capture](/docs/platform/linux.md)
- [Platform backend boundary](/docs/internals/platform-backends.md)
- [Platform prerequisites](/docs/getting-started/platform-prerequisites.md)

## Evidence boundary

The claims on **Capture application and microphone stems** are anchored to Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `examples/product_quickstart.rs:1-61` (`DIRECT`)

For **Capture application and microphone stems**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.
