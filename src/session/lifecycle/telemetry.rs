//! Runtime observation bindings and finalized Session-level snapshots.

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

use crate::capture::CaptureObservationReceipt;
use crate::endpoint::EndpointDriverObservations;
use crate::frame::{EndpointId, RouteId, StemId};
use crate::runtime::{
    AsyncOperatorObservations, AsyncOperatorOutputObservationHandle, EdgeObservations,
    PlanEdgeObservationHandle, PlanSourceObservationHandle,
};
use crate::session::declaration::OperatorInstanceId;

use super::observations::{
    SessionDerivedRouteMetrics, SessionExternalSourceMetrics, SessionOperatorInputMetrics,
    SessionOperatorMetrics, SessionRouteMetrics, SessionSourceActivityObservations,
    SessionSourceMetrics,
};

#[derive(Default)]
struct SourceActivityObservationState {
    session_started_at_ns: u64,
    first_frame_received_at_ns: AtomicU64,
    latest_frame_received_at_ns: AtomicU64,
    frames_received_total: AtomicU64,
}

#[derive(Clone)]
pub(super) struct SourceActivityObservationHandle {
    state: Arc<SourceActivityObservationState>,
}

impl SourceActivityObservationHandle {
    pub(super) fn new(session_started_at_ns: u64) -> Self {
        Self {
            state: Arc::new(SourceActivityObservationState {
                session_started_at_ns,
                ..SourceActivityObservationState::default()
            }),
        }
    }

    /// Records activity on the Session runtime worker after capture dequeue.
    /// The runtime has one writer per built-in Source.
    pub(super) fn observe_frame(&self, received_at_ns: u64) {
        if self.state.frames_received_total.load(Ordering::Relaxed) == 0 {
            self.state
                .first_frame_received_at_ns
                .store(received_at_ns, Ordering::Relaxed);
        }
        self.state
            .latest_frame_received_at_ns
            .store(received_at_ns, Ordering::Relaxed);
        let _ = self.state.frames_received_total.fetch_update(
            Ordering::Release,
            Ordering::Relaxed,
            |current| Some(current.saturating_add(1)),
        );
    }

    pub(super) fn observations(&self, observed_at_ns: u64) -> SessionSourceActivityObservations {
        let frames_received_total = self.state.frames_received_total.load(Ordering::Acquire);
        SessionSourceActivityObservations {
            session_started_at_ns: self.state.session_started_at_ns,
            observed_at_ns,
            first_frame_received_at_ns: (frames_received_total > 0).then(|| {
                self.state
                    .first_frame_received_at_ns
                    .load(Ordering::Relaxed)
            }),
            latest_frame_received_at_ns: (frames_received_total > 0).then(|| {
                self.state
                    .latest_frame_received_at_ns
                    .load(Ordering::Relaxed)
            }),
            frames_received_total,
        }
    }
}

pub(super) struct SourceObservationBinding {
    pub(super) stem_id: StemId,
    pub(super) capture: CaptureObservationReceipt,
    pub(super) ingress: PlanSourceObservationHandle,
    pub(super) activity: SourceActivityObservationHandle,
}

pub(super) struct RouteObservationBinding {
    pub(super) route_id: RouteId,
    pub(super) endpoint_id: EndpointId,
    pub(super) edge: PlanEdgeObservationHandle,
}

pub(super) struct DerivedRouteObservationBinding {
    pub(super) route_id: RouteId,
    pub(super) endpoint_id: EndpointId,
    pub(super) output: AsyncOperatorOutputObservationHandle,
}

pub(super) type IndexedSessionMetrics = (
    Box<[SessionSourceMetrics]>,
    Box<[SessionExternalSourceMetrics]>,
    Box<[SessionRouteMetrics]>,
    Box<[SessionOperatorMetrics]>,
    Box<[SessionDerivedRouteMetrics]>,
);

#[derive(Clone, Copy)]
pub(super) struct FinalEndpointObservation {
    pub(super) route_id: RouteId,
    pub(super) endpoint_id: EndpointId,
    pub(super) observations: EndpointDriverObservations,
    pub(super) finalization_failures_total: u64,
}

#[derive(Clone)]
pub(super) struct FinalOperatorObservation {
    pub(super) operator_instance_id: OperatorInstanceId,
    pub(super) input_delivery: EdgeObservations,
    pub(super) input_ports: Box<[SessionOperatorInputMetrics]>,
    pub(super) observations: AsyncOperatorObservations,
    pub(super) finalization_failures_total: u64,
}
