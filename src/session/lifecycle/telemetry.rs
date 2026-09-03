//! Runtime observation bindings and finalized Session-level snapshots.

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
    SessionOperatorMetrics, SessionRouteMetrics, SessionSourceMetrics,
};

pub(super) struct SourceObservationBinding {
    pub(super) stem_id: StemId,
    pub(super) capture: CaptureObservationReceipt,
    pub(super) ingress: PlanSourceObservationHandle,
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
