# Encode and decode Opus

<!-- claims: CLM-GUIDE-024-SCOPE-001,CLM-GUIDE-024-TEXT-001,CLM-GUIDE-024-TEXT-002,CLM-GUIDE-024-TEXT-003,CLM-GUIDE-024-TEXT-004,CLM-GUIDE-024-TEXT-005,CLM-GUIDE-024-TEXT-006,CLM-GUIDE-024-SOURCE-001 -->

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

## Concrete repository example

The executable repository test `given_sine_frame_when_codec_roundtrip_runs_then_sample_count_is_preserved` (`test-8b8900f4ef016b6914cc`) shows the concrete API sequence and asserted outcome at `tests/codec_opus_roundtrip.rs:4`.

```rust
};

#[test]
fn given_sine_frame_when_codec_roundtrip_runs_then_sample_count_is_preserved() {
    let pool = AudioBufferPool::new(4, POOL_SLOT_SAMPLES);
    let mut h = pool.acquire().unwrap();
    for (sample_index, sample) in h.as_mut_slice().iter_mut().enumerate() {
        let phase = sample_index as f32 * std::f32::consts::TAU * 440.0 / 48_000.0;
        *sample = phase.sin() * 0.25;
    }
    let frame = AudioFrame::try_new(
        StreamId::new(1),
        SourceId::new(1),
        0,
        0,
        SampleSpec::new(48_000, 1, SampleFormat::F32Interleaved),
        h,
    )
    .expect("valid codec frame");
    let mut enc = OpusEncoder::new().expect("OpusEncoder::new failed");
    let mut dec = OpusDecoder::new().expect("OpusDecoder::new failed");
    let mut encoded = Vec::with_capacity(OPUS_MAX_PACKET_BYTES);
    enc.encode_into(frame.samples(), &mut encoded)
        .expect("encode failed");
    let mut decoded = Vec::with_capacity(POOL_SLOT_SAMPLES);
    dec.decode_into(&encoded, &mut decoded, false)
        .expect("decode failed");
    assert_eq!(decoded.len(), POOL_SLOT_SAMPLES);
}
```

```bash
cargo test --all-features given_sine_frame_when_codec_roundtrip_runs_then_sample_count_is_preserved
```

## Important consequence

Keep encoder and decoder state aligned; a codec error does not establish safe replay of the same state.

## Verify the outcome

The encoder produces a packet and the matching decoder returns the expected frame shape under the repository round-trip test.

Executable evidence selected for **Encode and decode Opus** is limited to each test's recorded setup and assertions:

- `given_sine_frame_when_codec_roundtrip_runs_then_sample_count_is_preserved` — given sine frame when codec roundtrip runs then sample count is preserved (`tests/codec_opus_roundtrip.rs:4`; `test-8b8900f4ef016b6914cc`).
- `given_encoded_opus_packet_when_decoded_then_contains_960_samples` — given encoded opus packet when decoded then contains 960 samples (`src/codec/decoder.rs:184`; `test-18e769bfba9f736148f1`).
- `given_20ms_decoder_when_60ms_concealment_is_requested_then_typed_bound_error_is_returned` — given 20ms decoder when 60ms concealment is requested then typed bound error is returned (`src/codec/decoder.rs:246`; `test-f9eb27f8c697619303a9`).
- `given_mono_decoder_when_concealing_10ms_then_480_samples_are_appended` — given mono decoder when concealing 10ms then 480 samples are appended (`src/codec/decoder.rs:203`; `test-f3d9f0899054eb532922`).
- `given_stereo_decoder_when_concealing_20ms_then_1920_samples_are_appended` — given stereo decoder when concealing 20ms then 1920 samples are appended (`src/codec/decoder.rs:216`; `test-fa484e4be3d65e78f9fc`).
- `given_20ms_opus_frame_when_sampled_at_48khz_then_contains_960_samples` — given 20ms opus frame when sampled at 48khz then contains 960 samples (`src/codec/encoder.rs:329`; `test-c6b1615ef4535f8bea99`).
- `given_30s_sine_pcm_when_opus_round_trip_then_golden_file_invariants_pass` — given 30s sine pcm when opus round trip then golden file invariants pass (`src/codec/encoder.rs:672`; `test-77bb4d2c9b220a4d7130`).
- `given_960_sample_frame_when_encoded_then_packet_is_not_empty` — given 960 sample frame when encoded then packet is not empty (`src/codec/encoder.rs:334`; `test-751e9c684179c8d414af`).
- `given_configured_20ms_encoder_when_10ms_frame_arrives_then_exact_duration_is_enforced` — given configured 20ms encoder when 10ms frame arrives then exact duration is enforced (`src/codec/encoder.rs:553`; `test-5183ec8e78897fa604e5`).
- `given_optimised_encode_when_same_input_then_packet_bytes_identical` — given optimised encode when same input then packet bytes identical (`src/codec/encoder.rs:487`; `test-afeb7f4f7006561b144e`).
- `given_oversized_frame_when_encoded_then_error_is_typed_and_output_is_cleared` — given oversized frame when encoded then error is typed and output is cleared (`src/codec/encoder.rs:349`; `test-655f8034c539e1dc9c27`).
- `given_partial_stereo_frame_when_encoded_then_error_is_typed` — given partial stereo frame when encoded then error is typed (`src/codec/encoder.rs:368`; `test-9e747f83ae1e676c3e7e`).

## Failure signals

- `pocketstation::codec::decoder::OpusDecodeError` — `error-3d9be3e3f583928d23f4`
- `pocketstation::codec::decoder::OpusDecodeError` / `FrameDurationExceedsConfiguredMaximum` — `error-84f459285f24936c9e00`
- `pocketstation::codec::decoder::OpusDecodeError` / `Opus` — `error-e714868780d8cd7a5a64`
- `pocketstation::codec::encoder::OpusEncodeError` — `error-eda36a61ae1109dce21c`
- `pocketstation::codec::encoder::OpusEncodeError` / `InvalidFrameSampleCount` — `error-bef40f89fba6bbf83b1c`
- `pocketstation::codec::encoder::OpusEncodeError` / `Opus` — `error-0100bb86433fb5001cce`

## API reference

- [Opus Codec](/docs/concepts/opus-codec.md)
- [Codec](/docs/reference/codec.md)

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

The claims on **Encode and decode Opus** are anchored to Git snapshot `136e74888962558aa846d3143a19136a70936f45` and these primary files:

- `tests/codec_opus_roundtrip.rs:4-32` (`TESTED`)

For **Encode and decode Opus**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.
