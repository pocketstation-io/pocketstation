use std::sync::atomic::{AtomicU32, AtomicU64, Ordering};
use std::sync::Arc;

use crate::frame::{
    ClockDomainId, FrameLineage, FrameLineageError, LineagedAudioFrame, SessionId, SourceId, StemId,
};

use crate::capture::frame_stream::{
    captured_frame_stream_with_start_gate, CaptureDeliveryStartGate,
};
use crate::capture::{
    source_runtime_event_channel, CaptureError, CaptureMode, CaptureObservationHandle,
    CaptureObservations, CapturedFrameObservationHandle, CapturedFrameSender, CapturedFrameStream,
    CapturedFrameStreamStats, PermissionEpoch, SourceGeneration, SourceRuntimeEvent,
    SourceRuntimeEventObservationHandle, SourceRuntimeEventObservations, SourceRuntimeEventReceive,
    SourceRuntimeEventReceiver, SourceRuntimeEventSender,
};

/// Monotonic timestamp domain used by native capture backends.
pub const CAPTURE_MONOTONIC_CLOCK_DOMAIN_ID: ClockDomainId =
    crate::timing::PROCESS_MONOTONIC_CLOCK_DOMAIN_ID;

/// Stable session and stem identity assigned before an exact source is opened.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CaptureLineageSeed {
    session_id: SessionId,
    stem_id: StemId,
}

impl CaptureLineageSeed {
    pub const fn new(session_id: SessionId, stem_id: StemId) -> Self {
        Self {
            session_id,
            stem_id,
        }
    }

    pub const fn session_id(self) -> SessionId {
        self.session_id
    }

    pub const fn stem_id(self) -> StemId {
        self.stem_id
    }
}

/// Authoritative lineage state established only after native capture opens.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CaptureOpenMetadata {
    pub session_id: SessionId,
    pub source_id: SourceId,
    pub stem_id: StemId,
    pub clock_id: ClockDomainId,
    pub source_generation: SourceGeneration,
    pub discontinuity_epoch: u64,
    pub permission_epoch: PermissionEpoch,
}

/// Setup-time request for one bounded callback-oriented capture owner.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CapturePrepareRequest {
    pub mode: CaptureMode,
    pub lineage_seed: CaptureLineageSeed,
    pub frame_capacity_frames: usize,
    pub runtime_event_capacity_events: usize,
}

/// Callback delivery endpoints transferred to a prepared native backend.
///
/// Backends send pool-backed frames through `frame_sender` and publish typed
/// source failures through `runtime_event_sender`. Both sends are bounded and
/// non-blocking. A backend must not retain another unbounded delivery path.
pub struct CaptureDelivery {
    pub frame_sender: CapturedFrameSender,
    pub runtime_event_sender: SourceRuntimeEventSender,
}

/// Platform-neutral prepare/open boundary for callback-oriented capture.
///
/// Implementations validate and reserve setup-time resources in `prepare`.
/// They must not report a successful open until the native callback or worker
/// is ready to deliver into the supplied bounded endpoints.
pub trait CallbackCaptureBackend: Send + Sync {
    fn prepare(&self, mode: CaptureMode) -> Result<Box<dyn PreparedCaptureBackend>, CaptureError>;
}

/// Backend state that has passed validation but has not started delivery.
pub trait PreparedCaptureBackend: Send {
    fn open(
        self: Box<Self>,
        delivery: CaptureDelivery,
    ) -> Result<Box<dyn ActiveCaptureBackend>, CaptureError>;
}

/// Native capture resources owned for exactly one active capture.
///
/// Implementations must make `stop_and_join` bounded and must also stop and
/// reclaim the same resources from `Drop`. Dropping this owner is a
/// control-thread operation, never an audio-callback or realtime operation.
pub trait ActiveCaptureBackend: Send {
    /// Resolved native source identity for every frame emitted by this open.
    ///
    /// CaptureOwner validates this value before constructing authoritative
    /// captured lineage. Session configuration never supplies it.
    fn source_id(&self) -> SourceId;

    fn observation_handle(&self) -> CaptureObservationHandle;

    fn observations(&self) -> CaptureObservations;

    fn stop_and_join(self: Box<Self>) -> Result<CaptureObservations, CaptureError>;
}

/// Prepared capture plus its preallocated delivery endpoints.
///
/// This value is intentionally distinct from `CaptureOwner`: preparation may
/// succeed while native open still fails. Consuming `open` prevents a caller
/// from starting the same prepared backend twice.
pub struct PreparedCapture {
    backend: Box<dyn PreparedCaptureBackend>,
    delivery: CaptureDelivery,
    frame_stream: CapturedFrameStream,
    runtime_event_receiver: SourceRuntimeEventReceiver,
    lineage_seed: CaptureLineageSeed,
}

impl PreparedCapture {
    pub fn open(self) -> Result<CaptureOwner, CaptureError> {
        let frame_observations = self.frame_stream.observation_handle();
        let runtime_event_observations = self.runtime_event_receiver.observation_handle();
        let active_backend = self.backend.open(self.delivery)?;
        let source_id = active_backend.source_id();
        let observation_receipt = CaptureObservationReceipt {
            backend: active_backend.observation_handle(),
            frame_stream: frame_observations,
            runtime_events: runtime_event_observations,
        };
        Ok(CaptureOwner {
            active_backend,
            frame_stream: self.frame_stream,
            runtime_event_receiver: self.runtime_event_receiver,
            observation_receipt,
            open_metadata: CaptureOpenMetadata {
                session_id: self.lineage_seed.session_id(),
                source_id,
                stem_id: self.lineage_seed.stem_id(),
                clock_id: CAPTURE_MONOTONIC_CLOCK_DOMAIN_ID,
                source_generation: SourceGeneration::INITIAL,
                discontinuity_epoch: 0,
                permission_epoch: PermissionEpoch::INITIAL,
            },
            source_generation: AtomicU32::new(SourceGeneration::INITIAL.0),
            discontinuity_epoch: AtomicU64::new(0),
        })
    }
}

/// Aggregate observations from one active capture ownership boundary.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct CaptureOwnerObservations {
    pub backend: CaptureObservations,
    pub frame_stream: CapturedFrameStreamStats,
    pub runtime_events: SourceRuntimeEventObservations,
}

#[derive(Clone, Debug)]
pub struct CaptureObservationReceipt {
    backend: CaptureObservationHandle,
    frame_stream: CapturedFrameObservationHandle,
    runtime_events: SourceRuntimeEventObservationHandle,
}

impl CaptureObservationReceipt {
    pub fn observations(&self) -> CaptureOwnerObservations {
        CaptureOwnerObservations {
            backend: self.backend.observations(),
            frame_stream: self.frame_stream.observations(),
            runtime_events: self.runtime_events.observations(),
        }
    }
}

/// Final observations returned only after backend stop and join complete.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct CaptureStopOutcome {
    pub observations: CaptureOwnerObservations,
}

/// RAII owner for native capture, its bounded frame stream, and runtime events.
///
/// `active_backend` is owned directly, so ordinary `CaptureOwner` destruction
/// invokes the backend's required `Drop` cleanup automatically. Call
/// `stop_and_join` when the caller needs a typed result and final observations.
pub struct CaptureOwner {
    active_backend: Box<dyn ActiveCaptureBackend>,
    frame_stream: CapturedFrameStream,
    runtime_event_receiver: SourceRuntimeEventReceiver,
    observation_receipt: CaptureObservationReceipt,
    open_metadata: CaptureOpenMetadata,
    source_generation: AtomicU32,
    discontinuity_epoch: AtomicU64,
}

impl CaptureOwner {
    pub fn try_recv_runtime_event(&self) -> SourceRuntimeEventReceive {
        let received = self.runtime_event_receiver.try_recv();
        if let SourceRuntimeEventReceive::Event(event) = &received {
            self.observe_runtime_event(event);
        }
        received
    }

    pub fn try_next_lineaged_frame(
        &mut self,
    ) -> Result<Option<LineagedAudioFrame>, FrameLineageError> {
        let Some(frame) = self.frame_stream.try_next() else {
            return Ok(None);
        };
        let channels = usize::from(frame.channels).max(1);
        let samples_per_channel = frame.buffer.len() / channels;
        let duration_ns = u64::try_from(samples_per_channel)
            .unwrap_or(u64::MAX)
            .saturating_mul(1_000_000_000)
            .checked_div(u64::from(frame.sample_rate_hz))
            .unwrap_or(0);
        let metadata = self.open_metadata();
        let lineage = FrameLineage {
            session_id: metadata.session_id,
            source_id: metadata.source_id,
            stem_id: metadata.stem_id,
            clock_id: metadata.clock_id,
            sequence_num: frame.sequence_number,
            timestamp_start_ns: frame.timestamp_ns,
            duration_ns,
            source_generation: metadata.source_generation.0,
            discontinuity_epoch: metadata.discontinuity_epoch,
            permission_epoch: metadata.permission_epoch.0,
        };
        LineagedAudioFrame::new(frame, lineage).map(Some)
    }

    pub fn open_metadata(&self) -> CaptureOpenMetadata {
        CaptureOpenMetadata {
            source_generation: SourceGeneration(self.source_generation.load(Ordering::Acquire)),
            discontinuity_epoch: self.discontinuity_epoch.load(Ordering::Acquire),
            ..self.open_metadata
        }
    }

    #[cfg(any(test, feature = "internal-testing"))]
    pub fn frame_stream_closed(&self) -> bool {
        self.frame_stream.is_closed()
    }

    #[cfg(any(test, feature = "internal-testing"))]
    pub fn observations(&self) -> CaptureOwnerObservations {
        self.observation_receipt.observations()
    }

    pub fn observation_receipt(&self) -> CaptureObservationReceipt {
        self.observation_receipt.clone()
    }

    pub fn stop_and_join(self) -> Result<CaptureStopOutcome, CaptureError> {
        let Self {
            active_backend,
            frame_stream: _,
            runtime_event_receiver: _,
            observation_receipt,
            open_metadata: _,
            source_generation: _,
            discontinuity_epoch: _,
        } = self;
        let backend = active_backend.stop_and_join()?;
        let observations = observation_receipt.observations();
        Ok(CaptureStopOutcome {
            observations: CaptureOwnerObservations {
                backend,
                frame_stream: observations.frame_stream,
                runtime_events: observations.runtime_events,
            },
        })
    }

    fn observe_runtime_event(&self, event: &SourceRuntimeEvent) {
        let generation = match event {
            SourceRuntimeEvent::SourceUnavailable { generation, .. }
            | SourceRuntimeEvent::BackendFailure { generation, .. } => *generation,
        };
        self.source_generation
            .store(generation.0, Ordering::Release);
        self.discontinuity_epoch.fetch_add(1, Ordering::AcqRel);
    }
}

/// Prepares a bounded capture owner without starting native delivery.
#[cfg(any(test, feature = "internal-testing"))]
pub fn prepare_capture(
    backend: &dyn CallbackCaptureBackend,
    request: CapturePrepareRequest,
) -> Result<PreparedCapture, CaptureError> {
    prepare_capture_with_start_gate(backend, request, CaptureDeliveryStartGate::opened())
}

/// Prepares a bounded capture owner behind a caller-owned one-way start gate.
pub fn prepare_capture_with_start_gate(
    backend: &dyn CallbackCaptureBackend,
    request: CapturePrepareRequest,
    start_gate: Arc<CaptureDeliveryStartGate>,
) -> Result<PreparedCapture, CaptureError> {
    let lineage_seed = request.lineage_seed;
    let (frame_sender, frame_stream) =
        captured_frame_stream_with_start_gate(request.frame_capacity_frames, start_gate)?;
    let (runtime_event_sender, runtime_event_receiver) =
        source_runtime_event_channel(request.runtime_event_capacity_events)?;
    let prepared_backend = backend.prepare(request.mode)?;
    Ok(PreparedCapture {
        backend: prepared_backend,
        delivery: CaptureDelivery {
            frame_sender,
            runtime_event_sender,
        },
        frame_stream,
        runtime_event_receiver,
        lineage_seed,
    })
}

/// Joins one owned capture worker and preserves panic as a typed failure.
///
/// This is a control-thread operation. Callers with multiple workers must call
/// it for every owned handle before returning the first failure.
#[cfg(any(test, feature = "internal-testing", feature = "native-capture"))]
pub fn join_capture_worker(
    worker_thread: std::thread::JoinHandle<()>,
    worker: &'static str,
) -> Result<(), CaptureError> {
    worker_thread
        .join()
        .map_err(|_| CaptureError::CaptureWorkerPanicked { worker })
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::Arc;

    use crate::frame::{AudioBufferPool, AudioFrame, SessionId, SourceId, StemId, StreamId};

    use super::*;
    use crate::capture::{
        CaptureObservationCounters, CaptureRuntimeFailure, CaptureRuntimeFailureClass,
        SourceGeneration, SourceRecoveryRequirement, SourceRuntimeEvent, StableSourceId,
    };

    struct TestBackend {
        opened: Arc<AtomicBool>,
        stopped: Arc<AtomicBool>,
        emitted_source_id: SourceId,
        resolved_source_id: SourceId,
    }

    struct TestPreparedBackend {
        opened: Arc<AtomicBool>,
        stopped: Arc<AtomicBool>,
        emitted_source_id: SourceId,
        resolved_source_id: SourceId,
    }

    struct TestActiveBackend {
        counters: CaptureObservationCounters,
        stopped: Arc<AtomicBool>,
        resolved_source_id: SourceId,
    }

    impl CallbackCaptureBackend for TestBackend {
        fn prepare(
            &self,
            _mode: CaptureMode,
        ) -> Result<Box<dyn PreparedCaptureBackend>, CaptureError> {
            Ok(Box::new(TestPreparedBackend {
                opened: Arc::clone(&self.opened),
                stopped: Arc::clone(&self.stopped),
                emitted_source_id: self.emitted_source_id,
                resolved_source_id: self.resolved_source_id,
            }))
        }
    }

    impl PreparedCaptureBackend for TestPreparedBackend {
        fn open(
            self: Box<Self>,
            mut delivery: CaptureDelivery,
        ) -> Result<Box<dyn ActiveCaptureBackend>, CaptureError> {
            self.opened.store(true, Ordering::Release);
            let counters = CaptureObservationCounters::default();
            counters.observe_callback_buffer();
            counters.observe_enqueued_frame();
            let pool = AudioBufferPool::new(1, 960);
            let handle = pool.acquire().expect("test pool slot must be available");
            let frame = AudioFrame::new(StreamId(1), self.emitted_source_id, 3, 4, 1, handle);
            let _ = delivery.frame_sender.try_send(frame);
            let _ = delivery
                .runtime_event_sender
                .try_send(SourceRuntimeEvent::SourceUnavailable {
                    stable_id: StableSourceId::new(
                        crate::frame::Platform::Macos,
                        crate::capture::SourceKind::Application,
                        "test:application",
                    ),
                    generation: SourceGeneration::INITIAL,
                    recovery_requirement:
                        SourceRecoveryRequirement::ExplicitRediscoveryAndNewSession,
                    failure: CaptureRuntimeFailure {
                        operation: "test capture",
                        error_class: CaptureRuntimeFailureClass::SourceInstanceExited,
                    },
                });
            Ok(Box::new(TestActiveBackend {
                counters,
                stopped: Arc::clone(&self.stopped),
                resolved_source_id: self.resolved_source_id,
            }))
        }
    }

    impl ActiveCaptureBackend for TestActiveBackend {
        fn source_id(&self) -> SourceId {
            self.resolved_source_id
        }

        fn observation_handle(&self) -> CaptureObservationHandle {
            self.counters.observation_handle()
        }

        fn observations(&self) -> CaptureObservations {
            self.counters.snapshot()
        }

        fn stop_and_join(self: Box<Self>) -> Result<CaptureObservations, CaptureError> {
            self.stopped.store(true, Ordering::Release);
            Ok(self.counters.snapshot())
        }
    }

    impl Drop for TestActiveBackend {
        fn drop(&mut self) {
            self.stopped.store(true, Ordering::Release);
        }
    }

    fn lineage_seed() -> CaptureLineageSeed {
        CaptureLineageSeed::new(SessionId(11), StemId(12))
    }

    fn test_backend(opened: Arc<AtomicBool>, stopped: Arc<AtomicBool>) -> TestBackend {
        TestBackend {
            opened,
            stopped,
            emitted_source_id: SourceId(2),
            resolved_source_id: SourceId(2),
        }
    }

    #[test]
    fn given_prepared_capture_when_opened_then_bounded_delivery_is_owned() {
        let opened = Arc::new(AtomicBool::new(false));
        let stopped = Arc::new(AtomicBool::new(false));
        let backend = test_backend(Arc::clone(&opened), stopped);
        let prepared = prepare_capture(
            &backend,
            CapturePrepareRequest {
                mode: CaptureMode::SystemMix,
                lineage_seed: lineage_seed(),
                frame_capacity_frames: 2,
                runtime_event_capacity_events: 2,
            },
        )
        .expect("capture preparation must succeed");

        assert!(!opened.load(Ordering::Acquire));
        let mut owner = prepared.open().expect("capture open must succeed");

        assert!(opened.load(Ordering::Acquire));
        let frame = owner
            .try_next_lineaged_frame()
            .expect("lineage must be valid")
            .expect("frame must be delivered");
        assert_eq!(frame.frame().sequence_number, 3);
        assert_eq!(frame.lineage().session_id, SessionId(11));
        assert_eq!(frame.lineage().source_id, SourceId(2));
        assert_eq!(frame.lineage().stem_id, StemId(12));
        assert_eq!(frame.lineage().clock_id, CAPTURE_MONOTONIC_CLOCK_DOMAIN_ID);
        assert_eq!(
            frame.lineage().source_generation,
            SourceGeneration::INITIAL.0
        );
        assert_eq!(frame.lineage().permission_epoch, PermissionEpoch::INITIAL.0);
        let receipt = owner.observation_receipt();
        assert!(matches!(
            owner.try_recv_runtime_event(),
            SourceRuntimeEventReceive::Event(SourceRuntimeEvent::SourceUnavailable { .. })
        ));
        assert_eq!(owner.open_metadata().discontinuity_epoch, 1);
        assert_eq!(receipt.observations().backend.callback_buffers_total, 1);
        assert_eq!(receipt.observations().frame_stream.delivered_frames, 1);
        assert_eq!(
            receipt.observations().runtime_events.events_enqueued_total,
            1
        );
    }

    #[test]
    fn given_backend_frame_when_source_differs_from_open_identity_then_lineage_fails_closed() {
        let opened = Arc::new(AtomicBool::new(false));
        let stopped = Arc::new(AtomicBool::new(false));
        let backend = TestBackend {
            opened,
            stopped,
            emitted_source_id: SourceId(99),
            resolved_source_id: SourceId(2),
        };
        let mut owner = prepare_capture(
            &backend,
            CapturePrepareRequest {
                mode: CaptureMode::SystemMix,
                lineage_seed: lineage_seed(),
                frame_capacity_frames: 1,
                runtime_event_capacity_events: 1,
            },
        )
        .expect("capture preparation must succeed")
        .open()
        .expect("capture open must succeed");

        assert!(matches!(
            owner.try_next_lineaged_frame(),
            Err(FrameLineageError::Source)
        ));
    }

    #[test]
    fn given_active_capture_when_stopped_then_backend_is_joined() {
        let opened = Arc::new(AtomicBool::new(false));
        let stopped = Arc::new(AtomicBool::new(false));
        let backend = test_backend(opened, Arc::clone(&stopped));
        let owner = prepare_capture(
            &backend,
            CapturePrepareRequest {
                mode: CaptureMode::SystemMix,
                lineage_seed: lineage_seed(),
                frame_capacity_frames: 1,
                runtime_event_capacity_events: 1,
            },
        )
        .expect("capture preparation must succeed")
        .open()
        .expect("capture open must succeed");
        let receipt = owner.observation_receipt();

        let outcome = owner.stop_and_join().expect("capture stop must succeed");

        assert!(stopped.load(Ordering::Acquire));
        assert_eq!(outcome.observations.frame_stream.delivered_frames, 1);
        assert_eq!(outcome.observations.runtime_events.events_enqueued_total, 1);
        assert_eq!(receipt.observations(), outcome.observations);
    }

    #[test]
    fn given_active_capture_when_owner_is_dropped_then_backend_is_reclaimed() {
        let opened = Arc::new(AtomicBool::new(false));
        let stopped = Arc::new(AtomicBool::new(false));
        let backend = test_backend(opened, Arc::clone(&stopped));
        let owner = prepare_capture(
            &backend,
            CapturePrepareRequest {
                mode: CaptureMode::SystemMix,
                lineage_seed: lineage_seed(),
                frame_capacity_frames: 1,
                runtime_event_capacity_events: 1,
            },
        )
        .expect("capture preparation must succeed")
        .open()
        .expect("capture open must succeed");

        drop(owner);

        assert!(stopped.load(Ordering::Acquire));
    }

    #[test]
    fn given_zero_frame_capacity_when_preparing_then_backend_is_not_prepared() {
        let opened = Arc::new(AtomicBool::new(false));
        let stopped = Arc::new(AtomicBool::new(false));
        let backend = test_backend(Arc::clone(&opened), stopped);

        let prepared = prepare_capture(
            &backend,
            CapturePrepareRequest {
                mode: CaptureMode::SystemMix,
                lineage_seed: lineage_seed(),
                frame_capacity_frames: 0,
                runtime_event_capacity_events: 1,
            },
        );

        assert!(matches!(prepared, Err(CaptureError::InvalidStreamCapacity)));
        assert!(!opened.load(Ordering::Acquire));
    }

    #[test]
    fn given_panicking_capture_worker_when_joined_then_typed_failure_is_returned() {
        let worker_thread = std::thread::spawn(|| panic!("test worker panic"));

        let outcome = join_capture_worker(worker_thread, "test capture");

        assert_eq!(
            outcome,
            Err(CaptureError::CaptureWorkerPanicked {
                worker: "test capture"
            })
        );
    }
}
