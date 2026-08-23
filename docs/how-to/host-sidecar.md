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

- `given_sidecar_host_errors_when_classified_then_retryability_is_preserved` — given sidecar host errors when classified then retryability is preserved (`src/connector/sidecar.rs:286`; `test-72a5c76707ff849957fa`).
- `given_empty_input_group_when_sidecar_prepares_then_structured_error_is_returned` — given empty input group when sidecar prepares then structured error is returned (`src/connector/sidecar.rs:270`; `test-a819c552a02a127c977d`).
- `given_two_ready_sources_when_processed_then_each_source_dispatches_independently` — given two ready sources when processed then each source dispatches independently (`src/runtime/audio/runner.rs:595`; `test-c7a541844f8163662960`).
- `given_slow_future_when_deadline_expires_then_host_returns_typed_timeout` — given slow future when deadline expires then host returns typed timeout (`src/runtime/lifecycle/async_host.rs:141`; `test-61cb0441698a1984dac6`).
- `given_core_extension_oversized_sidecar_payload_when_encoded_then_fails_closed` — given core extension oversized sidecar payload when encoded then fails closed (`src/runtime/lifecycle/sidecar_protocol.rs:346`; `test-6c04502719d9fc0cdb98`).
- `given_core_extension_sidecar_message_when_round_tripped_then_identity_is_stable` — given core extension sidecar message when round tripped then identity is stable (`src/runtime/lifecycle/sidecar_protocol.rs:327`; `test-0d42eb96d05d1140bbc3`).
- `given_audio_output_without_audio_port_when_processed_then_worker_rejects_it` — given audio output without audio port when processed then worker rejects it (`src/runtime/signal/operator.rs:2466`; `test-e0f5021f2a3131ebe15b`).
- `given_node_reported_timeout_when_observed_then_it_is_not_a_process_failure` — given node reported timeout when observed then it is not a process failure (`src/runtime/signal/operator.rs:2546`; `test-6a86b6a1a672167d10ee`).
- `given_two_inputs_when_processed_then_nonterminal_and_terminal_reach_each_branch` — given two inputs when processed then nonterminal and terminal reach each branch (`src/runtime/signal/operator.rs:2079`; `test-f35227f30fb90260d27a`).
- `given_undeclared_output_role_when_processed_then_worker_rejects_it` — given undeclared output role when processed then worker rejects it (`src/runtime/signal/operator.rs:2443`; `test-92b28d962abbd9e52c60`).
- `given_wrong_output_class_when_processed_then_worker_rejects_it` — given wrong output class when processed then worker rejects it (`src/runtime/signal/operator.rs:2420`; `test-fa8ba14489ae1b1a99fa`).
- `given_connector_public_surface_when_inspected_then_managed_aliases_are_absent` — given connector public surface when inspected then managed aliases are absent (`tests/connector_contract.rs:26`; `test-81c56797a2883f88930a`).

## Failure signals

- `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostError` / `AlreadyReaped` — `error-c2bafd8527c7d490c2ad`
- `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostError` / `Kill` — `error-0ae879a99c0152bb21b1`
- `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostError` / `ProcessingTimeout` — `error-1a4656e895904a11844b`
- `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostError` / `UnknownSidecar` — `error-05dae13ea773072e958c`
- `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostError` / `Wait` — `error-bd84a65821e9e309a3c5`
- `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostError` — `error-216966e028c93292ad0e`
- `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostError` / `Closed` — `error-bf9b3507356148f9eff7`
- `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostError` / `ControlQueueFull` — `error-a6aa6d1e23c8870d9145`
- `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostError` / `DataQueueFull` — `error-9e4d5596e287983ab54d`
- `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostError` / `FrameTooLarge` — `error-e4b7949af0ed720f36ab`

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

The claims on **Host a managed-process sidecar** are anchored to Git snapshot `136e74888962558aa846d3143a19136a70936f45` and these primary files:

- `src/runtime/lifecycle/sidecar_host.rs:1-734` (`DIRECT`)

For **Host a managed-process sidecar**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.
