//! Deterministic canonical-engine fixture for external conformance harnesses.
//!
//! This feature is `LOOPBACK-ONLY`, disabled by default, and is not product
//! capture evidence.

use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, OnceLock};
use std::time::Duration;
use std::{path::PathBuf, thread};

use crate::capture::{
    ActiveCaptureBackend, CallbackCaptureBackend, CaptureDelivery, CaptureError, CaptureMode,
    CaptureObservationHandle, CaptureObservations, CapturedFrameDelivery, PreparedCaptureBackend,
};
use crate::frame::{AudioBufferPool, AudioFrame, SampleFormat, SampleSpec, SourceId, StreamId};
use crate::graph::PrepareContext;
use crate::session::{
    EndpointConfiguration, EndpointHandle, NativeSessionEngineHostOptions, OperatorId,
    PolledAudioEndpointConfig, SessionEngineHostBuildError, SessionEngineHostBuilder,
};

use crate::{
    EndpointCancellationOutcome, EndpointDriverFactory, EndpointDriverFinalization,
    EndpointDriverInput, EndpointDriverObservations, EndpointFailure, EndpointFailureStage,
    EndpointStartGate, PreparedEndpointDriver, RunningEndpointDriver, Session,
    SessionEndpointError, SessionError,
};

const FRAME_SAMPLES_PER_CHANNEL: usize = 960;
const FRAME_DURATION_NS: u64 = 20_000_000;
const FRAME_PACING_MS: u64 = 20;
const RECORDING_EDGE_CAPACITY_FRAMES: usize = crate::graph::plan::EDGE_RING_CAPACITY_FRAMES;
const SLOW_BRANCH_QUEUE_CAPACITY_FRAMES: usize = RECORDING_EDGE_CAPACITY_FRAMES / 2;
/// Frames emitted per source by the finite deterministic fixture.
///
/// This equals the canonical runtime edge capacity, so the recording branch
/// remains lossless even if its worker is not scheduled until capture ends.
/// The independently configured half-capacity polled branch still saturates.
pub const FRAMES_PER_SOURCE: u64 = RECORDING_EDGE_CAPACITY_FRAMES as u64;
pub const OBSERVED_CONNECTOR_OPERATOR_ID: &str = "io.pocketstation.conformance.connector.v1";

#[derive(Clone, Copy)]
enum FixtureSource {
    Application,
    Microphone,
}

impl FixtureSource {
    const fn stream_id(self) -> StreamId {
        match self {
            Self::Application => StreamId(101),
            Self::Microphone => StreamId(201),
        }
    }

    const fn source_id(self) -> SourceId {
        match self {
            Self::Application => SourceId(102),
            Self::Microphone => SourceId(202),
        }
    }

    const fn amplitude(self) -> f32 {
        match self {
            Self::Application => 0.25,
            Self::Microphone => 0.5,
        }
    }

    const fn channels(self) -> u8 {
        match self {
            Self::Application => 2,
            Self::Microphone => 1,
        }
    }
}

struct DeterministicCaptureBackend {
    timestamp_origin_ns: Arc<OnceLock<u64>>,
}

struct DeterministicPreparedCapture {
    source: FixtureSource,
    timestamp_origin_ns: Arc<OnceLock<u64>>,
}

struct DeterministicActiveCapture {
    stop_requested: Arc<AtomicBool>,
    worker: Option<std::thread::JoinHandle<()>>,
    source_id: SourceId,
}

impl CallbackCaptureBackend for DeterministicCaptureBackend {
    fn prepare(&self, mode: CaptureMode) -> Result<Box<dyn PreparedCaptureBackend>, CaptureError> {
        let source = match mode {
            CaptureMode::InputDevice(_) => FixtureSource::Microphone,
            CaptureMode::SystemMix
            | CaptureMode::Application(_)
            | CaptureMode::Process(_)
            | CaptureMode::ExactApplication { .. }
            | CaptureMode::ExactApplicationStable { .. } => FixtureSource::Application,
        };
        Ok(Box::new(DeterministicPreparedCapture {
            source,
            timestamp_origin_ns: Arc::clone(&self.timestamp_origin_ns),
        }))
    }
}

impl PreparedCaptureBackend for DeterministicPreparedCapture {
    fn open(
        self: Box<Self>,
        mut delivery: CaptureDelivery,
    ) -> Result<Box<dyn ActiveCaptureBackend>, CaptureError> {
        let stop_requested = Arc::new(AtomicBool::new(false));
        let worker_stop_requested = Arc::clone(&stop_requested);
        let source = self.source;
        let timestamp_origin_ns = *self
            .timestamp_origin_ns
            .get_or_init(|| crate::timing::monotonic_timestamp_ns().saturating_add(1_000_000));
        let worker = std::thread::spawn(move || {
            let samples_per_frame =
                FRAME_SAMPLES_PER_CHANNEL.saturating_mul(usize::from(source.channels()));
            let pool = AudioBufferPool::new(32, samples_per_frame);
            let mut sequence = 0_u64;
            while !worker_stop_requested.load(Ordering::Acquire) && sequence < FRAMES_PER_SOURCE {
                let Some(mut buffer) = pool.acquire() else {
                    thread::sleep(Duration::from_millis(1));
                    continue;
                };
                buffer.as_mut_slice().fill(source.amplitude());
                let frame = AudioFrame::new(
                    source.stream_id(),
                    source.source_id(),
                    sequence,
                    timestamp_origin_ns + sequence.saturating_mul(FRAME_DURATION_NS),
                    source.channels(),
                    buffer,
                );
                match delivery.frame_sender.try_send(frame) {
                    CapturedFrameDelivery::Delivered => {
                        sequence = sequence.saturating_add(1);
                        thread::sleep(Duration::from_millis(FRAME_PACING_MS));
                    }
                    CapturedFrameDelivery::DroppedNewest
                    | CapturedFrameDelivery::DiscardedBeforeStart => {
                        thread::sleep(Duration::from_millis(1));
                    }
                }
            }
        });
        Ok(Box::new(DeterministicActiveCapture {
            stop_requested,
            worker: Some(worker),
            source_id: source.source_id(),
        }))
    }
}

impl ActiveCaptureBackend for DeterministicActiveCapture {
    fn source_id(&self) -> SourceId {
        self.source_id
    }

    fn observation_handle(&self) -> CaptureObservationHandle {
        CaptureObservationHandle::default()
    }

    fn observations(&self) -> CaptureObservations {
        CaptureObservations::default()
    }

    fn stop_and_join(mut self: Box<Self>) -> Result<CaptureObservations, CaptureError> {
        self.stop_requested.store(true, Ordering::Release);
        if let Some(worker) = self.worker.take() {
            worker
                .join()
                .map_err(|_| CaptureError::CaptureWorkerPanicked {
                    worker: "Rust facade conformance capture worker",
                })?;
        }
        Ok(CaptureObservations::default())
    }
}

/// Drop contract: signal-only, allocation-free, blocking-free, log-free, panic-free.
impl Drop for DeterministicActiveCapture {
    fn drop(&mut self) {
        self.stop_requested.store(true, Ordering::Release);
        let _ = self.worker.take();
    }
}

pub fn session() -> Result<Session, SessionEngineHostBuildError> {
    session_with_optional_recording(None)
}

/// Creates the deterministic canonical-engine fixture with multistem recording.
pub fn session_with_recording(
    output_root: impl Into<PathBuf>,
) -> Result<Session, SessionEngineHostBuildError> {
    session_with_optional_recording(Some(output_root.into()))
}

/// Creates the deterministic canonical-engine fixture with a bounded Session
/// Session diagnostic trace recorder.
pub fn session_with_trace(
    path: impl Into<PathBuf>,
    capacity_records: usize,
) -> Result<Session, SessionEngineHostBuildError> {
    let mut session = session_with_optional_recording(None)?;
    session.session_trace = Some(crate::SessionTraceConfiguration {
        path: path.into(),
        capacity_records,
    });
    Ok(session)
}

/// Creates the deterministic canonical-engine fixture with both aligned
/// multistem recording and a bounded Session diagnostic trace.
pub fn session_with_recording_and_trace(
    output_root: impl Into<PathBuf>,
    trace_path: impl Into<PathBuf>,
    capacity_records: usize,
) -> Result<Session, SessionEngineHostBuildError> {
    let mut session = session_with_optional_recording(Some(output_root.into()))?;
    session.session_trace = Some(crate::SessionTraceConfiguration {
        path: trace_path.into(),
        capacity_records,
    });
    Ok(session)
}

fn session_with_optional_recording(
    output_root: Option<PathBuf>,
) -> Result<Session, SessionEngineHostBuildError> {
    let mut options = NativeSessionEngineHostOptions::default();
    if output_root.is_some() {
        options.polled_audio_endpoint = PolledAudioEndpointConfig {
            queue_capacity_frames: SLOW_BRANCH_QUEUE_CAPACITY_FRAMES,
            ..PolledAudioEndpointConfig::default()
        };
    }
    let prepare_context =
        PrepareContext::new(SampleSpec::new(48_000, 1, SampleFormat::F32Interleaved));
    let mut builder = SessionEngineHostBuilder::new(
        prepare_context,
        options.source_queue_capacity_frames,
        options.start_options,
    )?;
    let capture_backend: Arc<dyn CallbackCaptureBackend> = Arc::new(DeterministicCaptureBackend {
        timestamp_origin_ns: Arc::new(OnceLock::new()),
    });
    builder
        .set_application_backend(Arc::clone(&capture_backend))
        .set_microphone_backend(capture_backend);
    Session::with_host_builder(builder, options.polled_audio_endpoint, output_root)
}

/// Declares and registers a deterministic native connector used only by
/// cross-language conformance harnesses.
pub fn observed_connector(
    session: &Session,
    per_frame_delay: Duration,
) -> Result<EndpointHandle, ObservedEndpointError> {
    let operator_id = OperatorId::new(OBSERVED_CONNECTOR_OPERATOR_ID);
    let endpoint = session.connector(operator_id.clone(), EndpointConfiguration::new())?;
    session.register_connector_driver(
        operator_id,
        Arc::new(ObservedEndpointFactory { per_frame_delay }),
    )?;
    Ok(endpoint)
}

/// Declares and registers a deterministic native browser boundary used only
/// by cross-language conformance harnesses.
pub fn observed_browser(
    session: &Session,
    per_frame_delay: Duration,
) -> Result<EndpointHandle, ObservedEndpointError> {
    let endpoint = session.browser("https://receiver.invalid/conformance")?;
    session.register_browser_driver(Arc::new(ObservedEndpointFactory { per_frame_delay }))?;
    Ok(endpoint)
}

#[derive(Debug, thiserror::Error)]
pub enum ObservedEndpointError {
    #[error(transparent)]
    Declaration(#[from] SessionError),
    #[error(transparent)]
    Registration(#[from] SessionEndpointError),
}

struct ObservedEndpointFactory {
    per_frame_delay: Duration,
}

impl EndpointDriverFactory for ObservedEndpointFactory {
    fn prepare(
        &self,
        inputs: Vec<EndpointDriverInput>,
    ) -> Result<Box<dyn PreparedEndpointDriver>, EndpointFailure> {
        Ok(Box::new(PreparedObservedEndpoint {
            inputs,
            per_frame_delay: self.per_frame_delay,
        }))
    }
}

struct PreparedObservedEndpoint {
    inputs: Vec<EndpointDriverInput>,
    per_frame_delay: Duration,
}

impl PreparedEndpointDriver for PreparedObservedEndpoint {
    fn start(
        self: Box<Self>,
        start_gate: Arc<EndpointStartGate>,
    ) -> Result<Box<dyn RunningEndpointDriver>, EndpointFailure> {
        let stop_requested = Arc::new(AtomicBool::new(false));
        let worker_stop_requested = Arc::clone(&stop_requested);
        let frames_received_total = Arc::new(AtomicU64::new(0));
        let worker_frames_received_total = Arc::clone(&frames_received_total);
        let per_frame_delay = self.per_frame_delay;
        let mut receivers: Vec<_> = self
            .inputs
            .into_iter()
            .map(EndpointDriverInput::into_parts)
            .map(|(receiver, _context)| receiver)
            .collect();
        let worker = thread::Builder::new()
            .name("pocketstation-conformance-endpoint".to_owned())
            .spawn(move || {
                while !start_gate.is_open() && !worker_stop_requested.load(Ordering::Acquire) {
                    thread::yield_now();
                }
                while !worker_stop_requested.load(Ordering::Acquire) {
                    let mut received = false;
                    for receiver in &mut receivers {
                        if receiver.try_recv().is_some() {
                            worker_frames_received_total.fetch_add(1, Ordering::Relaxed);
                            received = true;
                            if !per_frame_delay.is_zero() {
                                thread::sleep(per_frame_delay);
                            }
                        }
                    }
                    if !received {
                        thread::sleep(Duration::from_millis(1));
                    }
                }
            })
            .map_err(|error| {
                EndpointFailure::new(
                    EndpointFailureStage::Start,
                    format!("conformance endpoint worker failed to start: {error}"),
                )
            })?;
        Ok(Box::new(RunningObservedEndpoint {
            stop_requested,
            worker: Some(worker),
            frames_received_total,
        }))
    }

    fn cancel_preparation(self: Box<Self>) -> EndpointCancellationOutcome {
        EndpointCancellationOutcome {
            observations: EndpointDriverObservations::default(),
            result: Ok(()),
        }
    }
}

struct RunningObservedEndpoint {
    stop_requested: Arc<AtomicBool>,
    worker: Option<thread::JoinHandle<()>>,
    frames_received_total: Arc<AtomicU64>,
}

impl RunningObservedEndpoint {
    fn current_observations(&self) -> EndpointDriverObservations {
        let frames = self.frames_received_total.load(Ordering::Acquire);
        EndpointDriverObservations {
            frames_received_total: frames,
            frames_delivered_total: frames,
            ..EndpointDriverObservations::default()
        }
    }
}

impl RunningEndpointDriver for RunningObservedEndpoint {
    fn observations(&self) -> EndpointDriverObservations {
        self.current_observations()
    }

    fn request_stop(&mut self) -> Result<(), EndpointFailure> {
        self.stop_requested.store(true, Ordering::Release);
        Ok(())
    }

    fn join_and_finalize(mut self: Box<Self>) -> EndpointDriverFinalization {
        self.stop_requested.store(true, Ordering::Release);
        let result = self.worker.take().map_or(Ok(()), |worker| {
            worker.join().map_err(|_| {
                EndpointFailure::new(
                    EndpointFailureStage::JoinFinalize,
                    "conformance endpoint worker panicked",
                )
            })
        });
        EndpointDriverFinalization {
            observations: self.current_observations(),
            result,
        }
    }
}

/// Drop contract: signal-only, allocation-free, blocking-free, log-free,
/// panic-free.
impl Drop for RunningObservedEndpoint {
    fn drop(&mut self) {
        self.stop_requested.store(true, Ordering::Release);
        let _ = self.worker.take();
    }
}
