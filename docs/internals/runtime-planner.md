# Runtime planner

<!-- claims: CLM-DOC-049-CAP-001,CLM-DOC-049-CAP-002,CLM-DOC-049-SOURCE-001 -->

## Scope

- **Compile Session declarations.** Validate declarations, resolve bindings, and lower a Session specification into an executable plan.
- **Prepare runtime resources.** Prepare source and endpoint runtimes while preserving the mapping back to declaration identities.

The scope of **Runtime planner** ends at the native contracts and executable conditions cited below. Platform qualification, performance, retry, and delivery require their own explicit evidence.

## Ownership map

- `src/graph/compile/plan.rs` owns part of this boundary.
- `src/session/compile/compiled.rs` owns part of this boundary.

## Compiler-visible surface

| Public declaration | Kind | Declared purpose | Source |
|---|---|---|---|
| `pocketstation::graph::compile::plan::RuntimePlanner` | struct | Validates the graph and produces the bounded runtime execution and memory plan. | `src/graph/compile/plan.rs:11` |
| `pocketstation::session::prepare::prepare_session_runtime` | function | Prepares session runtime for `prepare`. | `src/session/prepare/mod.rs:33` |
| `pocketstation::graph::compile::resolve::Compiler` | struct | Runs the ordered graph-validation passes that resolve a graph specification into executable IR. | `src/graph/compile/resolve.rs:444` |
| `pocketstation::session::compile::SessionCompiler` | struct | Compiles an immutable Session declaration into a validated graph and runtime plan. | `src/session/compile/mod.rs:41` |
| `pocketstation::session::compile::compiled::CompiledSession` | struct | Owns the validated Session specification and declarations produced by compilation. | `src/session/compile/compiled.rs:13` |
| `pocketstation::session::prepare::mappings::PreparedOperatorMapping` | struct | Correlates the prepared identities and runtime resources for prepared operator. | `src/session/prepare/mappings.rs:160` |
| `pocketstation::session::prepare::mappings::PreparedSignalRouteMapping` | struct | Correlates the prepared identities and runtime resources for prepared signal route. | `src/session/prepare/mappings.rs:131` |
| `pocketstation::session::prepare::mappings::PreparedSourceMapping` | struct | Correlates the prepared identities and runtime resources for prepared source. | `src/session/prepare/mappings.rs:18` |
| `pocketstation::session::prepare::mappings::PreparedWorkerMapping` | struct | Correlates the prepared identities and runtime resources for prepared worker. | `src/session/prepare/mappings.rs:35` |
| `pocketstation::session::prepare::prepared::PreparedSession` | struct | Setup-time ownership for one compiled Session. | `src/session/prepare/prepared.rs:18` |
| `pocketstation::graph::compile::resolve::CompileError` | enum | Classifies failures reported as compile error. | `src/graph/compile/resolve.rs:26` |
| `cancellation_requested` | function | Returns whether cancellation requested is true for `PreparedSession`. | `src/session/prepare/prepared.rs:73` |
| `compile` | function | Compiles its owned operation for `Compiler`. | `src/graph/compile/resolve.rs:464` |
| `compile` | function | Compiles its owned operation for `SessionCompiler`. | `src/session/compile/mod.rs:103` |
| `default` | function | Returns the default `RuntimePlanner` value. | `src/graph/compile/plan.rs:349` |
| `default` | function | Returns the default `Compiler` value. | `src/graph/compile/resolve.rs:513` |
| `edge_count` | function | Returns the edge count held by `CompiledSession`. | `src/session/compile/compiled.rs:52` |
| `endpoint_declarations` | function | Returns the endpoint declarations associated with `CompiledSession`. | `src/session/compile/compiled.rs:42` |
| `endpoint_id` | function | Returns the endpoint identifier held by `PreparedWorkerMapping`. | `src/session/prepare/mappings.rs:253` |
| `external_source_declarations` | function | Returns the external source declarations associated with `CompiledSession`. | `src/session/compile/compiled.rs:37` |
| `new` | function | Creates a new `RuntimePlanner`. | `src/graph/compile/plan.rs:14` |
| `new` | function | Creates a new `Compiler`. | `src/graph/compile/resolve.rs:449` |
| `new` | function | Creates a new `SessionCompiler`. | `src/session/compile/mod.rs:77` |
| `node_configuration` | function | Returns the node configuration held by `PreparedWorkerMapping`. | `src/session/prepare/mappings.rs:258` |

## Observed implementation patterns

- `typed_error` — `src/runtime/audio/runner.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `buffer_pool` — `src/session/extensions/audio_input/source.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `typed_error` — `src/session/declaration/typed_stream.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `buffer_pool` — `src/runtime/nodes.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `clock_correlation` — `src/session/lifecycle/running.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `buffer_pool` — `benches/runtime_plan.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `typed_error` — `src/session/extensions/tests/runtime.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `transactional_registration` — `src/session/lifecycle/events.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `typed_error` — `src/session/compile/tests.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `typed_error` — `src/session/extensions/tests/registry.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `transactional_registration` — `src/session/lifecycle/endpoint_transaction.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `buffer_pool` — `src/session/extensions/builtins.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `buffer_pool` — `src/graph/signal/envelope.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `typed_error` — `src/graph/named_ports.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `typed_error` — `src/session/lifecycle/host.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `bounded_queue` — `src/session/lifecycle/trace.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `typed_error` — `examples/product_quickstart.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `buffer_pool` — `src/runtime/audio/executor.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `typed_error` — `examples/operator-consumer/src/lib.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `typed_error` — `src/session/lifecycle/endpoint_transaction.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `transactional_registration` — `src/session/lifecycle/mod.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `typed_error` — `src/session/extensions/audio_input/buffer.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `typed_error` — `src/runtime/lifecycle/async_host.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `typed_error` — `src/graph/runtime_node.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `clock_correlation` — `src/session/extensions/audio_input/source.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `buffer_pool` — `src/runtime/audio/runner.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `buffer_pool` — `src/session/extensions/tests/runtime.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `transactional_registration` — `src/session/lifecycle/engine.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `typed_error` — `src/graph/signal/operator.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `transactional_registration` — `src/session/lifecycle/tests/running.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).

## Behavioral evidence

Executable evidence selected for **Runtime planner** is limited to each test's recorded setup and assertions:

- `given_fixed_graph_when_planned_then_runtime_plan_matches_golden_snapshot` — given fixed graph when planned then runtime plan matches golden snapshot (`src/graph/compile/plan.rs:803`; `test-79ec9d2ff6b56808169b`).
- `given_compiled_graph_when_instrumented_then_metric_ids_are_stable_and_distinct` — given compiled graph when instrumented then metric ids are stable and distinct (`src/graph/compile/plan.rs:785`; `test-1c092b4376cfedf5e86d`).
- `given_copy_to_branch_pool_edge_when_planned_then_copy_pool_memory_is_reserved` — given copy to branch pool edge when planned then copy pool memory is reserved (`src/graph/compile/plan.rs:734`; `test-1d8783b870fa351a933b`).
- `given_explicit_jitter_budget_when_planned_then_bounded_capacity_is_derived_from_frame_time` — given explicit jitter budget when planned then bounded capacity is derived from frame time (`src/graph/compile/plan.rs:755`; `test-28b5b52333fa1672705d`).
- `given_linear_realtime_chain_when_planned_then_single_partition_and_topo_order` — given linear realtime chain when planned then single partition and topo order (`src/graph/compile/plan.rs:834`; `test-9570c9e13e72112e73ef`).
- `given_linear_realtime_graph_when_planned_then_single_partition_in_topo_order` — given linear realtime graph when planned then single partition in topo order (`src/graph/compile/plan.rs:540`; `test-0ad4ea1abd7124d12740`).
- `given_many_input_port_with_multiple_sources_when_planned_then_one_fan_in_group` — given many input port with multiple sources when planned then one fan in group (`src/graph/compile/plan.rs:664`; `test-e8c1159b6f21b80587d2`).
- `given_move_exclusive_edge_in_fan_out_when_planned_then_ownership_is_rejected` — given move exclusive edge in fan out when planned then ownership is rejected (`src/graph/compile/plan.rs:640`; `test-6148b3a94bda4119135e`).
- `given_output_feeding_two_edges_when_planned_then_one_fan_out_group_with_two_targets` — given output feeding two edges when planned then one fan out group with two targets (`src/graph/compile/plan.rs:610`; `test-42437c4cb8f955f4df43`).
- `given_realtime_and_model_remote_nodes_when_planned_then_two_partitions_ordered_by_rank` — given realtime and model remote nodes when planned then two partitions ordered by rank (`src/graph/compile/plan.rs:561`; `test-917d8ff26f7ff8e13525`).
- `given_realtime_consumers_when_planned_then_every_edge_buffered_and_pool_positive` — given realtime consumers when planned then every edge buffered and pool positive (`src/graph/compile/plan.rs:706`; `test-977e6826ff8e82a0a94c`).
- `given_realtime_to_external_edge_when_planned_then_branch_pool_isolated_from_capture_pool` — given realtime to external edge when planned then branch pool isolated from capture pool (`src/graph/compile/plan.rs:591`; `test-ab3f88fbe7eaddfa92c8`).

## Stability boundary

**Runtime planner** describes internal ownership. Its private module layout is not a compatibility promise; compatibility comes from exported Rust declarations, the C header, manifests, error codes, and explicit compatibility artifacts.

## Related documentation

- [Build, prepare, and start](/docs/lifecycle/build-prepare-start.md)
- [Glossary](/docs/glossary.md)
- [PocketStation](/README.md)
- [Prepare resources before start](/docs/how-to/prepare-session.md)
- [Behavior evidence index](/docs/reference/behavior-evidence.md)
- [Rust API reference](/docs/reference/rust-api.md)
- [Session API](/docs/reference/session.md)
- [Session fails before start](/docs/troubleshooting/session-start.md)

## Evidence boundary

The claims on **Runtime planner** are anchored to Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `src/graph/compile/plan.rs:1-860` (`DIRECT`)
- `src/session/compile/compiled.rs:1-107` (`DIRECT`)

For **Runtime planner**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.
