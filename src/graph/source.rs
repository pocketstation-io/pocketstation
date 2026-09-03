use std::sync::Arc;

use crate::frame::SampleFormat;
use crate::graph::compile::{Compiler, RuntimePlanner};
use crate::graph::{
    AudioCaps, BinaryFormat, ChannelLayout, ConfigError, ExecutionPartition, ExecutionSafety,
    MediaCaps, Multiplicity, NodeConfig, NodeDefinition, NodeDescriptor, NodeRegistry, NodeTypeId,
    Pipeline, PortDirection, PortSpec, SignalSpec,
};

struct Definition {
    descriptor: NodeDescriptor,
}

impl NodeDefinition for Definition {
    fn descriptor(&self) -> NodeDescriptor {
        self.descriptor.clone()
    }

    fn validate_config(&self, _config: &NodeConfig) -> Result<(), ConfigError> {
        Ok(())
    }
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

#[test]
fn given_audio_and_typed_root_outputs_when_planned_then_each_branch_has_bounded_authority() {
    let mut registry = NodeRegistry::new();
    registry
        .register_definition(Arc::new(Definition {
            descriptor: NodeDescriptor {
                type_id: NodeTypeId::from("org.example.source-a.v1"),
                display_name: "test source root",
                inputs: Vec::new(),
                outputs: vec![
                    audio_port("audio", PortDirection::Output),
                    typed_port("signal", PortDirection::Output),
                ],
                execution: ExecutionPartition::BlockingWorker,
                safety: ExecutionSafety::AllocationAllowed,
                stateful: true,
            },
        }))
        .unwrap();
    registry
        .register_definition(Arc::new(Definition {
            descriptor: NodeDescriptor {
                type_id: NodeTypeId::from("org.example.audio-endpoint.v1"),
                display_name: "test audio endpoint",
                inputs: vec![audio_port("audio", PortDirection::Input)],
                outputs: Vec::new(),
                execution: ExecutionPartition::External,
                safety: ExecutionSafety::ExternalService,
                stateful: true,
            },
        }))
        .unwrap();
    registry
        .register_definition(Arc::new(Definition {
            descriptor: NodeDescriptor {
                type_id: NodeTypeId::from("org.example.typed-endpoint.v1"),
                display_name: "test typed endpoint",
                inputs: vec![typed_port("signal", PortDirection::Input)],
                outputs: Vec::new(),
                execution: ExecutionPartition::External,
                safety: ExecutionSafety::ExternalService,
                stateful: true,
            },
        }))
        .unwrap();

    let mut pipeline = Pipeline::new();
    let source = pipeline.add_node("org.example.source-a.v1", NodeConfig::new());
    let audio = pipeline.add_node("org.example.audio-endpoint.v1", NodeConfig::new());
    let typed = pipeline.add_node("org.example.typed-endpoint.v1", NodeConfig::new());
    let audio_edge = pipeline.connect(source.out("audio"), audio.in_("audio"));
    let typed_edge = pipeline.connect(source.out("signal"), typed.in_("signal"));
    let graph = Compiler::new()
        .compile(pipeline.into_spec(), &registry)
        .unwrap();
    let plan = RuntimePlanner::new().plan(&graph).unwrap();

    assert_eq!(plan.source_outputs.len(), 2);
    assert!(plan
        .source_outputs
        .iter()
        .all(|output| output.branch_edges.len() == 1));
    assert!(plan
        .memory_plan
        .edge_buffer(audio_edge)
        .is_some_and(|edge| edge.capacity_frames > 0));
    assert!(plan
        .typed_edge(typed_edge)
        .is_some_and(|edge| edge.capacity_signals > 0));
}
