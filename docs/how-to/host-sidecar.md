# Host a managed-process sidecar

<!-- claims: CLM-GUIDE-020-CAP-001,CLM-GUIDE-020-CAP-002,CLM-GUIDE-020-SOURCE-001 -->

## Scope

- **Host managed-process sidecars.** Exchange bounded protocol messages with a child process under explicit deadlines and lifecycle states.
- **Run connector workers.** Supervise connector delivery, acknowledgement, retry budgets, readiness, cancellation, drain, and abort.

These statements describe repository contracts at the documented snapshot. They do not extend platform qualification, performance, retry, or delivery guarantees beyond the native API contracts and executable evidence.

## Prerequisites

Read the linked concept and confirm that target platform, Cargo features, source or provider dependencies, and application-owned permission work match this task. Keep returned typed errors and outcomes available for verification.

## Procedure

1. Declare SidecarProcessSpec with bounded limits and deadlines.
2. Start the child through SidecarHost.
3. Exchange only declared message kinds.
4. Apply cancellation, drain, or abort through lifecycle state.
5. Inspect host snapshot and terminal error before restart.

## APIs used

| Public declaration | Kind | Declared purpose | Source |
|---|---|---|---|
| `pocketstation::runtime::lifecycle::sidecar_host::SidecarProcessSpec` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/runtime/lifecycle/sidecar_host.rs:71` |
| `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostError::ProcessingTimeout` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/runtime/lifecycle/sidecar_host.rs:717` |
| `pocketstation::runtime::lifecycle::sidecar_host::SidecarDeadlines` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/runtime/lifecycle/sidecar_host.rs:54` |
| `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostSnapshot` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/runtime/lifecycle/sidecar_host.rs:133` |
| `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostError` | enum | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/runtime/lifecycle/sidecar_host.rs:686` |
| `pocketstation::runtime::lifecycle::sidecar_host::SidecarState` | enum | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/runtime/lifecycle/sidecar_host.rs:21` |
| `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostError::AlreadyReaped` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/runtime/lifecycle/sidecar_host.rs:730` |
| `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostError::Closed` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/runtime/lifecycle/sidecar_host.rs:706` |
| `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostError::ControlQueueFull` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/runtime/lifecycle/sidecar_host.rs:704` |
| `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostError::DataQueueFull` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/runtime/lifecycle/sidecar_host.rs:702` |
| `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostError::FrameTooLarge` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/runtime/lifecycle/sidecar_host.rs:700` |
| `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostError::InvalidConfiguration` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/runtime/lifecycle/sidecar_host.rs:688` |
| `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostError::InvalidDataKind` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/runtime/lifecycle/sidecar_host.rs:724` |
| `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostError::InvalidState` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/runtime/lifecycle/sidecar_host.rs:719` |
| `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostError::Io` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/runtime/lifecycle/sidecar_host.rs:696` |
| `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostError::Kill` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/runtime/lifecycle/sidecar_host.rs:728` |

## Verify the outcome

The following test bodies are evidence only for their recorded setup:

- `given_sidecar_host_errors_when_classified_then_retryability_is_preserved` — given sidecar host errors when classified then retryability is preserved (`src/connector/sidecar.rs:286`; `test-98ad8a10ce6f978fe856`).
- `given_empty_input_group_when_sidecar_prepares_then_structured_error_is_returned` — given empty input group when sidecar prepares then structured error is returned (`src/connector/sidecar.rs:270`; `test-49bd18fb96d67fdba9bf`).
- `given_two_ready_sources_when_processed_then_each_source_dispatches_independently` — given two ready sources when processed then each source dispatches independently (`src/runtime/audio/runner.rs:595`; `test-218c72d3cc7560654f20`).
- `given_slow_future_when_deadline_expires_then_host_returns_typed_timeout` — given slow future when deadline expires then host returns typed timeout (`src/runtime/lifecycle/async_host.rs:141`; `test-9e709c123ea73a9a1332`).
- `given_core_extension_oversized_sidecar_payload_when_encoded_then_fails_closed` — given core extension oversized sidecar payload when encoded then fails closed (`src/runtime/lifecycle/sidecar_protocol.rs:346`; `test-f94228781a9717656566`).
- `given_core_extension_sidecar_message_when_round_tripped_then_identity_is_stable` — given core extension sidecar message when round tripped then identity is stable (`src/runtime/lifecycle/sidecar_protocol.rs:327`; `test-647c9ae6972c9c36f722`).
- `given_audio_output_without_audio_port_when_processed_then_worker_rejects_it` — given audio output without audio port when processed then worker rejects it (`src/runtime/signal/operator.rs:2466`; `test-f0e8c28b4853dfd07393`).
- `given_node_reported_timeout_when_observed_then_it_is_not_a_process_failure` — given node reported timeout when observed then it is not a process failure (`src/runtime/signal/operator.rs:2546`; `test-78fa5617e6bce70cd79c`).
- `given_two_inputs_when_processed_then_nonterminal_and_terminal_reach_each_branch` — given two inputs when processed then nonterminal and terminal reach each branch (`src/runtime/signal/operator.rs:2079`; `test-23bf71cedc9a9fc7172c`).
- `given_undeclared_output_role_when_processed_then_worker_rejects_it` — given undeclared output role when processed then worker rejects it (`src/runtime/signal/operator.rs:2443`; `test-bc6e3ebd53693f487168`).
- `given_wrong_output_class_when_processed_then_worker_rejects_it` — given wrong output class when processed then worker rejects it (`src/runtime/signal/operator.rs:2420`; `test-12ef395dca26d9d33f88`).
- `given_connector_public_surface_when_inspected_then_managed_aliases_are_absent` — given connector public surface when inspected then managed aliases are absent (`tests/connector_contract.rs:26`; `test-e1ff05b0ec4b54a78b0b`).

## Failure signals

- `pocketstation::connector::error::ConnectorErrorCodeError` / `TooLong` — `error-06f5c52aa07c86ca5062`
- `pocketstation::connector::transport::ConnectorAudioRecordError` / `InvalidSampleCount` — `error-093c41e2489cf1bb258d`
- `pocketstation::connector::transport::ConnectorAudioRecordError` — `error-0b1f3a3357a77fcef185`
- `pocketstation::connector::error::ConnectorErrorCodeError` / `Empty` — `error-0b71c9f1b1489e0d4f9a`
- `pocketstation::connector::error::ConnectorErrorBuildError` — `error-0bc8adb0641971704f74`
- `pocketstation::connector::configuration::ConnectorConfigurationErrorCode` / `TooManyFields` — `error-0c83ebde568152ad3edf`
- `pocketstation::connector::error::ConnectorErrorStage` / `Startup` — `error-0e62627edef059ecab22`
- `pocketstation::connector::manifest::ConnectorManifestError` / `InvalidManifestRevision` — `error-10517744910e14c23fc4`
- `pocketstation::connector::transport::ConnectorAudioRecordError` / `UnsupportedMinor` — `error-1082687e9dbfd2cadfc5`
- `pocketstation::runtime::lifecycle::sidecar_protocol::SidecarProtocolError` / `InvalidMagic` — `error-143cce14f0e71f68c4cf`
- `pocketstation::connector::configuration::ConnectorConfigurationErrorCode` / `InvalidValue` — `error-16fe034657303e4973f8`
- `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostError` / `Wait` — `error-19eabd878a9188bf94ce`

Retry only when the relevant API or error contract explicitly permits it. An error name, a transient-looking message, or a successful prior run is not retry evidence.

## Related documentation

- [Glossary](/docs/glossary.md)
- [PocketStation](/README.md)
- [Behavior evidence index](/docs/reference/behavior-evidence.md)
- [Configuration reference](/docs/reference/configuration.md)
- [Lifecycle evidence index](/docs/reference/lifecycle-evidence.md)
- [Protocol surface index](/docs/reference/protocol-surface.md)
- [Rust API reference](/docs/reference/rust-api.md)
- [A sidecar misses a deadline](/docs/troubleshooting/sidecar-deadline.md)

## Evidence boundary

This page was verified against Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `src/runtime/lifecycle/sidecar_host.rs:1-734` (`DIRECT`)

A file's presence proves implementation or declaration at this snapshot. It does not by itself prove physical-device qualification, operational performance, retry safety, or behavior outside the recorded test conditions.
