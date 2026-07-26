//! Bounded multi-source ingress for one realtime plan executor.
//!
//! The runner is polled by its owning processing thread. It does not spawn a
//! thread, block, or publish Session lifecycle state.

use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;

use pks_frame::AudioFrame;
use pks_graph::spec::NodeId;
use rtrb::{Consumer, Producer, RingBuffer};

use crate::executor::ExecError;
use crate::plan_executor::{PlanExecutionSummary, RealtimePlanExecutor};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlanRunnerDrainPolicy {
    DrainQueued,
    DiscardQueued,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct PlanSourceInputObservations {
    pub queue_capacity_frames: u64,
    pub queue_depth_frames: u64,
    pub queue_peak_frames: u64,
    pub frames_enqueued_total: u64,
    pub frames_delivered_total: u64,
    pub frames_rejected_full_total: u64,
    pub frames_rejected_cancelled_total: u64,
    pub frames_discarded_total: u64,
}

struct PlanSourceInputTelemetry {
    queue_capacity_frames: u64,
    frames_enqueued_total: AtomicU64,
    frames_delivered_total: AtomicU64,
    frames_rejected_full_total: AtomicU64,
    frames_rejected_cancelled_total: AtomicU64,
    frames_discarded_total: AtomicU64,
    queue_peak_frames: AtomicU64,
}

impl PlanSourceInputTelemetry {
    fn new(queue_capacity_frames: usize) -> Self {
        Self {
            queue_capacity_frames: queue_capacity_frames as u64,
            frames_enqueued_total: AtomicU64::new(0),
            frames_delivered_total: AtomicU64::new(0),
            frames_rejected_full_total: AtomicU64::new(0),
            frames_rejected_cancelled_total: AtomicU64::new(0),
            frames_discarded_total: AtomicU64::new(0),
            queue_peak_frames: AtomicU64::new(0),
        }
    }

    fn queue_depth_frames(&self) -> u64 {
        self.frames_enqueued_total
            .load(Ordering::Relaxed)
            .saturating_sub(
                self.frames_delivered_total
                    .load(Ordering::Relaxed)
                    .saturating_add(self.frames_discarded_total.load(Ordering::Relaxed)),
            )
    }

    fn observe_enqueue(&self) {
        self.frames_enqueued_total.fetch_add(1, Ordering::Relaxed);
        self.queue_peak_frames
            .fetch_max(self.queue_depth_frames(), Ordering::Relaxed);
    }

    fn snapshot(&self) -> PlanSourceInputObservations {
        PlanSourceInputObservations {
            queue_capacity_frames: self.queue_capacity_frames,
            queue_depth_frames: self.queue_depth_frames(),
            queue_peak_frames: self.queue_peak_frames.load(Ordering::Relaxed),
            frames_enqueued_total: self.frames_enqueued_total.load(Ordering::Relaxed),
            frames_delivered_total: self.frames_delivered_total.load(Ordering::Relaxed),
            frames_rejected_full_total: self.frames_rejected_full_total.load(Ordering::Relaxed),
            frames_rejected_cancelled_total: self
                .frames_rejected_cancelled_total
                .load(Ordering::Relaxed),
            frames_discarded_total: self.frames_discarded_total.load(Ordering::Relaxed),
        }
    }
}

#[derive(Clone)]
pub struct PlanRunnerCancellation {
    requested: Arc<AtomicBool>,
}

impl PlanRunnerCancellation {
    pub fn new() -> Self {
        Self {
            requested: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn request(&self) -> bool {
        !self.requested.swap(true, Ordering::AcqRel)
    }

    pub fn is_requested(&self) -> bool {
        self.requested.load(Ordering::Acquire)
    }
}

impl Default for PlanRunnerCancellation {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub enum PlanSourceSendError {
    Cancelled(AudioFrame),
    Full(AudioFrame),
}

pub struct PlanSourceSender {
    producer: Producer<AudioFrame>,
    cancellation: PlanRunnerCancellation,
    telemetry: Arc<PlanSourceInputTelemetry>,
}

impl PlanSourceSender {
    pub fn try_send(&mut self, frame: AudioFrame) -> Result<(), PlanSourceSendError> {
        if self.cancellation.is_requested() {
            self.telemetry
                .frames_rejected_cancelled_total
                .fetch_add(1, Ordering::Relaxed);
            return Err(PlanSourceSendError::Cancelled(frame));
        }
        match self.producer.push(frame) {
            Ok(()) => {
                self.telemetry.observe_enqueue();
                Ok(())
            }
            Err(rtrb::PushError::Full(frame)) => {
                self.telemetry
                    .frames_rejected_full_total
                    .fetch_add(1, Ordering::Relaxed);
                Err(PlanSourceSendError::Full(frame))
            }
        }
    }

    pub fn observations(&self) -> PlanSourceInputObservations {
        self.telemetry.snapshot()
    }
}

pub struct PlanSourceInput {
    source_node_id: NodeId,
    consumer: Consumer<AudioFrame>,
    telemetry: Arc<PlanSourceInputTelemetry>,
}

impl PlanSourceInput {
    pub fn source_node_id(&self) -> NodeId {
        self.source_node_id
    }

    pub fn observations(&self) -> PlanSourceInputObservations {
        self.telemetry.snapshot()
    }

    fn try_recv(&mut self) -> Option<AudioFrame> {
        let frame = self.consumer.pop().ok()?;
        self.telemetry
            .frames_delivered_total
            .fetch_add(1, Ordering::Relaxed);
        Some(frame)
    }

    fn discard_queued(&mut self) -> u64 {
        let mut discarded_frames = 0u64;
        while self.consumer.pop().is_ok() {
            discarded_frames = discarded_frames.saturating_add(1);
        }
        self.telemetry
            .frames_discarded_total
            .fetch_add(discarded_frames, Ordering::Relaxed);
        discarded_frames
    }
}

pub fn plan_source_channel(
    source_node_id: NodeId,
    queue_capacity_frames: usize,
    cancellation: PlanRunnerCancellation,
) -> Result<(PlanSourceSender, PlanSourceInput), PlanRunnerError> {
    if queue_capacity_frames == 0 {
        return Err(PlanRunnerError::ZeroSourceCapacity {
            source_node_id: source_node_id.index(),
        });
    }
    let (producer, consumer) = RingBuffer::new(queue_capacity_frames);
    let telemetry = Arc::new(PlanSourceInputTelemetry::new(queue_capacity_frames));
    Ok((
        PlanSourceSender {
            producer,
            cancellation,
            telemetry: Arc::clone(&telemetry),
        },
        PlanSourceInput {
            source_node_id,
            consumer,
            telemetry,
        },
    ))
}

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum PlanRunnerError {
    #[error("source node {source_node_id} has zero input capacity")]
    ZeroSourceCapacity { source_node_id: u32 },
    #[error("source node {source_node_id} is registered more than once")]
    DuplicateSource { source_node_id: u32 },
    #[error("runner work budget must be greater than zero")]
    ZeroWorkBudget,
    #[error("runner was already finished")]
    AlreadyFinished,
    #[error(transparent)]
    Execution(#[from] ExecError),
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct PlanRunnerStepSummary {
    pub source_frames_processed_total: u64,
    pub execution: PlanExecutionSummary,
}

impl PlanRunnerStepSummary {
    fn observe_execution(&mut self, execution: PlanExecutionSummary) {
        self.source_frames_processed_total = self.source_frames_processed_total.saturating_add(1);
        self.execution.nodes_executed = self
            .execution
            .nodes_executed
            .saturating_add(execution.nodes_executed);
        self.execution.edges_attempted = self
            .execution
            .edges_attempted
            .saturating_add(execution.edges_attempted);
        self.execution.edges_enqueued = self
            .execution
            .edges_enqueued
            .saturating_add(execution.edges_enqueued);
        self.execution.edges_dropped = self
            .execution
            .edges_dropped
            .saturating_add(execution.edges_dropped);
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct PlanRunnerFinishSummary {
    pub source_frames_processed_total: u64,
    pub source_frames_discarded_total: u64,
    pub drain_budget_exhausted: bool,
    pub execution: PlanExecutionSummary,
}

pub struct RealtimePlanRunner {
    executor: RealtimePlanExecutor,
    sources: Vec<PlanSourceInput>,
    cancellation: PlanRunnerCancellation,
    next_source_index: usize,
    finished: bool,
}

impl RealtimePlanRunner {
    pub fn new(
        executor: RealtimePlanExecutor,
        sources: Vec<PlanSourceInput>,
        cancellation: PlanRunnerCancellation,
    ) -> Result<Self, PlanRunnerError> {
        for (index, source) in sources.iter().enumerate() {
            if sources[..index]
                .iter()
                .any(|existing| existing.source_node_id == source.source_node_id)
            {
                return Err(PlanRunnerError::DuplicateSource {
                    source_node_id: source.source_node_id.index(),
                });
            }
        }
        Ok(Self {
            executor,
            sources,
            cancellation,
            next_source_index: 0,
            finished: false,
        })
    }

    pub fn process_ready(
        &mut self,
        work_budget_frames: usize,
    ) -> Result<PlanRunnerStepSummary, PlanRunnerError> {
        if self.cancellation.is_requested() {
            return Ok(PlanRunnerStepSummary::default());
        }
        self.process_ready_with_clock(work_budget_frames, pks_timing::monotonic_timestamp_ns)
    }

    pub fn source_observations(
        &self,
        source_node_id: NodeId,
    ) -> Option<PlanSourceInputObservations> {
        self.sources
            .iter()
            .find(|source| source.source_node_id == source_node_id)
            .map(PlanSourceInput::observations)
    }

    pub fn finish(
        &mut self,
        drain_policy: PlanRunnerDrainPolicy,
        work_budget_frames: usize,
    ) -> Result<PlanRunnerFinishSummary, PlanRunnerError> {
        self.finish_with_clock(
            drain_policy,
            work_budget_frames,
            pks_timing::monotonic_timestamp_ns,
        )
    }

    fn process_ready_with_clock(
        &mut self,
        work_budget_frames: usize,
        mut clock: impl FnMut() -> u64,
    ) -> Result<PlanRunnerStepSummary, PlanRunnerError> {
        if self.finished {
            return Err(PlanRunnerError::AlreadyFinished);
        }
        if work_budget_frames == 0 {
            return Err(PlanRunnerError::ZeroWorkBudget);
        }

        let mut summary = PlanRunnerStepSummary::default();
        while summary.source_frames_processed_total < work_budget_frames as u64 {
            let Some((source_node_id, frame)) = self.next_ready_source() else {
                break;
            };
            let execution = self.executor.execute_from(source_node_id, frame, clock())?;
            summary.observe_execution(execution);
        }
        Ok(summary)
    }

    fn finish_with_clock(
        &mut self,
        drain_policy: PlanRunnerDrainPolicy,
        work_budget_frames: usize,
        clock: impl FnMut() -> u64,
    ) -> Result<PlanRunnerFinishSummary, PlanRunnerError> {
        if self.finished {
            return Err(PlanRunnerError::AlreadyFinished);
        }
        self.cancellation.request();

        let processed = match drain_policy {
            PlanRunnerDrainPolicy::DrainQueued if work_budget_frames > 0 => {
                self.process_ready_with_clock(work_budget_frames, clock)?
            }
            PlanRunnerDrainPolicy::DrainQueued | PlanRunnerDrainPolicy::DiscardQueued => {
                PlanRunnerStepSummary::default()
            }
        };
        let queued_before_discard = self
            .sources
            .iter()
            .map(|source| source.observations().queue_depth_frames)
            .sum::<u64>();
        let source_frames_discarded_total = self
            .sources
            .iter_mut()
            .map(PlanSourceInput::discard_queued)
            .sum();
        self.finished = true;
        Ok(PlanRunnerFinishSummary {
            source_frames_processed_total: processed.source_frames_processed_total,
            source_frames_discarded_total,
            drain_budget_exhausted: drain_policy == PlanRunnerDrainPolicy::DrainQueued
                && queued_before_discard > 0,
            execution: processed.execution,
        })
    }

    fn next_ready_source(&mut self) -> Option<(NodeId, AudioFrame)> {
        if self.sources.is_empty() {
            return None;
        }
        for _ in 0..self.sources.len() {
            let source_index = self.next_source_index;
            self.next_source_index = (self.next_source_index + 1) % self.sources.len();
            let source = &mut self.sources[source_index];
            if let Some(frame) = source.try_recv() {
                return Some((source.source_node_id, frame));
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use pks_caps::{AudioCaps, ChannelLayout, MediaCaps, Multiplicity, PortDirection, PortSpec};
    use pks_frame::{AudioBufferPool, SampleFormat, SampleSpec, SourceId, StreamId};
    use pks_graph::compiler::Compiler;
    use pks_graph::dsl::Pipeline;
    use pks_graph::node::{ConfigError, NodeConfig, NodeDescriptor, NodeError, NodeTypeId};
    use pks_graph::partition::ExecutionPartition;
    use pks_graph::planner::RuntimePlanner;
    use pks_graph::registry::{NodeFactory, NodeRegistry};
    use pks_graph::{register_builtins, PrepareContext, RuntimeNode};

    use super::*;
    use crate::PlanEdgeFrame;

    struct WorkerSinkFactory(&'static str);

    impl NodeFactory for WorkerSinkFactory {
        fn descriptor(&self) -> NodeDescriptor {
            NodeDescriptor {
                type_id: NodeTypeId::from(self.0),
                display_name: "Runner test sink",
                inputs: vec![PortSpec {
                    name: "in".to_owned(),
                    direction: PortDirection::Input,
                    media: MediaCaps::Audio(AudioCaps {
                        sample_rate_hz: Some(48_000),
                        frame_samples: Some(1),
                        channel_layout: ChannelLayout::Mono,
                        format: SampleFormat::F32Interleaved,
                    }),
                    multiplicity: Multiplicity::One,
                    required: true,
                }],
                outputs: Vec::new(),
                execution: ExecutionPartition::AsyncWorker,
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
            Err(NodeError::Prepare(
                "worker sink is not instantiated by realtime runner".to_owned(),
            ))
        }
    }

    fn frame(source_id: u64, sequence_number: u64) -> AudioFrame {
        let pool = AudioBufferPool::new(1, 1);
        let mut buffer = pool.acquire().unwrap();
        buffer.as_mut_slice()[0] = source_id as f32;
        AudioFrame::new(
            StreamId(source_id),
            SourceId(source_id),
            sequence_number,
            sequence_number.saturating_add(1),
            1,
            buffer,
        )
    }

    type RunnerFixture = (
        RealtimePlanRunner,
        PlanRunnerCancellation,
        Vec<PlanSourceSender>,
        Vec<crate::PlanEdgeReceiver>,
        Vec<NodeId>,
    );

    fn runner_fixture() -> RunnerFixture {
        let mut registry = NodeRegistry::new();
        register_builtins(&mut registry);
        registry.register(Arc::new(WorkerSinkFactory("test.sink.left")));
        registry.register(Arc::new(WorkerSinkFactory("test.sink.right")));

        let mut pipeline = Pipeline::new();
        let left_source = pipeline.add_node("passthrough", NodeConfig::new());
        let right_source = pipeline.add_node("passthrough", NodeConfig::new());
        let left_sink = pipeline.add_node("test.sink.left", NodeConfig::new());
        let right_sink = pipeline.add_node("test.sink.right", NodeConfig::new());
        pipeline.connect(left_source.out("out"), left_sink.in_("in"));
        pipeline.connect(right_source.out("out"), right_sink.in_("in"));
        let ir = Compiler::new()
            .compile(pipeline.into_spec(), &registry)
            .unwrap();
        let plan = RuntimePlanner::new().plan(&ir).unwrap();
        let context = PrepareContext::new(SampleSpec::new(48_000, 1, SampleFormat::F32Interleaved));
        let (executor, workers) =
            RealtimePlanExecutor::new(&plan, &ir, &registry, &context).unwrap();
        let cancellation = PlanRunnerCancellation::new();
        let (left_sender, left_input) =
            plan_source_channel(left_source.id(), 4, cancellation.clone()).unwrap();
        let (right_sender, right_input) =
            plan_source_channel(right_source.id(), 4, cancellation.clone()).unwrap();
        let runner = RealtimePlanRunner::new(
            executor,
            vec![left_input, right_input],
            cancellation.clone(),
        )
        .unwrap();
        (
            runner,
            cancellation,
            vec![left_sender, right_sender],
            workers,
            vec![left_source.id(), right_source.id()],
        )
    }

    #[test]
    fn given_two_ready_sources_when_processed_then_each_source_dispatches_independently() {
        let (mut runner, _cancellation, mut senders, mut workers, source_ids) = runner_fixture();
        senders[0].try_send(frame(11, 0)).ok().unwrap();
        senders[1].try_send(frame(22, 0)).ok().unwrap();

        let summary = runner.process_ready_with_clock(2, || 100).unwrap();
        let delivered = workers
            .iter_mut()
            .map(|worker| worker.recv_at(101).unwrap())
            .map(|frame| match frame {
                PlanEdgeFrame::Exclusive(frame) => frame.source_id.0,
                PlanEdgeFrame::Shared(frame) => frame.source_id.0,
            })
            .collect::<Vec<_>>();

        assert_eq!(summary.source_frames_processed_total, 2);
        assert_eq!(summary.execution.edges_enqueued, 2);
        assert_eq!(delivered, vec![11, 22]);
        assert_eq!(
            runner
                .source_observations(source_ids[0])
                .unwrap()
                .frames_delivered_total,
            1
        );
        assert_eq!(
            runner
                .source_observations(source_ids[1])
                .unwrap()
                .frames_delivered_total,
            1
        );
    }

    #[test]
    fn given_queued_sources_when_cancelled_with_budget_then_drain_is_bounded_and_rest_discards() {
        let (mut runner, cancellation, mut senders, _workers, source_ids) = runner_fixture();
        for sequence_number in 0..3 {
            senders[0]
                .try_send(frame(11, sequence_number))
                .ok()
                .unwrap();
            senders[1]
                .try_send(frame(22, sequence_number))
                .ok()
                .unwrap();
        }

        let summary = runner
            .finish_with_clock(PlanRunnerDrainPolicy::DrainQueued, 2, || 100)
            .unwrap();
        let rejected = senders[0].try_send(frame(11, 4));

        assert!(cancellation.is_requested());
        assert_eq!(summary.source_frames_processed_total, 2);
        assert_eq!(summary.source_frames_discarded_total, 4);
        assert!(summary.drain_budget_exhausted);
        assert!(matches!(rejected, Err(PlanSourceSendError::Cancelled(_))));
        assert_eq!(
            runner
                .source_observations(source_ids[0])
                .unwrap()
                .queue_depth_frames,
            0
        );
        assert_eq!(
            runner
                .source_observations(source_ids[1])
                .unwrap()
                .queue_depth_frames,
            0
        );
    }

    #[test]
    fn given_queued_sources_when_cancelled_with_discard_then_no_frame_executes() {
        let (mut runner, _cancellation, mut senders, _workers, _source_ids) = runner_fixture();
        senders[0].try_send(frame(11, 0)).ok().unwrap();
        senders[1].try_send(frame(22, 0)).ok().unwrap();

        let summary = runner
            .finish_with_clock(PlanRunnerDrainPolicy::DiscardQueued, 0, || 100)
            .unwrap();

        assert_eq!(summary.source_frames_processed_total, 0);
        assert_eq!(summary.source_frames_discarded_total, 2);
        assert!(!summary.drain_budget_exhausted);
        assert_eq!(summary.execution, PlanExecutionSummary::default());
    }

    #[test]
    fn given_full_source_input_when_more_frames_arrive_then_newest_rejects_and_counts() {
        let (_runner, _fixture_cancellation, _senders, _workers, source_ids) = runner_fixture();
        let cancellation = PlanRunnerCancellation::new();
        let (mut sender, input) =
            plan_source_channel(source_ids[0], 1, cancellation.clone()).unwrap();
        sender.try_send(frame(11, 0)).ok().unwrap();

        let full_result = sender.try_send(frame(11, 1));
        cancellation.request();
        let cancelled_result = sender.try_send(frame(11, 2));
        let observations = input.observations();

        assert!(matches!(full_result, Err(PlanSourceSendError::Full(_))));
        assert!(matches!(
            cancelled_result,
            Err(PlanSourceSendError::Cancelled(_))
        ));
        assert_eq!(observations.queue_capacity_frames, 1);
        assert_eq!(observations.queue_depth_frames, 1);
        assert_eq!(observations.frames_enqueued_total, 1);
        assert_eq!(observations.frames_rejected_full_total, 1);
        assert_eq!(observations.frames_rejected_cancelled_total, 1);
    }
}
