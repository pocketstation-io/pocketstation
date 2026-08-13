use std::sync::Arc;

use crate::graph::compile::{CompileError, Compiler};
use crate::graph::{
    ConfigError, ExecutionPartition, MediaCaps, Multiplicity, NodeConfig, NodeDefinition,
    NodeDescriptor, NodeRegistry, NodeTypeId, Pipeline, PortDirection, PortSpec, SafetyContract,
    SignalSpec, TextFormat,
};

struct Definition(NodeDescriptor);

impl NodeDefinition for Definition {
    fn descriptor(&self) -> NodeDescriptor {
        self.0.clone()
    }

    fn validate_config(&self, _config: &NodeConfig) -> Result<(), ConfigError> {
        Ok(())
    }
}

fn port(name: &str, direction: PortDirection) -> PortSpec {
    PortSpec {
        name: name.to_owned(),
        direction,
        signal: SignalSpec::text(TextFormat::Utf8),
        media: MediaCaps::Text,
        multiplicity: Multiplicity::One,
        required: true,
    }
}

fn definition(type_id: &str, inputs: &[&str], outputs: &[&str]) -> Arc<dyn NodeDefinition> {
    Arc::new(Definition(NodeDescriptor {
        type_id: NodeTypeId::from(type_id),
        display_name: "named-port test definition",
        inputs: inputs
            .iter()
            .map(|name| port(name, PortDirection::Input))
            .collect(),
        outputs: outputs
            .iter()
            .map(|name| port(name, PortDirection::Output))
            .collect(),
        execution: ExecutionPartition::AsyncWorker,
        safety: SafetyContract::ExternalService,
        stateful: false,
    }))
}

fn registry() -> NodeRegistry {
    let mut registry = NodeRegistry::new();
    registry
        .register_definition(definition("source.context", &[], &["context"]))
        .expect("context source");
    registry
        .register_definition(definition("source.prompt", &[], &["prompt"]))
        .expect("prompt source");
    registry
        .register_definition(definition(
            "operator.named",
            &["context", "prompt"],
            &["answer", "diagnostics"],
        ))
        .expect("named operator");
    registry
}

#[test]
fn given_exact_named_inputs_when_compiled_then_one_multi_port_node_is_preserved() {
    let registry = registry();
    let mut pipeline = Pipeline::new();
    let context = pipeline.add_node("source.context", NodeConfig::new());
    let prompt = pipeline.add_node("source.prompt", NodeConfig::new());
    let operator = pipeline.add_node("operator.named", NodeConfig::new());
    pipeline.connect(context.out("context"), operator.in_("context"));
    pipeline.connect(prompt.out("prompt"), operator.in_("prompt"));

    let ir = Compiler::new()
        .compile(pipeline.into_spec(), &registry)
        .expect("named graph");

    assert_eq!(ir.node_count(), 3);
    assert_eq!(ir.edge_count(), 2);
    assert_eq!(ir.edges[0].spec.to.port, "context");
    assert_eq!(ir.edges[1].spec.to.port, "prompt");
}

#[test]
fn given_unknown_named_input_when_compiled_then_failure_precedes_runtime() {
    let registry = registry();
    let mut pipeline = Pipeline::new();
    let context = pipeline.add_node("source.context", NodeConfig::new());
    let operator = pipeline.add_node("operator.named", NodeConfig::new());
    pipeline.connect(context.out("context"), operator.in_("missing"));

    let result = Compiler::new().compile(pipeline.into_spec(), &registry);

    assert!(matches!(result, Err(CompileError::UnknownPort { port, .. }) if port == "missing"));
}
