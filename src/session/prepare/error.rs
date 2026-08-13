//! Typed failures produced while materializing a compiled Session.

use crate::frame::{EndpointId, RouteId, StemId};
use crate::graph::{EdgeId, NodeId};
use crate::runtime::{ExecError, PlanRunnerError};
use crate::session::{SourceInstanceId, SourceTypeId};

#[derive(Debug, thiserror::Error)]
pub enum SessionPrepareError {
    #[error(transparent)]
    Runtime(#[from] ExecError),
    #[error(transparent)]
    SourceChannel(#[from] PlanRunnerError),
    #[error("compiled stem {stem_id:?} has no matching source node")]
    MissingSourceNode { stem_id: StemId },
    #[error("compiled stem {stem_id:?} maps to more than one source node")]
    DuplicateSourceNode { stem_id: StemId },
    #[error("compiled external source instance {source_instance_id:?} output '{output_port}' has no matching audio ingress node")]
    MissingExternalAudioIngress {
        source_instance_id: SourceInstanceId,
        output_port: String,
    },
    #[error("compiled external source type {source_type_id} has no registered definition")]
    MissingExternalSourceDefinition { source_type_id: SourceTypeId },
    #[error("external source instance {source_instance_id:?} output '{output_port}' has incompatible or non-concrete PCM media")]
    InvalidExternalAudioMedia {
        source_instance_id: SourceInstanceId,
        output_port: String,
    },
    #[error("generated audio stem {stem_id:?} has no matching ingress node")]
    MissingGeneratedAudioIngress { stem_id: StemId },
    #[error("generated audio stem {stem_id:?} has incompatible or non-concrete PCM media")]
    InvalidGeneratedAudioMedia { stem_id: StemId },
    #[error("generated audio stem {stem_id:?} has no matching operator bridge edge")]
    MissingGeneratedAudioBridge { stem_id: StemId },
    #[error("compiled external source route {route_id:?} has no matching edge")]
    MissingExternalSourceRouteEdge { route_id: RouteId },
    #[error("worker edge {edge_id:?} target is absent from the compiled graph")]
    MissingWorkerTarget { edge_id: EdgeId },
    #[error("worker edge {edge_id:?} is absent from the compiled graph")]
    MissingWorkerEdge { edge_id: EdgeId },
    #[error("worker edge {edge_id:?} has no negotiated edge contract")]
    MissingWorkerEdgeContract { edge_id: EdgeId },
    #[error("worker edge {edge_id:?} has no bounded runtime capacity")]
    MissingWorkerCapacity { edge_id: EdgeId },
    #[error("audio endpoint worker edge {edge_id:?} has no concrete audio sample specification")]
    MissingWorkerSampleSpec { edge_id: EdgeId },
    #[error("worker edge {edge_id:?} target port '{port_name}' is absent or disagrees with the compiled signal/media")]
    InvalidOperatorInputPort { edge_id: EdgeId, port_name: String },
    #[error("worker edge {edge_id:?} maps to unknown route {route_id:?}")]
    UnknownWorkerRoute { edge_id: EdgeId, route_id: RouteId },
    #[error("worker route {route_id:?} is mapped more than once")]
    DuplicateWorkerRoute { route_id: RouteId },
    #[error(
        "worker edge {edge_id:?} metadata does not match route {route_id:?}: expected stem {expected_stem_id:?} and endpoint {expected_endpoint_id:?}, got stem {actual_stem_id:?} and endpoint {actual_endpoint_id:?}"
    )]
    WorkerRouteMismatch {
        edge_id: EdgeId,
        route_id: RouteId,
        expected_stem_id: StemId,
        actual_stem_id: StemId,
        expected_endpoint_id: EndpointId,
        actual_endpoint_id: EndpointId,
    },
    #[error("derived edge {edge_id:?} has no compiled typed-edge plan")]
    MissingTypedEdgePlan { edge_id: EdgeId },
    #[error("compiled operator node {node_id:?} has no registered async factory")]
    MissingAsyncOperatorFactory { node_id: NodeId },
    #[error("compiled operator node {node_id:?} does not match Session operator declaration")]
    OperatorDeclarationMismatch { node_id: NodeId },
    #[error("compiled node {node_id:?} has no typed Session binding")]
    MissingNodeBinding { node_id: NodeId },
    #[error("compiled node {node_id:?} has an incompatible typed Session binding")]
    IncompatibleNodeBinding { node_id: NodeId },
    #[error("compiled operator node {node_id:?} received the same input edge more than once")]
    DuplicateOperatorInput { node_id: NodeId },
    #[error("operator signal edge {edge_id:?} has no compiled input")]
    MissingOperatorSignalInput { edge_id: EdgeId },
    #[error("signal endpoint route {route_id:?} is mapped more than once")]
    DuplicateSignalRoute { route_id: RouteId },
    #[error("signal edge {edge_id:?} does not match endpoint route {route_id:?} declaration")]
    SignalRouteMismatch { edge_id: EdgeId, route_id: RouteId },
    #[error(
        "compiled plan produced {actual} audio, {actual_operator_inputs} operator-input, and {actual_signal_endpoints} signal-endpoint receivers; expected {expected} audio, {expected_operator_inputs} operator-input, and {expected_signal_endpoints} signal-endpoint receivers"
    )]
    WorkerTopologyMismatch {
        expected: usize,
        actual: usize,
        expected_operator_inputs: usize,
        actual_operator_inputs: usize,
        expected_signal_endpoints: usize,
        actual_signal_endpoints: usize,
    },
}
