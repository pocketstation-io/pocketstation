#[cfg(feature = "conformance-fixtures")]
use std::sync::atomic::{AtomicBool, AtomicU64, AtomicU8, Ordering};
use std::sync::Arc;
use std::time::Duration;
#[cfg(feature = "conformance-fixtures")]
use std::time::Instant;

use pocketstation::connector::{
    Connector, ConnectorConfiguration, ConnectorConfigurationConstraint,
    ConnectorConfigurationErrorCode, ConnectorConfigurationField,
    ConnectorConfigurationRequirement, ConnectorConfigurationSchema, ConnectorConfigurationValue,
    ConnectorConfigurationValueKind, ConnectorDeliveryReadiness, ConnectorError,
    ConnectorErrorCode, ConnectorErrorStage, ConnectorFactory, ConnectorHealth, ConnectorManifest,
    ConnectorReadinessPolicy, ConnectorRecovery, ConnectorRetryability, ConnectorRunOutcome,
    ConnectorSecret, ConnectorWorker, MAX_CONNECTOR_ERROR_MESSAGE_BYTES,
};
#[cfg(feature = "conformance-fixtures")]
use pocketstation::connector::{
    ConnectorDeliveryOutcome, ConnectorDriver, ConnectorDriverFactory, ConnectorInputDescriptor,
    ConnectorItem,
};
use pocketstation::{
    AudioCaps, ChannelLayout, EdgeContract, EndpointPortInput, EndpointPreparationGroup,
    ExecutionPartition, MediaCaps, Multiplicity, NodeDescriptor, NodeTypeId, OperatorId,
    PortDirection, PortSpec, SafetyContract, SampleFormat, SignalSpec,
};

#[test]
fn given_connector_public_surface_when_inspected_then_managed_aliases_are_absent() {
    let source = include_str!("../src/connector/mod.rs");
    for removed in [
        concat!("Managed", "Connector"),
        concat!("Managed", "ConnectorFactory"),
        concat!("pub fn ", "managed("),
    ] {
        assert!(
            !source.contains(removed),
            "removed connector API returned: {removed}"
        );
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
                "terminal_fail".to_owned(),
                "worker_panic".to_owned(),
                "saturate".to_owned(),
                "degraded".to_owned(),
                "never_ready".to_owned(),
            ])),
        ],
    )
    .expect("configuration schema")
}

fn manifest() -> ConnectorManifest {
    manifest_with_readiness(
        ConnectorReadinessPolicy::new(Duration::from_secs(10), Duration::from_millis(25), 1, 3)
            .expect("readiness policy"),
    )
}

fn manifest_with_readiness(readiness: ConnectorReadinessPolicy) -> ConnectorManifest {
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
        readiness,
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

fn fault_configuration(mode: &str) -> ConnectorConfiguration {
    configuration().with("mode", ConnectorConfigurationValue::Text(mode.to_owned()))
}

fn connector_error(code: &str, stage: ConnectorErrorStage, message: &str) -> ConnectorError {
    ConnectorError::new(
        ConnectorErrorCode::new(code).expect("static connector error code"),
        stage,
        ConnectorRetryability::Never,
        message,
    )
    .expect("static connector error")
}

struct RejectingFactory;

impl ConnectorFactory for RejectingFactory {
    fn prepare(
        &self,
        _inputs: Vec<EndpointPortInput>,
    ) -> Result<Box<dyn ConnectorWorker>, ConnectorError> {
        Err(connector_error(
            "test.prepare_rejected",
            ConnectorErrorStage::Prepare,
            "test factory is declaration-only",
        ))
    }
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
        .declare(&session, configuration(), EdgeContract::realtime_audio())
        .expect("connector declaration");
    assert_eq!(endpoint.session_id(), session.id());
    assert!(endpoint.connector_id().is_some());

    let other = pocketstation::Session::new();
    assert!(registered
        .declare(&other, configuration(), EdgeContract::realtime_audio())
        .is_err());
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
    let delivery_failure = connector_error(
        "test.delivery_failure",
        ConnectorErrorStage::Delivery,
        "delivery failed",
    )
    .into_endpoint_failure();
    assert_eq!(
        delivery_failure.stage(),
        pocketstation::EndpointFailureStage::JoinFinalize
    );
    assert_eq!(delivery_failure.code(), Some("test.delivery_failure"));
    assert_eq!(
        delivery_failure.retryability(),
        Some(pocketstation::EndpointFailureRetryability::Never)
    );
    assert_eq!(
        connector_error(
            "test.shutdown_failure",
            ConnectorErrorStage::Shutdown,
            "shutdown failed",
        )
        .into_endpoint_failure()
        .stage(),
        pocketstation::EndpointFailureStage::RequestStop
    );
}

#[cfg(feature = "conformance-fixtures")]
#[derive(Default)]
struct FaultControl {
    prepare_calls_total: AtomicU64,
    cancelled_preparations_total: AtomicU64,
    run_calls_total: AtomicU64,
    completed_runs_total: AtomicU64,
    shutdown_mode: AtomicU8,
}

#[cfg(feature = "conformance-fixtures")]
struct FaultFactory {
    control: Arc<FaultControl>,
}

#[cfg(feature = "conformance-fixtures")]
impl ConnectorFactory for FaultFactory {
    fn preparation_group(
        &self,
        route_id: pocketstation::RouteId,
        configuration: &pocketstation::graph::NodeConfig,
    ) -> Result<EndpointPreparationGroup, ConnectorError> {
        if configuration.get("mode") == Some("normal")
            || configuration.get("mode") == Some("degraded")
        {
            return Ok(EndpointPreparationGroup::Shared(
                pocketstation::EndpointGroupId::new("connector-contract-test"),
            ));
        }
        Ok(EndpointPreparationGroup::Route(route_id))
    }

    fn prepare(
        &self,
        inputs: Vec<EndpointPortInput>,
    ) -> Result<Box<dyn ConnectorWorker>, ConnectorError> {
        self.control
            .prepare_calls_total
            .fetch_add(1, Ordering::Relaxed);
        let mode = inputs
            .first()
            .and_then(|input| input.context().node_configuration().get("mode"))
            .unwrap_or("normal")
            .to_owned();
        if mode == "prepare_fail" {
            return Err(connector_error(
                "test.prepare_failure",
                ConnectorErrorStage::Prepare,
                "injected connector preparation failure",
            ));
        }
        Ok(Box::new(FaultWorker {
            control: Arc::clone(&self.control),
            mode,
            inputs,
        }))
    }
}

#[cfg(feature = "conformance-fixtures")]
struct FaultWorker {
    control: Arc<FaultControl>,
    mode: String,
    inputs: Vec<EndpointPortInput>,
}

#[cfg(feature = "conformance-fixtures")]
impl ConnectorWorker for FaultWorker {
    fn run(
        self: Box<Self>,
        context: pocketstation::connector::ConnectorContext,
    ) -> ConnectorRunOutcome {
        self.control.run_calls_total.fetch_add(1, Ordering::Relaxed);
        if self.mode != "never_ready" {
            let _ = context.report_readiness_success();
        }
        if self.mode == "worker_panic" {
            panic!("injected connector worker panic");
        }
        if self.mode == "terminal_fail" {
            return ConnectorRunOutcome::failure(connector_error(
                "test.terminal_failure",
                ConnectorErrorStage::Delivery,
                "injected connector terminal failure",
            ));
        }
        if self.mode == "degraded" {
            let reason = ConnectorErrorCode::new("test.provider_degraded").expect("reason");
            let _ = context.set_degraded(reason.clone());
            let _ = context.set_reconnecting(reason);
        }
        let mut receivers: Vec<_> = self
            .inputs
            .into_iter()
            .filter_map(|input| match input.into_parts().0 {
                pocketstation::EndpointReceiver::Audio { receiver, .. } => Some(receiver),
                pocketstation::EndpointReceiver::Signal(_) => None,
            })
            .collect();
        while !context.is_stop_requested() {
            let mut progressed = false;
            if self.mode != "saturate" {
                for receiver in &mut receivers {
                    if receiver.try_recv().is_some() {
                        context.record_frame_received(1);
                        context.record_frame_delivered(1);
                        progressed = true;
                    }
                }
            }
            if !progressed {
                let _ = context.wait_for_stop(Duration::from_millis(1));
            }
        }
        self.control.shutdown_mode.store(
            match context.shutdown_mode() {
                Some(pocketstation::EndpointShutdownMode::Drain) => 1,
                Some(pocketstation::EndpointShutdownMode::Abort) => 2,
                None => 0,
            },
            Ordering::Release,
        );
        self.control
            .completed_runs_total
            .fetch_add(1, Ordering::Relaxed);
        ConnectorRunOutcome::success()
    }

    fn cancel_preparation(self: Box<Self>) -> Result<(), ConnectorError> {
        self.control
            .cancelled_preparations_total
            .fetch_add(1, Ordering::Relaxed);
        Ok(())
    }
}

#[cfg(feature = "conformance-fixtures")]
fn register_fault_connector(
    session: &pocketstation::Session,
    control: Arc<FaultControl>,
) -> pocketstation::connector::RegisteredConnector {
    register_fault_connector_with_manifest(session, control, manifest())
}

#[cfg(feature = "conformance-fixtures")]
fn register_fault_connector_with_manifest(
    session: &pocketstation::Session,
    control: Arc<FaultControl>,
    manifest: ConnectorManifest,
) -> pocketstation::connector::RegisteredConnector {
    session
        .register_connector(
            Connector::new(manifest, Arc::new(FaultFactory { control })).expect("fault connector"),
        )
        .expect("fault connector registration")
}

#[cfg(feature = "conformance-fixtures")]
fn routed_fault_session(
    mode: &str,
) -> (
    pocketstation::Session,
    pocketstation::connector::RegisteredConnector,
    pocketstation::EndpointHandle,
    Arc<FaultControl>,
) {
    let control = Arc::new(FaultControl::default());
    let session = pocketstation::conformance::session().expect("conformance Session");
    let registered = register_fault_connector(&session, Arc::clone(&control));
    let endpoint = registered
        .declare(
            &session,
            fault_configuration(mode),
            EdgeContract::realtime_audio(),
        )
        .expect("fault endpoint");
    let application = session
        .capture(pocketstation::Source::application(
            pocketstation::ApplicationSelector::name("PocketStation Fixture"),
        ))
        .expect("application stem");
    let microphone = session
        .capture(pocketstation::Source::microphone_default())
        .expect("microphone stem");
    application.send(endpoint).expect("application route");
    microphone.send(endpoint).expect("microphone route");
    (session, registered, endpoint, control)
}

#[cfg(feature = "conformance-fixtures")]
#[derive(Default)]
struct DriverControl {
    prepared_inputs_total: AtomicU64,
    delivered_items_total: AtomicU64,
    typed_secret_observed: AtomicBool,
    route_edge_observed: AtomicBool,
    shutdown_mode: AtomicU8,
}

#[cfg(feature = "conformance-fixtures")]
struct DriverFactory {
    control: Arc<DriverControl>,
}

#[cfg(feature = "conformance-fixtures")]
impl ConnectorDriverFactory for DriverFactory {
    fn preparation_group(
        &self,
        _route_id: pocketstation::RouteId,
        configuration: &pocketstation::connector::ResolvedConnectorConfiguration,
    ) -> Result<EndpointPreparationGroup, ConnectorError> {
        if matches!(
            configuration.get("token"),
            Some(ConnectorConfigurationValue::Secret(_))
        ) {
            self.control
                .typed_secret_observed
                .store(true, Ordering::Release);
        }
        Ok(EndpointPreparationGroup::Shared(
            pocketstation::EndpointGroupId::new("connector-driver-contract-test"),
        ))
    }

    fn prepare(
        &self,
        inputs: &[ConnectorInputDescriptor],
    ) -> Result<Box<dyn ConnectorDriver>, ConnectorError> {
        self.control
            .prepared_inputs_total
            .store(inputs.len() as u64, Ordering::Release);
        self.control.route_edge_observed.store(
            inputs
                .iter()
                .all(|input| input.edge_contract().jitter_budget_ms() == Some(9)),
            Ordering::Release,
        );
        Ok(Box::new(TestDriver {
            control: Arc::clone(&self.control),
        }))
    }
}

#[cfg(feature = "conformance-fixtures")]
struct TestDriver {
    control: Arc<DriverControl>,
}

#[cfg(feature = "conformance-fixtures")]
impl ConnectorDriver for TestDriver {
    fn deliver(
        &mut self,
        item: ConnectorItem<'_>,
        _context: &pocketstation::connector::ConnectorContext,
    ) -> Result<ConnectorDeliveryOutcome, ConnectorError> {
        assert_eq!(item.input().port_name(), "audio");
        assert!(matches!(item, ConnectorItem::Audio { .. }));
        self.control
            .delivered_items_total
            .fetch_add(1, Ordering::Relaxed);
        Ok(ConnectorDeliveryOutcome::Delivered)
    }

    fn shutdown(
        &mut self,
        mode: pocketstation::EndpointShutdownMode,
        _context: &pocketstation::connector::ConnectorContext,
    ) -> Result<(), ConnectorError> {
        self.control.shutdown_mode.store(
            match mode {
                pocketstation::EndpointShutdownMode::Drain => 1,
                pocketstation::EndpointShutdownMode::Abort => 2,
            },
            Ordering::Release,
        );
        Ok(())
    }
}

#[cfg(feature = "conformance-fixtures")]
#[test]
fn given_connector_driver_when_two_stems_run_then_core_owns_typed_delivery_and_drain() {
    let control = Arc::new(DriverControl::default());
    let session = pocketstation::conformance::session().expect("conformance Session");
    let registered = session
        .register_connector(
            Connector::with_driver(
                manifest(),
                Arc::new(DriverFactory {
                    control: Arc::clone(&control),
                }),
            )
            .expect("connector driver"),
        )
        .expect("connector driver registration");
    let edge = EdgeContract::realtime_audio().with_jitter_budget_ms(Some(9));
    let endpoint = registered
        .declare(&session, configuration(), edge)
        .expect("connector endpoint");
    let application = session
        .capture(pocketstation::Source::application(
            pocketstation::ApplicationSelector::name("PocketStation Fixture"),
        ))
        .expect("application stem");
    let microphone = session
        .capture(pocketstation::Source::microphone_default())
        .expect("microphone stem");
    application.send(endpoint).expect("application route");
    microphone.send(endpoint).expect("microphone route");

    let mut running = session.start().expect("running Session");
    let deadline = Instant::now() + Duration::from_secs(2);
    while control.delivered_items_total.load(Ordering::Acquire) < 2 {
        assert!(Instant::now() < deadline, "connector driver must deliver");
        std::thread::sleep(Duration::from_millis(2));
    }
    assert!(running.stop().is_success());
    assert_eq!(control.prepared_inputs_total.load(Ordering::Acquire), 2);
    assert!(control.typed_secret_observed.load(Ordering::Acquire));
    assert!(control.route_edge_observed.load(Ordering::Acquire));
    assert_eq!(control.shutdown_mode.load(Ordering::Acquire), 1);
    let observation = registered
        .observations()
        .expect("connector observations")
        .into_iter()
        .next()
        .expect("connector runtime observation");
    assert!(observation.endpoint.frames_received_total >= 2);
    assert_eq!(
        observation.endpoint.frames_received_total,
        observation.endpoint.frames_delivered_total
    );
}

#[cfg(feature = "conformance-fixtures")]
#[test]
fn given_prior_preparation_when_connector_prepare_fails_then_prior_work_rolls_back() {
    let control = Arc::new(FaultControl::default());
    let session = pocketstation::conformance::session().expect("conformance Session");
    let connector = register_fault_connector(&session, Arc::clone(&control));
    let first = connector
        .declare(
            &session,
            fault_configuration("normal"),
            EdgeContract::realtime_audio(),
        )
        .expect("normal endpoint");
    let second = connector
        .declare(
            &session,
            fault_configuration("prepare_fail"),
            EdgeContract::realtime_audio(),
        )
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
fn given_grouped_connector_when_session_stops_then_one_worker_is_joined_and_observed() {
    let (session, registered, endpoint, control) = routed_fault_session("normal");
    let mut running = session.start().expect("running Session");
    let deadline = Instant::now() + Duration::from_secs(2);
    loop {
        let snapshots = registered.observations().expect("snapshots");
        if snapshots
            .first()
            .is_some_and(|snapshot| snapshot.endpoint.frames_delivered_total > 0)
        {
            break;
        }
        assert!(Instant::now() < deadline, "connector must deliver");
        std::thread::sleep(Duration::from_millis(2));
    }
    assert!(registered
        .observation(endpoint)
        .expect("same Session")
        .expect("observation")
        .snapshot()
        .expect("snapshot")
        .service_status
        .accepts_delivery());
    assert!(running.stop().is_success());
    assert_eq!(control.run_calls_total.load(Ordering::Relaxed), 1);
    assert_eq!(control.completed_runs_total.load(Ordering::Relaxed), 1);
    assert_eq!(control.shutdown_mode.load(Ordering::Acquire), 1);
    assert_eq!(registered.observations().expect("snapshots").len(), 1);
}

#[cfg(feature = "conformance-fixtures")]
#[test]
fn given_grouped_connector_when_session_is_cancelled_then_abort_intent_reaches_worker() {
    let (session, _registered, _endpoint, control) = routed_fault_session("normal");
    let mut running = session.start().expect("running Session");
    assert!(running.cancel().is_success());
    assert_eq!(control.completed_runs_total.load(Ordering::Relaxed), 1);
    assert_eq!(control.shutdown_mode.load(Ordering::Acquire), 2);
}

#[cfg(feature = "conformance-fixtures")]
#[test]
fn given_orthogonal_provider_status_when_reconnecting_then_endpoint_metrics_remain_canonical() {
    let (session, registered, endpoint, _control) = routed_fault_session("degraded");
    let mut running = session.start().expect("running Session");
    let deadline = Instant::now() + Duration::from_secs(2);
    loop {
        let observation = registered
            .observation(endpoint)
            .expect("same Session")
            .expect("observation")
            .snapshot()
            .expect("snapshot");
        if observation.service_status.recovery() == ConnectorRecovery::Reconnecting {
            assert_eq!(
                observation.service_status.delivery_readiness(),
                ConnectorDeliveryReadiness::NotReady
            );
            assert_eq!(
                observation.service_status.health(),
                ConnectorHealth::Degraded
            );
            assert!(!observation.service_status.accepts_delivery());
            assert_eq!(observation.reconnects_total, 1);
            break;
        }
        assert!(Instant::now() < deadline, "connector must report recovery");
        std::thread::sleep(Duration::from_millis(2));
    }
    assert!(running.stop().is_success());
}

#[cfg(feature = "conformance-fixtures")]
#[test]
fn given_connector_never_ready_when_startup_deadline_expires_then_failure_is_terminal() {
    let control = Arc::new(FaultControl::default());
    let session = pocketstation::conformance::session().expect("conformance Session");
    let readiness =
        ConnectorReadinessPolicy::new(Duration::from_millis(40), Duration::from_millis(5), 1, 1)
            .expect("short readiness policy");
    let registered = register_fault_connector_with_manifest(
        &session,
        control,
        manifest_with_readiness(readiness),
    );
    let endpoint = registered
        .declare(
            &session,
            fault_configuration("never_ready"),
            EdgeContract::realtime_audio(),
        )
        .expect("not-ready endpoint");
    let application = session
        .capture(pocketstation::Source::application(
            pocketstation::ApplicationSelector::name("PocketStation Fixture"),
        ))
        .expect("application stem");
    let microphone = session
        .capture(pocketstation::Source::microphone_default())
        .expect("microphone stem");
    application.send(endpoint).expect("application route");
    microphone.send(endpoint).expect("microphone route");
    let mut running = session.start().expect("running Session");
    std::thread::sleep(Duration::from_millis(80));
    assert!(!running.stop().is_success());
    let observation = registered
        .observation(endpoint)
        .expect("same Session")
        .expect("retained observation")
        .snapshot()
        .expect("terminal snapshot");
    assert_eq!(
        observation
            .last_error
            .as_ref()
            .map(|error| error.code().as_str()),
        Some("core.readiness_timeout")
    );
    assert_eq!(
        observation.service_status.delivery_readiness(),
        ConnectorDeliveryReadiness::NotReady
    );
}

#[cfg(feature = "conformance-fixtures")]
#[test]
fn given_worker_failure_or_panic_when_session_stops_then_endpoint_finalization_is_terminal() {
    for mode in ["terminal_fail", "worker_panic"] {
        let (session, registered, endpoint, _control) = routed_fault_session(mode);
        let mut running = session.start().expect("running Session");
        std::thread::sleep(Duration::from_millis(20));
        assert!(!running.stop().is_success(), "{mode} must be terminal");
        if mode == "terminal_fail" {
            let mut structured_failure = None;
            while let pocketstation::SessionEventReceive::Event(event) = running.try_recv_event() {
                if let pocketstation::SessionEventKind::Endpoint(failure) = event.kind() {
                    structured_failure = Some(failure.failure().clone());
                }
            }
            let failure = structured_failure.expect("terminal endpoint failure event");
            assert_eq!(failure.code(), Some("test.terminal_failure"));
            assert_eq!(
                failure.retryability(),
                Some(pocketstation::EndpointFailureRetryability::Never)
            );
        }
        let observation = registered
            .observation(endpoint)
            .expect("same Session")
            .expect("retained observation")
            .snapshot()
            .expect("terminal snapshot");
        assert_eq!(observation.failures_total, 1);
        assert!(observation.last_error.is_some());
    }
}

#[cfg(feature = "conformance-fixtures")]
#[test]
fn given_saturated_connector_route_when_observed_then_drops_are_visible_in_session_metrics() {
    let control = Arc::new(FaultControl::default());
    let session = pocketstation::conformance::session_for_saturation()
        .expect("saturation conformance Session");
    let connector = register_fault_connector(&session, control);
    let endpoint = connector
        .declare(
            &session,
            fault_configuration("saturate"),
            EdgeContract::realtime_audio(),
        )
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
    assert!(running.stop().is_success());
}
