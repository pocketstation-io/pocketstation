#![cfg(feature = "conformance-fixtures")]

use std::collections::BTreeSet;
use std::path::PathBuf;
use std::time::{Duration, Instant};

use pocketstation::{
    conformance, ApplicationSelector, SessionEventReceive, SessionStartCancellation,
    SessionStartErrorKind, SessionStopDisposition, Source,
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
                stem_ids.insert(batch.frame(index).unwrap().lineage().stem_id.0);
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
    let first = running.stop();
    let second = running.stop();
    assert_eq!(first.disposition(), SessionStopDisposition::Stopped);
    assert_eq!(second.disposition(), SessionStopDisposition::AlreadyStopped);
    assert_eq!(first.outcome(), second.outcome());
    assert!(first.is_success());
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
                let stem_id = batch.frame(index).unwrap().lineage().stem_id.0;
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
