# Audio reentry

<!-- claims: CLM-DOC-023-CAP-001,CLM-DOC-023-SOURCE-001 -->

## What it is

Audio reentry is the bounded crossing that accepts generated PCM from asynchronous work and returns it to a typed audio route.

## Why it exists

Async operators cannot write directly into the realtime lane without an ownership, format, capacity, and cancellation boundary. The bridge makes those constraints observable.

## Relationships

- An operator declares generated-audio output.
- `GeneratedAudioBridgeSpec` fixes Session, stem, stream, source, clock, sample format, frame size, and pool capacity.
- Bridge observations appear in Session metrics and terminal diagnostics.

## Invariants and guarantees

- PCM must match the declared sample specification and frame size.
- Pool and route capacity remain finite.
- Closed, cancelled, saturated, and invalid-format outcomes remain distinct.

## When you encounter it

- **Return generated audio** — Bridge asynchronous PCM output back into the bounded audio lane.

## Use it

- [Return generated PCM through a bridge](/docs/how-to/return-generated-audio.md)
- [Frames or signals are dropped](/docs/troubleshooting/drops-and-saturation.md)

## Scope

- **Bridge asynchronous output into audio.** Return generated PCM from asynchronous processing through an explicit bounded audio reentry bridge.

The scope of **Audio reentry** ends at the native contracts and executable conditions cited below. Platform qualification, performance, retry, and delivery require their own explicit evidence.

## Key API

| Public declaration | Kind | Declared purpose | Source |
|---|---|---|---|
| `pocketstation::runtime::audio::executor::PlanExecutionSummary` | struct | Reports the counters and terminal facts collected for plan execution. | `src/runtime/audio/executor.rs:37` |
| `pocketstation::runtime::audio::executor::RealtimePlanExecutor` | struct | Executes realtime plan according to its compiled plan and cancellation contract. | `src/runtime/audio/executor.rs:54` |
| `pocketstation::runtime::bridge::audio::GeneratedAudioBridge` | struct | Transfers generated audio across the bounded runtime boundary it owns. | `src/runtime/bridge/audio.rs:123` |
| `pocketstation::runtime::bridge::audio::GeneratedAudioBridgeSpec` | struct | Configures generated audio bridge behavior at its owning API boundary. | `src/runtime/bridge/audio.rs:19` |
| `pocketstation::runtime::audio::executor::ExecError` | enum | Classifies failures reported as exec error. | `src/runtime/audio/executor.rs:20` |
| `pocketstation::runtime::bridge::audio::GeneratedAudioBridgeStartError` | enum | Classifies failures reported as generated audio bridge start error. | `src/runtime/bridge/audio.rs:46` |
| `pocketstation::runtime::audio::executor::ExecError::Node` | variant | Reported when the owning operation encounters node. | `src/runtime/audio/executor.rs:22` |
| `pocketstation::runtime::bridge::audio::GeneratedAudioBridgeStartError::InvalidPoolSlots` | variant | Reported when the owning operation encounters invalid pool slots. | `src/runtime/bridge/audio.rs:52` |
| `pocketstation::runtime::bridge::audio::GeneratedAudioBridgeStartError::InvalidSampleSpec` | variant | Reported when the owning operation encounters invalid sample spec. | `src/runtime/bridge/audio.rs:48` |
| `pocketstation::runtime::bridge::audio::GeneratedAudioBridgeStartError::ThreadStart` | variant | Reported when the owning operation encounters thread start. | `src/runtime/bridge/audio.rs:54` |

## Executable evidence

Executable evidence selected for **Audio reentry** is limited to each test's recorded setup and assertions:

- `given_full_audio_ingress_when_bridge_sends_then_rejection_is_counted_exactly` — given full audio ingress when bridge sends then rejection is counted exactly (`src/runtime/bridge/audio.rs:497`; `test-b85e8c1c3aa436f769d2`).
- `given_operator_audio_when_bridged_then_owned_frame_enters_bounded_plan_source` — given operator audio when bridged then owned frame enters bounded plan source (`src/runtime/bridge/audio.rs:335`; `test-1dfdf14fa335d99dccdc`).
- `given_retained_audio_ingress_when_pool_is_exhausted_then_loss_is_counted_exactly` — given retained audio ingress when pool is exhausted then loss is counted exactly (`src/runtime/bridge/audio.rs:459`; `test-f2ba70a6cf90ac310e7e`).
- `given_audio_output_into_text_input_when_compiled_then_media_mismatch` — given audio output into text input when compiled then media mismatch (`src/graph/compile/resolve.rs:1079`; `test-25a30affa1f9b95f7f53`).
- `given_audio_signal_into_text_signal_when_compiled_then_signal_mismatch` — given audio signal into text signal when compiled then signal mismatch (`src/graph/compile/resolve.rs:1094`; `test-25dddc1e95b562d4b54d`).
- `given_allocation_allowed_contract_when_valid_for_audio_callback_then_false` — given allocation allowed contract when valid for audio callback then false (`src/graph/partition.rs:168`; `test-f5998630daea9ae651bd`).
- `given_audio_callback_partition_when_requires_realtime_safety_then_true` — given audio callback partition when requires realtime safety then true (`src/graph/partition.rs:128`; `test-056027ee49ff3bf9cee1`).
- `given_partitions_when_ranked_then_audio_callback_is_lowest` — given partitions when ranked then audio callback is lowest (`src/graph/partition.rs:141`; `test-50eee42cf13227aa601f`).
- `given_realtime_safe_contract_when_valid_for_audio_callback_then_true` — given realtime safe contract when valid for audio callback then true (`src/graph/partition.rs:162`; `test-c83fea1299e667c8836c`).
- `given_any_and_audio_when_negotiated_then_yields_audio` — given any and audio when negotiated then yields audio (`src/graph/ports.rs:496`; `test-035239ef3f7bf3e07aee`).
- `given_any_audio_caps_when_compat_checked_then_reflexive_and_symmetric` — given any audio caps when compat checked then reflexive and symmetric (`src/graph/ports.rs:607`; `test-be77c480bd937a3d1134`).
- `given_audio_and_text_when_media_compat_checked_then_incompatible` — given audio and text when media compat checked then incompatible (`src/graph/ports.rs:483`; `test-f42a6f2027b03f953271`).

## Related documentation

- [Glossary](/docs/glossary.md)
- [PocketStation](/README.md)
- [Return generated PCM through a bridge](/docs/how-to/return-generated-audio.md)
- [Behavior evidence index](/docs/reference/behavior-evidence.md)
- [Graph and route contracts](/docs/reference/graph.md)
- [Rust API reference](/docs/reference/rust-api.md)
- [Frames or signals are dropped](/docs/troubleshooting/drops-and-saturation.md)
- [Graph and signal failures](/docs/errors/graph-and-signals.md)

## Evidence boundary

The claims on **Audio reentry** are anchored to Git snapshot `136e74888962558aa846d3143a19136a70936f45` and these primary files:

- `src/runtime/bridge/audio.rs:1-529` (`DIRECT`)

For **Audio reentry**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.
