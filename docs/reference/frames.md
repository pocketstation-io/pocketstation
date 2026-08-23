# Frame and lineage API

<!-- claims: CLM-REF-004-CAP-001,CLM-REF-004-SOURCE-001 -->

## Scope

- **Preserve frame identity and lineage.** Carry source, stream, stem, route, clock, sequence, generation, and derivation identity with audio frames.

The scope of **Frame and lineage API** ends at the native contracts and executable conditions cited below. Platform qualification, performance, retry, and delivery require their own explicit evidence.

## Reference authority

For **Frame and lineage API**, the generated [docs.rs API](https://docs.rs/pocketstation/latest/pocketstation/) provides compiler-rendered signatures and navigation. This repository page adds the frozen evidence identifiers, responsibilities, and cross-boundary interpretation used by the documentation verifier.

## Public surface

| Evidence record | Declaration | Kind | Purpose | Source |
|---|---|---|---|---|
| sym-7c9bc39a246042394c48 | `pocketstation::frame::audio::POOL_SLOT_SAMPLES` | constant | Defines the public pool slot samples value. | `src/frame/audio.rs:10` |
| sym-c747591501f5767e9363 | `pocketstation::frame::audio::SAMPLE_RATE_HZ` | constant | Defines the public sample rate hertz value. | `src/frame/audio.rs:6` |
| sym-af2221631095d6323bae | `pocketstation::frame::pool::POOL_MAX_SLOTS` | constant | Defines the public pool max slots value. | `src/frame/pool.rs:11` |
| sym-75c0d9ea95fe08480487 | `pocketstation::frame::audio::AudioFrameBuildError` | enum | Classifies failures reported as audio frame build error. | `src/frame/audio.rs:51` |
| sym-e372b47263dc149e2565 | `pocketstation::frame::audio::FrameLineageError` | enum | Classifies failures reported as frame lineage error. | `src/frame/audio.rs:250` |
| sym-8dcddebc5c2a3910991b | `pocketstation::frame::audio::SampleFormat` | enum | Selects the sample format used by PocketStation. | `src/frame/audio.rs:13` |
| sym-4343e1e50b9eca8f5850 | `pocketstation::frame::lineage::FrameLineageBuildError` | enum | Classifies failures reported as frame lineage build error. | `src/frame/lineage.rs:93` |
| sym-7e350671a794cf32f491 | `pocketstation::frame::platform::Platform` | enum | Enumerates the supported platform cases. | `src/frame/platform.rs:4` |
| sym-9e482a2d7e66b0d9c0e6 | `pocketstation::frame::pool::AudioBufferWriteError` | enum | Classifies failures reported as audio buffer write error. | `src/frame/pool.rs:14` |
| sym-c5be09000b38bb0c866a | `acquire` | function | Attempts to acquire an available buffer slot from `AudioBufferPool`. | `src/frame/pool.rs:75` |
| sym-ba21839ffaec3909eb0b | `acquire_failures` | function | Returns the acquire failures associated with `AudioBufferPool`. | `src/frame/pool.rs:68` |
| sym-4393ee74246a52c49218 | `as_mut_slice` | function | Borrows `AudioBufferHandle` as mut slice. | `src/frame/pool.rs:218` |
| sym-cb4dc9d97c99890dec4a | `as_slice` | function | Borrows `AudioBufferHandle` as slice. | `src/frame/pool.rs:214` |
| sym-5895db9d0220cfc81825 | `as_slice` | function | Borrows `SharedAudioBufferHandle` as slice. | `src/frame/pool.rs:300` |
| sym-5c520eb1b2678b3a8e30 | `available_slots` | function | Returns the available slots associated with `AudioBufferPool`. | `src/frame/pool.rs:71` |
| sym-00c85c79fcb93baa197f | `channels` | function | Returns the channel count represented by `AudioFrame`. | `src/frame/audio.rs:130` |
| sym-9b9383895d480d72636f | `channels` | function | Returns the channel count represented by `SharedAudioFrame`. | `src/frame/audio.rs:200` |
| sym-f756cb02786f8b9e4a22 | `clock_id` | function | Returns the clock identifier held by `FrameLineage`. | `src/frame/lineage.rs:65` |
| sym-448b7fda12b3625ab426 | `copy_to_pool` | function | Copies the shared frame into storage acquired from the supplied pool for `SharedAudioFrame`. | `src/frame/audio.rs:233` |
| sym-dce49826a02f0e21e6a4 | `copy_to_pool` | function | Copies the shared frame into storage acquired from the supplied pool for `SharedLineagedAudioFrame`. | `src/frame/audio.rs:319` |
| sym-0e6ca5ef323b36dc1ffb | `discontinuity_epoch` | function | Returns the discontinuity epoch associated with `FrameLineage`. | `src/frame/lineage.rs:80` |
| sym-7c022772d35f9183ccfb | `drop` | function | Releases resources owned by `AudioBufferHandle`. | `src/frame/pool.rs:265` |
| sym-4d50a59cc7313401f655 | `drop` | function | Releases resources owned by `SharedAudioBufferHandle`. | `src/frame/pool.rs:322` |
| sym-073479ab2b9fc73b1ab0 | `duration_ns` | function | Returns the duration nanoseconds held by `FrameLineage`. | `src/frame/lineage.rs:74` |
| sym-b29386002c10cabfeedf | `fmt` | function | Formats `AudioBufferHandle` with the requested formatter. | `src/frame/pool.rs:273` |
| sym-5100d1c981bdddd8fe29 | `fmt` | function | Formats `SharedAudioBufferHandle` with the requested formatter. | `src/frame/pool.rs:328` |
| sym-36cac98d0b30ecc6fe30 | `format` | function | Returns the format associated with `AudioFrame`. | `src/frame/audio.rs:134` |
| sym-7c2c9f033567cc4cb1c7 | `format` | function | Returns the format associated with `SharedAudioFrame`. | `src/frame/audio.rs:204` |
| sym-6449e35cd6d5ee588e33 | `frame` | function | Returns the frame held by `LineagedAudioFrame`. | `src/frame/audio.rs:277` |
| sym-e54682a6f63431a1a44b | `frame` | function | Returns the frame held by `SharedLineagedAudioFrame`. | `src/frame/audio.rs:304` |
| sym-e1d1201c50927893a266 | `frame_samples_for_duration_ms` | function | Returns the frame samples for duration milliseconds held by `SampleSpec`. | `src/frame/audio.rs:33` |
| sym-d6638e90985ae20d2266 | `freeze` | function | Freezes mutable storage owned by `AudioFrame` into its shared immutable form. | `src/frame/audio.rs:150` |
| sym-18121b64fb4c656cfff9 | `freeze` | function | Freezes mutable storage owned by `LineagedAudioFrame` into its shared immutable form. | `src/frame/audio.rs:289` |
| sym-87c30352006bb76291ee | `freeze` | function | Freezes mutable storage owned by `AudioBufferHandle` into its shared immutable form. | `src/frame/pool.rs:246` |
| sym-a9c7fe9b88f1bfd173a5 | `get` | function | Returns the value held by `ClockDomainId`. | `src/frame/identity.rs:36` |
| sym-55c28d8253dcbcfabb81 | `index` | function | Returns the index held by `AudioBufferHandle`. | `src/frame/pool.rs:211` |
| sym-29252c2c91d60625b91c | `index` | function | Returns the index held by `SharedAudioBufferHandle`. | `src/frame/pool.rs:296` |
| sym-cf86a3d6bcb80a95cb9d | `into_parts` | function | Consumes `LineagedAudioFrame` and returns its component values. | `src/frame/audio.rs:285` |
| sym-694b0417ceb4093d53c5 | `is_empty` | function | Returns whether `AudioBufferHandle` contains no values. | `src/frame/pool.rs:208` |
| sym-377854407927a5521f81 | `is_empty` | function | Returns whether `SharedAudioBufferHandle` contains no values. | `src/frame/pool.rs:292` |
| sym-7f13b15d2c80392c67f2 | `is_in_use` | function | Returns whether in use applies to `AudioBufferPool`. | `src/frame/pool.rs:98` |
| sym-f4dfdeb380903e4528bd | `len` | function | Returns the number of values held by `AudioBufferHandle`. | `src/frame/pool.rs:205` |
| sym-708eb84562629092eee6 | `len` | function | Returns the number of values held by `SharedAudioBufferHandle`. | `src/frame/pool.rs:288` |
| sym-6aca508b0efb6ddab602 | `lineage` | function | Returns the frame lineage carried by `LineagedAudioFrame`. | `src/frame/audio.rs:281` |
| sym-c825aea26873c6f6ee1a | `lineage` | function | Returns the frame lineage carried by `SharedLineagedAudioFrame`. | `src/frame/audio.rs:308` |
| sym-5125e04d40034efc2ebe | `new` | function | Creates a new `SampleSpec`. | `src/frame/audio.rs:25` |
| sym-f947c3edff5ed55495b3 | `new` | function | Creates a new `LineagedAudioFrame`. | `src/frame/audio.rs:272` |
| sym-7cb01f758280769116b4 | `new` | function | Creates a new `ClockDomainId`. | `src/frame/identity.rs:32` |
| sym-7e14191fea2ca3eed1a3 | `new` | function | Creates a new `AudioBufferPool`. | `src/frame/pool.rs:39` |
| sym-140f0d05874e36bc8ab1 | `permission_epoch` | function | Returns the permission epoch held by `FrameLineage`. | `src/frame/lineage.rs:83` |
| sym-7aff210034f3493f65f7 | `sample_rate_hz` | function | Returns the sample rate hertz held by `AudioFrame`. | `src/frame/audio.rs:126` |
| sym-4a8991acef0dc4873962 | `sample_rate_hz` | function | Returns the sample rate hertz held by `SharedAudioFrame`. | `src/frame/audio.rs:196` |
| sym-a28d0bab384af6c24835 | `samples` | function | Returns the audio samples held by `AudioFrame`. | `src/frame/audio.rs:146` |
| sym-0000b34e0f76d20dfe6f | `samples` | function | Returns the audio samples held by `SharedAudioFrame`. | `src/frame/audio.rs:216` |
| sym-772339cccbc94efd4611 | `sequence_number` | function | Returns the sequence number held by `AudioFrame`. | `src/frame/audio.rs:142` |
| sym-91c14c9883179bf54302 | `sequence_number` | function | Returns the sequence number held by `SharedAudioFrame`. | `src/frame/audio.rs:212` |
| sym-34372e0f01d72dc147ea | `sequence_number` | function | Returns the sequence number held by `FrameLineage`. | `src/frame/lineage.rs:68` |
| sym-4725630db086a1a9ed42 | `session_id` | function | Returns the session identifier held by `FrameLineage`. | `src/frame/lineage.rs:56` |
| sym-a699fd81177045ee4b6e | `shared_ref_count` | function | Returns the shared ref count held by `AudioBufferPool`. | `src/frame/pool.rs:102` |
| sym-cfc84a3029a025cc281d | `shared_ref_count` | function | Returns the shared ref count held by `SharedAudioBufferHandle`. | `src/frame/pool.rs:315` |
| sym-14f0a5f35858fb569bf4 | `slot_count` | function | Returns the slot count held by `AudioBufferPool`. | `src/frame/pool.rs:65` |
| sym-a44fd84794d6cff47da9 | `slot_size` | function | Returns the slot size associated with `AudioBufferPool`. | `src/frame/pool.rs:62` |
| sym-5183f904654f35cdfad7 | `source_generation` | function | Returns the source generation associated with `FrameLineage`. | `src/frame/lineage.rs:77` |
| sym-20981e860d32b9b1cf29 | `source_id` | function | Returns the source identifier held by `AudioFrame`. | `src/frame/audio.rs:122` |
| sym-27e6d5f80ecefd5f91d5 | `source_id` | function | Returns the source identifier held by `SharedAudioFrame`. | `src/frame/audio.rs:192` |
| sym-9beb92b3b06757ff589b | `source_id` | function | Returns the source identifier held by `FrameLineage`. | `src/frame/lineage.rs:59` |
| sym-09598cb65ff60c6a276e | `stem_id` | function | Returns the stem identifier held by `FrameLineage`. | `src/frame/lineage.rs:62` |
| sym-513d916fdd74b9382d04 | `stream_id` | function | Returns the stream identifier held by `AudioFrame`. | `src/frame/audio.rs:118` |
| sym-20c8ee64a5e5bb493ef4 | `stream_id` | function | Returns the stream identifier held by `SharedAudioFrame`. | `src/frame/audio.rs:188` |
| sym-b201578c15154d7494bb | `timestamp_end_ns` | function | Returns the timestamp end nanoseconds held by `FrameLineage`. | `src/frame/lineage.rs:87` |
| sym-311c8660b26ac916a68c | `timestamp_ns` | function | Returns the timestamp nanoseconds held by `AudioFrame`. | `src/frame/audio.rs:138` |
| sym-6f2251357630ef9d5aed | `timestamp_ns` | function | Returns the timestamp nanoseconds held by `SharedAudioFrame`. | `src/frame/audio.rs:208` |
| sym-134ced1b0fa7572310f8 | `timestamp_start_ns` | function | Returns the timestamp start nanoseconds held by `FrameLineage`. | `src/frame/lineage.rs:71` |
| sym-0ed55f1e90f999634106 | `try_clone` | function | Attempts to clone through `SharedAudioFrame`. | `src/frame/audio.rs:220` |
| sym-690e4b013f6b5dfe27df | `try_clone` | function | Attempts to clone through `SharedLineagedAudioFrame`. | `src/frame/audio.rs:312` |
| sym-a37013eb54b74cccac7f | `try_clone` | function | Attempts to clone through `SharedAudioBufferHandle`. | `src/frame/pool.rs:304` |
| sym-d4c335f294f449d23a76 | `try_copy_from_slice` | function | Copies samples into this fixed-capacity slot without panicking. | `src/frame/pool.rs:240` |
| sym-83f62304819c589f4f4b | `try_new` | function | Creates a new `AudioFrame` after validating its inputs. | `src/frame/audio.rs:61` |
| sym-0828b915e37452bf1cc1 | `try_new` | function | Creates a new `FrameLineage` after validating its inputs. | `src/frame/lineage.rs:21` |
| sym-e9167f9b18af1e32aad6 | `try_set_len` | function | Changes the visible sample length without panicking. | `src/frame/pool.rs:228` |
| sym-fd1a5e4bf81be429c380 | `audio` | module | Types and operations for audio. | `src/frame/audio.rs:1` |
| sym-d9e7c0a66c2f232e1613 | `lineage` | module | Compact source-aware lineage carried on realtime audio frames. | `src/frame/lineage.rs:1` |
| sym-da89361ec9773f03cfbe | `pool` | module | Fixed-capacity realtime audio storage and ownership handles. | `src/frame/pool.rs:1` |
| sym-8577d9382df72191707a | `pocketstation::frame::audio::AudioFrame` | struct | Carries one audio payload together with its declared metadata. | `src/frame/audio.rs:39` |
| sym-e57bd2ef111f9b1c749b | `pocketstation::frame::audio::LineagedAudioFrame` | struct | An exclusive audio frame and the immutable lineage snapshot captured before the frame crosses a bounded edge. | `src/frame/audio.rs:266` |
| sym-2e4f5994a3848fc24f17 | `pocketstation::frame::audio::SampleSpec` | struct | Configures sample behavior at its owning API boundary. | `src/frame/audio.rs:18` |
| sym-29f6af9dddfae2ea461b | `pocketstation::frame::audio::SharedAudioFrame` | struct | Carries one shared audio payload together with its declared metadata. | `src/frame/audio.rs:176` |
| sym-31af59b4777ad01c6d45 | `pocketstation::frame::audio::SharedLineagedAudioFrame` | struct | Carries one shared lineaged audio payload together with its declared metadata. | `src/frame/audio.rs:298` |
| sym-644caca167920c58fe6a | `pocketstation::frame::identity::ClockDomainId` | struct | Uniquely identifies clock domain within its PocketStation ownership scope. | `src/frame/identity.rs:29` |
| sym-68f6f5cb9b238976cdc6 | `pocketstation::frame::identity::ConnectorId` | struct | Uniquely identifies connector within its PocketStation ownership scope. | `src/frame/identity.rs:25` |
| sym-651bdd72775fb5995723 | `pocketstation::frame::identity::EndpointId` | struct | Uniquely identifies endpoint within its PocketStation ownership scope. | `src/frame/identity.rs:24` |
| sym-e70f709a3e484b9d5d5c | `pocketstation::frame::identity::RouteId` | struct | Uniquely identifies route within its PocketStation ownership scope. | `src/frame/identity.rs:26` |
| sym-906bbaa23baea185d5aa | `pocketstation::frame::identity::SessionId` | struct | Uniquely identifies session within its PocketStation ownership scope. | `src/frame/identity.rs:22` |
| sym-009b1169b888de1dce8b | `pocketstation::frame::identity::SourceId` | struct | Uniquely identifies source within its PocketStation ownership scope. | `src/frame/identity.rs:21` |
| sym-82a2b9584ea1d7a2dae6 | `pocketstation::frame::identity::StemId` | struct | Uniquely identifies stem within its PocketStation ownership scope. | `src/frame/identity.rs:23` |
| sym-0010fb7e6f1802e2c0c1 | `pocketstation::frame::identity::StreamId` | struct | Uniquely identifies stream within its PocketStation ownership scope. | `src/frame/identity.rs:20` |
| sym-4f5d7dda4dde3e3ceaf9 | `pocketstation::frame::lineage::FrameLineage` | struct | Preserves source, stream, sequence, clock, generation, and discontinuity identity for an audio frame. | `src/frame/lineage.rs:6` |
| sym-de68a53d211a83bfc33c | `pocketstation::frame::pool::AudioBufferHandle` | struct | Owns bounded access to audio buffer. | `src/frame/pool.rs:198` |
| sym-73a3a95b920434157c7b | `pocketstation::frame::pool::AudioBufferPool` | struct | Owns fixed-capacity reusable audio slots and reports acquisition pressure without allocating per frame. | `src/frame/pool.rs:24` |
| sym-d1d3a10ca5f3d96627df | `pocketstation::frame::pool::SharedAudioBufferHandle` | struct | Owns bounded access to shared audio buffer. | `src/frame/pool.rs:281` |
| sym-46bd88499244eadd7644 | `audio::AudioFrameBuildError::MisalignedSamples::channels` | struct_field | Stores the channels used by `MisalignedSamples`. | `src/frame/audio.rs:57` |
| sym-c152aa4eaf517b7e0f38 | `audio::AudioFrameBuildError::MisalignedSamples::samples` | struct_field | Stores the samples used by `MisalignedSamples`. | `src/frame/audio.rs:57` |
| sym-d03c8216ec1500493017 | `audio::SampleSpec::channels` | struct_field | Stores the channels used by `SampleSpec`. | `src/frame/audio.rs:20` |
| sym-63a8cb9ef418fdd86d75 | `audio::SampleSpec::format` | struct_field | Stores the format used by `SampleSpec`. | `src/frame/audio.rs:21` |
| sym-aa5c5c984c98f972616a | `audio::SampleSpec::sample_rate_hz` | struct_field | Stores the sample rate value for `SampleSpec`, in hertz. | `src/frame/audio.rs:19` |
| sym-d4623f39a117cbed7815 | `pool::AudioBufferWriteError::CapacityExceeded::capacity_samples` | struct_field | Sets the capacity samples available to `CapacityExceeded`. | `src/frame/pool.rs:20` |
| sym-7fb9e3e159af61c62ea6 | `pool::AudioBufferWriteError::CapacityExceeded::requested_samples` | struct_field | Stores the requested samples used by `CapacityExceeded`. | `src/frame/pool.rs:19` |
| sym-19eaba59e9cfed66e693 | `pocketstation::frame::audio::AudioFrameBuildError::MisalignedSamples` | variant | Reported when the owning operation encounters misaligned samples. | `src/frame/audio.rs:57` |
| sym-eaf6e41fb330ca815075 | `pocketstation::frame::audio::AudioFrameBuildError::ZeroChannels` | variant | Reported when the owning operation encounters zero channels. | `src/frame/audio.rs:55` |
| sym-c684902a1f1c52d3b842 | `pocketstation::frame::audio::AudioFrameBuildError::ZeroSampleRate` | variant | Reported when the owning operation encounters zero sample rate. | `src/frame/audio.rs:53` |
| sym-2c22a35c423b0c4ec202 | `pocketstation::frame::audio::FrameLineageError::SequenceNumber` | variant | Reported when the owning operation encounters sequence number. | `src/frame/audio.rs:254` |
| sym-d5d0ca3146b94c396fdb | `pocketstation::frame::audio::FrameLineageError::Source` | variant | Reported when the owning operation encounters source. | `src/frame/audio.rs:252` |
| sym-85195a3401c0fe61e495 | `pocketstation::frame::audio::FrameLineageError::Timestamp` | variant | Reported when the owning operation encounters timestamp. | `src/frame/audio.rs:256` |
| sym-3a769b03af01431e0e85 | `pocketstation::frame::audio::SampleFormat::F32Interleaved` | variant | Selects f32 interleaved behavior for `SampleFormat`. | `src/frame/audio.rs:14` |
| sym-1189c02b7250275259f9 | `pocketstation::frame::lineage::FrameLineageBuildError::TimestampOverflow` | variant | Reported when the owning operation encounters timestamp overflow. | `src/frame/lineage.rs:99` |
| sym-8565c6b0e0b425dc0c01 | `pocketstation::frame::lineage::FrameLineageBuildError::ZeroDuration` | variant | Reported when the owning operation encounters zero duration. | `src/frame/lineage.rs:95` |
| sym-a1e996dbffa813d8ec0f | `pocketstation::frame::lineage::FrameLineageBuildError::ZeroSourceGeneration` | variant | Reported when the owning operation encounters zero source generation. | `src/frame/lineage.rs:97` |
| sym-a5faed540ad6de134b1c | `pocketstation::frame::platform::Platform::Android` | variant | Represents the android alternative defined by `Platform`. | `src/frame/platform.rs:9` |
| sym-d7895c7b24722dbb63c2 | `pocketstation::frame::platform::Platform::Ios` | variant | Represents the ios alternative defined by `Platform`. | `src/frame/platform.rs:8` |
| sym-9652799b7d15ebc70b3a | `pocketstation::frame::platform::Platform::Linux` | variant | Represents the linux alternative defined by `Platform`. | `src/frame/platform.rs:7` |
| sym-925c9311fede4c71c8fc | `pocketstation::frame::platform::Platform::Macos` | variant | Represents the macos alternative defined by `Platform`. | `src/frame/platform.rs:5` |
| sym-05ee566e953b53c50178 | `pocketstation::frame::platform::Platform::Unknown` | variant | Represents the unknown alternative defined by `Platform`. | `src/frame/platform.rs:11` |
| sym-2e39b3ebc8ab3c6e06bd | `pocketstation::frame::platform::Platform::Web` | variant | Represents the web alternative defined by `Platform`. | `src/frame/platform.rs:10` |
| sym-31d821d576822bb09505 | `pocketstation::frame::platform::Platform::Windows` | variant | Represents the windows alternative defined by `Platform`. | `src/frame/platform.rs:6` |
| sym-29e6dc01c36a90fb7576 | `pocketstation::frame::pool::AudioBufferWriteError::CapacityExceeded` | variant | Reported when the owning operation encounters capacity exceeded. | `src/frame/pool.rs:18` |

## Interpretation

The **Frame and lineage API** inventory records compiler-visible or extracted evidence at the frozen snapshot. A field marked unknown or not declared remains outside the published guarantee; use the native signature, owning error type, and cited test before relying on panic, blocking, cancellation, ordering, limits, retry, or recovery behavior.

## Related documentation

- [Frame identity and lineage](/docs/concepts/frame-lineage.md)
- [Glossary](/docs/glossary.md)
- [Memory ownership and buffer pools](/docs/internals/memory-ownership.md)
- [PocketStation](/README.md)
- [Realtime audio lane](/docs/internals/realtime-audio-lane.md)
- [Capture application and microphone stems](/docs/how-to/capture-app-and-mic.md)
- [Encode and decode Opus](/docs/how-to/opus-roundtrip.md)
- [Map source time into the Session timeline](/docs/how-to/map-source-time.md)

## Evidence boundary

The claims on **Frame and lineage API** are anchored to Git snapshot `136e74888962558aa846d3143a19136a70936f45` and these primary files:

- `src/frame/mod.rs:1-18` (`DIRECT`)

For **Frame and lineage API**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.
