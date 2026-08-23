use std::sync::atomic::{AtomicU64, Ordering};

use crate::capture::CaptureOwnerObservations;
use crate::endpoint::EndpointDriverObservations;
use crate::frame::{EndpointId, RouteId, SourceId, StemId};
use crate::runtime::{
    AsyncOperatorObservations, AsyncOperatorOutputObservations, EdgeObservations,
    GeneratedAudioBridgeObservations, PlanSourceInputObservations, SidecarHostSnapshot,
};

use crate::session::{
    OperatorInstanceId, PolledAudioObservations, SourceInstanceId, SourceRuntimeObservations,
};

/// Point-in-time observations for a session's bounded control-event queue.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SessionEventQueueObservations {
    #[doc = "Sets the capacity event count available to `SessionEventQueueObservations`."]
    pub capacity_event_count: u64,
    #[doc = "Stores the maximum event owned size for `SessionEventQueueObservations`, in bytes."]
    pub maximum_event_owned_bytes: u64,
    #[doc = "Stores the maximum buffered owned size for `SessionEventQueueObservations`, in bytes."]
    pub maximum_buffered_owned_bytes: u64,
    #[doc = "Reports the depth events observed by `SessionEventQueueObservations`."]
    pub depth_events: u64,
    #[doc = "Stores the depth owned size for `SessionEventQueueObservations`, in bytes."]
    pub depth_owned_bytes: u64,
    #[doc = "Reports the peak depth event count observed by `SessionEventQueueObservations`."]
    pub peak_depth_event_count: u64,
    #[doc = "Stores the peak depth owned size for `SessionEventQueueObservations`, in bytes."]
    pub peak_depth_owned_bytes: u64,
    #[doc = "Counts the total number of events enqueued observed by `SessionEventQueueObservations`."]
    pub events_enqueued_total: u64,
    #[doc = "Counts the total number of events dropped observed by `SessionEventQueueObservations`."]
    pub events_dropped_total: u64,
    #[doc = "Counts the total number of events dropped oversized observed by `SessionEventQueueObservations`."]
    pub events_dropped_oversized_total: u64,
    #[doc = "Counts the total number of receiver closed observed by `SessionEventQueueObservations`."]
    pub receiver_closed_total: u64,
}

/// Authoritative point-in-time observations for the current Session boundary.
///
/// The snapshot keeps control-event and foreign-audio queue truth together
/// without exposing either counter owner to a language adapter.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SessionMetricsSnapshot {
    event_queue: SessionEventQueueObservations,
    polled_audio: PolledAudioObservations,
    sources: Box<[SessionSourceMetrics]>,
    external_sources: Box<[SessionExternalSourceMetrics]>,
    routes: Box<[SessionRouteMetrics]>,
    operators: Box<[SessionOperatorMetrics]>,
    derived_routes: Box<[SessionDerivedRouteMetrics]>,
}

impl SessionMetricsSnapshot {
    pub(crate) const fn new(
        event_queue: SessionEventQueueObservations,
        polled_audio: PolledAudioObservations,
        sources: Box<[SessionSourceMetrics]>,
        external_sources: Box<[SessionExternalSourceMetrics]>,
        routes: Box<[SessionRouteMetrics]>,
        operators: Box<[SessionOperatorMetrics]>,
        derived_routes: Box<[SessionDerivedRouteMetrics]>,
    ) -> Self {
        Self {
            event_queue,
            polled_audio,
            sources,
            external_sources,
            routes,
            operators,
            derived_routes,
        }
    }

    #[doc = "Returns the event queue associated with `SessionMetricsSnapshot`."]
    pub const fn event_queue(&self) -> SessionEventQueueObservations {
        self.event_queue
    }

    #[doc = "Declares a bounded polled-audio endpoint on `SessionMetricsSnapshot`."]
    pub const fn polled_audio(&self) -> PolledAudioObservations {
        self.polled_audio
    }

    #[doc = "Returns the source count associated with `SessionMetricsSnapshot`."]
    pub fn source_count(&self) -> usize {
        self.sources.len()
    }

    #[doc = "Returns the source associated with `SessionMetricsSnapshot`."]
    pub fn source(&self, index: usize) -> Option<&SessionSourceMetrics> {
        self.sources.get(index)
    }

    #[doc = "Returns the external source count associated with `SessionMetricsSnapshot`."]
    pub fn external_source_count(&self) -> usize {
        self.external_sources.len()
    }

    #[doc = "Returns the external source associated with `SessionMetricsSnapshot`."]
    pub fn external_source(&self, index: usize) -> Option<&SessionExternalSourceMetrics> {
        self.external_sources.get(index)
    }

    #[doc = "Returns the route count associated with `SessionMetricsSnapshot`."]
    pub fn route_count(&self) -> usize {
        self.routes.len()
    }

    #[doc = "Returns the route associated with `SessionMetricsSnapshot`."]
    pub fn route(&self, index: usize) -> Option<&SessionRouteMetrics> {
        self.routes.get(index)
    }

    #[doc = "Returns the operator count associated with `SessionMetricsSnapshot`."]
    pub fn operator_count(&self) -> usize {
        self.operators.len()
    }

    #[doc = "Returns the operator associated with `SessionMetricsSnapshot`."]
    pub fn operator(&self, index: usize) -> Option<&SessionOperatorMetrics> {
        self.operators.get(index)
    }

    #[doc = "Returns the derived route count associated with `SessionMetricsSnapshot`."]
    pub fn derived_route_count(&self) -> usize {
        self.derived_routes.len()
    }

    #[doc = "Returns the derived route associated with `SessionMetricsSnapshot`."]
    pub fn derived_route(&self, index: usize) -> Option<&SessionDerivedRouteMetrics> {
        self.derived_routes.get(index)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[doc = "Reports the session source metrics collected at an observation boundary."]
pub struct SessionSourceMetrics {
    #[doc = "Identifies the stem associated with `SessionSourceMetrics`."]
    pub stem_id: StemId,
    #[doc = "Stores the capture associated with `SessionSourceMetrics`."]
    pub capture: CaptureOwnerObservations,
    #[doc = "Stores the ingress associated with `SessionSourceMetrics`."]
    pub ingress: PlanSourceInputObservations,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[doc = "Reports the session external source metrics collected at an observation boundary."]
pub struct SessionExternalSourceMetrics {
    #[doc = "Identifies the source instance associated with `SessionExternalSourceMetrics`."]
    pub source_instance_id: SourceInstanceId,
    #[doc = "Identifies the source associated with `SessionExternalSourceMetrics`."]
    pub source_id: SourceId,
    #[doc = "Stores the runtime associated with `SessionExternalSourceMetrics`."]
    pub runtime: SourceRuntimeObservations,
}

/// Exact bounded-queue and process-lifecycle accounting for one Session-owned
/// language-neutral sidecar.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SessionSidecarMetrics {
    #[doc = "Identifies the sidecar associated with `SessionSidecarMetrics`."]
    pub sidecar_id: u64,
    #[doc = "Stores the host associated with `SessionSidecarMetrics`."]
    pub host: SidecarHostSnapshot,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[doc = "Reports the session route metrics collected at an observation boundary."]
pub struct SessionRouteMetrics {
    #[doc = "Identifies the route associated with `SessionRouteMetrics`."]
    pub route_id: RouteId,
    #[doc = "Identifies the endpoint associated with `SessionRouteMetrics`."]
    pub endpoint_id: EndpointId,
    #[doc = "Stores the edge associated with `SessionRouteMetrics`."]
    pub edge: EdgeObservations,
    #[doc = "Stores the endpoint associated with `SessionRouteMetrics`."]
    pub endpoint: Option<EndpointDriverObservations>,
    #[doc = "Stores the endpoint observation stage associated with `SessionRouteMetrics`."]
    pub endpoint_observation_stage: EndpointObservationStage,
    #[doc = "Counts the total number of endpoint finalization failures observed by `SessionRouteMetrics`."]
    pub endpoint_finalization_failures_total: u64,
}

/// Interval covered by monotonic route counters.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SessionRouteObservationInterval {
    /// From route start through the instant of the Session snapshot.
    RouteLifetimeToSnapshot,
}

/// Explicit numerator, denominator, interval, and typed reasons for one route.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SessionRouteDropObservations {
    #[doc = "Identifies the route associated with `SessionRouteDropObservations`."]
    pub route_id: RouteId,
    #[doc = "Stores the interval associated with `SessionRouteDropObservations`."]
    pub interval: SessionRouteObservationInterval,
    #[doc = "Counts the total number of frames dropped observed by `SessionRouteDropObservations`."]
    pub frames_dropped_total: u64,
    #[doc = "Counts the total number of frames attempted observed by `SessionRouteDropObservations`."]
    pub frames_attempted_total: u64,
    #[doc = "Counts the total number of receiver unavailable drops observed by `SessionRouteDropObservations`."]
    pub receiver_unavailable_drops_total: u64,
    #[doc = "Counts the total number of queue full drops observed by `SessionRouteDropObservations`."]
    pub queue_full_drops_total: u64,
    #[doc = "Counts the total number of shared reference exhausted drops observed by `SessionRouteDropObservations`."]
    pub shared_reference_exhausted_drops_total: u64,
    #[doc = "Counts the total number of branch pool exhausted drops observed by `SessionRouteDropObservations`."]
    pub branch_pool_exhausted_drops_total: u64,
    #[doc = "Counts the total number of invalid copy policy drops observed by `SessionRouteDropObservations`."]
    pub invalid_copy_policy_drops_total: u64,
    #[doc = "Counts the total number of freeze failed drops observed by `SessionRouteDropObservations`."]
    pub freeze_failed_drops_total: u64,
}

impl SessionRouteDropObservations {
    #[doc = "Returns the drop rate pct associated with `SessionRouteDropObservations`."]
    pub fn drop_rate_pct(self) -> f64 {
        if self.frames_attempted_total == 0 {
            0.0
        } else {
            self.frames_dropped_total as f64 / self.frames_attempted_total as f64 * 100.0
        }
    }
}

/// Common-clock source timestamp to route-receive latency in nanoseconds.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SessionRouteLatencyObservations {
    #[doc = "Identifies the route associated with `SessionRouteLatencyObservations`."]
    pub route_id: RouteId,
    #[doc = "Stores the boundary associated with `SessionRouteLatencyObservations`."]
    pub boundary: SessionRouteLatencyBoundary,
    #[doc = "Stores the unit associated with `SessionRouteLatencyObservations`."]
    pub unit: SessionRouteLatencyUnit,
    #[doc = "Counts the total number of samples observed by `SessionRouteLatencyObservations`."]
    pub samples_total: u64,
    #[doc = "Counts the total number of missing or incompatible clock observed by `SessionRouteLatencyObservations`."]
    pub missing_or_incompatible_clock_total: u64,
    #[doc = "Counts the total number of future timestamp observed by `SessionRouteLatencyObservations`."]
    pub future_timestamp_total: u64,
    #[doc = "Stores the p50 value for `SessionRouteLatencyObservations`, in nanoseconds."]
    pub p50_ns: u64,
    #[doc = "Stores the p95 value for `SessionRouteLatencyObservations`, in nanoseconds."]
    pub p95_ns: u64,
    #[doc = "Stores the p99 value for `SessionRouteLatencyObservations`, in nanoseconds."]
    pub p99_ns: u64,
    #[doc = "Stores the max value for `SessionRouteLatencyObservations`, in nanoseconds."]
    pub max_ns: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[doc = "Enumerates the supported session route latency boundary cases."]
pub enum SessionRouteLatencyBoundary {
    #[doc = "Represents the source monotonic timestamp to route receive case of `SessionRouteLatencyBoundary`."]
    SourceMonotonicTimestampToRouteReceive,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[doc = "Enumerates the supported session route latency unit cases."]
pub enum SessionRouteLatencyUnit {
    #[doc = "Represents the nanoseconds case of `SessionRouteLatencyUnit`."]
    Nanoseconds,
}

impl SessionRouteMetrics {
    #[doc = "Returns the drop observations associated with `SessionRouteMetrics`."]
    pub fn drop_observations(self) -> SessionRouteDropObservations {
        SessionRouteDropObservations {
            route_id: self.route_id,
            interval: SessionRouteObservationInterval::RouteLifetimeToSnapshot,
            frames_dropped_total: self.edge.frames_dropped_total,
            frames_attempted_total: self.edge.frames_attempted_total(),
            receiver_unavailable_drops_total: self.edge.receiver_unavailable_drops_total,
            queue_full_drops_total: self.edge.queue_full_drops_total,
            shared_reference_exhausted_drops_total: self
                .edge
                .shared_reference_exhausted_drops_total,
            branch_pool_exhausted_drops_total: self.edge.branch_pool_exhausted_drops_total,
            invalid_copy_policy_drops_total: self.edge.invalid_copy_policy_drops_total,
            freeze_failed_drops_total: self.edge.freeze_failed_drops_total,
        }
    }

    #[doc = "Returns the source to receive latency associated with `SessionRouteMetrics`."]
    pub const fn source_to_receive_latency(self) -> SessionRouteLatencyObservations {
        SessionRouteLatencyObservations {
            route_id: self.route_id,
            boundary: SessionRouteLatencyBoundary::SourceMonotonicTimestampToRouteReceive,
            unit: SessionRouteLatencyUnit::Nanoseconds,
            samples_total: self.edge.source_timestamp_to_receive_samples_total,
            missing_or_incompatible_clock_total: self
                .edge
                .source_timestamp_to_receive_missing_total,
            future_timestamp_total: self.edge.source_timestamp_to_receive_future_total,
            p50_ns: self.edge.source_timestamp_to_receive_p50_ns,
            p95_ns: self.edge.source_timestamp_to_receive_p95_ns,
            p99_ns: self.edge.source_timestamp_to_receive_p99_ns,
            max_ns: self.edge.source_timestamp_to_receive_max_ns,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[doc = "Reports the session operator input metrics collected at an observation boundary."]
pub struct SessionOperatorInputMetrics {
    #[doc = "Stores the port name associated with `SessionOperatorInputMetrics`."]
    pub port_name: String,
    #[doc = "Stores the edge associated with `SessionOperatorInputMetrics`."]
    pub edge: EdgeObservations,
}

/// Exact boundedness and lifecycle accounting for one operator PCM output
/// re-entering the Session audio lane.
///
/// This is a Session observation contract. The bridge worker and its queue are
/// deliberately not public extension APIs.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SessionAudioReentryMetrics {
    operator_instance_id: OperatorInstanceId,
    stem_id: StemId,
    queue_capacity_signals: u64,
    queue_depth_signals: u64,
    queue_peak_signals: u64,
    signals_enqueued_total: u64,
    signals_received_total: u64,
    signals_dropped_total: u64,
    pool_slots: u64,
    frame_capacity_samples: u64,
    maximum_buffered_audio_bytes: u64,
    normalized_total: u64,
    invalid_total: u64,
    shared_audio_rejected_total: u64,
    pool_exhausted_total: u64,
    ingress_rejected_total: u64,
    audio_frames_enqueued_total: u64,
    cancellation_total: u64,
    joined: bool,
}

impl SessionAudioReentryMetrics {
    pub(crate) fn from_bridge(
        operator_instance_id: OperatorInstanceId,
        stem_id: StemId,
        bridge: GeneratedAudioBridgeObservations,
    ) -> Self {
        Self {
            operator_instance_id,
            stem_id,
            queue_capacity_signals: bridge.input_edge.capacity_signals,
            queue_depth_signals: bridge.input_edge.depth_signals,
            queue_peak_signals: bridge.input_edge.peak_depth_signals,
            signals_enqueued_total: bridge.input_edge.enqueued_total,
            signals_received_total: bridge.input_edge.received_total,
            signals_dropped_total: bridge.input_edge.dropped_total,
            pool_slots: bridge.pool_slots,
            frame_capacity_samples: bridge.frame_capacity_samples,
            maximum_buffered_audio_bytes: bridge.maximum_buffered_audio_bytes,
            normalized_total: bridge.normalized_total,
            invalid_total: bridge.invalid_total,
            shared_audio_rejected_total: bridge.shared_audio_rejected_total,
            pool_exhausted_total: bridge.pool_exhausted_total,
            ingress_rejected_total: bridge.ingress_rejected_total,
            audio_frames_enqueued_total: bridge.enqueued_total,
            cancellation_total: bridge.cancellation_total,
            joined: bridge.joined,
        }
    }

    #[doc = "Returns the operator instance identifier associated with `SessionAudioReentryMetrics`."]
    pub const fn operator_instance_id(self) -> OperatorInstanceId {
        self.operator_instance_id
    }

    #[doc = "Returns the stem identifier associated with `SessionAudioReentryMetrics`."]
    pub const fn stem_id(self) -> StemId {
        self.stem_id
    }

    #[doc = "Returns the queue capacity signals associated with `SessionAudioReentryMetrics`."]
    pub const fn queue_capacity_signals(self) -> u64 {
        self.queue_capacity_signals
    }

    #[doc = "Returns the queue depth signals associated with `SessionAudioReentryMetrics`."]
    pub const fn queue_depth_signals(self) -> u64 {
        self.queue_depth_signals
    }

    #[doc = "Returns the queue peak signals associated with `SessionAudioReentryMetrics`."]
    pub const fn queue_peak_signals(self) -> u64 {
        self.queue_peak_signals
    }

    #[doc = "Returns the signals enqueued total associated with `SessionAudioReentryMetrics`."]
    pub const fn signals_enqueued_total(self) -> u64 {
        self.signals_enqueued_total
    }

    #[doc = "Returns the signals received total associated with `SessionAudioReentryMetrics`."]
    pub const fn signals_received_total(self) -> u64 {
        self.signals_received_total
    }

    #[doc = "Returns the signals dropped total associated with `SessionAudioReentryMetrics`."]
    pub const fn signals_dropped_total(self) -> u64 {
        self.signals_dropped_total
    }

    #[doc = "Returns the pool slots associated with `SessionAudioReentryMetrics`."]
    pub const fn pool_slots(self) -> u64 {
        self.pool_slots
    }

    #[doc = "Returns the frame capacity samples associated with `SessionAudioReentryMetrics`."]
    pub const fn frame_capacity_samples(self) -> u64 {
        self.frame_capacity_samples
    }

    #[doc = "Returns the maximum buffered audio bytes associated with `SessionAudioReentryMetrics`."]
    pub const fn maximum_buffered_audio_bytes(self) -> u64 {
        self.maximum_buffered_audio_bytes
    }

    #[doc = "Returns the normalized total associated with `SessionAudioReentryMetrics`."]
    pub const fn normalized_total(self) -> u64 {
        self.normalized_total
    }

    #[doc = "Returns the invalid total associated with `SessionAudioReentryMetrics`."]
    pub const fn invalid_total(self) -> u64 {
        self.invalid_total
    }

    #[doc = "Returns the shared audio rejected total associated with `SessionAudioReentryMetrics`."]
    pub const fn shared_audio_rejected_total(self) -> u64 {
        self.shared_audio_rejected_total
    }

    #[doc = "Returns the pool exhausted total associated with `SessionAudioReentryMetrics`."]
    pub const fn pool_exhausted_total(self) -> u64 {
        self.pool_exhausted_total
    }

    #[doc = "Returns the ingress rejected total associated with `SessionAudioReentryMetrics`."]
    pub const fn ingress_rejected_total(self) -> u64 {
        self.ingress_rejected_total
    }

    #[doc = "Returns the audio frames enqueued total associated with `SessionAudioReentryMetrics`."]
    pub const fn audio_frames_enqueued_total(self) -> u64 {
        self.audio_frames_enqueued_total
    }

    #[doc = "Returns the cancellation total associated with `SessionAudioReentryMetrics`."]
    pub const fn cancellation_total(self) -> u64 {
        self.cancellation_total
    }

    #[doc = "Returns the joined associated with `SessionAudioReentryMetrics`."]
    pub const fn joined(self) -> bool {
        self.joined
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[doc = "Reports the session operator metrics collected at an observation boundary."]
pub struct SessionOperatorMetrics {
    #[doc = "Identifies the operator instance associated with `SessionOperatorMetrics`."]
    pub operator_instance_id: OperatorInstanceId,
    /// Sole counter authority for input delivered by the compiled Session plan.
    ///
    /// `worker.input_*` remains meaningful only for workers fed through the
    /// direct `AsyncOperatorInput` API. Compiled Session operators consume this
    /// plan edge, so callers must use these observations for input accounting.
    pub input_edge: EdgeObservations,
    /// Exact per-port input accounting. `input_edge` is the compatibility
    /// aggregate across this slice.
    pub input_ports: Box<[SessionOperatorInputMetrics]>,
    #[doc = "Stores the worker associated with `SessionOperatorMetrics`."]
    pub worker: AsyncOperatorObservations,
    #[doc = "Counts the total number of finalization failures observed by `SessionOperatorMetrics`."]
    pub finalization_failures_total: u64,
}

impl SessionOperatorMetrics {
    #[doc = "Returns the input port associated with `SessionOperatorMetrics`."]
    pub fn input_port(&self, name: &str) -> Option<&SessionOperatorInputMetrics> {
        self.input_ports.iter().find(|port| port.port_name == name)
    }
    #[doc = "Returns the input queue capacity frames associated with `SessionOperatorMetrics`."]
    pub const fn input_queue_capacity_frames(&self) -> u64 {
        self.input_edge.queue_capacity_frames
    }

    #[doc = "Returns the input queue depth frames associated with `SessionOperatorMetrics`."]
    pub const fn input_queue_depth_frames(&self) -> u64 {
        self.input_edge.queue_depth_frames
    }

    #[doc = "Returns the input queue peak frames associated with `SessionOperatorMetrics`."]
    pub const fn input_queue_peak_frames(&self) -> u64 {
        self.input_edge.queue_peak_frames
    }

    #[doc = "Returns the input attempted total associated with `SessionOperatorMetrics`."]
    pub fn input_attempted_total(&self) -> u64 {
        self.input_edge.frames_attempted_total()
    }

    #[doc = "Returns the input enqueued total associated with `SessionOperatorMetrics`."]
    pub const fn input_enqueued_total(&self) -> u64 {
        self.input_edge.frames_enqueued_total
    }

    #[doc = "Returns the input delivered total associated with `SessionOperatorMetrics`."]
    pub const fn input_delivered_total(&self) -> u64 {
        self.input_edge.frames_delivered_total
    }

    #[doc = "Returns the input dropped total associated with `SessionOperatorMetrics`."]
    pub const fn input_dropped_total(&self) -> u64 {
        self.input_edge.frames_dropped_total
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[doc = "Reports the session derived route metrics collected at an observation boundary."]
pub struct SessionDerivedRouteMetrics {
    #[doc = "Identifies the route associated with `SessionDerivedRouteMetrics`."]
    pub route_id: RouteId,
    #[doc = "Identifies the endpoint associated with `SessionDerivedRouteMetrics`."]
    pub endpoint_id: EndpointId,
    #[doc = "Carries the output produced by `SessionDerivedRouteMetrics`."]
    pub output: AsyncOperatorOutputObservations,
    #[doc = "Stores the endpoint associated with `SessionDerivedRouteMetrics`."]
    pub endpoint: Option<EndpointDriverObservations>,
    #[doc = "Stores the endpoint observation stage associated with `SessionDerivedRouteMetrics`."]
    pub endpoint_observation_stage: EndpointObservationStage,
    #[doc = "Counts the total number of endpoint finalization failures observed by `SessionDerivedRouteMetrics`."]
    pub endpoint_finalization_failures_total: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[doc = "Selects the endpoint observation stage used by PocketStation."]
pub enum EndpointObservationStage {
    #[doc = "Reports that the requested resource is unavailable."]
    Unavailable,
    #[doc = "Represents the live case of `EndpointObservationStage`."]
    Live,
    #[doc = "Represents the finalized case of `EndpointObservationStage`."]
    Finalized,
}

#[derive(Debug)]
pub(crate) struct SessionEventQueueCounters {
    capacity_event_count: u64,
    maximum_event_owned_bytes: u64,
    maximum_buffered_owned_bytes: u64,
    depth_events: AtomicU64,
    depth_owned_bytes: AtomicU64,
    peak_depth_event_count: AtomicU64,
    peak_depth_owned_bytes: AtomicU64,
    events_enqueued_total: AtomicU64,
    events_dropped_total: AtomicU64,
    events_dropped_oversized_total: AtomicU64,
    receiver_closed_total: AtomicU64,
}

pub(crate) enum SessionEventReservation {
    Reserved,
    Full,
    Oversized,
}

impl SessionEventQueueCounters {
    pub(crate) fn new(capacity_events: usize, maximum_event_owned_bytes: usize) -> Self {
        Self {
            capacity_event_count: capacity_events as u64,
            maximum_event_owned_bytes: maximum_event_owned_bytes as u64,
            maximum_buffered_owned_bytes: capacity_events.saturating_mul(maximum_event_owned_bytes)
                as u64,
            depth_events: AtomicU64::new(0),
            depth_owned_bytes: AtomicU64::new(0),
            peak_depth_event_count: AtomicU64::new(0),
            peak_depth_owned_bytes: AtomicU64::new(0),
            events_enqueued_total: AtomicU64::new(0),
            events_dropped_total: AtomicU64::new(0),
            events_dropped_oversized_total: AtomicU64::new(0),
            receiver_closed_total: AtomicU64::new(0),
        }
    }

    pub(crate) fn reserve_event(&self, owned_bytes: usize) -> SessionEventReservation {
        if owned_bytes as u64 > self.maximum_event_owned_bytes {
            self.events_dropped_total.fetch_add(1, Ordering::Relaxed);
            self.events_dropped_oversized_total
                .fetch_add(1, Ordering::Relaxed);
            return SessionEventReservation::Oversized;
        }
        let mut depth_events = self.depth_events.load(Ordering::Relaxed);
        loop {
            if depth_events >= self.capacity_event_count {
                self.events_dropped_total.fetch_add(1, Ordering::Relaxed);
                return SessionEventReservation::Full;
            }

            match self.depth_events.compare_exchange_weak(
                depth_events,
                depth_events + 1,
                Ordering::AcqRel,
                Ordering::Relaxed,
            ) {
                Ok(_) => {
                    self.peak_depth_event_count
                        .fetch_max(depth_events + 1, Ordering::Relaxed);
                    let depth_owned_bytes = self
                        .depth_owned_bytes
                        .fetch_add(owned_bytes as u64, Ordering::AcqRel)
                        .saturating_add(owned_bytes as u64);
                    self.peak_depth_owned_bytes
                        .fetch_max(depth_owned_bytes, Ordering::Relaxed);
                    return SessionEventReservation::Reserved;
                }
                Err(observed_depth_events) => depth_events = observed_depth_events,
            }
        }
    }

    pub(crate) fn observe_enqueued(&self) {
        self.events_enqueued_total.fetch_add(1, Ordering::Relaxed);
    }

    pub(crate) fn observe_send_full(&self, owned_bytes: usize) {
        self.cancel_reservation(owned_bytes);
        self.events_dropped_total.fetch_add(1, Ordering::Relaxed);
    }

    pub(crate) fn observe_receiver_closed(&self, owned_bytes: usize) {
        self.cancel_reservation(owned_bytes);
        self.events_dropped_total.fetch_add(1, Ordering::Relaxed);
        self.receiver_closed_total.fetch_add(1, Ordering::Relaxed);
    }

    pub(crate) fn observe_dequeued(&self, owned_bytes: usize) {
        let previous_depth_events = self.depth_events.fetch_sub(1, Ordering::AcqRel);
        debug_assert!(previous_depth_events > 0);
        let previous_depth_owned_bytes = self
            .depth_owned_bytes
            .fetch_sub(owned_bytes as u64, Ordering::AcqRel);
        debug_assert!(previous_depth_owned_bytes >= owned_bytes as u64);
    }

    pub(crate) fn snapshot(&self) -> SessionEventQueueObservations {
        SessionEventQueueObservations {
            capacity_event_count: self.capacity_event_count,
            maximum_event_owned_bytes: self.maximum_event_owned_bytes,
            maximum_buffered_owned_bytes: self.maximum_buffered_owned_bytes,
            depth_events: self.depth_events.load(Ordering::Acquire),
            depth_owned_bytes: self.depth_owned_bytes.load(Ordering::Acquire),
            peak_depth_event_count: self.peak_depth_event_count.load(Ordering::Relaxed),
            peak_depth_owned_bytes: self.peak_depth_owned_bytes.load(Ordering::Relaxed),
            events_enqueued_total: self.events_enqueued_total.load(Ordering::Relaxed),
            events_dropped_total: self.events_dropped_total.load(Ordering::Relaxed),
            events_dropped_oversized_total: self
                .events_dropped_oversized_total
                .load(Ordering::Relaxed),
            receiver_closed_total: self.receiver_closed_total.load(Ordering::Relaxed),
        }
    }

    fn cancel_reservation(&self, owned_bytes: usize) {
        let previous_depth_events = self.depth_events.fetch_sub(1, Ordering::AcqRel);
        debug_assert!(previous_depth_events > 0);
        let previous_depth_owned_bytes = self
            .depth_owned_bytes
            .fetch_sub(owned_bytes as u64, Ordering::AcqRel);
        debug_assert!(previous_depth_owned_bytes >= owned_bytes as u64);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_route_snapshot_when_drop_observed_then_rate_has_explicit_denominator_and_reasons() {
        let route = SessionRouteMetrics {
            route_id: RouteId(7),
            endpoint_id: EndpointId(8),
            edge: EdgeObservations {
                frames_enqueued_total: 3,
                frames_dropped_total: 1,
                queue_full_drops_total: 1,
                ..EdgeObservations::default()
            },
            endpoint: None,
            endpoint_observation_stage: EndpointObservationStage::Unavailable,
            endpoint_finalization_failures_total: 0,
        };

        let drops = route.drop_observations();
        assert_eq!(drops.route_id, RouteId(7));
        assert_eq!(drops.frames_dropped_total, 1);
        assert_eq!(drops.frames_attempted_total, 4);
        assert_eq!(drops.queue_full_drops_total, 1);
        assert_eq!(
            drops.interval,
            SessionRouteObservationInterval::RouteLifetimeToSnapshot
        );
        assert!((drops.drop_rate_pct() - 25.0).abs() < f64::EPSILON);
    }

    #[test]
    fn given_route_snapshot_when_latency_observed_then_boundary_units_and_coverage_are_explicit() {
        let route = SessionRouteMetrics {
            route_id: RouteId(7),
            endpoint_id: EndpointId(8),
            edge: EdgeObservations {
                source_timestamp_to_receive_samples_total: 9,
                source_timestamp_to_receive_missing_total: 2,
                source_timestamp_to_receive_future_total: 1,
                source_timestamp_to_receive_p95_ns: 42,
                ..EdgeObservations::default()
            },
            endpoint: None,
            endpoint_observation_stage: EndpointObservationStage::Unavailable,
            endpoint_finalization_failures_total: 0,
        };

        let latency = route.source_to_receive_latency();
        assert_eq!(latency.route_id, RouteId(7));
        assert_eq!(
            latency.boundary,
            SessionRouteLatencyBoundary::SourceMonotonicTimestampToRouteReceive
        );
        assert_eq!(latency.unit, SessionRouteLatencyUnit::Nanoseconds);
        assert_eq!(latency.samples_total, 9);
        assert_eq!(latency.missing_or_incompatible_clock_total, 2);
        assert_eq!(latency.future_timestamp_total, 1);
        assert_eq!(latency.p95_ns, 42);
    }
}
