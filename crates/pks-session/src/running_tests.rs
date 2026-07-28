use std::sync::atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use pks_caps::{AudioCaps, ChannelLayout, MediaCaps, Multiplicity, PortDirection, PortSpec};
use pks_capture::{
    ActiveCaptureBackend, CallbackCaptureBackend, CaptureDelivery, CaptureError, CaptureMode,
    CaptureObservationCounters, CaptureObservationHandle, CaptureObservations,
    CaptureRuntimeFailure, CaptureRuntimeFailureClass, CapturedFrameDelivery,
    PreparedCaptureBackend, SourceGeneration, SourceKind, SourceRecoveryRequirement,
    SourceRuntimeEvent, SourceRuntimeEventSender, StableSourceId,
};
use pks_endpoint::{
    EndpointCancellationOutcome, EndpointDriverFactory, EndpointDriverFinalization,
    EndpointDriverInput, EndpointDriverObservations, EndpointDriverRegistry, EndpointFailure,
    EndpointFailureStage, EndpointStartGate, PreparedEndpointDriver, RunningEndpointDriver,
};
use pks_frame::{
    AudioBufferPool, AudioFrame, EndpointId, Platform, RouteId, SampleFormat, SampleSpec,
    SessionId, SourceId, StemId, StreamId,
};
use pks_graph::{
    ConfigError, ExecutionPartition, NodeConfig, NodeDescriptor, NodeError, NodeFactory,
    NodeRegistry, NodeTypeId, PrepareContext, RuntimeNode,
};
use pks_runtime::{PlanEdgeFrame, PlanEdgeReceiver};

use crate::{
    prepare_session_runtime, start_prepared_session, ApplicationSelector, CaptureBackendSet,
    EndpointConfiguration, OperatorId, OperatorRegistry, Session, SessionCompiler,
    SessionEventKind, SessionEventReceive, SessionLifecycleState, SessionStartError,
    SessionStartOptions, SessionTerminalState, Source, APPLICATION_SOURCE_NODE_TYPE_ID,
    BROWSER_NODE_TYPE_ID, BROWSER_OPERATOR_ID, CONNECTOR_NODE_TYPE_ID,
    MICROPHONE_SOURCE_NODE_TYPE_ID, RECORDER_NODE_TYPE_ID, RECORDER_OPERATOR_ID,
};

const TEST_CONNECTOR_OPERATOR_ID: &str = "example.connector.running-session.v1";

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
        realtime_safe: source,
        stateful: true,
    }
}

fn registries() -> (NodeRegistry, OperatorRegistry) {
    let mut nodes = NodeRegistry::new();
    for node_type_id in [
        APPLICATION_SOURCE_NODE_TYPE_ID,
        MICROPHONE_SOURCE_NODE_TYPE_ID,
    ] {
        nodes.register(Arc::new(TestNodeFactory {
            descriptor: descriptor(node_type_id, TestNodeRole::Source),
        }));
    }
    for node_type_id in [
        CONNECTOR_NODE_TYPE_ID,
        BROWSER_NODE_TYPE_ID,
        RECORDER_NODE_TYPE_ID,
    ] {
        nodes.register(Arc::new(TestNodeFactory {
            descriptor: descriptor(node_type_id, TestNodeRole::Endpoint),
        }));
    }
    let mut operators = OperatorRegistry::new();
    for (operator_id, node_type_id) in [
        (TEST_CONNECTOR_OPERATOR_ID, CONNECTOR_NODE_TYPE_ID),
        (BROWSER_OPERATOR_ID, BROWSER_NODE_TYPE_ID),
        (RECORDER_OPERATOR_ID, RECORDER_NODE_TYPE_ID),
    ] {
        operators
            .register(OperatorId::new(operator_id), NodeTypeId::from(node_type_id))
            .expect("test operator registration must succeed");
    }
    (nodes, operators)
}

fn product_spec() -> crate::SessionSpec {
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

fn prepared_session(nodes: &NodeRegistry, operators: &OperatorRegistry) -> crate::PreparedSession {
    let compiled = SessionCompiler::new(nodes, operators)
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
    let (nodes, operators) = registries();
    let application = Arc::new(CaptureControl::default());
    let microphone = Arc::new(CaptureControl::default());
    let endpoints = Arc::new(EndpointControl::default());
    let application_backend = capture_backend(&application, 11);
    let microphone_backend = capture_backend(&microphone, 22);
    let registry = endpoint_registry(&endpoints);

    let mut running = start_prepared_session(
        prepared_session(&nodes, &operators),
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
    let (nodes, operators) = registries();
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
        prepared_session(&nodes, &operators),
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
    let (nodes, operators) = registries();
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
        prepared_session(&nodes, &operators),
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
    let (nodes, operators) = registries();
    let application = Arc::new(CaptureControl::default());
    let microphone = Arc::new(CaptureControl::default());
    let endpoints = Arc::new(EndpointControl::default());
    endpoints.fail_prepare_call.store(3, Ordering::Release);
    let registry = endpoint_registry(&endpoints);
    let application_backend = capture_backend(&application, 11);
    let microphone_backend = capture_backend(&microphone, 22);

    let result = start_prepared_session(
        prepared_session(&nodes, &operators),
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
    let (nodes, operators) = registries();
    let application = Arc::new(CaptureControl::default());
    let microphone = Arc::new(CaptureControl::default());
    microphone.fail_open.store(true, Ordering::Release);
    application.fail_stop.store(true, Ordering::Release);
    let endpoints = Arc::new(EndpointControl::default());
    let registry = endpoint_registry(&endpoints);
    let application_backend = capture_backend(&application, 11);
    let microphone_backend = capture_backend(&microphone, 22);

    let result = start_prepared_session(
        prepared_session(&nodes, &operators),
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
    let (nodes, operators) = registries();
    let application = Arc::new(CaptureControl::default());
    let microphone = Arc::new(CaptureControl::default());
    let endpoints = Arc::new(EndpointControl::default());
    endpoints.fail_start_call.store(2, Ordering::Release);
    endpoints.fail_join_finalize.store(true, Ordering::Release);
    let registry = endpoint_registry(&endpoints);
    let application_backend = capture_backend(&application, 11);
    let microphone_backend = capture_backend(&microphone, 22);

    let result = start_prepared_session(
        prepared_session(&nodes, &operators),
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
    failure: &mut crate::SessionStartFailure,
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
