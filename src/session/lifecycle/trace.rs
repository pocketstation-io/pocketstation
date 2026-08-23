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

const TRACE_MAGIC: &[u8; 8] = b"PKSTRC01";
const TRACE_VERSION: u16 = 1;
const HEADER_SIZE_BYTES: usize = 40;
const RECORD_SIZE_BYTES: usize = 88;
const FOOTER_SIZE_BYTES: usize = 64;
const RECORD_MARKER: u32 = 0x5245_4344;
const FOOTER_MARKER: u32 = 0x464F_4F54;
const FNV_OFFSET_BASIS: u64 = 0xcbf2_9ce4_8422_2325;
const FNV_PRIME: u64 = 0x0000_0100_0000_01b3;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[doc = "Selects the session trace record kind used by PocketStation."]
pub enum SessionTraceRecordKind {
    #[doc = "Selects lifecycle behavior for `SessionTraceRecordKind`."]
    Lifecycle {
        #[doc = "Stores the state used by `Lifecycle`."]
        state: SessionLifecycleState,
    },
    #[doc = "Selects source failure behavior for `SessionTraceRecordKind`."]
    SourceFailure {
        #[doc = "Identifies the stem identifier recorded by `SourceFailure`."]
        stem_id: StemId,
    },
    #[doc = "Selects endpoint failure behavior for `SessionTraceRecordKind`."]
    EndpointFailure {
        #[doc = "Identifies the route identifier recorded by `EndpointFailure`."]
        route_id: RouteId,
        #[doc = "Identifies the endpoint identifier recorded by `EndpointFailure`."]
        endpoint_id: EndpointId,
        #[doc = "Stores the stage code used by `EndpointFailure`."]
        stage_code: u8,
    },
    #[doc = "Selects rollback failure behavior for `SessionTraceRecordKind`."]
    RollbackFailure {
        #[doc = "Stores the stage used by `RollbackFailure`."]
        stage: SessionRollbackStage,
    },
    #[doc = "Selects finalization failure behavior for `SessionTraceRecordKind`."]
    FinalizationFailure {
        #[doc = "Stores the stage used by `FinalizationFailure`."]
        stage: SessionFinalizationStage,
    },
    #[doc = "Selects terminal behavior for `SessionTraceRecordKind`."]
    Terminal {
        #[doc = "Stores the state used by `Terminal`."]
        state: SessionTerminalState,
        #[doc = "Counts the total number of source failures observed by `Terminal`."]
        source_failures_total: u64,
        #[doc = "Counts the total number of endpoint failures observed by `Terminal`."]
        endpoint_failures_total: u64,
        #[doc = "Counts the total number of rollback failures observed by `Terminal`."]
        rollback_failures_total: u64,
        #[doc = "Counts the total number of finalization failures observed by `Terminal`."]
        finalization_failures_total: u64,
    },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[doc = "Records one immutable session trace observation."]
pub struct SessionTraceRecord {
    #[doc = "Stores the sequence index used by `SessionTraceRecord`."]
    pub sequence_index: u64,
    #[doc = "Stores the observed at value for `SessionTraceRecord`, in nanoseconds."]
    pub observed_at_ns: u64,
    #[doc = "Identifies the session identifier recorded by `SessionTraceRecord`."]
    pub session_id: SessionId,
    #[doc = "Stores the kind used by `SessionTraceRecord`."]
    pub kind: SessionTraceRecordKind,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum SessionTraceRecordDelivery {
    Enqueued,
    DroppedFull,
    Closed,
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[doc = "Reports the structured session trace recorder outcome."]
pub struct SessionTraceRecorderOutcome {
    #[doc = "Stores the path used by `SessionTraceRecorderOutcome`."]
    pub path: PathBuf,
    #[doc = "Counts the total number of records attempted observed by `SessionTraceRecorderOutcome`."]
    pub records_attempted_total: u64,
    #[doc = "Counts the total number of records enqueued observed by `SessionTraceRecorderOutcome`."]
    pub records_enqueued_total: u64,
    #[doc = "Counts the total number of records dropped observed by `SessionTraceRecorderOutcome`."]
    pub records_dropped_total: u64,
    #[doc = "Counts the total number of records written observed by `SessionTraceRecorderOutcome`."]
    pub records_written_total: u64,
    #[doc = "Stores the rolling hash used by `SessionTraceRecorderOutcome`."]
    pub rolling_hash: u64,
}

impl SessionTraceRecorderOutcome {
    #[doc = "Returns whether complete applies to `SessionTraceRecorderOutcome`."]
    pub fn is_complete(&self) -> bool {
        self.records_dropped_total == 0
            && self.records_attempted_total == self.records_enqueued_total
            && self.records_enqueued_total == self.records_written_total
    }
}

#[derive(Debug, thiserror::Error)]
#[doc = "Classifies failures reported as session trace recorder start error."]
pub enum SessionTraceRecorderStartError {
    #[error("session trace capacity must be greater than zero records")]
    #[doc = "Reports zero capacity."]
    ZeroCapacity,
    #[error("session trace output already exists: {path}")]
    #[doc = "Reports output exists."]
    OutputExists {
        #[doc = "Stores the path used by `OutputExists`."]
        path: PathBuf,
    },
    #[error("session trace I/O failed: {0}")]
    #[doc = "Reports I/O."]
    Io(#[from] std::io::Error),
}

#[derive(Debug, thiserror::Error)]
#[doc = "Classifies failures reported as session trace recorder finish error."]
pub enum SessionTraceRecorderFinishError {
    #[error("session trace command channel closed before finalization")]
    #[doc = "Reports channel closed."]
    ChannelClosed,
    #[error("session trace worker panicked")]
    #[doc = "Reports worker panicked."]
    WorkerPanicked,
    #[error("session trace I/O failed: {0}")]
    #[doc = "Reports I/O."]
    Io(#[from] std::io::Error),
}

#[derive(Clone, Debug)]
#[doc = "Owns bounded access to session trace recorder."]
pub struct SessionTraceRecorderHandle {
    sender: SyncSender<RecorderCommand>,
    counters: Arc<RecorderCounters>,
    accepting: Arc<AtomicBool>,
}

impl SessionTraceRecorderHandle {
    pub(crate) fn try_record_event(&self, event: &SessionEvent) -> SessionTraceRecordDelivery {
        if !self.accepting.load(Ordering::Acquire) {
            return SessionTraceRecordDelivery::Closed;
        }
        let sequence_index = self
            .counters
            .records_attempted_total
            .fetch_add(1, Ordering::Relaxed);
        let record = SessionTraceRecord {
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
                SessionTraceRecordDelivery::Enqueued
            }
            Err(TrySendError::Full(_)) => {
                self.counters
                    .records_dropped_total
                    .fetch_add(1, Ordering::Relaxed);
                SessionTraceRecordDelivery::DroppedFull
            }
            Err(TrySendError::Disconnected(_)) => {
                self.counters
                    .records_dropped_total
                    .fetch_add(1, Ordering::Relaxed);
                SessionTraceRecordDelivery::Closed
            }
        }
    }
}

#[doc = "Collects ordered lifecycle records and writes the trace artifact during Session finalization."]
pub struct SessionTraceRecorder {
    path: PathBuf,
    handle: SessionTraceRecorderHandle,
    worker: Option<JoinHandle<Result<WriterOutcome, std::io::Error>>>,
    outcome: Option<SessionTraceRecorderOutcome>,
}

impl SessionTraceRecorder {
    #[doc = "Starts the lifecycle represented by `SessionTraceRecorder`."]
    pub fn start(
        path: impl Into<PathBuf>,
        session_id: SessionId,
        capacity_records: usize,
    ) -> Result<Self, SessionTraceRecorderStartError> {
        if capacity_records == 0 {
            return Err(SessionTraceRecorderStartError::ZeroCapacity);
        }
        let path = path.into();
        let file = match OpenOptions::new().write(true).create_new(true).open(&path) {
            Ok(file) => file,
            Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {
                return Err(SessionTraceRecorderStartError::OutputExists { path });
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
            .name("pks-session-trace".to_owned())
            .spawn(move || writer_loop(writer, receiver, worker_counters))?;
        Ok(Self {
            path,
            handle: SessionTraceRecorderHandle {
                sender,
                counters,
                accepting,
            },
            worker: Some(worker),
            outcome: None,
        })
    }

    #[doc = "Returns the handle held by `SessionTraceRecorder`."]
    pub fn handle(&self) -> SessionTraceRecorderHandle {
        self.handle.clone()
    }

    #[doc = "Finishes work owned by `SessionTraceRecorder`."]
    pub fn finish(
        &mut self,
    ) -> Result<&SessionTraceRecorderOutcome, SessionTraceRecorderFinishError> {
        if self.outcome.is_none() {
            self.handle.accepting.store(false, Ordering::Release);
            self.handle
                .sender
                .send(RecorderCommand::Finish)
                .map_err(|_| SessionTraceRecorderFinishError::ChannelClosed)?;
            let worker = self
                .worker
                .take()
                .ok_or(SessionTraceRecorderFinishError::ChannelClosed)?;
            let writer_outcome = worker
                .join()
                .map_err(|_| SessionTraceRecorderFinishError::WorkerPanicked)??;
            self.outcome = Some(SessionTraceRecorderOutcome {
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
            .ok_or(SessionTraceRecorderFinishError::ChannelClosed)
    }

    #[doc = "Returns the outcome held by `SessionTraceRecorder`."]
    pub fn outcome(&self) -> Option<&SessionTraceRecorderOutcome> {
        self.outcome.as_ref()
    }
}

impl Drop for SessionTraceRecorder {
    #[doc = "Releases resources owned by `SessionTraceRecorder`."]
    fn drop(&mut self) {
        let _ = self.finish();
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[doc = "Contains the ordered lifecycle records read from a Session trace artifact."]
pub struct SessionTrace {
    session_id: SessionId,
    records: Box<[SessionTraceRecord]>,
    outcome: SessionTraceRecorderOutcome,
}

impl SessionTrace {
    #[doc = "Reads the persisted representation of `SessionTrace`."]
    pub fn read(path: impl AsRef<Path>) -> Result<Self, SessionTraceValidationError> {
        let path = path.as_ref();
        let bytes = std::fs::read(path)?;
        decode_trace(path, &bytes)
    }

    #[doc = "Returns the session identifier held by `SessionTrace`."]
    pub const fn session_id(&self) -> SessionId {
        self.session_id
    }

    #[doc = "Returns the records held by `SessionTrace`."]
    pub fn records(&self) -> &[SessionTraceRecord] {
        &self.records
    }

    #[doc = "Returns the outcome held by `SessionTrace`."]
    pub const fn outcome(&self) -> &SessionTraceRecorderOutcome {
        &self.outcome
    }

    #[doc = "Validates `SessionTrace` against its declared contract."]
    pub fn validate(&self) -> Result<SessionTraceValidation, SessionTraceValidationError> {
        if !self.outcome.is_complete() {
            return Err(SessionTraceValidationError::IncompleteTrace);
        }
        let mut lifecycle = Vec::new();
        let mut terminal = None;
        let mut previous_timestamp_ns = None;
        for (expected_sequence_index, record) in self.records.iter().enumerate() {
            if record.sequence_index != expected_sequence_index as u64 {
                return Err(SessionTraceValidationError::SequenceGap);
            }
            if record.session_id != self.session_id {
                return Err(SessionTraceValidationError::SessionMismatch);
            }
            if previous_timestamp_ns.is_some_and(|previous| record.observed_at_ns < previous) {
                return Err(SessionTraceValidationError::TimestampRegression);
            }
            previous_timestamp_ns = Some(record.observed_at_ns);
            if terminal.is_some() {
                return Err(SessionTraceValidationError::RecordAfterTerminal);
            }
            match record.kind {
                SessionTraceRecordKind::Lifecycle { state } => {
                    validate_lifecycle_transition(lifecycle.last().copied(), state)?;
                    lifecycle.push(state);
                }
                SessionTraceRecordKind::Terminal {
                    state,
                    source_failures_total,
                    endpoint_failures_total,
                    rollback_failures_total,
                    finalization_failures_total,
                } => {
                    validate_terminal(lifecycle.last().copied(), state)?;
                    terminal = Some(SessionTraceTerminal {
                        state,
                        source_failures_total,
                        endpoint_failures_total,
                        rollback_failures_total,
                        finalization_failures_total,
                    });
                }
                SessionTraceRecordKind::SourceFailure { .. }
                | SessionTraceRecordKind::EndpointFailure { .. }
                | SessionTraceRecordKind::RollbackFailure { .. }
                | SessionTraceRecordKind::FinalizationFailure { .. } => {}
            }
        }
        let terminal = terminal.ok_or(SessionTraceValidationError::MissingTerminal)?;
        Ok(SessionTraceValidation {
            session_id: self.session_id,
            lifecycle: lifecycle.into_boxed_slice(),
            terminal,
            records_validated_total: self.records.len() as u64,
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[doc = "Records the terminal Session disposition and component failures stored in a trace."]
pub struct SessionTraceTerminal {
    #[doc = "Stores the state used by `SessionTraceTerminal`."]
    pub state: SessionTerminalState,
    #[doc = "Counts the total number of source failures observed by `SessionTraceTerminal`."]
    pub source_failures_total: u64,
    #[doc = "Counts the total number of endpoint failures observed by `SessionTraceTerminal`."]
    pub endpoint_failures_total: u64,
    #[doc = "Counts the total number of rollback failures observed by `SessionTraceTerminal`."]
    pub rollback_failures_total: u64,
    #[doc = "Counts the total number of finalization failures observed by `SessionTraceTerminal`."]
    pub finalization_failures_total: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[doc = "Reports the validated identity and record count of a parsed Session trace."]
pub struct SessionTraceValidation {
    #[doc = "Identifies the session identifier recorded by `SessionTraceValidation`."]
    pub session_id: SessionId,
    #[doc = "Stores the lifecycle used by `SessionTraceValidation`."]
    pub lifecycle: Box<[SessionLifecycleState]>,
    #[doc = "Indicates whether terminal applies to `SessionTraceValidation`."]
    pub terminal: SessionTraceTerminal,
    #[doc = "Counts the total number of records validated observed by `SessionTraceValidation`."]
    pub records_validated_total: u64,
}

#[derive(Debug, thiserror::Error)]
#[doc = "Classifies failures reported as session trace validation error."]
pub enum SessionTraceValidationError {
    #[error("Session trace I/O failed: {0}")]
    #[doc = "Reports I/O."]
    Io(#[from] std::io::Error),
    #[error("Session trace magic is invalid")]
    #[doc = "Reports invalid magic."]
    InvalidMagic,
    #[error("Session trace version is unsupported")]
    #[doc = "Reports unsupported version."]
    UnsupportedVersion,
    #[error("Session trace layout is invalid")]
    #[doc = "Reports invalid layout."]
    InvalidLayout,
    #[error("Session trace is truncated")]
    #[doc = "Reports truncated."]
    Truncated,
    #[error("Session trace checksum is invalid")]
    #[doc = "Reports invalid checksum."]
    InvalidChecksum,
    #[error("Session trace is incomplete because records were dropped or not written")]
    #[doc = "Reports incomplete trace."]
    IncompleteTrace,
    #[error("Session trace contains a non-contiguous record sequence")]
    #[doc = "Reports sequence gap."]
    SequenceGap,
    #[error("Session trace contains a different Session identity")]
    #[doc = "Reports session mismatch."]
    SessionMismatch,
    #[error("Session trace monotonic timestamp regressed")]
    #[doc = "Reports timestamp regression."]
    TimestampRegression,
    #[error("Session trace lifecycle transition is invalid")]
    #[doc = "Reports invalid lifecycle transition."]
    InvalidLifecycleTransition,
    #[error("Session trace does not contain a terminal record")]
    #[doc = "Reports missing terminal."]
    MissingTerminal,
    #[error("Session trace terminal state does not match lifecycle state")]
    #[doc = "Reports terminal mismatch."]
    TerminalMismatch,
    #[error("Session trace contains a record after the terminal record")]
    #[doc = "Reports record after terminal."]
    RecordAfterTerminal,
    #[error("Session trace contains an unknown record type")]
    #[doc = "Reports unknown record type."]
    UnknownRecordType,
}

#[derive(Debug, Default)]
struct RecorderCounters {
    records_attempted_total: AtomicU64,
    records_enqueued_total: AtomicU64,
    records_dropped_total: AtomicU64,
}

enum RecorderCommand {
    Record(SessionTraceRecord),
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
        "session trace command channel closed",
    ))
}

fn record_kind(event: &SessionEventKind) -> SessionTraceRecordKind {
    match event {
        SessionEventKind::Lifecycle(state) => SessionTraceRecordKind::Lifecycle { state: *state },
        SessionEventKind::Source(failure) => SessionTraceRecordKind::SourceFailure {
            stem_id: failure.stem_id(),
        },
        SessionEventKind::Endpoint(failure) => SessionTraceRecordKind::EndpointFailure {
            route_id: failure.route_id(),
            endpoint_id: failure.endpoint_id(),
            stage_code: endpoint_stage_code(failure.stage()),
        },
        SessionEventKind::Rollback(failure) => SessionTraceRecordKind::RollbackFailure {
            stage: failure.stage(),
        },
        SessionEventKind::Finalization(failure) => SessionTraceRecordKind::FinalizationFailure {
            stage: failure.stage(),
        },
        SessionEventKind::Terminal(outcome) => SessionTraceRecordKind::Terminal {
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

fn encode_record(record: SessionTraceRecord) -> [u8; RECORD_SIZE_BYTES] {
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

fn decode_trace(path: &Path, bytes: &[u8]) -> Result<SessionTrace, SessionTraceValidationError> {
    if bytes.len() < HEADER_SIZE_BYTES + FOOTER_SIZE_BYTES {
        return Err(SessionTraceValidationError::Truncated);
    }
    if &bytes[..8] != TRACE_MAGIC {
        return Err(SessionTraceValidationError::InvalidMagic);
    }
    if get_u16(bytes, 8)? != TRACE_VERSION {
        return Err(SessionTraceValidationError::UnsupportedVersion);
    }
    if get_u16(bytes, 10)? as usize != HEADER_SIZE_BYTES
        || get_u16(bytes, 12)? as usize != RECORD_SIZE_BYTES
    {
        return Err(SessionTraceValidationError::InvalidLayout);
    }
    if get_u64(bytes, 32)? != fnv1a(FNV_OFFSET_BASIS, &bytes[..32]) {
        return Err(SessionTraceValidationError::InvalidChecksum);
    }
    let session_id = SessionId(get_u64(bytes, 16)?);
    let footer_offset = bytes.len() - FOOTER_SIZE_BYTES;
    let footer = &bytes[footer_offset..];
    if get_u32(footer, 0)? != FOOTER_MARKER
        || get_u16(footer, 4)? != TRACE_VERSION
        || get_u16(footer, 6)? as usize != FOOTER_SIZE_BYTES
    {
        return Err(SessionTraceValidationError::InvalidLayout);
    }
    if get_u64(footer, 48)? != fnv1a(FNV_OFFSET_BASIS, &footer[..48]) {
        return Err(SessionTraceValidationError::InvalidChecksum);
    }
    let records_written_total = get_u64(footer, 32)?;
    let record_bytes = &bytes[HEADER_SIZE_BYTES..footer_offset];
    if !record_bytes.len().is_multiple_of(RECORD_SIZE_BYTES)
        || record_bytes.len() / RECORD_SIZE_BYTES != records_written_total as usize
    {
        return Err(SessionTraceValidationError::InvalidLayout);
    }
    let mut records = Vec::with_capacity(records_written_total as usize);
    let mut rolling_hash = FNV_OFFSET_BASIS;
    for encoded in record_bytes.as_chunks::<RECORD_SIZE_BYTES>().0 {
        if get_u32(encoded, 0)? != RECORD_MARKER {
            return Err(SessionTraceValidationError::InvalidLayout);
        }
        if get_u64(encoded, 80)? != fnv1a(FNV_OFFSET_BASIS, &encoded[..80]) {
            return Err(SessionTraceValidationError::InvalidChecksum);
        }
        rolling_hash = fnv1a(rolling_hash, encoded);
        records.push(decode_record(encoded)?);
    }
    if rolling_hash != get_u64(footer, 40)? {
        return Err(SessionTraceValidationError::InvalidChecksum);
    }
    Ok(SessionTrace {
        session_id,
        records: records.into_boxed_slice(),
        outcome: SessionTraceRecorderOutcome {
            path: path.to_path_buf(),
            records_attempted_total: get_u64(footer, 8)?,
            records_enqueued_total: get_u64(footer, 16)?,
            records_dropped_total: get_u64(footer, 24)?,
            records_written_total,
            rolling_hash,
        },
    })
}

fn decode_record(bytes: &[u8]) -> Result<SessionTraceRecord, SessionTraceValidationError> {
    let values = [
        get_u64(bytes, 32)?,
        get_u64(bytes, 40)?,
        get_u64(bytes, 48)?,
        get_u64(bytes, 56)?,
        get_u64(bytes, 64)?,
        get_u64(bytes, 72)?,
    ];
    Ok(SessionTraceRecord {
        sequence_index: get_u64(bytes, 8)?,
        observed_at_ns: get_u64(bytes, 16)?,
        session_id: SessionId(get_u64(bytes, 24)?),
        kind: decode_kind(bytes[4], bytes[5], values)?,
    })
}

fn encode_kind(kind: SessionTraceRecordKind) -> (u8, u8, [u64; 6]) {
    match kind {
        SessionTraceRecordKind::Lifecycle { state } => (1, lifecycle_code(state), [0; 6]),
        SessionTraceRecordKind::SourceFailure { stem_id } => (2, 0, [stem_id.0, 0, 0, 0, 0, 0]),
        SessionTraceRecordKind::EndpointFailure {
            route_id,
            endpoint_id,
            stage_code,
        } => (3, stage_code, [route_id.0, endpoint_id.0, 0, 0, 0, 0]),
        SessionTraceRecordKind::RollbackFailure { stage } => {
            (4, rollback_stage_code(stage), [0; 6])
        }
        SessionTraceRecordKind::FinalizationFailure { stage } => {
            (5, finalization_stage_code(stage), [0; 6])
        }
        SessionTraceRecordKind::Terminal {
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
) -> Result<SessionTraceRecordKind, SessionTraceValidationError> {
    match kind_code {
        1 => Ok(SessionTraceRecordKind::Lifecycle {
            state: decode_lifecycle(detail_code)?,
        }),
        2 => Ok(SessionTraceRecordKind::SourceFailure {
            stem_id: StemId(values[0]),
        }),
        3 if (1..=5).contains(&detail_code) => Ok(SessionTraceRecordKind::EndpointFailure {
            route_id: RouteId(values[0]),
            endpoint_id: EndpointId(values[1]),
            stage_code: detail_code,
        }),
        3 => Err(SessionTraceValidationError::UnknownRecordType),
        4 => Ok(SessionTraceRecordKind::RollbackFailure {
            stage: decode_rollback_stage(detail_code)?,
        }),
        5 => Ok(SessionTraceRecordKind::FinalizationFailure {
            stage: decode_finalization_stage(detail_code)?,
        }),
        6 => Ok(SessionTraceRecordKind::Terminal {
            state: decode_terminal(detail_code)?,
            source_failures_total: values[0],
            endpoint_failures_total: values[1],
            rollback_failures_total: values[2],
            finalization_failures_total: values[3],
        }),
        _ => Err(SessionTraceValidationError::UnknownRecordType),
    }
}

fn validate_lifecycle_transition(
    previous: Option<SessionLifecycleState>,
    next: SessionLifecycleState,
) -> Result<(), SessionTraceValidationError> {
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
        Err(SessionTraceValidationError::InvalidLifecycleTransition)
    }
}

fn validate_terminal(
    lifecycle: Option<SessionLifecycleState>,
    terminal: SessionTerminalState,
) -> Result<(), SessionTraceValidationError> {
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
        Err(SessionTraceValidationError::TerminalMismatch)
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

fn decode_lifecycle(code: u8) -> Result<SessionLifecycleState, SessionTraceValidationError> {
    match code {
        1 => Ok(SessionLifecycleState::Starting),
        2 => Ok(SessionLifecycleState::Running),
        3 => Ok(SessionLifecycleState::Stopping),
        4 => Ok(SessionLifecycleState::Stopped),
        5 => Ok(SessionLifecycleState::Failed),
        _ => Err(SessionTraceValidationError::UnknownRecordType),
    }
}

fn terminal_code(state: SessionTerminalState) -> u8 {
    match state {
        SessionTerminalState::Stopped => 1,
        SessionTerminalState::Failed => 2,
    }
}

fn decode_terminal(code: u8) -> Result<SessionTerminalState, SessionTraceValidationError> {
    match code {
        1 => Ok(SessionTerminalState::Stopped),
        2 => Ok(SessionTerminalState::Failed),
        _ => Err(SessionTraceValidationError::UnknownRecordType),
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

fn decode_rollback_stage(code: u8) -> Result<SessionRollbackStage, SessionTraceValidationError> {
    match code {
        1 => Ok(SessionRollbackStage::CancelOperator),
        2 => Ok(SessionRollbackStage::CancelEndpointPreparation),
        3 => Ok(SessionRollbackStage::FinalizeStartedEndpoint),
        4 => Ok(SessionRollbackStage::StopOpenedCapture),
        5 => Ok(SessionRollbackStage::DiscardRuntimeQueues),
        _ => Err(SessionTraceValidationError::UnknownRecordType),
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
        SessionFinalizationStage::DrainSidecar => 7,
    }
}

fn decode_finalization_stage(
    code: u8,
) -> Result<SessionFinalizationStage, SessionTraceValidationError> {
    match code {
        1 => Ok(SessionFinalizationStage::StopCapture),
        2 => Ok(SessionFinalizationStage::DrainRuntime),
        3 => Ok(SessionFinalizationStage::DrainOperator),
        4 => Ok(SessionFinalizationStage::RequestEndpointStop),
        5 => Ok(SessionFinalizationStage::JoinEndpoint),
        6 => Ok(SessionFinalizationStage::FinalizeEndpoint),
        7 => Ok(SessionFinalizationStage::DrainSidecar),
        _ => Err(SessionTraceValidationError::UnknownRecordType),
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

fn get_u16(bytes: &[u8], offset: usize) -> Result<u16, SessionTraceValidationError> {
    bytes
        .get(offset..offset + 2)
        .and_then(|value| value.try_into().ok())
        .map(u16::from_le_bytes)
        .ok_or(SessionTraceValidationError::Truncated)
}

fn get_u32(bytes: &[u8], offset: usize) -> Result<u32, SessionTraceValidationError> {
    bytes
        .get(offset..offset + 4)
        .and_then(|value| value.try_into().ok())
        .map(u32::from_le_bytes)
        .ok_or(SessionTraceValidationError::Truncated)
}

fn get_u64(bytes: &[u8], offset: usize) -> Result<u64, SessionTraceValidationError> {
    bytes
        .get(offset..offset + 8)
        .and_then(|value| value.try_into().ok())
        .map(u64::from_le_bytes)
        .ok_or(SessionTraceValidationError::Truncated)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::session::{SessionEventKind, SessionTerminalOutcome};

    fn event(session_id: SessionId, kind: SessionEventKind) -> SessionEvent {
        SessionEvent::new(session_id, kind)
    }

    fn complete_trace(path: &Path) -> SessionTraceRecorderOutcome {
        let session_id = SessionId(17);
        let mut recorder = SessionTraceRecorder::start(path, session_id, 16).unwrap();
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
                SessionTraceRecordDelivery::Enqueued
            );
        }
        recorder.finish().unwrap().clone()
    }

    fn record(
        sequence_index: u64,
        observed_at_ns: u64,
        session_id: SessionId,
        kind: SessionTraceRecordKind,
    ) -> SessionTraceRecord {
        SessionTraceRecord {
            sequence_index,
            observed_at_ns,
            session_id,
            kind,
        }
    }

    fn synthetic_trace(
        records: Vec<SessionTraceRecord>,
        outcome: SessionTraceRecorderOutcome,
    ) -> SessionTrace {
        SessionTrace {
            session_id: SessionId(17),
            records: records.into_boxed_slice(),
            outcome,
        }
    }

    fn complete_outcome(record_count: u64) -> SessionTraceRecorderOutcome {
        SessionTraceRecorderOutcome {
            path: PathBuf::from("synthetic.pkstrace"),
            records_attempted_total: record_count,
            records_enqueued_total: record_count,
            records_dropped_total: 0,
            records_written_total: record_count,
            rolling_hash: 0,
        }
    }

    fn stopped_records() -> Vec<SessionTraceRecord> {
        let session_id = SessionId(17);
        vec![
            record(
                0,
                10,
                session_id,
                SessionTraceRecordKind::Lifecycle {
                    state: SessionLifecycleState::Starting,
                },
            ),
            record(
                1,
                20,
                session_id,
                SessionTraceRecordKind::Lifecycle {
                    state: SessionLifecycleState::Running,
                },
            ),
            record(
                2,
                30,
                session_id,
                SessionTraceRecordKind::Lifecycle {
                    state: SessionLifecycleState::Stopping,
                },
            ),
            record(
                3,
                40,
                session_id,
                SessionTraceRecordKind::Lifecycle {
                    state: SessionLifecycleState::Stopped,
                },
            ),
            record(
                4,
                50,
                session_id,
                SessionTraceRecordKind::Terminal {
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
    fn given_complete_trace_when_validated_then_lifecycle_and_terminal_match() {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("session.pkstrace");
        let outcome = complete_trace(&path);
        assert!(outcome.is_complete());

        let trace = SessionTrace::read(&path).unwrap();
        let validation = trace.validate().unwrap();

        assert_eq!(validation.session_id, SessionId(17));
        assert_eq!(
            validation.lifecycle.as_ref(),
            &[
                SessionLifecycleState::Starting,
                SessionLifecycleState::Running,
                SessionLifecycleState::Stopping,
                SessionLifecycleState::Stopped,
            ]
        );
        assert_eq!(validation.terminal.state, SessionTerminalState::Stopped);
        assert_eq!(validation.records_validated_total, 5);
    }

    #[test]
    fn given_corrupted_record_when_read_then_checksum_is_rejected() {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("session.pkstrace");
        complete_trace(&path);
        let mut bytes = std::fs::read(&path).unwrap();
        bytes[HEADER_SIZE_BYTES + 20] ^= 0x80;
        std::fs::write(&path, bytes).unwrap();

        assert!(matches!(
            SessionTrace::read(&path),
            Err(SessionTraceValidationError::InvalidChecksum)
        ));
    }

    #[test]
    fn given_truncated_trace_when_read_then_truncation_is_rejected() {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("session.pkstrace");
        complete_trace(&path);
        let mut bytes = std::fs::read(&path).unwrap();
        bytes.truncate(HEADER_SIZE_BYTES + 12);
        std::fs::write(&path, bytes).unwrap();

        assert!(matches!(
            SessionTrace::read(&path),
            Err(SessionTraceValidationError::Truncated)
        ));
    }

    #[test]
    fn given_unknown_version_when_read_then_version_is_rejected() {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("session.pkstrace");
        complete_trace(&path);
        let mut bytes = std::fs::read(&path).unwrap();
        put_u16(&mut bytes, 8, TRACE_VERSION + 1);
        std::fs::write(&path, bytes).unwrap();

        assert!(matches!(
            SessionTrace::read(&path),
            Err(SessionTraceValidationError::UnsupportedVersion)
        ));
    }

    #[test]
    fn given_invalid_lifecycle_when_validated_then_validation_fails_closed() {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("invalid.pkstrace");
        let session_id = SessionId(23);
        let mut recorder = SessionTraceRecorder::start(&path, session_id, 8).unwrap();
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
                SessionTraceRecordDelivery::Enqueued
            );
        }
        recorder.finish().unwrap();

        let trace = SessionTrace::read(&path).unwrap();
        assert!(matches!(
            trace.validate(),
            Err(SessionTraceValidationError::InvalidLifecycleTransition)
        ));
    }

    #[test]
    fn given_existing_output_when_started_then_recorder_fails_closed() {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("existing.pkstrace");
        std::fs::write(&path, b"existing").unwrap();

        assert!(matches!(
            SessionTraceRecorder::start(&path, SessionId(1), 8),
            Err(SessionTraceRecorderStartError::OutputExists { .. })
        ));
        assert_eq!(std::fs::read(&path).unwrap(), b"existing");
    }

    #[test]
    fn given_zero_capacity_when_started_then_recorder_fails_closed() {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("zero-capacity.pkstrace");

        assert!(matches!(
            SessionTraceRecorder::start(&path, SessionId(1), 0),
            Err(SessionTraceRecorderStartError::ZeroCapacity)
        ));
        assert!(!path.exists());
    }

    #[test]
    fn given_dropped_records_when_validated_then_trace_is_incomplete() {
        let records = stopped_records();
        let trace = synthetic_trace(
            records,
            SessionTraceRecorderOutcome {
                path: PathBuf::from("synthetic.pkstrace"),
                records_attempted_total: 6,
                records_enqueued_total: 5,
                records_dropped_total: 1,
                records_written_total: 5,
                rolling_hash: 0,
            },
        );

        assert!(matches!(
            trace.validate(),
            Err(SessionTraceValidationError::IncompleteTrace)
        ));
    }

    #[test]
    fn given_sequence_gap_when_validated_then_trace_is_rejected() {
        let mut records = stopped_records();
        records[2].sequence_index = 3;
        let trace = synthetic_trace(records, complete_outcome(5));

        assert!(matches!(
            trace.validate(),
            Err(SessionTraceValidationError::SequenceGap)
        ));
    }

    #[test]
    fn given_timestamp_regression_when_validated_then_trace_is_rejected() {
        let mut records = stopped_records();
        records[2].observed_at_ns = 5;
        let trace = synthetic_trace(records, complete_outcome(5));

        assert!(matches!(
            trace.validate(),
            Err(SessionTraceValidationError::TimestampRegression)
        ));
    }

    #[test]
    fn given_record_after_terminal_when_validated_then_trace_is_rejected() {
        let mut records = stopped_records();
        records.push(record(
            5,
            60,
            SessionId(17),
            SessionTraceRecordKind::SourceFailure { stem_id: StemId(9) },
        ));
        let trace = synthetic_trace(records, complete_outcome(6));

        assert!(matches!(
            trace.validate(),
            Err(SessionTraceValidationError::RecordAfterTerminal)
        ));
    }
}
