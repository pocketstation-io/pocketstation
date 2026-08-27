use std::fmt;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;

use crate::frame::{
    AudioBufferHandle, AudioBufferPool, AudioBufferWriteError, OutputGeneration,
    OutputGenerationError, OutputGenerationId, OutputGenerationState,
};
use crate::runtime::SignalEdgeSender;
use crate::timing::monotonic_timestamp_ns;

use super::AudioInputConfig;

pub struct AudioInputBuffer {
    writer_id: u64,
    buffer: AudioBufferHandle,
    sample_capacity: usize,
    discontinuity: bool,
    output_generation: Option<OutputGeneration>,
}

impl AudioInputBuffer {
    pub fn sample_capacity(&self) -> usize {
        self.sample_capacity
    }

    pub fn sample_count(&self) -> usize {
        self.buffer.len()
    }

    pub fn try_set_sample_count(
        &mut self,
        sample_count: usize,
    ) -> Result<(), AudioBufferWriteError> {
        self.buffer.try_set_len(sample_count)
    }

    pub fn try_copy_from_slice(&mut self, samples: &[f32]) -> Result<(), AudioBufferWriteError> {
        self.buffer.try_copy_from_slice(samples)
    }

    pub fn samples(&self) -> &[f32] {
        self.buffer.as_slice()
    }

    pub fn samples_mut(&mut self) -> &mut [f32] {
        self.buffer.as_mut_slice()
    }

    pub fn mark_discontinuity(&mut self) {
        self.discontinuity = true;
    }

    pub fn set_output_generation(&mut self, generation: &OutputGeneration) {
        self.output_generation = Some(generation.clone());
    }

    pub fn output_generation_id(&self) -> Option<OutputGenerationId> {
        self.output_generation.as_ref().map(OutputGeneration::id)
    }
}

impl fmt::Debug for AudioInputBuffer {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("AudioInputBuffer")
            .field("sample_count", &self.sample_count())
            .field("sample_capacity", &self.sample_capacity())
            .field("discontinuity", &self.discontinuity)
            .field("output_generation_id", &self.output_generation_id())
            .finish()
    }
}

#[derive(Default)]
pub(super) struct AudioInputState {
    pub(super) cancelled: AtomicBool,
    pub(super) closed: AtomicBool,
    accepted_total: AtomicU64,
    full_total: AtomicU64,
    invalid_total: AtomicU64,
    pub(super) discarded_output_frames_total: AtomicU64,
    inactive_output_writes_total: AtomicU64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AudioInputObservations {
    pub capacity_frames: u64,
    pub buffer_slots: u64,
    pub available_buffers: u64,
    pub accepted_total: u64,
    pub full_total: u64,
    pub invalid_total: u64,
    pub discarded_output_frames_total: u64,
    pub inactive_output_writes_total: u64,
    pub cancelled: bool,
    pub closed: bool,
}

pub(super) struct QueuedAudioInputFrame {
    pub(super) buffer: AudioBufferHandle,
    pub(super) sequence_number: u64,
    pub(super) timestamp_ns: u64,
    pub(super) duration_ns: u64,
    pub(super) discontinuity_epoch: u64,
    pub(super) output_generation: Option<OutputGeneration>,
}

pub struct AudioInputWriter {
    pub(super) writer_id: u64,
    pub(super) config: AudioInputConfig,
    pub(super) pool: Arc<AudioBufferPool>,
    pub(super) sender: Option<SignalEdgeSender<QueuedAudioInputFrame>>,
    pub(super) state: Arc<AudioInputState>,
    pub(super) next_sequence: u64,
    pub(super) next_timestamp_ns: Option<u64>,
    pub(super) discontinuity_epoch: u64,
    pub(super) output_generation_state: Arc<OutputGenerationState>,
}

impl AudioInputWriter {
    pub const fn configuration(&self) -> AudioInputConfig {
        self.config
    }

    /// Starts a replaceable output operation for this input.
    ///
    /// Starting a newer generation deactivates every older generation without
    /// closing the input or stopping the Session.
    pub fn begin_output_generation(&self) -> Result<OutputGeneration, OutputGenerationError> {
        self.output_generation_state.begin()
    }

    pub fn try_acquire(&self) -> Result<AudioInputBuffer, AudioInputBufferAcquireError> {
        if self.state.cancelled.load(Ordering::Acquire) {
            return Err(AudioInputBufferAcquireError::Cancelled);
        }
        if self.sender.is_none()
            || self.state.closed.load(Ordering::Acquire)
            || self
                .sender
                .as_ref()
                .is_some_and(|sender| sender.is_abandoned())
        {
            return Err(AudioInputBufferAcquireError::Closed);
        }
        let mut buffer = self
            .pool
            .acquire()
            .ok_or(AudioInputBufferAcquireError::Full)?;
        buffer
            .try_set_len(0)
            .map_err(|_| AudioInputBufferAcquireError::Full)?;
        Ok(AudioInputBuffer {
            writer_id: self.writer_id,
            buffer,
            sample_capacity: self.config.interleaved_samples_per_frame(),
            discontinuity: false,
            output_generation: None,
        })
    }

    pub fn try_write(&mut self, samples: &[f32]) -> Result<(), AudioInputWriteError> {
        let mut buffer = self.try_acquire().map_err(AudioInputWriteError::from)?;
        if let Err(error) = buffer.try_copy_from_slice(samples) {
            self.state.invalid_total.fetch_add(1, Ordering::Relaxed);
            return Err(AudioInputWriteError::new(
                AudioInputWriteErrorKind::InvalidBuffer(AudioInputBufferError::Capacity(error)),
                Some(buffer),
            ));
        }
        self.try_send(buffer)
    }

    /// Writes one complete frame owned by an output generation.
    pub fn try_write_for_output(
        &mut self,
        generation: &OutputGeneration,
        samples: &[f32],
    ) -> Result<(), AudioInputWriteError> {
        self.validate_output_generation(generation)?;
        let mut buffer = self.try_acquire().map_err(AudioInputWriteError::from)?;
        buffer.set_output_generation(generation);
        if let Err(error) = buffer.try_copy_from_slice(samples) {
            self.state.invalid_total.fetch_add(1, Ordering::Relaxed);
            return Err(AudioInputWriteError::new(
                AudioInputWriteErrorKind::InvalidBuffer(AudioInputBufferError::Capacity(error)),
                Some(buffer),
            ));
        }
        self.try_send(buffer)
    }

    pub fn try_send(&mut self, buffer: AudioInputBuffer) -> Result<(), AudioInputWriteError> {
        if self.state.cancelled.load(Ordering::Acquire) {
            return Err(AudioInputWriteError::new(
                AudioInputWriteErrorKind::Cancelled,
                Some(buffer),
            ));
        }
        let Some(sender) = self.sender.as_mut() else {
            return Err(AudioInputWriteError::new(
                AudioInputWriteErrorKind::Closed,
                Some(buffer),
            ));
        };
        if self.state.closed.load(Ordering::Acquire) || sender.is_abandoned() {
            return Err(AudioInputWriteError::new(
                AudioInputWriteErrorKind::Closed,
                Some(buffer),
            ));
        }
        if buffer.writer_id != self.writer_id {
            self.state.invalid_total.fetch_add(1, Ordering::Relaxed);
            return Err(AudioInputWriteError::new(
                AudioInputWriteErrorKind::InvalidBuffer(AudioInputBufferError::WrongSource),
                Some(buffer),
            ));
        }
        if let Some(generation) = &buffer.output_generation {
            if !self.output_generation_state.owns(generation) {
                self.state.invalid_total.fetch_add(1, Ordering::Relaxed);
                return Err(AudioInputWriteError::new(
                    AudioInputWriteErrorKind::InvalidBuffer(
                        AudioInputBufferError::WrongOutputGeneration,
                    ),
                    Some(buffer),
                ));
            }
            if generation.should_discard() {
                self.state
                    .inactive_output_writes_total
                    .fetch_add(1, Ordering::Relaxed);
                return Err(AudioInputWriteError::new(
                    AudioInputWriteErrorKind::OutputGenerationInactive(generation.id()),
                    Some(buffer),
                ));
            }
        }
        let sample_count = buffer.buffer.len();
        let channels = usize::from(self.config.sample_spec.channels);
        if sample_count == 0 {
            self.state.invalid_total.fetch_add(1, Ordering::Relaxed);
            return Err(AudioInputWriteError::new(
                AudioInputWriteErrorKind::InvalidBuffer(AudioInputBufferError::Empty),
                Some(buffer),
            ));
        }
        if !sample_count.is_multiple_of(channels) {
            self.state.invalid_total.fetch_add(1, Ordering::Relaxed);
            return Err(AudioInputWriteError::new(
                AudioInputWriteErrorKind::InvalidBuffer(AudioInputBufferError::MisalignedChannels),
                Some(buffer),
            ));
        }
        let expected_samples = self.config.interleaved_samples_per_frame();
        if sample_count != expected_samples {
            self.state.invalid_total.fetch_add(1, Ordering::Relaxed);
            return Err(AudioInputWriteError::new(
                AudioInputWriteErrorKind::InvalidBuffer(AudioInputBufferError::WrongFrameLength {
                    expected_samples,
                    actual_samples: sample_count,
                }),
                Some(buffer),
            ));
        }

        let sample_frames = sample_count / channels;
        let duration_ns = sample_duration_ns(sample_frames, self.config.sample_spec.sample_rate_hz);
        let timestamp_ns = self
            .next_timestamp_ns
            .unwrap_or_else(monotonic_timestamp_ns);
        let next_discontinuity_epoch = if buffer.discontinuity {
            self.discontinuity_epoch.saturating_add(1)
        } else {
            self.discontinuity_epoch
        };
        let queued = QueuedAudioInputFrame {
            buffer: buffer.buffer,
            sequence_number: self.next_sequence,
            timestamp_ns,
            duration_ns,
            discontinuity_epoch: next_discontinuity_epoch,
            output_generation: buffer.output_generation,
        };
        match sender.try_send(queued) {
            Ok(()) => {
                self.next_sequence = self.next_sequence.saturating_add(1);
                self.next_timestamp_ns = Some(timestamp_ns.saturating_add(duration_ns));
                self.discontinuity_epoch = next_discontinuity_epoch;
                self.state.accepted_total.fetch_add(1, Ordering::Relaxed);
                Ok(())
            }
            Err(error) => {
                self.state.full_total.fetch_add(1, Ordering::Relaxed);
                let queued = error.into_rejected();
                Err(AudioInputWriteError::new(
                    AudioInputWriteErrorKind::Full,
                    Some(AudioInputBuffer {
                        writer_id: self.writer_id,
                        buffer: queued.buffer,
                        sample_capacity: self.config.interleaved_samples_per_frame(),
                        discontinuity: queued.discontinuity_epoch > self.discontinuity_epoch,
                        output_generation: queued.output_generation,
                    }),
                ))
            }
        }
    }

    pub fn close(&mut self) {
        self.sender = None;
    }

    pub fn observations(&self) -> AudioInputObservations {
        AudioInputObservations {
            capacity_frames: self.config.capacity_frames as u64,
            buffer_slots: self.config.capacity_frames.saturating_add(1) as u64,
            available_buffers: self.pool.available_slots() as u64,
            accepted_total: self.state.accepted_total.load(Ordering::Relaxed),
            full_total: self.state.full_total.load(Ordering::Relaxed),
            invalid_total: self.state.invalid_total.load(Ordering::Relaxed),
            discarded_output_frames_total: self
                .state
                .discarded_output_frames_total
                .load(Ordering::Relaxed),
            inactive_output_writes_total: self
                .state
                .inactive_output_writes_total
                .load(Ordering::Relaxed),
            cancelled: self.state.cancelled.load(Ordering::Acquire),
            closed: self.sender.is_none() || self.state.closed.load(Ordering::Acquire),
        }
    }

    fn validate_output_generation(
        &self,
        generation: &OutputGeneration,
    ) -> Result<(), AudioInputWriteError> {
        if !self.output_generation_state.owns(generation) {
            self.state.invalid_total.fetch_add(1, Ordering::Relaxed);
            return Err(AudioInputWriteError::new(
                AudioInputWriteErrorKind::InvalidBuffer(
                    AudioInputBufferError::WrongOutputGeneration,
                ),
                None,
            ));
        }
        if generation.should_discard() {
            self.state
                .inactive_output_writes_total
                .fetch_add(1, Ordering::Relaxed);
            return Err(AudioInputWriteError::new(
                AudioInputWriteErrorKind::OutputGenerationInactive(generation.id()),
                None,
            ));
        }
        Ok(())
    }
}

impl fmt::Debug for AudioInputWriter {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("AudioInputWriter")
            .field("config", &self.config)
            .field("observations", &self.observations())
            .finish()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum AudioInputBufferAcquireError {
    #[error("audio input is full")]
    Full,
    #[error("audio input is closed")]
    Closed,
    #[error("audio input Session was cancelled")]
    Cancelled,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum AudioInputBufferError {
    #[error("audio buffer belongs to another audio input")]
    WrongSource,
    #[error("audio output generation belongs to another audio input")]
    WrongOutputGeneration,
    #[error("audio buffer contains no samples")]
    Empty,
    #[error("interleaved sample count is not divisible by the channel count")]
    MisalignedChannels,
    #[error("audio buffer contains {actual_samples} samples; expected exactly {expected_samples}")]
    WrongFrameLength {
        expected_samples: usize,
        actual_samples: usize,
    },
    #[error("audio buffer capacity rejected the samples: {0}")]
    Capacity(AudioBufferWriteError),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AudioInputWriteErrorKind {
    Full,
    Closed,
    Cancelled,
    OutputGenerationInactive(OutputGenerationId),
    InvalidBuffer(AudioInputBufferError),
}

pub struct AudioInputWriteError {
    kind: AudioInputWriteErrorKind,
    rejected: Option<AudioInputBuffer>,
}

impl AudioInputWriteError {
    fn new(kind: AudioInputWriteErrorKind, rejected: Option<AudioInputBuffer>) -> Self {
        Self { kind, rejected }
    }

    pub const fn kind(&self) -> AudioInputWriteErrorKind {
        self.kind
    }

    pub fn into_rejected(self) -> Option<AudioInputBuffer> {
        self.rejected
    }
}

impl From<AudioInputBufferAcquireError> for AudioInputWriteError {
    fn from(error: AudioInputBufferAcquireError) -> Self {
        let kind = match error {
            AudioInputBufferAcquireError::Full => AudioInputWriteErrorKind::Full,
            AudioInputBufferAcquireError::Closed => AudioInputWriteErrorKind::Closed,
            AudioInputBufferAcquireError::Cancelled => AudioInputWriteErrorKind::Cancelled,
        };
        Self::new(kind, None)
    }
}

impl fmt::Debug for AudioInputWriteError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("AudioInputWriteError")
            .field("kind", &self.kind)
            .field("has_rejected_buffer", &self.rejected.is_some())
            .finish()
    }
}

impl fmt::Display for AudioInputWriteError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind {
            AudioInputWriteErrorKind::Full => formatter.write_str("audio input is full"),
            AudioInputWriteErrorKind::Closed => formatter.write_str("audio input is closed"),
            AudioInputWriteErrorKind::Cancelled => {
                formatter.write_str("audio input Session was cancelled")
            }
            AudioInputWriteErrorKind::OutputGenerationInactive(generation_id) => write!(
                formatter,
                "audio output generation {} is no longer active",
                generation_id.get()
            ),
            AudioInputWriteErrorKind::InvalidBuffer(error) => error.fmt(formatter),
        }
    }
}

impl std::error::Error for AudioInputWriteError {}

fn sample_duration_ns(sample_frames: usize, sample_rate_hz: u32) -> u64 {
    (sample_frames as u128)
        .saturating_mul(1_000_000_000)
        .checked_div(u128::from(sample_rate_hz))
        .unwrap_or(0)
        .min(u128::from(u64::MAX)) as u64
}
