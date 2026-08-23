# Return generated PCM through a bridge

<!-- claims: CLM-GUIDE-013-CAP-001,CLM-GUIDE-013-CAP-002,CLM-GUIDE-013-SOURCE-001 -->

## Scope

- **Bridge asynchronous output into audio.** Return generated PCM from asynchronous processing through an explicit bounded audio reentry bridge.
- **Implement asynchronous operators.** Register operator factories that consume and emit named typed signals on the asynchronous execution lane.

The scope of **Return generated PCM through a bridge** ends at the native contracts and executable conditions cited below. Platform qualification, performance, retry, and delivery require their own explicit evidence.

## Prerequisites

An async operator output declared as generated audio and a target sample specification for the audio lane.

## Procedure

1. Declare generated-audio output.
2. Prepare the bounded audio-reentry bridge.
3. Produce PCM matching the target sample specification.
4. Write from the asynchronous lane.
5. Observe accepted, saturated, closed, or cancelled outcomes.

## Important consequence

Never write generated PCM directly into the realtime lane or silently change its lineage.

## Verify the outcome

PCM writes are accepted by the bridge and Session audio-reentry observations advance without format or capacity failure.

Executable evidence selected for **Return generated PCM through a bridge** is limited to each test's recorded setup and assertions:

- `given_full_audio_ingress_when_bridge_sends_then_rejection_is_counted_exactly` — given full audio ingress when bridge sends then rejection is counted exactly (`src/runtime/bridge/audio.rs:497`; `test-b85e8c1c3aa436f769d2`).
- `given_operator_audio_when_bridged_then_owned_frame_enters_bounded_plan_source` — given operator audio when bridged then owned frame enters bounded plan source (`src/runtime/bridge/audio.rs:335`; `test-1dfdf14fa335d99dccdc`).
- `given_retained_audio_ingress_when_pool_is_exhausted_then_loss_is_counted_exactly` — given retained audio ingress when pool is exhausted then loss is counted exactly (`src/runtime/bridge/audio.rs:459`; `test-f2ba70a6cf90ac310e7e`).
- `given_passthrough_node_when_process_then_returns_frame_unchanged` — given passthrough node when process then returns frame unchanged (`src/graph/builtins.rs:300`; `test-746c4f775eb51b59004b`).
- `given_non_numeric_config_when_get_f32_then_returns_none` — given non numeric config when get f32 then returns none (`src/graph/node.rs:384`; `test-1a0a35dbae1595b149f8`).
- `given_non_numeric_config_when_get_u32_then_returns_none` — given non numeric config when get u32 then returns none (`src/graph/node.rs:396`; `test-09ecb72e5cbce06551db`).
- `given_different_partitions_when_needs_bridge_then_true` — given different partitions when needs bridge then true (`src/graph/partition.rs:156`; `test-cb0708a574885e0cd05f`).
- `given_same_partition_when_needs_bridge_then_false` — given same partition when needs bridge then false (`src/graph/partition.rs:150`; `test-d6bebce72b2ffc90170e`).
- `given_mono_and_stereo_when_channel_count_then_returns_one_and_two` — given mono and stereo when channel count then returns one and two (`src/graph/ports.rs:441`; `test-c9594b0d6a2254fbfa3a`).
- `given_empty_registry_when_get_unknown_then_returns_none` — given empty registry when get unknown then returns none (`src/graph/registry.rs:196`; `test-d66fedb0ba400a13dd8b`).
- `given_registered_factory_when_get_then_returns_some` — given registered factory when get then returns some (`src/graph/registry.rs:187`; `test-168f11ada7ce5d1ce624`).
- `given_echo_async_node_when_process_after_prepare_then_envelope_is_returned` — given echo async node when process after prepare then envelope is returned (`src/graph/signal/envelope.rs:233`; `test-e8f76f0070ca38093fa3`).

## Failure signals

- `pocketstation::runtime::bridge::audio::GeneratedAudioBridgeStartError` — `error-63f2053742785ad18a10`
- `pocketstation::runtime::bridge::audio::GeneratedAudioBridgeStartError` / `InvalidPoolSlots` — `error-02ec1c68a8d5ab15798e`
- `pocketstation::runtime::bridge::audio::GeneratedAudioBridgeStartError` / `InvalidSampleSpec` — `error-84545df6f1ae274fc828`
- `pocketstation::runtime::bridge::audio::GeneratedAudioBridgeStartError` / `ThreadStart` — `error-aaf5b740ae12e00d87c6`
- `pocketstation::runtime::bridge::audio::GeneratedAudioBridgeStartError` / `ZeroFrameSamples` — `error-5cef71fc6ad8f2f7cae3`
- `pocketstation::graph::compile::resolve::CompileError` — `error-bcdc75c15c75e9bec3b3`
- `pocketstation::graph::node::ConfigError` — `error-d03179e7825865c4d32a`
- `pocketstation::graph::node::NodeDescriptorError` — `error-3d209a7f79c1e6b33f61`
- `pocketstation::graph::node::NodeError` — `error-298180bd40fbfb711fda`
- `pocketstation::graph::node::NodeError` / `Config` — `error-d63ea2db29ed8a858fcc`

## API reference

- [Audio Reentry](/docs/concepts/audio-reentry.md)
- [Observations](/docs/reference/observations.md)

| Public declaration | Kind | Declared purpose | Source |
|---|---|---|---|
| `pocketstation::runtime::bridge::audio::GeneratedAudioBridge` | struct | Transfers generated audio across the bounded runtime boundary it owns. | `src/runtime/bridge/audio.rs:123` |
| `pocketstation::runtime::bridge::audio::GeneratedAudioBridgeSpec` | struct | Configures generated audio bridge behavior at its owning API boundary. | `src/runtime/bridge/audio.rs:19` |
| `pocketstation::runtime::bridge::audio::GeneratedAudioBridgeStartError` | enum | Classifies failures reported as generated audio bridge start error. | `src/runtime/bridge/audio.rs:46` |
| `pocketstation::runtime::bridge::audio::GeneratedAudioBridgeStartError::InvalidPoolSlots` | variant | Reported when the owning operation encounters invalid pool slots. | `src/runtime/bridge/audio.rs:52` |
| `pocketstation::runtime::bridge::audio::GeneratedAudioBridgeStartError::InvalidSampleSpec` | variant | Reported when the owning operation encounters invalid sample spec. | `src/runtime/bridge/audio.rs:48` |
| `pocketstation::runtime::bridge::audio::GeneratedAudioBridgeStartError::ThreadStart` | variant | Reported when the owning operation encounters thread start. | `src/runtime/bridge/audio.rs:54` |
| `pocketstation::runtime::bridge::audio::GeneratedAudioBridgeStartError::ZeroFrameSamples` | variant | Reported when the owning operation encounters zero frame samples. | `src/runtime/bridge/audio.rs:50` |
| `GeneratedAudioBridgeSpec::clock_id` | struct_field | Identifies the clock identifier recorded by `GeneratedAudioBridgeSpec`. | `src/runtime/bridge/audio.rs:24` |

## Related documentation

- [Glossary](/docs/glossary.md)
- [PocketStation](/README.md)
- [Behavior evidence index](/docs/reference/behavior-evidence.md)
- [Graph and route contracts](/docs/reference/graph.md)
- [Rust API reference](/docs/reference/rust-api.md)
- [Frames or signals are dropped](/docs/troubleshooting/drops-and-saturation.md)
- [Graph and signal failures](/docs/errors/graph-and-signals.md)
- [Asynchronous operators](/docs/concepts/async-operators.md)

## Evidence boundary

The claims on **Return generated PCM through a bridge** are anchored to Git snapshot `136e74888962558aa846d3143a19136a70936f45` and these primary files:

- `src/runtime/bridge/audio.rs:1-529` (`DIRECT`)

For **Return generated PCM through a bridge**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.
