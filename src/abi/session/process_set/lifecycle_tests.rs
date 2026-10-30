use super::*;

#[test]
fn given_shutdown_requested_when_cloned_engine_is_used_then_new_work_is_rejected() {
    let engine = EngineRuntime::new(SessionRuntime::with_options(
        NativeSessionEngineHostOptions::default(),
        1,
    ));
    engine.closing_requested.store(true, Ordering::Release);
    assert_eq!(engine.check_open(), Err(AbiError::InvalidLifecycleState));
    assert!(!engine.start_cancellation.is_requested());
}

#[test]
fn given_uncertain_start_cleanup_when_destroyed_twice_then_session_and_engine_remain_owned() {
    let state = RuntimeState::new();
    let engine_handle = state
        .allocate_engine(NativeSessionEngineHostOptions::default())
        .unwrap();
    let engine = state.engine(engine_handle).unwrap();
    let session_handle = {
        let mut runtime = engine.runtime.lock().unwrap();
        let session_handle = runtime
            .sessions
            .insert(SessionObject::Failed { events: None })
            .unwrap();
        runtime.startup_cleanup_unconfirmed = true;
        engine.session_handle.set(session_handle).unwrap();
        engine.session_live.store(true, Ordering::Release);
        session_handle
    };
    for _ in 0..2 {
        assert!(matches!(
            state.destroy_session(engine_handle, session_handle),
            Err(AbiError::ProcessSetFailure { detail_code: 5 })
        ));
        assert!(matches!(
            state.release_engine(engine_handle),
            Err(AbiError::ProcessSetFailure { detail_code: 5 })
        ));
        assert_eq!(state.engine_is_live(engine_handle), Ok(true));
        assert!(engine.validate_session(session_handle).is_ok());
    }
}

#[test]
fn given_confirmed_failed_start_when_destroyed_then_old_handle_becomes_stale() {
    let state = RuntimeState::new();
    let engine_handle = state
        .allocate_engine(NativeSessionEngineHostOptions::default())
        .unwrap();
    let engine = state.engine(engine_handle).unwrap();
    let session_handle = {
        let mut runtime = engine.runtime.lock().unwrap();
        let session_handle = runtime
            .sessions
            .insert(SessionObject::Failed { events: None })
            .unwrap();
        engine.session_handle.set(session_handle).unwrap();
        engine.session_live.store(true, Ordering::Release);
        session_handle
    };
    state
        .destroy_session(engine_handle, session_handle)
        .unwrap();
    assert_eq!(
        engine.validate_session(session_handle),
        Err(AbiError::StaleHandle)
    );
    state.release_engine(engine_handle).unwrap();
    assert_eq!(
        state.engine_is_live(engine_handle),
        Err(AbiError::StaleHandle)
    );
}

use crate::capture::{
    ActiveCaptureBackend, CallbackCaptureBackend, CaptureDelivery, CaptureError, CaptureMode,
    CaptureObservationHandle, CaptureObservations, PreparedCaptureBackend,
};
use crate::graph::PrepareContext;
use std::sync::atomic::AtomicUsize;
use std::time::{Duration, Instant};

struct FaultBackend {
    fail_close: bool,
    opened_total: Arc<AtomicUsize>,
    closed_total: Arc<AtomicUsize>,
}
struct FaultPrepared(FaultBackend);
struct FaultActive(FaultBackend);
impl CallbackCaptureBackend for FaultBackend {
    fn prepare(&self, _mode: CaptureMode) -> Result<Box<dyn PreparedCaptureBackend>, CaptureError> {
        Ok(Box::new(FaultPrepared(FaultBackend {
            fail_close: self.fail_close,
            opened_total: Arc::clone(&self.opened_total),
            closed_total: Arc::clone(&self.closed_total),
        })))
    }
}
impl PreparedCaptureBackend for FaultPrepared {
    fn open(
        self: Box<Self>,
        _delivery: CaptureDelivery,
    ) -> Result<Box<dyn ActiveCaptureBackend>, CaptureError> {
        self.0.opened_total.fetch_add(1, Ordering::AcqRel);
        Ok(Box::new(FaultActive(self.0)))
    }
}
impl ActiveCaptureBackend for FaultActive {
    fn source_id(&self) -> crate::SourceId {
        crate::SourceId(99)
    }
    fn observation_handle(&self) -> CaptureObservationHandle {
        CaptureObservationHandle::default()
    }
    fn observations(&self) -> CaptureObservations {
        CaptureObservations::default()
    }
    fn stop_and_join(self: Box<Self>) -> Result<CaptureObservations, CaptureError> {
        self.0.closed_total.fetch_add(1, Ordering::AcqRel);
        if self.0.fail_close {
            Err(CaptureError::BackendInit(
                "injected uncertain finalization".into(),
            ))
        } else {
            Ok(CaptureObservations::default())
        }
    }
}
fn fixture(
    fail_close: bool,
) -> (
    Arc<RuntimeState>,
    PksSessionHandle,
    PksSessionHandle,
    Arc<AtomicUsize>,
    Arc<AtomicUsize>,
) {
    let opened_total = Arc::new(AtomicUsize::new(0));
    let closed_total = Arc::new(AtomicUsize::new(0));
    let backend: Arc<dyn CallbackCaptureBackend> = Arc::new(FaultBackend {
        fail_close,
        opened_total: Arc::clone(&opened_total),
        closed_total: Arc::clone(&closed_total),
    });
    let mut builder = SessionEngineHostBuilder::new(
        PrepareContext::new(crate::SampleSpec::new(
            48_000,
            1,
            crate::SampleFormat::F32Interleaved,
        )),
        8,
        crate::session::SessionStartOptions::default(),
    )
    .unwrap();
    builder
        .set_application_backend(Arc::clone(&backend))
        .set_microphone_backend(backend);
    builder
        .register_polled_audio_endpoint(crate::session::PolledAudioEndpointConfig::default())
        .unwrap();
    let state = Arc::new(RuntimeState::new());
    let engine = state
        .allocate_engine_with_host(builder.build().unwrap(), 1)
        .unwrap();
    let session = state
        .create_session(engine, "synthetic-close-fixture".into())
        .unwrap();
    state.compile_session(engine, session).unwrap();
    (state, engine, session, opened_total, closed_total)
}

#[test]
fn given_native_finalization_failure_when_stop_and_release_repeat_then_both_handles_remain_quarantined(
) {
    let (state, engine, session, opened_total, closed_total) = fixture(true);
    state.start_session(engine, session).unwrap();
    assert_eq!(opened_total.load(Ordering::Acquire), 2);
    assert!(state.stop_session(engine, session).is_err());
    let (outcome, uncertain) = state
        .with_engine(engine, |runtime| runtime.stop_outcome(session))
        .unwrap();
    assert!(uncertain);
    assert_eq!(outcome.capture_finalization_failures_total(), 2);
    for _ in 0..2 {
        assert!(state.stop_session(engine, session).is_err());
        assert!(state.destroy_session(engine, session).is_err());
        assert!(state.release_engine(engine).is_err());
        assert_eq!(state.engine_is_live(engine), Ok(true));
        assert!(state.session_state(engine, session).is_ok());
    }
    // The cached failed outcome does not pretend another native retry occurred.
    assert_eq!(closed_total.load(Ordering::Acquire), 2);
}

#[test]
fn given_confirmed_native_closure_when_engine_is_released_then_handles_are_stale() {
    let (state, engine, session, _, closed_total) = fixture(false);
    state.start_session(engine, session).unwrap();
    state.stop_session(engine, session).unwrap();
    assert!(
        !state
            .with_engine(engine, |runtime| runtime.stop_outcome(session))
            .unwrap()
            .1
    );
    state.destroy_session(engine, session).unwrap();
    state.release_engine(engine).unwrap();
    assert_eq!(state.engine_is_live(engine), Err(AbiError::StaleHandle));
    assert_eq!(closed_total.load(Ordering::Acquire), 2);
}

#[test]
fn given_start_queued_behind_runtime_lock_when_shutdown_wins_then_native_open_never_runs() {
    let (state, engine_handle, session, opened_total, _) = fixture(false);
    let engine = state.engine(engine_handle).unwrap();
    let held_runtime = engine.runtime.lock().unwrap();
    let start_state = Arc::clone(&state);
    let start = std::thread::spawn(move || start_state.start_session(engine_handle, session));
    let deadline_at = Instant::now() + Duration::from_secs(2);
    while engine.state() != PksSessionLifecycleState::Starting {
        assert!(Instant::now() < deadline_at, "start did not queue");
        std::thread::yield_now();
    }
    let shutdown_state = Arc::clone(&state);
    let shutdown = std::thread::spawn(move || shutdown_state.release_engine(engine_handle));
    while !engine.closing_requested.load(Ordering::Acquire) {
        assert!(
            Instant::now() < deadline_at,
            "shutdown did not fence queued work"
        );
        std::thread::yield_now();
    }
    drop(held_runtime);
    assert_eq!(start.join().unwrap(), Err(AbiError::InvalidLifecycleState));
    shutdown.join().unwrap().unwrap();
    assert_eq!(opened_total.load(Ordering::Acquire), 0);
    assert_eq!(
        state.engine_is_live(engine_handle),
        Err(AbiError::StaleHandle)
    );
}

#[test]
fn given_running_native_close_failure_when_session_destroyed_directly_then_failed_owner_remains_visible(
) {
    let (state, engine, session, _, _) = fixture(true);
    state.start_session(engine, session).unwrap();
    assert!(state.destroy_session(engine, session).is_err());
    assert_eq!(
        state.session_state(engine, session),
        Ok(PksSessionLifecycleState::Failed)
    );
    assert!(
        state
            .with_engine(engine, |runtime| runtime.stop_outcome(session))
            .unwrap()
            .1
    );
}
#[test]
fn given_running_native_close_failure_when_engine_destroyed_directly_then_failed_owner_remains_visible(
) {
    let (state, engine, session, _, _) = fixture(true);
    state.start_session(engine, session).unwrap();
    assert!(state.release_engine(engine).is_err());
    assert_eq!(
        state.session_state(engine, session),
        Ok(PksSessionLifecycleState::Failed)
    );
    assert!(
        state
            .with_engine(engine, |runtime| runtime.stop_outcome(session))
            .unwrap()
            .1
    );
}
