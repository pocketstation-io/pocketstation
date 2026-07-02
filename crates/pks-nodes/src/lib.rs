//! pocketstation-nodes — built-in `NodeFactory`s that populate a `NodeRegistry`
//! with real `RuntimeNode`s. `register_all` adds the graph crate's own builtins
//! (passthrough, gain) plus the DSP factories defined here so a compiled graph
//! can instantiate concrete nodes.

mod bridge_sink;
mod mic_source;
mod mix;
mod ml_nodes;
mod sink;
mod source;
mod system_source;

use std::sync::Arc;

use pks_graph::{register_builtins, NodeRegistry};

pub use bridge_sink::{BridgeSinkFactory, BridgeSinkNode, BridgeSinkTelemetry};
pub use mic_source::{MicSourceFactory, MicSourceNode, MicTelemetry};
pub use mix::{MonoMixFactory, MonoMixNode};
pub use ml_nodes::{EchoCancelFactory, NoiseSuppressFactory, VadFactory, WatermarkFactory};
pub use sink::{RecordingSinkFactory, RecordingSinkNode, RecordingTally};
pub use source::{SyntheticSourceFactory, SyntheticSourceNode};
pub use system_source::{SystemOutputSourceFactory, SystemOutputSourceNode, SystemOutputTelemetry};

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

    #[test]
    fn given_mic_source_factory_when_registered_then_registry_contains_source_mic() {
        let mut registry = NodeRegistry::new();
        register_all(&mut registry);

        let (mic_factory, _producer, _telemetry) = MicSourceFactory::with_default_capacity();
        registry.register(Arc::new(mic_factory));

        assert!(
            registry.contains(&NodeTypeId::from("source.mic")),
            "registry missing source.mic"
        );
    }

    #[test]
    fn given_source_and_bridge_in_executor_when_frames_pushed_then_consumer_receives_processed_output(
    ) {
        let cx = prepare_cx();

        // Wire: MicSourceFactory -> GainNode (0dB) -> BridgeSinkNode
        let (mic_factory, mut mic_producer, mic_telemetry) = MicSourceFactory::new(8);
        let mut mic_node = mic_factory.instantiate(&cx, &NodeConfig::new()).unwrap();
        mic_node.prepare(&cx).unwrap();

        let gain_config = NodeConfig::new().with("gain_db", "0.0");
        let gain_factory = pks_graph::builtins::GainFactory;
        let mut gain_node = gain_factory.instantiate(&cx, &gain_config).unwrap();
        gain_node.prepare(&cx).unwrap();

        let (bridge_factory, mut bridge_consumer, bridge_telemetry) = BridgeSinkFactory::new(8);
        let mut bridge_node = bridge_factory.instantiate(&cx, &NodeConfig::new()).unwrap();
        bridge_node.prepare(&cx).unwrap();

        // Push two frames into the mic source.
        let pool_a = pks_frame::AudioBufferPool::new(1, FRAME_SAMPLES);
        let mut handle_a = pool_a.acquire().unwrap();
        handle_a.copy_from_slice(&vec![0.3f32; FRAME_SAMPLES]);
        let frame_a = AudioFrame::new(StreamId(0), SourceId(0), 0, 0, 1, handle_a);

        let pool_b = pks_frame::AudioBufferPool::new(1, FRAME_SAMPLES);
        let mut handle_b = pool_b.acquire().unwrap();
        handle_b.copy_from_slice(&vec![0.8f32; FRAME_SAMPLES]);
        let frame_b = AudioFrame::new(StreamId(0), SourceId(0), 1, 0, 1, handle_b);

        mic_producer.push(frame_a).unwrap();
        mic_producer.push(frame_b).unwrap();

        // Drive the mini-pipeline for two ticks.
        for _ in 0..2 {
            let pool_tick = pks_frame::AudioBufferPool::new(1, FRAME_SAMPLES);
            let mut handle_tick = pool_tick.acquire().unwrap();
            handle_tick.copy_from_slice(&vec![0.0f32; FRAME_SAMPLES]);
            let silence = AudioFrame::new(StreamId(0), SourceId(0), 0, 0, 1, handle_tick);

            let from_mic = mic_node.process(silence).unwrap().unwrap();
            let from_gain = gain_node.process(from_mic).unwrap().unwrap();
            let sink_out = bridge_node.process(from_gain).unwrap();
            assert!(sink_out.is_none(), "sink must return None");
        }

        // Verify the bridge consumer received both frames in order.
        let out_a = bridge_consumer.pop().unwrap();
        let out_b = bridge_consumer.pop().unwrap();

        let peak_a = out_a
            .buffer
            .as_slice()
            .iter()
            .fold(0.0f32, |m, &s| m.max(s.abs()));
        let peak_b = out_b
            .buffer
            .as_slice()
            .iter()
            .fold(0.0f32, |m, &s| m.max(s.abs()));

        assert!(
            (peak_a - 0.3).abs() < 1e-6,
            "first frame amplitude wrong: peak={peak_a}"
        );
        assert!(
            (peak_b - 0.8).abs() < 1e-6,
            "second frame amplitude wrong: peak={peak_b}"
        );

        assert_eq!(mic_telemetry.frames_delivered(), 2);
        assert_eq!(bridge_telemetry.frames_pushed(), 2);
        assert_eq!(bridge_telemetry.overrun_count(), 0);
    }
}
