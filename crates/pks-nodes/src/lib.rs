//! pocketstation-nodes — built-in `NodeFactory`s that populate a `NodeRegistry`
//! with real `RuntimeNode`s. `register_all` adds the graph crate's own builtins
//! (passthrough, gain) plus the DSP factories defined here so a compiled graph
//! can instantiate concrete nodes.

mod mix;
mod ml_nodes;
mod sink;
mod source;

use std::sync::Arc;

use pks_graph::{register_builtins, NodeRegistry};

pub use mix::{MonoMixFactory, MonoMixNode};
pub use ml_nodes::{EchoCancelFactory, NoiseSuppressFactory, VadFactory, WatermarkFactory};
pub use sink::{RecordingSinkFactory, RecordingSinkNode, RecordingTally};
pub use source::{SyntheticSourceFactory, SyntheticSourceNode};

pub fn register_all(registry: &mut NodeRegistry) {
    register_builtins(registry);
    registry.register(Arc::new(SyntheticSourceFactory));
    registry.register(Arc::new(MonoMixFactory));
    registry.register(Arc::new(VadFactory));
    registry.register(Arc::new(NoiseSuppressFactory));
    registry.register(Arc::new(EchoCancelFactory));
    registry.register(Arc::new(WatermarkFactory));
    let (recording_sink, _tally) = RecordingSinkFactory::new();
    registry.register(Arc::new(recording_sink));
}

#[cfg(test)]
mod tests {
    use super::*;
    use pks_frame::{AudioBufferPool, AudioFrame, SampleFormat, SampleSpec, SourceId, StreamId};
    use pks_graph::compiler::Compiler;
    use pks_graph::dsl::AudioGraph;
    use pks_graph::node::PrepareContext;
    use pks_graph::planner::RuntimePlanner;
    use pks_graph::{NodeConfig, NodeFactory, NodeRegistry, NodeTypeId};

    const FRAME_SAMPLES: usize = 960;

    fn prepare_cx() -> PrepareContext {
        PrepareContext::new(SampleSpec::new(48_000, 1, SampleFormat::F32Interleaved))
    }

    #[test]
    fn given_registry_when_register_all_then_contains_every_builtin_and_dsp_type() {
        let mut registry = NodeRegistry::new();
        register_all(&mut registry);
        for type_id in [
            "passthrough",
            "gain",
            "transform.vad",
            "transform.noise_suppress",
            "transform.echo_cancel",
            "transform.watermark",
            "transform.mono_mix",
        ] {
            assert!(
                registry.contains(&NodeTypeId::from(type_id)),
                "registry missing {type_id}"
            );
        }
    }

    #[test]
    fn given_graph_of_dsp_nodes_when_compiled_then_ok() {
        let mut registry = NodeRegistry::new();
        register_all(&mut registry);

        let mut graph = AudioGraph::new();
        let source = graph.add_node("passthrough", NodeConfig::new());
        let vad = graph.add_node("transform.vad", NodeConfig::new());
        let gain = graph.add_node("gain", NodeConfig::new().with("gain_db", "0.0"));
        graph.connect(source.out("out"), vad.in_("in"));
        graph.connect(vad.out("out"), gain.in_("in"));

        let ir = Compiler::new()
            .compile(graph.into_spec(), &registry)
            .unwrap();
        assert_eq!(ir.topo_order().len(), 3);

        let plan = RuntimePlanner::new().plan(&ir).unwrap();
        assert_eq!(plan.node_order.len(), 3);
    }

    #[test]
    fn given_vad_factory_when_instantiate_then_produces_working_runtime_node() {
        let cx = prepare_cx();
        let mut node = VadFactory.instantiate(&cx, &NodeConfig::new()).unwrap();
        node.prepare(&cx).unwrap();

        let pool = AudioBufferPool::new(1, FRAME_SAMPLES);
        let mut handle = pool.acquire().unwrap();
        handle.copy_from_slice(&vec![0.3f32; FRAME_SAMPLES]);
        let frame = AudioFrame::new(StreamId(0), SourceId(0), 0, 0, 1, handle);

        let out = node.process(frame).unwrap();
        assert!(out.is_some());
    }
}
