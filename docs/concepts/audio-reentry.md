# Audio reentry

<!-- claims: CLM-DOC-023-CAP-001,CLM-DOC-023-SOURCE-001 -->

Return generated PCM from asynchronous processing through an explicit bounded audio reentry bridge.

## Scope

- **Bridge asynchronous output into audio.** Return generated PCM from asynchronous processing through an explicit bounded audio reentry bridge.

These statements describe repository contracts at the documented snapshot. They do not extend platform qualification, performance, retry, or delivery guarantees beyond the native API contracts and executable evidence.

## Contract surface

| Public declaration | Kind | Declared purpose | Source |
|---|---|---|---|
| `audio` | module | Allocation-free realtime audio execution lane. | `src/runtime/audio/mod.rs:1` |
| `audio` | function | Convenience constructor for PCM audio ports. | `src/graph/signal/spec.rs:269` |
| `encoded_audio` | function | Convenience constructor for encoded audio ports. | `src/graph/signal/spec.rs:274` |
| `is_audio` | function | Returns `true` for classes that carry real-time audio on the hot path. | `src/graph/signal/spec.rs:180` |
| `realtime_audio` | function | Generic realtime PCM edge. Concrete sample rate, frame size, and channel layout are negotiated from connected ports. | `src/graph/ports.rs:391` |
| `pocketstation::graph::partition::ExecutionPartition::AudioCallback` | variant | Platform OS audio callback — the strictest domain. | `src/graph/partition.rs:24` |
| `pocketstation::graph::signal::spec::SignalClass::EncodedAudio` | variant | Compressed audio bitstream (Opus packet, AAC frame, …). | `src/graph/signal/spec.rs:162` |
| `pocketstation::graph::signal::spec::SignalClass::PcmAudio` | variant | Interleaved PCM audio samples (format described by the edge AudioCaps). | `src/graph/signal/spec.rs:160` |
| `pocketstation::graph::ports::AudioCaps` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/graph/ports.rs:48` |
| `pocketstation::runtime::audio::router::EdgeObservations` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/runtime/audio/router.rs:122` |
| `pocketstation::runtime::audio::runner::PlanSourceInputObservations` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/runtime/audio/runner.rs:22` |
| `pocketstation::runtime::audio::executor::ExecError` | enum | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/runtime/audio/executor.rs:20` |
| `pocketstation::runtime::audio::runner::PlanRunnerError` | enum | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/runtime/audio/runner.rs:256` |
| `from_audio` | function | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/graph/signal/envelope.rs:27` |
| `pocketstation::graph::ports::LossPolicy::ConcealForAudio` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/graph/ports.rs:288` |
| `pocketstation::graph::ports::MediaCaps::Audio` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/graph/ports.rs:86` |
| `pocketstation::graph::ports::MediaCaps::EncodedAudio` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/graph/ports.rs:87` |
| `pocketstation::graph::ports::MediaKind::AudioEncoded` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/graph/ports.rs:18` |
| `pocketstation::graph::ports::MediaKind::AudioPcm` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/graph/ports.rs:17` |
| `pocketstation::graph::signal::payload::SignalPayload::Audio` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/graph/signal/payload.rs:11` |

## Where you encounter it

- **Return generated audio** — Bridge asynchronous PCM output back into the bounded audio lane.

## Behavior established by tests

The following test bodies are evidence only for their recorded setup:

- `given_audio_output_into_text_input_when_compiled_then_media_mismatch` — given audio output into text input when compiled then media mismatch (`src/graph/compile/resolve.rs:1079`; `test-ef4bd9407893b33d7b06`).
- `given_audio_signal_into_text_signal_when_compiled_then_signal_mismatch` — given audio signal into text signal when compiled then signal mismatch (`src/graph/compile/resolve.rs:1094`; `test-5516fcaedfc2241e34cc`).
- `given_allocation_allowed_contract_when_valid_for_audio_callback_then_false` — given allocation allowed contract when valid for audio callback then false (`src/graph/partition.rs:168`; `test-b01b34cf6e6f3258c91d`).
- `given_audio_callback_partition_when_requires_realtime_safety_then_true` — given audio callback partition when requires realtime safety then true (`src/graph/partition.rs:128`; `test-d54ff707a3d77d445397`).
- `given_partitions_when_ranked_then_audio_callback_is_lowest` — given partitions when ranked then audio callback is lowest (`src/graph/partition.rs:141`; `test-2dbca6af06117c1c9bdb`).
- `given_realtime_safe_contract_when_valid_for_audio_callback_then_true` — given realtime safe contract when valid for audio callback then true (`src/graph/partition.rs:162`; `test-0ed6d5e5aedfb9880e78`).
- `given_any_and_audio_when_negotiated_then_yields_audio` — given any and audio when negotiated then yields audio (`src/graph/ports.rs:496`; `test-b904fea87e8dcf2b473a`).
- `given_any_audio_caps_when_compat_checked_then_reflexive_and_symmetric` — given any audio caps when compat checked then reflexive and symmetric (`src/graph/ports.rs:607`; `test-c1f0182c0924086f9d64`).
- `given_audio_and_text_when_media_compat_checked_then_incompatible` — given audio and text when media compat checked then incompatible (`src/graph/ports.rs:483`; `test-49b93cf78810847cc5ff`).
- `given_audio_pair_when_media_compat_checked_then_compatible` — given audio pair when media compat checked then compatible (`src/graph/ports.rs:476`; `test-363da2d0a58f6635dc58`).
- `given_mismatched_rate_when_audio_compat_checked_then_incompatible` — given mismatched rate when audio compat checked then incompatible (`src/graph/ports.rs:467`; `test-8e8af7e321a63058b3c1`).
- `given_realtime_audio_when_built_then_physical_caps_remain_negotiable` — given realtime audio when built then physical caps remain negotiable (`src/graph/ports.rs:520`; `test-31c4ce2508a308db7cb9`).

## Boundaries

The compiler inventory establishes names, kinds, visibility, and signatures. Tests establish only their exercised conditions. Where retryability, ordering, cancellation, physical qualification, or recovery is not declared, this page leaves it unspecified.

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

This page was verified against Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `src/runtime/bridge/audio.rs:1-529` (`DIRECT`)

A file's presence proves implementation or declaration at this snapshot. It does not by itself prove physical-device qualification, operational performance, retry safety, or behavior outside the recorded test conditions.
