use std::collections::HashMap;

use pks_graph::compiler::{CompileError, Compiler};
use pks_graph::ir::GraphIr;
use pks_graph::planner::RuntimePlanner;
use pks_graph::{NodeConfig, NodeRegistry, NodeTypeId, Pipeline};

use crate::{
    ApplicationSelector, DeviceSelector, EndpointSpec, OperatorId, SessionError, SessionId,
    SessionSpec, Source, StemSpec,
};

pub const APPLICATION_SOURCE_NODE_TYPE_ID: &str = "source.application";
pub const MICROPHONE_SOURCE_NODE_TYPE_ID: &str = "source.microphone";
pub const CONNECTOR_NODE_TYPE_ID: &str = "endpoint.connector.external";
pub const BROWSER_NODE_TYPE_ID: &str = "endpoint.browser.remote";
pub const BROWSER_OPERATOR_ID: &str = "io.pocketstation.browser.webrtc.v1";
pub const RECORDER_NODE_TYPE_ID: &str = "endpoint.recording.multistem";
pub const RECORDER_OPERATOR_ID: &str = "io.pocketstation.recording.wav-stems.v1";
pub const RECORDING_GROUP_CONFIGURATION_KEY: &str = "recording_group_id";
pub const DEFAULT_MULTISTEM_RECORDING_GROUP_ID: &str = "session.multistem.default.v1";
const AUDIO_OUTPUT_PORT: &str = "audio";
const AUDIO_INPUT_PORT: &str = "audio";
const RESERVED_ENDPOINT_CONFIGURATION_KEYS: [&str; 6] = [
    "session_id",
    "stem_id",
    "endpoint_id",
    "route_id",
    "operator_id",
    "connector_id",
];

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum OperatorRegistryError {
    #[error("operator id cannot be empty")]
    EmptyOperatorId,
    #[error("node type id cannot be empty")]
    EmptyNodeTypeId,
    #[error("operator {operator_id} is already registered")]
    DuplicateOperator { operator_id: String },
}

#[derive(Default)]
pub struct OperatorRegistry {
    bindings: HashMap<OperatorId, NodeTypeId>,
}

impl OperatorRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(
        &mut self,
        operator_id: OperatorId,
        node_type_id: NodeTypeId,
    ) -> Result<(), OperatorRegistryError> {
        if operator_id.as_str().trim().is_empty() {
            return Err(OperatorRegistryError::EmptyOperatorId);
        }
        if node_type_id.as_str().trim().is_empty() {
            return Err(OperatorRegistryError::EmptyNodeTypeId);
        }
        if self.bindings.contains_key(&operator_id) {
            return Err(OperatorRegistryError::DuplicateOperator {
                operator_id: operator_id.as_str().to_owned(),
            });
        }
        self.bindings.insert(operator_id, node_type_id);
        Ok(())
    }

    pub fn node_type_id(&self, operator_id: &OperatorId) -> Option<&NodeTypeId> {
        self.bindings.get(operator_id)
    }

    pub fn contains(&self, operator_id: &OperatorId) -> bool {
        self.bindings.contains_key(operator_id)
    }

    pub fn len(&self) -> usize {
        self.bindings.len()
    }

    pub fn is_empty(&self) -> bool {
        self.bindings.is_empty()
    }
}

#[derive(Debug, thiserror::Error)]
pub enum SessionCompileError {
    #[error(transparent)]
    InvalidSpec(#[from] SessionError),
    #[error("operator {operator_id} is not registered")]
    UnknownOperator { operator_id: String },
    #[error(
        "operator {operator_id} resolves to node type {registered_node_type_id}, not {declared_node_type_id}"
    )]
    OperatorNodeTypeMismatch {
        operator_id: String,
        registered_node_type_id: String,
        declared_node_type_id: String,
    },
    #[error("endpoint {endpoint_id:?} configuration uses reserved key {key}")]
    ReservedEndpointConfigurationKey {
        endpoint_id: crate::EndpointId,
        key: String,
    },
    #[error("required source node type {node_type_id} is not registered")]
    UnknownSourceNodeType { node_type_id: String },
    #[error(transparent)]
    GraphCompile(#[from] CompileError),
    #[error(transparent)]
    RuntimePlan(#[from] pks_graph::plan::PlanError),
}

pub struct CompiledSession {
    spec: SessionSpec,
    graph_ir: GraphIr,
    runtime_plan: pks_graph::plan::RuntimePlan,
}

impl CompiledSession {
    pub const fn session_id(&self) -> SessionId {
        self.spec.session_id()
    }

    pub fn spec(&self) -> &SessionSpec {
        &self.spec
    }

    pub fn source_declarations(&self) -> &[StemSpec] {
        self.spec.stems()
    }

    pub fn endpoint_declarations(&self) -> &[EndpointSpec] {
        self.spec.endpoints()
    }

    pub fn node_count(&self) -> usize {
        self.graph_ir.node_count()
    }

    pub fn edge_count(&self) -> usize {
        self.graph_ir.edge_count()
    }

    pub fn planned_edge_count(&self) -> usize {
        self.runtime_plan.edge_count
    }

    pub(crate) fn into_runtime_parts(self) -> (SessionSpec, GraphIr, pks_graph::plan::RuntimePlan) {
        (self.spec, self.graph_ir, self.runtime_plan)
    }

    #[cfg(test)]
    pub(crate) fn graph_ir_mut(&mut self) -> &mut GraphIr {
        &mut self.graph_ir
    }
}

pub struct SessionCompiler<'registry> {
    node_registry: &'registry NodeRegistry,
    operator_registry: &'registry OperatorRegistry,
}

impl<'registry> SessionCompiler<'registry> {
    pub const fn new(
        node_registry: &'registry NodeRegistry,
        operator_registry: &'registry OperatorRegistry,
    ) -> Self {
        Self {
            node_registry,
            operator_registry,
        }
    }

    pub fn compile(&self, spec: SessionSpec) -> Result<CompiledSession, SessionCompileError> {
        spec.validate()?;
        self.validate_source_node_types(&spec)?;
        self.validate_endpoint_operators(&spec)?;

        let graph_spec = self.lower_graph_spec(&spec)?;
        let graph_ir = Compiler::new().compile(graph_spec, self.node_registry)?;
        let runtime_plan = RuntimePlanner::new().plan(&graph_ir)?;

        Ok(CompiledSession {
            spec,
            graph_ir,
            runtime_plan,
        })
    }

    fn validate_source_node_types(&self, spec: &SessionSpec) -> Result<(), SessionCompileError> {
        for stem in spec.stems() {
            let node_type_id = source_node_type_id(stem.source());
            if !self.node_registry.contains(&node_type_id) {
                return Err(SessionCompileError::UnknownSourceNodeType {
                    node_type_id: node_type_id.as_str().to_owned(),
                });
            }
        }
        Ok(())
    }

    fn validate_endpoint_operators(&self, spec: &SessionSpec) -> Result<(), SessionCompileError> {
        for endpoint in spec.endpoints() {
            if let Some((key, _)) = endpoint
                .configuration()
                .iter()
                .find(|(key, _)| RESERVED_ENDPOINT_CONFIGURATION_KEYS.contains(key))
            {
                return Err(SessionCompileError::ReservedEndpointConfigurationKey {
                    endpoint_id: endpoint.id(),
                    key: key.to_owned(),
                });
            }
            let Some(registered_node_type_id) =
                self.operator_registry.node_type_id(endpoint.operator_id())
            else {
                return Err(SessionCompileError::UnknownOperator {
                    operator_id: endpoint.operator_id().as_str().to_owned(),
                });
            };
            if registered_node_type_id != endpoint.node_type_id() {
                return Err(SessionCompileError::OperatorNodeTypeMismatch {
                    operator_id: endpoint.operator_id().as_str().to_owned(),
                    registered_node_type_id: registered_node_type_id.as_str().to_owned(),
                    declared_node_type_id: endpoint.node_type_id().as_str().to_owned(),
                });
            }
        }
        Ok(())
    }

    fn lower_graph_spec(
        &self,
        spec: &SessionSpec,
    ) -> Result<pks_graph::GraphSpec, SessionCompileError> {
        let mut pipeline = Pipeline::new();
        let mut source_nodes = HashMap::with_capacity(spec.stems().len());

        for stem in spec.stems() {
            let source_node = pipeline.add_node(
                source_node_type_id(stem.source()),
                source_node_config(spec.session_id(), stem),
            );
            source_nodes.insert(stem.id(), source_node);
        }

        for route in spec.routes() {
            let stem = spec
                .stems()
                .iter()
                .find(|stem| stem.id() == route.stem_id())
                .ok_or(SessionError::UnknownStem {
                    stem_id: route.stem_id(),
                })?;
            let endpoint = spec
                .endpoints()
                .iter()
                .find(|endpoint| endpoint.id() == route.endpoint_id())
                .ok_or(SessionError::UnknownEndpoint {
                    endpoint_id: route.endpoint_id(),
                })?;
            let source_node = source_nodes
                .get(&stem.id())
                .ok_or(SessionError::UnknownStem { stem_id: stem.id() })?;
            let endpoint_node = pipeline.add_node(
                endpoint.node_type_id().clone(),
                endpoint_node_config(spec.session_id(), stem, endpoint, route.id()),
            );
            pipeline.connect(
                source_node.out(AUDIO_OUTPUT_PORT),
                endpoint_node.in_(AUDIO_INPUT_PORT),
            );
        }
        Ok(pipeline.into_spec())
    }
}

fn source_node_type_id(source: &Source) -> NodeTypeId {
    match source {
        Source::Application(_) => NodeTypeId::from(APPLICATION_SOURCE_NODE_TYPE_ID),
        Source::Microphone(_) => NodeTypeId::from(MICROPHONE_SOURCE_NODE_TYPE_ID),
    }
}

fn source_node_config(session_id: SessionId, stem: &StemSpec) -> NodeConfig {
    let mut config = NodeConfig::new()
        .with("session_id", &session_id.0.to_string())
        .with("stem_id", &stem.id().0.to_string());
    match stem.source() {
        Source::Application(ApplicationSelector::BundleId(bundle_id)) => {
            config = config
                .with("selector_kind", "bundle_id")
                .with("selector_value", bundle_id);
        }
        Source::Application(ApplicationSelector::ProcessId(process_id)) => {
            config = config
                .with("selector_kind", "process_id")
                .with("selector_value", &process_id.get().to_string());
        }
        Source::Application(ApplicationSelector::ProcessInstance {
            process_id,
            stable_id,
        }) => {
            config = config
                .with("selector_kind", "process_instance")
                .with("selector_process_id", &process_id.get().to_string())
                .with("selector_stable_id", &stable_id.stable_key);
        }
        Source::Application(ApplicationSelector::StableId(source_id)) => {
            config = config
                .with("selector_kind", "stable_id")
                .with("selector_value", &source_id.stable_key);
        }
        Source::Application(ApplicationSelector::Name(name)) => {
            config = config
                .with("selector_kind", "name")
                .with("selector_value", name);
        }
        Source::Microphone(DeviceSelector::Default) => {
            config = config.with("selector_kind", "default");
        }
        Source::Microphone(DeviceSelector::Id(device_id)) => {
            config = config
                .with("selector_kind", "device_id")
                .with("selector_value", device_id.as_str());
        }
    }
    config
}

pub(crate) fn endpoint_node_config(
    session_id: SessionId,
    stem: &StemSpec,
    endpoint: &EndpointSpec,
    route_id: crate::RouteId,
) -> NodeConfig {
    let mut config = NodeConfig::new()
        .with("session_id", &session_id.0.to_string())
        .with("stem_id", &stem.id().0.to_string())
        .with("endpoint_id", &endpoint.id().0.to_string())
        .with("route_id", &route_id.0.to_string())
        .with("operator_id", endpoint.operator_id().as_str());
    if let Some(connector_id) = endpoint.connector_id() {
        config = config.with("connector_id", &connector_id.0.to_string());
    }
    for (key, value) in endpoint.configuration().iter() {
        config = config.with(key, value);
    }
    config
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{register_session_structural_nodes, EndpointConfiguration, Session};

    const TEST_CONNECTOR_OPERATOR_ID: &str = "example.connector.streaming-stt.v1";

    fn registries() -> (NodeRegistry, OperatorRegistry) {
        let mut node_registry = NodeRegistry::new();
        register_session_structural_nodes(&mut node_registry)
            .expect("Session structural registrations");

        let mut operator_registry = OperatorRegistry::new();
        operator_registry
            .register(
                OperatorId::new(TEST_CONNECTOR_OPERATOR_ID),
                NodeTypeId::from(CONNECTOR_NODE_TYPE_ID),
            )
            .expect("connector registration");
        operator_registry
            .register(
                OperatorId::new(BROWSER_OPERATOR_ID),
                NodeTypeId::from(BROWSER_NODE_TYPE_ID),
            )
            .expect("browser registration");
        operator_registry
            .register(
                OperatorId::new(RECORDER_OPERATOR_ID),
                NodeTypeId::from(RECORDER_NODE_TYPE_ID),
            )
            .expect("recorder registration");
        (node_registry, operator_registry)
    }

    fn product_spec() -> SessionSpec {
        let session = Session::new();
        let application = session
            .capture(Source::application(ApplicationSelector::name(
                "Meeting App",
            )))
            .expect("application declaration");
        let microphone = session
            .capture(Source::microphone_default())
            .expect("microphone declaration");
        let connector = session
            .connector(
                OperatorId::new(TEST_CONNECTOR_OPERATOR_ID),
                EndpointConfiguration::new().with("model", "test-only"),
            )
            .expect("connector declaration");
        let browser = session
            .browser("wss://receiver.example.test")
            .expect("browser declaration");

        application
            .send(connector)
            .expect("application connector route");
        application
            .send(browser)
            .expect("application browser route");
        application
            .record("application")
            .expect("application recorder route");
        microphone
            .send(connector)
            .expect("microphone connector route");
        microphone.send(browser).expect("microphone browser route");
        microphone
            .record("microphone")
            .expect("microphone recorder route");
        session.freeze().expect("product spec")
    }

    #[test]
    fn given_product_spec_when_compiled_then_six_independent_edges_are_planned() {
        let spec = product_spec();
        let (node_registry, operator_registry) = registries();

        let compiled = SessionCompiler::new(&node_registry, &operator_registry)
            .compile(spec)
            .expect("compiled product Session");

        assert_eq!(compiled.source_declarations().len(), 2);
        assert_eq!(compiled.endpoint_declarations().len(), 4);
        assert_eq!(compiled.node_count(), 8);
        assert_eq!(compiled.edge_count(), 6);
        assert_eq!(compiled.planned_edge_count(), 6);
    }

    #[test]
    fn given_unregistered_operator_when_compiled_then_typed_error_is_returned() {
        let spec = product_spec();
        let (node_registry, mut operator_registry) = registries();
        operator_registry
            .bindings
            .remove(&OperatorId::new(TEST_CONNECTOR_OPERATOR_ID));

        let result = SessionCompiler::new(&node_registry, &operator_registry).compile(spec);

        assert!(matches!(
            result,
            Err(SessionCompileError::UnknownOperator { .. })
        ));
    }

    #[test]
    fn given_operator_node_mismatch_when_compiled_then_typed_error_is_returned() {
        let spec = product_spec();
        let (node_registry, mut operator_registry) = registries();
        operator_registry.bindings.insert(
            OperatorId::new(TEST_CONNECTOR_OPERATOR_ID),
            NodeTypeId::from(BROWSER_NODE_TYPE_ID),
        );

        let result = SessionCompiler::new(&node_registry, &operator_registry).compile(spec);

        assert!(matches!(
            result,
            Err(SessionCompileError::OperatorNodeTypeMismatch { .. })
        ));
    }

    #[test]
    fn given_reserved_endpoint_key_when_compiled_then_typed_error_is_returned() {
        let session = Session::new();
        let microphone = session
            .capture(Source::microphone_default())
            .expect("microphone declaration");
        let connector = session
            .connector(
                OperatorId::new(TEST_CONNECTOR_OPERATOR_ID),
                EndpointConfiguration::new().with("session_id", "forged"),
            )
            .expect("connector declaration");
        microphone.send(connector).expect("connector route");
        let spec = session.freeze().expect("valid declaration");
        let (node_registry, operator_registry) = registries();

        let result = SessionCompiler::new(&node_registry, &operator_registry).compile(spec);

        assert!(matches!(
            result,
            Err(SessionCompileError::ReservedEndpointConfigurationKey { .. })
        ));
    }

    #[test]
    fn given_exact_process_instance_when_lowered_then_pid_and_stable_id_are_preserved() {
        let stable_id = pks_capture::StableSourceId::new(
            pks_frame::Platform::Windows,
            pks_capture::SourceKind::Application,
            "wasapi:pid:42:creation-100ns:133801234567890000",
        );
        let session = Session::new();
        let stem = session
            .capture(Source::application(ApplicationSelector::process_instance(
                crate::ProcessId::new(42),
                stable_id,
            )))
            .expect("exact process-instance declaration");
        let browser = session
            .browser("wss://receiver.example.test")
            .expect("browser declaration");
        stem.send(browser).expect("browser route");
        let spec = session.freeze().expect("exact process-instance spec");
        let config = source_node_config(spec.session_id(), &spec.stems()[0]);

        assert_eq!(config.get("selector_kind"), Some("process_instance"));
        assert_eq!(config.get("selector_process_id"), Some("42"));
        assert_eq!(
            config.get("selector_stable_id"),
            Some("wasapi:pid:42:creation-100ns:133801234567890000")
        );
        assert_eq!(config.get("selector_value"), None);
    }
}
