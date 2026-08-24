//! Typed failures produced while materializing a compiled Session.

use crate::frame::{EndpointId, RouteId, StemId};
use crate::graph::{EdgeId, NodeId};
use crate::runtime::{ExecError, PlanRunnerError};
use crate::session::{SourceInstanceId, SourceTypeId};

#[derive(Debug, thiserror::Error)]
#[doc = "Classifies failures produced during session resource preparation."]
pub enum SessionPrepareError {
    #[error(transparent)]
    #[doc = "Classifies a failure at the runtime stage or component of `SessionPrepareError`."]
    Runtime(#[from] ExecError),
    #[error(transparent)]
    #[doc = "Classifies a failure at the source channel stage or component of `SessionPrepareError`."]
    SourceChannel(#[from] PlanRunnerError),
    #[error("compiled stem {stem_id:?} has no matching source node")]
    #[doc = "Reports that the required source node is missing."]
    MissingSourceNode {
        #[doc = "Identifies the stem identifier recorded by `MissingSourceNode`."]
        stem_id: StemId,
    },
    #[error("compiled stem {stem_id:?} maps to more than one source node")]
    #[doc = "Reports that source node duplicates an existing declaration or record."]
    DuplicateSourceNode {
        #[doc = "Identifies the stem identifier recorded by `DuplicateSourceNode`."]
        stem_id: StemId,
    },
    #[error("compiled external source instance {source_instance_id:?} output '{output_port}' has no matching audio ingress node")]
    #[doc = "Reports that the required external audio ingress is missing."]
    MissingExternalAudioIngress {
        #[doc = "Identifies the source instance identifier recorded by `MissingExternalAudioIngress`."]
        source_instance_id: SourceInstanceId,
        #[doc = "References the output port participating in `MissingExternalAudioIngress`."]
        output_port: String,
    },
    #[error("compiled external source type {source_type_id} has no registered definition")]
    #[doc = "Reports that the required external source definition is missing."]
    MissingExternalSourceDefinition {
        #[doc = "Identifies the source type identifier recorded by `MissingExternalSourceDefinition`."]
        source_type_id: SourceTypeId,
    },
    #[error("external source instance {source_instance_id:?} output '{output_port}' has incompatible or non-concrete PCM media")]
    #[doc = "Reports that the supplied external audio media is invalid."]
    InvalidExternalAudioMedia {
        #[doc = "Identifies the source instance identifier recorded by `InvalidExternalAudioMedia`."]
        source_instance_id: SourceInstanceId,
        #[doc = "References the output port participating in `InvalidExternalAudioMedia`."]
        output_port: String,
    },
    #[error("generated audio stem {stem_id:?} has no matching ingress node")]
    #[doc = "Reports that the required generated audio ingress is missing."]
    MissingGeneratedAudioIngress {
        #[doc = "Identifies the stem identifier recorded by `MissingGeneratedAudioIngress`."]
        stem_id: StemId,
    },
    #[error("generated audio stem {stem_id:?} has incompatible or non-concrete PCM media")]
    #[doc = "Reports that the supplied generated audio media is invalid."]
    InvalidGeneratedAudioMedia {
        #[doc = "Identifies the stem identifier recorded by `InvalidGeneratedAudioMedia`."]
        stem_id: StemId,
    },
    #[error("generated audio stem {stem_id:?} has no matching operator bridge edge")]
    #[doc = "Reports that the required generated audio bridge is missing."]
    MissingGeneratedAudioBridge {
        #[doc = "Identifies the stem identifier recorded by `MissingGeneratedAudioBridge`."]
        stem_id: StemId,
    },
    #[error("compiled external source route {route_id:?} has no matching edge")]
    #[doc = "Reports that the required external source route edge is missing."]
    MissingExternalSourceRouteEdge {
        #[doc = "Identifies the route identifier recorded by `MissingExternalSourceRouteEdge`."]
        route_id: RouteId,
    },
    #[error("worker edge {edge_id:?} target is absent from the compiled graph")]
    #[doc = "Reports that the required worker target is missing."]
    MissingWorkerTarget {
        #[doc = "Identifies the edge identifier recorded by `MissingWorkerTarget`."]
        edge_id: EdgeId,
    },
    #[error("worker edge {edge_id:?} is absent from the compiled graph")]
    #[doc = "Reports that the required worker edge is missing."]
    MissingWorkerEdge {
        #[doc = "Identifies the edge identifier recorded by `MissingWorkerEdge`."]
        edge_id: EdgeId,
    },
    #[error("worker edge {edge_id:?} has no negotiated edge contract")]
    #[doc = "Reports that the required worker edge contract is missing."]
    MissingWorkerEdgeContract {
        #[doc = "Identifies the edge identifier recorded by `MissingWorkerEdgeContract`."]
        edge_id: EdgeId,
    },
    #[error("worker edge {edge_id:?} has no bounded runtime capacity")]
    #[doc = "Reports that the required worker capacity is missing."]
    MissingWorkerCapacity {
        #[doc = "Identifies the edge identifier recorded by `MissingWorkerCapacity`."]
        edge_id: EdgeId,
    },
    #[error("audio endpoint worker edge {edge_id:?} has no concrete audio sample specification")]
    #[doc = "Reports that the required worker sample spec is missing."]
    MissingWorkerSampleSpec {
        #[doc = "Identifies the edge identifier recorded by `MissingWorkerSampleSpec`."]
        edge_id: EdgeId,
    },
    #[error("worker edge {edge_id:?} target port '{port_name}' is absent or disagrees with the compiled signal/media")]
    #[doc = "Reports that the supplied operator input port is invalid."]
    InvalidOperatorInputPort {
        #[doc = "Identifies the edge identifier recorded by `InvalidOperatorInputPort`."]
        edge_id: EdgeId,
        #[doc = "Stores the human-readable port used to identify `InvalidOperatorInputPort`."]
        port_name: String,
    },
    #[error("worker edge {edge_id:?} maps to unknown route {route_id:?}")]
    #[doc = "Reports that the referenced worker route is not declared or registered."]
    UnknownWorkerRoute {
        #[doc = "Identifies the edge identifier recorded by `UnknownWorkerRoute`."]
        edge_id: EdgeId,
        #[doc = "Identifies the route identifier recorded by `UnknownWorkerRoute`."]
        route_id: RouteId,
    },
    #[error("worker route {route_id:?} is mapped more than once")]
    #[doc = "Reports that worker route duplicates an existing declaration or record."]
    DuplicateWorkerRoute {
        #[doc = "Identifies the route identifier recorded by `DuplicateWorkerRoute`."]
        route_id: RouteId,
    },
    #[error(
        "worker edge {edge_id:?} metadata does not match route {route_id:?}: expected stem {expected_stem_id:?} and endpoint {expected_endpoint_id:?}, got stem {actual_stem_id:?} and endpoint {actual_endpoint_id:?}"
    )]
    #[doc = "Reports that worker route does not match the expected contract."]
    WorkerRouteMismatch {
        #[doc = "Identifies the edge identifier recorded by `WorkerRouteMismatch`."]
        edge_id: EdgeId,
        #[doc = "Identifies the route identifier recorded by `WorkerRouteMismatch`."]
        route_id: RouteId,
        #[doc = "Identifies the expected stem identifier recorded by `WorkerRouteMismatch`."]
        expected_stem_id: StemId,
        #[doc = "Identifies the actual stem identifier recorded by `WorkerRouteMismatch`."]
        actual_stem_id: StemId,
        #[doc = "Identifies the expected endpoint identifier recorded by `WorkerRouteMismatch`."]
        expected_endpoint_id: EndpointId,
        #[doc = "Identifies the actual endpoint identifier recorded by `WorkerRouteMismatch`."]
        actual_endpoint_id: EndpointId,
    },
    #[error("derived edge {edge_id:?} has no compiled typed-edge plan")]
    #[doc = "Reports that the required typed edge plan is missing."]
    MissingTypedEdgePlan {
        #[doc = "Identifies the edge identifier recorded by `MissingTypedEdgePlan`."]
        edge_id: EdgeId,
    },
    #[error("compiled operator node {node_id:?} has no registered async factory")]
    #[doc = "Reports that the required async operator factory is missing."]
    MissingAsyncOperatorFactory {
        #[doc = "Identifies the node identifier recorded by `MissingAsyncOperatorFactory`."]
        node_id: NodeId,
    },
    #[error("compiled operator node {node_id:?} does not match Session operator declaration")]
    #[doc = "Reports that operator declaration does not match the expected contract."]
    OperatorDeclarationMismatch {
        #[doc = "Identifies the node identifier recorded by `OperatorDeclarationMismatch`."]
        node_id: NodeId,
    },
    #[error("compiled node {node_id:?} has no typed Session binding")]
    #[doc = "Reports that the required node binding is missing."]
    MissingNodeBinding {
        #[doc = "Identifies the node identifier recorded by `MissingNodeBinding`."]
        node_id: NodeId,
    },
    #[error("compiled node {node_id:?} has an incompatible typed Session binding")]
    #[doc = "Reports that node binding is incompatible with the required contract."]
    IncompatibleNodeBinding {
        #[doc = "Identifies the node identifier recorded by `IncompatibleNodeBinding`."]
        node_id: NodeId,
    },
    #[error("compiled operator node {node_id:?} received the same input edge more than once")]
    #[doc = "Reports that operator input duplicates an existing declaration or record."]
    DuplicateOperatorInput {
        #[doc = "Identifies the node identifier recorded by `DuplicateOperatorInput`."]
        node_id: NodeId,
    },
    #[error("operator signal edge {edge_id:?} has no compiled input")]
    #[doc = "Reports that the required operator signal input is missing."]
    MissingOperatorSignalInput {
        #[doc = "Identifies the edge identifier recorded by `MissingOperatorSignalInput`."]
        edge_id: EdgeId,
    },
    #[error("signal endpoint route {route_id:?} is mapped more than once")]
    #[doc = "Reports that signal route duplicates an existing declaration or record."]
    DuplicateSignalRoute {
        #[doc = "Identifies the route identifier recorded by `DuplicateSignalRoute`."]
        route_id: RouteId,
    },
    #[error("signal edge {edge_id:?} does not match endpoint route {route_id:?} declaration")]
    #[doc = "Reports that signal route does not match the expected contract."]
    SignalRouteMismatch {
        #[doc = "Identifies the edge identifier recorded by `SignalRouteMismatch`."]
        edge_id: EdgeId,
        #[doc = "Identifies the route identifier recorded by `SignalRouteMismatch`."]
        route_id: RouteId,
    },
    #[error(
        "compiled plan produced {actual} audio, {actual_operator_inputs} operator-input, and {actual_signal_endpoints} signal-endpoint receivers; expected {expected} audio, {expected_operator_inputs} operator-input, and {expected_signal_endpoints} signal-endpoint receivers"
    )]
    #[doc = "Reports that worker topology does not match the expected contract."]
    WorkerTopologyMismatch {
        #[doc = "Records the value expected by `WorkerTopologyMismatch`."]
        expected: usize,
        #[doc = "Records the value observed by `WorkerTopologyMismatch`."]
        actual: usize,
        #[doc = "References the expected operator inputs participating in `WorkerTopologyMismatch`."]
        expected_operator_inputs: usize,
        #[doc = "References the actual operator inputs participating in `WorkerTopologyMismatch`."]
        actual_operator_inputs: usize,
        #[doc = "References the expected signal endpoints participating in `WorkerTopologyMismatch`."]
        expected_signal_endpoints: usize,
        #[doc = "References the actual signal endpoints participating in `WorkerTopologyMismatch`."]
        actual_signal_endpoints: usize,
    },
}
