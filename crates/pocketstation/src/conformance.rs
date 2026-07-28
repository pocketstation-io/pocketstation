//! Deterministic canonical-engine fixture for external conformance harnesses.
//!
//! This feature is `LOOPBACK-ONLY`, disabled by default, and is not product
//! capture evidence.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

use pks_capture::{
    ActiveCaptureBackend, CallbackCaptureBackend, CaptureDelivery, CaptureError, CaptureMode,
    CaptureObservationHandle, CaptureObservations, CapturedFrameDelivery, PreparedCaptureBackend,
};
use pks_frame::{AudioBufferPool, AudioFrame, SampleFormat, SampleSpec, SourceId, StreamId};
use pks_graph::PrepareContext;
use pks_session::{
    NativeSessionEngineHostOptions, SessionEngineHostBuildError, SessionEngineHostBuilder,
};

use crate::Session;

#[derive(Clone, Copy)]
enum FixtureSource {
    Application,
    Microphone,
}

impl FixtureSource {
    const fn stream_id(self) -> StreamId {
        match self {
            Self::Application => StreamId(101),
            Self::Microphone => StreamId(201),
        }
    }

    const fn source_id(self) -> SourceId {
        match self {
            Self::Application => SourceId(102),
            Self::Microphone => SourceId(202),
        }
    }

    const fn timestamp_ns(self) -> u64 {
        match self {
            Self::Application => 1_000_000,
            Self::Microphone => 2_000_000,
        }
    }

    const fn amplitude(self) -> f32 {
        match self {
            Self::Application => 0.25,
            Self::Microphone => 0.5,
        }
    }
}

struct DeterministicCaptureBackend;

struct DeterministicPreparedCapture {
    source: FixtureSource,
}

struct DeterministicActiveCapture {
    stop_requested: Arc<AtomicBool>,
    worker: Option<std::thread::JoinHandle<()>>,
}

impl CallbackCaptureBackend for DeterministicCaptureBackend {
    fn prepare(&self, mode: CaptureMode) -> Result<Box<dyn PreparedCaptureBackend>, CaptureError> {
        let source = match mode {
            CaptureMode::InputDevice(_) => FixtureSource::Microphone,
            CaptureMode::SystemMix
            | CaptureMode::Application(_)
            | CaptureMode::Process(_)
            | CaptureMode::ExactApplication { .. }
            | CaptureMode::ExactApplicationStable { .. } => FixtureSource::Application,
        };
        Ok(Box::new(DeterministicPreparedCapture { source }))
    }
}

impl PreparedCaptureBackend for DeterministicPreparedCapture {
    fn open(
        self: Box<Self>,
        mut delivery: CaptureDelivery,
    ) -> Result<Box<dyn ActiveCaptureBackend>, CaptureError> {
        let stop_requested = Arc::new(AtomicBool::new(false));
        let worker_stop_requested = Arc::clone(&stop_requested);
        let source = self.source;
        let worker = std::thread::spawn(move || {
            let pool = AudioBufferPool::new(1, 4);
            while !worker_stop_requested.load(Ordering::Acquire) {
                let Some(mut buffer) = pool.acquire() else {
                    std::thread::sleep(Duration::from_millis(1));
                    continue;
                };
                buffer.copy_from_slice(&[source.amplitude(); 4]);
                let frame = AudioFrame::new(
                    source.stream_id(),
                    source.source_id(),
                    0,
                    source.timestamp_ns(),
                    1,
                    buffer,
                );
                match delivery.frame_sender.try_send(frame) {
                    CapturedFrameDelivery::Delivered => break,
                    CapturedFrameDelivery::DroppedNewest
                    | CapturedFrameDelivery::DiscardedBeforeStart => {
                        std::thread::sleep(Duration::from_millis(1));
                    }
                }
            }
        });
        Ok(Box::new(DeterministicActiveCapture {
            stop_requested,
            worker: Some(worker),
        }))
    }
}

impl ActiveCaptureBackend for DeterministicActiveCapture {
    fn observation_handle(&self) -> CaptureObservationHandle {
        CaptureObservationHandle::default()
    }

    fn observations(&self) -> CaptureObservations {
        CaptureObservations::default()
    }

    fn stop_and_join(mut self: Box<Self>) -> Result<CaptureObservations, CaptureError> {
        self.stop_requested.store(true, Ordering::Release);
        if let Some(worker) = self.worker.take() {
            worker
                .join()
                .map_err(|_| CaptureError::CaptureWorkerPanicked {
                    worker: "Rust facade conformance capture worker",
                })?;
        }
        Ok(CaptureObservations::default())
    }
}

/// Drop contract: signal-only, allocation-free, blocking-free, log-free, panic-free.
impl Drop for DeterministicActiveCapture {
    fn drop(&mut self) {
        self.stop_requested.store(true, Ordering::Release);
        let _ = self.worker.take();
    }
}

pub fn session() -> Result<Session, SessionEngineHostBuildError> {
    let options = NativeSessionEngineHostOptions::default();
    let prepare_context =
        PrepareContext::new(SampleSpec::new(48_000, 1, SampleFormat::F32Interleaved));
    let mut builder = SessionEngineHostBuilder::new(
        prepare_context,
        options.source_queue_capacity_frames,
        options.start_options,
    )?;
    let capture_backend: Arc<dyn CallbackCaptureBackend> = Arc::new(DeterministicCaptureBackend);
    builder
        .set_application_backend(Arc::clone(&capture_backend))
        .set_microphone_backend(capture_backend);
    let _ = builder.register_polled_audio_endpoint(options.polled_audio_endpoint)?;
    Session::with_host(builder.build()?)
}
