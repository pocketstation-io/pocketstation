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

- `given_full_audio_ingress_when_bridge_sends_then_rejection_is_counted_exactly` — given full audio ingress when bridge sends then rejection is counted exactly (`src/runtime/bridge/audio.rs:497`; `test-c49159871ef385421381`).
- `given_operator_audio_when_bridged_then_owned_frame_enters_bounded_plan_source` — given operator audio when bridged then owned frame enters bounded plan source (`src/runtime/bridge/audio.rs:335`; `test-0ae60369d5962ff55b0f`).
- `given_retained_audio_ingress_when_pool_is_exhausted_then_loss_is_counted_exactly` — given retained audio ingress when pool is exhausted then loss is counted exactly (`src/runtime/bridge/audio.rs:459`; `test-1664fa1aa12573253d70`).
- `given_passthrough_node_when_process_then_returns_frame_unchanged` — given passthrough node when process then returns frame unchanged (`src/graph/builtins.rs:300`; `test-681eed046f58e8486db9`).
- `given_non_numeric_config_when_get_f32_then_returns_none` — given non numeric config when get f32 then returns none (`src/graph/node.rs:384`; `test-e8a88787bf728526e24e`).
- `given_non_numeric_config_when_get_u32_then_returns_none` — given non numeric config when get u32 then returns none (`src/graph/node.rs:396`; `test-822e3a824202ce79818e`).
- `given_different_partitions_when_needs_bridge_then_true` — given different partitions when needs bridge then true (`src/graph/partition.rs:156`; `test-715ab55819026a5e2ee1`).
- `given_same_partition_when_needs_bridge_then_false` — given same partition when needs bridge then false (`src/graph/partition.rs:150`; `test-eff95761245ec9c01147`).
- `given_mono_and_stereo_when_channel_count_then_returns_one_and_two` — given mono and stereo when channel count then returns one and two (`src/graph/ports.rs:441`; `test-8304caec6a9e3b31e801`).
- `given_empty_registry_when_get_unknown_then_returns_none` — given empty registry when get unknown then returns none (`src/graph/registry.rs:196`; `test-b951b0b08bbe3b9fb23a`).
- `given_registered_factory_when_get_then_returns_some` — given registered factory when get then returns some (`src/graph/registry.rs:187`; `test-c3d829c8e4d87e90b4f0`).
- `given_echo_async_node_when_process_after_prepare_then_envelope_is_returned` — given echo async node when process after prepare then envelope is returned (`src/graph/signal/envelope.rs:233`; `test-9d67f3359220613efda8`).

## Failure signals

- `pocketstation::runtime::bridge::audio::GeneratedAudioBridgeStartError` — `error-06c9ed5aca510482d20b`
- `pocketstation::runtime::bridge::audio::GeneratedAudioBridgeStartError` / `InvalidPoolSlots` — `error-8e0b53591f08cb47068e`
- `pocketstation::runtime::bridge::audio::GeneratedAudioBridgeStartError` / `InvalidSampleSpec` — `error-e1157bd0c212286a966a`
- `pocketstation::runtime::bridge::audio::GeneratedAudioBridgeStartError` / `ThreadStart` — `error-6c1f04e94d399bc4d454`
- `pocketstation::runtime::bridge::audio::GeneratedAudioBridgeStartError` / `ZeroFrameSamples` — `error-a32547cd6342dd0315cc`
- `pocketstation::graph::compile::resolve::CompileError` — `error-ed02262328948395fc81`
- `pocketstation::graph::node::ConfigError` — `error-a40a6a70ccfabb71722d`
- `pocketstation::graph::node::NodeDescriptorError` — `error-9a066e9f78f364e655fd`
- `pocketstation::graph::node::NodeError` — `error-d9fa2ee902e569cf2691`
- `pocketstation::graph::node::NodeError` / `Config` — `error-a3f8758a059f27327504`

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

The claims on **Return generated PCM through a bridge** are anchored to Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `src/runtime/bridge/audio.rs:1-529` (`DIRECT`)

For **Return generated PCM through a bridge**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.
