//! Allocation-free capture callback observations.

use serde::Serialize;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize)]
pub struct CaptureObservations {
    pub callback_buffers_total: u64,
    pub frames_enqueued_total: u64,
    pub pool_exhausted_total: u64,
    pub dispatch_queue_full_total: u64,
    pub invalid_buffer_total: u64,
    pub oversized_buffer_total: u64,
    pub stream_errors_total: u64,
    pub timestamp_epoch_clamps_total: u64,
}

#[derive(Debug, Default)]
struct CaptureObservationValues {
    callback_buffers_total: AtomicU64,
    frames_enqueued_total: AtomicU64,
    pool_exhausted_total: AtomicU64,
    dispatch_queue_full_total: AtomicU64,
    invalid_buffer_total: AtomicU64,
    oversized_buffer_total: AtomicU64,
    stream_errors_total: AtomicU64,
    timestamp_epoch_clamps_total: AtomicU64,
}

#[derive(Clone, Debug, Default)]
pub struct CaptureObservationHandle {
    values: Arc<CaptureObservationValues>,
}

impl CaptureObservationHandle {
    pub fn observations(&self) -> CaptureObservations {
        snapshot_values(&self.values)
    }
}

/// Setup-time cloneable handle; every observation is one relaxed atomic
/// operation and remains allocation-free, lock-free, and log-free.
#[derive(Clone, Debug, Default)]
#[cfg(any(test, feature = "internal-testing", feature = "native-capture"))]
pub struct CaptureObservationCounters {
    values: Arc<CaptureObservationValues>,
}

#[cfg(any(test, feature = "internal-testing", feature = "native-capture"))]
impl CaptureObservationCounters {
    pub fn observe_callback_buffer(&self) {
        self.values
            .callback_buffers_total
            .fetch_add(1, Ordering::Relaxed);
    }

    pub fn observe_enqueued_frame(&self) {
        self.values
            .frames_enqueued_total
            .fetch_add(1, Ordering::Relaxed);
    }

    pub fn observe_pool_exhaustion(&self) {
        self.values
            .pool_exhausted_total
            .fetch_add(1, Ordering::Relaxed);
    }

    pub fn observe_dispatch_queue_full(&self) {
        self.observe_dispatch_queue_full_frames(1);
    }

    /// Records a known number of frames lost at a bounded native or Rust
    /// delivery edge.
    pub fn observe_dispatch_queue_full_frames(&self, lost_frames: u64) {
        self.values
            .dispatch_queue_full_total
            .fetch_add(lost_frames, Ordering::Relaxed);
    }

    pub fn observe_invalid_buffer(&self) {
        self.values
            .invalid_buffer_total
            .fetch_add(1, Ordering::Relaxed);
    }

    pub fn observe_oversized_buffer(&self) {
        self.values
            .oversized_buffer_total
            .fetch_add(1, Ordering::Relaxed);
    }

    pub fn observe_stream_error(&self) {
        self.values
            .stream_errors_total
            .fetch_add(1, Ordering::Relaxed);
    }

    pub fn observe_timestamp_epoch_clamp(&self) {
        self.values
            .timestamp_epoch_clamps_total
            .fetch_add(1, Ordering::Relaxed);
    }

    pub fn observation_handle(&self) -> CaptureObservationHandle {
        CaptureObservationHandle {
            values: Arc::clone(&self.values),
        }
    }

    pub fn snapshot(&self) -> CaptureObservations {
        snapshot_values(&self.values)
    }
}

fn snapshot_values(values: &CaptureObservationValues) -> CaptureObservations {
    CaptureObservations {
        callback_buffers_total: values.callback_buffers_total.load(Ordering::Relaxed),
        frames_enqueued_total: values.frames_enqueued_total.load(Ordering::Relaxed),
        pool_exhausted_total: values.pool_exhausted_total.load(Ordering::Relaxed),
        dispatch_queue_full_total: values.dispatch_queue_full_total.load(Ordering::Relaxed),
        invalid_buffer_total: values.invalid_buffer_total.load(Ordering::Relaxed),
        oversized_buffer_total: values.oversized_buffer_total.load(Ordering::Relaxed),
        stream_errors_total: values.stream_errors_total.load(Ordering::Relaxed),
        timestamp_epoch_clamps_total: values.timestamp_epoch_clamps_total.load(Ordering::Relaxed),
    }
}
