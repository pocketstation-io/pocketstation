use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

use pks_capture::{
    ActiveCaptureBackend, CallbackCaptureBackend, CaptureDelivery, CaptureError, CaptureMode,
    CaptureObservationHandle, CaptureObservations, CapturedFrameDelivery, PreparedCaptureBackend,
};
use pks_endpoint::{
    EndpointCancellationOutcome, EndpointDriverFactory, EndpointDriverFinalization,
    EndpointDriverInput, EndpointDriverObservations, EndpointFailure, EndpointStartGate,
    PreparedEndpointDriver, RunningEndpointDriver,
};
use pks_frame::{AudioBufferPool, AudioFrame, SampleFormat, SampleSpec, SourceId, StreamId};
use pks_graph::{NodeTypeId, PrepareContext};

use crate::{
    ApplicationSelector, CaptureBackendSet, DeviceSelector, EndpointConfiguration, OperatorId,
    PolledAudioEndpoint, PolledAudioEndpointConfig, PolledAudioPollError, Session,
    SessionCompileError, SessionEngineBuilder, SessionEngineRegistrationError,
    SessionEngineStartError, SessionStartError, SessionStartOptions, Source, BROWSER_NODE_TYPE_ID,
    BROWSER_OPERATOR_ID, CONNECTOR_NODE_TYPE_ID, RECORDER_NODE_TYPE_ID, RECORDER_OPERATOR_ID,
};

const CONNECTOR_OPERATOR_ID: &str = "test.connector.engine.v1";

#[derive(Default)]
struct TestCaptureBackend {
    fail_open: AtomicBool,
}

struct TestPreparedCapture {
    fail_open: bool,
}

struct TestActiveCapture;

struct DeliveringCaptureBackend;

struct DeliveringPreparedCapture;

struct DeliveringActiveCapture {
    stop_requested: Arc<AtomicBool>,
    worker: Option<std::thread::JoinHandle<()>>,
}

impl CallbackCaptureBackend for TestCaptureBackend {
    fn prepare(&self, _mode: CaptureMode) -> Result<Box<dyn PreparedCaptureBackend>, CaptureError> {
        Ok(Box::new(TestPreparedCapture {
            fail_open: self.fail_open.load(Ordering::Acquire),
        }))
    }
}

impl PreparedCaptureBackend for TestPreparedCapture {
    fn open(
        self: Box<Self>,
        _delivery: CaptureDelivery,
    ) -> Result<Box<dyn ActiveCaptureBackend>, CaptureError> {
        if self.fail_open {
            Err(CaptureError::BackendInit(
                "test capture open failure".to_owned(),
            ))
        } else {
            Ok(Box::new(TestActiveCapture))
        }
    }
}

impl ActiveCaptureBackend for TestActiveCapture {
    fn observation_handle(&self) -> CaptureObservationHandle {
        CaptureObservationHandle::default()
    }

    fn observations(&self) -> CaptureObservations {
        CaptureObservations::default()
    }

    fn stop_and_join(self: Box<Self>) -> Result<CaptureObservations, CaptureError> {
        Ok(CaptureObservations::default())
    }
}

impl CallbackCaptureBackend for DeliveringCaptureBackend {
    fn prepare(&self, _mode: CaptureMode) -> Result<Box<dyn PreparedCaptureBackend>, CaptureError> {
        Ok(Box::new(DeliveringPreparedCapture))
    }
}

impl PreparedCaptureBackend for DeliveringPreparedCapture {
    fn open(
        self: Box<Self>,
        delivery: CaptureDelivery,
    ) -> Result<Box<dyn ActiveCaptureBackend>, CaptureError> {
        let stop_requested = Arc::new(AtomicBool::new(false));
        let worker_stop_requested = Arc::clone(&stop_requested);
        let worker = std::thread::spawn(move || {
            let pool = AudioBufferPool::new(1, 4);
            let mut frame_sender = delivery.frame_sender;
            while !worker_stop_requested.load(Ordering::Acquire) {
                let Some(mut buffer) = pool.acquire() else {
                    std::thread::sleep(Duration::from_millis(1));
                    continue;
                };
                buffer.copy_from_slice(&[0.125, 0.25, 0.5, 1.0]);
                let frame = AudioFrame::new(StreamId(11), SourceId(12), 13, 14, 1, buffer);
                match frame_sender.try_send(frame) {
                    CapturedFrameDelivery::Delivered => break,
                    CapturedFrameDelivery::DroppedNewest
                    | CapturedFrameDelivery::DiscardedBeforeStart => {
                        std::thread::sleep(Duration::from_millis(1));
                    }
                }
            }
        });
        Ok(Box::new(DeliveringActiveCapture {
            stop_requested,
            worker: Some(worker),
        }))
    }
}

impl ActiveCaptureBackend for DeliveringActiveCapture {
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
                    worker: "deterministic capture worker",
                })?;
        }
        Ok(CaptureObservations::default())
    }
}

impl Drop for DeliveringActiveCapture {
    fn drop(&mut self) {
        self.stop_requested.store(true, Ordering::Release);
        if let Some(worker) = self.worker.take() {
            let _ = worker.join();
        }
    }
}

#[derive(Default)]
struct TestEndpointControl {
    starts_total: AtomicU64,
    started_before_gate_total: AtomicU64,
    mono_inputs_total: AtomicU64,
    stereo_inputs_total: AtomicU64,
}

struct TestEndpointFactory {
    control: Arc<TestEndpointControl>,
}

struct TestPreparedEndpoint {
    control: Arc<TestEndpointControl>,
}

struct TestRunningEndpoint;

impl EndpointDriverFactory for TestEndpointFactory {
    fn prepare(
        &self,
        inputs: Vec<EndpointDriverInput>,
    ) -> Result<Box<dyn PreparedEndpointDriver>, EndpointFailure> {
        assert!(!inputs.is_empty());
        for input in &inputs {
            match input.context().node_prepare_context().sample_spec.channels {
                1 => {
                    self.control
                        .mono_inputs_total
                        .fetch_add(1, Ordering::Relaxed);
                }
                2 => {
                    self.control
                        .stereo_inputs_total
                        .fetch_add(1, Ordering::Relaxed);
                }
                channels => panic!("unexpected endpoint input channel count {channels}"),
            }
        }
        Ok(Box::new(TestPreparedEndpoint {
            control: Arc::clone(&self.control),
        }))
    }
}

impl PreparedEndpointDriver for TestPreparedEndpoint {
    fn start(
        self: Box<Self>,
        start_gate: Arc<EndpointStartGate>,
    ) -> Result<Box<dyn RunningEndpointDriver>, EndpointFailure> {
        self.control.starts_total.fetch_add(1, Ordering::Relaxed);
        if start_gate.is_open() {
            self.control
                .started_before_gate_total
                .fetch_add(1, Ordering::Relaxed);
        }
        Ok(Box::new(TestRunningEndpoint))
    }

    fn cancel_preparation(self: Box<Self>) -> EndpointCancellationOutcome {
        EndpointCancellationOutcome {
            observations: EndpointDriverObservations::default(),
            result: Ok(()),
        }
    }
}

impl RunningEndpointDriver for TestRunningEndpoint {
    fn observations(&self) -> EndpointDriverObservations {
        EndpointDriverObservations::default()
    }

    fn request_stop(&mut self) -> Result<(), EndpointFailure> {
        Ok(())
    }

    fn join_and_finalize(self: Box<Self>) -> EndpointDriverFinalization {
        EndpointDriverFinalization {
            observations: EndpointDriverObservations::default(),
            result: Ok(()),
        }
    }
}

fn prepare_context() -> PrepareContext {
    PrepareContext::new(SampleSpec::new(48_000, 1, SampleFormat::F32Interleaved))
}

fn endpoint_factory(control: &Arc<TestEndpointControl>) -> Arc<dyn EndpointDriverFactory> {
    Arc::new(TestEndpointFactory {
        control: Arc::clone(control),
    })
}

fn product_session() -> Session {
    let session = Session::new();
    let application = session
        .capture(Source::Application(ApplicationSelector::Name(
            "test application".to_owned(),
        )))
        .expect("application declaration");
    let microphone = session
        .capture(Source::Microphone(DeviceSelector::Default))
        .expect("microphone declaration");
    let connector = session
        .connector(
            OperatorId::new(CONNECTOR_OPERATOR_ID),
            EndpointConfiguration::new(),
        )
        .expect("connector declaration");
    let browser = session
        .browser("https://receiver.test/session")
        .expect("browser declaration");
    application
        .send(connector)
        .expect("application connector route");
    microphone
        .send(connector)
        .expect("microphone connector route");
    application
        .send(browser)
        .expect("application browser route");
    microphone.send(browser).expect("microphone browser route");
    application
        .record("application")
        .expect("application record");
    microphone.record("microphone").expect("microphone record");
    session
}

#[test]
fn given_product_plan_when_prepared_then_stem_media_is_preserved() {
    let control = Arc::new(TestEndpointControl::default());
    let mut builder =
        SessionEngineBuilder::new(prepare_context(), 8, SessionStartOptions::default())
            .expect("engine builder");
    builder
        .register_endpoint_driver(
            OperatorId::new(CONNECTOR_OPERATOR_ID),
            NodeTypeId::from(CONNECTOR_NODE_TYPE_ID),
            endpoint_factory(&control),
        )
        .expect("connector registration");
    builder
        .register_endpoint_driver(
            OperatorId::new(BROWSER_OPERATOR_ID),
            NodeTypeId::from(BROWSER_NODE_TYPE_ID),
            endpoint_factory(&control),
        )
        .expect("browser registration");
    builder
        .register_endpoint_driver(
            OperatorId::new(RECORDER_OPERATOR_ID),
            NodeTypeId::from(RECORDER_NODE_TYPE_ID),
            endpoint_factory(&control),
        )
        .expect("recorder registration");
    let engine = builder.build().expect("engine build");
    let capture = TestCaptureBackend::default();

    let mut running = engine
        .start(
            product_session(),
            CaptureBackendSet {
                application: &capture,
                microphone: &capture,
            },
        )
        .expect("canonical Session start");

    assert_eq!(control.starts_total.load(Ordering::Relaxed), 5);
    assert_eq!(control.started_before_gate_total.load(Ordering::Relaxed), 0);
    assert_eq!(control.stereo_inputs_total.load(Ordering::Relaxed), 3);
    assert_eq!(control.mono_inputs_total.load(Ordering::Relaxed), 3);
    assert!(running.stop().is_success());
    assert!(running.stop().is_success());
}

#[test]
fn given_separated_engine_stages_when_started_then_canonical_owners_are_preserved() {
    let control = Arc::new(TestEndpointControl::default());
    let mut builder =
        SessionEngineBuilder::new(prepare_context(), 8, SessionStartOptions::default())
            .expect("engine builder");
    for (operator_id, node_type_id) in [
        (CONNECTOR_OPERATOR_ID, CONNECTOR_NODE_TYPE_ID),
        (BROWSER_OPERATOR_ID, BROWSER_NODE_TYPE_ID),
        (RECORDER_OPERATOR_ID, RECORDER_NODE_TYPE_ID),
    ] {
        builder
            .register_endpoint_driver(
                OperatorId::new(operator_id),
                NodeTypeId::from(node_type_id),
                endpoint_factory(&control),
            )
            .expect("endpoint registration");
    }
    let engine = builder.build().expect("engine build");
    let compiled = engine.compile(product_session()).expect("Session compile");
    let capture = TestCaptureBackend::default();
    let mut running = engine
        .start_compiled(
            compiled,
            CaptureBackendSet {
                application: &capture,
                microphone: &capture,
            },
        )
        .expect("prepared Session start");

    assert!(running.stop().is_success());
}

#[test]
fn given_deterministic_capture_when_polled_then_real_runtime_branch_copy_and_lineage_are_exposed() {
    let endpoint = PolledAudioEndpoint::new(PolledAudioEndpointConfig {
        queue_capacity_frames: 4,
        max_batch_frames: 2,
        max_outstanding_leases: 2,
    })
    .expect("polled endpoint");
    let receipt = endpoint.receipt();
    let mut builder =
        SessionEngineBuilder::new(prepare_context(), 4, SessionStartOptions::default())
            .expect("engine builder");
    builder
        .register_polled_audio_endpoint(&endpoint)
        .expect("polled endpoint registration");
    let engine = builder.build().expect("engine build");

    let session = Session::new();
    let session_id = session.id();
    let application = session
        .capture(Source::application(ApplicationSelector::name(
            "deterministic application",
        )))
        .expect("application declaration");
    let microphone = session
        .capture(Source::microphone(DeviceSelector::default()))
        .expect("microphone declaration");
    let application_output = session
        .polled_audio()
        .expect("application output declaration");
    let microphone_output = session
        .polled_audio()
        .expect("microphone output declaration");
    let application_route = application
        .send(application_output)
        .expect("application polled route");
    let microphone_route = microphone
        .send(microphone_output)
        .expect("microphone polled route");
    let capture = DeliveringCaptureBackend;
    let mut running = engine
        .start(
            session,
            CaptureBackendSet {
                application: &capture,
                microphone: &capture,
            },
        )
        .expect("canonical Session start");

    let deadline = Instant::now() + Duration::from_secs(2);
    let lease = loop {
        match receipt.try_poll() {
            Ok(lease) => break lease,
            Err(PolledAudioPollError::Empty) if Instant::now() < deadline => {
                std::thread::sleep(Duration::from_millis(1));
            }
            Err(error) => panic!("polled endpoint did not receive capture: {error}"),
        }
    };
    let frame = lease.frame(0).expect("captured frame");
    let samples_pointer = frame.samples().as_ptr();
    assert_eq!(frame.samples(), &[0.125, 0.25, 0.5, 1.0]);
    assert_eq!(frame.lineage().session_id, session_id);
    if frame.lineage().stem_id == application.id() {
        assert_eq!(frame.endpoint_id(), application_output.id());
        assert_eq!(
            frame.connector_id(),
            application_output
                .connector_id()
                .expect("connector identity")
        );
        assert_eq!(frame.route_id(), application_route);
    } else {
        assert_eq!(frame.lineage().stem_id, microphone.id());
        assert_eq!(frame.endpoint_id(), microphone_output.id());
        assert_eq!(
            frame.connector_id(),
            microphone_output
                .connector_id()
                .expect("connector identity")
        );
        assert_eq!(frame.route_id(), microphone_route);
    }
    assert_eq!(frame.lineage().source_id, SourceId(12));
    assert_eq!(frame.lineage().sequence_num, 13);
    assert_eq!(frame.lineage().timestamp_start_ns, 14);
    assert_eq!(receipt.observations().invalid_ownership_drops_total, 0);

    assert!(running.stop().is_success());
    assert_eq!(frame.samples().as_ptr(), samples_pointer);
    assert_eq!(frame.samples(), &[0.125, 0.25, 0.5, 1.0]);
    drop(lease);
    assert_eq!(receipt.observations().outstanding_leases, 0);
}

#[test]
fn given_duplicate_or_conflicting_operator_when_registering_then_builder_fails_typed() {
    let control = Arc::new(TestEndpointControl::default());
    let mut duplicate_builder =
        SessionEngineBuilder::new(prepare_context(), 8, SessionStartOptions::default())
            .expect("engine builder");
    duplicate_builder
        .register_endpoint_driver(
            OperatorId::new(CONNECTOR_OPERATOR_ID),
            NodeTypeId::from(CONNECTOR_NODE_TYPE_ID),
            endpoint_factory(&control),
        )
        .expect("first registration");
    let duplicate = duplicate_builder.register_endpoint_driver(
        OperatorId::new(CONNECTOR_OPERATOR_ID),
        NodeTypeId::from(CONNECTOR_NODE_TYPE_ID),
        endpoint_factory(&control),
    );
    assert!(matches!(
        duplicate,
        Err(SessionEngineRegistrationError::DuplicateEndpointDriver { .. })
    ));

    let conflict = duplicate_builder.register_endpoint_driver(
        OperatorId::new(CONNECTOR_OPERATOR_ID),
        NodeTypeId::from(BROWSER_NODE_TYPE_ID),
        endpoint_factory(&control),
    );
    assert!(matches!(
        conflict,
        Err(SessionEngineRegistrationError::OperatorNodeTypeConflict { .. })
    ));
}

#[test]
fn given_unregistered_declared_operator_when_starting_then_compile_failure_remains_typed() {
    let engine = SessionEngineBuilder::new(prepare_context(), 8, SessionStartOptions::default())
        .expect("engine builder")
        .build()
        .expect("engine build");
    let capture = TestCaptureBackend::default();

    let error = match engine.start(
        product_session(),
        CaptureBackendSet {
            application: &capture,
            microphone: &capture,
        },
    ) {
        Ok(_) => panic!("unregistered endpoint operators must fail"),
        Err(error) => error,
    };

    assert!(matches!(
        error,
        SessionEngineStartError::Compile(SessionCompileError::UnknownOperator { .. })
    ));
}

#[test]
fn given_capture_open_failure_when_starting_then_transactional_failure_is_preserved() {
    let control = Arc::new(TestEndpointControl::default());
    let mut builder =
        SessionEngineBuilder::new(prepare_context(), 8, SessionStartOptions::default())
            .expect("engine builder");
    for (operator_id, node_type_id) in [
        (CONNECTOR_OPERATOR_ID, CONNECTOR_NODE_TYPE_ID),
        (BROWSER_OPERATOR_ID, BROWSER_NODE_TYPE_ID),
        (RECORDER_OPERATOR_ID, RECORDER_NODE_TYPE_ID),
    ] {
        builder
            .register_endpoint_driver(
                OperatorId::new(operator_id),
                NodeTypeId::from(node_type_id),
                endpoint_factory(&control),
            )
            .expect("endpoint registration");
    }
    let engine = builder.build().expect("engine build");
    let capture = TestCaptureBackend {
        fail_open: AtomicBool::new(true),
    };

    let error = match engine.start(
        product_session(),
        CaptureBackendSet {
            application: &capture,
            microphone: &capture,
        },
    ) {
        Ok(_) => panic!("capture open must fail"),
        Err(error) => error,
    };
    let failure = error
        .start_failure()
        .expect("transactional start failure must remain available");
    assert!(matches!(
        failure.error(),
        SessionStartError::CaptureOpen { .. }
    ));
}
