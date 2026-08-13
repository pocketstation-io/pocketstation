use std::sync::Arc;

use crate::capture::CallbackCaptureBackend;
use crate::endpoint::{EndpointDriverFactory, EndpointDriverRegistryError};
use crate::frame::{SampleFormat, SampleSpec};
use crate::graph::{
    AsyncOperatorFactory, NodeDefinition, NodeRegistrationError, NodeTypeId, PrepareContext,
};
use crate::runtime::SidecarProcessSpec;

#[cfg(target_os = "linux")]
use crate::capture::platform::linux::DesktopCaptureBackend as NativeDesktopCaptureBackend;
#[cfg(target_os = "macos")]
use crate::capture::platform::macos::DesktopCaptureBackend as NativeDesktopCaptureBackend;
#[cfg(target_os = "windows")]
use crate::capture::platform::windows::DesktopCaptureBackend as NativeDesktopCaptureBackend;

use crate::session::{
    CaptureBackendSet, CompiledSession, OperatorId, PolledAudioEndpoint, PolledAudioEndpointConfig,
    PolledAudioEndpointConfigError, PolledAudioReceipt, RunningSession, Session, SessionEngine,
    SessionEngineBuildError, SessionEngineBuilder, SessionEngineStartError, SessionEventReceiver,
    SessionMetricsSnapshot, SessionRecordingReceipt, SessionStartCancellation, SessionStartOptions,
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
    recording_receipts: Box<[SessionRecordingReceipt]>,
}

impl SessionEngineHost {
    pub fn native(
        options: NativeSessionEngineHostOptions,
    ) -> Result<Self, SessionEngineHostBuildError> {
        build_native_host(options, None)
    }

    /// Builds the native Session host with one canonical multistem recorder.
    #[cfg(any(test, feature = "internal-testing"))]
    pub fn native_with_multistem_recording(
        options: NativeSessionEngineHostOptions,
        output_root: impl Into<std::path::PathBuf>,
    ) -> Result<Self, SessionEngineHostBuildError> {
        build_native_host(options, Some(output_root.into()))
    }

    pub fn compile(&self, session: Session) -> Result<CompiledSession, SessionEngineStartError> {
        self.engine.compile(session)
    }

    #[cfg(any(test, feature = "internal-testing"))]
    pub fn start(&self, session: Session) -> Result<RunningSession, SessionEngineStartError> {
        self.engine.start(
            session,
            CaptureBackendSet {
                application: self.application_backend.as_ref(),
                microphone: self.microphone_backend.as_ref(),
            },
        )
    }

    #[cfg(any(test, feature = "internal-testing"))]
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

    #[cfg(any(test, feature = "internal-testing"))]
    pub fn polled_audio_receipts_total(&self) -> usize {
        self.polled_audio_receipts.len()
    }

    pub fn recording_receipt(&self, index: usize) -> Option<SessionRecordingReceipt> {
        self.recording_receipts.get(index).cloned()
    }

    #[cfg(any(test, feature = "internal-testing"))]
    pub fn recording_receipts_total(&self) -> usize {
        self.recording_receipts.len()
    }

    pub fn metrics_snapshot(
        &self,
        events: &SessionEventReceiver,
        polled_audio_receipt_index: usize,
        running_session: Option<&RunningSession>,
    ) -> Option<SessionMetricsSnapshot> {
        let polled_audio = match self.polled_audio_receipts.get(polled_audio_receipt_index) {
            Some(receipt) => receipt.observations(),
            None if polled_audio_receipt_index == 0 && self.polled_audio_receipts.is_empty() => {
                // A typed-only Session has no audio retention endpoint. Its
                // source/operator/route observations remain authoritative and
                // the audio portion of the common snapshot is explicitly zero.
                crate::session::PolledAudioObservations::default()
            }
            None => return None,
        };
        let (sources, external_sources, routes, operators, derived_routes) = running_session
            .map_or_else(
                || {
                    (
                        Box::default(),
                        Box::default(),
                        Box::default(),
                        Box::default(),
                        Box::default(),
                    )
                },
                RunningSession::indexed_metrics_full,
            );
        Some(SessionMetricsSnapshot::new(
            events.observations(),
            polled_audio,
            sources,
            external_sources,
            routes,
            operators,
            derived_routes,
        ))
    }

    #[cfg(any(test, feature = "internal-testing"))]
    pub fn engine(&self) -> &SessionEngine {
        &self.engine
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NativeSessionEngineHostOptions {
    pub sample_spec: SampleSpec,
    pub source_queue_capacity_frames: usize,
    pub start_options: SessionStartOptions,
    pub polled_audio_endpoint: PolledAudioEndpointConfig,
}

impl Default for NativeSessionEngineHostOptions {
    fn default() -> Self {
        Self {
            sample_spec: SampleSpec::new(48_000, 1, SampleFormat::F32Interleaved),
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
    recording_receipts: Vec<SessionRecordingReceipt>,
}

impl SessionEngineHostBuilder {
    /// Creates the standard Session host builder with caller-owned capture
    /// backends. This is the reuse seam for CLIs, tests, and platform adapters
    /// that decorate native capture without rebuilding Session semantics.
    pub fn with_capture_backends(
        options: NativeSessionEngineHostOptions,
        application_backend: Arc<dyn CallbackCaptureBackend>,
        microphone_backend: Arc<dyn CallbackCaptureBackend>,
    ) -> Result<Self, SessionEngineHostBuildError> {
        let prepare_context = PrepareContext::new(options.sample_spec);
        let mut builder = Self::new(
            prepare_context,
            options.source_queue_capacity_frames,
            options.start_options,
        )?;
        builder
            .set_application_backend(application_backend)
            .set_microphone_backend(microphone_backend);
        Ok(builder)
    }

    /// Creates the production host builder with the platform's native capture
    /// backend, leaving endpoint registration open to the owning application.
    #[cfg(any(target_os = "linux", target_os = "macos", target_os = "windows"))]
    pub fn native(
        options: NativeSessionEngineHostOptions,
    ) -> Result<Self, SessionEngineHostBuildError> {
        let capture_backend: Arc<dyn CallbackCaptureBackend> =
            Arc::new(NativeDesktopCaptureBackend);
        Self::with_capture_backends(options, Arc::clone(&capture_backend), capture_backend)
    }

    /// Returns a typed unsupported-platform error on targets without a native
    /// PocketStation capture backend.
    #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
    pub fn native(
        _options: NativeSessionEngineHostOptions,
    ) -> Result<Self, SessionEngineHostBuildError> {
        Err(SessionEngineHostBuildError::UnsupportedPlatform)
    }

    pub fn new(
        prepare_context: crate::graph::PrepareContext,
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
            recording_receipts: Vec::new(),
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

    /// Registers one externally owned endpoint implementation with the
    /// canonical Session engine.
    pub fn register_audio_endpoint_driver(
        &mut self,
        operator_id: OperatorId,
        node_type_id: NodeTypeId,
        factory: Arc<dyn EndpointDriverFactory>,
    ) -> Result<&mut Self, SessionEngineHostBuildError> {
        self.engine_builder
            .register_audio_endpoint_driver(operator_id, node_type_id, factory)?;
        Ok(self)
    }

    pub fn register_endpoint(
        &mut self,
        operator_id: OperatorId,
        definition: Arc<dyn NodeDefinition>,
        factory: Arc<dyn EndpointDriverFactory>,
    ) -> Result<&mut Self, SessionEngineHostBuildError> {
        self.engine_builder
            .register_endpoint(operator_id, definition, factory)?;
        Ok(self)
    }

    pub fn register_async_operator(
        &mut self,
        factory: Arc<dyn AsyncOperatorFactory>,
    ) -> Result<&mut Self, SessionEngineHostBuildError> {
        self.engine_builder.register_async_operator(factory)?;
        Ok(self)
    }

    pub fn register_sidecar_process(
        &mut self,
        spec: SidecarProcessSpec,
    ) -> Result<&mut Self, SessionEngineHostBuildError> {
        self.engine_builder.register_sidecar_process(spec)?;
        Ok(self)
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

    pub fn register_multistem_recording(
        &mut self,
        output_root: impl Into<std::path::PathBuf>,
    ) -> Result<SessionRecordingReceipt, SessionEngineHostBuildError> {
        let receipt = self
            .engine_builder
            .register_multistem_recording(output_root)?;
        self.recording_receipts.push(receipt.clone());
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
            recording_receipts: self.recording_receipts.into_boxed_slice(),
        })
    }
}

#[derive(Debug, thiserror::Error)]
pub enum SessionEngineHostBuildError {
    #[error(transparent)]
    Engine(#[from] SessionEngineBuildError),
    #[error(transparent)]
    EndpointRegistration(#[from] EndpointDriverRegistryError),
    #[error(transparent)]
    EndpointExtensionRegistration(#[from] crate::session::EndpointExtensionRegistrationError),
    #[error(transparent)]
    OperatorRegistration(#[from] NodeRegistrationError),
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
    recording_root: Option<std::path::PathBuf>,
) -> Result<SessionEngineHost, SessionEngineHostBuildError> {
    let mut builder = SessionEngineHostBuilder::native(options)?;
    let _ = builder.register_polled_audio_endpoint(options.polled_audio_endpoint)?;
    if let Some(recording_root) = recording_root {
        let _ = builder.register_multistem_recording(recording_root)?;
    }
    builder.build()
}

#[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
fn build_native_host(
    _options: NativeSessionEngineHostOptions,
    _recording_root: Option<std::path::PathBuf>,
) -> Result<SessionEngineHost, SessionEngineHostBuildError> {
    Err(SessionEngineHostBuildError::UnsupportedPlatform)
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::Arc;
    use std::time::{Duration, Instant};

    use crate::capture::{
        ActiveCaptureBackend, CallbackCaptureBackend, CaptureDelivery, CaptureError, CaptureMode,
        CaptureObservationHandle, CaptureObservations, CapturedFrameDelivery,
        PreparedCaptureBackend,
    };
    use crate::endpoint::{
        EndpointCancellationOutcome, EndpointDriverFactory, EndpointDriverFinalization,
        EndpointDriverObservations, EndpointFailure, EndpointPortInput, EndpointStartGate,
        PreparedEndpointDriver, RunningEndpointDriver,
    };
    use crate::frame::{AudioBufferPool, AudioFrame, SampleFormat, SampleSpec, SourceId, StreamId};
    use crate::graph::{NodeTypeId, PrepareContext};

    use crate::session::{
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

    struct TestActiveCapture {
        stop_requested: Arc<AtomicBool>,
        worker: Option<std::thread::JoinHandle<()>>,
    }

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
            delivery: CaptureDelivery,
        ) -> Result<Box<dyn ActiveCaptureBackend>, CaptureError> {
            if self.fail_open {
                return Err(CaptureError::BackendInit(
                    "test capture open failure".to_owned(),
                ));
            }
            let stop_requested = Arc::new(AtomicBool::new(false));
            let worker_stop_requested = Arc::clone(&stop_requested);
            let worker = self.deliver_audio.then(|| {
                std::thread::spawn(move || {
                    let pool = AudioBufferPool::new(1, 4);
                    let mut frame_sender = delivery.frame_sender;
                    while !worker_stop_requested.load(Ordering::Acquire) {
                        let Some(mut buffer) = pool.acquire() else {
                            std::thread::sleep(Duration::from_millis(1));
                            continue;
                        };
                        buffer
                            .try_copy_from_slice(&[0.125, 0.25, 0.5, 1.0])
                            .expect("test samples fit the fixed-capacity buffer");
                        let frame = AudioFrame::new(StreamId(11), SourceId(12), 13, 14, 1, buffer);
                        match frame_sender.try_send(frame) {
                            CapturedFrameDelivery::Delivered => break,
                            CapturedFrameDelivery::DroppedNewest
                            | CapturedFrameDelivery::DiscardedBeforeStart => {
                                std::thread::sleep(Duration::from_millis(1));
                            }
                        }
                    }
                })
            });
            Ok(Box::new(TestActiveCapture {
                stop_requested,
                worker,
            }))
        }
    }

    impl ActiveCaptureBackend for TestActiveCapture {
        fn source_id(&self) -> SourceId {
            SourceId(12)
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
                        worker: "host test capture worker",
                    })?;
            }
            Ok(CaptureObservations::default())
        }
    }

    impl Drop for TestActiveCapture {
        fn drop(&mut self) {
            self.stop_requested.store(true, Ordering::Release);
            if let Some(worker) = self.worker.take() {
                let _ = worker.join();
            }
        }
    }

    struct TestEndpointFactory;

    struct TestPreparedEndpoint;

    struct TestRunningEndpoint;

    impl EndpointDriverFactory for TestEndpointFactory {
        fn prepare(
            &self,
            inputs: Vec<EndpointPortInput>,
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
                .register_audio_endpoint_driver(
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
    fn given_registered_multistem_recorder_when_host_built_then_receipt_is_retained() {
        let mut builder =
            SessionEngineHostBuilder::new(prepare_context(), 8, SessionStartOptions::default())
                .expect("host builder");
        builder.set_application_backend(Arc::new(TestCaptureBackend::default()));
        builder.set_microphone_backend(Arc::new(TestCaptureBackend::default()));
        let output_root = std::env::temp_dir().join(format!(
            "pocketstation-recording-registration-{}",
            std::process::id()
        ));
        let receipt = builder
            .register_multistem_recording(output_root)
            .expect("recording endpoint");

        let host = builder.build().expect("host build");

        assert_eq!(host.recording_receipts_total(), 1);
        assert!(host.recording_receipt(0).is_some());
        assert!(receipt.outcome().is_none());
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
                Err(crate::session::PolledAudioPollError::Empty) if Instant::now() < deadline => {
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
                == crate::session::EndpointObservationStage::Finalized));
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
                crate::session::SessionStartError::CapturePrepare { .. }
                | crate::session::SessionStartError::CaptureOpen { .. } => {}
                other => panic!("unexpected start failure: {other}"),
            },
            other => panic!("unexpected engine error: {other}"),
        }
    }
}
