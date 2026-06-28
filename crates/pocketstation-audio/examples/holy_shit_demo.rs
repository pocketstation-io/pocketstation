// Phase 0 capstone — the graph rescue, end to end on the real path:
//   build (DSL) → compile (validate + topo) → plan (partitions/memory/edge IDs)
//   → execute real audio through real DSP nodes → per-edge observability.
//
// This runs a genuinely executable realtime chain. It does NOT fake the full
// multi-source / STT / relay / browser demo: that needs DAG multi-input execution,
// remote model partitions, and Go-relay/device integration (next phase) — none of
// which are mocked here.

use pocketstation_frame::{
    AudioBufferPool, AudioFrame, SampleFormat, SampleSpec, SourceId, StreamId,
};
use pocketstation_graph::compiler::Compiler;
use pocketstation_graph::dsl::AudioGraph;
use pocketstation_graph::node::PrepareContext;
use pocketstation_graph::planner::RuntimePlanner;
use pocketstation_graph::{NodeConfig, NodeRegistry};
use pocketstation_metrics::{EdgeMetrics, EdgeObservation};
use pocketstation_nodes::register_all;
use pocketstation_runtime::PlanScheduler;

const FRAME_SAMPLES: usize = 960; // 20 ms @ 48 kHz mono
const FRAMES_TO_RUN: usize = 250; // 5 s of audio

fn main() {
    // 1. Build a real, executable graph via the DSL (produces an inert GraphSpec).
    let mut graph = AudioGraph::new();
    let src = graph.add_node(
        "source.synthetic",
        NodeConfig::new()
            .with("tone_hz", "440")
            .with("amplitude", "0.5"),
    );
    let mix = graph.add_node("transform.mono_mix", NodeConfig::new());
    let denoise = graph.add_node("transform.noise_suppress", NodeConfig::new());
    let vad = graph.add_node("transform.vad", NodeConfig::new());
    let watermark = graph.add_node("transform.watermark", NodeConfig::new());
    let gain = graph.add_node("gain", NodeConfig::new().with("gain_db", "-3.0"));
    let recorder = graph.add_node("sink.recording", NodeConfig::new());

    graph.connect(src.out("out"), mix.in_("in"));
    graph.connect(mix.out("out"), denoise.in_("in"));
    graph.connect(denoise.out("out"), vad.in_("in"));
    graph.connect(vad.out("out"), watermark.in_("in"));
    graph.connect(watermark.out("out"), gain.in_("in"));
    graph.connect(gain.out("out"), recorder.in_("in"));

    // 2. Compile (validate + topo sort) and lower to a RuntimePlan.
    let mut registry = NodeRegistry::new();
    register_all(&mut registry);
    let ir = Compiler::new()
        .compile(graph.into_spec(), &registry)
        .expect("graph compiles");
    let plan = RuntimePlanner::new().plan(&ir).expect("graph plans");

    println!(
        "holy_shit_demo: compiled {} nodes, {} edges → {} execution partition(s)",
        ir.node_count(),
        ir.edge_count(),
        plan.partitions.len(),
    );
    for partition in &plan.partitions {
        println!(
            "  partition {:?}: {} node(s)",
            partition.class,
            partition.nodes.len()
        );
    }
    println!(
        "  edges instrumented with metric IDs: {}",
        plan.edge_metrics.len()
    );

    // 3. Build an executor for the realtime partition and run real audio through it.
    let sample_spec = SampleSpec::new(48_000, 1, SampleFormat::F32Interleaved);
    let cx = PrepareContext::new(sample_spec);
    let mut executor = PlanScheduler::build_realtime_executor(&plan, &ir, &registry, &cx)
        .expect("realtime executor builds");

    let pool = AudioBufferPool::new(8, FRAME_SAMPLES);
    let output_edge = EdgeMetrics::default();

    for seq in 0..FRAMES_TO_RUN {
        let handle = pool.acquire().expect("pool slot");
        let frame = AudioFrame::new(StreamId(0), SourceId(0), seq as u64, 0, 1, handle);
        match executor.run_frame(frame) {
            Ok(Some(out)) => {
                output_edge.record_in(EdgeObservation::observe(out.buffer.as_slice()));
                output_edge.record_out();
            }
            Ok(None) => output_edge.record_dropped(),
            Err(e) => {
                eprintln!("holy_shit_demo: executor error: {e}");
                return;
            }
        }
    }

    // 4. Report per-edge observability measured from the real output stream.
    println!(
        "holy_shit_demo: ran {} realtime nodes for {} frames",
        executor.node_count(),
        FRAMES_TO_RUN,
    );
    println!(
        "  output edge: frames_in={}, frames_out={}, drop_rate={:.1}%, clipping_frames={}, worst_peak={:.1} dBFS",
        output_edge.frames_in.get(),
        output_edge.frames_out.get(),
        output_edge.drop_rate_pct(),
        output_edge.clipping_frames.get(),
        output_edge.worst_peak_dbfs.get_scaled(),
    );
}
