use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

use crate::capture::{CallbackCaptureBackend, CaptureError, CaptureMode, PreparedCaptureBackend};
use crate::endpoint::{
    EndpointCancellationOutcome, EndpointDriverFactory, EndpointDriverFinalization,
    EndpointDriverObservations, EndpointFailure, EndpointFailureStage, EndpointInputOrigin,
    EndpointPortInput, EndpointReceiver, EndpointStartGate, PreparedEndpointDriver,
    RunningEndpointDriver,
};
use crate::frame::{AudioBufferPool, AudioFrame, ClockDomainId, SampleFormat, SampleSpec};
use crate::graph::{
    AudioCaps, BinaryFormat, ChannelLayout, ConfigError, ExecutionPartition, ExecutionSafety,
    MediaCaps, Multiplicity, NodeConfig, NodeDefinition, NodeDescriptor, NodeTypeId, PortDirection,
    PortSpec, PrepareContext, SignalEnvelope, SignalLineage, SignalPayload, SignalSpec,
    SignalTiming,
};
use crate::session::{
    CaptureBackendSet, EndpointConfiguration, EndpointDescriptor, OperatorId, Session,
    SessionEngineBuilder, SessionStartOptions, SourceCancellation, SourceConfiguration,
    SourceDriver, SourceDriverError, SourceEmission, SourceFactory, SourceManifest,
    SourcePrepareContext, SourceSessionContext, SourceTypeId,
};

const SOURCE_TYPE: &str = "org.example.source.lifecycle.v1";
const SIGNAL_ID: &str = "org.example.lifecycle-signal.v1";
const SCHEMA_ID: &str = "urn:example:lifecycle-signal:v1";
const ENDPOINT_NODE: &str = "org.example.lifecycle-endpoint.v1";
const ENDPOINT_OPERATOR: &str = "org.example.lifecycle-endpoint-driver.v1";
const AUDIO_SOURCE_TYPE: &str = "org.example.source.lifecycle-audio.v1";
const AUDIO_ENDPOINT_NODE: &str = "org.example.lifecycle-audio-endpoint.v1";
const AUDIO_ENDPOINT_OPERATOR: &str = "org.example.lifecycle-audio-endpoint-driver.v1";
const FAILING_SOURCE_TYPE: &str = "org.example.source.lifecycle-failing.v1";

#[derive(Default)]
struct SourceControl {
    prepared_total: AtomicU64,
    closed_total: AtomicU64,
    emitted_total: AtomicU64,
}

struct LifecycleSourceFactory {
    manifest: SourceManifest,
    control: Arc<SourceControl>,
    fail_at: Option<u64>,
}

struct LifecycleSourceDriver {
    control: Arc<SourceControl>,
    session: Option<SourceSessionContext>,
    sequence: u64,
    fail_at: Option<u64>,
}

impl SourceDriver for LifecycleSourceDriver {
    fn prepare(&mut self, context: &SourcePrepareContext) -> Result<(), SourceDriverError> {
        self.session = context.session.clone();
        self.control.prepared_total.fetch_add(1, Ordering::Relaxed);
        Ok(())
    }

    fn next(
        &mut self,
        cancellation: &SourceCancellation,
    ) -> Result<Option<SourceEmission>, SourceDriverError> {
        if self.fail_at == Some(self.sequence) {
            return Err(SourceDriverError::Failed(
                "deterministic source failure".to_owned(),
            ));
        }
        if self.sequence >= 64 {
            while !cancellation.is_cancelled() {
                std::thread::sleep(Duration::from_millis(1));
            }
            return Ok(None);
        }
        if self.sequence == 0 {
            std::thread::sleep(Duration::from_millis(10));
        } else {
            std::thread::sleep(Duration::from_millis(1));
        }
        let session = self.session.as_ref().ok_or_else(|| {
            SourceDriverError::Failed("Session source context was not supplied".to_owned())
        })?;
        let output = session.output("signal").ok_or_else(|| {
            SourceDriverError::Failed("signal output identity is absent".to_owned())
        })?;
        let sequence = self.sequence;
        self.sequence = self.sequence.saturating_add(1);
        self.control.emitted_total.fetch_add(1, Ordering::Relaxed);
        let spec = SignalSpec::custom(SIGNAL_ID).with_schema(SCHEMA_ID);
        let envelope = SignalEnvelope::untracked(
            SignalPayload::Bytes(sequence.to_le_bytes().to_vec()),
            spec,
            sequence,
        )
        .with_lineage(
            SignalLineage {
                session_id: session.session_id,
                stream_id: output.stream_id,
                source_id: session.source_id,
                clock_id: ClockDomainId(7),
                sequence_number: sequence,
                source_generation: if sequence < 32 { 1 } else { 2 },
                discontinuity_epoch: if sequence < 32 { 0 } else { 1 },
                policy_epoch: if sequence < 32 { 0 } else { 1 },
            },
            SignalTiming {
                source_timestamp_ns: Some(sequence),
                observed_timestamp_ns: sequence,
                session_timestamp_ns: Some(sequence),
                duration_ns: None,
            },
        );
        Ok(Some(SourceEmission {
            output_port: "signal".to_owned(),
            envelope,
            terminal: false,
        }))
    }

    fn close(&mut self) -> Result<(), SourceDriverError> {
        self.control.closed_total.fetch_add(1, Ordering::Relaxed);
        Ok(())
    }
}

impl SourceFactory for LifecycleSourceFactory {
    fn manifest(&self) -> &SourceManifest {
        &self.manifest
    }

    fn validate_config(&self, _configuration: &SourceConfiguration) -> Result<(), ConfigError> {
        Ok(())
    }

    fn create(
        &self,
        _configuration: &SourceConfiguration,
    ) -> Result<Box<dyn SourceDriver>, SourceDriverError> {
        Ok(Box::new(LifecycleSourceDriver {
            control: Arc::clone(&self.control),
            session: None,
            sequence: 0,
            fail_at: self.fail_at,
        }))
    }
}

struct EndpointDefinition;

impl NodeDefinition for EndpointDefinition {
    fn descriptor(&self) -> NodeDescriptor {
        NodeDescriptor {
            type_id: NodeTypeId::from(ENDPOINT_NODE),
            display_name: "Lifecycle endpoint",
            inputs: vec![PortSpec {
                name: "signal".to_owned(),
                direction: PortDirection::Input,
                signal: SignalSpec::custom(SIGNAL_ID).with_schema(SCHEMA_ID),
                media: MediaCaps::Binary(BinaryFormat::Raw),
                multiplicity: Multiplicity::Many,
                required: true,
            }],
            outputs: Vec::new(),
            execution: ExecutionPartition::External,
            safety: ExecutionSafety::ExternalService,
            stateful: true,
        }
    }

    fn validate_config(&self, _config: &NodeConfig) -> Result<(), ConfigError> {
        Ok(())
    }
}

#[derive(Default)]
struct EndpointControl {
    received_total: AtomicU64,
    started_before_gate_total: AtomicU64,
}

struct LifecycleEndpointFactory {
    fast_control: Arc<EndpointControl>,
    slow_control: Arc<EndpointControl>,
}

struct PreparedLifecycleEndpoint {
    input: EndpointPortInput,
    control: Arc<EndpointControl>,
    consume: bool,
}

struct RunningLifecycleEndpoint {
    control: Arc<EndpointControl>,
    stop: Arc<AtomicBool>,
    worker: Option<std::thread::JoinHandle<()>>,
}

impl EndpointDriverFactory for LifecycleEndpointFactory {
    fn prepare(
        &self,
        mut inputs: Vec<EndpointPortInput>,
    ) -> Result<Box<dyn PreparedEndpointDriver>, EndpointFailure> {
        let input = inputs.pop().ok_or_else(|| {
            EndpointFailure::new(EndpointFailureStage::Prepare, "missing signal input")
        })?;
        assert!(matches!(input.receiver(), EndpointReceiver::Signal(_)));
        assert!(matches!(
            input.context().route_context().origin(),
            EndpointInputOrigin::Source { .. }
        ));
        let consume = input.context().node_configuration().get("consume") == Some("yes");
        let control = if consume {
            Arc::clone(&self.fast_control)
        } else {
            Arc::clone(&self.slow_control)
        };
        Ok(Box::new(PreparedLifecycleEndpoint {
            input,
            control,
            consume,
        }))
    }
}

impl PreparedEndpointDriver for PreparedLifecycleEndpoint {
    fn start(
        self: Box<Self>,
        start_gate: Arc<EndpointStartGate>,
    ) -> Result<Box<dyn RunningEndpointDriver>, EndpointFailure> {
        let Self {
            input,
            control,
            consume,
        } = *self;
        if start_gate.is_open() {
            control
                .started_before_gate_total
                .fetch_add(1, Ordering::Relaxed);
        }
        let (receiver, _) = input.into_parts();
        let EndpointReceiver::Signal(mut receiver) = receiver else {
            return Err(EndpointFailure::new(
                EndpointFailureStage::Start,
                "typed lifecycle endpoint received audio",
            ));
        };
        let stop = Arc::new(AtomicBool::new(false));
        let worker_stop = Arc::clone(&stop);
        let worker_control = Arc::clone(&control);
        let worker = std::thread::spawn(move || {
            while !start_gate.is_open() && !worker_stop.load(Ordering::Acquire) {
                std::thread::sleep(Duration::from_millis(1));
            }
            while !worker_stop.load(Ordering::Acquire) {
                if consume {
                    if receiver.recv().is_some() {
                        worker_control
                            .received_total
                            .fetch_add(1, Ordering::Relaxed);
                        continue;
                    }
                    if receiver.is_abandoned() {
                        break;
                    }
                }
                std::thread::sleep(Duration::from_millis(1));
            }
        });
        Ok(Box::new(RunningLifecycleEndpoint {
            control,
            stop,
            worker: Some(worker),
        }))
    }

    fn cancel_preparation(self: Box<Self>) -> EndpointCancellationOutcome {
        EndpointCancellationOutcome {
            observations: EndpointDriverObservations::default(),
            result: Ok(()),
        }
    }
}

impl RunningEndpointDriver for RunningLifecycleEndpoint {
    fn observations(&self) -> EndpointDriverObservations {
        EndpointDriverObservations {
            frames_received_total: self.control.received_total.load(Ordering::Relaxed),
            frames_delivered_total: self.control.received_total.load(Ordering::Relaxed),
            ..EndpointDriverObservations::default()
        }
    }

    fn request_stop(&mut self) -> Result<(), EndpointFailure> {
        self.stop.store(true, Ordering::Release);
        Ok(())
    }

    fn join_and_finalize(mut self: Box<Self>) -> EndpointDriverFinalization {
        self.stop.store(true, Ordering::Release);
        let result = self
            .worker
            .take()
            .expect("lifecycle endpoint worker")
            .join()
            .map_err(|_| {
                EndpointFailure::new(EndpointFailureStage::JoinFinalize, "worker panicked")
            });
        EndpointDriverFinalization {
            observations: self.observations(),
            result,
        }
    }
}

struct AudioSourceFactory {
    manifest: SourceManifest,
    control: Arc<SourceControl>,
}

struct AudioSourceDriver {
    control: Arc<SourceControl>,
    session: Option<SourceSessionContext>,
    sequence: u64,
    pool: Arc<AudioBufferPool>,
}

impl SourceDriver for AudioSourceDriver {
    fn prepare(&mut self, context: &SourcePrepareContext) -> Result<(), SourceDriverError> {
        self.session = context.session.clone();
        self.control.prepared_total.fetch_add(1, Ordering::Relaxed);
        Ok(())
    }

    fn next(
        &mut self,
        cancellation: &SourceCancellation,
    ) -> Result<Option<SourceEmission>, SourceDriverError> {
        if self.sequence >= 4 {
            while !cancellation.is_cancelled() {
                std::thread::sleep(Duration::from_millis(1));
            }
            return Ok(None);
        }
        let session = self.session.as_ref().ok_or_else(|| {
            SourceDriverError::Failed("Session audio context was not supplied".to_owned())
        })?;
        let output = session.output("audio").ok_or_else(|| {
            SourceDriverError::Failed("audio output identity is absent".to_owned())
        })?;
        let Some(mut buffer) = self.pool.acquire() else {
            std::thread::sleep(Duration::from_millis(1));
            return Ok(None);
        };
        buffer.as_mut_slice().fill(0.25);
        let sequence = self.sequence;
        self.sequence = self.sequence.saturating_add(1);
        let mut frame = AudioFrame::new(
            output.stream_id,
            session.source_id,
            sequence,
            sequence.saturating_mul(20_000_000),
            1,
            buffer,
        );
        frame.sample_rate_hz = 48_000;
        let envelope = SignalEnvelope::from_audio(frame, None).with_lineage(
            SignalLineage {
                session_id: session.session_id,
                stream_id: output.stream_id,
                source_id: session.source_id,
                clock_id: ClockDomainId(9),
                sequence_number: sequence,
                source_generation: 2,
                discontinuity_epoch: 3,
                policy_epoch: 4,
            },
            SignalTiming {
                source_timestamp_ns: Some(sequence.saturating_mul(20_000_000)),
                observed_timestamp_ns: sequence.saturating_mul(20_000_000),
                session_timestamp_ns: Some(sequence.saturating_mul(20_000_000)),
                duration_ns: Some(20_000_000),
            },
        );
        Ok(Some(SourceEmission {
            output_port: "audio".to_owned(),
            envelope,
            terminal: false,
        }))
    }

    fn close(&mut self) -> Result<(), SourceDriverError> {
        self.control.closed_total.fetch_add(1, Ordering::Relaxed);
        Ok(())
    }
}

impl SourceFactory for AudioSourceFactory {
    fn manifest(&self) -> &SourceManifest {
        &self.manifest
    }

    fn validate_config(&self, _configuration: &SourceConfiguration) -> Result<(), ConfigError> {
        Ok(())
    }

    fn create(
        &self,
        _configuration: &SourceConfiguration,
    ) -> Result<Box<dyn SourceDriver>, SourceDriverError> {
        Ok(Box::new(AudioSourceDriver {
            control: Arc::clone(&self.control),
            session: None,
            sequence: 0,
            pool: AudioBufferPool::new(8, 960),
        }))
    }
}

struct AudioEndpointDefinition;

impl NodeDefinition for AudioEndpointDefinition {
    fn descriptor(&self) -> NodeDescriptor {
        NodeDescriptor {
            type_id: NodeTypeId::from(AUDIO_ENDPOINT_NODE),
            display_name: "Lifecycle audio endpoint",
            inputs: vec![PortSpec {
                name: "audio".to_owned(),
                direction: PortDirection::Input,
                signal: SignalSpec::audio(),
                media: MediaCaps::Audio(AudioCaps {
                    sample_rate_hz: Some(48_000),
                    frame_samples: Some(960),
                    channel_layout: ChannelLayout::Mono,
                    format: SampleFormat::F32Interleaved,
                }),
                multiplicity: Multiplicity::Many,
                required: true,
            }],
            outputs: Vec::new(),
            execution: ExecutionPartition::BlockingWorker,
            safety: ExecutionSafety::AllocationAllowed,
            stateful: true,
        }
    }

    fn validate_config(&self, _config: &NodeConfig) -> Result<(), ConfigError> {
        Ok(())
    }
}

struct AudioEndpointFactory {
    control: Arc<EndpointControl>,
    expected_source_id: Arc<AtomicU64>,
}

struct PreparedAudioEndpoint {
    input: EndpointPortInput,
    control: Arc<EndpointControl>,
    expected_source_id: Arc<AtomicU64>,
}

impl EndpointDriverFactory for AudioEndpointFactory {
    fn prepare(
        &self,
        mut inputs: Vec<EndpointPortInput>,
    ) -> Result<Box<dyn PreparedEndpointDriver>, EndpointFailure> {
        let input = inputs.pop().ok_or_else(|| {
            EndpointFailure::new(EndpointFailureStage::Prepare, "missing audio input")
        })?;
        assert!(matches!(input.receiver(), EndpointReceiver::Audio { .. }));
        let EndpointInputOrigin::Source { source_id, .. } =
            input.context().route_context().origin()
        else {
            return Err(EndpointFailure::new(
                EndpointFailureStage::Prepare,
                "external audio source identity is absent",
            ));
        };
        self.expected_source_id
            .store(source_id.0, Ordering::Relaxed);
        Ok(Box::new(PreparedAudioEndpoint {
            input,
            control: Arc::clone(&self.control),
            expected_source_id: Arc::clone(&self.expected_source_id),
        }))
    }
}

impl PreparedEndpointDriver for PreparedAudioEndpoint {
    fn start(
        self: Box<Self>,
        start_gate: Arc<EndpointStartGate>,
    ) -> Result<Box<dyn RunningEndpointDriver>, EndpointFailure> {
        let Self {
            input,
            control,
            expected_source_id,
        } = *self;
        if start_gate.is_open() {
            control
                .started_before_gate_total
                .fetch_add(1, Ordering::Relaxed);
        }
        let (receiver, _) = input.into_parts();
        let EndpointReceiver::Audio { mut receiver, .. } = receiver else {
            return Err(EndpointFailure::new(
                EndpointFailureStage::Start,
                "audio endpoint received a signal input",
            ));
        };
        let stop = Arc::new(AtomicBool::new(false));
        let worker_stop = Arc::clone(&stop);
        let worker_control = Arc::clone(&control);
        let worker = std::thread::spawn(move || {
            while !start_gate.is_open() && !worker_stop.load(Ordering::Acquire) {
                std::thread::sleep(Duration::from_millis(1));
            }
            while !worker_stop.load(Ordering::Acquire) {
                if let Some(frame) = receiver.try_recv() {
                    assert_eq!(
                        frame.source_id().0,
                        expected_source_id.load(Ordering::Relaxed)
                    );
                    assert_eq!(frame.lineage().clock_id, ClockDomainId(9));
                    worker_control
                        .received_total
                        .fetch_add(1, Ordering::Relaxed);
                    continue;
                }
                if receiver.is_abandoned() {
                    break;
                }
                std::thread::sleep(Duration::from_millis(1));
            }
        });
        Ok(Box::new(RunningLifecycleEndpoint {
            control,
            stop,
            worker: Some(worker),
        }))
    }

    fn cancel_preparation(self: Box<Self>) -> EndpointCancellationOutcome {
        EndpointCancellationOutcome {
            observations: EndpointDriverObservations::default(),
            result: Ok(()),
        }
    }
}

struct ForbiddenCaptureBackend;

impl CallbackCaptureBackend for ForbiddenCaptureBackend {
    fn prepare(&self, _mode: CaptureMode) -> Result<Box<dyn PreparedCaptureBackend>, CaptureError> {
        panic!("typed-only external Session must not prepare a desktop capture backend")
    }
}

fn source_factory(control: Arc<SourceControl>) -> Arc<dyn SourceFactory> {
    Arc::new(LifecycleSourceFactory {
        manifest: SourceManifest {
            source_type_id: SourceTypeId::new(SOURCE_TYPE).unwrap(),
            revision: 1,
            implementation_generation: 1,
            outputs: vec![PortSpec {
                name: "signal".to_owned(),
                direction: PortDirection::Output,
                signal: SignalSpec::custom(SIGNAL_ID).with_schema(SCHEMA_ID),
                media: MediaCaps::Binary(BinaryFormat::Raw),
                multiplicity: Multiplicity::Many,
                required: true,
            }],
            execution: ExecutionPartition::BlockingWorker,
            safety: ExecutionSafety::AllocationAllowed,
        },
        control,
        fail_at: None,
    })
}

fn failing_source_factory(control: Arc<SourceControl>) -> Arc<dyn SourceFactory> {
    Arc::new(LifecycleSourceFactory {
        manifest: SourceManifest {
            source_type_id: SourceTypeId::new(FAILING_SOURCE_TYPE).unwrap(),
            revision: 1,
            implementation_generation: 1,
            outputs: vec![PortSpec {
                name: "signal".to_owned(),
                direction: PortDirection::Output,
                signal: SignalSpec::custom(SIGNAL_ID).with_schema(SCHEMA_ID),
                media: MediaCaps::Binary(BinaryFormat::Raw),
                multiplicity: Multiplicity::Many,
                required: true,
            }],
            execution: ExecutionPartition::BlockingWorker,
            safety: ExecutionSafety::AllocationAllowed,
        },
        control,
        fail_at: Some(4),
    })
}

fn audio_source_factory(control: Arc<SourceControl>) -> Arc<dyn SourceFactory> {
    Arc::new(AudioSourceFactory {
        manifest: SourceManifest {
            source_type_id: SourceTypeId::new(AUDIO_SOURCE_TYPE).unwrap(),
            revision: 1,
            implementation_generation: 1,
            outputs: vec![PortSpec {
                name: "audio".to_owned(),
                direction: PortDirection::Output,
                signal: SignalSpec::audio(),
                media: MediaCaps::Audio(AudioCaps {
                    sample_rate_hz: Some(48_000),
                    frame_samples: Some(960),
                    channel_layout: ChannelLayout::Mono,
                    format: SampleFormat::F32Interleaved,
                }),
                multiplicity: Multiplicity::Many,
                required: true,
            }],
            execution: ExecutionPartition::BlockingWorker,
            safety: ExecutionSafety::AllocationAllowed,
        },
        control,
    })
}

fn endpoint(session: &Session, consume: bool) -> crate::session::EndpointHandle {
    session
        .endpoint(
            EndpointDescriptor::new(
                NodeTypeId::from(ENDPOINT_NODE),
                OperatorId::new(ENDPOINT_OPERATOR),
            )
            .with_configuration(
                EndpointConfiguration::new().with("consume", if consume { "yes" } else { "no" }),
            ),
        )
        .unwrap()
}

#[test]
fn given_typed_source_when_one_branch_saturates_then_other_branch_and_shutdown_remain_real() {
    let source_control = Arc::new(SourceControl::default());
    let fast_endpoint = Arc::new(EndpointControl::default());
    let slow_endpoint = Arc::new(EndpointControl::default());
    let mut builder = SessionEngineBuilder::new(
        PrepareContext::new(SampleSpec::new(48_000, 1, SampleFormat::F32Interleaved)),
        8,
        SessionStartOptions::default(),
    )
    .unwrap();
    builder
        .register_source_factory(source_factory(Arc::clone(&source_control)))
        .unwrap();
    builder
        .register_endpoint(
            OperatorId::new(ENDPOINT_OPERATOR),
            Arc::new(EndpointDefinition),
            Arc::new(LifecycleEndpointFactory {
                fast_control: Arc::clone(&fast_endpoint),
                slow_control: Arc::clone(&slow_endpoint),
            }),
        )
        .unwrap();
    let engine = builder.build().unwrap();

    let session = Session::new();
    let source = session
        .source(
            SourceTypeId::new(SOURCE_TYPE).unwrap(),
            SourceConfiguration::default(),
        )
        .unwrap();
    let output = source.output("signal").unwrap();
    output.send(endpoint(&session, true)).unwrap();
    // The same registered factory owns both route instances; the second route
    // deliberately never consumes and therefore exercises bounded saturation.
    // Its per-route configuration is retained in the prepare context.
    output.send(endpoint(&session, false)).unwrap();

    let forbidden_capture = ForbiddenCaptureBackend;
    let mut running = engine
        .start(
            session,
            CaptureBackendSet {
                application: &forbidden_capture,
                microphone: &forbidden_capture,
            },
        )
        .unwrap();

    let deadline = Instant::now() + Duration::from_secs(2);
    while (fast_endpoint.received_total.load(Ordering::Relaxed) < 16
        || source_control.emitted_total.load(Ordering::Relaxed) < 64)
        && Instant::now() < deadline
    {
        std::thread::sleep(Duration::from_millis(2));
    }
    let (_, external_sources, _, _, typed_routes) = running.indexed_metrics_full();
    assert_eq!(external_sources.len(), 1);
    assert!(external_sources[0].runtime.emitted_total >= 16);
    assert!(external_sources[0].runtime.dropped_total > 0);
    assert_eq!(external_sources[0].runtime.discontinuity_total, 1);
    assert_eq!(external_sources[0].runtime.recovery_total, 1);
    assert_eq!(external_sources[0].runtime.policy_change_total, 1);
    assert!(fast_endpoint.received_total.load(Ordering::Relaxed) >= 16);
    assert_eq!(slow_endpoint.received_total.load(Ordering::Relaxed), 0);
    assert_eq!(typed_routes.len(), 2);
    assert_eq!(source_control.prepared_total.load(Ordering::Relaxed), 1);

    let outcome = running.stop();
    assert!(outcome.is_success());
    let (_, external_sources, _, _, _) = running.indexed_metrics_full();
    assert!(external_sources[0].runtime.joined);
    assert_eq!(external_sources[0].runtime.cancellation_total, 1);
    assert_eq!(source_control.closed_total.load(Ordering::Relaxed), 1);
    assert_eq!(
        fast_endpoint
            .started_before_gate_total
            .load(Ordering::Relaxed),
        0
    );
    assert_eq!(
        slow_endpoint
            .started_before_gate_total
            .load(Ordering::Relaxed),
        0
    );
}

#[test]
fn given_one_external_source_failure_when_session_runs_then_unrelated_source_completes() {
    let healthy_control = Arc::new(SourceControl::default());
    let failing_control = Arc::new(SourceControl::default());
    let consumed = Arc::new(EndpointControl::default());
    let unused = Arc::new(EndpointControl::default());
    let mut builder = SessionEngineBuilder::new(
        PrepareContext::new(SampleSpec::new(48_000, 1, SampleFormat::F32Interleaved)),
        8,
        SessionStartOptions::default(),
    )
    .unwrap();
    builder
        .register_source_factory(source_factory(Arc::clone(&healthy_control)))
        .unwrap();
    builder
        .register_source_factory(failing_source_factory(Arc::clone(&failing_control)))
        .unwrap();
    builder
        .register_endpoint(
            OperatorId::new(ENDPOINT_OPERATOR),
            Arc::new(EndpointDefinition),
            Arc::new(LifecycleEndpointFactory {
                fast_control: Arc::clone(&consumed),
                slow_control: unused,
            }),
        )
        .unwrap();
    let engine = builder.build().unwrap();

    let session = Session::new();
    let healthy = session
        .source(
            SourceTypeId::new(SOURCE_TYPE).unwrap(),
            SourceConfiguration::default(),
        )
        .unwrap();
    healthy
        .output("signal")
        .unwrap()
        .send(endpoint(&session, true))
        .unwrap();
    let failing = session
        .source(
            SourceTypeId::new(FAILING_SOURCE_TYPE).unwrap(),
            SourceConfiguration::default(),
        )
        .unwrap();
    failing
        .output("signal")
        .unwrap()
        .send(endpoint(&session, true))
        .unwrap();

    let forbidden_capture = ForbiddenCaptureBackend;
    let mut running = engine
        .start(
            session,
            CaptureBackendSet {
                application: &forbidden_capture,
                microphone: &forbidden_capture,
            },
        )
        .unwrap();
    let deadline = Instant::now() + Duration::from_secs(2);
    while healthy_control.emitted_total.load(Ordering::Relaxed) < 64 && Instant::now() < deadline {
        std::thread::sleep(Duration::from_millis(2));
    }
    assert_eq!(healthy_control.emitted_total.load(Ordering::Relaxed), 64);
    assert_eq!(failing_control.emitted_total.load(Ordering::Relaxed), 4);
    let (_, external_sources, _, _, _) = running.indexed_metrics_full();
    let failing_metrics = external_sources
        .iter()
        .find(|metrics| metrics.source_id == failing.source_id())
        .expect("failing source metrics");
    assert_eq!(failing_metrics.runtime.failure_total, 1);
    let healthy_metrics = external_sources
        .iter()
        .find(|metrics| metrics.source_id == healthy.source_id())
        .expect("healthy source metrics");
    assert_eq!(healthy_metrics.runtime.failure_total, 0);
    assert_eq!(healthy_metrics.runtime.emitted_total, 64);

    let outcome = running.stop();
    assert!(!outcome.is_success());
    assert_eq!(healthy_control.closed_total.load(Ordering::Relaxed), 1);
    assert_eq!(failing_control.closed_total.load(Ordering::Relaxed), 1);
}

#[test]
fn given_external_pcm_source_when_session_runs_then_audio_uses_bounded_ingress_with_source_identity(
) {
    let source_control = Arc::new(SourceControl::default());
    let endpoint_control = Arc::new(EndpointControl::default());
    let observed_source_id = Arc::new(AtomicU64::new(0));
    let mut builder = SessionEngineBuilder::new(
        PrepareContext::new(SampleSpec::new(48_000, 1, SampleFormat::F32Interleaved)),
        8,
        SessionStartOptions::default(),
    )
    .unwrap();
    builder
        .register_source_factory(audio_source_factory(Arc::clone(&source_control)))
        .unwrap();
    builder
        .register_endpoint(
            OperatorId::new(AUDIO_ENDPOINT_OPERATOR),
            Arc::new(AudioEndpointDefinition),
            Arc::new(AudioEndpointFactory {
                control: Arc::clone(&endpoint_control),
                expected_source_id: Arc::clone(&observed_source_id),
            }),
        )
        .unwrap();
    let engine = builder.build().unwrap();

    let session = Session::new();
    let source = session
        .source(
            SourceTypeId::new(AUDIO_SOURCE_TYPE).unwrap(),
            SourceConfiguration::default(),
        )
        .unwrap();
    source
        .output("audio")
        .unwrap()
        .send(
            session
                .endpoint(EndpointDescriptor::new(
                    NodeTypeId::from(AUDIO_ENDPOINT_NODE),
                    OperatorId::new(AUDIO_ENDPOINT_OPERATOR),
                ))
                .unwrap(),
        )
        .unwrap();
    let expected_source_id = source.source_id();
    let forbidden_capture = ForbiddenCaptureBackend;
    let mut running = engine
        .start(
            session,
            CaptureBackendSet {
                application: &forbidden_capture,
                microphone: &forbidden_capture,
            },
        )
        .unwrap();

    let deadline = Instant::now() + Duration::from_secs(2);
    while endpoint_control.received_total.load(Ordering::Relaxed) < 4 && Instant::now() < deadline {
        std::thread::sleep(Duration::from_millis(2));
    }
    assert_eq!(
        observed_source_id.load(Ordering::Relaxed),
        expected_source_id.0
    );
    assert_eq!(endpoint_control.received_total.load(Ordering::Relaxed), 4);
    let (built_in_sources, external_sources, raw_routes, _, _) = running.indexed_metrics_full();
    assert!(built_in_sources.is_empty());
    assert_eq!(external_sources.len(), 1);
    assert_eq!(raw_routes.len(), 1);
    assert_eq!(raw_routes[0].edge.frames_delivered_total, 4);

    assert!(running.stop().is_success());
    assert_eq!(source_control.closed_total.load(Ordering::Relaxed), 1);
    assert_eq!(
        endpoint_control
            .started_before_gate_total
            .load(Ordering::Relaxed),
        0
    );
}
