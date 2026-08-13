use std::sync::Arc;

use crate::endpoint::{
    EndpointDriverFactory, EndpointFailure, EndpointFailureStage, EndpointPortInput,
    PreparedEndpointDriver,
};
use crate::frame::{SampleFormat, SampleSpec};
use crate::graph::{
    AsyncNode, AsyncOperatorFactory, AsyncOperatorManifest, AudioCaps, BackpressurePolicy,
    BinaryFormat, ChannelLayout, ConfigError, CopyPolicy, EdgeContract, ExecutionPartition,
    MediaCaps, Multiplicity, NodeConfig, NodeDefinition, NodeDescriptor, NodeError, NodeTypeId,
    OperatorCancellationPolicy, OperatorDeadlinePolicy, OperatorFailurePolicy,
    OperatorOutputRolePolicy, OperatorPermissionPolicy, PortDirection, PortSpec, PrepareContext,
    SafetyContract, SemanticRole, SignalSpec, TextFormat,
};
use crate::session::{
    DeviceSelector, EndpointDescriptor, Operator, OperatorConfiguration, OperatorId, Session,
    SessionCompileError, SessionEngine, SessionEngineBuilder, SessionStartOptions, Source,
    SourceCancellation, SourceConfiguration, SourceDriver, SourceDriverError, SourceEmission,
    SourceFactory, SourceManifest, SourcePrepareContext, SourceRegistrationError, SourceTypeId,
};

const SOURCE_TYPE_ID: &str = "org.example.source-a.v1";
const TYPED_ENDPOINT_NODE_TYPE_ID: &str = "org.example.endpoint.typed.v1";
const TYPED_ENDPOINT_OPERATOR_ID: &str = "org.example.endpoint.typed-driver.v1";
const AUDIO_ENDPOINT_NODE_TYPE_ID: &str = "org.example.endpoint.audio.v1";
const AUDIO_ENDPOINT_OPERATOR_ID: &str = "org.example.endpoint.audio-driver.v1";
const TEXT_ENDPOINT_NODE_TYPE_ID: &str = "org.example.endpoint.text.v1";
const TEXT_ENDPOINT_OPERATOR_ID: &str = "org.example.endpoint.text-driver.v1";
const OPERATOR_NODE_TYPE_ID: &str = "org.example.operator-a.v1";
const OPERATOR_ID: &str = "org.example.operator-a-driver.v1";

struct CompileOnlySourceFactory {
    manifest: SourceManifest,
}

struct CompileOnlySourceDriver;

impl SourceDriver for CompileOnlySourceDriver {
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

impl SourceFactory for CompileOnlySourceFactory {
    fn manifest(&self) -> &SourceManifest {
        &self.manifest
    }

    fn validate_config(&self, configuration: &SourceConfiguration) -> Result<(), ConfigError> {
        match configuration.get("mode") {
            Some("accepted") => Ok(()),
            _ => Err(ConfigError::Missing("mode=accepted".to_owned())),
        }
    }

    fn create(
        &self,
        _configuration: &SourceConfiguration,
    ) -> Result<Box<dyn SourceDriver>, SourceDriverError> {
        Ok(Box::new(CompileOnlySourceDriver))
    }
}

struct CompileOnlyEndpointDefinition {
    descriptor: NodeDescriptor,
}

impl NodeDefinition for CompileOnlyEndpointDefinition {
    fn descriptor(&self) -> NodeDescriptor {
        self.descriptor.clone()
    }

    fn validate_config(&self, _config: &NodeConfig) -> Result<(), ConfigError> {
        Ok(())
    }
}

struct CompileOnlyEndpointFactory;

impl EndpointDriverFactory for CompileOnlyEndpointFactory {
    fn prepare(
        &self,
        _inputs: Vec<EndpointPortInput>,
    ) -> Result<Box<dyn PreparedEndpointDriver>, EndpointFailure> {
        Err(EndpointFailure::new(
            EndpointFailureStage::Prepare,
            "compile-only endpoint must not prepare",
        ))
    }
}

struct CompileOnlyOperatorFactory {
    manifest: AsyncOperatorManifest,
}

impl AsyncOperatorFactory for CompileOnlyOperatorFactory {
    fn manifest(&self) -> &AsyncOperatorManifest {
        &self.manifest
    }

    fn validate_config(&self, _configuration: &NodeConfig) -> Result<(), ConfigError> {
        Ok(())
    }

    fn create(&self, _configuration: &NodeConfig) -> Result<Box<dyn AsyncNode>, NodeError> {
        Err(NodeError::Prepare(
            "compile-only operator must not create".to_owned(),
        ))
    }
}

fn source_type_id() -> SourceTypeId {
    SourceTypeId::new(SOURCE_TYPE_ID).unwrap()
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
        multiplicity: Multiplicity::Many,
        required: true,
    }
}

fn typed_port(name: &str, direction: PortDirection) -> PortSpec {
    PortSpec {
        name: name.to_owned(),
        direction,
        signal: SignalSpec::custom("org.example.signal-a.v1")
            .with_schema("urn:example:signal-a:v1"),
        media: MediaCaps::Binary(BinaryFormat::Raw),
        multiplicity: Multiplicity::Many,
        required: true,
    }
}

fn text_port(name: &str, direction: PortDirection) -> PortSpec {
    PortSpec {
        name: name.to_owned(),
        direction,
        signal: SignalSpec::text(TextFormat::Utf8).with_role("org.example.output.v1"),
        media: MediaCaps::Text,
        multiplicity: Multiplicity::Many,
        required: true,
    }
}

fn source_factory() -> Arc<dyn SourceFactory> {
    Arc::new(CompileOnlySourceFactory {
        manifest: SourceManifest {
            source_type_id: source_type_id(),
            revision: 3,
            generation: 2,
            outputs: vec![
                typed_port("signal", PortDirection::Output),
                audio_port("audio", PortDirection::Output),
            ],
            execution: ExecutionPartition::BlockingWorker,
            safety: SafetyContract::AllocationAllowed,
        },
    })
}

fn endpoint_descriptor(
    node_type_id: &'static str,
    input: PortSpec,
) -> (Arc<dyn NodeDefinition>, Arc<dyn EndpointDriverFactory>) {
    (
        Arc::new(CompileOnlyEndpointDefinition {
            descriptor: NodeDescriptor {
                type_id: NodeTypeId::from(node_type_id),
                display_name: "compile-only endpoint",
                inputs: vec![input],
                outputs: Vec::new(),
                execution: ExecutionPartition::External,
                safety: SafetyContract::ExternalService,
                stateful: true,
            },
        }),
        Arc::new(CompileOnlyEndpointFactory),
    )
}

fn operator_factory() -> Arc<dyn AsyncOperatorFactory> {
    let mut input_edge = EdgeContract::bounded_async();
    input_edge.media = MediaCaps::Binary(BinaryFormat::Raw);
    input_edge.backpressure = BackpressurePolicy::DropNewest;
    input_edge.copy_policy = CopyPolicy::CopyToBranchPool;
    let mut output_edge = EdgeContract::bounded_async();
    output_edge.media = MediaCaps::Text;
    Arc::new(CompileOnlyOperatorFactory {
        manifest: AsyncOperatorManifest {
            operator_id: OperatorId::new(OPERATOR_ID),
            revision: 1,
            generation: 1,
            node: NodeDescriptor {
                type_id: NodeTypeId::from(OPERATOR_NODE_TYPE_ID),
                display_name: "compile-only operator",
                inputs: vec![typed_port("signal", PortDirection::Input)],
                outputs: vec![text_port("text", PortDirection::Output)],
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
                allowed: vec![SemanticRole::new("org.example.output.v1")],
                terminal: vec![SemanticRole::new("org.example.output.v1")],
            },
        },
    })
}

fn builder(register_source: bool) -> SessionEngineBuilder {
    let mut builder = SessionEngineBuilder::new(
        PrepareContext::new(SampleSpec::new(48_000, 1, SampleFormat::F32Interleaved)),
        8,
        SessionStartOptions::default(),
    )
    .unwrap();
    if register_source {
        builder.register_source_factory(source_factory()).unwrap();
    }
    for (node_type_id, operator_id, port) in [
        (
            TYPED_ENDPOINT_NODE_TYPE_ID,
            TYPED_ENDPOINT_OPERATOR_ID,
            typed_port("signal", PortDirection::Input),
        ),
        (
            AUDIO_ENDPOINT_NODE_TYPE_ID,
            AUDIO_ENDPOINT_OPERATOR_ID,
            audio_port("audio", PortDirection::Input),
        ),
        (
            TEXT_ENDPOINT_NODE_TYPE_ID,
            TEXT_ENDPOINT_OPERATOR_ID,
            text_port("text", PortDirection::Input),
        ),
    ] {
        let (definition, factory) = endpoint_descriptor(node_type_id, port);
        builder
            .register_endpoint(OperatorId::new(operator_id), definition, factory)
            .unwrap();
    }
    builder
}

fn configuration() -> SourceConfiguration {
    let mut configuration = SourceConfiguration::default();
    configuration.insert("mode", "accepted");
    configuration
}

fn endpoint(
    session: &Session,
    node_type_id: &'static str,
    operator_id: &'static str,
) -> crate::session::EndpointHandle {
    session
        .endpoint(EndpointDescriptor::new(
            NodeTypeId::from(node_type_id),
            OperatorId::new(operator_id),
        ))
        .unwrap()
}

fn compile(engine: &SessionEngine, session: Session) -> crate::session::CompiledSession {
    engine.compile(session).unwrap()
}

#[test]
fn given_external_custom_output_when_compiled_then_session_identity_and_typed_plan_are_preserved() {
    let engine = builder(true).build().unwrap();
    let session = Session::new();
    let source = session.source(source_type_id(), configuration()).unwrap();
    let first = source.output("signal").unwrap();
    let same = source.output("signal").unwrap();
    assert_eq!(first.stream_id(), same.stream_id());
    first
        .send(endpoint(
            &session,
            TYPED_ENDPOINT_NODE_TYPE_ID,
            TYPED_ENDPOINT_OPERATOR_ID,
        ))
        .unwrap();

    let compiled = compile(&engine, session);

    assert!(compiled.source_declarations().is_empty());
    assert_eq!(compiled.external_source_declarations().len(), 1);
    assert_eq!(
        compiled.spec().source_outputs()[0].stream_id(),
        first.stream_id()
    );
    assert_eq!(
        compiled.external_source_declarations()[0].source_id(),
        source.source_id()
    );
    assert_eq!(compiled.planned_source_output_count(), 1);
    assert_eq!(compiled.planned_typed_edge_count(), 1);
    assert_eq!(compiled.planned_audio_edge_count(), 0);
}

#[test]
fn given_external_pcm_output_when_compiled_then_bounded_audio_edge_is_planned() {
    let engine = builder(true).build().unwrap();
    let session = Session::new();
    let source = session.source(source_type_id(), configuration()).unwrap();
    source
        .output("audio")
        .unwrap()
        .send(endpoint(
            &session,
            AUDIO_ENDPOINT_NODE_TYPE_ID,
            AUDIO_ENDPOINT_OPERATOR_ID,
        ))
        .unwrap();

    let compiled = compile(&engine, session);

    assert_eq!(compiled.planned_source_output_count(), 1);
    assert_eq!(compiled.planned_typed_edge_count(), 0);
    assert_eq!(compiled.planned_audio_edge_count(), 1);
}

#[test]
fn given_external_output_through_operator_when_compiled_then_normal_typed_edges_are_used() {
    let mut builder = builder(true);
    builder.register_async_operator(operator_factory()).unwrap();
    let engine = builder.build().unwrap();
    let session = Session::new();
    let source = session.source(source_type_id(), configuration()).unwrap();
    let transformed = source
        .output("signal")
        .unwrap()
        .through(Operator::new(
            OperatorId::new(OPERATOR_ID),
            OperatorConfiguration::new(),
        ))
        .unwrap();
    transformed
        .send(endpoint(
            &session,
            TEXT_ENDPOINT_NODE_TYPE_ID,
            TEXT_ENDPOINT_OPERATOR_ID,
        ))
        .unwrap();

    let compiled = compile(&engine, session);

    assert_eq!(compiled.node_count(), 3);
    assert_eq!(compiled.planned_source_output_count(), 1);
    assert_eq!(compiled.planned_typed_edge_count(), 2);
}

#[test]
fn given_unknown_manifest_output_when_compiled_then_typed_error_is_returned() {
    let engine = builder(true).build().unwrap();
    let session = Session::new();
    session
        .source(source_type_id(), configuration())
        .unwrap()
        .output("missing")
        .unwrap()
        .send(endpoint(
            &session,
            TYPED_ENDPOINT_NODE_TYPE_ID,
            TYPED_ENDPOINT_OPERATOR_ID,
        ))
        .unwrap();

    let error = engine.compile(session).err().unwrap();

    assert!(matches!(
        error,
        crate::session::SessionEngineStartError::Compile(
            SessionCompileError::UnknownExternalSourceOutput { .. }
        )
    ));
}

#[test]
fn given_unregistered_external_source_when_compiled_then_registry_error_is_typed() {
    let engine = builder(false).build().unwrap();
    let session = Session::new();
    session
        .source(source_type_id(), configuration())
        .unwrap()
        .output("signal")
        .unwrap()
        .send(endpoint(
            &session,
            TYPED_ENDPOINT_NODE_TYPE_ID,
            TYPED_ENDPOINT_OPERATOR_ID,
        ))
        .unwrap();

    let error = engine.compile(session).err().unwrap();

    assert!(matches!(
        error,
        crate::session::SessionEngineStartError::Compile(
            SessionCompileError::UnknownExternalSource { .. }
        )
    ));
}

#[test]
fn given_builtin_microphone_when_compiled_after_extension_then_existing_stem_path_is_unchanged() {
    let engine = builder(true).build().unwrap();
    let session = Session::new();
    session
        .capture(Source::microphone(DeviceSelector::default()))
        .unwrap()
        .send(endpoint(
            &session,
            AUDIO_ENDPOINT_NODE_TYPE_ID,
            AUDIO_ENDPOINT_OPERATOR_ID,
        ))
        .unwrap();

    let compiled = compile(&engine, session);

    assert_eq!(compiled.source_declarations().len(), 1);
    assert!(compiled.external_source_declarations().is_empty());
    assert_eq!(compiled.planned_audio_edge_count(), 1);
}

#[test]
fn given_source_type_conflicting_with_builtin_node_when_registered_then_conflict_is_typed() {
    let mut builder = builder(false);
    let conflict = Arc::new(CompileOnlySourceFactory {
        manifest: SourceManifest {
            source_type_id: SourceTypeId::new(crate::session::MICROPHONE_SOURCE_NODE_TYPE_ID)
                .unwrap(),
            revision: 1,
            generation: 1,
            outputs: vec![audio_port("audio", PortDirection::Output)],
            execution: ExecutionPartition::BlockingWorker,
            safety: SafetyContract::AllocationAllowed,
        },
    });

    let error = builder.register_source_factory(conflict).err().unwrap();

    assert!(matches!(
        error,
        SourceRegistrationError::NodeTypeConflict(_)
    ));
}
