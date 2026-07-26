use crate::{
    captured_frame_stream, source_runtime_event_channel, CaptureError, CaptureMode,
    CaptureObservations, CapturedFrameSender, CapturedFrameStream, CapturedFrameStreamStats,
    SourceRuntimeEventObservations, SourceRuntimeEventReceive, SourceRuntimeEventReceiver,
    SourceRuntimeEventSender,
};

/// Setup-time request for one bounded callback-oriented capture owner.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CapturePrepareRequest {
    pub mode: CaptureMode,
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
}

impl PreparedCapture {
    pub fn open(self) -> Result<CaptureOwner, CaptureError> {
        let active_backend = self.backend.open(self.delivery)?;
        Ok(CaptureOwner {
            active_backend,
            frame_stream: self.frame_stream,
            runtime_event_receiver: self.runtime_event_receiver,
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
}

impl CaptureOwner {
    pub fn try_next_frame(&mut self) -> Option<pks_frame::AudioFrame> {
        self.frame_stream.try_next()
    }

    pub fn try_recv_runtime_event(&self) -> SourceRuntimeEventReceive {
        self.runtime_event_receiver.try_recv()
    }

    pub fn frame_stream_closed(&self) -> bool {
        self.frame_stream.is_closed()
    }

    pub fn observations(&self) -> CaptureOwnerObservations {
        CaptureOwnerObservations {
            backend: self.active_backend.observations(),
            frame_stream: self.frame_stream.stats(),
            runtime_events: self.runtime_event_receiver.observations(),
        }
    }

    pub fn stop_and_join(self) -> Result<CaptureStopOutcome, CaptureError> {
        let Self {
            active_backend,
            frame_stream,
            runtime_event_receiver,
        } = self;
        let backend = active_backend.stop_and_join()?;
        Ok(CaptureStopOutcome {
            observations: CaptureOwnerObservations {
                backend,
                frame_stream: frame_stream.stats(),
                runtime_events: runtime_event_receiver.observations(),
            },
        })
    }
}

/// Prepares a bounded capture owner without starting native delivery.
pub fn prepare_capture(
    backend: &dyn CallbackCaptureBackend,
    request: CapturePrepareRequest,
) -> Result<PreparedCapture, CaptureError> {
    let (frame_sender, frame_stream) = captured_frame_stream(request.frame_capacity_frames)?;
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
    })
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::Arc;

    use pks_frame::{AudioBufferPool, AudioFrame, SourceId, StreamId};

    use super::*;
    use crate::{
        CaptureObservationCounters, CaptureRuntimeFailure, CaptureRuntimeFailureClass,
        SourceGeneration, SourceRecoveryRequirement, SourceRuntimeEvent, StableSourceId,
    };

    struct TestBackend {
        opened: Arc<AtomicBool>,
        stopped: Arc<AtomicBool>,
    }

    struct TestPreparedBackend {
        opened: Arc<AtomicBool>,
        stopped: Arc<AtomicBool>,
    }

    struct TestActiveBackend {
        stopped: Arc<AtomicBool>,
    }

    impl CallbackCaptureBackend for TestBackend {
        fn prepare(
            &self,
            _mode: CaptureMode,
        ) -> Result<Box<dyn PreparedCaptureBackend>, CaptureError> {
            Ok(Box::new(TestPreparedBackend {
                opened: Arc::clone(&self.opened),
                stopped: Arc::clone(&self.stopped),
            }))
        }
    }

    impl PreparedCaptureBackend for TestPreparedBackend {
        fn open(
            self: Box<Self>,
            mut delivery: CaptureDelivery,
        ) -> Result<Box<dyn ActiveCaptureBackend>, CaptureError> {
            self.opened.store(true, Ordering::Release);
            let pool = AudioBufferPool::new(1, 960);
            let handle = pool.acquire().expect("test pool slot must be available");
            let frame = AudioFrame::new(StreamId(1), SourceId(2), 3, 4, 1, handle);
            let _ = delivery.frame_sender.try_send(frame);
            let _ = delivery
                .runtime_event_sender
                .try_send(SourceRuntimeEvent::SourceUnavailable {
                    stable_id: StableSourceId::new(
                        pks_frame::Platform::Macos,
                        crate::SourceKind::Application,
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
                stopped: Arc::clone(&self.stopped),
            }))
        }
    }

    impl ActiveCaptureBackend for TestActiveBackend {
        fn observations(&self) -> CaptureObservations {
            CaptureObservationCounters::default().snapshot()
        }

        fn stop_and_join(self: Box<Self>) -> Result<CaptureObservations, CaptureError> {
            self.stopped.store(true, Ordering::Release);
            Ok(CaptureObservationCounters::default().snapshot())
        }
    }

    impl Drop for TestActiveBackend {
        fn drop(&mut self) {
            self.stopped.store(true, Ordering::Release);
        }
    }

    #[test]
    fn given_prepared_capture_when_opened_then_bounded_delivery_is_owned() {
        let opened = Arc::new(AtomicBool::new(false));
        let stopped = Arc::new(AtomicBool::new(false));
        let backend = TestBackend {
            opened: Arc::clone(&opened),
            stopped,
        };
        let prepared = prepare_capture(
            &backend,
            CapturePrepareRequest {
                mode: CaptureMode::SystemMix,
                frame_capacity_frames: 2,
                runtime_event_capacity_events: 2,
            },
        )
        .expect("capture preparation must succeed");

        assert!(!opened.load(Ordering::Acquire));
        let mut owner = prepared.open().expect("capture open must succeed");

        assert!(opened.load(Ordering::Acquire));
        assert_eq!(
            owner
                .try_next_frame()
                .expect("frame must be delivered")
                .sequence_number,
            3
        );
        assert!(matches!(
            owner.try_recv_runtime_event(),
            SourceRuntimeEventReceive::Event(SourceRuntimeEvent::SourceUnavailable { .. })
        ));
    }

    #[test]
    fn given_active_capture_when_stopped_then_backend_is_joined() {
        let opened = Arc::new(AtomicBool::new(false));
        let stopped = Arc::new(AtomicBool::new(false));
        let backend = TestBackend {
            opened,
            stopped: Arc::clone(&stopped),
        };
        let owner = prepare_capture(
            &backend,
            CapturePrepareRequest {
                mode: CaptureMode::SystemMix,
                frame_capacity_frames: 1,
                runtime_event_capacity_events: 1,
            },
        )
        .expect("capture preparation must succeed")
        .open()
        .expect("capture open must succeed");

        let outcome = owner.stop_and_join().expect("capture stop must succeed");

        assert!(stopped.load(Ordering::Acquire));
        assert_eq!(outcome.observations.frame_stream.delivered_frames, 1);
        assert_eq!(outcome.observations.runtime_events.events_enqueued_total, 1);
    }

    #[test]
    fn given_active_capture_when_owner_is_dropped_then_backend_is_reclaimed() {
        let opened = Arc::new(AtomicBool::new(false));
        let stopped = Arc::new(AtomicBool::new(false));
        let backend = TestBackend {
            opened,
            stopped: Arc::clone(&stopped),
        };
        let owner = prepare_capture(
            &backend,
            CapturePrepareRequest {
                mode: CaptureMode::SystemMix,
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
        let backend = TestBackend {
            opened: Arc::clone(&opened),
            stopped,
        };

        let prepared = prepare_capture(
            &backend,
            CapturePrepareRequest {
                mode: CaptureMode::SystemMix,
                frame_capacity_frames: 0,
                runtime_event_capacity_events: 1,
            },
        );

        assert!(matches!(prepared, Err(CaptureError::InvalidStreamCapacity)));
        assert!(!opened.load(Ordering::Acquire));
    }
}
