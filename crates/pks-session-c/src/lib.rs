mod abi;
#[cfg(feature = "conformance-fixtures")]
mod conformance_fixture;
mod error;
mod handle;
mod runtime;

pub use abi::{
    PksSessionAbiVersion, PksSessionAppMicDeclaration, PksSessionAudioBatch, PksSessionAudioFrame,
    PksSessionEndpointObservationStage, PksSessionEngineConfig, PksSessionEvent,
    PksSessionEventKind, PksSessionHandle, PksSessionHandleKind, PksSessionLifecycleState,
    PksSessionMetricsSnapshot, PksSessionRouteMetrics, PksSessionSampleFormat,
    PksSessionSourceMetrics, PksSessionStatus, PksSessionStatusCode, PksSessionUtf8,
    PKS_SESSION_ABI_MAJOR, PKS_SESSION_ABI_MINOR,
};

use std::mem::{align_of, size_of};
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::slice;
use std::str;
use std::sync::OnceLock;

use abi::PksSessionStatusCode::InternalPanic;
use error::AbiError;
use pks_session::{NativeSessionEngineHostOptions, PolledAudioEndpointConfig, SessionStartOptions};
use runtime::RuntimeState;

fn runtime_state() -> &'static RuntimeState {
    static RUNTIME: OnceLock<RuntimeState> = OnceLock::new();
    RUNTIME.get_or_init(RuntimeState::new)
}

fn guard_abi_version(requested_abi_major: u16, requested_abi_minor: u16) -> Result<(), AbiError> {
    if requested_abi_major != PKS_SESSION_ABI_MAJOR {
        return Err(AbiError::UnsupportedAbiMajor);
    }
    if requested_abi_minor > PKS_SESSION_ABI_MINOR {
        return Err(AbiError::UnsupportedAbiMinor);
    }
    Ok(())
}

fn guard_record_size(record_size_bytes: u32, required_size_bytes: usize) -> Result<(), AbiError> {
    if record_size_bytes < required_size_bytes as u32 {
        return Err(AbiError::InvalidStructSize);
    }
    Ok(())
}

fn guard_pointer_alignment<T>(pointer: *const T) -> Result<(), AbiError> {
    if pointer.is_null() {
        return Err(AbiError::NullArgument);
    }
    if !(pointer as usize).is_multiple_of(align_of::<T>()) {
        return Err(AbiError::MisalignedPointer);
    }
    Ok(())
}

unsafe fn read_record<T: Copy>(input: *const T) -> Result<T, AbiError> {
    guard_pointer_alignment(input)?;
    // SAFETY: The caller contract requires one readable, aligned T record for
    // the duration of this call. Alignment and nullability were checked above.
    Ok(unsafe { input.read() })
}

#[repr(C)]
#[derive(Clone, Copy)]
struct VersionedRecordHeader {
    struct_size_bytes: u32,
    abi_major: u16,
    abi_minor: u16,
}

unsafe fn read_versioned_record<T: Copy>(input: *const T) -> Result<T, AbiError> {
    guard_pointer_alignment(input)?;
    // SAFETY: The caller contract requires the fixed eight-byte versioned
    // record header to be readable before any current-version tail is read.
    let header = unsafe { input.cast::<VersionedRecordHeader>().read() };
    guard_abi_version(header.abi_major, header.abi_minor)?;
    guard_record_size(header.struct_size_bytes, size_of::<T>())?;
    // SAFETY: The validated size declares that one full current T record is
    // readable, and nullability/alignment were checked before reading.
    unsafe { read_record(input) }
}

unsafe fn write_record<T>(output: *mut T, value: T) -> Result<(), AbiError> {
    guard_pointer_alignment(output.cast_const())?;
    // SAFETY: The caller contract requires one writable, aligned T record for
    // the duration of this call. Alignment and nullability were checked above.
    unsafe { output.write(value) };
    Ok(())
}

unsafe fn copy_utf8(view: PksSessionUtf8) -> Result<String, AbiError> {
    if view.len_bytes == 0 {
        return Err(AbiError::InvalidArgument);
    }
    guard_pointer_alignment(view.data)?;
    // SAFETY: The caller contract requires `len_bytes` readable bytes at
    // `data` for the duration of this call. The adapter copies before return.
    let bytes = unsafe { slice::from_raw_parts(view.data, view.len_bytes as usize) };
    let value = str::from_utf8(bytes).map_err(|_| AbiError::InvalidArgument)?;
    if value.trim().is_empty() {
        return Err(AbiError::InvalidArgument);
    }
    Ok(value.to_owned())
}

fn engine_options(
    config: PksSessionEngineConfig,
) -> Result<NativeSessionEngineHostOptions, AbiError> {
    guard_abi_version(config.abi_major, config.abi_minor)?;
    guard_record_size(
        config.struct_size_bytes,
        size_of::<PksSessionEngineConfig>(),
    )?;
    let to_usize = |value: u32| usize::try_from(value).map_err(|_| AbiError::InvalidArgument);
    Ok(NativeSessionEngineHostOptions {
        source_queue_capacity_frames: to_usize(config.source_queue_capacity_frames)?,
        start_options: SessionStartOptions {
            capture_frame_capacity_frames: to_usize(config.capture_frame_capacity_frames)?,
            capture_runtime_event_capacity_events: to_usize(
                config.capture_runtime_event_capacity_count,
            )?,
            runtime_work_budget_frames: to_usize(config.runtime_work_budget_frames)?,
            runtime_idle_poll_ms: config.runtime_idle_poll_ms,
            runtime_ready_timeout_ms: config.runtime_ready_timeout_ms,
            session_event_capacity_events: to_usize(config.session_event_capacity_count)?,
        },
        polled_audio_endpoint: PolledAudioEndpointConfig {
            queue_capacity_frames: to_usize(config.audio_queue_capacity_frames)?,
            max_batch_frames: to_usize(config.audio_max_batch_frames)?,
            max_outstanding_leases: to_usize(config.audio_max_outstanding_leases)?,
        },
    })
}

fn abi_call(function: impl FnOnce() -> Result<PksSessionStatus, AbiError>) -> PksSessionStatus {
    match catch_unwind(AssertUnwindSafe(function)) {
        Ok(Ok(status)) => status,
        Ok(Err(error)) => error.status(),
        Err(_) => PksSessionStatus::new(InternalPanic, 0),
    }
}

#[unsafe(no_mangle)]
/// Returns the current portable Session ABI version record.
///
/// # Safety
///
/// `output_version` must address one writable, aligned
/// `PksSessionAbiVersion` record for this call.
pub unsafe extern "C" fn pks_session_abi_get_version(
    output_version: *mut PksSessionAbiVersion,
) -> PksSessionStatus {
    abi_call(|| {
        guard_pointer_alignment(output_version.cast_const())?;
        // SAFETY: This export forwards the documented caller contract.
        unsafe { write_record(output_version, PksSessionAbiVersion::current()) }?;
        Ok(PksSessionStatus::ok())
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn pks_session_abi_is_compatible(
    requested_abi_major: u16,
    requested_abi_minor: u16,
    requested_struct_size_bytes: u32,
) -> PksSessionStatus {
    abi_call(|| {
        guard_abi_version(requested_abi_major, requested_abi_minor)?;
        guard_record_size(
            requested_struct_size_bytes,
            size_of::<PksSessionAbiVersion>(),
        )?;
        Ok(PksSessionStatus::ok())
    })
}

#[unsafe(no_mangle)]
/// Creates one real native Session engine with bounded queues.
///
/// # Safety
///
/// `config` must address one readable, aligned `PksSessionEngineConfig`
/// record. `output_engine` must address one writable, aligned handle.
pub unsafe extern "C" fn pks_session_engine_create(
    config: *const PksSessionEngineConfig,
    output_engine: *mut PksSessionHandle,
) -> PksSessionStatus {
    abi_call(|| {
        guard_pointer_alignment(output_engine.cast_const())?;
        // SAFETY: This export forwards the documented caller contract.
        let config = unsafe { read_versioned_record(config) }?;
        let engine = runtime_state().allocate_engine(engine_options(config)?)?;
        // SAFETY: This export forwards the documented caller contract.
        unsafe { write_record(output_engine, engine) }?;
        Ok(PksSessionStatus::ok())
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn pks_session_engine_destroy(engine: PksSessionHandle) -> PksSessionStatus {
    abi_call(|| {
        runtime_state().release_engine(engine)?;
        Ok(PksSessionStatus::ok())
    })
}

#[unsafe(no_mangle)]
/// Reports whether an engine handle remains live.
///
/// # Safety
///
/// `output_is_live` must address one writable, aligned `uint32_t`.
pub unsafe extern "C" fn pks_session_engine_is_live(
    engine: PksSessionHandle,
    output_is_live: *mut u32,
) -> PksSessionStatus {
    abi_call(|| {
        guard_pointer_alignment(output_is_live.cast_const())?;
        let is_live = runtime_state().engine_is_live(engine)?;
        // SAFETY: This export forwards the documented caller contract.
        unsafe { write_record(output_is_live, u32::from(is_live)) }?;
        Ok(PksSessionStatus::ok())
    })
}

#[unsafe(no_mangle)]
/// Creates the current narrow application-plus-default-microphone declaration.
///
/// # Safety
///
/// `declaration` must address one readable, aligned declaration record and its
/// UTF-8 view must remain readable for this call. `output_session` must address
/// one writable, aligned handle.
pub unsafe extern "C" fn pks_session_create_app_mic(
    engine: PksSessionHandle,
    declaration: *const PksSessionAppMicDeclaration,
    output_session: *mut PksSessionHandle,
) -> PksSessionStatus {
    abi_call(|| {
        guard_pointer_alignment(output_session.cast_const())?;
        // SAFETY: This export forwards the documented caller contract.
        let declaration = unsafe { read_versioned_record(declaration) }?;
        // SAFETY: This export forwards the documented caller contract.
        let application_name = unsafe { copy_utf8(declaration.application_name) }?;
        let session = runtime_state().create_session(engine, application_name)?;
        // SAFETY: This export forwards the documented caller contract.
        unsafe { write_record(output_session, session) }?;
        Ok(PksSessionStatus::ok())
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn pks_session_compile(
    engine: PksSessionHandle,
    session: PksSessionHandle,
) -> PksSessionStatus {
    abi_call(|| {
        runtime_state().compile_session(engine, session)?;
        Ok(PksSessionStatus::ok())
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn pks_session_start(
    engine: PksSessionHandle,
    session: PksSessionHandle,
) -> PksSessionStatus {
    abi_call(|| {
        runtime_state().start_session(engine, session)?;
        Ok(PksSessionStatus::ok())
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn pks_session_stop(
    engine: PksSessionHandle,
    session: PksSessionHandle,
) -> PksSessionStatus {
    abi_call(|| {
        runtime_state().stop_session(engine, session)?;
        Ok(PksSessionStatus::ok())
    })
}

#[unsafe(no_mangle)]
/// Returns the authoritative state owned by the real Session value.
///
/// # Safety
///
/// `output_state` must address one writable, aligned `uint32_t`.
pub unsafe extern "C" fn pks_session_get_state(
    engine: PksSessionHandle,
    session: PksSessionHandle,
    output_state: *mut u32,
) -> PksSessionStatus {
    abi_call(|| {
        guard_pointer_alignment(output_state.cast_const())?;
        let state = runtime_state().session_state(engine, session)?;
        // SAFETY: This export forwards the documented caller contract.
        unsafe { write_record(output_state, state as u32) }?;
        Ok(PksSessionStatus::ok())
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn pks_session_destroy(
    engine: PksSessionHandle,
    session: PksSessionHandle,
) -> PksSessionStatus {
    abi_call(|| {
        runtime_state().destroy_session(engine, session)?;
        Ok(PksSessionStatus::ok())
    })
}

#[unsafe(no_mangle)]
/// Polls one event from the Session's bounded control queue.
///
/// # Safety
///
/// `output_event` must address one writable, aligned `PksSessionEvent`.
pub unsafe extern "C" fn pks_session_event_poll(
    engine: PksSessionHandle,
    session: PksSessionHandle,
    output_event: *mut PksSessionEvent,
) -> PksSessionStatus {
    abi_call(|| {
        guard_pointer_alignment(output_event.cast_const())?;
        let event = runtime_state().with_engine(engine, |runtime| runtime.poll_event(session))?;
        // SAFETY: This export forwards the documented caller contract.
        unsafe { write_record(output_event, event) }?;
        Ok(PksSessionStatus::ok())
    })
}

#[unsafe(no_mangle)]
/// Copies one point-in-time bounded queue and lease observation snapshot.
///
/// # Safety
///
/// `output_metrics` must address one writable, aligned snapshot record.
pub unsafe extern "C" fn pks_session_metrics_poll(
    engine: PksSessionHandle,
    session: PksSessionHandle,
    output_metrics: *mut PksSessionMetricsSnapshot,
) -> PksSessionStatus {
    abi_call(|| {
        guard_pointer_alignment(output_metrics.cast_const())?;
        let metrics = runtime_state().with_engine(engine, |runtime| runtime.metrics(session))?;
        let events = metrics.event_queue();
        let audio = metrics.polled_audio();
        let snapshot = PksSessionMetricsSnapshot {
            struct_size_bytes: size_of::<PksSessionMetricsSnapshot>() as u32,
            abi_major: PKS_SESSION_ABI_MAJOR,
            abi_minor: PKS_SESSION_ABI_MINOR,
            reserved: 0,
            event_capacity_count: events.capacity_event_count,
            event_depth_count: events.depth_events,
            event_peak_depth_count: events.peak_depth_event_count,
            events_enqueued_total: events.events_enqueued_total,
            events_dropped_total: events.events_dropped_total,
            event_receiver_closed_total: events.receiver_closed_total,
            audio_queue_capacity_frames: audio.queue_capacity_frames,
            audio_queue_depth_frames: audio.queue_depth_frames,
            audio_queue_peak_frames: audio.queue_peak_frames,
            audio_frames_received_total: audio.frames_received_total,
            audio_frames_delivered_total: audio.frames_delivered_total,
            audio_queue_full_drops_total: audio.queue_full_drops_total,
            audio_invalid_ownership_drops_total: audio.invalid_ownership_drops_total,
            audio_lease_capacity_count: audio.lease_capacity_count,
            audio_outstanding_leases_count: audio.outstanding_leases,
            audio_lease_exhausted_total: audio.lease_exhausted_total,
            audio_batches_polled_total: audio.batches_polled_total,
            audio_frames_polled_total: audio.frames_polled_total,
        };
        // SAFETY: This export forwards the documented caller contract.
        unsafe { write_record(output_metrics, snapshot) }?;
        Ok(PksSessionStatus::ok())
    })
}

#[unsafe(no_mangle)]
/// Copies the number of source observation records available by index.
///
/// # Safety
///
/// `output_count` must address one writable, aligned `uint32_t`.
pub unsafe extern "C" fn pks_session_source_metrics_count(
    engine: PksSessionHandle,
    session: PksSessionHandle,
    output_count: *mut u32,
) -> PksSessionStatus {
    abi_call(|| {
        guard_pointer_alignment(output_count.cast_const())?;
        let metrics = runtime_state().with_engine(engine, |runtime| runtime.metrics(session))?;
        let count = u32::try_from(metrics.source_count()).map_err(|_| AbiError::InvalidArgument)?;
        // SAFETY: This export forwards the documented caller contract.
        unsafe { write_record(output_count, count) }?;
        Ok(PksSessionStatus::ok())
    })
}

#[unsafe(no_mangle)]
/// Copies the number of route observation records available by index.
///
/// # Safety
///
/// `output_count` must address one writable, aligned `uint32_t`.
pub unsafe extern "C" fn pks_session_route_metrics_count(
    engine: PksSessionHandle,
    session: PksSessionHandle,
    output_count: *mut u32,
) -> PksSessionStatus {
    abi_call(|| {
        guard_pointer_alignment(output_count.cast_const())?;
        let metrics = runtime_state().with_engine(engine, |runtime| runtime.metrics(session))?;
        let count = u32::try_from(metrics.route_count()).map_err(|_| AbiError::InvalidArgument)?;
        // SAFETY: This export forwards the documented caller contract.
        unsafe { write_record(output_count, count) }?;
        Ok(PksSessionStatus::ok())
    })
}

#[unsafe(no_mangle)]
/// Copies one source observation record by stable Session declaration index.
///
/// # Safety
///
/// `output_metrics` must address one writable, aligned source metrics record.
pub unsafe extern "C" fn pks_session_source_metrics_at(
    engine: PksSessionHandle,
    session: PksSessionHandle,
    source_index: u32,
    output_metrics: *mut PksSessionSourceMetrics,
) -> PksSessionStatus {
    abi_call(|| {
        guard_pointer_alignment(output_metrics.cast_const())?;
        let metrics = runtime_state().with_engine(engine, |runtime| runtime.metrics(session))?;
        let source = metrics
            .source(source_index as usize)
            .ok_or(AbiError::IndexOutOfRange)?;
        let capture = source.capture;
        let ingress = source.ingress;
        let record = PksSessionSourceMetrics {
            struct_size_bytes: size_of::<PksSessionSourceMetrics>() as u32,
            abi_major: PKS_SESSION_ABI_MAJOR,
            abi_minor: PKS_SESSION_ABI_MINOR,
            stem_id: source.stem_id.0,
            capture_callback_buffers_total: capture.backend.callback_buffers_total,
            capture_frames_enqueued_total: capture.backend.frames_enqueued_total,
            capture_pool_exhausted_total: capture.backend.pool_exhausted_total,
            capture_dispatch_queue_full_total: capture.backend.dispatch_queue_full_total,
            capture_invalid_buffer_total: capture.backend.invalid_buffer_total,
            capture_oversized_buffer_total: capture.backend.oversized_buffer_total,
            capture_stream_errors_total: capture.backend.stream_errors_total,
            capture_timestamp_epoch_clamps_total: capture.backend.timestamp_epoch_clamps_total,
            capture_stream_delivered_frames_total: capture.frame_stream.delivered_frames,
            capture_stream_dropped_newest_frames_total: capture.frame_stream.dropped_newest_frames,
            capture_runtime_events_enqueued_total: capture.runtime_events.events_enqueued_total,
            capture_runtime_events_dropped_total: capture.runtime_events.events_dropped_total,
            ingress_queue_capacity_frames: ingress.queue_capacity_frames,
            ingress_queue_depth_frames: ingress.queue_depth_frames,
            ingress_queue_peak_frames: ingress.queue_peak_frames,
            ingress_frames_enqueued_total: ingress.frames_enqueued_total,
            ingress_frames_delivered_total: ingress.frames_delivered_total,
            ingress_frames_rejected_full_total: ingress.frames_rejected_full_total,
            ingress_frames_rejected_cancelled_total: ingress.frames_rejected_cancelled_total,
            ingress_frames_discarded_total: ingress.frames_discarded_total,
        };
        // SAFETY: This export forwards the documented caller contract.
        unsafe { write_record(output_metrics, record) }?;
        Ok(PksSessionStatus::ok())
    })
}

#[unsafe(no_mangle)]
/// Copies one route observation record by stable Session declaration index.
///
/// # Safety
///
/// `output_metrics` must address one writable, aligned route metrics record.
pub unsafe extern "C" fn pks_session_route_metrics_at(
    engine: PksSessionHandle,
    session: PksSessionHandle,
    route_index: u32,
    output_metrics: *mut PksSessionRouteMetrics,
) -> PksSessionStatus {
    abi_call(|| {
        guard_pointer_alignment(output_metrics.cast_const())?;
        let metrics = runtime_state().with_engine(engine, |runtime| runtime.metrics(session))?;
        let route = metrics
            .route(route_index as usize)
            .ok_or(AbiError::IndexOutOfRange)?;
        let edge = route.edge;
        let endpoint = route.endpoint.unwrap_or_default();
        let record = PksSessionRouteMetrics {
            struct_size_bytes: size_of::<PksSessionRouteMetrics>() as u32,
            abi_major: PKS_SESSION_ABI_MAJOR,
            abi_minor: PKS_SESSION_ABI_MINOR,
            route_id: route.route_id.0,
            endpoint_id: route.endpoint_id.0,
            edge_queue_capacity_frames: edge.queue_capacity_frames,
            edge_queue_depth_frames: edge.queue_depth_frames,
            edge_queue_peak_frames: edge.queue_peak_frames,
            edge_frames_enqueued_total: edge.frames_enqueued_total,
            edge_frames_delivered_total: edge.frames_delivered_total,
            edge_frames_dropped_total: edge.frames_dropped_total,
            edge_overruns_total: edge.overruns_total,
            edge_receiver_unavailable_drops_total: edge.receiver_unavailable_drops_total,
            edge_queue_full_drops_total: edge.queue_full_drops_total,
            edge_shared_reference_exhausted_drops_total: edge
                .shared_reference_exhausted_drops_total,
            edge_branch_pool_exhausted_drops_total: edge.branch_pool_exhausted_drops_total,
            edge_invalid_copy_policy_drops_total: edge.invalid_copy_policy_drops_total,
            edge_freeze_failed_drops_total: edge.freeze_failed_drops_total,
            edge_discontinuities_total: edge.discontinuities_total,
            edge_source_identity_discontinuities_total: edge.source_identity_discontinuities_total,
            edge_sequence_discontinuities_total: edge.sequence_discontinuities_total,
            edge_timestamp_discontinuities_total: edge.timestamp_discontinuities_total,
            edge_lineage_epoch_discontinuities_total: edge.lineage_epoch_discontinuities_total,
            edge_manually_reported_discontinuities_total: edge
                .manually_reported_discontinuities_total,
            edge_enqueue_to_receive_samples_total: edge.enqueue_to_receive_samples_total,
            edge_enqueue_to_receive_invalid_order_total: edge
                .enqueue_to_receive_invalid_order_total,
            edge_enqueue_to_receive_p50_ns: edge.enqueue_to_receive_p50_ns,
            edge_enqueue_to_receive_p95_ns: edge.enqueue_to_receive_p95_ns,
            edge_enqueue_to_receive_p99_ns: edge.enqueue_to_receive_p99_ns,
            edge_enqueue_to_receive_max_ns: edge.enqueue_to_receive_max_ns,
            edge_source_timestamp_to_receive_samples_total: edge
                .source_timestamp_to_receive_samples_total,
            edge_source_timestamp_to_receive_missing_total: edge
                .source_timestamp_to_receive_missing_total,
            edge_source_timestamp_to_receive_future_total: edge
                .source_timestamp_to_receive_future_total,
            edge_source_timestamp_to_receive_p50_ns: edge.source_timestamp_to_receive_p50_ns,
            edge_source_timestamp_to_receive_p95_ns: edge.source_timestamp_to_receive_p95_ns,
            edge_source_timestamp_to_receive_p99_ns: edge.source_timestamp_to_receive_p99_ns,
            edge_source_timestamp_to_receive_max_ns: edge.source_timestamp_to_receive_max_ns,
            edge_worker_failures_total: edge.worker_failures_total,
            edge_shutdown_discarded_total: edge.shutdown_discarded_total,
            endpoint_frames_received_total: endpoint.frames_received_total,
            endpoint_frames_delivered_total: endpoint.frames_delivered_total,
            endpoint_frames_dropped_total: endpoint.frames_dropped_total,
            endpoint_discontinuities_total: endpoint.discontinuities_total,
            endpoint_failures_total: endpoint.failures_total,
            endpoint_observation_stage: match route.endpoint_observation_stage {
                pks_session::EndpointObservationStage::Unavailable => {
                    PksSessionEndpointObservationStage::Unavailable as u32
                }
                pks_session::EndpointObservationStage::Live => {
                    PksSessionEndpointObservationStage::Live as u32
                }
                pks_session::EndpointObservationStage::Finalized => {
                    PksSessionEndpointObservationStage::Finalized as u32
                }
            },
            reserved: 0,
            endpoint_finalization_failures_total: route.endpoint_finalization_failures_total,
        };
        // SAFETY: This export forwards the documented caller contract.
        unsafe { write_record(output_metrics, record) }?;
        Ok(PksSessionStatus::ok())
    })
}

#[unsafe(no_mangle)]
/// Acquires one immutable bounded audio batch lease.
///
/// # Safety
///
/// `output_batch` must address one writable, aligned batch record.
pub unsafe extern "C" fn pks_session_audio_poll(
    engine: PksSessionHandle,
    session: PksSessionHandle,
    output_batch: *mut PksSessionAudioBatch,
) -> PksSessionStatus {
    abi_call(|| {
        guard_pointer_alignment(output_batch.cast_const())?;
        let (handle, frame_count) =
            runtime_state().with_engine_mut(engine, |runtime| runtime.poll_audio(session))?;
        let batch = PksSessionAudioBatch {
            struct_size_bytes: size_of::<PksSessionAudioBatch>() as u32,
            abi_major: PKS_SESSION_ABI_MAJOR,
            abi_minor: PKS_SESSION_ABI_MINOR,
            frame_count,
            reserved: 0,
            handle,
        };
        // SAFETY: This export forwards the documented caller contract.
        unsafe { write_record(output_batch, batch) }?;
        Ok(PksSessionStatus::ok())
    })
}

#[unsafe(no_mangle)]
/// Returns one frame descriptor whose samples remain valid until batch release.
///
/// # Safety
///
/// `output_frame` must address one writable, aligned frame record.
pub unsafe extern "C" fn pks_session_audio_batch_frame(
    engine: PksSessionHandle,
    batch: PksSessionHandle,
    frame_index: u32,
    output_frame: *mut PksSessionAudioFrame,
) -> PksSessionStatus {
    abi_call(|| {
        guard_pointer_alignment(output_frame.cast_const())?;
        let frame = runtime_state()
            .with_engine(engine, |runtime| runtime.audio_frame(batch, frame_index))?;
        // SAFETY: This export forwards the documented caller contract.
        unsafe { write_record(output_frame, frame) }?;
        Ok(PksSessionStatus::ok())
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn pks_session_audio_batch_release(
    engine: PksSessionHandle,
    batch: PksSessionHandle,
) -> PksSessionStatus {
    abi_call(|| {
        runtime_state().with_engine_mut(engine, |runtime| runtime.release_audio(batch))?;
        Ok(PksSessionStatus::ok())
    })
}

#[cfg(test)]
mod tests {
    use std::mem::{offset_of, size_of};
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::{Arc, Condvar, Mutex};
    use std::time::{Duration, Instant};

    use pks_capture::{
        ActiveCaptureBackend, CallbackCaptureBackend, CaptureDelivery, CaptureError, CaptureMode,
        CaptureObservationHandle, CaptureObservations, CapturedFrameDelivery,
        PreparedCaptureBackend,
    };
    use pks_frame::{AudioBufferPool, AudioFrame, SampleFormat, SampleSpec, SourceId, StreamId};
    use pks_graph::PrepareContext;
    use pks_session::{PolledAudioEndpointConfig, SessionEngineHostBuilder, SessionStartOptions};

    use super::{
        abi_call, pks_session_abi_get_version, pks_session_abi_is_compatible,
        pks_session_audio_batch_frame, pks_session_audio_batch_release, pks_session_audio_poll,
        pks_session_compile, pks_session_create_app_mic, pks_session_destroy,
        pks_session_engine_create, pks_session_engine_destroy, pks_session_engine_is_live,
        pks_session_get_state, pks_session_metrics_poll, pks_session_route_metrics_at,
        pks_session_route_metrics_count, pks_session_source_metrics_at,
        pks_session_source_metrics_count, pks_session_start, pks_session_stop, runtime_state,
        PksSessionAbiVersion, PksSessionAppMicDeclaration, PksSessionAudioBatch,
        PksSessionAudioFrame, PksSessionEngineConfig, PksSessionHandle, PksSessionLifecycleState,
        PksSessionMetricsSnapshot, PksSessionRouteMetrics, PksSessionSampleFormat,
        PksSessionSourceMetrics, PksSessionStatusCode, PksSessionUtf8, PKS_SESSION_ABI_MAJOR,
        PKS_SESSION_ABI_MINOR,
    };

    struct DeliveringCaptureBackend;

    struct DeliveringPreparedCapture;

    struct DeliveringActiveCapture {
        stop_requested: Arc<AtomicBool>,
        worker: Option<std::thread::JoinHandle<()>>,
    }

    impl DeliveringActiveCapture {
        fn idle() -> Self {
            Self {
                stop_requested: Arc::new(AtomicBool::new(false)),
                worker: None,
            }
        }
    }

    #[derive(Default)]
    struct BlockingOpenState {
        entered: bool,
        released: bool,
    }

    struct BlockingCaptureBackend {
        state: Arc<(Mutex<BlockingOpenState>, Condvar)>,
    }

    struct BlockingPreparedCapture {
        state: Arc<(Mutex<BlockingOpenState>, Condvar)>,
    }

    impl CallbackCaptureBackend for DeliveringCaptureBackend {
        fn prepare(
            &self,
            _mode: CaptureMode,
        ) -> Result<Box<dyn PreparedCaptureBackend>, CaptureError> {
            Ok(Box::new(DeliveringPreparedCapture))
        }
    }

    impl PreparedCaptureBackend for DeliveringPreparedCapture {
        fn open(
            self: Box<Self>,
            mut delivery: CaptureDelivery,
        ) -> Result<Box<dyn ActiveCaptureBackend>, CaptureError> {
            let pool = AudioBufferPool::new(1, 4);
            let mut buffer = pool
                .acquire()
                .ok_or_else(|| CaptureError::BackendInit("test buffer unavailable".to_owned()))?;
            buffer.copy_from_slice(&[0.125, 0.25, 0.5, 1.0]);
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
                    buffer.copy_from_slice(&[0.125, 0.25, 0.5, 1.0]);
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
            Ok(Box::new(DeliveringActiveCapture {
                stop_requested,
                worker: Some(worker),
            }))
        }
    }

    impl ActiveCaptureBackend for DeliveringActiveCapture {
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
                        worker: "C ABI test capture worker",
                    })?;
            }
            Ok(CaptureObservations::default())
        }
    }

    impl Drop for DeliveringActiveCapture {
        fn drop(&mut self) {
            self.stop_requested.store(true, Ordering::Release);
            if let Some(worker) = self.worker.take() {
                let _ = worker.join();
            }
        }
    }

    impl CallbackCaptureBackend for BlockingCaptureBackend {
        fn prepare(
            &self,
            _mode: CaptureMode,
        ) -> Result<Box<dyn PreparedCaptureBackend>, CaptureError> {
            Ok(Box::new(BlockingPreparedCapture {
                state: Arc::clone(&self.state),
            }))
        }
    }

    impl PreparedCaptureBackend for BlockingPreparedCapture {
        fn open(
            self: Box<Self>,
            _delivery: CaptureDelivery,
        ) -> Result<Box<dyn ActiveCaptureBackend>, CaptureError> {
            let (state, changed) = &*self.state;
            let mut state = state
                .lock()
                .map_err(|_| CaptureError::BackendInit("blocking state poisoned".to_owned()))?;
            state.entered = true;
            changed.notify_all();
            while !state.released {
                state = changed
                    .wait(state)
                    .map_err(|_| CaptureError::BackendInit("blocking state poisoned".to_owned()))?;
            }
            Ok(Box::new(DeliveringActiveCapture::idle()))
        }
    }

    fn deterministic_engine(lease_capacity_count: usize) -> PksSessionHandle {
        let prepare_context =
            PrepareContext::new(SampleSpec::new(48_000, 1, SampleFormat::F32Interleaved));
        let mut builder =
            SessionEngineHostBuilder::new(prepare_context, 8, SessionStartOptions::default())
                .expect("deterministic host builder");
        let capture_backend: Arc<dyn CallbackCaptureBackend> = Arc::new(DeliveringCaptureBackend);
        builder
            .set_application_backend(Arc::clone(&capture_backend))
            .set_microphone_backend(capture_backend);
        let _ = builder
            .register_polled_audio_endpoint(PolledAudioEndpointConfig {
                queue_capacity_frames: 8,
                max_batch_frames: 1,
                max_outstanding_leases: lease_capacity_count,
            })
            .expect("deterministic polled endpoint");
        runtime_state()
            .allocate_engine_with_host(
                builder.build().expect("deterministic Session host"),
                lease_capacity_count,
            )
            .expect("deterministic engine")
    }

    fn blocking_engine(state: Arc<(Mutex<BlockingOpenState>, Condvar)>) -> PksSessionHandle {
        let prepare_context =
            PrepareContext::new(SampleSpec::new(48_000, 1, SampleFormat::F32Interleaved));
        let mut builder =
            SessionEngineHostBuilder::new(prepare_context, 8, SessionStartOptions::default())
                .expect("blocking host builder");
        let capture_backend: Arc<dyn CallbackCaptureBackend> =
            Arc::new(BlockingCaptureBackend { state });
        builder
            .set_application_backend(Arc::clone(&capture_backend))
            .set_microphone_backend(capture_backend);
        let _ = builder
            .register_polled_audio_endpoint(PolledAudioEndpointConfig::default())
            .expect("blocking polled endpoint");
        runtime_state()
            .allocate_engine_with_host(
                builder.build().expect("blocking Session host"),
                PolledAudioEndpointConfig::default().max_outstanding_leases,
            )
            .expect("blocking engine")
    }

    fn app_mic_declaration() -> PksSessionAppMicDeclaration {
        static APPLICATION: &[u8] = b"deterministic application";
        PksSessionAppMicDeclaration::new(PksSessionUtf8 {
            data: APPLICATION.as_ptr(),
            len_bytes: APPLICATION.len() as u32,
        })
    }

    fn poll_audio_until_ready(
        engine: PksSessionHandle,
        session: PksSessionHandle,
    ) -> PksSessionAudioBatch {
        let deadline = Instant::now() + Duration::from_secs(1);
        loop {
            let mut batch = PksSessionAudioBatch {
                struct_size_bytes: 0,
                abi_major: 0,
                abi_minor: 0,
                frame_count: 0,
                reserved: 0,
                handle: PksSessionHandle::invalid(),
            };
            // SAFETY: The test supplies one valid writable batch record.
            let status = unsafe { pks_session_audio_poll(engine, session, &mut batch) };
            if status.code == PksSessionStatusCode::Ok as u32 {
                return batch;
            }
            assert_eq!(status.code, PksSessionStatusCode::WouldBlock as u32);
            assert!(
                Instant::now() < deadline,
                "deterministic audio did not reach the ABI lease"
            );
            std::thread::sleep(Duration::from_millis(1));
        }
    }

    #[test]
    fn given_native_engine_when_created_then_real_session_declaration_compiles() {
        let config = PksSessionEngineConfig::default();
        let mut engine = PksSessionHandle::invalid();
        let application = b"PocketStation Control Application Source";
        let declaration = PksSessionAppMicDeclaration::new(PksSessionUtf8 {
            data: application.as_ptr(),
            len_bytes: application.len() as u32,
        });
        let mut session = PksSessionHandle::invalid();
        let mut state = u32::MAX;

        // SAFETY: The test passes valid aligned records and output handles.
        let create_engine = unsafe { pks_session_engine_create(&config, &mut engine) };
        // SAFETY: The UTF-8 bytes and output handle remain valid for the call.
        let create_session =
            unsafe { pks_session_create_app_mic(engine, &declaration, &mut session) };
        let compile = pks_session_compile(engine, session);
        // SAFETY: The output state is valid and aligned for the call.
        let read_state = unsafe { pks_session_get_state(engine, session, &mut state) };
        let destroy_session = pks_session_destroy(engine, session);
        let destroy_engine = pks_session_engine_destroy(engine);

        assert_eq!(create_engine.code, PksSessionStatusCode::Ok as u32);
        assert_eq!(create_session.code, PksSessionStatusCode::Ok as u32);
        assert_eq!(compile.code, PksSessionStatusCode::Ok as u32);
        assert_eq!(read_state.code, PksSessionStatusCode::Ok as u32);
        assert_eq!(state, PksSessionLifecycleState::Compiled as u32);
        assert_eq!(destroy_session.code, PksSessionStatusCode::Ok as u32);
        assert_eq!(destroy_engine.code, PksSessionStatusCode::Ok as u32);
    }

    #[test]
    fn given_destroyed_engine_when_reused_then_stale_handle_is_rejected() {
        let config = PksSessionEngineConfig::default();
        let mut engine = PksSessionHandle::invalid();
        let mut is_live = 0_u32;

        // SAFETY: The test passes valid aligned records and output values.
        let create = unsafe { pks_session_engine_create(&config, &mut engine) };
        let destroy = pks_session_engine_destroy(engine);
        let second_destroy = pks_session_engine_destroy(engine);
        // SAFETY: The output live flag is valid and aligned for the call.
        let live = unsafe { pks_session_engine_is_live(engine, &mut is_live) };

        assert_eq!(create.code, PksSessionStatusCode::Ok as u32);
        assert_eq!(destroy.code, PksSessionStatusCode::Ok as u32);
        assert_eq!(
            second_destroy.code,
            PksSessionStatusCode::StaleHandle as u32
        );
        assert_eq!(live.code, PksSessionStatusCode::StaleHandle as u32);
    }

    #[test]
    fn given_abi_query_when_version_requested_then_struct_is_returned() {
        let mut version = PksSessionAbiVersion {
            struct_size_bytes: 0,
            abi_major: 0,
            abi_minor: 0,
        };

        // SAFETY: The test passes a valid writable version record.
        let status = unsafe { pks_session_abi_get_version(&mut version) };

        assert_eq!(status.code, PksSessionStatusCode::Ok as u32);
        assert_eq!(version.abi_major, PKS_SESSION_ABI_MAJOR);
    }

    #[test]
    fn given_wrong_major_when_checked_then_compatibility_fails() {
        let status = pks_session_abi_is_compatible(
            PKS_SESSION_ABI_MAJOR.saturating_add(1),
            PKS_SESSION_ABI_MINOR,
            size_of::<PksSessionAbiVersion>() as u32,
        );

        assert_eq!(
            status.code,
            PksSessionStatusCode::UnsupportedAbiMajor as u32
        );
    }

    #[test]
    fn given_checked_header_layout_when_compared_then_all_public_records_match() {
        assert_eq!(size_of::<PksSessionAbiVersion>(), 8);
        assert_eq!(size_of::<PksSessionHandle>(), 24);
        assert_eq!(size_of::<super::PksSessionStatus>(), 8);
        assert_eq!(size_of::<PksSessionUtf8>(), 16);
        assert_eq!(size_of::<PksSessionEngineConfig>(), 56);
        assert_eq!(size_of::<PksSessionAppMicDeclaration>(), 24);
        assert_eq!(size_of::<super::PksSessionEvent>(), 64);
        assert_eq!(size_of::<PksSessionMetricsSnapshot>(), 160);
        assert_eq!(size_of::<PksSessionSourceMetrics>(), 176);
        assert_eq!(size_of::<PksSessionRouteMetrics>(), 352);
        assert_eq!(size_of::<PksSessionAudioBatch>(), 40);
        assert_eq!(size_of::<PksSessionAudioFrame>(), 144);
        assert_eq!(offset_of!(super::PksSessionEvent, session_id), 24);
        assert_eq!(
            offset_of!(PksSessionMetricsSnapshot, event_capacity_count),
            16
        );
        assert_eq!(offset_of!(PksSessionAudioBatch, handle), 16);
        assert_eq!(offset_of!(PksSessionAudioFrame, session_id), 24);
        assert_eq!(offset_of!(PksSessionAudioFrame, samples), 128);
    }

    #[test]
    fn given_deterministic_session_when_polled_then_audio_lease_is_bounded_and_stable() {
        let engine = deterministic_engine(1);
        let declaration = app_mic_declaration();
        let mut session = PksSessionHandle::invalid();

        // SAFETY: The declaration and output handle are valid for this call.
        let create = unsafe { pks_session_create_app_mic(engine, &declaration, &mut session) };
        let compile = pks_session_compile(engine, session);
        let start = pks_session_start(engine, session);
        let batch = poll_audio_until_ready(engine, session);
        let mut frame = PksSessionAudioFrame {
            struct_size_bytes: 0,
            abi_major: 0,
            abi_minor: 0,
            sample_format: PksSessionSampleFormat::F32Interleaved,
            sample_rate_hz: 0,
            channel_count: 0,
            reserved: 0,
            session_id: 0,
            source_id: 0,
            stem_id: 0,
            clock_id: 0,
            sequence_num: 0,
            timestamp_start_ns: 0,
            duration_ns: 0,
            source_generation: 0,
            discontinuity_epoch: 0,
            permission_epoch: 0,
            endpoint_id: 0,
            connector_id: 0,
            route_id: 0,
            samples: std::ptr::null(),
            sample_count: 0,
            reserved_tail: 0,
        };
        // SAFETY: The test holds the batch lease and supplies a writable frame record.
        let read_frame =
            unsafe { pks_session_audio_batch_frame(engine, batch.handle, 0, &mut frame) };
        let sample_pointer = frame.samples;
        // SAFETY: The successful frame descriptor and held lease guarantee one sample.
        let first_sample = unsafe { *sample_pointer };
        let mut exhausted_batch = PksSessionAudioBatch {
            struct_size_bytes: 0,
            abi_major: 0,
            abi_minor: 0,
            frame_count: 0,
            reserved: 0,
            handle: PksSessionHandle::invalid(),
        };
        // SAFETY: The test supplies one valid writable batch record.
        let exhausted = unsafe { pks_session_audio_poll(engine, session, &mut exhausted_batch) };
        let mut metrics = PksSessionMetricsSnapshot {
            struct_size_bytes: 0,
            abi_major: 0,
            abi_minor: 0,
            reserved: 0,
            event_capacity_count: 0,
            event_depth_count: 0,
            event_peak_depth_count: 0,
            events_enqueued_total: 0,
            events_dropped_total: 0,
            event_receiver_closed_total: 0,
            audio_queue_capacity_frames: 0,
            audio_queue_depth_frames: 0,
            audio_queue_peak_frames: 0,
            audio_frames_received_total: 0,
            audio_frames_delivered_total: 0,
            audio_queue_full_drops_total: 0,
            audio_invalid_ownership_drops_total: 0,
            audio_lease_capacity_count: 0,
            audio_outstanding_leases_count: 0,
            audio_lease_exhausted_total: 0,
            audio_batches_polled_total: 0,
            audio_frames_polled_total: 0,
        };
        // SAFETY: The test supplies one valid writable metrics record.
        let metrics_status = unsafe { pks_session_metrics_poll(engine, session, &mut metrics) };
        let mut source_metrics_count = 0;
        let mut route_metrics_count = 0;
        // SAFETY: The test supplies valid writable count records.
        let source_metrics_count_status =
            unsafe { pks_session_source_metrics_count(engine, session, &mut source_metrics_count) };
        // SAFETY: The test supplies valid writable count records.
        let route_metrics_count_status =
            unsafe { pks_session_route_metrics_count(engine, session, &mut route_metrics_count) };
        // SAFETY: Both records contain only integer fields and zero is a valid
        // initialization for every field before the ABI overwrites the record.
        let mut source_metrics = unsafe { std::mem::zeroed::<PksSessionSourceMetrics>() };
        // SAFETY: See the preceding record initialization invariant.
        let mut route_metrics = unsafe { std::mem::zeroed::<PksSessionRouteMetrics>() };
        // SAFETY: The test supplies valid writable indexed metric records.
        let source_metrics_status =
            unsafe { pks_session_source_metrics_at(engine, session, 0, &mut source_metrics) };
        // SAFETY: The test supplies valid writable indexed metric records.
        let route_metrics_status =
            unsafe { pks_session_route_metrics_at(engine, session, 0, &mut route_metrics) };
        // SAFETY: The output pointer is valid; index two is beyond two sources.
        let source_out_of_range =
            unsafe { pks_session_source_metrics_at(engine, session, 2, &mut source_metrics) };
        let stop = pks_session_stop(engine, session);
        // SAFETY: Final indexed observations remain readable while Session lives.
        let final_route_metrics =
            unsafe { pks_session_route_metrics_at(engine, session, 0, &mut route_metrics) };
        let mut frame_after_stop = frame;
        // SAFETY: The held batch lease remains valid across Session stop.
        let read_after_stop = unsafe {
            pks_session_audio_batch_frame(engine, batch.handle, 0, &mut frame_after_stop)
        };
        let release = pks_session_audio_batch_release(engine, batch.handle);
        let second_release = pks_session_audio_batch_release(engine, batch.handle);
        let destroy_session = pks_session_destroy(engine, session);
        // SAFETY: The output record is writable; the destroyed Session handle
        // must be rejected before any indexed observation access.
        let stale_route_metrics =
            unsafe { pks_session_route_metrics_at(engine, session, 0, &mut route_metrics) };
        let mut replacement_session = PksSessionHandle::invalid();
        // SAFETY: The declaration and output handle are valid for this call.
        let recreate =
            unsafe { pks_session_create_app_mic(engine, &declaration, &mut replacement_session) };
        let destroy_engine = pks_session_engine_destroy(engine);

        assert_eq!(create.code, PksSessionStatusCode::Ok as u32);
        assert_eq!(compile.code, PksSessionStatusCode::Ok as u32);
        assert_eq!(start.code, PksSessionStatusCode::Ok as u32);
        assert_eq!(batch.frame_count, 1);
        assert_eq!(read_frame.code, PksSessionStatusCode::Ok as u32);
        assert_eq!(frame.sample_format, PksSessionSampleFormat::F32Interleaved);
        assert_eq!(frame.sample_count, 4);
        assert_eq!(first_sample, 0.125);
        assert_eq!(exhausted.code, PksSessionStatusCode::NoCapacity as u32);
        assert_eq!(metrics_status.code, PksSessionStatusCode::Ok as u32);
        assert_eq!(metrics.audio_lease_capacity_count, 1);
        assert_eq!(metrics.audio_outstanding_leases_count, 1);
        assert_eq!(metrics.audio_lease_exhausted_total, 1);
        assert_eq!(
            source_metrics_count_status.code,
            PksSessionStatusCode::Ok as u32
        );
        assert_eq!(
            route_metrics_count_status.code,
            PksSessionStatusCode::Ok as u32
        );
        assert_eq!(source_metrics_count, 2);
        assert_eq!(route_metrics_count, 2);
        assert_eq!(source_metrics_status.code, PksSessionStatusCode::Ok as u32);
        assert_ne!(source_metrics.stem_id, 0);
        assert_eq!(route_metrics_status.code, PksSessionStatusCode::Ok as u32);
        assert_ne!(route_metrics.route_id, 0);
        assert_ne!(route_metrics.endpoint_id, 0);
        assert_eq!(
            source_out_of_range.code,
            PksSessionStatusCode::IndexOutOfRange as u32
        );
        assert_eq!(stop.code, PksSessionStatusCode::Ok as u32);
        assert_eq!(final_route_metrics.code, PksSessionStatusCode::Ok as u32);
        assert_eq!(read_after_stop.code, PksSessionStatusCode::Ok as u32);
        assert_eq!(frame_after_stop.samples, sample_pointer);
        assert_eq!(release.code, PksSessionStatusCode::Ok as u32);
        assert_eq!(
            second_release.code,
            PksSessionStatusCode::StaleHandle as u32
        );
        assert_eq!(destroy_session.code, PksSessionStatusCode::Ok as u32);
        assert_eq!(
            stale_route_metrics.code,
            PksSessionStatusCode::StaleHandle as u32
        );
        assert_eq!(recreate.code, PksSessionStatusCode::NoCapacity as u32);
        assert_eq!(destroy_engine.code, PksSessionStatusCode::Ok as u32);
    }

    #[test]
    fn given_blocking_start_when_stopped_concurrently_then_cancellation_is_bounded_and_truthful() {
        let blocking_state = Arc::new((Mutex::new(BlockingOpenState::default()), Condvar::new()));
        let engine = blocking_engine(Arc::clone(&blocking_state));
        let declaration = app_mic_declaration();
        let mut session = PksSessionHandle::invalid();
        // SAFETY: The declaration and output handle are valid for this call.
        let create = unsafe { pks_session_create_app_mic(engine, &declaration, &mut session) };
        let compile = pks_session_compile(engine, session);
        assert_eq!(create.code, PksSessionStatusCode::Ok as u32);
        assert_eq!(compile.code, PksSessionStatusCode::Ok as u32);

        let start_thread = std::thread::spawn(move || pks_session_start(engine, session));
        let (state, changed) = &*blocking_state;
        let mut open_state = state.lock().expect("blocking state lock");
        while !open_state.entered {
            open_state = changed.wait(open_state).expect("blocking state wait");
        }
        drop(open_state);

        let mut lifecycle_state = u32::MAX;
        // SAFETY: The output state is valid and aligned for the call.
        let starting = unsafe { pks_session_get_state(engine, session, &mut lifecycle_state) };
        assert_eq!(starting.code, PksSessionStatusCode::Ok as u32);
        assert_eq!(lifecycle_state, PksSessionLifecycleState::Starting as u32);

        let stop_thread = std::thread::spawn(move || pks_session_stop(engine, session));
        let state_deadline = Instant::now() + Duration::from_secs(1);
        loop {
            // SAFETY: The output state is valid and aligned for the call.
            let status = unsafe { pks_session_get_state(engine, session, &mut lifecycle_state) };
            assert_eq!(status.code, PksSessionStatusCode::Ok as u32);
            if lifecycle_state == PksSessionLifecycleState::Stopping as u32 {
                break;
            }
            assert!(
                Instant::now() < state_deadline,
                "concurrent stop did not publish Stopping"
            );
            std::thread::yield_now();
        }

        let release_started_at = Instant::now();
        let mut open_state = state.lock().expect("blocking state lock");
        open_state.released = true;
        changed.notify_all();
        drop(open_state);

        let start = start_thread.join().expect("start thread");
        let stop = stop_thread.join().expect("stop thread");
        assert!(
            release_started_at.elapsed() < Duration::from_secs(1),
            "cancellation did not finish within the bounded test window"
        );
        assert_eq!(start.code, PksSessionStatusCode::Cancelled as u32);
        assert_eq!(stop.code, PksSessionStatusCode::Ok as u32);
        // SAFETY: The output state is valid and aligned for the call.
        let stopped = unsafe { pks_session_get_state(engine, session, &mut lifecycle_state) };
        assert_eq!(stopped.code, PksSessionStatusCode::Ok as u32);
        assert_eq!(lifecycle_state, PksSessionLifecycleState::Stopped as u32);
        assert_eq!(
            pks_session_engine_destroy(engine).code,
            PksSessionStatusCode::Ok as u32
        );
    }

    #[test]
    fn given_blocking_start_when_started_twice_then_second_start_cannot_corrupt_state() {
        let blocking_state = Arc::new((Mutex::new(BlockingOpenState::default()), Condvar::new()));
        let engine = blocking_engine(Arc::clone(&blocking_state));
        let declaration = app_mic_declaration();
        let mut session = PksSessionHandle::invalid();
        // SAFETY: The declaration and output handle are valid for this call.
        let create = unsafe { pks_session_create_app_mic(engine, &declaration, &mut session) };
        let compile = pks_session_compile(engine, session);
        assert_eq!(create.code, PksSessionStatusCode::Ok as u32);
        assert_eq!(compile.code, PksSessionStatusCode::Ok as u32);

        let start_thread = std::thread::spawn(move || pks_session_start(engine, session));
        let (state, changed) = &*blocking_state;
        let mut open_state = state.lock().expect("blocking state lock");
        while !open_state.entered {
            open_state = changed.wait(open_state).expect("blocking state wait");
        }
        drop(open_state);

        let second_start = pks_session_start(engine, session);
        assert_eq!(
            second_start.code,
            PksSessionStatusCode::InvalidLifecycleState as u32
        );
        let mut lifecycle_state = u32::MAX;
        // SAFETY: The output state is valid and aligned for the call.
        let state_status = unsafe { pks_session_get_state(engine, session, &mut lifecycle_state) };
        assert_eq!(state_status.code, PksSessionStatusCode::Ok as u32);
        assert_eq!(lifecycle_state, PksSessionLifecycleState::Starting as u32);

        let stop_thread = std::thread::spawn(move || pks_session_stop(engine, session));
        let state_deadline = Instant::now() + Duration::from_secs(1);
        loop {
            // SAFETY: The output state is valid and aligned for the call.
            let status = unsafe { pks_session_get_state(engine, session, &mut lifecycle_state) };
            assert_eq!(status.code, PksSessionStatusCode::Ok as u32);
            if lifecycle_state == PksSessionLifecycleState::Stopping as u32 {
                break;
            }
            assert!(Instant::now() < state_deadline, "stop did not win its CAS");
            std::thread::yield_now();
        }
        let mut open_state = state.lock().expect("blocking state lock");
        open_state.released = true;
        changed.notify_all();
        drop(open_state);

        assert_eq!(
            start_thread.join().expect("start thread").code,
            PksSessionStatusCode::Cancelled as u32
        );
        assert_eq!(
            stop_thread.join().expect("stop thread").code,
            PksSessionStatusCode::Ok as u32
        );
        assert_eq!(
            pks_session_engine_destroy(engine).code,
            PksSessionStatusCode::Ok as u32
        );
    }

    #[test]
    fn given_panicking_abi_operation_when_contained_then_internal_panic_is_returned() {
        let status =
            abi_call(|| -> Result<_, super::AbiError> { panic!("contained ABI test panic") });

        assert_eq!(status.code, PksSessionStatusCode::InternalPanic as u32);
    }
}
