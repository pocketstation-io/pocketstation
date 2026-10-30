use super::super::source::{SourceOutputIdentity, SourceSessionContext};
use super::*;
use crate::capture::{CaptureObservationCounters, CapturedFrameDelivery};
use crate::graph::SignalContinuityTracker;
use crate::{AudioBufferPool, AudioFrame, SampleFormat, SessionId, SourceId, StreamId};

fn driver() -> ProcessSetDriver {
    ProcessSetDriver {
        pending: Arc::new(Mutex::new(BTreeMap::new())),
        duration: AudioFrameDuration::default(),
        sample_spec: SampleSpec::new(48_000, 2, SampleFormat::F32Interleaved),
        context: Some(SourceSessionContext {
            session_id: SessionId(1),
            source_id: SourceId(2),
            outputs: vec![SourceOutputIdentity {
                output_port: OUTPUT.into(),
                stream_id: StreamId(3),
            }],
        }),
        failure: None,
        cleanup: Arc::default(),
        last_sequence: None,
        discontinuity_epoch: 0,
        last_loss_counts: [0; 5],
        capture: None,
        capture_observations: None,
        frames: None,
        events: None,
    }
}
fn frame(pool: &Arc<AudioBufferPool>, sequence: u64) -> AudioFrame {
    let mut buffer = pool.acquire().unwrap();
    buffer.try_copy_from_slice(&[0.0; 1920]).unwrap();
    AudioFrame::try_new(
        StreamId(9),
        SourceId(8),
        sequence,
        sequence * 20_000_000,
        SampleSpec::new(48_000, 2, SampleFormat::F32Interleaved),
        buffer,
    )
    .unwrap()
}
#[test]
fn given_bounded_queue_drop_when_source_continues_then_sequence_and_discontinuity_are_explicit() {
    let pool = AudioBufferPool::new(4, 1920);
    let (mut sender, receiver) = crate::capture::captured_frame_stream_with_start_gate(
        1,
        crate::capture::CaptureDeliveryStartGate::opened(),
    )
    .unwrap();
    let mut source = driver();
    source.frames = Some(receiver);
    assert_eq!(
        sender.try_send(frame(&pool, 0)),
        CapturedFrameDelivery::Delivered
    );
    assert_eq!(
        sender.try_send(frame(&pool, 1)),
        CapturedFrameDelivery::DroppedNewest
    );
    let first = source.frames.as_mut().unwrap().try_next().unwrap();
    let first = source.emission(first).unwrap().unwrap();
    assert_eq!(
        sender.try_send(frame(&pool, 2)),
        CapturedFrameDelivery::Delivered
    );
    let next = source.frames.as_mut().unwrap().try_next().unwrap();
    let next = source.emission(next).unwrap().unwrap();
    let mut tracker = SignalContinuityTracker::default();
    tracker.observe(&first.envelope).unwrap();
    assert!(
        tracker
            .observe(&next.envelope)
            .unwrap()
            .discontinuity_observed
    );
    assert_eq!(next.envelope.lineage().unwrap().sequence_number(), 2);
}
#[test]
fn given_native_ring_loss_with_contiguous_frames_when_emitted_then_existing_tracker_observes_gap() {
    let pool = AudioBufferPool::new(4, 1920);
    let mut source = driver();
    let counters = CaptureObservationCounters::default();
    source.capture_observations = Some(counters.observation_handle());
    let first = source.emission(frame(&pool, 0)).unwrap().unwrap();
    counters.observe_dispatch_queue_full_frames(1);
    let next = source.emission(frame(&pool, 1)).unwrap().unwrap();
    let mut tracker = SignalContinuityTracker::default();
    tracker.observe(&first.envelope).unwrap();
    assert!(
        tracker
            .observe(&next.envelope)
            .unwrap()
            .discontinuity_observed
    );
    assert_eq!(next.envelope.lineage().unwrap().sequence_number(), 1);
}
#[test]
fn given_actual_cleanup_receipt_when_startup_fails_then_uncertainty_wins_without_false_clean_hold()
{
    let factory = ProcessSetFactory::new(
        SampleSpec::new(48_000, 1, SampleFormat::F32Interleaved),
        AudioFrameDuration::default(),
    )
    .unwrap();
    let ordinary = crate::SessionStartError::new(
        crate::SessionStartErrorCode::RuntimePrepareFailed,
        "original startup failure",
    );
    assert_eq!(
        ordinary
            .clone()
            .with_process_set_cleanup(Some(&factory))
            .code(),
        crate::SessionStartErrorCode::RuntimePrepareFailed
    );
    factory
        .cleanup
        .record(Some(Box::new(crate::CaptureError::BackendStatus {
            operation: "closing native tap",
            status_code: -50,
        })));
    let error = ordinary.with_process_set_cleanup(Some(&factory));
    assert_eq!(
        error.code(),
        crate::SessionStartErrorCode::CaptureCleanupUnconfirmed
    );
    assert!(error.message().contains("original startup failure"));
    assert!(error.message().contains("-50"));
    assert_eq!(error.rollback_failures_total(), 1);
}

struct EndedFactory {
    manifest: SourceManifest,
    source: Mutex<Option<ProcessSetDriver>>,
    close_fails: bool,
}
struct EndedDriver {
    source: ProcessSetDriver,
    close_fails: bool,
}
impl SourceFactory for EndedFactory {
    fn manifest(&self) -> &SourceManifest {
        &self.manifest
    }
    fn validate_config(&self, _: &SourceConfiguration) -> Result<(), ConfigError> {
        Ok(())
    }
    fn create(&self, _: &SourceConfiguration) -> Result<Box<dyn SourceDriver>, SourceDriverError> {
        Ok(Box::new(EndedDriver {
            source: self.source.lock().unwrap().take().unwrap(),
            close_fails: self.close_fails,
        }))
    }
}
impl SourceDriver for EndedDriver {
    fn prepare(&mut self, context: &SourcePrepareContext) -> Result<(), SourceDriverError> {
        self.source.context = context.session.clone();
        Ok(())
    }
    fn next(
        &mut self,
        cancellation: &SourceCancellation,
    ) -> Result<Option<SourceEmission>, SourceDriverError> {
        self.source.next(cancellation)
    }
    fn close(&mut self) -> Result<(), SourceDriverError> {
        if self.close_fails {
            Err(self.source.fail(crate::CaptureError::BackendStatus {
                operation: "closing exact capture",
                status_code: -50,
            }))
        } else {
            self.source.close()
        }
    }
    fn take_failure_cause(&mut self) -> Option<Box<dyn std::error::Error + Send + Sync>> {
        self.source.take_failure_cause()
    }
}
#[test]
fn given_original_processes_end_when_source_runtime_joins_then_read_failure_and_close_uncertainty_are_distinct(
) {
    use super::super::source::{SourceOutputBranchSpec, SourceRegistry};
    for close_fails in [false, true] {
        let mut source = driver();
        let (sender, events) = crate::capture::source_runtime_event_channel(2).unwrap();
        source.events = Some(events);
        let _ = sender.try_send(crate::SourceRuntimeEvent::SourceUnavailable {
            stable_id: crate::StableSourceId::new(
                crate::Platform::Macos,
                crate::SourceKind::Application,
                "exact-test-scope",
            ),
            generation: crate::SourceGeneration::INITIAL,
            recovery_requirement:
                crate::SourceRecoveryRequirement::ExplicitRediscoveryAndNewSession,
            failure: crate::CaptureRuntimeFailure {
                operation: "observe selected instance lifetime",
                error_class: crate::CaptureRuntimeFailureClass::SourceInstanceExited,
            },
        });
        let production = ProcessSetFactory::new(
            SampleSpec::new(48_000, 1, SampleFormat::F32Interleaved),
            AudioFrameDuration::default(),
        )
        .unwrap();
        let manifest = production.manifest().clone();
        let source_type = manifest.source_type_id().clone();
        let mut registry = SourceRegistry::default();
        registry
            .register(Arc::new(EndedFactory {
                manifest,
                source: Mutex::new(Some(source)),
                close_fails,
            }))
            .unwrap();
        let branches = [SourceOutputBranchSpec {
            output_port: OUTPUT.into(),
            branch: crate::runtime::TypedEdgeBranchSpec {
                capacity_signals: 2,
                route_settings: crate::graph::RouteSettings::realtime_audio()
                    .with_max_payload_bytes(1920 * 4),
            },
        }];
        let (prepared, _receivers) = registry
            .prepare_session(
                &source_type,
                &SourceConfiguration::default(),
                &branches,
                SourceSessionContext {
                    session_id: SessionId(1),
                    source_id: SourceId(2),
                    outputs: vec![SourceOutputIdentity {
                        output_port: OUTPUT.into(),
                        stream_id: StreamId(3),
                    }],
                },
            )
            .unwrap();
        let mut running = prepared.start().unwrap();
        let observations = running.observations();
        let error = running.join().unwrap_err();
        assert_eq!(error.capture_close_unconfirmed(), close_fails);
        if !close_fails {
            match error {
                super::super::source::SourceRuntimeError::DriverCause {
                    operation, cause, ..
                } => {
                    assert_eq!(operation, "read");
                    assert!(matches!(
                        cause.downcast_ref::<crate::CaptureError>(),
                        Some(crate::CaptureError::SourceUnavailable { .. })
                    ));
                }
                other => panic!("unexpected error: {other}"),
            }
        }
        assert!(observations.snapshot().joined);
        assert_eq!(observations.snapshot().failure_total, 1);
    }
}

struct StartupFactory {
    manifest: SourceManifest,
    cleanup: Arc<CaptureCleanupReceipt>,
    closes: Arc<std::sync::atomic::AtomicU64>,
    prepare_fails: bool,
    close_fails: bool,
}
struct StartupDriver {
    cleanup: Arc<CaptureCleanupReceipt>,
    closes: Arc<std::sync::atomic::AtomicU64>,
    prepare_fails: bool,
    close_fails: bool,
}
impl SourceFactory for StartupFactory {
    fn manifest(&self) -> &SourceManifest {
        &self.manifest
    }
    fn validate_config(&self, _: &SourceConfiguration) -> Result<(), ConfigError> {
        Ok(())
    }
    fn create(&self, _: &SourceConfiguration) -> Result<Box<dyn SourceDriver>, SourceDriverError> {
        Ok(Box::new(StartupDriver {
            cleanup: Arc::clone(&self.cleanup),
            closes: Arc::clone(&self.closes),
            prepare_fails: self.prepare_fails,
            close_fails: self.close_fails,
        }))
    }
}
impl SourceDriver for StartupDriver {
    fn prepare(&mut self, _: &SourcePrepareContext) -> Result<(), SourceDriverError> {
        if self.prepare_fails {
            Err(SourceDriverError::Failed(
                "second source refused preparation".into(),
            ))
        } else {
            Ok(())
        }
    }
    fn next(
        &mut self,
        _: &SourceCancellation,
    ) -> Result<Option<SourceEmission>, SourceDriverError> {
        Ok(None)
    }
    fn close(&mut self) -> Result<(), SourceDriverError> {
        self.closes.fetch_add(1, Ordering::Relaxed);
        if self.close_fails {
            // Inject only the native leaf's error; exercise actual prepared
            // owner Drop, Session rollback and ordinary facade precedence.
            self.cleanup
                .record(Some(Box::new(crate::CaptureError::BackendStatus {
                    operation: "closing owned capture",
                    status_code: -50,
                })));
            Err(SourceDriverError::Failed(
                "owned capture close failed".into(),
            ))
        } else {
            Ok(())
        }
    }
}
#[test]
fn given_second_source_prepare_fails_when_first_cleanup_is_unconfirmed_then_public_start_retains_scope(
) {
    for close_fails in [false, true] {
        let session = crate::Session::new();
        let production = ProcessSetFactory::new(
            SampleSpec::new(48_000, 1, SampleFormat::F32Interleaved),
            AudioFrameDuration::default(),
        )
        .unwrap();
        *session.process_set_factory.lock().unwrap() = Some(Arc::clone(&production));
        let closes = Arc::new(std::sync::atomic::AtomicU64::new(0));
        let endpoint = session.polled_audio().unwrap();
        for (name, prepare_fails) in [("first", false), ("second", true)] {
            let id = SourceTypeId::new(format!("io.example.source.{name}.v1")).unwrap();
            let manifest = SourceManifest::new(
                id.clone(),
                1,
                1,
                production.manifest().outputs().to_vec(),
                ExecutionPartition::BlockingWorker,
                ExecutionSafety::AllocationAllowed,
            )
            .unwrap();
            session
                .register_source(Arc::new(StartupFactory {
                    manifest,
                    cleanup: Arc::clone(&production.cleanup),
                    closes: Arc::clone(&closes),
                    prepare_fails,
                    close_fails: !prepare_fails && close_fails,
                }))
                .unwrap();
            session
                .source(id, SourceConfiguration::default())
                .unwrap()
                .output(OUTPUT)
                .unwrap()
                .send(endpoint)
                .unwrap();
        }
        let error = match session.start() {
            Err(error) => error,
            Ok(mut running) => {
                let _ = running.stop();
                panic!("second prepare unexpectedly succeeded")
            }
        };
        assert!(
            closes.load(Ordering::Relaxed) > 0,
            "first prepared owner was not closed"
        );
        assert!(error
            .message()
            .contains("second source refused preparation"));
        assert_eq!(
            error.code(),
            if close_fails {
                crate::SessionStartErrorCode::CaptureCleanupUnconfirmed
            } else {
                crate::SessionStartErrorCode::RuntimePrepareFailed
            }
        );
    }
}
