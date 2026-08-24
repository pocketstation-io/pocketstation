# Frame and codec failures

<!-- claims: CLM-ERR-008-SCOPE-001,CLM-ERR-008-TEXT-001,CLM-ERR-008-TEXT-002,CLM-ERR-008-SOURCE-001 -->

## Scope

- **Preserve frame identity and lineage.** Carry source, stream, stem, route, clock, sequence, generation, and derivation identity with audio frames.
- **Encode and decode Opus.** Configure stateful Opus encoders and decoders and convert between PocketStation audio frames and packets.
- **Classify public failures.** Expose stable typed errors and cross-boundary error codes without inferring retry or recovery guarantees.

The scope of **Frame and codec failures** ends at the native contracts and executable conditions cited below. Platform qualification, performance, retry, and delivery require their own explicit evidence.

## Reference authority

For **Frame and codec failures**, the generated [docs.rs API](https://docs.rs/pocketstation/latest/pocketstation/) provides compiler-rendered signatures and navigation. This repository page adds the frozen evidence identifiers, responsibilities, and cross-boundary interpretation used by the documentation verifier.

## Error inventory

| Evidence record | Type | Variant | Trigger | Developer action | Retryable | Retry basis | Recoverable | Recovery action | Test status | Tests | Defined |
|---|---|---|---|---|---|---|---|---|---|---|---|
| error-0100bb86433fb5001cce | `pocketstation::codec::encoder::OpusEncodeError` | `Opus` | Reported when Opus encode failed: {0}. | Use the returned context to correct the reported condition; preserve the error when no recovery contract is declared. | not_declared | No retry guarantee is inferred from an error or variant name. | not_declared | Use the returned context to correct the reported condition; preserve the error when no recovery contract is declared. | linked | test-1de6d8af11e6b4a84d8d, test-6cc31685bdbf93a097b4, test-77bb4d2c9b220a4d7130, test-8c9746a0022eeccced89, test-a92cd87770b1856a8cc6, test-afeb7f4f7006561b144e, test-b498f929dd0caaa847e8, test-ce093aae4da89b29c34b, test-e9cf0ccf7086e77434e1 | `src/codec/encoder.rs:141` |
| error-2b8f0ff088781e4e95f2 | `pocketstation::frame::audio::FrameLineageError` | `SequenceNumber` | Reported when frame sequence number does not match its lineage. | Use the returned context to correct the reported condition; preserve the error when no recovery contract is declared. | not_declared | No retry guarantee is inferred from an error or variant name. | not_declared | Use the returned context to correct the reported condition; preserve the error when no recovery contract is declared. | no_direct_test_link_extracted | none linked | `src/frame/audio.rs:254` |
| error-3443075b6ff00cc1238a | `pocketstation::frame::audio::AudioFrameBuildError` | `MisalignedSamples` | Reported when audio frame sample count {samples} is not divisible by {channels} channels. | Use the returned context to correct the reported condition; preserve the error when no recovery contract is declared. | not_declared | No retry guarantee is inferred from an error or variant name. | not_declared | Use the returned context to correct the reported condition; preserve the error when no recovery contract is declared. | no_direct_test_link_extracted | none linked | `src/frame/audio.rs:57` |
| error-3b2c9b59de8f2f820444 | `pocketstation::frame::lineage::FrameLineageBuildError` | `ZeroDuration` | Reported when frame lineage duration must be non-zero. | Correct the zero duration condition reported by the returned fields before repeating the operation. | not_declared | No retry guarantee is inferred from an error or variant name. | not_declared | Correct the zero duration condition reported by the returned fields before repeating the operation. | no_direct_test_link_extracted | none linked | `src/frame/lineage.rs:95` |
| error-3b387cfa688d69e5f34c | `pocketstation::frame::audio::AudioFrameBuildError` | `ZeroChannels` | Reported when audio frame channel count must be non-zero. | Correct the zero channels condition reported by the returned fields before repeating the operation. | not_declared | No retry guarantee is inferred from an error or variant name. | not_declared | Correct the zero channels condition reported by the returned fields before repeating the operation. | no_direct_test_link_extracted | none linked | `src/frame/audio.rs:55` |
| error-3d9be3e3f583928d23f4 | `pocketstation::codec::decoder::OpusDecodeError` | type | Returned by operations whose signature names pocketstation::codec::decoder::OpusDecodeError. | Preserve `OpusDecodeError` and inspect the owning operation's typed fields before choosing recovery or presentation. | not_declared | No retry guarantee is inferred from an error or variant name. | not_declared | Preserve `OpusDecodeError` and inspect the owning operation's typed fields before choosing recovery or presentation. | linked | test-f9eb27f8c697619303a9 | `src/codec/decoder.rs:25` |
| error-3e38559ce8d474b9cbb5 | `pocketstation::frame::lineage::FrameLineageBuildError` | type | Returned by operations whose signature names pocketstation::frame::lineage::FrameLineageBuildError. | Preserve `FrameLineageBuildError` and inspect the owning operation's typed fields before choosing recovery or presentation. | not_declared | No retry guarantee is inferred from an error or variant name. | not_declared | Preserve `FrameLineageBuildError` and inspect the owning operation's typed fields before choosing recovery or presentation. | no_direct_test_link_extracted | none linked | `src/frame/lineage.rs:93` |
| error-3f103351e84a2fa4fd07 | `pocketstation::frame::pool::AudioBufferWriteError` | `CapacityExceeded` | Reported when audio buffer write of {requested_samples} samples exceeds capacity {capacity_samples}. | Use the returned context to correct the reported condition; preserve the error when no recovery contract is declared. | not_declared | No retry guarantee is inferred from an error or variant name. | not_declared | Use the returned context to correct the reported condition; preserve the error when no recovery contract is declared. | no_direct_test_link_extracted | none linked | `src/frame/pool.rs:18` |
| error-5999f78167ca350ab77a | `pocketstation::frame::audio::FrameLineageError` | `Timestamp` | Reported when frame timestamp does not match its lineage. | Use the returned context to correct the reported condition; preserve the error when no recovery contract is declared. | not_declared | No retry guarantee is inferred from an error or variant name. | not_declared | Use the returned context to correct the reported condition; preserve the error when no recovery contract is declared. | no_direct_test_link_extracted | none linked | `src/frame/audio.rs:256` |
| error-797d18adab9af15852b8 | `pocketstation::frame::lineage::FrameLineageBuildError` | `TimestampOverflow` | Reported when frame lineage timestamp range overflows u64 nanoseconds. | Use the returned context to correct the reported condition; preserve the error when no recovery contract is declared. | not_declared | No retry guarantee is inferred from an error or variant name. | not_declared | Use the returned context to correct the reported condition; preserve the error when no recovery contract is declared. | no_direct_test_link_extracted | none linked | `src/frame/lineage.rs:99` |
| error-84f459285f24936c9e00 | `pocketstation::codec::decoder::OpusDecodeError` | `FrameDurationExceedsConfiguredMaximum` | Reported when requested {requested_samples_per_channel} Opus samples per channel exceeds configured maximum {maximum_samples_per_channel}. | Use the returned context to correct the reported condition; preserve the error when no recovery contract is declared. | not_declared | No retry guarantee is inferred from an error or variant name. | not_declared | Use the returned context to correct the reported condition; preserve the error when no recovery contract is declared. | linked | test-f9eb27f8c697619303a9 | `src/codec/decoder.rs:29` |
| error-8bad0cabba73294a3557 | `pocketstation::frame::lineage::FrameLineageBuildError` | `ZeroSourceGeneration` | Reported when frame lineage source generation must be non-zero. | Correct the zero source generation condition reported by the returned fields before repeating the operation. | not_declared | No retry guarantee is inferred from an error or variant name. | not_declared | Correct the zero source generation condition reported by the returned fields before repeating the operation. | no_direct_test_link_extracted | none linked | `src/frame/lineage.rs:97` |
| error-a3cd22082905dcb2eed4 | `pocketstation::frame::audio::AudioFrameBuildError` | type | Returned by operations whose signature names pocketstation::frame::audio::AudioFrameBuildError. | Preserve `AudioFrameBuildError` and inspect the owning operation's typed fields before choosing recovery or presentation. | not_declared | No retry guarantee is inferred from an error or variant name. | not_declared | Preserve `AudioFrameBuildError` and inspect the owning operation's typed fields before choosing recovery or presentation. | no_direct_test_link_extracted | none linked | `src/frame/audio.rs:51` |
| error-b19484849f30aededbb0 | `pocketstation::frame::pool::AudioBufferWriteError` | type | Returned by operations whose signature names pocketstation::frame::pool::AudioBufferWriteError. | Preserve `AudioBufferWriteError` and inspect the owning operation's typed fields before choosing recovery or presentation. | not_declared | No retry guarantee is inferred from an error or variant name. | not_declared | Preserve `AudioBufferWriteError` and inspect the owning operation's typed fields before choosing recovery or presentation. | no_direct_test_link_extracted | none linked | `src/frame/pool.rs:14` |
| error-bef40f89fba6bbf83b1c | `pocketstation::codec::encoder::OpusEncodeError` | `InvalidFrameSampleCount` | Reported when Opus frame has {sample_count} interleaved samples; expected {expected_sample_count} for {channels} channels. | Correct the invalid frame sample count condition reported by the returned fields before repeating the operation. | not_declared | No retry guarantee is inferred from an error or variant name. | not_declared | Correct the invalid frame sample count condition reported by the returned fields before repeating the operation. | linked | test-5183ec8e78897fa604e5, test-655f8034c539e1dc9c27, test-9e747f83ae1e676c3e7e | `src/codec/encoder.rs:135` |
| error-c220db0d4b0d7f6df540 | `pocketstation::frame::audio::FrameLineageError` | `Source` | Reported when frame source id does not match its lineage. | Use the returned context to correct the reported condition; preserve the error when no recovery contract is declared. | not_declared | No retry guarantee is inferred from an error or variant name. | not_declared | Use the returned context to correct the reported condition; preserve the error when no recovery contract is declared. | linked | test-0098e9bf5859cd4840f9, test-0226f46b368cc7dec827, test-04616ce7442a986e3b43, test-081f9254eabd3bfeaad1, test-0920f0be863672d2298e, test-0db570036043ad373bee, test-125f099fa90218b83809, test-143dd58ce522cc34e2ef, test-17c702ffaf38dad01e0a, test-1de39e5016795b432d16, test-2076461784bfa8508fc9, test-25e654305e19fe3fc41a, test-284127121760cbb5874f, test-2c31d24f3d3f3a3bbc51, test-334cf92fbce87a10db8a, test-391bc055ecec6559c5ee, test-440e0d0f038bd27e531f, test-6243ccc1339da4e5bacb, test-668e2a246f514118dc91, test-699acb553a82f87d1f40, test-6b9e356534c04d2e2c3a, test-76c3eb4c4fd13e959a1c, test-7d6c18ed486400271167, test-805d755d4acd2257ba9b, test-82f8ec2b9c0fa3a0eb0b, test-8a30a347c6b7a008fb97, test-8e301580cdd23a244478, test-aec2c4ee7ff8efede00a, test-b1797fb7f4f0913afd38, test-b269c7b61711642b5e6e, test-b38e3392783a645fee8b, test-b5fe30e5dbde18fe390e, test-b731381bbb6154df587d, test-bcc1f480b1bf9ae0efa7, test-c204d11ecd759d78439f, test-c3e77a402eefae2294d5, test-cf209e0baa5c521e52ae, test-d6368a5336840b3615d8, test-d74d1c0449808ea58c4f, test-de5b0287b18425cccf03, test-e776b9a7887232e16fbc, test-e84db4efcd6a7145550a, test-e868470f819453421dd7, test-ea5b06c730a73a1dc9ca, test-f5d1e9009c62e6cb57d5, test-f98e0a98874ff7dfbdf8, test-f9ee710abe61af2dce08 | `src/frame/audio.rs:252` |
| error-d0403764652d0f905e6f | `pocketstation::frame::audio::AudioFrameBuildError` | `ZeroSampleRate` | Reported when audio frame sample rate must be non-zero. | Correct the zero sample rate condition reported by the returned fields before repeating the operation. | not_declared | No retry guarantee is inferred from an error or variant name. | not_declared | Correct the zero sample rate condition reported by the returned fields before repeating the operation. | no_direct_test_link_extracted | none linked | `src/frame/audio.rs:53` |
| error-e714868780d8cd7a5a64 | `pocketstation::codec::decoder::OpusDecodeError` | `Opus` | Reported when Opus decode failed: {0}. | Use the returned context to correct the reported condition; preserve the error when no recovery contract is declared. | not_declared | No retry guarantee is inferred from an error or variant name. | not_declared | Use the returned context to correct the reported condition; preserve the error when no recovery contract is declared. | linked | test-1de6d8af11e6b4a84d8d, test-6cc31685bdbf93a097b4, test-77bb4d2c9b220a4d7130, test-8c9746a0022eeccced89, test-a92cd87770b1856a8cc6, test-afeb7f4f7006561b144e, test-b498f929dd0caaa847e8, test-ce093aae4da89b29c34b, test-e9cf0ccf7086e77434e1 | `src/codec/decoder.rs:34` |
| error-e949cbe497712baa070b | `pocketstation::frame::audio::FrameLineageError` | type | Returned by operations whose signature names pocketstation::frame::audio::FrameLineageError. | Preserve `FrameLineageError` and inspect the owning operation's typed fields before choosing recovery or presentation. | not_declared | No retry guarantee is inferred from an error or variant name. | not_declared | Preserve `FrameLineageError` and inspect the owning operation's typed fields before choosing recovery or presentation. | linked | test-805d755d4acd2257ba9b, test-cf209e0baa5c521e52ae | `src/frame/audio.rs:250` |
| error-eda36a61ae1109dce21c | `pocketstation::codec::encoder::OpusEncodeError` | type | Returned by operations whose signature names pocketstation::codec::encoder::OpusEncodeError. | Preserve `OpusEncodeError` and inspect the owning operation's typed fields before choosing recovery or presentation. | not_declared | No retry guarantee is inferred from an error or variant name. | not_declared | Preserve `OpusEncodeError` and inspect the owning operation's typed fields before choosing recovery or presentation. | linked | test-5183ec8e78897fa604e5, test-655f8034c539e1dc9c27, test-9e747f83ae1e676c3e7e | `src/codec/encoder.rs:131` |

## Interpretation

The **Frame and codec failures** inventory records compiler-visible or extracted evidence at the frozen snapshot. A field marked unknown or not declared remains outside the published guarantee; use the native signature, owning error type, and cited test before relying on panic, blocking, cancellation, ordering, limits, retry, or recovery behavior.

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

The claims on **Frame and codec failures** are anchored to Git snapshot `136e74888962558aa846d3143a19136a70936f45` and these primary files:

- `src/frame/audio.rs:6-6` (`DIRECT`)
- `src/frame/audio.rs:10-10` (`DIRECT`)
- `src/frame/audio.rs:12-12` (`DIRECT`)
- `src/frame/audio.rs:12-12` (`DIRECT`)
- `src/frame/audio.rs:12-12` (`DIRECT`)
- `src/frame/audio.rs:13-15` (`DIRECT`)
- `src/frame/audio.rs:14-14` (`DIRECT`)
- `src/frame/audio.rs:17-17` (`DIRECT`)
- `src/frame/audio.rs:17-17` (`DIRECT`)
- `src/frame/audio.rs:17-17` (`DIRECT`)
- `src/frame/audio.rs:18-22` (`DIRECT`)
- `src/frame/audio.rs:19-19` (`DIRECT`)
- `src/frame/audio.rs:20-20` (`DIRECT`)
- `src/frame/audio.rs:21-21` (`DIRECT`)
- `src/frame/audio.rs:25-31` (`DIRECT`)
- `src/frame/audio.rs:33-35` (`DIRECT`)
- `src/frame/audio.rs:38-38` (`DIRECT`)
- `src/frame/audio.rs:39-48` (`DIRECT`)
- `src/frame/audio.rs:40-40` (`DIRECT`)
- `src/frame/audio.rs:41-41` (`DIRECT`)
- `src/frame/audio.rs:42-42` (`DIRECT`)
- `src/frame/audio.rs:43-43` (`DIRECT`)
- `src/frame/audio.rs:44-44` (`DIRECT`)
- `src/frame/audio.rs:45-45` (`DIRECT`)
- `src/codec/mod.rs:1-4` (`DECLARED`)

For **Frame and codec failures**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.
