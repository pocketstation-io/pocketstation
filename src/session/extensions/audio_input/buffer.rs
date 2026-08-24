use std::fmt;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;

use crate::frame::{AudioBufferHandle, AudioBufferPool, AudioBufferWriteError};
use crate::runtime::SignalEdgeSender;
use crate::timing::monotonic_timestamp_ns;

use super::AudioInputConfig;

#[doc = "Leases bounded PCM storage from an external-audio input until the caller submits or releases it."]
pub struct AudioInputBuffer {
    writer_id: u64,
    buffer: AudioBufferHandle,
    sample_capacity: usize,
    discontinuity: bool,
}

impl AudioInputBuffer {
    #[doc = "Returns the sample capacity held by `AudioInputBuffer`."]
    pub fn sample_capacity(&self) -> usize {
        self.sample_capacity
    }

    #[doc = "Returns the sample count held by `AudioInputBuffer`."]
    pub fn sample_count(&self) -> usize {
        self.buffer.len()
    }

    #[doc = "Attempts to set sample count through `AudioInputBuffer`."]
    pub fn try_set_sample_count(
        &mut self,
        sample_count: usize,
    ) -> Result<(), AudioBufferWriteError> {
        self.buffer.try_set_len(sample_count)
    }

    #[doc = "Attempts to copy from slice through `AudioInputBuffer`."]
    pub fn try_copy_from_slice(&mut self, samples: &[f32]) -> Result<(), AudioBufferWriteError> {
        self.buffer.try_copy_from_slice(samples)
    }

    #[doc = "Returns the audio samples held by `AudioInputBuffer`."]
    pub fn samples(&self) -> &[f32] {
        self.buffer.as_slice()
    }

    #[doc = "Returns the samples mut held by `AudioInputBuffer`."]
    pub fn samples_mut(&mut self) -> &mut [f32] {
        self.buffer.as_mut_slice()
    }

    #[doc = "Marks the next value from `AudioInputBuffer` as discontinuous."]
    pub fn mark_discontinuity(&mut self) {
        self.discontinuity = true;
    }
}

impl fmt::Debug for AudioInputBuffer {
    #[doc = "Formats `AudioInputBuffer` with the requested formatter."]
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("AudioInputBuffer")
            .field("sample_count", &self.sample_count())
            .field("sample_capacity", &self.sample_capacity())
            .field("discontinuity", &self.discontinuity)
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
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "Reports the audio input observations collected at an observation boundary."]
pub struct AudioInputObservations {
    #[doc = "Sets the capacity frames available to `AudioInputObservations`."]
    pub capacity_frames: u64,
    #[doc = "Contains the buffer slots owned or reported by `AudioInputObservations`."]
    pub buffer_slots: u64,
    #[doc = "Contains the available buffers owned or reported by `AudioInputObservations`."]
    pub available_buffers: u64,
    #[doc = "Counts the total number of accepted observed by `AudioInputObservations`."]
    pub accepted_total: u64,
    #[doc = "Counts the total number of full observed by `AudioInputObservations`."]
    pub full_total: u64,
    #[doc = "Counts the total number of invalid observed by `AudioInputObservations`."]
    pub invalid_total: u64,
    #[doc = "Reports whether cancelled is true for `AudioInputObservations`."]
    pub cancelled: bool,
    #[doc = "Reports whether closed is true for `AudioInputObservations`."]
    pub closed: bool,
}

pub(super) struct QueuedAudioInputFrame {
    pub(super) buffer: AudioBufferHandle,
    pub(super) sequence_number: u64,
    pub(super) timestamp_ns: u64,
    pub(super) duration_ns: u64,
    pub(super) discontinuity_epoch: u64,
}

#[doc = "Sends audio input values across its declared ownership boundary."]
pub struct AudioInputWriter {
    pub(super) writer_id: u64,
    pub(super) config: AudioInputConfig,
    pub(super) pool: Arc<AudioBufferPool>,
    pub(super) sender: Option<SignalEdgeSender<QueuedAudioInputFrame>>,
    pub(super) state: Arc<AudioInputState>,
    pub(super) next_sequence: u64,
    pub(super) next_timestamp_ns: Option<u64>,
    pub(super) discontinuity_epoch: u64,
}

impl AudioInputWriter {
    #[doc = "Returns the configuration held by `AudioInputWriter`."]
    pub const fn configuration(&self) -> AudioInputConfig {
        self.config
    }

    #[doc = "Attempts to acquire through `AudioInputWriter`."]
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
        })
    }

    #[doc = "Attempts to write through `AudioInputWriter`."]
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

    #[doc = "Attempts to send a value through `AudioInputWriter` without waiting for capacity."]
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
                    }),
                ))
            }
        }
    }

    #[doc = "Closes `AudioInputWriter` to further work."]
    pub fn close(&mut self) {
        self.sender = None;
    }

    #[doc = "Returns the observations exposed by `AudioInputWriter`."]
    pub fn observations(&self) -> AudioInputObservations {
        AudioInputObservations {
            capacity_frames: self.config.capacity_frames as u64,
            buffer_slots: self.config.capacity_frames.saturating_add(1) as u64,
            available_buffers: self.pool.available_slots() as u64,
            accepted_total: self.state.accepted_total.load(Ordering::Relaxed),
            full_total: self.state.full_total.load(Ordering::Relaxed),
            invalid_total: self.state.invalid_total.load(Ordering::Relaxed),
            cancelled: self.state.cancelled.load(Ordering::Acquire),
            closed: self.sender.is_none() || self.state.closed.load(Ordering::Acquire),
        }
    }
}

impl fmt::Debug for AudioInputWriter {
    #[doc = "Formats `AudioInputWriter` with the requested formatter."]
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("AudioInputWriter")
            .field("config", &self.config)
            .field("observations", &self.observations())
            .finish()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
#[doc = "Classifies failures surfaced by audio input buffer acquire operations."]
pub enum AudioInputBufferAcquireError {
    #[error("audio input is full")]
    #[doc = "Reports that bounded capacity is full."]
    Full,
    #[error("audio input is closed")]
    #[doc = "Reports that the underlying channel or resource is closed."]
    Closed,
    #[error("audio input Session was cancelled")]
    #[doc = "Indicates that the operation was cancelled."]
    Cancelled,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
#[doc = "Classifies failures surfaced by audio input buffer operations."]
pub enum AudioInputBufferError {
    #[error("audio buffer belongs to another audio input")]
    #[doc = "Reports that source does not match the required identity or contract."]
    WrongSource,
    #[error("audio buffer contains no samples")]
    #[doc = "Represents an empty value or collection."]
    Empty,
    #[error("interleaved sample count is not divisible by the channel count")]
    #[doc = "Reports that channels does not satisfy the required alignment."]
    MisalignedChannels,
    #[error("audio buffer contains {actual_samples} samples; expected exactly {expected_samples}")]
    #[doc = "Reports that frame length does not match the required identity or contract."]
    WrongFrameLength {
        #[doc = "Contains the expected samples owned or reported by `WrongFrameLength`."]
        expected_samples: usize,
        #[doc = "Contains the actual samples owned or reported by `WrongFrameLength`."]
        actual_samples: usize,
    },
    #[error("audio buffer capacity rejected the samples: {0}")]
    #[doc = "Classifies a failure at the capacity stage or component of `AudioInputBufferError`."]
    Capacity(AudioBufferWriteError),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "Selects the audio input write error kind used by PocketStation."]
pub enum AudioInputWriteErrorKind {
    #[doc = "Reports that bounded capacity is full."]
    Full,
    #[doc = "Reports that the underlying channel or resource is closed."]
    Closed,
    #[doc = "Indicates that the operation was cancelled."]
    Cancelled,
    #[doc = "Classifies an external-audio write failure as invalid buffer."]
    InvalidBuffer(AudioInputBufferError),
}

#[doc = "Classifies failures produced during audio input writing."]
pub struct AudioInputWriteError {
    kind: AudioInputWriteErrorKind,
    rejected: Option<AudioInputBuffer>,
}

impl AudioInputWriteError {
    fn new(kind: AudioInputWriteErrorKind, rejected: Option<AudioInputBuffer>) -> Self {
        Self { kind, rejected }
    }

    #[doc = "Returns the kind represented by `AudioInputWriteError`."]
    pub const fn kind(&self) -> AudioInputWriteErrorKind {
        self.kind
    }

    #[doc = "Converts `AudioInputWriteError` into rejected."]
    pub fn into_rejected(self) -> Option<AudioInputBuffer> {
        self.rejected
    }
}

impl From<AudioInputBufferAcquireError> for AudioInputWriteError {
    #[doc = "Converts the supplied value into `AudioInputWriteError`."]
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
    #[doc = "Formats `AudioInputWriteError` with the requested formatter."]
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("AudioInputWriteError")
            .field("kind", &self.kind)
            .field("has_rejected_buffer", &self.rejected.is_some())
            .finish()
    }
}

impl fmt::Display for AudioInputWriteError {
    #[doc = "Formats `AudioInputWriteError` with the requested formatter."]
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind {
            AudioInputWriteErrorKind::Full => formatter.write_str("audio input is full"),
            AudioInputWriteErrorKind::Closed => formatter.write_str("audio input is closed"),
            AudioInputWriteErrorKind::Cancelled => {
                formatter.write_str("audio input Session was cancelled")
            }
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
