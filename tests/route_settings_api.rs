use pocketstation::{
    BackpressurePolicy, DeliveryPolicy, EdgeContract, EdgeObservabilityLevel, EndpointDescriptor,
    ExecutionSafety, NodeTypeId, OperatorId, RouteLatencyMeasurement, RouteObservability,
    RouteSettings, SafetyContract, SessionRouteLatencyBoundary, SignalClass,
};

#[test]
fn given_delivery_policy_when_applied_then_route_media_and_delivery_remain_separate() {
    let delivery = DeliveryPolicy::realtime_audio()
        .with_backpressure(BackpressurePolicy::DropOldest)
        .with_jitter_budget_ms(Some(25));
    let settings = RouteSettings::realtime_audio().with_delivery_policy(delivery);

    assert_eq!(settings.delivery_policy(), delivery);
    assert_eq!(settings.backpressure(), BackpressurePolicy::DropOldest);
    assert_eq!(settings.jitter_budget_ms(), Some(25));
}

#[test]
fn given_endpoint_descriptor_when_route_settings_selected_then_they_are_readable() {
    let settings = RouteSettings::bounded_async();
    let descriptor = EndpointDescriptor::new(
        NodeTypeId::from("io.pocketstation.test.endpoint.v1"),
        OperatorId::new("io.pocketstation.test.operator.v1"),
    )
    .with_route_settings(settings);

    assert_eq!(descriptor.route_settings(), Some(settings));
}

#[test]
fn given_compatibility_names_when_used_then_they_resolve_to_the_clearer_types() {
    let old_route_name = EdgeContract::realtime_audio();
    let route_settings: RouteSettings = old_route_name;
    let old_safety_name = SafetyContract::NetworkAllowed;
    let execution_safety: ExecutionSafety = old_safety_name;
    let old_measurement_name = SessionRouteLatencyBoundary::SourceMonotonicTimestampToRouteReceive;
    let measurement: RouteLatencyMeasurement = old_measurement_name;
    let old_observability_name = EdgeObservabilityLevel::Counters;
    let route_observability: RouteObservability = old_observability_name;

    assert_eq!(route_settings, RouteSettings::realtime_audio());
    assert_eq!(execution_safety, ExecutionSafety::NetworkAllowed);
    assert_eq!(
        measurement,
        RouteLatencyMeasurement::SourceMonotonicTimestampToRouteReceive
    );
    assert_eq!(route_observability, RouteObservability::Counters);
    assert!(SignalClass::PcmAudio.is_audio());
}
