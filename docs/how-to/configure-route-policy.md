# Choose route capacity and loss policy

<!-- claims: CLM-GUIDE-008-CAP-001,CLM-GUIDE-008-CAP-002,CLM-GUIDE-008-SOURCE-001 -->

## Scope

- **Route realtime audio.** Deliver pooled audio frames through independent fixed-capacity routes governed by explicit edge policy.
- **Describe graph contracts.** Declare typed ports, media capabilities, partition safety, copy, loss, delivery, and observability policy.

These statements describe repository contracts at the documented snapshot. They do not extend platform qualification, performance, retry, or delivery guarantees beyond the native API contracts and executable evidence.

## Prerequisites

Read the linked concept and confirm that target platform, Cargo features, source or provider dependencies, and application-owned permission work match this task. Keep returned typed errors and outcomes available for verification.

## Procedure

1. Identify producer and consumer partitions.
2. Choose finite capacity.
3. Select backpressure, loss, copy, delivery, and observation policies.
4. Compile and handle rejected contracts.
5. Measure queue depth, saturation, and drops before changing capacity.

## APIs used

| Public declaration | Kind | Declared purpose | Source |
|---|---|---|---|
| `pocketstation::graph::ports::LossPolicy` | enum | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/graph/ports.rs:287` |
| `pocketstation::graph::ports::LossPolicy::ConcealForAudio` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/graph/ports.rs:288` |
| `pocketstation::graph::ports::LossPolicy::DropAllowed` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/graph/ports.rs:290` |
| `pocketstation::graph::ports::LossPolicy::MustDeliverOrFail` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/graph/ports.rs:289` |
| `pocketstation::graph::signal::operator::OperatorDeadlinePolicy` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/graph/signal/operator.rs:52` |
| `pocketstation::graph::signal::operator::OperatorOutputRolePolicy` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/graph/signal/operator.rs:69` |
| `pocketstation::graph::signal::operator::OperatorPermissionPolicy` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/graph/signal/operator.rs:46` |
| `pocketstation::runtime::audio::router::EdgeObservations` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/runtime/audio/router.rs:122` |
| `pocketstation::graph::ports::BackpressurePolicy` | enum | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/graph/ports.rs:265` |
| `pocketstation::graph::ports::CopyPolicy` | enum | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/graph/ports.rs:280` |
| `pocketstation::graph::signal::operator::OperatorCancellationPolicy` | enum | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/graph/signal/operator.rs:57` |
| `pocketstation::graph::signal::operator::OperatorFailurePolicy` | enum | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/graph/signal/operator.rs:63` |
| `capacity_signals` | function | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/graph/node.rs:361` |
| `copy_policy` | function | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/graph/ports.rs:353` |
| `loss` | function | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/graph/ports.rs:349` |
| `policy_epoch` | function | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/graph/signal/lineage.rs:80` |

## Verify the outcome

The following test bodies are evidence only for their recorded setup:

- `given_explicit_jitter_budget_when_planned_then_bounded_capacity_is_derived_from_frame_time` — given explicit jitter budget when planned then bounded capacity is derived from frame time (`src/graph/compile/plan.rs:755`; `test-28b5b52333fa1672705d`).
- `given_compiled_text_edge_when_router_builds_then_only_audio_edge_gets_audio_receiver` — given compiled text edge when router builds then only audio edge gets audio receiver (`src/runtime/audio/router.rs:983`; `test-c5f24b62056cfa546c3a`).
- `given_retained_audio_ingress_when_pool_is_exhausted_then_loss_is_counted_exactly` — given retained audio ingress when pool is exhausted then loss is counted exactly (`src/runtime/bridge/audio.rs:459`; `test-1664fa1aa12573253d70`).
- `given_capacity_above_global_bound_when_fanout_built_then_setup_fails` — given capacity above global bound when fanout built then setup fails (`src/runtime/signal/edge.rs:575`; `test-f0893eafe636572bd65e`).
- `given_full_terminal_branch_when_finished_then_final_loss_fails_closed` — given full terminal branch when finished then final loss fails closed (`src/runtime/signal/operator.rs:2371`; `test-df034af78b2cc7bf98f9`).
- `given_prepare_context_capacity_disagrees_with_runtime_edge_when_spawned_then_prepare_fails_closed` — given prepare context capacity disagrees with runtime edge when spawned then prepare fails closed (`src/runtime/signal/operator.rs:1808`; `test-ef78893c6bb92b613da0`).
- `given_preallocated_three_edge_router_when_frame_dispatched_then_no_heap_allocation_occurs` — given preallocated three edge router when frame dispatched then no heap allocation occurs (`tests/runtime_plan_router_alloc.rs:53`; `test-49ea3414f8bb90cf86b1`).
- `given_external_consumer_when_declared_then_provider_and_typed_endpoint_use_public_api` — given external consumer when declared then provider and typed endpoint use public api (`examples/operator-consumer/src/lib.rs:120`; `test-ace9b7d11da2036ce899`).
- `given_gain_config_with_non_numeric_gain_db_when_validate_then_invalid_error` — given gain config with non numeric gain db when validate then invalid error (`src/graph/builtins.rs:256`; `test-0e41065f28a838e0deaf`).
- `given_gain_config_with_valid_gain_db_when_validate_then_ok` — given gain config with valid gain db when validate then ok (`src/graph/builtins.rs:264`; `test-c5d54824499f245c4c6c`).
- `given_gain_config_without_gain_db_when_validate_then_missing_error` — given gain config without gain db when validate then missing error (`src/graph/builtins.rs:249`; `test-c2584e0bcdbbb154dfa1`).
- `given_mono_frame_when_mono_mixed_then_frame_is_unchanged` — given mono frame when mono mixed then frame is unchanged (`src/graph/builtins.rs:330`; `test-d76ec44bacdca3f6a506`).

## Failure signals

- `pocketstation::graph::node::NodeDescriptorError` / `InvalidSafetyContract` — `error-04b7031025a9b635fdbf`
- `pocketstation::graph::node::ConfigError` — `error-0be8ad81000b2924c24c`
- `pocketstation::graph::compile::resolve::CompileError` — `error-0da3f91a5f274a27ab76`
- `pocketstation::graph::signal::operator::AsyncOperatorManifestError` / `ZeroProcessTimeout` — `error-10e3a522fa28fccdfc60`
- `pocketstation::runtime::lifecycle::sidecar_protocol::SidecarProtocolError` / `InvalidMagic` — `error-143cce14f0e71f68c4cf`
- `pocketstation::graph::signal::operator::OperatorFailurePolicy` / `StopWorker` — `error-14ca51fa44623142d004`
- `pocketstation::graph::node::NodeError` / `Process` — `error-170066b0b40a26e0e33d`
- `pocketstation::graph::signal::continuity::SignalContinuityError` / `SequenceGapWithoutDiscontinuity` — `error-18565faf820bbf8e2650`
- `pocketstation::graph::compile::resolve::CompileError` / `MediaMismatch` — `error-1877b4a7bdffa5d7ed88`
- `pocketstation::graph::signal::continuity::SignalContinuityError` / `InvalidEnvelope` — `error-1897c7da4711d75eb14d`
- `pocketstation::graph::plan::PlanError` / `MoveExclusiveFanOut` — `error-18d1485abaf31198b6d8`
- `pocketstation::graph::node::NodeDescriptorError` / `EmptyDisplayName` — `error-1981cbd27763ca5ffcbe`

Retry only when the relevant API or error contract explicitly permits it. An error name, a transient-looking message, or a successful prior run is not retry evidence.

## Related documentation

- [Glossary](/docs/glossary.md)
- [PocketStation](/README.md)
- [Fan out one source](/docs/how-to/fan-out-source.md)
- [Behavior evidence index](/docs/reference/behavior-evidence.md)
- [Rust API reference](/docs/reference/rust-api.md)
- [Frames or signals are dropped](/docs/troubleshooting/drops-and-saturation.md)
- [Size bounded routes from observations](/docs/best-practices/route-sizing.md)
- [Architecture overview](/docs/architecture/overview.md)

## Evidence boundary

This page was verified against Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `src/graph/ports.rs:1-618` (`DIRECT`)

A file's presence proves implementation or declaration at this snapshot. It does not by itself prove physical-device qualification, operational performance, retry safety, or behavior outside the recorded test conditions.
