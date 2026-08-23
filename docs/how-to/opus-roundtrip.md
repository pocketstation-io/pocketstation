# Encode and decode Opus

<!-- claims: CLM-GUIDE-024-CAP-001,CLM-GUIDE-024-CAP-002,CLM-GUIDE-024-SOURCE-001 -->

## Scope

- **Encode and decode Opus.** Configure stateful Opus encoders and decoders and convert between PocketStation audio frames and packets.
- **Preserve frame identity and lineage.** Carry source, stream, stem, route, clock, sequence, generation, and derivation identity with audio frames.

The scope of **Encode and decode Opus** ends at the native contracts and executable conditions cited below. Platform qualification, performance, retry, and delivery require their own explicit evidence.

## Prerequisites

An Opus profile supported by the crate and PCM frames matching its sample rate, channels, and duration.

## Procedure

1. Choose an evidenced Opus profile and SampleSpec.
2. Construct a stateful encoder.
3. Encode only accepted frame formats.
4. Construct the matching decoder and decode packets.
5. Use the round-trip test as executable compatibility evidence.

## Important consequence

Keep encoder and decoder state aligned; a codec error does not establish safe replay of the same state.

## Verify the outcome

The encoder produces a packet and the matching decoder returns the expected frame shape under the repository round-trip test.

Executable evidence selected for **Encode and decode Opus** is limited to each test's recorded setup and assertions:

- `given_sine_frame_when_codec_roundtrip_runs_then_sample_count_is_preserved` — given sine frame when codec roundtrip runs then sample count is preserved (`tests/codec_opus_roundtrip.rs:4`; `test-f2c6d3780d291652810b`).
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

## Failure signals

- `pocketstation::codec::decoder::OpusDecodeError` — `error-9b6a20dfec56d0f963ec`
- `pocketstation::codec::decoder::OpusDecodeError` / `FrameDurationExceedsConfiguredMaximum` — `error-4055838a830f20f7900a`
- `pocketstation::codec::decoder::OpusDecodeError` / `Opus` — `error-7b6f20bfd81327986061`
- `pocketstation::codec::encoder::OpusEncodeError` — `error-ae09263b8f4f85f0d5e8`
- `pocketstation::codec::encoder::OpusEncodeError` / `InvalidFrameSampleCount` — `error-edbece7c0fc9e4199d02`
- `pocketstation::codec::encoder::OpusEncodeError` / `Opus` — `error-3beedf48b3ab09500606`

## API reference

- [Opus Codec](/docs/concepts/opus-codec.md)
- [Codec](/docs/reference/codec.md)

| Public declaration | Kind | Declared purpose | Source |
|---|---|---|---|
| `pocketstation::codec::decoder::OpusDecoder` | struct | Real Opus decoder wrapping libopus via the `opus` crate. | `src/codec/decoder.rs:15` |
| `pocketstation::codec::encoder::OpusConfig` | struct | Explicit configuration for an Opus encoder instance. | `src/codec/encoder.rs:72` |
| `pocketstation::codec::encoder::OpusEncoder` | struct | Real Opus encoder wrapping libopus via the `opus` crate. | `src/codec/encoder.rs:157` |
| `pocketstation::codec::decoder::OpusDecodeError` | enum | Classifies failures reported as opus decode error. | `src/codec/decoder.rs:25` |
| `pocketstation::codec::encoder::OpusApplication` | enum | Selects the Opus encoder mode used to tune speech or general audio. | `src/codec/encoder.rs:58` |
| `pocketstation::codec::encoder::OpusChannels` | enum | Typed channel count for Opus — prevents silent u8 misuse. | `src/codec/encoder.rs:27` |
| `pocketstation::codec::encoder::OpusEncodeError` | enum | Classifies failures reported as opus encode error. | `src/codec/encoder.rs:131` |
| `pocketstation::codec::encoder::OpusFrameDuration` | enum | Supported Opus frame duration at 48 kHz. | `src/codec/encoder.rs:7` |

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

The claims on **Encode and decode Opus** are anchored to Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `tests/codec_opus_roundtrip.rs:1-33` (`DIRECT`)

For **Encode and decode Opus**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.
