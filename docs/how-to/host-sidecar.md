# Host a managed-process sidecar

<!-- claims: CLM-GUIDE-020-CAP-001,CLM-GUIDE-020-CAP-002,CLM-GUIDE-020-SOURCE-001 -->

## Scope

- **Host managed-process sidecars.** Exchange bounded protocol messages with a child process under explicit deadlines and lifecycle states.
- **Run connector workers.** Supervise connector delivery, acknowledgement, readiness, cancellation, drain, and abort while reporting retry attempts and typed retryability.

The scope of **Host a managed-process sidecar** ends at the native contracts and executable conditions cited below. Platform qualification, performance, retry, and delivery require their own explicit evidence.

## Prerequisites

A bounded `SidecarProcessSpec`, declared message kinds, and explicit startup and shutdown deadlines.

## Procedure

1. Declare SidecarProcessSpec with bounded limits and deadlines.
2. Start the child through SidecarHost.
3. Exchange only declared message kinds.
4. Apply cancellation, drain, or abort through lifecycle state.
5. Inspect host snapshot and terminal error before restart.

## Important consequence

A managed process boundary does not itself provide sandboxing or authentication.

## Verify the outcome

The child reaches running state, exchanges only accepted messages, and ends in the requested drained or aborted terminal state.

Executable evidence selected for **Host a managed-process sidecar** is limited to each test's recorded setup and assertions:

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

- `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostError` / `AlreadyReaped` — `error-e3a2e354214fc48a985a`
- `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostError` / `Kill` — `error-b5e61d9a83637a8b0a55`
- `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostError` / `ProcessingTimeout` — `error-4330ee949bb0b39b87c5`
- `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostError` / `UnknownSidecar` — `error-9fd66aa4b6879249a810`
- `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostError` / `Wait` — `error-953c2a508ab4a201718f`
- `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostError` — `error-4a7bbf78f1eef4f31cda`
- `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostError` / `Closed` — `error-edccf83596e248e6faba`
- `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostError` / `ControlQueueFull` — `error-bdb3331ea4ef7fe66d05`
- `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostError` / `DataQueueFull` — `error-bc92e00331c1093a3a5f`
- `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostError` / `FrameTooLarge` — `error-d2843fac19f48d7e718e`

## API reference

- [Sidecars](/docs/concepts/sidecars.md)
- [Sidecar Protocol](/docs/reference/sidecar-protocol.md)

| Public declaration | Kind | Declared purpose | Source |
|---|---|---|---|
| `pocketstation::runtime::lifecycle::sidecar_host::SidecarProcessSpec` | struct | Configures sidecar process behavior at its owning API boundary. | `src/runtime/lifecycle/sidecar_host.rs:71` |
| `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostError::ProcessingTimeout` | variant | Reported when the owning operation encounters processing timeout. | `src/runtime/lifecycle/sidecar_host.rs:717` |
| `pocketstation::runtime::lifecycle::sidecar_host::SidecarDeadlines` | struct | Sets finite startup, I/O, shutdown, and reap deadlines for a sidecar process. | `src/runtime/lifecycle/sidecar_host.rs:54` |
| `pocketstation::runtime::lifecycle::sidecar_host::SidecarHost` | struct | Owns the resources and lifecycle for sidecar. | `src/runtime/lifecycle/sidecar_host.rs:157` |
| `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostObservations` | struct | Reports the sidecar host observations collected at an observation boundary. | `src/runtime/lifecycle/sidecar_host.rs:109` |
| `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostSnapshot` | struct | Reports the sidecar host snapshot collected at an observation boundary. | `src/runtime/lifecycle/sidecar_host.rs:133` |
| `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostError` | enum | Classifies failures reported as sidecar host error. | `src/runtime/lifecycle/sidecar_host.rs:686` |
| `pocketstation::runtime::lifecycle::sidecar_host::SidecarState` | enum | Selects the sidecar state used by PocketStation. | `src/runtime/lifecycle/sidecar_host.rs:21` |

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

The claims on **Host a managed-process sidecar** are anchored to Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `src/runtime/lifecycle/sidecar_host.rs:1-734` (`DIRECT`)

For **Host a managed-process sidecar**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.
