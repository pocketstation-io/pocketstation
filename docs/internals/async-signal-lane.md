# Asynchronous signal lane

<!-- claims: CLM-DOC-051-SCOPE-001,CLM-DOC-051-TEXT-001,CLM-DOC-051-TEXT-002,CLM-DOC-051-TEXT-003,CLM-DOC-051-TEXT-004,CLM-DOC-051-SOURCE-001 -->

## Scope

- **Implement asynchronous operators.** Register operator factories that consume and emit named typed signals on the asynchronous execution lane.
- **Carry typed signals.** Represent audio-adjacent text, event, binary, metric, and custom-schema payloads with timing and lineage.

The scope of **Asynchronous signal lane** ends at the native contracts and executable conditions cited below. Platform qualification, performance, retry, and delivery require their own explicit evidence.

## Ownership map

- `src/runtime/signal/edge.rs` owns part of this boundary.
- `src/runtime/signal/operator.rs` owns part of this boundary.

## Compiler-visible surface

| Public declaration | Kind | Declared purpose | Source |
|---|---|---|---|
| `pocketstation::graph::signal::operator::AsyncNode` | trait | Async operator contract for model, connector, transport, and control-plane work. | `src/graph/signal/operator.rs:13` |
| `pocketstation::graph::signal::operator::AsyncOperatorFactory` | trait | Implement this trait to provide async operator behavior to PocketStation; its methods define the preparation and runtime contract. | `src/graph/signal/operator.rs:368` |
| `pocketstation::graph::signal::envelope::SignalEnvelope` | struct | Carries a typed signal payload together with timing, lineage, continuity, and terminal metadata. | `src/graph/signal/envelope.rs:6` |
| `pocketstation::graph::signal::operator::AsyncOperatorManifest` | struct | Declares an asynchronous operator's ports, execution partition, failure policy, and cancellation policy. | `src/graph/signal/operator.rs:127` |
| `pocketstation::graph::signal::operator::OperatorDeadlinePolicy` | struct | Configures operator deadline behavior at its owning API boundary. | `src/graph/signal/operator.rs:52` |
| `pocketstation::graph::signal::operator::OperatorOutputRolePolicy` | struct | Configures operator output role behavior at its owning API boundary. | `src/graph/signal/operator.rs:69` |
| `pocketstation::graph::signal::operator::OperatorPermissionPolicy` | struct | Configures operator permission behavior at its owning API boundary. | `src/graph/signal/operator.rs:46` |
| `pocketstation::graph::signal::spec::SchemaRef` | struct | Reference to an external schema document. | `src/graph/signal/spec.rs:87` |
| `pocketstation::graph::signal::spec::SemanticRole` | struct | Semantic role annotation on a port. | `src/graph/signal/spec.rs:57` |
| `pocketstation::graph::signal::spec::SignalId` | struct | Opaque identifier for a custom signal type. | `src/graph/signal/spec.rs:22` |
| `pocketstation::graph::signal::spec::SignalSpec` | struct | Full signal contract for a single port. | `src/graph/signal/spec.rs:205` |
| `pocketstation::runtime::signal::edge::SignalEdgeObservationHandle` | struct | Holds the ownership or bounded access represented by signal edge observation handle. | `src/runtime/signal/edge.rs:49` |
| `pocketstation::runtime::signal::edge::SignalEdgeObservations` | struct | Reports the signal edge observations collected at an observation boundary. | `src/runtime/signal/edge.rs:31` |
| `pocketstation::runtime::signal::edge::SignalEdgeReceiver` | struct | Receives signal edge values across its declared ownership boundary. | `src/runtime/signal/edge.rs:204` |
| `pocketstation::runtime::signal::edge::SignalEdgeSendError` | struct | Reports a signal edge send error. | `src/runtime/signal/edge.rs:118` |
| `pocketstation::runtime::signal::edge::TypedEdgeBranchSpec` | struct | Configures typed edge branch behavior at its owning API boundary. | `src/runtime/signal/edge.rs:248` |
| `pocketstation::runtime::signal::edge::TypedEdgeFanout` | struct | Publishes one immutable signal envelope to the bounded branches of a compiled fan-out edge. | `src/runtime/signal/edge.rs:259` |
| `pocketstation::runtime::signal::edge::TypedEdgePublishReport` | struct | Reports how many fan-out branches accepted or dropped one published signal. | `src/runtime/signal/edge.rs:380` |
| `pocketstation::runtime::signal::operator::AsyncOperatorWorker` | struct | Owns the asynchronous operator task, typed I/O, cancellation, and terminal join result. | `src/runtime/signal/operator.rs:250` |
| `pocketstation::runtime::signal::operator::CompiledOperatorInputContract` | struct | Declares the validated constraints applied to compiled operator input. | `src/runtime/signal/operator.rs:103` |
| `pocketstation::graph::signal::envelope::SignalEnvelopeError` | enum | Classifies failures surfaced by signal envelope operations. | `src/graph/signal/envelope.rs:137` |
| `pocketstation::graph::signal::operator::AsyncOperatorManifestError` | enum | Classifies failures surfaced by async operator manifest operations. | `src/graph/signal/operator.rs:321` |
| `pocketstation::graph::signal::operator::OperatorCancellationPolicy` | enum | Selects the operator cancellation policy used by PocketStation. | `src/graph/signal/operator.rs:57` |
| `pocketstation::graph::signal::operator::OperatorFailurePolicy` | enum | Selects the operator failure policy used by PocketStation. | `src/graph/signal/operator.rs:63` |

## Observed implementation patterns

- `typed_error` — `src/runtime/audio/runner.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `buffer_pool` — `src/runtime/nodes.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `buffer_pool` — `benches/runtime_plan.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `typed_error` — `src/graph/signal/continuity.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `clock_correlation` — `src/runtime/signal/edge.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `clock_correlation` — `src/runtime/signal/operator.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `typed_error` — `src/graph/signal/timing.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `buffer_pool` — `src/graph/signal/envelope.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `typed_error` — `src/graph/named_ports.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `typed_error` — `src/graph/signal/preparation.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `buffer_pool` — `src/runtime/audio/executor.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `typed_error` — `examples/operator-consumer/src/lib.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `typed_error` — `src/runtime/lifecycle/async_host.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `typed_error` — `src/graph/runtime_node.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `buffer_pool` — `src/runtime/audio/runner.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `typed_error` — `src/graph/node.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `typed_error` — `src/graph/signal/operator.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `clock_correlation` — `src/runtime/audio/router.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `typed_error` — `src/graph/builtins.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `typed_error` — `src/runtime/nodes.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `typed_error` — `src/graph/signal/spec.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `sidecar_isolation` — `src/runtime/lifecycle/sidecar_protocol.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `sidecar_isolation` — `src/runtime/lifecycle/sidecar_host.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `clock_correlation` — `src/graph/signal/lineage.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `typed_error` — `src/runtime/bridge/audio.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `typed_error` — `src/runtime/signal/io.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `typed_error` — `src/graph/ports.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `typed_error` — `src/runtime/signal/operator.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `clock_correlation` — `src/runtime/bridge/audio.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `typed_error` — `src/runtime/lifecycle/sidecar_protocol.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).

## Behavioral evidence

Executable evidence selected for **Asynchronous signal lane** is limited to each test's recorded setup and assertions:

- `given_full_owned_signal_edge_when_audio_sent_then_frame_returns_without_allocation` — given full owned signal edge when audio sent then frame returns without allocation (`src/runtime/signal/edge.rs:460`; `test-3e7e1369cb8a03a6d22a`).
- `given_one_branch_when_signal_published_then_receiver_has_exclusive_ownership` — given one branch when signal published then receiver has exclusive ownership (`src/runtime/signal/edge.rs:626`; `test-ff5044918a12088e3cc1`).
- `given_registered_signal_consumer_when_item_enqueued_then_parked_thread_is_woken` — given registered signal consumer when item enqueued then parked thread is woken (`src/runtime/signal/edge.rs:469`; `test-2904c9464f0c7db65ee2`).
- `given_every_nonaudio_signal_class_when_worker_prepares_then_exact_signal_context_is_received` — given every nonaudio signal class when worker prepares then exact signal context is received (`src/runtime/signal/operator.rs:1752`; `test-0769819bf6f85fc4186c`).
- `given_capacity_above_global_bound_when_fanout_built_then_setup_fails` — given capacity above global bound when fanout built then setup fails (`src/runtime/signal/edge.rs:575`; `test-c8da19b53530dc618e8e`).
- `given_independent_shared_branches_when_one_saturates_then_other_continues` — given independent shared branches when one saturates then other continues (`src/runtime/signal/edge.rs:493`; `test-1b404c215b1ead1443e9`).
- `given_missing_or_zero_payload_limit_when_fanout_built_then_setup_fails` — given missing or zero payload limit when fanout built then setup fails (`src/runtime/signal/edge.rs:591`; `test-d9d74fd1b5a23d91b838`).
- `given_payload_above_branch_limit_when_published_then_all_branches_reject_before_fanout` — given payload above branch limit when published then all branches reject before fanout (`src/runtime/signal/edge.rs:536`; `test-26e5374e5c20a32afac4`).
- `given_payload_limit_above_global_bound_when_fanout_built_then_setup_fails` — given payload limit above global bound when fanout built then setup fails (`src/runtime/signal/edge.rs:609`; `test-6401065553d368b9596c`).
- `given_audio_output_without_audio_port_when_processed_then_worker_rejects_it` — given audio output without audio port when processed then worker rejects it (`src/runtime/signal/operator.rs:2466`; `test-e0f5021f2a3131ebe15b`).
- `given_cancellation_when_operator_has_pending_state_then_no_final_is_fabricated` — given cancellation when operator has pending state then no final is fabricated (`src/runtime/signal/operator.rs:2308`; `test-245c72f8c086e47e1ada`).
- `given_compiled_lineaged_edge_when_worker_runs_then_exact_session_stem_is_preserved` — given compiled lineaged edge when worker runs then exact session stem is preserved (`src/runtime/signal/operator.rs:2208`; `test-9e1c8ad04d302a8bf88b`).

## Stability boundary

**Asynchronous signal lane** describes internal ownership. Its private module layout is not a compatibility promise; compatibility comes from exported Rust declarations, the C header, manifests, error codes, and explicit compatibility artifacts.

## Related documentation

- [Glossary](/docs/glossary.md)
- [PocketStation](/README.md)
- [Implement an asynchronous operator](/docs/how-to/implement-operator.md)
- [Behavior evidence index](/docs/reference/behavior-evidence.md)
- [Graph and route contracts](/docs/reference/graph.md)
- [Rust API reference](/docs/reference/rust-api.md)
- [Frames or signals are dropped](/docs/troubleshooting/drops-and-saturation.md)
- [Graph and signal failures](/docs/errors/graph-and-signals.md)

## Evidence boundary

The claims on **Asynchronous signal lane** are anchored to Git snapshot `136e74888962558aa846d3143a19136a70936f45` and these primary files:

- `src/runtime/signal/edge.rs:15-15` (`DIRECT`)
- `src/runtime/signal/edge.rs:17-17` (`DIRECT`)
- `src/runtime/signal/edge.rs:18-28` (`DIRECT`)
- `src/runtime/signal/edge.rs:19-19` (`DIRECT`)
- `src/runtime/signal/edge.rs:20-20` (`DIRECT`)
- `src/runtime/signal/edge.rs:21-21` (`DIRECT`)
- `src/runtime/signal/edge.rs:22-22` (`DIRECT`)
- `src/runtime/signal/edge.rs:23-23` (`DIRECT`)
- `src/runtime/signal/edge.rs:24-24` (`DIRECT`)
- `src/runtime/signal/edge.rs:25-25` (`DIRECT`)
- `src/runtime/signal/edge.rs:26-26` (`DIRECT`)
- `src/runtime/signal/edge.rs:27-27` (`DIRECT`)
- `src/runtime/signal/edge.rs:30-30` (`DIRECT`)
- `src/runtime/signal/edge.rs:30-30` (`DIRECT`)
- `src/runtime/signal/edge.rs:30-30` (`DIRECT`)
- `src/runtime/signal/edge.rs:31-46` (`DIRECT`)
- `src/runtime/signal/edge.rs:32-32` (`DIRECT`)
- `src/runtime/signal/edge.rs:33-33` (`DIRECT`)
- `src/runtime/signal/edge.rs:34-34` (`DIRECT`)
- `src/runtime/signal/edge.rs:35-35` (`DIRECT`)
- `src/runtime/signal/edge.rs:36-36` (`DIRECT`)
- `src/runtime/signal/edge.rs:37-37` (`DIRECT`)
- `src/runtime/signal/edge.rs:38-38` (`DIRECT`)
- `src/runtime/signal/edge.rs:44-44` (`DIRECT`)
- `src/runtime/signal/operator.rs:31-31` (`DIRECT`)
- `src/runtime/signal/operator.rs:33-44` (`DIRECT`)
- `src/runtime/signal/operator.rs:35-35` (`DIRECT`)
- `src/runtime/signal/operator.rs:35-35` (`DIRECT`)
- `src/runtime/signal/operator.rs:36-39` (`DIRECT`)
- `src/runtime/signal/operator.rs:37-37` (`DIRECT`)
- `src/runtime/signal/operator.rs:38-38` (`DIRECT`)
- `src/runtime/signal/operator.rs:40-43` (`DIRECT`)
- `src/runtime/signal/operator.rs:41-41` (`DIRECT`)
- `src/runtime/signal/operator.rs:42-42` (`DIRECT`)
- `src/runtime/signal/operator.rs:46-52` (`DIRECT`)
- `src/runtime/signal/operator.rs:47-50` (`DIRECT`)
- `src/runtime/signal/operator.rs:48-48` (`DIRECT`)
- `src/runtime/signal/operator.rs:49-49` (`DIRECT`)
- `src/runtime/signal/operator.rs:51-51` (`DIRECT`)
- `src/runtime/signal/operator.rs:51-51` (`DIRECT`)
- `src/runtime/signal/operator.rs:55-86` (`DIRECT`)
- `src/runtime/signal/operator.rs:88-99` (`DIRECT`)
- `src/runtime/signal/operator.rs:102-102` (`DIRECT`)
- `src/runtime/signal/operator.rs:102-102` (`DIRECT`)
- `src/runtime/signal/operator.rs:102-102` (`DIRECT`)
- `src/runtime/signal/operator.rs:103-114` (`DIRECT`)
- `src/runtime/signal/operator.rs:104-104` (`DIRECT`)
- `src/runtime/signal/operator.rs:105-105` (`DIRECT`)

For **Asynchronous signal lane**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.
