//! Typed resource mappings between a compiled graph and runtime preparation.

use std::sync::Arc;

use crate::endpoint::OperatorId;
use crate::frame::SampleSpec;
use crate::frame::{ConnectorId, EndpointId, RouteId, SourceId, StemId, StreamId};
use crate::graph::{
    AsyncOperatorFactory, EdgeContract, EdgeId, MediaCaps, NodeConfig, NodeId, NodeTypeId,
    PrepareContext, SignalSpec,
};
use crate::runtime::{
    AsyncOperatorOutputBranchSpec, PlanEdgeReceiver, PlanSourceSender, TypedEdgeBranchSpec,
};
use crate::session::declaration::OperatorInstanceId;
use crate::session::{SourceConfiguration, SourceInstanceId, SourceTypeId};

#[doc = "Correlates the prepared identities and runtime resources for prepared source."]
pub struct PreparedSourceMapping {
    pub(crate) stem_id: StemId,
    pub(crate) sender: PlanSourceSender,
}

impl PreparedSourceMapping {
    #[cfg(any(test, feature = "internal-testing"))]
    #[doc = "Returns the stem identifier held by `PreparedSourceMapping`."]
    pub const fn stem_id(&self) -> StemId {
        self.stem_id
    }

    #[cfg(any(test, feature = "internal-testing"))]
    #[doc = "Returns the sender observations held by `PreparedSourceMapping`."]
    pub fn sender_observations(&self) -> crate::runtime::PlanSourceInputObservations {
        self.sender.observations()
    }
}

#[doc = "Correlates the prepared identities and runtime resources for prepared worker."]
pub struct PreparedWorkerMapping {
    pub(crate) route_id: RouteId,
    pub(crate) endpoint_id: EndpointId,
    pub(crate) connector_id: Option<ConnectorId>,
    pub(crate) node_configuration: NodeConfig,
    pub(crate) receiver: PlanEdgeReceiver,
    pub(crate) prepare_context: PrepareContext,
    pub(crate) input_port: String,
    pub(crate) signal_spec: SignalSpec,
    pub(crate) media: MediaCaps,
    pub(crate) edge_contract: EdgeContract,
    pub(crate) origin: PreparedWorkerOrigin,
}

#[derive(Clone, Copy, Debug)]
pub(crate) enum PreparedWorkerOrigin {
    Stem(StemId),
    SignalIngress {
        stem_id: StemId,
        source_id: SourceId,
        stream_id: StreamId,
    },
}

impl PreparedWorkerOrigin {
    pub(crate) const fn stem_id(&self) -> StemId {
        match self {
            Self::Stem(stem_id) | Self::SignalIngress { stem_id, .. } => *stem_id,
        }
    }
}

pub(crate) struct PreparedExternalSourceMapping {
    pub(crate) instance_id: SourceInstanceId,
    pub(crate) source_id: SourceId,
    pub(crate) source_type_id: SourceTypeId,
    pub(crate) configuration: SourceConfiguration,
    pub(crate) branches: Vec<PreparedExternalSourceBranch>,
}

pub(crate) struct PreparedExternalSourceBranch {
    pub(crate) output_port: String,
    pub(crate) stream_id: StreamId,
    pub(crate) branch: TypedEdgeBranchSpec,
    pub(crate) target: PreparedExternalSourceTarget,
}

pub(crate) enum PreparedExternalSourceTarget {
    AudioIngress(PreparedExternalAudioIngress),
    TypedEndpoint(PreparedExternalTypedRoute),
    OperatorInput(PreparedExternalOperatorInput),
}

pub(crate) struct PreparedExternalOperatorInput {
    pub(crate) operator_instance_id: OperatorInstanceId,
    pub(crate) input_port: String,
    pub(crate) edge_id: EdgeId,
    pub(crate) signal_spec: SignalSpec,
    pub(crate) media: MediaCaps,
    pub(crate) edge_contract: EdgeContract,
    pub(crate) capacity_signals: usize,
}

pub(crate) struct PreparedExternalAudioIngress {
    pub(crate) stem_id: StemId,
    pub(crate) stream_id: StreamId,
    pub(crate) source_id: SourceId,
    pub(crate) sample_spec: SampleSpec,
    pub(crate) samples_per_frame: usize,
    pub(crate) sender: PlanSourceSender,
}

pub(crate) struct PreparedGeneratedAudioIngress {
    pub(crate) stem_id: StemId,
    pub(crate) stream_id: StreamId,
    pub(crate) source_id: SourceId,
    pub(crate) sample_spec: SampleSpec,
    pub(crate) samples_per_frame: usize,
    pub(crate) sender: PlanSourceSender,
}

pub(crate) struct PreparedExternalTypedRoute {
    pub(crate) route_id: RouteId,
    pub(crate) endpoint_id: EndpointId,
    pub(crate) connector_id: Option<ConnectorId>,
    pub(crate) endpoint_operator_id: OperatorId,
    pub(crate) endpoint_node_type_id: NodeTypeId,
    pub(crate) stream_id: StreamId,
    pub(crate) source_id: SourceId,
    pub(crate) node_configuration: NodeConfig,
    pub(crate) input_port: String,
    pub(crate) signal_spec: SignalSpec,
    pub(crate) media: MediaCaps,
    pub(crate) edge_contract: EdgeContract,
}

#[doc = "Correlates the prepared identities and runtime resources for prepared signal route."]
pub struct PreparedSignalRouteMapping {
    pub(crate) route_id: RouteId,
    pub(crate) stem_id: Option<StemId>,
    pub(crate) signal_origin: Option<(SourceId, StreamId)>,
    pub(crate) endpoint_id: EndpointId,
    pub(crate) connector_id: Option<ConnectorId>,
    pub(crate) node_configuration: NodeConfig,
    pub(crate) input_port: String,
    pub(crate) signal_spec: SignalSpec,
    pub(crate) output_branch: AsyncOperatorOutputBranchSpec,
}

impl PreparedSignalRouteMapping {
    #[cfg(any(test, feature = "internal-testing"))]
    pub const fn route_id(&self) -> RouteId {
        self.route_id
    }

    #[cfg(any(test, feature = "internal-testing"))]
    pub const fn stem_id(&self) -> Option<StemId> {
        self.stem_id
    }

    #[cfg(any(test, feature = "internal-testing"))]
    pub const fn endpoint_id(&self) -> EndpointId {
        self.endpoint_id
    }
}

#[doc = "Correlates the prepared identities and runtime resources for prepared operator."]
pub struct PreparedOperatorMapping {
    pub(crate) node_id: NodeId,
    pub(crate) instance_id: OperatorInstanceId,
    pub(crate) factory: Arc<dyn AsyncOperatorFactory>,
    pub(crate) node_configuration: NodeConfig,
    pub(crate) inputs: Vec<PreparedOperatorInputMapping>,
    pub(crate) outputs: Vec<PreparedOperatorOutputMapping>,
}

pub(crate) enum PreparedOperatorInputMapping {
    Compiled {
        stem_id: StemId,
        input_port: String,
        signal_spec: SignalSpec,
        media: MediaCaps,
        edge_contract: EdgeContract,
        capacity_signals: usize,
        receiver: PlanEdgeReceiver,
    },
    Typed {
        edge_id: EdgeId,
        input_port: String,
        signal_spec: SignalSpec,
        media: MediaCaps,
        edge_contract: EdgeContract,
        capacity_signals: usize,
        origin: PreparedTypedInputOrigin,
    },
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) enum PreparedTypedInputOrigin {
    SourceOutput {
        source_instance_id: SourceInstanceId,
        output_port: String,
    },
    OperatorOutput {
        operator_instance_id: OperatorInstanceId,
        output_port: String,
    },
}

pub(crate) struct PreparedOperatorOutputMapping {
    pub(crate) output_port: String,
    pub(crate) branch: AsyncOperatorOutputBranchSpec,
    pub(crate) target: PreparedOperatorOutputTarget,
}

pub(crate) enum PreparedOperatorOutputTarget {
    OperatorInput {
        operator_instance_id: OperatorInstanceId,
        input_port: String,
    },
    SignalEndpoint(Box<PreparedSignalRouteMapping>),
    GeneratedAudio(PreparedGeneratedAudioIngress),
}

impl PreparedOperatorMapping {
    #[cfg(any(test, feature = "internal-testing"))]
    pub const fn node_id(&self) -> NodeId {
        self.node_id
    }

    #[cfg(any(test, feature = "internal-testing"))]
    pub fn output_count(&self) -> usize {
        self.outputs.len()
    }

    #[cfg(any(test, feature = "internal-testing"))]
    pub fn signal_routes(&self) -> Vec<&PreparedSignalRouteMapping> {
        self.outputs
            .iter()
            .filter_map(|output| match &output.target {
                PreparedOperatorOutputTarget::SignalEndpoint(route) => Some(route.as_ref()),
                PreparedOperatorOutputTarget::OperatorInput { .. }
                | PreparedOperatorOutputTarget::GeneratedAudio(_) => None,
            })
            .collect()
    }
}

impl PreparedWorkerMapping {
    #[cfg(any(test, feature = "internal-testing"))]
    #[doc = "Returns the route identifier held by `PreparedWorkerMapping`."]
    pub const fn route_id(&self) -> RouteId {
        self.route_id
    }

    #[cfg(any(test, feature = "internal-testing"))]
    #[doc = "Returns the stem identifier held by `PreparedWorkerMapping`."]
    pub const fn stem_id(&self) -> StemId {
        self.origin.stem_id()
    }

    #[cfg(any(test, feature = "internal-testing"))]
    #[doc = "Returns the endpoint identifier held by `PreparedWorkerMapping`."]
    pub const fn endpoint_id(&self) -> EndpointId {
        self.endpoint_id
    }

    #[cfg(any(test, feature = "internal-testing"))]
    #[doc = "Returns the node configuration held by `PreparedWorkerMapping`."]
    pub const fn node_configuration(&self) -> &NodeConfig {
        &self.node_configuration
    }

    #[cfg(any(test, feature = "internal-testing"))]
    #[doc = "Returns the receiver observations held by `PreparedWorkerMapping`."]
    pub fn receiver_observations(&self) -> crate::runtime::EdgeObservations {
        self.receiver.observations()
    }

    #[cfg(any(test, feature = "internal-testing"))]
    #[doc = "Returns the immutable preparation context retained by `PreparedWorkerMapping`."]
    pub const fn prepare_context(&self) -> &PrepareContext {
        &self.prepare_context
    }
}
