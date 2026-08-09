use std::sync::atomic::{AtomicU64, Ordering};

use crate::capture::CaptureOwnerObservations;
use crate::endpoint::EndpointDriverObservations;
use crate::frame::{EndpointId, RouteId, SourceId, StemId};
use crate::runtime::{
    AsyncOperatorObservations, AsyncOperatorOutputObservations, EdgeObservations,
    PlanSourceInputObservations,
};

use crate::session::{
    OperatorInstanceId, PolledAudioObservations, SourceInstanceId, SourceRuntimeObservations,
};

/// Point-in-time observations for a session's bounded control-event queue.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SessionEventQueueObservations {
    pub capacity_event_count: u64,
    pub depth_events: u64,
    pub peak_depth_event_count: u64,
    pub events_enqueued_total: u64,
    pub events_dropped_total: u64,
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

    pub const fn event_queue(&self) -> SessionEventQueueObservations {
        self.event_queue
    }

    pub const fn polled_audio(&self) -> PolledAudioObservations {
        self.polled_audio
    }

    pub fn source_count(&self) -> usize {
        self.sources.len()
    }

    pub fn source(&self, index: usize) -> Option<&SessionSourceMetrics> {
        self.sources.get(index)
    }

    pub fn external_source_count(&self) -> usize {
        self.external_sources.len()
    }

    pub fn external_source(&self, index: usize) -> Option<&SessionExternalSourceMetrics> {
        self.external_sources.get(index)
    }

    pub fn route_count(&self) -> usize {
        self.routes.len()
    }

    pub fn route(&self, index: usize) -> Option<&SessionRouteMetrics> {
        self.routes.get(index)
    }

    pub fn operator_count(&self) -> usize {
        self.operators.len()
    }

    pub fn operator(&self, index: usize) -> Option<&SessionOperatorMetrics> {
        self.operators.get(index)
    }

    pub fn derived_route_count(&self) -> usize {
        self.derived_routes.len()
    }

    pub fn derived_route(&self, index: usize) -> Option<&SessionDerivedRouteMetrics> {
        self.derived_routes.get(index)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SessionSourceMetrics {
    pub stem_id: StemId,
    pub capture: CaptureOwnerObservations,
    pub ingress: PlanSourceInputObservations,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SessionExternalSourceMetrics {
    pub source_instance_id: SourceInstanceId,
    pub source_id: SourceId,
    pub runtime: SourceRuntimeObservations,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SessionRouteMetrics {
    pub route_id: RouteId,
    pub endpoint_id: EndpointId,
    pub edge: EdgeObservations,
    pub endpoint: Option<EndpointDriverObservations>,
    pub endpoint_observation_stage: EndpointObservationStage,
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
    pub route_id: RouteId,
    pub interval: SessionRouteObservationInterval,
    pub frames_dropped_total: u64,
    pub frames_attempted_total: u64,
    pub receiver_unavailable_drops_total: u64,
    pub queue_full_drops_total: u64,
    pub shared_reference_exhausted_drops_total: u64,
    pub branch_pool_exhausted_drops_total: u64,
    pub invalid_copy_policy_drops_total: u64,
    pub freeze_failed_drops_total: u64,
}

impl SessionRouteDropObservations {
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
    pub route_id: RouteId,
    pub boundary: SessionRouteLatencyBoundary,
    pub unit: SessionRouteLatencyUnit,
    pub samples_total: u64,
    pub missing_or_incompatible_clock_total: u64,
    pub future_timestamp_total: u64,
    pub p50_ns: u64,
    pub p95_ns: u64,
    pub p99_ns: u64,
    pub max_ns: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SessionRouteLatencyBoundary {
    SourceMonotonicTimestampToRouteReceive,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SessionRouteLatencyUnit {
    Nanoseconds,
}

impl SessionRouteMetrics {
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SessionOperatorMetrics {
    pub operator_instance_id: OperatorInstanceId,
    /// Sole counter authority for input delivered by the compiled Session plan.
    ///
    /// `worker.input_*` remains meaningful only for workers fed through the
    /// direct `AsyncOperatorInput` API. Compiled Session operators consume this
    /// plan edge, so callers must use these observations for input accounting.
    pub input_edge: EdgeObservations,
    pub worker: AsyncOperatorObservations,
    pub finalization_failures_total: u64,
}

impl SessionOperatorMetrics {
    pub const fn input_queue_capacity_frames(&self) -> u64 {
        self.input_edge.queue_capacity_frames
    }

    pub const fn input_queue_depth_frames(&self) -> u64 {
        self.input_edge.queue_depth_frames
    }

    pub const fn input_queue_peak_frames(&self) -> u64 {
        self.input_edge.queue_peak_frames
    }

    pub fn input_attempted_total(&self) -> u64 {
        self.input_edge.frames_attempted_total()
    }

    pub const fn input_enqueued_total(&self) -> u64 {
        self.input_edge.frames_enqueued_total
    }

    pub const fn input_delivered_total(&self) -> u64 {
        self.input_edge.frames_delivered_total
    }

    pub const fn input_dropped_total(&self) -> u64 {
        self.input_edge.frames_dropped_total
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SessionDerivedRouteMetrics {
    pub route_id: RouteId,
    pub endpoint_id: EndpointId,
    pub output: AsyncOperatorOutputObservations,
    pub endpoint: Option<EndpointDriverObservations>,
    pub endpoint_observation_stage: EndpointObservationStage,
    pub endpoint_finalization_failures_total: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EndpointObservationStage {
    Unavailable,
    Live,
    Finalized,
}

#[derive(Debug)]
pub(crate) struct SessionEventQueueCounters {
    capacity_event_count: u64,
    depth_events: AtomicU64,
    peak_depth_event_count: AtomicU64,
    events_enqueued_total: AtomicU64,
    events_dropped_total: AtomicU64,
    receiver_closed_total: AtomicU64,
}

impl SessionEventQueueCounters {
    pub(crate) fn new(capacity_events: usize) -> Self {
        Self {
            capacity_event_count: capacity_events as u64,
            depth_events: AtomicU64::new(0),
            peak_depth_event_count: AtomicU64::new(0),
            events_enqueued_total: AtomicU64::new(0),
            events_dropped_total: AtomicU64::new(0),
            receiver_closed_total: AtomicU64::new(0),
        }
    }

    pub(crate) fn reserve_event(&self) -> bool {
        let mut depth_events = self.depth_events.load(Ordering::Relaxed);
        loop {
            if depth_events >= self.capacity_event_count {
                self.events_dropped_total.fetch_add(1, Ordering::Relaxed);
                return false;
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
                    return true;
                }
                Err(observed_depth_events) => depth_events = observed_depth_events,
            }
        }
    }

    pub(crate) fn observe_enqueued(&self) {
        self.events_enqueued_total.fetch_add(1, Ordering::Relaxed);
    }

    pub(crate) fn observe_send_full(&self) {
        self.cancel_reservation();
        self.events_dropped_total.fetch_add(1, Ordering::Relaxed);
    }

    pub(crate) fn observe_receiver_closed(&self) {
        self.cancel_reservation();
        self.events_dropped_total.fetch_add(1, Ordering::Relaxed);
        self.receiver_closed_total.fetch_add(1, Ordering::Relaxed);
    }

    pub(crate) fn observe_dequeued(&self) {
        let previous_depth_events = self.depth_events.fetch_sub(1, Ordering::AcqRel);
        debug_assert!(previous_depth_events > 0);
    }

    pub(crate) fn snapshot(&self) -> SessionEventQueueObservations {
        SessionEventQueueObservations {
            capacity_event_count: self.capacity_event_count,
            depth_events: self.depth_events.load(Ordering::Acquire),
            peak_depth_event_count: self.peak_depth_event_count.load(Ordering::Relaxed),
            events_enqueued_total: self.events_enqueued_total.load(Ordering::Relaxed),
            events_dropped_total: self.events_dropped_total.load(Ordering::Relaxed),
            receiver_closed_total: self.receiver_closed_total.load(Ordering::Relaxed),
        }
    }

    fn cancel_reservation(&self) {
        let previous_depth_events = self.depth_events.fetch_sub(1, Ordering::AcqRel);
        debug_assert!(previous_depth_events > 0);
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
