# Encode and decode Opus

<!-- claims: CLM-GUIDE-024-CAP-001,CLM-GUIDE-024-CAP-002,CLM-GUIDE-024-SOURCE-001 -->

## Scope

- **Encode and decode Opus.** Configure stateful Opus encoders and decoders and convert between PocketStation audio frames and packets.
- **Preserve frame identity and lineage.** Carry source, stream, stem, route, clock, sequence, generation, and derivation identity with audio frames.

These statements describe repository contracts at the documented snapshot. They do not extend platform qualification, performance, retry, or delivery guarantees beyond the native API contracts and executable evidence.

## Prerequisites

Read the linked concept and confirm that target platform, Cargo features, source or provider dependencies, and application-owned permission work match this task. Keep returned typed errors and outcomes available for verification.

## Procedure

1. Choose an evidenced Opus profile and SampleSpec.
2. Construct a stateful encoder.
3. Encode only accepted frame formats.
4. Construct the matching decoder and decode packets.
5. Use the round-trip test as executable compatibility evidence.

## APIs used

| Public declaration | Kind | Declared purpose | Source |
|---|---|---|---|
| `pocketstation::codec::decoder::OpusDecoder` | struct | Real Opus decoder wrapping libopus via the `opus` crate. | `src/codec/decoder.rs:15` |
| `pocketstation::codec::encoder::OpusConfig` | struct | Explicit configuration for an Opus encoder instance. | `src/codec/encoder.rs:72` |
| `pocketstation::codec::encoder::OpusEncoder` | struct | Real Opus encoder wrapping libopus via the `opus` crate. | `src/codec/encoder.rs:157` |
| `pocketstation::codec::encoder::OpusApplication` | enum | Opus application mode. | `src/codec/encoder.rs:58` |
| `pocketstation::codec::encoder::OpusChannels` | enum | Typed channel count for Opus — prevents silent u8 misuse. | `src/codec/encoder.rs:27` |
| `pocketstation::codec::encoder::OpusFrameDuration` | enum | Supported Opus frame duration at 48 kHz. | `src/codec/encoder.rs:7` |
| `pocketstation::codec::encoder::OpusSampleRate` | enum | Typed sample rate. Opus internally always uses 48 kHz; this type makes the constraint explicit rather than hiding it behind a `u32` constant. | `src/codec/encoder.rs:44` |
| `pocketstation::codec::encoder::OpusApplication::Audio` | variant | Optimised for audio quality (music/broadcast). | `src/codec/encoder.rs:64` |
| `pocketstation::codec::encoder::OpusApplication::LowDelay` | variant | Optimised for low algorithmic delay. Use for real-time voice agents. | `src/codec/encoder.rs:62` |
| `pocketstation::codec::encoder::OpusApplication::Voip` | variant | Optimised for voice (VOIP). Default for PocketStation broadcast. | `src/codec/encoder.rs:60` |
| `pocketstation::codec::decoder::OpusDecodeError` | enum | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/codec/decoder.rs:25` |
| `pocketstation::codec::encoder::OpusEncodeError` | enum | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/codec/encoder.rs:131` |
| `pocketstation::codec::decoder::OpusDecodeError::FrameDurationExceedsConfiguredMaximum` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/codec/decoder.rs:29` |
| `pocketstation::codec::decoder::OpusDecodeError::Opus` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/codec/decoder.rs:34` |
| `pocketstation::codec::encoder::OpusChannels::Mono` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/codec/encoder.rs:28` |
| `pocketstation::codec::encoder::OpusChannels::Stereo` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/codec/encoder.rs:29` |

## Verify the outcome

The following test bodies are evidence only for their recorded setup:

- `given_encoded_opus_packet_when_decoded_then_contains_960_samples` — given encoded opus packet when decoded then contains 960 samples (`src/codec/decoder.rs:184`; `test-e9a038c59aa6148e49f9`).
- `given_20ms_decoder_when_60ms_concealment_is_requested_then_typed_bound_error_is_returned` — given 20ms decoder when 60ms concealment is requested then typed bound error is returned (`src/codec/decoder.rs:246`; `test-f2b28e6d34edfbf95af0`).
- `given_mono_decoder_when_concealing_10ms_then_480_samples_are_appended` — given mono decoder when concealing 10ms then 480 samples are appended (`src/codec/decoder.rs:203`; `test-7f059c025029ab48d42b`).
- `given_stereo_decoder_when_concealing_20ms_then_1920_samples_are_appended` — given stereo decoder when concealing 20ms then 1920 samples are appended (`src/codec/decoder.rs:216`; `test-a33c833b56c360107d03`).
- `given_20ms_opus_frame_when_sampled_at_48khz_then_contains_960_samples` — given 20ms opus frame when sampled at 48khz then contains 960 samples (`src/codec/encoder.rs:329`; `test-c4d783dc09c5ae8fcc9c`).
- `given_30s_sine_pcm_when_opus_round_trip_then_golden_file_invariants_pass` — given 30s sine pcm when opus round trip then golden file invariants pass (`src/codec/encoder.rs:672`; `test-d58d461257beb4e70632`).
- `given_960_sample_frame_when_encoded_then_packet_is_not_empty` — given 960 sample frame when encoded then packet is not empty (`src/codec/encoder.rs:334`; `test-6bdc61a7c604b08c13a4`).
- `given_configured_20ms_encoder_when_10ms_frame_arrives_then_exact_duration_is_enforced` — given configured 20ms encoder when 10ms frame arrives then exact duration is enforced (`src/codec/encoder.rs:553`; `test-bf950e1644072a871d53`).
- `given_optimised_encode_when_same_input_then_packet_bytes_identical` — given optimised encode when same input then packet bytes identical (`src/codec/encoder.rs:487`; `test-b46acca2f325d3bbcfb1`).
- `given_oversized_frame_when_encoded_then_error_is_typed_and_output_is_cleared` — given oversized frame when encoded then error is typed and output is cleared (`src/codec/encoder.rs:349`; `test-91992c707d12d6b613a9`).
- `given_partial_stereo_frame_when_encoded_then_error_is_typed` — given partial stereo frame when encoded then error is typed (`src/codec/encoder.rs:368`; `test-807b1db710dc9ad5f27a`).
- `given_sine_wave_when_opus_round_trip_runs_then_approximate_magnitude_is_preserved` — given sine wave when opus round trip runs then approximate magnitude is preserved (`src/codec/encoder.rs:393`; `test-a80dbf6b40aa2cc2df6c`).

## Failure signals

- `pocketstation::frame::pool::AudioBufferWriteError` / `CapacityExceeded` — `error-2317926ecc3df1fe0485`
- `pocketstation::frame::lineage::FrameLineageBuildError` / `ZeroSourceGeneration` — `error-2333fb8ed9ffc64dfe3d`
- `pocketstation::frame::lineage::FrameLineageBuildError` / `ZeroDuration` — `error-36112cc71bb577df5cc6`
- `pocketstation::frame::audio::AudioFrameBuildError` / `ZeroSampleRate` — `error-3d530ffcc82f2ae60152`
- `pocketstation::frame::pool::AudioBufferWriteError` — `error-44d619f15116bb8d5f0e`
- `pocketstation::frame::audio::AudioFrameBuildError` — `error-47bd33a1cf3d0c5fa264`
- `pocketstation::codec::encoder::OpusEncodeError` / `Opus` — `error-7f9c7f9db13f5030ecb1`
- `pocketstation::frame::lineage::FrameLineageBuildError` — `error-886f021bf510039ccdbb`
- `pocketstation::codec::encoder::OpusEncodeError` / `InvalidFrameSampleCount` — `error-a9fc3232ddadf6734ba1`
- `pocketstation::codec::encoder::OpusEncodeError` — `error-ab24633d76ea98a177e1`
- `pocketstation::codec::decoder::OpusDecodeError` / `FrameDurationExceedsConfiguredMaximum` — `error-bd82320c958728697aec`
- `pocketstation::frame::lineage::FrameLineageBuildError` / `TimestampOverflow` — `error-bd9d2580f5c500ca2920`

Retry only when the relevant API or error contract explicitly permits it. An error name, a transient-looking message, or a successful prior run is not retry evidence.

## Related documentation

- [Glossary](/docs/glossary.md)
- [PocketStation](/README.md)
- [Behavior evidence index](/docs/reference/behavior-evidence.md)
- [Rust API reference](/docs/reference/rust-api.md)
- [Frame and codec failures](/docs/errors/frames-and-codec.md)
- [Opus conversion fails](/docs/troubleshooting/opus.md)
- [Frame identity and lineage](/docs/concepts/frame-lineage.md)
- [Memory ownership and buffer pools](/docs/internals/memory-ownership.md)

## Evidence boundary

This page was verified against Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `tests/codec_opus_roundtrip.rs:1-33` (`DIRECT`)

A file's presence proves implementation or declaration at this snapshot. It does not by itself prove physical-device qualification, operational performance, retry safety, or behavior outside the recorded test conditions.
