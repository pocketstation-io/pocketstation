use assert_no_alloc::{assert_no_alloc, AllocDisabler};
use pks_frame::{AudioBufferPool, AudioFrame, SourceId, StreamId};
use pks_graph::compiler::Compiler;
use pks_graph::dsl::Pipeline;
use pks_graph::node::{NodeConfig, PrepareContext};
use pks_graph::planner::RuntimePlanner;
use pks_graph::register_builtins;
use pks_graph::registry::NodeRegistry;
use pks_runtime::{
    plan_source_channel, PlanEdgeRouter, PlanRunnerCancellation, RealtimePlanExecutor,
    RealtimePlanRunner,
};

#[global_allocator]
static ALLOCATOR: AllocDisabler = AllocDisabler;

#[test]
fn given_preallocated_three_edge_router_when_frame_dispatched_then_no_heap_allocation_occurs() {
    // Given
    let mut registry = NodeRegistry::new();
    register_builtins(&mut registry);
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
    let frame = AudioFrame::new(StreamId(1), SourceId(2), 3, 4, 1, buffer);

    // When
    let summary = assert_no_alloc(|| router.dispatch_from(source_id, "out", frame, 5));

    // Then
    assert_eq!(summary.enqueued_edges, 3);
    assert_eq!(summary.dropped_edges, 0);
}

#[test]
fn given_prepared_realtime_plan_when_connected_nodes_execute_then_no_heap_allocation_occurs() {
    // Given
    let mut registry = NodeRegistry::new();
    register_builtins(&mut registry);
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
    let context = PrepareContext::new(pks_frame::SampleSpec::new(
        48_000,
        1,
        pks_frame::SampleFormat::F32Interleaved,
    ));
    let (mut executor, workers) =
        RealtimePlanExecutor::new(&plan, &ir, &registry, &context).unwrap();
    assert!(workers.is_empty());
    let pool = AudioBufferPool::new(1, 2);
    let mut buffer = pool.acquire().unwrap();
    buffer.copy_from_slice(&[0.25, -0.5]);
    let frame = AudioFrame::new(StreamId(1), SourceId(2), 3, 4, 1, buffer);

    // When
    let summary = assert_no_alloc(|| executor.execute_from(source_id, frame, 5)).unwrap();

    // Then
    assert_eq!(summary.nodes_executed, 4);
    assert_eq!(summary.edges_enqueued, 3);
    assert_eq!(summary.edges_dropped, 0);
}

#[test]
fn given_prepared_multi_source_runner_when_ready_frames_process_then_no_heap_allocation_occurs() {
    // Given
    let mut registry = NodeRegistry::new();
    register_builtins(&mut registry);
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
    let context = PrepareContext::new(pks_frame::SampleSpec::new(
        48_000,
        1,
        pks_frame::SampleFormat::F32Interleaved,
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
    first_sender
        .try_send(AudioFrame::new(
            StreamId(1),
            SourceId(1),
            1,
            1,
            1,
            first_buffer,
        ))
        .ok()
        .unwrap();
    second_sender
        .try_send(AudioFrame::new(
            StreamId(2),
            SourceId(2),
            1,
            1,
            1,
            second_buffer,
        ))
        .ok()
        .unwrap();

    // When
    let summary = assert_no_alloc(|| runner.process_ready(2)).unwrap();

    // Then
    assert_eq!(summary.source_frames_processed_total, 2);
    assert_eq!(summary.execution.nodes_executed, 4);
    assert_eq!(summary.execution.edges_enqueued, 2);
    assert_eq!(summary.execution.edges_dropped, 0);
}
