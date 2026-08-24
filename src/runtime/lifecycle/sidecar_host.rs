use std::ffi::OsString;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::process::{Child, ChildStdin, ChildStdout, Command, ExitStatus, Stdio};
use std::sync::atomic::{AtomicBool, AtomicU64, AtomicU8, Ordering};
use std::sync::mpsc::{self, Receiver, SyncSender, TryRecvError, TrySendError};
use std::sync::Arc;
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

use super::{SidecarMessage, SidecarMessageKind, SidecarProtocolError, SidecarProtocolLimits};

const LENGTH_PREFIX_BYTES: usize = 4;
const CONTROL_CAPACITY_MESSAGES: usize = 4;
const WRITER_POLL_INTERVAL: Duration = Duration::from_millis(1);
const PROCESS_POLL_INTERVAL: Duration = Duration::from_millis(2);
pub(crate) const CONTROL_SIGNAL_ID: &str = "pks.sidecar.control.v1";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
#[doc = "Selects the sidecar state used by PocketStation."]
pub enum SidecarState {
    #[doc = "Indicates the spawned state for `SidecarState`."]
    Spawned = 0,
    #[doc = "Indicates the hello state for `SidecarState`."]
    Hello = 1,
    #[doc = "Indicates the manifest state for `SidecarState`."]
    Manifest = 2,
    #[doc = "Indicates the configure state for `SidecarState`."]
    Configure = 3,
    #[doc = "Indicates the ready state for `SidecarState`."]
    Ready = 4,
    #[doc = "Indicates the running state for `SidecarState`."]
    Running = 5,
    #[doc = "Indicates the cancelling state for `SidecarState`."]
    Cancelling = 6,
    #[doc = "Indicates the closing state for `SidecarState`."]
    Closing = 7,
    #[doc = "Reports that the underlying channel or resource is closed."]
    Closed = 8,
    #[doc = "Indicates the reaped state for `SidecarState`."]
    Reaped = 9,
    #[doc = "Indicates the failed state for `SidecarState`."]
    Failed = 10,
}

impl SidecarState {
    fn from_u8(value: u8) -> Self {
        match value {
            0 => Self::Spawned,
            1 => Self::Hello,
            2 => Self::Manifest,
            3 => Self::Configure,
            4 => Self::Ready,
            5 => Self::Running,
            6 => Self::Cancelling,
            7 => Self::Closing,
            8 => Self::Closed,
            9 => Self::Reaped,
            _ => Self::Failed,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "Sets finite startup, I/O, shutdown, and reap deadlines for a sidecar process."]
pub struct SidecarDeadlines {
    #[doc = "Indicates whether ready applies to `SidecarDeadlines`."]
    pub ready: Duration,
    #[doc = "Sets the processing duration enforced by `SidecarDeadlines`."]
    pub processing: Duration,
    #[doc = "Sets the shutdown duration enforced by `SidecarDeadlines`."]
    pub shutdown: Duration,
}

impl Default for SidecarDeadlines {
    #[doc = "Returns the default `SidecarDeadlines` value."]
    fn default() -> Self {
        Self {
            ready: Duration::from_secs(5),
            processing: Duration::from_secs(5),
            shutdown: Duration::from_secs(2),
        }
    }
}

#[derive(Debug, Clone)]
#[doc = "Configures sidecar process."]
pub struct SidecarProcessSpec {
    #[doc = "Identifies the id recorded by `SidecarProcessSpec`."]
    pub id: u64,
    #[doc = "Points to the executable launched for `SidecarProcessSpec`."]
    pub program: PathBuf,
    #[doc = "Contains the arguments owned or reported by `SidecarProcessSpec`."]
    pub arguments: Vec<OsString>,
    #[doc = "Contains the serialized configuration passed to `SidecarProcessSpec`."]
    pub configuration: Vec<u8>,
    #[doc = "Sets the data capacity messages available to `SidecarProcessSpec`."]
    pub data_capacity_messages: usize,
    #[doc = "Contains the protocol limits owned or reported by `SidecarProcessSpec`."]
    pub protocol_limits: SidecarProtocolLimits,
    #[doc = "Contains the deadlines owned or reported by `SidecarProcessSpec`."]
    pub deadlines: SidecarDeadlines,
}

impl SidecarProcessSpec {
    #[doc = "Creates a new `SidecarProcessSpec`."]
    pub fn new(id: u64, program: impl Into<PathBuf>) -> Self {
        Self {
            id,
            program: program.into(),
            arguments: Vec::new(),
            configuration: Vec::new(),
            data_capacity_messages: 64,
            protocol_limits: SidecarProtocolLimits::default(),
            deadlines: SidecarDeadlines::default(),
        }
    }
}

#[derive(Default)]
struct SidecarCounters {
    state: AtomicU8,
    state_transitions: AtomicU64,
    data_enqueued_total: AtomicU64,
    data_received_total: AtomicU64,
    data_dropped_total: AtomicU64,
    protocol_failures_total: AtomicU64,
    timeouts_total: AtomicU64,
    forced_kills_total: AtomicU64,
    reaps_total: AtomicU64,
}

#[derive(Clone)]
#[doc = "Reports the sidecar host observations collected at an observation boundary."]
pub struct SidecarHostObservations {
    counters: Arc<SidecarCounters>,
}

impl SidecarHostObservations {
    pub fn snapshot(&self) -> SidecarHostSnapshot {
        SidecarHostSnapshot {
            state: SidecarState::from_u8(self.counters.state.load(Ordering::Acquire)),
            state_transitions: self.counters.state_transitions.load(Ordering::Acquire),
            data_enqueued_total: self.counters.data_enqueued_total.load(Ordering::Acquire),
            data_received_total: self.counters.data_received_total.load(Ordering::Acquire),
            data_dropped_total: self.counters.data_dropped_total.load(Ordering::Acquire),
            protocol_failures_total: self
                .counters
                .protocol_failures_total
                .load(Ordering::Acquire),
            timeouts_total: self.counters.timeouts_total.load(Ordering::Acquire),
            forced_kills_total: self.counters.forced_kills_total.load(Ordering::Acquire),
            reaps_total: self.counters.reaps_total.load(Ordering::Acquire),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "Reports the sidecar host snapshot collected at an observation boundary."]
pub struct SidecarHostSnapshot {
    #[doc = "Records the state selected for `SidecarHostSnapshot`."]
    pub state: SidecarState,
    #[doc = "Contains the state transitions owned or reported by `SidecarHostSnapshot`."]
    pub state_transitions: u64,
    #[doc = "Counts the total number of data enqueued observed by `SidecarHostSnapshot`."]
    pub data_enqueued_total: u64,
    #[doc = "Counts the total number of data received observed by `SidecarHostSnapshot`."]
    pub data_received_total: u64,
    #[doc = "Counts the total number of data dropped observed by `SidecarHostSnapshot`."]
    pub data_dropped_total: u64,
    #[doc = "Counts the total number of protocol failures observed by `SidecarHostSnapshot`."]
    pub protocol_failures_total: u64,
    #[doc = "Counts the total number of timeouts observed by `SidecarHostSnapshot`."]
    pub timeouts_total: u64,
    #[doc = "Counts the total number of forced kills observed by `SidecarHostSnapshot`."]
    pub forced_kills_total: u64,
    #[doc = "Counts the total number of reaps observed by `SidecarHostSnapshot`."]
    pub reaps_total: u64,
}

impl SidecarHostSnapshot {
    #[doc = "Returns the visited held by `SidecarHostSnapshot`."]
    pub const fn visited(self, state: SidecarState) -> bool {
        self.state_transitions & (1u64 << state as u8) != 0
    }
}

enum ReaderEvent {
    Control(SidecarMessage),
    Failure(SidecarHostError),
    Eof,
}

#[doc = "Owns the resources and lifecycle for sidecar."]
pub struct SidecarHost {
    id: u64,
    child: Option<Child>,
    data_tx: Option<SyncSender<SidecarMessage>>,
    control_tx: Option<SyncSender<SidecarMessage>>,
    incoming_data_rx: Receiver<SidecarMessage>,
    reader_event_rx: Receiver<ReaderEvent>,
    stop_threads: Arc<AtomicBool>,
    writer: Option<JoinHandle<Result<(), SidecarHostError>>>,
    reader: Option<JoinHandle<()>>,
    deadlines: SidecarDeadlines,
    manifest: Vec<u8>,
    observations: SidecarHostObservations,
    reaped: bool,
}

impl SidecarHost {
    #[doc = "Spawns its owned operation for `SidecarHost`."]
    pub fn spawn(spec: SidecarProcessSpec) -> Result<Self, SidecarHostError> {
        validate_spec(&spec)?;
        let mut child = Command::new(&spec.program)
            .args(&spec.arguments)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .map_err(|error| SidecarHostError::Spawn(error.to_string()))?;
        let Some(stdin) = child.stdin.take() else {
            reap_failed_spawn(&mut child);
            return Err(SidecarHostError::MissingPipe("stdin"));
        };
        let Some(stdout) = child.stdout.take() else {
            reap_failed_spawn(&mut child);
            return Err(SidecarHostError::MissingPipe("stdout"));
        };
        let (data_tx, data_rx) = mpsc::sync_channel(spec.data_capacity_messages);
        let (control_tx, control_rx) = mpsc::sync_channel(CONTROL_CAPACITY_MESSAGES);
        let (incoming_data_tx, incoming_data_rx) = mpsc::sync_channel(spec.data_capacity_messages);
        let (reader_event_tx, reader_event_rx) = mpsc::sync_channel(CONTROL_CAPACITY_MESSAGES);
        let stop_threads = Arc::new(AtomicBool::new(false));
        let counters = Arc::new(SidecarCounters::default());
        counters
            .state
            .store(SidecarState::Spawned as u8, Ordering::Release);
        counters
            .state_transitions
            .store(1u64 << SidecarState::Spawned as u8, Ordering::Release);
        let writer = spawn_writer(
            stdin,
            data_rx,
            control_rx,
            Arc::clone(&stop_threads),
            spec.protocol_limits,
        )?;
        let reader = match spawn_reader(
            stdout,
            incoming_data_tx,
            reader_event_tx,
            Arc::clone(&stop_threads),
            spec.protocol_limits,
            Arc::clone(&counters),
        ) {
            Ok(reader) => reader,
            Err(error) => {
                stop_threads.store(true, Ordering::Release);
                drop(data_tx);
                drop(control_tx);
                reap_failed_spawn(&mut child);
                let _ = writer.join();
                return Err(error);
            }
        };
        let observations = SidecarHostObservations { counters };
        let mut host = Self {
            id: spec.id,
            child: Some(child),
            data_tx: Some(data_tx),
            control_tx: Some(control_tx),
            incoming_data_rx,
            reader_event_rx,
            stop_threads,
            writer: Some(writer),
            reader: Some(reader),
            deadlines: spec.deadlines,
            manifest: Vec::new(),
            observations,
            reaped: false,
        };
        let handshake = host.handshake(spec.configuration);
        if let Err(error) = handshake {
            host.mark_failed(&error);
            host.force_reap();
            return Err(error);
        }
        Ok(host)
    }

    #[doc = "Returns the id held by `SidecarHost`."]
    pub const fn id(&self) -> u64 {
        self.id
    }

    #[doc = "Returns the state held by `SidecarHost`."]
    pub fn state(&self) -> SidecarState {
        self.observations.snapshot().state
    }

    #[doc = "Returns the observations exposed by `SidecarHost`."]
    pub fn observations(&self) -> SidecarHostObservations {
        self.observations.clone()
    }

    #[doc = "Attempts to send signal through `SidecarHost`."]
    pub fn try_send_signal(&self, message: SidecarMessage) -> Result<(), SidecarHostError> {
        if self.state() != SidecarState::Running {
            return Err(SidecarHostError::InvalidState {
                expected: SidecarState::Running,
                actual: self.state(),
            });
        }
        if message.kind != SidecarMessageKind::Signal {
            return Err(SidecarHostError::InvalidDataKind(message.kind));
        }
        match self
            .data_tx
            .as_ref()
            .ok_or(SidecarHostError::Closed)?
            .try_send(message)
        {
            Ok(()) => {
                self.observations
                    .counters
                    .data_enqueued_total
                    .fetch_add(1, Ordering::Relaxed);
                Ok(())
            }
            Err(TrySendError::Full(_)) => {
                self.observations
                    .counters
                    .data_dropped_total
                    .fetch_add(1, Ordering::Relaxed);
                Err(SidecarHostError::DataQueueFull)
            }
            Err(TrySendError::Disconnected(_)) => Err(SidecarHostError::Closed),
        }
    }

    #[doc = "Attempts to receive signal through `SidecarHost`."]
    pub fn try_receive_signal(&self) -> Result<Option<SidecarMessage>, SidecarHostError> {
        match self.incoming_data_rx.try_recv() {
            Ok(message) => Ok(Some(message)),
            Err(TryRecvError::Empty) => Ok(None),
            Err(TryRecvError::Disconnected) => Err(SidecarHostError::Closed),
        }
    }

    #[doc = "Receives and decodes the next signal message from `SidecarHost`."]
    pub fn receive_signal(&self) -> Result<SidecarMessage, SidecarHostError> {
        self.incoming_data_rx
            .recv_timeout(self.deadlines.processing)
            .map_err(|error| match error {
                mpsc::RecvTimeoutError::Timeout => {
                    self.observations
                        .counters
                        .timeouts_total
                        .fetch_add(1, Ordering::Relaxed);
                    SidecarHostError::ProcessingTimeout
                }
                mpsc::RecvTimeoutError::Disconnected => SidecarHostError::Closed,
            })
    }

    #[doc = "Cancels and reap for `SidecarHost`."]
    pub fn cancel_and_reap(&mut self) -> Result<ExitStatus, SidecarHostError> {
        self.shutdown(SidecarMessageKind::Cancel, SidecarState::Cancelling)
    }

    #[doc = "Closes `SidecarHost` and reaps its child process."]
    pub fn close_and_reap(&mut self) -> Result<ExitStatus, SidecarHostError> {
        self.shutdown(SidecarMessageKind::Close, SidecarState::Closing)
    }

    fn handshake(&mut self, configuration: Vec<u8>) -> Result<(), SidecarHostError> {
        self.send_control(control_message(SidecarMessageKind::Hello, Vec::new()))?;
        self.expect_control(SidecarMessageKind::Hello, self.deadlines.ready)?;
        self.set_state(SidecarState::Hello);
        let manifest = self.expect_control(SidecarMessageKind::Manifest, self.deadlines.ready)?;
        self.manifest = manifest.payload;
        self.set_state(SidecarState::Manifest);
        self.send_control(control_message(
            SidecarMessageKind::Configure,
            configuration,
        ))?;
        self.set_state(SidecarState::Configure);
        self.expect_control(SidecarMessageKind::Ready, self.deadlines.ready)?;
        self.set_state(SidecarState::Ready);
        self.set_state(SidecarState::Running);
        Ok(())
    }

    fn shutdown(
        &mut self,
        kind: SidecarMessageKind,
        state: SidecarState,
    ) -> Result<ExitStatus, SidecarHostError> {
        if self.reaped {
            return Err(SidecarHostError::AlreadyReaped);
        }
        self.set_state(state);
        let acknowledgement = self
            .send_control(control_message(kind, Vec::new()))
            .and_then(|()| {
                self.expect_control(SidecarMessageKind::Closed, self.deadlines.shutdown)
            });
        if acknowledgement.is_err() {
            self.kill_if_running();
        } else {
            self.set_state(SidecarState::Closed);
        }
        let status = self.wait_and_reap(self.deadlines.shutdown)?;
        self.stop_io_threads();
        acknowledgement?;
        Ok(status)
    }

    fn send_control(&self, message: SidecarMessage) -> Result<(), SidecarHostError> {
        self.control_tx
            .as_ref()
            .ok_or(SidecarHostError::Closed)?
            .try_send(message)
            .map_err(|error| match error {
                TrySendError::Full(_) => SidecarHostError::ControlQueueFull,
                TrySendError::Disconnected(_) => SidecarHostError::Closed,
            })
    }

    fn expect_control(
        &self,
        expected: SidecarMessageKind,
        deadline: Duration,
    ) -> Result<SidecarMessage, SidecarHostError> {
        match self.reader_event_rx.recv_timeout(deadline) {
            Ok(ReaderEvent::Control(message)) if message.kind == expected => Ok(message),
            Ok(ReaderEvent::Control(message)) => Err(SidecarHostError::UnexpectedMessage {
                expected,
                actual: message.kind,
            }),
            Ok(ReaderEvent::Failure(error)) => Err(error),
            Ok(ReaderEvent::Eof) => Err(SidecarHostError::UnexpectedEof),
            Err(mpsc::RecvTimeoutError::Timeout) => {
                self.observations
                    .counters
                    .timeouts_total
                    .fetch_add(1, Ordering::Relaxed);
                Err(SidecarHostError::Timeout(expected))
            }
            Err(mpsc::RecvTimeoutError::Disconnected) => Err(SidecarHostError::Closed),
        }
    }

    fn wait_and_reap(&mut self, deadline: Duration) -> Result<ExitStatus, SidecarHostError> {
        let started = Instant::now();
        loop {
            let child = self.child.as_mut().ok_or(SidecarHostError::AlreadyReaped)?;
            if let Some(status) = child
                .try_wait()
                .map_err(|error| SidecarHostError::Wait(error.to_string()))?
            {
                self.child.take();
                self.reaped = true;
                self.observations
                    .counters
                    .reaps_total
                    .fetch_add(1, Ordering::Relaxed);
                self.set_state(SidecarState::Reaped);
                return Ok(status);
            }
            if started.elapsed() >= deadline {
                self.observations
                    .counters
                    .timeouts_total
                    .fetch_add(1, Ordering::Relaxed);
                child
                    .kill()
                    .map_err(|error| SidecarHostError::Kill(error.to_string()))?;
                self.observations
                    .counters
                    .forced_kills_total
                    .fetch_add(1, Ordering::Relaxed);
                let status = child
                    .wait()
                    .map_err(|error| SidecarHostError::Wait(error.to_string()))?;
                self.child.take();
                self.reaped = true;
                self.observations
                    .counters
                    .reaps_total
                    .fetch_add(1, Ordering::Relaxed);
                self.set_state(SidecarState::Reaped);
                return Ok(status);
            }
            thread::sleep(PROCESS_POLL_INTERVAL);
        }
    }

    fn set_state(&self, state: SidecarState) {
        self.observations
            .counters
            .state
            .store(state as u8, Ordering::Release);
        self.observations
            .counters
            .state_transitions
            .fetch_or(1u64 << state as u8, Ordering::AcqRel);
    }

    fn mark_failed(&self, error: &SidecarHostError) {
        self.set_state(SidecarState::Failed);
        if matches!(error, SidecarHostError::Protocol(_)) {
            self.observations
                .counters
                .protocol_failures_total
                .fetch_add(1, Ordering::Relaxed);
        }
    }

    fn kill_if_running(&mut self) {
        let Some(child) = self.child.as_mut() else {
            return;
        };
        let running = child.try_wait().ok().flatten().is_none();
        if running && child.kill().is_ok() {
            self.observations
                .counters
                .forced_kills_total
                .fetch_add(1, Ordering::Relaxed);
        }
    }

    fn force_reap(&mut self) {
        if let Some(child) = self.child.as_mut() {
            let running = child.try_wait().ok().flatten().is_none();
            if running && child.kill().is_ok() {
                self.observations
                    .counters
                    .forced_kills_total
                    .fetch_add(1, Ordering::Relaxed);
            }
            let _ = child.wait();
            self.observations
                .counters
                .reaps_total
                .fetch_add(1, Ordering::Relaxed);
        }
        self.child.take();
        self.reaped = true;
        self.stop_io_threads();
    }

    fn stop_io_threads(&mut self) {
        self.stop_threads.store(true, Ordering::Release);
        self.data_tx.take();
        self.control_tx.take();
        if let Some(writer) = self.writer.take() {
            let _ = writer.join();
        }
        if let Some(reader) = self.reader.take() {
            let _ = reader.join();
        }
    }
}

impl Drop for SidecarHost {
    #[doc = "Releases resources owned by `SidecarHost`."]
    fn drop(&mut self) {
        if !self.reaped {
            self.force_reap();
        }
    }
}

fn spawn_writer(
    mut stdin: ChildStdin,
    data_rx: Receiver<SidecarMessage>,
    control_rx: Receiver<SidecarMessage>,
    stop: Arc<AtomicBool>,
    limits: SidecarProtocolLimits,
) -> Result<JoinHandle<Result<(), SidecarHostError>>, SidecarHostError> {
    thread::Builder::new()
        .name("pks-sidecar-writer".to_owned())
        .spawn(move || {
            while !stop.load(Ordering::Acquire) {
                match control_rx.try_recv() {
                    Ok(message) => write_message(&mut stdin, &message, limits)?,
                    Err(TryRecvError::Disconnected) => break,
                    Err(TryRecvError::Empty) => match data_rx.try_recv() {
                        Ok(message) => write_message(&mut stdin, &message, limits)?,
                        Err(TryRecvError::Disconnected) => break,
                        Err(TryRecvError::Empty) => thread::sleep(WRITER_POLL_INTERVAL),
                    },
                }
            }
            Ok(())
        })
        .map_err(|error| SidecarHostError::ThreadSpawn(error.to_string()))
}

fn spawn_reader(
    mut stdout: ChildStdout,
    incoming_data_tx: SyncSender<SidecarMessage>,
    reader_event_tx: SyncSender<ReaderEvent>,
    stop: Arc<AtomicBool>,
    limits: SidecarProtocolLimits,
    counters: Arc<SidecarCounters>,
) -> Result<JoinHandle<()>, SidecarHostError> {
    thread::Builder::new()
        .name("pks-sidecar-reader".to_owned())
        .spawn(move || {
            while !stop.load(Ordering::Acquire) {
                match read_message(&mut stdout, limits) {
                    Ok(message) if matches!(message.kind, SidecarMessageKind::Signal) => {
                        match incoming_data_tx.try_send(message) {
                            Ok(()) => {
                                counters.data_received_total.fetch_add(1, Ordering::Relaxed);
                            }
                            Err(TrySendError::Full(_)) => {
                                counters.data_dropped_total.fetch_add(1, Ordering::Relaxed);
                            }
                            Err(TrySendError::Disconnected(_)) => break,
                        }
                    }
                    Ok(message) => {
                        if reader_event_tx
                            .try_send(ReaderEvent::Control(message))
                            .is_err()
                        {
                            counters
                                .protocol_failures_total
                                .fetch_add(1, Ordering::Relaxed);
                            break;
                        }
                    }
                    Err(SidecarHostError::UnexpectedEof) => {
                        let _ = reader_event_tx.try_send(ReaderEvent::Eof);
                        break;
                    }
                    Err(error) => {
                        counters
                            .protocol_failures_total
                            .fetch_add(1, Ordering::Relaxed);
                        let _ = reader_event_tx.try_send(ReaderEvent::Failure(error));
                        break;
                    }
                }
            }
        })
        .map_err(|error| SidecarHostError::ThreadSpawn(error.to_string()))
}

fn reap_failed_spawn(child: &mut Child) {
    let _ = child.kill();
    let _ = child.wait();
}

fn write_message(
    writer: &mut impl Write,
    message: &SidecarMessage,
    limits: SidecarProtocolLimits,
) -> Result<(), SidecarHostError> {
    let frame = message.encode(limits)?;
    let frame_bytes = u32::try_from(frame.len()).map_err(|_| SidecarHostError::FrameTooLarge)?;
    writer
        .write_all(&frame_bytes.to_le_bytes())
        .and_then(|()| writer.write_all(&frame))
        .and_then(|()| writer.flush())
        .map_err(|error| SidecarHostError::Io(error.to_string()))
}

fn read_message(
    reader: &mut impl Read,
    limits: SidecarProtocolLimits,
) -> Result<SidecarMessage, SidecarHostError> {
    let mut prefix = [0u8; LENGTH_PREFIX_BYTES];
    read_exact_or_eof(reader, &mut prefix)?;
    let frame_bytes = u32::from_le_bytes(prefix) as usize;
    if frame_bytes > limits.max_frame_bytes()? {
        return Err(SidecarHostError::FrameTooLarge);
    }
    let mut frame = vec![0u8; frame_bytes];
    read_exact_or_eof(reader, &mut frame)?;
    SidecarMessage::decode(&frame, limits).map_err(SidecarHostError::from)
}

fn read_exact_or_eof(reader: &mut impl Read, bytes: &mut [u8]) -> Result<(), SidecarHostError> {
    reader.read_exact(bytes).map_err(|error| {
        if error.kind() == std::io::ErrorKind::UnexpectedEof {
            SidecarHostError::UnexpectedEof
        } else {
            SidecarHostError::Io(error.to_string())
        }
    })
}

fn control_message(kind: SidecarMessageKind, payload: Vec<u8>) -> SidecarMessage {
    SidecarMessage {
        kind,
        terminal: false,
        stream_id: 0,
        sequence_number: 0,
        timestamp_ns: 0,
        signal_id: CONTROL_SIGNAL_ID.to_owned(),
        role: None,
        schema: None,
        payload,
    }
}

fn validate_spec(spec: &SidecarProcessSpec) -> Result<(), SidecarHostError> {
    if spec.program.as_os_str().is_empty() {
        return Err(SidecarHostError::InvalidConfiguration("program is empty"));
    }
    if spec.data_capacity_messages == 0 {
        return Err(SidecarHostError::InvalidConfiguration(
            "data capacity must be greater than zero messages",
        ));
    }
    if spec.deadlines.ready.is_zero()
        || spec.deadlines.processing.is_zero()
        || spec.deadlines.shutdown.is_zero()
    {
        return Err(SidecarHostError::InvalidConfiguration(
            "deadlines must be greater than zero",
        ));
    }
    spec.protocol_limits.max_frame_bytes()?;
    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[doc = "Classifies failures surfaced by sidecar host operations."]
pub enum SidecarHostError {
    #[error("sidecar configuration is invalid: {0}")]
    #[doc = "Reports that the supplied configuration is invalid."]
    InvalidConfiguration(&'static str),
    #[error("sidecar spawn failed: {0}")]
    #[doc = "Classifies a failure at the spawn stage or component of `SidecarHostError`."]
    Spawn(String),
    #[error("sidecar I/O thread spawn failed: {0}")]
    #[doc = "Classifies a failure at the thread spawn stage or component of `SidecarHostError`."]
    ThreadSpawn(String),
    #[error("sidecar child did not expose {0}")]
    #[doc = "Reports that the required pipe is missing."]
    MissingPipe(&'static str),
    #[error("sidecar I/O failed: {0}")]
    #[doc = "Reports an operating-system or filesystem I/O failure."]
    Io(String),
    #[error("sidecar protocol failed: {0}")]
    #[doc = "Classifies a failure at the protocol stage or component of `SidecarHostError`."]
    Protocol(#[from] SidecarProtocolError),
    #[error("sidecar frame exceeds the configured bound")]
    #[doc = "Reports that frame exceeds the supported size limit."]
    FrameTooLarge,
    #[error("sidecar data queue is full")]
    #[doc = "Reports that the bounded data queue has no remaining capacity."]
    DataQueueFull,
    #[error("sidecar reserved control queue is full")]
    #[doc = "Reports that the bounded control queue has no remaining capacity."]
    ControlQueueFull,
    #[error("sidecar channel is closed")]
    #[doc = "Reports that the underlying channel or resource is closed."]
    Closed,
    #[error("sidecar stdout closed unexpectedly")]
    #[doc = "Reports that eof is not valid in the current protocol or lifecycle state."]
    UnexpectedEof,
    #[error("sidecar expected {expected:?} but received {actual:?}")]
    #[doc = "Reports that message is not valid in the current protocol or lifecycle state."]
    UnexpectedMessage {
        #[doc = "Records the value expected by `UnexpectedMessage`."]
        expected: SidecarMessageKind,
        #[doc = "Records the value observed by `UnexpectedMessage`."]
        actual: SidecarMessageKind,
    },
    #[error("sidecar timed out waiting for {0:?}")]
    #[doc = "Reports that the operation exceeded its deadline."]
    Timeout(SidecarMessageKind),
    #[error("sidecar timed out waiting for a processed signal")]
    #[doc = "Reports that processing exceeded its deadline."]
    ProcessingTimeout,
    #[error("sidecar expected state {expected:?} but is {actual:?}")]
    #[doc = "Reports that the supplied state is invalid."]
    InvalidState {
        #[doc = "Records the value expected by `InvalidState`."]
        expected: SidecarState,
        #[doc = "Records the value observed by `InvalidState`."]
        actual: SidecarState,
    },
    #[error("sidecar data queue accepts Signal messages, not {0:?}")]
    #[doc = "Reports that the supplied data kind is invalid."]
    InvalidDataKind(SidecarMessageKind),
    #[error("sidecar process wait failed: {0}")]
    #[doc = "Classifies a failure at the wait stage or component of `SidecarHostError`."]
    Wait(String),
    #[error("sidecar process kill failed: {0}")]
    #[doc = "Classifies a failure at the kill stage or component of `SidecarHostError`."]
    Kill(String),
    #[error("sidecar process was already reaped")]
    #[doc = "Reports that reaped already occurred before this operation."]
    AlreadyReaped,
    #[error("sidecar process ID {0} is not owned by this Session")]
    #[doc = "Reports that the referenced sidecar is not declared or registered."]
    UnknownSidecar(u64),
}
