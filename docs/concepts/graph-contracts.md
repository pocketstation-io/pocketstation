# Graph contracts

<!-- claims: CLM-DOC-020-CAP-001,CLM-DOC-020-SOURCE-001 -->

## What it is

Graph contracts describe port direction, signal and media compatibility, multiplicity, execution partition, safety, copy, loss, delivery, and observability before a route is compiled.

## Why it exists

A processing graph crosses ownership and execution boundaries. Explicit contracts let the compiler reject incompatible or unsafe connections rather than discovering them after workers start.

## Relationships

- Node and operator manifests own named ports.
- Edge contracts control finite delivery between compatible ports.
- The runtime plan preserves the validated topology and policies.

## Invariants and guarantees

- Input and output direction must agree.
- Signal and media specifications must be compatible.
- Realtime partitions cannot claim operations that violate their declared safety contract.

## When you encounter it

- **Add an asynchronous operator** — Declare typed ports, implement an operator factory, and route its output.

## Use it

- [Choose route capacity and loss policy](/docs/how-to/configure-route-policy.md)
- [Connect named operator ports](/docs/how-to/connect-operator-ports.md)
- [Graph API reference](/docs/reference/graph.md)

## Scope

- **Describe graph contracts.** Declare typed ports, media capabilities, partition safety, copy, loss, delivery, and observability policy.

The scope of **Graph contracts** ends at the native contracts and executable conditions cited below. Platform qualification, performance, retry, and delivery require their own explicit evidence.

## Key API

| Public declaration | Kind | Declared purpose | Source |
|---|---|---|---|
| `pocketstation::graph::ports::AudioCaps` | struct | Declares the sample formats, channel layouts, and rates accepted by an audio port. | `src/graph/ports.rs:48` |
| `pocketstation::graph::ports::EdgeContract` | struct | Declares the validated constraints applied to edge. | `src/graph/ports.rs:311` |
| `pocketstation::graph::ports::PortSpec` | struct | Configures port behavior at its owning API boundary. | `src/graph/ports.rs:175` |
| `pocketstation::graph::spec::EdgeId` | struct | Uniquely identifies edge within its PocketStation ownership scope. | `src/graph/spec.rs:22` |
| `pocketstation::graph::spec::EdgeSpec` | struct | Configures edge behavior at its owning API boundary. | `src/graph/spec.rs:50` |
| `pocketstation::graph::spec::GraphSpec` | struct | Configures graph behavior at its owning API boundary. | `src/graph/spec.rs:58` |
| `pocketstation::graph::spec::InputPortRef` | struct | Names an operator or endpoint input port used as the target of a graph connection. | `src/graph/spec.rs:37` |
| `pocketstation::graph::spec::NodeId` | struct | Uniquely identifies node within its PocketStation ownership scope. | `src/graph/spec.rs:8` |
| `pocketstation::graph::spec::NodeSpec` | struct | Configures node behavior at its owning API boundary. | `src/graph/spec.rs:43` |
| `pocketstation::graph::spec::OutputPortRef` | struct | Names an operator output port used as the origin of a graph connection. | `src/graph/spec.rs:31` |

## Executable evidence

Executable evidence selected for **Graph contracts** is limited to each test's recorded setup and assertions:

- `given_any_and_audio_when_negotiated_then_yields_audio` — given any and audio when negotiated then yields audio (`src/graph/ports.rs:496`; `test-b904fea87e8dcf2b473a`).
- `given_any_audio_caps_when_compat_checked_then_reflexive_and_symmetric` — given any audio caps when compat checked then reflexive and symmetric (`src/graph/ports.rs:607`; `test-c1f0182c0924086f9d64`).
- `given_any_layout_when_compat_checked_both_directions_then_matches` — given any layout when compat checked both directions then matches (`src/graph/ports.rs:448`; `test-53058cf141f24f476947`).
- `given_any_media_when_compat_checked_both_directions_then_matches` — given any media when compat checked both directions then matches (`src/graph/ports.rs:489`; `test-52064d0ac0dc51bea641`).
- `given_audio_and_text_when_media_compat_checked_then_incompatible` — given audio and text when media compat checked then incompatible (`src/graph/ports.rs:483`; `test-49b93cf78810847cc5ff`).
- `given_audio_pair_when_media_compat_checked_then_compatible` — given audio pair when media compat checked then compatible (`src/graph/ports.rs:476`; `test-363da2d0a58f6635dc58`).
- `given_bounded_async_when_built_then_contains_no_payload_or_clock_origin_assumption` — given bounded async when built then contains no payload or clock origin assumption (`src/graph/ports.rs:537`; `test-764d8a62597c3f9220c7`).
- `given_custom_signal_without_schema_when_checked_then_binary_media_rejects_it` — given custom signal without schema when checked then binary media rejects it (`src/graph/ports.rs:575`; `test-2d3675af8d6c3a4d6a26`).
- `given_incompatible_media_when_negotiated_then_none` — given incompatible media when negotiated then none (`src/graph/ports.rs:502`; `test-9e99ae329b9711ba02b0`).
- `given_mismatched_rate_when_audio_compat_checked_then_incompatible` — given mismatched rate when audio compat checked then incompatible (`src/graph/ports.rs:467`; `test-8e8af7e321a63058b3c1`).
- `given_mono_and_stereo_when_channel_count_then_returns_one_and_two` — given mono and stereo when channel count then returns one and two (`src/graph/ports.rs:441`; `test-8304caec6a9e3b31e801`).
- `given_observability_levels_when_ranked_then_ordered_ascending` — given observability levels when ranked then ordered ascending (`src/graph/ports.rs:553`; `test-91b82bbdd4b3f972899f`).

## Related documentation

- [Glossary](/docs/glossary.md)
- [PocketStation](/README.md)
- [Choose route capacity and loss policy](/docs/how-to/configure-route-policy.md)
- [Connect named operator ports](/docs/how-to/connect-operator-ports.md)
- [Fan out one source](/docs/how-to/fan-out-source.md)
- [Implement an asynchronous operator](/docs/how-to/implement-operator.md)
- [Behavior evidence index](/docs/reference/behavior-evidence.md)
- [Graph and route contracts](/docs/reference/graph.md)

## Evidence boundary

The claims on **Graph contracts** are anchored to Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `src/graph/ports.rs:1-618` (`DIRECT`)

For **Graph contracts**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.
