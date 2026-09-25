#![cfg(feature = "conformance-fixtures")]

use std::collections::BTreeSet;
use std::path::PathBuf;
use std::time::{Duration, Instant};

use pocketstation::{
    conformance, ApplicationSelector, CaptureNativeFormat, CaptureSampleRepresentation, DeviceId,
    DeviceSelector, SessionEventReceive, SessionStartCancellation, SessionStartErrorKind,
    SessionStopDisposition, Source,
};

fn artifact_root(test_name: &str) -> PathBuf {
    std::env::temp_dir().join(format!("pocketstation-{test_name}-{}", std::process::id()))
}

#[test]
fn given_fixture_session_when_started_then_two_stems_cross_canonical_engine() {
    let session = conformance::session().unwrap();
    let application = session
        .capture(Source::application(ApplicationSelector::name(
            "conformance application",
        )))
        .unwrap();
    let microphone = session.capture(Source::microphone_default()).unwrap();
    let application_audio = session.polled_audio().unwrap();
    let microphone_audio = session.polled_audio().unwrap();
    application.send(application_audio).unwrap();
    microphone.send(microphone_audio).unwrap();

    let mut running = session.start().unwrap();
    let deadline = Instant::now() + Duration::from_secs(2);
    let mut stem_ids = BTreeSet::new();
    while Instant::now() < deadline && stem_ids.len() < 2 {
        if let Ok(batch) = running.try_poll_audio() {
            for index in 0..batch.len() {
                stem_ids.insert(batch.frame(index).unwrap().lineage().stem_id().get());
            }
        }
        std::thread::sleep(Duration::from_millis(1));
    }

    assert_eq!(stem_ids.len(), 2);
    assert!(running.audio_observations().frames_polled_total >= 2);
    assert!(!matches!(
        running.try_recv_event(),
        SessionEventReceive::Closed
    ));
    let metrics = running.metrics_snapshot().unwrap();
    assert!(metrics.source(0).is_some());
    assert!(metrics.source(1).is_some());
    assert!(metrics.source(2).is_none());
    assert_eq!(metrics.source_activity_count(), metrics.source_count());
    assert_eq!(metrics.source_native_format_count(), metrics.source_count());
    for index in 0..metrics.source_count() {
        let activity = metrics
            .source_activity(index)
            .expect("every built-in Source has aligned activity observations");
        assert!(activity.frames_received_total > 0);
        assert!(activity.first_frame_received_at_ns.is_some());
        assert!(activity.latest_frame_received_at_ns.is_some());
        assert!(activity.session_started_at_ns <= activity.observed_at_ns);
        let native_format = metrics
            .source_native_format(index)
            .expect("every built-in Source has aligned native-format observations")
            .opened_native_format
            .expect("the deterministic capture fixture reports its opened format");
        assert_eq!(native_format.sample_rate_hz, 48_000);
        assert_eq!(
            native_format.sample_representation,
            CaptureSampleRepresentation::Float32
        );
    }
    assert_eq!(
        metrics
            .source_native_format(0)
            .and_then(|observation| observation.opened_native_format),
        Some(CaptureNativeFormat {
            sample_rate_hz: 48_000,
            channel_count: 2,
            sample_representation: CaptureSampleRepresentation::Float32,
        })
    );
    assert_eq!(
        metrics
            .source_native_format(1)
            .and_then(|observation| observation.opened_native_format),
        Some(CaptureNativeFormat {
            sample_rate_hz: 48_000,
            channel_count: 1,
            sample_representation: CaptureSampleRepresentation::Float32,
        })
    );
    assert!(metrics.source_activity(metrics.source_count()).is_none());
    let first = running.stop();
    let second = running.stop();
    assert_eq!(first.disposition(), SessionStopDisposition::Stopped);
    assert_eq!(second.disposition(), SessionStopDisposition::AlreadyStopped);
    assert_eq!(first.outcome(), second.outcome());
    assert!(first.is_success());
}

#[test]
fn given_hfp_selector_when_microphone_is_replaced_then_native_format_and_identity_change() {
    let session = conformance::session().unwrap();
    let application = session
        .capture(Source::application(ApplicationSelector::name(
            "conformance application",
        )))
        .unwrap();
    let microphone = session.capture(Source::microphone_default()).unwrap();
    let output = session.polled_audio().unwrap();
    application.send(output).unwrap();
    microphone.send(output).unwrap();

    let mut running = session.start().unwrap();
    let replacement = running
        .replace_microphone_source(
            microphone.id(),
            DeviceSelector::id(DeviceId::new(conformance::HFP_MICROPHONE_DEVICE_ID)),
        )
        .unwrap();

    assert_eq!(replacement.previous_source_id.get(), 202);
    assert_eq!(replacement.source_id.get(), 204);
    assert_eq!(replacement.source_generation, 2);
    assert_eq!(replacement.discontinuity_epoch, 1);
    assert_eq!(
        replacement.opened_native_format,
        Some(CaptureNativeFormat {
            sample_rate_hz: 16_000,
            channel_count: 1,
            sample_representation: CaptureSampleRepresentation::SignedInteger16,
        })
    );

    let deadline = Instant::now() + Duration::from_secs(2);
    let mut replacement_frame_observed = false;
    while Instant::now() < deadline && !replacement_frame_observed {
        if let Ok(batch) = running.try_poll_audio() {
            replacement_frame_observed = (0..batch.len()).any(|index| {
                batch.frame(index).is_some_and(|frame| {
                    let lineage = frame.lineage();
                    lineage.stem_id() == microphone.id()
                        && lineage.source_id().get() == 204
                        && lineage.source_generation() == 2
                        && lineage.discontinuity_epoch() == 1
                })
            });
        }
        std::thread::sleep(Duration::from_millis(1));
    }
    assert!(replacement_frame_observed);
    assert!(running.stop().is_success());
}

#[test]
fn given_requested_cancellation_when_fixture_started_then_start_fails_typed() {
    let session = conformance::session().unwrap();
    let application = session
        .capture(Source::application(ApplicationSelector::name(
            "conformance application",
        )))
        .unwrap();
    let microphone = session.capture(Source::microphone_default()).unwrap();
    application.send(session.polled_audio().unwrap()).unwrap();
    microphone.send(session.polled_audio().unwrap()).unwrap();
    let cancellation = SessionStartCancellation::default();
    cancellation.request();

    let error = session.start_cancellable(cancellation).err().unwrap();
    assert!(error.is_cancelled());
}

#[test]
fn given_empty_application_selector_when_declared_then_error_is_typed() {
    let session = conformance::session().unwrap();
    let application = session
        .capture(Source::application(ApplicationSelector::name(" ")))
        .unwrap();
    let microphone = session.capture(Source::microphone_default()).unwrap();
    application.send(session.polled_audio().unwrap()).unwrap();
    microphone.send(session.polled_audio().unwrap()).unwrap();
    let error = session.start().err().unwrap();
    assert_eq!(error.kind(), SessionStartErrorKind::InvalidSelector);
}

#[test]
fn given_recording_routes_without_root_when_started_then_configuration_error_is_typed() {
    let session = conformance::session().unwrap();
    let application = session
        .capture(Source::application(ApplicationSelector::name(
            "conformance application",
        )))
        .unwrap();
    let microphone = session.capture(Source::microphone_default()).unwrap();
    application.record("application").unwrap();
    microphone.record("microphone").unwrap();

    let error = session.start().err().unwrap();
    assert_eq!(
        error.kind(),
        SessionStartErrorKind::MissingRecordingConfiguration,
        "{error:?}"
    );
    assert_eq!(
        error.code().as_str(),
        "session.missing_recording_configuration"
    );
}

#[test]
fn given_recording_root_when_two_stems_finish_then_terminal_outcome_is_exposed() {
    let output_root = artifact_root("recording-outcome");
    let session = conformance::session_with_recording(&output_root).unwrap();
    let application = session
        .capture(Source::application(ApplicationSelector::name(
            "conformance application",
        )))
        .unwrap();
    let microphone = session.capture(Source::microphone_default()).unwrap();
    application.send(session.polled_audio().unwrap()).unwrap();
    microphone.send(session.polled_audio().unwrap()).unwrap();
    application.record("application").unwrap();
    microphone.record("microphone").unwrap();

    let mut running = session.start().unwrap();
    let deadline = Instant::now() + Duration::from_secs(2);
    let mut frame_counts = std::collections::BTreeMap::<u64, u64>::new();
    while Instant::now() < deadline
        && frame_counts.values().filter(|count| **count >= 3).count() < 2
    {
        if let Ok(batch) = running.try_poll_audio() {
            for index in 0..batch.len() {
                let stem_id = batch.frame(index).unwrap().lineage().stem_id().get();
                *frame_counts.entry(stem_id).or_default() += 1;
            }
        }
        std::thread::sleep(Duration::from_millis(1));
    }
    assert_eq!(frame_counts.len(), 2);
    assert!(frame_counts.values().all(|count| *count >= 3));

    // Deliberately stop consuming this bounded destination. The recorder is a
    // separate branch and must continue to completion while this queue drops.
    let completion_deadline = Instant::now() + Duration::from_secs(2);
    let mut sources_completed = false;
    while Instant::now() < completion_deadline {
        let metrics = running.metrics_snapshot().unwrap();
        sources_completed = (0..metrics.source_count()).all(|index| {
            metrics.source(index).is_some_and(|source| {
                source.ingress.frames_delivered_total >= conformance::FRAMES_PER_SOURCE
            })
        });
        if sources_completed {
            break;
        }
        std::thread::sleep(Duration::from_millis(1));
    }
    assert!(sources_completed);
    let saturation_deadline = Instant::now() + Duration::from_secs(2);
    let slow_branch = loop {
        let observations = running.audio_observations();
        if observations.queue_full_drops_total > 0 || Instant::now() >= saturation_deadline {
            break observations;
        }
        std::thread::sleep(Duration::from_millis(1));
    };
    assert!(slow_branch.queue_peak_frames <= slow_branch.queue_capacity_frames);
    assert!(slow_branch.queue_full_drops_total > 0);

    let stop = running.stop();
    let mut terminal_events = Vec::new();
    while let SessionEventReceive::Event(event) = running.try_recv_event() {
        terminal_events.push(event);
    }
    let outcome = running.recording_outcome();
    assert!(
        stop.is_success(),
        "{stop:?}; recording={outcome:?}; events={terminal_events:?}"
    );
    let outcome = outcome.unwrap();
    assert_eq!(
        outcome.state,
        pocketstation::SessionRecordingState::Complete
    );
    assert_eq!(outcome.completed_stems, 2);
    assert_eq!(outcome.failed_stems, 0);
    assert_eq!(outcome.stems.len(), 2);
    assert!(
        outcome
            .stems
            .iter()
            .all(|stem| stem.written_frames == conformance::FRAMES_PER_SOURCE
                && stem.error.is_none()
                && stem.edge_observations.frames_dropped_total == 0
                && stem.edge_observations.discontinuities_total == 0),
        "{outcome:?}"
    );
    assert!(outcome.session_dir.exists());
}
