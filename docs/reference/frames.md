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
| sym-331441db632dcf64c0b7 | `pocketstation::frame::audio::POOL_SLOT_SAMPLES` | constant | Defines the public pool slot samples value. | `src/frame/audio.rs:10` |
| sym-14e035ed0b0d39c2bb72 | `pocketstation::frame::audio::SAMPLE_RATE_HZ` | constant | Defines the public sample rate hertz value. | `src/frame/audio.rs:6` |
| sym-d76758b7388e90ce34c5 | `pocketstation::frame::pool::POOL_MAX_SLOTS` | constant | Defines the public pool max slots value. | `src/frame/pool.rs:11` |
| sym-04d73a434600a5d3f46b | `pocketstation::frame::audio::AudioFrameBuildError` | enum | Classifies failures reported as audio frame build error. | `src/frame/audio.rs:51` |
| sym-2ef18de7c50ff69f9668 | `pocketstation::frame::audio::FrameLineageError` | enum | Classifies failures reported as frame lineage error. | `src/frame/audio.rs:250` |
| sym-4126c27a577e7dc06b43 | `pocketstation::frame::audio::SampleFormat` | enum | Selects the sample format used by PocketStation. | `src/frame/audio.rs:13` |
| sym-1ec57531bcf0b3720cf6 | `pocketstation::frame::lineage::FrameLineageBuildError` | enum | Classifies failures reported as frame lineage build error. | `src/frame/lineage.rs:93` |
| sym-f9317dbcae260dd5577a | `pocketstation::frame::platform::Platform` | enum | Enumerates the supported platform cases. | `src/frame/platform.rs:4` |
| sym-7ee17b056959ed78ed91 | `pocketstation::frame::pool::AudioBufferWriteError` | enum | Classifies failures reported as audio buffer write error. | `src/frame/pool.rs:14` |
| sym-17b81c499a05173f1deb | `acquire` | function | Attempts to acquire an available buffer slot from `AudioBufferPool`. | `src/frame/pool.rs:75` |
| sym-c1e159b4daa087bdfc39 | `acquire_failures` | function | Returns the acquire failures associated with `AudioBufferPool`. | `src/frame/pool.rs:68` |
| sym-f9871f0dbb9b665b5715 | `as_mut_slice` | function | Borrows `AudioBufferHandle` as mut slice. | `src/frame/pool.rs:218` |
| sym-4a7e2a80ef427e839b4d | `as_slice` | function | Borrows `AudioBufferHandle` as slice. | `src/frame/pool.rs:214` |
| sym-0cf65d4a441597a1309e | `as_slice` | function | Borrows `SharedAudioBufferHandle` as slice. | `src/frame/pool.rs:300` |
| sym-1735fdd774ca425845f1 | `available_slots` | function | Returns the available slots associated with `AudioBufferPool`. | `src/frame/pool.rs:71` |
| sym-74fbbf79422965ab388b | `channels` | function | Returns the channel count represented by `AudioFrame`. | `src/frame/audio.rs:130` |
| sym-bdb64c3d02ab60c407e2 | `channels` | function | Returns the channel count represented by `SharedAudioFrame`. | `src/frame/audio.rs:200` |
| sym-40e0d80cb91a22bab5a3 | `clock_id` | function | Returns the clock identifier held by `FrameLineage`. | `src/frame/lineage.rs:65` |
| sym-ec92f1b89383f257d00b | `copy_to_pool` | function | Copies the shared frame into storage acquired from the supplied pool for `SharedAudioFrame`. | `src/frame/audio.rs:233` |
| sym-7417542e02f081e0c7b8 | `copy_to_pool` | function | Copies the shared frame into storage acquired from the supplied pool for `SharedLineagedAudioFrame`. | `src/frame/audio.rs:319` |
| sym-65059d3406a37d9860f0 | `discontinuity_epoch` | function | Returns the discontinuity epoch associated with `FrameLineage`. | `src/frame/lineage.rs:80` |
| sym-88cc3ed40c8d28013862 | `drop` | function | Releases resources owned by `AudioBufferHandle`. | `src/frame/pool.rs:265` |
| sym-ec8f2db8bb6488a7e187 | `drop` | function | Releases resources owned by `SharedAudioBufferHandle`. | `src/frame/pool.rs:322` |
| sym-4295ba8996165d5da0d8 | `duration_ns` | function | Returns the duration nanoseconds held by `FrameLineage`. | `src/frame/lineage.rs:74` |
| sym-244cebda9945d000bb40 | `fmt` | function | Formats `AudioBufferHandle` with the requested formatter. | `src/frame/pool.rs:273` |
| sym-3ccd0b6574bc3bbb1327 | `fmt` | function | Formats `SharedAudioBufferHandle` with the requested formatter. | `src/frame/pool.rs:328` |
| sym-0e92e7ecb8481aedc211 | `format` | function | Returns the format associated with `AudioFrame`. | `src/frame/audio.rs:134` |
| sym-2429c171fd87812f1d03 | `format` | function | Returns the format associated with `SharedAudioFrame`. | `src/frame/audio.rs:204` |
| sym-f56967a038cd84555880 | `frame` | function | Returns the frame held by `LineagedAudioFrame`. | `src/frame/audio.rs:277` |
| sym-e0b65c424a06edc7d26f | `frame` | function | Returns the frame held by `SharedLineagedAudioFrame`. | `src/frame/audio.rs:304` |
| sym-80fa7314bf0e0f7e13d6 | `frame_samples_for_duration_ms` | function | Returns the frame samples for duration milliseconds held by `SampleSpec`. | `src/frame/audio.rs:33` |
| sym-bdb1ea7aca5c1c2514a4 | `freeze` | function | Freezes mutable storage owned by `AudioFrame` into its shared immutable form. | `src/frame/audio.rs:150` |
| sym-a13662ae36441b39d89b | `freeze` | function | Freezes mutable storage owned by `LineagedAudioFrame` into its shared immutable form. | `src/frame/audio.rs:289` |
| sym-1b1890c3445f6f82f996 | `freeze` | function | Freezes mutable storage owned by `AudioBufferHandle` into its shared immutable form. | `src/frame/pool.rs:246` |
| sym-6ecc9053f5721cdadb3c | `get` | function | Returns the value held by `ClockDomainId`. | `src/frame/identity.rs:36` |
| sym-bedba89a88c30341a367 | `index` | function | Returns the index held by `AudioBufferHandle`. | `src/frame/pool.rs:211` |
| sym-5a6667fe9280b73e0c0f | `index` | function | Returns the index held by `SharedAudioBufferHandle`. | `src/frame/pool.rs:296` |
| sym-10764124105b472e68ff | `into_parts` | function | Consumes `LineagedAudioFrame` and returns its component values. | `src/frame/audio.rs:285` |
| sym-aca5edc5ef1101ccaa3d | `is_empty` | function | Returns whether `AudioBufferHandle` contains no values. | `src/frame/pool.rs:208` |
| sym-8e8655689ed1798e3144 | `is_empty` | function | Returns whether `SharedAudioBufferHandle` contains no values. | `src/frame/pool.rs:292` |
| sym-4ff0ee20cadb8ece9974 | `is_in_use` | function | Returns whether in use applies to `AudioBufferPool`. | `src/frame/pool.rs:98` |
| sym-4c98f24a698c9d467f1f | `len` | function | Returns the number of values held by `AudioBufferHandle`. | `src/frame/pool.rs:205` |
| sym-4b32d1471be01559d2be | `len` | function | Returns the number of values held by `SharedAudioBufferHandle`. | `src/frame/pool.rs:288` |
| sym-6f85ec7dff6aef7ebe86 | `lineage` | function | Returns the frame lineage carried by `LineagedAudioFrame`. | `src/frame/audio.rs:281` |
| sym-f74c3ed1333764b92d58 | `lineage` | function | Returns the frame lineage carried by `SharedLineagedAudioFrame`. | `src/frame/audio.rs:308` |
| sym-dd69a151b1b2a2989687 | `new` | function | Creates a new `SampleSpec`. | `src/frame/audio.rs:25` |
| sym-77b3385d98c4250014df | `new` | function | Creates a new `LineagedAudioFrame`. | `src/frame/audio.rs:272` |
| sym-9602da8c0f25fa84fac5 | `new` | function | Creates a new `ClockDomainId`. | `src/frame/identity.rs:32` |
| sym-44f3c327815dfe7bb5e6 | `new` | function | Creates a new `AudioBufferPool`. | `src/frame/pool.rs:39` |
| sym-7a99f9e74f8e8ab86c8f | `permission_epoch` | function | Returns the permission epoch held by `FrameLineage`. | `src/frame/lineage.rs:83` |
| sym-7bb8bdf536ae8c9f679d | `sample_rate_hz` | function | Returns the sample rate hertz held by `AudioFrame`. | `src/frame/audio.rs:126` |
| sym-780919ce9519b32a0016 | `sample_rate_hz` | function | Returns the sample rate hertz held by `SharedAudioFrame`. | `src/frame/audio.rs:196` |
| sym-d7128c49608f942689a5 | `samples` | function | Returns the audio samples held by `AudioFrame`. | `src/frame/audio.rs:146` |
| sym-55a9f79ef93ef938179a | `samples` | function | Returns the audio samples held by `SharedAudioFrame`. | `src/frame/audio.rs:216` |
| sym-5c04463df13b6837b4e5 | `sequence_number` | function | Returns the sequence number held by `AudioFrame`. | `src/frame/audio.rs:142` |
| sym-a7e53c983feee0f91554 | `sequence_number` | function | Returns the sequence number held by `SharedAudioFrame`. | `src/frame/audio.rs:212` |
| sym-d9a573e9517b85c8a8f3 | `sequence_number` | function | Returns the sequence number held by `FrameLineage`. | `src/frame/lineage.rs:68` |
| sym-90a7dbdce4ff0ac99abe | `session_id` | function | Returns the session identifier held by `FrameLineage`. | `src/frame/lineage.rs:56` |
| sym-3fe113f7e869d83b241a | `shared_ref_count` | function | Returns the shared ref count held by `AudioBufferPool`. | `src/frame/pool.rs:102` |
| sym-d7e58eb464e00e23004c | `shared_ref_count` | function | Returns the shared ref count held by `SharedAudioBufferHandle`. | `src/frame/pool.rs:315` |
| sym-0f15aeec2635e7c05d0c | `slot_count` | function | Returns the slot count held by `AudioBufferPool`. | `src/frame/pool.rs:65` |
| sym-384658cb5081274530d3 | `slot_size` | function | Returns the slot size associated with `AudioBufferPool`. | `src/frame/pool.rs:62` |
| sym-a7abe406c11df0c70626 | `source_generation` | function | Returns the source generation associated with `FrameLineage`. | `src/frame/lineage.rs:77` |
| sym-413007befbdea96468e9 | `source_id` | function | Returns the source identifier held by `AudioFrame`. | `src/frame/audio.rs:122` |
| sym-109944a5099c0b998d76 | `source_id` | function | Returns the source identifier held by `SharedAudioFrame`. | `src/frame/audio.rs:192` |
| sym-90d41b0a81b73aa89b65 | `source_id` | function | Returns the source identifier held by `FrameLineage`. | `src/frame/lineage.rs:59` |
| sym-7da7bec585f5e361fef0 | `stem_id` | function | Returns the stem identifier held by `FrameLineage`. | `src/frame/lineage.rs:62` |
| sym-1a7c40cfd0e0592759b0 | `stream_id` | function | Returns the stream identifier held by `AudioFrame`. | `src/frame/audio.rs:118` |
| sym-95f96b2f2639f19c6149 | `stream_id` | function | Returns the stream identifier held by `SharedAudioFrame`. | `src/frame/audio.rs:188` |
| sym-b6b4a844db15ffe22dd2 | `timestamp_end_ns` | function | Returns the timestamp end nanoseconds held by `FrameLineage`. | `src/frame/lineage.rs:87` |
| sym-a29004a6bd0080e13a04 | `timestamp_ns` | function | Returns the timestamp nanoseconds held by `AudioFrame`. | `src/frame/audio.rs:138` |
| sym-26218b0d80b65da873c1 | `timestamp_ns` | function | Returns the timestamp nanoseconds held by `SharedAudioFrame`. | `src/frame/audio.rs:208` |
| sym-64a5722d27d4c081535f | `timestamp_start_ns` | function | Returns the timestamp start nanoseconds held by `FrameLineage`. | `src/frame/lineage.rs:71` |
| sym-ef7332c6f9f647fe5aec | `try_clone` | function | Attempts to clone through `SharedAudioFrame`. | `src/frame/audio.rs:220` |
| sym-a47007a0acae1169e418 | `try_clone` | function | Attempts to clone through `SharedLineagedAudioFrame`. | `src/frame/audio.rs:312` |
| sym-bf3505e13225bc33563e | `try_clone` | function | Attempts to clone through `SharedAudioBufferHandle`. | `src/frame/pool.rs:304` |
| sym-c87a1b1d86e245f5d21b | `try_copy_from_slice` | function | Copies samples into this fixed-capacity slot without panicking. | `src/frame/pool.rs:240` |
| sym-a3ed90fa20535487bd72 | `try_new` | function | Creates a new `AudioFrame` after validating its inputs. | `src/frame/audio.rs:61` |
| sym-103b407fd5cd4c1ca485 | `try_new` | function | Creates a new `FrameLineage` after validating its inputs. | `src/frame/lineage.rs:21` |
| sym-20dbac7745fd63cf7ef5 | `try_set_len` | function | Changes the visible sample length without panicking. | `src/frame/pool.rs:228` |
| sym-3e387f2a2fdf47f09b7d | `audio` | module | Types and operations for audio. | `src/frame/audio.rs:1` |
| sym-9fa6d8a8cc9db89e2fb3 | `lineage` | module | Compact source-aware lineage carried on realtime audio frames. | `src/frame/lineage.rs:1` |
| sym-30a0f1ed866f7e53eb12 | `pool` | module | Fixed-capacity realtime audio storage and ownership handles. | `src/frame/pool.rs:1` |
| sym-fc1f92d68a20921380f3 | `pocketstation::frame::audio::AudioFrame` | struct | Carries one audio payload together with its declared metadata. | `src/frame/audio.rs:39` |
| sym-e84366f3ca396d712e65 | `pocketstation::frame::audio::LineagedAudioFrame` | struct | An exclusive audio frame and the immutable lineage snapshot captured before the frame crosses a bounded edge. | `src/frame/audio.rs:266` |
| sym-1745667d6843cfaaf2e6 | `pocketstation::frame::audio::SampleSpec` | struct | Configures sample behavior at its owning API boundary. | `src/frame/audio.rs:18` |
| sym-55e4e4b2bf2b792d424b | `pocketstation::frame::audio::SharedAudioFrame` | struct | Carries one shared audio payload together with its declared metadata. | `src/frame/audio.rs:176` |
| sym-ec68dca3da1e27df59ad | `pocketstation::frame::audio::SharedLineagedAudioFrame` | struct | Carries one shared lineaged audio payload together with its declared metadata. | `src/frame/audio.rs:298` |
| sym-cf299e85b856722d5c82 | `pocketstation::frame::identity::ClockDomainId` | struct | Uniquely identifies clock domain within its PocketStation ownership scope. | `src/frame/identity.rs:29` |
| sym-54e80634d3c8f1d68c51 | `pocketstation::frame::identity::ConnectorId` | struct | Uniquely identifies connector within its PocketStation ownership scope. | `src/frame/identity.rs:25` |
| sym-329abdbf68985263c68e | `pocketstation::frame::identity::EndpointId` | struct | Uniquely identifies endpoint within its PocketStation ownership scope. | `src/frame/identity.rs:24` |
| sym-081d3a4abc3e6c70e88d | `pocketstation::frame::identity::RouteId` | struct | Uniquely identifies route within its PocketStation ownership scope. | `src/frame/identity.rs:26` |
| sym-e0967f4574129b9a2554 | `pocketstation::frame::identity::SessionId` | struct | Uniquely identifies session within its PocketStation ownership scope. | `src/frame/identity.rs:22` |
| sym-d50d47dff9e2a7898f88 | `pocketstation::frame::identity::SourceId` | struct | Uniquely identifies source within its PocketStation ownership scope. | `src/frame/identity.rs:21` |
| sym-383b7bf043152137963b | `pocketstation::frame::identity::StemId` | struct | Uniquely identifies stem within its PocketStation ownership scope. | `src/frame/identity.rs:23` |
| sym-bc74b3d384bea7406755 | `pocketstation::frame::identity::StreamId` | struct | Uniquely identifies stream within its PocketStation ownership scope. | `src/frame/identity.rs:20` |
| sym-c34282f9a743c50c5b56 | `pocketstation::frame::lineage::FrameLineage` | struct | Preserves source, stream, sequence, clock, generation, and discontinuity identity for an audio frame. | `src/frame/lineage.rs:6` |
| sym-de7b5a299f41a9ec535b | `pocketstation::frame::pool::AudioBufferHandle` | struct | Owns bounded access to audio buffer. | `src/frame/pool.rs:198` |
| sym-3537e580cc2bbebe2007 | `pocketstation::frame::pool::AudioBufferPool` | struct | Owns fixed-capacity reusable audio slots and reports acquisition pressure without allocating per frame. | `src/frame/pool.rs:24` |
| sym-d167211f444b21d144e8 | `pocketstation::frame::pool::SharedAudioBufferHandle` | struct | Owns bounded access to shared audio buffer. | `src/frame/pool.rs:281` |
| sym-63d4caa3f8249ce4806e | `audio::AudioFrameBuildError::MisalignedSamples::channels` | struct_field | Stores the channels used by `MisalignedSamples`. | `src/frame/audio.rs:57` |
| sym-020cbffec15678965496 | `audio::AudioFrameBuildError::MisalignedSamples::samples` | struct_field | Stores the samples used by `MisalignedSamples`. | `src/frame/audio.rs:57` |
| sym-4f16ce993cc48329e3f7 | `audio::SampleSpec::channels` | struct_field | Stores the channels used by `SampleSpec`. | `src/frame/audio.rs:20` |
| sym-52d5039021f6cd50053c | `audio::SampleSpec::format` | struct_field | Stores the format used by `SampleSpec`. | `src/frame/audio.rs:21` |
| sym-059bce41080500fbc90a | `audio::SampleSpec::sample_rate_hz` | struct_field | Stores the sample rate value for `SampleSpec`, in hertz. | `src/frame/audio.rs:19` |
| sym-6f802083c5c1d7266efd | `pool::AudioBufferWriteError::CapacityExceeded::capacity_samples` | struct_field | Sets the capacity samples available to `CapacityExceeded`. | `src/frame/pool.rs:20` |
| sym-15ef9e6300ed6340bc47 | `pool::AudioBufferWriteError::CapacityExceeded::requested_samples` | struct_field | Stores the requested samples used by `CapacityExceeded`. | `src/frame/pool.rs:19` |
| sym-bd67e35bf3477713d4ed | `pocketstation::frame::audio::AudioFrameBuildError::MisalignedSamples` | variant | Reported when the owning operation encounters misaligned samples. | `src/frame/audio.rs:57` |
| sym-2f30cf24c8cc92e83cc7 | `pocketstation::frame::audio::AudioFrameBuildError::ZeroChannels` | variant | Reported when the owning operation encounters zero channels. | `src/frame/audio.rs:55` |
| sym-b059c49197ff40cdabb6 | `pocketstation::frame::audio::AudioFrameBuildError::ZeroSampleRate` | variant | Reported when the owning operation encounters zero sample rate. | `src/frame/audio.rs:53` |
| sym-05582883e7c54d857255 | `pocketstation::frame::audio::FrameLineageError::SequenceNumber` | variant | Reported when the owning operation encounters sequence number. | `src/frame/audio.rs:254` |
| sym-4f15d6eda96842b8d846 | `pocketstation::frame::audio::FrameLineageError::Source` | variant | Reported when the owning operation encounters source. | `src/frame/audio.rs:252` |
| sym-1a41011a0c9cf77b1b3e | `pocketstation::frame::audio::FrameLineageError::Timestamp` | variant | Reported when the owning operation encounters timestamp. | `src/frame/audio.rs:256` |
| sym-ba6ef304059f1dc62977 | `pocketstation::frame::audio::SampleFormat::F32Interleaved` | variant | Selects f32 interleaved behavior for `SampleFormat`. | `src/frame/audio.rs:14` |
| sym-5851089e4a084a802423 | `pocketstation::frame::lineage::FrameLineageBuildError::TimestampOverflow` | variant | Reported when the owning operation encounters timestamp overflow. | `src/frame/lineage.rs:99` |
| sym-802fa64e9e6b997312ae | `pocketstation::frame::lineage::FrameLineageBuildError::ZeroDuration` | variant | Reported when the owning operation encounters zero duration. | `src/frame/lineage.rs:95` |
| sym-82871ec25b1689ff12cd | `pocketstation::frame::lineage::FrameLineageBuildError::ZeroSourceGeneration` | variant | Reported when the owning operation encounters zero source generation. | `src/frame/lineage.rs:97` |
| sym-e9754b2fdc87d4bffcc4 | `pocketstation::frame::platform::Platform::Android` | variant | Represents the android alternative defined by `Platform`. | `src/frame/platform.rs:9` |
| sym-01f17ea0041484e3e7cf | `pocketstation::frame::platform::Platform::Ios` | variant | Represents the ios alternative defined by `Platform`. | `src/frame/platform.rs:8` |
| sym-83e779059510799c1958 | `pocketstation::frame::platform::Platform::Linux` | variant | Represents the linux alternative defined by `Platform`. | `src/frame/platform.rs:7` |
| sym-42bd7a670de741864ac3 | `pocketstation::frame::platform::Platform::Macos` | variant | Represents the macos alternative defined by `Platform`. | `src/frame/platform.rs:5` |
| sym-4dd8cdd75b2df7843b4a | `pocketstation::frame::platform::Platform::Unknown` | variant | Represents the unknown alternative defined by `Platform`. | `src/frame/platform.rs:11` |
| sym-994c16c5d185d949e045 | `pocketstation::frame::platform::Platform::Web` | variant | Represents the web alternative defined by `Platform`. | `src/frame/platform.rs:10` |
| sym-eda937305eab45906b57 | `pocketstation::frame::platform::Platform::Windows` | variant | Represents the windows alternative defined by `Platform`. | `src/frame/platform.rs:6` |
| sym-197b96354549fa29229a | `pocketstation::frame::pool::AudioBufferWriteError::CapacityExceeded` | variant | Reported when the owning operation encounters capacity exceeded. | `src/frame/pool.rs:18` |

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

The claims on **Frame and lineage API** are anchored to Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `src/frame/mod.rs:1-18` (`DIRECT`)

For **Frame and lineage API**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.
