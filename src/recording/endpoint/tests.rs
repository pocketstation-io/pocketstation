use std::fs::File;
use std::thread;
use std::time::{Duration, Instant};

use crate::endpoint::{
    endpoint_start_gate, EndpointDriverRegistry, EndpointPrepareContext, EndpointRouteContext,
    OperatorId, SessionTimelineOrigin,
};
use crate::frame::{
    AudioBufferPool, AudioFrame, ClockDomainId, EndpointId, FrameLineage, LineagedAudioFrame,
    RouteId, SampleFormat, SampleSpec, SourceId, StemId, StreamId,
};
use crate::graph::compile::Compiler;
use crate::graph::compile::RuntimePlanner;
use crate::graph::dsl::Pipeline;
use crate::graph::node::{NodeConfig, PrepareContext};
use crate::graph::register_builtins;
use crate::graph::registry::NodeRegistry;
use crate::graph::spec::{EdgeId, NodeId};
use crate::graph::{MediaCaps, NodeTypeId, RouteSettings, SignalSpec};
use crate::runtime::{PlanEdgeReceiver, PlanEdgeRouter};
use tempfile::TempDir;

use super::*;
const SESSION_ID: SessionId = SessionId(42);
const GROUP_ID: &str = "session.multistem.default.v1";
const OPERATOR_ID: &str = "io.pocketstation.recording.wav-stems.v1";
const NODE_TYPE_ID: &str = "endpoint.recording.multistem";
const FRAME_SAMPLES: usize = 960;

fn input(receiver: PlanEdgeReceiver, endpoint_id: EndpointId, label: &str) -> EndpointPortInput {
    let prepare_context =
        PrepareContext::new(SampleSpec::new(48_000, 1, SampleFormat::F32Interleaved));
    EndpointPortInput::audio(
        "audio",
        SignalSpec::audio(),
        MediaCaps::Any,
        RouteSettings::realtime_audio(),
        receiver,
        prepare_context,
        EndpointPrepareContext::new(
            SESSION_ID,
            endpoint_id,
            EndpointRouteContext::from_source(
                RouteId(endpoint_id.0),
                SourceId(endpoint_id.0),
                StreamId(endpoint_id.0),
                None,
            ),
            SessionTimelineOrigin::from_monotonic_timestamp_ns(1),
            NodeConfig::new()
                .with("stem_name", label)
                .with("recording_group_id", GROUP_ID),
        ),
    )
}

fn session_input(
    receiver: PlanEdgeReceiver,
    endpoint_id: EndpointId,
    stem_id: StemId,
    route_id: RouteId,
    label: &str,
    timeline_origin_ns: u64,
) -> EndpointPortInput {
    let prepare_context =
        PrepareContext::new(SampleSpec::new(48_000, 1, SampleFormat::F32Interleaved));
    EndpointPortInput::audio(
        "audio",
        SignalSpec::audio(),
        MediaCaps::Any,
        RouteSettings::realtime_audio(),
        receiver,
        prepare_context,
        EndpointPrepareContext::new(
            SESSION_ID,
            endpoint_id,
            EndpointRouteContext::from_stem(route_id, stem_id),
            SessionTimelineOrigin::from_monotonic_timestamp_ns(timeline_origin_ns),
            NodeConfig::new()
                .with("stem_name", label)
                .with("recording_group_id", GROUP_ID),
        ),
    )
}

fn lineaged_frame_with_permission(
    source_id: u64,
    stem_id: u64,
    sequence_number: u64,
    permission_epoch: u64,
    value: f32,
) -> LineagedAudioFrame {
    let pool = AudioBufferPool::new(1, FRAME_SAMPLES);
    let mut buffer = pool.acquire().unwrap();
    buffer
        .try_copy_from_slice(&vec![value; FRAME_SAMPLES])
        .expect("test samples fit the fixed-capacity buffer");
    let timestamp_ns = sequence_number.saturating_mul(20_000_000);
    LineagedAudioFrame::new(
        AudioFrame::new(
            StreamId(source_id),
            SourceId(source_id),
            sequence_number,
            timestamp_ns,
            1,
            buffer,
        ),
        FrameLineage {
            session_id: SESSION_ID,
            source_id: SourceId(source_id),
            stem_id: StemId(stem_id),
            clock_id: ClockDomainId(source_id as u32),
            sequence_num: sequence_number,
            timestamp_start_ns: timestamp_ns,
            duration_ns: 20_000_000,
            source_generation: 7,
            discontinuity_epoch: 0,
            permission_epoch,
        },
    )
    .unwrap()
}

fn router_with_sources(
    source_count: usize,
) -> (
    PlanEdgeRouter,
    Vec<PlanEdgeReceiver>,
    Vec<NodeId>,
    Vec<EdgeId>,
) {
    let mut registry = NodeRegistry::new();
    register_builtins(&mut registry).unwrap();
    let mut graph = Pipeline::new();
    let mut source_ids = Vec::with_capacity(source_count);
    for _ in 0..source_count {
        let source = graph.add_node("passthrough", NodeConfig::new());
        let sink = graph.add_node("passthrough", NodeConfig::new());
        graph.connect(source.out("out"), sink.in_("in"));
        source_ids.push(source.id());
    }
    let ir = Compiler::new()
        .compile(graph.into_spec(), &registry)
        .unwrap();
    let plan = RuntimePlanner::new().plan(&ir).unwrap();
    let (router, receivers) = PlanEdgeRouter::new(&plan, &ir).unwrap();
    let edge_ids = receivers.iter().map(PlanEdgeReceiver::edge_id).collect();
    (router, receivers, source_ids, edge_ids)
}

fn session_endpoint_registry(
    coordinator: SessionMultistemEndpointCoordinator,
) -> (EndpointDriverRegistry, OperatorId, NodeTypeId) {
    let operator_id = OperatorId::new(OPERATOR_ID);
    let node_type_id = NodeTypeId::from(NODE_TYPE_ID);
    let mut registry = EndpointDriverRegistry::new();
    registry
        .register(
            operator_id.clone(),
            node_type_id.clone(),
            Arc::new(coordinator),
        )
        .unwrap();
    (registry, operator_id, node_type_id)
}

fn wait_for_received(running: &crate::endpoint::RunningEndpoint, expected_frames: u64) {
    let deadline = Instant::now() + Duration::from_secs(2);
    while Instant::now() < deadline {
        if running.observations().frames_received_total >= expected_frames {
            return;
        }
        thread::sleep(Duration::from_millis(1));
    }
    panic!(
        "recording endpoint did not receive {expected_frames} frames: {:?}",
        running.observations()
    );
}

fn wait_for_failure(running: &crate::endpoint::RunningEndpoint) {
    let deadline = Instant::now() + Duration::from_secs(2);
    while Instant::now() < deadline {
        if running.observations().failures_total > 0 {
            return;
        }
        thread::sleep(Duration::from_millis(1));
    }
    panic!("recording endpoint did not report its worker failure");
}

#[test]
fn given_session_context_and_two_first_frames_when_recorded_then_manifest_derives_capture_lineage_and_common_origin(
) {
    let temp_dir = TempDir::new().unwrap();
    let coordinator =
        SessionMultistemEndpointCoordinator::new(temp_dir.path(), EndpointGroupId::new(GROUP_ID));
    let receipt = coordinator.receipt();
    let (registry, operator_id, node_type_id) = session_endpoint_registry(coordinator);
    let (mut router, mut receivers, source_nodes, _edge_ids) = router_with_sources(2);
    let prepared = registry
        .prepare_batch(
            &operator_id,
            &node_type_id,
            vec![
                session_input(
                    receivers.remove(0),
                    EndpointId(101),
                    StemId(11),
                    RouteId(21),
                    "application",
                    0,
                ),
                session_input(
                    receivers.remove(0),
                    EndpointId(102),
                    StemId(12),
                    RouteId(22),
                    "microphone",
                    0,
                ),
            ],
        )
        .unwrap();
    let (gate_controller, gate) = endpoint_start_gate();
    let mut running = prepared.start(gate).unwrap();
    gate_controller.open();

    router.dispatch_from(
        source_nodes[0],
        "out",
        lineaged_frame_with_permission(31, 11, 0, 4, 0.25),
        1,
    );
    router.dispatch_from(
        source_nodes[1],
        "out",
        lineaged_frame_with_permission(32, 12, 0, 5, -0.5),
        1,
    );
    wait_for_received(&running, 2);
    running.request_stop();
    let finalization = running.join_and_finalize();

    assert!(finalization.is_success());
    let outcome = receipt.result().expect("recording receipt must finalize");
    assert_eq!(outcome.state, RecordingState::Complete);
    assert_eq!(outcome.completed_stems, 2);
    let manifest: serde_json::Value =
        serde_json::from_reader(File::open(outcome.session_dir.join("manifest.json")).unwrap())
            .unwrap();
    let stems = manifest["stems"].as_array().unwrap();
    assert_eq!(stems[0]["source_id"], 31);
    assert_eq!(stems[0]["stem_id"], 11);
    assert_eq!(stems[0]["clock_id"], 31);
    assert_eq!(stems[0]["source_generation"], 7);
    assert_eq!(stems[0]["permission_epoch"], 4);
    assert_eq!(stems[0]["source_timeline_origin_ns"], 0);
    assert_eq!(stems[0]["session_timeline_origin_ns"], 0);
    assert_eq!(stems[1]["source_id"], 32);
    assert_eq!(stems[1]["stem_id"], 12);
    assert_eq!(stems[1]["permission_epoch"], 5);
    assert_eq!(stems[1]["source_timeline_origin_ns"], 0);
    assert_eq!(stems[1]["session_timeline_origin_ns"], 0);
}

#[test]
fn given_one_late_stem_when_other_stems_are_active_then_each_active_stem_records_without_loss() {
    let temp_dir = TempDir::new().unwrap();
    let coordinator =
        SessionMultistemEndpointCoordinator::new(temp_dir.path(), EndpointGroupId::new(GROUP_ID));
    let receipt = coordinator.receipt();
    let (registry, operator_id, node_type_id) = session_endpoint_registry(coordinator);
    let (mut router, mut receivers, source_nodes, _edge_ids) = router_with_sources(3);
    let prepared = registry
        .prepare_batch(
            &operator_id,
            &node_type_id,
            vec![
                session_input(
                    receivers.remove(0),
                    EndpointId(101),
                    StemId(11),
                    RouteId(21),
                    "application",
                    0,
                ),
                session_input(
                    receivers.remove(0),
                    EndpointId(102),
                    StemId(12),
                    RouteId(22),
                    "microphone",
                    0,
                ),
                session_input(
                    receivers.remove(0),
                    EndpointId(103),
                    StemId(13),
                    RouteId(23),
                    "assistant",
                    0,
                ),
            ],
        )
        .unwrap();
    let (gate_controller, gate) = endpoint_start_gate();
    let mut running = prepared.start(gate).unwrap();
    gate_controller.open();

    for sequence_number in 0..20 {
        router.dispatch_from(
            source_nodes[0],
            "out",
            lineaged_frame_with_permission(31, 11, sequence_number, 4, 0.25),
            sequence_number.saturating_mul(20_000_000),
        );
        router.dispatch_from(
            source_nodes[1],
            "out",
            lineaged_frame_with_permission(32, 12, sequence_number, 5, -0.5),
            sequence_number.saturating_mul(20_000_000),
        );
        wait_for_received(&running, (sequence_number + 1) * 2);
    }
    router.dispatch_from(
        source_nodes[2],
        "out",
        lineaged_frame_with_permission(33, 13, 0, 6, 0.75),
        400_000_000,
    );
    wait_for_received(&running, 41);
    running.request_stop();
    let finalization = running.join_and_finalize();

    assert!(finalization.is_success());
    let outcome = receipt.result().expect("recording receipt must finalize");
    assert_eq!(outcome.state, RecordingState::Complete);
    assert_eq!(outcome.completed_stems, 3);
    assert_eq!(outcome.stems[0].written_frames, 20);
    assert_eq!(outcome.stems[1].written_frames, 20);
    assert_eq!(outcome.stems[2].written_frames, 1);
    assert!(outcome
        .stems
        .iter()
        .all(|stem| stem.edge_observations.frames_dropped_total == 0));
}

#[test]
fn given_an_accepted_first_frame_when_drain_starts_immediately_then_recording_preserves_it() {
    let temp_dir = TempDir::new().unwrap();
    let coordinator =
        SessionMultistemEndpointCoordinator::new(temp_dir.path(), EndpointGroupId::new(GROUP_ID));
    let receipt = coordinator.receipt();
    let (registry, operator_id, node_type_id) = session_endpoint_registry(coordinator);
    let (mut router, mut receivers, source_nodes, _edge_ids) = router_with_sources(1);
    let prepared = registry
        .prepare_batch(
            &operator_id,
            &node_type_id,
            vec![session_input(
                receivers.pop().unwrap(),
                EndpointId(101),
                StemId(11),
                RouteId(21),
                "assistant",
                0,
            )],
        )
        .unwrap();
    let (gate_controller, gate) = endpoint_start_gate();
    let mut running = prepared.start(gate).unwrap();
    gate_controller.open();

    router.dispatch_from(
        source_nodes[0],
        "out",
        lineaged_frame_with_permission(31, 11, 0, 4, 0.25),
        1,
    );
    drop(router);
    running.request_shutdown(EndpointShutdownMode::Drain);
    let finalization = running.join_and_finalize();

    assert!(finalization.is_success());
    assert_eq!(finalization.observations.frames_received_total, 1);
    assert_eq!(finalization.observations.frames_delivered_total, 1);
    let outcome = receipt.result().expect("recording receipt must finalize");
    assert_eq!(outcome.state, RecordingState::Complete);
    assert_eq!(outcome.stems[0].written_frames, 1);
}

#[test]
fn given_session_recorder_input_without_audio_stem_origin_when_prepared_then_it_is_rejected() {
    let temp_dir = TempDir::new().unwrap();
    let coordinator =
        SessionMultistemEndpointCoordinator::new(temp_dir.path(), EndpointGroupId::new(GROUP_ID));
    let (registry, operator_id, node_type_id) = session_endpoint_registry(coordinator);
    let (_router, mut receivers, _source_nodes, _edge_ids) = router_with_sources(1);

    let error = match registry.prepare_batch(
        &operator_id,
        &node_type_id,
        vec![input(
            receivers.pop().unwrap(),
            EndpointId(101),
            "application",
        )],
    ) {
        Ok(_) => panic!("missing audio stem origin must fail preparation"),
        Err(error) => error,
    };

    assert!(error.to_string().contains("is not bound to an audio stem"));
    assert!(!temp_dir.path().join("session-42").exists());
}

#[test]
fn given_derived_permission_epoch_when_later_frame_changes_it_then_recording_fails_closed() {
    let temp_dir = TempDir::new().unwrap();
    let coordinator =
        SessionMultistemEndpointCoordinator::new(temp_dir.path(), EndpointGroupId::new(GROUP_ID));
    let receipt = coordinator.receipt();
    let (registry, operator_id, node_type_id) = session_endpoint_registry(coordinator);
    let (mut router, mut receivers, source_nodes, _edge_ids) = router_with_sources(1);
    let prepared = registry
        .prepare_batch(
            &operator_id,
            &node_type_id,
            vec![session_input(
                receivers.pop().unwrap(),
                EndpointId(101),
                StemId(11),
                RouteId(21),
                "application",
                0,
            )],
        )
        .unwrap();
    let (gate_controller, gate) = endpoint_start_gate();
    let mut running = prepared.start(gate).unwrap();
    gate_controller.open();

    router.dispatch_from(
        source_nodes[0],
        "out",
        lineaged_frame_with_permission(31, 11, 0, 4, 0.25),
        1,
    );
    wait_for_received(&running, 1);
    router.dispatch_from(
        source_nodes[0],
        "out",
        lineaged_frame_with_permission(31, 11, 1, 5, 0.5),
        20_000_001,
    );
    wait_for_failure(&running);
    running.request_stop();
    let finalization = running.join_and_finalize();

    assert!(!finalization.is_success());
    assert_eq!(finalization.observations.frames_received_total, 2);
    assert_eq!(finalization.observations.failures_total, 1);
    let outcome = receipt
        .result()
        .expect("failed recording receipt must finalize");
    assert_eq!(outcome.state, RecordingState::Incomplete);
    assert!(outcome.stems[0]
        .error
        .as_deref()
        .is_some_and(|error| error.contains("PermissionEpoch")));
}
