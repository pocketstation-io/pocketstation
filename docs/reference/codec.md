# Opus codec API

<!-- claims: CLM-REF-016-CAP-001,CLM-REF-016-SOURCE-001 -->

## Scope

- **Encode and decode Opus.** Configure stateful Opus encoders and decoders and convert between PocketStation audio frames and packets.

The scope of **Opus codec API** ends at the native contracts and executable conditions cited below. Platform qualification, performance, retry, and delivery require their own explicit evidence.

## Reference authority

For **Opus codec API**, the generated [docs.rs API](https://docs.rs/pocketstation/latest/pocketstation/) provides compiler-rendered signatures and navigation. This repository page adds the frozen evidence identifiers, responsibilities, and cross-boundary interpretation used by the documentation verifier.

## Public surface

| Evidence record | Declaration | Kind | Purpose | Source |
|---|---|---|---|---|
| sym-e337789afb958cf9dbf0 | `pocketstation::codec::constants::OPUS_FRAME_SAMPLES` | constant | 20 ms frame = 960 samples at 48 kHz (AUDIO-012). | `src/codec/constants.rs:5` |
| sym-b9f643402b922afb3fbb | `pocketstation::codec::constants::OPUS_MAX_PACKET_BYTES` | constant | Maximum number of bytes the Opus encoder can emit per 20 ms frame. libopus guarantees this upper bound. | `src/codec/constants.rs:13` |
| sym-ccf921e9b7f8fda11b0c | `pocketstation::codec::constants::OPUS_SAMPLE_RATE_HZ` | constant | 48 000 Hz, mono, VOIP application profile (AUDIO-012 default). | `src/codec/constants.rs:2` |
| sym-3a17d11b79ead1143bbc | `pocketstation::codec::constants::VOICE_AGENT_FRAME_SAMPLES` | constant | 10 ms frame = 480 samples at 48 kHz (voice-agent low-latency mode, RFC 6716 §3.1). Ten milliseconds of mono PCM at 48 kHz. | `src/codec/constants.rs:9` |
| sym-1396fee9ba1a6fac3da4 | `pocketstation::codec::decoder::OpusDecodeError` | enum | Classifies failures reported as opus decode error. | `src/codec/decoder.rs:25` |
| sym-76c962dec9e247b27a0b | `pocketstation::codec::encoder::OpusApplication` | enum | Selects the Opus encoder mode used to tune speech or general audio. | `src/codec/encoder.rs:58` |
| sym-05ac9bbc5f498492100a | `pocketstation::codec::encoder::OpusChannels` | enum | Typed channel count for Opus — prevents silent u8 misuse. | `src/codec/encoder.rs:27` |
| sym-a8a0d170d170e25329c3 | `pocketstation::codec::encoder::OpusEncodeError` | enum | Classifies failures reported as opus encode error. | `src/codec/encoder.rs:131` |
| sym-96b7637b5d8bfeeb0b34 | `pocketstation::codec::encoder::OpusFrameDuration` | enum | Supported Opus frame duration at 48 kHz. | `src/codec/encoder.rs:7` |
| sym-8afbed3a6983510a1a34 | `pocketstation::codec::encoder::OpusSampleRate` | enum | Typed sample rate. Opus internally always uses 48 kHz; this type makes the constraint explicit rather than hiding it behind a `u32` constant. | `src/codec/encoder.rs:44` |
| sym-aa25e74e66f28fe8efaf | `pocketstation::codec::profile::StreamProfile` | enum | Enumerates the supported stream profile cases. | `src/codec/profile.rs:11` |
| sym-b458165e8136073dfb32 | `application` | function | Returns the application held by `StreamProfile`. | `src/codec/profile.rs:41` |
| sym-685ef51f186153db0929 | `bitrate_kbps` | function | Returns the bitrate kbps associated with `StreamProfile`. | `src/codec/profile.rs:51` |
| sym-be0ad9663b006de72f97 | `channels` | function | Returns the channel count represented by `StreamProfile`. | `src/codec/profile.rs:21` |
| sym-b64a7061c2e4a315d65e | `count` | function | Returns the count associated with `OpusChannels`. | `src/codec/encoder.rs:33` |
| sym-ce095fd9d80ee307bb44 | `decode_into` | function | Decode a compressed Opus packet into i16 samples, then convert to f32. | `src/codec/decoder.rs:81` |
| sym-6b088b00b1c70ccd8a09 | `decode_plc_into` | function | Conceal one missing packet while preserving libopus decoder state. | `src/codec/decoder.rs:116` |
| sym-b02bba0830f20e8d62da | `default` | function | Returns the default `OpusDecoder` value. | `src/codec/decoder.rs:175` |
| sym-68385e80df96d1bfbd0b | `default` | function | Returns the default `OpusConfig` value. | `src/codec/encoder.rs:92` |
| sym-bf39d5649ed2cf6816ae | `default` | function | Returns the default `OpusEncoder` value. | `src/codec/encoder.rs:303` |
| sym-bd08dd5e0b763265e78c | `encode_into` | function | Encode an interleaved PCM slice into `out`. | `src/codec/encoder.rs:235` |
| sym-659bb287ac18a802b571 | `frame_duration` | function | Returns the frame duration associated with `StreamProfile`. | `src/codec/profile.rs:31` |
| sym-1d82ffe4b3fa9063e868 | `frame_ms` | function | Returns the frame milliseconds held by `StreamProfile`. | `src/codec/profile.rs:60` |
| sym-a80bc336b907a200eb6c | `from_config` | function | Create an encoder from an explicit OpusConfig. | `src/codec/encoder.rs:173` |
| sym-e605d954cd2a4de7a570 | `hz` | function | Returns the hz associated with `OpusSampleRate`. | `src/codec/encoder.rs:49` |
| sym-a456b018e1b43fa68057 | `is_stereo` | function | Returns whether stereo applies to `StreamProfile`. | `src/codec/profile.rs:69` |
| sym-c570bc1e1b1cb54d5277 | `new` | function | Mono decoder (48 kHz). Back-compatible default for the existing pipeline. | `src/codec/decoder.rs:39` |
| sym-69d564ec2595d96c9068 | `new` | function | Create a new encoder with default config (48 kHz, mono, Voip, 20 ms). | `src/codec/encoder.rs:168` |
| sym-e13219d16a165056747a | `opus_config` | function | Returns the opus config held by `StreamProfile`. | `src/codec/profile.rs:73` |
| sym-c39686ced9b7cca3273f | `samples_at_48k` | function | Returns the samples at 48k associated with `OpusFrameDuration`. | `src/codec/encoder.rs:15` |
| sym-8774dae4ad64c18d818f | `set_bitrate_kbps` | function | Update the live encoder bitrate. Called by CODEC_HINT handler (AUDIO-021). `kbps` = 0 switches to Opus auto (VBR). Safe to call mid-stream. | `src/codec/encoder.rs:280` |
| sym-fb5a243d1ff52e6d66fb | `set_complexity` | function | Set encoder complexity (0 = fastest, 10 = highest quality). | `src/codec/encoder.rs:274` |
| sym-64020a60cae03227b5c5 | `stereo_broadcast` | function | 20 ms stereo audio transport profile with an explicit bitrate. | `src/codec/encoder.rs:116` |
| sym-227e98c682197657ad15 | `validate_frame_sample_count` | function | Validate an interleaved frame length without reading its samples. | `src/codec/encoder.rs:210` |
| sym-a1257ac50f1e954932ce | `voice_broadcast` | function | Standard 20 ms mono voice transport profile with in-band FEC. | `src/codec/encoder.rs:108` |
| sym-a15f96724e9ad314cc50 | `with_channels` | function | Decoder for an explicit channel layout and a maximum 20 ms packet. | `src/codec/decoder.rs:44` |
| sym-70f9f3ea7a5631909337 | `with_max_frame_duration` | function | Decoder with an explicit maximum packet duration. | `src/codec/decoder.rs:53` |
| sym-db747a3f71f6ea35f826 | `pocketstation::codec` | module | Real Opus encode, decode, and packet-loss concealment primitives. | `src/codec/mod.rs:1` |
| sym-036dc1f9db8126134b2a | `pocketstation::codec::decoder::OpusDecoder` | struct | Real Opus decoder wrapping libopus via the `opus` crate. | `src/codec/decoder.rs:15` |
| sym-da3f913eba209c5fe886 | `pocketstation::codec::encoder::OpusConfig` | struct | Explicit configuration for an Opus encoder instance. | `src/codec/encoder.rs:72` |
| sym-7c89d9af3a6611d2d853 | `pocketstation::codec::encoder::OpusEncoder` | struct | Real Opus encoder wrapping libopus via the `opus` crate. | `src/codec/encoder.rs:157` |
| sym-c4bc476bb4fb56c40af9 | `OpusConfig::application` | struct_field | Selects the Opus application mode used when the encoder is created. | `src/codec/encoder.rs:80` |
| sym-2b5616507ada775062c4 | `OpusConfig::bitrate_kbps` | struct_field | Target bitrate in kbps. None = Opus auto (variable bitrate). | `src/codec/encoder.rs:82` |
| sym-007e3405aec6a783cc0d | `OpusConfig::channels` | struct_field | Selects the mono or stereo channel layout accepted by the encoder. | `src/codec/encoder.rs:76` |
| sym-2d0fcaa9179bea6d3de5 | `OpusConfig::complexity` | struct_field | Encoder complexity 0–10. Higher = better quality, more CPU. | `src/codec/encoder.rs:84` |
| sym-4484c0aeca53126e0cb2 | `OpusConfig::dtx` | struct_field | Discontinuous transmission (silence suppression). | `src/codec/encoder.rs:86` |
| sym-3bb363207da0a142620f | `OpusConfig::fec` | struct_field | In-band forward error correction. | `src/codec/encoder.rs:88` |
| sym-191188211296f8867449 | `OpusConfig::frame_duration` | struct_field | Frame duration. Default: 20 ms (AUDIO-012). | `src/codec/encoder.rs:78` |
| sym-dd0722b130cdb77f7f4e | `OpusConfig::sample_rate` | struct_field | Sample rate. Opus only supports 48 kHz internally. | `src/codec/encoder.rs:74` |
| sym-923be3ef9dae457fab1d | `OpusDecodeError::FrameDurationExceedsConfiguredMaximum::maximum_samples_per_channel` | struct_field | Stores the maximum samples per channel used by `FrameDurationExceedsConfiguredMaximum`. | `src/codec/decoder.rs:31` |
| sym-243218e6112065479597 | `OpusDecodeError::FrameDurationExceedsConfiguredMaximum::requested_samples_per_channel` | struct_field | Stores the requested samples per channel used by `FrameDurationExceedsConfiguredMaximum`. | `src/codec/decoder.rs:30` |
| sym-e32f4a59510868afd2bc | `OpusEncodeError::InvalidFrameSampleCount::channels` | struct_field | Stores the channels used by `InvalidFrameSampleCount`. | `src/codec/encoder.rs:137` |
| sym-67adeb362ec785bcc0fe | `OpusEncodeError::InvalidFrameSampleCount::expected_sample_count` | struct_field | Stores the number of expected sample represented by `InvalidFrameSampleCount`. | `src/codec/encoder.rs:138` |
| sym-ff5649d385c39e7d37d4 | `OpusEncodeError::InvalidFrameSampleCount::sample_count` | struct_field | Stores the number of sample represented by `InvalidFrameSampleCount`. | `src/codec/encoder.rs:136` |
| sym-10a4cf032c94e76b3506 | `pocketstation::codec::decoder::OpusDecodeError::FrameDurationExceedsConfiguredMaximum` | variant | Reported when the owning operation encounters frame duration exceeds configured maximum. | `src/codec/decoder.rs:29` |
| sym-f417caaef2042123ce64 | `pocketstation::codec::decoder::OpusDecodeError::Opus` | variant | Reported when the owning operation encounters opus. | `src/codec/decoder.rs:34` |
| sym-ecc00b68a32277246721 | `pocketstation::codec::encoder::OpusApplication::Audio` | variant | Optimised for audio quality (music/broadcast). | `src/codec/encoder.rs:64` |
| sym-1652077fd260d98f177c | `pocketstation::codec::encoder::OpusApplication::LowDelay` | variant | Optimised for low algorithmic delay. Use for real-time voice agents. | `src/codec/encoder.rs:62` |
| sym-cbf6c012d02d0ae69f39 | `pocketstation::codec::encoder::OpusApplication::Voip` | variant | Optimised for voice (VOIP). Default for PocketStation broadcast. | `src/codec/encoder.rs:60` |
| sym-9c6b15493cde15faf095 | `pocketstation::codec::encoder::OpusChannels::Mono` | variant | Represents the mono alternative defined by `OpusChannels`. | `src/codec/encoder.rs:28` |
| sym-3c73cd164c2199acfa05 | `pocketstation::codec::encoder::OpusChannels::Stereo` | variant | Represents the stereo alternative defined by `OpusChannels`. | `src/codec/encoder.rs:29` |
| sym-377c0738bf43a1977a97 | `pocketstation::codec::encoder::OpusEncodeError::InvalidFrameSampleCount` | variant | Reported when the owning operation encounters invalid frame sample count. | `src/codec/encoder.rs:135` |
| sym-f8c0585f37697c747541 | `pocketstation::codec::encoder::OpusEncodeError::Opus` | variant | Reported when the owning operation encounters opus. | `src/codec/encoder.rs:141` |
| sym-e932cc0c78380cd86c51 | `pocketstation::codec::encoder::OpusFrameDuration::Ms10` | variant | Represents the ms10 alternative defined by `OpusFrameDuration`. | `src/codec/encoder.rs:8` |
| sym-18f0d7c4b9b79ff0cdc2 | `pocketstation::codec::encoder::OpusFrameDuration::Ms20` | variant | Represents the ms20 alternative defined by `OpusFrameDuration`. | `src/codec/encoder.rs:9` |
| sym-8cf875a3f62bd248876e | `pocketstation::codec::encoder::OpusFrameDuration::Ms40` | variant | Represents the ms40 alternative defined by `OpusFrameDuration`. | `src/codec/encoder.rs:10` |
| sym-a27eab0d01739805fe30 | `pocketstation::codec::encoder::OpusFrameDuration::Ms60` | variant | Represents the ms60 alternative defined by `OpusFrameDuration`. | `src/codec/encoder.rs:11` |
| sym-3c06e15db5dea6f25652 | `pocketstation::codec::encoder::OpusSampleRate::Hz48000` | variant | Represents the hz48000 alternative defined by `OpusSampleRate`. | `src/codec/encoder.rs:45` |
| sym-77eec08117142051b187 | `pocketstation::codec::profile::StreamProfile::BroadcastStereo20ms` | variant | Represents the broadcast stereo20ms alternative defined by `StreamProfile`. | `src/codec/profile.rs:16` |
| sym-3f3ddb38bbd3cd82a214 | `pocketstation::codec::profile::StreamProfile::HifiStereo20ms` | variant | Represents the hifi stereo20ms alternative defined by `StreamProfile`. | `src/codec/profile.rs:17` |
| sym-bd0cd0fdb9cfa1a50c72 | `pocketstation::codec::profile::StreamProfile::MusicStereo10ms` | variant | Represents the music stereo10ms alternative defined by `StreamProfile`. | `src/codec/profile.rs:15` |
| sym-ca2cefc05155eb7ef074 | `pocketstation::codec::profile::StreamProfile::MusicStereo20ms` | variant | Represents the music stereo20ms alternative defined by `StreamProfile`. | `src/codec/profile.rs:14` |
| sym-8a5da12da7e042a23279 | `pocketstation::codec::profile::StreamProfile::VoiceAgentMono10ms` | variant | Represents the voice agent mono10ms alternative defined by `StreamProfile`. | `src/codec/profile.rs:13` |
| sym-abccff0cbe71b4b13565 | `pocketstation::codec::profile::StreamProfile::VoiceMono20ms` | variant | Represents the voice mono20ms alternative defined by `StreamProfile`. | `src/codec/profile.rs:12` |

## Interpretation

The **Opus codec API** inventory records compiler-visible or extracted evidence at the frozen snapshot. A field marked unknown or not declared remains outside the published guarantee; use the native signature, owning error type, and cited test before relying on panic, blocking, cancellation, ordering, limits, retry, or recovery behavior.

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

The claims on **Opus codec API** are anchored to Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `src/codec/mod.rs:1-20` (`DIRECT`)

For **Opus codec API**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.
