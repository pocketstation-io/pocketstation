use std::error::Error;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

use pks_frame::{
    AudioBufferPool, AudioFrame, ClockDomainId, SampleFormat, SessionId, SourceId, StemId, StreamId,
};
use pks_graph::compiler::Compiler;
use pks_graph::dsl::Pipeline;
use pks_graph::node::{
    ConfigError, NodeConfig, NodeDescriptor, NodeError, NodeTypeId, PrepareContext,
};
use pks_graph::partition::ExecutionPartition;
use pks_graph::planner::RuntimePlanner;
use pks_graph::registry::{NodeFactory, NodeRegistry};
use pks_graph::runtime_node::RuntimeNode;
use pks_graph::spec::NodeId;
use pks_graph::EdgeId;
use pks_nodes::{
    MultistemRecording, PermissionDecision, RecorderStemConfig, RecordingState, StemLabel,
};
use pks_runtime::{EdgeObservations, PlanEdgeReceiver, PlanEdgeRouter};
use pks_timing::TimelineMapping;
use serde::Serialize;

const SAMPLE_RATE_HZ: u32 = 48_000;
const CHANNELS: u8 = 1;
const FRAME_DURATION_MS: u64 = 20;
const FRAME_DURATION_NS: u64 = FRAME_DURATION_MS * 1_000_000;
const FRAME_SAMPLES: usize = 960;
const CONNECTOR_TYPE_ID: &str = "example.connector";
const BROWSER_TYPE_ID: &str = "example.browser_receiver";
const RECORDER_TYPE_ID: &str = "example.multistem_recorder";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
enum Cell {
    Normal,
    SlowConnector,
    SlowRecorder,
    ConnectorFailure,
    RecorderFailure,
}

impl Cell {
    fn parse(value: &str) -> Result<Self, String> {
        match value {
            "normal" => Ok(Self::Normal),
            "slow-connector" => Ok(Self::SlowConnector),
            "slow-recorder" => Ok(Self::SlowRecorder),
            "connector-failure" => Ok(Self::ConnectorFailure),
            "recorder-failure" => Ok(Self::RecorderFailure),
            _ => Err(format!("unknown cell '{value}'")),
        }
    }
}

struct Args {
    duration_seconds: u64,
    output: PathBuf,
    cell: Cell,
}

impl Args {
    fn parse() -> Result<Self, String> {
        let mut duration_seconds = None;
        let mut output = None;
        let mut cell = None;
        let mut arguments = std::env::args().skip(1);
        while let Some(argument) = arguments.next() {
            let value = arguments
                .next()
                .ok_or_else(|| format!("missing value after {argument}"))?;
            match argument.as_str() {
                "--duration-seconds" => {
                    duration_seconds = Some(
                        value
                            .parse::<u64>()
                            .map_err(|error| format!("invalid duration '{value}': {error}"))?,
                    );
                }
                "--output" => output = Some(PathBuf::from(value)),
                "--cell" => cell = Some(Cell::parse(&value)?),
                _ => return Err(format!("unknown argument '{argument}'")),
            }
        }
        let duration_seconds = duration_seconds.ok_or("--duration-seconds is required")?;
        if duration_seconds == 0 {
            return Err("--duration-seconds must be greater than zero".to_owned());
        }
        Ok(Self {
            duration_seconds,
            output: output.ok_or("--output is required")?,
            cell: cell.ok_or("--cell is required")?,
        })
    }
}

struct EndpointFactory {
    type_id: &'static str,
    display_name: &'static str,
    execution: ExecutionPartition,
}

impl NodeFactory for EndpointFactory {
    fn descriptor(&self) -> NodeDescriptor {
        NodeDescriptor {
            type_id: NodeTypeId::from(self.type_id),
            display_name: self.display_name,
            inputs: vec![pks_caps::PortSpec {
                name: "in".to_owned(),
                direction: pks_caps::PortDirection::Input,
                media: pks_caps::MediaCaps::Audio(pks_caps::AudioCaps {
                    sample_rate_hz: Some(SAMPLE_RATE_HZ),
                    frame_samples: Some(FRAME_SAMPLES),
                    channel_layout: pks_caps::ChannelLayout::Mono,
                    format: SampleFormat::F32Interleaved,
                }),
                multiplicity: pks_caps::Multiplicity::One,
                required: true,
            }],
            outputs: Vec::new(),
            execution: self.execution,
            realtime_safe: false,
            stateful: true,
        }
    }

    fn validate_config(&self, _config: &NodeConfig) -> Result<(), ConfigError> {
        Ok(())
    }

    fn instantiate(
        &self,
        _context: &PrepareContext,
        _config: &NodeConfig,
    ) -> Result<Box<dyn RuntimeNode>, NodeError> {
        Ok(Box::new(EndpointTerminal))
    }
}

struct EndpointTerminal;

impl RuntimeNode for EndpointTerminal {
    fn prepare(&mut self, _context: &PrepareContext) -> Result<(), NodeError> {
        Ok(())
    }

    fn process(&mut self, _frame: AudioFrame) -> Result<Option<AudioFrame>, NodeError> {
        Ok(None)
    }
}

struct Topology {
    router: PlanEdgeRouter,
    application_source: NodeId,
    microphone_source: NodeId,
    connector_receivers: Vec<PlanEdgeReceiver>,
    browser_receivers: Vec<PlanEdgeReceiver>,
    recorder_receivers: Vec<PlanEdgeReceiver>,
}

fn build_topology(cell: Cell) -> Result<Topology, Box<dyn Error>> {
    let mut registry = NodeRegistry::new();
    pks_graph::register_builtins(&mut registry);
    registry.register(Arc::new(EndpointFactory {
        type_id: CONNECTOR_TYPE_ID,
        display_name: "Example connector",
        execution: ExecutionPartition::AsyncWorker,
    }));
    registry.register(Arc::new(EndpointFactory {
        type_id: BROWSER_TYPE_ID,
        display_name: "Example browser receiver",
        execution: ExecutionPartition::External,
    }));
    registry.register(Arc::new(EndpointFactory {
        type_id: RECORDER_TYPE_ID,
        display_name: "Multistem recorder",
        execution: ExecutionPartition::BlockingWorker,
    }));

    let mut pipeline = Pipeline::new();
    let application = pipeline.add_node("passthrough", NodeConfig::new());
    let microphone = pipeline.add_node("passthrough", NodeConfig::new());
    let application_source = application.id();
    let microphone_source = microphone.id();
    let mut connector_nodes = Vec::new();
    let mut browser_nodes = Vec::new();
    let mut recorder_nodes = Vec::new();
    for source in [&application, &microphone] {
        let connector = pipeline.add_node(CONNECTOR_TYPE_ID, NodeConfig::new());
        let browser = pipeline.add_node(BROWSER_TYPE_ID, NodeConfig::new());
        let recorder = pipeline.add_node(RECORDER_TYPE_ID, NodeConfig::new());
        pipeline.connect(source.out("out"), connector.in_("in"));
        pipeline.connect(source.out("out"), browser.in_("in"));
        pipeline.connect(source.out("out"), recorder.in_("in"));
        connector_nodes.push(connector.id());
        browser_nodes.push(browser.id());
        recorder_nodes.push(recorder.id());
    }

    let ir = Compiler::new().compile(pipeline.into_spec(), &registry)?;
    let mut plan = RuntimePlanner::new().plan(&ir)?;
    for buffer in &mut plan.memory_plan.edge_buffers {
        let target_node = ir
            .edges
            .iter()
            .find(|edge| edge.spec.id == buffer.edge)
            .map(|edge| edge.spec.to.node)
            .ok_or("planned edge absent from IR")?;
        buffer.capacity_frames = match cell {
            Cell::SlowRecorder if recorder_nodes.contains(&target_node) => 1,
            Cell::SlowConnector if connector_nodes.contains(&target_node) => 8,
            _ => 64,
        };
    }
    let (router, receivers) = PlanEdgeRouter::new(&plan, &ir)?;
    let mut connector_receivers = Vec::new();
    let mut browser_receivers = Vec::new();
    let mut recorder_receivers = Vec::new();
    for receiver in receivers {
        let target_node = receiver.to().node;
        if connector_nodes.contains(&target_node) {
            connector_receivers.push(receiver);
        } else if browser_nodes.contains(&target_node) {
            browser_receivers.push(receiver);
        } else if recorder_nodes.contains(&target_node) {
            recorder_receivers.push(receiver);
        } else {
            return Err(format!("unexpected endpoint node {}", target_node.index()).into());
        }
    }
    Ok(Topology {
        router,
        application_source,
        microphone_source,
        connector_receivers,
        browser_receivers,
        recorder_receivers,
    })
}

#[derive(Clone, Copy)]
struct DestinationBehavior {
    delay_ms: u64,
    fail_after_frames: Option<u64>,
}

struct DestinationWorker {
    stop_requested: Arc<AtomicBool>,
    join_handle: JoinHandle<DestinationWorkerOutcome>,
}

impl DestinationWorker {
    fn spawn(label: String, mut receiver: PlanEdgeReceiver, behavior: DestinationBehavior) -> Self {
        let edge_id = receiver.edge_id();
        let stop_requested = Arc::new(AtomicBool::new(false));
        let worker_stop = Arc::clone(&stop_requested);
        let join_handle = thread::Builder::new()
            .name(format!("pks-proof-{label}"))
            .spawn(move || {
                let mut delivered_frames = 0u64;
                let mut failed = false;
                loop {
                    if let Some(_frame) = receiver.recv_at(pks_capture::monotonic_timestamp_ns()) {
                        delivered_frames = delivered_frames.saturating_add(1);
                        if behavior
                            .fail_after_frames
                            .is_some_and(|limit| delivered_frames >= limit)
                        {
                            receiver.mark_worker_failure();
                            failed = true;
                            break;
                        }
                        if behavior.delay_ms > 0 {
                            thread::sleep(Duration::from_millis(behavior.delay_ms));
                        }
                        continue;
                    }
                    if worker_stop.load(Ordering::Acquire) {
                        break;
                    }
                    thread::park_timeout(Duration::from_millis(1));
                }
                DestinationWorkerOutcome {
                    label,
                    edge_id,
                    delivered_frames,
                    failed,
                    observations: receiver.observations(),
                }
            })
            .expect("destination worker thread");
        Self {
            stop_requested,
            join_handle,
        }
    }

    fn stop(self) -> DestinationWorkerOutcome {
        self.stop_requested.store(true, Ordering::Release);
        self.join_handle.thread().unpark();
        self.join_handle.join().expect("destination worker")
    }
}

struct DestinationWorkerOutcome {
    label: String,
    edge_id: EdgeId,
    delivered_frames: u64,
    failed: bool,
    observations: EdgeObservations,
}

#[derive(Serialize)]
struct DestinationOutcome {
    label: String,
    delivered_frames: u64,
    failed: bool,
    observations: ObservationSummary,
}

#[derive(Clone, Copy, Serialize)]
struct ObservationSummary {
    queue_capacity_frames: u64,
    queue_peak_frames: u64,
    frames_enqueued_total: u64,
    frames_delivered_total: u64,
    frames_dropped_total: u64,
    overruns_total: u64,
    worker_failures_total: u64,
}

impl From<EdgeObservations> for ObservationSummary {
    fn from(value: EdgeObservations) -> Self {
        Self {
            queue_capacity_frames: value.queue_capacity_frames,
            queue_peak_frames: value.queue_peak_frames,
            frames_enqueued_total: value.frames_enqueued_total,
            frames_delivered_total: value.frames_delivered_total,
            frames_dropped_total: value.frames_dropped_total,
            overruns_total: value.overruns_total,
            worker_failures_total: value.worker_failures_total,
        }
    }
}

#[derive(Serialize)]
struct CellSummary {
    cell: Cell,
    duration_seconds: u64,
    generated_frames_per_stem: u64,
    recording_state: String,
    recording_session_dir: String,
    destinations: Vec<DestinationOutcome>,
}

fn recorder_config(label: &str, source_id: u64, stem_id: u64, clock_id: u32) -> RecorderStemConfig {
    RecorderStemConfig {
        session_id: SessionId(5_000),
        source_id: SourceId(source_id),
        stem_id: StemId(stem_id),
        clock_id: ClockDomainId(clock_id),
        source_generation: 1,
        permission_epoch: 1,
        permission: PermissionDecision::Allowed,
        label: StemLabel::new(label).expect("static stem label"),
        sample_rate_hz: SAMPLE_RATE_HZ,
        channels: CHANNELS,
        timeline_mapping: TimelineMapping::new(0, 0),
    }
}

fn dispatch_frame(
    router: &mut PlanEdgeRouter,
    source_node: NodeId,
    source_id: u64,
    sequence_number: u64,
    timestamp_ns: u64,
    frequency_hz: f32,
    pool: &Arc<AudioBufferPool>,
) -> Result<(), Box<dyn Error>> {
    let mut buffer = pool.acquire().ok_or("capture pool exhausted")?;
    pks_audio::fill_sine(
        buffer.as_mut_slice(),
        SAMPLE_RATE_HZ,
        frequency_hz,
        sequence_number.saturating_mul(FRAME_SAMPLES as u64),
    );
    let frame = AudioFrame::new(
        StreamId(source_id),
        SourceId(source_id),
        sequence_number,
        timestamp_ns,
        CHANNELS,
        buffer,
    );
    router.dispatch_from(
        source_node,
        "out",
        frame,
        pks_capture::monotonic_timestamp_ns(),
    );
    Ok(())
}

fn write_summary(path: &Path, summary: &CellSummary) -> Result<(), Box<dyn Error>> {
    let file = std::fs::File::create(path)?;
    serde_json::to_writer_pretty(file, summary)?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse().map_err(|error| format!("argument error: {error}"))?;
    if args.output.exists() {
        return Err(format!("output already exists: {}", args.output.display()).into());
    }
    std::fs::create_dir_all(&args.output)?;
    let mut topology = build_topology(args.cell)?;

    let connector_behavior = DestinationBehavior {
        delay_ms: if args.cell == Cell::SlowConnector {
            40
        } else {
            0
        },
        fail_after_frames: (args.cell == Cell::ConnectorFailure).then_some(5),
    };
    let mut destination_workers = Vec::new();
    for (index, receiver) in topology.connector_receivers.drain(..).enumerate() {
        destination_workers.push(DestinationWorker::spawn(
            format!("connector-{index}"),
            receiver,
            connector_behavior,
        ));
    }
    for (index, receiver) in topology.browser_receivers.drain(..).enumerate() {
        destination_workers.push(DestinationWorker::spawn(
            format!("browser-{index}"),
            receiver,
            DestinationBehavior {
                delay_ms: 0,
                fail_after_frames: None,
            },
        ));
    }

    let application_pool = AudioBufferPool::new(2, FRAME_SAMPLES);
    let microphone_pool = AudioBufferPool::new(2, FRAME_SAMPLES);
    let mut sequence_number = 0u64;
    if args.cell == Cell::SlowRecorder {
        for _ in 0..2 {
            let timestamp_ns = sequence_number.saturating_mul(FRAME_DURATION_NS);
            dispatch_frame(
                &mut topology.router,
                topology.application_source,
                1,
                sequence_number,
                timestamp_ns,
                440.0,
                &application_pool,
            )?;
            dispatch_frame(
                &mut topology.router,
                topology.microphone_source,
                2,
                sequence_number,
                timestamp_ns,
                220.0,
                &microphone_pool,
            )?;
            sequence_number = sequence_number.saturating_add(1);
        }
    }

    let application_recording_source = if args.cell == Cell::RecorderFailure {
        99
    } else {
        1
    };
    let mut recorder_receivers = topology.recorder_receivers.drain(..);
    let recording = MultistemRecording::start(
        &args.output,
        SessionId(5_000),
        vec![
            (
                recorder_config("application", application_recording_source, 11, 101),
                recorder_receivers
                    .next()
                    .ok_or("missing application recorder edge")?,
            ),
            (
                recorder_config("microphone", 2, 12, 102),
                recorder_receivers
                    .next()
                    .ok_or("missing microphone recorder edge")?,
            ),
        ],
    )?;

    let frames_to_generate = args
        .duration_seconds
        .saturating_mul(1_000 / FRAME_DURATION_MS);
    let stop_sequence_number = sequence_number.saturating_add(frames_to_generate);
    let run_started = Instant::now();
    while sequence_number < stop_sequence_number {
        let timestamp_ns = sequence_number.saturating_mul(FRAME_DURATION_NS);
        dispatch_frame(
            &mut topology.router,
            topology.application_source,
            1,
            sequence_number,
            timestamp_ns,
            440.0,
            &application_pool,
        )?;
        dispatch_frame(
            &mut topology.router,
            topology.microphone_source,
            2,
            sequence_number,
            timestamp_ns,
            220.0,
            &microphone_pool,
        )?;
        sequence_number = sequence_number.saturating_add(1);
        let target_elapsed = Duration::from_nanos(
            sequence_number
                .saturating_sub(if args.cell == Cell::SlowRecorder {
                    2
                } else {
                    0
                })
                .saturating_mul(FRAME_DURATION_NS),
        );
        if let Some(wait) = target_elapsed.checked_sub(run_started.elapsed()) {
            thread::sleep(wait);
        }
    }

    let recording_outcome = recording.finish()?;
    let destinations = destination_workers
        .into_iter()
        .map(DestinationWorker::stop)
        .map(|outcome| DestinationOutcome {
            label: outcome.label,
            delivered_frames: outcome.delivered_frames,
            failed: outcome.failed,
            observations: topology
                .router
                .observations(outcome.edge_id)
                .unwrap_or(outcome.observations)
                .into(),
        })
        .collect::<Vec<_>>();
    let expected_recording_state = if args.cell == Cell::RecorderFailure {
        RecordingState::Incomplete
    } else {
        RecordingState::Complete
    };
    if recording_outcome.state != expected_recording_state {
        return Err(format!(
            "recording state {:?}, expected {:?}",
            recording_outcome.state, expected_recording_state
        )
        .into());
    }
    let browser_failure = destinations
        .iter()
        .filter(|destination| destination.label.starts_with("browser"))
        .any(|destination| destination.failed || destination.observations.frames_dropped_total > 0);
    if browser_failure {
        return Err("healthy browser destination failed or dropped frames".into());
    }
    let connector_destinations = destinations
        .iter()
        .filter(|destination| destination.label.starts_with("connector"));
    match args.cell {
        Cell::SlowConnector => {
            if !connector_destinations
                .clone()
                .all(|destination| destination.observations.frames_dropped_total > 0)
            {
                return Err("slow connector cell did not produce connector-only drops".into());
            }
        }
        Cell::ConnectorFailure => {
            if !connector_destinations.clone().all(|destination| {
                destination.failed && destination.observations.worker_failures_total == 1
            }) {
                return Err("connector failure cell did not expose every worker failure".into());
            }
        }
        Cell::Normal | Cell::SlowRecorder | Cell::RecorderFailure => {
            if connector_destinations.clone().any(|destination| {
                destination.failed || destination.observations.frames_dropped_total > 0
            }) {
                return Err("healthy connector destination failed or dropped frames".into());
            }
        }
    }
    if args.cell == Cell::SlowRecorder {
        let recorder_metrics = std::fs::read_to_string(
            recording_outcome
                .session_dir
                .join("metrics")
                .join("destinations.jsonl"),
        )?;
        let recorder_dropped = recorder_metrics
            .lines()
            .map(serde_json::from_str::<serde_json::Value>)
            .collect::<Result<Vec<_>, _>>()?
            .iter()
            .all(|metrics| metrics["frames_dropped_total"].as_u64().unwrap_or(0) > 0);
        if !recorder_dropped {
            return Err("slow recorder cell did not produce recorder-only drops".into());
        }
    }

    let summary = CellSummary {
        cell: args.cell,
        duration_seconds: args.duration_seconds,
        generated_frames_per_stem: sequence_number,
        recording_state: format!("{:?}", recording_outcome.state).to_lowercase(),
        recording_session_dir: recording_outcome.session_dir.display().to_string(),
        destinations,
    };
    write_summary(&args.output.join("cell-summary.json"), &summary)?;
    println!("{}", serde_json::to_string_pretty(&summary)?);
    Ok(())
}
