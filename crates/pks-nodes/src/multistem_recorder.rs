use std::fmt;
use std::fs::{self, File};
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::Duration;

use hound::{SampleFormat as WavSampleFormat, WavSpec, WavWriter};
use pks_endpoint::EndpointStartGate;
use pks_frame::{ClockDomainId, SessionId, SourceId, StemId};
use pks_runtime::{EdgeObservations, PlanEdgeFrame, PlanEdgeReceiver};
use pks_timing::TimelineMapping;
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
    #[error("recording publication failed: {0}")]
    Publication(String),
    #[error("recording cannot roll back after its start gate opened")]
    RollbackAfterStart,
    #[error(transparent)]
    Rollback(#[from] RecordingRollbackFailure),
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("WAV error: {0}")]
    Wav(#[from] hound::Error),
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
}

#[derive(Debug)]
pub struct RecordingRollbackFailure {
    worker_failures: Vec<RecordingRollbackWorkerFailure>,
    cleanup_failure: Option<std::io::Error>,
}

impl RecordingRollbackFailure {
    pub fn worker_failures(&self) -> &[RecordingRollbackWorkerFailure] {
        &self.worker_failures
    }

    pub const fn cleanup_failure(&self) -> Option<&std::io::Error> {
        self.cleanup_failure.as_ref()
    }
}

impl fmt::Display for RecordingRollbackFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "recording rollback failed for {} worker(s)",
            self.worker_failures.len()
        )?;
        if let Some(cleanup_failure) = &self.cleanup_failure {
            write!(
                formatter,
                "; pending artifact cleanup failed: {cleanup_failure}"
            )?;
        }
        Ok(())
    }
}

impl std::error::Error for RecordingRollbackFailure {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.cleanup_failure
            .as_ref()
            .map(|failure| failure as &(dyn std::error::Error + 'static))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordingRollbackWorkerFailure {
    label: String,
    error: String,
}

impl RecordingRollbackWorkerFailure {
    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn error(&self) -> &str {
        &self.error
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StemLabel(String);

impl StemLabel {
    pub fn new(value: impl Into<String>) -> Result<Self, RecorderError> {
        let value = value.into();
        let valid = !value.is_empty()
            && value.len() <= 64
            && value.bytes().all(|byte| {
                byte.is_ascii_lowercase() || byte.is_ascii_digit() || b"-_".contains(&byte)
            });
        if !valid {
            return Err(RecorderError::InvalidStemLabel(value));
        }
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PermissionDecision {
    Allowed,
    Denied,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PermissionScope {
    SessionCaptureGrant,
}

#[derive(Debug, Clone)]
pub struct RecorderStemConfig {
    pub session_id: SessionId,
    pub source_id: SourceId,
    pub stem_id: StemId,
    pub clock_id: ClockDomainId,
    pub source_generation: u32,
    pub permission_epoch: u64,
    pub permission_scope: PermissionScope,
    pub permission: PermissionDecision,
    pub label: StemLabel,
    pub sample_rate_hz: u32,
    pub channels: u8,
    pub timeline_mapping: TimelineMapping,
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
    pending_session_dir: Option<PathBuf>,
    start_gate: Option<Arc<EndpointStartGate>>,
    configs: Vec<RecorderStemConfig>,
    workers: Vec<RecorderWorker>,
    finished: bool,
}

impl MultistemRecording {
    pub fn start(
        output_root: impl AsRef<Path>,
        session_id: SessionId,
        stems: Vec<(RecorderStemConfig, PlanEdgeReceiver)>,
    ) -> Result<Self, RecorderError> {
        Self::start_with_gate(output_root, session_id, stems, None)
    }

    pub(crate) fn start_gated(
        output_root: impl AsRef<Path>,
        session_id: SessionId,
        stems: Vec<(RecorderStemConfig, PlanEdgeReceiver)>,
        start_gate: Arc<EndpointStartGate>,
    ) -> Result<Self, RecorderError> {
        Self::start_with_gate(output_root, session_id, stems, Some(start_gate))
    }

    fn start_with_gate(
        output_root: impl AsRef<Path>,
        session_id: SessionId,
        stems: Vec<(RecorderStemConfig, PlanEdgeReceiver)>,
        start_gate: Option<Arc<EndpointStartGate>>,
    ) -> Result<Self, RecorderError> {
        let session_dir = output_root
            .as_ref()
            .join(format!("session-{}", session_id.0));
        if session_dir.exists() {
            return Err(RecorderError::OutputExists(session_dir));
        }
        let pending_session_dir = start_gate.as_ref().map(|_| {
            output_root
                .as_ref()
                .join(format!(".session-{}.pending", session_id.0))
        });
        if let Some(pending_session_dir) = &pending_session_dir {
            if pending_session_dir.exists() {
                return Err(RecorderError::OutputExists(pending_session_dir.clone()));
            }
        }
        let working_session_dir = pending_session_dir.as_ref().unwrap_or(&session_dir).clone();
        let publication = pending_session_dir.as_ref().map(|pending_session_dir| {
            Arc::new(RecordingPublication::new(
                pending_session_dir.clone(),
                session_dir.clone(),
            ))
        });

        let mut configs = Vec::with_capacity(stems.len());
        for (config, _) in &stems {
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
        fs::create_dir(&working_session_dir)?;
        fs::create_dir(working_session_dir.join("stems"))?;
        fs::create_dir(working_session_dir.join("events"))?;
        fs::create_dir(working_session_dir.join("metrics"))?;

        write_permission_events(&working_session_dir, &configs)?;
        write_manifest(
            &working_session_dir,
            &ManifestDocument::initial(session_id, &configs),
        )?;

        let mut workers = Vec::with_capacity(stems.len());
        for (config, receiver) in stems {
            match RecorderWorker::spawn(
                session_dir.clone(),
                config,
                receiver,
                start_gate.clone(),
                publication.clone(),
            ) {
                Ok(worker) => workers.push(worker),
                Err(error) => {
                    for worker in &workers {
                        worker.request_stop();
                    }
                    for worker in workers {
                        let _ = worker.join();
                    }
                    if let Some(pending_session_dir) = &pending_session_dir {
                        fs::remove_dir_all(pending_session_dir)?;
                    }
                    return Err(error);
                }
            }
        }

        Ok(Self {
            session_id,
            session_dir,
            pending_session_dir,
            start_gate,
            configs,
            workers,
            finished: false,
        })
    }

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

    pub(crate) fn rollback_before_start(mut self) -> Result<RecordingObservations, RecorderError> {
        if self
            .start_gate
            .as_ref()
            .is_some_and(|start_gate| start_gate.is_open())
        {
            return Err(RecorderError::RollbackAfterStart);
        }
        let observations = self.observations();
        self.request_stop();
        let worker_failures = self
            .workers
            .drain(..)
            .filter_map(|worker| {
                let outcome = worker.join();
                outcome.error.map(|error| RecordingRollbackWorkerFailure {
                    label: outcome.label,
                    error,
                })
            })
            .collect::<Vec<_>>();
        let cleanup_failure = self
            .pending_session_dir
            .as_ref()
            .and_then(|pending_session_dir| fs::remove_dir_all(pending_session_dir).err());
        self.finished = true;
        if worker_failures.is_empty() && cleanup_failure.is_none() {
            Ok(observations)
        } else {
            Err(RecorderError::Rollback(RecordingRollbackFailure {
                worker_failures,
                cleanup_failure,
            }))
        }
    }

    pub fn finish(mut self) -> Result<RecordingOutcome, RecorderError> {
        self.finalize(None)
    }

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
        if let Some(pending_session_dir) = &self.pending_session_dir {
            let _ = fs::remove_dir_all(pending_session_dir);
        }
    }
}

struct RecordingPublication {
    pending_session_dir: PathBuf,
    session_dir: PathBuf,
    result: Mutex<Option<Result<(), String>>>,
}

impl RecordingPublication {
    fn new(pending_session_dir: PathBuf, session_dir: PathBuf) -> Self {
        Self {
            pending_session_dir,
            session_dir,
            result: Mutex::new(None),
        }
    }

    fn publish(&self) -> Result<(), RecorderError> {
        let mut result = self
            .result
            .lock()
            .map_err(|_| RecorderError::Publication("publication lock was poisoned".to_owned()))?;
        if result.is_none() {
            *result = Some(
                fs::rename(&self.pending_session_dir, &self.session_dir)
                    .map_err(|error| error.to_string()),
            );
        }
        match result.as_ref() {
            Some(Ok(())) => Ok(()),
            Some(Err(error)) => Err(RecorderError::Publication(error.clone())),
            None => Err(RecorderError::Publication(
                "publication result was not recorded".to_owned(),
            )),
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
    fn spawn(
        session_dir: PathBuf,
        config: RecorderStemConfig,
        receiver: PlanEdgeReceiver,
        start_gate: Option<Arc<EndpointStartGate>>,
        publication: Option<Arc<RecordingPublication>>,
    ) -> Result<Self, RecorderError> {
        let label = config.label.as_str().to_owned();
        let stop_requested = Arc::new(AtomicBool::new(false));
        let worker_stop = Arc::clone(&stop_requested);
        let telemetry = Arc::new(RecorderWorkerTelemetry::default());
        let worker_telemetry = Arc::clone(&telemetry);
        let join_handle = thread::Builder::new()
            .name(format!("pks-recorder-{label}"))
            .spawn(move || {
                run_stem_worker(
                    &session_dir,
                    config,
                    receiver,
                    worker_stop,
                    start_gate,
                    publication,
                    worker_telemetry,
                )
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

fn run_stem_worker(
    session_dir: &Path,
    config: RecorderStemConfig,
    mut receiver: PlanEdgeReceiver,
    stop_requested: Arc<AtomicBool>,
    start_gate: Option<Arc<EndpointStartGate>>,
    publication: Option<Arc<RecordingPublication>>,
    telemetry: Arc<RecorderWorkerTelemetry>,
) -> StemWorkerOutcome {
    let result = run_stem_worker_inner(
        session_dir,
        &config,
        &mut receiver,
        &stop_requested,
        start_gate.as_deref(),
        publication.as_deref(),
        &telemetry,
    );
    if result.is_err() {
        telemetry.failures_total.fetch_add(1, Ordering::Relaxed);
        receiver.mark_worker_failure();
    }
    let observations = receiver.observations();
    match result {
        Ok(Some(report)) => StemWorkerOutcome {
            label: config.label.as_str().to_owned(),
            report: Some(report),
            error: None,
            observations,
        },
        Ok(None) => StemWorkerOutcome {
            label: config.label.as_str().to_owned(),
            report: None,
            error: None,
            observations,
        },
        Err(error) => StemWorkerOutcome {
            label: config.label.as_str().to_owned(),
            report: None,
            error: Some(error.to_string()),
            observations,
        },
    }
}

fn run_stem_worker_inner(
    session_dir: &Path,
    config: &RecorderStemConfig,
    receiver: &mut PlanEdgeReceiver,
    stop_requested: &AtomicBool,
    start_gate: Option<&EndpointStartGate>,
    publication: Option<&RecordingPublication>,
    telemetry: &RecorderWorkerTelemetry,
) -> Result<Option<StemWorkerReport>, RecorderError> {
    if let Some(start_gate) = start_gate {
        while !start_gate.is_open() {
            if stop_requested.load(Ordering::Acquire) {
                return Ok(None);
            }
            thread::park_timeout(Duration::from_millis(WORKER_IDLE_WAIT_MS));
        }
        publication
            .ok_or_else(|| {
                RecorderError::Publication(
                    "gated recorder worker is missing publication ownership".to_owned(),
                )
            })?
            .publish()?;
    }
    let wav_path = session_dir
        .join("stems")
        .join(format!("{}.wav", config.label.as_str()));
    let event_path = stem_event_path(session_dir, &config.label);
    let mut writer = WavWriter::create(
        &wav_path,
        WavSpec {
            channels: u16::from(config.channels),
            sample_rate: config.sample_rate_hz,
            bits_per_sample: 32,
            sample_format: WavSampleFormat::Float,
        },
    )?;
    let mut event_writer = BufWriter::new(File::create(event_path)?);
    let mut state = StemWriteState::new(config.timeline_mapping.session_origin_ns);

    loop {
        if start_gate.is_some_and(|gate| !gate.is_open()) {
            if stop_requested.load(Ordering::Acquire) {
                break;
            }
            thread::park_timeout(Duration::from_millis(WORKER_IDLE_WAIT_MS));
            continue;
        }
        if let Some(frame) = receiver.try_recv() {
            telemetry
                .frames_received_total
                .fetch_add(1, Ordering::Relaxed);
            let written_before = state.written_frames;
            let stale_before = state.stale_frames;
            let discontinuities_before = state.gap_ranges.len();
            state.write_frame(config, &frame, &mut writer, &mut event_writer)?;
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
            continue;
        }
        if stop_requested.load(Ordering::Acquire) {
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
    fn new(session_origin_ns: u64) -> Self {
        Self {
            expected_timestamp_ns: Some(session_origin_ns),
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

        if let Some(expected_timestamp_ns) = self.expected_timestamp_ns {
            if normalized_timestamp_ns < expected_timestamp_ns {
                self.stale_frames = self.stale_frames.saturating_add(1);
                self.record_gap(
                    config,
                    DiscontinuityKind::OverlapRejected,
                    normalized_timestamp_ns,
                    expected_timestamp_ns,
                    Some(frame.sequence_number()),
                    Some(frame.sequence_number()),
                    event_writer,
                )?;
                return Ok(());
            }
            if normalized_timestamp_ns > expected_timestamp_ns {
                let gap_duration_ns = normalized_timestamp_ns - expected_timestamp_ns;
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
                    normalized_timestamp_ns,
                    self.expected_sequence,
                    frame.sequence_number().checked_sub(1),
                    event_writer,
                )?;
            }
        }

        for sample in frame.samples() {
            writer.write_sample(*sample)?;
        }
        self.first_timestamp_ns
            .get_or_insert(normalized_timestamp_ns);
        self.expected_timestamp_ns = Some(
            normalized_timestamp_ns
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
mod tests {
    use super::*;
    use pks_endpoint::endpoint_start_gate;
    use pks_frame::{AudioBufferPool, AudioFrame, StreamId};
    use pks_graph::compiler::Compiler;
    use pks_graph::dsl::Pipeline;
    use pks_graph::node::NodeConfig;
    use pks_graph::planner::RuntimePlanner;
    use pks_graph::register_builtins;
    use pks_graph::registry::NodeRegistry;
    use pks_graph::spec::NodeId;
    use pks_runtime::PlanEdgeRouter;
    use tempfile::TempDir;

    const FRAME_SAMPLES: usize = 960;

    fn frame(source_id: u64, sequence_number: u64, timestamp_ns: u64, value: f32) -> AudioFrame {
        let pool = AudioBufferPool::new(1, FRAME_SAMPLES);
        let mut buffer = pool.acquire().unwrap();
        buffer.copy_from_slice(&vec![value; FRAME_SAMPLES]);
        AudioFrame::new(
            StreamId(source_id),
            SourceId(source_id),
            sequence_number,
            timestamp_ns,
            1,
            buffer,
        )
    }

    fn stem_config(
        session_id: u64,
        source_id: u64,
        stem_id: u64,
        clock_id: u32,
        label: &str,
        source_origin_ns: u64,
        session_origin_ns: u64,
    ) -> RecorderStemConfig {
        RecorderStemConfig {
            session_id: SessionId(session_id),
            source_id: SourceId(source_id),
            stem_id: StemId(stem_id),
            clock_id: ClockDomainId(clock_id),
            source_generation: 1,
            permission_epoch: 1,
            permission_scope: PermissionScope::SessionCaptureGrant,
            permission: PermissionDecision::Allowed,
            label: StemLabel::new(label).unwrap(),
            sample_rate_hz: 48_000,
            channels: 1,
            timeline_mapping: TimelineMapping::new(source_origin_ns, session_origin_ns),
        }
    }

    fn router_with_sources(
        source_count: usize,
    ) -> (PlanEdgeRouter, Vec<PlanEdgeReceiver>, Vec<NodeId>) {
        let mut registry = NodeRegistry::new();
        register_builtins(&mut registry);
        let mut graph = Pipeline::new();
        let mut source_ids = Vec::with_capacity(source_count);
        for _ in 0..source_count {
            let source = graph.add_node("passthrough", NodeConfig::new());
            let sink = graph.add_node("passthrough", NodeConfig::new());
            graph.connect(source.out("out"), sink.in_("in"));
            source_ids.push(source.id());
        }
        let ir = Compiler::new()
            .compile(graph.into_spec(), &registry)
            .unwrap();
        let plan = RuntimePlanner::new().plan(&ir).unwrap();
        let (router, receivers) = PlanEdgeRouter::new(&plan, &ir).unwrap();
        (router, receivers, source_ids)
    }

    #[test]
    fn given_fractional_stereo_gap_when_silence_is_sized_then_channels_remain_aligned() {
        let silence_samples = samples_for_duration_ns(190_342_250, 48_000, 2);

        assert_eq!(silence_samples % 2, 0);
        assert_eq!(silence_samples / 2, 9_136);
    }

    #[test]
    fn given_two_clock_mapped_stems_when_finished_then_two_aligned_playable_wavs_are_written() {
        let temp_dir = TempDir::new().unwrap();
        let (mut router, mut receivers, source_nodes) = router_with_sources(2);
        let application_receiver = receivers
            .iter()
            .position(|receiver| receiver.from().node == source_nodes[0])
            .map(|index| receivers.swap_remove(index))
            .unwrap();
        let microphone_receiver = receivers.pop().unwrap();
        let recording = MultistemRecording::start(
            temp_dir.path(),
            SessionId(42),
            vec![
                (
                    stem_config(42, 1, 11, 101, "application", 1_000, 10_000),
                    application_receiver,
                ),
                (
                    stem_config(42, 2, 12, 102, "microphone", 5_000, 10_000),
                    microphone_receiver,
                ),
            ],
        )
        .unwrap();

        router.dispatch_from(source_nodes[0], "out", frame(1, 0, 1_000, 0.25), 10_000);
        router.dispatch_from(source_nodes[1], "out", frame(2, 0, 5_000, -0.5), 10_000);
        let outcome = recording.finish().unwrap();

        assert_eq!(outcome.state, RecordingState::Complete);
        assert_eq!(outcome.completed_stems, 2);
        for label in ["application", "microphone"] {
            let reader = hound::WavReader::open(
                outcome
                    .session_dir
                    .join("stems")
                    .join(format!("{label}.wav")),
            )
            .unwrap();
            assert_eq!(reader.spec().sample_rate, 48_000);
            assert_eq!(reader.spec().channels, 1);
            assert_eq!(reader.duration(), FRAME_SAMPLES as u32);
        }
        let manifest: serde_json::Value =
            serde_json::from_reader(File::open(outcome.session_dir.join("manifest.json")).unwrap())
                .unwrap();
        assert_eq!(manifest["state"], "complete");
        assert_eq!(manifest["stems"][0]["first_timestamp_ns"], 10_000);
        assert_eq!(manifest["stems"][1]["first_timestamp_ns"], 10_000);
        assert!(manifest["stems"][0]["checksum"].as_str().is_some());
        let permission_events =
            fs::read_to_string(outcome.session_dir.join("events").join("permissions.jsonl"))
                .unwrap();
        let first_permission: serde_json::Value =
            serde_json::from_str(permission_events.lines().next().unwrap()).unwrap();
        assert_eq!(first_permission["scope"], "session_capture_grant");
        assert_eq!(first_permission["decision"], "allowed");
    }

    #[test]
    fn given_timestamp_and_sequence_gap_when_finished_then_silence_and_events_preserve_time() {
        let temp_dir = TempDir::new().unwrap();
        let (mut router, mut receivers, source_nodes) = router_with_sources(1);
        let recording = MultistemRecording::start(
            temp_dir.path(),
            SessionId(7),
            vec![(
                stem_config(7, 1, 20, 5, "application", 0, 0),
                receivers.pop().unwrap(),
            )],
        )
        .unwrap();

        router.dispatch_from(source_nodes[0], "out", frame(1, 0, 0, 0.25), 0);
        router.dispatch_from(
            source_nodes[0],
            "out",
            frame(1, 2, 40_000_000, 0.5),
            40_000_000,
        );
        let outcome = recording.finish().unwrap();

        let reader =
            hound::WavReader::open(outcome.session_dir.join("stems").join("application.wav"))
                .unwrap();
        assert_eq!(reader.duration(), (FRAME_SAMPLES * 3) as u32);
        let events = fs::read_to_string(
            outcome
                .session_dir
                .join("events")
                .join("discontinuities.jsonl"),
        )
        .unwrap();
        assert!(events.contains("timestamp_gap"));
        assert!(events.contains("sequence_gap"));
        let manifest: serde_json::Value =
            serde_json::from_reader(File::open(outcome.session_dir.join("manifest.json")).unwrap())
                .unwrap();
        assert_eq!(
            manifest["stems"][0]["silence_filled_samples"],
            FRAME_SAMPLES as u64
        );
    }

    #[test]
    fn given_queued_audio_when_recording_cancelled_then_wav_header_is_playable_and_manifest_incomplete(
    ) {
        let temp_dir = TempDir::new().unwrap();
        let (mut router, mut receivers, source_nodes) = router_with_sources(1);
        let recording = MultistemRecording::start(
            temp_dir.path(),
            SessionId(8),
            vec![(
                stem_config(8, 1, 21, 5, "microphone", 0, 0),
                receivers.pop().unwrap(),
            )],
        )
        .unwrap();
        router.dispatch_from(source_nodes[0], "out", frame(1, 0, 0, 0.25), 0);

        let outcome = recording.cancel("session cancelled by caller").unwrap();

        assert_eq!(outcome.state, RecordingState::Incomplete);
        let reader =
            hound::WavReader::open(outcome.session_dir.join("stems").join("microphone.wav"))
                .unwrap();
        assert_eq!(reader.duration(), FRAME_SAMPLES as u32);
        let manifest: serde_json::Value =
            serde_json::from_reader(File::open(outcome.session_dir.join("manifest.json")).unwrap())
                .unwrap();
        assert_eq!(manifest["state"], "incomplete");
        assert_eq!(manifest["errors"][0], "session cancelled by caller");
    }

    #[test]
    fn given_failed_recorder_branch_when_more_frames_dispatched_then_healthy_branch_continues() {
        let temp_dir = TempDir::new().unwrap();
        let mut registry = NodeRegistry::new();
        register_builtins(&mut registry);
        let mut graph = Pipeline::new();
        let source = graph.add_node("passthrough", NodeConfig::new());
        let recorder_sink = graph.add_node("passthrough", NodeConfig::new());
        let healthy_sink = graph.add_node("passthrough", NodeConfig::new());
        let recorder_edge = graph.connect(source.out("out"), recorder_sink.in_("in"));
        let healthy_edge = graph.connect(source.out("out"), healthy_sink.in_("in"));
        let ir = Compiler::new()
            .compile(graph.into_spec(), &registry)
            .unwrap();
        let plan = RuntimePlanner::new().plan(&ir).unwrap();
        let (mut router, mut receivers) = PlanEdgeRouter::new(&plan, &ir).unwrap();
        let recorder_index = receivers
            .iter()
            .position(|receiver| receiver.edge_id() == recorder_edge)
            .unwrap();
        let recorder_receiver = receivers.swap_remove(recorder_index);
        let healthy_receiver = receivers
            .iter_mut()
            .find(|receiver| receiver.edge_id() == healthy_edge)
            .unwrap();
        let recording = MultistemRecording::start(
            temp_dir.path(),
            SessionId(9),
            vec![(
                stem_config(9, 99, 30, 5, "application", 0, 0),
                recorder_receiver,
            )],
        )
        .unwrap();

        router.dispatch_from(source.id(), "out", frame(1, 0, 0, 0.25), 0);
        assert_eq!(healthy_receiver.try_recv().unwrap().sequence_number(), 0);
        thread::sleep(Duration::from_millis(10));
        router.dispatch_from(source.id(), "out", frame(1, 1, 20_000_000, 0.5), 20_000_000);
        assert_eq!(healthy_receiver.try_recv().unwrap().sequence_number(), 1);
        let outcome = recording.finish().unwrap();

        assert_eq!(outcome.state, RecordingState::Incomplete);
        assert_eq!(outcome.failed_stems, 1);
        let metrics = fs::read_to_string(
            outcome
                .session_dir
                .join("metrics")
                .join("destinations.jsonl"),
        )
        .unwrap();
        let metrics: serde_json::Value = serde_json::from_str(metrics.trim()).unwrap();
        assert_eq!(metrics["worker_failures_total"], 1);
        let manifest: serde_json::Value =
            serde_json::from_reader(File::open(outcome.session_dir.join("manifest.json")).unwrap())
                .unwrap();
        assert_eq!(manifest["state"], "incomplete");
        assert!(manifest["stems"][0]["error"]
            .as_str()
            .unwrap()
            .contains("received source"));
    }

    #[test]
    fn given_multiple_worker_and_cleanup_failures_when_rolled_back_then_all_failures_are_preserved()
    {
        let temp_dir = TempDir::new().unwrap();
        let pending_path = temp_dir.path().join(".session-10.pending");
        fs::write(&pending_path, b"not a directory").unwrap();
        let (_gate_controller, gate) = endpoint_start_gate();
        let failed_worker = RecorderWorker {
            label: "failed".to_owned(),
            stop_requested: Arc::new(AtomicBool::new(false)),
            join_handle: thread::spawn(|| StemWorkerOutcome {
                label: "failed".to_owned(),
                report: None,
                error: Some("deterministic pre-gate failure".to_owned()),
                observations: EdgeObservations::default(),
            }),
            telemetry: Arc::new(RecorderWorkerTelemetry::default()),
        };
        let panicked_worker = RecorderWorker {
            label: "panicked".to_owned(),
            stop_requested: Arc::new(AtomicBool::new(false)),
            join_handle: thread::spawn(|| -> StemWorkerOutcome {
                panic!("deterministic pre-gate panic");
            }),
            telemetry: Arc::new(RecorderWorkerTelemetry::default()),
        };
        let recording = MultistemRecording {
            session_id: SessionId(10),
            session_dir: temp_dir.path().join("session-10"),
            pending_session_dir: Some(pending_path),
            start_gate: Some(gate),
            configs: Vec::new(),
            workers: vec![failed_worker, panicked_worker],
            finished: false,
        };

        let error = recording.rollback_before_start().unwrap_err();

        let RecorderError::Rollback(rollback) = error else {
            panic!("expected typed rollback failure");
        };
        assert_eq!(rollback.worker_failures().len(), 2);
        assert_eq!(rollback.worker_failures()[0].label(), "failed");
        assert_eq!(
            rollback.worker_failures()[0].error(),
            "deterministic pre-gate failure"
        );
        assert_eq!(rollback.worker_failures()[1].label(), "panicked");
        assert!(rollback.worker_failures()[1]
            .error()
            .contains("worker 'panicked' panicked"));
        assert!(rollback.cleanup_failure().is_some());
    }
}
