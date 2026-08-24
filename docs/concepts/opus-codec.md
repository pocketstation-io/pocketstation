# Opus codec state

<!-- claims: CLM-DOC-033-SCOPE-001,CLM-DOC-033-TEXT-001,CLM-DOC-033-TEXT-002,CLM-DOC-033-TEXT-003,CLM-DOC-033-TEXT-004,CLM-DOC-033-TEXT-005,CLM-DOC-033-TEXT-006,CLM-DOC-033-SOURCE-001 -->

## What it is

PocketStation's Opus API owns stateful encoder and decoder instances configured by sample rate, channels, frame duration, application mode, and bitrate.

## Why it exists

Opus frames have constrained rates and sizes, and codec state spans calls. Typed profiles keep those constraints close to construction and conversion.

## Relationships

- `OpusConfig` creates encoder state.
- Audio frames provide PCM input and packet output retains the selected profile.
- The decoder must accept the corresponding packet and output shape.

## Invariants and guarantees

- Only supported Opus sample rates, channel layouts, and frame durations are accepted.
- Input PCM length must match the selected frame configuration.
- Codec errors remain typed and do not imply that retrying the same state is safe.

## When you encounter it

- **Encode and decode a stream** — Configure Opus state and convert audio frames to packets and back.

## Use it

- [Encode and decode Opus](/docs/how-to/opus-roundtrip.md)
- [Opus conversion fails](/docs/troubleshooting/opus.md)
- [Codec reference](/docs/reference/codec.md)

## Scope

- **Encode and decode Opus.** Configure stateful Opus encoders and decoders and convert between PocketStation audio frames and packets.

The scope of **Opus codec state** ends at the native contracts and executable conditions cited below. Platform qualification, performance, retry, and delivery require their own explicit evidence.

## Key API

| Public declaration | Kind | Declared purpose | Source |
|---|---|---|---|
| `pocketstation::codec::decoder::OpusDecoder` | struct | Real Opus decoder wrapping libopus via the `opus` crate. | `src/codec/decoder.rs:15` |
| `pocketstation::codec::encoder::OpusConfig` | struct | Explicit configuration for an Opus encoder instance. | `src/codec/encoder.rs:72` |
| `pocketstation::codec::encoder::OpusEncoder` | struct | Real Opus encoder wrapping libopus via the `opus` crate. | `src/codec/encoder.rs:157` |
| `pocketstation::codec::decoder::OpusDecodeError` | enum | Classifies failures produced during opus decoding. | `src/codec/decoder.rs:25` |
| `pocketstation::codec::encoder::OpusApplication` | enum | Selects the Opus encoder mode used to tune speech or general audio. | `src/codec/encoder.rs:58` |
| `pocketstation::codec::encoder::OpusChannels` | enum | Typed channel count for Opus — prevents silent u8 misuse. | `src/codec/encoder.rs:27` |
| `pocketstation::codec::encoder::OpusEncodeError` | enum | Classifies failures produced during opus encoding. | `src/codec/encoder.rs:131` |
| `pocketstation::codec::encoder::OpusFrameDuration` | enum | Supported Opus frame duration at 48 kHz. | `src/codec/encoder.rs:7` |
| `pocketstation::codec::encoder::OpusSampleRate` | enum | Typed sample rate. Opus internally always uses 48 kHz; this type makes the constraint explicit rather than hiding it behind a `u32` constant. | `src/codec/encoder.rs:44` |
| `pocketstation::codec::decoder::OpusDecodeError::FrameDurationExceedsConfiguredMaximum` | variant | Classifies a failure at the frame duration exceeds configured maximum stage or component of `OpusDecodeError`. | `src/codec/decoder.rs:29` |

## Executable evidence

Executable evidence selected for **Opus codec state** is limited to each test's recorded setup and assertions:

- `given_encoded_opus_packet_when_decoded_then_contains_960_samples` — given encoded opus packet when decoded then contains 960 samples (`src/codec/decoder.rs:184`; `test-18e769bfba9f736148f1`).
- `given_20ms_opus_frame_when_sampled_at_48khz_then_contains_960_samples` — given 20ms opus frame when sampled at 48khz then contains 960 samples (`src/codec/encoder.rs:329`; `test-c6b1615ef4535f8bea99`).
- `given_30s_sine_pcm_when_opus_round_trip_then_golden_file_invariants_pass` — given 30s sine pcm when opus round trip then golden file invariants pass (`src/codec/encoder.rs:672`; `test-77bb4d2c9b220a4d7130`).
- `given_sine_wave_when_opus_round_trip_runs_then_approximate_magnitude_is_preserved` — given sine wave when opus round trip runs then approximate magnitude is preserved (`src/codec/encoder.rs:393`; `test-8c9746a0022eeccced89`).
- `given_20ms_decoder_when_60ms_concealment_is_requested_then_typed_bound_error_is_returned` — given 20ms decoder when 60ms concealment is requested then typed bound error is returned (`src/codec/decoder.rs:246`; `test-f9eb27f8c697619303a9`).
- `given_mono_decoder_when_concealing_10ms_then_480_samples_are_appended` — given mono decoder when concealing 10ms then 480 samples are appended (`src/codec/decoder.rs:203`; `test-f3d9f0899054eb532922`).
- `given_presized_output_when_concealing_repeatedly_then_capacity_stays_fixed` — given presized output when concealing repeatedly then capacity stays fixed (`src/codec/decoder.rs:229`; `test-0a190981712f34057776`).
- `given_stereo_decoder_when_concealing_20ms_then_1920_samples_are_appended` — given stereo decoder when concealing 20ms then 1920 samples are appended (`src/codec/decoder.rs:216`; `test-fa484e4be3d65e78f9fc`).
- `given_60ms_stereo_configuration_when_exact_frame_arrives_then_fixed_scratch_accepts_it` — given 60ms stereo configuration when exact frame arrives then fixed scratch accepts it (`src/codec/encoder.rs:571`; `test-0c8f77a1048c94269c79`).
- `given_960_sample_frame_when_encoded_then_packet_is_not_empty` — given 960 sample frame when encoded then packet is not empty (`src/codec/encoder.rs:334`; `test-751e9c684179c8d414af`).
- `given_configured_20ms_encoder_when_10ms_frame_arrives_then_exact_duration_is_enforced` — given configured 20ms encoder when 10ms frame arrives then exact duration is enforced (`src/codec/encoder.rs:553`; `test-5183ec8e78897fa604e5`).
- `given_optimised_encode_when_same_input_then_packet_bytes_identical` — given optimised encode when same input then packet bytes identical (`src/codec/encoder.rs:487`; `test-afeb7f4f7006561b144e`).

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

The claims on **Opus codec state** are anchored to Git snapshot `136e74888962558aa846d3143a19136a70936f45` and these primary files:

- `src/codec/encoder.rs:6-6` (`DIRECT`)
- `src/codec/encoder.rs:6-6` (`DIRECT`)
- `src/codec/encoder.rs:6-6` (`DIRECT`)
- `src/codec/encoder.rs:7-12` (`DIRECT`)
- `src/codec/encoder.rs:8-8` (`DIRECT`)
- `src/codec/encoder.rs:9-9` (`DIRECT`)
- `src/codec/encoder.rs:10-10` (`DIRECT`)
- `src/codec/encoder.rs:11-11` (`DIRECT`)
- `src/codec/encoder.rs:15-22` (`DIRECT`)
- `src/codec/encoder.rs:26-26` (`DIRECT`)
- `src/codec/encoder.rs:26-26` (`DIRECT`)
- `src/codec/encoder.rs:26-26` (`DIRECT`)
- `src/codec/encoder.rs:27-30` (`DIRECT`)
- `src/codec/encoder.rs:28-28` (`DIRECT`)
- `src/codec/encoder.rs:29-29` (`DIRECT`)
- `src/codec/encoder.rs:33-38` (`DIRECT`)
- `src/codec/encoder.rs:43-43` (`DIRECT`)
- `src/codec/encoder.rs:43-43` (`DIRECT`)
- `src/codec/encoder.rs:43-43` (`DIRECT`)
- `src/codec/encoder.rs:44-46` (`DIRECT`)
- `src/codec/encoder.rs:45-45` (`DIRECT`)
- `src/codec/encoder.rs:49-53` (`DIRECT`)
- `src/codec/encoder.rs:57-57` (`DIRECT`)
- `src/codec/encoder.rs:57-57` (`DIRECT`)
- `src/codec/decoder.rs:15-22` (`DIRECT`)
- `src/codec/decoder.rs:16-16` (`DIRECT`)
- `src/codec/decoder.rs:20-20` (`DIRECT`)
- `src/codec/decoder.rs:21-21` (`DIRECT`)
- `src/codec/decoder.rs:24-24` (`DIRECT`)
- `src/codec/decoder.rs:24-24` (`DIRECT`)
- `src/codec/decoder.rs:24-24` (`DIRECT`)
- `src/codec/decoder.rs:24-24` (`DIRECT`)
- `src/codec/decoder.rs:25-35` (`DIRECT`)
- `src/codec/decoder.rs:29-32` (`DIRECT`)
- `src/codec/decoder.rs:30-30` (`DIRECT`)
- `src/codec/decoder.rs:31-31` (`DIRECT`)
- `src/codec/decoder.rs:34-34` (`DIRECT`)
- `src/codec/decoder.rs:34-34` (`DIRECT`)
- `src/codec/decoder.rs:39-41` (`DIRECT`)
- `src/codec/decoder.rs:44-46` (`DIRECT`)
- `src/codec/decoder.rs:53-66` (`DIRECT`)
- `src/codec/decoder.rs:81-109` (`DIRECT`)
- `src/codec/decoder.rs:116-150` (`DIRECT`)
- `src/codec/decoder.rs:153-172` (`DIRECT`)
- `src/codec/decoder.rs:175-177` (`DIRECT`)

For **Opus codec state**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.
