# Opus codec API

<!-- claims: CLM-REF-016-CAP-001,CLM-REF-016-SOURCE-001 -->

## Scope

- **Encode and decode Opus.** Configure stateful Opus encoders and decoders and convert between PocketStation audio frames and packets.

These statements describe repository contracts at the documented snapshot. They do not extend platform qualification, performance, retry, or delivery guarantees beyond the native API contracts and executable evidence.

## Reference authority

The generated [docs.rs API](https://docs.rs/pocketstation/latest/pocketstation/) is the exhaustive symbol-level Rust reference. Human reference pages organize responsibilities and cross-boundary behavior; they do not duplicate every rustdoc signature.

## Public surface

| Declaration | Kind | Purpose | Source |
|---|---|---|---|
| `pocketstation::codec::constants::OPUS_FRAME_SAMPLES` | constant | 20 ms frame = 960 samples at 48 kHz (AUDIO-012). | `src/codec/constants.rs:5` |
| `pocketstation::codec::constants::OPUS_MAX_PACKET_BYTES` | constant | Maximum number of bytes the Opus encoder can emit per 20 ms frame. libopus guarantees this upper bound. | `src/codec/constants.rs:13` |
| `pocketstation::codec::constants::OPUS_SAMPLE_RATE_HZ` | constant | 48 000 Hz, mono, VOIP application profile (AUDIO-012 default). | `src/codec/constants.rs:2` |
| `pocketstation::codec::constants::VOICE_AGENT_FRAME_SAMPLES` | constant | 10 ms frame = 480 samples at 48 kHz (voice-agent low-latency mode, RFC 6716 §3.1). Ten milliseconds of mono PCM at 48 kHz. | `src/codec/constants.rs:9` |
| `pocketstation::codec::decoder::OpusDecodeError` | enum | Classifies failures reported as opus decode error. | `src/codec/decoder.rs:25` |
| `pocketstation::codec::encoder::OpusApplication` | enum | Opus application mode. | `src/codec/encoder.rs:58` |
| `pocketstation::codec::encoder::OpusChannels` | enum | Typed channel count for Opus — prevents silent u8 misuse. | `src/codec/encoder.rs:27` |
| `pocketstation::codec::encoder::OpusEncodeError` | enum | Classifies failures reported as opus encode error. | `src/codec/encoder.rs:131` |
| `pocketstation::codec::encoder::OpusFrameDuration` | enum | Supported Opus frame duration at 48 kHz. | `src/codec/encoder.rs:7` |
| `pocketstation::codec::encoder::OpusSampleRate` | enum | Typed sample rate. Opus internally always uses 48 kHz; this type makes the constraint explicit rather than hiding it behind a `u32` constant. | `src/codec/encoder.rs:44` |
| `pocketstation::codec::profile::StreamProfile` | enum | Enumerates the supported stream profile cases. | `src/codec/profile.rs:11` |
| `application` | function | Returns the application associated with `StreamProfile`. | `src/codec/profile.rs:41` |
| `bitrate_kbps` | function | Returns the bitrate kbps associated with `StreamProfile`. | `src/codec/profile.rs:51` |
| `channels` | function | Returns the channel count represented by `StreamProfile`. | `src/codec/profile.rs:21` |
| `count` | function | Returns the count associated with `OpusChannels`. | `src/codec/encoder.rs:33` |
| `decode_into` | function | Decode a compressed Opus packet into i16 samples, then convert to f32. | `src/codec/decoder.rs:81` |
| `decode_plc_into` | function | Conceal one missing packet while preserving libopus decoder state. | `src/codec/decoder.rs:116` |
| `default` | function | Returns the default `OpusDecoder` value. | `src/codec/decoder.rs:175` |
| `default` | function | Returns the default `OpusConfig` value. | `src/codec/encoder.rs:92` |
| `default` | function | Returns the default `OpusEncoder` value. | `src/codec/encoder.rs:303` |
| `encode_into` | function | Encode an interleaved PCM slice into `out`. | `src/codec/encoder.rs:235` |
| `frame_duration` | function | Returns the frame duration associated with `StreamProfile`. | `src/codec/profile.rs:31` |
| `frame_ms` | function | Returns the frame milliseconds associated with `StreamProfile`. | `src/codec/profile.rs:60` |
| `from_config` | function | Create an encoder from an explicit OpusConfig. | `src/codec/encoder.rs:173` |
| `hz` | function | Returns the hz associated with `OpusSampleRate`. | `src/codec/encoder.rs:49` |
| `is_stereo` | function | Returns whether stereo applies to `StreamProfile`. | `src/codec/profile.rs:69` |
| `new` | function | Mono decoder (48 kHz). Back-compatible default for the existing pipeline. | `src/codec/decoder.rs:39` |
| `new` | function | Create a new encoder with default config (48 kHz, mono, Voip, 20 ms). | `src/codec/encoder.rs:168` |
| `opus_config` | function | Returns the opus config associated with `StreamProfile`. | `src/codec/profile.rs:73` |
| `samples_at_48k` | function | Returns the samples at 48k associated with `OpusFrameDuration`. | `src/codec/encoder.rs:15` |
| `set_bitrate_kbps` | function | Update the live encoder bitrate. Called by CODEC_HINT handler (AUDIO-021). `kbps` = 0 switches to Opus auto (VBR). Safe to call mid-stream. | `src/codec/encoder.rs:280` |
| `set_complexity` | function | Set encoder complexity (0 = fastest, 10 = highest quality). | `src/codec/encoder.rs:274` |
| `stereo_broadcast` | function | 20 ms stereo audio transport profile with an explicit bitrate. | `src/codec/encoder.rs:116` |
| `validate_frame_sample_count` | function | Validate an interleaved frame length without reading its samples. | `src/codec/encoder.rs:210` |
| `voice_broadcast` | function | Standard 20 ms mono voice transport profile with in-band FEC. | `src/codec/encoder.rs:108` |
| `with_channels` | function | Decoder for an explicit channel layout and a maximum 20 ms packet. | `src/codec/decoder.rs:44` |
| `with_max_frame_duration` | function | Decoder with an explicit maximum packet duration. | `src/codec/decoder.rs:53` |
| `pocketstation::codec` | module | Real Opus encode, decode, and packet-loss concealment primitives. | `src/codec/mod.rs:1` |
| `pocketstation::codec::decoder::OpusDecoder` | struct | Real Opus decoder wrapping libopus via the `opus` crate. | `src/codec/decoder.rs:15` |
| `pocketstation::codec::encoder::OpusConfig` | struct | Explicit configuration for an Opus encoder instance. | `src/codec/encoder.rs:72` |
| `pocketstation::codec::encoder::OpusEncoder` | struct | Real Opus encoder wrapping libopus via the `opus` crate. | `src/codec/encoder.rs:157` |
| `OpusConfig::application` | struct_field | Opus application mode. | `src/codec/encoder.rs:80` |
| `OpusConfig::bitrate_kbps` | struct_field | Target bitrate in kbps. None = Opus auto (variable bitrate). | `src/codec/encoder.rs:82` |
| `OpusConfig::channels` | struct_field | Channel layout. | `src/codec/encoder.rs:76` |
| `OpusConfig::complexity` | struct_field | Encoder complexity 0–10. Higher = better quality, more CPU. | `src/codec/encoder.rs:84` |
| `OpusConfig::dtx` | struct_field | Discontinuous transmission (silence suppression). | `src/codec/encoder.rs:86` |
| `OpusConfig::fec` | struct_field | In-band forward error correction. | `src/codec/encoder.rs:88` |
| `OpusConfig::frame_duration` | struct_field | Frame duration. Default: 20 ms (AUDIO-012). | `src/codec/encoder.rs:78` |
| `OpusConfig::sample_rate` | struct_field | Sample rate. Opus only supports 48 kHz internally. | `src/codec/encoder.rs:74` |
| `OpusDecodeError::FrameDurationExceedsConfiguredMaximum::maximum_samples_per_channel` | struct_field | Stores the maximum samples per channel associated with `FrameDurationExceedsConfiguredMaximum`. | `src/codec/decoder.rs:31` |
| `OpusDecodeError::FrameDurationExceedsConfiguredMaximum::requested_samples_per_channel` | struct_field | Stores the requested samples per channel associated with `FrameDurationExceedsConfiguredMaximum`. | `src/codec/decoder.rs:30` |
| `OpusEncodeError::InvalidFrameSampleCount::channels` | struct_field | Stores the channels associated with `InvalidFrameSampleCount`. | `src/codec/encoder.rs:137` |
| `OpusEncodeError::InvalidFrameSampleCount::expected_sample_count` | struct_field | Stores the number of expected sample represented by `InvalidFrameSampleCount`. | `src/codec/encoder.rs:138` |
| `OpusEncodeError::InvalidFrameSampleCount::sample_count` | struct_field | Stores the number of sample represented by `InvalidFrameSampleCount`. | `src/codec/encoder.rs:136` |
| `pocketstation::codec::decoder::OpusDecodeError::FrameDurationExceedsConfiguredMaximum` | variant | Reports frame duration exceeds configured maximum. | `src/codec/decoder.rs:29` |
| `pocketstation::codec::decoder::OpusDecodeError::Opus` | variant | Reports opus. | `src/codec/decoder.rs:34` |
| `pocketstation::codec::encoder::OpusApplication::Audio` | variant | Optimised for audio quality (music/broadcast). | `src/codec/encoder.rs:64` |
| `pocketstation::codec::encoder::OpusApplication::LowDelay` | variant | Optimised for low algorithmic delay. Use for real-time voice agents. | `src/codec/encoder.rs:62` |
| `pocketstation::codec::encoder::OpusApplication::Voip` | variant | Optimised for voice (VOIP). Default for PocketStation broadcast. | `src/codec/encoder.rs:60` |
| `pocketstation::codec::encoder::OpusChannels::Mono` | variant | Represents the mono case of `OpusChannels`. | `src/codec/encoder.rs:28` |
| `pocketstation::codec::encoder::OpusChannels::Stereo` | variant | Represents the stereo case of `OpusChannels`. | `src/codec/encoder.rs:29` |
| `pocketstation::codec::encoder::OpusEncodeError::InvalidFrameSampleCount` | variant | Reports invalid frame sample count. | `src/codec/encoder.rs:135` |
| `pocketstation::codec::encoder::OpusEncodeError::Opus` | variant | Reports opus. | `src/codec/encoder.rs:141` |
| `pocketstation::codec::encoder::OpusFrameDuration::Ms10` | variant | Represents the ms10 case of `OpusFrameDuration`. | `src/codec/encoder.rs:8` |
| `pocketstation::codec::encoder::OpusFrameDuration::Ms20` | variant | Represents the ms20 case of `OpusFrameDuration`. | `src/codec/encoder.rs:9` |
| `pocketstation::codec::encoder::OpusFrameDuration::Ms40` | variant | Represents the ms40 case of `OpusFrameDuration`. | `src/codec/encoder.rs:10` |
| `pocketstation::codec::encoder::OpusFrameDuration::Ms60` | variant | Represents the ms60 case of `OpusFrameDuration`. | `src/codec/encoder.rs:11` |
| `pocketstation::codec::encoder::OpusSampleRate::Hz48000` | variant | Represents the hz48000 case of `OpusSampleRate`. | `src/codec/encoder.rs:45` |
| `pocketstation::codec::profile::StreamProfile::BroadcastStereo20ms` | variant | Represents the broadcast stereo20ms case of `StreamProfile`. | `src/codec/profile.rs:16` |
| `pocketstation::codec::profile::StreamProfile::HifiStereo20ms` | variant | Represents the hifi stereo20ms case of `StreamProfile`. | `src/codec/profile.rs:17` |
| `pocketstation::codec::profile::StreamProfile::MusicStereo10ms` | variant | Represents the music stereo10ms case of `StreamProfile`. | `src/codec/profile.rs:15` |
| `pocketstation::codec::profile::StreamProfile::MusicStereo20ms` | variant | Represents the music stereo20ms case of `StreamProfile`. | `src/codec/profile.rs:14` |
| `pocketstation::codec::profile::StreamProfile::VoiceAgentMono10ms` | variant | Represents the voice agent mono10ms case of `StreamProfile`. | `src/codec/profile.rs:13` |
| `pocketstation::codec::profile::StreamProfile::VoiceMono20ms` | variant | Represents the voice mono20ms case of `StreamProfile`. | `src/codec/profile.rs:12` |

## Interpretation

An inventory row establishes that a declaration, test, lifecycle operation, configuration surface, or protocol element exists at the frozen snapshot. Fields shown as unknown remain deliberately unspecified. Consult the native API and error contract before relying on panic behavior, blocking, cancellation, ordering, limits, retry, or recovery.

## Related documentation

- [Glossary](/docs/glossary.md)
- [Opus codec state](/docs/concepts/opus-codec.md)
- [PocketStation](/README.md)
- [Encode and decode Opus](/docs/how-to/opus-roundtrip.md)
- [Behavior evidence index](/docs/reference/behavior-evidence.md)
- [Configuration reference](/docs/reference/configuration.md)
- [Rust API reference](/docs/reference/rust-api.md)
- [Frame and codec failures](/docs/errors/frames-and-codec.md)

## Evidence boundary

This page was verified against Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `src/codec/mod.rs:1-20` (`DIRECT`)

A file's presence proves implementation or declaration at this snapshot. It does not by itself prove physical-device qualification, operational performance, retry safety, or behavior outside the recorded test conditions.
