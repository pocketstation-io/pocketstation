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
        crate::endpoint::EndpointGroupId::new("recording.test.v1"),
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

    assert!(outcome.session_dir.join("manifest.json").is_file());
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
    assert_eq!(manifest["schema_version"], 2);
    assert_eq!(manifest["session_id"], 42);
    assert_eq!(manifest["recording_group_id"], "recording.test.v1");
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
        crate::endpoint::EndpointGroupId::new("recording.test.v1"),
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
    let clip = outcome
        .read_clip(
            SessionId(7),
            StemId(20),
            crate::RecordingClipWindow::new(0, 60_000_000).unwrap(),
        )
        .unwrap();
    assert_eq!(clip.sample_frames, (FRAME_SAMPLES * 3) as u64);
    assert_eq!(clip.discontinuities.len(), 2);
    let samples: Vec<f32> = hound::WavReader::new(std::io::Cursor::new(clip.wav))
        .unwrap()
        .into_samples::<f32>()
        .map(Result::unwrap)
        .collect();
    assert!(samples[..FRAME_SAMPLES]
        .iter()
        .all(|sample| *sample == 0.25));
    assert!(samples[FRAME_SAMPLES..FRAME_SAMPLES * 2]
        .iter()
        .all(|sample| *sample == 0.0));
    assert!(samples[FRAME_SAMPLES * 2..]
        .iter()
        .all(|sample| *sample == 0.5));
    // Half-open clips exclude a gap ending exactly at their start, but retain
    // an instantaneous sequence-reset observation at that same start.
    let tail = outcome
        .read_clip(
            SessionId(7),
            StemId(20),
            crate::RecordingClipWindow::new(40_000_000, 60_000_000).unwrap(),
        )
        .unwrap();
    assert_eq!(tail.discontinuities.len(), 1);
    assert_eq!(tail.discontinuities[0].kind, DiscontinuityKind::SequenceGap);
    let before_gap = outcome
        .read_clip(
            SessionId(7),
            StemId(20),
            crate::RecordingClipWindow::new(0, 20_000_000).unwrap(),
        )
        .unwrap();
    assert!(before_gap.discontinuities.is_empty());
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
        crate::endpoint::EndpointGroupId::new("recording.test.v1"),
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
        crate::endpoint::EndpointGroupId::new("recording.test.v1"),
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

#[derive(Clone, Copy)]
enum OutputFault {
    None,
    WriteAfter(usize),
    Flush,
    Seek,
}

struct FaultingOutput {
    file: File,
    fault: OutputFault,
    armed: Arc<AtomicBool>,
    accepted_bytes: usize,
}

impl Write for FaultingOutput {
    fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
        if self.armed.load(Ordering::Relaxed) {
            if let OutputFault::WriteAfter(limit_bytes) = self.fault {
                let remaining_bytes = limit_bytes.saturating_sub(self.accepted_bytes);
                if remaining_bytes == 0 {
                    return Err(std::io::Error::other("injected ENOSPC during PCM write"));
                }
                let count = self
                    .file
                    .write(&bytes[..bytes.len().min(remaining_bytes)])?;
                self.accepted_bytes += count;
                return Ok(count);
            }
        }
        self.file.write(bytes)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        if self.armed.load(Ordering::Relaxed) && matches!(self.fault, OutputFault::Flush) {
            return Err(std::io::Error::other("injected WAV flush failure"));
        }
        self.file.flush()
    }
}

impl Seek for FaultingOutput {
    fn seek(&mut self, position: std::io::SeekFrom) -> std::io::Result<u64> {
        if self.armed.load(Ordering::Relaxed) && matches!(self.fault, OutputFault::Seek) {
            return Err(std::io::Error::other("injected WAV header seek failure"));
        }
        self.file.seek(position)
    }
}

fn output_fault_outcome(
    directory: &Path,
    fault: OutputFault,
    second_frame: bool,
    fail_checksum: bool,
    label: &str,
) -> (RecorderStemConfig, StemWorkerOutcome) {
    fs::create_dir_all(directory.join("stems")).unwrap();
    fs::create_dir_all(directory.join("events")).unwrap();
    let (mut router, mut receivers, nodes) = router_with_sources(1);
    let (source_id, stem_id) = if label == "microphone" {
        (21, 22)
    } else {
        (11, 12)
    };
    let config = stem_config(90, source_id, stem_id, 13, label, 1_000, 10_000);
    if second_frame {
        router.dispatch_from(
            nodes[0],
            "out",
            frame(&config, 1, 20_001_000, 0.5),
            20_001_000,
        );
    }
    let mut runtime = StemWorkerRuntime {
        session_dir: directory.to_owned(),
        config: config.clone(),
        receiver: receivers.pop().unwrap(),
        stop_requested: Arc::new(AtomicBool::new(true)),
        telemetry: Arc::new(RecorderWorkerTelemetry::default()),
        initial_frame: Some(PlanEdgeFrame::Exclusive(frame(&config, 0, 1_000, 0.25))),
    };
    let mut report = None;
    let result = runtime.run_with_output(
        &mut report,
        |path, spec| {
            let armed = Arc::new(AtomicBool::new(false));
            let writer = WavWriter::new(
                FaultingOutput {
                    file: File::create(path)?,
                    fault,
                    armed: Arc::clone(&armed),
                    accepted_bytes: 0,
                },
                spec,
            )?;
            armed.store(true, Ordering::Relaxed);
            Ok(writer)
        },
        |path| {
            if fail_checksum {
                Err(std::io::Error::other("injected checksum read failure").into())
            } else {
                checksum_fnv1a64(path)
            }
        },
    );
    let telemetry = Arc::clone(&runtime.telemetry);
    let outcome = runtime.into_outcome(report, result);
    assert_eq!(
        telemetry.snapshot().failures_total,
        u64::from(outcome.error.is_some())
    );
    (config, outcome)
}

fn manifest_for_outcome(
    config: &RecorderStemConfig,
    outcome: StemWorkerOutcome,
) -> ManifestDocument {
    ManifestDocument::session_finished(
        config.session_id,
        &crate::endpoint::EndpointGroupId::new("recording.test.v1"),
        &[SessionStemDeclaration {
            session_id: config.session_id,
            stem_id: config.stem_id,
            label: config.label.clone(),
            sample_rate_hz: config.sample_rate_hz,
            channels: config.channels,
            session_timeline_origin_ns: config.timeline_mapping.session_origin_ns,
        }],
        &[SessionStemWorkerOutcome {
            config: Some(config.clone()),
            worker: outcome,
        }],
    )
}

#[test]
fn given_mid_frame_storage_failure_when_worker_ends_then_accepted_prefix_is_preserved_incomplete() {
    let temp = TempDir::new().unwrap();
    let (config, outcome) = output_fault_outcome(
        temp.path(),
        OutputFault::WriteAfter(FRAME_SAMPLES * 4 + 8),
        true,
        false,
        "application",
    );
    let report = outcome.report.as_ref().unwrap();
    assert_eq!(report.written_frames, 1);
    assert_eq!(report.first_timestamp_ns, Some(10_000));
    assert_eq!(report.final_timestamp_ns, Some(20_010_000));
    assert!(outcome
        .error
        .as_deref()
        .unwrap()
        .contains("injected ENOSPC"));
    // A partial second frame exists physically but is not a complete logical frame.
    assert!(
        fs::metadata(temp.path().join("stems/application.wav"))
            .unwrap()
            .len()
            > 68 + 3_840
    );
    let manifest = manifest_for_outcome(&config, outcome);
    let stem = &manifest.stems[0];
    assert_eq!(manifest.state, RecordingState::Incomplete);
    assert_eq!(stem.finalization_state, RecordingState::Incomplete);
    assert_eq!(stem.written_frames, 1);
    assert_eq!(stem.first_timestamp_ns, Some(10_000));
    assert_eq!(stem.final_timestamp_ns, Some(20_010_000));
    assert_eq!(stem.clock_id, Some(config.clock_id.0));
    assert_eq!(stem.source_timeline_origin_ns, Some(1_000));
    assert!(stem.wav_bytes.is_none());
    assert!(stem.checksum.is_none());
    assert_eq!(manifest.errors.len(), 1);
}

#[test]
fn given_finalization_io_failures_when_worker_ends_then_known_clock_and_counts_are_not_erased() {
    for fault in [OutputFault::Flush, OutputFault::Seek] {
        let temp = TempDir::new().unwrap();
        let (config, outcome) =
            output_fault_outcome(temp.path(), fault, false, false, "application");
        let original_error = outcome.error.clone().unwrap();
        let manifest = manifest_for_outcome(&config, outcome);
        assert_eq!(manifest.state, RecordingState::Incomplete);
        let stem = &manifest.stems[0];
        assert_eq!(stem.finalization_state, RecordingState::Incomplete);
        assert_eq!(stem.written_frames, 1);
        assert_eq!(stem.first_timestamp_ns, Some(10_000));
        assert_eq!(stem.final_timestamp_ns, Some(20_010_000));
        assert!(stem.wav_bytes.is_none());
        assert!(stem.checksum.is_none());
        assert_eq!(stem.error.as_ref(), Some(&original_error));
        assert_eq!(manifest.errors, vec![original_error]);
    }
}

#[test]
fn given_checksum_failure_after_wav_finalize_when_reported_then_size_is_known_but_integrity_is_not()
{
    let temp = TempDir::new().unwrap();
    let (config, outcome) =
        output_fault_outcome(temp.path(), OutputFault::None, false, true, "application");
    let manifest = manifest_for_outcome(&config, outcome);
    let stem = &manifest.stems[0];
    assert_eq!(manifest.state, RecordingState::Incomplete);
    assert_eq!(stem.finalization_state, RecordingState::Incomplete);
    assert_eq!(stem.written_frames, 1);
    assert_eq!(
        stem.wav_bytes,
        Some(
            fs::metadata(temp.path().join("stems/application.wav"))
                .unwrap()
                .len()
        )
    );
    assert!(stem.checksum.is_none());
    assert!(stem
        .error
        .as_deref()
        .unwrap()
        .contains("checksum read failure"));
}

#[test]
fn given_ordinary_output_when_worker_finishes_then_complete_file_and_checksum_remain_verified() {
    let temp = TempDir::new().unwrap();
    let (config, outcome) =
        output_fault_outcome(temp.path(), OutputFault::None, true, false, "application");
    assert!(outcome.error.is_none());
    let manifest = manifest_for_outcome(&config, outcome);
    assert_eq!(manifest.state, RecordingState::Complete);
    assert!(manifest.errors.is_empty());
    let stem = &manifest.stems[0];
    assert_eq!(stem.finalization_state, RecordingState::Complete);
    assert_eq!(stem.written_frames, 2);
    assert_eq!(stem.first_timestamp_ns, Some(10_000));
    assert_eq!(stem.final_timestamp_ns, Some(40_010_000));
    assert_eq!(
        stem.checksum.as_deref(),
        Some(
            checksum_fnv1a64(&temp.path().join("stems/application.wav"))
                .unwrap()
                .as_str()
        )
    );
    assert_eq!(
        hound::WavReader::open(temp.path().join("stems/application.wav"))
            .unwrap()
            .duration(),
        1_920
    );
}

#[test]
fn given_no_accepted_frame_when_write_fails_then_missing_timing_stays_unknown() {
    let temp = TempDir::new().unwrap();
    let (config, outcome) = output_fault_outcome(
        temp.path(),
        OutputFault::WriteAfter(0),
        false,
        false,
        "application",
    );
    let manifest = manifest_for_outcome(&config, outcome);
    let stem = &manifest.stems[0];
    assert_eq!(manifest.state, RecordingState::Incomplete);
    assert_eq!(stem.written_frames, 0);
    assert!(stem.first_timestamp_ns.is_none());
    assert!(stem.final_timestamp_ns.is_none());
    assert!(stem.wav_bytes.is_none());
    assert!(stem.checksum.is_none());
    assert!(stem.error.is_some());
}

#[test]
fn given_partial_report_when_another_stem_finishes_then_healthy_stem_and_legacy_manifest_are_preserved(
) {
    let temp = TempDir::new().unwrap();
    let (bad_config, bad) = output_fault_outcome(
        &temp.path().join("bad"),
        OutputFault::Flush,
        false,
        false,
        "application",
    );
    let (good_config, good) = output_fault_outcome(
        &temp.path().join("good"),
        OutputFault::None,
        false,
        false,
        "microphone",
    );
    let manifest = ManifestDocument::finished(
        SessionId(90),
        &crate::endpoint::EndpointGroupId::new("recording.test.v1"),
        &[bad_config, good_config],
        &[bad, good],
        None,
    );
    assert_eq!(manifest.state, RecordingState::Incomplete);
    assert_eq!(manifest.errors.len(), 1);
    assert_eq!(
        manifest.stems[0].finalization_state,
        RecordingState::Incomplete
    );
    assert_eq!(manifest.stems[0].written_frames, 1);
    assert_eq!(
        manifest.stems[1].finalization_state,
        RecordingState::Complete
    );
    assert_eq!(manifest.stems[1].written_frames, 1);
    assert!(manifest.stems[1].checksum.is_some());
    // The current schema accepts null file integrity facts without invented zeros.
    let json = serde_json::to_vec(&manifest).unwrap();
    let restored: ManifestDocument = serde_json::from_slice(&json).unwrap();
    assert!(restored.stems[0].wav_bytes.is_none());
    assert_eq!(restored.stems[0].first_timestamp_ns, Some(10_000));
    assert_eq!(
        restored.stems[1].finalization_state,
        RecordingState::Complete
    );
}

#[test]
fn given_late_worker_failure_when_recording_finishes_then_public_outcome_keeps_accepted_frames() {
    let temp = TempDir::new().unwrap();
    let (mut router, mut receivers, nodes) = router_with_sources(1);
    let config = stem_config(91, 11, 12, 13, "application", 1_000, 10_000);
    let wrong = stem_config(91, 99, 12, 13, "application", 1_000, 10_000);
    router.dispatch_from(
        nodes[0],
        "out",
        frame(&wrong, 1, 20_001_000, 0.5),
        20_001_000,
    );
    let recording = MultistemRecording::start_observed(
        temp.path(),
        SessionId(91),
        crate::endpoint::EndpointGroupId::new("recording.test.v1"),
        vec![(
            config.clone(),
            receivers.pop().unwrap(),
            PlanEdgeFrame::Exclusive(frame(&config, 0, 1_000, 0.25)),
        )],
    )
    .unwrap();
    let outcome = recording.finish().unwrap();
    assert_eq!(outcome.state, RecordingState::Incomplete);
    assert_eq!(outcome.failed_stems, 1);
    assert_eq!(outcome.completed_stems, 0);
    assert_eq!(outcome.stems[0].written_frames, 1);
    assert!(outcome.stems[0]
        .error
        .as_deref()
        .unwrap()
        .contains("lineage Source"));
    let manifest: ManifestDocument =
        serde_json::from_reader(File::open(outcome.session_dir.join("manifest.json")).unwrap())
            .unwrap();
    assert_eq!(manifest.stems[0].first_timestamp_ns, Some(10_000));
    assert_eq!(manifest.stems[0].final_timestamp_ns, Some(20_010_000));
    assert_eq!(manifest.stems[0].written_frames, 1);
    assert_eq!(manifest.errors.len(), 1);
    assert!(outcome
        .read_clip(
            SessionId(91),
            config.stem_id,
            crate::RecordingClipWindow::new(10_000, 20_010_000).unwrap(),
        )
        .is_err());
}

fn production_recording_with_late_failure(root: &Path) -> SessionMultistemRecording {
    let (mut router, mut receivers, nodes) = router_with_sources(1);
    let config = stem_config(92, 11, 12, 13, "application", 10_000, 10_000);
    let wrong = stem_config(92, 99, 12, 13, "application", 10_000, 10_000);
    for (source, sequence, timestamp_ns) in [(&config, 0, 10_000), (&wrong, 1, 20_010_000)] {
        router.dispatch_from(
            nodes[0],
            "out",
            frame(source, sequence, timestamp_ns, 0.25),
            timestamp_ns,
        );
    }
    SessionMultistemRecording::start(
        root,
        SessionId(92),
        crate::endpoint::EndpointGroupId::new("recording.test.v1"),
        vec![SessionRecorderStemInput {
            session_id: config.session_id,
            stem_id: config.stem_id,
            label: config.label,
            sample_rate_hz: config.sample_rate_hz,
            channels: config.channels,
            session_timeline_origin_ns: config.timeline_mapping.session_origin_ns,
            receiver: receivers.pop().unwrap(),
        }],
    )
    .unwrap()
}

#[test]
fn given_production_session_worker_failure_when_finish_runs_then_public_and_manifest_verdicts_agree(
) {
    let temp = TempDir::new().unwrap();
    let outcome = production_recording_with_late_failure(temp.path())
        .finish()
        .unwrap();
    let manifest: ManifestDocument =
        serde_json::from_reader(File::open(outcome.session_dir.join("manifest.json")).unwrap())
            .unwrap();
    assert_eq!(outcome.state, RecordingState::Incomplete);
    assert_eq!(outcome.completed_stems, 0);
    assert_eq!(outcome.failed_stems, 1);
    assert_eq!(outcome.stems[0].written_frames, 1);
    assert_eq!(outcome.stems[0].error, manifest.stems[0].error);
    assert!(outcome.stems[0]
        .error
        .as_deref()
        .unwrap()
        .contains("lineage Source"));
    assert_eq!(manifest.stems[0].written_frames, 1);
    assert_eq!(manifest.stems[0].first_timestamp_ns, Some(0));
    assert_eq!(manifest.stems[0].final_timestamp_ns, Some(20_000_000));
    assert_eq!(
        manifest.stems[0].finalization_state,
        RecordingState::Incomplete
    );
    assert_eq!(manifest.errors.len(), 1);
}

#[test]
fn given_worker_and_auxiliary_write_failure_when_production_finish_runs_then_both_causes_survive() {
    let temp = TempDir::new().unwrap();
    let recording = production_recording_with_late_failure(temp.path());
    let session_dir = recording.session_dir.clone();
    let blocked_path = session_dir.join("metrics/destinations.jsonl");
    fs::create_dir(&blocked_path).unwrap();
    let secondary_kind = File::create(&blocked_path).unwrap_err().kind();
    let error = recording.finish().unwrap_err();
    let text = error.to_string();
    assert!(text.contains("recording worker failure:"));
    assert!(text.contains("lineage Source"));
    assert!(text.contains("finalization artifact failure:"));
    assert!(matches!(error, RecorderError::Io(ref error) if error.kind() == secondary_kind));
    // No manifest persistence success is invented after the actual I/O error.
    let retained: ManifestDocument =
        serde_json::from_reader(File::open(session_dir.join("manifest.json")).unwrap()).unwrap();
    assert_eq!(retained.state, RecordingState::Recording);
    assert_ne!(
        retained.stems[0].finalization_state,
        RecordingState::Complete
    );
}

#[test]
fn given_missing_integrity_without_worker_error_when_projected_then_public_stem_keeps_unverified_error(
) {
    let temp = TempDir::new().unwrap();
    let (config, mut outcome) =
        output_fault_outcome(temp.path(), OutputFault::None, false, false, "application");
    outcome.report.as_mut().unwrap().checksum_fnv1a64 = None;
    assert!(outcome.error.is_none());
    let stem = ManifestStem::from_outcome(&config, &outcome);
    let public = stem.recording_outcome(&outcome);
    assert_eq!(stem.finalization_state, RecordingState::Incomplete);
    assert_eq!(public.written_frames, 1);
    assert_eq!(
        public.error.as_deref(),
        Some("recorder worker finalization is unverified")
    );
}

#[test]
fn given_no_worker_failure_when_artifact_write_fails_then_original_io_error_is_unchanged() {
    let error = RecorderError::Io(std::io::Error::from_raw_os_error(28));
    let original = error.to_string();
    let retained = retain_worker_error_context::<()>(Err(error), std::iter::empty()).unwrap_err();
    assert_eq!(retained.to_string(), original);
    assert!(matches!(retained, RecorderError::Io(error) if error.raw_os_error() == Some(28)));
}
