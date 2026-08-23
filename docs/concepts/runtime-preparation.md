# Runtime preparation

<!-- claims: CLM-DOC-015-CAP-001,CLM-DOC-015-SOURCE-001 -->

## What it is

Session compilation validates a frozen declaration, resolves named bindings and media compatibility, and lowers the result into an executable plan.

## Why it exists

Compilation separates structural mistakes from resource-opening failures. Developers can therefore distinguish an invalid graph from a device, endpoint, or worker that could not be prepared.

## Relationships

- `SessionSpec` is the compiler input.
- Graph contracts provide port, media, partition, and edge requirements.
- `SessionCompileDiagnostic` projects the compile stage, stable error code, affected component IDs, and explanatory fields without reducing the failure to display text.
- `CompiledSession` is passed to preparation, which owns external resource work.

## Invariants and guarantees

- Unknown, duplicate, foreign, or incompatible declarations fail before runtime execution.
- A required named input must be bound.
- Multiple stems connected to one operator retain separate source-aware input bindings.
- Compilation does not prove that a native source or endpoint can start.

## When you encounter it

You encounter runtime preparation through its declaration and runtime APIs.

## Use it

- [Prepare resources before start](/docs/how-to/prepare-session.md)
- [Connect named operator ports](/docs/how-to/connect-operator-ports.md)
- [Session fails before start](/docs/troubleshooting/session-start.md)

## Scope

- **Prepare runtime resources.** Prepare source and endpoint runtimes while preserving the mapping back to declaration identities.

The scope of **Runtime preparation** ends at the native contracts and executable conditions cited below. Platform qualification, performance, retry, and delivery require their own explicit evidence.

## Key API

| Public declaration | Kind | Declared purpose | Source |
|---|---|---|---|
| `pocketstation::session::prepare::prepare_session_runtime` | function | Prepares session runtime for `prepare`. | `src/session/prepare/mod.rs:33` |
| `pocketstation::session::prepare::mappings::PreparedOperatorMapping` | struct | Correlates the prepared identities and runtime resources for prepared operator. | `src/session/prepare/mappings.rs:160` |
| `pocketstation::session::prepare::mappings::PreparedSignalRouteMapping` | struct | Correlates the prepared identities and runtime resources for prepared signal route. | `src/session/prepare/mappings.rs:131` |
| `pocketstation::session::prepare::mappings::PreparedSourceMapping` | struct | Correlates the prepared identities and runtime resources for prepared source. | `src/session/prepare/mappings.rs:18` |
| `pocketstation::session::prepare::mappings::PreparedWorkerMapping` | struct | Correlates the prepared identities and runtime resources for prepared worker. | `src/session/prepare/mappings.rs:35` |
| `pocketstation::session::prepare::prepared::PreparedSession` | struct | Setup-time ownership for one compiled Session. | `src/session/prepare/prepared.rs:18` |
| `cancellation_requested` | function | Returns whether cancellation requested is true for `PreparedSession`. | `src/session/prepare/prepared.rs:73` |
| `endpoint_id` | function | Returns the endpoint identifier held by `PreparedWorkerMapping`. | `src/session/prepare/mappings.rs:253` |
| `node_configuration` | function | Returns the node configuration held by `PreparedWorkerMapping`. | `src/session/prepare/mappings.rs:258` |
| `operator_mappings` | function | Returns the operator mappings associated with `PreparedSession`. | `src/session/prepare/prepared.rs:56` |

## Executable evidence

Executable evidence selected for **Runtime preparation** is limited to each test's recorded setup and assertions:

- `given_sync_caller_when_future_executes_then_result_returns_from_owned_runtime` — given sync caller when future executes then result returns from owned runtime (`src/runtime/lifecycle/async_host.rs:120`; `test-59f57b9ccdba8c9ca59d`).
- `given_tokio_caller_when_sync_lifecycle_executes_then_no_nested_runtime_panics` — given tokio caller when sync lifecycle executes then no nested runtime panics (`src/runtime/lifecycle/async_host.rs:129`; `test-11ac2e93d34a2efd0e98`).
- `given_prepare_context_capacity_disagrees_with_runtime_edge_when_spawned_then_prepare_fails_closed` — given prepare context capacity disagrees with runtime edge when spawned then prepare fails closed (`src/runtime/signal/operator.rs:1808`; `test-94655e5366915899c2bd`).
- `given_compiled_derived_route_when_runtime_prepared_then_compiled_topology_is_preserved` — given compiled derived route when runtime prepared then compiled topology is preserved (`src/session/compile/tests.rs:659`; `test-21f8c08b6457bb762def`).
- `given_required_named_input_missing_when_compiled_then_failure_precedes_graph_runtime` — given required named input missing when compiled then failure precedes graph runtime (`src/session/compile/tests.rs:384`; `test-e868470f819453421dd7`).
- `given_deterministic_capture_when_polled_then_real_runtime_branch_copy_and_lineage_are_exposed` — given deterministic capture when polled then real runtime branch copy and lineage are exposed (`src/session/lifecycle/tests/engine.rs:440`; `test-b1797fb7f4f0913afd38`).
- `given_one_source_failure_when_runtime_continues_then_healthy_source_frame_is_delivered` — given one source failure when runtime continues then healthy source frame is delivered (`src/session/lifecycle/tests/running.rs:1470`; `test-b5fe30e5dbde18fe390e`).
- `given_connected_gain_plan_when_executed_then_only_connected_nodes_run_and_worker_receives_output` — given connected gain plan when executed then only connected nodes run and worker receives output (`src/runtime/audio/executor.rs:331`; `test-3f9281677e5af26dc9ad`).
- `given_lineaged_frame_when_realtime_operator_executes_then_output_keeps_capture_epochs` — given lineaged frame when realtime operator executes then output keeps capture epochs (`src/runtime/audio/executor.rs:411`; `test-aee462488aef78361374`).
- `given_realtime_fan_out_when_executed_then_each_mutating_branch_gets_independent_copy` — given realtime fan out when executed then each mutating branch gets independent copy (`src/runtime/audio/executor.rs:369`; `test-8b303620bdafeb3aa260`).
- `given_compiled_text_edge_when_router_builds_then_only_audio_edge_gets_audio_receiver` — given compiled text edge when router builds then only audio edge gets audio receiver (`src/runtime/audio/router.rs:1012`; `test-687c08c4ebc7699d891b`).
- `given_enqueued_and_dropped_frames_when_observed_then_drop_rate_uses_all_attempts` — given enqueued and dropped frames when observed then drop rate uses all attempts (`src/runtime/audio/router.rs:1272`; `test-81f2a37c65fc1321fb4b`).

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

The claims on **Runtime preparation** are anchored to Git snapshot `136e74888962558aa846d3143a19136a70936f45` and these primary files:

- `src/session/prepare/mod.rs:1-1331` (`DIRECT`)

For **Runtime preparation**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.
