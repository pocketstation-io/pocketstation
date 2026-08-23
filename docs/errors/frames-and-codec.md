# Frame and codec failures

<!-- claims: CLM-ERR-008-CAP-001,CLM-ERR-008-CAP-002,CLM-ERR-008-CAP-003,CLM-ERR-008-SOURCE-001,CLM-ERR-008-ERROR-0001,CLM-ERR-008-ERROR-0002,CLM-ERR-008-ERROR-0003,CLM-ERR-008-ERROR-0004,CLM-ERR-008-ERROR-0005,CLM-ERR-008-ERROR-0006,CLM-ERR-008-ERROR-0007,CLM-ERR-008-ERROR-0008,CLM-ERR-008-ERROR-0009,CLM-ERR-008-ERROR-0010,CLM-ERR-008-ERROR-0011,CLM-ERR-008-ERROR-0012,CLM-ERR-008-ERROR-0013,CLM-ERR-008-ERROR-0014,CLM-ERR-008-ERROR-0015,CLM-ERR-008-ERROR-0016 -->

## Scope

- **Preserve frame identity and lineage.** Carry source, stream, stem, route, clock, sequence, generation, and derivation identity with audio frames.
- **Encode and decode Opus.** Configure stateful Opus encoders and decoders and convert between PocketStation audio frames and packets.
- **Classify public failures.** Expose stable typed errors and cross-boundary error codes without inferring retry or recovery guarantees.

These statements describe repository contracts at the documented snapshot. They do not extend platform qualification, performance, retry, or delivery guarantees beyond the native API contracts and executable evidence.

## Reference authority

The generated [docs.rs API](https://docs.rs/pocketstation/latest/pocketstation/) is the exhaustive symbol-level Rust reference. Human reference pages organize responsibilities and cross-boundary behavior; they do not duplicate every rustdoc signature.

## Error inventory

| Evidence record | Type | Variant | Retryable | Recoverable | Defined |
|---|---|---|---|---|---|
| error-2317926ecc3df1fe0485 | `pocketstation::frame::pool::AudioBufferWriteError` | `CapacityExceeded` | unknown | unknown | `src/frame/pool.rs:18` |
| error-2333fb8ed9ffc64dfe3d | `pocketstation::frame::lineage::FrameLineageBuildError` | `ZeroSourceGeneration` | unknown | unknown | `src/frame/lineage.rs:97` |
| error-36112cc71bb577df5cc6 | `pocketstation::frame::lineage::FrameLineageBuildError` | `ZeroDuration` | unknown | unknown | `src/frame/lineage.rs:95` |
| error-3d530ffcc82f2ae60152 | `pocketstation::frame::audio::AudioFrameBuildError` | `ZeroSampleRate` | unknown | unknown | `src/frame/audio.rs:53` |
| error-44d619f15116bb8d5f0e | `pocketstation::frame::pool::AudioBufferWriteError` | type | unknown | unknown | `src/frame/pool.rs:14` |
| error-47bd33a1cf3d0c5fa264 | `pocketstation::frame::audio::AudioFrameBuildError` | type | unknown | unknown | `src/frame/audio.rs:51` |
| error-7f9c7f9db13f5030ecb1 | `pocketstation::codec::encoder::OpusEncodeError` | `Opus` | unknown | unknown | `src/codec/encoder.rs:141` |
| error-886f021bf510039ccdbb | `pocketstation::frame::lineage::FrameLineageBuildError` | type | unknown | unknown | `src/frame/lineage.rs:93` |
| error-a9fc3232ddadf6734ba1 | `pocketstation::codec::encoder::OpusEncodeError` | `InvalidFrameSampleCount` | unknown | unknown | `src/codec/encoder.rs:135` |
| error-ab24633d76ea98a177e1 | `pocketstation::codec::encoder::OpusEncodeError` | type | unknown | unknown | `src/codec/encoder.rs:131` |
| error-bd82320c958728697aec | `pocketstation::codec::decoder::OpusDecodeError` | `FrameDurationExceedsConfiguredMaximum` | unknown | unknown | `src/codec/decoder.rs:29` |
| error-bd9d2580f5c500ca2920 | `pocketstation::frame::lineage::FrameLineageBuildError` | `TimestampOverflow` | unknown | unknown | `src/frame/lineage.rs:99` |
| error-d087ea5f521ea8ed0cf1 | `pocketstation::codec::decoder::OpusDecodeError` | type | unknown | unknown | `src/codec/decoder.rs:25` |
| error-ea56c43f0b56c9b86350 | `pocketstation::codec::decoder::OpusDecodeError` | `Opus` | unknown | unknown | `src/codec/decoder.rs:34` |
| error-ec0790bb6edfcc3d5058 | `pocketstation::frame::audio::AudioFrameBuildError` | `ZeroChannels` | unknown | unknown | `src/frame/audio.rs:55` |
| error-fd6606b3c0707d21bb0f | `pocketstation::frame::audio::AudioFrameBuildError` | `MisalignedSamples` | unknown | unknown | `src/frame/audio.rs:57` |

## Interpretation

An inventory row establishes that a declaration, test, lifecycle operation, configuration surface, or protocol element exists at the frozen snapshot. Fields shown as unknown remain deliberately unspecified. Consult the native API and error contract before relying on panic behavior, blocking, cancellation, ordering, limits, retry, or recovery.

## Related documentation

- [Glossary](/docs/glossary.md)
- [PocketStation](/README.md)
- [Behavior evidence index](/docs/reference/behavior-evidence.md)
- [Rust API reference](/docs/reference/rust-api.md)
- [Opus conversion fails](/docs/troubleshooting/opus.md)
- [Encode and decode Opus](/docs/how-to/opus-roundtrip.md)
- [A capture source disappears](/docs/troubleshooting/source-loss.md)
- [Error and status model](/docs/concepts/error-model.md)

## Evidence boundary

This page was verified against Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `src/frame/audio.rs:1-636` (`DIRECT`)
- `src/codec/mod.rs:1-20` (`DIRECT`)

A file's presence proves implementation or declaration at this snapshot. It does not by itself prove physical-device qualification, operational performance, retry safety, or behavior outside the recorded test conditions.
