use pocketstation::{
    BackpressurePolicy, DeliveryPolicy, EndpointDescriptor, NodeTypeId, OperatorId, RouteSettings,
    SignalClass,
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
fn given_root_signal_class_when_used_then_it_remains_public() {
    assert!(SignalClass::PcmAudio.is_audio());
}
