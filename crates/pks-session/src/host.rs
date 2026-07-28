use std::sync::Arc;

use pks_capture::CallbackCaptureBackend;
use pks_frame::{SampleFormat, SampleSpec};
use pks_graph::PrepareContext;

#[cfg(target_os = "linux")]
use pks_capture_linux::DesktopCaptureBackend as NativeDesktopCaptureBackend;
#[cfg(target_os = "macos")]
use pks_capture_macos::DesktopCaptureBackend as NativeDesktopCaptureBackend;
#[cfg(target_os = "windows")]
use pks_capture_windows::DesktopCaptureBackend as NativeDesktopCaptureBackend;

use crate::{
    CaptureBackendSet, CompiledSession, PolledAudioEndpoint, PolledAudioEndpointConfig,
    PolledAudioEndpointConfigError, PolledAudioReceipt, RunningSession, Session, SessionEngine,
    SessionEngineBuildError, SessionEngineBuilder, SessionEngineRegistrationError,
    SessionEngineStartError, SessionEventReceiver, SessionMetricsSnapshot,
    SessionStartCancellation, SessionStartOptions,
};

/// Safe host-owned Session environment for foreign-language adapters.
///
/// The host owns the real capture backends, the canonical Session engine, and
/// any bounded polled-audio receipts registered for foreign retention. Future
/// portability layers can project this owner without inventing a second
/// lifecycle or media-runtime authority.
pub struct SessionEngineHost {
    engine: SessionEngine,
    application_backend: Arc<dyn CallbackCaptureBackend>,
    microphone_backend: Arc<dyn CallbackCaptureBackend>,
    polled_audio_receipts: Box<[PolledAudioReceipt]>,
}

impl SessionEngineHost {
    pub fn native(
        options: NativeSessionEngineHostOptions,
    ) -> Result<Self, SessionEngineHostBuildError> {
        build_native_host(options)
    }

    pub fn compile(&self, session: Session) -> Result<CompiledSession, SessionEngineStartError> {
        self.engine.compile(session)
    }

    pub fn start(&self, session: Session) -> Result<RunningSession, SessionEngineStartError> {
        self.engine.start(
            session,
            CaptureBackendSet {
                application: self.application_backend.as_ref(),
                microphone: self.microphone_backend.as_ref(),
            },
        )
    }

    pub fn start_compiled(
        &self,
        compiled: CompiledSession,
    ) -> Result<RunningSession, SessionEngineStartError> {
        self.engine.start_compiled(
            compiled,
            CaptureBackendSet {
                application: self.application_backend.as_ref(),
                microphone: self.microphone_backend.as_ref(),
            },
        )
    }

    pub fn start_compiled_cancellable(
        &self,
        compiled: CompiledSession,
        start_cancellation: SessionStartCancellation,
    ) -> Result<RunningSession, SessionEngineStartError> {
        self.engine.start_compiled_cancellable(
            compiled,
            CaptureBackendSet {
                application: self.application_backend.as_ref(),
                microphone: self.microphone_backend.as_ref(),
            },
            start_cancellation,
        )
    }

    pub fn polled_audio_receipt(&self, index: usize) -> Option<PolledAudioReceipt> {
        self.polled_audio_receipts.get(index).cloned()
    }

    pub fn polled_audio_receipts_total(&self) -> usize {
        self.polled_audio_receipts.len()
    }

    pub fn metrics_snapshot(
        &self,
        events: &SessionEventReceiver,
        polled_audio_receipt_index: usize,
        running_session: Option<&RunningSession>,
    ) -> Option<SessionMetricsSnapshot> {
        self.polled_audio_receipts
            .get(polled_audio_receipt_index)
            .map(|receipt| {
                let (sources, routes) = running_session.map_or_else(
                    || (Box::default(), Box::default()),
                    RunningSession::indexed_metrics,
                );
                SessionMetricsSnapshot::new(
                    events.observations(),
                    receipt.observations(),
                    sources,
                    routes,
                )
            })
    }

    pub fn engine(&self) -> &SessionEngine {
        &self.engine
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NativeSessionEngineHostOptions {
    pub source_queue_capacity_frames: usize,
    pub start_options: SessionStartOptions,
    pub polled_audio_endpoint: PolledAudioEndpointConfig,
}

impl Default for NativeSessionEngineHostOptions {
    fn default() -> Self {
        Self {
            source_queue_capacity_frames: 32,
            start_options: SessionStartOptions::default(),
            polled_audio_endpoint: PolledAudioEndpointConfig::default(),
        }
    }
}

/// Setup-time owner for the canonical Session host.
///
/// This builder deliberately mirrors the engine's real registration seams while
/// also requiring the two concrete capture backends needed by the current
/// product slice. The portable C surface can depend on this owner without
/// synthesizing a parallel runtime.
pub struct SessionEngineHostBuilder {
    engine_builder: SessionEngineBuilder,
    application_backend: Option<Arc<dyn CallbackCaptureBackend>>,
    microphone_backend: Option<Arc<dyn CallbackCaptureBackend>>,
    polled_audio_receipts: Vec<PolledAudioReceipt>,
}

impl SessionEngineHostBuilder {
    pub fn new(
        prepare_context: pks_graph::PrepareContext,
        source_queue_capacity_frames: usize,
        start_options: SessionStartOptions,
    ) -> Result<Self, SessionEngineBuildError> {
        Ok(Self {
            engine_builder: SessionEngineBuilder::new(
                prepare_context,
                source_queue_capacity_frames,
                start_options,
            )?,
            application_backend: None,
            microphone_backend: None,
            polled_audio_receipts: Vec::new(),
        })
    }

    pub fn set_application_backend(
        &mut self,
        backend: Arc<dyn CallbackCaptureBackend>,
    ) -> &mut Self {
        self.application_backend = Some(backend);
        self
    }

    pub fn set_microphone_backend(
        &mut self,
        backend: Arc<dyn CallbackCaptureBackend>,
    ) -> &mut Self {
        self.microphone_backend = Some(backend);
        self
    }

    pub fn engine_builder(&mut self) -> &mut SessionEngineBuilder {
        &mut self.engine_builder
    }

    pub fn register_polled_audio_endpoint(
        &mut self,
        config: PolledAudioEndpointConfig,
    ) -> Result<PolledAudioReceipt, SessionEngineHostBuildError> {
        let endpoint = PolledAudioEndpoint::new(config)?;
        self.engine_builder
            .register_polled_audio_endpoint(&endpoint)?;
        let receipt = endpoint.receipt();
        self.polled_audio_receipts.push(receipt.clone());
        Ok(receipt)
    }

    pub fn build(self) -> Result<SessionEngineHost, SessionEngineHostBuildError> {
        let application_backend = self
            .application_backend
            .ok_or(SessionEngineHostBuildError::MissingApplicationBackend)?;
        let microphone_backend = self
            .microphone_backend
            .ok_or(SessionEngineHostBuildError::MissingMicrophoneBackend)?;
        Ok(SessionEngineHost {
            engine: self.engine_builder.build()?,
            application_backend,
            microphone_backend,
            polled_audio_receipts: self.polled_audio_receipts.into_boxed_slice(),
        })
    }
}

#[derive(Debug, thiserror::Error)]
pub enum SessionEngineHostBuildError {
    #[error(transparent)]
    Engine(#[from] SessionEngineBuildError),
    #[error(transparent)]
    EndpointRegistration(#[from] SessionEngineRegistrationError),
    #[error(transparent)]
    PolledAudioEndpoint(#[from] PolledAudioEndpointConfigError),
    #[error("application capture backend is required")]
    MissingApplicationBackend,
    #[error("microphone capture backend is required")]
    MissingMicrophoneBackend,
    #[error("native Session capture composition is unsupported on this target")]
    UnsupportedPlatform,
}

#[cfg(any(target_os = "linux", target_os = "macos", target_os = "windows"))]
fn build_native_host(
    options: NativeSessionEngineHostOptions,
) -> Result<SessionEngineHost, SessionEngineHostBuildError> {
    let prepare_context =
        PrepareContext::new(SampleSpec::new(48_000, 1, SampleFormat::F32Interleaved));
    let mut builder = SessionEngineHostBuilder::new(
        prepare_context,
        options.source_queue_capacity_frames,
        options.start_options,
    )?;
    let capture_backend: Arc<dyn CallbackCaptureBackend> = Arc::new(NativeDesktopCaptureBackend);
    builder
        .set_application_backend(Arc::clone(&capture_backend))
        .set_microphone_backend(capture_backend);
    let _ = builder.register_polled_audio_endpoint(options.polled_audio_endpoint)?;
    builder.build()
}

#[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
fn build_native_host(
    _options: NativeSessionEngineHostOptions,
) -> Result<SessionEngineHost, SessionEngineHostBuildError> {
    Err(SessionEngineHostBuildError::UnsupportedPlatform)
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::Arc;
    use std::time::{Duration, Instant};

    use pks_capture::{
        ActiveCaptureBackend, CallbackCaptureBackend, CaptureDelivery, CaptureError, CaptureMode,
        CaptureObservationHandle, CaptureObservations, PreparedCaptureBackend,
    };
    use pks_endpoint::{
        EndpointCancellationOutcome, EndpointDriverFactory, EndpointDriverFinalization,
        EndpointDriverInput, EndpointDriverObservations, EndpointFailure, EndpointStartGate,
        PreparedEndpointDriver, RunningEndpointDriver,
    };
    use pks_frame::{AudioBufferPool, AudioFrame, SampleFormat, SampleSpec, SourceId, StreamId};
    use pks_graph::{NodeTypeId, PrepareContext};

    use crate::{
        ApplicationSelector, DeviceSelector, EndpointConfiguration, OperatorId,
        PolledAudioEndpointConfig, Session, SessionEngineStartError, SessionStartOptions, Source,
        BROWSER_NODE_TYPE_ID, BROWSER_OPERATOR_ID, CONNECTOR_NODE_TYPE_ID, RECORDER_NODE_TYPE_ID,
        RECORDER_OPERATOR_ID,
    };

    use super::{SessionEngineHostBuildError, SessionEngineHostBuilder};

    const CONNECTOR_OPERATOR_ID: &str = "test.connector.host.v1";

    #[derive(Default)]
    struct TestCaptureBackend {
        fail_open: AtomicBool,
        deliver_audio: AtomicBool,
    }

    struct TestPreparedCapture {
        fail_open: bool,
        deliver_audio: bool,
    }

    struct TestActiveCapture;

    impl CallbackCaptureBackend for TestCaptureBackend {
        fn prepare(
            &self,
            _mode: CaptureMode,
        ) -> Result<Box<dyn PreparedCaptureBackend>, CaptureError> {
            Ok(Box::new(TestPreparedCapture {
                fail_open: self.fail_open.load(Ordering::Acquire),
                deliver_audio: self.deliver_audio.load(Ordering::Acquire),
            }))
        }
    }

    impl PreparedCaptureBackend for TestPreparedCapture {
        fn open(
            self: Box<Self>,
            mut delivery: CaptureDelivery,
        ) -> Result<Box<dyn ActiveCaptureBackend>, CaptureError> {
            if self.fail_open {
                return Err(CaptureError::BackendInit(
                    "test capture open failure".to_owned(),
                ));
            }
            if self.deliver_audio {
                let pool = AudioBufferPool::new(1, 4);
                let mut buffer = pool.acquire().expect("deterministic capture buffer");
                buffer.copy_from_slice(&[0.125, 0.25, 0.5, 1.0]);
                let frame = AudioFrame::new(StreamId(11), SourceId(12), 13, 14, 1, buffer);
                let _ = delivery.frame_sender.try_send(frame);
            }
            Ok(Box::new(TestActiveCapture))
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

    struct TestEndpointFactory;

    struct TestPreparedEndpoint;

    struct TestRunningEndpoint;

    impl EndpointDriverFactory for TestEndpointFactory {
        fn prepare(
            &self,
            inputs: Vec<EndpointDriverInput>,
        ) -> Result<Box<dyn PreparedEndpointDriver>, EndpointFailure> {
            assert!(!inputs.is_empty());
            Ok(Box::new(TestPreparedEndpoint))
        }
    }

    impl PreparedEndpointDriver for TestPreparedEndpoint {
        fn start(
            self: Box<Self>,
            _start_gate: Arc<EndpointStartGate>,
        ) -> Result<Box<dyn RunningEndpointDriver>, EndpointFailure> {
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
        application
            .send(connector)
            .expect("application connector route");
        microphone
            .send(connector)
            .expect("microphone connector route");
        let browser = session
            .browser("https://receiver.test/session")
            .expect("browser declaration");
        let application_output = session
            .polled_audio()
            .expect("application polled output declaration");
        let microphone_output = session
            .polled_audio()
            .expect("microphone polled output declaration");
        application
            .send(browser)
            .expect("application browser route");
        microphone.send(browser).expect("microphone browser route");
        application
            .send(application_output)
            .expect("application polled route");
        microphone
            .send(microphone_output)
            .expect("microphone polled route");
        application
            .record("application")
            .expect("application record");
        microphone.record("microphone").expect("microphone record");
        session
    }

    fn register_default_endpoints(builder: &mut SessionEngineHostBuilder) {
        for (operator_id, node_type_id) in [
            (CONNECTOR_OPERATOR_ID, CONNECTOR_NODE_TYPE_ID),
            (BROWSER_OPERATOR_ID, BROWSER_NODE_TYPE_ID),
            (RECORDER_OPERATOR_ID, RECORDER_NODE_TYPE_ID),
        ] {
            builder
                .engine_builder()
                .register_endpoint_driver(
                    OperatorId::new(operator_id),
                    NodeTypeId::from(node_type_id),
                    Arc::new(TestEndpointFactory),
                )
                .expect("endpoint registration");
        }
    }

    #[test]
    fn given_missing_application_backend_when_host_built_then_error_is_typed() {
        let mut builder =
            SessionEngineHostBuilder::new(prepare_context(), 8, SessionStartOptions::default())
                .expect("host builder");
        builder.set_microphone_backend(Arc::new(TestCaptureBackend::default()));

        let error = match builder.build() {
            Ok(_) => panic!("application backend is required"),
            Err(error) => error,
        };

        assert_eq!(
            error.to_string(),
            SessionEngineHostBuildError::MissingApplicationBackend.to_string()
        );
    }

    #[test]
    fn given_registered_polled_endpoint_when_host_built_then_receipt_is_retained() {
        let mut builder =
            SessionEngineHostBuilder::new(prepare_context(), 8, SessionStartOptions::default())
                .expect("host builder");
        register_default_endpoints(&mut builder);
        builder.set_application_backend(Arc::new(TestCaptureBackend::default()));
        builder.set_microphone_backend(Arc::new(TestCaptureBackend::default()));
        let receipt = builder
            .register_polled_audio_endpoint(PolledAudioEndpointConfig::default())
            .expect("polled endpoint");

        let host = builder.build().expect("host build");

        assert_eq!(host.polled_audio_receipts_total(), 1);
        assert!(host.polled_audio_receipt(0).is_some());
        assert_eq!(
            host.polled_audio_receipt(0)
                .expect("host receipt")
                .observations()
                .registered_endpoints,
            receipt.observations().registered_endpoints
        );
    }

    #[test]
    fn given_host_owned_backends_when_started_then_polled_audio_and_stop_are_real() {
        let mut builder =
            SessionEngineHostBuilder::new(prepare_context(), 8, SessionStartOptions::default())
                .expect("host builder");
        register_default_endpoints(&mut builder);
        let application = Arc::new(TestCaptureBackend::default());
        application.deliver_audio.store(true, Ordering::Release);
        let microphone = Arc::new(TestCaptureBackend::default());
        microphone.deliver_audio.store(true, Ordering::Release);
        builder.set_application_backend(application);
        builder.set_microphone_backend(microphone);
        let receipt = builder
            .register_polled_audio_endpoint(PolledAudioEndpointConfig {
                queue_capacity_frames: 4,
                max_batch_frames: 4,
                max_outstanding_leases: 2,
            })
            .expect("polled endpoint");
        let host = builder.build().expect("host build");

        let mut running = host.start(product_session()).expect("host start");
        let events = running
            .take_event_receiver()
            .expect("Session event receiver");
        let deadline = Instant::now() + Duration::from_secs(2);
        let batch = loop {
            match receipt.try_poll() {
                Ok(batch) => break batch,
                Err(crate::PolledAudioPollError::Empty) if Instant::now() < deadline => {
                    std::thread::sleep(Duration::from_millis(1));
                }
                Err(error) => panic!("polled batch: {error}"),
            }
        };

        assert!(!batch.is_empty());
        let live_metrics = host
            .metrics_snapshot(&events, 0, Some(&running))
            .expect("live Session metrics");
        assert_eq!(live_metrics.source_count(), 2);
        assert_eq!(live_metrics.route_count(), 8);
        assert!(live_metrics.source(0).is_some());
        assert!(live_metrics.source(2).is_none());
        assert!(live_metrics.route(7).is_some());
        assert!(live_metrics.route(8).is_none());
        assert!(running.stop().is_success());
        let final_metrics = host
            .metrics_snapshot(&events, 0, Some(&running))
            .expect("final Session metrics");
        assert_eq!(final_metrics.source_count(), 2);
        assert_eq!(final_metrics.route_count(), 8);
        assert!(final_metrics
            .route(0)
            .is_some_and(|route| route.endpoint_observation_stage
                == crate::EndpointObservationStage::Finalized));
    }

    #[test]
    fn given_host_owned_backend_failure_when_started_then_error_remains_typed() {
        let mut builder =
            SessionEngineHostBuilder::new(prepare_context(), 8, SessionStartOptions::default())
                .expect("host builder");
        register_default_endpoints(&mut builder);
        let application = Arc::new(TestCaptureBackend::default());
        let microphone = Arc::new(TestCaptureBackend::default());
        microphone.fail_open.store(true, Ordering::Release);
        builder.set_application_backend(application);
        builder.set_microphone_backend(microphone);
        builder
            .register_polled_audio_endpoint(PolledAudioEndpointConfig::default())
            .expect("polled endpoint");
        let host = builder.build().expect("host build");

        let error = match host.start(product_session()) {
            Ok(_) => panic!("capture failure must remain typed"),
            Err(error) => error,
        };

        match error {
            SessionEngineStartError::Start(start_failure) => match start_failure.error() {
                crate::SessionStartError::CapturePrepare { .. }
                | crate::SessionStartError::CaptureOpen { .. } => {}
                other => panic!("unexpected start failure: {other}"),
            },
            other => panic!("unexpected engine error: {other}"),
        }
    }
}
