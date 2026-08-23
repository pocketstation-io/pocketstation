# Architecture overview

<!-- claims: CLM-DOC-048-CAP-001,CLM-DOC-048-CAP-002,CLM-DOC-048-CAP-003,CLM-DOC-048-CAP-004,CLM-DOC-048-SOURCE-001 -->

## Scope

- **Compile Session declarations.** Validate declarations, resolve bindings, and lower a Session specification into an executable plan.
- **Start, cancel, stop, and finalize a Session.** Coordinate Session startup, rollback, cancellation, steady-state ownership, stop, and terminal outcomes.
- **Route realtime audio.** Deliver pooled audio frames through independent fixed-capacity routes governed by explicit edge policy.
- **Carry typed signals.** Represent audio-adjacent text, event, binary, metric, and custom-schema payloads with timing and lineage.

The scope of **Architecture overview** ends at the native contracts and executable conditions cited below. Platform qualification, performance, retry, and delivery require their own explicit evidence.

## Ownership map

- `src/lib.rs` owns part of this boundary.
- `src/runtime/mod.rs` owns part of this boundary.

## Compiler-visible surface

| Public declaration | Kind | Declared purpose | Source |
|---|---|---|---|
| `pocketstation` | module | # PocketStation | `src/lib.rs:1` |
| `pocketstation::RunningSession` | struct | Owns a started Session together with event, polling, recording, trace, and stop resources. | `src/lib.rs:789` |
| `pocketstation::Session` | struct | Owns a mutable Session declaration and the host configuration used to compile, prepare, and start it. | `src/lib.rs:236` |
| `pocketstation::SessionBuilder` | struct | Setup-time configuration for the public Rust Session. | `src/lib.rs:275` |
| `pocketstation::SessionCancelResult` | struct | Reports the structured session cancel result. | `src/lib.rs:1123` |
| `pocketstation::SessionStartError` | struct | Stable façade error for Session startup. | `src/lib.rs:955` |
| `pocketstation::SessionStopResult` | struct | Reports the structured session stop result. | `src/lib.rs:1143` |
| `pocketstation::graph::compile::resolve::Compiler` | struct | Runs the ordered graph-validation passes that resolve a graph specification into executable IR. | `src/graph/compile/resolve.rs:444` |
| `pocketstation::graph::ports::AudioCaps` | struct | Declares the sample formats, channel layouts, and rates accepted by an audio port. | `src/graph/ports.rs:48` |
| `pocketstation::graph::ports::EdgeContract` | struct | Declares the validated constraints applied to edge. | `src/graph/ports.rs:311` |
| `pocketstation::graph::ports::PortSpec` | struct | Configures port behavior at its owning API boundary. | `src/graph/ports.rs:175` |
| `pocketstation::graph::signal::envelope::SignalEnvelope` | struct | Carries a typed signal payload together with timing, lineage, continuity, and terminal metadata. | `src/graph/signal/envelope.rs:6` |
| `pocketstation::graph::signal::spec::SchemaRef` | struct | Reference to an external schema document. | `src/graph/signal/spec.rs:87` |
| `pocketstation::graph::signal::spec::SemanticRole` | struct | Semantic role annotation on a port. | `src/graph/signal/spec.rs:57` |
| `pocketstation::graph::signal::spec::SignalId` | struct | Opaque identifier for a custom signal type. | `src/graph/signal/spec.rs:22` |
| `pocketstation::graph::signal::spec::SignalSpec` | struct | Full signal contract for a single port. | `src/graph/signal/spec.rs:205` |
| `pocketstation::runtime::audio::router::DispatchSummary` | struct | Reports the counters and terminal facts collected for dispatch. | `src/runtime/audio/router.rs:696` |
| `pocketstation::runtime::audio::router::EdgeObservations` | struct | Reports the edge observations collected at an observation boundary. | `src/runtime/audio/router.rs:142` |
| `pocketstation::runtime::audio::router::PlanEdgeObservationHandle` | struct | Cloneable read-only access to one plan edge's authoritative live telemetry. | `src/runtime/audio/router.rs:231` |
| `pocketstation::runtime::audio::router::PlanEdgeReceiver` | struct | Receives plan edge values across its declared ownership boundary. | `src/runtime/audio/router.rs:508` |
| `pocketstation::runtime::audio::router::PlanEdgeRouter` | struct | Routes plan edge according to the compiled edge contracts. | `src/runtime/audio/router.rs:704` |
| `pocketstation::runtime::audio::runner::PlanRunnerCancellation` | struct | Shares a lock-free cancellation flag between the Session owner and the realtime plan runner. | `src/runtime/audio/runner.rs:89` |
| `pocketstation::runtime::audio::runner::PlanRunnerFinishSummary` | struct | Reports the counters and terminal facts collected for plan runner finish. | `src/runtime/audio/runner.rs:298` |
| `pocketstation::runtime::audio::runner::PlanRunnerStepSummary` | struct | Reports the counters and terminal facts collected for plan runner step. | `src/runtime/audio/runner.rs:270` |

## Observed implementation patterns

- `typed_error` — `src/connector/error.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `typed_error` — `src/runtime/audio/runner.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `typed_error` — `src/native_extension/library.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `buffer_pool` — `src/session/extensions/audio_input/source.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `clock_correlation` — `src/frame/audio.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `typed_error` — `src/capture/platform/macos/session_backend.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `bounded_queue` — `src/session/lifecycle/running.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `buffer_pool` — `examples/whisper-transcribe/src/lib.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `typed_error` — `src/session/declaration/typed_stream.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `buffer_pool` — `src/runtime/nodes.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `typed_error` — `src/endpoint/registry.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `clock_correlation` — `src/session/lifecycle/running.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `buffer_pool` — `benches/runtime_plan.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `typed_error` — `src/connector/worker/endpoint_adapter.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `typed_error` — `src/session/extensions/tests/runtime.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `typed_error` — `src/session/compile/tests.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `typed_error` — `src/session/extensions/tests/registry.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `buffer_pool` — `tests/codec_hot_path_alloc.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `typed_error` — `src/graph/signal/continuity.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `typed_error` — `examples/whisper-transcribe/src/process_evidence.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `typed_error` — `src/frame/pool.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `clock_correlation` — `src/runtime/signal/edge.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `typed_error` — `src/abi/extension.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `clock_correlation` — `src/timing/domain.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `typed_error` — `src/session/declaration/spec.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `typed_error` — `src/recording/endpoint.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `typed_error` — `src/connector/readiness.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `buffer_pool` — `tests/capture_callback_source_contract.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `clock_correlation` — `src/runtime/signal/operator.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `buffer_pool` — `tests/public_api_boundary.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).

## Behavioral evidence

Executable evidence selected for **Architecture overview** is limited to each test's recorded setup and assertions:

- `given_external_consumer_when_declared_then_provider_and_typed_endpoint_use_public_api` — given external consumer when declared then provider and typed endpoint use public api (`examples/operator-consumer/src/lib.rs:120`; `test-f98e0a98874ff7dfbdf8`).
- `given_discontinuity_change_inside_window_when_processed_then_window_is_rejected` — given discontinuity change inside window when processed then window is rejected (`examples/whisper-transcribe/src/lib.rs:1400`; `test-2a5f4a8f1e04f5b220c4`).
- `given_hung_provider_when_deadline_expires_then_child_is_killed_and_reaped` — given hung provider when deadline expires then child is killed and reaped (`examples/whisper-transcribe/src/lib.rs:1108`; `test-aa794f0809b00c2d3394`).
- `given_instance_timeout_when_manifest_resolves_then_deadline_matches_configuration` — given instance timeout when manifest resolves then deadline matches configuration (`examples/whisper-transcribe/src/lib.rs:1055`; `test-899bb5750fda98d0832b`).
- `given_lineaged_window_when_transcribed_then_derived_range_covers_every_frame` — given lineaged window when transcribed then derived range covers every frame (`examples/whisper-transcribe/src/lib.rs:1311`; `test-5978528a8ea570fad70d`).
- `given_missing_binary_when_prepare_runs_then_connector_fails_closed` — given missing binary when prepare runs then connector fails closed (`examples/whisper-transcribe/src/lib.rs:1098`; `test-bd5ed751c752083c7711`).
- `given_outer_cancellation_when_process_is_active_then_child_receipt_is_finalized` — given outer cancellation when process is active then child receipt is finalized (`examples/whisper-transcribe/src/lib.rs:1220`; `test-841a6b80171cfb0f55e8`).
- `given_permission_change_inside_window_when_processed_then_window_is_rejected` — given permission change inside window when processed then window is rejected (`examples/whisper-transcribe/src/lib.rs:1419`; `test-1601ba20883aee1ac630`).
- `given_process_evidence_when_provider_succeeds_then_actual_invocation_is_persisted` — given process evidence when provider succeeds then actual invocation is persisted (`examples/whisper-transcribe/src/lib.rs:1129`; `test-004f9f3662355f6c02cc`).
- `given_process_evidence_when_provider_times_out_then_kill_and_reap_are_persisted` — given process evidence when provider times out then kill and reap are persisted (`examples/whisper-transcribe/src/lib.rs:1180`; `test-2ed5fdd4ba19977c8dc9`).
- `given_source_change_inside_window_when_processed_then_window_is_rejected_and_reset` — given source change inside window when processed then window is rejected and reset (`examples/whisper-transcribe/src/lib.rs:1379`; `test-384d43cad3cb43576f09`).
- `given_two_complete_windows_when_finished_then_partials_and_single_final_cover_stream` — given two complete windows when finished then partials and single final cover stream (`examples/whisper-transcribe/src/lib.rs:1338`; `test-46ae6f451f003a166202`).

## Stability boundary

**Architecture overview** describes internal ownership. Its private module layout is not a compatibility promise; compatibility comes from exported Rust declarations, the C header, manifests, error codes, and explicit compatibility artifacts.

## Related documentation

- [Glossary](/docs/glossary.md)
- [PocketStation](/README.md)
- [Behavior evidence index](/docs/reference/behavior-evidence.md)
- [Rust API reference](/docs/reference/rust-api.md)
- [Build, prepare, and start](/docs/lifecycle/build-prepare-start.md)
- [Prepare resources before start](/docs/how-to/prepare-session.md)
- [Session API](/docs/reference/session.md)
- [Frames or signals are dropped](/docs/troubleshooting/drops-and-saturation.md)

## Evidence boundary

The claims on **Architecture overview** are anchored to Git snapshot `136e74888962558aa846d3143a19136a70936f45` and these primary files:

- `src/lib.rs:1-1161` (`DIRECT`)
- `src/runtime/mod.rs:1-20` (`DIRECT`)

For **Architecture overview**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.
