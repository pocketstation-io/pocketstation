# Frame and lineage API

<!-- claims: CLM-REF-004-CAP-001,CLM-REF-004-SOURCE-001 -->

## Scope

- **Preserve frame identity and lineage.** Carry source, stream, stem, route, clock, sequence, generation, and derivation identity with audio frames.

These statements describe repository contracts at the documented snapshot. They do not extend platform qualification, performance, retry, or delivery guarantees beyond the native API contracts and executable evidence.

## Reference authority

The generated [docs.rs API](https://docs.rs/pocketstation/latest/pocketstation/) is the exhaustive symbol-level Rust reference. Human reference pages organize responsibilities and cross-boundary behavior; they do not duplicate every rustdoc signature.

## Public surface

| Declaration | Kind | Purpose | Source |
|---|---|---|---|
| `pocketstation::frame::audio::AudioFrameBuildError` | enum | Classifies failures reported as audio frame build error. | `src/frame/audio.rs:51` |
| `pocketstation::frame::audio::SampleFormat` | enum | Selects the sample format used by PocketStation. | `src/frame/audio.rs:13` |
| `pocketstation::frame::lineage::FrameLineageBuildError` | enum | Classifies failures reported as frame lineage build error. | `src/frame/lineage.rs:93` |
| `pocketstation::frame::platform::Platform` | enum | Enumerates the supported platform cases. | `src/frame/platform.rs:4` |
| `pocketstation::frame::pool::AudioBufferWriteError` | enum | Classifies failures reported as audio buffer write error. | `src/frame/pool.rs:14` |
| `acquire` | function | Attempts to acquire an available buffer slot from `AudioBufferPool`. | `src/frame/pool.rs:75` |
| `acquire_failures` | function | Returns the acquire failures associated with `AudioBufferPool`. | `src/frame/pool.rs:68` |
| `as_mut_slice` | function | Borrows `AudioBufferHandle` as mut slice. | `src/frame/pool.rs:218` |
| `as_slice` | function | Borrows `AudioBufferHandle` as slice. | `src/frame/pool.rs:214` |
| `as_slice` | function | Borrows `SharedAudioBufferHandle` as slice. | `src/frame/pool.rs:300` |
| `available_slots` | function | Returns the available slots associated with `AudioBufferPool`. | `src/frame/pool.rs:71` |
| `channels` | function | Returns the channel count represented by `AudioFrame`. | `src/frame/audio.rs:130` |
| `channels` | function | Returns the channel count represented by `SharedAudioFrame`. | `src/frame/audio.rs:200` |
| `clock_id` | function | Returns the clock identifier associated with `FrameLineage`. | `src/frame/lineage.rs:65` |
| `copy_to_pool` | function | Copies the shared frame into storage acquired from the supplied pool for `SharedAudioFrame`. | `src/frame/audio.rs:233` |
| `discontinuity_epoch` | function | Returns the discontinuity epoch associated with `FrameLineage`. | `src/frame/lineage.rs:80` |
| `drop` | function | Releases resources owned by `AudioBufferHandle`. | `src/frame/pool.rs:265` |
| `drop` | function | Releases resources owned by `SharedAudioBufferHandle`. | `src/frame/pool.rs:322` |
| `duration_ns` | function | Returns the duration nanoseconds associated with `FrameLineage`. | `src/frame/lineage.rs:74` |
| `fmt` | function | Formats `AudioBufferHandle` with the requested formatter. | `src/frame/pool.rs:273` |
| `fmt` | function | Formats `SharedAudioBufferHandle` with the requested formatter. | `src/frame/pool.rs:328` |
| `format` | function | Returns the format associated with `AudioFrame`. | `src/frame/audio.rs:134` |
| `format` | function | Returns the format associated with `SharedAudioFrame`. | `src/frame/audio.rs:204` |
| `frame_samples_for_duration_ms` | function | Returns the frame samples for duration milliseconds associated with `SampleSpec`. | `src/frame/audio.rs:33` |
| `freeze` | function | Freezes mutable storage owned by `AudioFrame` into its shared immutable form. | `src/frame/audio.rs:150` |
| `freeze` | function | Freezes mutable storage owned by `AudioBufferHandle` into its shared immutable form. | `src/frame/pool.rs:246` |
| `get` | function | Returns the value held by `ClockDomainId`. | `src/frame/identity.rs:36` |
| `index` | function | Returns the index associated with `AudioBufferHandle`. | `src/frame/pool.rs:211` |
| `index` | function | Returns the index associated with `SharedAudioBufferHandle`. | `src/frame/pool.rs:296` |
| `is_empty` | function | Returns whether `AudioBufferHandle` contains no values. | `src/frame/pool.rs:208` |
| `is_empty` | function | Returns whether `SharedAudioBufferHandle` contains no values. | `src/frame/pool.rs:292` |
| `is_in_use` | function | Returns whether in use applies to `AudioBufferPool`. | `src/frame/pool.rs:98` |
| `len` | function | Returns the number of values held by `AudioBufferHandle`. | `src/frame/pool.rs:205` |
| `len` | function | Returns the number of values held by `SharedAudioBufferHandle`. | `src/frame/pool.rs:288` |
| `new` | function | Creates a new `SampleSpec`. | `src/frame/audio.rs:25` |
| `new` | function | Creates a new `ClockDomainId`. | `src/frame/identity.rs:32` |
| `new` | function | Creates a new `AudioBufferPool`. | `src/frame/pool.rs:39` |
| `permission_epoch` | function | Returns the permission epoch associated with `FrameLineage`. | `src/frame/lineage.rs:83` |
| `sample_rate_hz` | function | Returns the sample rate hertz associated with `AudioFrame`. | `src/frame/audio.rs:126` |
| `sample_rate_hz` | function | Returns the sample rate hertz associated with `SharedAudioFrame`. | `src/frame/audio.rs:196` |
| `samples` | function | Returns the audio samples held by `AudioFrame`. | `src/frame/audio.rs:146` |
| `samples` | function | Returns the audio samples held by `SharedAudioFrame`. | `src/frame/audio.rs:216` |
| `sequence_number` | function | Returns the sequence number associated with `AudioFrame`. | `src/frame/audio.rs:142` |
| `sequence_number` | function | Returns the sequence number associated with `SharedAudioFrame`. | `src/frame/audio.rs:212` |
| `sequence_number` | function | Returns the sequence number associated with `FrameLineage`. | `src/frame/lineage.rs:68` |
| `session_id` | function | Returns the session identifier associated with `FrameLineage`. | `src/frame/lineage.rs:56` |
| `shared_ref_count` | function | Returns the shared ref count associated with `AudioBufferPool`. | `src/frame/pool.rs:102` |
| `shared_ref_count` | function | Returns the shared ref count associated with `SharedAudioBufferHandle`. | `src/frame/pool.rs:315` |
| `slot_count` | function | Returns the slot count associated with `AudioBufferPool`. | `src/frame/pool.rs:65` |
| `slot_size` | function | Returns the slot size associated with `AudioBufferPool`. | `src/frame/pool.rs:62` |
| `source_generation` | function | Returns the source generation associated with `FrameLineage`. | `src/frame/lineage.rs:77` |
| `source_id` | function | Returns the source identifier associated with `AudioFrame`. | `src/frame/audio.rs:122` |
| `source_id` | function | Returns the source identifier associated with `SharedAudioFrame`. | `src/frame/audio.rs:192` |
| `source_id` | function | Returns the source identifier associated with `FrameLineage`. | `src/frame/lineage.rs:59` |
| `stem_id` | function | Returns the stem identifier associated with `FrameLineage`. | `src/frame/lineage.rs:62` |
| `stream_id` | function | Returns the stream identifier associated with `AudioFrame`. | `src/frame/audio.rs:118` |
| `stream_id` | function | Returns the stream identifier associated with `SharedAudioFrame`. | `src/frame/audio.rs:188` |
| `timestamp_end_ns` | function | Returns the timestamp end nanoseconds associated with `FrameLineage`. | `src/frame/lineage.rs:87` |
| `timestamp_ns` | function | Returns the timestamp nanoseconds associated with `AudioFrame`. | `src/frame/audio.rs:138` |
| `timestamp_ns` | function | Returns the timestamp nanoseconds associated with `SharedAudioFrame`. | `src/frame/audio.rs:208` |
| `timestamp_start_ns` | function | Returns the timestamp start nanoseconds associated with `FrameLineage`. | `src/frame/lineage.rs:71` |
| `try_clone` | function | Attempts to clone through `SharedAudioFrame`. | `src/frame/audio.rs:220` |
| `try_clone` | function | Attempts to clone through `SharedAudioBufferHandle`. | `src/frame/pool.rs:304` |
| `try_copy_from_slice` | function | Copies samples into this fixed-capacity slot without panicking. | `src/frame/pool.rs:240` |
| `try_new` | function | Creates a new `AudioFrame` after validating its inputs. | `src/frame/audio.rs:61` |
| `try_new` | function | Creates a new `FrameLineage` after validating its inputs. | `src/frame/lineage.rs:21` |
| `try_set_len` | function | Changes the visible sample length without panicking. | `src/frame/pool.rs:228` |
| `audio` | module | Types and operations for audio. | `src/frame/audio.rs:1` |
| `lineage` | module | Compact source-aware lineage carried on realtime audio frames. | `src/frame/lineage.rs:1` |
| `pool` | module | Fixed-capacity realtime audio storage and ownership handles. | `src/frame/pool.rs:1` |
| `pocketstation::frame::audio::AudioFrame` | struct | Represents audio frame in the PocketStation API. | `src/frame/audio.rs:39` |
| `pocketstation::frame::audio::SampleSpec` | struct | Configures sample. | `src/frame/audio.rs:18` |
| `pocketstation::frame::audio::SharedAudioFrame` | struct | Represents shared audio frame in the PocketStation API. | `src/frame/audio.rs:176` |
| `pocketstation::frame::identity::ClockDomainId` | struct | Uniquely identifies clock domain. | `src/frame/identity.rs:29` |
| `pocketstation::frame::identity::ConnectorId` | struct | Uniquely identifies connector. | `src/frame/identity.rs:25` |
| `pocketstation::frame::identity::EndpointId` | struct | Uniquely identifies endpoint. | `src/frame/identity.rs:24` |
| `pocketstation::frame::identity::RouteId` | struct | Uniquely identifies route. | `src/frame/identity.rs:26` |
| `pocketstation::frame::identity::SessionId` | struct | Uniquely identifies session. | `src/frame/identity.rs:22` |
| `pocketstation::frame::identity::SourceId` | struct | Uniquely identifies source. | `src/frame/identity.rs:21` |
| `pocketstation::frame::identity::StemId` | struct | Uniquely identifies stem. | `src/frame/identity.rs:23` |
| `pocketstation::frame::identity::StreamId` | struct | Uniquely identifies stream. | `src/frame/identity.rs:20` |
| `pocketstation::frame::lineage::FrameLineage` | struct | Represents frame lineage in the PocketStation API. | `src/frame/lineage.rs:6` |
| `pocketstation::frame::pool::AudioBufferHandle` | struct | Owns bounded access to audio buffer. | `src/frame/pool.rs:198` |
| `pocketstation::frame::pool::AudioBufferPool` | struct | Represents audio buffer pool in the PocketStation API. | `src/frame/pool.rs:24` |
| `pocketstation::frame::pool::SharedAudioBufferHandle` | struct | Owns bounded access to shared audio buffer. | `src/frame/pool.rs:281` |
| `audio::AudioFrameBuildError::MisalignedSamples::channels` | struct_field | Stores the channels associated with `MisalignedSamples`. | `src/frame/audio.rs:57` |
| `audio::AudioFrameBuildError::MisalignedSamples::samples` | struct_field | Stores the samples associated with `MisalignedSamples`. | `src/frame/audio.rs:57` |
| `audio::SampleSpec::channels` | struct_field | Stores the channels associated with `SampleSpec`. | `src/frame/audio.rs:20` |
| `audio::SampleSpec::format` | struct_field | Stores the format associated with `SampleSpec`. | `src/frame/audio.rs:21` |
| `audio::SampleSpec::sample_rate_hz` | struct_field | Stores the sample rate value for `SampleSpec`, in hertz. | `src/frame/audio.rs:19` |
| `pool::AudioBufferWriteError::CapacityExceeded::capacity_samples` | struct_field | Sets the capacity samples available to `CapacityExceeded`. | `src/frame/pool.rs:20` |
| `pool::AudioBufferWriteError::CapacityExceeded::requested_samples` | struct_field | Stores the requested samples associated with `CapacityExceeded`. | `src/frame/pool.rs:19` |
| `pocketstation::frame::audio::AudioFrameBuildError::MisalignedSamples` | variant | Reports misaligned samples. | `src/frame/audio.rs:57` |
| `pocketstation::frame::audio::AudioFrameBuildError::ZeroChannels` | variant | Reports zero channels. | `src/frame/audio.rs:55` |
| `pocketstation::frame::audio::AudioFrameBuildError::ZeroSampleRate` | variant | Reports zero sample rate. | `src/frame/audio.rs:53` |
| `pocketstation::frame::audio::SampleFormat::F32Interleaved` | variant | Selects f32 interleaved behavior for `SampleFormat`. | `src/frame/audio.rs:14` |
| `pocketstation::frame::lineage::FrameLineageBuildError::TimestampOverflow` | variant | Reports timestamp overflow. | `src/frame/lineage.rs:99` |
| `pocketstation::frame::lineage::FrameLineageBuildError::ZeroDuration` | variant | Reports zero duration. | `src/frame/lineage.rs:95` |
| `pocketstation::frame::lineage::FrameLineageBuildError::ZeroSourceGeneration` | variant | Reports zero source generation. | `src/frame/lineage.rs:97` |
| `pocketstation::frame::platform::Platform::Android` | variant | Represents the android case of `Platform`. | `src/frame/platform.rs:9` |
| `pocketstation::frame::platform::Platform::Ios` | variant | Represents the ios case of `Platform`. | `src/frame/platform.rs:8` |
| `pocketstation::frame::platform::Platform::Linux` | variant | Represents the linux case of `Platform`. | `src/frame/platform.rs:7` |
| `pocketstation::frame::platform::Platform::Macos` | variant | Represents the macos case of `Platform`. | `src/frame/platform.rs:5` |
| `pocketstation::frame::platform::Platform::Unknown` | variant | Represents the unknown case of `Platform`. | `src/frame/platform.rs:11` |
| `pocketstation::frame::platform::Platform::Web` | variant | Represents the web case of `Platform`. | `src/frame/platform.rs:10` |
| `pocketstation::frame::platform::Platform::Windows` | variant | Represents the windows case of `Platform`. | `src/frame/platform.rs:6` |
| `pocketstation::frame::pool::AudioBufferWriteError::CapacityExceeded` | variant | Reports capacity exceeded. | `src/frame/pool.rs:18` |

## Interpretation

An inventory row establishes that a declaration, test, lifecycle operation, configuration surface, or protocol element exists at the frozen snapshot. Fields shown as unknown remain deliberately unspecified. Consult the native API and error contract before relying on panic behavior, blocking, cancellation, ordering, limits, retry, or recovery.

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

This page was verified against Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `src/frame/mod.rs:1-18` (`DIRECT`)

A file's presence proves implementation or declaration at this snapshot. It does not by itself prove physical-device qualification, operational performance, retry safety, or behavior outside the recorded test conditions.
