use std::fs::{File, OpenOptions};
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::mpsc::{self, Receiver, SyncSender, TrySendError};
use std::sync::Arc;
use std::thread::{self, JoinHandle};

use crate::frame::{EndpointId, RouteId, SessionId, StemId};

use crate::session::{
    SessionEvent, SessionEventKind, SessionFinalizationStage, SessionLifecycleState,
    SessionRollbackStage, SessionTerminalState,
};

const TRACE_MAGIC: &[u8; 8] = b"PKSFLT01";
const TRACE_VERSION: u16 = 1;
const HEADER_SIZE_BYTES: usize = 40;
const RECORD_SIZE_BYTES: usize = 88;
const FOOTER_SIZE_BYTES: usize = 64;
const RECORD_MARKER: u32 = 0x5245_4344;
const FOOTER_MARKER: u32 = 0x464F_4F54;
const FNV_OFFSET_BASIS: u64 = 0xcbf2_9ce4_8422_2325;
const FNV_PRIME: u64 = 0x0000_0100_0000_01b3;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SessionFlightRecordKind {
    Lifecycle {
        state: SessionLifecycleState,
    },
    SourceFailure {
        stem_id: StemId,
    },
    EndpointFailure {
        route_id: RouteId,
        endpoint_id: EndpointId,
        stage_code: u8,
    },
    RollbackFailure {
        stage: SessionRollbackStage,
    },
    FinalizationFailure {
        stage: SessionFinalizationStage,
    },
    Terminal {
        state: SessionTerminalState,
        source_failures_total: u64,
        endpoint_failures_total: u64,
        rollback_failures_total: u64,
        finalization_failures_total: u64,
    },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SessionFlightRecord {
    pub sequence_index: u64,
    pub observed_at_ns: u64,
    pub session_id: SessionId,
    pub kind: SessionFlightRecordKind,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum SessionFlightRecordDelivery {
    Enqueued,
    DroppedFull,
    Closed,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SessionFlightRecorderOutcome {
    pub path: PathBuf,
    pub records_attempted_total: u64,
    pub records_enqueued_total: u64,
    pub records_dropped_total: u64,
    pub records_written_total: u64,
    pub rolling_hash: u64,
}

impl SessionFlightRecorderOutcome {
    pub fn is_complete(&self) -> bool {
        self.records_dropped_total == 0
            && self.records_attempted_total == self.records_enqueued_total
            && self.records_enqueued_total == self.records_written_total
    }
}

#[derive(Debug, thiserror::Error)]
pub enum SessionFlightRecorderStartError {
    #[error("flight-recorder capacity must be greater than zero records")]
    ZeroCapacity,
    #[error("flight-recorder output already exists: {path}")]
    OutputExists { path: PathBuf },
    #[error("flight-recorder I/O failed: {0}")]
    Io(#[from] std::io::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum SessionFlightRecorderFinishError {
    #[error("flight-recorder command channel closed before finalization")]
    ChannelClosed,
    #[error("flight-recorder worker panicked")]
    WorkerPanicked,
    #[error("flight-recorder I/O failed: {0}")]
    Io(#[from] std::io::Error),
}

#[derive(Clone, Debug)]
pub struct SessionFlightRecorderHandle {
    sender: SyncSender<RecorderCommand>,
    counters: Arc<RecorderCounters>,
    accepting: Arc<AtomicBool>,
}

impl SessionFlightRecorderHandle {
    pub(crate) fn try_record_event(&self, event: &SessionEvent) -> SessionFlightRecordDelivery {
        if !self.accepting.load(Ordering::Acquire) {
            return SessionFlightRecordDelivery::Closed;
        }
        let sequence_index = self
            .counters
            .records_attempted_total
            .fetch_add(1, Ordering::Relaxed);
        let record = SessionFlightRecord {
            sequence_index,
            observed_at_ns: crate::timing::monotonic_timestamp_ns(),
            session_id: event.session_id(),
            kind: record_kind(event.kind()),
        };
        match self.sender.try_send(RecorderCommand::Record(record)) {
            Ok(()) => {
                self.counters
                    .records_enqueued_total
                    .fetch_add(1, Ordering::Relaxed);
                SessionFlightRecordDelivery::Enqueued
            }
            Err(TrySendError::Full(_)) => {
                self.counters
                    .records_dropped_total
                    .fetch_add(1, Ordering::Relaxed);
                SessionFlightRecordDelivery::DroppedFull
            }
            Err(TrySendError::Disconnected(_)) => {
                self.counters
                    .records_dropped_total
                    .fetch_add(1, Ordering::Relaxed);
                SessionFlightRecordDelivery::Closed
            }
        }
    }
}

pub struct SessionFlightRecorder {
    path: PathBuf,
    handle: SessionFlightRecorderHandle,
    worker: Option<JoinHandle<Result<WriterOutcome, std::io::Error>>>,
    outcome: Option<SessionFlightRecorderOutcome>,
}

impl SessionFlightRecorder {
    pub fn start(
        path: impl Into<PathBuf>,
        session_id: SessionId,
        capacity_records: usize,
    ) -> Result<Self, SessionFlightRecorderStartError> {
        if capacity_records == 0 {
            return Err(SessionFlightRecorderStartError::ZeroCapacity);
        }
        let path = path.into();
        let file = match OpenOptions::new().write(true).create_new(true).open(&path) {
            Ok(file) => file,
            Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {
                return Err(SessionFlightRecorderStartError::OutputExists { path });
            }
            Err(error) => return Err(error.into()),
        };
        let mut writer = BufWriter::new(file);
        write_header(&mut writer, session_id, capacity_records as u64)?;
        let (sender, receiver) = mpsc::sync_channel(capacity_records);
        let counters = Arc::new(RecorderCounters::default());
        let accepting = Arc::new(AtomicBool::new(true));
        let worker_counters = Arc::clone(&counters);
        let worker = thread::Builder::new()
            .name("pks-flight-recorder".to_owned())
            .spawn(move || writer_loop(writer, receiver, worker_counters))?;
        Ok(Self {
            path,
            handle: SessionFlightRecorderHandle {
                sender,
                counters,
                accepting,
            },
            worker: Some(worker),
            outcome: None,
        })
    }

    pub fn handle(&self) -> SessionFlightRecorderHandle {
        self.handle.clone()
    }

    pub fn finish(
        &mut self,
    ) -> Result<&SessionFlightRecorderOutcome, SessionFlightRecorderFinishError> {
        if self.outcome.is_none() {
            self.handle.accepting.store(false, Ordering::Release);
            self.handle
                .sender
                .send(RecorderCommand::Finish)
                .map_err(|_| SessionFlightRecorderFinishError::ChannelClosed)?;
            let worker = self
                .worker
                .take()
                .ok_or(SessionFlightRecorderFinishError::ChannelClosed)?;
            let writer_outcome = worker
                .join()
                .map_err(|_| SessionFlightRecorderFinishError::WorkerPanicked)??;
            self.outcome = Some(SessionFlightRecorderOutcome {
                path: self.path.clone(),
                records_attempted_total: self
                    .handle
                    .counters
                    .records_attempted_total
                    .load(Ordering::Acquire),
                records_enqueued_total: self
                    .handle
                    .counters
                    .records_enqueued_total
                    .load(Ordering::Acquire),
                records_dropped_total: self
                    .handle
                    .counters
                    .records_dropped_total
                    .load(Ordering::Acquire),
                records_written_total: writer_outcome.records_written_total,
                rolling_hash: writer_outcome.rolling_hash,
            });
        }
        self.outcome
            .as_ref()
            .ok_or(SessionFlightRecorderFinishError::ChannelClosed)
    }

    pub fn outcome(&self) -> Option<&SessionFlightRecorderOutcome> {
        self.outcome.as_ref()
    }
}

impl Drop for SessionFlightRecorder {
    fn drop(&mut self) {
        let _ = self.finish();
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SessionFlightTrace {
    session_id: SessionId,
    records: Box<[SessionFlightRecord]>,
    outcome: SessionFlightRecorderOutcome,
}

impl SessionFlightTrace {
    pub fn read(path: impl AsRef<Path>) -> Result<Self, SessionFlightReplayError> {
        let path = path.as_ref();
        let bytes = std::fs::read(path)?;
        decode_trace(path, &bytes)
    }

    pub const fn session_id(&self) -> SessionId {
        self.session_id
    }

    pub fn records(&self) -> &[SessionFlightRecord] {
        &self.records
    }

    pub const fn outcome(&self) -> &SessionFlightRecorderOutcome {
        &self.outcome
    }

    pub fn replay(&self) -> Result<SessionFlightReplay, SessionFlightReplayError> {
        if !self.outcome.is_complete() {
            return Err(SessionFlightReplayError::IncompleteTrace);
        }
        let mut lifecycle = Vec::new();
        let mut terminal = None;
        let mut previous_timestamp_ns = None;
        for (expected_sequence_index, record) in self.records.iter().enumerate() {
            if record.sequence_index != expected_sequence_index as u64 {
                return Err(SessionFlightReplayError::SequenceGap);
            }
            if record.session_id != self.session_id {
                return Err(SessionFlightReplayError::SessionMismatch);
            }
            if previous_timestamp_ns.is_some_and(|previous| record.observed_at_ns < previous) {
                return Err(SessionFlightReplayError::TimestampRegression);
            }
            previous_timestamp_ns = Some(record.observed_at_ns);
            if terminal.is_some() {
                return Err(SessionFlightReplayError::RecordAfterTerminal);
            }
            match record.kind {
                SessionFlightRecordKind::Lifecycle { state } => {
                    validate_lifecycle_transition(lifecycle.last().copied(), state)?;
                    lifecycle.push(state);
                }
                SessionFlightRecordKind::Terminal {
                    state,
                    source_failures_total,
                    endpoint_failures_total,
                    rollback_failures_total,
                    finalization_failures_total,
                } => {
                    validate_terminal(lifecycle.last().copied(), state)?;
                    terminal = Some(SessionFlightTerminal {
                        state,
                        source_failures_total,
                        endpoint_failures_total,
                        rollback_failures_total,
                        finalization_failures_total,
                    });
                }
                SessionFlightRecordKind::SourceFailure { .. }
                | SessionFlightRecordKind::EndpointFailure { .. }
                | SessionFlightRecordKind::RollbackFailure { .. }
                | SessionFlightRecordKind::FinalizationFailure { .. } => {}
            }
        }
        let terminal = terminal.ok_or(SessionFlightReplayError::MissingTerminal)?;
        Ok(SessionFlightReplay {
            session_id: self.session_id,
            lifecycle: lifecycle.into_boxed_slice(),
            terminal,
            records_replayed_total: self.records.len() as u64,
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SessionFlightTerminal {
    pub state: SessionTerminalState,
    pub source_failures_total: u64,
    pub endpoint_failures_total: u64,
    pub rollback_failures_total: u64,
    pub finalization_failures_total: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SessionFlightReplay {
    pub session_id: SessionId,
    pub lifecycle: Box<[SessionLifecycleState]>,
    pub terminal: SessionFlightTerminal,
    pub records_replayed_total: u64,
}

#[derive(Debug, thiserror::Error)]
pub enum SessionFlightReplayError {
    #[error("flight trace I/O failed: {0}")]
    Io(#[from] std::io::Error),
    #[error("flight trace magic is invalid")]
    InvalidMagic,
    #[error("flight trace version is unsupported")]
    UnsupportedVersion,
    #[error("flight trace layout is invalid")]
    InvalidLayout,
    #[error("flight trace is truncated")]
    Truncated,
    #[error("flight trace checksum is invalid")]
    InvalidChecksum,
    #[error("flight trace is incomplete because records were dropped or not written")]
    IncompleteTrace,
    #[error("flight trace contains a non-contiguous record sequence")]
    SequenceGap,
    #[error("flight trace contains a different Session identity")]
    SessionMismatch,
    #[error("flight trace monotonic timestamp regressed")]
    TimestampRegression,
    #[error("flight trace lifecycle transition is invalid")]
    InvalidLifecycleTransition,
    #[error("flight trace does not contain a terminal record")]
    MissingTerminal,
    #[error("flight trace terminal state does not match lifecycle state")]
    TerminalMismatch,
    #[error("flight trace contains a record after the terminal record")]
    RecordAfterTerminal,
    #[error("flight trace contains an unknown record type")]
    UnknownRecordType,
}

#[derive(Debug, Default)]
struct RecorderCounters {
    records_attempted_total: AtomicU64,
    records_enqueued_total: AtomicU64,
    records_dropped_total: AtomicU64,
}

enum RecorderCommand {
    Record(SessionFlightRecord),
    Finish,
}

struct WriterOutcome {
    records_written_total: u64,
    rolling_hash: u64,
}

fn writer_loop(
    mut writer: BufWriter<File>,
    receiver: Receiver<RecorderCommand>,
    counters: Arc<RecorderCounters>,
) -> Result<WriterOutcome, std::io::Error> {
    let mut records_written_total = 0u64;
    let mut rolling_hash = FNV_OFFSET_BASIS;
    while let Ok(command) = receiver.recv() {
        match command {
            RecorderCommand::Record(record) => {
                let encoded = encode_record(record);
                writer.write_all(&encoded)?;
                rolling_hash = fnv1a(rolling_hash, &encoded);
                records_written_total = records_written_total.saturating_add(1);
            }
            RecorderCommand::Finish => {
                let attempted = counters.records_attempted_total.load(Ordering::Acquire);
                let enqueued = counters.records_enqueued_total.load(Ordering::Acquire);
                let dropped = counters.records_dropped_total.load(Ordering::Acquire);
                write_footer(
                    &mut writer,
                    attempted,
                    enqueued,
                    dropped,
                    records_written_total,
                    rolling_hash,
                )?;
                writer.flush()?;
                return Ok(WriterOutcome {
                    records_written_total,
                    rolling_hash,
                });
            }
        }
    }
    Err(std::io::Error::new(
        std::io::ErrorKind::BrokenPipe,
        "flight-recorder command channel closed",
    ))
}

fn record_kind(event: &SessionEventKind) -> SessionFlightRecordKind {
    match event {
        SessionEventKind::Lifecycle(state) => SessionFlightRecordKind::Lifecycle { state: *state },
        SessionEventKind::Source(failure) => SessionFlightRecordKind::SourceFailure {
            stem_id: failure.stem_id(),
        },
        SessionEventKind::Endpoint(failure) => SessionFlightRecordKind::EndpointFailure {
            route_id: failure.route_id(),
            endpoint_id: failure.endpoint_id(),
            stage_code: endpoint_stage_code(failure.stage()),
        },
        SessionEventKind::Rollback(failure) => SessionFlightRecordKind::RollbackFailure {
            stage: failure.stage(),
        },
        SessionEventKind::Finalization(failure) => SessionFlightRecordKind::FinalizationFailure {
            stage: failure.stage(),
        },
        SessionEventKind::Terminal(outcome) => SessionFlightRecordKind::Terminal {
            state: outcome.state(),
            source_failures_total: outcome.source_failures().len() as u64,
            endpoint_failures_total: outcome.endpoint_failures().len() as u64,
            rollback_failures_total: outcome.rollback_failures().len() as u64,
            finalization_failures_total: outcome.finalization_failures().len() as u64,
        },
    }
}

fn write_header(
    writer: &mut BufWriter<File>,
    session_id: SessionId,
    capacity_record_count: u64,
) -> Result<(), std::io::Error> {
    let mut bytes = [0u8; HEADER_SIZE_BYTES];
    bytes[0..8].copy_from_slice(TRACE_MAGIC);
    put_u16(&mut bytes, 8, TRACE_VERSION);
    put_u16(&mut bytes, 10, HEADER_SIZE_BYTES as u16);
    put_u16(&mut bytes, 12, RECORD_SIZE_BYTES as u16);
    put_u64(&mut bytes, 16, session_id.0);
    put_u64(&mut bytes, 24, capacity_record_count);
    let checksum = fnv1a(FNV_OFFSET_BASIS, &bytes[..32]);
    put_u64(&mut bytes, 32, checksum);
    writer.write_all(&bytes)
}

fn encode_record(record: SessionFlightRecord) -> [u8; RECORD_SIZE_BYTES] {
    let mut bytes = [0u8; RECORD_SIZE_BYTES];
    put_u32(&mut bytes, 0, RECORD_MARKER);
    let (kind_code, detail_code, values) = encode_kind(record.kind);
    bytes[4] = kind_code;
    bytes[5] = detail_code;
    put_u64(&mut bytes, 8, record.sequence_index);
    put_u64(&mut bytes, 16, record.observed_at_ns);
    put_u64(&mut bytes, 24, record.session_id.0);
    for (index, value) in values.into_iter().enumerate() {
        put_u64(&mut bytes, 32 + index * 8, value);
    }
    let checksum = fnv1a(FNV_OFFSET_BASIS, &bytes[..80]);
    put_u64(&mut bytes, 80, checksum);
    bytes
}

fn write_footer(
    writer: &mut BufWriter<File>,
    attempted: u64,
    enqueued: u64,
    dropped: u64,
    written: u64,
    rolling_hash: u64,
) -> Result<(), std::io::Error> {
    let mut bytes = [0u8; FOOTER_SIZE_BYTES];
    put_u32(&mut bytes, 0, FOOTER_MARKER);
    put_u16(&mut bytes, 4, TRACE_VERSION);
    put_u16(&mut bytes, 6, FOOTER_SIZE_BYTES as u16);
    put_u64(&mut bytes, 8, attempted);
    put_u64(&mut bytes, 16, enqueued);
    put_u64(&mut bytes, 24, dropped);
    put_u64(&mut bytes, 32, written);
    put_u64(&mut bytes, 40, rolling_hash);
    let checksum = fnv1a(FNV_OFFSET_BASIS, &bytes[..48]);
    put_u64(&mut bytes, 48, checksum);
    writer.write_all(&bytes)
}

fn decode_trace(path: &Path, bytes: &[u8]) -> Result<SessionFlightTrace, SessionFlightReplayError> {
    if bytes.len() < HEADER_SIZE_BYTES + FOOTER_SIZE_BYTES {
        return Err(SessionFlightReplayError::Truncated);
    }
    if &bytes[..8] != TRACE_MAGIC {
        return Err(SessionFlightReplayError::InvalidMagic);
    }
    if get_u16(bytes, 8)? != TRACE_VERSION {
        return Err(SessionFlightReplayError::UnsupportedVersion);
    }
    if get_u16(bytes, 10)? as usize != HEADER_SIZE_BYTES
        || get_u16(bytes, 12)? as usize != RECORD_SIZE_BYTES
    {
        return Err(SessionFlightReplayError::InvalidLayout);
    }
    if get_u64(bytes, 32)? != fnv1a(FNV_OFFSET_BASIS, &bytes[..32]) {
        return Err(SessionFlightReplayError::InvalidChecksum);
    }
    let session_id = SessionId(get_u64(bytes, 16)?);
    let footer_offset = bytes.len() - FOOTER_SIZE_BYTES;
    let footer = &bytes[footer_offset..];
    if get_u32(footer, 0)? != FOOTER_MARKER
        || get_u16(footer, 4)? != TRACE_VERSION
        || get_u16(footer, 6)? as usize != FOOTER_SIZE_BYTES
    {
        return Err(SessionFlightReplayError::InvalidLayout);
    }
    if get_u64(footer, 48)? != fnv1a(FNV_OFFSET_BASIS, &footer[..48]) {
        return Err(SessionFlightReplayError::InvalidChecksum);
    }
    let records_written_total = get_u64(footer, 32)?;
    let record_bytes = &bytes[HEADER_SIZE_BYTES..footer_offset];
    if !record_bytes.len().is_multiple_of(RECORD_SIZE_BYTES)
        || record_bytes.len() / RECORD_SIZE_BYTES != records_written_total as usize
    {
        return Err(SessionFlightReplayError::InvalidLayout);
    }
    let mut records = Vec::with_capacity(records_written_total as usize);
    let mut rolling_hash = FNV_OFFSET_BASIS;
    for encoded in record_bytes.chunks_exact(RECORD_SIZE_BYTES) {
        if get_u32(encoded, 0)? != RECORD_MARKER {
            return Err(SessionFlightReplayError::InvalidLayout);
        }
        if get_u64(encoded, 80)? != fnv1a(FNV_OFFSET_BASIS, &encoded[..80]) {
            return Err(SessionFlightReplayError::InvalidChecksum);
        }
        rolling_hash = fnv1a(rolling_hash, encoded);
        records.push(decode_record(encoded)?);
    }
    if rolling_hash != get_u64(footer, 40)? {
        return Err(SessionFlightReplayError::InvalidChecksum);
    }
    Ok(SessionFlightTrace {
        session_id,
        records: records.into_boxed_slice(),
        outcome: SessionFlightRecorderOutcome {
            path: path.to_path_buf(),
            records_attempted_total: get_u64(footer, 8)?,
            records_enqueued_total: get_u64(footer, 16)?,
            records_dropped_total: get_u64(footer, 24)?,
            records_written_total,
            rolling_hash,
        },
    })
}

fn decode_record(bytes: &[u8]) -> Result<SessionFlightRecord, SessionFlightReplayError> {
    let values = [
        get_u64(bytes, 32)?,
        get_u64(bytes, 40)?,
        get_u64(bytes, 48)?,
        get_u64(bytes, 56)?,
        get_u64(bytes, 64)?,
        get_u64(bytes, 72)?,
    ];
    Ok(SessionFlightRecord {
        sequence_index: get_u64(bytes, 8)?,
        observed_at_ns: get_u64(bytes, 16)?,
        session_id: SessionId(get_u64(bytes, 24)?),
        kind: decode_kind(bytes[4], bytes[5], values)?,
    })
}

fn encode_kind(kind: SessionFlightRecordKind) -> (u8, u8, [u64; 6]) {
    match kind {
        SessionFlightRecordKind::Lifecycle { state } => (1, lifecycle_code(state), [0; 6]),
        SessionFlightRecordKind::SourceFailure { stem_id } => (2, 0, [stem_id.0, 0, 0, 0, 0, 0]),
        SessionFlightRecordKind::EndpointFailure {
            route_id,
            endpoint_id,
            stage_code,
        } => (3, stage_code, [route_id.0, endpoint_id.0, 0, 0, 0, 0]),
        SessionFlightRecordKind::RollbackFailure { stage } => {
            (4, rollback_stage_code(stage), [0; 6])
        }
        SessionFlightRecordKind::FinalizationFailure { stage } => {
            (5, finalization_stage_code(stage), [0; 6])
        }
        SessionFlightRecordKind::Terminal {
            state,
            source_failures_total,
            endpoint_failures_total,
            rollback_failures_total,
            finalization_failures_total,
        } => (
            6,
            terminal_code(state),
            [
                source_failures_total,
                endpoint_failures_total,
                rollback_failures_total,
                finalization_failures_total,
                0,
                0,
            ],
        ),
    }
}

fn decode_kind(
    kind_code: u8,
    detail_code: u8,
    values: [u64; 6],
) -> Result<SessionFlightRecordKind, SessionFlightReplayError> {
    match kind_code {
        1 => Ok(SessionFlightRecordKind::Lifecycle {
            state: decode_lifecycle(detail_code)?,
        }),
        2 => Ok(SessionFlightRecordKind::SourceFailure {
            stem_id: StemId(values[0]),
        }),
        3 if (1..=5).contains(&detail_code) => Ok(SessionFlightRecordKind::EndpointFailure {
            route_id: RouteId(values[0]),
            endpoint_id: EndpointId(values[1]),
            stage_code: detail_code,
        }),
        3 => Err(SessionFlightReplayError::UnknownRecordType),
        4 => Ok(SessionFlightRecordKind::RollbackFailure {
            stage: decode_rollback_stage(detail_code)?,
        }),
        5 => Ok(SessionFlightRecordKind::FinalizationFailure {
            stage: decode_finalization_stage(detail_code)?,
        }),
        6 => Ok(SessionFlightRecordKind::Terminal {
            state: decode_terminal(detail_code)?,
            source_failures_total: values[0],
            endpoint_failures_total: values[1],
            rollback_failures_total: values[2],
            finalization_failures_total: values[3],
        }),
        _ => Err(SessionFlightReplayError::UnknownRecordType),
    }
}

fn validate_lifecycle_transition(
    previous: Option<SessionLifecycleState>,
    next: SessionLifecycleState,
) -> Result<(), SessionFlightReplayError> {
    let valid = matches!(
        (previous, next),
        (None, SessionLifecycleState::Starting)
            | (
                Some(SessionLifecycleState::Starting),
                SessionLifecycleState::Running | SessionLifecycleState::Failed
            )
            | (
                Some(SessionLifecycleState::Running),
                SessionLifecycleState::Stopping
            )
            | (
                Some(SessionLifecycleState::Stopping),
                SessionLifecycleState::Stopped | SessionLifecycleState::Failed
            )
    );
    if valid {
        Ok(())
    } else {
        Err(SessionFlightReplayError::InvalidLifecycleTransition)
    }
}

fn validate_terminal(
    lifecycle: Option<SessionLifecycleState>,
    terminal: SessionTerminalState,
) -> Result<(), SessionFlightReplayError> {
    let matches = matches!(
        (lifecycle, terminal),
        (
            Some(SessionLifecycleState::Stopped),
            SessionTerminalState::Stopped
        ) | (
            Some(SessionLifecycleState::Failed),
            SessionTerminalState::Failed
        )
    );
    if matches {
        Ok(())
    } else {
        Err(SessionFlightReplayError::TerminalMismatch)
    }
}

fn lifecycle_code(state: SessionLifecycleState) -> u8 {
    match state {
        SessionLifecycleState::Starting => 1,
        SessionLifecycleState::Running => 2,
        SessionLifecycleState::Stopping => 3,
        SessionLifecycleState::Stopped => 4,
        SessionLifecycleState::Failed => 5,
    }
}

fn decode_lifecycle(code: u8) -> Result<SessionLifecycleState, SessionFlightReplayError> {
    match code {
        1 => Ok(SessionLifecycleState::Starting),
        2 => Ok(SessionLifecycleState::Running),
        3 => Ok(SessionLifecycleState::Stopping),
        4 => Ok(SessionLifecycleState::Stopped),
        5 => Ok(SessionLifecycleState::Failed),
        _ => Err(SessionFlightReplayError::UnknownRecordType),
    }
}

fn terminal_code(state: SessionTerminalState) -> u8 {
    match state {
        SessionTerminalState::Stopped => 1,
        SessionTerminalState::Failed => 2,
    }
}

fn decode_terminal(code: u8) -> Result<SessionTerminalState, SessionFlightReplayError> {
    match code {
        1 => Ok(SessionTerminalState::Stopped),
        2 => Ok(SessionTerminalState::Failed),
        _ => Err(SessionFlightReplayError::UnknownRecordType),
    }
}

fn rollback_stage_code(stage: SessionRollbackStage) -> u8 {
    match stage {
        SessionRollbackStage::CancelOperator => 1,
        SessionRollbackStage::CancelEndpointPreparation => 2,
        SessionRollbackStage::FinalizeStartedEndpoint => 3,
        SessionRollbackStage::StopOpenedCapture => 4,
        SessionRollbackStage::DiscardRuntimeQueues => 5,
    }
}

fn decode_rollback_stage(code: u8) -> Result<SessionRollbackStage, SessionFlightReplayError> {
    match code {
        1 => Ok(SessionRollbackStage::CancelOperator),
        2 => Ok(SessionRollbackStage::CancelEndpointPreparation),
        3 => Ok(SessionRollbackStage::FinalizeStartedEndpoint),
        4 => Ok(SessionRollbackStage::StopOpenedCapture),
        5 => Ok(SessionRollbackStage::DiscardRuntimeQueues),
        _ => Err(SessionFlightReplayError::UnknownRecordType),
    }
}

fn finalization_stage_code(stage: SessionFinalizationStage) -> u8 {
    match stage {
        SessionFinalizationStage::StopCapture => 1,
        SessionFinalizationStage::DrainRuntime => 2,
        SessionFinalizationStage::DrainOperator => 3,
        SessionFinalizationStage::RequestEndpointStop => 4,
        SessionFinalizationStage::JoinEndpoint => 5,
        SessionFinalizationStage::FinalizeEndpoint => 6,
    }
}

fn decode_finalization_stage(
    code: u8,
) -> Result<SessionFinalizationStage, SessionFlightReplayError> {
    match code {
        1 => Ok(SessionFinalizationStage::StopCapture),
        2 => Ok(SessionFinalizationStage::DrainRuntime),
        3 => Ok(SessionFinalizationStage::DrainOperator),
        4 => Ok(SessionFinalizationStage::RequestEndpointStop),
        5 => Ok(SessionFinalizationStage::JoinEndpoint),
        6 => Ok(SessionFinalizationStage::FinalizeEndpoint),
        _ => Err(SessionFlightReplayError::UnknownRecordType),
    }
}

fn endpoint_stage_code(stage: crate::endpoint::EndpointFailureStage) -> u8 {
    match stage {
        crate::endpoint::EndpointFailureStage::Prepare => 1,
        crate::endpoint::EndpointFailureStage::CancelPreparation => 2,
        crate::endpoint::EndpointFailureStage::Start => 3,
        crate::endpoint::EndpointFailureStage::RequestStop => 4,
        crate::endpoint::EndpointFailureStage::JoinFinalize => 5,
    }
}

fn fnv1a(mut state: u64, bytes: &[u8]) -> u64 {
    for byte in bytes {
        state ^= u64::from(*byte);
        state = state.wrapping_mul(FNV_PRIME);
    }
    state
}

fn put_u16(bytes: &mut [u8], offset: usize, value: u16) {
    bytes[offset..offset + 2].copy_from_slice(&value.to_le_bytes());
}

fn put_u32(bytes: &mut [u8], offset: usize, value: u32) {
    bytes[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
}

fn put_u64(bytes: &mut [u8], offset: usize, value: u64) {
    bytes[offset..offset + 8].copy_from_slice(&value.to_le_bytes());
}

fn get_u16(bytes: &[u8], offset: usize) -> Result<u16, SessionFlightReplayError> {
    bytes
        .get(offset..offset + 2)
        .and_then(|value| value.try_into().ok())
        .map(u16::from_le_bytes)
        .ok_or(SessionFlightReplayError::Truncated)
}

fn get_u32(bytes: &[u8], offset: usize) -> Result<u32, SessionFlightReplayError> {
    bytes
        .get(offset..offset + 4)
        .and_then(|value| value.try_into().ok())
        .map(u32::from_le_bytes)
        .ok_or(SessionFlightReplayError::Truncated)
}

fn get_u64(bytes: &[u8], offset: usize) -> Result<u64, SessionFlightReplayError> {
    bytes
        .get(offset..offset + 8)
        .and_then(|value| value.try_into().ok())
        .map(u64::from_le_bytes)
        .ok_or(SessionFlightReplayError::Truncated)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::session::{SessionEventKind, SessionTerminalOutcome};

    fn event(session_id: SessionId, kind: SessionEventKind) -> SessionEvent {
        SessionEvent::new(session_id, kind)
    }

    fn complete_trace(path: &Path) -> SessionFlightRecorderOutcome {
        let session_id = SessionId(17);
        let mut recorder = SessionFlightRecorder::start(path, session_id, 16).unwrap();
        let handle = recorder.handle();
        for kind in [
            SessionEventKind::Lifecycle(SessionLifecycleState::Starting),
            SessionEventKind::Lifecycle(SessionLifecycleState::Running),
            SessionEventKind::Lifecycle(SessionLifecycleState::Stopping),
            SessionEventKind::Lifecycle(SessionLifecycleState::Stopped),
            SessionEventKind::Terminal(SessionTerminalOutcome::new(
                session_id,
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
            )),
        ] {
            assert_eq!(
                handle.try_record_event(&event(session_id, kind)),
                SessionFlightRecordDelivery::Enqueued
            );
        }
        recorder.finish().unwrap().clone()
    }

    fn record(
        sequence_index: u64,
        observed_at_ns: u64,
        session_id: SessionId,
        kind: SessionFlightRecordKind,
    ) -> SessionFlightRecord {
        SessionFlightRecord {
            sequence_index,
            observed_at_ns,
            session_id,
            kind,
        }
    }

    fn synthetic_trace(
        records: Vec<SessionFlightRecord>,
        outcome: SessionFlightRecorderOutcome,
    ) -> SessionFlightTrace {
        SessionFlightTrace {
            session_id: SessionId(17),
            records: records.into_boxed_slice(),
            outcome,
        }
    }

    fn complete_outcome(record_count: u64) -> SessionFlightRecorderOutcome {
        SessionFlightRecorderOutcome {
            path: PathBuf::from("synthetic.pksflight"),
            records_attempted_total: record_count,
            records_enqueued_total: record_count,
            records_dropped_total: 0,
            records_written_total: record_count,
            rolling_hash: 0,
        }
    }

    fn stopped_records() -> Vec<SessionFlightRecord> {
        let session_id = SessionId(17);
        vec![
            record(
                0,
                10,
                session_id,
                SessionFlightRecordKind::Lifecycle {
                    state: SessionLifecycleState::Starting,
                },
            ),
            record(
                1,
                20,
                session_id,
                SessionFlightRecordKind::Lifecycle {
                    state: SessionLifecycleState::Running,
                },
            ),
            record(
                2,
                30,
                session_id,
                SessionFlightRecordKind::Lifecycle {
                    state: SessionLifecycleState::Stopping,
                },
            ),
            record(
                3,
                40,
                session_id,
                SessionFlightRecordKind::Lifecycle {
                    state: SessionLifecycleState::Stopped,
                },
            ),
            record(
                4,
                50,
                session_id,
                SessionFlightRecordKind::Terminal {
                    state: SessionTerminalState::Stopped,
                    source_failures_total: 0,
                    endpoint_failures_total: 0,
                    rollback_failures_total: 0,
                    finalization_failures_total: 0,
                },
            ),
        ]
    }

    #[test]
    fn given_complete_trace_when_replayed_then_lifecycle_and_terminal_match() {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("session.pksflight");
        let outcome = complete_trace(&path);
        assert!(outcome.is_complete());

        let trace = SessionFlightTrace::read(&path).unwrap();
        let replay = trace.replay().unwrap();

        assert_eq!(replay.session_id, SessionId(17));
        assert_eq!(
            replay.lifecycle.as_ref(),
            &[
                SessionLifecycleState::Starting,
                SessionLifecycleState::Running,
                SessionLifecycleState::Stopping,
                SessionLifecycleState::Stopped,
            ]
        );
        assert_eq!(replay.terminal.state, SessionTerminalState::Stopped);
        assert_eq!(replay.records_replayed_total, 5);
    }

    #[test]
    fn given_corrupted_record_when_read_then_checksum_is_rejected() {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("session.pksflight");
        complete_trace(&path);
        let mut bytes = std::fs::read(&path).unwrap();
        bytes[HEADER_SIZE_BYTES + 20] ^= 0x80;
        std::fs::write(&path, bytes).unwrap();

        assert!(matches!(
            SessionFlightTrace::read(&path),
            Err(SessionFlightReplayError::InvalidChecksum)
        ));
    }

    #[test]
    fn given_truncated_trace_when_read_then_truncation_is_rejected() {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("session.pksflight");
        complete_trace(&path);
        let mut bytes = std::fs::read(&path).unwrap();
        bytes.truncate(HEADER_SIZE_BYTES + 12);
        std::fs::write(&path, bytes).unwrap();

        assert!(matches!(
            SessionFlightTrace::read(&path),
            Err(SessionFlightReplayError::Truncated)
        ));
    }

    #[test]
    fn given_unknown_version_when_read_then_version_is_rejected() {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("session.pksflight");
        complete_trace(&path);
        let mut bytes = std::fs::read(&path).unwrap();
        put_u16(&mut bytes, 8, TRACE_VERSION + 1);
        std::fs::write(&path, bytes).unwrap();

        assert!(matches!(
            SessionFlightTrace::read(&path),
            Err(SessionFlightReplayError::UnsupportedVersion)
        ));
    }

    #[test]
    fn given_invalid_lifecycle_when_replayed_then_replay_fails_closed() {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("invalid.pksflight");
        let session_id = SessionId(23);
        let mut recorder = SessionFlightRecorder::start(&path, session_id, 8).unwrap();
        let handle = recorder.handle();
        for kind in [
            SessionEventKind::Lifecycle(SessionLifecycleState::Starting),
            SessionEventKind::Lifecycle(SessionLifecycleState::Stopping),
            SessionEventKind::Lifecycle(SessionLifecycleState::Stopped),
            SessionEventKind::Terminal(SessionTerminalOutcome::new(
                session_id,
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
            )),
        ] {
            assert_eq!(
                handle.try_record_event(&event(session_id, kind)),
                SessionFlightRecordDelivery::Enqueued
            );
        }
        recorder.finish().unwrap();

        let trace = SessionFlightTrace::read(&path).unwrap();
        assert!(matches!(
            trace.replay(),
            Err(SessionFlightReplayError::InvalidLifecycleTransition)
        ));
    }

    #[test]
    fn given_existing_output_when_started_then_recorder_fails_closed() {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("existing.pksflight");
        std::fs::write(&path, b"existing").unwrap();

        assert!(matches!(
            SessionFlightRecorder::start(&path, SessionId(1), 8),
            Err(SessionFlightRecorderStartError::OutputExists { .. })
        ));
        assert_eq!(std::fs::read(&path).unwrap(), b"existing");
    }

    #[test]
    fn given_zero_capacity_when_started_then_recorder_fails_closed() {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("zero-capacity.pksflight");

        assert!(matches!(
            SessionFlightRecorder::start(&path, SessionId(1), 0),
            Err(SessionFlightRecorderStartError::ZeroCapacity)
        ));
        assert!(!path.exists());
    }

    #[test]
    fn given_dropped_records_when_replayed_then_trace_is_incomplete() {
        let records = stopped_records();
        let trace = synthetic_trace(
            records,
            SessionFlightRecorderOutcome {
                path: PathBuf::from("synthetic.pksflight"),
                records_attempted_total: 6,
                records_enqueued_total: 5,
                records_dropped_total: 1,
                records_written_total: 5,
                rolling_hash: 0,
            },
        );

        assert!(matches!(
            trace.replay(),
            Err(SessionFlightReplayError::IncompleteTrace)
        ));
    }

    #[test]
    fn given_sequence_gap_when_replayed_then_trace_is_rejected() {
        let mut records = stopped_records();
        records[2].sequence_index = 3;
        let trace = synthetic_trace(records, complete_outcome(5));

        assert!(matches!(
            trace.replay(),
            Err(SessionFlightReplayError::SequenceGap)
        ));
    }

    #[test]
    fn given_timestamp_regression_when_replayed_then_trace_is_rejected() {
        let mut records = stopped_records();
        records[2].observed_at_ns = 5;
        let trace = synthetic_trace(records, complete_outcome(5));

        assert!(matches!(
            trace.replay(),
            Err(SessionFlightReplayError::TimestampRegression)
        ));
    }

    #[test]
    fn given_record_after_terminal_when_replayed_then_trace_is_rejected() {
        let mut records = stopped_records();
        records.push(record(
            5,
            60,
            SessionId(17),
            SessionFlightRecordKind::SourceFailure { stem_id: StemId(9) },
        ));
        let trace = synthetic_trace(records, complete_outcome(6));

        assert!(matches!(
            trace.replay(),
            Err(SessionFlightReplayError::RecordAfterTerminal)
        ));
    }
}
