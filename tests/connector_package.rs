use std::sync::Arc;
use std::time::Duration;

use pocketstation::connector::{
    Connector, ConnectorComponentId, ConnectorComponentKind, ConnectorConfiguration,
    ConnectorConfigurationSchema, ConnectorDeliveryOutcome, ConnectorDriver,
    ConnectorDriverFactory, ConnectorError, ConnectorInputDescriptor, ConnectorItem,
    ConnectorManifest, ConnectorPackage, ConnectorPackageId, ConnectorReadinessPolicy,
};
use pocketstation::{
    AsyncNode, AsyncOperatorFactory, AsyncOperatorManifest, AsyncOperatorPrepareContext, AudioCaps,
    BackpressurePolicy, ChannelLayout, ConfigError, CopyPolicy, EdgeContract,
    EndpointPreparationGroup, ExecutionPartition, MediaCaps, Multiplicity, NodeDescriptor,
    NodeError, NodeTypeId, OperatorCancellationPolicy, OperatorConfiguration,
    OperatorDeadlinePolicy, OperatorFailurePolicy, OperatorId, OperatorOutputRolePolicy,
    OperatorPermissionPolicy, PortDirection, PortSpec, SafetyContract, SampleFormat, Session,
    SignalEnvelope, SignalSpec, SourceCancellation, SourceConfiguration, SourceDriver,
    SourceDriverError, SourceEmission, SourceFactory, SourceManifest, SourcePrepareContext,
    SourceTypeId,
};

const SOURCE_COMPONENT: &str = "capture";
const OPERATOR_COMPONENT: &str = "transform";
const ENDPOINT_COMPONENT: &str = "publisher";
const SOURCE_TYPE: &str = "dev.pocketstation.test.package.source.v1";
const OPERATOR_TYPE: &str = "dev.pocketstation.test.package.operator.v1";
const OPERATOR_NODE: &str = "dev.pocketstation.test.package.operator.node.v1";
const ENDPOINT_OPERATOR: &str = "dev.pocketstation.test.package.endpoint.v1";
const ENDPOINT_NODE: &str = "dev.pocketstation.test.package.endpoint.node.v1";

fn audio_media() -> MediaCaps {
    MediaCaps::Audio(AudioCaps {
        sample_rate_hz: Some(48_000),
        frame_samples: Some(960),
        channel_layout: ChannelLayout::Mono,
        format: SampleFormat::F32Interleaved,
    })
}

fn audio_port(name: &str, direction: PortDirection) -> PortSpec {
    PortSpec::new(
        name,
        direction,
        SignalSpec::audio(),
        audio_media(),
        Multiplicity::Many,
        true,
    )
    .expect("audio port")
}

struct PackageSourceFactory {
    manifest: SourceManifest,
}

struct PackageSourceDriver;

impl SourceDriver for PackageSourceDriver {
    fn prepare(&mut self, _context: &SourcePrepareContext) -> Result<(), SourceDriverError> {
        Ok(())
    }

    fn next(
        &mut self,
        _cancellation: &SourceCancellation,
    ) -> Result<Option<SourceEmission>, SourceDriverError> {
        Ok(None)
    }

    fn close(&mut self) -> Result<(), SourceDriverError> {
        Ok(())
    }
}

impl SourceFactory for PackageSourceFactory {
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
        Ok(Box::new(PackageSourceDriver))
    }
}

fn source_factory() -> Arc<dyn SourceFactory> {
    Arc::new(PackageSourceFactory {
        manifest: SourceManifest::new(
            SourceTypeId::new(SOURCE_TYPE).expect("source type"),
            1,
            1,
            vec![audio_port("audio", PortDirection::Output)],
            ExecutionPartition::BlockingWorker,
            SafetyContract::AllocationAllowed,
        )
        .expect("source manifest"),
    })
}

struct PackageOperatorFactory {
    manifest: AsyncOperatorManifest,
}

struct PackageOperator;

impl AsyncNode for PackageOperator {
    fn prepare<'a>(
        &'a mut self,
        _context: &'a AsyncOperatorPrepareContext,
    ) -> pocketstation::AsyncNodeFuture<'a, Result<(), NodeError>> {
        Box::pin(async { Ok(()) })
    }

    fn process<'a>(
        &'a mut self,
        input: SignalEnvelope,
    ) -> pocketstation::AsyncNodeFuture<'a, Result<Vec<SignalEnvelope>, NodeError>> {
        Box::pin(async move { Ok(vec![input]) })
    }
}

impl AsyncOperatorFactory for PackageOperatorFactory {
    fn manifest(&self) -> &AsyncOperatorManifest {
        &self.manifest
    }

    fn validate_config(&self, _configuration: &OperatorConfiguration) -> Result<(), ConfigError> {
        Ok(())
    }

    fn create(
        &self,
        _configuration: &OperatorConfiguration,
    ) -> Result<Box<dyn AsyncNode>, NodeError> {
        Ok(Box::new(PackageOperator))
    }
}

fn operator_factory() -> Arc<dyn AsyncOperatorFactory> {
    let input_edge = EdgeContract::bounded_async()
        .with_media(audio_media())
        .with_backpressure(BackpressurePolicy::DropNewest)
        .with_copy_policy(CopyPolicy::CopyToBranchPool);
    let output_edge = EdgeContract::bounded_async().with_media(audio_media());
    Arc::new(PackageOperatorFactory {
        manifest: AsyncOperatorManifest::new(
            OperatorId::new(OPERATOR_TYPE),
            1,
            1,
            NodeDescriptor::new(
                NodeTypeId::from(OPERATOR_NODE),
                "Package operator",
                vec![audio_port("audio", PortDirection::Input)],
                vec![audio_port("audio", PortDirection::Output)],
                ExecutionPartition::AsyncWorker,
                SafetyContract::AllocationAllowed,
                false,
            )
            .expect("operator node"),
            input_edge,
            output_edge,
            8,
            OperatorPermissionPolicy {
                network_allowed: false,
                filesystem_allowed: false,
            },
            OperatorDeadlinePolicy {
                process_timeout_ms: 500,
            },
            OperatorCancellationPolicy::DiscardQueued,
            OperatorFailurePolicy::StopWorker,
            OperatorOutputRolePolicy::default(),
        )
        .expect("operator manifest"),
    })
}

struct PackageEndpointFactory;
struct PackageEndpointDriver;

impl ConnectorDriverFactory for PackageEndpointFactory {
    fn preparation_group(
        &self,
        route_id: pocketstation::RouteId,
        _configuration: &pocketstation::connector::ResolvedConnectorConfiguration,
    ) -> Result<EndpointPreparationGroup, ConnectorError> {
        Ok(EndpointPreparationGroup::Route(route_id))
    }

    fn prepare(
        &self,
        _inputs: &[ConnectorInputDescriptor],
    ) -> Result<Box<dyn ConnectorDriver>, ConnectorError> {
        Ok(Box::new(PackageEndpointDriver))
    }
}

impl ConnectorDriver for PackageEndpointDriver {
    fn deliver(
        &mut self,
        _item: ConnectorItem<'_>,
        _context: &pocketstation::connector::ConnectorContext,
    ) -> Result<ConnectorDeliveryOutcome, ConnectorError> {
        Ok(ConnectorDeliveryOutcome::Delivered)
    }
}

fn endpoint_connector() -> Connector {
    let manifest = ConnectorManifest::new(
        1,
        OperatorId::new(ENDPOINT_OPERATOR),
        env!("CARGO_PKG_VERSION"),
        NodeDescriptor::new(
            NodeTypeId::from(ENDPOINT_NODE),
            "Package endpoint",
            vec![audio_port("audio", PortDirection::Input)],
            Vec::new(),
            ExecutionPartition::AsyncWorker,
            SafetyContract::NetworkAllowed,
            true,
        )
        .expect("endpoint node"),
        ConnectorConfigurationSchema::new(1, Vec::new()).expect("configuration schema"),
        EdgeContract::realtime_audio(),
        ConnectorReadinessPolicy::new(Duration::from_secs(1), Duration::from_millis(5), 1, 1)
            .expect("readiness policy"),
    )
    .expect("connector manifest");
    Connector::with_driver(manifest, Arc::new(PackageEndpointFactory)).expect("connector")
}

#[test]
fn given_composed_connector_package_when_installed_then_existing_session_authorities_execute() {
    let mut package = ConnectorPackage::new(
        1,
        ConnectorPackageId::new("dev.pocketstation.test.composed").expect("package id"),
        env!("CARGO_PKG_VERSION"),
    )
    .expect("package");
    package
        .add_source(
            ConnectorComponentId::new(SOURCE_COMPONENT).expect("component id"),
            source_factory(),
        )
        .expect("source component");
    package
        .add_operator(
            ConnectorComponentId::new(OPERATOR_COMPONENT).expect("component id"),
            operator_factory(),
        )
        .expect("operator component");
    package
        .add_endpoint(
            ConnectorComponentId::new(ENDPOINT_COMPONENT).expect("component id"),
            endpoint_connector(),
        )
        .expect("endpoint component");

    let manifest = package.manifest();
    assert_eq!(manifest.components().len(), 3);
    assert_eq!(
        manifest.components()[0].kind(),
        ConnectorComponentKind::Source
    );
    assert_eq!(
        manifest.components()[1].kind(),
        ConnectorComponentKind::Operator
    );
    assert_eq!(
        manifest.components()[2].kind(),
        ConnectorComponentKind::Endpoint
    );

    let session = Session::new();
    let installed = package.install(&session).expect("package installation");
    assert_eq!(installed.session_id(), session.id());

    let endpoint_component = ConnectorComponentId::new(ENDPOINT_COMPONENT).expect("component id");
    let endpoint = installed
        .endpoint(&endpoint_component)
        .expect("registered endpoint")
        .declare(&session, ConnectorConfiguration::new())
        .expect("endpoint declaration");
    let source = session
        .source(
            SourceTypeId::new(SOURCE_TYPE).expect("source type"),
            SourceConfiguration::default(),
        )
        .expect("source declaration");
    source
        .output("audio")
        .expect("audio output")
        .send(endpoint)
        .expect("source route");

    let mut running = session.start().expect("running Session");
    assert!(running.stop().is_success());
}

#[test]
fn given_duplicate_component_identity_when_composed_then_package_rejects_it_before_install() {
    let mut package = ConnectorPackage::new(
        1,
        ConnectorPackageId::new("dev.pocketstation.test.duplicate").expect("package id"),
        env!("CARGO_PKG_VERSION"),
    )
    .expect("package");
    let component = ConnectorComponentId::new("shared").expect("component id");
    package
        .add_source(component.clone(), source_factory())
        .expect("source component");
    assert!(package.add_operator(component, operator_factory()).is_err());
}

#[test]
fn given_existing_authority_conflict_when_package_installs_then_no_component_is_registered() {
    let session = Session::new();
    session
        .register_connector(endpoint_connector())
        .expect("existing endpoint registration");

    let mut conflicting = ConnectorPackage::new(
        1,
        ConnectorPackageId::new("dev.pocketstation.test.atomic-conflict").expect("package id"),
        env!("CARGO_PKG_VERSION"),
    )
    .expect("package");
    conflicting
        .add_source(
            ConnectorComponentId::new(SOURCE_COMPONENT).expect("component id"),
            source_factory(),
        )
        .expect("source component");
    conflicting
        .add_endpoint(
            ConnectorComponentId::new(ENDPOINT_COMPONENT).expect("component id"),
            endpoint_connector(),
        )
        .expect("endpoint component");
    assert!(conflicting.install(&session).is_err());

    let mut source_only = ConnectorPackage::new(
        1,
        ConnectorPackageId::new("dev.pocketstation.test.atomic-retry").expect("package id"),
        env!("CARGO_PKG_VERSION"),
    )
    .expect("package");
    source_only
        .add_source(
            ConnectorComponentId::new(SOURCE_COMPONENT).expect("component id"),
            source_factory(),
        )
        .expect("source component");
    source_only
        .install(&session)
        .expect("failed package must not leak source registration");
}
