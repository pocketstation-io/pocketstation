# Signals and streams

<!-- claims: CLM-DOC-022-CAP-001,CLM-DOC-022-SOURCE-001 -->

## What it is

Signals carry text, event, metrics, control, binary, custom-schema, or audio-adjacent payloads with a declared specification, timing, and lineage. Streams are declaration handles used to compose their routes.

## Why it exists

Typed envelopes keep payload interpretation and provenance explicit across asynchronous operators instead of treating every message as an unstructured byte buffer.

## Relationships

- `SignalSpec` identifies the payload class and optional schema.
- `SignalEnvelope` owns payload, timing, and lineage.
- Typed stream handles connect compatible operator ports in a Session declaration.

## Invariants and guarantees

- A payload must match the receiving port's signal specification.
- Lineage remains attached across a derived signal chain.
- Payload and queue limits are enforced before fan-out.

## When you encounter it

- **Add an asynchronous operator** — Declare typed ports, implement an operator factory, and route its output.

## Use it

- [Implement an asynchronous operator](/docs/how-to/implement-operator.md)
- [Connect named operator ports](/docs/how-to/connect-operator-ports.md)
- [Signals API reference](/docs/reference/graph.md)

## Scope

- **Carry typed signals.** Represent audio-adjacent text, event, binary, metric, and custom-schema payloads with timing and lineage.

The scope of **Signals and streams** ends at the native contracts and executable conditions cited below. Platform qualification, performance, retry, and delivery require their own explicit evidence.

## Key API

| Public declaration | Kind | Declared purpose | Source |
|---|---|---|---|
| `pocketstation::graph::signal::spec::SignalSpec` | struct | Full signal contract for a single port. | `src/graph/signal/spec.rs:205` |
| `pocketstation::graph::signal::spec::SignalSpecError` | enum | Classifies failures reported as signal spec error. | `src/graph/signal/spec.rs:351` |
| `pocketstation::graph::signal::envelope::SignalEnvelopeError::InvalidSignalSpec` | variant | Reported when the owning operation encounters invalid signal spec. | `src/graph/signal/envelope.rs:139` |
| `pocketstation::graph::signal::spec::SignalSpecError::EmptyCustomId` | variant | Reported when the owning operation encounters empty custom identifier. | `src/graph/signal/spec.rs:353` |
| `pocketstation::graph::signal::spec::SignalSpecError::EmptyRole` | variant | Reported when the owning operation encounters empty role. | `src/graph/signal/spec.rs:355` |
| `pocketstation::graph::signal::spec::SignalSpecError::EmptySchema` | variant | Reported when the owning operation encounters empty schema. | `src/graph/signal/spec.rs:357` |
| `pocketstation::graph::signal::envelope::SignalEnvelope` | struct | Carries a typed signal payload together with timing, lineage, continuity, and terminal metadata. | `src/graph/signal/envelope.rs:6` |
| `pocketstation::graph::signal::spec::SchemaRef` | struct | Reference to an external schema document. | `src/graph/signal/spec.rs:87` |
| `pocketstation::graph::signal::spec::SemanticRole` | struct | Semantic role annotation on a port. | `src/graph/signal/spec.rs:57` |
| `pocketstation::graph::signal::spec::SignalId` | struct | Opaque identifier for a custom signal type. | `src/graph/signal/spec.rs:22` |

## Executable evidence

Executable evidence selected for **Signals and streams** is limited to each test's recorded setup and assertions:

- `given_contiguous_signals_when_replayed_then_continuity_is_deterministic` — given contiguous signals when replayed then continuity is deterministic (`src/graph/signal/envelope.rs:390`; `test-d0dc80cc2da279b6a618`).
- `given_audio_frame_lineage_when_enveloped_then_generic_lineage_is_projected` — given audio frame lineage when enveloped then generic lineage is projected (`src/graph/signal/envelope.rs:332`; `test-e2f8e7a7f18caa97e503`).
- `given_echo_async_node_when_process_after_prepare_then_envelope_is_returned` — given echo async node when process after prepare then envelope is returned (`src/graph/signal/envelope.rs:233`; `test-9d67f3359220613efda8`).
- `given_echo_async_node_when_process_before_prepare_then_error_is_returned` — given echo async node when process before prepare then error is returned (`src/graph/signal/envelope.rs:251`; `test-bfea57e87a139988d3b9`).
- `given_fundamental_payloads_when_enveloped_then_specs_are_symmetric` — given fundamental payloads when enveloped then specs are symmetric (`src/graph/signal/envelope.rs:261`; `test-80aa65affe9c51f00bde`).
- `given_gap_without_discontinuity_when_replayed_then_rejected` — given gap without discontinuity when replayed then rejected (`src/graph/signal/envelope.rs:420`; `test-92478dc6bd675ec61686`).
- `given_generic_lineage_when_enveloped_then_no_frame_lineage_is_required` — given generic lineage when enveloped then no frame lineage is required (`src/graph/signal/envelope.rs:305`; `test-cd345e8d13b510a9eaa5`).
- `given_payload_and_incompatible_spec_when_validated_then_rejected` — given payload and incompatible spec when validated then rejected (`src/graph/signal/envelope.rs:294`; `test-33e18236d232fb399a6a`).
- `given_recovery_without_discontinuity_when_replayed_then_rejected` — given recovery without discontinuity when replayed then rejected (`src/graph/signal/envelope.rs:431`; `test-d84f14d43b88331c2181`).
- `given_text_storage_when_checked_against_text_spec_then_representation_is_supported` — given text storage when checked against text spec then representation is supported (`src/graph/signal/envelope.rs:227`; `test-675fc593bb481d67a026`).
- `given_supported_non_audio_signals_when_checked_then_media_is_symmetric` — given supported non audio signals when checked then media is symmetric (`src/graph/ports.rs:559`; `test-d97a306ad6dc3558e082`).
- `given_external_consumer_when_declared_then_provider_and_typed_endpoint_use_public_api` — given external consumer when declared then provider and typed endpoint use public api (`examples/operator-consumer/src/lib.rs:120`; `test-ace9b7d11da2036ce899`).

## Related documentation

- [Architecture overview](/docs/architecture/overview.md)
- [Asynchronous signal lane](/docs/internals/async-signal-lane.md)
- [Glossary](/docs/glossary.md)
- [PocketStation](/README.md)
- [Implement an asynchronous operator](/docs/how-to/implement-operator.md)
- [Behavior evidence index](/docs/reference/behavior-evidence.md)
- [Graph and route contracts](/docs/reference/graph.md)
- [Rust API reference](/docs/reference/rust-api.md)

## Evidence boundary

The claims on **Signals and streams** are anchored to Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `src/graph/signal/envelope.rs:1-444` (`DIRECT`)
- `src/session/declaration/typed_stream.rs:1-215` (`DIRECT`)

For **Signals and streams**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.
