# Runtime planner

<!-- claims: CLM-DOC-049-CAP-001,CLM-DOC-049-CAP-002,CLM-DOC-049-SOURCE-001 -->

## Scope

- **Compile Session declarations.** Validate declarations, resolve bindings, and lower a Session specification into an executable plan.
- **Prepare runtime resources.** Prepare source and endpoint runtimes while preserving the mapping back to declaration identities.

These statements describe repository contracts at the documented snapshot. They do not extend platform qualification, performance, retry, or delivery guarantees beyond the native API contracts and executable evidence.

## Ownership map

- `src/graph/compile/plan.rs` owns part of this boundary.
- `src/session/compile/compiled.rs` owns part of this boundary.

## Compiler-visible surface

| Public declaration | Kind | Declared purpose | Source |
|---|---|---|---|
| `pocketstation::graph::runtime_node::RuntimeNode` | trait | Realtime invariant: for nodes whose ExecutionClass::is_realtime is true, process() must stay alloc-free, lock-free, log-free, and blocking-free (LAW 15). All working state is sized once in prepare() and reused for the lifetime of the node. | `src/graph/runtime_node.rs:7` |
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
| `RuntimeNode::prepare` | function | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/graph/runtime_node.rs:8` |
| `RuntimeNode::process` | function | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/graph/runtime_node.rs:9` |
| `runtime_events_total` | function | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/session/lifecycle/start_contract.rs:358` |
| `runtime_failures_total` | function | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/session/lifecycle/start_contract.rs:346` |
| `runtime_worker_panicked` | function | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/session/lifecycle/start_contract.rs:342` |
| `pocketstation::runtime::signal::io::AsyncOperatorOutputObservations` | type_alias | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/runtime/signal/io.rs:77` |

## Observed implementation patterns

- `typed_error` — `src/runtime/audio/runner.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `buffer_pool` — `src/session/extensions/audio_input/source.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `typed_error` — `src/session/declaration/typed_stream.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `buffer_pool` — `src/runtime/nodes.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `clock_correlation` — `src/session/lifecycle/running.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `sidecar_isolation` — `src/runtime/lifecycle/async_host.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `buffer_pool` — `benches/runtime_plan.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `typed_error` — `src/session/extensions/tests/runtime.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `bounded_queue` — `src/graph/ports.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `typed_error` — `src/session/compile/tests.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `typed_error` — `src/session/extensions/tests/registry.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `transactional_registration` — `src/session/lifecycle/endpoint_transaction.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `sidecar_isolation` — `benches/runtime_plan.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `sidecar_isolation` — `src/session/mod.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `buffer_pool` — `src/session/extensions/builtins.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `buffer_pool` — `src/graph/signal/envelope.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `bounded_queue` — `src/graph/compile/plan.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `typed_error` — `src/graph/named_ports.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `typed_error` — `src/session/lifecycle/host.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `typed_error` — `examples/product_quickstart.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `sidecar_isolation` — `src/session/lifecycle/observations.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `buffer_pool` — `src/runtime/audio/executor.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `buffer_pool` — `src/graph/plan.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `sidecar_isolation` — `src/runtime/signal/operator.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `typed_error` — `examples/operator-consumer/src/lib.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `typed_error` — `src/session/lifecycle/endpoint_transaction.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `transactional_registration` — `src/session/lifecycle/mod.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `typed_error` — `src/session/extensions/audio_input/buffer.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `typed_error` — `src/runtime/lifecycle/async_host.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `typed_error` — `src/graph/runtime_node.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).

## Behavioral evidence

The following test bodies are evidence only for their recorded setup:

- `given_fixed_graph_when_planned_then_runtime_plan_matches_golden_snapshot` — given fixed graph when planned then runtime plan matches golden snapshot (`src/graph/compile/plan.rs:803`; `test-79ec9d2ff6b56808169b`).
- `given_internal_runtime_node_ids_when_audited_then_they_use_endpoint_vocabulary` — given internal runtime node ids when audited then they use endpoint vocabulary (`src/graph/identifier.rs:224`; `test-4134b294aea35bdc9255`).
- `given_unknown_named_input_when_compiled_then_failure_precedes_runtime` — given unknown named input when compiled then failure precedes runtime (`src/graph/named_ports.rs:87`; `test-547f292fc53c973e1c33`).
- `given_duplicate_runtime_node_when_registered_then_first_authority_is_preserved` — given duplicate runtime node when registered then first authority is preserved (`src/graph/registry.rs:214`; `test-aa4e55522d6667752261`).
- `given_sync_caller_when_future_executes_then_result_returns_from_owned_runtime` — given sync caller when future executes then result returns from owned runtime (`src/runtime/lifecycle/async_host.rs:120`; `test-d5a7aaac26a126b55f7d`).
- `given_tokio_caller_when_sync_lifecycle_executes_then_no_nested_runtime_panics` — given tokio caller when sync lifecycle executes then no nested runtime panics (`src/runtime/lifecycle/async_host.rs:129`; `test-905856446bbeb46b221d`).
- `given_prepare_context_capacity_disagrees_with_runtime_edge_when_spawned_then_prepare_fails_closed` — given prepare context capacity disagrees with runtime edge when spawned then prepare fails closed (`src/runtime/signal/operator.rs:1808`; `test-ef78893c6bb92b613da0`).
- `given_compiled_derived_route_when_runtime_prepared_then_compiled_topology_is_preserved` — given compiled derived route when runtime prepared then compiled topology is preserved (`src/session/compile/tests.rs:659`; `test-f38493cc0593f603aece`).
- `given_required_named_input_missing_when_compiled_then_failure_precedes_graph_runtime` — given required named input missing when compiled then failure precedes graph runtime (`src/session/compile/tests.rs:384`; `test-99e881c59d26f6126f74`).
- `given_deterministic_capture_when_polled_then_real_runtime_branch_copy_and_lineage_are_exposed` — given deterministic capture when polled then real runtime branch copy and lineage are exposed (`src/session/lifecycle/tests/engine.rs:440`; `test-162e3f6748e6f4f9bf07`).
- `given_one_source_failure_when_runtime_continues_then_healthy_source_frame_is_delivered` — given one source failure when runtime continues then healthy source frame is delivered (`src/session/lifecycle/tests/running.rs:1468`; `test-35a321e64379e644e1b7`).
- `given_external_consumer_when_declared_then_provider_and_typed_endpoint_use_public_api` — given external consumer when declared then provider and typed endpoint use public api (`examples/operator-consumer/src/lib.rs:120`; `test-ace9b7d11da2036ce899`).

## Stability boundary

This page explains internals. Public compatibility comes from exported Rust declarations, the C header, manifests, error codes, and explicit compatibility artifacts—not private module layout.

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

This page was verified against Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `src/graph/compile/plan.rs:1-860` (`DIRECT`)
- `src/session/compile/compiled.rs:1-107` (`DIRECT`)

A file's presence proves implementation or declaration at this snapshot. It does not by itself prove physical-device qualification, operational performance, retry safety, or behavior outside the recorded test conditions.
