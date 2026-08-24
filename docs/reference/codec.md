# Opus codec API

<!-- claims: CLM-REF-016-SCOPE-001,CLM-REF-016-TEXT-001,CLM-REF-016-TEXT-002,CLM-REF-016-SOURCE-001 -->

## Scope

- **Encode and decode Opus.** Configure stateful Opus encoders and decoders and convert between PocketStation audio frames and packets.

The scope of **Opus codec API** ends at the native contracts and executable conditions cited below. Platform qualification, performance, retry, and delivery require their own explicit evidence.

## Reference authority

For **Opus codec API**, the generated [docs.rs API](https://docs.rs/pocketstation/latest/pocketstation/) provides compiler-rendered signatures and navigation. This repository page adds the frozen evidence identifiers, responsibilities, and cross-boundary interpretation used by the documentation verifier.

## Public surface

| Evidence record | Declaration | Kind | Purpose | Source |
|---|---|---|---|---|
| sym-fef590d7370c703d21f0 | `pocketstation::codec::constants::OPUS_FRAME_SAMPLES` | constant | 20 ms frame = 960 samples at 48 kHz (AUDIO-012). | `src/codec/constants.rs:5` |
| sym-2a520fd0f59b0d4e8007 | `pocketstation::codec::constants::OPUS_MAX_PACKET_BYTES` | constant | Maximum number of bytes the Opus encoder can emit per 20 ms frame. libopus guarantees this upper bound. | `src/codec/constants.rs:13` |
| sym-a09db02191ca16a128fa | `pocketstation::codec::constants::OPUS_SAMPLE_RATE_HZ` | constant | 48 000 Hz, mono, VOIP application profile (AUDIO-012 default). | `src/codec/constants.rs:2` |
| sym-0873bf4291fe4840e3c7 | `pocketstation::codec::constants::VOICE_AGENT_FRAME_SAMPLES` | constant | 10 ms frame = 480 samples at 48 kHz (voice-agent low-latency mode, RFC 6716 §3.1). Ten milliseconds of mono PCM at 48 kHz. | `src/codec/constants.rs:9` |
| sym-222549994b61881e820d | `pocketstation::codec::decoder::OpusDecodeError` | enum | Classifies failures produced during opus decoding. | `src/codec/decoder.rs:25` |
| sym-8a3a4031d3f6082ba4c1 | `pocketstation::codec::encoder::OpusApplication` | enum | Selects the Opus encoder mode used to tune speech or general audio. | `src/codec/encoder.rs:58` |
| sym-9226f5cf30a6da6fae37 | `pocketstation::codec::encoder::OpusChannels` | enum | Typed channel count for Opus — prevents silent u8 misuse. | `src/codec/encoder.rs:27` |
| sym-184d3f010be6d087cba0 | `pocketstation::codec::encoder::OpusEncodeError` | enum | Classifies failures produced during opus encoding. | `src/codec/encoder.rs:131` |
| sym-4a458387887842682d94 | `pocketstation::codec::encoder::OpusFrameDuration` | enum | Supported Opus frame duration at 48 kHz. | `src/codec/encoder.rs:7` |
| sym-8cbe8e70763fec07c936 | `pocketstation::codec::encoder::OpusSampleRate` | enum | Typed sample rate. Opus internally always uses 48 kHz; this type makes the constraint explicit rather than hiding it behind a `u32` constant. | `src/codec/encoder.rs:44` |
| sym-292549adb1d699be0ee1 | `pocketstation::codec::profile::StreamProfile` | enum | Selects the supported Opus stream profile used for codec validation. | `src/codec/profile.rs:11` |
| sym-6f89da131606829b0842 | `application` | function | Returns the application held by `StreamProfile`. | `src/codec/profile.rs:41` |
| sym-8f2182e0218cb6561476 | `bitrate_kbps` | function | Returns the bitrate kbps associated with `StreamProfile`. | `src/codec/profile.rs:51` |
| sym-c488d4533e942b4a7698 | `channels` | function | Returns the channel count represented by `StreamProfile`. | `src/codec/profile.rs:21` |
| sym-25b9f1dfb7e287566018 | `count` | function | Returns the count associated with `OpusChannels`. | `src/codec/encoder.rs:33` |
| sym-10aacc26c0e2fa7a7394 | `decode_into` | function | Decode a compressed Opus packet into i16 samples, then convert to f32. | `src/codec/decoder.rs:81` |
| sym-5841419d47fad0e02f75 | `decode_plc_into` | function | Conceal one missing packet while preserving libopus decoder state. | `src/codec/decoder.rs:116` |
| sym-b5f91227cb949970ac0c | `default` | function | Returns the default `OpusDecoder` value. | `src/codec/decoder.rs:175` |
| sym-9e807c1498052f3fa447 | `default` | function | Returns the default `OpusConfig` value. | `src/codec/encoder.rs:92` |
| sym-66537e7c28d4505fc8eb | `default` | function | Returns the default `OpusEncoder` value. | `src/codec/encoder.rs:303` |
| sym-dc78315d25ca0081c076 | `encode_into` | function | Encode an interleaved PCM slice into `out`. | `src/codec/encoder.rs:235` |
| sym-b2a72b13670ff17c8d51 | `frame_duration` | function | Returns the frame duration associated with `StreamProfile`. | `src/codec/profile.rs:31` |
| sym-b49dc5d31d95b5b4cddc | `frame_ms` | function | Returns the frame milliseconds held by `StreamProfile`. | `src/codec/profile.rs:60` |
| sym-ed540708783254c6310f | `from_config` | function | Create an encoder from an explicit OpusConfig. | `src/codec/encoder.rs:173` |
| sym-951ac3ec1d62f3926c6c | `hz` | function | Returns the hz associated with `OpusSampleRate`. | `src/codec/encoder.rs:49` |
| sym-fa0244c5440eb6e18364 | `is_stereo` | function | Reports whether stereo is true for `StreamProfile`. | `src/codec/profile.rs:69` |
| sym-83741bae354adc93a18c | `new` | function | Mono decoder (48 kHz). Back-compatible default for the existing pipeline. | `src/codec/decoder.rs:39` |
| sym-f69c5522aea4db917a27 | `new` | function | Create a new encoder with default config (48 kHz, mono, Voip, 20 ms). | `src/codec/encoder.rs:168` |
| sym-af5e48f013d42753aaaf | `opus_config` | function | Returns the opus config held by `StreamProfile`. | `src/codec/profile.rs:73` |
| sym-c561ccb29a1915c9b687 | `samples_at_48k` | function | Returns the samples at 48k associated with `OpusFrameDuration`. | `src/codec/encoder.rs:15` |
| sym-174cfb8ae938db7b41e2 | `set_bitrate_kbps` | function | Update the live encoder bitrate. Called by CODEC_HINT handler (AUDIO-021). `kbps` = 0 switches to Opus auto (VBR). Safe to call mid-stream. | `src/codec/encoder.rs:280` |
| sym-82a0413af3d835e8998b | `set_complexity` | function | Set encoder complexity (0 = fastest, 10 = highest quality). | `src/codec/encoder.rs:274` |
| sym-d34077d6fda42b2b23e2 | `stereo_broadcast` | function | 20 ms stereo audio transport profile with an explicit bitrate. | `src/codec/encoder.rs:116` |
| sym-f0ed0a688dd79c5febf5 | `validate_frame_sample_count` | function | Validate an interleaved frame length without reading its samples. | `src/codec/encoder.rs:210` |
| sym-81cb0865578354002668 | `voice_broadcast` | function | Standard 20 ms mono voice transport profile with in-band FEC. | `src/codec/encoder.rs:108` |
| sym-12bc89c2553a5ab2528e | `with_channels` | function | Decoder for an explicit channel layout and a maximum 20 ms packet. | `src/codec/decoder.rs:44` |
| sym-69d0d4189eca9444a498 | `with_max_frame_duration` | function | Decoder with an explicit maximum packet duration. | `src/codec/decoder.rs:53` |
| sym-e739e53f46a6fb3cfe72 | `pocketstation::codec` | module | Real Opus encode, decode, and packet-loss concealment primitives. | `src/codec/mod.rs:1` |
| sym-900e448d428e76f25c9a | `pocketstation::codec::decoder::OpusDecoder` | struct | Real Opus decoder wrapping libopus via the `opus` crate. | `src/codec/decoder.rs:15` |
| sym-5b5d7a123f884a58a5bd | `pocketstation::codec::encoder::OpusConfig` | struct | Explicit configuration for an Opus encoder instance. | `src/codec/encoder.rs:72` |
| sym-b9d251ce5c10d6980b9b | `pocketstation::codec::encoder::OpusEncoder` | struct | Real Opus encoder wrapping libopus via the `opus` crate. | `src/codec/encoder.rs:157` |
| sym-b62b4c170af777f0ba26 | `OpusConfig::application` | struct_field | Selects the Opus application mode used when the encoder is created. | `src/codec/encoder.rs:80` |
| sym-517b83dbabadfb9c7e13 | `OpusConfig::bitrate_kbps` | struct_field | Target bitrate in kbps. None = Opus auto (variable bitrate). | `src/codec/encoder.rs:82` |
| sym-16b114736cc7634805ac | `OpusConfig::channels` | struct_field | Selects the mono or stereo channel layout accepted by the encoder. | `src/codec/encoder.rs:76` |
| sym-c80e591dd7da39fe1ec6 | `OpusConfig::complexity` | struct_field | Encoder complexity 0–10. Higher = better quality, more CPU. | `src/codec/encoder.rs:84` |
| sym-063407a74191d8ee5726 | `OpusConfig::dtx` | struct_field | Discontinuous transmission (silence suppression). | `src/codec/encoder.rs:86` |
| sym-a834c990f442972720d1 | `OpusConfig::fec` | struct_field | In-band forward error correction. | `src/codec/encoder.rs:88` |
| sym-261d6440732347b21057 | `OpusConfig::frame_duration` | struct_field | Frame duration. Default: 20 ms (AUDIO-012). | `src/codec/encoder.rs:78` |
| sym-658cc1d8211b18ff7a02 | `OpusConfig::sample_rate` | struct_field | Sample rate. Opus only supports 48 kHz internally. | `src/codec/encoder.rs:74` |
| sym-8689ac88deced0826cdd | `OpusDecodeError::FrameDurationExceedsConfiguredMaximum::maximum_samples_per_channel` | struct_field | Records the configured maximum frame length, in samples per channel, enforced by `FrameDurationExceedsConfiguredMaximum`. | `src/codec/decoder.rs:31` |
| sym-f7da454cbdb2d87e5835 | `OpusDecodeError::FrameDurationExceedsConfiguredMaximum::requested_samples_per_channel` | struct_field | Records the requested frame length, in samples per channel, that caused `FrameDurationExceedsConfiguredMaximum`. | `src/codec/decoder.rs:30` |
| sym-280c69ee026bde4fe5d3 | `OpusEncodeError::InvalidFrameSampleCount::channels` | struct_field | Contains the channels owned or reported by `InvalidFrameSampleCount`. | `src/codec/encoder.rs:137` |
| sym-ed3451897f9409138985 | `OpusEncodeError::InvalidFrameSampleCount::expected_sample_count` | struct_field | Stores the number of expected sample represented by `InvalidFrameSampleCount`. | `src/codec/encoder.rs:138` |
| sym-58d3176756f862642135 | `OpusEncodeError::InvalidFrameSampleCount::sample_count` | struct_field | Stores the number of sample represented by `InvalidFrameSampleCount`. | `src/codec/encoder.rs:136` |
| sym-6360a1745e73e35a1ee9 | `pocketstation::codec::decoder::OpusDecodeError::FrameDurationExceedsConfiguredMaximum` | variant | Classifies a failure at the frame duration exceeds configured maximum stage or component of `OpusDecodeError`. | `src/codec/decoder.rs:29` |
| sym-d1e880d40246e0d56838 | `pocketstation::codec::decoder::OpusDecodeError::Opus` | variant | Classifies a failure at the opus stage or component of `OpusDecodeError`. | `src/codec/decoder.rs:34` |
| sym-f4f8b8b6ade7b3bcb55e | `pocketstation::codec::encoder::OpusApplication::Audio` | variant | Optimised for audio quality (music/broadcast). | `src/codec/encoder.rs:64` |
| sym-623597840615b67edfeb | `pocketstation::codec::encoder::OpusApplication::LowDelay` | variant | Optimised for low algorithmic delay. Use for real-time voice agents. | `src/codec/encoder.rs:62` |
| sym-8d3176b17cebeed6a833 | `pocketstation::codec::encoder::OpusApplication::Voip` | variant | Optimised for voice (VOIP). Default for PocketStation broadcast. | `src/codec/encoder.rs:60` |
| sym-b70ca21af71a48b9f225 | `pocketstation::codec::encoder::OpusChannels::Mono` | variant | Represents the mono alternative defined by `OpusChannels`. | `src/codec/encoder.rs:28` |
| sym-83fd05f9f594d0f8ffa2 | `pocketstation::codec::encoder::OpusChannels::Stereo` | variant | Represents the stereo alternative defined by `OpusChannels`. | `src/codec/encoder.rs:29` |
| sym-acc7fcb0b9445e901454 | `pocketstation::codec::encoder::OpusEncodeError::InvalidFrameSampleCount` | variant | Reports that the supplied frame sample count is invalid. | `src/codec/encoder.rs:135` |
| sym-4dfe2a1585087596cbf4 | `pocketstation::codec::encoder::OpusEncodeError::Opus` | variant | Classifies a failure at the opus stage or component of `OpusEncodeError`. | `src/codec/encoder.rs:141` |
| sym-d470ca3d4a4d83df93be | `pocketstation::codec::encoder::OpusFrameDuration::Ms10` | variant | Represents the ms10 alternative defined by `OpusFrameDuration`. | `src/codec/encoder.rs:8` |
| sym-0a66371187b386d36cec | `pocketstation::codec::encoder::OpusFrameDuration::Ms20` | variant | Represents the ms20 alternative defined by `OpusFrameDuration`. | `src/codec/encoder.rs:9` |
| sym-c6c7a8575e6bf02d6e24 | `pocketstation::codec::encoder::OpusFrameDuration::Ms40` | variant | Represents the ms40 alternative defined by `OpusFrameDuration`. | `src/codec/encoder.rs:10` |
| sym-e06392f2828535772ac2 | `pocketstation::codec::encoder::OpusFrameDuration::Ms60` | variant | Represents the ms60 alternative defined by `OpusFrameDuration`. | `src/codec/encoder.rs:11` |
| sym-7a6b4409e8e011fcc1d9 | `pocketstation::codec::encoder::OpusSampleRate::Hz48000` | variant | Represents the hz48000 alternative defined by `OpusSampleRate`. | `src/codec/encoder.rs:45` |
| sym-fcabf47da79c199416e3 | `pocketstation::codec::profile::StreamProfile::BroadcastStereo20ms` | variant | Represents the broadcast stereo20ms alternative defined by `StreamProfile`. | `src/codec/profile.rs:16` |
| sym-cfb5a8c73f8d95b492fa | `pocketstation::codec::profile::StreamProfile::HifiStereo20ms` | variant | Represents the hifi stereo20ms alternative defined by `StreamProfile`. | `src/codec/profile.rs:17` |
| sym-d394608f197561065186 | `pocketstation::codec::profile::StreamProfile::MusicStereo10ms` | variant | Represents the music stereo10ms alternative defined by `StreamProfile`. | `src/codec/profile.rs:15` |
| sym-98403b70ef3f8b4d9277 | `pocketstation::codec::profile::StreamProfile::MusicStereo20ms` | variant | Represents the music stereo20ms alternative defined by `StreamProfile`. | `src/codec/profile.rs:14` |
| sym-840a2a98af4eaa31725f | `pocketstation::codec::profile::StreamProfile::VoiceAgentMono10ms` | variant | Represents the voice agent mono10ms alternative defined by `StreamProfile`. | `src/codec/profile.rs:13` |
| sym-dadff3a786bf8dbc65bc | `pocketstation::codec::profile::StreamProfile::VoiceMono20ms` | variant | Represents the voice mono20ms alternative defined by `StreamProfile`. | `src/codec/profile.rs:12` |

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

The claims on **Opus codec API** are anchored to Git snapshot `136e74888962558aa846d3143a19136a70936f45` and these primary files:

- `src/codec/mod.rs:1-4` (`DECLARED`)

For **Opus codec API**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.
