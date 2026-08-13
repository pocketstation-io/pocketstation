use std::fs::{self, File};
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;
use std::thread::{self, JoinHandle};
use std::time::Duration;

use super::config::{
    PermissionDecision, PermissionScope, RecorderLineageField, RecorderStemConfig, StemLabel,
};
use crate::frame::SessionId;
use crate::runtime::{EdgeObservations, PlanEdgeFrame, PlanEdgeReceiver};
use hound::{SampleFormat as WavSampleFormat, WavSpec, WavWriter};
use serde::Serialize;

const MANIFEST_SCHEMA_VERSION: u32 = 1;
const WORKER_IDLE_WAIT_MS: u64 = 1;
const MAX_MANIFEST_GAPS: usize = 1_024;
const MAX_SILENCE_GAP_NS: u64 = 3_600_000_000_000; // one hour

#[derive(Debug, thiserror::Error)]
pub enum RecorderError {
    #[error("recording output already exists: {0}")]
    OutputExists(PathBuf),
    #[error("invalid stem label '{0}'")]
    InvalidStemLabel(String),
    #[error("duplicate stem label '{0}'")]
    DuplicateStemLabel(String),
    #[error("stem '{label}' belongs to session {actual}, expected {expected}")]
    SessionMismatch {
        label: String,
        actual: u64,
        expected: u64,
    },
    #[error("permission denied for stem '{0}'")]
    PermissionDenied(String),
    #[error("stem '{label}' has invalid sample spec {sample_rate_hz} Hz/{channels} ch")]
    InvalidSampleSpec {
        label: String,
        sample_rate_hz: u32,
        channels: u8,
    },
    #[error("stem '{label}' received source {actual}, expected {expected}")]
    SourceMismatch {
        label: String,
        actual: u64,
        expected: u64,
    },
    #[error("stem '{label}' frame lineage {field:?} is {actual}, expected {expected}")]
    LineageMismatch {
        label: String,
        field: RecorderLineageField,
        actual: u64,
        expected: u64,
    },
    #[error("stem '{label}' frame spec is {actual_rate_hz} Hz/{actual_channels} ch, expected {expected_rate_hz} Hz/{expected_channels} ch")]
    FrameSpecMismatch {
        label: String,
        actual_rate_hz: u32,
        actual_channels: u8,
        expected_rate_hz: u32,
        expected_channels: u8,
    },
    #[error("stem '{0}' has a frame whose samples are not channel-aligned")]
    UnalignedSamples(String),
    #[error("stem '{0}' timestamp cannot be normalized")]
    TimestampOutOfRange(String),
    #[error("stem '{label}' gap {duration_ns} ns exceeds the one-hour proof limit")]
    GapTooLarge { label: String, duration_ns: u64 },
    #[error("stem '{0}' exceeded the bounded manifest gap count")]
    TooManyGaps(String),
    #[error("recorder worker '{0}' panicked")]
    WorkerPanicked(String),
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("WAV error: {0}")]
    Wav(#[from] hound::Error),
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RecordingState {
    Recording,
    Complete,
    Incomplete,
}

#[derive(Debug, Clone, Serialize)]
pub struct DiscontinuityRecord {
    pub stem_id: u64,
    pub label: String,
    pub kind: DiscontinuityKind,
    pub timestamp_start_ns: u64,
    pub timestamp_end_ns: u64,
    pub sequence_start: Option<u64>,
    pub sequence_end: Option<u64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DiscontinuityKind {
    TimestampGap,
    SequenceGap,
    OverlapRejected,
}

#[derive(Debug, Clone)]
pub struct RecordingOutcome {
    pub session_dir: PathBuf,
    pub state: RecordingState,
    pub completed_stems: usize,
    pub failed_stems: usize,
    pub stems: Vec<RecordingStemOutcome>,
}

#[derive(Debug, Clone)]
pub struct RecordingStemOutcome {
    pub label: String,
    pub written_frames: u64,
    pub stale_frames: u64,
    pub gap_ranges: Vec<DiscontinuityRecord>,
    pub error: Option<String>,
    pub edge_observations: EdgeObservations,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct RecordingObservations {
    pub frames_received_total: u64,
    pub frames_written_total: u64,
    pub frames_rejected_total: u64,
    pub discontinuities_total: u64,
    pub failures_total: u64,
}

pub struct MultistemRecording {
    session_id: SessionId,
    session_dir: PathBuf,
    configs: Vec<RecorderStemConfig>,
    workers: Vec<RecorderWorker>,
    finished: bool,
}

impl MultistemRecording {
    pub(crate) fn start_observed(
        output_root: impl AsRef<Path>,
        session_id: SessionId,
        stems: Vec<(RecorderStemConfig, PlanEdgeReceiver, PlanEdgeFrame)>,
    ) -> Result<Self, RecorderError> {
        let stems = stems
            .into_iter()
            .map(|(config, receiver, initial_frame)| RecorderStemInput {
                config,
                receiver,
                initial_frame: Some(initial_frame),
            })
            .collect();
        Self::start_with_inputs(output_root, session_id, stems)
    }

    fn start_with_inputs(
        output_root: impl AsRef<Path>,
        session_id: SessionId,
        stems: Vec<RecorderStemInput>,
    ) -> Result<Self, RecorderError> {
        let session_dir = output_root
            .as_ref()
            .join(format!("session-{}", session_id.0));
        if session_dir.exists() {
            return Err(RecorderError::OutputExists(session_dir));
        }
        let mut configs = Vec::with_capacity(stems.len());
        for stem in &stems {
            let config = &stem.config;
            if config.session_id != session_id {
                return Err(RecorderError::SessionMismatch {
                    label: config.label.as_str().to_owned(),
                    actual: config.session_id.0,
                    expected: session_id.0,
                });
            }
            if config.permission == PermissionDecision::Denied {
                return Err(RecorderError::PermissionDenied(
                    config.label.as_str().to_owned(),
                ));
            }
            if config.sample_rate_hz == 0 || config.channels == 0 {
                return Err(RecorderError::InvalidSampleSpec {
                    label: config.label.as_str().to_owned(),
                    sample_rate_hz: config.sample_rate_hz,
                    channels: config.channels,
                });
            }
            if configs
                .iter()
                .any(|existing: &RecorderStemConfig| existing.label == config.label)
            {
                return Err(RecorderError::DuplicateStemLabel(
                    config.label.as_str().to_owned(),
                ));
            }
            configs.push(config.clone());
        }

        fs::create_dir_all(output_root.as_ref())?;
        fs::create_dir(&session_dir)?;
        fs::create_dir(session_dir.join("stems"))?;
        fs::create_dir(session_dir.join("events"))?;
        fs::create_dir(session_dir.join("metrics"))?;

        write_permission_events(&session_dir, &configs)?;
        write_manifest(
            &session_dir,
            &ManifestDocument::initial(session_id, &configs),
        )?;

        let mut workers = Vec::with_capacity(stems.len());
        for stem in stems {
            match RecorderWorker::spawn(session_dir.clone(), stem) {
                Ok(worker) => workers.push(worker),
                Err(error) => {
                    for worker in &workers {
                        worker.request_stop();
                    }
                    for worker in workers {
                        let _ = worker.join();
                    }
                    fs::remove_dir_all(&session_dir)?;
                    return Err(error);
                }
            }
        }

        Ok(Self {
            session_id,
            session_dir,
            configs,
            workers,
            finished: false,
        })
    }

    #[cfg(test)]
    pub fn session_dir(&self) -> &Path {
        &self.session_dir
    }

    pub fn observations(&self) -> RecordingObservations {
        self.workers.iter().map(RecorderWorker::observations).fold(
            RecordingObservations::default(),
            |mut total, current| {
                total.frames_received_total = total
                    .frames_received_total
                    .saturating_add(current.frames_received_total);
                total.frames_written_total = total
                    .frames_written_total
                    .saturating_add(current.frames_written_total);
                total.frames_rejected_total = total
                    .frames_rejected_total
                    .saturating_add(current.frames_rejected_total);
                total.discontinuities_total = total
                    .discontinuities_total
                    .saturating_add(current.discontinuities_total);
                total.failures_total = total.failures_total.saturating_add(current.failures_total);
                total
            },
        )
    }

    pub fn request_stop(&self) {
        for worker in &self.workers {
            worker.request_stop();
        }
    }

    pub fn finish(mut self) -> Result<RecordingOutcome, RecorderError> {
        self.finalize(None)
    }

    #[cfg(test)]
    pub fn cancel(mut self, reason: impl Into<String>) -> Result<RecordingOutcome, RecorderError> {
        self.finalize(Some(reason.into()))
    }

    fn finalize(
        &mut self,
        cancellation_error: Option<String>,
    ) -> Result<RecordingOutcome, RecorderError> {
        for worker in &self.workers {
            worker.request_stop();
        }

        let mut outcomes = Vec::with_capacity(self.workers.len());
        for worker in self.workers.drain(..) {
            outcomes.push(worker.join());
        }

        merge_discontinuity_events(&self.session_dir, &self.configs)?;
        write_destination_metrics(&self.session_dir, &outcomes)?;
        let manifest = ManifestDocument::finished(
            self.session_id,
            &self.configs,
            &outcomes,
            cancellation_error,
        );
        write_manifest(&self.session_dir, &manifest)?;
        write_summary(&self.session_dir, &manifest)?;

        let completed_stems = outcomes
            .iter()
            .filter(|outcome| outcome.report.is_some())
            .count();
        let failed_stems = outcomes.len().saturating_sub(completed_stems);
        let state = manifest.state;
        let stems = outcomes
            .iter()
            .map(|outcome| RecordingStemOutcome {
                label: outcome.label.clone(),
                written_frames: outcome
                    .report
                    .as_ref()
                    .map_or(0, |report| report.written_frames),
                stale_frames: outcome
                    .report
                    .as_ref()
                    .map_or(0, |report| report.stale_frames),
                gap_ranges: outcome
                    .report
                    .as_ref()
                    .map_or_else(Vec::new, |report| report.gap_ranges.clone()),
                error: outcome.error.clone(),
                edge_observations: outcome.observations,
            })
            .collect();
        self.finished = true;
        Ok(RecordingOutcome {
            session_dir: self.session_dir.clone(),
            state,
            completed_stems,
            failed_stems,
            stems,
        })
    }
}

struct RecorderStemInput {
    config: RecorderStemConfig,
    receiver: PlanEdgeReceiver,
    initial_frame: Option<PlanEdgeFrame>,
}

impl Drop for MultistemRecording {
    fn drop(&mut self) {
        if self.finished {
            return;
        }
        for worker in &self.workers {
            worker.request_stop();
        }
        for worker in self.workers.drain(..) {
            let _ = worker.join();
        }
    }
}

struct RecorderWorker {
    label: String,
    stop_requested: Arc<AtomicBool>,
    join_handle: JoinHandle<StemWorkerOutcome>,
    telemetry: Arc<RecorderWorkerTelemetry>,
}

impl RecorderWorker {
    fn spawn(session_dir: PathBuf, stem: RecorderStemInput) -> Result<Self, RecorderError> {
        let RecorderStemInput {
            config,
            receiver,
            initial_frame,
        } = stem;
        let label = config.label.as_str().to_owned();
        let stop_requested = Arc::new(AtomicBool::new(false));
        let worker_stop = Arc::clone(&stop_requested);
        let telemetry = Arc::new(RecorderWorkerTelemetry::default());
        let worker_telemetry = Arc::clone(&telemetry);
        let join_handle = thread::Builder::new()
            .name(format!("pks-recorder-{label}"))
            .spawn(move || {
                run_stem_worker(StemWorkerRuntime {
                    session_dir,
                    config,
                    receiver,
                    stop_requested: worker_stop,
                    telemetry: worker_telemetry,
                    initial_frame,
                })
            })?;
        Ok(Self {
            label,
            stop_requested,
            join_handle,
            telemetry,
        })
    }

    fn request_stop(&self) {
        self.stop_requested.store(true, Ordering::Release);
        self.join_handle.thread().unpark();
    }

    fn join(self) -> StemWorkerOutcome {
        self.join_handle
            .join()
            .unwrap_or_else(|_| StemWorkerOutcome {
                label: self.label.clone(),
                report: None,
                error: Some(RecorderError::WorkerPanicked(self.label).to_string()),
                observations: EdgeObservations {
                    worker_failures_total: 1,
                    ..EdgeObservations::default()
                },
            })
    }

    fn observations(&self) -> RecordingObservations {
        self.telemetry.snapshot()
    }
}

#[derive(Default)]
struct RecorderWorkerTelemetry {
    frames_received_total: AtomicU64,
    frames_written_total: AtomicU64,
    frames_rejected_total: AtomicU64,
    discontinuities_total: AtomicU64,
    failures_total: AtomicU64,
}

impl RecorderWorkerTelemetry {
    fn snapshot(&self) -> RecordingObservations {
        RecordingObservations {
            frames_received_total: self.frames_received_total.load(Ordering::Relaxed),
            frames_written_total: self.frames_written_total.load(Ordering::Relaxed),
            frames_rejected_total: self.frames_rejected_total.load(Ordering::Relaxed),
            discontinuities_total: self.discontinuities_total.load(Ordering::Relaxed),
            failures_total: self.failures_total.load(Ordering::Relaxed),
        }
    }
}

struct StemWorkerOutcome {
    label: String,
    report: Option<StemWorkerReport>,
    error: Option<String>,
    observations: EdgeObservations,
}

#[derive(Debug)]
struct StemWorkerReport {
    first_timestamp_ns: Option<u64>,
    final_timestamp_ns: Option<u64>,
    written_frames: u64,
    silence_filled_samples: u64,
    stale_frames: u64,
    gap_ranges: Vec<DiscontinuityRecord>,
    wav_bytes: u64,
    checksum_fnv1a64: String,
}

struct StemWorkerRuntime {
    session_dir: PathBuf,
    config: RecorderStemConfig,
    receiver: PlanEdgeReceiver,
    stop_requested: Arc<AtomicBool>,
    telemetry: Arc<RecorderWorkerTelemetry>,
    initial_frame: Option<PlanEdgeFrame>,
}

fn run_stem_worker(mut runtime: StemWorkerRuntime) -> StemWorkerOutcome {
    let result = runtime.run();
    if result.is_err() {
        runtime
            .telemetry
            .failures_total
            .fetch_add(1, Ordering::Relaxed);
        runtime.receiver.mark_worker_failure();
    }
    let observations = runtime.receiver.observations();
    match result {
        Ok(Some(report)) => StemWorkerOutcome {
            label: runtime.config.label.as_str().to_owned(),
            report: Some(report),
            error: None,
            observations,
        },
        Ok(None) => StemWorkerOutcome {
            label: runtime.config.label.as_str().to_owned(),
            report: None,
            error: None,
            observations,
        },
        Err(error) => StemWorkerOutcome {
            label: runtime.config.label.as_str().to_owned(),
            report: None,
            error: Some(error.to_string()),
            observations,
        },
    }
}

impl StemWorkerRuntime {
    fn run(&mut self) -> Result<Option<StemWorkerReport>, RecorderError> {
        let wav_path = self
            .session_dir
            .join("stems")
            .join(format!("{}.wav", self.config.label.as_str()));
        let event_path = stem_event_path(&self.session_dir, &self.config.label);
        let mut writer = WavWriter::create(
            &wav_path,
            WavSpec {
                channels: u16::from(self.config.channels),
                sample_rate: self.config.sample_rate_hz,
                bits_per_sample: 32,
                sample_format: WavSampleFormat::Float,
            },
        )?;
        let mut event_writer = BufWriter::new(File::create(event_path)?);
        let mut state = StemWriteState::new(self.config.timeline_mapping.session_origin_ns);

        if let Some(frame) = self.initial_frame.take() {
            observe_and_write_frame(
                &mut state,
                &self.config,
                &frame,
                &mut writer,
                &mut event_writer,
                &self.telemetry,
            )?;
        }

        loop {
            if let Some(frame) = self.receiver.try_recv() {
                observe_and_write_frame(
                    &mut state,
                    &self.config,
                    &frame,
                    &mut writer,
                    &mut event_writer,
                    &self.telemetry,
                )?;
                continue;
            }
            if self.stop_requested.load(Ordering::Acquire) {
                break;
            }
            thread::park_timeout(Duration::from_millis(WORKER_IDLE_WAIT_MS));
        }

        event_writer.flush()?;
        writer.finalize()?;
        let wav_bytes = fs::metadata(&wav_path)?.len();
        let checksum_fnv1a64 = checksum_fnv1a64(&wav_path)?;
        Ok(Some(StemWorkerReport {
            first_timestamp_ns: state.first_timestamp_ns,
            final_timestamp_ns: state.expected_timestamp_ns,
            written_frames: state.written_frames,
            silence_filled_samples: state.silence_filled_samples,
            stale_frames: state.stale_frames,
            gap_ranges: state.gap_ranges,
            wav_bytes,
            checksum_fnv1a64,
        }))
    }
}

fn observe_and_write_frame(
    state: &mut StemWriteState,
    config: &RecorderStemConfig,
    frame: &PlanEdgeFrame,
    writer: &mut WavWriter<BufWriter<File>>,
    event_writer: &mut BufWriter<File>,
    telemetry: &RecorderWorkerTelemetry,
) -> Result<(), RecorderError> {
    telemetry
        .frames_received_total
        .fetch_add(1, Ordering::Relaxed);
    let written_before = state.written_frames;
    let stale_before = state.stale_frames;
    let discontinuities_before = state.gap_ranges.len();
    state.write_frame(config, frame, writer, event_writer)?;
    telemetry.frames_written_total.fetch_add(
        state.written_frames.saturating_sub(written_before),
        Ordering::Relaxed,
    );
    telemetry.frames_rejected_total.fetch_add(
        state.stale_frames.saturating_sub(stale_before),
        Ordering::Relaxed,
    );
    telemetry.discontinuities_total.fetch_add(
        state
            .gap_ranges
            .len()
            .saturating_sub(discontinuities_before) as u64,
        Ordering::Relaxed,
    );
    Ok(())
}

struct StemWriteState {
    expected_timestamp_ns: Option<u64>,
    expected_sequence: Option<u64>,
    first_timestamp_ns: Option<u64>,
    written_frames: u64,
    silence_filled_samples: u64,
    stale_frames: u64,
    gap_ranges: Vec<DiscontinuityRecord>,
}

impl StemWriteState {
    fn new(_session_origin_ns: u64) -> Self {
        Self {
            // The first real frame establishes this stem's recording origin.
            // Session start may precede physical capture permission/startup by
            // seconds; manufacturing that interval as PCM silence is not a
            // source discontinuity.
            expected_timestamp_ns: None,
            expected_sequence: None,
            first_timestamp_ns: None,
            written_frames: 0,
            silence_filled_samples: 0,
            stale_frames: 0,
            gap_ranges: Vec::new(),
        }
    }

    fn write_frame(
        &mut self,
        config: &RecorderStemConfig,
        frame: &PlanEdgeFrame,
        writer: &mut WavWriter<BufWriter<File>>,
        event_writer: &mut BufWriter<File>,
    ) -> Result<(), RecorderError> {
        validate_frame_lineage(config, frame)?;
        if frame.source_id() != config.source_id {
            return Err(RecorderError::SourceMismatch {
                label: config.label.as_str().to_owned(),
                actual: frame.source_id().0,
                expected: config.source_id.0,
            });
        }
        if frame.sample_rate_hz() != config.sample_rate_hz || frame.channels() != config.channels {
            return Err(RecorderError::FrameSpecMismatch {
                label: config.label.as_str().to_owned(),
                actual_rate_hz: frame.sample_rate_hz(),
                actual_channels: frame.channels(),
                expected_rate_hz: config.sample_rate_hz,
                expected_channels: config.channels,
            });
        }
        let channel_count = usize::from(config.channels);
        if channel_count == 0 || !frame.samples().len().is_multiple_of(channel_count) {
            return Err(RecorderError::UnalignedSamples(
                config.label.as_str().to_owned(),
            ));
        }
        let normalized_timestamp_ns = config
            .timeline_mapping
            .normalize_timestamp_ns(frame.timestamp_ns())
            .ok_or_else(|| RecorderError::TimestampOutOfRange(config.label.as_str().to_owned()))?;
        let frame_samples_per_channel = frame.samples().len() / channel_count;
        let frame_duration_ns = duration_ns(frame_samples_per_channel, config.sample_rate_hz);

        let sequence_is_continuous = self
            .expected_sequence
            .is_none_or(|expected| frame.sequence_number() == expected);
        if let Some(expected_sequence) = self.expected_sequence {
            if frame.sequence_number() > expected_sequence {
                self.record_gap(
                    config,
                    DiscontinuityKind::SequenceGap,
                    normalized_timestamp_ns,
                    normalized_timestamp_ns,
                    Some(expected_sequence),
                    Some(frame.sequence_number().saturating_sub(1)),
                    event_writer,
                )?;
            }
        }

        let effective_timestamp_ns = if sequence_is_continuous {
            // A continuous source sequence is contiguous PCM authority. Capture
            // host timestamps routinely carry sub-frame scheduling jitter and
            // integer-nanosecond rounding; snapping them avoids fake silence,
            // overlap rejection, and an unbounded discontinuity ledger.
            self.expected_timestamp_ns
                .unwrap_or(normalized_timestamp_ns)
        } else {
            normalized_timestamp_ns
        };

        if !sequence_is_continuous {
            if let Some(expected_timestamp_ns) = self.expected_timestamp_ns {
                if effective_timestamp_ns < expected_timestamp_ns {
                    self.stale_frames = self.stale_frames.saturating_add(1);
                    self.record_gap(
                        config,
                        DiscontinuityKind::OverlapRejected,
                        effective_timestamp_ns,
                        expected_timestamp_ns,
                        Some(frame.sequence_number()),
                        Some(frame.sequence_number()),
                        event_writer,
                    )?;
                    return Ok(());
                }
                if effective_timestamp_ns > expected_timestamp_ns {
                    let gap_duration_ns = effective_timestamp_ns - expected_timestamp_ns;
                    if gap_duration_ns > MAX_SILENCE_GAP_NS {
                        return Err(RecorderError::GapTooLarge {
                            label: config.label.as_str().to_owned(),
                            duration_ns: gap_duration_ns,
                        });
                    }
                    let silence_samples = samples_for_duration_ns(
                        gap_duration_ns,
                        config.sample_rate_hz,
                        config.channels,
                    );
                    for _ in 0..silence_samples {
                        writer.write_sample(0.0f32)?;
                    }
                    self.silence_filled_samples = self
                        .silence_filled_samples
                        .saturating_add(silence_samples as u64);
                    self.record_gap(
                        config,
                        DiscontinuityKind::TimestampGap,
                        expected_timestamp_ns,
                        effective_timestamp_ns,
                        self.expected_sequence,
                        frame.sequence_number().checked_sub(1),
                        event_writer,
                    )?;
                }
            }
        }

        for sample in frame.samples() {
            writer.write_sample(*sample)?;
        }
        self.first_timestamp_ns
            .get_or_insert(effective_timestamp_ns);
        self.expected_timestamp_ns = Some(
            effective_timestamp_ns
                .checked_add(frame_duration_ns)
                .ok_or_else(|| {
                    RecorderError::TimestampOutOfRange(config.label.as_str().to_owned())
                })?,
        );
        self.expected_sequence = frame.sequence_number().checked_add(1);
        self.written_frames = self.written_frames.saturating_add(1);
        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    fn record_gap(
        &mut self,
        config: &RecorderStemConfig,
        kind: DiscontinuityKind,
        timestamp_start_ns: u64,
        timestamp_end_ns: u64,
        sequence_start: Option<u64>,
        sequence_end: Option<u64>,
        event_writer: &mut BufWriter<File>,
    ) -> Result<(), RecorderError> {
        if self.gap_ranges.len() >= MAX_MANIFEST_GAPS {
            return Err(RecorderError::TooManyGaps(config.label.as_str().to_owned()));
        }
        let record = DiscontinuityRecord {
            stem_id: config.stem_id.0,
            label: config.label.as_str().to_owned(),
            kind,
            timestamp_start_ns,
            timestamp_end_ns,
            sequence_start,
            sequence_end,
        };
        serde_json::to_writer(&mut *event_writer, &record)?;
        event_writer.write_all(b"\n")?;
        self.gap_ranges.push(record);
        Ok(())
    }
}

fn validate_frame_lineage(
    config: &RecorderStemConfig,
    frame: &PlanEdgeFrame,
) -> Result<(), RecorderError> {
    let lineage = frame.lineage();
    validate_lineage_field(
        config,
        RecorderLineageField::Session,
        lineage.session_id.0,
        config.session_id.0,
    )?;
    validate_lineage_field(
        config,
        RecorderLineageField::Source,
        lineage.source_id.0,
        config.source_id.0,
    )?;
    validate_lineage_field(
        config,
        RecorderLineageField::Stem,
        lineage.stem_id.0,
        config.stem_id.0,
    )?;
    validate_lineage_field(
        config,
        RecorderLineageField::Clock,
        u64::from(lineage.clock_id.0),
        u64::from(config.clock_id.0),
    )?;
    validate_lineage_field(
        config,
        RecorderLineageField::SourceGeneration,
        u64::from(lineage.source_generation),
        u64::from(config.source_generation),
    )?;
    validate_lineage_field(
        config,
        RecorderLineageField::PermissionEpoch,
        lineage.permission_epoch,
        config.permission_epoch,
    )
}

fn validate_lineage_field(
    config: &RecorderStemConfig,
    field: RecorderLineageField,
    actual: u64,
    expected: u64,
) -> Result<(), RecorderError> {
    if actual == expected {
        return Ok(());
    }
    Err(RecorderError::LineageMismatch {
        label: config.label.as_str().to_owned(),
        field,
        actual,
        expected,
    })
}

fn duration_ns(samples_per_channel: usize, sample_rate_hz: u32) -> u64 {
    (samples_per_channel as u128)
        .saturating_mul(1_000_000_000)
        .checked_div(u128::from(sample_rate_hz))
        .unwrap_or(0)
        .min(u128::from(u64::MAX)) as u64
}

fn samples_for_duration_ns(duration_ns: u64, sample_rate_hz: u32, channels: u8) -> usize {
    let sample_frames = u128::from(duration_ns)
        .saturating_mul(u128::from(sample_rate_hz))
        .checked_div(1_000_000_000)
        .unwrap_or(0);
    sample_frames
        .saturating_mul(u128::from(channels))
        .min(usize::MAX as u128) as usize
}

fn checksum_fnv1a64(path: &Path) -> Result<String, RecorderError> {
    let mut reader = BufReader::new(File::open(path)?);
    let mut checksum = 0xcbf2_9ce4_8422_2325u64;
    let mut buffer = [0u8; 8_192];
    loop {
        let read_bytes = reader.read(&mut buffer)?;
        if read_bytes == 0 {
            break;
        }
        for byte in &buffer[..read_bytes] {
            checksum ^= u64::from(*byte);
            checksum = checksum.wrapping_mul(0x0000_0100_0000_01b3);
        }
    }
    Ok(format!("{checksum:016x}"))
}

#[derive(Serialize)]
struct PermissionEvent {
    session_id: u64,
    source_id: u64,
    stem_id: u64,
    permission_epoch: u64,
    scope: PermissionScope,
    decision: PermissionDecision,
}

fn write_permission_events(
    session_dir: &Path,
    configs: &[RecorderStemConfig],
) -> Result<(), RecorderError> {
    let mut writer = BufWriter::new(File::create(
        session_dir.join("events").join("permissions.jsonl"),
    )?);
    for config in configs {
        serde_json::to_writer(
            &mut writer,
            &PermissionEvent {
                session_id: config.session_id.0,
                source_id: config.source_id.0,
                stem_id: config.stem_id.0,
                permission_epoch: config.permission_epoch,
                scope: config.permission_scope,
                decision: config.permission,
            },
        )?;
        writer.write_all(b"\n")?;
    }
    writer.flush()?;
    Ok(())
}

fn stem_event_path(session_dir: &Path, label: &StemLabel) -> PathBuf {
    session_dir
        .join("events")
        .join(format!(".discontinuities-{}.jsonl", label.as_str()))
}

fn merge_discontinuity_events(
    session_dir: &Path,
    configs: &[RecorderStemConfig],
) -> Result<(), RecorderError> {
    let mut output = BufWriter::new(File::create(
        session_dir.join("events").join("discontinuities.jsonl"),
    )?);
    for config in configs {
        let path = stem_event_path(session_dir, &config.label);
        if !path.exists() {
            continue;
        }
        {
            let mut input = BufReader::new(File::open(&path)?);
            std::io::copy(&mut input, &mut output)?;
        }
        fs::remove_file(path)?;
    }
    output.flush()?;
    Ok(())
}

#[derive(Serialize)]
struct DestinationMetrics {
    label: String,
    enqueue_to_receive_latency_boundary: &'static str,
    source_timestamp_latency_boundary: &'static str,
    queue_capacity_frames: u64,
    queue_depth_frames: u64,
    queue_peak_frames: u64,
    frames_enqueued_total: u64,
    frames_delivered_total: u64,
    frames_dropped_total: u64,
    frames_attempted_total: u64,
    drop_rate_pct: f64,
    overruns_total: u64,
    receiver_unavailable_drops_total: u64,
    queue_full_drops_total: u64,
    shared_reference_exhausted_drops_total: u64,
    branch_pool_exhausted_drops_total: u64,
    invalid_copy_policy_drops_total: u64,
    freeze_failed_drops_total: u64,
    discontinuities_total: u64,
    source_identity_discontinuities_total: u64,
    sequence_discontinuities_total: u64,
    timestamp_discontinuities_total: u64,
    manually_reported_discontinuities_total: u64,
    enqueue_to_receive_samples_total: u64,
    enqueue_to_receive_invalid_order_total: u64,
    enqueue_to_receive_p50_ns: u64,
    enqueue_to_receive_p95_ns: u64,
    enqueue_to_receive_p99_ns: u64,
    enqueue_to_receive_max_ns: u64,
    source_timestamp_to_receive_samples_total: u64,
    source_timestamp_to_receive_missing_total: u64,
    source_timestamp_to_receive_future_total: u64,
    source_timestamp_to_receive_p50_ns: u64,
    source_timestamp_to_receive_p95_ns: u64,
    source_timestamp_to_receive_p99_ns: u64,
    source_timestamp_to_receive_max_ns: u64,
    worker_failures_total: u64,
    shutdown_discarded_total: u64,
}

impl DestinationMetrics {
    fn from_observations(label: String, observations: EdgeObservations) -> Self {
        let frames_attempted_total = observations.frames_attempted_total();
        let drop_rate_pct = observations.drop_rate_pct();
        Self {
            label,
            enqueue_to_receive_latency_boundary: "edge-enqueue-to-receiver-pop",
            source_timestamp_latency_boundary: "frame-source-timestamp-to-edge-receive",
            queue_capacity_frames: observations.queue_capacity_frames,
            queue_depth_frames: observations.queue_depth_frames,
            queue_peak_frames: observations.queue_peak_frames,
            frames_enqueued_total: observations.frames_enqueued_total,
            frames_delivered_total: observations.frames_delivered_total,
            frames_dropped_total: observations.frames_dropped_total,
            frames_attempted_total,
            drop_rate_pct,
            overruns_total: observations.overruns_total,
            receiver_unavailable_drops_total: observations.receiver_unavailable_drops_total,
            queue_full_drops_total: observations.queue_full_drops_total,
            shared_reference_exhausted_drops_total: observations
                .shared_reference_exhausted_drops_total,
            branch_pool_exhausted_drops_total: observations.branch_pool_exhausted_drops_total,
            invalid_copy_policy_drops_total: observations.invalid_copy_policy_drops_total,
            freeze_failed_drops_total: observations.freeze_failed_drops_total,
            discontinuities_total: observations.discontinuities_total,
            source_identity_discontinuities_total: observations
                .source_identity_discontinuities_total,
            sequence_discontinuities_total: observations.sequence_discontinuities_total,
            timestamp_discontinuities_total: observations.timestamp_discontinuities_total,
            manually_reported_discontinuities_total: observations
                .manually_reported_discontinuities_total,
            enqueue_to_receive_samples_total: observations.enqueue_to_receive_samples_total,
            enqueue_to_receive_invalid_order_total: observations
                .enqueue_to_receive_invalid_order_total,
            enqueue_to_receive_p50_ns: observations.enqueue_to_receive_p50_ns,
            enqueue_to_receive_p95_ns: observations.enqueue_to_receive_p95_ns,
            enqueue_to_receive_p99_ns: observations.enqueue_to_receive_p99_ns,
            enqueue_to_receive_max_ns: observations.enqueue_to_receive_max_ns,
            source_timestamp_to_receive_samples_total: observations
                .source_timestamp_to_receive_samples_total,
            source_timestamp_to_receive_missing_total: observations
                .source_timestamp_to_receive_missing_total,
            source_timestamp_to_receive_future_total: observations
                .source_timestamp_to_receive_future_total,
            source_timestamp_to_receive_p50_ns: observations.source_timestamp_to_receive_p50_ns,
            source_timestamp_to_receive_p95_ns: observations.source_timestamp_to_receive_p95_ns,
            source_timestamp_to_receive_p99_ns: observations.source_timestamp_to_receive_p99_ns,
            source_timestamp_to_receive_max_ns: observations.source_timestamp_to_receive_max_ns,
            worker_failures_total: observations.worker_failures_total,
            shutdown_discarded_total: observations.shutdown_discarded_total,
        }
    }
}

fn write_destination_metrics(
    session_dir: &Path,
    outcomes: &[StemWorkerOutcome],
) -> Result<(), RecorderError> {
    let mut writer = BufWriter::new(File::create(
        session_dir.join("metrics").join("destinations.jsonl"),
    )?);
    for outcome in outcomes {
        serde_json::to_writer(
            &mut writer,
            &DestinationMetrics::from_observations(outcome.label.clone(), outcome.observations),
        )?;
        writer.write_all(b"\n")?;
    }
    writer.flush()?;
    Ok(())
}

#[derive(Serialize)]
struct ManifestDocument {
    schema_version: u32,
    session_id: u64,
    state: RecordingState,
    stems: Vec<ManifestStem>,
    errors: Vec<String>,
}

impl ManifestDocument {
    fn initial(session_id: SessionId, configs: &[RecorderStemConfig]) -> Self {
        Self {
            schema_version: MANIFEST_SCHEMA_VERSION,
            session_id: session_id.0,
            state: RecordingState::Recording,
            stems: configs
                .iter()
                .map(|config| ManifestStem::new(config, RecordingState::Recording))
                .collect(),
            errors: Vec::new(),
        }
    }

    fn finished(
        session_id: SessionId,
        configs: &[RecorderStemConfig],
        outcomes: &[StemWorkerOutcome],
        cancellation_error: Option<String>,
    ) -> Self {
        let mut errors: Vec<String> = cancellation_error.into_iter().collect();
        let stems = configs
            .iter()
            .zip(outcomes)
            .map(|(config, outcome)| match &outcome.report {
                Some(report) => ManifestStem::from_report(config, report),
                None => {
                    let message = outcome
                        .error
                        .clone()
                        .unwrap_or_else(|| "recorder worker failed without an error".to_owned());
                    errors.push(message.clone());
                    ManifestStem::failed(config, message)
                }
            })
            .collect();
        Self {
            schema_version: MANIFEST_SCHEMA_VERSION,
            session_id: session_id.0,
            state: if errors.is_empty() {
                RecordingState::Complete
            } else {
                RecordingState::Incomplete
            },
            stems,
            errors,
        }
    }
}

#[derive(Serialize)]
struct ManifestStem {
    label: String,
    wav_path: String,
    session_id: u64,
    source_id: u64,
    stem_id: u64,
    clock_id: u32,
    source_generation: u32,
    permission_epoch: u64,
    source_timeline_origin_ns: u64,
    session_timeline_origin_ns: u64,
    sample_rate_hz: u32,
    channels: u8,
    sample_format: &'static str,
    finalization_state: RecordingState,
    first_timestamp_ns: Option<u64>,
    final_timestamp_ns: Option<u64>,
    written_frames: u64,
    silence_filled_samples: u64,
    stale_frames: u64,
    gap_ranges: Vec<DiscontinuityRecord>,
    wav_bytes: Option<u64>,
    checksum_algorithm: &'static str,
    checksum: Option<String>,
    error: Option<String>,
}

impl ManifestStem {
    fn new(config: &RecorderStemConfig, state: RecordingState) -> Self {
        Self {
            label: config.label.as_str().to_owned(),
            wav_path: format!("stems/{}.wav", config.label.as_str()),
            session_id: config.session_id.0,
            source_id: config.source_id.0,
            stem_id: config.stem_id.0,
            clock_id: config.clock_id.0,
            source_generation: config.source_generation,
            permission_epoch: config.permission_epoch,
            source_timeline_origin_ns: config.timeline_mapping.source_origin_ns,
            session_timeline_origin_ns: config.timeline_mapping.session_origin_ns,
            sample_rate_hz: config.sample_rate_hz,
            channels: config.channels,
            sample_format: "f32_interleaved",
            finalization_state: state,
            first_timestamp_ns: None,
            final_timestamp_ns: None,
            written_frames: 0,
            silence_filled_samples: 0,
            stale_frames: 0,
            gap_ranges: Vec::new(),
            wav_bytes: None,
            checksum_algorithm: "fnv1a64",
            checksum: None,
            error: None,
        }
    }

    fn from_report(config: &RecorderStemConfig, report: &StemWorkerReport) -> Self {
        let mut manifest = Self::new(config, RecordingState::Complete);
        manifest.first_timestamp_ns = report.first_timestamp_ns;
        manifest.final_timestamp_ns = report.final_timestamp_ns;
        manifest.written_frames = report.written_frames;
        manifest.silence_filled_samples = report.silence_filled_samples;
        manifest.stale_frames = report.stale_frames;
        manifest.gap_ranges.clone_from(&report.gap_ranges);
        manifest.wav_bytes = Some(report.wav_bytes);
        manifest.checksum = Some(report.checksum_fnv1a64.clone());
        manifest
    }

    fn failed(config: &RecorderStemConfig, error: String) -> Self {
        let mut manifest = Self::new(config, RecordingState::Incomplete);
        manifest.error = Some(error);
        manifest
    }
}

#[derive(Serialize)]
struct RecordingSummary {
    session_id: u64,
    state: RecordingState,
    completed_stems: usize,
    failed_stems: usize,
}

fn write_summary(session_dir: &Path, manifest: &ManifestDocument) -> Result<(), RecorderError> {
    let completed_stems = manifest
        .stems
        .iter()
        .filter(|stem| stem.finalization_state == RecordingState::Complete)
        .count();
    write_json_atomic(
        &session_dir.join("metrics").join("summary.json"),
        &RecordingSummary {
            session_id: manifest.session_id,
            state: manifest.state,
            completed_stems,
            failed_stems: manifest.stems.len().saturating_sub(completed_stems),
        },
    )
}

fn write_manifest(session_dir: &Path, manifest: &ManifestDocument) -> Result<(), RecorderError> {
    write_json_atomic(&session_dir.join("manifest.json"), manifest)
}

fn write_json_atomic(path: &Path, value: &impl Serialize) -> Result<(), RecorderError> {
    let temporary_path = path.with_extension("json.next");
    let file = File::create(&temporary_path)?;
    let mut writer = BufWriter::new(file);
    serde_json::to_writer_pretty(&mut writer, value)?;
    writer.write_all(b"\n")?;
    writer.flush()?;
    writer.get_ref().sync_all()?;
    fs::rename(temporary_path, path)?;
    Ok(())
}

#[cfg(test)]
mod tests;
