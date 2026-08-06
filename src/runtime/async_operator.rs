use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Duration;

use crate::frame::{AudioFrame, SessionId, SourceId, StemId};
use crate::graph::{
    AsyncEnvelope, AsyncNode, AsyncOperatorFactory, AsyncOperatorManifest, AsyncSignal,
    EdgeContract, LossPolicy, NodeConfig, NodeError, OperatorCancellationPolicy,
    OperatorFailurePolicy, PrepareContext, TRANSCRIPT_FINAL_ROLE, TRANSCRIPT_PARTIAL_ROLE,
};
use crate::graph::{EdgeId, NodeId};
use rtrb::{Consumer, Producer, RingBuffer};
use tokio::sync::Notify;
use tokio::task::JoinHandle;

use crate::runtime::{AsyncBridge, AsyncBridgeReceiver, AsyncBridgeSendError, AsyncBridgeSender};
use crate::runtime::{PlanEdgeFrame, PlanEdgeReceiver};

const ASYNC_OPERATOR_IDLE_POLL_INTERVAL: Duration = Duration::from_millis(1);

#[derive(Default)]
struct AsyncOperatorObservationState {
    input_attempted_total: AtomicU64,
    input_dropped_total: AtomicU64,
    processed_total: AtomicU64,
    output_emitted_total: AtomicU64,
    output_dropped_total: AtomicU64,
    transcript_partial_total: AtomicU64,
    transcript_final_total: AtomicU64,
    process_failure_total: AtomicU64,
    timeout_total: AtomicU64,
    cancellation_total: AtomicU64,
    graceful_finish_total: AtomicU64,
    idle_poll_total: AtomicU64,
    ready: AtomicBool,
    joined: AtomicBool,
    ready_notify: Notify,
    terminal_notify: Notify,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AsyncOperatorObservations {
    pub input_attempted_total: u64,
    pub input_dropped_total: u64,
    pub processed_total: u64,
    pub output_emitted_total: u64,
    pub output_dropped_total: u64,
    pub transcript_partial_total: u64,
    pub transcript_final_total: u64,
    pub process_failure_total: u64,
    pub timeout_total: u64,
    pub cancellation_total: u64,
    pub graceful_finish_total: u64,
    pub idle_poll_total: u64,
    pub ready: bool,
    pub joined: bool,
}

#[derive(Clone)]
pub struct AsyncOperatorObservationHandle {
    state: Arc<AsyncOperatorObservationState>,
}

impl AsyncOperatorObservationHandle {
    pub fn snapshot(&self) -> AsyncOperatorObservations {
        AsyncOperatorObservations {
            input_attempted_total: self.state.input_attempted_total.load(Ordering::Relaxed),
            input_dropped_total: self.state.input_dropped_total.load(Ordering::Relaxed),
            processed_total: self.state.processed_total.load(Ordering::Relaxed),
            output_emitted_total: self.state.output_emitted_total.load(Ordering::Relaxed),
            output_dropped_total: self.state.output_dropped_total.load(Ordering::Relaxed),
            transcript_partial_total: self.state.transcript_partial_total.load(Ordering::Relaxed),
            transcript_final_total: self.state.transcript_final_total.load(Ordering::Relaxed),
            process_failure_total: self.state.process_failure_total.load(Ordering::Relaxed),
            timeout_total: self.state.timeout_total.load(Ordering::Relaxed),
            cancellation_total: self.state.cancellation_total.load(Ordering::Relaxed),
            graceful_finish_total: self.state.graceful_finish_total.load(Ordering::Relaxed),
            idle_poll_total: self.state.idle_poll_total.load(Ordering::Relaxed),
            ready: self.state.ready.load(Ordering::Acquire),
            joined: self.state.joined.load(Ordering::Acquire),
        }
    }

    pub async fn wait_ready(&self) -> bool {
        loop {
            let notified = self.state.ready_notify.notified();
            if self.state.ready.load(Ordering::Acquire) {
                return true;
            }
            if self.state.joined.load(Ordering::Acquire) {
                return false;
            }
            notified.await;
        }
    }

    pub async fn wait_terminal(&self) {
        loop {
            let notified = self.state.terminal_notify.notified();
            if self.state.joined.load(Ordering::Acquire) {
                return;
            }
            notified.await;
        }
    }
}

pub struct AsyncOperatorInput {
    sender: AsyncBridgeSender,
    observations: Arc<AsyncOperatorObservationState>,
}

#[derive(Debug, thiserror::Error)]
#[error("this async operator consumes a compiled plan edge and has no direct input sender")]
pub struct AsyncOperatorInputAccessError;

impl AsyncOperatorInput {
    pub fn send(&mut self, envelope: AsyncEnvelope) -> Result<(), AsyncBridgeSendError> {
        self.observations
            .input_attempted_total
            .fetch_add(1, Ordering::Relaxed);
        let result = self.sender.send(envelope);
        if result.is_err() {
            self.observations
                .input_dropped_total
                .fetch_add(1, Ordering::Relaxed);
        }
        result
    }

    pub fn send_audio(
        &mut self,
        frame: AudioFrame,
        sequence_number: u64,
        timestamp_ns: u64,
    ) -> Result<(), AudioFrame> {
        self.observations
            .input_attempted_total
            .fetch_add(1, Ordering::Relaxed);
        let result = self.sender.send_audio(frame, sequence_number, timestamp_ns);
        if result.is_err() {
            self.observations
                .input_dropped_total
                .fetch_add(1, Ordering::Relaxed);
        }
        result
    }
}

pub struct AsyncOperatorOutput {
    receiver: Consumer<Arc<AsyncEnvelope>>,
    observations: Arc<AsyncOperatorOutputObservationState>,
}

#[derive(Clone)]
pub struct AsyncOperatorOutputObservationHandle {
    observations: Arc<AsyncOperatorOutputObservationState>,
}

#[derive(Debug, Clone, Copy)]
pub struct AsyncOperatorOutputBranchSpec {
    pub capacity_signals: usize,
    pub edge_contract: EdgeContract,
}

#[derive(Default)]
struct AsyncOperatorOutputObservationState {
    delivered_total: AtomicU64,
    dropped_total: AtomicU64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AsyncOperatorOutputObservations {
    pub delivered_total: u64,
    pub dropped_total: u64,
}

impl AsyncOperatorOutput {
    pub fn recv(&mut self) -> Option<Arc<AsyncEnvelope>> {
        self.receiver.pop().ok()
    }

    pub fn observations(&self) -> AsyncOperatorOutputObservations {
        AsyncOperatorOutputObservations {
            delivered_total: self.observations.delivered_total.load(Ordering::Relaxed),
            dropped_total: self.observations.dropped_total.load(Ordering::Relaxed),
        }
    }

    pub fn observation_handle(&self) -> AsyncOperatorOutputObservationHandle {
        AsyncOperatorOutputObservationHandle {
            observations: Arc::clone(&self.observations),
        }
    }
}

impl AsyncOperatorOutputObservationHandle {
    pub fn snapshot(&self) -> AsyncOperatorOutputObservations {
        AsyncOperatorOutputObservations {
            delivered_total: self.observations.delivered_total.load(Ordering::Relaxed),
            dropped_total: self.observations.dropped_total.load(Ordering::Relaxed),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum AsyncOperatorWorkerError {
    #[error("operator prepare failed: {0}")]
    Prepare(NodeError),
    #[error("operator prepare exceeded {timeout_ms} ms")]
    PrepareTimeout { timeout_ms: u32 },
    #[error("operator process failed: {0}")]
    Process(NodeError),
    #[error("operator process exceeded {timeout_ms} ms")]
    Timeout { timeout_ms: u32 },
    #[error("operator close failed: {0}")]
    Close(NodeError),
    #[error("operator close exceeded {timeout_ms} ms")]
    CloseTimeout { timeout_ms: u32 },
    #[error("operator cancellation cleanup failed: {0}")]
    Cancel(NodeError),
    #[error("operator cancellation cleanup exceeded {timeout_ms} ms")]
    CancelTimeout { timeout_ms: u32 },
    #[error("async derived fan-out does not accept audio output")]
    UnsupportedAudioOutput,
    #[error("async operator output has no derived lineage")]
    MissingDerivedLineage,
    #[error("async operator output lineage does not match its registered manifest")]
    DerivedLineageMismatch,
    #[error("async operator output signal does not match its registered output port")]
    OutputSignalMismatch,
    #[error("async operator output role is not declared by its registered manifest")]
    UndeclaredOutputRole,
    #[error("async operator manifest has no output port at runtime")]
    MissingOutputContract,
    #[error("final transcript was rejected by full output branch {branch_index}")]
    FinalOutputDropped { branch_index: usize },
    #[error("operator worker task failed: {0}")]
    Join(#[from] tokio::task::JoinError),
    #[error("compiled async input requires a lineaged exclusive plan-edge frame, received {kind}")]
    InvalidPlanInput { kind: &'static str },
    #[error("compiled async input lineage does not match its Session stem contract")]
    PlanInputLineageMismatch,
}

enum AsyncOperatorWorkerSource {
    Direct(AsyncBridgeReceiver),
    Compiled {
        receiver: PlanEdgeReceiver,
        lineage: CompiledOperatorInputContract,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CompiledOperatorInputContract {
    pub edge_id: EdgeId,
    pub operator_node: NodeId,
    pub session_id: SessionId,
    pub stem_id: StemId,
    pub source_id: Option<SourceId>,
}

impl AsyncOperatorWorkerSource {
    fn recv(&mut self) -> Result<Option<AsyncEnvelope>, AsyncOperatorWorkerError> {
        match self {
            Self::Direct(receiver) => Ok(receiver.recv()),
            Self::Compiled {
                receiver,
                lineage: lineage_contract,
            } => receiver.try_recv().map_or(Ok(None), |frame| match frame {
                PlanEdgeFrame::LineagedExclusive(frame) => {
                    let (frame, lineage) = frame.into_parts();
                    if lineage.session_id != lineage_contract.session_id
                        || lineage.stem_id != lineage_contract.stem_id
                        || lineage_contract
                            .source_id
                            .is_some_and(|source_id| source_id != lineage.source_id)
                    {
                        return Err(AsyncOperatorWorkerError::PlanInputLineageMismatch);
                    }
                    Ok(Some(AsyncEnvelope::from_audio(frame, Some(lineage))))
                }
                PlanEdgeFrame::Exclusive(_) => Err(AsyncOperatorWorkerError::InvalidPlanInput {
                    kind: "raw exclusive",
                }),
                PlanEdgeFrame::Shared(_) => {
                    Err(AsyncOperatorWorkerError::InvalidPlanInput { kind: "raw shared" })
                }
                PlanEdgeFrame::LineagedShared(_) => {
                    Err(AsyncOperatorWorkerError::InvalidPlanInput {
                        kind: "lineaged shared",
                    })
                }
            }),
        }
    }

    fn is_abandoned(&self) -> bool {
        match self {
            Self::Direct(receiver) => receiver.is_abandoned(),
            Self::Compiled { receiver, .. } => receiver.is_abandoned(),
        }
    }
}

pub struct AsyncOperatorWorker {
    input: Option<AsyncOperatorInput>,
    cancellation: Arc<AtomicBool>,
    cancellation_notify: Arc<Notify>,
    observations: AsyncOperatorObservationHandle,
    join: JoinHandle<Result<(), AsyncOperatorWorkerError>>,
}

type AsyncOperatorOutputSender = (
    Producer<Arc<AsyncEnvelope>>,
    Arc<AsyncOperatorOutputObservationState>,
    LossPolicy,
);

fn build_output_branches(
    output_branch_specs: &[AsyncOperatorOutputBranchSpec],
) -> (Vec<AsyncOperatorOutputSender>, Vec<AsyncOperatorOutput>) {
    let mut output_senders = Vec::with_capacity(output_branch_specs.len());
    let mut outputs = Vec::with_capacity(output_branch_specs.len());
    for branch in output_branch_specs {
        let (sender, receiver) = RingBuffer::new(branch.capacity_signals);
        let branch_observations = Arc::new(AsyncOperatorOutputObservationState::default());
        output_senders.push((
            sender,
            Arc::clone(&branch_observations),
            branch.edge_contract.loss,
        ));
        outputs.push(AsyncOperatorOutput {
            receiver,
            observations: branch_observations,
        });
    }
    (output_senders, outputs)
}

impl AsyncOperatorWorker {
    pub fn spawn(
        factory: Arc<dyn AsyncOperatorFactory>,
        configuration: &NodeConfig,
        prepare_context: PrepareContext,
        output_branch_specs: &[AsyncOperatorOutputBranchSpec],
    ) -> Result<(Self, Vec<AsyncOperatorOutput>), NodeError> {
        let manifest = factory.resolve_manifest(configuration)?;
        let (input_sender, input_receiver) = AsyncBridge::new(manifest.queue_capacity_frames);
        let input = AsyncOperatorInput {
            sender: input_sender,
            observations: Arc::new(AsyncOperatorObservationState::default()),
        };
        Self::spawn_with_source(
            factory,
            configuration,
            prepare_context,
            output_branch_specs,
            AsyncOperatorWorkerSource::Direct(input_receiver),
            Some(input),
        )
    }

    pub async fn prepare_and_spawn_from_plan_edge(
        factory: Arc<dyn AsyncOperatorFactory>,
        configuration: &NodeConfig,
        prepare_context: PrepareContext,
        input_receiver: PlanEdgeReceiver,
        input_contract: CompiledOperatorInputContract,
        output_branch_specs: &[AsyncOperatorOutputBranchSpec],
    ) -> Result<(Self, Vec<AsyncOperatorOutput>), AsyncOperatorWorkerError> {
        if input_receiver.edge_id() != input_contract.edge_id
            || input_receiver.to().node != input_contract.operator_node
        {
            return Err(AsyncOperatorWorkerError::Prepare(NodeError::Prepare(
                "compiled async input receiver does not match its edge/node contract".to_owned(),
            )));
        }
        if output_branch_specs.is_empty()
            || output_branch_specs
                .iter()
                .any(|branch| branch.capacity_signals == 0)
        {
            return Err(AsyncOperatorWorkerError::Prepare(NodeError::Prepare(
                "async operator requires non-zero capacity for every output branch".to_owned(),
            )));
        }
        let manifest = factory
            .resolve_manifest(configuration)
            .map_err(NodeError::Config)
            .map_err(AsyncOperatorWorkerError::Prepare)?;
        manifest.validate().map_err(|error| {
            AsyncOperatorWorkerError::Prepare(NodeError::Prepare(error.to_string()))
        })?;
        let mut node = factory
            .create(configuration)
            .map_err(AsyncOperatorWorkerError::Prepare)?;
        let timeout_duration =
            Duration::from_millis(u64::from(manifest.deadline.process_timeout_ms));
        tokio::time::timeout(timeout_duration, node.prepare(&prepare_context))
            .await
            .map_err(|_| AsyncOperatorWorkerError::PrepareTimeout {
                timeout_ms: manifest.deadline.process_timeout_ms,
            })?
            .map_err(AsyncOperatorWorkerError::Prepare)?;

        let observations = Arc::new(AsyncOperatorObservationState::default());
        observations.ready.store(true, Ordering::Release);
        let cancellation = Arc::new(AtomicBool::new(false));
        let cancellation_notify = Arc::new(Notify::new());
        let (output_senders, outputs) = build_output_branches(output_branch_specs);
        let task_observations = Arc::clone(&observations);
        let task_cancellation = Arc::clone(&cancellation);
        let task_cancellation_notify = Arc::clone(&cancellation_notify);
        let join = tokio::spawn(async move {
            let result = run_prepared_worker(
                manifest,
                node,
                AsyncOperatorWorkerSource::Compiled {
                    receiver: input_receiver,
                    lineage: input_contract,
                },
                output_senders,
                task_cancellation,
                task_cancellation_notify,
                Arc::clone(&task_observations),
            )
            .await;
            task_observations.joined.store(true, Ordering::Release);
            task_observations.ready_notify.notify_waiters();
            task_observations.terminal_notify.notify_waiters();
            result
        });
        Ok((
            Self {
                input: None,
                cancellation,
                cancellation_notify,
                observations: AsyncOperatorObservationHandle {
                    state: observations,
                },
                join,
            },
            outputs,
        ))
    }

    fn spawn_with_source(
        factory: Arc<dyn AsyncOperatorFactory>,
        configuration: &NodeConfig,
        prepare_context: PrepareContext,
        output_branch_specs: &[AsyncOperatorOutputBranchSpec],
        input_source: AsyncOperatorWorkerSource,
        mut direct_input: Option<AsyncOperatorInput>,
    ) -> Result<(Self, Vec<AsyncOperatorOutput>), NodeError> {
        if output_branch_specs.is_empty()
            || output_branch_specs
                .iter()
                .any(|branch| branch.capacity_signals == 0)
        {
            return Err(NodeError::Prepare(
                "async operator requires non-zero capacity for every output branch".to_owned(),
            ));
        }
        let manifest = factory.resolve_manifest(configuration)?;
        manifest
            .validate()
            .map_err(|error| NodeError::Prepare(error.to_string()))?;
        let node = factory.create(configuration)?;
        let observations = Arc::new(AsyncOperatorObservationState::default());
        if let Some(input) = direct_input.as_mut() {
            input.observations = Arc::clone(&observations);
        }
        let cancellation = Arc::new(AtomicBool::new(false));
        let cancellation_notify = Arc::new(Notify::new());
        let (output_senders, outputs) = build_output_branches(output_branch_specs);
        let task_observations = Arc::clone(&observations);
        let task_cancellation = Arc::clone(&cancellation);
        let task_cancellation_notify = Arc::clone(&cancellation_notify);
        let join = tokio::spawn(async move {
            let result = run_worker(
                manifest,
                node,
                prepare_context,
                input_source,
                output_senders,
                task_cancellation,
                task_cancellation_notify,
                Arc::clone(&task_observations),
            )
            .await;
            task_observations.joined.store(true, Ordering::Release);
            task_observations.ready_notify.notify_waiters();
            task_observations.terminal_notify.notify_waiters();
            result
        });
        Ok((
            Self {
                input: direct_input,
                cancellation,
                cancellation_notify,
                observations: AsyncOperatorObservationHandle {
                    state: observations,
                },
                join,
            },
            outputs,
        ))
    }

    pub fn input_mut(&mut self) -> Result<&mut AsyncOperatorInput, AsyncOperatorInputAccessError> {
        self.input.as_mut().ok_or(AsyncOperatorInputAccessError)
    }

    pub fn observations(&self) -> AsyncOperatorObservationHandle {
        self.observations.clone()
    }

    pub async fn cancel_and_join(self) -> Result<(), AsyncOperatorWorkerError> {
        self.cancellation.store(true, Ordering::Release);
        self.cancellation_notify.notify_one();
        drop(self.input);
        self.join.await?
    }

    pub async fn finish_and_join(self) -> Result<(), AsyncOperatorWorkerError> {
        drop(self.input);
        self.join.await?
    }
}

#[allow(clippy::too_many_arguments)]
async fn run_worker(
    manifest: AsyncOperatorManifest,
    mut node: Box<dyn AsyncNode>,
    prepare_context: PrepareContext,
    input: AsyncOperatorWorkerSource,
    output_branches: Vec<AsyncOperatorOutputSender>,
    cancellation: Arc<AtomicBool>,
    cancellation_notify: Arc<Notify>,
    observations: Arc<AsyncOperatorObservationState>,
) -> Result<(), AsyncOperatorWorkerError> {
    let timeout_duration = Duration::from_millis(u64::from(manifest.deadline.process_timeout_ms));
    tokio::time::timeout(timeout_duration, node.prepare(&prepare_context))
        .await
        .map_err(|_| AsyncOperatorWorkerError::PrepareTimeout {
            timeout_ms: manifest.deadline.process_timeout_ms,
        })?
        .map_err(AsyncOperatorWorkerError::Prepare)?;
    observations.ready.store(true, Ordering::Release);
    observations.ready_notify.notify_waiters();
    run_prepared_worker(
        manifest,
        node,
        input,
        output_branches,
        cancellation,
        cancellation_notify,
        observations,
    )
    .await
}

async fn run_prepared_worker(
    manifest: AsyncOperatorManifest,
    mut node: Box<dyn AsyncNode>,
    mut input: AsyncOperatorWorkerSource,
    mut output_branches: Vec<AsyncOperatorOutputSender>,
    cancellation: Arc<AtomicBool>,
    cancellation_notify: Arc<Notify>,
    observations: Arc<AsyncOperatorObservationState>,
) -> Result<(), AsyncOperatorWorkerError> {
    let timeout_duration = Duration::from_millis(u64::from(manifest.deadline.process_timeout_ms));
    let processing_result = run_operator_loop(
        &manifest,
        node.as_mut(),
        &mut input,
        &mut output_branches,
        &cancellation,
        &cancellation_notify,
        &observations,
        timeout_duration,
    )
    .await;
    let close_result = match tokio::time::timeout(timeout_duration, node.close()).await {
        Ok(result) => result.map_err(AsyncOperatorWorkerError::Close),
        Err(_) => Err(AsyncOperatorWorkerError::CloseTimeout {
            timeout_ms: manifest.deadline.process_timeout_ms,
        }),
    };
    processing_result?;
    close_result
}

#[allow(clippy::too_many_arguments)]
async fn run_operator_loop(
    manifest: &AsyncOperatorManifest,
    node: &mut dyn AsyncNode,
    input: &mut AsyncOperatorWorkerSource,
    output_branches: &mut [AsyncOperatorOutputSender],
    cancellation: &AtomicBool,
    cancellation_notify: &Notify,
    observations: &AsyncOperatorObservationState,
    timeout_duration: Duration,
) -> Result<(), AsyncOperatorWorkerError> {
    enum ProcessAttempt {
        Completed(Result<Vec<AsyncEnvelope>, NodeError>),
        TimedOut,
        Cancelled,
    }

    let mut cancellation_cleanup_done = false;
    loop {
        let envelope = if cancellation.load(Ordering::Acquire) {
            if !cancellation_cleanup_done {
                observations
                    .cancellation_total
                    .fetch_add(1, Ordering::Relaxed);
                cancel_node(node, timeout_duration, manifest.deadline.process_timeout_ms).await?;
                cancellation_cleanup_done = true;
            }
            if manifest.cancellation == OperatorCancellationPolicy::DiscardQueued {
                break;
            }
            let Some(envelope) = input.recv()? else {
                break;
            };
            envelope
        } else {
            let Some(envelope) = input.recv()? else {
                if input.is_abandoned() {
                    let emitted = match tokio::time::timeout(timeout_duration, node.flush()).await {
                        Ok(Ok(emitted)) => emitted,
                        Ok(Err(error)) => {
                            observations
                                .process_failure_total
                                .fetch_add(1, Ordering::Relaxed);
                            return Err(AsyncOperatorWorkerError::Process(error));
                        }
                        Err(_) => {
                            observations.timeout_total.fetch_add(1, Ordering::Relaxed);
                            cancel_node(
                                node,
                                timeout_duration,
                                manifest.deadline.process_timeout_ms,
                            )
                            .await?;
                            return Err(AsyncOperatorWorkerError::Timeout {
                                timeout_ms: manifest.deadline.process_timeout_ms,
                            });
                        }
                    };
                    observations
                        .graceful_finish_total
                        .fetch_add(1, Ordering::Relaxed);
                    fan_out_outputs(manifest, emitted, output_branches, observations)?;
                    break;
                }
                observations.idle_poll_total.fetch_add(1, Ordering::Relaxed);
                tokio::select! {
                    _ = cancellation_notify.notified() => {}
                    _ = tokio::time::sleep(ASYNC_OPERATOR_IDLE_POLL_INTERVAL) => {}
                }
                continue;
            };
            envelope
        };
        let process_attempt = {
            let process = node.process(envelope);
            tokio::pin!(process);
            tokio::select! {
                result = &mut process => ProcessAttempt::Completed(result),
                _ = tokio::time::sleep(timeout_duration) => ProcessAttempt::TimedOut,
                _ = cancellation_notify.notified() => ProcessAttempt::Cancelled,
            }
        };
        let emitted = match process_attempt {
            ProcessAttempt::Completed(Ok(emitted)) => emitted,
            ProcessAttempt::Completed(Err(NodeError::ProcessTimeout { timeout_ms })) => {
                observations.timeout_total.fetch_add(1, Ordering::Relaxed);
                cancel_node(node, timeout_duration, manifest.deadline.process_timeout_ms).await?;
                if manifest.failure == OperatorFailurePolicy::StopWorker {
                    return Err(AsyncOperatorWorkerError::Timeout { timeout_ms });
                }
                continue;
            }
            ProcessAttempt::Completed(Err(error)) => {
                observations
                    .process_failure_total
                    .fetch_add(1, Ordering::Relaxed);
                if manifest.failure == OperatorFailurePolicy::StopWorker {
                    return Err(AsyncOperatorWorkerError::Process(error));
                }
                continue;
            }
            ProcessAttempt::TimedOut => {
                observations.timeout_total.fetch_add(1, Ordering::Relaxed);
                cancel_node(node, timeout_duration, manifest.deadline.process_timeout_ms).await?;
                if manifest.failure == OperatorFailurePolicy::StopWorker {
                    return Err(AsyncOperatorWorkerError::Timeout {
                        timeout_ms: manifest.deadline.process_timeout_ms,
                    });
                }
                continue;
            }
            ProcessAttempt::Cancelled => {
                observations
                    .cancellation_total
                    .fetch_add(1, Ordering::Relaxed);
                cancel_node(node, timeout_duration, manifest.deadline.process_timeout_ms).await?;
                break;
            }
        };
        observations.processed_total.fetch_add(1, Ordering::Relaxed);
        fan_out_outputs(manifest, emitted, output_branches, observations)?;
    }
    Ok(())
}

async fn cancel_node(
    node: &mut dyn AsyncNode,
    timeout_duration: Duration,
    timeout_ms: u32,
) -> Result<(), AsyncOperatorWorkerError> {
    match tokio::time::timeout(timeout_duration, node.cancel()).await {
        Ok(result) => result.map_err(AsyncOperatorWorkerError::Cancel),
        Err(_) => Err(AsyncOperatorWorkerError::CancelTimeout { timeout_ms }),
    }
}

fn fan_out_outputs(
    manifest: &AsyncOperatorManifest,
    emitted: Vec<AsyncEnvelope>,
    output_branches: &mut [AsyncOperatorOutputSender],
    observations: &AsyncOperatorObservationState,
) -> Result<(), AsyncOperatorWorkerError> {
    let output_contract = manifest
        .output_ports()
        .next()
        .ok_or(AsyncOperatorWorkerError::MissingOutputContract)?;
    for envelope in emitted {
        if matches!(&envelope.signal, AsyncSignal::Audio(_)) {
            return Err(AsyncOperatorWorkerError::UnsupportedAudioOutput);
        }
        if envelope.signal_spec.class != output_contract.signal.class
            || envelope.signal_spec.schema != output_contract.signal.schema
        {
            return Err(AsyncOperatorWorkerError::OutputSignalMismatch);
        }
        if !manifest
            .output_roles
            .accepts(&envelope.signal_spec, &output_contract.signal)
        {
            return Err(AsyncOperatorWorkerError::UndeclaredOutputRole);
        }
        let derived_lineage = envelope
            .derived_lineage
            .as_ref()
            .ok_or(AsyncOperatorWorkerError::MissingDerivedLineage)?;
        if derived_lineage.operator_id != manifest.operator_id
            || derived_lineage.operator_revision != manifest.revision
            || derived_lineage.operator_generation != manifest.generation
        {
            return Err(AsyncOperatorWorkerError::DerivedLineageMismatch);
        }
        observe_transcript_role(&envelope, observations);
        let is_terminal = manifest.output_roles.is_terminal(&envelope.signal_spec);
        if is_terminal {
            for (branch_index, (branch, branch_observations, loss_policy)) in
                output_branches.iter().enumerate()
            {
                if *loss_policy == LossPolicy::NeverDropFinalTranscript && branch.is_full() {
                    observations
                        .output_dropped_total
                        .fetch_add(1, Ordering::Relaxed);
                    branch_observations
                        .dropped_total
                        .fetch_add(1, Ordering::Relaxed);
                    return Err(AsyncOperatorWorkerError::FinalOutputDropped { branch_index });
                }
            }
        }
        let shared = Arc::new(envelope);
        for (branch_index, (branch, branch_observations, loss_policy)) in
            output_branches.iter_mut().enumerate()
        {
            match branch.push(Arc::clone(&shared)) {
                Ok(()) => {
                    observations
                        .output_emitted_total
                        .fetch_add(1, Ordering::Relaxed);
                    branch_observations
                        .delivered_total
                        .fetch_add(1, Ordering::Relaxed);
                }
                Err(rtrb::PushError::Full(_)) => {
                    observations
                        .output_dropped_total
                        .fetch_add(1, Ordering::Relaxed);
                    branch_observations
                        .dropped_total
                        .fetch_add(1, Ordering::Relaxed);
                    if is_terminal && *loss_policy == LossPolicy::NeverDropFinalTranscript {
                        return Err(AsyncOperatorWorkerError::FinalOutputDropped { branch_index });
                    }
                }
            }
        }
    }
    Ok(())
}

fn observe_transcript_role(envelope: &AsyncEnvelope, observations: &AsyncOperatorObservationState) {
    match envelope.signal_spec.role.as_ref().map(|role| role.as_str()) {
        Some(TRANSCRIPT_PARTIAL_ROLE) => {
            observations
                .transcript_partial_total
                .fetch_add(1, Ordering::Relaxed);
        }
        Some(TRANSCRIPT_FINAL_ROLE) => {
            observations
                .transcript_final_total
                .fetch_add(1, Ordering::Relaxed);
        }
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::AtomicUsize;

    use crate::frame::{
        AudioBufferPool, ClockDomainId, FrameLineage, LineagedAudioFrame, SampleFormat, SampleSpec,
        SessionId, SourceId, StemId, StreamId,
    };
    use crate::graph::compiler::Compiler;
    use crate::graph::planner::RuntimePlanner;
    use crate::graph::{
        register_builtins, transcript_final_spec, transcript_partial_spec, AudioCaps,
        ChannelLayout, ConfigError, DerivedSignalLineage, EventFormat, ExecutionPartition,
        MediaCaps, Multiplicity, NodeDescriptor, NodeRegistrationError, NodeRegistry, NodeTypeId,
        OperatorDeadlinePolicy, OperatorId, OperatorOutputRolePolicy, OperatorPermissionPolicy,
        Pipeline, PortDirection, PortSpec, SafetyContract, SemanticRole, SignalSpec, TextFormat,
    };

    use super::*;

    #[derive(Clone, Copy)]
    enum TestBehavior {
        Transcript,
        FlushFinal,
        PrepareFail,
        WrongClass,
        WrongRole,
        Audio,
        Slow,
        ReportedTimeout,
    }

    struct TestFactory {
        manifest: AsyncOperatorManifest,
        behavior: TestBehavior,
        closed: Arc<AtomicBool>,
        cancelled: Arc<AtomicBool>,
    }

    impl AsyncOperatorFactory for TestFactory {
        fn manifest(&self) -> &AsyncOperatorManifest {
            &self.manifest
        }

        fn validate_config(&self, _configuration: &NodeConfig) -> Result<(), ConfigError> {
            Ok(())
        }

        fn resolve_manifest(
            &self,
            configuration: &NodeConfig,
        ) -> Result<AsyncOperatorManifest, ConfigError> {
            let mut manifest = self.manifest.clone();
            if let Some(timeout_ms) = configuration.get_u32("process_timeout_ms") {
                manifest.deadline.process_timeout_ms = timeout_ms;
            }
            Ok(manifest)
        }

        fn create(&self, _configuration: &NodeConfig) -> Result<Box<dyn AsyncNode>, NodeError> {
            Ok(Box::new(TestNode {
                behavior: self.behavior,
                outputs_total: AtomicUsize::new(0),
                last_lineage: None,
                last_sequence_number: 0,
                last_timestamp_ns: 0,
                closed: Arc::clone(&self.closed),
                cancelled: Arc::clone(&self.cancelled),
                operator_id: self.manifest.operator_id.clone(),
                revision: self.manifest.revision,
                generation: self.manifest.generation,
            }))
        }
    }

    struct TestNode {
        behavior: TestBehavior,
        outputs_total: AtomicUsize,
        last_lineage: Option<FrameLineage>,
        last_sequence_number: u64,
        last_timestamp_ns: u64,
        closed: Arc<AtomicBool>,
        cancelled: Arc<AtomicBool>,
        operator_id: OperatorId,
        revision: u32,
        generation: u32,
    }

    impl AsyncNode for TestNode {
        fn prepare<'a>(
            &'a mut self,
            _context: &'a PrepareContext,
        ) -> crate::graph::AsyncNodeFuture<'a, Result<(), NodeError>> {
            Box::pin(async move {
                if matches!(self.behavior, TestBehavior::PrepareFail) {
                    Err(NodeError::Prepare("test prepare failure".to_owned()))
                } else {
                    Ok(())
                }
            })
        }

        fn process<'a>(
            &'a mut self,
            mut input: AsyncEnvelope,
        ) -> crate::graph::AsyncNodeFuture<'a, Result<Vec<AsyncEnvelope>, NodeError>> {
            Box::pin(async move {
                if matches!(self.behavior, TestBehavior::Slow) {
                    tokio::time::sleep(Duration::from_millis(50)).await;
                }
                if matches!(self.behavior, TestBehavior::ReportedTimeout) {
                    return Err(NodeError::ProcessTimeout { timeout_ms: 7 });
                }
                let output_index = self.outputs_total.fetch_add(1, Ordering::Relaxed);
                let base = input
                    .lineage
                    .ok_or_else(|| NodeError::Process("missing test lineage".to_owned()))?;
                self.last_lineage = Some(base);
                self.last_sequence_number = input.sequence_number;
                self.last_timestamp_ns = input.timestamp_ns;
                let (signal, specification) = match self.behavior {
                    TestBehavior::Transcript
                    | TestBehavior::Slow
                    | TestBehavior::ReportedTimeout => {
                        let specification = if output_index == 0 {
                            transcript_partial_spec()
                        } else {
                            transcript_final_spec()
                        };
                        (
                            AsyncSignal::Text(format!("transcript-{output_index}")),
                            specification,
                        )
                    }
                    TestBehavior::FlushFinal => (
                        AsyncSignal::Text(format!("partial-{output_index}")),
                        transcript_partial_spec(),
                    ),
                    TestBehavior::PrepareFail => (
                        AsyncSignal::Text("unreachable".to_owned()),
                        transcript_partial_spec(),
                    ),
                    TestBehavior::WrongClass => (
                        AsyncSignal::Event(Vec::new()),
                        SignalSpec::event(EventFormat::Json).with_role(TRANSCRIPT_PARTIAL_ROLE),
                    ),
                    TestBehavior::WrongRole => (
                        AsyncSignal::Text("summary".to_owned()),
                        SignalSpec::text(TextFormat::Utf8).with_role("summary.final"),
                    ),
                    TestBehavior::Audio => (
                        std::mem::replace(&mut input.signal, AsyncSignal::Control(Vec::new())),
                        SignalSpec::audio(),
                    ),
                };
                let mut output = input.map_signal(signal, specification);
                output.derived_lineage = Some(
                    DerivedSignalLineage::new(
                        base,
                        base.timestamp_end_ns(),
                        self.operator_id.clone(),
                        self.revision,
                        self.generation,
                        None,
                    )
                    .map_err(|error| NodeError::Process(error.to_string()))?,
                );
                Ok(vec![output])
            })
        }

        fn flush<'a>(
            &'a mut self,
        ) -> crate::graph::AsyncNodeFuture<'a, Result<Vec<AsyncEnvelope>, NodeError>> {
            Box::pin(async move {
                if !matches!(self.behavior, TestBehavior::FlushFinal) {
                    return Ok(Vec::new());
                }
                let Some(base) = self.last_lineage.take() else {
                    return Ok(Vec::new());
                };
                let mut output = AsyncEnvelope::new(
                    AsyncSignal::Text("final".to_owned()),
                    self.last_sequence_number,
                    self.last_timestamp_ns,
                );
                output.signal_spec = transcript_final_spec();
                output.source_id = Some(base.source_id);
                output.lineage = Some(base);
                output.derived_lineage = Some(
                    DerivedSignalLineage::new(
                        base,
                        base.timestamp_end_ns(),
                        self.operator_id.clone(),
                        self.revision,
                        self.generation,
                        None,
                    )
                    .map_err(|error| NodeError::Process(error.to_string()))?,
                );
                Ok(vec![output])
            })
        }

        fn cancel<'a>(&'a mut self) -> crate::graph::AsyncNodeFuture<'a, Result<(), NodeError>> {
            Box::pin(async move {
                self.cancelled.store(true, Ordering::Release);
                Ok(())
            })
        }

        fn close<'a>(&'a mut self) -> crate::graph::AsyncNodeFuture<'a, Result<(), NodeError>> {
            Box::pin(async move {
                self.closed.store(true, Ordering::Release);
                Ok(())
            })
        }
    }

    fn port(name: &str, direction: PortDirection, signal: SignalSpec) -> PortSpec {
        let media = if signal.class.is_audio() {
            MediaCaps::Audio(AudioCaps {
                sample_rate_hz: Some(16_000),
                frame_samples: Some(160),
                channel_layout: ChannelLayout::Mono,
                format: SampleFormat::F32Interleaved,
            })
        } else {
            MediaCaps::Text
        };
        PortSpec {
            name: name.to_owned(),
            direction,
            signal,
            media,
            multiplicity: Multiplicity::One,
            required: true,
        }
    }

    fn manifest(
        operator_id: &str,
        node_type_id: &str,
        timeout_ms: u32,
        failure: OperatorFailurePolicy,
    ) -> AsyncOperatorManifest {
        let mut input_edge = crate::graph::EdgeContract::voice_default();
        input_edge.media = MediaCaps::Audio(AudioCaps {
            sample_rate_hz: Some(16_000),
            frame_samples: Some(160),
            channel_layout: ChannelLayout::Mono,
            format: SampleFormat::F32Interleaved,
        });
        input_edge.copy_policy = crate::graph::CopyPolicy::CopyToBranchPool;
        let mut output_edge = crate::graph::EdgeContract::model_default();
        output_edge.media = MediaCaps::Text;
        AsyncOperatorManifest {
            operator_id: OperatorId::new(operator_id),
            revision: 1,
            generation: 1,
            node: NodeDescriptor {
                type_id: NodeTypeId::from(node_type_id),
                display_name: "Test STT",
                inputs: vec![port("audio", PortDirection::Input, SignalSpec::audio())],
                outputs: vec![port(
                    "transcript",
                    PortDirection::Output,
                    SignalSpec::text(TextFormat::Utf8).with_role("transcript"),
                )],
                execution: ExecutionPartition::AsyncWorker,
                safety: SafetyContract::AllocationAllowed,
                stateful: true,
            },
            input_edge,
            output_edge,
            queue_capacity_frames: 1,
            permission: OperatorPermissionPolicy {
                network_allowed: false,
                filesystem_allowed: false,
            },
            deadline: OperatorDeadlinePolicy {
                process_timeout_ms: timeout_ms,
            },
            cancellation: OperatorCancellationPolicy::DiscardQueued,
            failure,
            output_roles: OperatorOutputRolePolicy {
                allowed: vec![
                    SemanticRole::new(TRANSCRIPT_PARTIAL_ROLE),
                    SemanticRole::new(TRANSCRIPT_FINAL_ROLE),
                ],
                terminal: vec![SemanticRole::new(TRANSCRIPT_FINAL_ROLE)],
            },
        }
    }

    fn factory(
        operator_id: &str,
        node_type_id: &str,
        behavior: TestBehavior,
        timeout_ms: u32,
        failure: OperatorFailurePolicy,
        closed: Arc<AtomicBool>,
    ) -> Arc<TestFactory> {
        Arc::new(TestFactory {
            manifest: manifest(operator_id, node_type_id, timeout_ms, failure),
            behavior,
            closed,
            cancelled: Arc::new(AtomicBool::new(false)),
        })
    }

    fn envelope(sequence_number: u64) -> AsyncEnvelope {
        let pool = AudioBufferPool::new(1, 160);
        let buffer = pool.acquire().unwrap();
        let source_id = SourceId(5);
        let mut frame = crate::frame::AudioFrame::new(
            StreamId(3),
            source_id,
            sequence_number,
            sequence_number * 10_000_000,
            1,
            buffer,
        );
        frame.sample_rate_hz = 16_000;
        AsyncEnvelope::from_audio(
            frame,
            Some(FrameLineage {
                session_id: SessionId(7),
                source_id,
                stem_id: StemId(11),
                clock_id: ClockDomainId(13),
                sequence_num: sequence_number,
                timestamp_start_ns: sequence_number * 10_000_000,
                duration_ns: 10_000_000,
                source_generation: 1,
                discontinuity_epoch: 0,
                permission_epoch: 1,
            }),
        )
    }

    fn prepare_context() -> PrepareContext {
        PrepareContext::new(SampleSpec::new(16_000, 1, SampleFormat::F32Interleaved))
    }

    fn output_branch(capacity_signals: usize) -> AsyncOperatorOutputBranchSpec {
        AsyncOperatorOutputBranchSpec {
            capacity_signals,
            edge_contract: crate::graph::EdgeContract::model_default(),
        }
    }

    #[test]
    fn given_duplicate_async_node_type_when_registered_then_first_authority_is_preserved() {
        let closed = Arc::new(AtomicBool::new(false));
        let mut registry = NodeRegistry::new();
        registry
            .register_async(factory(
                "operator.first",
                "operator.stt",
                TestBehavior::Transcript,
                100,
                OperatorFailurePolicy::Continue,
                Arc::clone(&closed),
            ))
            .unwrap();

        let result = registry.register_async(factory(
            "operator.second",
            "operator.stt",
            TestBehavior::Transcript,
            100,
            OperatorFailurePolicy::Continue,
            closed,
        ));

        assert!(matches!(
            result,
            Err(NodeRegistrationError::DuplicateNodeType { .. })
        ));
        assert_eq!(
            registry
                .async_factory(&NodeTypeId::from("operator.stt"))
                .unwrap()
                .manifest()
                .operator_id
                .as_str(),
            "operator.first"
        );

        let operator_conflict = registry.register_async(factory(
            "operator.first",
            "operator.other-node",
            TestBehavior::Transcript,
            100,
            OperatorFailurePolicy::Continue,
            Arc::new(AtomicBool::new(false)),
        ));
        assert!(matches!(
            operator_conflict,
            Err(NodeRegistrationError::DuplicateOperatorId { .. })
        ));
        assert_eq!(
            registry
                .async_node_type_id(&OperatorId::new("operator.first"))
                .unwrap()
                .as_str(),
            "operator.stt"
        );
    }

    #[tokio::test(flavor = "current_thread")]
    async fn given_full_input_branch_when_sent_then_overflow_is_counted_and_join_is_bounded() {
        let closed = Arc::new(AtomicBool::new(false));
        let (mut worker, _outputs) = AsyncOperatorWorker::spawn(
            factory(
                "operator.overflow",
                "operator.overflow.node",
                TestBehavior::Transcript,
                100,
                OperatorFailurePolicy::Continue,
                Arc::clone(&closed),
            ),
            &NodeConfig::new(),
            prepare_context(),
            &[output_branch(1)],
        )
        .unwrap();
        worker.input_mut().unwrap().send(envelope(0)).unwrap();
        assert!(worker.input_mut().unwrap().send(envelope(1)).is_err());
        let observations = worker.observations();

        worker.cancel_and_join().await.unwrap();

        let snapshot = observations.snapshot();
        assert_eq!(snapshot.input_attempted_total, 2);
        assert_eq!(snapshot.input_dropped_total, 1);
        assert_eq!(snapshot.cancellation_total, 1);
        assert!(snapshot.joined);
        assert!(closed.load(Ordering::Acquire));
    }

    #[tokio::test(flavor = "current_thread")]
    async fn given_two_transcript_inputs_when_processed_then_partial_and_final_reach_each_branch() {
        let closed = Arc::new(AtomicBool::new(false));
        let (mut worker, mut outputs) = AsyncOperatorWorker::spawn(
            factory(
                "operator.transcript",
                "operator.transcript.node",
                TestBehavior::Transcript,
                100,
                OperatorFailurePolicy::Continue,
                closed,
            ),
            &NodeConfig::new(),
            prepare_context(),
            &[output_branch(2), output_branch(2)],
        )
        .unwrap();
        worker.input_mut().unwrap().send(envelope(0)).unwrap();
        tokio::task::yield_now().await;
        worker.input_mut().unwrap().send(envelope(1)).unwrap();
        tokio::time::sleep(Duration::from_millis(10)).await;

        for output in &mut outputs {
            let partial = output.recv().unwrap();
            let final_signal = output.recv().unwrap();
            assert_eq!(
                partial.signal_spec.role.as_ref().map(|role| role.as_str()),
                Some(TRANSCRIPT_PARTIAL_ROLE)
            );
            assert_eq!(
                final_signal
                    .signal_spec
                    .role
                    .as_ref()
                    .map(|role| role.as_str()),
                Some(TRANSCRIPT_FINAL_ROLE)
            );
            assert!(partial.derived_lineage.is_some());
            assert_eq!(
                output.observations(),
                AsyncOperatorOutputObservations {
                    delivered_total: 2,
                    dropped_total: 0
                }
            );
        }
        let observations = worker.observations();
        worker.cancel_and_join().await.unwrap();
        let snapshot = observations.snapshot();
        assert_eq!(snapshot.transcript_partial_total, 1);
        assert_eq!(snapshot.transcript_final_total, 1);
        assert_eq!(snapshot.output_emitted_total, 4);
        assert!(snapshot.joined);
    }

    #[tokio::test(flavor = "current_thread")]
    async fn given_graceful_finish_when_operator_has_pending_state_then_one_final_is_emitted() {
        let closed = Arc::new(AtomicBool::new(false));
        let (mut worker, mut outputs) = AsyncOperatorWorker::spawn(
            factory(
                "operator.finish",
                "operator.finish.node",
                TestBehavior::FlushFinal,
                100,
                OperatorFailurePolicy::Continue,
                Arc::clone(&closed),
            ),
            &NodeConfig::new(),
            prepare_context(),
            &[output_branch(2)],
        )
        .unwrap();
        worker.input_mut().unwrap().send(envelope(0)).unwrap();
        let observations = worker.observations();

        worker.finish_and_join().await.unwrap();

        let output = &mut outputs[0];
        assert_eq!(
            output
                .recv()
                .unwrap()
                .signal_spec
                .role
                .as_ref()
                .map(|role| role.as_str()),
            Some(TRANSCRIPT_PARTIAL_ROLE)
        );
        assert_eq!(
            output
                .recv()
                .unwrap()
                .signal_spec
                .role
                .as_ref()
                .map(|role| role.as_str()),
            Some(TRANSCRIPT_FINAL_ROLE)
        );
        assert!(output.recv().is_none());
        let snapshot = observations.snapshot();
        assert_eq!(snapshot.transcript_partial_total, 1);
        assert_eq!(snapshot.transcript_final_total, 1);
        assert_eq!(snapshot.graceful_finish_total, 1);
        assert_eq!(snapshot.cancellation_total, 0);
        assert!(snapshot.joined);
        assert!(closed.load(Ordering::Acquire));
    }

    #[tokio::test(flavor = "current_thread")]
    async fn given_prepare_failure_when_readiness_is_awaited_then_waiter_returns_false() {
        let (worker, _outputs) = AsyncOperatorWorker::spawn(
            factory(
                "operator.prepare-fail",
                "operator.prepare-fail.node",
                TestBehavior::PrepareFail,
                100,
                OperatorFailurePolicy::StopWorker,
                Arc::new(AtomicBool::new(false)),
            ),
            &NodeConfig::new(),
            prepare_context(),
            &[output_branch(1)],
        )
        .unwrap();
        let observations = worker.observations();

        let ready = tokio::time::timeout(Duration::from_millis(100), observations.wait_ready())
            .await
            .unwrap();

        assert!(!ready);
        assert!(matches!(
            worker.finish_and_join().await.unwrap_err(),
            AsyncOperatorWorkerError::Prepare(NodeError::Prepare(_))
        ));
        assert!(observations.snapshot().joined);
    }

    #[tokio::test(flavor = "current_thread")]
    async fn given_compiled_lineaged_edge_when_worker_runs_then_exact_session_stem_is_preserved() {
        let operator_factory = factory(
            "operator.compiled",
            "operator.compiled.node",
            TestBehavior::FlushFinal,
            100,
            OperatorFailurePolicy::Continue,
            Arc::new(AtomicBool::new(false)),
        );
        let mut registry = NodeRegistry::new();
        register_builtins(&mut registry).unwrap();
        registry.register_async(operator_factory.clone()).unwrap();
        let mut graph = Pipeline::new();
        let source = graph.add_node("passthrough", NodeConfig::new());
        let operator = graph.add_node("operator.compiled.node", NodeConfig::new());
        graph.connect_with(
            source.out("out"),
            operator.in_("audio"),
            operator_factory.manifest.input_edge,
        );
        let ir = Compiler::new()
            .compile(graph.into_spec(), &registry)
            .unwrap();
        let plan = RuntimePlanner::new().plan(&ir).unwrap();
        let (mut router, mut receivers) = crate::runtime::PlanEdgeRouter::new(&plan, &ir).unwrap();
        let receiver = receivers.pop().unwrap();
        let edge_id = receiver.edge_id();
        let input = envelope(0);
        let lineage = input.lineage.unwrap();
        let AsyncSignal::Audio(frame) = input.signal else {
            panic!("test envelope must contain audio");
        };
        let sent = router.dispatch_lineaged_from(
            source.id(),
            "out",
            LineagedAudioFrame::new(frame, lineage).unwrap(),
            1,
        );
        assert_eq!(sent.enqueued_edges, 1);
        drop(router);

        let (worker, mut outputs) = AsyncOperatorWorker::prepare_and_spawn_from_plan_edge(
            operator_factory,
            &NodeConfig::new(),
            prepare_context(),
            receiver,
            CompiledOperatorInputContract {
                edge_id,
                operator_node: operator.id(),
                session_id: SessionId(7),
                stem_id: StemId(11),
                source_id: Some(SourceId(5)),
            },
            &[output_branch(2)],
        )
        .await
        .unwrap();
        worker.finish_and_join().await.unwrap();

        let partial = outputs[0].recv().unwrap();
        let final_output = outputs[0].recv().unwrap();
        assert_eq!(partial.derived_lineage.as_ref().unwrap().base, lineage);
        assert_eq!(final_output.derived_lineage.as_ref().unwrap().base, lineage);
        assert_eq!(
            final_output
                .signal_spec
                .role
                .as_ref()
                .map(|role| role.as_str()),
            Some(TRANSCRIPT_FINAL_ROLE)
        );
    }

    #[tokio::test(flavor = "current_thread")]
    async fn given_cancellation_when_operator_has_pending_state_then_no_final_is_fabricated() {
        let closed = Arc::new(AtomicBool::new(false));
        let (mut worker, mut outputs) = AsyncOperatorWorker::spawn(
            factory(
                "operator.cancel",
                "operator.cancel.node",
                TestBehavior::FlushFinal,
                100,
                OperatorFailurePolicy::Continue,
                Arc::clone(&closed),
            ),
            &NodeConfig::new(),
            prepare_context(),
            &[output_branch(2)],
        )
        .unwrap();
        worker.input_mut().unwrap().send(envelope(0)).unwrap();
        tokio::time::sleep(Duration::from_millis(10)).await;
        let partial = outputs[0].recv().unwrap();
        assert_eq!(
            partial.signal_spec.role.as_ref().map(|role| role.as_str()),
            Some(TRANSCRIPT_PARTIAL_ROLE)
        );
        let observations = worker.observations();

        worker.cancel_and_join().await.unwrap();

        assert!(outputs[0].recv().is_none());
        let snapshot = observations.snapshot();
        assert_eq!(snapshot.transcript_final_total, 0);
        assert_eq!(snapshot.graceful_finish_total, 0);
        assert_eq!(snapshot.cancellation_total, 1);
        assert!(closed.load(Ordering::Acquire));
    }

    #[tokio::test(flavor = "current_thread")]
    async fn given_idle_worker_when_cancelled_then_cancel_hook_runs_before_close() {
        let cancelled = Arc::new(AtomicBool::new(false));
        let closed = Arc::new(AtomicBool::new(false));
        let operator_factory = Arc::new(TestFactory {
            manifest: manifest(
                "operator.idle-cancel",
                "operator.idle-cancel.node",
                100,
                OperatorFailurePolicy::Continue,
            ),
            behavior: TestBehavior::FlushFinal,
            closed: Arc::clone(&closed),
            cancelled: Arc::clone(&cancelled),
        });
        let (worker, _outputs) = AsyncOperatorWorker::spawn(
            operator_factory,
            &NodeConfig::new(),
            prepare_context(),
            &[output_branch(1)],
        )
        .unwrap();
        let observations = worker.observations();
        assert!(observations.wait_ready().await);

        worker.cancel_and_join().await.unwrap();

        assert!(cancelled.load(Ordering::Acquire));
        assert!(closed.load(Ordering::Acquire));
        assert_eq!(observations.snapshot().cancellation_total, 1);
    }

    #[tokio::test(flavor = "current_thread")]
    async fn given_full_terminal_branch_when_finished_then_final_loss_fails_closed() {
        let (mut worker, mut outputs) = AsyncOperatorWorker::spawn(
            factory(
                "operator.final-loss",
                "operator.final-loss.node",
                TestBehavior::FlushFinal,
                100,
                OperatorFailurePolicy::Continue,
                Arc::new(AtomicBool::new(false)),
            ),
            &NodeConfig::new(),
            prepare_context(),
            &[output_branch(2), output_branch(1)],
        )
        .unwrap();
        worker.input_mut().unwrap().send(envelope(0)).unwrap();

        let error = worker.finish_and_join().await.unwrap_err();

        assert!(matches!(
            error,
            AsyncOperatorWorkerError::FinalOutputDropped { branch_index: 1 }
        ));
        assert_eq!(outputs[0].observations().dropped_total, 0);
        assert_eq!(outputs[1].observations().dropped_total, 1);
        assert_eq!(
            outputs[0]
                .recv()
                .unwrap()
                .signal_spec
                .role
                .as_ref()
                .map(|role| role.as_str()),
            Some(TRANSCRIPT_PARTIAL_ROLE)
        );
        assert!(outputs[0].recv().is_none());
        assert_eq!(
            outputs[1]
                .recv()
                .unwrap()
                .signal_spec
                .role
                .as_ref()
                .map(|role| role.as_str()),
            Some(TRANSCRIPT_PARTIAL_ROLE)
        );
        assert!(outputs[1].recv().is_none());
    }

    #[tokio::test(flavor = "current_thread")]
    async fn given_wrong_output_class_when_processed_then_worker_rejects_it() {
        let (mut worker, _outputs) = AsyncOperatorWorker::spawn(
            factory(
                "operator.wrong-class",
                "operator.wrong-class.node",
                TestBehavior::WrongClass,
                100,
                OperatorFailurePolicy::Continue,
                Arc::new(AtomicBool::new(false)),
            ),
            &NodeConfig::new(),
            prepare_context(),
            &[output_branch(1)],
        )
        .unwrap();
        worker.input_mut().unwrap().send(envelope(0)).unwrap();

        assert!(matches!(
            worker.finish_and_join().await.unwrap_err(),
            AsyncOperatorWorkerError::OutputSignalMismatch
        ));
    }

    #[tokio::test(flavor = "current_thread")]
    async fn given_undeclared_output_role_when_processed_then_worker_rejects_it() {
        let (mut worker, _outputs) = AsyncOperatorWorker::spawn(
            factory(
                "operator.wrong-role",
                "operator.wrong-role.node",
                TestBehavior::WrongRole,
                100,
                OperatorFailurePolicy::Continue,
                Arc::new(AtomicBool::new(false)),
            ),
            &NodeConfig::new(),
            prepare_context(),
            &[output_branch(1)],
        )
        .unwrap();
        worker.input_mut().unwrap().send(envelope(0)).unwrap();

        assert!(matches!(
            worker.finish_and_join().await.unwrap_err(),
            AsyncOperatorWorkerError::UndeclaredOutputRole
        ));
    }

    #[tokio::test(flavor = "current_thread")]
    async fn given_audio_output_when_processed_then_derived_worker_rejects_it() {
        let (mut worker, _outputs) = AsyncOperatorWorker::spawn(
            factory(
                "operator.audio-output",
                "operator.audio-output.node",
                TestBehavior::Audio,
                100,
                OperatorFailurePolicy::Continue,
                Arc::new(AtomicBool::new(false)),
            ),
            &NodeConfig::new(),
            prepare_context(),
            &[output_branch(1)],
        )
        .unwrap();
        worker.input_mut().unwrap().send(envelope(0)).unwrap();

        assert!(matches!(
            worker.finish_and_join().await.unwrap_err(),
            AsyncOperatorWorkerError::UnsupportedAudioOutput
        ));
    }

    #[tokio::test(flavor = "current_thread")]
    async fn given_slow_operator_when_deadline_expires_then_timeout_cancel_and_join_are_observed() {
        let closed = Arc::new(AtomicBool::new(false));
        let (mut worker, _outputs) = AsyncOperatorWorker::spawn(
            factory(
                "operator.slow",
                "operator.slow.node",
                TestBehavior::Slow,
                5,
                OperatorFailurePolicy::Continue,
                Arc::clone(&closed),
            ),
            &NodeConfig::new(),
            prepare_context(),
            &[output_branch(1)],
        )
        .unwrap();
        worker.input_mut().unwrap().send(envelope(0)).unwrap();
        tokio::time::sleep(Duration::from_millis(15)).await;
        let observations = worker.observations();

        worker.cancel_and_join().await.unwrap();

        let snapshot = observations.snapshot();
        assert_eq!(snapshot.timeout_total, 1);
        assert_eq!(snapshot.cancellation_total, 1);
        assert_eq!(snapshot.transcript_final_total, 0);
        assert_eq!(snapshot.graceful_finish_total, 0);
        assert!(snapshot.joined);
        assert!(closed.load(Ordering::Acquire));
    }

    #[tokio::test(flavor = "current_thread")]
    async fn given_instance_deadline_when_worker_runs_then_configured_timeout_is_authoritative() {
        let (mut worker, _outputs) = AsyncOperatorWorker::spawn(
            factory(
                "operator.configured-deadline",
                "operator.configured-deadline.node",
                TestBehavior::Slow,
                500,
                OperatorFailurePolicy::Continue,
                Arc::new(AtomicBool::new(false)),
            ),
            &NodeConfig::new().with("process_timeout_ms", "5"),
            prepare_context(),
            &[output_branch(1)],
        )
        .unwrap();
        worker.input_mut().unwrap().send(envelope(0)).unwrap();
        tokio::time::sleep(Duration::from_millis(15)).await;
        let observations = worker.observations();

        worker.cancel_and_join().await.unwrap();

        let snapshot = observations.snapshot();
        assert_eq!(snapshot.timeout_total, 1);
        assert_eq!(snapshot.process_failure_total, 0);
    }

    #[tokio::test(flavor = "current_thread")]
    async fn given_node_reported_timeout_when_observed_then_it_is_not_a_process_failure() {
        let (mut worker, _outputs) = AsyncOperatorWorker::spawn(
            factory(
                "operator.reported-timeout",
                "operator.reported-timeout.node",
                TestBehavior::ReportedTimeout,
                100,
                OperatorFailurePolicy::Continue,
                Arc::new(AtomicBool::new(false)),
            ),
            &NodeConfig::new(),
            prepare_context(),
            &[output_branch(1)],
        )
        .unwrap();
        worker.input_mut().unwrap().send(envelope(0)).unwrap();
        let observations = worker.observations();

        worker.finish_and_join().await.unwrap();

        let snapshot = observations.snapshot();
        assert_eq!(snapshot.timeout_total, 1);
        assert_eq!(snapshot.process_failure_total, 0);
    }
}
