# Runtime preparation

<!-- claims: CLM-DOC-015-CAP-001,CLM-DOC-015-SOURCE-001 -->

Prepare source and endpoint runtimes while preserving the mapping back to declaration identities.

## Scope

- **Prepare runtime resources.** Prepare source and endpoint runtimes while preserving the mapping back to declaration identities.

These statements describe repository contracts at the documented snapshot. They do not extend platform qualification, performance, retry, or delivery guarantees beyond the native API contracts and executable evidence.

## Contract surface

| Public declaration | Kind | Declared purpose | Source |
|---|---|---|---|
| `pocketstation::session::error_code::SessionRuntimeErrorCode` | enum | Stable language-neutral code for a running-Session projection failure. | `src/session/error_code.rs:116` |
| `pocketstation::runtime::audio::router::EdgeObservations` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/runtime/audio/router.rs:122` |
| `pocketstation::runtime::audio::runner::PlanSourceInputObservations` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/runtime/audio/runner.rs:22` |
| `pocketstation::runtime::lifecycle::sidecar_host::SidecarDeadlines` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/runtime/lifecycle/sidecar_host.rs:54` |
| `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostSnapshot` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/runtime/lifecycle/sidecar_host.rs:133` |
| `pocketstation::runtime::lifecycle::sidecar_host::SidecarProcessSpec` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/runtime/lifecycle/sidecar_host.rs:71` |
| `pocketstation::runtime::lifecycle::sidecar_protocol::SidecarMessage` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/runtime/lifecycle/sidecar_protocol.rs:73` |
| `pocketstation::runtime::lifecycle::sidecar_protocol::SidecarProtocolLimits` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/runtime/lifecycle/sidecar_protocol.rs:43` |
| `pocketstation::runtime::signal::edge::SignalEdgeObservations` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/runtime/signal/edge.rs:31` |
| `pocketstation::runtime::signal::observations::AsyncOperatorObservations` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/runtime/signal/observations.rs:29` |
| `pocketstation::session::extensions::source::SourceRuntimeObservations` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/session/extensions/source.rs:394` |
| `pocketstation::runtime::audio::executor::ExecError` | enum | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/runtime/audio/executor.rs:20` |
| `pocketstation::runtime::audio::runner::PlanRunnerError` | enum | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/runtime/audio/runner.rs:256` |
| `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostError` | enum | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/runtime/lifecycle/sidecar_host.rs:686` |
| `pocketstation::runtime::lifecycle::sidecar_host::SidecarState` | enum | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/runtime/lifecycle/sidecar_host.rs:21` |
| `pocketstation::runtime::lifecycle::sidecar_protocol::SidecarMessageKind` | enum | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/runtime/lifecycle/sidecar_protocol.rs:9` |
| `pocketstation::runtime::lifecycle::sidecar_protocol::SidecarProtocolError` | enum | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/runtime/lifecycle/sidecar_protocol.rs:292` |
| `runtime_events_total` | function | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/session/lifecycle/start_contract.rs:358` |
| `runtime_failures_total` | function | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/session/lifecycle/start_contract.rs:346` |
| `runtime_worker_panicked` | function | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/session/lifecycle/start_contract.rs:342` |

## Where you encounter it

The current capability model has no separate end-to-end journey for this concept.

## Behavior established by tests

The following test bodies are evidence only for their recorded setup:

- `given_sync_caller_when_future_executes_then_result_returns_from_owned_runtime` — given sync caller when future executes then result returns from owned runtime (`src/runtime/lifecycle/async_host.rs:120`; `test-d5a7aaac26a126b55f7d`).
- `given_tokio_caller_when_sync_lifecycle_executes_then_no_nested_runtime_panics` — given tokio caller when sync lifecycle executes then no nested runtime panics (`src/runtime/lifecycle/async_host.rs:129`; `test-905856446bbeb46b221d`).
- `given_prepare_context_capacity_disagrees_with_runtime_edge_when_spawned_then_prepare_fails_closed` — given prepare context capacity disagrees with runtime edge when spawned then prepare fails closed (`src/runtime/signal/operator.rs:1808`; `test-ef78893c6bb92b613da0`).
- `given_compiled_derived_route_when_runtime_prepared_then_compiled_topology_is_preserved` — given compiled derived route when runtime prepared then compiled topology is preserved (`src/session/compile/tests.rs:659`; `test-f38493cc0593f603aece`).
- `given_required_named_input_missing_when_compiled_then_failure_precedes_graph_runtime` — given required named input missing when compiled then failure precedes graph runtime (`src/session/compile/tests.rs:384`; `test-99e881c59d26f6126f74`).
- `given_deterministic_capture_when_polled_then_real_runtime_branch_copy_and_lineage_are_exposed` — given deterministic capture when polled then real runtime branch copy and lineage are exposed (`src/session/lifecycle/tests/engine.rs:440`; `test-162e3f6748e6f4f9bf07`).
- `given_one_source_failure_when_runtime_continues_then_healthy_source_frame_is_delivered` — given one source failure when runtime continues then healthy source frame is delivered (`src/session/lifecycle/tests/running.rs:1468`; `test-35a321e64379e644e1b7`).
- `given_connected_gain_plan_when_executed_then_only_connected_nodes_run_and_worker_receives_output` — given connected gain plan when executed then only connected nodes run and worker receives output (`src/runtime/audio/executor.rs:331`; `test-cd64bb966db1f193ea6f`).
- `given_lineaged_frame_when_realtime_operator_executes_then_output_keeps_capture_epochs` — given lineaged frame when realtime operator executes then output keeps capture epochs (`src/runtime/audio/executor.rs:411`; `test-14905efc2e19f82a8cb2`).
- `given_realtime_fan_out_when_executed_then_each_mutating_branch_gets_independent_copy` — given realtime fan out when executed then each mutating branch gets independent copy (`src/runtime/audio/executor.rs:369`; `test-c0c81ff42570a02c1eb9`).
- `observations` — observations (`src/runtime/audio/executor.rs:185`; `test-8e5dda8471ef4129edb9`).
- `from` — from (`src/runtime/audio/router.rs:510`; `test-bd1711e374cc4ec84e26`).

## Boundaries

The compiler inventory establishes names, kinds, visibility, and signatures. Tests establish only their exercised conditions. Where retryability, ordering, cancellation, physical qualification, or recovery is not declared, this page leaves it unspecified.

## Related documentation

- [Build, prepare, and start](/docs/lifecycle/build-prepare-start.md)
- [Glossary](/docs/glossary.md)
- [PocketStation](/README.md)
- [Runtime planner](/docs/internals/runtime-planner.md)
- [Prepare resources before start](/docs/how-to/prepare-session.md)
- [Behavior evidence index](/docs/reference/behavior-evidence.md)
- [Rust API reference](/docs/reference/rust-api.md)
- [Session API](/docs/reference/session.md)

## Evidence boundary

This page was verified against Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `src/session/prepare/mod.rs:1-1290` (`DIRECT`)

A file's presence proves implementation or declaration at this snapshot. It does not by itself prove physical-device qualification, operational performance, retry safety, or behavior outside the recorded test conditions.
