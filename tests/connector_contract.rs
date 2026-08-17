#[cfg(feature = "conformance-fixtures")]
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;
#[cfg(feature = "conformance-fixtures")]
use std::time::{Duration, Instant};

use pocketstation::connector::{
    Connector, ConnectorConfiguration, ConnectorConfigurationConstraint,
    ConnectorConfigurationErrorCode, ConnectorConfigurationField,
    ConnectorConfigurationRequirement, ConnectorConfigurationSchema, ConnectorConfigurationValue,
    ConnectorConfigurationValueKind, ConnectorDeliveryPolicy, ConnectorError, ConnectorErrorCode,
    ConnectorErrorStage, ConnectorManifest, ConnectorObservationHandle, ConnectorReadiness,
    ConnectorReadinessPolicy, ConnectorRetryPolicy, ConnectorRetryability, ConnectorSecret,
    MAX_CONNECTOR_ERROR_MESSAGE_BYTES,
};
use pocketstation::{
    AudioCaps, ChannelLayout, EdgeContract, EndpointDriverFactory, EndpointFailure,
    EndpointFailureStage, EndpointPortInput, ExecutionPartition, MediaCaps, Multiplicity,
    NodeDescriptor, NodeTypeId, OperatorId, PortDirection, PortSpec, PreparedEndpointDriver,
    SafetyContract, SampleFormat, SignalSpec,
};

struct RejectingFactory;

impl EndpointDriverFactory for RejectingFactory {
    fn prepare(
        &self,
        _inputs: Vec<EndpointPortInput>,
    ) -> Result<Box<dyn PreparedEndpointDriver>, EndpointFailure> {
        Err(EndpointFailure::new(
            EndpointFailureStage::Prepare,
            "test factory is declaration-only",
        ))
    }
}

fn configuration_schema() -> ConnectorConfigurationSchema {
    ConnectorConfigurationSchema::new(
        1,
        vec![
            ConnectorConfigurationField::new(
                "token",
                ConnectorConfigurationValueKind::Secret,
                ConnectorConfigurationRequirement::Required,
                "Provider authentication token.",
            )
            .with_constraint(ConnectorConfigurationConstraint::NonEmpty),
            ConnectorConfigurationField::new(
                "region",
                ConnectorConfigurationValueKind::Text,
                ConnectorConfigurationRequirement::Default(ConnectorConfigurationValue::Text(
                    "global".to_owned(),
                )),
                "Provider routing region.",
            )
            .with_constraint(ConnectorConfigurationConstraint::OneOf(vec![
                "global".to_owned(),
                "ca".to_owned(),
            ])),
            ConnectorConfigurationField::new(
                "mode",
                ConnectorConfigurationValueKind::Text,
                ConnectorConfigurationRequirement::Default(ConnectorConfigurationValue::Text(
                    "normal".to_owned(),
                )),
                "Conformance fault injection mode.",
            )
            .with_constraint(ConnectorConfigurationConstraint::OneOf(vec![
                "normal".to_owned(),
                "prepare_fail".to_owned(),
                "start_fail".to_owned(),
                "join_fail".to_owned(),
                "worker_panic".to_owned(),
                "saturate".to_owned(),
            ])),
        ],
    )
    .expect("configuration schema")
}

fn manifest() -> ConnectorManifest {
    let media = MediaCaps::Audio(AudioCaps {
        sample_rate_hz: None,
        frame_samples: None,
        channel_layout: ChannelLayout::Any,
        format: SampleFormat::F32Interleaved,
    });
    let input = PortSpec::new(
        "audio",
        PortDirection::Input,
        SignalSpec::audio(),
        media,
        Multiplicity::Many,
        true,
    )
    .expect("audio input");
    let node = NodeDescriptor::new(
        NodeTypeId::from("dev.pocketstation.test.connector.node.v1"),
        "Connector contract test",
        vec![input],
        Vec::new(),
        ExecutionPartition::AsyncWorker,
        SafetyContract::NetworkAllowed,
        true,
    )
    .expect("connector node");
    ConnectorManifest::new(
        1,
        OperatorId::new("dev.pocketstation.test.connector.v1"),
        "1.0.0",
        node,
        configuration_schema(),
        ConnectorDeliveryPolicy::new(EdgeContract::realtime_audio(), 64).expect("delivery policy"),
        ConnectorRetryPolicy::new(5, 2_000, 100, 2_000, 10_000, 2_000, 20).expect("retry policy"),
        ConnectorReadinessPolicy::new(10_000, 250, 1, 3).expect("readiness policy"),
    )
    .expect("connector manifest")
}

fn configuration() -> ConnectorConfiguration {
    ConnectorConfiguration::new().with(
        "token",
        ConnectorConfigurationValue::Secret(
            ConnectorSecret::new("test-secret-value").expect("secret"),
        ),
    )
}

#[test]
fn given_typed_schema_when_configuration_resolves_then_defaults_apply_and_secrets_redact() {
    let configuration = configuration();
    let debug = format!("{configuration:?}");
    assert!(!debug.contains("test-secret-value"));
    assert!(debug.contains("<redacted>"));

    let resolved = configuration_schema()
        .resolve(&configuration)
        .expect("resolved configuration");
    assert_eq!(
        resolved.get("region"),
        Some(&ConnectorConfigurationValue::Text("global".to_owned()))
    );

    let invalid = ConnectorConfiguration::new().with(
        "token",
        ConnectorConfigurationValue::Text("not-classified-as-secret".to_owned()),
    );
    assert_eq!(
        configuration_schema()
            .resolve(&invalid)
            .expect_err("wrong type")
            .code(),
        ConnectorConfigurationErrorCode::WrongType
    );

    let endpoint =
        pocketstation::EndpointConfiguration::new().with_sensitive("token", "test-secret-value");
    let node =
        pocketstation::OperatorConfiguration::new().with_sensitive("token", "test-secret-value");
    assert!(!format!("{endpoint:?}").contains("test-secret-value"));
    assert!(!format!("{node:?}").contains("test-secret-value"));
}

#[test]
fn given_registered_connector_when_declared_then_identity_is_session_scoped() {
    let session = pocketstation::Session::new();
    let registered = session
        .register_connector(
            Connector::new(manifest(), Arc::new(RejectingFactory)).expect("connector"),
        )
        .expect("registration");
    assert_eq!(registered.session_id(), session.id());
    assert_eq!(
        registered.manifest().operator_id().as_str(),
        "dev.pocketstation.test.connector.v1"
    );

    let endpoint = registered
        .declare(&session, configuration())
        .expect("connector declaration");
    assert_eq!(endpoint.session_id(), session.id());
    assert!(endpoint.connector_id().is_some());

    let other = pocketstation::Session::new();
    assert!(registered.declare(&other, configuration()).is_err());
}

#[test]
fn given_duplicate_connector_identity_when_registered_then_registration_is_rejected() {
    let session = pocketstation::Session::new();
    session
        .register_connector(
            Connector::new(manifest(), Arc::new(RejectingFactory)).expect("connector"),
        )
        .expect("first registration");
    let duplicate = session.register_connector(
        Connector::new(manifest(), Arc::new(RejectingFactory)).expect("connector"),
    );
    assert!(duplicate.is_err());
}

#[test]
fn given_connector_observations_when_state_changes_then_degradation_and_loss_are_explicit() {
    let observations = ConnectorObservationHandle::new();
    assert!(observations
        .transition(ConnectorReadiness::Ready)
        .expect("ready transition"));
    observations.record_received(12);
    observations.record_delivered(10);
    observations.record_dropped(2);
    observations.record_retry();
    assert!(observations
        .transition(ConnectorReadiness::Reconnecting)
        .expect("reconnect transition"));
    assert!(observations
        .transition(ConnectorReadiness::Failed)
        .expect("failed transition"));
    assert!(observations.transition(ConnectorReadiness::Ready).is_err());

    let snapshot = observations.snapshot().expect("observations");
    assert_eq!(snapshot.readiness, ConnectorReadiness::Failed);
    assert_eq!(snapshot.items_received_total, 12);
    assert_eq!(snapshot.items_delivered_total, 10);
    assert_eq!(snapshot.items_dropped_total, 2);
    assert_eq!(snapshot.retry_attempts_total, 1);
    assert_eq!(snapshot.reconnects_total, 1);
}

#[test]
fn given_connector_error_when_inspected_then_code_is_stable_and_machine_readable() {
    let code =
        ConnectorErrorCode::new("relay.signaling.answer_timeout").expect("stable error code");
    assert_eq!(code.as_str(), "relay.signaling.answer_timeout");
    assert!(ConnectorErrorCode::new("Relay Signaling Failure").is_err());
    assert!(ConnectorError::new(
        code,
        ConnectorErrorStage::Readiness,
        ConnectorRetryability::Retryable,
        "x".repeat(MAX_CONNECTOR_ERROR_MESSAGE_BYTES + 1),
    )
    .is_err());
}

#[cfg(feature = "conformance-fixtures")]
#[derive(Default)]
struct FaultControl {
    prepare_calls_total: AtomicU64,
    cancelled_preparations_total: AtomicU64,
    start_calls_total: AtomicU64,
    stop_calls_total: AtomicU64,
    join_calls_total: AtomicU64,
}

#[cfg(feature = "conformance-fixtures")]
struct FaultFactory {
    control: Arc<FaultControl>,
}

#[cfg(feature = "conformance-fixtures")]
impl EndpointDriverFactory for FaultFactory {
    fn prepare(
        &self,
        inputs: Vec<EndpointPortInput>,
    ) -> Result<Box<dyn PreparedEndpointDriver>, EndpointFailure> {
        self.control
            .prepare_calls_total
            .fetch_add(1, Ordering::Relaxed);
        let mode = inputs
            .first()
            .and_then(|input| input.context().node_configuration().get("mode"))
            .unwrap_or("normal")
            .to_owned();
        if mode == "prepare_fail" {
            return Err(EndpointFailure::new(
                EndpointFailureStage::Prepare,
                "injected connector preparation failure",
            ));
        }
        Ok(Box::new(FaultPrepared {
            control: Arc::clone(&self.control),
            mode,
            inputs,
        }))
    }
}

#[cfg(feature = "conformance-fixtures")]
struct FaultPrepared {
    control: Arc<FaultControl>,
    mode: String,
    inputs: Vec<EndpointPortInput>,
}

#[cfg(feature = "conformance-fixtures")]
impl PreparedEndpointDriver for FaultPrepared {
    fn start(
        self: Box<Self>,
        start_gate: Arc<pocketstation::EndpointStartGate>,
    ) -> Result<Box<dyn pocketstation::RunningEndpointDriver>, EndpointFailure> {
        self.control
            .start_calls_total
            .fetch_add(1, Ordering::Relaxed);
        if self.mode == "start_fail" {
            return Err(EndpointFailure::new(
                EndpointFailureStage::Start,
                "injected connector start failure",
            ));
        }
        let stop_requested = Arc::new(AtomicBool::new(false));
        let worker = if self.mode == "worker_panic" {
            let stop_requested = Arc::clone(&stop_requested);
            Some(std::thread::spawn(move || {
                while !start_gate.is_open() && !stop_requested.load(Ordering::Acquire) {
                    std::thread::yield_now();
                }
                panic!("injected connector worker panic");
            }))
        } else {
            None
        };
        Ok(Box::new(FaultRunning {
            control: Arc::clone(&self.control),
            mode: self.mode,
            _inputs: self.inputs,
            stop_requested,
            worker,
        }))
    }

    fn cancel_preparation(self: Box<Self>) -> pocketstation::EndpointCancellationOutcome {
        self.control
            .cancelled_preparations_total
            .fetch_add(1, Ordering::Relaxed);
        pocketstation::EndpointCancellationOutcome {
            observations: pocketstation::EndpointDriverObservations::default(),
            result: Ok(()),
        }
    }
}

#[cfg(feature = "conformance-fixtures")]
struct FaultRunning {
    control: Arc<FaultControl>,
    mode: String,
    _inputs: Vec<EndpointPortInput>,
    stop_requested: Arc<AtomicBool>,
    worker: Option<std::thread::JoinHandle<()>>,
}

#[cfg(feature = "conformance-fixtures")]
impl pocketstation::RunningEndpointDriver for FaultRunning {
    fn observations(&self) -> pocketstation::EndpointDriverObservations {
        pocketstation::EndpointDriverObservations::default()
    }

    fn request_stop(&mut self) -> Result<(), EndpointFailure> {
        self.control
            .stop_calls_total
            .fetch_add(1, Ordering::Relaxed);
        self.stop_requested.store(true, Ordering::Release);
        Ok(())
    }

    fn join_and_finalize(mut self: Box<Self>) -> pocketstation::EndpointDriverFinalization {
        self.control
            .join_calls_total
            .fetch_add(1, Ordering::Relaxed);
        let worker_result = self.worker.take().map_or(Ok(()), |worker| {
            worker.join().map_err(|_| {
                EndpointFailure::new(
                    EndpointFailureStage::JoinFinalize,
                    "connector worker panicked",
                )
            })
        });
        let result = if self.mode == "join_fail" {
            Err(EndpointFailure::new(
                EndpointFailureStage::JoinFinalize,
                "injected connector join failure",
            ))
        } else {
            worker_result
        };
        pocketstation::EndpointDriverFinalization {
            observations: self.observations(),
            result,
        }
    }
}

#[cfg(feature = "conformance-fixtures")]
impl Drop for FaultRunning {
    fn drop(&mut self) {
        self.stop_requested.store(true, Ordering::Release);
        let _ = self.worker.take();
    }
}

#[cfg(feature = "conformance-fixtures")]
fn fault_configuration(mode: &str) -> ConnectorConfiguration {
    configuration().with("mode", ConnectorConfigurationValue::Text(mode.to_owned()))
}

#[cfg(feature = "conformance-fixtures")]
fn fault_session(
    control: Arc<FaultControl>,
) -> (
    pocketstation::Session,
    pocketstation::connector::RegisteredConnector,
) {
    let session = pocketstation::conformance::session().expect("conformance Session");
    let registered = register_fault_connector(&session, control);
    (session, registered)
}

#[cfg(feature = "conformance-fixtures")]
fn register_fault_connector(
    session: &pocketstation::Session,
    control: Arc<FaultControl>,
) -> pocketstation::connector::RegisteredConnector {
    let connector =
        Connector::new(manifest(), Arc::new(FaultFactory { control })).expect("fault connector");
    session
        .register_connector(connector)
        .expect("fault connector registration")
}

#[cfg(feature = "conformance-fixtures")]
#[test]
fn given_prior_preparation_when_connector_prepare_fails_then_prior_work_rolls_back() {
    let control = Arc::new(FaultControl::default());
    let (session, connector) = fault_session(Arc::clone(&control));
    let first = connector
        .declare(&session, fault_configuration("normal"))
        .expect("normal endpoint");
    let second = connector
        .declare(&session, fault_configuration("prepare_fail"))
        .expect("failing endpoint");
    let application = session
        .capture(pocketstation::Source::application(
            pocketstation::ApplicationSelector::name("PocketStation Fixture"),
        ))
        .expect("application stem");
    let microphone = session
        .capture(pocketstation::Source::microphone_default())
        .expect("microphone stem");
    application.send(first).expect("first route");
    microphone.send(second).expect("second route");

    assert!(session.start().is_err());
    assert_eq!(control.prepare_calls_total.load(Ordering::Relaxed), 2);
    assert_eq!(
        control.cancelled_preparations_total.load(Ordering::Relaxed),
        1
    );
}

#[cfg(feature = "conformance-fixtures")]
#[test]
fn given_prepared_connectors_when_connector_start_fails_then_start_is_transactional() {
    let control = Arc::new(FaultControl::default());
    let (session, connector) = fault_session(Arc::clone(&control));
    let endpoint = connector
        .declare(&session, fault_configuration("start_fail"))
        .expect("start-failing endpoint");
    let application = session
        .capture(pocketstation::Source::application(
            pocketstation::ApplicationSelector::name("PocketStation Fixture"),
        ))
        .expect("application stem");
    let microphone = session
        .capture(pocketstation::Source::microphone_default())
        .expect("microphone stem");
    application.send(endpoint).expect("route");
    microphone.send(endpoint).expect("microphone route");

    assert!(session.start().is_err());
    assert_eq!(control.start_calls_total.load(Ordering::Relaxed), 1);
}

#[cfg(feature = "conformance-fixtures")]
#[test]
fn given_running_connector_when_session_stops_then_stop_and_join_are_called() {
    let control = Arc::new(FaultControl::default());
    let (session, connector) = fault_session(Arc::clone(&control));
    let endpoint = connector
        .declare(&session, fault_configuration("normal"))
        .expect("normal endpoint");
    let application = session
        .capture(pocketstation::Source::application(
            pocketstation::ApplicationSelector::name("PocketStation Fixture"),
        ))
        .expect("application stem");
    let microphone = session
        .capture(pocketstation::Source::microphone_default())
        .expect("microphone stem");
    application.send(endpoint).expect("route");
    microphone.send(endpoint).expect("microphone route");
    let mut running = session.start().expect("running Session");

    assert!(running.cancel().is_success());
    assert_eq!(control.stop_calls_total.load(Ordering::Relaxed), 2);
    assert_eq!(control.join_calls_total.load(Ordering::Relaxed), 2);
}

#[cfg(feature = "conformance-fixtures")]
fn run_terminal_fault(mode: &str) -> bool {
    let control = Arc::new(FaultControl::default());
    let (session, connector) = fault_session(control);
    let endpoint = connector
        .declare(&session, fault_configuration(mode))
        .expect("fault endpoint");
    let application = session
        .capture(pocketstation::Source::application(
            pocketstation::ApplicationSelector::name("PocketStation Fixture"),
        ))
        .expect("application stem");
    let microphone = session
        .capture(pocketstation::Source::microphone_default())
        .expect("microphone stem");
    application.send(endpoint).expect("route");
    microphone.send(endpoint).expect("microphone route");
    let mut running = session.start().expect("running Session");
    if mode == "worker_panic" {
        std::thread::sleep(Duration::from_millis(10));
    }
    running.stop().is_success()
}

#[cfg(feature = "conformance-fixtures")]
#[test]
fn given_join_failure_or_worker_panic_when_session_stops_then_failure_is_terminal() {
    assert!(!run_terminal_fault("join_fail"));
    assert!(!run_terminal_fault("worker_panic"));
}

#[cfg(feature = "conformance-fixtures")]
#[test]
fn given_saturated_connector_route_when_observed_then_drops_are_visible_in_metrics() {
    let control = Arc::new(FaultControl::default());
    let session = pocketstation::conformance::session_for_saturation()
        .expect("saturation conformance Session");
    let connector = register_fault_connector(&session, control);
    let endpoint = connector
        .declare(&session, fault_configuration("saturate"))
        .expect("saturating endpoint");
    let application = session
        .capture(pocketstation::Source::application(
            pocketstation::ApplicationSelector::name("PocketStation Fixture"),
        ))
        .expect("application stem");
    let microphone = session
        .capture(pocketstation::Source::microphone_default())
        .expect("microphone stem");
    let application_route = application.send(endpoint).expect("route");
    microphone.send(endpoint).expect("microphone route");
    let mut running = session.start().expect("running Session");
    let deadline = Instant::now() + Duration::from_secs(3);
    loop {
        let metrics = running.metrics_snapshot().expect("metrics");
        let saturated = (0..metrics.route_count())
            .filter_map(|index| metrics.route(index))
            .any(|route| {
                route.route_id == application_route && route.edge.queue_full_drops_total > 0
            });
        if saturated {
            break;
        }
        assert!(Instant::now() < deadline, "route must visibly saturate");
        std::thread::sleep(Duration::from_millis(5));
    }
    let _ = running.stop();
}
