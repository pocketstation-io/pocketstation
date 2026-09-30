use crate::{ChannelLayout, ConfigError};

pub(super) const SAMPLE_RATE_HZ: u32 = 48_000;
pub(super) const ENGINE_FRAME_SAMPLES: usize = 480; // WebRTC's 10 ms at 48 kHz
pub(super) const MAXIMUM_PENDING_AUDIO_MS: u32 = 80;
pub(super) const OUTPUT_POOL_FRAMES: usize = 16; // holds two maximum pending batches
pub(super) const PROCESS_DEADLINE_MS: u32 = 100;

/// Reference and capture layouts negotiated by Session; channels remain separate.
#[derive(Clone, Copy, Debug)]
pub(crate) enum Channels {
    Mono,
    Stereo,
}

impl Channels {
    pub(crate) fn count(self) -> u8 {
        match self {
            Self::Mono => 1,
            Self::Stereo => 2,
        }
    }

    pub(crate) fn layout(self) -> ChannelLayout {
        match self {
            Self::Mono => ChannelLayout::Mono,
            Self::Stereo => ChannelLayout::Stereo,
        }
    }
}

/// Both inputs must already share one clock domain and 48 kHz sample cadence.
/// This contract does not infer an acoustic route or acquire additional audio.
#[derive(Clone, Copy, Debug)]
pub(crate) struct AecConfiguration {
    pub frame_duration_ms: u32,
    pub microphone_channels: Channels,
    pub reference_channels: Channels,
    /// Maximum per-source timestamp residual before resetting adaptation.
    pub maximum_cadence_error_ns: u64,
}

impl AecConfiguration {
    #[cfg(test)]
    pub fn new(frame_duration_ms: u32, reference_channels: Channels) -> Self {
        Self::with_channels(frame_duration_ms, reference_channels, reference_channels)
    }

    pub fn with_channels(
        frame_duration_ms: u32,
        microphone_channels: Channels,
        reference_channels: Channels,
    ) -> Self {
        Self {
            frame_duration_ms,
            microphone_channels,
            reference_channels,
            maximum_cadence_error_ns: 1_000_000,
        }
    }

    pub(crate) fn validate(self) -> Result<(), ConfigError> {
        if !matches!(self.frame_duration_ms, 10 | 20) {
            return Err(ConfigError::Invalid {
                key: "frame_duration_ms".into(),
                reason: "expected 10 or 20 ms".into(),
            });
        }
        if self.maximum_cadence_error_ns > 2_000_000 {
            return Err(ConfigError::Invalid {
                key: "maximum_cadence_error_ns".into(),
                reason: "must not exceed 2 ms; larger cadence jumps require recovery".into(),
            });
        }
        Ok(())
    }

    pub(crate) fn frame_samples(self) -> usize {
        (self.frame_duration_ms * SAMPLE_RATE_HZ / 1_000) as usize
    }
    pub(crate) fn capacity_frames(self) -> usize {
        (MAXIMUM_PENDING_AUDIO_MS / self.frame_duration_ms) as usize
    }
}
