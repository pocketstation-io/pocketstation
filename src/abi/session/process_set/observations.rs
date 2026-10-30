use std::mem::size_of;

use super::records::{PksSessionExternalSourceMetrics, PksSessionStopOutcome};
use super::{validate_header, version_header};
use crate::abi::session::{
    abi_call, runtime_state, write_record, AbiError, PksSessionHandle, PksSessionStatus,
};

/// Returns the number of actual registered-source runtime observations.
///
/// # Safety
/// `output_count` must be writable and aligned for one u32 for this call.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn pks_session_external_source_metrics_count(
    engine: PksSessionHandle,
    session: PksSessionHandle,
    output_count: *mut u32,
) -> PksSessionStatus {
    abi_call(|| {
        super::super::guard_pointer_alignment(output_count.cast_const())?;
        let metrics = runtime_state().with_engine(engine, |runtime| runtime.metrics(session))?;
        let source_count =
            u32::try_from(metrics.external_source_count()).map_err(|_| AbiError::NoCapacity)?;
        // SAFETY: The documented scalar output was checked before state access.
        unsafe { write_record(output_count, source_count) }?;
        Ok(PksSessionStatus::ok())
    })
}

/// Copies one actual registered-source observation; no synthetic lineage fields.
///
/// # Safety
/// `output` must have an initialized ABI 1.2 header and the declared writable
/// extent. No pointer into the runtime is returned or retained.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn pks_session_external_source_metrics_at(
    engine: PksSessionHandle,
    session: PksSessionHandle,
    source_index: u32,
    output: *mut PksSessionExternalSourceMetrics,
) -> PksSessionStatus {
    abi_call(|| {
        // SAFETY: Only the initialized header is read before validating size.
        unsafe { validate_header(output.cast_const()) }?;
        let metrics = runtime_state().with_engine(engine, |runtime| runtime.metrics(session))?;
        let source = metrics
            .external_source(source_index as usize)
            .ok_or(AbiError::IndexOutOfRange)?;
        let (struct_size_bytes, abi_major, abi_minor) =
            version_header(size_of::<PksSessionExternalSourceMetrics>());
        let value = PksSessionExternalSourceMetrics {
            struct_size_bytes,
            abi_major,
            abi_minor,
            source_instance_id: source.source_instance_id.value(),
            source_id: source.source_id.get(),
            emitted_total: source.runtime.emitted_total,
            dropped_total: source.runtime.dropped_total,
            failure_total: source.runtime.failure_total,
            cancellation_total: source.runtime.cancellation_total,
            discontinuity_total: source.runtime.discontinuity_total,
            recovery_total: source.runtime.recovery_total,
            policy_change_total: source.runtime.policy_change_total,
            ready: u32::from(source.runtime.ready),
            joined: u32::from(source.runtime.joined),
        };
        // SAFETY: The caller's complete output extent was validated above.
        unsafe { write_record(output, value) }?;
        Ok(PksSessionStatus::ok())
    })
}

/// Copies an actual stop result. Before a stop result exists this fails with
/// InvalidLifecycleState; it never fabricates a zero-success result.
///
/// # Safety
/// `output` must have an initialized ABI 1.2 header and a writable record extent.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn pks_session_stop_outcome_poll(
    engine: PksSessionHandle,
    session: PksSessionHandle,
    output: *mut PksSessionStopOutcome,
) -> PksSessionStatus {
    abi_call(|| {
        // SAFETY: Forward the initialized versioned output-header contract.
        unsafe { validate_header(output.cast_const()) }?;
        let (outcome, cleanup_unconfirmed) =
            runtime_state().with_engine(engine, |runtime| runtime.stop_outcome(session))?;
        let (struct_size_bytes, abi_major, abi_minor) =
            version_header(size_of::<PksSessionStopOutcome>());
        let value = PksSessionStopOutcome {
            struct_size_bytes,
            abi_major,
            abi_minor,
            success: u32::from(outcome.is_success() && !cleanup_unconfirmed),
            runtime_worker_panicked: u32::from(outcome.runtime_worker_panicked()),
            capture_finalization_failures_total: outcome.capture_finalization_failures_total(),
            operator_finalization_failures_total: outcome.operator_finalization_failures_total(),
            endpoint_finalization_failures_total: outcome.endpoint_finalization_failures_total(),
            runtime_failures_total: outcome.runtime_failures_total(),
            lineage_failures_total: outcome.lineage_failures_total(),
            source_send_rejections_total: outcome.source_send_rejections_total(),
            runtime_events_total: outcome.runtime_events_total(),
            cleanup_unconfirmed: u32::from(cleanup_unconfirmed),
            reserved_zero: 0,
        };
        // SAFETY: Header validation established the writable known prefix.
        unsafe { write_record(output, value) }?;
        Ok(PksSessionStatus::ok())
    })
}
