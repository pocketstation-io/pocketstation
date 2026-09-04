//! Deterministic Session fixture for external conformance harnesses.
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
    EndpointHandle, NativeSessionEngineHostOptions, OperatorId, PolledAudioEndpointConfig,
    SessionEngineHostBuildError, SessionEngineHostBuilder,
};

use crate::{
    EndpointCancellationOutcome, EndpointDriverFactory, EndpointDriverFinalization,
    EndpointDriverObservations, EndpointFailure, EndpointFailureStage, EndpointPortInput,
    EndpointReceiver, EndpointStartGate, PreparedEndpointDriver, RunningEndpointDriver, Session,
    SessionEndpointError, SessionError,
};

const FRAME_SAMPLES_PER_CHANNEL: usize = 960;
const FRAME_DURATION_NS: u64 = 20_000_000;
const FRAME_PACING_MS: u64 = 20;
const RECORDING_EDGE_CAPACITY_FRAMES: usize = crate::graph::plan::EDGE_RING_CAPACITY_FRAMES;
const SLOW_BRANCH_QUEUE_CAPACITY_FRAMES: usize = RECORDING_EDGE_CAPACITY_FRAMES / 2;
/// Frames emitted per source by the finite deterministic fixture.
///
/// This equals the runtime edge capacity, so the recording branch
/// remains lossless even if its worker is not scheduled until capture ends.
/// The independently configured half-capacity polled branch still saturates.
pub const FRAMES_PER_SOURCE: u64 = RECORDING_EDGE_CAPACITY_FRAMES as u64;
pub const OBSERVED_CONNECTOR_OPERATOR_ID: &str = "io.pocketstation.conformance.connector.v1";

#[derive(Clone, Copy)]
enum FixtureSource {
    Application,
    SystemAudio,
    Microphone,
}

impl FixtureSource {
    const fn stream_id(self) -> StreamId {
        match self {
            Self::Application => StreamId(101),
            Self::SystemAudio => StreamId(151),
            Self::Microphone => StreamId(201),
        }
    }

    const fn source_id(self) -> SourceId {
        match self {
            Self::Application => SourceId(102),
            Self::SystemAudio => SourceId(152),
            Self::Microphone => SourceId(202),
        }
    }

    const fn amplitude(self) -> f32 {
        match self {
            Self::Application => 0.25,
            Self::SystemAudio => 0.375,
            Self::Microphone => 0.5,
        }
    }

    const fn channels(self) -> u8 {
        match self {
            Self::Application | Self::SystemAudio => 2,
            Self::Microphone => 1,
        }
    }
}

struct DeterministicCaptureBackend {
    timestamp_origin_ns: Arc<OnceLock<u64>>,
    frames_per_source: u64,
}

struct DeterministicPreparedCapture {
    source: FixtureSource,
    timestamp_origin_ns: Arc<OnceLock<u64>>,
    frames_per_source: u64,
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
            CaptureMode::SystemMix => FixtureSource::SystemAudio,
            CaptureMode::Application(_)
            | CaptureMode::Process(_)
            | CaptureMode::ExactApplication { .. }
            | CaptureMode::ExactApplicationStable { .. } => FixtureSource::Application,
        };
        Ok(Box::new(DeterministicPreparedCapture {
            source,
            timestamp_origin_ns: Arc::clone(&self.timestamp_origin_ns),
            frames_per_source: self.frames_per_source,
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
        let frames_per_source = self.frames_per_source;
        let timestamp_origin_ns = *self
            .timestamp_origin_ns
            .get_or_init(|| crate::timing::monotonic_timestamp_ns().saturating_add(1_000_000));
        let worker = std::thread::spawn(move || {
            let samples_per_frame =
                FRAME_SAMPLES_PER_CHANNEL.saturating_mul(usize::from(source.channels()));
            let pool = AudioBufferPool::new(32, samples_per_frame);
            let mut sequence = 0_u64;
            while !worker_stop_requested.load(Ordering::Acquire) && sequence < frames_per_source {
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

/// Drop requirements: signal-only, allocation-free, blocking-free, log-free, panic-free.
impl Drop for DeterministicActiveCapture {
    fn drop(&mut self) {
        self.stop_requested.store(true, Ordering::Release);
        let _ = self.worker.take();
    }
}

pub fn session() -> Result<Session, SessionEngineHostBuildError> {
    session_with_options(None, FRAMES_PER_SOURCE)
}

/// Creates a finite fixture that produces enough frames to overflow a
/// deliberately unconsumed route.
pub fn session_for_saturation() -> Result<Session, SessionEngineHostBuildError> {
    session_with_options(None, FRAMES_PER_SOURCE.saturating_mul(4))
}

/// Creates the deterministic Session fixture with multistem recording.
pub fn session_with_recording(
    output_root: impl Into<PathBuf>,
) -> Result<Session, SessionEngineHostBuildError> {
    session_with_options(Some(output_root.into()), FRAMES_PER_SOURCE)
}

/// Creates the deterministic Session fixture with a bounded diagnostic trace.
pub fn session_with_trace(
    path: impl Into<PathBuf>,
    capacity_records: usize,
) -> Result<Session, SessionEngineHostBuildError> {
    let mut session = session_with_options(None, FRAMES_PER_SOURCE)?;
    session.session_trace = Some(crate::SessionTraceConfiguration {
        path: path.into(),
        capacity_records,
    });
    Ok(session)
}

/// Creates the deterministic Session fixture with both aligned
/// multistem recording and a bounded Session diagnostic trace.
pub fn session_with_recording_and_trace(
    output_root: impl Into<PathBuf>,
    trace_path: impl Into<PathBuf>,
    capacity_records: usize,
) -> Result<Session, SessionEngineHostBuildError> {
    let mut session = session_with_options(Some(output_root.into()), FRAMES_PER_SOURCE)?;
    session.session_trace = Some(crate::SessionTraceConfiguration {
        path: trace_path.into(),
        capacity_records,
    });
    Ok(session)
}

fn session_with_options(
    output_root: Option<PathBuf>,
    frames_per_source: u64,
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
        frames_per_source,
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
    let input = crate::PortSpec::new(
        "audio",
        crate::PortDirection::Input,
        crate::SignalSpec::audio(),
        crate::MediaCaps::Audio(crate::AudioCaps {
            sample_rate_hz: None,
            frame_samples: None,
            channel_layout: crate::ChannelLayout::Any,
            format: crate::SampleFormat::F32Interleaved,
        }),
        crate::Multiplicity::Many,
        true,
    )
    .map_err(|error| ObservedEndpointError::Contract(error.to_string()))?;
    let node = crate::NodeDescriptor::new(
        crate::NodeTypeId::from("io.pocketstation.conformance.connector.node.v1"),
        "PocketStation conformance connector",
        vec![input],
        Vec::new(),
        crate::ExecutionPartition::AsyncWorker,
        crate::ExecutionSafety::AllocationAllowed,
        true,
    )
    .map_err(|error| ObservedEndpointError::Contract(error.to_string()))?;
    let configuration = crate::connector::ConnectorConfigurationSchema::new(1, Vec::new())
        .map_err(|error| ObservedEndpointError::Contract(error.to_string()))?;
    let readiness = crate::connector::ConnectorReadinessPolicy::new(
        Duration::from_secs(2),
        Duration::from_millis(10),
        1,
        1,
    )
    .map_err(|error| ObservedEndpointError::Contract(error.to_string()))?;
    let manifest = crate::connector::ConnectorManifest::new(
        1,
        OperatorId::new(OBSERVED_CONNECTOR_OPERATOR_ID),
        env!("CARGO_PKG_VERSION"),
        node,
        configuration,
        readiness,
    )
    .map_err(|error| ObservedEndpointError::Contract(error.to_string()))?;
    let connector = crate::connector::Connector::new(
        manifest,
        Arc::new(ObservedConnectorFactory { per_frame_delay }),
    )
    .map_err(|error| ObservedEndpointError::Contract(error.to_string()))?;
    let registered = session.register_connector(connector)?;
    Ok(registered.declare(
        session,
        crate::connector::ConnectorConfiguration::new(),
        crate::RouteSettings::realtime_audio(),
    )?)
}

/// Declares and registers a deterministic native browser receiver used only
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
    #[error("invalid conformance Connector declaration: {0}")]
    Contract(String),
    #[error(transparent)]
    Declaration(#[from] SessionError),
    #[error(transparent)]
    Registration(#[from] SessionEndpointError),
    #[error(transparent)]
    ConnectorRegistration(#[from] crate::connector::ConnectorRegistrationError),
    #[error(transparent)]
    ConnectorDeclaration(#[from] crate::connector::ConnectorDeclarationError),
}

struct ObservedEndpointFactory {
    per_frame_delay: Duration,
}

struct ObservedConnectorFactory {
    per_frame_delay: Duration,
}

impl crate::connector::ConnectorFactory for ObservedConnectorFactory {
    fn prepare(
        &self,
        inputs: Vec<EndpointPortInput>,
    ) -> Result<Box<dyn crate::connector::ConnectorWorker>, crate::connector::ConnectorError> {
        Ok(Box::new(ObservedConnectorWorker {
            inputs,
            per_frame_delay: self.per_frame_delay,
        }))
    }
}

struct ObservedConnectorWorker {
    inputs: Vec<EndpointPortInput>,
    per_frame_delay: Duration,
}

impl crate::connector::ConnectorWorker for ObservedConnectorWorker {
    fn run(
        self: Box<Self>,
        context: crate::connector::ConnectorContext,
    ) -> crate::connector::ConnectorRunOutcome {
        let mut receivers: Vec<_> = self
            .inputs
            .into_iter()
            .map(EndpointPortInput::into_parts)
            .filter_map(|(receiver, _context)| match receiver {
                EndpointReceiver::Audio { receiver, .. } => Some(receiver),
                EndpointReceiver::Signal(_) => None,
            })
            .collect();
        let _ = context.report_readiness_success();
        while !context.is_stop_requested() {
            let mut received = false;
            for receiver in &mut receivers {
                if receiver.try_recv().is_some() {
                    context.record_frame_received(1);
                    context.record_frame_delivered(1);
                    received = true;
                    if !self.per_frame_delay.is_zero() {
                        thread::sleep(self.per_frame_delay);
                    }
                }
            }
            if !received {
                let _ = context.wait_for_stop(Duration::from_millis(1));
            }
        }
        crate::connector::ConnectorRunOutcome::success()
    }
}

impl EndpointDriverFactory for ObservedEndpointFactory {
    fn prepare(
        &self,
        inputs: Vec<EndpointPortInput>,
    ) -> Result<Box<dyn PreparedEndpointDriver>, EndpointFailure> {
        Ok(Box::new(PreparedObservedEndpoint {
            inputs,
            per_frame_delay: self.per_frame_delay,
        }))
    }
}

struct PreparedObservedEndpoint {
    inputs: Vec<EndpointPortInput>,
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
            .map(EndpointPortInput::into_parts)
            .filter_map(|(receiver, _context)| match receiver {
                EndpointReceiver::Audio { receiver, .. } => Some(receiver),
                EndpointReceiver::Signal(_) => None,
            })
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

/// Drop requirements: signal-only, allocation-free, blocking-free, log-free,
/// panic-free.
impl Drop for RunningObservedEndpoint {
    fn drop(&mut self) {
        self.stop_requested.store(true, Ordering::Release);
        let _ = self.worker.take();
    }
}

// Cross-language conformance uses deliberately neutral vocabulary. These
// identities are shared verbatim by Rust, C, managed SDK, and PKSS fixtures;
// no Rust type identity is serialized into the ABI or sidecar protocol.
pub const EXTENSION_SIGNAL_ID: &str = "org.pocketstation.conformance.signal.v1";
pub const EXTENSION_SCHEMA_ID: &str = "urn:pocketstation:conformance:signal:v1";
pub const EXTENSION_ROLE_ID: &str = "org.pocketstation.conformance.terminal.v1";
pub const EXTENSION_SOURCE_TYPE_ID: &str = "org.pocketstation.conformance.source.fixture.v1";
pub const EXTENSION_OPERATOR_ID: &str = "org.pocketstation.conformance.operator.v1";
pub const EXTENSION_OPERATOR_NODE_ID: &str = "org.pocketstation.conformance.operator-node.v1";
pub const EXTENSION_ENDPOINT_ID: &str = "org.pocketstation.conformance.endpoint.v1";
pub const EXTENSION_ENDPOINT_NODE_ID: &str = "org.pocketstation.conformance.endpoint-node.v1";
pub const EXTENSION_SOURCE_PORT: &str = "out";
pub const EXTENSION_OPERATOR_INPUT_PORT: &str = "in";
pub const EXTENSION_OPERATOR_OUTPUT_PORT: &str = "out";
pub const EXTENSION_ENDPOINT_INPUT_PORT: &str = "in";
pub const EXTENSION_INPUT_PAYLOAD: &[u8] = b"seed";
pub const EXTENSION_OUTPUT_PAYLOAD: &[u8] = b"seed!";

/// Language-neutral outcome returned by the conformance fixture.
///
/// This is test evidence, not a second runtime API. All counters come from the
/// `Session` and its registered Source, Operator, and Endpoint owners.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExtensionConformanceReport {
    pub signal_id: &'static str,
    pub schema_id: &'static str,
    pub role_id: &'static str,
    pub source_type_id: &'static str,
    pub operator_id: &'static str,
    pub endpoint_id: &'static str,
    pub input_payload: &'static str,
    pub output_payload: &'static str,
    pub failure_requested: bool,
    pub source_prepared_total: u64,
    pub source_emitted_total: u64,
    pub source_closed_total: u64,
    pub operator_prepared_total: u64,
    pub operator_processed_total: u64,
    pub operator_output_total: u64,
    pub operator_failure_total: u64,
    pub operator_closed_total: u64,
    pub endpoint_prepared_total: u64,
    pub endpoint_received_total: u64,
    pub endpoint_stopped_total: u64,
    pub endpoint_finalized_total: u64,
    pub lifecycle_event_total: u64,
    pub terminal_event_total: u64,
    pub queue_capacity_signals: u64,
    pub queue_peak_signals: u64,
    pub route_capacity_signals: u64,
    pub route_peak_signals: u64,
    pub route_delivered_total: u64,
    pub maximum_buffered_payload_bytes: u64,
    pub stop_success: bool,
}

#[derive(Default)]
struct ExtensionSourceControl {
    prepared: AtomicU64,
    closed: AtomicU64,
}

struct ExtensionSourceFactory {
    manifest: crate::SourceManifest,
    control: Arc<ExtensionSourceControl>,
}

struct ExtensionSourceDriver {
    control: Arc<ExtensionSourceControl>,
    session: Option<crate::SourceSessionContext>,
    emitted: bool,
}

impl crate::SourceDriver for ExtensionSourceDriver {
    fn prepare(
        &mut self,
        context: &crate::SourcePrepareContext,
    ) -> Result<(), crate::SourceDriverError> {
        self.session = context.session.clone();
        self.control.prepared.fetch_add(1, Ordering::Relaxed);
        Ok(())
    }

    fn next(
        &mut self,
        _cancellation: &crate::SourceCancellation,
    ) -> Result<Option<crate::SourceEmission>, crate::SourceDriverError> {
        if self.emitted {
            return Ok(None);
        }
        let session = self.session.as_ref().ok_or_else(|| {
            crate::SourceDriverError::Failed("Session source context is missing".to_owned())
        })?;
        let output = session.output(EXTENSION_SOURCE_PORT).ok_or_else(|| {
            crate::SourceDriverError::Failed("conformance source output is missing".to_owned())
        })?;
        self.emitted = true;
        let envelope = crate::SignalEnvelope::untracked(
            crate::SignalPayload::Bytes(EXTENSION_INPUT_PAYLOAD.to_vec()),
            extension_signal_spec(),
            11,
        )
        .with_lineage(
            crate::SignalLineage {
                session_id: session.session_id,
                stream_id: output.stream_id,
                source_id: session.source_id,
                clock_id: crate::ClockDomainId::new(7),
                sequence_number: 11,
                source_generation: 1,
                discontinuity_epoch: 0,
                policy_epoch: 0,
            },
            crate::SignalTiming {
                source_timestamp_ns: Some(13),
                observed_timestamp_ns: 13,
                session_timestamp_ns: Some(13),
                duration_ns: Some(1_000_000),
            },
        );
        Ok(Some(crate::SourceEmission {
            output_port: EXTENSION_SOURCE_PORT.to_owned(),
            envelope,
            terminal: true,
        }))
    }

    fn close(&mut self) -> Result<(), crate::SourceDriverError> {
        self.control.closed.fetch_add(1, Ordering::Relaxed);
        Ok(())
    }
}

impl crate::SourceFactory for ExtensionSourceFactory {
    fn manifest(&self) -> &crate::SourceManifest {
        &self.manifest
    }

    fn validate_config(
        &self,
        _configuration: &crate::SourceConfiguration,
    ) -> Result<(), crate::ConfigError> {
        Ok(())
    }

    fn create(
        &self,
        _configuration: &crate::SourceConfiguration,
    ) -> Result<Box<dyn crate::SourceDriver>, crate::SourceDriverError> {
        Ok(Box::new(ExtensionSourceDriver {
            control: Arc::clone(&self.control),
            session: None,
            emitted: false,
        }))
    }
}

#[derive(Default)]
struct ExtensionOperatorControl {
    prepared: AtomicU64,
    closed: AtomicU64,
}

struct ExtensionOperatorFactory {
    manifest: crate::AsyncOperatorManifest,
    control: Arc<ExtensionOperatorControl>,
    fail: bool,
}

struct ExtensionOperatorNode {
    control: Arc<ExtensionOperatorControl>,
    fail: bool,
}

impl crate::AsyncNode for ExtensionOperatorNode {
    fn prepare<'a>(
        &'a mut self,
        _context: &'a crate::AsyncOperatorPrepareContext,
    ) -> crate::AsyncNodeFuture<'a, Result<(), crate::NodeError>> {
        Box::pin(async move {
            self.control.prepared.fetch_add(1, Ordering::Relaxed);
            Ok(())
        })
    }

    fn process<'a>(
        &'a mut self,
        input: crate::SignalEnvelope,
    ) -> crate::AsyncNodeFuture<'a, Result<Vec<crate::SignalEnvelope>, crate::NodeError>> {
        Box::pin(async move {
            if self.fail {
                return Err(crate::NodeError::Process(
                    "deterministic conformance failure".to_owned(),
                ));
            }
            let base = input.lineage().ok_or_else(|| {
                crate::NodeError::Process(
                    "conformance operator input lineage is missing".to_owned(),
                )
            })?;
            let timing = input.timing();
            let crate::SignalPayload::Bytes(input_payload) = input.payload() else {
                return Err(crate::NodeError::Process(
                    "conformance operator received a non-byte payload".to_owned(),
                ));
            };
            let mut payload = input_payload.clone();
            payload.push(b'!');
            let derivation = crate::SignalDerivation::new(
                base,
                timing,
                crate::OperatorId::new(EXTENSION_OPERATOR_ID),
                1,
                1,
                None,
            )
            .map_err(|error| crate::NodeError::Process(error.to_string()))?;
            let output = input
                .map_payload(
                    crate::SignalPayload::Bytes(payload),
                    extension_signal_spec(),
                )
                .with_derivation(derivation);
            Ok(vec![output])
        })
    }

    fn close<'a>(&'a mut self) -> crate::AsyncNodeFuture<'a, Result<(), crate::NodeError>> {
        Box::pin(async move {
            self.control.closed.fetch_add(1, Ordering::Relaxed);
            Ok(())
        })
    }
}

impl crate::AsyncOperatorFactory for ExtensionOperatorFactory {
    fn manifest(&self) -> &crate::AsyncOperatorManifest {
        &self.manifest
    }

    fn validate_config(
        &self,
        _configuration: &crate::OperatorConfiguration,
    ) -> Result<(), crate::ConfigError> {
        Ok(())
    }

    fn create(
        &self,
        _configuration: &crate::OperatorConfiguration,
    ) -> Result<Box<dyn crate::AsyncNode>, crate::NodeError> {
        Ok(Box::new(ExtensionOperatorNode {
            control: Arc::clone(&self.control),
            fail: self.fail,
        }))
    }
}

#[derive(Default)]
struct ExtensionEndpointControl {
    prepared: AtomicU64,
    received: AtomicU64,
    invalid: AtomicU64,
    stopped: AtomicU64,
    finalized: AtomicU64,
}

struct ExtensionEndpointDefinition;

impl crate::NodeDefinition for ExtensionEndpointDefinition {
    fn descriptor(&self) -> crate::NodeDescriptor {
        crate::NodeDescriptor {
            type_id: crate::NodeTypeId::from(EXTENSION_ENDPOINT_NODE_ID),
            display_name: "Cross-language conformance endpoint",
            inputs: vec![extension_port(
                EXTENSION_ENDPOINT_INPUT_PORT,
                crate::PortDirection::Input,
            )],
            outputs: Vec::new(),
            execution: crate::ExecutionPartition::External,
            safety: crate::ExecutionSafety::ExternalService,
            stateful: true,
        }
    }

    fn validate_config(
        &self,
        _configuration: &crate::OperatorConfiguration,
    ) -> Result<(), crate::ConfigError> {
        Ok(())
    }
}

struct ExtensionEndpointFactory {
    control: Arc<ExtensionEndpointControl>,
}

struct PreparedExtensionEndpoint {
    input: crate::EndpointPortInput,
    control: Arc<ExtensionEndpointControl>,
}

struct RunningExtensionEndpoint {
    control: Arc<ExtensionEndpointControl>,
    stop: Arc<AtomicBool>,
    worker: Option<thread::JoinHandle<()>>,
}

impl crate::EndpointDriverFactory for ExtensionEndpointFactory {
    fn prepare(
        &self,
        mut inputs: Vec<crate::EndpointPortInput>,
    ) -> Result<Box<dyn crate::PreparedEndpointDriver>, crate::EndpointFailure> {
        if inputs.len() != 1 {
            return Err(crate::EndpointFailure::new(
                crate::EndpointFailureStage::Prepare,
                "conformance endpoint requires exactly one input",
            ));
        }
        let input = inputs.pop().ok_or_else(|| {
            crate::EndpointFailure::new(
                crate::EndpointFailureStage::Prepare,
                "conformance endpoint input is missing",
            )
        })?;
        if input.port_name() != EXTENSION_ENDPOINT_INPUT_PORT
            || input.signal_spec() != &extension_signal_spec()
            || !matches!(input.receiver(), crate::EndpointReceiver::Signal(_))
        {
            return Err(crate::EndpointFailure::new(
                crate::EndpointFailureStage::Prepare,
                "conformance endpoint input contract mismatch",
            ));
        }
        self.control.prepared.fetch_add(1, Ordering::Relaxed);
        Ok(Box::new(PreparedExtensionEndpoint {
            input,
            control: Arc::clone(&self.control),
        }))
    }
}

impl crate::PreparedEndpointDriver for PreparedExtensionEndpoint {
    fn start(
        self: Box<Self>,
        start_gate: Arc<crate::EndpointStartGate>,
    ) -> Result<Box<dyn crate::RunningEndpointDriver>, crate::EndpointFailure> {
        let (receiver, _) = self.input.into_parts();
        let crate::EndpointReceiver::Signal(mut receiver) = receiver else {
            return Err(crate::EndpointFailure::new(
                crate::EndpointFailureStage::Start,
                "conformance endpoint received an audio edge",
            ));
        };
        let stop = Arc::new(AtomicBool::new(false));
        let worker_stop = Arc::clone(&stop);
        let control = Arc::clone(&self.control);
        let worker_control = Arc::clone(&control);
        let worker = thread::Builder::new()
            .name("pocketstation-extension-conformance-endpoint".to_owned())
            .spawn(move || {
                while !start_gate.is_open() && !worker_stop.load(Ordering::Acquire) {
                    thread::yield_now();
                }
                while !worker_stop.load(Ordering::Acquire) {
                    if let Some(envelope) = receiver.try_recv() {
                        let valid = envelope.signal_spec() == &extension_signal_spec()
                            && matches!(
                                envelope.payload(),
                                crate::SignalPayload::Bytes(payload)
                                    if payload.as_slice() == EXTENSION_OUTPUT_PAYLOAD
                            );
                        if valid {
                            worker_control.received.fetch_add(1, Ordering::Relaxed);
                        } else {
                            worker_control.invalid.fetch_add(1, Ordering::Relaxed);
                        }
                        continue;
                    }
                    if receiver.is_abandoned() {
                        break;
                    }
                    thread::sleep(Duration::from_millis(1));
                }
            })
            .map_err(|error| {
                crate::EndpointFailure::new(
                    crate::EndpointFailureStage::Start,
                    format!("conformance endpoint worker failed to start: {error}"),
                )
            })?;
        Ok(Box::new(RunningExtensionEndpoint {
            control,
            stop,
            worker: Some(worker),
        }))
    }

    fn cancel_preparation(self: Box<Self>) -> crate::EndpointCancellationOutcome {
        crate::EndpointCancellationOutcome {
            observations: crate::EndpointDriverObservations::default(),
            result: Ok(()),
        }
    }
}

impl RunningExtensionEndpoint {
    fn observations(&self) -> crate::EndpointDriverObservations {
        let received = self.control.received.load(Ordering::Relaxed);
        crate::EndpointDriverObservations {
            frames_received_total: received,
            frames_delivered_total: received,
            failures_total: self.control.invalid.load(Ordering::Relaxed),
            ..crate::EndpointDriverObservations::default()
        }
    }
}

impl crate::RunningEndpointDriver for RunningExtensionEndpoint {
    fn observations(&self) -> crate::EndpointDriverObservations {
        self.observations()
    }

    fn request_stop(&mut self) -> Result<(), crate::EndpointFailure> {
        self.control.stopped.fetch_add(1, Ordering::Relaxed);
        self.stop.store(true, Ordering::Release);
        Ok(())
    }

    fn join_and_finalize(mut self: Box<Self>) -> crate::EndpointDriverFinalization {
        self.stop.store(true, Ordering::Release);
        let result = self.worker.take().map_or(Ok(()), |worker| {
            worker.join().map_err(|_| {
                crate::EndpointFailure::new(
                    crate::EndpointFailureStage::JoinFinalize,
                    "conformance endpoint worker panicked",
                )
            })
        });
        self.control.finalized.fetch_add(1, Ordering::Relaxed);
        crate::EndpointDriverFinalization {
            observations: self.observations(),
            result,
        }
    }
}

impl Drop for RunningExtensionEndpoint {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::Release);
        let _ = self.worker.take();
    }
}

/// Executes the neutral typed Source -> `Stream<T>` -> Operator -> Endpoint
/// vector through the public Session.
pub fn run_extension_vector(failure_requested: bool) -> Result<ExtensionConformanceReport, String> {
    let source_control = Arc::new(ExtensionSourceControl::default());
    let operator_control = Arc::new(ExtensionOperatorControl::default());
    let endpoint_control = Arc::new(ExtensionEndpointControl::default());
    let source_manifest = crate::SourceManifest::new(
        crate::SourceTypeId::new(EXTENSION_SOURCE_TYPE_ID).map_err(|error| error.to_string())?,
        1,
        1,
        vec![extension_port(
            EXTENSION_SOURCE_PORT,
            crate::PortDirection::Output,
        )],
        crate::ExecutionPartition::BlockingWorker,
        crate::ExecutionSafety::AllocationAllowed,
    )
    .map_err(|error| error.to_string())?;
    let operator_manifest = extension_operator_manifest()?;
    let source_factory: Arc<dyn crate::SourceFactory> = Arc::new(ExtensionSourceFactory {
        manifest: source_manifest,
        control: Arc::clone(&source_control),
    });
    let operator_factory: Arc<dyn crate::AsyncOperatorFactory> =
        Arc::new(ExtensionOperatorFactory {
            manifest: operator_manifest.clone(),
            control: Arc::clone(&operator_control),
            fail: failure_requested,
        });
    let session = crate::Session::new();
    session
        .register_source(source_factory)
        .map_err(|error| error.to_string())?;
    session
        .register_operator(operator_factory)
        .map_err(|error| error.to_string())?;
    session
        .register_endpoint(
            crate::OperatorId::new(EXTENSION_ENDPOINT_ID),
            Arc::new(ExtensionEndpointDefinition),
            Arc::new(ExtensionEndpointFactory {
                control: Arc::clone(&endpoint_control),
            }),
        )
        .map_err(|error| error.to_string())?;

    let source = session
        .source(
            crate::SourceTypeId::new(EXTENSION_SOURCE_TYPE_ID)
                .map_err(|error| error.to_string())?,
            crate::SourceConfiguration::default(),
        )
        .map_err(|error| error.to_string())?;
    let output = source
        .output(EXTENSION_SOURCE_PORT)
        .map_err(|error| error.to_string())?;
    let typed = crate::Stream::<ExtensionSignal>::from_source_output(output)
        .map_err(|error| error.to_string())?;
    let typed_operator = crate::TypedOperator::<ExtensionSignal, ExtensionSignal>::new(
        crate::Operator::new(
            crate::OperatorId::new(EXTENSION_OPERATOR_ID),
            crate::OperatorConfiguration::new(),
        ),
        &operator_manifest,
        Some(EXTENSION_OPERATOR_INPUT_PORT),
        Some(EXTENSION_OPERATOR_OUTPUT_PORT),
    )
    .map_err(|error| error.to_string())?;
    let transformed = typed
        .through(typed_operator)
        .map_err(|error| error.to_string())?;
    let endpoint = session
        .endpoint(crate::EndpointDescriptor::new(
            crate::NodeTypeId::from(EXTENSION_ENDPOINT_NODE_ID),
            crate::OperatorId::new(EXTENSION_ENDPOINT_ID),
        ))
        .map_err(|error| error.to_string())?;
    transformed
        .send(endpoint)
        .map_err(|error| error.to_string())?;

    let mut running = session.start().map_err(|error| error.to_string())?;
    let deadline = std::time::Instant::now() + Duration::from_secs(2);
    let mut lifecycle_event_total = 0_u64;
    let mut terminal_event_total = 0_u64;
    let snapshot = loop {
        while let crate::SessionEventReceive::Event(event) = running.try_recv_event() {
            match event.kind() {
                crate::SessionEventKind::Lifecycle(_) => {
                    lifecycle_event_total = lifecycle_event_total.saturating_add(1)
                }
                crate::SessionEventKind::Terminal(_) => {
                    terminal_event_total = terminal_event_total.saturating_add(1)
                }
                _ => {}
            }
        }
        let snapshot = running
            .metrics_snapshot()
            .map_err(|error| error.to_string())?;
        let terminal = snapshot.operator(0).is_some_and(|operator| {
            if failure_requested {
                operator.worker.process_failure_total == 1
            } else {
                operator.worker.processed_total == 1
                    && endpoint_control.received.load(Ordering::Acquire) == 1
            }
        });
        if terminal {
            break snapshot;
        }
        if std::time::Instant::now() >= deadline {
            let _ = running.stop();
            return Err("conformance vector did not reach its terminal condition".to_owned());
        }
        thread::sleep(Duration::from_millis(1));
    };
    let source = snapshot
        .external_source(0)
        .ok_or_else(|| "conformance source metrics are missing".to_owned())?;
    let operator = snapshot
        .operator(0)
        .ok_or_else(|| "conformance operator metrics are missing".to_owned())?;
    let route = snapshot
        .derived_route(0)
        .ok_or_else(|| "conformance route metrics are missing".to_owned())?;
    let stop_success = running.stop().is_success();
    while let crate::SessionEventReceive::Event(event) = running.try_recv_event() {
        match event.kind() {
            crate::SessionEventKind::Lifecycle(_) => {
                lifecycle_event_total = lifecycle_event_total.saturating_add(1)
            }
            crate::SessionEventKind::Terminal(_) => {
                terminal_event_total = terminal_event_total.saturating_add(1)
            }
            _ => {}
        }
    }
    if endpoint_control.invalid.load(Ordering::Relaxed) != 0 {
        return Err("conformance endpoint observed an invalid signal".to_owned());
    }
    Ok(ExtensionConformanceReport {
        signal_id: EXTENSION_SIGNAL_ID,
        schema_id: EXTENSION_SCHEMA_ID,
        role_id: EXTENSION_ROLE_ID,
        source_type_id: EXTENSION_SOURCE_TYPE_ID,
        operator_id: EXTENSION_OPERATOR_ID,
        endpoint_id: EXTENSION_ENDPOINT_ID,
        input_payload: "seed",
        output_payload: "seed!",
        failure_requested,
        source_prepared_total: source_control.prepared.load(Ordering::Relaxed),
        source_emitted_total: source.runtime.emitted_total,
        source_closed_total: source_control.closed.load(Ordering::Relaxed),
        operator_prepared_total: operator_control.prepared.load(Ordering::Relaxed),
        operator_processed_total: operator.worker.processed_total,
        operator_output_total: operator.worker.output_emitted_total,
        operator_failure_total: operator.worker.process_failure_total,
        operator_closed_total: operator_control.closed.load(Ordering::Relaxed),
        endpoint_prepared_total: endpoint_control.prepared.load(Ordering::Relaxed),
        endpoint_received_total: endpoint_control.received.load(Ordering::Relaxed),
        endpoint_stopped_total: endpoint_control.stopped.load(Ordering::Relaxed),
        endpoint_finalized_total: endpoint_control.finalized.load(Ordering::Relaxed),
        lifecycle_event_total,
        terminal_event_total,
        queue_capacity_signals: operator.input_queue_capacity_frames(),
        queue_peak_signals: operator.input_queue_peak_frames(),
        route_capacity_signals: route.output.capacity_signals,
        route_peak_signals: route.output.peak_depth_signals,
        route_delivered_total: route.output.received_total,
        maximum_buffered_payload_bytes: operator
            .input_queue_capacity_frames()
            .saturating_mul(4_096),
        stop_success,
    })
}

pub struct ExtensionSignal;

impl crate::StreamSignal for ExtensionSignal {
    fn signal_spec() -> crate::SignalSpec {
        extension_signal_spec()
    }
}

fn extension_signal_spec() -> crate::SignalSpec {
    crate::SignalSpec::custom(EXTENSION_SIGNAL_ID)
        .with_role(EXTENSION_ROLE_ID)
        .with_schema(EXTENSION_SCHEMA_ID)
}

fn extension_port(name: &str, direction: crate::PortDirection) -> crate::PortSpec {
    crate::PortSpec {
        name: name.to_owned(),
        direction,
        signal: extension_signal_spec(),
        media: crate::MediaCaps::Binary(crate::BinaryFormat::Raw),
        multiplicity: crate::Multiplicity::Many,
        required: true,
    }
}

fn extension_operator_manifest() -> Result<crate::AsyncOperatorManifest, String> {
    let mut input_edge = crate::RouteSettings::bounded_async();
    input_edge.media = crate::MediaCaps::Binary(crate::BinaryFormat::Raw);
    input_edge.backpressure = crate::BackpressurePolicy::DropNewest;
    input_edge.copy_policy = crate::CopyPolicy::CopyToBranchPool;
    let mut output_edge = crate::RouteSettings::bounded_async();
    output_edge.media = crate::MediaCaps::Binary(crate::BinaryFormat::Raw);
    output_edge.copy_policy = crate::CopyPolicy::CopyToBranchPool;
    crate::AsyncOperatorManifest::new(
        crate::OperatorId::new(EXTENSION_OPERATOR_ID),
        1,
        1,
        crate::NodeDescriptor {
            type_id: crate::NodeTypeId::from(EXTENSION_OPERATOR_NODE_ID),
            display_name: "Cross-language conformance operator",
            inputs: vec![extension_port(
                EXTENSION_OPERATOR_INPUT_PORT,
                crate::PortDirection::Input,
            )],
            outputs: vec![extension_port(
                EXTENSION_OPERATOR_OUTPUT_PORT,
                crate::PortDirection::Output,
            )],
            execution: crate::ExecutionPartition::AsyncWorker,
            safety: crate::ExecutionSafety::AllocationAllowed,
            stateful: true,
        },
        input_edge,
        output_edge,
        8,
        crate::OperatorPermissionPolicy {
            network_allowed: false,
            filesystem_allowed: false,
        },
        crate::OperatorDeadlinePolicy {
            process_timeout_ms: 500,
        },
        crate::OperatorCancellationPolicy::DiscardQueued,
        crate::OperatorFailurePolicy::StopWorker,
        crate::OperatorOutputRolePolicy {
            allowed: vec![crate::SemanticRole::new(EXTENSION_ROLE_ID)],
            terminal: vec![crate::SemanticRole::new(EXTENSION_ROLE_ID)],
        },
    )
    .map_err(|error| error.to_string())
}
