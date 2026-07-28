#![cfg(feature = "conformance-fixtures")]

use std::collections::BTreeSet;
use std::time::{Duration, Instant};

use pocketstation::{
    conformance, ApplicationSelector, SessionEventReceive, SessionStartCancellation,
    SessionStartErrorKind, SessionStopDisposition, Source,
};

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
