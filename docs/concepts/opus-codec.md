# Opus codec state

<!-- claims: CLM-DOC-033-CAP-001,CLM-DOC-033-SOURCE-001 -->

Configure stateful Opus encoders and decoders and convert between PocketStation audio frames and packets.

## Scope

- **Encode and decode Opus.** Configure stateful Opus encoders and decoders and convert between PocketStation audio frames and packets.

These statements describe repository contracts at the documented snapshot. They do not extend platform qualification, performance, retry, or delivery guarantees beyond the native API contracts and executable evidence.

## Contract surface

| Public declaration | Kind | Declared purpose | Source |
|---|---|---|---|
| `pocketstation::codec::decoder::OpusDecoder` | struct | Real Opus decoder wrapping libopus via the `opus` crate. | `src/codec/decoder.rs:15` |
| `pocketstation::codec::encoder::OpusConfig` | struct | Explicit configuration for an Opus encoder instance. | `src/codec/encoder.rs:72` |
| `pocketstation::codec::encoder::OpusEncoder` | struct | Real Opus encoder wrapping libopus via the `opus` crate. | `src/codec/encoder.rs:157` |
| `pocketstation::codec::encoder::OpusApplication` | enum | Opus application mode. | `src/codec/encoder.rs:58` |
| `pocketstation::codec::encoder::OpusChannels` | enum | Typed channel count for Opus — prevents silent u8 misuse. | `src/codec/encoder.rs:27` |
| `pocketstation::codec::encoder::OpusFrameDuration` | enum | Supported Opus frame duration at 48 kHz. | `src/codec/encoder.rs:7` |
| `pocketstation::codec::encoder::OpusSampleRate` | enum | Typed sample rate. Opus internally always uses 48 kHz; this type makes the constraint explicit rather than hiding it behind a `u32` constant. | `src/codec/encoder.rs:44` |
| `pocketstation::codec::constants::OPUS_FRAME_SAMPLES` | constant | 20 ms frame = 960 samples at 48 kHz (AUDIO-012). | `src/codec/constants.rs:5` |
| `pocketstation::codec::constants::OPUS_MAX_PACKET_BYTES` | constant | Maximum number of bytes the Opus encoder can emit per 20 ms frame. libopus guarantees this upper bound. | `src/codec/constants.rs:13` |
| `pocketstation::codec::constants::OPUS_SAMPLE_RATE_HZ` | constant | 48 000 Hz, mono, VOIP application profile (AUDIO-012 default). | `src/codec/constants.rs:2` |
| `pocketstation::codec::encoder::OpusApplication::Audio` | variant | Optimised for audio quality (music/broadcast). | `src/codec/encoder.rs:64` |
| `pocketstation::codec::encoder::OpusApplication::LowDelay` | variant | Optimised for low algorithmic delay. Use for real-time voice agents. | `src/codec/encoder.rs:62` |
| `pocketstation::codec::encoder::OpusApplication::Voip` | variant | Optimised for voice (VOIP). Default for PocketStation broadcast. | `src/codec/encoder.rs:60` |
| `pocketstation::codec::decoder::OpusDecodeError` | enum | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/codec/decoder.rs:25` |
| `pocketstation::codec::encoder::OpusEncodeError` | enum | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/codec/encoder.rs:131` |
| `pocketstation::codec::decoder::OpusDecodeError::FrameDurationExceedsConfiguredMaximum` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/codec/decoder.rs:29` |
| `pocketstation::codec::decoder::OpusDecodeError::Opus` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/codec/decoder.rs:34` |
| `pocketstation::codec::encoder::OpusChannels::Mono` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/codec/encoder.rs:28` |
| `pocketstation::codec::encoder::OpusChannels::Stereo` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/codec/encoder.rs:29` |
| `pocketstation::codec::encoder::OpusEncodeError::InvalidFrameSampleCount` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/codec/encoder.rs:135` |

## Where you encounter it

- **Encode and decode a stream** — Configure Opus state and convert audio frames to packets and back.

## Behavior established by tests

The following test bodies are evidence only for their recorded setup:

- `given_encoded_opus_packet_when_decoded_then_contains_960_samples` — given encoded opus packet when decoded then contains 960 samples (`src/codec/decoder.rs:184`; `test-e9a038c59aa6148e49f9`).
- `given_20ms_opus_frame_when_sampled_at_48khz_then_contains_960_samples` — given 20ms opus frame when sampled at 48khz then contains 960 samples (`src/codec/encoder.rs:329`; `test-c4d783dc09c5ae8fcc9c`).
- `given_30s_sine_pcm_when_opus_round_trip_then_golden_file_invariants_pass` — given 30s sine pcm when opus round trip then golden file invariants pass (`src/codec/encoder.rs:672`; `test-d58d461257beb4e70632`).
- `given_sine_wave_when_opus_round_trip_runs_then_approximate_magnitude_is_preserved` — given sine wave when opus round trip runs then approximate magnitude is preserved (`src/codec/encoder.rs:393`; `test-a80dbf6b40aa2cc2df6c`).
- `given_sine_frame_when_codec_roundtrip_runs_then_sample_count_is_preserved` — given sine frame when codec roundtrip runs then sample count is preserved (`tests/codec_opus_roundtrip.rs:4`; `test-f2c6d3780d291652810b`).
- `given_20ms_decoder_when_60ms_concealment_is_requested_then_typed_bound_error_is_returned` — given 20ms decoder when 60ms concealment is requested then typed bound error is returned (`src/codec/decoder.rs:246`; `test-f2b28e6d34edfbf95af0`).
- `given_mono_decoder_when_concealing_10ms_then_480_samples_are_appended` — given mono decoder when concealing 10ms then 480 samples are appended (`src/codec/decoder.rs:203`; `test-7f059c025029ab48d42b`).
- `given_presized_output_when_concealing_repeatedly_then_capacity_stays_fixed` — given presized output when concealing repeatedly then capacity stays fixed (`src/codec/decoder.rs:229`; `test-3a6668288d371a23c543`).
- `given_stereo_decoder_when_concealing_20ms_then_1920_samples_are_appended` — given stereo decoder when concealing 20ms then 1920 samples are appended (`src/codec/decoder.rs:216`; `test-a33c833b56c360107d03`).
- `given_60ms_stereo_configuration_when_exact_frame_arrives_then_fixed_scratch_accepts_it` — given 60ms stereo configuration when exact frame arrives then fixed scratch accepts it (`src/codec/encoder.rs:571`; `test-27e8b931577e02ce30ec`).
- `given_960_sample_frame_when_encoded_then_packet_is_not_empty` — given 960 sample frame when encoded then packet is not empty (`src/codec/encoder.rs:334`; `test-6bdc61a7c604b08c13a4`).
- `given_configured_20ms_encoder_when_10ms_frame_arrives_then_exact_duration_is_enforced` — given configured 20ms encoder when 10ms frame arrives then exact duration is enforced (`src/codec/encoder.rs:553`; `test-bf950e1644072a871d53`).

## Boundaries

The compiler inventory establishes names, kinds, visibility, and signatures. Tests establish only their exercised conditions. Where retryability, ordering, cancellation, physical qualification, or recovery is not declared, this page leaves it unspecified.

## Related documentation

- [Glossary](/docs/glossary.md)
- [PocketStation](/README.md)
- [Encode and decode Opus](/docs/how-to/opus-roundtrip.md)
- [Behavior evidence index](/docs/reference/behavior-evidence.md)
- [Configuration reference](/docs/reference/configuration.md)
- [Opus codec API](/docs/reference/codec.md)
- [Rust API reference](/docs/reference/rust-api.md)
- [Frame and codec failures](/docs/errors/frames-and-codec.md)

## Evidence boundary

This page was verified against Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `src/codec/encoder.rs:1-860` (`DIRECT`)
- `src/codec/decoder.rs:1-267` (`DIRECT`)

A file's presence proves implementation or declaration at this snapshot. It does not by itself prove physical-device qualification, operational performance, retry safety, or behavior outside the recorded test conditions.
