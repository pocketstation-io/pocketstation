use super::*;
use crate::frame::Platform;
use std::num::NonZeroU32;

fn runtime_backend_failure(status_code: i32) -> SourceRuntimeEvent {
    SourceRuntimeEvent::BackendFailure {
        stable_id: StableSourceId::new(
            Platform::Windows,
            SourceKind::Application,
            "wasapi:pid:4242:creation-100ns:100",
        ),
        generation: SourceGeneration::INITIAL,
        failure: CaptureRuntimeFailure {
            operation: "read WASAPI packet",
            error_class: CaptureRuntimeFailureClass::PlatformStatus { status_code },
        },
    }
}
#[test]
fn given_runtime_event_when_sent_then_exact_identity_and_platform_status_are_retained() {
    let (sender, receiver) = source_runtime_event_channel(1).expect("valid capacity");
    let event = runtime_backend_failure(-2_004_284_412);
    let event_owned_bytes = event.owned_bytes() as u64;
    assert_eq!(
        sender.try_send(event.clone()),
        SourceRuntimeEventDelivery::Enqueued
    );
    assert_eq!(receiver.try_recv(), SourceRuntimeEventReceive::Event(event));
    assert_eq!(
        receiver.observations(),
        SourceRuntimeEventObservations {
            capacity_event_count: 1,
            maximum_event_owned_bytes: MAX_SOURCE_RUNTIME_EVENT_OWNED_BYTES as u64,
            maximum_buffered_owned_bytes: MAX_SOURCE_RUNTIME_EVENT_OWNED_BYTES as u64,
            depth_events: 0,
            depth_owned_bytes: 0,
            peak_depth_owned_bytes: event_owned_bytes,
            events_enqueued_total: 1,
            events_dropped_total: 0,
            events_dropped_oversized_total: 0,
        }
    );
}
#[test]
fn given_full_runtime_event_channel_when_published_then_newest_event_is_dropped_and_counted() {
    let (sender, _receiver) = source_runtime_event_channel(1).expect("valid capacity");
    let observations = sender.observation_handle();
    assert_eq!(
        sender.try_send(runtime_backend_failure(1)),
        SourceRuntimeEventDelivery::Enqueued
    );
    assert_eq!(
        sender.try_send(runtime_backend_failure(2)),
        SourceRuntimeEventDelivery::DroppedFull
    );
    assert_eq!(
        observations.observations(),
        SourceRuntimeEventObservations {
            capacity_event_count: 1,
            maximum_event_owned_bytes: MAX_SOURCE_RUNTIME_EVENT_OWNED_BYTES as u64,
            maximum_buffered_owned_bytes: MAX_SOURCE_RUNTIME_EVENT_OWNED_BYTES as u64,
            depth_events: 1,
            depth_owned_bytes: runtime_backend_failure(1).owned_bytes() as u64,
            peak_depth_owned_bytes: (runtime_backend_failure(1).owned_bytes() * 2) as u64,
            events_enqueued_total: 1,
            events_dropped_total: 1,
            events_dropped_oversized_total: 0,
        }
    );
}
#[test]
fn given_oversized_runtime_event_when_published_then_owned_memory_is_rejected_before_enqueue() {
    let (sender, receiver) = source_runtime_event_channel(2).expect("valid capacity");
    let oversized = SourceRuntimeEvent::BackendFailure {
        stable_id: StableSourceId::new(
            Platform::Linux,
            SourceKind::Application,
            "x".repeat(MAX_SOURCE_RUNTIME_EVENT_OWNED_BYTES),
        ),
        generation: SourceGeneration::INITIAL,
        failure: CaptureRuntimeFailure {
            operation: "test oversized event",
            error_class: CaptureRuntimeFailureClass::SourceInstanceExited,
        },
    };
    assert_eq!(
        sender.try_send(oversized),
        SourceRuntimeEventDelivery::DroppedOversized
    );
    assert_eq!(receiver.try_recv(), SourceRuntimeEventReceive::Empty);
    let observations = receiver.observations();
    assert_eq!(observations.depth_events, 0);
    assert_eq!(observations.depth_owned_bytes, 0);
    assert_eq!(observations.events_dropped_total, 1);
    assert_eq!(observations.events_dropped_oversized_total, 1);
}
#[test]
fn given_dropped_runtime_event_receiver_when_published_then_closed_is_reported() {
    let (sender, receiver) = source_runtime_event_channel(1).expect("valid capacity");
    drop(receiver);
    assert_eq!(
        sender.try_send(runtime_backend_failure(1)),
        SourceRuntimeEventDelivery::ReceiverClosed
    );
}
#[test]
fn given_backend_failure_publisher_when_owner_ends_then_event_and_closure_are_observable() {
    let (sender, receiver) = source_runtime_event_channel(1).unwrap();
    let stable_id = StableSourceId::new(Platform::Linux, SourceKind::Application, "pw-node:test");
    assert_eq!(
        publish_backend_failure(
            &sender,
            stable_id.clone(),
            SourceGeneration::INITIAL,
            "test native worker",
            CaptureRuntimeFailureClass::BackendClass {
                class: "worker-exited".to_owned(),
            },
        ),
        SourceRuntimeEventDelivery::Enqueued
    );
    assert!(matches!(
        receiver.try_recv(),
        SourceRuntimeEventReceive::Event(SourceRuntimeEvent::BackendFailure {
            stable_id: event_stable_id,
            ..
        }) if event_stable_id == stable_id
    ));
    drop(sender);
    assert_eq!(receiver.try_recv(), SourceRuntimeEventReceive::Closed);
}
#[test]
fn given_zero_runtime_event_capacity_when_created_then_invalid_capacity_is_reported() {
    assert_eq!(
        source_runtime_event_channel(0).expect_err("zero capacity must fail"),
        CaptureError::InvalidRuntimeEventCapacity
    );
}
#[test]
fn given_capture_events_when_observed_then_snapshot_preserves_each_boundary() {
    let counters = CaptureObservationCounters::default();
    let observations = counters.observation_handle();
    counters.observe_callback_buffer();
    counters.observe_enqueued_frame();
    counters.observe_pool_exhaustion();
    counters.observe_dispatch_queue_full();
    counters.observe_dispatch_queue_full_frames(3);
    counters.observe_invalid_buffer();
    counters.observe_oversized_buffer();
    counters.observe_stream_error();
    counters.observe_timestamp_epoch_clamp();
    assert_eq!(
        observations.observations(),
        CaptureObservations {
            callback_buffers_total: 1,
            frames_enqueued_total: 1,
            pool_exhausted_total: 1,
            dispatch_queue_full_total: 4,
            invalid_buffer_total: 1,
            oversized_buffer_total: 1,
            stream_errors_total: 1,
            timestamp_epoch_clamps_total: 1,
        }
    );
}
#[test]
fn given_stable_source_id_when_derived_twice_then_same_source_id() {
    let id = StableSourceId::new(
        Platform::Macos,
        SourceKind::Application,
        "com.spotify.client",
    );
    assert_eq!(id.source_id(), id.source_id());
}
#[test]
fn given_canonical_capture_identity_when_derived_then_source_id_matches_stable_vector() {
    let identity =
        StableSourceId::new(Platform::Macos, SourceKind::Application, "com.acme.meeting");
    assert_eq!(
        identity.source_id(),
        crate::frame::SourceId(11_340_425_655_975_972_100)
    );
}
#[test]
fn given_two_different_stable_ids_when_derived_then_different_source_ids() {
    let a = StableSourceId::new(
        Platform::Macos,
        SourceKind::Application,
        "com.spotify.client",
    );
    let b = StableSourceId::new(Platform::Macos, SourceKind::Application, "com.apple.music");
    assert_ne!(a.source_id(), b.source_id());
}
#[test]
fn given_capture_error_not_supported_when_displayed_then_contains_not_supported() {
    let msg = CaptureError::NotSupported.to_string();
    assert!(msg.contains("not supported"), "got: {msg}");
}
#[test]
fn given_default_capture_mode_when_compared_then_is_system_mix() {
    assert_eq!(CaptureMode::default(), CaptureMode::SystemMix);
}
#[test]
fn given_stable_input_selector_when_wrapped_then_capture_mode_preserves_identity() {
    let selector = InputDeviceSelector::StableId("coreaudio:device-7".to_owned());
    assert_eq!(
        CaptureMode::InputDevice(selector.clone()),
        CaptureMode::InputDevice(selector)
    );
}
#[test]
fn given_exact_windows_process_when_contract_inspected_then_scope_is_process_tree() {
    let mode = CaptureMode::ExactApplication {
        process_id: 42,
        stable_id: StableSourceId::new(Platform::Windows, SourceKind::Application, "process:42"),
    };
    assert_eq!(
        mode.selector_persistence_scope(),
        SelectorPersistenceScope::ProcessLifetime
    );
    assert_eq!(
        mode.process_tree_scope(Platform::Windows),
        ProcessTreeScope::SelectedProcessAndDescendants
    );
}
#[test]
fn given_exact_macos_process_when_contract_inspected_then_scope_is_process_only() {
    let mode = CaptureMode::ExactApplication {
        process_id: 42,
        stable_id: StableSourceId::new(
            Platform::Macos,
            SourceKind::Application,
            "com.acme.meeting",
        ),
    };
    assert_eq!(
        mode.process_tree_scope(Platform::Macos),
        ProcessTreeScope::SelectedProcessOnly
    );
}
#[test]
fn given_default_and_exact_microphones_when_contract_inspected_then_lifetimes_differ() {
    assert_eq!(
        CaptureMode::InputDevice(InputDeviceSelector::Default).selector_persistence_scope(),
        SelectorPersistenceScope::SessionDefaultDevice
    );
    assert_eq!(
        CaptureMode::InputDevice(InputDeviceSelector::StableId("device-7".to_owned()))
            .selector_persistence_scope(),
        SelectorPersistenceScope::DeviceIdentity
    );
}
#[test]
fn given_source_generation_when_rediscovered_then_generation_advances() {
    assert_eq!(SourceGeneration::INITIAL.next(), SourceGeneration(2));
}
#[test]
fn given_source_unavailable_error_when_displayed_then_stable_identity_is_retained() {
    let error = CaptureError::SourceUnavailable {
        stable_key: "device-7".to_owned(),
    };
    assert!(error.to_string().contains("device-7"));
}
#[test]
fn given_mode_unsupported_error_when_displayed_then_contains_not_supported() {
    let err = CaptureError::ModeUnsupported(CaptureMode::Process(1234));
    assert!(err.to_string().contains("not supported"));
}
#[test]
fn given_monotonic_capture_clock_when_sampled_then_never_moves_backwards() {
    let first = monotonic_timestamp_ns();
    let second = monotonic_timestamp_ns();
    assert!(first > 0);
    assert!(second >= first);
}
#[test]
fn given_capture_sample_timeline_when_callback_arrival_varies_then_media_cadence_is_exact() {
    let mut timeline = CaptureSampleTimeline::new(NonZeroU32::new(48_000).unwrap());
    let first_timestamp_ns = timeline.advance(960);
    std::thread::sleep(std::time::Duration::from_millis(1));
    let second_timestamp_ns = timeline.advance(480);
    let third_timestamp_ns = timeline.advance(480);
    assert_eq!(second_timestamp_ns - first_timestamp_ns, 20_000_000);
    assert_eq!(third_timestamp_ns - second_timestamp_ns, 10_000_000);
}
#[test]
fn given_small_capture_chunks_when_one_second_elapses_then_rounding_does_not_drift() {
    let mut timeline = CaptureSampleTimeline::new(NonZeroU32::new(48_000).unwrap());
    let origin_timestamp_ns = timeline.advance(128);
    for _ in 1..375 {
        timeline.advance(128);
    }
    let one_second_timestamp_ns = timeline.advance(128);
    assert_eq!(one_second_timestamp_ns - origin_timestamp_ns, 1_000_000_000);
}
#[test]
fn given_native_source_gap_when_advanced_then_gap_is_preserved_once() {
    let mut timeline = CaptureSampleTimeline::new(NonZeroU32::new(48_000).unwrap());
    let first = timeline.advance_from_source_position(10_000, 480).unwrap();
    let after_gap = timeline.advance_from_source_position(10_960, 480).unwrap();
    assert_eq!(after_gap - first, 20_000_000);
}
#[test]
fn given_native_source_overlap_when_advanced_then_timeline_fails_closed() {
    let mut timeline = CaptureSampleTimeline::new(NonZeroU32::new(48_000).unwrap());
    timeline.advance_from_source_position(10_000, 480).unwrap();
    assert_eq!(
        timeline.advance_from_source_position(10_479, 480),
        Err(CaptureSampleTimelineError::SourcePositionMovedBackward {
            expected_at_least: 10_480,
            observed: 10_479,
        })
    );
}
#[test]
fn given_exact_application_after_open_when_authorization_snapshotted_then_scope_stays_exact() {
    let mut source = fake_source(
        SourceKind::Application,
        "com.acme.meeting",
        Some("com.acme.meeting"),
        SourceState::Playing,
    );
    source.process_id = Some(42);
    let snapshot = CaptureAuthorizationSnapshot::after_successful_open(
        &source,
        CaptureSessionGrant::GrantedByExplicitSelection,
        PermissionEpoch::INITIAL,
    );
    assert_eq!(snapshot.capability, CaptureCapabilityState::Available);
    assert_eq!(
        snapshot.capture_scope,
        CaptureScope::ExactApplication {
            stable_id: "com.acme.meeting".to_owned()
        }
    );
    assert_eq!(
        snapshot.identity_strength,
        SourceIdentityStrength::ApplicationIdAndProcessId
    );
    assert_eq!(
        snapshot.application_policy,
        ApplicationPolicyObservation::NotObservable
    );
    assert_eq!(snapshot.os_permission, PermissionObservation::NotObservable);
    assert_eq!(snapshot.open_outcome, CaptureOpenOutcome::Succeeded);
}
#[test]
fn given_process_only_application_when_identity_inspected_then_strength_is_not_overstated() {
    let mut source = fake_source(
        SourceKind::Application,
        "powershell",
        None,
        SourceState::Playing,
    );
    source.process_id = Some(42);
    assert_eq!(
        source.identity_strength(),
        SourceIdentityStrength::ProcessId
    );
}
#[test]
fn given_output_device_uid_when_identity_inspected_then_strength_is_stable_device_uid() {
    let mut source = fake_source(
        SourceKind::OutputDevice,
        "coreaudio:built-in-output",
        None,
        SourceState::Available,
    );
    source.device_uid = Some("coreaudio:built-in-output".to_owned());
    assert_eq!(
        source.identity_strength(),
        SourceIdentityStrength::StableDeviceUid
    );
    let snapshot = CaptureAuthorizationSnapshot::before_open(
        &source,
        CaptureSessionGrant::NotEvaluated,
        PermissionEpoch::INITIAL,
    );
    assert_eq!(
        snapshot.identity_strength,
        SourceIdentityStrength::StableDeviceUid
    );
}
#[test]
fn given_exact_microphone_after_open_when_authorization_snapshotted_then_device_uid_is_retained() {
    let mut source = fake_source(
        SourceKind::InputDevice,
        "coreaudio:built-in-mic",
        None,
        SourceState::Available,
    );
    source.device_uid = Some("coreaudio:built-in-mic".to_owned());
    let snapshot = CaptureAuthorizationSnapshot::after_successful_open(
        &source,
        CaptureSessionGrant::GrantedByExplicitSelection,
        PermissionEpoch::INITIAL,
    );
    assert_eq!(
        snapshot.capture_scope,
        CaptureScope::ExactInputDevice {
            stable_id: "coreaudio:built-in-mic".to_owned()
        }
    );
    assert_eq!(
        snapshot.identity_strength,
        SourceIdentityStrength::StableDeviceUid
    );
    assert_eq!(
        snapshot.application_policy,
        ApplicationPolicyObservation::NotApplicable
    );
}
#[test]
fn given_unclassified_backend_failure_when_snapshotted_then_permission_is_not_guessed() {
    let source = fake_source(
        SourceKind::Application,
        "com.acme.meeting",
        Some("com.acme.meeting"),
        SourceState::Available,
    );
    let snapshot = CaptureAuthorizationSnapshot::after_failed_open(
        &source,
        CaptureSessionGrant::GrantedByExplicitSelection,
        PermissionEpoch::INITIAL,
    );
    assert_eq!(snapshot.os_permission, PermissionObservation::NotObservable);
    assert_eq!(snapshot.open_outcome, CaptureOpenOutcome::BackendFailed);
}
#[test]
fn given_authoritative_permission_when_snapshotted_then_platform_state_is_preserved() {
    let source = fake_source(
        SourceKind::InputDevice,
        "coreaudio:built-in-mic",
        None,
        SourceState::Available,
    );
    let snapshot = CaptureAuthorizationSnapshot::from_open_observations(
        &source,
        CaptureSessionGrant::GrantedByExplicitSelection,
        PermissionEpoch::INITIAL,
        PermissionObservation::Denied,
        ApplicationPolicyObservation::NotApplicable,
        CaptureOpenOutcome::BackendFailed,
    );
    assert_eq!(snapshot.os_permission, PermissionObservation::Denied);
    assert!(snapshot.observed_at_ns > 0);
}
#[test]
fn given_revoked_permission_when_snapshotted_then_revocation_and_new_epoch_are_preserved() {
    let source = fake_source(
        SourceKind::InputDevice,
        "coreaudio:built-in-mic",
        None,
        SourceState::PermissionBlocked,
    );
    let revoked_epoch = PermissionEpoch::INITIAL.next();
    let snapshot = CaptureAuthorizationSnapshot::from_open_observations(
        &source,
        CaptureSessionGrant::Denied,
        revoked_epoch,
        PermissionObservation::Revoked,
        ApplicationPolicyObservation::NotApplicable,
        CaptureOpenOutcome::PermissionDenied,
    );
    assert_eq!(snapshot.os_permission, PermissionObservation::Revoked);
    assert_eq!(snapshot.permission_epoch, revoked_epoch);
    assert_eq!(snapshot.session_grant, CaptureSessionGrant::Denied);
    assert_eq!(snapshot.open_outcome, CaptureOpenOutcome::PermissionDenied);
    assert!(snapshot.observed_at_ns > 0);
}
#[test]
fn given_permission_lifecycle_when_authorization_changes_then_epoch_and_kind_are_canonical() {
    let mut lifecycle = CapturePermissionLifecycle::new(PermissionObservation::Allowed);
    assert_eq!(lifecycle.observe(PermissionObservation::Allowed), None);
    let revoked = lifecycle.observe(PermissionObservation::Denied).unwrap();
    assert_eq!(revoked.kind, SourceLifecycleEventKind::PermissionRevoked);
    assert_eq!(revoked.previous, PermissionObservation::Allowed);
    assert_eq!(revoked.current, PermissionObservation::Denied);
    assert_eq!(revoked.permission_epoch, PermissionEpoch(2));
    let changed = lifecycle
        .observe(PermissionObservation::NotDetermined)
        .unwrap();
    assert_eq!(changed.kind, SourceLifecycleEventKind::PermissionChanged);
    assert_eq!(changed.permission_epoch, PermissionEpoch(3));
    assert_eq!(lifecycle.current(), PermissionObservation::NotDetermined);
    assert_eq!(lifecycle.permission_epoch(), PermissionEpoch(3));
}
#[test]
fn given_unavailable_source_when_snapshotted_then_capability_is_unavailable() {
    let source = fake_source(
        SourceKind::Application,
        "com.acme.meeting",
        Some("com.acme.meeting"),
        SourceState::Unavailable,
    );
    let snapshot = CaptureAuthorizationSnapshot::before_open(
        &source,
        CaptureSessionGrant::GrantedByExplicitSelection,
        PermissionEpoch::INITIAL,
    );
    assert_eq!(snapshot.capability, CaptureCapabilityState::Unavailable);
}
#[test]
fn given_authorization_transition_when_epoch_advanced_then_previous_snapshot_is_not_reused() {
    assert_eq!(PermissionEpoch::INITIAL.next(), PermissionEpoch(2));
}
fn fake_source(
    kind: SourceKind,
    name: &str,
    app_id: Option<&str>,
    state: SourceState,
) -> CaptureSource {
    CaptureSource {
        stable_id: StableSourceId::new(Platform::Macos, kind, name),
        name: name.to_owned(),
        process_id: None,
        app_id: app_id.map(|a| a.to_owned()),
        device_uid: None,
        state,
        sample_rate_hz: 48_000,
        channels: 2,
    }
}
