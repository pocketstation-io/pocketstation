# Select a process-scoped source

<!-- claims: CLM-GUIDE-004-CAP-001,CLM-GUIDE-004-CAP-002,CLM-GUIDE-004-SOURCE-001 -->

## Scope

- **Select and resolve capture sources.** Discover capture candidates and resolve application, process, device, and system queries to stable source identities.
- **Observe permission and source lifecycle.** Query non-prompting authorization where observable and receive source-generation, loss, and permission-epoch changes.

These statements describe repository contracts at the documented snapshot. They do not extend platform qualification, performance, retry, or delivery guarantees beyond the native API contracts and executable evidence.

## Prerequisites

Read the linked concept and confirm that target platform, Cargo features, source or provider dependencies, and application-owned permission work match this task. Keep returned typed errors and outcomes available for verification.

## Procedure

1. Discover candidates through the source provider.
2. Build a process or application query with the required scope.
3. Resolve the query and retain stable source identity.
4. Observe generation changes instead of assuming process identity is permanent.
5. Handle empty or ambiguous resolution as a typed result.

## APIs used

| Public declaration | Kind | Declared purpose | Source |
|---|---|---|---|
| `pocketstation::capture::selection::ProcessTreeScope` | enum | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/capture/selection.rs:83` |
| `pocketstation::capture::authorization::SourceIdentityStrength::ApplicationIdAndProcessId` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/capture/authorization.rs:258` |
| `pocketstation::capture::authorization::SourceIdentityStrength::ProcessId` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/capture/authorization.rs:260` |
| `pocketstation::capture::selection::CaptureMode::Process` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/capture/selection.rs:20` |
| `pocketstation::capture::selection::ProcessTreeScope::ApplicationIdentity` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/capture/selection.rs:86` |
| `pocketstation::capture::selection::ProcessTreeScope::NotApplicable` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/capture/selection.rs:87` |
| `pocketstation::capture::selection::ProcessTreeScope::SelectedProcessAndDescendants` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/capture/selection.rs:85` |
| `pocketstation::capture::selection::ProcessTreeScope::SelectedProcessOnly` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/capture/selection.rs:84` |
| `pocketstation::capture::selection::SelectorPersistenceScope::ProcessLifetime` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/capture/selection.rs:74` |
| `identity::CaptureSource::process_id` | struct_field | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/capture/identity.rs:85` |
| `selection::CaptureMode::ExactApplication::process_id` | struct_field | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/capture/selection.rs:22` |
| `selection` | module | Capture selection semantics; control-plane only. | `src/capture/selection.rs:1` |
| `ActiveCaptureBackend::source_id` | function | Resolved native source identity for every frame emitted by this open. | `src/capture/capture_owner.rs:105` |
| `process_tree_scope` | function | Reports the native process boundary represented by this discovery result without making the CLI reconstruct a private capture mode. | `src/capture/identity.rs:140` |
| `process_tree_scope` | function | Reports the process boundary requested from the native backend. | `src/capture/selection.rs:55` |
| `selector_persistence_scope` | function | Reports how long this discovered selector can be reused without rediscovery. The capture owner remains authoritative for opening it. | `src/capture/identity.rs:114` |

## Verify the outcome

The following test bodies are evidence only for their recorded setup:

- `given_process_scoped_exact_selector_when_identity_is_transient_then_matching_pid_is_allowed` — given process scoped exact selector when identity is transient then matching pid is allowed (`src/capture/platform/linux/pipewire.rs:1980`; `test-281d496a5f325c196fe0`).
- `given_pipewire_process_callbacks_when_source_changes_then_realtime_contract_remains_explicit` — given pipewire process callbacks when source changes then realtime contract remains explicit (`tests/capture_callback_source_contract.rs:99`; `test-8e4f6e83c1cbdb5caf59`).
- `given_backend_frame_when_source_differs_from_open_identity_then_lineage_fails_closed` — given backend frame when source differs from open identity then lineage fails closed (`src/capture/capture_owner.rs:511`; `test-a8dbef4f3b61c752ce0e`).
- `given_capture_mode_when_channels_selected_then_microphone_is_mono_and_output_is_stereo` — given capture mode when channels selected then microphone is mono and output is stereo (`src/capture/platform/linux/pipewire.rs:1837`; `test-c28f1242d8a2b60457db`).
- `given_exact_application_selector_when_identity_is_transient_then_selection_fails_closed` — given exact application selector when identity is transient then selection fails closed (`src/capture/platform/linux/pipewire.rs:1962`; `test-1a09c0b9480a09c36429`).
- `given_exact_application_selector_when_multiple_nodes_match_then_selection_is_ambiguous` — given exact application selector when multiple nodes match then selection is ambiguous (`src/capture/platform/linux/pipewire.rs:1998`; `test-69d4e0c97753aed54953`).
- `given_exact_application_selector_when_one_live_node_matches_then_current_target_is_selected` — given exact application selector when one live node matches then current target is selected (`src/capture/platform/linux/pipewire.rs:1932`; `test-7477ad1c961dad51886d`).
- `given_missing_exact_source_when_classified_then_stable_key_is_preserved` — given missing exact source when classified then stable key is preserved (`src/capture/platform/linux/pipewire.rs:1894`; `test-50620fcc9117c7ad3cf6`).
- `given_process_mode_when_node_not_found_then_backend_init_error_not_system_mix` — given process mode when node not found then backend init error not system mix (`src/capture/platform/linux/pipewire.rs:2131`; `test-d95b10aa2227cf4f9ffb`).
- `given_process_mode_when_pipewire_unavailable_then_mode_unsupported_not_system_mix` — given process mode when pipewire unavailable then mode unsupported not system mix (`src/capture/platform/linux/pipewire.rs:2083`; `test-06acab2e5df43578641f`).
- `given_capture_before_callback_when_mapped_then_process_timestamp_preserves_delay` — given capture before callback when mapped then process timestamp preserves delay (`src/capture/platform/macos/input.rs:351`; `test-8a2ea38f6f2c1b3ffa2f`).
- `given_capture_before_process_epoch_when_mapped_then_timestamp_is_earliest_representable` — given capture before process epoch when mapped then timestamp is earliest representable (`src/capture/platform/macos/input.rs:364`; `test-9519b3f93a4a0e689bcc`).

## Failure signals

- `pocketstation::capture::events::CaptureRuntimeFailure` — `error-11b972ad42d5de880e06`
- `pocketstation::capture::events::CaptureRuntimeFailureClass` / `BackendClass` — `error-29e952ae7432566a9e95`
- `pocketstation::capture::authorization::CaptureError` / `CaptureWorkerPanicked` — `error-365f9b6fbda74eb0d631`
- `pocketstation::capture::authorization::CaptureError` / `PermissionDenied` — `error-38030156125346a8e892`
- `pocketstation::capture::authorization::CaptureError` / `NotSupported` — `error-3b4b5393164d9f6f12a5`
- `pocketstation::capture::events::CaptureRuntimeFailureClass` / `PlatformStatus` — `error-3c6fcc22deb2f54788ba`
- `pocketstation::capture::authorization::CaptureError` / `SourceUnavailable` — `error-71c87f975acc9e22a402`
- `pocketstation::capture::authorization::CaptureError` / `BackendSetupRequired` — `error-8db0fec69a9c7158ffdf`
- `pocketstation::capture::authorization::CaptureError` — `error-96ffe4bc4254583d1e17`
- `pocketstation::capture::events::CaptureRuntimeFailureClass` / `SourceInstanceExited` — `error-a9c0f7dfff744e9ba6b7`
- `pocketstation::capture::authorization::CaptureError` / `BackendInit` — `error-b320ea1cba2b3c8dc4c7`
- `pocketstation::capture::authorization::CaptureError` / `InvalidStreamCapacity` — `error-bcf5d4d897b6bd0784bf`

Retry only when the relevant API or error contract explicitly permits it. An error name, a transient-looking message, or a successful prior run is not retry evidence.

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

This page was verified against Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `src/session/declaration/selector.rs:1-215` (`DIRECT`)

A file's presence proves implementation or declaration at this snapshot. It does not by itself prove physical-device qualification, operational performance, retry safety, or behavior outside the recorded test conditions.
