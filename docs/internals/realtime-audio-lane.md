# Realtime audio lane

<!-- claims: CLM-DOC-050-CAP-001,CLM-DOC-050-CAP-002,CLM-DOC-050-SOURCE-001 -->

## Scope

- **Preserve frame identity and lineage.** Carry source, stream, stem, route, clock, sequence, generation, and derivation identity with audio frames.
- **Route realtime audio.** Deliver pooled audio frames through independent fixed-capacity routes governed by explicit edge policy.

The scope of **Realtime audio lane** ends at the native contracts and executable conditions cited below. Platform qualification, performance, retry, and delivery require their own explicit evidence.

## Ownership map

- `src/frame/pool.rs` owns part of this boundary.
- `src/runtime/audio/runner.rs` owns part of this boundary.

## Compiler-visible surface

| Public declaration | Kind | Declared purpose | Source |
|---|---|---|---|
| `pocketstation::runtime::audio::router::PlanEdgeObservationHandle` | struct | Cloneable read-only access to one plan edge's authoritative live telemetry. | `src/runtime/audio/router.rs:211` |
| `pocketstation::runtime::audio::router::PlanEdgeReceiver` | struct | Receives plan edge values across its declared ownership boundary. | `src/runtime/audio/router.rs:488` |
| `pocketstation::runtime::audio::router::PlanEdgeRouter` | struct | Routes plan edge according to the compiled edge contracts. | `src/runtime/audio/router.rs:675` |
| `pocketstation::runtime::audio::runner::RealtimePlanRunner` | struct | Executes realtime plan according to its compiled plan and cancellation contract. | `src/runtime/audio/runner.rs:305` |
| `pocketstation::runtime::audio::router::PlanEdgeFrame` | enum | Enumerates the supported plan edge frame cases. | `src/runtime/audio/router.rs:29` |
| `pocketstation::runtime::audio::router::PlanEdgeFrame::Exclusive` | variant | Represents the exclusive alternative defined by `PlanEdgeFrame`. | `src/runtime/audio/router.rs:30` |
| `pocketstation::runtime::audio::router::PlanEdgeFrame::Shared` | variant | Represents the shared alternative defined by `PlanEdgeFrame`. | `src/runtime/audio/router.rs:31` |
| `audio` | module | Types and operations for audio. | `src/frame/audio.rs:1` |
| `pocketstation::frame::audio::AudioFrame` | struct | Carries one audio payload together with its declared metadata. | `src/frame/audio.rs:39` |
| `pocketstation::frame::audio::LineagedAudioFrame` | struct | An exclusive audio frame and the immutable lineage snapshot captured before the frame crosses a bounded edge. | `src/frame/audio.rs:266` |
| `pocketstation::frame::audio::SampleSpec` | struct | Configures sample behavior at its owning API boundary. | `src/frame/audio.rs:18` |
| `pocketstation::frame::audio::SharedAudioFrame` | struct | Carries one shared audio payload together with its declared metadata. | `src/frame/audio.rs:176` |
| `pocketstation::frame::audio::SharedLineagedAudioFrame` | struct | Carries one shared lineaged audio payload together with its declared metadata. | `src/frame/audio.rs:298` |
| `pocketstation::frame::pool::AudioBufferHandle` | struct | Owns bounded access to audio buffer. | `src/frame/pool.rs:198` |
| `pocketstation::frame::pool::AudioBufferPool` | struct | Owns fixed-capacity reusable audio slots and reports acquisition pressure without allocating per frame. | `src/frame/pool.rs:24` |
| `pocketstation::frame::pool::SharedAudioBufferHandle` | struct | Owns bounded access to shared audio buffer. | `src/frame/pool.rs:281` |
| `pocketstation::runtime::audio::router::DispatchSummary` | struct | Reports the counters and terminal facts collected for dispatch. | `src/runtime/audio/router.rs:667` |
| `pocketstation::runtime::audio::router::EdgeObservations` | struct | Reports the edge observations collected at an observation boundary. | `src/runtime/audio/router.rs:122` |
| `pocketstation::runtime::audio::runner::PlanRunnerCancellation` | struct | Shares a lock-free cancellation flag between the Session owner and the realtime plan runner. | `src/runtime/audio/runner.rs:89` |
| `pocketstation::runtime::audio::runner::PlanRunnerFinishSummary` | struct | Reports the counters and terminal facts collected for plan runner finish. | `src/runtime/audio/runner.rs:298` |
| `pocketstation::runtime::audio::runner::PlanRunnerStepSummary` | struct | Reports the counters and terminal facts collected for plan runner step. | `src/runtime/audio/runner.rs:270` |
| `pocketstation::runtime::audio::runner::PlanSourceInput` | struct | Carries typed input for plan source. | `src/runtime/audio/runner.rs:188` |
| `pocketstation::runtime::audio::runner::PlanSourceInputObservations` | struct | Reports the plan source input observations collected at an observation boundary. | `src/runtime/audio/runner.rs:22` |
| `pocketstation::runtime::audio::runner::PlanSourceObservationHandle` | struct | Owns bounded access to plan source observation. | `src/runtime/audio/runner.rs:138` |

## Observed implementation patterns

- `typed_error` — `src/runtime/audio/runner.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `clock_correlation` — `src/frame/audio.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `buffer_pool` — `src/runtime/nodes.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `buffer_pool` — `benches/runtime_plan.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `typed_error` — `src/frame/pool.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `buffer_pool` — `src/runtime/audio/executor.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `typed_error` — `src/runtime/lifecycle/async_host.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `clock_correlation` — `src/frame/lineage.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `buffer_pool` — `src/runtime/audio/runner.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `clock_correlation` — `src/runtime/audio/router.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `typed_error` — `src/runtime/nodes.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `buffer_pool` — `benches/audio_buffer_pool.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `buffer_pool` — `src/frame/audio.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `sidecar_isolation` — `src/runtime/lifecycle/sidecar_protocol.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `sidecar_isolation` — `src/runtime/lifecycle/sidecar_host.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `typed_error` — `src/runtime/bridge/audio.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `typed_error` — `src/runtime/signal/operator.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `typed_error` — `src/runtime/lifecycle/sidecar_protocol.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `buffer_pool` — `src/runtime/signal/operator.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `buffer_pool` — `src/runtime/audio/router.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `bounded_queue` — `src/runtime/signal/edge.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `typed_error` — `src/runtime/audio/executor.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `clock_correlation` — `tests/runtime_plan_router_alloc.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `buffer_pool` — `src/frame/pool.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `buffer_pool` — `tests/runtime_plan_router_alloc.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `clock_correlation` — `benches/runtime_plan.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `buffer_pool` — `src/runtime/bridge/audio.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `bounded_queue` — `src/runtime/signal/operator.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `typed_error` — `src/frame/audio.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `typed_error` — `src/runtime/lifecycle/sidecar_host.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).

## Behavioral evidence

Executable evidence selected for **Realtime audio lane** is limited to each test's recorded setup and assertions:

- `given_full_source_input_when_more_frames_arrive_then_newest_rejects_and_counts` — given full source input when more frames arrive then newest rejects and counts (`src/runtime/audio/runner.rs:704`; `test-9884a85b98ea454bb6cf`).
- `given_queued_sources_when_cancelled_with_budget_then_drain_is_bounded_and_rest_discards` — given queued sources when cancelled with budget then drain is bounded and rest discards (`src/runtime/audio/runner.rs:636`; `test-31632b8eb3f0b3c90934`).
- `given_queued_sources_when_cancelled_with_discard_then_no_frame_executes` — given queued sources when cancelled with discard then no frame executes (`src/runtime/audio/runner.rs:682`; `test-e8ced072a71ada7c3d25`).
- `given_two_ready_sources_when_processed_then_each_source_dispatches_independently` — given two ready sources when processed then each source dispatches independently (`src/runtime/audio/runner.rs:595`; `test-218c72d3cc7560654f20`).
- `given_lineaged_frame_when_realtime_operator_executes_then_output_keeps_capture_epochs` — given lineaged frame when realtime operator executes then output keeps capture epochs (`src/runtime/audio/executor.rs:411`; `test-14905efc2e19f82a8cb2`).
- `given_realtime_fan_out_when_executed_then_each_mutating_branch_gets_independent_copy` — given realtime fan out when executed then each mutating branch gets independent copy (`src/runtime/audio/executor.rs:369`; `test-c0c81ff42570a02c1eb9`).
- `given_compiled_text_edge_when_router_builds_then_only_audio_edge_gets_audio_receiver` — given compiled text edge when router builds then only audio edge gets audio receiver (`src/runtime/audio/router.rs:983`; `test-c5f24b62056cfa546c3a`).
- `given_full_audio_ingress_when_bridge_sends_then_rejection_is_counted_exactly` — given full audio ingress when bridge sends then rejection is counted exactly (`src/runtime/bridge/audio.rs:497`; `test-c49159871ef385421381`).
- `given_operator_audio_when_bridged_then_owned_frame_enters_bounded_plan_source` — given operator audio when bridged then owned frame enters bounded plan source (`src/runtime/bridge/audio.rs:335`; `test-0ae60369d5962ff55b0f`).
- `given_retained_audio_ingress_when_pool_is_exhausted_then_loss_is_counted_exactly` — given retained audio ingress when pool is exhausted then loss is counted exactly (`src/runtime/bridge/audio.rs:459`; `test-1664fa1aa12573253d70`).
- `given_full_owned_signal_edge_when_audio_sent_then_frame_returns_without_allocation` — given full owned signal edge when audio sent then frame returns without allocation (`src/runtime/signal/edge.rs:460`; `test-bf7094bd0d63b90cc8fe`).
- `given_audio_output_without_audio_port_when_processed_then_worker_rejects_it` — given audio output without audio port when processed then worker rejects it (`src/runtime/signal/operator.rs:2466`; `test-f0e8c28b4853dfd07393`).

## Stability boundary

**Realtime audio lane** describes internal ownership. Its private module layout is not a compatibility promise; compatibility comes from exported Rust declarations, the C header, manifests, error codes, and explicit compatibility artifacts.

## Related documentation

- [Glossary](/docs/glossary.md)
- [Memory ownership and buffer pools](/docs/internals/memory-ownership.md)
- [PocketStation](/README.md)
- [Behavior evidence index](/docs/reference/behavior-evidence.md)
- [Rust API reference](/docs/reference/rust-api.md)
- [Keep realtime callbacks bounded](/docs/best-practices/realtime-boundaries.md)
- [Architecture overview](/docs/architecture/overview.md)
- [Frame identity and lineage](/docs/concepts/frame-lineage.md)

## Evidence boundary

The claims on **Realtime audio lane** are anchored to Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `src/runtime/audio/runner.rs:1-746` (`DIRECT`)
- `src/frame/pool.rs:1-336` (`DIRECT`)

For **Realtime audio lane**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.
