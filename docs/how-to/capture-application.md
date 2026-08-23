# Capture a desktop application

<!-- claims: CLM-GUIDE-001-CAP-001,CLM-GUIDE-001-CAP-002,CLM-GUIDE-001-CAP-003,CLM-GUIDE-001-SOURCE-001 -->

## Scope

- **Select and resolve capture sources.** Discover capture candidates and resolve application, process, device, and system queries to stable source identities.
- **Capture application audio.** Prepare application-scoped capture through the platform backend selected for the current target.
- **Observe permission and source lifecycle.** Query non-prompting authorization where observable and receive source-generation, loss, and permission-epoch changes.

The scope of **Capture a desktop application** ends at the native contracts and executable conditions cited below. Platform qualification, performance, retry, and delivery require their own explicit evidence.

## Prerequisites

A target with application capture support, host-owned capture permission, and an application selector that can resolve the intended process.

## Procedure

1. Create a Session declaration.
2. Build an ApplicationSelector whose evidence matches your selection need.
3. Declare the application Source and attach a consumer route.
4. Start the Session and retain RunningSession.
5. Observe frames or typed capture failures, then stop and inspect the outcome.

## Important consequence

Separate selector resolution, permission, source opening, and route delivery; each can fail independently.

## Verify the outcome

Frames from the selected application arrive on its polled route, and stop returns without a source or route failure.

Executable evidence selected for **Capture a desktop application** is limited to each test's recorded setup and assertions:

- `given_active_capture_when_owner_is_dropped_then_backend_is_reclaimed` — given active capture when owner is dropped then backend is reclaimed (`src/capture/capture_owner.rs:567`; `test-c55d7a75628c1be024f1`).
- `given_active_capture_when_stopped_then_backend_is_joined` — given active capture when stopped then backend is joined (`src/capture/capture_owner.rs:540`; `test-4f65c4d2e20b5226cd4f`).
- `given_panicking_capture_worker_when_joined_then_typed_failure_is_returned` — given panicking capture worker when joined then typed failure is returned (`src/capture/capture_owner.rs:610`; `test-889c6cfb54cc924fc2b4`).
- `given_prepared_capture_when_opened_then_bounded_delivery_is_owned` — given prepared capture when opened then bounded delivery is owned (`src/capture/capture_owner.rs:463`; `test-8de0974346f9110044c2`).
- `given_application_mode_when_pipewire_unavailable_then_mode_unsupported_not_system_mix` — given application mode when pipewire unavailable then mode unsupported not system mix (`src/capture/platform/linux/pipewire.rs:2097`; `test-a19b8c36cc500e40f220`).
- `given_capture_mode_when_channels_selected_then_microphone_is_mono_and_output_is_stereo` — given capture mode when channels selected then microphone is mono and output is stereo (`src/capture/platform/linux/pipewire.rs:1837`; `test-c28f1242d8a2b60457db`).
- `given_exact_application_selector_when_identity_is_transient_then_selection_fails_closed` — given exact application selector when identity is transient then selection fails closed (`src/capture/platform/linux/pipewire.rs:1962`; `test-1a09c0b9480a09c36429`).
- `given_exact_application_selector_when_multiple_nodes_match_then_selection_is_ambiguous` — given exact application selector when multiple nodes match then selection is ambiguous (`src/capture/platform/linux/pipewire.rs:1998`; `test-69d4e0c97753aed54953`).
- `given_exact_application_selector_when_one_live_node_matches_then_current_target_is_selected` — given exact application selector when one live node matches then current target is selected (`src/capture/platform/linux/pipewire.rs:1932`; `test-7477ad1c961dad51886d`).
- `given_exact_stable_application_when_pipewire_unavailable_then_mode_is_not_weakened` — given exact stable application when pipewire unavailable then mode is not weakened (`src/capture/platform/linux/pipewire.rs:2111`; `test-b6715ab214572748c3d2`).
- `given_exhausted_capture_pool_when_acquiring_then_failure_is_observed_once` — given exhausted capture pool when acquiring then failure is observed once (`src/capture/platform/linux/pipewire.rs:1855`; `test-bcfd12a436362de05085`).
- `given_negotiated_format_when_channel_count_changes_then_capture_fails_closed` — given negotiated format when channel count changes then capture fails closed (`src/capture/platform/linux/pipewire.rs:2066`; `test-1993ee9e15230d1f6226`).

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

- [Application Capture](/docs/concepts/application-capture.md)
- [Capture](/docs/reference/capture.md)

| Public declaration | Kind | Declared purpose | Source |
|---|---|---|---|
| `pocketstation::capture::platform::macos::session_backend::DesktopCaptureBackend` | struct | macOS adapter from the platform-neutral Session capture contract to the existing CoreAudio/input RAII owner. | `src/capture/platform/macos/session_backend.rs:11` |
| `pocketstation::capture::authorization::ApplicationPolicyObservation` | enum | Classifies the observable application policy observation. | `src/capture/authorization.rs:231` |
| `pocketstation::capture::query::application_capture_available` | function | Reports whether this host exposes the native application-capture facility. | `src/capture/query.rs:64` |
| `pocketstation::capture::authorization::ApplicationPolicyObservation::Allowed` | variant | Selects allowed behavior for `ApplicationPolicyObservation`. | `src/capture/authorization.rs:232` |
| `pocketstation::capture::authorization::ApplicationPolicyObservation::Denied` | variant | Selects denied behavior for `ApplicationPolicyObservation`. | `src/capture/authorization.rs:233` |
| `pocketstation::capture::authorization::ApplicationPolicyObservation::NotApplicable` | variant | Selects not applicable behavior for `ApplicationPolicyObservation`. | `src/capture/authorization.rs:235` |
| `pocketstation::capture::authorization::ApplicationPolicyObservation::NotObservable` | variant | Selects not observable behavior for `ApplicationPolicyObservation`. | `src/capture/authorization.rs:234` |
| `pocketstation::capture::authorization::CaptureScope::ExactApplication` | variant | Selects exact application behavior for `CaptureScope`. | `src/capture/authorization.rs:249` |

## Related documentation

- [Glossary](/docs/glossary.md)
- [PocketStation](/README.md)
- [Behavior evidence index](/docs/reference/behavior-evidence.md)
- [Capture API](/docs/reference/capture.md)
- [Rust API reference](/docs/reference/rust-api.md)
- [Capture failures](/docs/errors/capture.md)
- [No application audio arrives](/docs/troubleshooting/no-application-audio.md)
- [Linux capture](/docs/platform/linux.md)

## Evidence boundary

The claims on **Capture a desktop application** are anchored to Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `examples/product_quickstart.rs:1-61` (`DIRECT`)

For **Capture a desktop application**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.
