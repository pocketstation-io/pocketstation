#[doc = "Types and operations for audio."]
use std::sync::Arc;

use super::pool::{AudioBufferHandle, AudioBufferPool, SharedAudioBufferHandle};
use crate::frame::{FrameLineage, SourceId, StreamId};

pub const SAMPLE_RATE_HZ: u32 = 48_000;
#[cfg(test)]
const FRAME_DURATION_MS: u32 = 20;
#[cfg(any(test, feature = "internal-testing", target_os = "linux"))]
pub const POOL_SLOT_SAMPLES: usize = 960; // 20ms × 48kHz

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "Selects the sample format used by PocketStation."]
pub enum SampleFormat {
    #[doc = "Selects f32 interleaved behavior for `SampleFormat`."]
    F32Interleaved,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "Configures sample."]
pub struct SampleSpec {
    #[doc = "Stores the sample rate value for `SampleSpec`, in hertz."]
    pub sample_rate_hz: u32,
    #[doc = "Stores the channels used by `SampleSpec`."]
    pub channels: u8,
    #[doc = "Stores the format used by `SampleSpec`."]
    pub format: SampleFormat,
}

impl SampleSpec {
    #[doc = "Creates a new `SampleSpec`."]
    pub fn new(sample_rate_hz: u32, channels: u8, format: SampleFormat) -> Self {
        Self {
            sample_rate_hz,
            channels,
            format,
        }
    }

    #[doc = "Returns the frame samples for duration milliseconds held by `SampleSpec`."]
    pub fn frame_samples_for_duration_ms(&self, duration_ms: u32) -> usize {
        (self.sample_rate_hz * duration_ms / 1000) as usize * self.channels as usize
    }
}

#[derive(Debug)]
#[doc = "Carries one audio payload together with its declared metadata."]
pub struct AudioFrame {
    pub(crate) stream_id: StreamId,
    pub(crate) source_id: SourceId,
    pub(crate) sample_rate_hz: u32,
    pub(crate) channels: u8,
    pub(crate) format: SampleFormat,
    pub(crate) timestamp_ns: u64,
    pub(crate) sequence_number: u64,
    pub(crate) buffer: AudioBufferHandle,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
#[doc = "Classifies failures reported as audio frame build error."]
pub enum AudioFrameBuildError {
    #[error("audio frame sample rate must be non-zero")]
    #[doc = "Reports zero sample rate."]
    ZeroSampleRate,
    #[error("audio frame channel count must be non-zero")]
    #[doc = "Reports zero channels."]
    ZeroChannels,
    #[error("audio frame sample count {samples} is not divisible by {channels} channels")]
    #[doc = "Reports misaligned samples."]
    MisalignedSamples {
        #[doc = "Stores the samples used by `MisalignedSamples`."]
        samples: usize,
        #[doc = "Stores the channels used by `MisalignedSamples`."]
        channels: u8,
    },
}

impl AudioFrame {
    #[doc = "Creates a new `AudioFrame` after validating its inputs."]
    pub fn try_new(
        stream_id: StreamId,
        source_id: SourceId,
        sequence_number: u64,
        timestamp_ns: u64,
        sample_spec: SampleSpec,
        buffer: AudioBufferHandle,
    ) -> Result<Self, AudioFrameBuildError> {
        if sample_spec.sample_rate_hz == 0 {
            return Err(AudioFrameBuildError::ZeroSampleRate);
        }
        if sample_spec.channels == 0 {
            return Err(AudioFrameBuildError::ZeroChannels);
        }
        if !buffer
            .len()
            .is_multiple_of(usize::from(sample_spec.channels))
        {
            return Err(AudioFrameBuildError::MisalignedSamples {
                samples: buffer.len(),
                channels: sample_spec.channels,
            });
        }
        Ok(Self {
            stream_id,
            source_id,
            sample_rate_hz: sample_spec.sample_rate_hz,
            channels: sample_spec.channels,
            format: sample_spec.format,
            timestamp_ns,
            sequence_number,
            buffer,
        })
    }

    /// Constructs a frame after the owning backend validated its negotiated
    /// sample specification and buffer shape during setup.
    pub(crate) fn new(
        stream_id: StreamId,
        source_id: SourceId,
        sequence_number: u64,
        timestamp_ns: u64,
        channels: u8,
        buffer: AudioBufferHandle,
    ) -> Self {
        Self {
            stream_id,
            source_id,
            sample_rate_hz: SAMPLE_RATE_HZ,
            channels,
            format: SampleFormat::F32Interleaved,
            timestamp_ns,
            sequence_number,
            buffer,
        }
    }

    #[doc = "Returns the stream identifier held by `AudioFrame`."]
    pub const fn stream_id(&self) -> StreamId {
        self.stream_id
    }

    #[doc = "Returns the source identifier held by `AudioFrame`."]
    pub const fn source_id(&self) -> SourceId {
        self.source_id
    }

    #[doc = "Returns the sample rate hertz held by `AudioFrame`."]
    pub const fn sample_rate_hz(&self) -> u32 {
        self.sample_rate_hz
    }

    #[doc = "Returns the channel count represented by `AudioFrame`."]
    pub const fn channels(&self) -> u8 {
        self.channels
    }

    #[doc = "Returns the format held by `AudioFrame`."]
    pub const fn format(&self) -> SampleFormat {
        self.format
    }

    #[doc = "Returns the timestamp nanoseconds held by `AudioFrame`."]
    pub const fn timestamp_ns(&self) -> u64 {
        self.timestamp_ns
    }

    #[doc = "Returns the sequence number held by `AudioFrame`."]
    pub const fn sequence_number(&self) -> u64 {
        self.sequence_number
    }

    #[doc = "Returns the audio samples held by `AudioFrame`."]
    pub fn samples(&self) -> &[f32] {
        self.buffer.as_slice()
    }

    #[doc = "Freezes mutable storage owned by `AudioFrame` into its shared immutable form."]
    pub fn freeze(self) -> Option<SharedAudioFrame> {
        let Self {
            stream_id,
            source_id,
            sample_rate_hz,
            channels,
            format,
            timestamp_ns,
            sequence_number,
            buffer,
        } = self;
        let buffer = buffer.freeze().ok()?;
        Some(SharedAudioFrame {
            stream_id,
            source_id,
            sample_rate_hz,
            channels,
            format,
            timestamp_ns,
            sequence_number,
            buffer,
        })
    }
}

#[derive(Debug)]
#[doc = "Carries one shared audio payload together with its declared metadata."]
pub struct SharedAudioFrame {
    pub(crate) stream_id: StreamId,
    pub(crate) source_id: SourceId,
    pub(crate) sample_rate_hz: u32,
    pub(crate) channels: u8,
    pub(crate) format: SampleFormat,
    pub(crate) timestamp_ns: u64,
    pub(crate) sequence_number: u64,
    pub(crate) buffer: SharedAudioBufferHandle,
}

impl SharedAudioFrame {
    #[doc = "Returns the stream identifier held by `SharedAudioFrame`."]
    pub const fn stream_id(&self) -> StreamId {
        self.stream_id
    }

    #[doc = "Returns the source identifier held by `SharedAudioFrame`."]
    pub const fn source_id(&self) -> SourceId {
        self.source_id
    }

    #[doc = "Returns the sample rate hertz held by `SharedAudioFrame`."]
    pub const fn sample_rate_hz(&self) -> u32 {
        self.sample_rate_hz
    }

    #[doc = "Returns the channel count represented by `SharedAudioFrame`."]
    pub const fn channels(&self) -> u8 {
        self.channels
    }

    #[doc = "Returns the format held by `SharedAudioFrame`."]
    pub const fn format(&self) -> SampleFormat {
        self.format
    }

    #[doc = "Returns the timestamp nanoseconds held by `SharedAudioFrame`."]
    pub const fn timestamp_ns(&self) -> u64 {
        self.timestamp_ns
    }

    #[doc = "Returns the sequence number held by `SharedAudioFrame`."]
    pub const fn sequence_number(&self) -> u64 {
        self.sequence_number
    }

    #[doc = "Returns the audio samples held by `SharedAudioFrame`."]
    pub fn samples(&self) -> &[f32] {
        self.buffer.as_slice()
    }

    #[doc = "Attempts to clone through `SharedAudioFrame`."]
    pub fn try_clone(&self) -> Option<Self> {
        Some(Self {
            stream_id: self.stream_id,
            source_id: self.source_id,
            sample_rate_hz: self.sample_rate_hz,
            channels: self.channels,
            format: self.format,
            timestamp_ns: self.timestamp_ns,
            sequence_number: self.sequence_number,
            buffer: self.buffer.try_clone()?,
        })
    }

    #[doc = "Copies the shared frame into storage acquired from the supplied pool for `SharedAudioFrame`."]
    pub fn copy_to_pool(&self, pool: &Arc<AudioBufferPool>) -> Option<AudioFrame> {
        let mut buffer = pool.acquire()?;
        buffer.try_copy_from_slice(self.buffer.as_slice()).ok()?;
        Some(AudioFrame {
            stream_id: self.stream_id,
            source_id: self.source_id,
            sample_rate_hz: self.sample_rate_hz,
            channels: self.channels,
            format: self.format,
            timestamp_ns: self.timestamp_ns,
            sequence_number: self.sequence_number,
            buffer,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum FrameLineageError {
    #[error("frame source id does not match its lineage")]
    Source,
    #[error("frame sequence number does not match its lineage")]
    SequenceNumber,
    #[error("frame timestamp does not match its lineage")]
    Timestamp,
}

/// An exclusive audio frame and the immutable lineage snapshot captured before
/// the frame crosses a bounded edge.
///
/// This envelope keeps dynamic source-generation, discontinuity, and
/// permission epochs attached to the samples they describe. Construction is
/// allocation-free and validates the fields duplicated by `AudioFrame`.
#[derive(Debug)]
pub struct LineagedAudioFrame {
    frame: AudioFrame,
    lineage: FrameLineage,
}

impl LineagedAudioFrame {
    pub fn new(frame: AudioFrame, lineage: FrameLineage) -> Result<Self, FrameLineageError> {
        validate_frame_lineage(&frame, lineage)?;
        Ok(Self { frame, lineage })
    }

    pub const fn frame(&self) -> &AudioFrame {
        &self.frame
    }

    pub const fn lineage(&self) -> FrameLineage {
        self.lineage
    }

    pub fn into_parts(self) -> (AudioFrame, FrameLineage) {
        (self.frame, self.lineage)
    }

    pub fn freeze(self) -> Option<SharedLineagedAudioFrame> {
        Some(SharedLineagedAudioFrame {
            frame: self.frame.freeze()?,
            lineage: self.lineage,
        })
    }
}

#[derive(Debug)]
pub struct SharedLineagedAudioFrame {
    frame: SharedAudioFrame,
    lineage: FrameLineage,
}

impl SharedLineagedAudioFrame {
    pub const fn frame(&self) -> &SharedAudioFrame {
        &self.frame
    }

    pub const fn lineage(&self) -> FrameLineage {
        self.lineage
    }

    pub fn try_clone(&self) -> Option<Self> {
        Some(Self {
            frame: self.frame.try_clone()?,
            lineage: self.lineage,
        })
    }

    pub fn copy_to_pool(&self, pool: &Arc<AudioBufferPool>) -> Option<LineagedAudioFrame> {
        Some(LineagedAudioFrame {
            frame: self.frame.copy_to_pool(pool)?,
            lineage: self.lineage,
        })
    }
}

fn validate_frame_lineage(
    frame: &AudioFrame,
    lineage: FrameLineage,
) -> Result<(), FrameLineageError> {
    if frame.source_id != lineage.source_id {
        return Err(FrameLineageError::Source);
    }
    if frame.sequence_number != lineage.sequence_num {
        return Err(FrameLineageError::SequenceNumber);
    }
    if frame.timestamp_ns != lineage.timestamp_start_ns {
        return Err(FrameLineageError::Timestamp);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frame::{ClockDomainId, SessionId, StemId};

    #[test]
    fn given_pool_with_4_slots_when_all_acquired_then_next_acquire_returns_none() {
        // Given
        let pool = AudioBufferPool::new(4, 16);

        // When
        let a = pool.acquire().unwrap();
        let b = pool.acquire().unwrap();
        let c = pool.acquire().unwrap();
        let d = pool.acquire().unwrap();

        // Then
        assert!(pool.acquire().is_none());
        drop(a);
        drop(b);
        drop(c);
        drop(d);
        assert!(pool.acquire().is_some());
    }

    #[test]
    fn given_pool_acquisition_and_release_when_observed_then_available_slots_are_exact() {
        let pool = AudioBufferPool::new(2, 4);
        assert_eq!(pool.available_slots(), 2);

        let handle = pool.acquire().unwrap();
        assert_eq!(pool.available_slots(), 1);

        drop(handle);
        assert_eq!(pool.available_slots(), 2);
    }

    #[test]
    fn given_acquired_handle_when_copy_from_slice_then_length_matches_data() {
        // Given
        let pool = AudioBufferPool::new(1, 8);
        let mut h = pool.acquire().unwrap();

        // When
        h.try_copy_from_slice(&[1.0, 2.0, 3.0]).unwrap();

        // Then
        assert_eq!(h.len(), 3);
        assert_eq!(h.as_slice(), &[1.0, 2.0, 3.0]);
    }

    #[test]
    fn given_full_64_slot_pool_when_acquire_then_returns_none_and_increments_failures() {
        // Given
        let pool = AudioBufferPool::new(64, 4);

        // When
        let handles: Vec<_> = (0..64).map(|_| pool.acquire().unwrap()).collect();
        let extra = pool.acquire();

        // Then
        assert_eq!(pool.acquire_failures(), 1);
        assert!(extra.is_none());
        drop(handles);
    }

    #[test]
    fn given_exhausted_pool_when_handle_dropped_then_reacquire_succeeds() {
        // Given
        let pool = AudioBufferPool::new(1, 4);
        let h = pool.acquire().unwrap();
        assert!(pool.acquire().is_none());

        // When
        drop(h);

        // Then
        assert!(pool.acquire().is_some());
    }

    #[test]
    fn given_pool_when_acquire_and_release_then_in_use_flag_tracks_state() {
        // Given
        let pool = AudioBufferPool::new(2, 4);
        let h = pool.acquire().unwrap();
        let slot = h.index();

        // When / Then
        assert!(pool.is_in_use(slot));
        drop(h);
        assert!(!pool.is_in_use(slot));
    }

    #[test]
    fn given_48khz_mono_spec_when_frame_samples_for_20ms_then_returns_960() {
        // Given
        let spec = SampleSpec::new(SAMPLE_RATE_HZ, 1, SampleFormat::F32Interleaved);

        // When
        let samples = spec.frame_samples_for_duration_ms(FRAME_DURATION_MS);

        // Then
        assert_eq!(samples, 960);
    }

    #[test]
    fn given_48khz_stereo_spec_when_frame_samples_for_20ms_then_returns_1920() {
        // Given
        let spec = SampleSpec::new(SAMPLE_RATE_HZ, 2, SampleFormat::F32Interleaved);

        // When
        let samples = spec.frame_samples_for_duration_ms(FRAME_DURATION_MS);

        // Then
        assert_eq!(samples, 1920);
    }

    #[test]
    fn given_frame_lineage_when_timestamp_end_requested_then_duration_is_saturating() {
        // Given
        let lineage = FrameLineage {
            session_id: SessionId(1),
            source_id: SourceId(2),
            stem_id: StemId(3),
            clock_id: ClockDomainId(4),
            sequence_num: 5,
            timestamp_start_ns: u64::MAX - 2,
            duration_ns: 10,
            source_generation: 6,
            discontinuity_epoch: 7,
            permission_epoch: 8,
        };

        // When / Then
        assert_eq!(lineage.timestamp_end_ns(), u64::MAX);
    }

    #[test]
    fn given_matching_frame_and_lineage_when_frozen_then_epochs_survive_fanout() {
        // Given
        let pool = AudioBufferPool::new(2, 4);
        let mut buffer = pool.acquire().unwrap();
        buffer.try_copy_from_slice(&[0.1, 0.2, 0.3, 0.4]).unwrap();
        let frame = AudioFrame::new(StreamId(1), SourceId(2), 3, 4, 1, buffer);
        let lineage = FrameLineage {
            session_id: SessionId(5),
            source_id: SourceId(2),
            stem_id: StemId(6),
            clock_id: ClockDomainId(7),
            sequence_num: 3,
            timestamp_start_ns: 4,
            duration_ns: 80_000,
            source_generation: 8,
            discontinuity_epoch: 9,
            permission_epoch: 10,
        };

        // When
        let exclusive = LineagedAudioFrame::new(frame, lineage).unwrap();
        let shared = exclusive.freeze().unwrap();
        let second_delivery = shared.try_clone().unwrap();
        let copied = second_delivery.copy_to_pool(&pool).unwrap();

        // Then
        assert_eq!(shared.lineage(), lineage);
        assert_eq!(copied.lineage(), lineage);
        assert_eq!(copied.frame().buffer.as_slice(), &[0.1, 0.2, 0.3, 0.4]);
    }

    #[test]
    fn given_mismatched_dynamic_frame_identity_when_enveloped_then_rejected() {
        // Given
        let pool = AudioBufferPool::new(1, 4);
        let buffer = pool.acquire().unwrap();
        let frame = AudioFrame::new(StreamId(1), SourceId(2), 3, 4, 1, buffer);
        let lineage = FrameLineage {
            session_id: SessionId(5),
            source_id: SourceId(99),
            stem_id: StemId(6),
            clock_id: ClockDomainId(7),
            sequence_num: 3,
            timestamp_start_ns: 4,
            duration_ns: 80_000,
            source_generation: 8,
            discontinuity_epoch: 9,
            permission_epoch: 10,
        };

        // When
        let result = LineagedAudioFrame::new(frame, lineage);

        // Then
        assert!(matches!(result, Err(FrameLineageError::Source)));
    }

    #[test]
    fn given_frozen_buffer_with_many_consumers_when_final_handle_drops_then_slot_is_reused() {
        // Given
        let pool = AudioBufferPool::new(1, 4);
        let mut exclusive = pool.acquire().unwrap();
        exclusive
            .try_copy_from_slice(&[0.1, 0.2, 0.3, 0.4])
            .unwrap();
        let shared = exclusive.freeze().unwrap();
        let second = shared.try_clone().unwrap();
        let third = shared.try_clone().unwrap();

        // When / Then
        assert_eq!(shared.shared_ref_count(), 3);
        assert!(pool.acquire().is_none());
        drop(second);
        drop(shared);
        assert!(pool.acquire().is_none());
        drop(third);
        assert!(pool.acquire().is_some());
    }

    #[test]
    fn given_rejected_shared_branch_when_handle_drops_then_reference_is_released() {
        // Given
        let pool = AudioBufferPool::new(1, 2);
        let shared = pool.acquire().unwrap().freeze().unwrap();
        let rejected_branch = shared.try_clone().unwrap();

        // When
        drop(rejected_branch);

        // Then
        assert_eq!(shared.shared_ref_count(), 1);
    }

    #[test]
    fn given_queued_shared_branches_when_queue_drops_then_all_references_are_released() {
        // Given
        let pool = AudioBufferPool::new(1, 2);
        let shared = pool.acquire().unwrap().freeze().unwrap();
        let queued = vec![shared.try_clone().unwrap(), shared.try_clone().unwrap()];
        assert_eq!(shared.shared_ref_count(), 3);

        // When
        drop(queued);

        // Then
        assert_eq!(shared.shared_ref_count(), 1);
    }

    #[test]
    fn given_shared_frame_when_copied_to_branch_pool_then_samples_are_independent() {
        // Given
        let source_pool = AudioBufferPool::new(1, 3);
        let branch_pool = AudioBufferPool::new(1, 3);
        let mut buffer = source_pool.acquire().unwrap();
        buffer.try_copy_from_slice(&[0.25, -0.5, 0.75]).unwrap();
        let shared = AudioFrame::new(StreamId(11), SourceId(2), 5, 6, 1, buffer)
            .freeze()
            .unwrap();

        // When
        let mut branch = shared.copy_to_pool(&branch_pool).unwrap();
        branch.buffer.as_mut_slice()[0] = 1.0;

        // Then
        assert_eq!(shared.buffer.as_slice(), &[0.25, -0.5, 0.75]);
        assert_eq!(branch.buffer.as_slice(), &[1.0, -0.5, 0.75]);
        assert_eq!(branch.source_id, shared.source_id);
        assert_eq!(branch.sequence_number, shared.sequence_number);
        assert_eq!(branch.timestamp_ns, shared.timestamp_ns);
        assert!(shared.copy_to_pool(&branch_pool).is_none());
    }

    #[test]
    fn given_shared_reference_at_max_when_clone_attempted_then_clone_fails_without_wraparound() {
        // Given
        let pool = AudioBufferPool::new(1, 1);
        let shared = pool.acquire().unwrap().freeze().unwrap();
        pool.set_shared_ref_count_for_testing(0, usize::MAX);

        // When / Then
        assert!(shared.try_clone().is_none());
        assert_eq!(pool.shared_ref_count(0), usize::MAX);
    }

    #[test]
    fn given_zero_shared_references_when_release_attempted_then_underflow_is_rejected() {
        // Given
        let pool = AudioBufferPool::new(1, 1);

        // When / Then
        assert!(!pool.release_shared_for_testing(0));
        assert_eq!(pool.shared_ref_count(0), 0);
        assert!(pool.acquire().is_some());
    }
}
