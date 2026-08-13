use crate::frame::{EndpointId, RouteId, SessionId, StemId};
use crate::graph::NodeConfig;

use super::{EndpointPrepareContext, EndpointRouteContext, SessionTimelineOrigin};

#[test]
fn given_endpoint_context_when_constructed_then_route_and_timeline_are_required() {
    let context = EndpointPrepareContext::new(
        SessionId(1),
        EndpointId(2),
        EndpointRouteContext::from_stem(RouteId(4), StemId(3)),
        SessionTimelineOrigin::from_monotonic_timestamp_ns(5),
        NodeConfig::new(),
    );

    assert_eq!(context.route_context().audio_stem_id(), Some(StemId(3)));
    assert_eq!(context.route_context().route_id(), RouteId(4));
    assert_eq!(
        context.session_timeline_origin().monotonic_timestamp_ns(),
        5
    );
}

#[test]
fn given_source_route_when_constructed_then_source_origin_is_not_a_synthetic_stem() {
    let context = EndpointPrepareContext::new(
        SessionId(1),
        EndpointId(2),
        EndpointRouteContext::from_source(
            RouteId(4),
            crate::frame::SourceId(6),
            crate::frame::StreamId(7),
            None,
        ),
        SessionTimelineOrigin::from_monotonic_timestamp_ns(5),
        NodeConfig::new(),
    );

    let route_context = context.route_context();
    assert_eq!(route_context.audio_stem_id(), None);
    assert_eq!(route_context.route_id(), RouteId(4));
}
