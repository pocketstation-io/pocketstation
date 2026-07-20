use assert_no_alloc::{assert_no_alloc, AllocDisabler};
use pks_frame::{AudioBufferPool, AudioFrame, SourceId, StreamId};
use pks_graph::compiler::Compiler;
use pks_graph::dsl::Pipeline;
use pks_graph::node::{NodeConfig, PrepareContext};
use pks_graph::planner::RuntimePlanner;
use pks_graph::register_builtins;
use pks_graph::registry::NodeRegistry;
use pks_runtime::{PlanEdgeRouter, RealtimePlanExecutor};

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
