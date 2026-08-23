# Asynchronous signal lane

<!-- claims: CLM-DOC-051-CAP-001,CLM-DOC-051-CAP-002,CLM-DOC-051-SOURCE-001 -->

## Scope

- **Implement asynchronous operators.** Register operator factories that consume and emit named typed signals on the asynchronous execution lane.
- **Carry typed signals.** Represent audio-adjacent text, event, binary, metric, and custom-schema payloads with timing and lineage.

These statements describe repository contracts at the documented snapshot. They do not extend platform qualification, performance, retry, or delivery guarantees beyond the native API contracts and executable evidence.

## Ownership map

- `src/runtime/signal/edge.rs` owns part of this boundary.
- `src/runtime/signal/operator.rs` owns part of this boundary.

## Compiler-visible surface

| Public declaration | Kind | Declared purpose | Source |
|---|---|---|---|
| `pocketstation::graph::plan::PlanError::MissingOutputSignal` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/graph/plan.rs:29` |
| `PlanError::MissingOutputSignal::edge` | struct_field | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/graph/plan.rs:29` |
| `signal` | module | Bounded asynchronous signal execution lane. | `src/runtime/signal/mod.rs:1` |
| `pocketstation::graph::signal::operator::AsyncNode` | trait | Async operator contract for model, connector, transport, and control-plane work. | `src/graph/signal/operator.rs:13` |
| `pocketstation::graph::signal::lineage::SignalDerivation` | struct | Source-independent record of the signal consumed by an operator. | `src/graph/signal/lineage.rs:97` |
| `pocketstation::graph::signal::preparation::AsyncOperatorPrepareContext` | struct | Complete graph-owned preparation contract for one asynchronous Operator. | `src/graph/signal/preparation.rs:22` |
| `pocketstation::graph::signal::spec::SchemaRef` | struct | Reference to an external schema document. | `src/graph/signal/spec.rs:87` |
| `pocketstation::graph::signal::spec::SemanticRole` | struct | Semantic role annotation on a port. | `src/graph/signal/spec.rs:57` |
| `pocketstation::graph::signal::spec::SignalId` | struct | Opaque identifier for a custom signal type. | `src/graph/signal/spec.rs:22` |
| `pocketstation::graph::signal::spec::SignalSpec` | struct | Full signal contract for a single port. | `src/graph/signal/spec.rs:205` |
| `pocketstation::graph::signal::spec::BinaryFormat` | enum | Binary encoding hint for `SignalClass::Binary`. | `src/graph/signal/spec.rs:141` |
| `pocketstation::graph::signal::spec::Codec` | enum | Audio encoding format for `SignalClass::EncodedAudio`. | `src/graph/signal/spec.rs:113` |
| `pocketstation::graph::signal::spec::EventFormat` | enum | Event structure hint for `SignalClass::Event`. | `src/graph/signal/spec.rs:132` |
| `pocketstation::graph::signal::spec::SignalClass` | enum | The fundamental class of data flowing through a port. | `src/graph/signal/spec.rs:156` |
| `pocketstation::graph::signal::spec::TextFormat` | enum | Text encoding hint for `SignalClass::Text`. | `src/graph/signal/spec.rs:124` |
| `pocketstation::graph::signal::preparation::AsyncOperatorEdgePrepareContext` | type_alias | Exact bounded graph edge supplied to an asynchronous Operator at prepare time. | `src/graph/signal/preparation.rs:18` |
| `pocketstation::graph::signal::spec::SignalClass::Any` | variant | Wildcard accepted only at deliberately open graph boundaries. | `src/graph/signal/spec.rs:158` |
| `pocketstation::graph::signal::spec::SignalClass::Binary` | variant | Opaque binary blob. | `src/graph/signal/spec.rs:172` |
| `pocketstation::graph::signal::spec::SignalClass::Control` | variant | Graph control messages (route patches, session lifecycle, mute/unmute). | `src/graph/signal/spec.rs:170` |
| `pocketstation::graph::signal::spec::SignalClass::Custom` | variant | Extension point for community / vendor signals. Use a stable reverse-domain identifier. | `src/graph/signal/spec.rs:175` |
| `pocketstation::graph::signal::spec::SignalClass::EncodedAudio` | variant | Compressed audio bitstream (Opus packet, AAC frame, …). | `src/graph/signal/spec.rs:162` |
| `pocketstation::graph::signal::spec::SignalClass::Event` | variant | Discrete event payloads. | `src/graph/signal/spec.rs:166` |
| `pocketstation::graph::signal::spec::SignalClass::Metrics` | variant | Telemetry and observability counters / gauges. | `src/graph/signal/spec.rs:168` |
| `pocketstation::graph::signal::spec::SignalClass::PcmAudio` | variant | Interleaved PCM audio samples (format described by the edge AudioCaps). | `src/graph/signal/spec.rs:160` |

## Observed implementation patterns

- `typed_error` — `src/runtime/audio/runner.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `buffer_pool` — `src/runtime/nodes.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `sidecar_isolation` — `src/runtime/lifecycle/async_host.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `buffer_pool` — `benches/runtime_plan.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `bounded_queue` — `src/graph/ports.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `sidecar_isolation` — `benches/runtime_plan.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `buffer_pool` — `src/graph/signal/envelope.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `bounded_queue` — `src/graph/compile/plan.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `typed_error` — `src/graph/named_ports.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `buffer_pool` — `src/runtime/audio/executor.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `buffer_pool` — `src/graph/plan.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `sidecar_isolation` — `src/runtime/signal/operator.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `typed_error` — `examples/operator-consumer/src/lib.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `typed_error` — `src/runtime/lifecycle/async_host.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `typed_error` — `src/graph/runtime_node.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `sidecar_isolation` — `src/runtime/signal/io.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `sidecar_isolation` — `src/runtime/audio/router.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `buffer_pool` — `src/runtime/audio/runner.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `buffer_pool` — `src/graph/compile/plan.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `sidecar_isolation` — `src/runtime/signal/edge.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `typed_error` — `src/graph/signal/operator.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `bounded_queue` — `src/runtime/nodes.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `sidecar_isolation` — `src/runtime/nodes.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `clock_correlation` — `src/runtime/audio/router.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `sidecar_isolation` — `tests/runtime_plan_router_alloc.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `typed_error` — `src/graph/builtins.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `bounded_queue` — `src/runtime/audio/router.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `typed_error` — `src/runtime/nodes.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `sidecar_isolation` — `src/runtime/signal/mod.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `sidecar_isolation` — `src/runtime/lifecycle/sidecar_protocol.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).

## Behavioral evidence

The following test bodies are evidence only for their recorded setup:

- `given_audio_signal_into_text_signal_when_compiled_then_signal_mismatch` — given audio signal into text signal when compiled then signal mismatch (`src/graph/compile/resolve.rs:1094`; `test-5516fcaedfc2241e34cc`).
- `given_shipped_signal_wire_ids_when_audited_then_protocol_namespace_stays_versioned` — given shipped signal wire ids when audited then protocol namespace stays versioned (`src/graph/identifier.rs:180`; `test-2ffb33b44ce1e1a84da1`).
- `given_custom_signal_without_schema_when_checked_then_binary_media_rejects_it` — given custom signal without schema when checked then binary media rejects it (`src/graph/ports.rs:575`; `test-2d3675af8d6c3a4d6a26`).
- `given_supported_non_audio_signals_when_checked_then_media_is_symmetric` — given supported non audio signals when checked then media is symmetric (`src/graph/ports.rs:559`; `test-d97a306ad6dc3558e082`).
- `given_contiguous_signals_when_replayed_then_continuity_is_deterministic` — given contiguous signals when replayed then continuity is deterministic (`src/graph/signal/envelope.rs:390`; `test-d0dc80cc2da279b6a618`).
- `given_custom_signal_with_role_when_built_then_fields_accessible` — given custom signal with role when built then fields accessible (`src/graph/signal/spec.rs:389`; `test-592b8677db2ba5e30ba7`).
- `given_signal_id_when_as_str_then_returns_inner` — given signal id when as str then returns inner (`src/graph/signal/spec.rs:422`; `test-3fa05de5ac89880ef9e0`).
- `given_full_owned_signal_edge_when_audio_sent_then_frame_returns_without_allocation` — given full owned signal edge when audio sent then frame returns without allocation (`src/runtime/signal/edge.rs:460`; `test-bf7094bd0d63b90cc8fe`).
- `given_one_branch_when_signal_published_then_receiver_has_exclusive_ownership` — given one branch when signal published then receiver has exclusive ownership (`src/runtime/signal/edge.rs:626`; `test-9674b8f8edebf8590582`).
- `given_registered_signal_consumer_when_item_enqueued_then_parked_thread_is_woken` — given registered signal consumer when item enqueued then parked thread is woken (`src/runtime/signal/edge.rs:469`; `test-2c211fee5a326cacf730`).
- `given_every_nonaudio_signal_class_when_worker_prepares_then_exact_signal_context_is_received` — given every nonaudio signal class when worker prepares then exact signal context is received (`src/runtime/signal/operator.rs:1752`; `test-8298f7b73ae7319aa84e`).
- `given_external_consumer_when_declared_then_provider_and_typed_endpoint_use_public_api` — given external consumer when declared then provider and typed endpoint use public api (`examples/operator-consumer/src/lib.rs:120`; `test-ace9b7d11da2036ce899`).

## Stability boundary

This page explains internals. Public compatibility comes from exported Rust declarations, the C header, manifests, error codes, and explicit compatibility artifacts—not private module layout.

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

This page was verified against Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `src/runtime/signal/edge.rs:1-651` (`DIRECT`)
- `src/runtime/signal/operator.rs:1-2573` (`DIRECT`)

A file's presence proves implementation or declaration at this snapshot. It does not by itself prove physical-device qualification, operational performance, retry safety, or behavior outside the recorded test conditions.
