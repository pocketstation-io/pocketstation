use std::sync::Arc;

use crate::frame::{FrameLineage, SampleFormat, SampleSpec, SourceId, StreamId};
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

    #[doc = "Returns the source identifier held by `EndpointAudioFrame`."]
    pub fn source_id(&self) -> SourceId {
        self.frame.source_id()
    }

    #[doc = "Returns the stream identifier held by `EndpointAudioFrame`."]
    pub fn stream_id(&self) -> StreamId {
        self.frame.stream_id()
    }

    #[doc = "Returns the sequence number held by `EndpointAudioFrame`."]
    pub fn sequence_number(&self) -> u64 {
        self.frame.sequence_number()
    }

    #[doc = "Returns the timestamp nanoseconds held by `EndpointAudioFrame`."]
    pub fn timestamp_ns(&self) -> u64 {
        self.frame.timestamp_ns()
    }

    #[doc = "Returns the sample rate hertz held by `EndpointAudioFrame`."]
    pub fn sample_rate_hz(&self) -> u32 {
        self.frame.sample_rate_hz()
    }

    #[doc = "Returns the channel count represented by `EndpointAudioFrame`."]
    pub fn channels(&self) -> u8 {
        self.frame.channels()
    }

    #[doc = "Returns the sample format held by `EndpointAudioFrame`."]
    pub fn sample_format(&self) -> SampleFormat {
        self.frame.sample_format()
    }

    #[doc = "Returns the audio samples held by `EndpointAudioFrame`."]
    pub fn samples(&self) -> &[f32] {
        self.frame.samples()
    }

    #[doc = "Returns the frame lineage carried by `EndpointAudioFrame`."]
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

    #[doc = "Attempts to receive the next value from `EndpointAudioReceiver` without waiting."]
    pub fn try_recv(&mut self) -> Option<EndpointAudioFrame> {
        self.receiver
            .try_recv()
            .map(|frame| EndpointAudioFrame { frame })
    }

    #[doc = "Returns whether abandoned applies to `EndpointAudioReceiver`."]
    pub fn is_abandoned(&self) -> bool {
        self.receiver.is_abandoned()
    }

    #[doc = "Marks the next value from `EndpointAudioReceiver` as discontinuous."]
    pub fn mark_discontinuity(&self) {
        self.receiver.mark_discontinuity();
    }

    #[doc = "Returns the mark worker failure held by `EndpointAudioReceiver`."]
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

    #[doc = "Attempts to receive the next value from `EndpointSignalReceiver` without waiting."]
    pub fn try_recv(&mut self) -> Option<Arc<SignalEnvelope>> {
        self.receiver.recv()
    }

    #[doc = "Receives the next value from `EndpointSignalReceiver`."]
    pub fn recv(&mut self) -> Option<Arc<SignalEnvelope>> {
        self.try_recv()
    }

    #[doc = "Returns whether abandoned applies to `EndpointSignalReceiver`."]
    pub fn is_abandoned(&self) -> bool {
        self.receiver.is_abandoned()
    }
}

#[doc = "Enumerates the supported endpoint receiver cases."]
pub enum EndpointReceiver {
    #[doc = "Represents the audio case of `EndpointReceiver`."]
    Audio {
        #[doc = "Stores the receiver used by `Audio`."]
        receiver: EndpointAudioReceiver,
        #[doc = "Stores the sample spec used by `Audio`."]
        sample_spec: SampleSpec,
    },
    #[doc = "Represents the signal case of `EndpointReceiver`."]
    Signal(EndpointSignalReceiver),
}

#[doc = "Carries typed input for endpoint port."]
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

    #[doc = "Returns the port name held by `EndpointPortInput`."]
    pub fn port_name(&self) -> &str {
        &self.port_name
    }

    #[doc = "Returns the signal spec held by `EndpointPortInput`."]
    pub const fn signal_spec(&self) -> &SignalSpec {
        &self.signal
    }

    #[doc = "Returns the media held by `EndpointPortInput`."]
    pub const fn media(&self) -> &MediaCaps {
        &self.media
    }

    #[doc = "Returns the edge contract held by `EndpointPortInput`."]
    pub const fn edge_contract(&self) -> &EdgeContract {
        &self.edge_contract
    }

    #[doc = "Returns the context held by `EndpointPortInput`."]
    pub const fn context(&self) -> &EndpointPrepareContext {
        &self.context
    }

    #[doc = "Returns the receiver held by `EndpointPortInput`."]
    pub const fn receiver(&self) -> &EndpointReceiver {
        &self.receiver
    }

    #[doc = "Consumes `EndpointPortInput` and returns its component values."]
    pub fn into_parts(self) -> (EndpointReceiver, EndpointPrepareContext) {
        (self.receiver, self.context)
    }
}

#[doc = "Implement this trait to provide endpoint behavior to PocketStation; its methods define the preparation and runtime contract."]
pub trait EndpointDriverFactory: Send + Sync {
    #[doc = "Returns the preparation group held by `EndpointDriverFactory`."]
    fn preparation_group(
        &self,
        route_id: crate::frame::RouteId,
        _configuration: &crate::graph::NodeConfig,
    ) -> Result<EndpointPreparationGroup, EndpointFailure> {
        Ok(EndpointPreparationGroup::Route(route_id))
    }

    #[doc = "Prepares resources required by `EndpointDriverFactory`."]
    fn prepare(
        &self,
        inputs: Vec<EndpointPortInput>,
    ) -> Result<Box<dyn PreparedEndpointDriver>, EndpointFailure>;
}
