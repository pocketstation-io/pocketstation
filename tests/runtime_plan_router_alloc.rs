use assert_no_alloc::{assert_no_alloc, AllocDisabler};
use pocketstation::internal::frame::{
    AudioBufferPool, AudioFrame, ClockDomainId, FrameLineage, LineagedAudioFrame, SessionId,
    SourceId, StemId, StreamId,
};
use pocketstation::internal::graph::compiler::Compiler;
use pocketstation::internal::graph::dsl::Pipeline;
use pocketstation::internal::graph::node::{NodeConfig, PrepareContext};
use pocketstation::internal::graph::planner::RuntimePlanner;
use pocketstation::internal::graph::register_builtins;
use pocketstation::internal::graph::registry::NodeRegistry;
use pocketstation::internal::runtime::{
    plan_source_channel, PlanEdgeRouter, PlanRunnerCancellation, PlanSourceSendOutcome,
    RealtimePlanExecutor, RealtimePlanRunner,
};

#[global_allocator]
static ALLOCATOR: AllocDisabler = AllocDisabler;

fn with_lineage(frame: AudioFrame) -> LineagedAudioFrame {
    let lineage = FrameLineage {
        session_id: SessionId(42),
        source_id: frame.source_id,
        stem_id: StemId(frame.source_id.0),
        clock_id: ClockDomainId(3),
        sequence_num: frame.sequence_number,
        timestamp_start_ns: frame.timestamp_ns,
        duration_ns: 20_000_000,
        source_generation: 4,
        discontinuity_epoch: 5,
        permission_epoch: 6,
    };
    LineagedAudioFrame::new(frame, lineage).unwrap()
}

#[test]
fn given_preallocated_three_edge_router_when_frame_dispatched_then_no_heap_allocation_occurs() {
    // Given
    let mut registry = NodeRegistry::new();
    register_builtins(&mut registry).unwrap();
    let mut graph = Pipeline::new();
    let source = graph.add_node("passthrough", NodeConfig::new());
    let source_id = source.id();
    for _ in 0..3 {
        let sink = graph.add_node("passthrough", NodeConfig::new());
        graph.connect(source.out("out"), sink.in_("in"));
    }
    let ir = Compiler::new()
        .compile(graph.into_spec(), &registry)
        .unwrap();
    let plan = RuntimePlanner::new().plan(&ir).unwrap();
    let (mut router, _receivers) = PlanEdgeRouter::new(&plan, &ir).unwrap();
    let pool = AudioBufferPool::new(1, 2);
    let mut buffer = pool.acquire().unwrap();
    buffer.copy_from_slice(&[0.25, -0.5]);
    let frame = with_lineage(AudioFrame::new(StreamId(1), SourceId(2), 3, 4, 1, buffer));

    // When
    let summary = assert_no_alloc(|| router.dispatch_lineaged_from(source_id, "out", frame, 5));

    // Then
    assert_eq!(summary.enqueued_edges, 3);
    assert_eq!(summary.dropped_edges, 0);
}

#[test]
fn given_prepared_realtime_plan_when_connected_nodes_execute_then_no_heap_allocation_occurs() {
    // Given
    let mut registry = NodeRegistry::new();
    register_builtins(&mut registry).unwrap();
    let mut graph = Pipeline::new();
    let source = graph.add_node("passthrough", NodeConfig::new());
    let source_id = source.id();
    for _ in 0..3 {
        let sink = graph.add_node("passthrough", NodeConfig::new());
        graph.connect(source.out("out"), sink.in_("in"));
    }
    let ir = Compiler::new()
        .compile(graph.into_spec(), &registry)
        .unwrap();
    let plan = RuntimePlanner::new().plan(&ir).unwrap();
    let context = PrepareContext::new(pocketstation::internal::frame::SampleSpec::new(
        48_000,
        1,
        pocketstation::internal::frame::SampleFormat::F32Interleaved,
    ));
    let (mut executor, workers) =
        RealtimePlanExecutor::new(&plan, &ir, &registry, &context).unwrap();
    assert!(workers.is_empty());
    let pool = AudioBufferPool::new(1, 2);
    let mut buffer = pool.acquire().unwrap();
    buffer.copy_from_slice(&[0.25, -0.5]);
    let frame = with_lineage(AudioFrame::new(StreamId(1), SourceId(2), 3, 4, 1, buffer));

    // When
    let summary = assert_no_alloc(|| executor.execute_lineaged_from(source_id, frame, 5)).unwrap();

    // Then
    assert_eq!(summary.nodes_executed, 4);
    assert_eq!(summary.edges_enqueued, 3);
    assert_eq!(summary.edges_dropped, 0);
}

#[test]
fn given_prepared_multi_source_runner_when_ready_frames_process_then_no_heap_allocation_occurs() {
    // Given
    let mut registry = NodeRegistry::new();
    register_builtins(&mut registry).unwrap();
    let mut graph = Pipeline::new();
    let first_source = graph.add_node("passthrough", NodeConfig::new());
    let second_source = graph.add_node("passthrough", NodeConfig::new());
    let first_sink = graph.add_node("passthrough", NodeConfig::new());
    let second_sink = graph.add_node("passthrough", NodeConfig::new());
    graph.connect(first_source.out("out"), first_sink.in_("in"));
    graph.connect(second_source.out("out"), second_sink.in_("in"));
    let ir = Compiler::new()
        .compile(graph.into_spec(), &registry)
        .unwrap();
    let plan = RuntimePlanner::new().plan(&ir).unwrap();
    let context = PrepareContext::new(pocketstation::internal::frame::SampleSpec::new(
        48_000,
        1,
        pocketstation::internal::frame::SampleFormat::F32Interleaved,
    ));
    let (executor, workers) = RealtimePlanExecutor::new(&plan, &ir, &registry, &context).unwrap();
    assert!(workers.is_empty());
    let cancellation = PlanRunnerCancellation::new();
    let (mut first_sender, first_input) =
        plan_source_channel(first_source.id(), 1, cancellation.clone()).unwrap();
    let (mut second_sender, second_input) =
        plan_source_channel(second_source.id(), 1, cancellation.clone()).unwrap();
    let mut runner =
        RealtimePlanRunner::new(executor, vec![first_input, second_input], cancellation).unwrap();
    let first_pool = AudioBufferPool::new(1, 2);
    let second_pool = AudioBufferPool::new(1, 2);
    let mut first_buffer = first_pool.acquire().unwrap();
    let mut second_buffer = second_pool.acquire().unwrap();
    first_buffer.copy_from_slice(&[0.25, -0.5]);
    second_buffer.copy_from_slice(&[-0.25, 0.5]);
    assert!(matches!(
        first_sender.try_send(with_lineage(AudioFrame::new(
            StreamId(1),
            SourceId(1),
            1,
            1,
            1,
            first_buffer,
        ))),
        PlanSourceSendOutcome::Enqueued
    ));
    assert!(matches!(
        second_sender.try_send(with_lineage(AudioFrame::new(
            StreamId(2),
            SourceId(2),
            1,
            1,
            1,
            second_buffer,
        ))),
        PlanSourceSendOutcome::Enqueued
    ));

    // When
    let summary = assert_no_alloc(|| runner.process_ready(2)).unwrap();

    // Then
    assert_eq!(summary.source_frames_processed_total, 2);
    assert_eq!(summary.execution.nodes_executed, 4);
    assert_eq!(summary.execution.edges_enqueued, 2);
    assert_eq!(summary.execution.edges_dropped, 0);
}
