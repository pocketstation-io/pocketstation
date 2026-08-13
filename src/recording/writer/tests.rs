use super::*;
use crate::frame::{
    AudioBufferPool, AudioFrame, ClockDomainId, FrameLineage, LineagedAudioFrame, SourceId, StemId,
    StreamId,
};
use crate::graph::compile::Compiler;
use crate::graph::compile::RuntimePlanner;
use crate::graph::dsl::Pipeline;
use crate::graph::node::NodeConfig;
use crate::graph::register_builtins;
use crate::graph::registry::NodeRegistry;
use crate::graph::spec::NodeId;
use crate::runtime::PlanEdgeRouter;
use crate::timing::TimelineMapping;
use tempfile::TempDir;

const FRAME_SAMPLES: usize = 960;

fn frame(
    config: &RecorderStemConfig,
    sequence_number: u64,
    timestamp_ns: u64,
    value: f32,
) -> LineagedAudioFrame {
    let pool = AudioBufferPool::new(1, FRAME_SAMPLES);
    let mut buffer = pool.acquire().unwrap();
    buffer
        .try_copy_from_slice(&vec![value; FRAME_SAMPLES])
        .expect("test frame fits the fixed-capacity buffer");
    LineagedAudioFrame::new(
        AudioFrame::new(
            StreamId(config.source_id.0),
            config.source_id,
            sequence_number,
            timestamp_ns,
            1,
            buffer,
        ),
        FrameLineage {
            session_id: config.session_id,
            source_id: config.source_id,
            stem_id: config.stem_id,
            clock_id: config.clock_id,
            sequence_num: sequence_number,
            timestamp_start_ns: timestamp_ns,
            duration_ns: 20_000_000,
            source_generation: config.source_generation,
            discontinuity_epoch: 0,
            permission_epoch: config.permission_epoch,
        },
    )
    .unwrap()
}

fn stem_config(
    session_id: u64,
    source_id: u64,
    stem_id: u64,
    clock_id: u32,
    label: &str,
    source_origin_ns: u64,
    session_origin_ns: u64,
) -> RecorderStemConfig {
    RecorderStemConfig {
        session_id: SessionId(session_id),
        source_id: SourceId(source_id),
        stem_id: StemId(stem_id),
        clock_id: ClockDomainId(clock_id),
        source_generation: 1,
        permission_epoch: 1,
        permission_scope: PermissionScope::SessionCaptureGrant,
        permission: PermissionDecision::Allowed,
        label: StemLabel::new(label).unwrap(),
        sample_rate_hz: 48_000,
        channels: 1,
        timeline_mapping: TimelineMapping::new(source_origin_ns, session_origin_ns),
    }
}

fn router_with_sources(
    source_count: usize,
) -> (PlanEdgeRouter, Vec<PlanEdgeReceiver>, Vec<NodeId>) {
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
    (router, receivers, source_ids)
}

#[test]
fn given_fractional_stereo_gap_when_silence_is_sized_then_channels_remain_aligned() {
    let silence_samples = samples_for_duration_ns(190_342_250, 48_000, 2);

    assert_eq!(silence_samples % 2, 0);
    assert_eq!(silence_samples / 2, 9_136);
}

#[test]
fn given_two_clock_mapped_stems_when_finished_then_two_aligned_playable_wavs_are_written() {
    let temp_dir = TempDir::new().unwrap();
    let (_router, mut receivers, source_nodes) = router_with_sources(2);
    let application_receiver = receivers
        .iter()
        .position(|receiver| receiver.from().node == source_nodes[0])
        .map(|index| receivers.swap_remove(index))
        .unwrap();
    let microphone_receiver = receivers.pop().unwrap();
    let application = stem_config(42, 1, 11, 101, "application", 1_000, 10_000);
    let microphone = stem_config(42, 2, 12, 102, "microphone", 5_000, 10_000);
    let application_initial = frame(&application, 0, 1_000, 0.25);
    let microphone_initial = frame(&microphone, 0, 5_000, -0.5);
    let recording = MultistemRecording::start_observed(
        temp_dir.path(),
        SessionId(42),
        vec![
            (
                application,
                application_receiver,
                PlanEdgeFrame::Exclusive(application_initial),
            ),
            (
                microphone,
                microphone_receiver,
                PlanEdgeFrame::Exclusive(microphone_initial),
            ),
        ],
    )
    .unwrap();
    let outcome = recording.finish().unwrap();

    assert_eq!(outcome.state, RecordingState::Complete);
    assert_eq!(outcome.completed_stems, 2);
    for label in ["application", "microphone"] {
        let reader = hound::WavReader::open(
            outcome
                .session_dir
                .join("stems")
                .join(format!("{label}.wav")),
        )
        .unwrap();
        assert_eq!(reader.spec().sample_rate, 48_000);
        assert_eq!(reader.spec().channels, 1);
        assert_eq!(reader.duration(), FRAME_SAMPLES as u32);
    }
    let manifest: serde_json::Value =
        serde_json::from_reader(File::open(outcome.session_dir.join("manifest.json")).unwrap())
            .unwrap();
    assert_eq!(manifest["state"], "complete");
    assert_eq!(manifest["stems"][0]["first_timestamp_ns"], 10_000);
    assert_eq!(manifest["stems"][1]["first_timestamp_ns"], 10_000);
    assert!(manifest["stems"][0]["checksum"].as_str().is_some());
    let permission_events =
        fs::read_to_string(outcome.session_dir.join("events").join("permissions.jsonl")).unwrap();
    let first_permission: serde_json::Value =
        serde_json::from_str(permission_events.lines().next().unwrap()).unwrap();
    assert_eq!(first_permission["scope"], "session_capture_grant");
    assert_eq!(first_permission["decision"], "allowed");
}

#[test]
fn given_timestamp_and_sequence_gap_when_finished_then_silence_and_events_preserve_time() {
    let temp_dir = TempDir::new().unwrap();
    let (mut router, mut receivers, source_nodes) = router_with_sources(1);
    let config = stem_config(7, 1, 20, 5, "application", 0, 0);
    let initial = frame(&config, 0, 0, 0.25);
    let recording = MultistemRecording::start_observed(
        temp_dir.path(),
        SessionId(7),
        vec![(
            config.clone(),
            receivers.pop().unwrap(),
            PlanEdgeFrame::Exclusive(initial),
        )],
    )
    .unwrap();

    router.dispatch_from(
        source_nodes[0],
        "out",
        frame(&config, 2, 40_000_000, 0.5),
        40_000_000,
    );
    let outcome = recording.finish().unwrap();

    let reader =
        hound::WavReader::open(outcome.session_dir.join("stems").join("application.wav")).unwrap();
    assert_eq!(reader.duration(), (FRAME_SAMPLES * 3) as u32);
    let events = fs::read_to_string(
        outcome
            .session_dir
            .join("events")
            .join("discontinuities.jsonl"),
    )
    .unwrap();
    assert!(events.contains("timestamp_gap"));
    assert!(events.contains("sequence_gap"));
    let manifest: serde_json::Value =
        serde_json::from_reader(File::open(outcome.session_dir.join("manifest.json")).unwrap())
            .unwrap();
    assert_eq!(
        manifest["stems"][0]["silence_filled_samples"],
        FRAME_SAMPLES as u64
    );
}

#[test]
fn given_queued_audio_when_recording_cancelled_then_wav_header_is_playable_and_manifest_incomplete()
{
    let temp_dir = TempDir::new().unwrap();
    let (_router, mut receivers, _source_nodes) = router_with_sources(1);
    let config = stem_config(8, 1, 21, 5, "microphone", 0, 0);
    let initial = frame(&config, 0, 0, 0.25);
    let recording = MultistemRecording::start_observed(
        temp_dir.path(),
        SessionId(8),
        vec![(
            config,
            receivers.pop().unwrap(),
            PlanEdgeFrame::Exclusive(initial),
        )],
    )
    .unwrap();

    let outcome = recording.cancel("session cancelled by caller").unwrap();

    assert_eq!(outcome.state, RecordingState::Incomplete);
    let reader =
        hound::WavReader::open(outcome.session_dir.join("stems").join("microphone.wav")).unwrap();
    assert_eq!(reader.duration(), FRAME_SAMPLES as u32);
    let manifest: serde_json::Value =
        serde_json::from_reader(File::open(outcome.session_dir.join("manifest.json")).unwrap())
            .unwrap();
    assert_eq!(manifest["state"], "incomplete");
    assert_eq!(manifest["errors"][0], "session cancelled by caller");
}

#[test]
fn given_failed_recorder_branch_when_more_frames_dispatched_then_healthy_branch_continues() {
    let temp_dir = TempDir::new().unwrap();
    let mut registry = NodeRegistry::new();
    register_builtins(&mut registry).unwrap();
    let mut graph = Pipeline::new();
    let source = graph.add_node("passthrough", NodeConfig::new());
    let recorder_sink = graph.add_node("passthrough", NodeConfig::new());
    let healthy_sink = graph.add_node("passthrough", NodeConfig::new());
    let recorder_edge = graph.connect(source.out("out"), recorder_sink.in_("in"));
    let healthy_edge = graph.connect(source.out("out"), healthy_sink.in_("in"));
    let ir = Compiler::new()
        .compile(graph.into_spec(), &registry)
        .unwrap();
    let plan = RuntimePlanner::new().plan(&ir).unwrap();
    let (mut router, mut receivers) = PlanEdgeRouter::new(&plan, &ir).unwrap();
    let recorder_index = receivers
        .iter()
        .position(|receiver| receiver.edge_id() == recorder_edge)
        .unwrap();
    let recorder_receiver = receivers.swap_remove(recorder_index);
    let healthy_receiver = receivers
        .iter_mut()
        .find(|receiver| receiver.edge_id() == healthy_edge)
        .unwrap();
    let expected = stem_config(9, 99, 30, 5, "application", 0, 0);
    let actual = stem_config(9, 1, 30, 5, "application", 0, 0);
    let rejected_initial = frame(&actual, 0, 0, 0.25);
    let recording = MultistemRecording::start_observed(
        temp_dir.path(),
        SessionId(9),
        vec![(
            expected,
            recorder_receiver,
            PlanEdgeFrame::Exclusive(rejected_initial),
        )],
    )
    .unwrap();

    router.dispatch_from(source.id(), "out", frame(&actual, 0, 0, 0.25), 0);
    assert_eq!(healthy_receiver.try_recv().unwrap().sequence_number(), 0);
    thread::sleep(Duration::from_millis(10));
    router.dispatch_from(
        source.id(),
        "out",
        frame(&actual, 1, 20_000_000, 0.5),
        20_000_000,
    );
    assert_eq!(healthy_receiver.try_recv().unwrap().sequence_number(), 1);
    let outcome = recording.finish().unwrap();

    assert_eq!(outcome.state, RecordingState::Incomplete);
    assert_eq!(outcome.failed_stems, 1);
    let metrics = fs::read_to_string(
        outcome
            .session_dir
            .join("metrics")
            .join("destinations.jsonl"),
    )
    .unwrap();
    let metrics: serde_json::Value = serde_json::from_str(metrics.trim()).unwrap();
    assert_eq!(metrics["worker_failures_total"], 1);
    let manifest: serde_json::Value =
        serde_json::from_reader(File::open(outcome.session_dir.join("manifest.json")).unwrap())
            .unwrap();
    assert_eq!(manifest["state"], "incomplete");
    assert!(manifest["stems"][0]["error"]
        .as_str()
        .unwrap()
        .contains("frame lineage Source"));
}
