use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

use crate::frame::{SessionId, SourceId, StemId};
use crate::graph::{
    AsyncNode, AsyncOperatorEdgePrepareContext, AsyncOperatorFactory, AsyncOperatorManifest,
    AsyncOperatorPrepareContext, EdgeContract, MediaCaps, NodeConfig, NodeError,
    OperatorCancellationPolicy, OperatorFailurePolicy, PortDirection, SignalEnvelope,
    SignalPayload, SignalSpec,
};
use crate::graph::{EdgeId, NodeId};
use tokio::sync::Notify;
use tokio::task::JoinHandle;

use super::error::AsyncOperatorWorkerError;
#[cfg(any(test, feature = "internal-testing"))]
use super::io::{
    AsyncOperatorInput, AsyncOperatorInputAccessError, AsyncOperatorOutput,
    AsyncOperatorOutputBranchSpec,
};
use super::io::{
    AsyncOperatorNamedOutput, AsyncOperatorNamedOutputBranchSpec, AsyncOperatorTypedInput,
};
use super::observations::{AsyncOperatorObservationHandle, AsyncOperatorObservationState};
use crate::runtime::{PlanEdgeFrame, PlanEdgeReceiver};
#[cfg(any(test, feature = "internal-testing"))]
use crate::runtime::{SignalEdge, SignalEdgeReceiver};
use crate::runtime::{TypedEdgeFanout, TypedEdgePublishError, TypedEdgeReceiver};

const ASYNC_OPERATOR_IDLE_POLL_INTERVAL: Duration = Duration::from_millis(1);

enum AsyncOperatorWorkerSource {
    #[cfg(any(test, feature = "internal-testing"))]
    Direct(SignalEdgeReceiver<SignalEnvelope>),
    Compiled {
        receiver: PlanEdgeReceiver,
        lineage: Box<CompiledOperatorInputContract>,
    },
    Typed {
        port_name: String,
        receiver: TypedEdgeReceiver,
    },
}

pub(crate) enum SessionOperatorInput {
    Compiled {
        receiver: PlanEdgeReceiver,
        contract: CompiledOperatorInputContract,
    },
    Typed(AsyncOperatorTypedInput),
}

impl SessionOperatorInput {
    fn prepare_edge(&self) -> Result<AsyncOperatorEdgePrepareContext, NodeError> {
        match self {
            Self::Compiled { receiver, contract } => {
                if receiver.edge_id() != contract.edge_id
                    || receiver.to().node != contract.operator_node
                {
                    return Err(NodeError::Prepare(
                        "compiled async input receiver does not match its edge/node contract"
                            .to_owned(),
                    ));
                }
                prepare_edge(
                    Some(contract.edge_id),
                    &contract.input_port,
                    PortDirection::Input,
                    contract.signal_spec.clone(),
                    contract.media,
                    contract.edge_contract,
                    contract.capacity_signals,
                )
            }
            Self::Typed(input) => prepare_edge(
                input.edge_id,
                &input.port_name,
                PortDirection::Input,
                input.signal_spec.clone(),
                input.media,
                input.edge_contract,
                input.capacity_signals,
            ),
        }
    }

    fn into_source(self) -> AsyncOperatorWorkerSource {
        match self {
            Self::Compiled { receiver, contract } => AsyncOperatorWorkerSource::Compiled {
                receiver,
                lineage: Box::new(contract),
            },
            Self::Typed(input) => AsyncOperatorWorkerSource::Typed {
                port_name: input.port_name,
                receiver: input.receiver,
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompiledOperatorInputContract {
    pub edge_id: EdgeId,
    pub operator_node: NodeId,
    pub session_id: SessionId,
    pub stem_id: StemId,
    pub source_id: Option<SourceId>,
    pub input_port: String,
    pub signal_spec: SignalSpec,
    pub media: MediaCaps,
    pub edge_contract: EdgeContract,
    pub capacity_signals: usize,
}

impl AsyncOperatorWorkerSource {
    fn recv(&mut self) -> Result<Option<SignalEnvelope>, AsyncOperatorWorkerError> {
        match self {
            #[cfg(any(test, feature = "internal-testing"))]
            Self::Direct(receiver) => Ok(receiver.recv()),
            Self::Compiled {
                receiver,
                lineage: lineage_contract,
            } => receiver.try_recv().map_or(Ok(None), |frame| match frame {
                PlanEdgeFrame::Exclusive(frame) => {
                    let (frame, lineage, output_generation) =
                        frame.into_parts_with_output_generation();
                    if lineage.session_id != lineage_contract.session_id
                        || lineage.stem_id != lineage_contract.stem_id
                        || lineage_contract
                            .source_id
                            .is_some_and(|source_id| source_id != lineage.source_id)
                    {
                        return Err(AsyncOperatorWorkerError::PlanInputLineageMismatch);
                    }
                    Ok(Some(
                        SignalEnvelope::from_audio(frame, Some(lineage))
                            .with_output_generation(output_generation),
                    ))
                }
                PlanEdgeFrame::Shared(_) => {
                    Err(AsyncOperatorWorkerError::InvalidPlanInput { kind: "shared" })
                }
            }),
            Self::Typed { receiver, .. } => {
                receiver
                    .recv()
                    .map_or(Ok(None), |envelope| match Arc::try_unwrap(envelope) {
                        Ok(envelope) => Ok(Some(envelope)),
                        Err(shared) => {
                            let Some(payload) = clone_non_audio_payload(&shared.payload) else {
                                return Err(AsyncOperatorWorkerError::SharedAudioTypedInput);
                            };
                            Ok(Some(SignalEnvelope {
                                payload,
                                spec: shared.spec.clone(),
                                timing: shared.timing,
                                lineage: shared.lineage,
                                derivation: shared.derivation.clone(),
                                output_generation: shared.output_generation.clone(),
                            }))
                        }
                    })
            }
        }
    }

    fn is_abandoned(&self) -> bool {
        match self {
            #[cfg(any(test, feature = "internal-testing"))]
            Self::Direct(receiver) => receiver.is_abandoned(),
            Self::Compiled { receiver, .. } => receiver.is_abandoned(),
            Self::Typed { receiver, .. } => receiver.is_abandoned(),
        }
    }

    fn port_name<'a>(&'a self, manifest: &'a AsyncOperatorManifest) -> Option<&'a str> {
        match self {
            #[cfg(any(test, feature = "internal-testing"))]
            Self::Direct(_) => manifest.input_ports().next().map(|port| port.name.as_str()),
            Self::Compiled { .. } => manifest.input_ports().next().map(|port| port.name.as_str()),
            Self::Typed { port_name, .. } => Some(port_name),
        }
    }
}

struct AsyncOperatorWorkerInputs {
    sources: Vec<AsyncOperatorWorkerSource>,
    next_index: usize,
}

impl AsyncOperatorWorkerInputs {
    #[cfg(any(test, feature = "internal-testing"))]
    fn one(source: AsyncOperatorWorkerSource) -> Self {
        Self {
            sources: vec![source],
            next_index: 0,
        }
    }

    fn recv(
        &mut self,
        manifest: &AsyncOperatorManifest,
    ) -> Result<Option<(String, SignalEnvelope)>, AsyncOperatorWorkerError> {
        if self.sources.is_empty() {
            return Ok(None);
        }
        for offset in 0..self.sources.len() {
            let index = (self.next_index + offset) % self.sources.len();
            if let Some(envelope) = self.sources[index].recv()? {
                let port_name = self.sources[index].port_name(manifest).ok_or(
                    AsyncOperatorWorkerError::UnknownInputPort {
                        port_name: String::new(),
                    },
                )?;
                let port = manifest
                    .input_ports()
                    .find(|port| port.name == port_name)
                    .ok_or_else(|| AsyncOperatorWorkerError::UnknownInputPort {
                        port_name: port_name.to_owned(),
                    })?;
                if envelope.spec.class != port.signal.class
                    || envelope.spec.schema != port.signal.schema
                    || !port.media.supports_signal(&envelope.spec)
                {
                    return Err(AsyncOperatorWorkerError::OutputSignalMismatch);
                }
                self.next_index = (index + 1) % self.sources.len();
                return Ok(Some((port_name.to_owned(), envelope)));
            }
        }
        Ok(None)
    }

    fn is_abandoned(&self) -> bool {
        self.sources
            .iter()
            .all(AsyncOperatorWorkerSource::is_abandoned)
    }
}

struct NamedOutputFanout {
    port_name: String,
    fanout: TypedEdgeFanout,
}

fn clone_non_audio_payload(payload: &SignalPayload) -> Option<SignalPayload> {
    match payload {
        SignalPayload::Audio(_) => None,
        SignalPayload::Text(text) => Some(SignalPayload::Text(text.clone())),
        SignalPayload::Bytes(bytes) => Some(SignalPayload::Bytes(bytes.clone())),
    }
}

pub struct AsyncOperatorWorker {
    #[cfg(any(test, feature = "internal-testing"))]
    input: Option<AsyncOperatorInput>,
    cancellation: Arc<AtomicBool>,
    cancellation_notify: Arc<Notify>,
    observations: AsyncOperatorObservationHandle,
    join: JoinHandle<Result<(), AsyncOperatorWorkerError>>,
}

#[cfg(any(test, feature = "internal-testing"))]
fn build_output_branches(
    manifest: &AsyncOperatorManifest,
    output_branch_specs: &[AsyncOperatorOutputBranchSpec],
) -> Result<(Vec<NamedOutputFanout>, Vec<AsyncOperatorOutput>), NodeError> {
    let mut output_ports = manifest.output_ports();
    let output_port = output_ports
        .next()
        .ok_or_else(|| NodeError::Prepare("async operator has no output port".to_owned()))?;
    if output_ports.next().is_some() {
        return Err(NodeError::Prepare(
            "simple async operator spawn requires exactly one output port".to_owned(),
        ));
    }
    let (fanout, outputs) = TypedEdgeFanout::new(output_branch_specs)
        .map_err(|error| NodeError::Prepare(error.to_string()))?;
    Ok((
        vec![NamedOutputFanout {
            port_name: output_port.name.clone(),
            fanout,
        }],
        outputs,
    ))
}

fn build_named_output_branches(
    manifest: &AsyncOperatorManifest,
    output_branch_specs: &[AsyncOperatorNamedOutputBranchSpec<'_>],
) -> Result<(Vec<NamedOutputFanout>, Vec<AsyncOperatorNamedOutput>), NodeError> {
    if output_branch_specs.is_empty() {
        return Err(NodeError::Prepare(
            "async operator requires at least one output branch".to_owned(),
        ));
    }
    let mut fanouts = Vec::new();
    let mut outputs = Vec::with_capacity(output_branch_specs.len());
    for port in manifest.output_ports() {
        let matching = output_branch_specs
            .iter()
            .filter(|specification| specification.output_port == port.name)
            .collect::<Vec<_>>();
        if matching.is_empty() {
            continue;
        }
        let branches = matching
            .iter()
            .map(|specification| specification.branch)
            .collect::<Vec<_>>();
        let (fanout, receivers) = TypedEdgeFanout::new(&branches)
            .map_err(|error| NodeError::Prepare(error.to_string()))?;
        outputs.extend(
            receivers
                .into_iter()
                .map(|receiver| AsyncOperatorNamedOutput {
                    output_port: port.name.clone(),
                    receiver,
                }),
        );
        fanouts.push(NamedOutputFanout {
            port_name: port.name.clone(),
            fanout,
        });
    }
    if outputs.len() != output_branch_specs.len() {
        let unknown = output_branch_specs
            .iter()
            .find(|specification| {
                !manifest
                    .output_ports()
                    .any(|port| port.name == specification.output_port)
            })
            .map_or("", |specification| specification.output_port);
        return Err(NodeError::Prepare(format!(
            "async operator output port '{unknown}' is not declared"
        )));
    }
    Ok((fanouts, outputs))
}

fn prepare_edge(
    edge_id: Option<EdgeId>,
    port_name: &str,
    direction: PortDirection,
    signal: SignalSpec,
    media: MediaCaps,
    edge_contract: EdgeContract,
    capacity_signals: usize,
) -> Result<AsyncOperatorEdgePrepareContext, NodeError> {
    AsyncOperatorEdgePrepareContext::new(
        edge_id,
        port_name,
        direction,
        signal,
        media,
        edge_contract,
        capacity_signals,
    )
}

#[cfg(any(test, feature = "internal-testing"))]
fn simple_prepare_context(
    manifest: &AsyncOperatorManifest,
    input: Option<&CompiledOperatorInputContract>,
    output_branch_specs: &[AsyncOperatorOutputBranchSpec],
) -> Result<AsyncOperatorPrepareContext, NodeError> {
    let mut input_ports = manifest.input_ports();
    let input_port = input_ports
        .next()
        .ok_or_else(|| NodeError::Prepare("async operator has no input port".to_owned()))?;
    if input_ports.next().is_some() {
        return Err(NodeError::Prepare(
            "simple async operator preparation requires exactly one input port".to_owned(),
        ));
    }
    let input_edge = match input {
        Some(input) => prepare_edge(
            Some(input.edge_id),
            &input.input_port,
            PortDirection::Input,
            input.signal_spec.clone(),
            input.media,
            input.edge_contract,
            input.capacity_signals,
        )?,
        None => prepare_edge(
            None,
            &input_port.name,
            PortDirection::Input,
            input_port.signal.clone(),
            input_port.media,
            manifest.input_edge,
            manifest.queue_capacity_frames,
        )?,
    };
    let mut output_ports = manifest.output_ports();
    let output_port = output_ports
        .next()
        .ok_or_else(|| NodeError::Prepare("async operator has no output port".to_owned()))?;
    if output_ports.next().is_some() {
        return Err(NodeError::Prepare(
            "simple async operator preparation requires exactly one output port".to_owned(),
        ));
    }
    let mut edges = Vec::with_capacity(1 + output_branch_specs.len());
    edges.push(input_edge);
    for branch in output_branch_specs {
        edges.push(prepare_edge(
            None,
            &output_port.name,
            PortDirection::Output,
            output_port.signal.clone(),
            output_port.media,
            branch.edge_contract,
            branch.capacity_signals,
        )?);
    }
    let context = AsyncOperatorPrepareContext::new(manifest.node.execution, edges)?;
    validate_prepare_context(manifest, &context)?;
    Ok(context)
}

#[cfg(any(test, feature = "internal-testing"))]
fn composed_prepare_context(
    manifest: &AsyncOperatorManifest,
    typed_inputs: &[AsyncOperatorTypedInput],
    output_branch_specs: &[AsyncOperatorNamedOutputBranchSpec<'_>],
) -> Result<AsyncOperatorPrepareContext, NodeError> {
    let mut edges = Vec::with_capacity(typed_inputs.len() + output_branch_specs.len());
    for input in typed_inputs {
        edges.push(prepare_edge(
            input.edge_id,
            &input.port_name,
            PortDirection::Input,
            input.signal_spec.clone(),
            input.media,
            input.edge_contract,
            input.capacity_signals,
        )?);
    }
    for output in output_branch_specs {
        let port = manifest
            .output_ports()
            .find(|port| port.name == output.output_port)
            .ok_or_else(|| {
                NodeError::Prepare(format!(
                    "async operator output port '{}' is not declared",
                    output.output_port
                ))
            })?;
        edges.push(prepare_edge(
            None,
            output.output_port,
            PortDirection::Output,
            port.signal.clone(),
            port.media,
            output.branch.edge_contract,
            output.branch.capacity_signals,
        )?);
    }
    let context = AsyncOperatorPrepareContext::new(manifest.node.execution, edges)?;
    validate_prepare_context(manifest, &context)?;
    Ok(context)
}

fn session_composed_prepare_context(
    manifest: &AsyncOperatorManifest,
    inputs: &[SessionOperatorInput],
    output_branch_specs: &[AsyncOperatorNamedOutputBranchSpec<'_>],
) -> Result<AsyncOperatorPrepareContext, NodeError> {
    let mut edges = Vec::with_capacity(inputs.len() + output_branch_specs.len());
    for input in inputs {
        edges.push(input.prepare_edge()?);
    }
    for output in output_branch_specs {
        let port = manifest
            .output_ports()
            .find(|port| port.name == output.output_port)
            .ok_or_else(|| {
                NodeError::Prepare(format!(
                    "async operator output port '{}' is not declared",
                    output.output_port
                ))
            })?;
        edges.push(prepare_edge(
            None,
            output.output_port,
            PortDirection::Output,
            port.signal.clone(),
            port.media,
            output.branch.edge_contract,
            output.branch.capacity_signals,
        )?);
    }
    let context = AsyncOperatorPrepareContext::new(manifest.node.execution, edges)?;
    validate_prepare_context(manifest, &context)?;
    Ok(context)
}

fn validate_prepare_context(
    manifest: &AsyncOperatorManifest,
    context: &AsyncOperatorPrepareContext,
) -> Result<(), NodeError> {
    if context.execution_partition() != manifest.node.execution {
        return Err(NodeError::Prepare(
            "async operator prepare partition does not match its manifest".to_owned(),
        ));
    }
    for edge in context.inputs().iter().chain(context.outputs()) {
        let port = match edge.direction() {
            PortDirection::Input => manifest
                .input_ports()
                .find(|port| port.name == edge.port_name()),
            PortDirection::Output => manifest
                .output_ports()
                .find(|port| port.name == edge.port_name()),
        }
        .ok_or_else(|| {
            NodeError::Prepare(format!(
                "async operator prepare port '{}' is not declared",
                edge.port_name()
            ))
        })?;
        if edge.signal() != &port.signal
            || !edge.media().is_compatible_with(&port.media)
            || !edge.edge_contract().media.is_compatible_with(&edge.media())
        {
            return Err(NodeError::Prepare(format!(
                "async operator prepare port '{}' disagrees with its manifest",
                edge.port_name()
            )));
        }
    }
    Ok(())
}

impl AsyncOperatorWorker {
    #[cfg(any(test, feature = "internal-testing"))]
    pub fn spawn(
        factory: Arc<dyn AsyncOperatorFactory>,
        configuration: &NodeConfig,
        output_branch_specs: &[AsyncOperatorOutputBranchSpec],
    ) -> Result<(Self, Vec<AsyncOperatorOutput>), NodeError> {
        let manifest = factory.resolve_manifest(configuration)?;
        let prepare_context = simple_prepare_context(&manifest, None, output_branch_specs)?;
        let (input_sender, input_receiver) = SignalEdge::bounded(manifest.queue_capacity_frames);
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

    /// Starts a directly-fed worker with an already negotiated signal-shaped
    /// prepare context. Session-owned graph execution uses the compiled-edge
    /// path; this entry point exists for external harnesses that negotiate the
    /// boundary before constructing a full Session.
    #[cfg(any(test, feature = "internal-testing"))]
    pub fn spawn_with_context(
        factory: Arc<dyn AsyncOperatorFactory>,
        configuration: &NodeConfig,
        prepare_context: AsyncOperatorPrepareContext,
        output_branch_specs: &[AsyncOperatorOutputBranchSpec],
    ) -> Result<(Self, Vec<AsyncOperatorOutput>), NodeError> {
        let manifest = factory.resolve_manifest(configuration)?;
        validate_prepare_context(&manifest, &prepare_context)?;
        if prepare_context.inputs().len() != 1
            || prepare_context.inputs()[0].capacity_signals() != manifest.queue_capacity_frames
            || prepare_context.outputs().len() != output_branch_specs.len()
            || prepare_context
                .outputs()
                .iter()
                .zip(output_branch_specs)
                .any(|(context, branch)| {
                    context.capacity_signals() != branch.capacity_signals
                        || context.edge_contract() != branch.edge_contract
                })
        {
            return Err(NodeError::Prepare(
                "direct async operator prepare context disagrees with bounded runtime edges"
                    .to_owned(),
            ));
        }
        let (input_sender, input_receiver) = SignalEdge::bounded(manifest.queue_capacity_frames);
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

    #[cfg(any(test, feature = "internal-testing"))]
    pub fn spawn_composed(
        factory: Arc<dyn AsyncOperatorFactory>,
        configuration: &NodeConfig,
        typed_inputs: Vec<AsyncOperatorTypedInput>,
        output_branch_specs: &[AsyncOperatorNamedOutputBranchSpec<'_>],
    ) -> Result<(Self, Vec<AsyncOperatorNamedOutput>), NodeError> {
        if typed_inputs.is_empty() {
            return Err(NodeError::Prepare(
                "composed async operator requires at least one typed input".to_owned(),
            ));
        }
        let manifest = factory.resolve_manifest(configuration)?;
        manifest
            .validate()
            .map_err(|error| NodeError::Prepare(error.to_string()))?;
        for input in &typed_inputs {
            if !manifest
                .input_ports()
                .any(|port| port.name == input.port_name)
            {
                return Err(NodeError::Prepare(format!(
                    "async operator input port '{}' is not declared",
                    input.port_name
                )));
            }
        }
        let node = factory.create(configuration)?;
        let prepare_context =
            composed_prepare_context(&manifest, &typed_inputs, output_branch_specs)?;
        let observations = Arc::new(AsyncOperatorObservationState::default());
        let cancellation = Arc::new(AtomicBool::new(false));
        let cancellation_notify = Arc::new(Notify::new());
        let (output_senders, outputs) =
            build_named_output_branches(&manifest, output_branch_specs)?;
        let input_sources = typed_inputs
            .into_iter()
            .map(|input| AsyncOperatorWorkerSource::Typed {
                port_name: input.port_name,
                receiver: input.receiver,
            })
            .collect();
        let task_observations = Arc::clone(&observations);
        let task_cancellation = Arc::clone(&cancellation);
        let task_cancellation_notify = Arc::clone(&cancellation_notify);
        let join = tokio::spawn(async move {
            let result = run_worker(
                manifest,
                node,
                prepare_context,
                AsyncOperatorWorkerInputs {
                    sources: input_sources,
                    next_index: 0,
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
                #[cfg(any(test, feature = "internal-testing"))]
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

    pub(crate) async fn prepare_and_spawn_session_composed(
        factory: Arc<dyn AsyncOperatorFactory>,
        configuration: &NodeConfig,
        inputs: Vec<SessionOperatorInput>,
        output_branch_specs: &[AsyncOperatorNamedOutputBranchSpec<'_>],
    ) -> Result<(Self, Vec<AsyncOperatorNamedOutput>), AsyncOperatorWorkerError> {
        if inputs.is_empty() {
            return Err(AsyncOperatorWorkerError::Prepare(NodeError::Prepare(
                "Session-composed async operator requires at least one input".to_owned(),
            )));
        }
        let manifest = factory
            .resolve_manifest(configuration)
            .map_err(NodeError::Config)
            .map_err(AsyncOperatorWorkerError::Prepare)?;
        manifest.validate().map_err(|error| {
            AsyncOperatorWorkerError::Prepare(NodeError::Prepare(error.to_string()))
        })?;
        let prepare_context =
            session_composed_prepare_context(&manifest, &inputs, output_branch_specs)
                .map_err(AsyncOperatorWorkerError::Prepare)?;
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
        let (output_senders, outputs) = build_named_output_branches(&manifest, output_branch_specs)
            .map_err(AsyncOperatorWorkerError::Prepare)?;
        let input_sources = inputs
            .into_iter()
            .map(SessionOperatorInput::into_source)
            .collect();
        let task_observations = Arc::clone(&observations);
        let task_cancellation = Arc::clone(&cancellation);
        let task_cancellation_notify = Arc::clone(&cancellation_notify);
        let join = tokio::spawn(async move {
            let result = run_prepared_worker(
                manifest,
                node,
                AsyncOperatorWorkerInputs {
                    sources: input_sources,
                    next_index: 0,
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
                #[cfg(any(test, feature = "internal-testing"))]
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

    #[cfg(any(test, feature = "internal-testing"))]
    pub async fn prepare_and_spawn_from_plan_edge(
        factory: Arc<dyn AsyncOperatorFactory>,
        configuration: &NodeConfig,
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
        let prepare_context =
            simple_prepare_context(&manifest, Some(&input_contract), output_branch_specs)
                .map_err(AsyncOperatorWorkerError::Prepare)?;
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
        let (output_senders, outputs) = build_output_branches(&manifest, output_branch_specs)
            .map_err(AsyncOperatorWorkerError::Prepare)?;
        let task_observations = Arc::clone(&observations);
        let task_cancellation = Arc::clone(&cancellation);
        let task_cancellation_notify = Arc::clone(&cancellation_notify);
        let join = tokio::spawn(async move {
            let result = run_prepared_worker(
                manifest,
                node,
                AsyncOperatorWorkerInputs::one(AsyncOperatorWorkerSource::Compiled {
                    receiver: input_receiver,
                    lineage: Box::new(input_contract),
                }),
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

    #[cfg(any(test, feature = "internal-testing"))]
    fn spawn_with_source(
        factory: Arc<dyn AsyncOperatorFactory>,
        configuration: &NodeConfig,
        prepare_context: AsyncOperatorPrepareContext,
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
        validate_prepare_context(&manifest, &prepare_context)?;
        let node = factory.create(configuration)?;
        let observations = Arc::new(AsyncOperatorObservationState::default());
        if let Some(input) = direct_input.as_mut() {
            input.observations = Arc::clone(&observations);
        }
        let cancellation = Arc::new(AtomicBool::new(false));
        let cancellation_notify = Arc::new(Notify::new());
        let (output_senders, outputs) = build_output_branches(&manifest, output_branch_specs)?;
        let task_observations = Arc::clone(&observations);
        let task_cancellation = Arc::clone(&cancellation);
        let task_cancellation_notify = Arc::clone(&cancellation_notify);
        let join = tokio::spawn(async move {
            let result = run_worker(
                manifest,
                node,
                prepare_context,
                AsyncOperatorWorkerInputs::one(input_source),
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

    #[cfg(any(test, feature = "internal-testing"))]
    pub fn input_mut(&mut self) -> Result<&mut AsyncOperatorInput, AsyncOperatorInputAccessError> {
        self.input.as_mut().ok_or(AsyncOperatorInputAccessError)
    }

    pub fn observations(&self) -> AsyncOperatorObservationHandle {
        self.observations.clone()
    }

    pub async fn cancel_and_join(self) -> Result<(), AsyncOperatorWorkerError> {
        self.cancellation.store(true, Ordering::Release);
        self.cancellation_notify.notify_one();
        #[cfg(any(test, feature = "internal-testing"))]
        drop(self.input);
        self.join.await?
    }

    pub async fn finish_and_join(self) -> Result<(), AsyncOperatorWorkerError> {
        #[cfg(any(test, feature = "internal-testing"))]
        drop(self.input);
        self.join.await?
    }
}

#[cfg(any(test, feature = "internal-testing"))]
#[allow(clippy::too_many_arguments)]
async fn run_worker(
    manifest: AsyncOperatorManifest,
    mut node: Box<dyn AsyncNode>,
    prepare_context: AsyncOperatorPrepareContext,
    input: AsyncOperatorWorkerInputs,
    output_branches: Vec<NamedOutputFanout>,
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
    mut input: AsyncOperatorWorkerInputs,
    mut output_branches: Vec<NamedOutputFanout>,
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
    input: &mut AsyncOperatorWorkerInputs,
    output_branches: &mut [NamedOutputFanout],
    cancellation: &AtomicBool,
    cancellation_notify: &Notify,
    observations: &AsyncOperatorObservationState,
    timeout_duration: Duration,
) -> Result<(), AsyncOperatorWorkerError> {
    enum ProcessAttempt {
        Completed(Result<Vec<SignalEnvelope>, NodeError>),
        TimedOut,
        Cancelled,
    }

    let mut cancellation_cleanup_done = false;
    loop {
        let (input_port, envelope) = if cancellation.load(Ordering::Acquire) {
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
            let Some(envelope) = input.recv(manifest)? else {
                break;
            };
            envelope
        } else {
            let Some(envelope) = input.recv(manifest)? else {
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
            let process = node.process_port(&input_port, envelope);
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
    emitted: Vec<SignalEnvelope>,
    output_branches: &mut [NamedOutputFanout],
    observations: &AsyncOperatorObservationState,
) -> Result<(), AsyncOperatorWorkerError> {
    for envelope in emitted {
        let matching_ports = manifest
            .output_ports()
            .filter(|port| {
                envelope.spec.class == port.signal.class
                    && envelope.spec.schema == port.signal.schema
                    && port.media.supports_signal(&envelope.spec)
            })
            .collect::<Vec<_>>();
        let output_contract = match matching_ports.as_slice() {
            [] => return Err(AsyncOperatorWorkerError::OutputSignalMismatch),
            [output] => *output,
            _ => {
                let exact_roles = matching_ports
                    .iter()
                    .filter(|port| {
                        port.signal
                            .role
                            .as_ref()
                            .zip(envelope.spec.role.as_ref())
                            .is_some_and(|(declared, actual)| declared.as_str() == actual.as_str())
                    })
                    .collect::<Vec<_>>();
                match exact_roles.as_slice() {
                    [output] => **output,
                    _ => return Err(AsyncOperatorWorkerError::AmbiguousOutputPort),
                }
            }
        };
        if !manifest
            .output_roles
            .accepts(&envelope.spec, &output_contract.signal)
        {
            return Err(AsyncOperatorWorkerError::UndeclaredOutputRole);
        }
        let output_branches = output_branches
            .iter_mut()
            .find(|branches| branches.port_name == output_contract.name)
            .ok_or(AsyncOperatorWorkerError::MissingOutputContract)?;
        let derivation = envelope
            .derivation
            .as_ref()
            .ok_or(AsyncOperatorWorkerError::MissingDerivedLineage)?;
        if derivation.operator_id != manifest.operator_id
            || derivation.operator_revision != manifest.revision
            || derivation.operator_generation != manifest.generation
        {
            return Err(AsyncOperatorWorkerError::DerivedLineageMismatch);
        }
        envelope
            .validate()
            .map_err(|_| AsyncOperatorWorkerError::OutputSignalMismatch)?;
        let is_terminal = manifest.output_roles.is_terminal(&envelope.spec);
        if is_terminal {
            observations
                .output_terminal_total
                .fetch_add(1, Ordering::Relaxed);
        } else {
            observations
                .output_nonterminal_total
                .fetch_add(1, Ordering::Relaxed);
        }
        match output_branches.fanout.publish(envelope, is_terminal) {
            Ok(report) => {
                observations
                    .output_emitted_total
                    .fetch_add(report.delivered_total, Ordering::Relaxed);
                observations
                    .output_dropped_total
                    .fetch_add(report.dropped_total, Ordering::Relaxed);
            }
            Err(TypedEdgePublishError::RequiredBranchFull { branch_index }) => {
                observations
                    .output_dropped_total
                    .fetch_add(1, Ordering::Relaxed);
                return Err(AsyncOperatorWorkerError::TerminalOutputDropped { branch_index });
            }
            Err(TypedEdgePublishError::InvalidEnvelope(_)) => {
                return Err(AsyncOperatorWorkerError::OutputSignalMismatch);
            }
            Err(TypedEdgePublishError::PayloadTooLarge {
                branch_index,
                payload_bytes,
                max_payload_bytes,
            }) => {
                observations
                    .output_dropped_total
                    .fetch_add(1, Ordering::Relaxed);
                return Err(AsyncOperatorWorkerError::OutputPayloadTooLarge {
                    branch_index,
                    payload_bytes,
                    max_payload_bytes,
                });
            }
            Err(TypedEdgePublishError::NoBranches) => {
                return Err(AsyncOperatorWorkerError::MissingOutputContract);
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::AtomicUsize;
    use std::sync::Mutex;

    use crate::frame::{
        AudioBufferPool, ClockDomainId, FrameLineage, LineagedAudioFrame, SampleFormat, SessionId,
        SourceId, StemId, StreamId,
    };
    use crate::graph::compile::{Compiler, RuntimePlanner};
    use crate::graph::{
        register_builtins, AudioCaps, BinaryFormat, ChannelLayout, ConfigError, EventFormat,
        ExecutionPartition, MediaCaps, Multiplicity, NodeDescriptor, NodeRegistrationError,
        NodeRegistry, NodeTypeId, OperatorDeadlinePolicy, OperatorId, OperatorOutputRolePolicy,
        OperatorPermissionPolicy, Pipeline, PortDirection, PortSpec, SafetyContract, SemanticRole,
        SignalDerivation, SignalLineage, SignalSpec, SignalTiming, TextFormat,
    };

    use super::*;

    const TEST_NONTERMINAL_ROLE: &str = "test.output.nonterminal";
    const TEST_TERMINAL_ROLE: &str = "test.output.terminal";

    fn nonterminal_spec() -> SignalSpec {
        SignalSpec::text(TextFormat::Utf8).with_role(TEST_NONTERMINAL_ROLE)
    }

    fn terminal_spec() -> SignalSpec {
        SignalSpec::text(TextFormat::Utf8).with_role(TEST_TERMINAL_ROLE)
    }

    #[derive(Clone, Copy)]
    enum TestBehavior {
        NonterminalThenTerminal,
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
                last_timing: None,
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
        last_lineage: Option<SignalLineage>,
        last_timing: Option<SignalTiming>,
        closed: Arc<AtomicBool>,
        cancelled: Arc<AtomicBool>,
        operator_id: OperatorId,
        revision: u32,
        generation: u32,
    }

    impl AsyncNode for TestNode {
        fn prepare<'a>(
            &'a mut self,
            _context: &'a AsyncOperatorPrepareContext,
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
            mut input: SignalEnvelope,
        ) -> crate::graph::AsyncNodeFuture<'a, Result<Vec<SignalEnvelope>, NodeError>> {
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
                let timing = input.timing;
                self.last_lineage = Some(base);
                self.last_timing = Some(timing);
                let (signal, specification) = match self.behavior {
                    TestBehavior::NonterminalThenTerminal
                    | TestBehavior::Slow
                    | TestBehavior::ReportedTimeout => {
                        let specification = if output_index == 0 {
                            nonterminal_spec()
                        } else {
                            terminal_spec()
                        };
                        (
                            SignalPayload::Text(format!("output-{output_index}")),
                            specification,
                        )
                    }
                    TestBehavior::FlushFinal => (
                        SignalPayload::Text(format!("partial-{output_index}")),
                        nonterminal_spec(),
                    ),
                    TestBehavior::PrepareFail => (
                        SignalPayload::Text("unreachable".to_owned()),
                        nonterminal_spec(),
                    ),
                    TestBehavior::WrongClass => (
                        SignalPayload::Bytes(Vec::new()),
                        SignalSpec::event(EventFormat::Json).with_role(TEST_NONTERMINAL_ROLE),
                    ),
                    TestBehavior::WrongRole => (
                        SignalPayload::Text("summary".to_owned()),
                        SignalSpec::text(TextFormat::Utf8).with_role("summary.final"),
                    ),
                    TestBehavior::Audio => (
                        std::mem::replace(&mut input.payload, SignalPayload::Bytes(Vec::new())),
                        SignalSpec::audio(),
                    ),
                };
                let mut output = input.map_payload(signal, specification);
                output.derivation = Some(
                    SignalDerivation::new(
                        base,
                        timing,
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
        ) -> crate::graph::AsyncNodeFuture<'a, Result<Vec<SignalEnvelope>, NodeError>> {
            Box::pin(async move {
                if !matches!(self.behavior, TestBehavior::FlushFinal) {
                    return Ok(Vec::new());
                }
                let (Some(base), Some(timing)) =
                    (self.last_lineage.take(), self.last_timing.take())
                else {
                    return Ok(Vec::new());
                };
                let mut output = SignalEnvelope::untracked(
                    SignalPayload::Text("final".to_owned()),
                    terminal_spec(),
                    timing.observed_timestamp_ns,
                )
                .with_lineage(base, timing);
                output.derivation = Some(
                    SignalDerivation::new(
                        base,
                        timing,
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
        let mut input_edge = crate::graph::EdgeContract::realtime_audio();
        input_edge.media = MediaCaps::Audio(AudioCaps {
            sample_rate_hz: Some(16_000),
            frame_samples: Some(160),
            channel_layout: ChannelLayout::Mono,
            format: SampleFormat::F32Interleaved,
        });
        input_edge.copy_policy = crate::graph::CopyPolicy::CopyToBranchPool;
        let mut output_edge = crate::graph::EdgeContract::bounded_async();
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
                    SemanticRole::new(TEST_NONTERMINAL_ROLE),
                    SemanticRole::new(TEST_TERMINAL_ROLE),
                ],
                terminal: vec![SemanticRole::new(TEST_TERMINAL_ROLE)],
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

    fn envelope(sequence_number: u64) -> SignalEnvelope {
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
        SignalEnvelope::from_audio(
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

    fn output_branch(capacity_signals: usize) -> AsyncOperatorOutputBranchSpec {
        let mut edge_contract = crate::graph::EdgeContract::bounded_async();
        edge_contract.media = MediaCaps::Text;
        AsyncOperatorOutputBranchSpec {
            capacity_signals,
            edge_contract,
        }
    }

    fn text_typed_input(
        port_name: &str,
        receiver: TypedEdgeReceiver,
        capacity_signals: usize,
    ) -> AsyncOperatorTypedInput {
        let mut edge_contract = crate::graph::EdgeContract::bounded_async();
        edge_contract.media = MediaCaps::Text;
        AsyncOperatorTypedInput {
            port_name: port_name.to_owned(),
            receiver,
            edge_id: None,
            signal_spec: SignalSpec::text(TextFormat::Utf8),
            media: MediaCaps::Text,
            edge_contract,
            capacity_signals,
        }
    }

    fn text_input_factory(operator_id: &str, node_type_id: &str) -> Arc<TestFactory> {
        let factory = factory(
            operator_id,
            node_type_id,
            TestBehavior::NonterminalThenTerminal,
            100,
            OperatorFailurePolicy::Continue,
            Arc::new(AtomicBool::new(false)),
        );
        let mut manifest = factory.manifest.clone();
        manifest.node.inputs = vec![port(
            "input",
            PortDirection::Input,
            SignalSpec::text(TextFormat::Utf8),
        )];
        manifest.input_edge.media = MediaCaps::Text;
        Arc::new(TestFactory {
            manifest,
            behavior: factory.behavior,
            closed: Arc::clone(&factory.closed),
            cancelled: Arc::clone(&factory.cancelled),
        })
    }

    fn text_envelope(sequence_number: u64) -> SignalEnvelope {
        envelope(sequence_number).map_payload(
            SignalPayload::Text(format!("input-{sequence_number}")),
            SignalSpec::text(TextFormat::Utf8),
        )
    }

    struct PrepareCaptureFactory {
        manifest: AsyncOperatorManifest,
        observed: Arc<Mutex<Vec<AsyncOperatorPrepareContext>>>,
    }

    impl AsyncOperatorFactory for PrepareCaptureFactory {
        fn manifest(&self) -> &AsyncOperatorManifest {
            &self.manifest
        }

        fn validate_config(&self, _configuration: &NodeConfig) -> Result<(), ConfigError> {
            Ok(())
        }

        fn create(&self, _configuration: &NodeConfig) -> Result<Box<dyn AsyncNode>, NodeError> {
            Ok(Box::new(PrepareCaptureNode {
                observed: Arc::clone(&self.observed),
            }))
        }
    }

    struct PrepareCaptureNode {
        observed: Arc<Mutex<Vec<AsyncOperatorPrepareContext>>>,
    }

    impl AsyncNode for PrepareCaptureNode {
        fn prepare<'a>(
            &'a mut self,
            context: &'a AsyncOperatorPrepareContext,
        ) -> crate::graph::AsyncNodeFuture<'a, Result<(), NodeError>> {
            Box::pin(async move {
                self.observed.lock().unwrap().push(context.clone());
                Ok(())
            })
        }

        fn process<'a>(
            &'a mut self,
            _input: SignalEnvelope,
        ) -> crate::graph::AsyncNodeFuture<'a, Result<Vec<SignalEnvelope>, NodeError>> {
            Box::pin(async { Ok(Vec::new()) })
        }
    }

    fn prepare_capture_manifest(
        index: usize,
        signal: SignalSpec,
        media: MediaCaps,
    ) -> AsyncOperatorManifest {
        let mut input_edge = EdgeContract::bounded_async();
        input_edge.media = media;
        input_edge.backpressure = crate::graph::BackpressurePolicy::DropNewest;
        input_edge.copy_policy = crate::graph::CopyPolicy::CopyToBranchPool;
        let mut output_edge = EdgeContract::bounded_async();
        output_edge.media = media;
        AsyncOperatorManifest {
            operator_id: OperatorId::new(format!("operator.prepare.{index}")),
            revision: 1,
            generation: 1,
            node: NodeDescriptor {
                type_id: NodeTypeId::from("operator.prepare.capture"),
                display_name: "Prepare capture",
                inputs: vec![PortSpec {
                    name: "input".to_owned(),
                    direction: PortDirection::Input,
                    signal: signal.clone(),
                    media,
                    multiplicity: Multiplicity::One,
                    required: true,
                }],
                outputs: vec![PortSpec {
                    name: "output".to_owned(),
                    direction: PortDirection::Output,
                    signal,
                    media,
                    multiplicity: Multiplicity::One,
                    required: true,
                }],
                execution: ExecutionPartition::AsyncWorker,
                safety: SafetyContract::AllocationAllowed,
                stateful: false,
            },
            input_edge,
            output_edge,
            queue_capacity_frames: 3,
            permission: OperatorPermissionPolicy {
                network_allowed: false,
                filesystem_allowed: false,
            },
            deadline: OperatorDeadlinePolicy {
                process_timeout_ms: 100,
            },
            cancellation: OperatorCancellationPolicy::DiscardQueued,
            failure: OperatorFailurePolicy::StopWorker,
            output_roles: OperatorOutputRolePolicy::default(),
        }
    }

    #[tokio::test(flavor = "current_thread")]
    async fn given_every_nonaudio_signal_class_when_worker_prepares_then_exact_signal_context_is_received(
    ) {
        let cases = vec![
            (SignalSpec::text(TextFormat::Utf8), MediaCaps::Text),
            (SignalSpec::event(EventFormat::Json), MediaCaps::Event),
            (SignalSpec::metrics(), MediaCaps::Metrics),
            (SignalSpec::control(), MediaCaps::Control),
            (
                SignalSpec::binary(BinaryFormat::Raw),
                MediaCaps::Binary(BinaryFormat::Raw),
            ),
            (
                SignalSpec::custom("org.example.prepare-signal.v1")
                    .with_schema("json-schema:org.example.prepare-signal.v1"),
                MediaCaps::Binary(BinaryFormat::Raw),
            ),
        ];

        for (index, (signal, media)) in cases.into_iter().enumerate() {
            let observed = Arc::new(Mutex::new(Vec::new()));
            let manifest = prepare_capture_manifest(index, signal.clone(), media);
            let output_edge = manifest.output_edge;
            let (worker, _outputs) = AsyncOperatorWorker::spawn(
                Arc::new(PrepareCaptureFactory {
                    manifest,
                    observed: Arc::clone(&observed),
                }),
                &NodeConfig::new(),
                &[AsyncOperatorOutputBranchSpec {
                    capacity_signals: 5,
                    edge_contract: output_edge,
                }],
            )
            .unwrap();
            assert!(worker.observations().wait_ready().await);
            worker.cancel_and_join().await.unwrap();

            let contexts = observed.lock().unwrap();
            assert_eq!(contexts.len(), 1);
            let context = &contexts[0];
            assert_eq!(
                context.execution_partition(),
                ExecutionPartition::AsyncWorker
            );
            assert_eq!(context.inputs()[0].port_name(), "input");
            assert_eq!(context.inputs()[0].signal(), &signal);
            assert_eq!(context.inputs()[0].media(), media);
            assert_eq!(context.inputs()[0].capacity_signals(), 3);
            assert_eq!(context.outputs()[0].port_name(), "output");
            assert_eq!(context.outputs()[0].signal(), &signal);
            assert_eq!(context.outputs()[0].media(), media);
            assert_eq!(context.outputs()[0].capacity_signals(), 5);
        }
    }

    #[test]
    fn given_prepare_context_capacity_disagrees_with_runtime_edge_when_spawned_then_prepare_fails_closed(
    ) {
        let signal = SignalSpec::text(TextFormat::Utf8);
        let media = MediaCaps::Text;
        let observed = Arc::new(Mutex::new(Vec::new()));
        let manifest = prepare_capture_manifest(99, signal.clone(), media);
        let input_edge = manifest.input_edge;
        let output_edge = manifest.output_edge;
        let context = AsyncOperatorPrepareContext::new(
            ExecutionPartition::AsyncWorker,
            vec![
                AsyncOperatorEdgePrepareContext::new(
                    None,
                    "input",
                    PortDirection::Input,
                    signal.clone(),
                    media,
                    input_edge,
                    3,
                )
                .unwrap(),
                AsyncOperatorEdgePrepareContext::new(
                    None,
                    "output",
                    PortDirection::Output,
                    signal,
                    media,
                    output_edge,
                    4,
                )
                .unwrap(),
            ],
        )
        .unwrap();

        let result = AsyncOperatorWorker::spawn_with_context(
            Arc::new(PrepareCaptureFactory { manifest, observed }),
            &NodeConfig::new(),
            context,
            &[AsyncOperatorOutputBranchSpec {
                capacity_signals: 5,
                edge_contract: output_edge,
            }],
        );

        assert!(
            matches!(result, Err(NodeError::Prepare(message)) if message.contains("disagrees"))
        );
    }

    #[tokio::test(flavor = "current_thread")]
    async fn given_operator_composition_with_three_external_operators_then_derived_output_crosses_each_bounded_edge(
    ) {
        let (mut first, mut first_outputs) = AsyncOperatorWorker::spawn(
            factory(
                "operator.chain.first",
                "operator.chain.first.node",
                TestBehavior::NonterminalThenTerminal,
                100,
                OperatorFailurePolicy::Continue,
                Arc::new(AtomicBool::new(false)),
            ),
            &NodeConfig::new(),
            &[output_branch(4)],
        )
        .unwrap();
        let (second, mut second_outputs) = AsyncOperatorWorker::spawn_composed(
            text_input_factory("operator.chain.second", "operator.chain.second.node"),
            &NodeConfig::new(),
            vec![text_typed_input("input", first_outputs.remove(0), 4)],
            &[AsyncOperatorNamedOutputBranchSpec {
                output_port: "transcript",
                branch: output_branch(4),
            }],
        )
        .unwrap();
        let (third, mut third_outputs) = AsyncOperatorWorker::spawn_composed(
            text_input_factory("operator.chain.third", "operator.chain.third.node"),
            &NodeConfig::new(),
            vec![text_typed_input(
                "input",
                second_outputs.remove(0).receiver,
                4,
            )],
            &[AsyncOperatorNamedOutputBranchSpec {
                output_port: "transcript",
                branch: output_branch(4),
            }],
        )
        .unwrap();
        first.input_mut().unwrap().send(envelope(1)).unwrap();
        first.finish_and_join().await.unwrap();
        second.finish_and_join().await.unwrap();
        third.finish_and_join().await.unwrap();

        let output = third_outputs
            .remove(0)
            .receiver
            .recv()
            .expect("third output");
        assert!(matches!(output.payload, SignalPayload::Text(_)));
        assert_eq!(
            output.derivation.as_ref().unwrap().operator_id.as_str(),
            "operator.chain.third"
        );
    }

    #[tokio::test(flavor = "current_thread")]
    async fn given_operator_composition_with_named_multi_input_output_manifest_then_each_declared_port_executes(
    ) {
        let mut manifest = manifest(
            "operator.named",
            "operator.named.node",
            100,
            OperatorFailurePolicy::Continue,
        );
        manifest.node.inputs = vec![
            port(
                "left",
                PortDirection::Input,
                SignalSpec::text(TextFormat::Utf8),
            ),
            port(
                "right",
                PortDirection::Input,
                SignalSpec::text(TextFormat::Utf8),
            ),
        ];
        manifest.node.outputs = vec![
            port("partial", PortDirection::Output, nonterminal_spec()),
            port("final", PortDirection::Output, terminal_spec()),
        ];
        manifest.input_edge.media = MediaCaps::Text;
        manifest.output_edge.media = MediaCaps::Text;
        let factory = Arc::new(TestFactory {
            manifest,
            behavior: TestBehavior::NonterminalThenTerminal,
            closed: Arc::new(AtomicBool::new(false)),
            cancelled: Arc::new(AtomicBool::new(false)),
        });
        let branch = output_branch(2);
        let (mut left_sender, mut left_receivers) = TypedEdgeFanout::new(&[branch]).unwrap();
        let (mut right_sender, mut right_receivers) = TypedEdgeFanout::new(&[branch]).unwrap();
        let (worker, mut outputs) = AsyncOperatorWorker::spawn_composed(
            factory,
            &NodeConfig::new(),
            vec![
                text_typed_input("left", left_receivers.remove(0), 2),
                text_typed_input("right", right_receivers.remove(0), 2),
            ],
            &[
                AsyncOperatorNamedOutputBranchSpec {
                    output_port: "partial",
                    branch: output_branch(2),
                },
                AsyncOperatorNamedOutputBranchSpec {
                    output_port: "final",
                    branch: output_branch(2),
                },
            ],
        )
        .unwrap();
        left_sender.publish(text_envelope(1), false).unwrap();
        right_sender.publish(text_envelope(2), false).unwrap();
        drop(left_sender);
        drop(right_sender);
        worker.finish_and_join().await.unwrap();

        outputs.sort_by(|left, right| left.output_port.cmp(&right.output_port));
        let final_output = outputs[0].receiver.recv().expect("final output");
        let partial_output = outputs[1].receiver.recv().expect("partial output");
        assert_eq!(
            final_output.spec.role.as_ref().unwrap().as_str(),
            TEST_TERMINAL_ROLE
        );
        assert_eq!(
            partial_output.spec.role.as_ref().unwrap().as_str(),
            TEST_NONTERMINAL_ROLE
        );
    }

    #[test]
    fn given_duplicate_async_node_type_when_registered_then_first_authority_is_preserved() {
        let closed = Arc::new(AtomicBool::new(false));
        let mut registry = NodeRegistry::new();
        registry
            .register_async(factory(
                "operator.first",
                "operator.stt",
                TestBehavior::NonterminalThenTerminal,
                100,
                OperatorFailurePolicy::Continue,
                Arc::clone(&closed),
            ))
            .unwrap();

        let result = registry.register_async(factory(
            "operator.second",
            "operator.stt",
            TestBehavior::NonterminalThenTerminal,
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
            TestBehavior::NonterminalThenTerminal,
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
                TestBehavior::NonterminalThenTerminal,
                100,
                OperatorFailurePolicy::Continue,
                Arc::clone(&closed),
            ),
            &NodeConfig::new(),
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
    async fn given_two_inputs_when_processed_then_nonterminal_and_terminal_reach_each_branch() {
        let closed = Arc::new(AtomicBool::new(false));
        let (mut worker, mut outputs) = AsyncOperatorWorker::spawn(
            factory(
                "operator.transcript",
                "operator.transcript.node",
                TestBehavior::NonterminalThenTerminal,
                100,
                OperatorFailurePolicy::Continue,
                closed,
            ),
            &NodeConfig::new(),
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
                partial.spec.role.as_ref().map(|role| role.as_str()),
                Some(TEST_NONTERMINAL_ROLE)
            );
            assert_eq!(
                final_signal.spec.role.as_ref().map(|role| role.as_str()),
                Some(TEST_TERMINAL_ROLE)
            );
            assert!(partial.derivation.is_some());
            let output_observations = output.observations();
            assert_eq!(output_observations.capacity_signals, 2);
            assert_eq!(output_observations.enqueued_total, 2);
            assert_eq!(output_observations.received_total, 2);
            assert_eq!(output_observations.dropped_total, 0);
        }
        let observations = worker.observations();
        worker.cancel_and_join().await.unwrap();
        let snapshot = observations.snapshot();
        assert_eq!(snapshot.output_nonterminal_total, 1);
        assert_eq!(snapshot.output_terminal_total, 1);
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
                .spec
                .role
                .as_ref()
                .map(|role| role.as_str()),
            Some(TEST_NONTERMINAL_ROLE)
        );
        assert_eq!(
            output
                .recv()
                .unwrap()
                .spec
                .role
                .as_ref()
                .map(|role| role.as_str()),
            Some(TEST_TERMINAL_ROLE)
        );
        assert!(output.recv().is_none());
        let snapshot = observations.snapshot();
        assert_eq!(snapshot.output_nonterminal_total, 1);
        assert_eq!(snapshot.output_terminal_total, 1);
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
        let timing = input.timing;
        let frame_lineage = FrameLineage {
            session_id: lineage.session_id,
            source_id: lineage.source_id,
            stem_id: StemId(11),
            clock_id: lineage.clock_id,
            sequence_num: lineage.sequence_number,
            timestamp_start_ns: timing.source_timestamp_ns.unwrap(),
            duration_ns: timing.duration_ns.unwrap(),
            source_generation: lineage.source_generation,
            discontinuity_epoch: lineage.discontinuity_epoch,
            permission_epoch: lineage.policy_epoch,
        };
        let SignalPayload::Audio(frame) = input.payload else {
            panic!("test envelope must contain audio");
        };
        let sent = router.dispatch_from(
            source.id(),
            "out",
            LineagedAudioFrame::new(frame, frame_lineage).unwrap(),
            1,
        );
        assert_eq!(sent.enqueued_edges, 1);
        drop(router);
        let input_port = operator_factory.manifest.node.inputs[0].clone();
        let input_edge_contract = operator_factory.manifest.input_edge;
        let input_capacity_signals = plan
            .memory_plan
            .edge_buffer(edge_id)
            .unwrap()
            .capacity_frames;

        let (worker, mut outputs) = AsyncOperatorWorker::prepare_and_spawn_from_plan_edge(
            operator_factory,
            &NodeConfig::new(),
            receiver,
            CompiledOperatorInputContract {
                edge_id,
                operator_node: operator.id(),
                session_id: SessionId(7),
                stem_id: StemId(11),
                source_id: Some(SourceId(5)),
                input_port: input_port.name,
                signal_spec: input_port.signal,
                media: input_port.media,
                edge_contract: input_edge_contract,
                capacity_signals: input_capacity_signals,
            },
            &[output_branch(2)],
        )
        .await
        .unwrap();
        worker.finish_and_join().await.unwrap();

        let partial = outputs[0].recv().unwrap();
        let final_output = outputs[0].recv().unwrap();
        assert_eq!(
            partial.derivation.as_ref().unwrap().upstream_lineage,
            lineage
        );
        assert_eq!(
            final_output.derivation.as_ref().unwrap().upstream_lineage,
            lineage
        );
        assert_eq!(
            final_output.spec.role.as_ref().map(|role| role.as_str()),
            Some(TEST_TERMINAL_ROLE)
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
            &[output_branch(2)],
        )
        .unwrap();
        worker.input_mut().unwrap().send(envelope(0)).unwrap();
        tokio::time::sleep(Duration::from_millis(10)).await;
        let partial = outputs[0].recv().unwrap();
        assert_eq!(
            partial.spec.role.as_ref().map(|role| role.as_str()),
            Some(TEST_NONTERMINAL_ROLE)
        );
        let observations = worker.observations();

        worker.cancel_and_join().await.unwrap();

        assert!(outputs[0].recv().is_none());
        let snapshot = observations.snapshot();
        assert_eq!(snapshot.output_terminal_total, 0);
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
        let (worker, _outputs) =
            AsyncOperatorWorker::spawn(operator_factory, &NodeConfig::new(), &[output_branch(1)])
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
            &[output_branch(2), output_branch(1)],
        )
        .unwrap();
        worker.input_mut().unwrap().send(envelope(0)).unwrap();

        let error = worker.finish_and_join().await.unwrap_err();

        assert!(matches!(
            error,
            AsyncOperatorWorkerError::TerminalOutputDropped { branch_index: 1 }
        ));
        assert_eq!(outputs[0].observations().dropped_total, 0);
        assert_eq!(outputs[1].observations().dropped_total, 1);
        assert_eq!(
            outputs[0]
                .recv()
                .unwrap()
                .spec
                .role
                .as_ref()
                .map(|role| role.as_str()),
            Some(TEST_NONTERMINAL_ROLE)
        );
        assert!(outputs[0].recv().is_none());
        assert_eq!(
            outputs[1]
                .recv()
                .unwrap()
                .spec
                .role
                .as_ref()
                .map(|role| role.as_str()),
            Some(TEST_NONTERMINAL_ROLE)
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
    async fn given_audio_output_without_audio_port_when_processed_then_worker_rejects_it() {
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
        assert_eq!(snapshot.output_terminal_total, 0);
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
