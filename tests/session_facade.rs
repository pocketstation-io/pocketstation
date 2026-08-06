#[cfg(feature = "conformance-fixtures")]
use std::thread;
#[cfg(feature = "conformance-fixtures")]
use std::time::{Duration, Instant};

#[cfg(feature = "conformance-fixtures")]
use pocketstation::SessionRecordingState;
use pocketstation::{Session, Source};

#[test]
fn given_public_facade_when_session_declared_then_canonical_types_are_used() {
    let require_source: fn(Source) -> Source = |source| source;
    let _ = require_source(Source::microphone_default());

    let session_constructor = Session::new;
    let _ = session_constructor;

    let configured = Session::builder().recording_root("recordings").build();
    let _ = configured.id();
}

#[cfg(feature = "conformance-fixtures")]
#[test]
fn given_public_facade_when_external_destinations_run_then_all_branches_receive_media() {
    let recording_root = tempfile::tempdir().expect("temporary recording root");
    let session = pocketstation::conformance::session_with_recording(recording_root.path())
        .expect("canonical conformance Session");
    let connector = pocketstation::conformance::observed_connector(&session, Duration::ZERO)
        .expect("observed connector");
    let browser = pocketstation::conformance::observed_browser(&session, Duration::from_millis(25))
        .expect("observed browser");

    let app = session
        .capture(Source::application(
            pocketstation::ApplicationSelector::name("PocketStation Fixture"),
        ))
        .expect("application stem");
    let mic = session
        .capture(Source::microphone_default())
        .expect("microphone stem");

    let app_connector_route = app.send(connector).expect("application to connector");
    let mic_connector_route = mic.send(connector).expect("microphone to connector");
    let app_browser_route = app.send(browser).expect("application to browser");
    let mic_browser_route = mic.send(browser).expect("microphone to browser");
    app.record("application").expect("application recording");
    mic.record("microphone").expect("microphone recording");

    let mut running = session.start().expect("running Session");
    wait_for_external_routes(
        &running,
        &[
            app_connector_route.0,
            mic_connector_route.0,
            app_browser_route.0,
            mic_browser_route.0,
        ],
    );

    let stop = running.stop();
    assert!(stop.is_success(), "all endpoint branches must finalize");

    let recording = running
        .recording_outcome()
        .expect("Session-owned recording outcome");
    assert_eq!(recording.state, SessionRecordingState::Complete);
    assert_eq!(recording.completed_stems, 2);
    assert_eq!(recording.failed_stems, 0);
}

#[cfg(feature = "conformance-fixtures")]
#[test]
fn given_public_facade_when_flight_recording_enabled_then_trace_replays_complete_lifecycle() {
    let directory = tempfile::tempdir().expect("temporary flight-recorder root");
    let trace_path = directory.path().join("session.pksflight");
    let session = pocketstation::conformance::session_with_flight_recording(&trace_path, 32)
        .expect("canonical conformance Session");
    let session_id = session.id();
    let application = session
        .capture(Source::application(
            pocketstation::ApplicationSelector::name("PocketStation Fixture"),
        ))
        .expect("application stem");
    let microphone = session
        .capture(Source::microphone_default())
        .expect("microphone stem");
    application
        .send(session.polled_audio().expect("application audio endpoint"))
        .expect("application audio route");
    microphone
        .send(session.polled_audio().expect("microphone audio endpoint"))
        .expect("microphone audio route");

    let mut running = session.start().expect("running Session");
    wait_for_both_stems(&running);
    let stop = running.stop();
    assert!(stop.is_success(), "Session must stop cleanly");

    let outcome = running
        .flight_recording_outcome()
        .expect("flight-recorder outcome")
        .expect("flight-recorder finalization");
    assert!(outcome.is_complete(), "flight trace must be lossless");

    let trace = pocketstation::SessionFlightTrace::read(&trace_path).expect("read flight trace");
    let replay = trace.replay().expect("replay flight trace");
    assert_eq!(replay.session_id, session_id);
    assert_eq!(
        replay.lifecycle.as_ref(),
        &[
            pocketstation::SessionLifecycleState::Starting,
            pocketstation::SessionLifecycleState::Running,
            pocketstation::SessionLifecycleState::Stopping,
            pocketstation::SessionLifecycleState::Stopped,
        ]
    );
    assert_eq!(
        replay.terminal.state,
        pocketstation::SessionTerminalState::Stopped
    );
    assert_eq!(replay.records_replayed_total, 5);
}

#[cfg(feature = "conformance-fixtures")]
#[test]
fn given_stopped_public_session_when_new_session_starts_then_capture_restarts_cleanly() {
    for _ in 0..2 {
        let session = pocketstation::conformance::session().expect("canonical conformance Session");
        let application = session
            .capture(Source::application(
                pocketstation::ApplicationSelector::name("PocketStation Fixture"),
            ))
            .expect("application stem");
        let microphone = session
            .capture(Source::microphone_default())
            .expect("microphone stem");
        let audio = session.polled_audio().expect("polled audio endpoint");
        application.send(audio).expect("application audio route");
        microphone.send(audio).expect("microphone audio route");

        let mut running = session.start().expect("running Session");
        wait_for_both_stems(&running);
        assert!(running.stop().is_success(), "Session must stop cleanly");
    }
}

#[cfg(feature = "conformance-fixtures")]
fn wait_for_both_stems(running: &pocketstation::RunningSession) {
    let deadline = Instant::now() + Duration::from_secs(5);
    let mut stems = std::collections::BTreeSet::new();
    loop {
        if let Ok(batch) = running.try_poll_audio() {
            for index in 0..batch.len() {
                let frame = batch.frame(index).expect("valid bounded audio frame");
                stems.insert(frame.lineage().stem_id.0);
            }
            if stems.len() == 2 {
                return;
            }
        }
        assert!(
            Instant::now() < deadline,
            "application and microphone media must arrive before the deadline"
        );
        thread::sleep(Duration::from_millis(2));
    }
}

#[cfg(feature = "conformance-fixtures")]
fn wait_for_external_routes(running: &pocketstation::RunningSession, expected_route_ids: &[u64]) {
    let deadline = Instant::now() + Duration::from_secs(5);
    loop {
        let snapshot = running.metrics_snapshot().expect("Session metrics");
        let routes: Vec<_> = (0..snapshot.route_count())
            .filter_map(|index| snapshot.route(index).copied())
            .filter(|route| expected_route_ids.contains(&route.route_id.0))
            .collect();
        if routes.len() == expected_route_ids.len()
            && routes.iter().all(|route| {
                route.edge.frames_delivered_total > 0
                    && route
                        .endpoint
                        .is_some_and(|endpoint| endpoint.frames_received_total > 0)
            })
        {
            return;
        }
        assert!(
            Instant::now() < deadline,
            "all connector and browser routes must deliver before the deadline"
        );
        thread::sleep(Duration::from_millis(2));
    }
}
