//! Recent PCM owned by an ordinary Session endpoint worker, never callbacks.

mod endpoint;
mod state;
#[cfg(test)]
mod tests;

use std::io::Cursor;
use std::sync::{Arc, Mutex};

use super::{RecordedStem, RecordingClip, RecordingClipWindow, MAX_RECORDING_CLIP_BYTES};
use crate::frame::StemId;

pub(crate) use endpoint::HistoryFactory;

/// Limits shared by all stems in one history. Oldest buffers are evicted first.
/// PCM bytes and buffer count are separate limits; endpoint queues are separate.
#[derive(Debug, Clone, Copy)]
pub struct AudioHistoryConfig {
    pub retention_ns: u64,
    pub max_pcm_bytes: usize,
    pub max_buffers: usize,
}

impl Default for AudioHistoryConfig {
    fn default() -> Self {
        Self {
            retention_ns: 30_000_000_000,
            max_pcm_bytes: 16 * 1024 * 1024,
            max_buffers: 4096,
        }
    }
}

impl AudioHistoryConfig {
    pub(crate) fn validate(self) -> Result<(), AudioHistoryError> {
        if self.retention_ns == 0
            || self.retention_ns > 120_000_000_000
            || self.max_pcm_bytes == 0
            || self.max_pcm_bytes > 64 * 1024 * 1024
            || self.max_buffers == 0
            || self.max_buffers > 65_536
        {
            return Err(AudioHistoryError::InvalidLimits);
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AudioHistoryState {
    Preparing,
    Running,
    Complete,
    Cancelled,
    Failed,
}

#[derive(Debug, Clone, Copy)]
pub struct AudioHistoryObservations {
    pub state: AudioHistoryState,
    pub retained_pcm_bytes: usize,
    pub retained_buffers: usize,
    pub received_buffers_total: u64,
    pub evicted_buffers_total: u64,
    pub rejected_buffers_total: u64,
    pub discontinuities_total: u64,
    pub source_resets_total: u64,
}

/// Read recent independently routed audio while its Session runs.
///
/// Call from a control worker. A future end returns `NotReady`; retry after
/// more audio arrives. Expired or missing context fails explicitly. No requests
/// or audio copies accumulate in a hidden pending-trigger queue.
///
/// ```no_run
/// use pocketstation::{Session, AudioHistoryConfig, AudioInputConfig,
///     RecordingClipWindow, SampleSpec, SampleFormat};
/// # fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let spec = SampleSpec::new(48_000, 1, SampleFormat::F32Interleaved);
/// let session = Session::builder().sample_spec(spec).build();
/// let history = session.audio_history(AudioHistoryConfig::default())?;
/// let mut input = session.audio_input(AudioInputConfig::new(spec, 16, 960)?)?;
/// input.output().retain_audio()?;
/// let mut running = session.start()?;
/// // Feed/capture audio, then use the detector's Session-time interval:
/// if let Some(stem) = history.stems()?.first() {
///     let window = RecordingClipWindow::new(stem.first_timestamp_ns,
///         stem.final_timestamp_ns)?;
///     let clip = history.read_clip(stem.stem_id, window)?;
///     assert!(!clip.wav.is_empty());
/// }
/// input.close();
/// running.stop();
/// # Ok(()) }
/// ```
#[derive(Clone)]
pub struct AudioHistory {
    shared: Arc<Mutex<state::HistoryState>>,
}

impl AudioHistory {
    pub(crate) fn new(config: AudioHistoryConfig) -> Result<Self, AudioHistoryError> {
        config.validate()?;
        Ok(Self {
            shared: Arc::new(Mutex::new(state::HistoryState::new(config))),
        })
    }

    /// Metadata is available only after a source delivers its first real frame.
    pub fn stems(&self) -> Result<Vec<RecordedStem>, AudioHistoryError> {
        Ok(self
            .shared
            .lock()
            .map_err(|_| AudioHistoryError::Failed)?
            .metadata())
    }

    pub fn observations(&self) -> Result<AudioHistoryObservations, AudioHistoryError> {
        Ok(self
            .shared
            .lock()
            .map_err(|_| AudioHistoryError::Failed)?
            .observations)
    }

    /// Copy at most 32 MiB from retained PCM, then encode after releasing the
    /// history mutex. Sample bounds use the same half-open ns windows as files.
    /// The caller owns returned WAV bytes and limits concurrent requests.
    pub fn read_clip(
        &self,
        stem_id: StemId,
        window: RecordingClipWindow,
    ) -> Result<RecordingClip, AudioHistoryError> {
        let snapshot = self
            .shared
            .lock()
            .map_err(|_| AudioHistoryError::Failed)?
            .snapshot(stem_id, window)?;
        let mut output = Cursor::new(Vec::with_capacity(snapshot.samples.len() * 4 + 128));
        let spec = hound::WavSpec {
            channels: snapshot.stem.channels,
            sample_rate: snapshot.stem.sample_rate_hz,
            bits_per_sample: 32,
            sample_format: hound::SampleFormat::Float,
        };
        let mut writer = hound::WavWriter::new(&mut output, spec)?;
        for value in snapshot.samples {
            writer.write_sample(value)?;
        }
        writer.finalize()?;
        let wav = output.into_inner();
        if wav.len() > MAX_RECORDING_CLIP_BYTES {
            return Err(AudioHistoryError::SizeLimit);
        }
        Ok(RecordingClip {
            wav,
            stem: snapshot.stem,
            requested: window,
            actual: snapshot.actual,
            first_sample_frame: snapshot.first_sample_frame,
            sample_frames: snapshot.sample_frames,
            discontinuities: Vec::new(),
        })
    }

    /// Discard retained audio without altering capture or other destinations.
    /// Returned clips are owned copies and are unaffected. This is not shutdown.
    pub fn clear(&self) -> Result<(), AudioHistoryError> {
        self.shared
            .lock()
            .map_err(|_| AudioHistoryError::Failed)?
            .clear();
        Ok(())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum AudioHistoryError {
    #[error("history requires positive retention up to 120 seconds, 1..=67108864 PCM bytes and 1..=65536 buffers")]
    InvalidLimits,
    #[error("stem has not delivered audio to this history")]
    UnknownStem,
    #[error("requested context has not arrived yet")]
    NotReady,
    #[error("requested context has expired or was cleared")]
    Expired,
    #[error("requested context contains missing or discontinuous audio")]
    MissingContext,
    #[error("history capture ended before this context arrived")]
    Ended,
    #[error("history was cancelled and its retained audio discarded")]
    Cancelled,
    #[error("history worker or shared state failed")]
    Failed,
    #[error("clip or input exceeds its PCM byte limit")]
    SizeLimit,
    #[error("source identity, format, timing or samples are invalid")]
    InvalidFrame,
    #[error(transparent)]
    Wav(#[from] hound::Error),
}

impl AudioHistoryError {
    pub const fn code(&self) -> &'static str {
        match self {
            Self::InvalidLimits => "recording.history_invalid_limits",
            Self::UnknownStem => "recording.history_unknown_stem",
            Self::NotReady => "recording.history_not_ready",
            Self::Expired => "recording.history_expired",
            Self::MissingContext => "recording.history_missing_context",
            Self::Ended => "recording.history_ended",
            Self::Cancelled => "recording.history_cancelled",
            Self::Failed => "recording.history_failed",
            Self::SizeLimit => "recording.history_size_limit",
            Self::InvalidFrame => "recording.history_invalid_frame",
            Self::Wav(_) => "recording.history_wav",
        }
    }
}
