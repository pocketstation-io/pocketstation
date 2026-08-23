//! Typed failures produced while materializing a compiled Session.

use crate::frame::{EndpointId, RouteId, StemId};
use crate::graph::{EdgeId, NodeId};
use crate::runtime::{ExecError, PlanRunnerError};
use crate::session::{SourceInstanceId, SourceTypeId};

#[derive(Debug, thiserror::Error)]
#[doc = "Classifies failures reported as session prepare error."]
pub enum SessionPrepareError {
    #[error(transparent)]
    #[doc = "Reports runtime."]
    Runtime(#[from] ExecError),
    #[error(transparent)]
    #[doc = "Reports source channel."]
    SourceChannel(#[from] PlanRunnerError),
    #[error("compiled stem {stem_id:?} has no matching source node")]
    #[doc = "Reports missing source node."]
    MissingSourceNode {
        #[doc = "Identifies the stem associated with `MissingSourceNode`."]
        stem_id: StemId,
    },
    #[error("compiled stem {stem_id:?} maps to more than one source node")]
    #[doc = "Reports duplicate source node."]
    DuplicateSourceNode {
        #[doc = "Identifies the stem associated with `DuplicateSourceNode`."]
        stem_id: StemId,
    },
    #[error("compiled external source instance {source_instance_id:?} output '{output_port}' has no matching audio ingress node")]
    #[doc = "Reports missing external audio ingress."]
    MissingExternalAudioIngress {
        #[doc = "Identifies the source instance associated with `MissingExternalAudioIngress`."]
        source_instance_id: SourceInstanceId,
        #[doc = "Stores the output port associated with `MissingExternalAudioIngress`."]
        output_port: String,
    },
    #[error("compiled external source type {source_type_id} has no registered definition")]
    #[doc = "Reports missing external source definition."]
    MissingExternalSourceDefinition {
        #[doc = "Identifies the source type associated with `MissingExternalSourceDefinition`."]
        source_type_id: SourceTypeId,
    },
    #[error("external source instance {source_instance_id:?} output '{output_port}' has incompatible or non-concrete PCM media")]
    #[doc = "Reports invalid external audio media."]
    InvalidExternalAudioMedia {
        #[doc = "Identifies the source instance associated with `InvalidExternalAudioMedia`."]
        source_instance_id: SourceInstanceId,
        #[doc = "Stores the output port associated with `InvalidExternalAudioMedia`."]
        output_port: String,
    },
    #[error("generated audio stem {stem_id:?} has no matching ingress node")]
    #[doc = "Reports missing generated audio ingress."]
    MissingGeneratedAudioIngress {
        #[doc = "Identifies the stem associated with `MissingGeneratedAudioIngress`."]
        stem_id: StemId,
    },
    #[error("generated audio stem {stem_id:?} has incompatible or non-concrete PCM media")]
    #[doc = "Reports invalid generated audio media."]
    InvalidGeneratedAudioMedia {
        #[doc = "Identifies the stem associated with `InvalidGeneratedAudioMedia`."]
        stem_id: StemId,
    },
    #[error("generated audio stem {stem_id:?} has no matching operator bridge edge")]
    #[doc = "Reports missing generated audio bridge."]
    MissingGeneratedAudioBridge {
        #[doc = "Identifies the stem associated with `MissingGeneratedAudioBridge`."]
        stem_id: StemId,
    },
    #[error("compiled external source route {route_id:?} has no matching edge")]
    #[doc = "Reports missing external source route edge."]
    MissingExternalSourceRouteEdge {
        #[doc = "Identifies the route associated with `MissingExternalSourceRouteEdge`."]
        route_id: RouteId,
    },
    #[error("worker edge {edge_id:?} target is absent from the compiled graph")]
    #[doc = "Reports missing worker target."]
    MissingWorkerTarget {
        #[doc = "Identifies the edge associated with `MissingWorkerTarget`."]
        edge_id: EdgeId,
    },
    #[error("worker edge {edge_id:?} is absent from the compiled graph")]
    #[doc = "Reports missing worker edge."]
    MissingWorkerEdge {
        #[doc = "Identifies the edge associated with `MissingWorkerEdge`."]
        edge_id: EdgeId,
    },
    #[error("worker edge {edge_id:?} has no negotiated edge contract")]
    #[doc = "Reports missing worker edge contract."]
    MissingWorkerEdgeContract {
        #[doc = "Identifies the edge associated with `MissingWorkerEdgeContract`."]
        edge_id: EdgeId,
    },
    #[error("worker edge {edge_id:?} has no bounded runtime capacity")]
    #[doc = "Reports missing worker capacity."]
    MissingWorkerCapacity {
        #[doc = "Identifies the edge associated with `MissingWorkerCapacity`."]
        edge_id: EdgeId,
    },
    #[error("audio endpoint worker edge {edge_id:?} has no concrete audio sample specification")]
    #[doc = "Reports missing worker sample spec."]
    MissingWorkerSampleSpec {
        #[doc = "Identifies the edge associated with `MissingWorkerSampleSpec`."]
        edge_id: EdgeId,
    },
    #[error("worker edge {edge_id:?} target port '{port_name}' is absent or disagrees with the compiled signal/media")]
    #[doc = "Reports invalid operator input port."]
    InvalidOperatorInputPort {
        #[doc = "Identifies the edge associated with `InvalidOperatorInputPort`."]
        edge_id: EdgeId,
        #[doc = "Stores the port name associated with `InvalidOperatorInputPort`."]
        port_name: String,
    },
    #[error("worker edge {edge_id:?} maps to unknown route {route_id:?}")]
    #[doc = "Reports unknown worker route."]
    UnknownWorkerRoute {
        #[doc = "Identifies the edge associated with `UnknownWorkerRoute`."]
        edge_id: EdgeId,
        #[doc = "Identifies the route associated with `UnknownWorkerRoute`."]
        route_id: RouteId,
    },
    #[error("worker route {route_id:?} is mapped more than once")]
    #[doc = "Reports duplicate worker route."]
    DuplicateWorkerRoute {
        #[doc = "Identifies the route associated with `DuplicateWorkerRoute`."]
        route_id: RouteId,
    },
    #[error(
        "worker edge {edge_id:?} metadata does not match route {route_id:?}: expected stem {expected_stem_id:?} and endpoint {expected_endpoint_id:?}, got stem {actual_stem_id:?} and endpoint {actual_endpoint_id:?}"
    )]
    #[doc = "Reports worker route mismatch."]
    WorkerRouteMismatch {
        #[doc = "Identifies the edge associated with `WorkerRouteMismatch`."]
        edge_id: EdgeId,
        #[doc = "Identifies the route associated with `WorkerRouteMismatch`."]
        route_id: RouteId,
        #[doc = "Identifies the expected stem associated with `WorkerRouteMismatch`."]
        expected_stem_id: StemId,
        #[doc = "Identifies the actual stem associated with `WorkerRouteMismatch`."]
        actual_stem_id: StemId,
        #[doc = "Identifies the expected endpoint associated with `WorkerRouteMismatch`."]
        expected_endpoint_id: EndpointId,
        #[doc = "Identifies the actual endpoint associated with `WorkerRouteMismatch`."]
        actual_endpoint_id: EndpointId,
    },
    #[error("derived edge {edge_id:?} has no compiled typed-edge plan")]
    #[doc = "Reports missing typed edge plan."]
    MissingTypedEdgePlan {
        #[doc = "Identifies the edge associated with `MissingTypedEdgePlan`."]
        edge_id: EdgeId,
    },
    #[error("compiled operator node {node_id:?} has no registered async factory")]
    #[doc = "Reports missing async operator factory."]
    MissingAsyncOperatorFactory {
        #[doc = "Identifies the node associated with `MissingAsyncOperatorFactory`."]
        node_id: NodeId,
    },
    #[error("compiled operator node {node_id:?} does not match Session operator declaration")]
    #[doc = "Reports operator declaration mismatch."]
    OperatorDeclarationMismatch {
        #[doc = "Identifies the node associated with `OperatorDeclarationMismatch`."]
        node_id: NodeId,
    },
    #[error("compiled node {node_id:?} has no typed Session binding")]
    #[doc = "Reports missing node binding."]
    MissingNodeBinding {
        #[doc = "Identifies the node associated with `MissingNodeBinding`."]
        node_id: NodeId,
    },
    #[error("compiled node {node_id:?} has an incompatible typed Session binding")]
    #[doc = "Reports incompatible node binding."]
    IncompatibleNodeBinding {
        #[doc = "Identifies the node associated with `IncompatibleNodeBinding`."]
        node_id: NodeId,
    },
    #[error("compiled operator node {node_id:?} received the same input edge more than once")]
    #[doc = "Reports duplicate operator input."]
    DuplicateOperatorInput {
        #[doc = "Identifies the node associated with `DuplicateOperatorInput`."]
        node_id: NodeId,
    },
    #[error("operator signal edge {edge_id:?} has no compiled input")]
    #[doc = "Reports missing operator signal input."]
    MissingOperatorSignalInput {
        #[doc = "Identifies the edge associated with `MissingOperatorSignalInput`."]
        edge_id: EdgeId,
    },
    #[error("signal endpoint route {route_id:?} is mapped more than once")]
    #[doc = "Reports duplicate signal route."]
    DuplicateSignalRoute {
        #[doc = "Identifies the route associated with `DuplicateSignalRoute`."]
        route_id: RouteId,
    },
    #[error("signal edge {edge_id:?} does not match endpoint route {route_id:?} declaration")]
    #[doc = "Reports signal route mismatch."]
    SignalRouteMismatch {
        #[doc = "Identifies the edge associated with `SignalRouteMismatch`."]
        edge_id: EdgeId,
        #[doc = "Identifies the route associated with `SignalRouteMismatch`."]
        route_id: RouteId,
    },
    #[error(
        "compiled plan produced {actual} audio, {actual_operator_inputs} operator-input, and {actual_signal_endpoints} signal-endpoint receivers; expected {expected} audio, {expected_operator_inputs} operator-input, and {expected_signal_endpoints} signal-endpoint receivers"
    )]
    #[doc = "Reports worker topology mismatch."]
    WorkerTopologyMismatch {
        #[doc = "Records the value expected by `WorkerTopologyMismatch`."]
        expected: usize,
        #[doc = "Records the value observed by `WorkerTopologyMismatch`."]
        actual: usize,
        #[doc = "Stores the expected operator inputs associated with `WorkerTopologyMismatch`."]
        expected_operator_inputs: usize,
        #[doc = "Stores the actual operator inputs associated with `WorkerTopologyMismatch`."]
        actual_operator_inputs: usize,
        #[doc = "Stores the expected signal endpoints associated with `WorkerTopologyMismatch`."]
        expected_signal_endpoints: usize,
        #[doc = "Stores the actual signal endpoints associated with `WorkerTopologyMismatch`."]
        actual_signal_endpoints: usize,
    },
}
