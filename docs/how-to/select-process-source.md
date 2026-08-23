# Select a process-scoped source

<!-- claims: CLM-GUIDE-004-CAP-001,CLM-GUIDE-004-CAP-002,CLM-GUIDE-004-SOURCE-001 -->

## Scope

- **Select and resolve capture sources.** Discover capture candidates and resolve application, process, device, and system queries to stable source identities.
- **Observe permission and source lifecycle.** Query non-prompting authorization where observable and receive source-generation, loss, and permission-epoch changes.

The scope of **Select a process-scoped source** ends at the native contracts and executable conditions cited below. Platform qualification, performance, retry, and delivery require their own explicit evidence.

## Prerequisites

A discovery result that exposes process-instance identity rather than only a display name.

## Procedure

1. Discover candidates through the source provider.
2. Build a process or application query with the required scope.
3. Resolve the query and retain stable source identity.
4. Observe generation changes instead of assuming process identity is permanent.
5. Handle empty or ambiguous resolution as a typed result.

## Important consequence

Reject zero process IDs, empty stable keys, and non-application identities instead of weakening the selector.

## Verify the outcome

Resolution returns the intended exact process instance and later generation changes remain observable.

Executable evidence selected for **Select a process-scoped source** is limited to each test's recorded setup and assertions:

- `given_process_scoped_exact_selector_when_identity_is_transient_then_matching_pid_is_allowed` — given process scoped exact selector when identity is transient then matching pid is allowed (`src/capture/platform/linux/pipewire.rs:1980`; `test-575dc7b197243c56d8f1`).
- `given_pipewire_process_callbacks_when_source_changes_then_realtime_contract_remains_explicit` — given pipewire process callbacks when source changes then realtime contract remains explicit (`tests/capture_callback_source_contract.rs:99`; `test-f907b9d183fd1d5d047f`).
- `given_backend_frame_when_source_differs_from_open_identity_then_lineage_fails_closed` — given backend frame when source differs from open identity then lineage fails closed (`src/capture/capture_owner.rs:511`; `test-805d755d4acd2257ba9b`).
- `given_capture_mode_when_channels_selected_then_microphone_is_mono_and_output_is_stereo` — given capture mode when channels selected then microphone is mono and output is stereo (`src/capture/platform/linux/pipewire.rs:1837`; `test-df5c7fa69c2c79a8f2a1`).
- `given_exact_application_selector_when_identity_is_transient_then_selection_fails_closed` — given exact application selector when identity is transient then selection fails closed (`src/capture/platform/linux/pipewire.rs:1962`; `test-1e40dd4ec9e96cd35eb7`).
- `given_exact_application_selector_when_multiple_nodes_match_then_selection_is_ambiguous` — given exact application selector when multiple nodes match then selection is ambiguous (`src/capture/platform/linux/pipewire.rs:1998`; `test-2843e96f914d98065a94`).
- `given_exact_application_selector_when_one_live_node_matches_then_current_target_is_selected` — given exact application selector when one live node matches then current target is selected (`src/capture/platform/linux/pipewire.rs:1932`; `test-15388b47d24aa21999f6`).
- `given_missing_exact_source_when_classified_then_stable_key_is_preserved` — given missing exact source when classified then stable key is preserved (`src/capture/platform/linux/pipewire.rs:1894`; `test-d288558b68fc54333e50`).
- `given_process_mode_when_node_not_found_then_backend_init_error_not_system_mix` — given process mode when node not found then backend init error not system mix (`src/capture/platform/linux/pipewire.rs:2131`; `test-b704602af68d2c7a0b53`).
- `given_process_mode_when_pipewire_unavailable_then_mode_unsupported_not_system_mix` — given process mode when pipewire unavailable then mode unsupported not system mix (`src/capture/platform/linux/pipewire.rs:2083`; `test-afbc5fb711b5d1e4c0fa`).
- `given_capture_before_callback_when_mapped_then_process_timestamp_preserves_delay` — given capture before callback when mapped then process timestamp preserves delay (`src/capture/platform/macos/input.rs:358`; `test-de7d536ac9b0edc1d4da`).
- `given_capture_before_process_epoch_when_mapped_then_timestamp_is_earliest_representable` — given capture before process epoch when mapped then timestamp is earliest representable (`src/capture/platform/macos/input.rs:371`; `test-dc164b0e06605b749d99`).

## Failure signals

- `pocketstation::capture::authorization::CaptureError` / `SourceUnavailable` — `error-fb207c871b52ba476b04`
- `pocketstation::capture::authorization::CaptureError` / `InvalidRuntimeEventCapacity` — `error-c838e8f36c42c18a2a83`
- `pocketstation::capture::events::CaptureRuntimeFailureClass` / `SourceInstanceExited` — `error-677a63665bbdf8a0715a`
- `pocketstation::capture::timeline::CaptureSampleTimelineError` / `SourcePositionMovedBackward` — `error-012287cf4e78fb89426b`
- `pocketstation::capture::timeline::CaptureSampleTimelineError` / `SourcePositionOverflow` — `error-c17db2e1686bee7e86be`

## API reference

- [Source Selection](/docs/concepts/source-selection.md)
- [Capture](/docs/reference/capture.md)

| Public declaration | Kind | Declared purpose | Source |
|---|---|---|---|
| `pocketstation::capture::selection::ProcessTreeScope` | enum | Selects the process tree scope used by PocketStation. | `src/capture/selection.rs:83` |
| `pocketstation::capture::authorization::SourceIdentityStrength::ApplicationIdAndProcessId` | variant | Represents the application id and process identifier alternative defined by `SourceIdentityStrength`. | `src/capture/authorization.rs:258` |
| `pocketstation::capture::authorization::SourceIdentityStrength::ProcessId` | variant | Represents the process identifier alternative defined by `SourceIdentityStrength`. | `src/capture/authorization.rs:260` |
| `pocketstation::capture::selection::CaptureMode::Process` | variant | Selects process behavior for `CaptureMode`. | `src/capture/selection.rs:20` |
| `pocketstation::capture::selection::ProcessTreeScope::ApplicationIdentity` | variant | Selects application identity behavior for `ProcessTreeScope`. | `src/capture/selection.rs:86` |
| `pocketstation::capture::selection::ProcessTreeScope::NotApplicable` | variant | Selects not applicable behavior for `ProcessTreeScope`. | `src/capture/selection.rs:87` |
| `pocketstation::capture::selection::ProcessTreeScope::SelectedProcessAndDescendants` | variant | Selects selected process and descendants behavior for `ProcessTreeScope`. | `src/capture/selection.rs:85` |
| `pocketstation::capture::selection::ProcessTreeScope::SelectedProcessOnly` | variant | Selects selected process only behavior for `ProcessTreeScope`. | `src/capture/selection.rs:84` |

## Related documentation

- [Glossary](/docs/glossary.md)
- [PocketStation](/README.md)
- [Capture a desktop application](/docs/how-to/capture-application.md)
- [Capture system audio](/docs/how-to/capture-system-audio.md)
- [Behavior evidence index](/docs/reference/behavior-evidence.md)
- [Capture API](/docs/reference/capture.md)
- [Rust API reference](/docs/reference/rust-api.md)
- [A capture source disappears](/docs/troubleshooting/source-loss.md)

## Evidence boundary

The claims on **Select a process-scoped source** are anchored to Git snapshot `136e74888962558aa846d3143a19136a70936f45` and these primary files:

- `src/session/declaration/selector.rs:1-215` (`DIRECT`)

For **Select a process-scoped source**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.
