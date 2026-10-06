//! Source-preserving replay on a blocking control worker, never a capture callback.
//! Detectors supply Session-time intervals; this module does not classify signals.

use std::fs::{self, File};
use std::io::{BufReader, Cursor, Read, Seek, SeekFrom};
use std::path::{Component, Path, PathBuf};

use super::writer::{checksum_fnv1a64_reader, ManifestDocument, RecorderError};
use super::{RecordingState, RECORDING_MANIFEST_FILE_NAME, RECORDING_MANIFEST_SCHEMA_VERSION};
use crate::frame::{ClockDomainId, SessionId, SourceId, StemId};

/// Maximum encoded WAV bytes returned by one clip request (32 MiB).
pub const MAX_RECORDING_CLIP_BYTES: usize = 32 * 1024 * 1024;
const MAX_SOURCE_BYTES: u64 = 1024 * 1024 * 1024;
const MAX_MANIFEST_BYTES: u64 = 2 * 1024 * 1024;
const MAX_WINDOW_NS: u64 = 120_000_000_000;
const SECOND_NS: u128 = 1_000_000_000;

/// A half-open interval in the recording's Session clock, in nanoseconds.
/// Any external audio, text, telemetry or control detector may supply this interval.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RecordingClipWindow {
    start_ns: u64,
    end_ns: u64,
}

impl RecordingClipWindow {
    /// Validate a nonempty interval of at most 120 seconds.
    pub fn new(start_ns: u64, end_ns: u64) -> Result<Self, RecordingClipError> {
        if end_ns <= start_ns || end_ns - start_ns > MAX_WINDOW_NS {
            return Err(RecordingClipError::InvalidWindow);
        }
        Ok(Self { start_ns, end_ns })
    }

    /// Add context around a detector's interval. Before-context stops at clock zero;
    /// overflow and windows longer than 120 seconds are rejected, never wrapped.
    pub fn around(
        start_ns: u64,
        end_ns: u64,
        before_ns: u64,
        after_ns: u64,
    ) -> Result<Self, RecordingClipError> {
        if end_ns <= start_ns {
            return Err(RecordingClipError::InvalidWindow);
        }
        if before_ns > MAX_WINDOW_NS || after_ns > MAX_WINDOW_NS {
            return Err(RecordingClipError::InvalidWindow);
        }
        Self::new(
            start_ns.saturating_sub(before_ns),
            end_ns
                .checked_add(after_ns)
                .ok_or(RecordingClipError::InvalidWindow)?,
        )
    }

    /// Inclusive requested start in Session nanoseconds.
    pub const fn start_ns(self) -> u64 {
        self.start_ns
    }
    /// Exclusive requested end in Session nanoseconds.
    pub const fn end_ns(self) -> u64 {
        self.end_ns
    }
}

/// Exact persisted identity and format of one independently recorded stem.
#[derive(Debug, Clone)]
pub struct RecordedStem {
    /// Display label; selection and provenance use the identity fields below.
    pub label: String,
    /// Owning Session, not the application's conversation identifier.
    pub session_id: SessionId,
    /// Original captured or external Source.
    pub source_id: SourceId,
    /// Independently recorded stem.
    pub stem_id: StemId,
    /// Original source clock domain, before Session normalization.
    pub clock_id: ClockDomainId,
    /// Source attachment generation preserved by the recorder.
    pub source_generation: u32,
    /// Permission epoch of the recorded media.
    pub permission_epoch: u64,
    /// Actual recording sample rate in Hz.
    pub sample_rate_hz: u32,
    /// Interleaved channel count; no channel mixing is performed.
    pub channels: u16,
    /// Session timestamp of WAV sample zero, already normalized by the recorder.
    pub first_timestamp_ns: u64,
    /// Exclusive Session timestamp of the final recorded buffer.
    pub final_timestamp_ns: u64,
}

/// A bounded float32 WAV clip with its original source and timing observations.
#[derive(Debug)]
pub struct RecordingClip {
    /// Encoded PCM WAV bytes, at most `MAX_RECORDING_CLIP_BYTES`.
    pub wav: Vec<u8>,
    /// Unmodified source, stem, clock, generation and permission provenance.
    pub stem: RecordedStem,
    /// Original detector/context interval.
    pub requested: RecordingClipWindow,
    /// Actual sample-aligned interval, shortened at recording boundaries.
    pub actual: RecordingClipWindow,
    /// First PCM sample frame relative to WAV sample zero, not buffer count.
    pub first_sample_frame: u64,
    /// PCM sample frames per channel in this clip.
    pub sample_frames: u64,
    /// Recorded discontinuities intersecting the actual interval, including
    /// timestamp gaps whose stored PCM is silence. No missing audio is invented.
    pub discontinuities: Vec<super::writer::DiscontinuityRecord>,
}

/// An inspected finalized recording. This owns the bounded original manifest;
/// every extraction rechecks it and the selected WAV on the same open handle.
/// The caller owns authorization for this directory and schedules blocking I/O.
pub struct RecordedAudio {
    directory: PathBuf,
    manifest_bytes: Vec<u8>,
    manifest: ManifestDocument,
    stems: Vec<RecordedStem>,
}

impl super::RecordingOutcome {
    /// Extract context from this finalized Session recording. Run on a blocking
    /// control worker; authorization remains with the owning application.
    pub fn read_clip(
        &self,
        session_id: SessionId,
        stem_id: StemId,
        window: RecordingClipWindow,
    ) -> Result<RecordingClip, RecordingClipError> {
        if self.state != RecordingState::Complete {
            return Err(RecordingClipError::InvalidRecording);
        }
        RecordedAudio::open(&self.session_dir, session_id)?.read_clip(stem_id, window)
    }
}

impl RecordedAudio {
    /// Inspect an existing complete recording for the expected Session identity.
    /// Reject malformed, oversized, unfinished or ambiguous manifests.
    ///
    /// The application authorizes storage access and supplies an external
    /// detector's interval in Session nanoseconds. This runs on a blocking
    /// worker after recording has stopped:
    ///
    /// ```no_run
    /// use pocketstation::{RecordedAudio, RecordingClip, RecordingClipError,
    ///     RecordingClipWindow, SessionId, StemId};
    /// fn context_clip(directory: &std::path::Path, session: SessionId,
    ///     stem: StemId, detected_start_ns: u64, detected_end_ns: u64)
    ///     -> Result<RecordingClip, RecordingClipError>
    /// {
    ///     let recording = RecordedAudio::open(directory, session)?;
    ///     let window = RecordingClipWindow::around(detected_start_ns,
    ///         detected_end_ns, 5_000_000_000, 5_000_000_000)?;
    ///     recording.read_clip(stem, window)
    /// }
    /// ```
    pub fn open(
        directory: impl AsRef<Path>,
        session_id: SessionId,
    ) -> Result<Self, RecordingClipError> {
        let directory = directory.as_ref().to_path_buf();
        if !fs::symlink_metadata(&directory)?.file_type().is_dir() {
            return Err(RecordingClipError::InvalidRecording);
        }
        let manifest_bytes = read_manifest(&directory)?;
        let manifest: ManifestDocument = serde_json::from_slice(&manifest_bytes)?;
        if manifest.schema_version != RECORDING_MANIFEST_SCHEMA_VERSION
            || manifest.session_id != session_id.0
            || manifest.state != RecordingState::Complete
            || !manifest.errors.is_empty()
            || manifest.stems.is_empty()
            || manifest.stems.len() > 64
        {
            return Err(RecordingClipError::InvalidRecording);
        }
        let mut stems = Vec::with_capacity(manifest.stems.len());
        for entry in &manifest.stems {
            if entry.session_id != session_id.0
                || entry.finalization_state != RecordingState::Complete
                || entry.error.is_some()
                || entry.sample_format != "f32_interleaved"
                || entry.checksum_algorithm != "fnv1a64"
                || !(8_000..=192_000).contains(&entry.sample_rate_hz)
                || !(1..=32).contains(&entry.channels)
                || entry.gap_ranges.len() > 1024
                || entry.gap_ranges.iter().any(|gap| {
                    gap.stem_id != entry.stem_id
                        || gap.label != entry.label
                        || gap.timestamp_end_ns < gap.timestamp_start_ns
                })
                || entry.wav_bytes.is_none()
                || entry.checksum.is_none()
                || stems.iter().any(|stem: &RecordedStem| {
                    stem.stem_id.0 == entry.stem_id || stem.label == entry.label
                })
            {
                return Err(RecordingClipError::InvalidRecording);
            }
            let first = entry
                .first_timestamp_ns
                .ok_or(RecordingClipError::InvalidRecording)?;
            let final_ns = entry
                .final_timestamp_ns
                .ok_or(RecordingClipError::InvalidRecording)?;
            if final_ns <= first {
                return Err(RecordingClipError::InvalidRecording);
            }
            stems.push(RecordedStem {
                label: entry.label.clone(),
                session_id,
                source_id: SourceId(
                    entry
                        .source_id
                        .ok_or(RecordingClipError::InvalidRecording)?,
                ),
                stem_id: StemId(entry.stem_id),
                clock_id: ClockDomainId(
                    entry.clock_id.ok_or(RecordingClipError::InvalidRecording)?,
                ),
                source_generation: entry
                    .source_generation
                    .ok_or(RecordingClipError::InvalidRecording)?,
                permission_epoch: entry
                    .permission_epoch
                    .ok_or(RecordingClipError::InvalidRecording)?,
                sample_rate_hz: entry.sample_rate_hz,
                channels: u16::from(entry.channels),
                first_timestamp_ns: first,
                final_timestamp_ns: final_ns,
            });
        }
        Ok(Self {
            directory,
            manifest_bytes,
            manifest,
            stems,
        })
    }

    /// Independently recorded stems; callers select an exact `stem_id`.
    pub fn stems(&self) -> &[RecordedStem] {
        &self.stems
    }

    /// Extract one stem without remixing or changing its sample format.
    /// Rounds start down and end up to retain requested samples, then clips to
    /// available media. Integrity errors fail closed; gaps remain observable.
    /// Maximum source size is 1 GiB; maximum clip is 120 seconds and 32 MiB.
    pub fn read_clip(
        &self,
        stem_id: StemId,
        window: RecordingClipWindow,
    ) -> Result<RecordingClip, RecordingClipError> {
        if read_manifest(&self.directory)? != self.manifest_bytes {
            return Err(RecordingClipError::ChangedRecording);
        }
        let index = self
            .stems
            .iter()
            .position(|stem| stem.stem_id == stem_id)
            .ok_or(RecordingClipError::UnknownStem)?;
        let stem = &self.stems[index];
        let entry = &self.manifest.stems[index];
        let path = safe_file(&self.directory, &entry.wav_path)?;
        let mut file = File::open(path)?;
        let bytes = file.metadata()?.len();
        if bytes > MAX_SOURCE_BYTES {
            return Err(RecordingClipError::SizeLimit);
        }
        if Some(bytes) != entry.wav_bytes
            || checksum_fnv1a64_reader(&mut file)?
                != entry
                    .checksum
                    .as_deref()
                    .ok_or(RecordingClipError::InvalidRecording)?
        {
            return Err(RecordingClipError::ChangedRecording);
        }
        file.seek(SeekFrom::Start(0))?;
        let mut reader = hound::WavReader::new(BufReader::new(file))?;
        let spec = reader.spec();
        if spec.sample_rate != stem.sample_rate_hz
            || spec.channels != stem.channels
            || spec.bits_per_sample != 32
            || spec.sample_format != hound::SampleFormat::Float
            || !reader.len().is_multiple_of(u32::from(spec.channels))
        {
            return Err(RecordingClipError::InvalidRecording);
        }
        let total_frames = u64::from(reader.duration());
        let rate = u128::from(spec.sample_rate);
        let start = window.start_ns.max(stem.first_timestamp_ns);
        let end = window.end_ns.min(stem.final_timestamp_ns);
        if start >= end {
            return Err(RecordingClipError::NoAudio);
        }
        let first = (u128::from(start - stem.first_timestamp_ns) * rate / SECOND_NS) as u64;
        let last = ((u128::from(end - stem.first_timestamp_ns) * rate).div_ceil(SECOND_NS) as u64)
            .min(total_frames);
        if first >= last {
            return Err(RecordingClipError::NoAudio);
        }
        let sample_count = (last - first) * u64::from(spec.channels);
        let output_bytes = sample_count
            .checked_mul(4)
            .and_then(|n| n.checked_add(128))
            .ok_or(RecordingClipError::SizeLimit)?;
        if output_bytes > MAX_RECORDING_CLIP_BYTES as u64 {
            return Err(RecordingClipError::SizeLimit);
        }
        reader.seek(u32::try_from(first).map_err(|_| RecordingClipError::SizeLimit)?)?;
        let mut output = Cursor::new(Vec::with_capacity(output_bytes as usize));
        let mut writer = hound::WavWriter::new(&mut output, spec)?;
        let mut samples = reader.samples::<f32>();
        for _ in 0..sample_count {
            let value = samples.next().ok_or(RecordingClipError::NoAudio)??;
            if !value.is_finite() {
                return Err(RecordingClipError::InvalidRecording);
            }
            writer.write_sample(value)?;
        }
        writer.finalize()?;
        // Sample rounding can expand a valid requested interval by less than
        // two samples. The encoded-byte bound above still applies.
        let actual = RecordingClipWindow {
            start_ns: stem
                .first_timestamp_ns
                .checked_add((u128::from(first) * SECOND_NS / rate) as u64)
                .ok_or(RecordingClipError::InvalidWindow)?,
            end_ns: stem
                .first_timestamp_ns
                .checked_add((u128::from(last) * SECOND_NS / rate) as u64)
                .ok_or(RecordingClipError::InvalidWindow)?,
        };
        let discontinuities = entry
            .gap_ranges
            .iter()
            .filter(|gap| {
                gap.timestamp_start_ns < actual.end_ns
                    && (gap.timestamp_end_ns > actual.start_ns
                        || (gap.timestamp_start_ns == gap.timestamp_end_ns
                            && gap.timestamp_start_ns >= actual.start_ns))
            })
            .cloned()
            .collect();
        if read_manifest(&self.directory)? != self.manifest_bytes {
            return Err(RecordingClipError::ChangedRecording);
        }
        Ok(RecordingClip {
            wav: output.into_inner(),
            stem: stem.clone(),
            requested: window,
            actual,
            first_sample_frame: first,
            sample_frames: last - first,
            discontinuities,
        })
    }
}

fn read_manifest(directory: &Path) -> Result<Vec<u8>, RecordingClipError> {
    let path = safe_file(directory, RECORDING_MANIFEST_FILE_NAME)?;
    if fs::metadata(&path)?.len() > MAX_MANIFEST_BYTES {
        return Err(RecordingClipError::SizeLimit);
    }
    let mut bytes = Vec::new();
    File::open(path)?
        .take(MAX_MANIFEST_BYTES + 1)
        .read_to_end(&mut bytes)?;
    if bytes.len() as u64 > MAX_MANIFEST_BYTES {
        return Err(RecordingClipError::SizeLimit);
    }
    Ok(bytes)
}

fn safe_file(directory: &Path, relative: &str) -> Result<PathBuf, RecordingClipError> {
    let relative = Path::new(relative);
    if relative.as_os_str().is_empty()
        || !relative
            .components()
            .all(|part| matches!(part, Component::Normal(_)))
    {
        return Err(RecordingClipError::InvalidRecording);
    }
    let mut path = directory.to_path_buf();
    for part in relative.components() {
        path.push(part);
        if fs::symlink_metadata(&path)?.file_type().is_symlink() {
            return Err(RecordingClipError::InvalidRecording);
        }
    }
    if !fs::metadata(&path)?.is_file() {
        return Err(RecordingClipError::InvalidRecording);
    }
    Ok(path)
}

/// Typed failure of recording inspection or bounded clip extraction.
#[derive(Debug, thiserror::Error)]
pub enum RecordingClipError {
    /// Invalid, empty, overflowing or overlong Session-time interval.
    #[error("clip interval must be nonempty and at most 120 seconds")]
    InvalidWindow,
    /// Unsupported, unfinished or malformed recording/lineage/format.
    #[error("recording is not a complete source-aware float32 WAV recording")]
    InvalidRecording,
    /// No independently recorded stem has this identity.
    #[error("clip stem does not belong to this recording")]
    UnknownStem,
    /// Requested interval has no available samples.
    #[error("clip interval has no recorded audio")]
    NoAudio,
    /// Manifest changed or persisted WAV failed its recorder integrity check.
    #[error("recording changed or failed integrity verification")]
    ChangedRecording,
    /// Explicit source, manifest or output resource limit was exceeded.
    #[error("clip or recording exceeds its bounded size limit")]
    SizeLimit,
    /// Filesystem failure on the blocking control worker.
    #[error(transparent)]
    Io(#[from] std::io::Error),
    /// WAV parsing or encoding failed.
    #[error(transparent)]
    Wav(#[from] hound::Error),
    /// Manifest decoding failed.
    #[error(transparent)]
    Json(#[from] serde_json::Error),
    /// Recorder checksum read failed.
    #[error(transparent)]
    Recorder(#[from] RecorderError),
}

impl RecordingClipError {
    /// Stable classification shared by native SDKs; messages retain error detail.
    pub const fn code(&self) -> &'static str {
        match self {
            Self::InvalidWindow => "recording.clip_invalid_window",
            Self::InvalidRecording => "recording.clip_invalid_recording",
            Self::UnknownStem => "recording.clip_unknown_stem",
            Self::NoAudio => "recording.clip_no_audio",
            Self::ChangedRecording => "recording.clip_changed_recording",
            Self::SizeLimit => "recording.clip_size_limit",
            Self::Io(_) => "recording.clip_io",
            Self::Wav(_) => "recording.clip_wav",
            Self::Json(_) => "recording.clip_json",
            Self::Recorder(_) => "recording.clip_checksum",
        }
    }
}
