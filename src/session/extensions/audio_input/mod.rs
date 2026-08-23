mod buffer;
mod source;

use std::fmt;

use crate::session::declaration::{SourceInstanceHandle, SourceOutputHandle};
use crate::{SampleFormat, SampleSpec};

pub use buffer::{
    AudioInputBuffer, AudioInputBufferAcquireError, AudioInputBufferError, AudioInputObservations,
    AudioInputWriteError, AudioInputWriteErrorKind, AudioInputWriter,
};
pub(crate) use source::AudioInputFactory;
pub use source::{AudioInputError, PcmSource};

const MAX_CAPACITY_FRAMES: usize = 63;

/// Stable runtime identity of the underlying PCM source implementation.
pub const PCM_SOURCE_TYPE_ID: &str = "io.pocketstation.source.pcm.v1";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "Configures audio input."]
pub struct AudioInputConfig {
    sample_spec: SampleSpec,
    capacity_frames: usize,
    frame_samples_per_channel: usize,
}

impl AudioInputConfig {
    #[doc = "Creates a new `AudioInputConfig`."]
    pub fn new(
        sample_spec: SampleSpec,
        capacity_frames: usize,
        frame_samples_per_channel: usize,
    ) -> Result<Self, AudioInputConfigError> {
        if sample_spec.sample_rate_hz == 0 {
            return Err(AudioInputConfigError::ZeroSampleRate);
        }
        if !matches!(sample_spec.channels, 1 | 2) {
            return Err(AudioInputConfigError::UnsupportedChannelCount);
        }
        if sample_spec.format != SampleFormat::F32Interleaved {
            return Err(AudioInputConfigError::UnsupportedSampleFormat);
        }
        if !(1..=MAX_CAPACITY_FRAMES).contains(&capacity_frames) {
            return Err(AudioInputConfigError::InvalidCapacity);
        }
        if frame_samples_per_channel == 0 {
            return Err(AudioInputConfigError::ZeroFrameSamples);
        }
        frame_samples_per_channel
            .checked_mul(usize::from(sample_spec.channels))
            .ok_or(AudioInputConfigError::FrameSampleCountOverflow)?;
        Ok(Self {
            sample_spec,
            capacity_frames,
            frame_samples_per_channel,
        })
    }

    #[doc = "Returns the sample spec held by `AudioInputConfig`."]
    pub const fn sample_spec(self) -> SampleSpec {
        self.sample_spec
    }

    #[doc = "Returns the capacity frames held by `AudioInputConfig`."]
    pub const fn capacity_frames(self) -> usize {
        self.capacity_frames
    }

    #[doc = "Returns the frame samples per channel held by `AudioInputConfig`."]
    pub const fn frame_samples_per_channel(self) -> usize {
        self.frame_samples_per_channel
    }

    pub(super) fn interleaved_samples_per_frame(self) -> usize {
        self.frame_samples_per_channel * usize::from(self.sample_spec.channels)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
#[doc = "Classifies failures reported as audio input config error."]
pub enum AudioInputConfigError {
    #[error("audio input sample rate must be non-zero")]
    #[doc = "Reports zero sample rate."]
    ZeroSampleRate,
    #[error("audio input currently supports mono or stereo PCM")]
    #[doc = "Reports unsupported channel count."]
    UnsupportedChannelCount,
    #[error("audio input currently supports F32 interleaved PCM")]
    #[doc = "Reports unsupported sample format."]
    UnsupportedSampleFormat,
    #[error("audio input capacity must be between 1 and 63 frames")]
    #[doc = "Reports invalid capacity."]
    InvalidCapacity,
    #[error("audio input frame sample count must be non-zero")]
    #[doc = "Reports zero frame samples."]
    ZeroFrameSamples,
    #[error("audio input frame sample count overflows the platform size")]
    #[doc = "Reports frame sample count overflow."]
    FrameSampleCountOverflow,
}

/// Intent-first façade for feeding audio already owned by the embedding
/// application into a Session.
pub struct AudioInput {
    pcm: PcmSource,
}

impl AudioInput {
    pub(crate) const fn new(pcm: PcmSource) -> Self {
        Self { pcm }
    }

    #[doc = "Returns the source held by `AudioInput`."]
    pub const fn source(&self) -> &SourceInstanceHandle {
        self.pcm.source()
    }

    #[doc = "Returns the output held by `AudioInput`."]
    pub const fn output(&self) -> &SourceOutputHandle {
        self.pcm.output()
    }

    /// Acquires one preallocated buffer owned by this input.
    pub fn try_acquire(&self) -> Result<AudioInputBuffer, AudioInputBufferAcquireError> {
        self.pcm.writer().try_acquire()
    }

    /// Writes one complete interleaved frame without blocking.
    pub fn try_write(&mut self, samples: &[f32]) -> Result<(), AudioInputWriteError> {
        self.pcm.writer_mut().try_write(samples)
    }

    /// Submits one previously acquired buffer without blocking.
    pub fn try_send(&mut self, buffer: AudioInputBuffer) -> Result<(), AudioInputWriteError> {
        self.pcm.writer_mut().try_send(buffer)
    }

    /// Closes this application-owned input after its accepted frames drain.
    pub fn close(&mut self) {
        self.pcm.writer_mut().close();
    }

    #[doc = "Returns the observations exposed by `AudioInput`."]
    pub fn observations(&self) -> AudioInputObservations {
        self.pcm.writer().observations()
    }

    /// Converts the convenience façade into explicit source, output, and
    /// producer ownership.
    pub fn into_pcm_source(self) -> PcmSource {
        self.pcm
    }
}

impl fmt::Debug for AudioInput {
    #[doc = "Formats `AudioInput` with the requested formatter."]
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("AudioInput")
            .field("source", self.source())
            .field("output", self.output())
            .field("observations", &self.observations())
            .finish()
    }
}
