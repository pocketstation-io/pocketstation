# Choose route capacity and loss policy

<!-- claims: CLM-GUIDE-008-CAP-001,CLM-GUIDE-008-CAP-002,CLM-GUIDE-008-SOURCE-001 -->

## Scope

- **Route realtime audio.** Deliver pooled audio frames through independent fixed-capacity routes governed by explicit edge policy.
- **Describe graph contracts.** Declare typed ports, media capabilities, partition safety, copy, loss, delivery, and observability policy.

The scope of **Choose route capacity and loss policy** ends at the native contracts and executable conditions cited below. Platform qualification, performance, retry, and delivery require their own explicit evidence.

## Prerequisites

The producer and consumer execution partitions, acceptable loss behavior, and observations you can measure.

## Procedure

1. Identify producer and consumer partitions.
2. Choose finite capacity.
3. Select backpressure, loss, copy, delivery, and observation policies.
4. Compile and handle rejected contracts.
5. Measure queue depth, saturation, and drops before changing capacity.

## Important consequence

Capacity is a workload decision; change it only after observing the constrained route.

## Verify the outcome

The declaration compiles, route depth stays within capacity, and saturation or drops match the selected policy.

Executable evidence selected for **Choose route capacity and loss policy** is limited to each test's recorded setup and assertions:

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

## Failure signals

- `pocketstation::graph::ports::PortSpecError` — `error-5601bc96e0f09d517ffa`
- `pocketstation::graph::ports::PortSpecError` / `EmptyName` — `error-de68a6f4314abffa41f2`
- `pocketstation::graph::ports::PortSpecError` / `InvalidSignal` — `error-144a7c7033b72fb3ebe8`
- `pocketstation::graph::ports::PortSpecError` / `SignalMediaMismatch` — `error-41dfb8544f872cc47db6`
- `pocketstation::runtime::audio::router::PlanRouterError` / `ZeroCapacity` — `error-7255bf1a56077c9e285a`
- `pocketstation::graph::signal::continuity::SignalContinuityError` / `PolicyRegressed` — `error-f2258ab42f5ae5dce1b8`
- `pocketstation::graph::signal::operator::AsyncOperatorManifestError` / `UnsupportedInputCopyPolicy` — `error-6d81801c1dd5a761b3e3`
- `pocketstation::graph::signal::operator::AsyncOperatorManifestError` / `ZeroQueueCapacity` — `error-ef879570e842b0c2eb11`
- `pocketstation::graph::signal::operator::OperatorFailurePolicy` — `error-44fb15ca3442b2f85ae6`
- `pocketstation::graph::signal::operator::OperatorFailurePolicy` / `Continue` — `error-6ed8b75a0a575926d427`

## API reference

- [Graph Contracts](/docs/concepts/graph-contracts.md)
- [Route Sizing](/docs/best-practices/route-sizing.md)

| Public declaration | Kind | Declared purpose | Source |
|---|---|---|---|
| `pocketstation::graph::ports::LossPolicy` | enum | Selects the loss policy used by PocketStation. | `src/graph/ports.rs:287` |
| `pocketstation::graph::ports::LossPolicy::ConcealForAudio` | variant | Selects conceal for audio behavior for `LossPolicy`. | `src/graph/ports.rs:288` |
| `pocketstation::graph::ports::LossPolicy::DropAllowed` | variant | Selects drop allowed behavior for `LossPolicy`. | `src/graph/ports.rs:290` |
| `pocketstation::graph::ports::LossPolicy::MustDeliverOrFail` | variant | Selects must deliver or fail behavior for `LossPolicy`. | `src/graph/ports.rs:289` |
| `pocketstation::runtime::audio::router::PlanRouterError::ZeroCapacity` | variant | Reported when the owning operation encounters zero capacity. | `src/runtime/audio/router.rs:21` |
| `PlanRouterError::ZeroCapacity::edge_id` | struct_field | Identifies the edge identifier recorded by `ZeroCapacity`. | `src/runtime/audio/router.rs:21` |
| `pocketstation::runtime::audio::router::DispatchSummary` | struct | Reports the counters and terminal facts collected for dispatch. | `src/runtime/audio/router.rs:667` |
| `pocketstation::runtime::audio::router::EdgeObservations` | struct | Reports the edge observations collected at an observation boundary. | `src/runtime/audio/router.rs:122` |

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

The claims on **Choose route capacity and loss policy** are anchored to Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `src/graph/ports.rs:1-618` (`DIRECT`)

For **Choose route capacity and loss policy**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.
