#[cfg(feature = "conformance-fixtures")]
use std::thread;
#[cfg(feature = "conformance-fixtures")]
use std::time::{Duration, Instant};

#[cfg(feature = "conformance-fixtures")]
use pocketstation::SessionRecordingState;
use pocketstation::{
    ApplicationSelector, Platform, ProcessId, Session, Source, SourceKind, StableSourceId,
};

#[test]
fn given_public_facade_when_session_declared_then_canonical_types_are_used() {
    let require_source: fn(Source) -> Source = |source| source;
    let _ = require_source(Source::microphone_default());
    let _ = require_source(Source::system_audio());

    let session_constructor = Session::new;
    let _ = session_constructor;

    let configured = Session::builder().recording_root("recordings").build();
    let _ = configured.id();
}

#[cfg(feature = "conformance-fixtures")]
#[test]
fn given_system_audio_when_session_runs_then_system_mix_keeps_its_own_stem() {
    let session = pocketstation::conformance::session().expect("canonical conformance Session");
    let system_audio = session
        .capture(Source::system_audio())
        .expect("system audio stem");
    let expected_stem_id = system_audio.id();
    system_audio
        .send(session.polled_audio().expect("system audio endpoint"))
        .expect("system audio route");

    let mut running = session.start().expect("running Session");
    let deadline = Instant::now() + Duration::from_secs(5);
    let (observed_stem_id, observed_source_id) = loop {
        if let Ok(batch) = running.try_poll_audio() {
            if let Some(frame) = batch.frame(0) {
                break (frame.lineage().stem_id(), frame.lineage().source_id().get());
            }
        }
        assert!(Instant::now() < deadline, "system audio frame timed out");
        thread::sleep(Duration::from_millis(1));
    };

    assert_eq!(observed_stem_id, expected_stem_id);
    assert_eq!(observed_source_id, 152);
    assert!(running.stop().is_success());
}

#[test]
fn given_application_selector_inputs_when_declared_then_public_facade_remains_concise() {
    let explicit_constructor: fn(ApplicationSelector) -> Source = Source::application;
    let stable_id = StableSourceId::new(Platform::Macos, SourceKind::Application, "us.zoom.xos");

    assert_eq!(
        Source::application("Zoom"),
        explicit_constructor(ApplicationSelector::name("Zoom"))
    );
    assert_eq!(
        Source::application(ProcessId::new(1234)),
        explicit_constructor(ApplicationSelector::process_id(ProcessId::new(1234)))
    );
    assert_eq!(
        Source::application(&stable_id),
        explicit_constructor(ApplicationSelector::stable_id(stable_id))
    );
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
            app_connector_route.get(),
            mic_connector_route.get(),
            app_browser_route.get(),
            mic_browser_route.get(),
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
fn given_public_facade_when_session_trace_enabled_then_trace_replays_complete_lifecycle() {
    let directory = tempfile::tempdir().expect("temporary session trace root");
    let trace_path = directory.path().join("session.pkstrace");
    let session = pocketstation::conformance::session_with_trace(&trace_path, 32)
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
        .session_trace_outcome()
        .expect("session trace outcome")
        .expect("session trace finalization");
    assert!(outcome.is_complete(), "Session trace must be lossless");

    let trace = pocketstation::SessionTrace::read(&trace_path).expect("read Session trace");
    let validation = trace.validate().expect("validate Session trace");
    assert_eq!(validation.session_id, session_id);
    assert_eq!(
        validation.lifecycle.as_ref(),
        &[
            pocketstation::SessionLifecycleState::Starting,
            pocketstation::SessionLifecycleState::Running,
            pocketstation::SessionLifecycleState::Stopping,
            pocketstation::SessionLifecycleState::Stopped,
        ]
    );
    assert_eq!(
        validation.terminal.state,
        pocketstation::SessionTerminalState::Stopped
    );
    assert_eq!(validation.records_validated_total, 5);
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
                stems.insert(frame.lineage().stem_id().get());
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
            .filter(|route| expected_route_ids.contains(&route.route_id.get()))
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
