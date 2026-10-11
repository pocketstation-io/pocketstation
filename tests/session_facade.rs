#[cfg(feature = "conformance-fixtures")]
use std::thread;
#[cfg(feature = "conformance-fixtures")]
use std::time::{Duration, Instant};

#[cfg(feature = "conformance-fixtures")]
use pocketstation::SessionRecordingState;
use pocketstation::{
    ApplicationSelector, DeviceId, DeviceSelector, Platform, ProcessId, Session, Source,
    SourceKind, StableSourceId,
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

mod public_rollback {
    use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
    use std::sync::Arc;

    use pocketstation::{
        ActiveCaptureBackend, AudioHistoryConfig, CallbackCaptureBackend, CaptureDelivery,
        CaptureError, CaptureMode, CaptureObservationHandle, CaptureObservations,
        CaptureOpenCancellation, CaptureOpenFailure, Operator, OperatorConfiguration, OperatorId,
        PreparedCaptureBackend, Session, SessionStartCancellation, SessionStartError,
        SessionStartErrorCode, SessionStartErrorKind, Source, SourceId,
    };

    #[derive(Clone, Copy)]
    enum Open {
        Active { cancel: bool, cleanup_fails: bool },
        Failed { cleanup_fails: bool },
    }

    #[derive(Default)]
    struct Calls {
        prepare: AtomicU64,
        open: AtomicU64,
        stop: AtomicU64,
        observed: AtomicBool,
    }

    struct Backend {
        behavior: Open,
        source_id: SourceId,
        calls: Arc<Calls>,
    }

    struct Prepared(Backend);
    struct Active(Backend);

    fn cleanup_error() -> CaptureError {
        CaptureError::BackendStatus {
            operation: "MOCKED capture unregister",
            status_code: -50,
        }
    }

    impl CallbackCaptureBackend for Backend {
        fn prepare(&self, _: CaptureMode) -> Result<Box<dyn PreparedCaptureBackend>, CaptureError> {
            self.calls.prepare.fetch_add(1, Ordering::Relaxed);
            Ok(Box::new(Prepared(Backend {
                behavior: self.behavior,
                source_id: self.source_id,
                calls: Arc::clone(&self.calls),
            })))
        }
    }

    impl PreparedCaptureBackend for Prepared {
        fn open(
            self: Box<Self>,
            _: CaptureDelivery,
        ) -> Result<Box<dyn ActiveCaptureBackend>, CaptureError> {
            panic!("public Session must use its cancellable acquisition path");
        }

        fn open_cancellable(
            self: Box<Self>,
            _: CaptureDelivery,
            cancellation: &CaptureOpenCancellation,
        ) -> Result<Box<dyn ActiveCaptureBackend>, CaptureOpenFailure> {
            self.0.calls.open.fetch_add(1, Ordering::Relaxed);
            self.0.calls.observed.store(
                cancellation.native_call_observations().is_some(),
                Ordering::Release,
            );
            match self.0.behavior {
                Open::Active { cancel, .. } => {
                    if cancel {
                        cancellation.request();
                    }
                    Ok(Box::new(Active(self.0)))
                }
                Open::Failed { cleanup_fails } => Err(CaptureOpenFailure::Backend {
                    source: CaptureError::BackendStatus {
                        operation: "MOCKED capture start",
                        status_code: 0x10004003,
                    },
                    cleanup_error: cleanup_fails.then(cleanup_error),
                }),
            }
        }
    }

    impl ActiveCaptureBackend for Active {
        fn source_id(&self) -> SourceId {
            self.0.source_id
        }

        fn observation_handle(&self) -> CaptureObservationHandle {
            CaptureObservationHandle::default()
        }

        fn observations(&self) -> CaptureObservations {
            CaptureObservations::default()
        }

        fn stop_and_join(self: Box<Self>) -> Result<CaptureObservations, CaptureError> {
            self.0.calls.stop.fetch_add(1, Ordering::Relaxed);
            if matches!(
                self.0.behavior,
                Open::Active {
                    cleanup_fails: true,
                    ..
                }
            ) {
                Err(cleanup_error())
            } else {
                Ok(CaptureObservations::default())
            }
        }
    }

    fn composition(
        application: Open,
        microphone: Open,
        recording_root: Option<&std::path::Path>,
    ) -> (Session, Arc<Calls>, Arc<Calls>) {
        let app_calls = Arc::new(Calls::default());
        let mic_calls = Arc::new(Calls::default());
        let builder = Session::builder().capture_backends(
            Arc::new(Backend {
                behavior: application,
                source_id: SourceId::new(101),
                calls: Arc::clone(&app_calls),
            }),
            Arc::new(Backend {
                behavior: microphone,
                source_id: SourceId::new(102),
                calls: Arc::clone(&mic_calls),
            }),
        );
        let session = match recording_root {
            Some(root) => builder.recording_root(root).build(),
            None => builder.build(),
        };
        let _history = session
            .audio_history(AudioHistoryConfig::default())
            .expect("bounded history declaration");
        let app = session
            .capture(Source::application("MOCKED application"))
            .unwrap();
        let mic = session.capture(Source::microphone_default()).unwrap();
        app.record("application").unwrap();
        mic.record("microphone").unwrap();
        app.retain_audio().unwrap();
        mic.retain_audio().unwrap();
        (session, app_calls, mic_calls)
    }

    fn failure(session: Session, token: SessionStartCancellation) -> SessionStartError {
        match session.start_cancellable(token) {
            Err(error) => error,
            Ok(mut running) => {
                let _ = running.cancel();
                panic!("MOCKED startup fault must reject the public Session");
            }
        }
    }

    #[test]
    fn given_observed_public_token_when_cloned_and_used_by_session_then_optional_read_state_reaches_backend(
    ) {
        let ordinary = SessionStartCancellation::default();
        assert!(ordinary.native_call_observations().is_none());
        let token = SessionStartCancellation::observed();
        let clone = token.clone();
        let initial = clone.native_call_observations().unwrap();
        assert_eq!(
            initial.current.len(),
            pocketstation::NATIVE_CALL_CURRENT_CAPACITY
        );
        assert_eq!(
            initial.completed.len(),
            pocketstation::NATIVE_CALL_COMPLETED_CAPACITY
        );
        assert!(initial.current.iter().all(Option::is_none));
        assert!(initial.completed.iter().all(Option::is_none));
        let root = tempfile::tempdir().unwrap();
        let (session, app, mic) = composition(
            Open::Active {
                cancel: true,
                cleanup_fails: false,
            },
            Open::Failed {
                cleanup_fails: false,
            },
            Some(root.path()),
        );
        let error = failure(session, token);
        assert!(error.is_cancelled());
        assert!(clone.is_requested());
        assert_eq!(error.rollback_failures_total(), 0);
        assert!(app.observed.load(Ordering::Acquire));
        assert_eq!(app.stop.load(Ordering::Acquire), 1);
        assert_eq!(mic.prepare.load(Ordering::Acquire), 0);
        // A MOCKED backend that makes no native calls cannot fabricate spans.
        let final_snapshot = clone.native_call_observations().unwrap();
        assert_eq!(final_snapshot.completed_calls_total, 0);
        assert!(final_snapshot.current.iter().all(Option::is_none));
    }

    #[test]
    fn given_late_open_cancel_when_public_recording_history_starts_then_cleanup_count_and_cancel_code_survive(
    ) {
        for cleanup_fails in [false, true] {
            let root = tempfile::tempdir().unwrap();
            let (session, app, mic) = composition(
                Open::Active {
                    cancel: true,
                    cleanup_fails,
                },
                Open::Failed {
                    cleanup_fails: false,
                },
                Some(root.path()),
            );
            let error = failure(session, SessionStartCancellation::default());
            assert_eq!(error.rollback_failures_total(), u64::from(cleanup_fails));
            assert_eq!(error.code(), SessionStartErrorCode::StartCancelled);
            assert_eq!(error.kind(), SessionStartErrorKind::Cancelled);
            assert!(error.is_cancelled());
            assert_eq!(error.message(), "Session start failed: Session transactional start failed: Session start was cancelled");
            assert!(error.compile_diagnostic().is_none());
            assert_eq!(error.clone(), error);
            assert_eq!(app.open.load(Ordering::Acquire), 1);
            assert_eq!(app.stop.load(Ordering::Acquire), 1);
            assert_eq!(mic.prepare.load(Ordering::Acquire), 0);
            assert_eq!(mic.open.load(Ordering::Acquire), 0);
        }
    }

    #[test]
    fn given_backend_open_failure_when_public_recording_history_starts_then_primary_error_and_cleanup_count_survive(
    ) {
        for cleanup_fails in [false, true] {
            let root = tempfile::tempdir().unwrap();
            let (session, app, mic) = composition(
                Open::Failed { cleanup_fails },
                Open::Failed {
                    cleanup_fails: false,
                },
                Some(root.path()),
            );
            let error = failure(session, SessionStartCancellation::default());
            assert_eq!(error.rollback_failures_total(), u64::from(cleanup_fails));
            assert_eq!(error.code(), SessionStartErrorCode::CaptureBackendFailed);
            assert_eq!(error.kind(), SessionStartErrorKind::Engine);
            assert!(!error.is_cancelled());
            assert!(error
                .message()
                .ends_with("capture backend failed while MOCKED capture start: status 268451843"));
            assert!(!error.message().contains("unregister"));
            assert!(error.compile_diagnostic().is_none());
            assert_eq!(app.open.load(Ordering::Acquire), 1);
            assert_eq!(app.stop.load(Ordering::Acquire), 0);
            assert_eq!(mic.prepare.load(Ordering::Acquire), 0);
        }
    }

    #[test]
    fn given_prior_capture_cleanup_failure_when_microphone_open_fails_then_public_count_retains_every_reported_failure(
    ) {
        for microphone_cleanup_fails in [false, true] {
            let root = tempfile::tempdir().unwrap();
            let (session, app, mic) = composition(
                Open::Active {
                    cancel: false,
                    cleanup_fails: true,
                },
                Open::Failed {
                    cleanup_fails: microphone_cleanup_fails,
                },
                Some(root.path()),
            );
            let error = failure(session, SessionStartCancellation::default());
            assert_eq!(
                error.rollback_failures_total(),
                1 + u64::from(microphone_cleanup_fails)
            );
            assert_eq!(error.code(), SessionStartErrorCode::CaptureBackendFailed);
            assert!(!error.is_cancelled());
            assert_eq!(app.stop.load(Ordering::Acquire), 1);
            assert_eq!(mic.open.load(Ordering::Acquire), 1);
        }
    }

    #[test]
    fn given_requested_cancel_when_public_recording_history_starts_then_no_capture_opens_or_rollback_failure_is_reported(
    ) {
        let root = tempfile::tempdir().unwrap();
        let (session, app, mic) = composition(
            Open::Failed {
                cleanup_fails: true,
            },
            Open::Failed {
                cleanup_fails: true,
            },
            Some(root.path()),
        );
        let token = SessionStartCancellation::default();
        token.request();
        let error = failure(session, token);
        assert_eq!(error.rollback_failures_total(), 0);
        assert!(error.is_cancelled());
        assert_eq!(app.prepare.load(Ordering::Acquire), 0);
        assert_eq!(mic.prepare.load(Ordering::Acquire), 0);
    }

    #[test]
    fn given_missing_recording_root_when_public_recording_history_starts_then_preacquisition_error_has_zero_reported_failures(
    ) {
        let (session, app, mic) = composition(
            Open::Failed {
                cleanup_fails: true,
            },
            Open::Failed {
                cleanup_fails: true,
            },
            None,
        );
        let error = failure(session, SessionStartCancellation::default());
        assert_eq!(error.rollback_failures_total(), 0);
        assert_eq!(
            error.code(),
            SessionStartErrorCode::MissingRecordingConfiguration
        );
        assert_eq!(
            error.kind(),
            SessionStartErrorKind::MissingRecordingConfiguration
        );
        assert_eq!(
            error.message(),
            "recording routes require an explicit Session recording root"
        );
        assert!(!error.is_cancelled());
        assert!(error.compile_diagnostic().is_none());
        assert_eq!(app.prepare.load(Ordering::Acquire), 0);
        assert_eq!(mic.prepare.load(Ordering::Acquire), 0);
    }

    #[test]
    fn given_unregistered_operator_when_public_recording_history_compiles_then_diagnostic_and_zero_reported_count_survive(
    ) {
        let root = tempfile::tempdir().unwrap();
        let (session, app, mic) = composition(
            Open::Failed {
                cleanup_fails: true,
            },
            Open::Failed {
                cleanup_fails: true,
            },
            Some(root.path()),
        );
        let operator_id = "example.operator.unregistered.v1";
        let compiler_input = session
            .capture(Source::application("MOCKED compiler input"))
            .unwrap();
        let output = compiler_input
            .through(Operator::new(
                OperatorId::new(operator_id),
                OperatorConfiguration::new(),
            ))
            .unwrap();
        output.send(session.polled_audio().unwrap()).unwrap();
        let error = failure(session, SessionStartCancellation::default());
        assert_eq!(error.rollback_failures_total(), 0);
        assert_eq!(error.code(), SessionStartErrorCode::CompileFailed);
        assert_eq!(error.kind(), SessionStartErrorKind::Engine);
        assert!(!error.is_cancelled());
        assert_eq!(error.message(), "Session start failed: Session compilation failed: async operator example.operator.unregistered.v1 is not registered");
        let diagnostic = error
            .compile_diagnostic()
            .expect("public compiler diagnostic");
        assert_eq!(diagnostic.code(), "compile.unknown_async_operator");
        assert_eq!(diagnostic.operator_id(), Some(operator_id));
        assert_eq!(diagnostic.node_index(), None);
        assert_eq!(error.clone().compile_diagnostic(), Some(diagnostic));
        assert_eq!(app.prepare.load(Ordering::Acquire), 0);
        assert_eq!(mic.prepare.load(Ordering::Acquire), 0);
    }
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
#[test]
fn given_running_public_session_when_microphone_is_reopened_with_exact_device_then_lineage_advances(
) {
    let session = pocketstation::conformance::session().expect("canonical conformance Session");
    let application = session
        .capture(Source::application("PocketStation Fixture"))
        .expect("application stem");
    let microphone = session
        .capture(Source::microphone_default())
        .expect("microphone stem");
    let microphone_stem_id = microphone.id();
    let audio = session.polled_audio().expect("polled audio endpoint");
    application.send(audio).expect("application audio route");
    microphone.send(audio).expect("microphone audio route");

    let mut running = session.start().expect("running Session");
    wait_for_both_stems(&running);
    let replacement = running
        .reopen_microphone_source(
            microphone_stem_id,
            DeviceSelector::id(DeviceId::new("conformance:microphone")),
        )
        .expect("explicit microphone reopen");

    assert_eq!(replacement.stem_id, microphone_stem_id);
    assert_eq!(replacement.previous_source_id.get(), 202);
    assert_eq!(replacement.source_id.get(), 203);
    assert_eq!(replacement.source_generation, 2);
    assert_eq!(replacement.discontinuity_epoch, 1);

    let snapshot = running.metrics_snapshot().expect("Session metrics");
    let observation = (0..snapshot.source_replacement_count())
        .filter_map(|index| snapshot.source_replacement(index))
        .find(|observation| observation.stem_id == microphone_stem_id)
        .expect("microphone replacement observations");
    assert_eq!(observation.attempts_total, 1);
    assert_eq!(observation.completed_total, 1);
    assert_eq!(observation.attached_source_id, Some(replacement.source_id));
    assert_eq!(observation.source_generation, 2);
    assert_eq!(observation.discontinuity_epoch, 1);
    assert!(running.stop().is_success());
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
