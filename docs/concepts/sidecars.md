# Sidecar lifecycle

<!-- claims: CLM-DOC-029-SCOPE-001,CLM-DOC-029-TEXT-001,CLM-DOC-029-TEXT-002,CLM-DOC-029-TEXT-003,CLM-DOC-029-TEXT-004,CLM-DOC-029-TEXT-005,CLM-DOC-029-TEXT-006,CLM-DOC-029-SOURCE-001 -->

## What it is

A sidecar is a managed child process that exchanges bounded messages with a host through an explicit startup, readiness, cancellation, drain, abort, and terminal-state protocol.

## Why it exists

Some integrations must run outside the Rust process. The sidecar contract keeps process lifetime, message limits, and deadlines observable instead of treating the child as an unbounded subprocess call.

## Relationships

- `SidecarProcessSpec` declares the command and protocol limits.
- `SidecarHost` owns the child and lifecycle deadlines.
- A connector can adapt the sidecar boundary through its driver contract.

## Invariants and guarantees

- Messages remain within declared kinds and byte limits.
- Drain and abort are different shutdown modes.
- Process isolation does not imply authentication or sandboxing.

## When you encounter it

- **Host an out-of-process worker** — Spawn a sidecar and enforce bounded messages, deadlines, cancellation, and terminal state.

## Use it

- [Host a managed-process sidecar](/docs/how-to/host-sidecar.md)
- [A sidecar misses a deadline](/docs/troubleshooting/sidecar-deadline.md)
- [Security boundaries](/docs/security/boundaries.md)

## Scope

- **Host managed-process sidecars.** Exchange bounded protocol messages with a child process under explicit deadlines and lifecycle states.

The scope of **Sidecar lifecycle** ends at the native contracts and executable conditions cited below. Platform qualification, performance, retry, and delivery require their own explicit evidence.

## Key API

| Public declaration | Kind | Declared purpose | Source |
|---|---|---|---|
| `pocketstation::runtime::lifecycle::sidecar_host::SidecarDeadlines` | struct | Sets finite startup, I/O, shutdown, and reap deadlines for a sidecar process. | `src/runtime/lifecycle/sidecar_host.rs:54` |
| `pocketstation::runtime::lifecycle::sidecar_host::SidecarHost` | struct | Owns the resources and lifecycle for sidecar. | `src/runtime/lifecycle/sidecar_host.rs:157` |
| `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostObservations` | struct | Reports the sidecar host observations collected at an observation boundary. | `src/runtime/lifecycle/sidecar_host.rs:109` |
| `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostSnapshot` | struct | Reports the sidecar host snapshot collected at an observation boundary. | `src/runtime/lifecycle/sidecar_host.rs:133` |
| `pocketstation::runtime::lifecycle::sidecar_host::SidecarProcessSpec` | struct | Configures sidecar process behavior at its owning API boundary. | `src/runtime/lifecycle/sidecar_host.rs:71` |
| `pocketstation::runtime::lifecycle::sidecar_protocol::SidecarMessage` | struct | Carries one typed control or signal message across the sidecar protocol. | `src/runtime/lifecycle/sidecar_protocol.rs:73` |
| `pocketstation::runtime::lifecycle::sidecar_protocol::SidecarProtocolLimits` | struct | Sets the maximum sidecar message and buffered-byte sizes enforced by protocol I/O. | `src/runtime/lifecycle/sidecar_protocol.rs:43` |
| `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostError` | enum | Classifies failures surfaced by sidecar host operations. | `src/runtime/lifecycle/sidecar_host.rs:686` |
| `pocketstation::runtime::lifecycle::sidecar_host::SidecarState` | enum | Selects the sidecar state used by PocketStation. | `src/runtime/lifecycle/sidecar_host.rs:21` |
| `pocketstation::runtime::lifecycle::sidecar_protocol::SidecarMessageKind` | enum | Selects the sidecar message kind used by PocketStation. | `src/runtime/lifecycle/sidecar_protocol.rs:9` |

## Executable evidence

Executable evidence selected for **Sidecar lifecycle** is limited to each test's recorded setup and assertions:

- `given_empty_input_group_when_sidecar_prepares_then_structured_error_is_returned` — given empty input group when sidecar prepares then structured error is returned (`src/connector/sidecar.rs:270`; `test-a819c552a02a127c977d`).
- `given_sidecar_host_errors_when_classified_then_retryability_is_preserved` — given sidecar host errors when classified then retryability is preserved (`src/connector/sidecar.rs:286`; `test-72a5c76707ff849957fa`).
- `given_tokio_caller_when_sync_lifecycle_executes_then_no_nested_runtime_panics` — given tokio caller when sync lifecycle executes then no nested runtime panics (`src/runtime/lifecycle/async_host.rs:129`; `test-11ac2e93d34a2efd0e98`).
- `given_core_extension_oversized_sidecar_payload_when_encoded_then_fails_closed` — given core extension oversized sidecar payload when encoded then fails closed (`src/runtime/lifecycle/sidecar_protocol.rs:346`; `test-6c04502719d9fc0cdb98`).
- `given_core_extension_sidecar_message_when_round_tripped_then_identity_is_stable` — given core extension sidecar message when round tripped then identity is stable (`src/runtime/lifecycle/sidecar_protocol.rs:327`; `test-0d42eb96d05d1140bbc3`).
- `given_provider_owned_field_name_when_resolved_then_core_preserves_it_opaquely` — given provider owned field name when resolved then core preserves it opaquely (`src/connector/configuration.rs:642`; `test-c7a1a4edccbfbf6d9c04`).
- `given_audio_record_when_round_tripped_then_transport_and_lineage_identity_are_preserved` — given audio record when round tripped then transport and lineage identity are preserved (`src/connector/transport.rs:633`; `test-415f6fa4b5693884f093`).
- `given_invalid_audio_record_when_decoded_then_trailing_and_oversized_payloads_are_rejected` — given invalid audio record when decoded then trailing and oversized payloads are rejected (`src/connector/transport.rs:642`; `test-ea1a0a780bd315ebcbb6`).
- `given_invalid_configuration_record_when_decoded_then_unknown_kinds_and_trailing_bytes_are_rejected` — given invalid configuration record when decoded then unknown kinds and trailing bytes are rejected (`src/connector/transport.rs:691`; `test-61989b4b80b2e98a9e25`).
- `given_typed_configuration_when_round_tripped_then_types_and_secret_redaction_are_preserved` — given typed configuration when round tripped then types and secret redaction are preserved (`src/connector/transport.rs:660`; `test-615c91bbd90e08b449bb`).
- `given_drain_then_abort_when_requested_then_shutdown_intent_upgrades_monotonically` — given drain then abort when requested then shutdown intent upgrades monotonically (`src/connector/worker/coordination.rs:216`; `test-01257fc4936a2d7e629a`).
- `given_connected_gain_plan_when_executed_then_only_connected_nodes_run_and_worker_receives_output` — given connected gain plan when executed then only connected nodes run and worker receives output (`src/runtime/audio/executor.rs:331`; `test-3f9281677e5af26dc9ad`).

## Related documentation

- [Glossary](/docs/glossary.md)
- [PocketStation](/README.md)
- [Host a managed-process sidecar](/docs/how-to/host-sidecar.md)
- [Behavior evidence index](/docs/reference/behavior-evidence.md)
- [Configuration reference](/docs/reference/configuration.md)
- [Lifecycle evidence index](/docs/reference/lifecycle-evidence.md)
- [Protocol surface index](/docs/reference/protocol-surface.md)
- [Rust API reference](/docs/reference/rust-api.md)

## Evidence boundary

The claims on **Sidecar lifecycle** are anchored to Git snapshot `136e74888962558aa846d3143a19136a70936f45` and these primary files:

- `src/runtime/lifecycle/sidecar_host.rs:13-13` (`DIRECT`)
- `src/runtime/lifecycle/sidecar_host.rs:14-14` (`DIRECT`)
- `src/runtime/lifecycle/sidecar_host.rs:15-15` (`DIRECT`)
- `src/runtime/lifecycle/sidecar_host.rs:16-16` (`DIRECT`)
- `src/runtime/lifecycle/sidecar_host.rs:17-17` (`DIRECT`)
- `src/runtime/lifecycle/sidecar_host.rs:19-19` (`DIRECT`)
- `src/runtime/lifecycle/sidecar_host.rs:19-19` (`DIRECT`)
- `src/runtime/lifecycle/sidecar_host.rs:19-19` (`DIRECT`)
- `src/runtime/lifecycle/sidecar_host.rs:21-33` (`DIRECT`)
- `src/runtime/lifecycle/sidecar_host.rs:22-22` (`DIRECT`)
- `src/runtime/lifecycle/sidecar_host.rs:23-23` (`DIRECT`)
- `src/runtime/lifecycle/sidecar_host.rs:24-24` (`DIRECT`)
- `src/runtime/lifecycle/sidecar_host.rs:25-25` (`DIRECT`)
- `src/runtime/lifecycle/sidecar_host.rs:26-26` (`DIRECT`)
- `src/runtime/lifecycle/sidecar_host.rs:27-27` (`DIRECT`)
- `src/runtime/lifecycle/sidecar_host.rs:28-28` (`DIRECT`)
- `src/runtime/lifecycle/sidecar_host.rs:29-29` (`DIRECT`)
- `src/runtime/lifecycle/sidecar_host.rs:30-30` (`DIRECT`)
- `src/runtime/lifecycle/sidecar_host.rs:31-31` (`DIRECT`)
- `src/runtime/lifecycle/sidecar_host.rs:32-32` (`DIRECT`)
- `src/runtime/lifecycle/sidecar_host.rs:36-50` (`DIRECT`)
- `src/runtime/lifecycle/sidecar_host.rs:53-53` (`DIRECT`)
- `src/runtime/lifecycle/sidecar_host.rs:53-53` (`DIRECT`)
- `src/runtime/lifecycle/sidecar_host.rs:53-53` (`DIRECT`)

For **Sidecar lifecycle**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.
