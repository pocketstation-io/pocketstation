use std::sync::Arc;

use crate::frame::{FrameLineage, SampleSpec, SourceId};
use crate::graph::{EdgeContract, MediaCaps, PrepareContext, SignalEnvelope, SignalSpec};
use crate::runtime::{
    PlanEdgeFrame, PlanEdgeObservationHandle, PlanEdgeReceiver, TypedEdgeReceiver,
};

use crate::endpoint::{
    EndpointFailure, EndpointPreparationGroup, EndpointPrepareContext, PreparedEndpointDriver,
};

/// Read-only audio frame delivered to an external endpoint.
///
/// The concrete realtime edge ownership remains private to PocketStation. This
/// view deliberately exposes only signal data and lineage needed by an
/// endpoint implementation.
pub struct EndpointAudioFrame {
    frame: PlanEdgeFrame,
}

impl EndpointAudioFrame {
    pub(crate) fn into_inner(self) -> PlanEdgeFrame {
        self.frame
    }

    pub fn source_id(&self) -> SourceId {
        self.frame.source_id()
    }

    pub fn sequence_number(&self) -> u64 {
        self.frame.sequence_number()
    }

    pub fn timestamp_ns(&self) -> u64 {
        self.frame.timestamp_ns()
    }

    pub fn sample_rate_hz(&self) -> u32 {
        self.frame.sample_rate_hz()
    }

    pub fn channels(&self) -> u8 {
        self.frame.channels()
    }

    pub fn samples(&self) -> &[f32] {
        self.frame.samples()
    }

    pub fn lineage(&self) -> FrameLineage {
        self.frame.lineage()
    }
}

/// Exclusive consumer for one bounded realtime-audio endpoint edge.
///
/// The underlying queue, router, and frame ownership variants are not part of
/// the public extension API.
pub struct EndpointAudioReceiver {
    receiver: PlanEdgeReceiver,
}

impl EndpointAudioReceiver {
    pub(crate) const fn new(receiver: PlanEdgeReceiver) -> Self {
        Self { receiver }
    }

    pub(crate) fn into_inner(self) -> PlanEdgeReceiver {
        self.receiver
    }

    #[cfg(feature = "internal-testing")]
    pub fn into_plan_edge_receiver(self) -> PlanEdgeReceiver {
        self.receiver
    }

    pub fn try_recv(&mut self) -> Option<EndpointAudioFrame> {
        self.receiver
            .try_recv()
            .map(|frame| EndpointAudioFrame { frame })
    }

    pub fn is_abandoned(&self) -> bool {
        self.receiver.is_abandoned()
    }

    pub fn mark_discontinuity(&self) {
        self.receiver.mark_discontinuity();
    }

    pub fn mark_worker_failure(&self) {
        self.receiver.mark_worker_failure();
    }

    /// Snapshots the bounded edge counters for this endpoint input.
    ///
    /// The concrete queue and router remain private; external endpoints receive
    /// only immutable observations needed for delivery/failure accounting.
    pub fn observations(&self) -> crate::runtime::EdgeObservations {
        self.receiver.observations()
    }

    pub(crate) fn observation_handle(&self) -> PlanEdgeObservationHandle {
        self.receiver.observation_handle()
    }

    #[cfg(feature = "internal-testing")]
    pub fn plan_edge_observation_handle(&self) -> PlanEdgeObservationHandle {
        self.receiver.observation_handle()
    }
}

/// Exclusive consumer for one bounded asynchronous signal endpoint edge.
pub struct EndpointSignalReceiver {
    receiver: TypedEdgeReceiver,
}

impl EndpointSignalReceiver {
    pub(crate) const fn new(receiver: TypedEdgeReceiver) -> Self {
        Self { receiver }
    }

    pub fn try_recv(&mut self) -> Option<Arc<SignalEnvelope>> {
        self.receiver.recv()
    }

    pub fn recv(&mut self) -> Option<Arc<SignalEnvelope>> {
        self.try_recv()
    }

    pub fn is_abandoned(&self) -> bool {
        self.receiver.is_abandoned()
    }
}

pub enum EndpointReceiver {
    Audio {
        receiver: EndpointAudioReceiver,
        sample_spec: SampleSpec,
    },
    Signal(EndpointSignalReceiver),
}

pub struct EndpointPortInput {
    port_name: String,
    signal: SignalSpec,
    media: MediaCaps,
    edge_contract: EdgeContract,
    receiver: EndpointReceiver,
    context: EndpointPrepareContext,
}

impl EndpointPortInput {
    pub(crate) fn audio(
        port_name: impl Into<String>,
        signal: SignalSpec,
        media: MediaCaps,
        edge_contract: EdgeContract,
        receiver: PlanEdgeReceiver,
        prepare_context: PrepareContext,
        context: EndpointPrepareContext,
    ) -> Self {
        Self {
            port_name: port_name.into(),
            signal,
            media,
            edge_contract,
            receiver: EndpointReceiver::Audio {
                receiver: EndpointAudioReceiver::new(receiver),
                sample_spec: prepare_context.sample_spec,
            },
            context,
        }
    }

    pub(crate) fn signal(
        port_name: impl Into<String>,
        signal: SignalSpec,
        media: MediaCaps,
        edge_contract: EdgeContract,
        receiver: TypedEdgeReceiver,
        context: EndpointPrepareContext,
    ) -> Self {
        Self {
            port_name: port_name.into(),
            signal,
            media,
            edge_contract,
            receiver: EndpointReceiver::Signal(EndpointSignalReceiver::new(receiver)),
            context,
        }
    }

    pub fn port_name(&self) -> &str {
        &self.port_name
    }

    pub const fn signal_spec(&self) -> &SignalSpec {
        &self.signal
    }

    pub const fn media(&self) -> &MediaCaps {
        &self.media
    }

    pub const fn edge_contract(&self) -> &EdgeContract {
        &self.edge_contract
    }

    pub const fn context(&self) -> &EndpointPrepareContext {
        &self.context
    }

    pub const fn receiver(&self) -> &EndpointReceiver {
        &self.receiver
    }

    pub fn into_parts(self) -> (EndpointReceiver, EndpointPrepareContext) {
        (self.receiver, self.context)
    }
}

pub trait EndpointDriverFactory: Send + Sync {
    fn preparation_group(
        &self,
        route_id: crate::frame::RouteId,
        _configuration: &crate::graph::NodeConfig,
    ) -> Result<EndpointPreparationGroup, EndpointFailure> {
        Ok(EndpointPreparationGroup::Route(route_id))
    }

    fn prepare(
        &self,
        inputs: Vec<EndpointPortInput>,
    ) -> Result<Box<dyn PreparedEndpointDriver>, EndpointFailure>;
}
