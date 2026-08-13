use std::sync::Arc;

use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};
use pocketstation::internal::frame::{
    AudioBufferPool, AudioFrame, ClockDomainId, FrameLineage, LineagedAudioFrame, SampleFormat,
    SampleSpec, SessionId, SourceId, StemId, StreamId,
};
use pocketstation::internal::graph::compiler::Compiler;
use pocketstation::internal::graph::dsl::Pipeline;
use pocketstation::internal::graph::node::{NodeConfig, PrepareContext};
use pocketstation::internal::graph::planner::RuntimePlanner;
use pocketstation::internal::graph::register_builtins;
use pocketstation::internal::graph::registry::NodeRegistry;
use pocketstation::internal::graph::NodeId;
use pocketstation::internal::runtime::{
    plan_source_channel, PlanEdgeReceiver, PlanEdgeRouter, PlanRunnerCancellation,
    PlanSourceSendOutcome, PlanSourceSender, RealtimePlanExecutor, RealtimePlanRunner,
};

const FRAME_SAMPLES: usize = 960;

fn lineaged_frame(
    pool: &Arc<AudioBufferPool>,
    stream_id: StreamId,
    source_id: SourceId,
    sequence_number: u64,
) -> LineagedAudioFrame {
    let frame = AudioFrame::try_new(
        stream_id,
        source_id,
        sequence_number,
        sequence_number.saturating_mul(20_000_000),
        SampleSpec::new(48_000, 1, SampleFormat::F32Interleaved),
        pool.acquire().expect("benchmark pool exhausted"),
    )
    .expect("valid benchmark frame");
    let lineage = FrameLineage::try_new(
        SessionId::new(42),
        source_id,
        StemId::new(source_id.get()),
        ClockDomainId::new(3),
        sequence_number,
        frame.timestamp_ns(),
        20_000_000,
        1,
        1,
        1,
    )
    .expect("valid benchmark lineage");
    LineagedAudioFrame::new(frame, lineage).expect("matching frame lineage")
}

fn three_branch_plan() -> (
    NodeRegistry,
    pocketstation::internal::graph::ir::GraphIr,
    pocketstation::internal::graph::plan::RuntimePlan,
    NodeId,
) {
    let mut registry = NodeRegistry::new();
    register_builtins(&mut registry).expect("built-in registration");
    let mut graph = Pipeline::new();
    let source = graph.add_node("passthrough", NodeConfig::new());
    let source_id = source.id();
    for _ in 0..3 {
        let sink = graph.add_node("passthrough", NodeConfig::new());
        graph.connect(source.out("out"), sink.in_("in"));
    }
    let ir = Compiler::new()
        .compile(graph.into_spec(), &registry)
        .expect("compiled benchmark graph");
    let plan = RuntimePlanner::new().plan(&ir).expect("planned graph");
    (registry, ir, plan, source_id)
}

fn drain(receivers: &mut [PlanEdgeReceiver]) {
    for receiver in receivers {
        black_box(receiver.try_recv().expect("routed benchmark frame"));
    }
}

fn bench_runtime_plan(c: &mut Criterion) {
    let mut group = c.benchmark_group("runtime_plan");
    group.throughput(Throughput::Elements(1));

    group.bench_function("router_three_branch_steady_state", |b| {
        let (_registry, ir, plan, source_id) = three_branch_plan();
        let (mut router, mut receivers) = PlanEdgeRouter::new(&plan, &ir).expect("prepared router");
        let pool = AudioBufferPool::new(64, FRAME_SAMPLES);
        let mut sequence_number = 0_u64;
        b.iter(|| {
            sequence_number = sequence_number.wrapping_add(1);
            let frame = lineaged_frame(
                &pool,
                StreamId::new(1),
                SourceId::new(1),
                black_box(sequence_number),
            );
            let summary = router.dispatch_from(source_id, "out", frame, sequence_number);
            assert_eq!(summary.enqueued_edges, 3);
            drain(&mut receivers);
            black_box(summary);
        });
    });

    group.bench_function("executor_source_plus_three_nodes_steady_state", |b| {
        let (registry, ir, plan, source_id) = three_branch_plan();
        let context = PrepareContext::new(SampleSpec::new(48_000, 1, SampleFormat::F32Interleaved));
        let (mut executor, workers) =
            RealtimePlanExecutor::new(&plan, &ir, &registry, &context).expect("prepared executor");
        assert!(workers.is_empty());
        let pool = AudioBufferPool::new(64, FRAME_SAMPLES);
        let mut sequence_number = 0_u64;
        b.iter(|| {
            sequence_number = sequence_number.wrapping_add(1);
            let frame = lineaged_frame(
                &pool,
                StreamId::new(1),
                SourceId::new(1),
                black_box(sequence_number),
            );
            let summary = executor
                .execute_from(source_id, frame, sequence_number)
                .expect("runtime plan execution");
            assert_eq!(summary.nodes_executed, 4);
            assert_eq!(summary.edges_enqueued, 3);
            black_box(summary);
        });
    });

    group.bench_function("runner_two_sources_steady_state", |b| {
        let mut registry = NodeRegistry::new();
        register_builtins(&mut registry).expect("built-in registration");
        let mut graph = Pipeline::new();
        let first_source = graph.add_node("passthrough", NodeConfig::new());
        let second_source = graph.add_node("passthrough", NodeConfig::new());
        let first_sink = graph.add_node("passthrough", NodeConfig::new());
        let second_sink = graph.add_node("passthrough", NodeConfig::new());
        graph.connect(first_source.out("out"), first_sink.in_("in"));
        graph.connect(second_source.out("out"), second_sink.in_("in"));
        let ir = Compiler::new()
            .compile(graph.into_spec(), &registry)
            .expect("compiled benchmark graph");
        let plan = RuntimePlanner::new().plan(&ir).expect("planned graph");
        let context = PrepareContext::new(SampleSpec::new(48_000, 1, SampleFormat::F32Interleaved));
        let (executor, workers) =
            RealtimePlanExecutor::new(&plan, &ir, &registry, &context).expect("prepared executor");
        assert!(workers.is_empty());
        let cancellation = PlanRunnerCancellation::new();
        let (mut first_sender, first_input) =
            plan_source_channel(first_source.id(), 4, cancellation.clone())
                .expect("first source channel");
        let (mut second_sender, second_input) =
            plan_source_channel(second_source.id(), 4, cancellation.clone())
                .expect("second source channel");
        let mut runner =
            RealtimePlanRunner::new(executor, vec![first_input, second_input], cancellation)
                .expect("prepared runner");
        let first_pool = AudioBufferPool::new(64, FRAME_SAMPLES);
        let second_pool = AudioBufferPool::new(64, FRAME_SAMPLES);
        let mut sequence_number = 0_u64;

        b.iter(|| {
            sequence_number = sequence_number.wrapping_add(1);
            send_source_frame(
                &mut first_sender,
                lineaged_frame(
                    &first_pool,
                    StreamId::new(1),
                    SourceId::new(1),
                    black_box(sequence_number),
                ),
            );
            send_source_frame(
                &mut second_sender,
                lineaged_frame(
                    &second_pool,
                    StreamId::new(2),
                    SourceId::new(2),
                    black_box(sequence_number),
                ),
            );
            let summary = runner.process_ready(2).expect("runner processing");
            assert_eq!(summary.source_frames_processed_total, 2);
            assert_eq!(summary.execution.nodes_executed, 4);
            black_box(summary);
        });
    });

    group.finish();
}

fn send_source_frame(sender: &mut PlanSourceSender, frame: LineagedAudioFrame) {
    assert!(matches!(
        sender.try_send(frame),
        PlanSourceSendOutcome::Enqueued
    ));
}

criterion_group!(benches, bench_runtime_plan);
criterion_main!(benches);
