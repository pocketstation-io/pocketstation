//! Native processing reports in this file are MOCKED, not native AEC.
//! Session opening, replacement, preparation, rollback and metrics are exercised.

use pocketstation::{
    capture_processing_observations, ActiveCaptureBackend, CallbackCaptureBackend, CaptureDelivery,
    CaptureError, CaptureMode, CaptureObservationHandle, CaptureObservations,
    CaptureProcessingObservationHandle, CaptureProcessingObservations, CaptureProcessingReporter,
    PreparedCaptureBackend, Session, Source, SourceId,
};
use std::sync::{
    atomic::{AtomicU64, Ordering},
    Arc, Mutex,
};

static NEXT_SOURCE_BASE_ID: AtomicU64 = AtomicU64::new(100);

#[derive(Default)]
struct BackendState {
    source_base_id: u64,
    opens_total: AtomicU64,
    active_total: AtomicU64,
    reports: Mutex<Vec<CaptureProcessingReporter>>,
    deliveries: Mutex<Vec<Option<CaptureDelivery>>>,
    channels: Mutex<Vec<u8>>,
    requested_modes: Mutex<Vec<CaptureMode>>,
    source_ids: Mutex<Vec<SourceId>>,
    native_reports: Mutex<Vec<Option<pocketstation::NativeAecRouteReporter>>>,
}

struct ProcessingBackend {
    facts: Mutex<Option<CaptureProcessingObservations>>,
    state: Arc<BackendState>,
}

impl ProcessingBackend {
    fn new(facts: Option<CaptureProcessingObservations>) -> Arc<Self> {
        Arc::new(Self {
            facts: Mutex::new(facts),
            state: Arc::new(BackendState {
                source_base_id: NEXT_SOURCE_BASE_ID.fetch_add(100, Ordering::SeqCst),
                ..BackendState::default()
            }),
        })
    }

    fn report(&self, open_index: usize, facts: CaptureProcessingObservations) {
        self.state.reports.lock().unwrap()[open_index].replace(facts);
    }

    #[cfg(feature = "aec")]
    fn next_open(&self, facts: CaptureProcessingObservations) {
        *self.facts.lock().unwrap() = Some(facts);
    }

    // Prepared PCM is sent only from the test's control thread.
    fn frame(&self, open_index: usize, sequence: u64, value: f32) -> pocketstation::AudioFrame {
        use pocketstation::{AudioBufferPool, AudioFrame, SampleFormat, SampleSpec, StreamId};
        let channels = self.state.channels.lock().unwrap()[open_index];
        let pool = AudioBufferPool::new(1, 960 * usize::from(channels));
        let mut buffer = pool.acquire().unwrap();
        buffer.as_mut_slice().fill(value);
        AudioFrame::try_new(
            StreamId::new(10 + open_index as u64),
            self.state.source_ids.lock().unwrap()[open_index],
            sequence,
            1_000_000_000 + sequence * 20_000_000,
            SampleSpec::new(48_000, channels, SampleFormat::F32Interleaved),
            buffer,
        )
        .unwrap()
    }

    fn send(&self, open_index: usize, sequence: u64, value: f32) {
        let frame = self.frame(open_index, sequence, value);
        assert_eq!(
            self.state.deliveries.lock().unwrap()[open_index]
                .as_mut()
                .unwrap()
                .frame_sender
                .try_send(frame),
            pocketstation::CapturedFrameDelivery::Delivered
        );
    }

    fn fail(&self, open_index: usize) {
        use pocketstation::{
            CaptureRuntimeFailure, CaptureRuntimeFailureClass, Platform, SourceGeneration,
            SourceKind, SourceRuntimeEvent, SourceRuntimeEventDelivery, StableSourceId,
        };
        let event = SourceRuntimeEvent::BackendFailure {
            stable_id: StableSourceId::new(
                Platform::Unknown,
                SourceKind::InputDevice,
                "processing-fixture",
            ),
            generation: SourceGeneration::INITIAL,
            failure: CaptureRuntimeFailure {
                operation: "test capture failure",
                error_class: CaptureRuntimeFailureClass::SourceInstanceExited,
            },
        };
        assert_eq!(
            self.state.deliveries.lock().unwrap()[open_index]
                .as_ref()
                .unwrap()
                .runtime_event_sender
                .try_send(event),
            SourceRuntimeEventDelivery::Enqueued
        );
    }
}

struct PreparedBackend {
    channels: u8,
    facts: Option<CaptureProcessingObservations>,
    state: Arc<BackendState>,
    native: Option<native_selection::RouteFacts>,
}

impl CallbackCaptureBackend for ProcessingBackend {
    fn prepare(&self, mode: CaptureMode) -> Result<Box<dyn PreparedCaptureBackend>, CaptureError> {
        self.state
            .requested_modes
            .lock()
            .unwrap()
            .push(mode.clone());
        Ok(Box::new(PreparedBackend {
            channels: if matches!(mode, CaptureMode::InputDevice(_)) {
                1
            } else {
                2
            },
            facts: *self.facts.lock().unwrap(),
            state: Arc::clone(&self.state),
            native: None,
        }))
    }
}

struct ActiveBackend {
    source_id: SourceId,
    processing: Option<CaptureProcessingObservationHandle>,
    state: Arc<BackendState>,
    open_index: usize,
    native: Option<pocketstation::NativeAecRouteHandle>,
}

impl PreparedCaptureBackend for PreparedBackend {
    fn open(
        self: Box<Self>,
        delivery: CaptureDelivery,
    ) -> Result<Box<dyn ActiveCaptureBackend>, CaptureError> {
        let open_count = self.state.opens_total.fetch_add(1, Ordering::SeqCst);
        let source_id = self.native.as_ref().map_or_else(
            || SourceId::new(open_count + self.state.source_base_id),
            |native| native.microphone.source_id(),
        );
        let (native_reporter, native) = self.native.map_or((None, None), |facts| {
            let route = pocketstation::NativeAecRoute::new(
                facts.source_id.unwrap_or(source_id),
                facts.microphone,
                facts.playback_device,
                facts.enabled,
                facts.bypassed,
                facts.reference_confirmed,
            );
            let (reporter, handle) = pocketstation::native_aec_route(route);
            (Some(reporter), Some(handle))
        });
        let processing = self.facts.map(|facts| {
            let (reporter, handle) = capture_processing_observations();
            reporter.replace(facts);
            self.state.reports.lock().unwrap().push(reporter);
            handle
        });
        self.state.active_total.fetch_add(1, Ordering::SeqCst);
        let open_index = self.state.deliveries.lock().unwrap().len();
        self.state.deliveries.lock().unwrap().push(Some(delivery));
        self.state.channels.lock().unwrap().push(self.channels);
        self.state.source_ids.lock().unwrap().push(source_id);
        self.state
            .native_reports
            .lock()
            .unwrap()
            .push(native_reporter);
        Ok(Box::new(ActiveBackend {
            source_id,
            processing,
            state: Arc::clone(&self.state),
            open_index,
            native,
        }))
    }
}

impl ActiveCaptureBackend for ActiveBackend {
    fn source_id(&self) -> SourceId {
        self.source_id
    }
    fn processing_observation_handle(&self) -> Option<CaptureProcessingObservationHandle> {
        self.processing.clone()
    }
    fn native_aec_route_handle(&self) -> Option<pocketstation::NativeAecRouteHandle> {
        self.native.clone()
    }
    fn observation_handle(&self) -> CaptureObservationHandle {
        CaptureObservationHandle::default()
    }
    fn observations(&self) -> CaptureObservations {
        CaptureObservations::default()
    }
    fn stop_and_join(self: Box<Self>) -> Result<CaptureObservations, CaptureError> {
        Ok(CaptureObservations::default())
    }
}

// Test-only ownership accounting. This executes on Session's control thread.
impl Drop for ActiveBackend {
    fn drop(&mut self) {
        self.state.deliveries.lock().unwrap()[self.open_index].take();
        self.state.active_total.fetch_sub(1, Ordering::SeqCst);
    }
}

fn facts(supported: Option<bool>, processed: Option<bool>) -> CaptureProcessingObservations {
    CaptureProcessingObservations {
        native_aec_supported: supported,
        echo_processed: processed,
        raw_audio_available: None,
    }
}

fn session(application: &Arc<ProcessingBackend>, microphone: &Arc<ProcessingBackend>) -> Session {
    Session::builder()
        .sample_spec(pocketstation::SampleSpec::new(
            48_000,
            2,
            pocketstation::SampleFormat::F32Interleaved,
        ))
        .audio_frame_duration(pocketstation::AudioFrameDuration::Ms20)
        .capture_backends(application.clone(), microphone.clone())
        .build()
}

#[test]
fn given_backend_without_processing_query_when_session_opens_then_input_is_unknown() {
    let app = ProcessingBackend::new(None);
    let mic = ProcessingBackend::new(None);
    let session = session(&app, &mic);
    let microphone = session.capture(Source::microphone_default()).unwrap();
    microphone.send(session.polled_audio().unwrap()).unwrap();
    let mut running = session.start().unwrap();
    let snapshot = running.metrics_snapshot().unwrap();
    let opened = snapshot.source_processing(0).unwrap();
    assert_eq!(opened.stem_id, microphone.id());
    assert_eq!(
        opened.attached_source_id,
        Some(SourceId::new(mic.state.source_base_id))
    );
    assert_eq!(opened.processing, CaptureProcessingObservations::default());
    assert_eq!(snapshot.source_processing_count(), 1);
    assert!(snapshot.source_processing(1).is_none());
    assert_eq!(app.state.opens_total.load(Ordering::SeqCst), 0);
    assert!(running.stop().is_success());
    assert_eq!(mic.state.active_total.load(Ordering::SeqCst), 0);
}

mod native_selection {
    use super::*;
    use pocketstation::{
        DeviceId, DeviceSelector, NativeAecRequest, NativePlaybackReference, Platform, SourceKind,
        StableSourceId,
    };

    // MOCKED native reports; actual Session media, selection and teardown.
    #[derive(Clone)]
    pub(super) struct RouteFacts {
        pub(super) source_id: Option<SourceId>,
        pub(super) microphone: StableSourceId,
        pub(super) playback_device: DeviceId,
        pub(super) enabled: bool,
        pub(super) bypassed: bool,
        pub(super) reference_confirmed: bool,
    }

    impl RouteFacts {
        fn matching() -> Self {
            Self {
                source_id: None,
                microphone: StableSourceId::new(
                    Platform::Unknown,
                    SourceKind::InputDevice,
                    "mic-a",
                ),
                playback_device: DeviceId::new("speaker-a"),
                enabled: true,
                bypassed: false,
                reference_confirmed: true,
            }
        }
    }

    struct NativeBackend {
        capture: Arc<ProcessingBackend>,
        route: Mutex<RouteFacts>,
        requests: Mutex<Vec<DeviceId>>,
    }

    impl NativeBackend {
        fn new(route: RouteFacts) -> Arc<Self> {
            Arc::new(Self {
                capture: ProcessingBackend::new(Some(facts(Some(true), Some(true)))),
                route: Mutex::new(route),
                requests: Mutex::new(Vec::new()),
            })
        }
    }

    impl CallbackCaptureBackend for NativeBackend {
        fn prepare(
            &self,
            mode: CaptureMode,
        ) -> Result<Box<dyn PreparedCaptureBackend>, CaptureError> {
            self.capture.prepare(mode)
        }
        fn prepare_native_aec(
            &self,
            mode: CaptureMode,
            request: &NativeAecRequest,
        ) -> Result<Box<dyn PreparedCaptureBackend>, CaptureError> {
            self.capture
                .state
                .requested_modes
                .lock()
                .unwrap()
                .push(mode);
            self.requests
                .lock()
                .unwrap()
                .push(request.playback_device().clone());
            Ok(Box::new(PreparedBackend {
                channels: 1,
                facts: *self.capture.facts.lock().unwrap(),
                state: Arc::clone(&self.capture.state),
                native: Some(self.route.lock().unwrap().clone()),
            }))
        }
    }

    fn native_session(app: &Arc<ProcessingBackend>, mic: &Arc<NativeBackend>) -> Session {
        Session::builder()
            .sample_spec(pocketstation::SampleSpec::new(
                48_000,
                2,
                pocketstation::SampleFormat::F32Interleaved,
            ))
            .audio_frame_duration(pocketstation::AudioFrameDuration::Ms20)
            .capture_backends(app.clone(), mic.clone())
            .build()
    }

    fn microphone(session: &Session) -> pocketstation::StemHandle {
        let microphone = session
            .capture(Source::microphone(DeviceSelector::id(DeviceId::new(
                "mic-a",
            ))))
            .unwrap();
        session
            .native_aec(
                &microphone,
                NativePlaybackReference::output(DeviceId::new("speaker-a")),
            )
            .unwrap();
        microphone
    }

    #[test]
    fn given_matching_native_route_when_session_delivers_then_same_microphone_and_no_portable_engine_are_used(
    ) {
        let app = ProcessingBackend::new(None);
        let mic = NativeBackend::new(RouteFacts::matching());
        let session = native_session(&app, &mic);
        let application = session.capture(Source::application("Safari")).unwrap();
        let microphone = microphone(&session);
        let endpoint = session.polled_audio().unwrap();
        application.send(endpoint).unwrap();
        microphone.send(endpoint).unwrap();
        let mut running = session.start().unwrap();
        assert_eq!(running.metrics_snapshot().unwrap().operator_count(), 0);
        assert_eq!(mic.capture.state.opens_total.load(Ordering::SeqCst), 1);
        assert_eq!(*mic.requests.lock().unwrap(), [DeviceId::new("speaker-a")]);
        assert_eq!(
            *mic.capture.state.requested_modes.lock().unwrap(),
            [CaptureMode::InputDevice(
                pocketstation::InputDeviceSelector::StableId("mic-a".into())
            )]
        );
        app.send(0, 0, 0.125);
        mic.capture.send(0, 0, 0.5);
        wait_audio(
            &mut running,
            &[
                (application.id(), Some(0.125)),
                (microphone.id(), Some(0.5)),
            ],
            None,
        );
        assert!(running.stop().is_success());
        assert_eq!(mic.capture.state.active_total.load(Ordering::SeqCst), 0);
    }

    #[test]
    fn given_backend_without_native_support_when_requested_then_default_prepare_rejects_without_opening(
    ) {
        let app = ProcessingBackend::new(None);
        let mic = ProcessingBackend::new(None);
        let session = session(&app, &mic);
        let microphone = microphone(&session);
        microphone.send(session.polled_audio().unwrap()).unwrap();
        let error = match session.start() {
            Err(error) => error,
            Ok(mut running) => {
                let _ = running.stop();
                panic!("unsupported native route started")
            }
        };
        assert!(
            error.to_string().to_lowercase().contains("native"),
            "{error}"
        );
        assert_eq!(mic.state.opens_total.load(Ordering::SeqCst), 0);
        assert_eq!(app.state.opens_total.load(Ordering::SeqCst), 0);
    }

    #[test]
    fn given_unusable_or_mismatched_native_route_when_opened_then_session_rejects_and_joins_capture(
    ) {
        let mut wrong_render = RouteFacts::matching();
        wrong_render.playback_device = DeviceId::new("speaker-b");
        let mut wrong_microphone = RouteFacts::matching();
        wrong_microphone.microphone.stable_key = "mic-b".into();
        let mut wrong_source = RouteFacts::matching();
        wrong_source.source_id = Some(SourceId::new(1));
        let mut disabled = RouteFacts::matching();
        disabled.enabled = false;
        let mut bypassed = RouteFacts::matching();
        bypassed.bypassed = true;
        let mut unconfirmed = RouteFacts::matching();
        unconfirmed.reference_confirmed = false;
        for (label, route) in [
            ("wrong render device", wrong_render),
            ("wrong microphone device", wrong_microphone),
            ("wrong source identity", wrong_source),
            ("disabled native effect", disabled),
            ("bypassed native effect", bypassed),
            ("unconfirmed reference", unconfirmed),
        ] {
            let app = ProcessingBackend::new(None);
            let mic = NativeBackend::new(route);
            let session = native_session(&app, &mic);
            let microphone = microphone(&session);
            microphone.send(session.polled_audio().unwrap()).unwrap();
            if let Ok(mut running) = session.start() {
                let _ = running.stop();
                panic!("invalid native route started: {label}")
            }
            assert_eq!(mic.capture.state.opens_total.load(Ordering::SeqCst), 1);
            assert_eq!(mic.capture.state.active_total.load(Ordering::SeqCst), 0);
            assert_eq!(app.state.opens_total.load(Ordering::SeqCst), 0);
        }
    }

    #[test]
    fn given_active_native_route_when_invalidated_then_queued_microphone_is_not_delivered_and_app_continues(
    ) {
        use std::time::{Duration, Instant};
        let app = ProcessingBackend::new(None);
        let mic = NativeBackend::new(RouteFacts::matching());
        let session = native_session(&app, &mic);
        let application = session.capture(Source::application("Safari")).unwrap();
        let microphone = microphone(&session);
        let endpoint = session.polled_audio().unwrap();
        application.send(endpoint).unwrap();
        microphone.send(endpoint).unwrap();
        let mut running = session.start().unwrap();
        app.send(0, 0, 0.125);
        mic.capture.send(0, 0, 0.25);
        wait_audio(
            &mut running,
            &[
                (application.id(), Some(0.125)),
                (microphone.id(), Some(0.25)),
            ],
            None,
        );
        let frame = mic.capture.frame(0, 1, 0.875);
        // Control-thread barrier keeps teardown from taking the sender before
        // a deliberately stale frame has been queued after invalidation.
        {
            let mut deliveries = mic.capture.state.deliveries.lock().unwrap();
            let reporters = mic.capture.state.native_reports.lock().unwrap();
            reporters[0].as_ref().unwrap().invalidate();
            reporters[0].as_ref().unwrap().invalidate();
            assert_eq!(
                deliveries[0].as_mut().unwrap().frame_sender.try_send(frame),
                pocketstation::CapturedFrameDelivery::Delivered
            );
        }
        let deadline = Instant::now() + Duration::from_secs(2);
        while running
            .metrics_snapshot()
            .unwrap()
            .source_processing(1)
            .unwrap()
            .attached_source_id
            .is_some()
        {
            assert!(
                Instant::now() < deadline,
                "invalidated native route stayed attached"
            );
            std::thread::sleep(Duration::from_millis(1));
        }
        app.send(0, 1, 0.375);
        wait_audio(
            &mut running,
            &[(application.id(), Some(0.375))],
            Some(microphone.id()),
        );
        assert!(!running.stop().is_success());
        assert_eq!(mic.capture.state.active_total.load(Ordering::SeqCst), 0);
    }

    #[test]
    fn given_native_route_when_replacement_render_mismatches_then_old_route_remains_usable() {
        let app = ProcessingBackend::new(None);
        let mic = NativeBackend::new(RouteFacts::matching());
        let session = native_session(&app, &mic);
        let application = session.capture(Source::application("Safari")).unwrap();
        let microphone = microphone(&session);
        let endpoint = session.polled_audio().unwrap();
        application.send(endpoint).unwrap();
        microphone.send(endpoint).unwrap();
        let mut running = session.start().unwrap();
        let before = *running
            .metrics_snapshot()
            .unwrap()
            .source_processing(1)
            .unwrap();
        mic.route.lock().unwrap().playback_device = DeviceId::new("speaker-b");
        assert!(running
            .replace_microphone_source(microphone.id(), DeviceSelector::id(DeviceId::new("mic-a")))
            .is_err());
        assert_eq!(
            *running
                .metrics_snapshot()
                .unwrap()
                .source_processing(1)
                .unwrap(),
            before
        );
        assert_eq!(mic.capture.state.active_total.load(Ordering::SeqCst), 1);
        app.send(0, 0, 0.125);
        mic.capture.send(0, 0, 0.25);
        wait_audio(
            &mut running,
            &[
                (application.id(), Some(0.125)),
                (microphone.id(), Some(0.25)),
            ],
            None,
        );
        assert_eq!(mic.requests.lock().unwrap().len(), 2);
        assert!(running.stop().is_success());
    }
}

#[test]
fn given_processing_report_when_updated_then_session_preserves_support_and_active_distinction() {
    let app = ProcessingBackend::new(None);
    let mic = ProcessingBackend::new(Some(facts(Some(true), Some(false))));
    let session = session(&app, &mic);
    let microphone = session.capture(Source::microphone_default()).unwrap();
    microphone.send(session.polled_audio().unwrap()).unwrap();
    let mut running = session.start().unwrap();
    assert_eq!(
        running
            .metrics_snapshot()
            .unwrap()
            .source_processing(0)
            .unwrap()
            .processing,
        facts(Some(true), Some(false))
    );
    mic.report(0, facts(Some(true), Some(true)));
    assert_eq!(
        running
            .metrics_snapshot()
            .unwrap()
            .source_processing(0)
            .unwrap()
            .processing,
        facts(Some(true), Some(true))
    );
    mic.report(0, CaptureProcessingObservations::default());
    assert_eq!(
        running
            .metrics_snapshot()
            .unwrap()
            .source_processing(0)
            .unwrap()
            .processing,
        CaptureProcessingObservations::default()
    );
    assert!(running.stop().is_success());
}

#[test]
fn given_reopened_microphone_when_old_report_changes_then_only_current_open_is_observed() {
    let app = ProcessingBackend::new(None);
    let mic = ProcessingBackend::new(Some(facts(Some(true), Some(false))));
    let session = session(&app, &mic);
    let microphone = session.capture(Source::microphone_default()).unwrap();
    microphone.send(session.polled_audio().unwrap()).unwrap();
    let mut running = session.start().unwrap();
    let before = *running
        .metrics_snapshot()
        .unwrap()
        .source_processing(0)
        .unwrap();
    let replacement = running
        .reopen_microphone_source(microphone.id(), pocketstation::DeviceSelector::default())
        .unwrap();
    mic.report(0, facts(Some(true), Some(true)));
    let current = *running
        .metrics_snapshot()
        .unwrap()
        .source_processing(0)
        .unwrap();
    assert_ne!(current.attached_source_id, before.attached_source_id);
    assert_eq!(current.attached_source_id, Some(replacement.source_id));
    assert_eq!(current.source_generation, replacement.source_generation);
    assert_eq!(current.discontinuity_epoch, replacement.discontinuity_epoch);
    assert_eq!(current.processing, facts(Some(true), Some(false)));
    mic.report(1, facts(Some(true), Some(true)));
    assert_eq!(
        running
            .metrics_snapshot()
            .unwrap()
            .source_processing(0)
            .unwrap()
            .processing
            .echo_processed,
        Some(true)
    );
    assert!(running.stop().is_success());
    assert_eq!(mic.state.active_total.load(Ordering::SeqCst), 0);
}

#[test]
fn given_application_only_capture_when_started_then_microphone_is_never_opened() {
    let app = ProcessingBackend::new(None);
    let mic = ProcessingBackend::new(Some(facts(Some(true), Some(true))));
    let session = session(&app, &mic);
    let application = session.capture(Source::application("Safari")).unwrap();
    application.send(session.polled_audio().unwrap()).unwrap();
    let mut running = session.start().unwrap();
    assert_eq!(
        running
            .metrics_snapshot()
            .unwrap()
            .source_processing_count(),
        1
    );
    assert_eq!(mic.state.opens_total.load(Ordering::SeqCst), 0);
    assert!(running.stop().is_success());
}

#[cfg(feature = "aec")]
#[test]
fn given_opened_processed_microphone_when_portable_aec_requested_then_start_rolls_back() {
    use pocketstation::{EchoAudioInput, PlaybackReference};
    let app = ProcessingBackend::new(None);
    let mic = ProcessingBackend::new(Some(facts(Some(true), Some(true))));
    let session = session(&app, &mic);
    let application = session.capture(Source::application("Safari")).unwrap();
    let microphone = session.capture(Source::microphone_default()).unwrap();
    let processed = session
        .echo_cancel(
            EchoAudioInput::from(&microphone).with_echo_processing(false),
            PlaybackReference::selected_application(&application),
        )
        .unwrap();
    processed
        .audio()
        .send(session.polled_audio().unwrap())
        .unwrap();
    let error = match session.start() {
        Err(error) => error,
        Ok(mut running) => {
            let _ = running.stop();
            panic!("processed native input admitted to a second AEC");
        }
    };
    assert!(
        error
            .to_string()
            .contains("already supplies echo-processed audio"),
        "{error}"
    );
    assert_eq!(
        processed.observations().processed_microphone_frames_total,
        0
    );
    assert_eq!(app.state.active_total.load(Ordering::SeqCst), 0);
    assert_eq!(mic.state.active_total.load(Ordering::SeqCst), 0);
}

#[cfg(feature = "aec")]
#[test]
fn given_supported_inactive_aec_when_explicit_portable_aec_requested_then_preparation_succeeds() {
    use pocketstation::PlaybackReference;
    let app = ProcessingBackend::new(Some(facts(Some(true), Some(true))));
    let mic = ProcessingBackend::new(Some(facts(Some(true), Some(false))));
    let session = session(&app, &mic);
    let application = session.capture(Source::application("Safari")).unwrap();
    let microphone = session.capture(Source::microphone_default()).unwrap();
    let processed = session
        .echo_cancel(
            &microphone,
            PlaybackReference::selected_application(&application),
        )
        .unwrap();
    processed
        .audio()
        .send(session.polled_audio().unwrap())
        .unwrap();
    let mut running = session.start().unwrap();
    assert_eq!(
        running
            .metrics_snapshot()
            .unwrap()
            .source_processing(1)
            .unwrap()
            .processing
            .echo_processed,
        Some(false)
    );
    assert!(running.stop().is_success());
}

fn wait_audio(
    running: &mut pocketstation::RunningSession,
    wanted: &[(pocketstation::StemId, Option<f32>)],
    forbidden: Option<pocketstation::StemId>,
) {
    use std::time::{Duration, Instant};
    let deadline = Instant::now() + Duration::from_secs(2);
    let mut seen = vec![false; wanted.len()];
    while seen.iter().any(|value| !value) {
        if let Ok(batch) = running.try_poll_audio() {
            for index in 0..batch.len() {
                let frame = batch.frame(index).unwrap();
                assert_ne!(Some(frame.lineage().stem_id()), forbidden);
                for (index, (stem_id, value)) in wanted.iter().enumerate() {
                    if frame.lineage().stem_id() == *stem_id {
                        assert_eq!(frame.samples().len(), 960 * usize::from(frame.channels()));
                        if let Some(value) = value {
                            assert!(frame.samples().iter().all(|sample| sample == value));
                        }
                        seen[index] = true;
                    }
                }
            }
        }
        assert!(Instant::now() < deadline, "missing audio stems: {seen:?}");
        if seen.iter().any(|value| !value) {
            std::thread::sleep(Duration::from_millis(1));
        }
    }
}

#[cfg(feature = "aec")]
#[test]
fn given_running_portable_aec_when_native_processing_toggles_then_stage_fails_and_raw_stems_continue(
) {
    use pocketstation::PlaybackReference;
    use std::time::{Duration, Instant};
    let app = ProcessingBackend::new(None);
    let mic = ProcessingBackend::new(Some(facts(Some(true), Some(false))));
    let session = session(&app, &mic);
    let application = session.capture(Source::application("Safari")).unwrap();
    let microphone = session.capture(Source::microphone_default()).unwrap();
    let processed = session
        .echo_cancel(
            &microphone,
            PlaybackReference::selected_application(&application),
        )
        .unwrap();
    let endpoint = session.polled_audio().unwrap();
    application.send(endpoint).unwrap();
    microphone.send(endpoint).unwrap();
    processed.audio().send(endpoint).unwrap();
    let mut running = session.start().unwrap();
    app.send(0, 0, 0.125);
    mic.send(0, 0, 0.0625);
    wait_audio(
        &mut running,
        &[
            (application.id(), Some(0.125)),
            (microphone.id(), Some(0.0625)),
            (processed.audio().id(), None),
        ],
        None,
    );
    let before = processed.observations().processed_microphone_frames_total;
    assert!(before > 0);
    mic.report(0, facts(Some(true), Some(true)));
    mic.report(0, facts(Some(true), Some(false)));
    app.send(0, 1, 0.25);
    mic.send(0, 1, 0.5);
    let deadline = Instant::now() + Duration::from_secs(2);
    while running
        .metrics_snapshot()
        .unwrap()
        .operator(0)
        .unwrap()
        .worker
        .process_failure_total
        == 0
    {
        assert!(
            Instant::now() < deadline,
            "portable worker did not retain revocation"
        );
        std::thread::sleep(Duration::from_millis(1));
    }
    wait_audio(
        &mut running,
        &[(application.id(), Some(0.25)), (microphone.id(), Some(0.5))],
        Some(processed.audio().id()),
    );
    assert_eq!(
        processed.observations().processed_microphone_frames_total,
        before
    );
    assert!(!running.stop().is_success());
    assert_eq!(app.state.active_total.load(Ordering::SeqCst), 0);
    assert_eq!(mic.state.active_total.load(Ordering::SeqCst), 0);
}

#[cfg(feature = "aec")]
#[test]
fn given_portable_aec_when_replacement_is_processed_then_old_source_and_application_remain_usable()
{
    use pocketstation::{DeviceSelector, PlaybackReference};
    let app = ProcessingBackend::new(None);
    let mic = ProcessingBackend::new(Some(facts(Some(true), Some(false))));
    let session = session(&app, &mic);
    let application = session.capture(Source::application("Safari")).unwrap();
    let microphone = session.capture(Source::microphone_default()).unwrap();
    let processed = session
        .echo_cancel(
            &microphone,
            PlaybackReference::selected_application(&application),
        )
        .unwrap();
    let endpoint = session.polled_audio().unwrap();
    application.send(endpoint).unwrap();
    microphone.send(endpoint).unwrap();
    processed.audio().send(endpoint).unwrap();
    let mut running = session.start().unwrap();
    let before = *running
        .metrics_snapshot()
        .unwrap()
        .source_processing(1)
        .unwrap();
    mic.next_open(facts(Some(true), Some(true)));
    let error = running
        .replace_microphone_source(microphone.id(), DeviceSelector::default())
        .unwrap_err();
    assert!(error.to_string().contains("processed"), "{error}");
    assert_eq!(
        *running
            .metrics_snapshot()
            .unwrap()
            .source_processing(1)
            .unwrap(),
        before
    );
    assert_eq!(mic.state.opens_total.load(Ordering::SeqCst), 2);
    assert_eq!(mic.state.active_total.load(Ordering::SeqCst), 1);
    app.send(0, 0, 0.25);
    mic.send(0, 0, 0.125);
    wait_audio(
        &mut running,
        &[
            (application.id(), Some(0.25)),
            (microphone.id(), Some(0.125)),
            (processed.audio().id(), None),
        ],
        None,
    );
    assert!(running.stop().is_success());
    assert_eq!(mic.state.active_total.load(Ordering::SeqCst), 0);
}

#[test]
fn given_backend_failure_when_capture_detaches_then_processing_is_unknown_and_application_continues(
) {
    use std::time::{Duration, Instant};
    let app = ProcessingBackend::new(None);
    let mic = ProcessingBackend::new(Some(facts(Some(true), Some(true))));
    let session = session(&app, &mic);
    let application = session.capture(Source::application("Safari")).unwrap();
    let microphone = session.capture(Source::microphone_default()).unwrap();
    let endpoint = session.polled_audio().unwrap();
    application.send(endpoint).unwrap();
    microphone.send(endpoint).unwrap();
    let mut running = session.start().unwrap();
    let before = *running
        .metrics_snapshot()
        .unwrap()
        .source_processing(1)
        .unwrap();
    mic.fail(0);
    let deadline = Instant::now() + Duration::from_secs(2);
    loop {
        let current = *running
            .metrics_snapshot()
            .unwrap()
            .source_processing(1)
            .unwrap();
        if current.attached_source_id.is_none() {
            assert!(current.source_generation > before.source_generation);
            assert_eq!(current.processing, CaptureProcessingObservations::default());
            break;
        }
        assert!(
            Instant::now() < deadline,
            "failed capture observations stayed attached"
        );
        std::thread::sleep(Duration::from_millis(1));
    }
    app.send(0, 0, 0.375);
    wait_audio(
        &mut running,
        &[(application.id(), Some(0.375))],
        Some(microphone.id()),
    );
    assert!(!running.stop().is_success());
    assert_eq!(mic.state.active_total.load(Ordering::SeqCst), 0);
}
