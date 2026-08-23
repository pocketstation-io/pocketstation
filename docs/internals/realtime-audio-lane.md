# Realtime audio lane

<!-- claims: CLM-DOC-050-CAP-001,CLM-DOC-050-CAP-002,CLM-DOC-050-SOURCE-001 -->

## Scope

- **Preserve frame identity and lineage.** Carry source, stream, stem, route, clock, sequence, generation, and derivation identity with audio frames.
- **Route realtime audio.** Deliver pooled audio frames through independent fixed-capacity routes governed by explicit edge policy.

These statements describe repository contracts at the documented snapshot. They do not extend platform qualification, performance, retry, or delivery guarantees beyond the native API contracts and executable evidence.

## Ownership map

- `src/frame/pool.rs` owns part of this boundary.
- `src/runtime/audio/runner.rs` owns part of this boundary.

## Compiler-visible surface

| Public declaration | Kind | Declared purpose | Source |
|---|---|---|---|
| `audio` | module | Allocation-free realtime audio execution lane. | `src/runtime/audio/mod.rs:1` |
| `audio` | module | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/frame/audio.rs:1` |
| `pocketstation::frame::audio::AudioFrame` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/frame/audio.rs:39` |
| `pocketstation::frame::audio::SampleSpec` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/frame/audio.rs:18` |
| `pocketstation::frame::audio::SharedAudioFrame` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/frame/audio.rs:176` |
| `pocketstation::frame::pool::AudioBufferHandle` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/frame/pool.rs:198` |
| `pocketstation::frame::pool::AudioBufferPool` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/frame/pool.rs:24` |
| `pocketstation::frame::pool::SharedAudioBufferHandle` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/frame/pool.rs:281` |
| `pocketstation::runtime::audio::router::EdgeObservations` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/runtime/audio/router.rs:122` |
| `pocketstation::runtime::audio::runner::PlanSourceInputObservations` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/runtime/audio/runner.rs:22` |
| `pocketstation::frame::audio::AudioFrameBuildError` | enum | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/frame/audio.rs:51` |
| `pocketstation::frame::audio::SampleFormat` | enum | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/frame/audio.rs:13` |
| `pocketstation::frame::pool::AudioBufferWriteError` | enum | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/frame/pool.rs:14` |
| `pocketstation::runtime::audio::executor::ExecError` | enum | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/runtime/audio/executor.rs:20` |
| `pocketstation::runtime::audio::runner::PlanRunnerError` | enum | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/runtime/audio/runner.rs:256` |
| `pocketstation::frame::audio::AudioFrameBuildError::MisalignedSamples` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/frame/audio.rs:57` |
| `pocketstation::frame::audio::AudioFrameBuildError::ZeroChannels` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/frame/audio.rs:55` |
| `pocketstation::frame::audio::AudioFrameBuildError::ZeroSampleRate` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/frame/audio.rs:53` |
| `pocketstation::frame::audio::SampleFormat::F32Interleaved` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/frame/audio.rs:14` |
| `pocketstation::frame::pool::AudioBufferWriteError::CapacityExceeded` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/frame/pool.rs:18` |
| `pocketstation::runtime::audio::executor::ExecError::Node` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/runtime/audio/executor.rs:22` |
| `pocketstation::runtime::audio::runner::PlanRunnerError::AlreadyFinished` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/runtime/audio/runner.rs:264` |
| `pocketstation::runtime::audio::runner::PlanRunnerError::DuplicateSource` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/runtime/audio/runner.rs:260` |
| `pocketstation::runtime::audio::runner::PlanRunnerError::Execution` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/runtime/audio/runner.rs:266` |

## Observed implementation patterns

- `typed_error` — `src/runtime/audio/runner.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `clock_correlation` — `src/frame/audio.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `buffer_pool` — `src/runtime/nodes.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `sidecar_isolation` — `src/runtime/lifecycle/async_host.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `buffer_pool` — `benches/runtime_plan.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `typed_error` — `src/frame/pool.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `sidecar_isolation` — `benches/runtime_plan.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `buffer_pool` — `src/runtime/audio/executor.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `sidecar_isolation` — `src/runtime/signal/operator.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `typed_error` — `src/runtime/lifecycle/async_host.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `clock_correlation` — `src/frame/lineage.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `sidecar_isolation` — `src/runtime/signal/io.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `sidecar_isolation` — `src/runtime/audio/router.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `buffer_pool` — `src/runtime/audio/runner.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `sidecar_isolation` — `src/runtime/signal/edge.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `bounded_queue` — `src/runtime/nodes.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `sidecar_isolation` — `src/runtime/nodes.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `clock_correlation` — `src/runtime/audio/router.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `sidecar_isolation` — `tests/runtime_plan_router_alloc.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `bounded_queue` — `src/runtime/audio/router.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `typed_error` — `src/runtime/nodes.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `sidecar_isolation` — `src/runtime/signal/mod.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `buffer_pool` — `benches/audio_buffer_pool.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `buffer_pool` — `src/frame/audio.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `sidecar_isolation` — `src/runtime/lifecycle/sidecar_protocol.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `sidecar_isolation` — `src/runtime/lifecycle/sidecar_host.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `typed_error` — `src/runtime/signal/operator.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `typed_error` — `src/runtime/lifecycle/sidecar_protocol.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `buffer_pool` — `src/runtime/signal/operator.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `sidecar_isolation` — `src/runtime/audio/runner.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).

## Behavioral evidence

The following test bodies are evidence only for their recorded setup:

- `given_lineaged_frame_when_realtime_operator_executes_then_output_keeps_capture_epochs` — given lineaged frame when realtime operator executes then output keeps capture epochs (`src/runtime/audio/executor.rs:411`; `test-14905efc2e19f82a8cb2`).
- `given_realtime_fan_out_when_executed_then_each_mutating_branch_gets_independent_copy` — given realtime fan out when executed then each mutating branch gets independent copy (`src/runtime/audio/executor.rs:369`; `test-c0c81ff42570a02c1eb9`).
- `given_compiled_text_edge_when_router_builds_then_only_audio_edge_gets_audio_receiver` — given compiled text edge when router builds then only audio edge gets audio receiver (`src/runtime/audio/router.rs:983`; `test-c5f24b62056cfa546c3a`).
- `given_full_audio_ingress_when_bridge_sends_then_rejection_is_counted_exactly` — given full audio ingress when bridge sends then rejection is counted exactly (`src/runtime/bridge/audio.rs:497`; `test-c49159871ef385421381`).
- `given_operator_audio_when_bridged_then_owned_frame_enters_bounded_plan_source` — given operator audio when bridged then owned frame enters bounded plan source (`src/runtime/bridge/audio.rs:335`; `test-0ae60369d5962ff55b0f`).
- `given_retained_audio_ingress_when_pool_is_exhausted_then_loss_is_counted_exactly` — given retained audio ingress when pool is exhausted then loss is counted exactly (`src/runtime/bridge/audio.rs:459`; `test-1664fa1aa12573253d70`).
- `given_full_owned_signal_edge_when_audio_sent_then_frame_returns_without_allocation` — given full owned signal edge when audio sent then frame returns without allocation (`src/runtime/signal/edge.rs:460`; `test-bf7094bd0d63b90cc8fe`).
- `send_audio` — send audio (`src/runtime/signal/edge.rs:180`; `test-3b451567af9e3df48cdb`).
- `given_audio_output_without_audio_port_when_processed_then_worker_rejects_it` — given audio output without audio port when processed then worker rejects it (`src/runtime/signal/operator.rs:2466`; `test-f0e8c28b4853dfd07393`).
- `given_every_nonaudio_signal_class_when_worker_prepares_then_exact_signal_context_is_received` — given every nonaudio signal class when worker prepares then exact signal context is received (`src/runtime/signal/operator.rs:1752`; `test-8298f7b73ae7319aa84e`).
- `given_prepared_realtime_plan_when_connected_nodes_execute_then_no_heap_allocation_occurs` — given prepared realtime plan when connected nodes execute then no heap allocation occurs (`tests/runtime_plan_router_alloc.rs:89`; `test-8f6b3c13a8c5d31e5914`).
- `given_48khz_mono_spec_when_frame_samples_for_20ms_then_returns_960` — given 48khz mono spec when frame samples for 20ms then returns 960 (`src/frame/audio.rs:434`; `test-bf95ac4b2316d447ed6b`).

## Stability boundary

This page explains internals. Public compatibility comes from exported Rust declarations, the C header, manifests, error codes, and explicit compatibility artifacts—not private module layout.

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

This page was verified against Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `src/runtime/audio/runner.rs:1-746` (`DIRECT`)
- `src/frame/pool.rs:1-336` (`DIRECT`)

A file's presence proves implementation or declaration at this snapshot. It does not by itself prove physical-device qualification, operational performance, retry safety, or behavior outside the recorded test conditions.
