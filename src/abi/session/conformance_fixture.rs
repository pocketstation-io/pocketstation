use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

use crate::capture::{
    ActiveCaptureBackend, CallbackCaptureBackend, CaptureDelivery, CaptureError, CaptureMode,
    CaptureObservationHandle, CaptureObservations, CapturedFrameDelivery, PreparedCaptureBackend,
};
use crate::frame::{AudioBufferPool, AudioFrame, SampleFormat, SampleSpec, SourceId, StreamId};
use crate::graph::PrepareContext;
use crate::session::{NativeSessionEngineHostOptions, SessionEngineHost, SessionEngineHostBuilder};

use crate::abi::session::{
    abi_call, engine_options, guard_pointer_alignment, read_versioned_record, runtime_state,
    write_record, AbiError, PksSessionEngineConfig, PksSessionHandle, PksSessionStatus,
};

struct DeterministicCaptureBackend;

struct DeterministicPreparedCapture;

struct DeterministicActiveCapture {
    stop_requested: Arc<AtomicBool>,
    worker: Option<std::thread::JoinHandle<()>>,
}

impl CallbackCaptureBackend for DeterministicCaptureBackend {
    fn prepare(&self, _mode: CaptureMode) -> Result<Box<dyn PreparedCaptureBackend>, CaptureError> {
        Ok(Box::new(DeterministicPreparedCapture))
    }
}

impl PreparedCaptureBackend for DeterministicPreparedCapture {
    fn open(
        self: Box<Self>,
        mut delivery: CaptureDelivery,
    ) -> Result<Box<dyn ActiveCaptureBackend>, CaptureError> {
        let pool = AudioBufferPool::new(1, 4);
        let mut buffer = pool.acquire().ok_or_else(|| {
            CaptureError::BackendInit("conformance capture buffer unavailable".to_owned())
        })?;
        buffer
            .try_copy_from_slice(&[0.125, 0.25, 0.5, 1.0])
            .expect("fixture samples fit the fixed-capacity buffer");
        let frame = AudioFrame::new(StreamId(11), SourceId(12), 13, 14, 1, buffer);
        let _ = delivery.frame_sender.try_send(frame);

        let stop_requested = Arc::new(AtomicBool::new(false));
        let worker_stop_requested = Arc::clone(&stop_requested);
        let worker = std::thread::spawn(move || {
            let pool = AudioBufferPool::new(1, 4);
            while !worker_stop_requested.load(Ordering::Acquire) {
                let Some(mut buffer) = pool.acquire() else {
                    std::thread::sleep(Duration::from_millis(1));
                    continue;
                };
                buffer
                    .try_copy_from_slice(&[0.125, 0.25, 0.5, 1.0])
                    .expect("fixture samples fit the fixed-capacity buffer");
                let frame = AudioFrame::new(StreamId(11), SourceId(12), 13, 14, 2, buffer);
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
    fn source_id(&self) -> SourceId {
        SourceId(12)
    }

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
                    worker: "C conformance capture worker",
                })?;
        }
        Ok(CaptureObservations::default())
    }
}

impl Drop for DeterministicActiveCapture {
    fn drop(&mut self) {
        self.stop_requested.store(true, Ordering::Release);
        if let Some(worker) = self.worker.take() {
            let _ = worker.join();
        }
    }
}

fn conformance_host(
    options: NativeSessionEngineHostOptions,
) -> Result<SessionEngineHost, AbiError> {
    let prepare_context =
        PrepareContext::new(SampleSpec::new(48_000, 1, SampleFormat::F32Interleaved));
    let mut builder = SessionEngineHostBuilder::new(
        prepare_context,
        options.source_queue_capacity_frames,
        options.start_options,
    )
    .map_err(|_| AbiError::BackendFailure)?;
    let capture_backend: Arc<dyn CallbackCaptureBackend> = Arc::new(DeterministicCaptureBackend);
    builder
        .set_application_backend(Arc::clone(&capture_backend))
        .set_microphone_backend(capture_backend);
    let _ = builder
        .register_polled_audio_endpoint(options.polled_audio_endpoint)
        .map_err(|_| AbiError::BackendFailure)?;
    builder.build().map_err(|_| AbiError::BackendFailure)
}

#[unsafe(no_mangle)]
/// Creates a deterministic canonical engine for the non-production C harness.
///
/// # Safety
///
/// `config` and `output_engine` follow the same contract as
/// `pks_session_engine_create`.
pub unsafe extern "C" fn pks_session_conformance_engine_create(
    config: *const PksSessionEngineConfig,
    output_engine: *mut PksSessionHandle,
) -> PksSessionStatus {
    abi_call(|| {
        guard_pointer_alignment(output_engine.cast_const())?;
        // SAFETY: This fixture export forwards the documented caller contract.
        let config = unsafe { read_versioned_record(config) }?;
        let options = engine_options(config)?;
        let lease_capacity_count = options.polled_audio_endpoint.max_outstanding_leases;
        let engine = runtime_state()
            .allocate_engine_with_host(conformance_host(options)?, lease_capacity_count)?;
        // SAFETY: This fixture export forwards the documented caller contract.
        unsafe { write_record(output_engine, engine) }?;
        Ok(PksSessionStatus::ok())
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn pks_session_conformance_panic() -> PksSessionStatus {
    abi_call(|| -> Result<_, AbiError> { panic!("intentional C conformance panic") })
}
