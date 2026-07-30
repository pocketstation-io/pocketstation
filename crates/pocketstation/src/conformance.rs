//! Deterministic canonical-engine fixture for external conformance harnesses.
//!
//! This feature is `LOOPBACK-ONLY`, disabled by default, and is not product
//! capture evidence.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, OnceLock};
use std::time::Duration;
use std::{path::PathBuf, thread};

use pks_capture::{
    ActiveCaptureBackend, CallbackCaptureBackend, CaptureDelivery, CaptureError, CaptureMode,
    CaptureObservationHandle, CaptureObservations, CapturedFrameDelivery, PreparedCaptureBackend,
};
use pks_frame::{AudioBufferPool, AudioFrame, SampleFormat, SampleSpec, SourceId, StreamId};
use pks_graph::PrepareContext;
use pks_session::{
    NativeSessionEngineHostOptions, PolledAudioEndpointConfig, SessionEngineHostBuildError,
    SessionEngineHostBuilder,
};

use crate::Session;

const FRAME_SAMPLES_PER_CHANNEL: usize = 960;
const FRAME_DURATION_NS: u64 = 20_000_000;
const FRAME_PACING_MS: u64 = 20;
const RECORDING_EDGE_CAPACITY_FRAMES: usize = pks_graph::plan::EDGE_RING_CAPACITY_FRAMES;
const SLOW_BRANCH_QUEUE_CAPACITY_FRAMES: usize = RECORDING_EDGE_CAPACITY_FRAMES / 2;
/// Frames emitted per source by the finite deterministic fixture.
///
/// This equals the canonical runtime edge capacity, so the recording branch
/// remains lossless even if its worker is not scheduled until capture ends.
/// The independently configured half-capacity polled branch still saturates.
pub const FRAMES_PER_SOURCE: u64 = RECORDING_EDGE_CAPACITY_FRAMES as u64;

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

    const fn amplitude(self) -> f32 {
        match self {
            Self::Application => 0.25,
            Self::Microphone => 0.5,
        }
    }

    const fn channels(self) -> u8 {
        match self {
            Self::Application => 2,
            Self::Microphone => 1,
        }
    }
}

struct DeterministicCaptureBackend {
    timestamp_origin_ns: Arc<OnceLock<u64>>,
}

struct DeterministicPreparedCapture {
    source: FixtureSource,
    timestamp_origin_ns: Arc<OnceLock<u64>>,
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
        Ok(Box::new(DeterministicPreparedCapture {
            source,
            timestamp_origin_ns: Arc::clone(&self.timestamp_origin_ns),
        }))
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
        let timestamp_origin_ns = *self
            .timestamp_origin_ns
            .get_or_init(|| pks_timing::monotonic_timestamp_ns().saturating_add(1_000_000));
        let worker = std::thread::spawn(move || {
            let samples_per_frame =
                FRAME_SAMPLES_PER_CHANNEL.saturating_mul(usize::from(source.channels()));
            let pool = AudioBufferPool::new(32, samples_per_frame);
            let mut sequence = 0_u64;
            while !worker_stop_requested.load(Ordering::Acquire) && sequence < FRAMES_PER_SOURCE {
                let Some(mut buffer) = pool.acquire() else {
                    thread::sleep(Duration::from_millis(1));
                    continue;
                };
                buffer.as_mut_slice().fill(source.amplitude());
                let frame = AudioFrame::new(
                    source.stream_id(),
                    source.source_id(),
                    sequence,
                    timestamp_origin_ns + sequence.saturating_mul(FRAME_DURATION_NS),
                    source.channels(),
                    buffer,
                );
                match delivery.frame_sender.try_send(frame) {
                    CapturedFrameDelivery::Delivered => {
                        sequence = sequence.saturating_add(1);
                        thread::sleep(Duration::from_millis(FRAME_PACING_MS));
                    }
                    CapturedFrameDelivery::DroppedNewest
                    | CapturedFrameDelivery::DiscardedBeforeStart => {
                        thread::sleep(Duration::from_millis(1));
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
    session_with_optional_recording(None)
}

/// Creates the deterministic canonical-engine fixture with multistem recording.
pub fn session_with_recording(
    output_root: impl Into<PathBuf>,
) -> Result<Session, SessionEngineHostBuildError> {
    session_with_optional_recording(Some(output_root.into()))
}

fn session_with_optional_recording(
    output_root: Option<PathBuf>,
) -> Result<Session, SessionEngineHostBuildError> {
    let mut options = NativeSessionEngineHostOptions::default();
    if output_root.is_some() {
        options.polled_audio_endpoint = PolledAudioEndpointConfig {
            queue_capacity_frames: SLOW_BRANCH_QUEUE_CAPACITY_FRAMES,
            ..PolledAudioEndpointConfig::default()
        };
    }
    let prepare_context =
        PrepareContext::new(SampleSpec::new(48_000, 1, SampleFormat::F32Interleaved));
    let mut builder = SessionEngineHostBuilder::new(
        prepare_context,
        options.source_queue_capacity_frames,
        options.start_options,
    )?;
    let capture_backend: Arc<dyn CallbackCaptureBackend> = Arc::new(DeterministicCaptureBackend {
        timestamp_origin_ns: Arc::new(OnceLock::new()),
    });
    builder
        .set_application_backend(Arc::clone(&capture_backend))
        .set_microphone_backend(capture_backend);
    let _ = builder.register_polled_audio_endpoint(options.polled_audio_endpoint)?;
    if let Some(output_root) = output_root {
        let _ = builder.register_multistem_recording(output_root)?;
    }
    Session::with_host(builder.build()?)
}
