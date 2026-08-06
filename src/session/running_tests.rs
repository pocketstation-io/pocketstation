use std::sync::atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use crate::capture::{
    ActiveCaptureBackend, CallbackCaptureBackend, CaptureDelivery, CaptureError, CaptureMode,
    CaptureObservationCounters, CaptureObservationHandle, CaptureObservations,
    CaptureRuntimeFailure, CaptureRuntimeFailureClass, CapturedFrameDelivery,
    PreparedCaptureBackend, SourceGeneration, SourceKind, SourceRecoveryRequirement,
    SourceRuntimeEvent, SourceRuntimeEventSender, StableSourceId,
};
use crate::endpoint::{
    DerivedEndpointDriverInput, EndpointCancellationOutcome, EndpointDriverFactory,
    EndpointDriverFinalization, EndpointDriverInput, EndpointDriverObservations,
    EndpointDriverRegistry, EndpointFailure, EndpointFailureStage, EndpointStartGate,
    PreparedEndpointDriver, RunningEndpointDriver,
};
use crate::frame::{
    AudioBufferPool, AudioFrame, EndpointId, Platform, RouteId, SampleFormat, SampleSpec,
    SessionId, SourceId, StemId, StreamId,
};
use crate::graph::{
    transcript_final_spec, transcript_partial_spec, AsyncEnvelope, AsyncNode, AsyncNodeFuture,
    AsyncOperatorFactory, AsyncOperatorManifest, AsyncSignal, AudioCaps, ChannelLayout,
    ConfigError, CopyPolicy, DerivedSignalLineage, EdgeContract, ExecutionPartition, MediaCaps,
    Multiplicity, NodeConfig, NodeDefinition, NodeDescriptor, NodeError, NodeFactory, NodeRegistry,
    NodeTypeId, OperatorCancellationPolicy, OperatorDeadlinePolicy, OperatorFailurePolicy,
    OperatorOutputRolePolicy, OperatorPermissionPolicy, PortDirection, PortSpec, PrepareContext,
    RuntimeNode, SafetyContract, SemanticRole, SignalSpec, TextFormat, TRANSCRIPT_FINAL_ROLE,
    TRANSCRIPT_PARTIAL_ROLE,
};
use crate::runtime::{PlanEdgeFrame, PlanEdgeReceiver};

use crate::session::{
    prepare_session_runtime, start_prepared_session, ApplicationSelector, CaptureBackendSet,
    EndpointConfiguration, EndpointDescriptor, Operator, OperatorConfiguration, OperatorId,
    Session, SessionCompiler, SessionEngineBuilder, SessionEngineStartError, SessionEventKind,
    SessionEventReceive, SessionLifecycleState, SessionStartError, SessionStartOptions,
    SessionTerminalState, Source, APPLICATION_SOURCE_NODE_TYPE_ID, BROWSER_NODE_TYPE_ID,
    BROWSER_OPERATOR_ID, CONNECTOR_NODE_TYPE_ID, MICROPHONE_SOURCE_NODE_TYPE_ID,
    RECORDER_NODE_TYPE_ID, RECORDER_OPERATOR_ID,
};

const TEST_CONNECTOR_OPERATOR_ID: &str = "example.connector.running-session.v1";
const TEST_ASYNC_OPERATOR_ID: &str = "example.operator.running-stt.v1";
const TEST_ASYNC_NODE_TYPE_ID: &str = "operator.running-stt.test";
const TEST_TEXT_ENDPOINT_OPERATOR_ID: &str = "example.endpoint.running-text.v1";
const TEST_TEXT_ENDPOINT_NODE_TYPE_ID: &str = "endpoint.running-text.test";

struct TestNode;

impl RuntimeNode for TestNode {
    fn prepare(&mut self, _context: &PrepareContext) -> Result<(), NodeError> {
        Ok(())
    }

    fn process(&mut self, frame: AudioFrame) -> Result<Option<AudioFrame>, NodeError> {
        Ok(Some(frame))
    }
}

struct TestNodeFactory {
    descriptor: NodeDescriptor,
}

impl NodeFactory for TestNodeFactory {
    fn descriptor(&self) -> NodeDescriptor {
        self.descriptor.clone()
    }

    fn validate_config(&self, _config: &NodeConfig) -> Result<(), ConfigError> {
        Ok(())
    }

    fn instantiate(
        &self,
        _context: &PrepareContext,
        _config: &NodeConfig,
    ) -> Result<Box<dyn RuntimeNode>, NodeError> {
        Ok(Box::new(TestNode))
    }
}

fn audio_port(name: &str, direction: PortDirection) -> PortSpec {
    PortSpec {
        name: name.to_owned(),
        direction,
        signal: SignalSpec::audio(),
        media: MediaCaps::Audio(AudioCaps {
            sample_rate_hz: Some(48_000),
            frame_samples: Some(960),
            channel_layout: ChannelLayout::Mono,
            format: SampleFormat::F32Interleaved,
        }),
        multiplicity: Multiplicity::One,
        required: true,
    }
}

#[derive(Clone, Copy)]
enum TestNodeRole {
    Source,
    Endpoint,
}

fn descriptor(node_type_id: &'static str, role: TestNodeRole) -> NodeDescriptor {
    let source = matches!(role, TestNodeRole::Source);
    NodeDescriptor {
        type_id: NodeTypeId::from(node_type_id),
        display_name: "Explicit RunningSession test node",
        inputs: if source {
            Vec::new()
        } else {
            vec![audio_port("audio", PortDirection::Input)]
        },
        outputs: if source {
            vec![audio_port("audio", PortDirection::Output)]
        } else {
            Vec::new()
        },
        execution: if source {
            ExecutionPartition::RealtimeCpu
        } else {
            ExecutionPartition::AsyncWorker
        },
        safety: if source {
            SafetyContract::RealtimeSafe
        } else {
            SafetyContract::AllocationAllowed
        },
        stateful: true,
    }
}

fn node_registry() -> NodeRegistry {
    let mut nodes = NodeRegistry::new();
    for node_type_id in [
        APPLICATION_SOURCE_NODE_TYPE_ID,
        MICROPHONE_SOURCE_NODE_TYPE_ID,
    ] {
        nodes
            .register(Arc::new(TestNodeFactory {
                descriptor: descriptor(node_type_id, TestNodeRole::Source),
            }))
            .unwrap();
    }
    for node_type_id in [
        CONNECTOR_NODE_TYPE_ID,
        BROWSER_NODE_TYPE_ID,
        RECORDER_NODE_TYPE_ID,
    ] {
        nodes
            .register(Arc::new(TestNodeFactory {
                descriptor: descriptor(node_type_id, TestNodeRole::Endpoint),
            }))
            .unwrap();
    }
    nodes
}

fn product_spec() -> crate::session::SessionSpec {
    let session = Session::new();
    let application = session
        .capture(Source::application(ApplicationSelector::name(
            "Meeting App",
        )))
        .expect("application declaration must succeed");
    let microphone = session
        .capture(Source::microphone_default())
        .expect("microphone declaration must succeed");
    let connector = session
        .connector(
            OperatorId::new(TEST_CONNECTOR_OPERATOR_ID),
            EndpointConfiguration::new(),
        )
        .expect("connector declaration must succeed");
    let browser = session
        .browser("wss://receiver.example.test")
        .expect("browser declaration must succeed");
    for stem in [&application, &microphone] {
        stem.send(connector).expect("connector route must succeed");
        stem.send(browser).expect("browser route must succeed");
    }
    application
        .record("application")
        .expect("application recording route must succeed");
    microphone
        .record("microphone")
        .expect("microphone recording route must succeed");
    session.freeze().expect("product spec must freeze")
}

fn context() -> PrepareContext {
    PrepareContext::new(SampleSpec::new(48_000, 1, SampleFormat::F32Interleaved))
}

#[derive(Default)]
struct CaptureControl {
    prepare_calls_total: AtomicU64,
    open_calls_total: AtomicU64,
    startup_frames_count: AtomicU64,
    live_prepared_total: AtomicUsize,
    live_active_total: AtomicUsize,
    stop_calls_total: AtomicU64,
    fail_prepare: AtomicBool,
    fail_open: AtomicBool,
    fail_stop: AtomicBool,
    emit_source_unavailable: AtomicBool,
}

struct TestCaptureBackend {
    control: Arc<CaptureControl>,
    source_id: SourceId,
}

struct TestPreparedCapture {
    control: Arc<CaptureControl>,
    source_id: SourceId,
}

struct TestActiveCapture {
    control: Arc<CaptureControl>,
    source_id: SourceId,
    counters: CaptureObservationCounters,
    stop_requested: Arc<AtomicBool>,
    worker: Option<std::thread::JoinHandle<()>>,
    _runtime_event_sender: SourceRuntimeEventSender,
}

impl CallbackCaptureBackend for TestCaptureBackend {
    fn prepare(&self, _mode: CaptureMode) -> Result<Box<dyn PreparedCaptureBackend>, CaptureError> {
        self.control
            .prepare_calls_total
            .fetch_add(1, Ordering::Relaxed);
        if self.control.fail_prepare.load(Ordering::Acquire) {
            return Err(CaptureError::BackendInit("test prepare failure".to_owned()));
        }
        self.control
            .live_prepared_total
            .fetch_add(1, Ordering::Relaxed);
        Ok(Box::new(TestPreparedCapture {
            control: Arc::clone(&self.control),
            source_id: self.source_id,
        }))
    }
}

impl PreparedCaptureBackend for TestPreparedCapture {
    fn open(
        self: Box<Self>,
        delivery: CaptureDelivery,
    ) -> Result<Box<dyn ActiveCaptureBackend>, CaptureError> {
        self.control
            .open_calls_total
            .fetch_add(1, Ordering::Relaxed);
        if self.control.fail_open.load(Ordering::Acquire) {
            return Err(CaptureError::BackendInit("test open failure".to_owned()));
        }
        let startup_frames_count = self
            .control
            .startup_frames_count
            .load(Ordering::Acquire)
            .max(1);
        let CaptureDelivery {
            mut frame_sender,
            runtime_event_sender,
        } = delivery;
        let pool = AudioBufferPool::new(64, 960);
        for sequence_num in 1..=startup_frames_count {
            let mut buffer = pool
                .acquire()
                .expect("test capture pool must provide one slot per startup frame");
            buffer.set_len(960);
            let frame = AudioFrame::new(
                StreamId(self.source_id.0),
                self.source_id,
                sequence_num,
                sequence_num.saturating_mul(20_000_000),
                1,
                buffer,
            );
            let _ = frame_sender.try_send(frame);
        }
        if self.control.emit_source_unavailable.load(Ordering::Acquire) {
            let _ = runtime_event_sender.try_send(SourceRuntimeEvent::SourceUnavailable {
                stable_id: StableSourceId::new(
                    Platform::Unknown,
                    SourceKind::Application,
                    format!("test-source-{}", self.source_id.0),
                ),
                generation: SourceGeneration::INITIAL.next(),
                recovery_requirement: SourceRecoveryRequirement::ExplicitRediscoveryAndNewSession,
                failure: CaptureRuntimeFailure {
                    operation: "test capture lifecycle",
                    error_class: CaptureRuntimeFailureClass::SourceInstanceExited,
                },
            });
        }
        let stop_requested = Arc::new(AtomicBool::new(false));
        let worker_stop_requested = Arc::clone(&stop_requested);
        let source_id = self.source_id;
        let worker = std::thread::spawn(move || {
            let pool = AudioBufferPool::new(1, 960);
            let sequence_num = startup_frames_count.saturating_add(1);
            while !worker_stop_requested.load(Ordering::Acquire) {
                let Some(mut buffer) = pool.acquire() else {
                    std::thread::sleep(Duration::from_millis(1));
                    continue;
                };
                buffer.set_len(960);
                let frame = AudioFrame::new(
                    StreamId(source_id.0),
                    source_id,
                    sequence_num,
                    sequence_num.saturating_mul(20_000_000),
                    1,
                    buffer,
                );
                match frame_sender.try_send(frame) {
                    CapturedFrameDelivery::Delivered => break,
                    CapturedFrameDelivery::DroppedNewest
                    | CapturedFrameDelivery::DiscardedBeforeStart => {
                        std::thread::sleep(Duration::from_millis(1));
                    }
                }
            }
        });
        self.control
            .live_active_total
            .fetch_add(1, Ordering::Relaxed);
        Ok(Box::new(TestActiveCapture {
            control: Arc::clone(&self.control),
            source_id: self.source_id,
            counters: CaptureObservationCounters::default(),
            stop_requested,
            worker: Some(worker),
            _runtime_event_sender: runtime_event_sender,
        }))
    }
}

impl Drop for TestPreparedCapture {
    fn drop(&mut self) {
        self.control
            .live_prepared_total
            .fetch_sub(1, Ordering::Relaxed);
    }
}

impl ActiveCaptureBackend for TestActiveCapture {
    fn source_id(&self) -> SourceId {
        self.source_id
    }

    fn observation_handle(&self) -> CaptureObservationHandle {
        self.counters.observation_handle()
    }

    fn observations(&self) -> CaptureObservations {
        self.counters.snapshot()
    }

    fn stop_and_join(mut self: Box<Self>) -> Result<CaptureObservations, CaptureError> {
        self.control
            .stop_calls_total
            .fetch_add(1, Ordering::Relaxed);
        self.stop_requested.store(true, Ordering::Release);
        if let Some(worker) = self.worker.take() {
            worker
                .join()
                .map_err(|_| CaptureError::CaptureWorkerPanicked {
                    worker: "test capture worker",
                })?;
        }
        if self.control.fail_stop.load(Ordering::Acquire) {
            Err(CaptureError::BackendStatus {
                operation: "test capture stop",
                status_code: -1,
            })
        } else {
            Ok(self.counters.snapshot())
        }
    }
}

impl Drop for TestActiveCapture {
    fn drop(&mut self) {
        self.stop_requested.store(true, Ordering::Release);
        if let Some(worker) = self.worker.take() {
            let _ = worker.join();
        }
        self.control
            .live_active_total
            .fetch_sub(1, Ordering::Relaxed);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct PreparedRouteContextObservation {
    session_id: SessionId,
    endpoint_id: EndpointId,
    stem_id: StemId,
    route_id: RouteId,
    session_timeline_origin_ns: u64,
}

#[derive(Default)]
struct EndpointControl {
    prepare_calls_total: AtomicU64,
    start_calls_total: AtomicU64,
    live_prepared_total: AtomicUsize,
    live_running_total: AtomicUsize,
    deliveries_total: AtomicU64,
    pre_gate_deliveries_total: AtomicU64,
    lineage_stem_mask: AtomicU64,
    stop_requested: AtomicBool,
    fail_prepare_call: AtomicU64,
    fail_start_call: AtomicU64,
    fail_join_finalize: AtomicBool,
    consume_after_gate_delay_ms: AtomicU64,
    prepared_route_contexts: Mutex<Vec<PreparedRouteContextObservation>>,
}

struct TestEndpointFactory {
    control: Arc<EndpointControl>,
}

struct TestPreparedEndpoint {
    control: Arc<EndpointControl>,
    receivers: Vec<PlanEdgeReceiver>,
    live: bool,
}

struct TestRunningEndpoint {
    control: Arc<EndpointControl>,
    worker: Option<std::thread::JoinHandle<()>>,
    live: bool,
}

impl EndpointDriverFactory for TestEndpointFactory {
    fn prepare(
        &self,
        inputs: Vec<EndpointDriverInput>,
    ) -> Result<Box<dyn PreparedEndpointDriver>, EndpointFailure> {
        let call = self
            .control
            .prepare_calls_total
            .fetch_add(1, Ordering::Relaxed)
            .saturating_add(1);
        if self.control.fail_prepare_call.load(Ordering::Acquire) == call {
            return Err(EndpointFailure::new(
                EndpointFailureStage::Prepare,
                "test endpoint prepare failure",
            ));
        }
        let observations = inputs
            .iter()
            .map(|input| {
                let context = input.context();
                let route_context = context.route_context().ok_or_else(|| {
                    EndpointFailure::new(
                        EndpointFailureStage::Prepare,
                        "Session endpoint input omitted its typed route context",
                    )
                })?;
                let session_timeline_origin =
                    context.session_timeline_origin().ok_or_else(|| {
                        EndpointFailure::new(
                            EndpointFailureStage::Prepare,
                            "Session endpoint input omitted its timeline origin",
                        )
                    })?;
                Ok(PreparedRouteContextObservation {
                    session_id: context.session_id(),
                    endpoint_id: context.endpoint_id(),
                    stem_id: route_context.stem_id(),
                    route_id: route_context.route_id(),
                    session_timeline_origin_ns: session_timeline_origin.monotonic_timestamp_ns(),
                })
            })
            .collect::<Result<Vec<_>, EndpointFailure>>()?;
        self.control
            .prepared_route_contexts
            .lock()
            .map_err(|_| {
                EndpointFailure::new(
                    EndpointFailureStage::Prepare,
                    "test endpoint context observations are unavailable",
                )
            })?
            .extend(observations);
        self.control
            .live_prepared_total
            .fetch_add(1, Ordering::Relaxed);
        Ok(Box::new(TestPreparedEndpoint {
            control: Arc::clone(&self.control),
            receivers: inputs
                .into_iter()
                .map(EndpointDriverInput::into_parts)
                .map(|(receiver, _context)| receiver)
                .collect(),
            live: true,
        }))
    }
}

impl PreparedEndpointDriver for TestPreparedEndpoint {
    fn start(
        mut self: Box<Self>,
        start_gate: Arc<EndpointStartGate>,
    ) -> Result<Box<dyn RunningEndpointDriver>, EndpointFailure> {
        let call = self
            .control
            .start_calls_total
            .fetch_add(1, Ordering::Relaxed)
            .saturating_add(1);
        if self.control.fail_start_call.load(Ordering::Acquire) == call {
            return Err(EndpointFailure::new(
                EndpointFailureStage::Start,
                "test endpoint start failure",
            ));
        }
        let mut receivers = std::mem::take(&mut self.receivers);
        self.live = false;
        self.control
            .live_prepared_total
            .fetch_sub(1, Ordering::Relaxed);
        self.control
            .live_running_total
            .fetch_add(1, Ordering::Relaxed);
        let control = Arc::clone(&self.control);
        let worker = std::thread::spawn(move || {
            while !start_gate.is_open() && !control.stop_requested.load(Ordering::Acquire) {
                std::thread::sleep(Duration::from_millis(1));
            }
            std::thread::sleep(Duration::from_millis(
                control.consume_after_gate_delay_ms.load(Ordering::Acquire),
            ));
            while !control.stop_requested.load(Ordering::Acquire) {
                let mut delivery_observed = false;
                for receiver in &mut receivers {
                    if let Some(frame) = receiver.try_recv() {
                        delivery_observed = true;
                        if !start_gate.is_open() {
                            control
                                .pre_gate_deliveries_total
                                .fetch_add(1, Ordering::Relaxed);
                        }
                        observe_endpoint_frame(&control, frame);
                    }
                }
                if !delivery_observed {
                    std::thread::sleep(Duration::from_millis(1));
                }
            }
        });
        Ok(Box::new(TestRunningEndpoint {
            control: Arc::clone(&self.control),
            worker: Some(worker),
            live: true,
        }))
    }

    fn cancel_preparation(mut self: Box<Self>) -> EndpointCancellationOutcome {
        self.live = false;
        self.control
            .live_prepared_total
            .fetch_sub(1, Ordering::Relaxed);
        EndpointCancellationOutcome {
            observations: EndpointDriverObservations::default(),
            result: Ok(()),
        }
    }
}

impl Drop for TestPreparedEndpoint {
    fn drop(&mut self) {
        if self.live {
            self.control
                .live_prepared_total
                .fetch_sub(1, Ordering::Relaxed);
        }
    }
}

impl RunningEndpointDriver for TestRunningEndpoint {
    fn observations(&self) -> EndpointDriverObservations {
        EndpointDriverObservations {
            frames_received_total: self.control.deliveries_total.load(Ordering::Relaxed),
            frames_delivered_total: self.control.deliveries_total.load(Ordering::Relaxed),
            ..EndpointDriverObservations::default()
        }
    }

    fn request_stop(&mut self) -> Result<(), EndpointFailure> {
        self.control.stop_requested.store(true, Ordering::Release);
        Ok(())
    }

    fn join_and_finalize(mut self: Box<Self>) -> EndpointDriverFinalization {
        self.control.stop_requested.store(true, Ordering::Release);
        let result = self
            .worker
            .take()
            .map(std::thread::JoinHandle::join)
            .map_or(Ok(()), |result| {
                result.map_err(|_| {
                    EndpointFailure::new(
                        EndpointFailureStage::JoinFinalize,
                        "test endpoint worker panicked",
                    )
                })
            });
        let result = if self.control.fail_join_finalize.load(Ordering::Acquire) {
            Err(EndpointFailure::new(
                EndpointFailureStage::JoinFinalize,
                "test endpoint finalization failure",
            ))
        } else {
            result
        };
        EndpointDriverFinalization {
            observations: self.observations(),
            result,
        }
    }
}

impl Drop for TestRunningEndpoint {
    fn drop(&mut self) {
        self.control.stop_requested.store(true, Ordering::Release);
        if let Some(worker) = self.worker.take() {
            let _ = worker.join();
        }
        if self.live {
            self.control
                .live_running_total
                .fetch_sub(1, Ordering::Relaxed);
            self.live = false;
        }
    }
}

fn observe_endpoint_frame(control: &EndpointControl, frame: PlanEdgeFrame) {
    control.deliveries_total.fetch_add(1, Ordering::Relaxed);
    if let Some(lineage) = frame.lineage() {
        let bit = 1u64.checked_shl(lineage.stem_id.0 as u32).unwrap_or(0);
        control.lineage_stem_mask.fetch_or(bit, Ordering::Relaxed);
    }
}

#[derive(Default)]
struct AsyncOperatorControl {
    prepared_sample_rate_hz: AtomicU64,
    process_started_total: AtomicU64,
    cancel_total: AtomicU64,
    close_total: AtomicU64,
    block_process: AtomicBool,
    fail_prepare: AtomicBool,
}

struct RunningTestAsyncFactory {
    control: Arc<AsyncOperatorControl>,
    manifest: AsyncOperatorManifest,
}

impl RunningTestAsyncFactory {
    fn new(control: Arc<AsyncOperatorControl>, sample_rate_hz: u32) -> Self {
        let audio = MediaCaps::Audio(AudioCaps {
            sample_rate_hz: Some(sample_rate_hz),
            frame_samples: None,
            channel_layout: ChannelLayout::Mono,
            format: SampleFormat::F32Interleaved,
        });
        let mut input_edge = EdgeContract::voice_default();
        input_edge.media = audio;
        input_edge.copy_policy = CopyPolicy::CopyToBranchPool;
        let mut output_edge = EdgeContract::model_default();
        output_edge.media = MediaCaps::Text;
        Self {
            control,
            manifest: AsyncOperatorManifest {
                operator_id: OperatorId::new(TEST_ASYNC_OPERATOR_ID),
                revision: 1,
                generation: 1,
                node: NodeDescriptor {
                    type_id: NodeTypeId::from(TEST_ASYNC_NODE_TYPE_ID),
                    display_name: "Running Session test STT",
                    inputs: vec![PortSpec {
                        name: "audio".to_owned(),
                        direction: PortDirection::Input,
                        signal: SignalSpec::audio(),
                        media: audio,
                        multiplicity: Multiplicity::One,
                        required: true,
                    }],
                    outputs: vec![PortSpec {
                        name: "transcript".to_owned(),
                        direction: PortDirection::Output,
                        signal: SignalSpec::text(TextFormat::Utf8).with_role("transcript"),
                        media: MediaCaps::Text,
                        multiplicity: Multiplicity::One,
                        required: true,
                    }],
                    execution: ExecutionPartition::AsyncWorker,
                    safety: SafetyContract::AllocationAllowed,
                    stateful: true,
                },
                input_edge,
                output_edge,
                queue_capacity_frames: 8,
                permission: OperatorPermissionPolicy {
                    network_allowed: false,
                    filesystem_allowed: false,
                },
                deadline: OperatorDeadlinePolicy {
                    process_timeout_ms: 500,
                },
                cancellation: OperatorCancellationPolicy::DiscardQueued,
                failure: OperatorFailurePolicy::StopWorker,
                output_roles: OperatorOutputRolePolicy {
                    allowed: vec![
                        SemanticRole::new(TRANSCRIPT_PARTIAL_ROLE),
                        SemanticRole::new(TRANSCRIPT_FINAL_ROLE),
                    ],
                    terminal: vec![SemanticRole::new(TRANSCRIPT_FINAL_ROLE)],
                },
            },
        }
    }
}

impl AsyncOperatorFactory for RunningTestAsyncFactory {
    fn manifest(&self) -> &AsyncOperatorManifest {
        &self.manifest
    }

    fn validate_config(&self, _configuration: &NodeConfig) -> Result<(), ConfigError> {
        Ok(())
    }

    fn create(&self, _configuration: &NodeConfig) -> Result<Box<dyn AsyncNode>, NodeError> {
        Ok(Box::new(RunningTestAsyncNode {
            control: Arc::clone(&self.control),
            last_lineage: None,
        }))
    }
}

struct RunningTestAsyncNode {
    control: Arc<AsyncOperatorControl>,
    last_lineage: Option<crate::frame::FrameLineage>,
}

impl RunningTestAsyncNode {
    fn transcript(
        lineage: crate::frame::FrameLineage,
        role: SignalSpec,
        text: &str,
    ) -> Result<AsyncEnvelope, NodeError> {
        let mut output = AsyncEnvelope::new(
            AsyncSignal::Text(text.to_owned()),
            lineage.sequence_num,
            lineage.timestamp_start_ns,
        );
        output.signal_spec = role;
        output.source_id = Some(lineage.source_id);
        output.lineage = Some(lineage);
        output.derived_lineage = Some(
            DerivedSignalLineage::new(
                lineage,
                lineage.timestamp_end_ns(),
                OperatorId::new(TEST_ASYNC_OPERATOR_ID),
                1,
                1,
                None,
            )
            .map_err(|error| NodeError::Process(error.to_string()))?,
        );
        Ok(output)
    }
}

impl AsyncNode for RunningTestAsyncNode {
    fn prepare<'a>(
        &'a mut self,
        context: &'a PrepareContext,
    ) -> AsyncNodeFuture<'a, Result<(), NodeError>> {
        Box::pin(async move {
            self.control.prepared_sample_rate_hz.store(
                u64::from(context.sample_spec.sample_rate_hz),
                Ordering::Release,
            );
            if self.control.fail_prepare.load(Ordering::Acquire) {
                return Err(NodeError::Prepare(
                    "test operator prepare failure".to_owned(),
                ));
            }
            Ok(())
        })
    }

    fn process<'a>(
        &'a mut self,
        input: AsyncEnvelope,
    ) -> AsyncNodeFuture<'a, Result<Vec<AsyncEnvelope>, NodeError>> {
        Box::pin(async move {
            self.control
                .process_started_total
                .fetch_add(1, Ordering::Relaxed);
            let lineage = input
                .lineage
                .ok_or_else(|| NodeError::Process("test input omitted lineage".to_owned()))?;
            self.last_lineage = Some(lineage);
            if self.control.block_process.load(Ordering::Acquire) {
                std::future::pending::<()>().await;
            }
            Ok(vec![Self::transcript(
                lineage,
                transcript_partial_spec(),
                "partial",
            )?])
        })
    }

    fn flush<'a>(&'a mut self) -> AsyncNodeFuture<'a, Result<Vec<AsyncEnvelope>, NodeError>> {
        Box::pin(async move {
            self.last_lineage.take().map_or_else(
                || Ok(Vec::new()),
                |lineage| {
                    Ok(vec![Self::transcript(
                        lineage,
                        transcript_final_spec(),
                        "final",
                    )?])
                },
            )
        })
    }

    fn cancel<'a>(&'a mut self) -> AsyncNodeFuture<'a, Result<(), NodeError>> {
        Box::pin(async move {
            self.control.cancel_total.fetch_add(1, Ordering::Relaxed);
            Ok(())
        })
    }

    fn close<'a>(&'a mut self) -> AsyncNodeFuture<'a, Result<(), NodeError>> {
        Box::pin(async move {
            self.control.close_total.fetch_add(1, Ordering::Relaxed);
            Ok(())
        })
    }
}

struct TextEndpointDefinition;

impl NodeDefinition for TextEndpointDefinition {
    fn descriptor(&self) -> NodeDescriptor {
        NodeDescriptor {
            type_id: NodeTypeId::from(TEST_TEXT_ENDPOINT_NODE_TYPE_ID),
            display_name: "Running Session text endpoint",
            inputs: vec![PortSpec {
                name: "transcript".to_owned(),
                direction: PortDirection::Input,
                signal: SignalSpec::text(TextFormat::Utf8).with_role("transcript"),
                media: MediaCaps::Text,
                multiplicity: Multiplicity::One,
                required: true,
            }],
            outputs: Vec::new(),
            execution: ExecutionPartition::External,
            safety: SafetyContract::ExternalService,
            stateful: true,
        }
    }

    fn validate_config(&self, _config: &NodeConfig) -> Result<(), ConfigError> {
        Ok(())
    }
}

#[derive(Default)]
struct DerivedEndpointControl {
    prepare_total: AtomicU64,
    partial_total: AtomicU64,
    final_total: AtomicU64,
    typed_context_total: AtomicU64,
}

struct DerivedTextEndpointFactory {
    control: Arc<DerivedEndpointControl>,
}

struct PreparedDerivedTextEndpoint {
    control: Arc<DerivedEndpointControl>,
    outputs: Vec<crate::runtime::AsyncOperatorOutput>,
}

struct RunningDerivedTextEndpoint {
    control: Arc<DerivedEndpointControl>,
    outputs: Vec<crate::runtime::AsyncOperatorOutput>,
}

impl EndpointDriverFactory for DerivedTextEndpointFactory {
    fn prepare(
        &self,
        _inputs: Vec<EndpointDriverInput>,
    ) -> Result<Box<dyn PreparedEndpointDriver>, EndpointFailure> {
        Err(EndpointFailure::new(
            EndpointFailureStage::Prepare,
            "test text endpoint accepts only derived inputs",
        ))
    }

    fn prepare_derived(
        &self,
        inputs: Vec<DerivedEndpointDriverInput>,
    ) -> Result<Box<dyn PreparedEndpointDriver>, EndpointFailure> {
        self.control.prepare_total.fetch_add(1, Ordering::Relaxed);
        let mut outputs = Vec::with_capacity(inputs.len());
        for input in inputs {
            let (output, context) = input.into_parts();
            if context.derived_signal_spec().is_some()
                && context.derived_media() == Some(&MediaCaps::Text)
                && context.derived_edge_contract().is_some()
            {
                self.control
                    .typed_context_total
                    .fetch_add(1, Ordering::Relaxed);
            }
            outputs.push(output);
        }
        Ok(Box::new(PreparedDerivedTextEndpoint {
            control: Arc::clone(&self.control),
            outputs,
        }))
    }
}

impl PreparedEndpointDriver for PreparedDerivedTextEndpoint {
    fn start(
        self: Box<Self>,
        _start_gate: Arc<EndpointStartGate>,
    ) -> Result<Box<dyn RunningEndpointDriver>, EndpointFailure> {
        Ok(Box::new(RunningDerivedTextEndpoint {
            control: self.control,
            outputs: self.outputs,
        }))
    }

    fn cancel_preparation(self: Box<Self>) -> EndpointCancellationOutcome {
        EndpointCancellationOutcome {
            observations: EndpointDriverObservations::default(),
            result: Ok(()),
        }
    }
}

impl RunningEndpointDriver for RunningDerivedTextEndpoint {
    fn observations(&self) -> EndpointDriverObservations {
        let delivered = self
            .control
            .partial_total
            .load(Ordering::Relaxed)
            .saturating_add(self.control.final_total.load(Ordering::Relaxed));
        EndpointDriverObservations {
            frames_received_total: delivered,
            frames_delivered_total: delivered,
            ..EndpointDriverObservations::default()
        }
    }

    fn request_stop(&mut self) -> Result<(), EndpointFailure> {
        Ok(())
    }

    fn join_and_finalize(mut self: Box<Self>) -> EndpointDriverFinalization {
        for output in &mut self.outputs {
            while let Some(envelope) = output.recv() {
                match envelope.signal_spec.role.as_ref().map(SemanticRole::as_str) {
                    Some(TRANSCRIPT_PARTIAL_ROLE) => {
                        self.control.partial_total.fetch_add(1, Ordering::Relaxed);
                    }
                    Some(TRANSCRIPT_FINAL_ROLE) => {
                        self.control.final_total.fetch_add(1, Ordering::Relaxed);
                    }
                    _ => {}
                }
            }
        }
        EndpointDriverFinalization {
            observations: self.observations(),
            result: Ok(()),
        }
    }
}

fn derived_runtime_session() -> Session {
    let session = Session::new();
    let application = session
        .capture(Source::application(ApplicationSelector::name(
            "Operator test application",
        )))
        .expect("application declaration");
    let microphone = session
        .capture(Source::microphone_default())
        .expect("microphone declaration");
    let connector = session
        .connector(
            OperatorId::new(TEST_CONNECTOR_OPERATOR_ID),
            EndpointConfiguration::new(),
        )
        .expect("connector declaration");
    let first = session
        .endpoint(EndpointDescriptor::new(
            NodeTypeId::from(TEST_TEXT_ENDPOINT_NODE_TYPE_ID),
            OperatorId::new(TEST_TEXT_ENDPOINT_OPERATOR_ID),
        ))
        .expect("first text endpoint declaration");
    let second = session
        .endpoint(EndpointDescriptor::new(
            NodeTypeId::from(TEST_TEXT_ENDPOINT_NODE_TYPE_ID),
            OperatorId::new(TEST_TEXT_ENDPOINT_OPERATOR_ID),
        ))
        .expect("second text endpoint declaration");
    let transcript = microphone
        .through(Operator::new(
            OperatorId::new(TEST_ASYNC_OPERATOR_ID),
            OperatorConfiguration::new().with("language", "auto"),
        ))
        .expect("operator declaration");
    application
        .send(connector)
        .expect("application connector route");
    transcript.send(first).expect("first derived route");
    transcript.send(second).expect("second derived route");
    session
}

fn derived_runtime_engine(
    operator_control: &Arc<AsyncOperatorControl>,
    endpoint_control: &Arc<DerivedEndpointControl>,
    raw_endpoint_control: &Arc<EndpointControl>,
) -> crate::session::SessionEngine {
    let mut builder = SessionEngineBuilder::new(context(), 8, SessionStartOptions::default())
        .expect("derived engine builder");
    builder
        .register_endpoint_driver(
            OperatorId::new(TEST_CONNECTOR_OPERATOR_ID),
            NodeTypeId::from(CONNECTOR_NODE_TYPE_ID),
            Arc::new(TestEndpointFactory {
                control: Arc::clone(raw_endpoint_control),
            }),
        )
        .expect("test connector driver registration");
    builder
        .register_async_operator(Arc::new(RunningTestAsyncFactory::new(
            Arc::clone(operator_control),
            48_000,
        )))
        .expect("test async operator registration");
    builder
        .register_endpoint_definition(Arc::new(TextEndpointDefinition))
        .expect("test text endpoint definition registration");
    builder
        .register_endpoint_driver(
            OperatorId::new(TEST_TEXT_ENDPOINT_OPERATOR_ID),
            NodeTypeId::from(TEST_TEXT_ENDPOINT_NODE_TYPE_ID),
            Arc::new(DerivedTextEndpointFactory {
                control: Arc::clone(endpoint_control),
            }),
        )
        .expect("test text endpoint driver registration");
    builder.build().expect("derived engine build")
}

fn wait_for_operator_process(control: &AsyncOperatorControl) {
    let deadline = Instant::now() + Duration::from_secs(1);
    while control.process_started_total.load(Ordering::Acquire) == 0 && Instant::now() < deadline {
        std::thread::sleep(Duration::from_millis(1));
    }
    assert!(
        control.process_started_total.load(Ordering::Acquire) > 0,
        "capture input must reach the async Operator before the deadline"
    );
}

#[test]
fn given_typed_operator_routes_when_stopped_then_partial_final_and_metrics_are_truthful() {
    let operator = Arc::new(AsyncOperatorControl::default());
    let derived_endpoints = Arc::new(DerivedEndpointControl::default());
    let raw_endpoints = Arc::new(EndpointControl::default());
    let application = Arc::new(CaptureControl::default());
    let microphone = Arc::new(CaptureControl::default());
    let application_backend = capture_backend(&application, 11);
    let microphone_backend = capture_backend(&microphone, 22);
    let engine = derived_runtime_engine(&operator, &derived_endpoints, &raw_endpoints);

    let mut running = engine
        .start(
            derived_runtime_session(),
            capture_backend_set(&application_backend, &microphone_backend),
        )
        .expect("typed derived Session start");
    wait_for_operator_process(&operator);
    let outcome = running.stop();
    let (_sources, raw_routes, operators, derived_routes) = running.indexed_metrics_full();

    assert!(outcome.is_success());
    assert_eq!(raw_routes.len(), 1);
    assert_eq!(
        operator.prepared_sample_rate_hz.load(Ordering::Acquire),
        48_000
    );
    assert_eq!(operator.cancel_total.load(Ordering::Acquire), 0);
    assert_eq!(operator.close_total.load(Ordering::Acquire), 1);
    assert_eq!(
        derived_endpoints
            .typed_context_total
            .load(Ordering::Acquire),
        2
    );
    assert_eq!(derived_endpoints.partial_total.load(Ordering::Acquire), 2);
    assert_eq!(derived_endpoints.final_total.load(Ordering::Acquire), 2);
    assert_eq!(operators.len(), 1);
    assert_eq!(operators[0].worker.graceful_finish_total, 1);
    assert_eq!(operators[0].worker.cancellation_total, 0);
    assert!(operators[0].input_attempted_total() > 0);
    assert_eq!(
        operators[0].input_delivered_total() + operators[0].input_dropped_total(),
        operators[0].input_attempted_total()
    );
    assert_eq!(
        operators[0].input_enqueued_total(),
        operators[0].input_delivered_total()
    );
    assert!(operators[0].input_queue_capacity_frames() > 0);
    assert!(operators[0].input_queue_peak_frames() > 0);
    assert_eq!(operators[0].input_queue_depth_frames(), 0);
    assert_eq!(operators[0].finalization_failures_total, 0);
    assert_eq!(derived_routes.len(), 2);
    assert!(derived_routes.iter().all(|route| {
        route.output.delivered_total == 2
            && route.output.dropped_total == 0
            && route.endpoint.is_some()
            && route.endpoint_finalization_failures_total == 0
    }));
    assert_no_live_owners(&application, &microphone, &raw_endpoints);
}

#[test]
fn given_blocked_operator_when_cancelled_then_session_cancellation_is_bounded_and_observed() {
    let operator = Arc::new(AsyncOperatorControl::default());
    operator.block_process.store(true, Ordering::Release);
    let derived_endpoints = Arc::new(DerivedEndpointControl::default());
    let raw_endpoints = Arc::new(EndpointControl::default());
    let application = Arc::new(CaptureControl::default());
    let microphone = Arc::new(CaptureControl::default());
    let application_backend = capture_backend(&application, 11);
    let microphone_backend = capture_backend(&microphone, 22);
    let engine = derived_runtime_engine(&operator, &derived_endpoints, &raw_endpoints);

    let mut running = engine
        .start(
            derived_runtime_session(),
            capture_backend_set(&application_backend, &microphone_backend),
        )
        .expect("blocked Operator Session start");
    wait_for_operator_process(&operator);
    let started = Instant::now();
    let outcome = running.cancel();
    let elapsed = started.elapsed();
    let (_sources, _raw_routes, operators, derived_routes) = running.indexed_metrics_full();

    assert!(outcome.is_success());
    assert!(
        elapsed < Duration::from_secs(2),
        "Session cancellation exceeded its bounded Operator lifecycle"
    );
    assert_eq!(operator.cancel_total.load(Ordering::Acquire), 1);
    assert_eq!(operator.close_total.load(Ordering::Acquire), 1);
    assert_eq!(operators.len(), 1);
    assert_eq!(operators[0].worker.cancellation_total, 1);
    assert_eq!(operators[0].worker.graceful_finish_total, 0);
    assert_eq!(derived_endpoints.final_total.load(Ordering::Acquire), 0);
    assert_eq!(derived_routes.len(), 2);
    assert_no_live_owners(&application, &microphone, &raw_endpoints);
}

#[test]
fn given_operator_prepare_failure_when_started_then_all_prior_owners_roll_back() {
    let operator = Arc::new(AsyncOperatorControl::default());
    operator.fail_prepare.store(true, Ordering::Release);
    let derived_endpoints = Arc::new(DerivedEndpointControl::default());
    let raw_endpoints = Arc::new(EndpointControl::default());
    let application = Arc::new(CaptureControl::default());
    let microphone = Arc::new(CaptureControl::default());
    let application_backend = capture_backend(&application, 11);
    let microphone_backend = capture_backend(&microphone, 22);
    let engine = derived_runtime_engine(&operator, &derived_endpoints, &raw_endpoints);

    let error = match engine.start(
        derived_runtime_session(),
        capture_backend_set(&application_backend, &microphone_backend),
    ) {
        Ok(_) => panic!("Operator preparation must fail"),
        Err(error) => error,
    };

    assert!(matches!(
        error,
        SessionEngineStartError::Start(ref failure)
            if matches!(failure.error(), SessionStartError::OperatorPrepare { .. })
    ));
    assert_eq!(operator.cancel_total.load(Ordering::Acquire), 0);
    assert_eq!(operator.close_total.load(Ordering::Acquire), 0);
    assert_eq!(derived_endpoints.prepare_total.load(Ordering::Acquire), 0);
    assert_no_live_owners(&application, &microphone, &raw_endpoints);
}

fn endpoint_registry(control: &Arc<EndpointControl>) -> EndpointDriverRegistry {
    let mut registry = EndpointDriverRegistry::new();
    for (operator_id, node_type_id) in [
        (TEST_CONNECTOR_OPERATOR_ID, CONNECTOR_NODE_TYPE_ID),
        (BROWSER_OPERATOR_ID, BROWSER_NODE_TYPE_ID),
        (RECORDER_OPERATOR_ID, RECORDER_NODE_TYPE_ID),
    ] {
        registry
            .register(
                OperatorId::new(operator_id),
                NodeTypeId::from(node_type_id),
                Arc::new(TestEndpointFactory {
                    control: Arc::clone(control),
                }),
            )
            .expect("test endpoint driver registration must succeed");
    }
    registry
}

fn prepared_session(
    nodes: &NodeRegistry,
    endpoints: &EndpointDriverRegistry,
) -> crate::session::PreparedSession {
    let compiled = SessionCompiler::new(nodes, endpoints)
        .compile(product_spec())
        .expect("product Session must compile");
    prepare_session_runtime(compiled, nodes, &context(), 8).expect("product runtime must prepare")
}

fn capture_backend(control: &Arc<CaptureControl>, source_id: u64) -> TestCaptureBackend {
    TestCaptureBackend {
        control: Arc::clone(control),
        source_id: SourceId(source_id),
    }
}

fn capture_backend_set<'backend>(
    application: &'backend TestCaptureBackend,
    microphone: &'backend TestCaptureBackend,
) -> CaptureBackendSet<'backend> {
    CaptureBackendSet {
        application,
        microphone,
    }
}

fn assert_no_live_owners(
    application: &CaptureControl,
    microphone: &CaptureControl,
    endpoints: &EndpointControl,
) {
    assert_eq!(application.live_prepared_total.load(Ordering::Relaxed), 0);
    assert_eq!(application.live_active_total.load(Ordering::Relaxed), 0);
    assert_eq!(microphone.live_prepared_total.load(Ordering::Relaxed), 0);
    assert_eq!(microphone.live_active_total.load(Ordering::Relaxed), 0);
    assert_eq!(endpoints.live_prepared_total.load(Ordering::Relaxed), 0);
    assert_eq!(endpoints.live_running_total.load(Ordering::Relaxed), 0);
}

#[test]
fn given_two_sources_when_started_then_gate_lineage_and_repeated_stop_are_truthful() {
    let nodes = node_registry();
    let application = Arc::new(CaptureControl::default());
    let microphone = Arc::new(CaptureControl::default());
    let endpoints = Arc::new(EndpointControl::default());
    let application_backend = capture_backend(&application, 11);
    let microphone_backend = capture_backend(&microphone, 22);
    let registry = endpoint_registry(&endpoints);

    let mut running = start_prepared_session(
        prepared_session(&nodes, &registry),
        capture_backend_set(&application_backend, &microphone_backend),
        &registry,
        SessionStartOptions::default(),
    )
    .expect("transactional Session startup must succeed");
    let events = running
        .take_event_receiver()
        .expect("running Session must expose its sole event receiver");
    std::thread::sleep(Duration::from_millis(30));
    let first = running.stop();
    let second = running.stop();

    assert_eq!(first, second);
    assert!(first.is_success());
    assert_eq!(
        endpoints.pre_gate_deliveries_total.load(Ordering::Relaxed),
        0
    );
    assert_eq!(endpoints.deliveries_total.load(Ordering::Relaxed), 6);
    assert_eq!(endpoints.prepare_calls_total.load(Ordering::Relaxed), 5);
    assert_eq!(
        endpoints
            .lineage_stem_mask
            .load(Ordering::Relaxed)
            .count_ones(),
        2
    );
    let prepared_route_contexts = endpoints
        .prepared_route_contexts
        .lock()
        .expect("prepared route context observations must remain available");
    assert_eq!(prepared_route_contexts.len(), 6);
    let session_timeline_origin_ns = prepared_route_contexts[0].session_timeline_origin_ns;
    assert!(session_timeline_origin_ns > 0);
    assert!(prepared_route_contexts.iter().all(|context| {
        context.session_id == running.session_id()
            && context.session_timeline_origin_ns == session_timeline_origin_ns
    }));
    assert_eq!(
        prepared_route_contexts
            .iter()
            .map(|context| context.route_id.0)
            .collect::<std::collections::BTreeSet<_>>()
            .len(),
        6
    );
    assert_eq!(
        prepared_route_contexts
            .iter()
            .map(|context| context.stem_id.0)
            .collect::<std::collections::BTreeSet<_>>()
            .len(),
        2
    );
    assert!(prepared_route_contexts
        .iter()
        .all(|context| context.endpoint_id.0 > 0));
    drop(prepared_route_contexts);
    assert_no_live_owners(&application, &microphone, &endpoints);
    let mut event_kinds = Vec::new();
    while let SessionEventReceive::Event(event) = events.try_recv() {
        event_kinds.push(event.kind().clone());
    }
    assert!(matches!(
        event_kinds.first(),
        Some(SessionEventKind::Lifecycle(SessionLifecycleState::Starting))
    ));
    assert!(event_kinds.iter().any(|event| matches!(
        event,
        SessionEventKind::Lifecycle(SessionLifecycleState::Running)
    )));
    assert!(event_kinds.iter().any(|event| matches!(
        event,
        SessionEventKind::Lifecycle(SessionLifecycleState::Stopping)
    )));
    assert!(event_kinds.iter().any(|event| matches!(
        event,
        SessionEventKind::Lifecycle(SessionLifecycleState::Stopped)
    )));
    assert!(matches!(
        event_kinds.last(),
        Some(SessionEventKind::Terminal(terminal))
            if terminal.state() == SessionTerminalState::Stopped
    ));
}

#[test]
fn given_capture_backlog_when_session_starts_then_no_destination_edge_overflows() {
    let nodes = node_registry();
    let application = Arc::new(CaptureControl::default());
    application
        .startup_frames_count
        .store(16, Ordering::Release);
    let microphone = Arc::new(CaptureControl::default());
    microphone.startup_frames_count.store(16, Ordering::Release);
    let endpoints = Arc::new(EndpointControl::default());
    endpoints
        .consume_after_gate_delay_ms
        .store(25, Ordering::Release);
    let application_backend = capture_backend(&application, 11);
    let microphone_backend = capture_backend(&microphone, 22);
    let registry = endpoint_registry(&endpoints);

    let mut running = start_prepared_session(
        prepared_session(&nodes, &registry),
        capture_backend_set(&application_backend, &microphone_backend),
        &registry,
        SessionStartOptions::default(),
    )
    .expect("transactional Session startup must succeed");
    let delivery_deadline = Instant::now() + Duration::from_secs(1);
    while endpoints.deliveries_total.load(Ordering::Acquire) < 6
        && Instant::now() < delivery_deadline
    {
        std::thread::sleep(Duration::from_millis(1));
    }
    assert_eq!(
        endpoints.deliveries_total.load(Ordering::Acquire),
        6,
        "both post-start source frames must reach all three destinations"
    );
    let (sources, routes) = running.indexed_metrics();
    let outcome = running.stop();

    assert!(outcome.is_success());
    assert_eq!(sources.len(), 2);
    assert!(sources.iter().all(|source| {
        source
            .capture
            .frame_stream
            .frames_discarded_before_start_total
            >= 16
            && source.ingress.frames_enqueued_total == 1
            && source.ingress.frames_delivered_total == 1
            && source.ingress.frames_rejected_full_total == 0
            && source.ingress.frames_rejected_cancelled_total == 0
            && source.ingress.frames_discarded_total == 0
    }));
    assert!(
        routes
            .iter()
            .all(|route| route.edge.frames_dropped_total == 0),
        "capture frames accumulated before Running must not overflow destination edges"
    );
    assert_no_live_owners(&application, &microphone, &endpoints);
}

#[test]
fn given_one_source_failure_when_runtime_continues_then_healthy_source_frame_is_delivered() {
    let nodes = node_registry();
    let application = Arc::new(CaptureControl::default());
    application
        .emit_source_unavailable
        .store(true, Ordering::Release);
    let microphone = Arc::new(CaptureControl::default());
    let endpoints = Arc::new(EndpointControl::default());
    let application_backend = capture_backend(&application, 11);
    let microphone_backend = capture_backend(&microphone, 22);
    let registry = endpoint_registry(&endpoints);

    let mut running = start_prepared_session(
        prepared_session(&nodes, &registry),
        capture_backend_set(&application_backend, &microphone_backend),
        &registry,
        SessionStartOptions::default(),
    )
    .expect("transactional Session startup must succeed");
    let events = running
        .take_event_receiver()
        .expect("running Session must expose its event receiver");
    std::thread::sleep(Duration::from_millis(30));
    let outcome = running.stop();

    assert!(!outcome.is_success());
    assert_eq!(endpoints.deliveries_total.load(Ordering::Relaxed), 3);
    let mut source_failures_total = 0;
    while let SessionEventReceive::Event(event) = events.try_recv() {
        if matches!(event.kind(), SessionEventKind::Source(_)) {
            source_failures_total += 1;
        }
    }
    assert_eq!(source_failures_total, 1);
    assert_no_live_owners(&application, &microphone, &endpoints);
}

#[test]
fn given_endpoint_prepare_failure_when_started_then_every_prior_owner_rolls_back() {
    let nodes = node_registry();
    let application = Arc::new(CaptureControl::default());
    let microphone = Arc::new(CaptureControl::default());
    let endpoints = Arc::new(EndpointControl::default());
    endpoints.fail_prepare_call.store(3, Ordering::Release);
    let registry = endpoint_registry(&endpoints);
    let application_backend = capture_backend(&application, 11);
    let microphone_backend = capture_backend(&microphone, 22);

    let result = start_prepared_session(
        prepared_session(&nodes, &registry),
        capture_backend_set(&application_backend, &microphone_backend),
        &registry,
        SessionStartOptions::default(),
    );

    assert!(matches!(
        result.as_ref().map_err(|failure| failure.error()),
        Err(SessionStartError::EndpointPrepare { .. })
    ));
    assert_no_live_owners(&application, &microphone, &endpoints);
}

#[test]
fn given_second_capture_open_failure_when_started_then_captures_and_endpoints_roll_back() {
    let nodes = node_registry();
    let application = Arc::new(CaptureControl::default());
    let microphone = Arc::new(CaptureControl::default());
    microphone.fail_open.store(true, Ordering::Release);
    application.fail_stop.store(true, Ordering::Release);
    let endpoints = Arc::new(EndpointControl::default());
    let registry = endpoint_registry(&endpoints);
    let application_backend = capture_backend(&application, 11);
    let microphone_backend = capture_backend(&microphone, 22);

    let result = start_prepared_session(
        prepared_session(&nodes, &registry),
        capture_backend_set(&application_backend, &microphone_backend),
        &registry,
        SessionStartOptions::default(),
    );

    let mut failure = match result {
        Ok(_) => panic!("second capture open must fail"),
        Err(failure) => failure,
    };
    assert!(matches!(
        failure.error(),
        SessionStartError::CaptureOpen { .. }
    ));
    assert_eq!(failure.rollback_failures().len(), 1);
    assert_failed_start_events(&mut failure, 1);
    assert_eq!(application.stop_calls_total.load(Ordering::Relaxed), 1);
    assert_no_live_owners(&application, &microphone, &endpoints);
}

#[test]
fn given_endpoint_start_failure_when_started_then_all_acquisitions_roll_back() {
    let nodes = node_registry();
    let application = Arc::new(CaptureControl::default());
    let microphone = Arc::new(CaptureControl::default());
    let endpoints = Arc::new(EndpointControl::default());
    endpoints.fail_start_call.store(2, Ordering::Release);
    endpoints.fail_join_finalize.store(true, Ordering::Release);
    let registry = endpoint_registry(&endpoints);
    let application_backend = capture_backend(&application, 11);
    let microphone_backend = capture_backend(&microphone, 22);

    let result = start_prepared_session(
        prepared_session(&nodes, &registry),
        capture_backend_set(&application_backend, &microphone_backend),
        &registry,
        SessionStartOptions::default(),
    );

    let mut failure = match result {
        Ok(_) => panic!("endpoint start must fail"),
        Err(failure) => failure,
    };
    assert!(matches!(
        failure.error(),
        SessionStartError::EndpointStart { .. }
    ));
    assert!(!failure.rollback_failures().is_empty());
    let rollback_failures_total = failure.rollback_failures().len();
    assert_failed_start_events(&mut failure, rollback_failures_total);
    assert_no_live_owners(&application, &microphone, &endpoints);
}

fn assert_failed_start_events(
    failure: &mut crate::session::SessionStartFailure,
    expected_rollback_failures: usize,
) {
    let events = failure
        .take_event_receiver()
        .expect("post-channel startup failure must retain its event receiver");
    let mut rollback_events = 0usize;
    let mut failed = false;
    let mut terminal = None;
    while let SessionEventReceive::Event(event) = events.try_recv() {
        match event.kind() {
            SessionEventKind::Rollback(_) => rollback_events += 1,
            SessionEventKind::Lifecycle(SessionLifecycleState::Failed) => failed = true,
            SessionEventKind::Terminal(outcome) => terminal = Some(outcome.clone()),
            _ => {}
        }
    }
    assert_eq!(rollback_events, expected_rollback_failures);
    assert!(failed);
    let terminal = terminal.expect("failed startup must publish a terminal outcome");
    assert_eq!(terminal.state(), SessionTerminalState::Failed);
    assert_eq!(
        terminal.rollback_failures().len(),
        expected_rollback_failures
    );
}
