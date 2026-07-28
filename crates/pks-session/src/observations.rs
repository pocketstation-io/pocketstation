use std::sync::atomic::{AtomicU64, Ordering};

use pks_capture::CaptureOwnerObservations;
use pks_endpoint::EndpointDriverObservations;
use pks_frame::{EndpointId, RouteId, StemId};
use pks_runtime::{EdgeObservations, PlanSourceInputObservations};

use crate::PolledAudioObservations;

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
    routes: Box<[SessionRouteMetrics]>,
}

impl SessionMetricsSnapshot {
    pub(crate) const fn new(
        event_queue: SessionEventQueueObservations,
        polled_audio: PolledAudioObservations,
        sources: Box<[SessionSourceMetrics]>,
        routes: Box<[SessionRouteMetrics]>,
    ) -> Self {
        Self {
            event_queue,
            polled_audio,
            sources,
            routes,
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

    pub fn route_count(&self) -> usize {
        self.routes.len()
    }

    pub fn route(&self, index: usize) -> Option<&SessionRouteMetrics> {
        self.routes.get(index)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SessionSourceMetrics {
    pub stem_id: StemId,
    pub capture: CaptureOwnerObservations,
    pub ingress: PlanSourceInputObservations,
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
