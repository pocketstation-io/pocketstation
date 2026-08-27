//! Executable bounded edge routing for a compiled `RuntimePlan`.

use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;

use crate::frame::{AudioBufferPool, FrameLineage, LineagedAudioFrame, SharedLineagedAudioFrame};
use crate::graph::ir::GraphIr;
use crate::graph::plan::RuntimePlan;
use crate::graph::spec::{EdgeId, InputPortRef, NodeId, OutputPortRef};
use crate::graph::{CopyPolicy, MediaCaps};
use rtrb::{Consumer, Producer, RingBuffer};

const LATENCY_HISTOGRAM_BUCKETS: usize = 64;
const TIMESTAMP_CONTINUITY_TOLERANCE_NS: u64 = 1_000;

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum PlanRouterError {
    #[error("edge {edge_id:?} has no memory plan")]
    MissingMemoryPlan { edge_id: EdgeId },
    #[error("edge {edge_id:?} has zero capacity")]
    ZeroCapacity { edge_id: EdgeId },
    #[error("edge {edge_id:?} has invalid bytes_per_frame {bytes_per_frame}")]
    InvalidFrameBytes {
        edge_id: EdgeId,
        bytes_per_frame: usize,
    },
}

pub enum PlanEdgeFrame {
    Exclusive(LineagedAudioFrame),
    Shared(SharedLineagedAudioFrame),
}

impl PlanEdgeFrame {
    pub fn stream_id(&self) -> crate::frame::StreamId {
        match self {
            Self::Exclusive(frame) => frame.frame().stream_id,
            Self::Shared(frame) => frame.frame().stream_id,
        }
    }

    pub fn source_id(&self) -> crate::frame::SourceId {
        match self {
            Self::Exclusive(frame) => frame.frame().source_id,
            Self::Shared(frame) => frame.frame().source_id,
        }
    }

    pub fn sequence_number(&self) -> u64 {
        match self {
            Self::Exclusive(frame) => frame.frame().sequence_number,
            Self::Shared(frame) => frame.frame().sequence_number,
        }
    }

    pub fn timestamp_ns(&self) -> u64 {
        match self {
            Self::Exclusive(frame) => frame.frame().timestamp_ns,
            Self::Shared(frame) => frame.frame().timestamp_ns,
        }
    }

    pub fn sample_rate_hz(&self) -> u32 {
        match self {
            Self::Exclusive(frame) => frame.frame().sample_rate_hz,
            Self::Shared(frame) => frame.frame().sample_rate_hz,
        }
    }

    pub fn channels(&self) -> u8 {
        match self {
            Self::Exclusive(frame) => frame.frame().channels,
            Self::Shared(frame) => frame.frame().channels,
        }
    }

    pub fn sample_format(&self) -> crate::frame::SampleFormat {
        match self {
            Self::Exclusive(frame) => frame.frame().format,
            Self::Shared(frame) => frame.frame().format,
        }
    }

    pub fn samples(&self) -> &[f32] {
        match self {
            Self::Exclusive(frame) => frame.frame().buffer.as_slice(),
            Self::Shared(frame) => frame.frame().buffer.as_slice(),
        }
    }

    pub fn lineage(&self) -> FrameLineage {
        match self {
            Self::Exclusive(frame) => frame.lineage(),
            Self::Shared(frame) => frame.lineage(),
        }
    }

    pub fn output_generation(&self) -> Option<&crate::frame::OutputGeneration> {
        match self {
            Self::Exclusive(frame) => frame.output_generation(),
            Self::Shared(frame) => frame.output_generation(),
        }
    }

    fn should_discard(&self) -> bool {
        self.output_generation()
            .is_some_and(crate::frame::OutputGeneration::should_discard)
    }
}

impl std::fmt::Debug for PlanEdgeFrame {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Exclusive(frame) => f
                .debug_tuple("Exclusive")
                .field(&frame.frame().sequence_number)
                .field(&frame.lineage())
                .finish(),
            Self::Shared(frame) => f
                .debug_tuple("Shared")
                .field(&frame.frame().sequence_number)
                .field(&frame.lineage())
                .finish(),
        }
    }
}

struct QueuedPlanEdgeFrame {
    frame: PlanEdgeFrame,
    enqueued_at_ns: u64,
}

pub(crate) struct PlanEdgeReceipt {
    frame: PlanEdgeFrame,
    enqueued_at_ns: u64,
    received_at_ns: u64,
}

impl PlanEdgeReceipt {
    pub(crate) fn into_frame(self) -> PlanEdgeFrame {
        self.frame
    }

    pub(crate) const fn enqueued_at_ns(&self) -> u64 {
        self.enqueued_at_ns
    }

    pub(crate) const fn received_at_ns(&self) -> u64 {
        self.received_at_ns
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct EdgeObservations {
    pub queue_capacity_frames: u64,
    pub queue_depth_frames: u64,
    pub queue_peak_frames: u64,
    pub frames_enqueued_total: u64,
    pub frames_delivered_total: u64,
    pub frames_dropped_total: u64,
    pub overruns_total: u64,
    pub receiver_unavailable_drops_total: u64,
    pub queue_full_drops_total: u64,
    pub shared_reference_exhausted_drops_total: u64,
    pub branch_pool_exhausted_drops_total: u64,
    pub invalid_copy_policy_drops_total: u64,
    pub freeze_failed_drops_total: u64,
    pub discontinuities_total: u64,
    pub source_identity_discontinuities_total: u64,
    pub sequence_discontinuities_total: u64,
    pub timestamp_discontinuities_total: u64,
    pub lineage_epoch_discontinuities_total: u64,
    pub manually_reported_discontinuities_total: u64,
    pub enqueue_to_receive_samples_total: u64,
    pub enqueue_to_receive_invalid_order_total: u64,
    pub enqueue_to_receive_p50_ns: u64,
    pub enqueue_to_receive_p95_ns: u64,
    pub enqueue_to_receive_p99_ns: u64,
    pub enqueue_to_receive_max_ns: u64,
    pub source_timestamp_to_receive_samples_total: u64,
    pub source_timestamp_to_receive_missing_total: u64,
    pub source_timestamp_to_receive_future_total: u64,
    pub source_timestamp_to_receive_p50_ns: u64,
    pub source_timestamp_to_receive_p95_ns: u64,
    pub source_timestamp_to_receive_p99_ns: u64,
    pub source_timestamp_to_receive_max_ns: u64,
    pub worker_failures_total: u64,
    pub shutdown_discarded_total: u64,
}

impl EdgeObservations {
    pub fn frames_attempted_total(self) -> u64 {
        self.frames_enqueued_total
            .saturating_add(self.frames_dropped_total)
    }

    pub fn drop_rate_pct(self) -> f64 {
        let frames_attempted_total = self.frames_attempted_total();
        if frames_attempted_total == 0 {
            0.0
        } else {
            self.frames_dropped_total as f64 / frames_attempted_total as f64 * 100.0
        }
    }
}

struct EdgeTelemetry {
    queue_capacity_frames: u64,
    enqueued_total: AtomicU64,
    delivered_total: AtomicU64,
    dropped_total: AtomicU64,
    overruns_total: AtomicU64,
    receiver_unavailable_drops_total: AtomicU64,
    queue_full_drops_total: AtomicU64,
    shared_reference_exhausted_drops_total: AtomicU64,
    branch_pool_exhausted_drops_total: AtomicU64,
    invalid_copy_policy_drops_total: AtomicU64,
    freeze_failed_drops_total: AtomicU64,
    discontinuities_total: AtomicU64,
    source_identity_discontinuities_total: AtomicU64,
    sequence_discontinuities_total: AtomicU64,
    timestamp_discontinuities_total: AtomicU64,
    lineage_epoch_discontinuities_total: AtomicU64,
    manually_reported_discontinuities_total: AtomicU64,
    shutdown_discarded_total: AtomicU64,
    discarded_output_frames_total: AtomicU64,
    queue_peak_frames: AtomicU64,
    worker_failures_total: AtomicU64,
    enqueue_to_receive_histogram: [AtomicU64; LATENCY_HISTOGRAM_BUCKETS],
    enqueue_to_receive_invalid_order_total: AtomicU64,
    enqueue_to_receive_max_ns: AtomicU64,
    source_timestamp_to_receive_histogram: [AtomicU64; LATENCY_HISTOGRAM_BUCKETS],
    source_timestamp_to_receive_max_ns: AtomicU64,
    source_timestamp_to_receive_missing_total: AtomicU64,
    source_timestamp_to_receive_future_total: AtomicU64,
}

/// Cloneable read-only access to one plan edge's authoritative live telemetry.
///
/// Endpoint workers can retain this handle after taking ownership of a
/// [`PlanEdgeReceiver`]. The handle shares the existing atomic telemetry and
/// cannot mutate counters or edge lifecycle state.
#[derive(Clone)]
pub struct PlanEdgeObservationHandle {
    telemetry: Arc<EdgeTelemetry>,
}

impl PlanEdgeObservationHandle {
    /// Returns a point-in-time snapshot of the edge's live observations.
    pub fn observations(&self) -> EdgeObservations {
        self.telemetry.snapshot()
    }

    /// Returns frames removed after their sender cancelled the owning output.
    pub fn discarded_output_frames_total(&self) -> u64 {
        self.telemetry
            .discarded_output_frames_total
            .load(Ordering::Relaxed)
    }
}

impl EdgeTelemetry {
    fn new(capacity_frames: usize) -> Self {
        Self {
            queue_capacity_frames: capacity_frames as u64,
            enqueued_total: AtomicU64::new(0),
            delivered_total: AtomicU64::new(0),
            dropped_total: AtomicU64::new(0),
            overruns_total: AtomicU64::new(0),
            receiver_unavailable_drops_total: AtomicU64::new(0),
            queue_full_drops_total: AtomicU64::new(0),
            shared_reference_exhausted_drops_total: AtomicU64::new(0),
            branch_pool_exhausted_drops_total: AtomicU64::new(0),
            invalid_copy_policy_drops_total: AtomicU64::new(0),
            freeze_failed_drops_total: AtomicU64::new(0),
            discontinuities_total: AtomicU64::new(0),
            source_identity_discontinuities_total: AtomicU64::new(0),
            sequence_discontinuities_total: AtomicU64::new(0),
            timestamp_discontinuities_total: AtomicU64::new(0),
            lineage_epoch_discontinuities_total: AtomicU64::new(0),
            manually_reported_discontinuities_total: AtomicU64::new(0),
            shutdown_discarded_total: AtomicU64::new(0),
            discarded_output_frames_total: AtomicU64::new(0),
            queue_peak_frames: AtomicU64::new(0),
            worker_failures_total: AtomicU64::new(0),
            enqueue_to_receive_histogram: std::array::from_fn(|_| AtomicU64::new(0)),
            enqueue_to_receive_invalid_order_total: AtomicU64::new(0),
            enqueue_to_receive_max_ns: AtomicU64::new(0),
            source_timestamp_to_receive_histogram: std::array::from_fn(|_| AtomicU64::new(0)),
            source_timestamp_to_receive_max_ns: AtomicU64::new(0),
            source_timestamp_to_receive_missing_total: AtomicU64::new(0),
            source_timestamp_to_receive_future_total: AtomicU64::new(0),
        }
    }

    fn queue_depth_frames(&self) -> u64 {
        self.enqueued_total.load(Ordering::Relaxed).saturating_sub(
            self.delivered_total
                .load(Ordering::Relaxed)
                .saturating_add(self.shutdown_discarded_total.load(Ordering::Relaxed))
                .saturating_add(self.discarded_output_frames_total.load(Ordering::Relaxed)),
        )
    }

    fn observe_output_discard(&self) {
        self.discarded_output_frames_total
            .fetch_add(1, Ordering::Relaxed);
    }

    fn observe_enqueue(&self) {
        self.enqueued_total.fetch_add(1, Ordering::Relaxed);
        self.queue_peak_frames
            .fetch_max(self.queue_depth_frames(), Ordering::Relaxed);
    }

    fn observe_delivery(&self, queued: &QueuedPlanEdgeFrame, delivered_at_ns: u64) {
        self.delivered_total.fetch_add(1, Ordering::Relaxed);
        if delivered_at_ns < queued.enqueued_at_ns {
            self.enqueue_to_receive_invalid_order_total
                .fetch_add(1, Ordering::Relaxed);
        } else {
            let latency_ns = delivered_at_ns - queued.enqueued_at_ns;
            let bucket = latency_bucket(latency_ns);
            self.enqueue_to_receive_histogram[bucket].fetch_add(1, Ordering::Relaxed);
            self.enqueue_to_receive_max_ns
                .fetch_max(latency_ns, Ordering::Relaxed);
        }
        let timestamp_ns = queued.frame.timestamp_ns();
        let shares_receive_clock =
            queued.frame.lineage().clock_id == crate::timing::PROCESS_MONOTONIC_CLOCK_DOMAIN_ID;
        if timestamp_ns == 0 || !shares_receive_clock {
            self.source_timestamp_to_receive_missing_total
                .fetch_add(1, Ordering::Relaxed);
        } else if timestamp_ns > delivered_at_ns {
            self.source_timestamp_to_receive_future_total
                .fetch_add(1, Ordering::Relaxed);
        } else {
            let frame_age_ns = delivered_at_ns - timestamp_ns;
            let frame_age_bucket = latency_bucket(frame_age_ns);
            self.source_timestamp_to_receive_histogram[frame_age_bucket]
                .fetch_add(1, Ordering::Relaxed);
            self.source_timestamp_to_receive_max_ns
                .fetch_max(frame_age_ns, Ordering::Relaxed);
        }
    }

    fn observe_drop(&self, reason: EdgeDropReason) {
        self.dropped_total.fetch_add(1, Ordering::Relaxed);
        match reason {
            EdgeDropReason::ReceiverUnavailable => {
                self.receiver_unavailable_drops_total
                    .fetch_add(1, Ordering::Relaxed);
            }
            EdgeDropReason::QueueFull => {
                self.queue_full_drops_total.fetch_add(1, Ordering::Relaxed);
                self.overruns_total.fetch_add(1, Ordering::Relaxed);
            }
            EdgeDropReason::SharedReferenceExhausted => {
                self.shared_reference_exhausted_drops_total
                    .fetch_add(1, Ordering::Relaxed);
            }
            EdgeDropReason::BranchPoolExhausted => {
                self.branch_pool_exhausted_drops_total
                    .fetch_add(1, Ordering::Relaxed);
                self.overruns_total.fetch_add(1, Ordering::Relaxed);
            }
            EdgeDropReason::InvalidCopyPolicy => {
                self.invalid_copy_policy_drops_total
                    .fetch_add(1, Ordering::Relaxed);
            }
            EdgeDropReason::FreezeFailed => {
                self.freeze_failed_drops_total
                    .fetch_add(1, Ordering::Relaxed);
            }
        }
    }

    fn snapshot(&self) -> EdgeObservations {
        let histogram: [u64; LATENCY_HISTOGRAM_BUCKETS] = std::array::from_fn(|index| {
            self.enqueue_to_receive_histogram[index].load(Ordering::Relaxed)
        });
        let source_timestamp_to_receive_histogram: [u64; LATENCY_HISTOGRAM_BUCKETS] =
            std::array::from_fn(|index| {
                self.source_timestamp_to_receive_histogram[index].load(Ordering::Relaxed)
            });
        EdgeObservations {
            queue_capacity_frames: self.queue_capacity_frames,
            queue_depth_frames: self.queue_depth_frames(),
            queue_peak_frames: self.queue_peak_frames.load(Ordering::Relaxed),
            frames_enqueued_total: self.enqueued_total.load(Ordering::Relaxed),
            frames_delivered_total: self.delivered_total.load(Ordering::Relaxed),
            frames_dropped_total: self.dropped_total.load(Ordering::Relaxed),
            overruns_total: self.overruns_total.load(Ordering::Relaxed),
            receiver_unavailable_drops_total: self
                .receiver_unavailable_drops_total
                .load(Ordering::Relaxed),
            queue_full_drops_total: self.queue_full_drops_total.load(Ordering::Relaxed),
            shared_reference_exhausted_drops_total: self
                .shared_reference_exhausted_drops_total
                .load(Ordering::Relaxed),
            branch_pool_exhausted_drops_total: self
                .branch_pool_exhausted_drops_total
                .load(Ordering::Relaxed),
            invalid_copy_policy_drops_total: self
                .invalid_copy_policy_drops_total
                .load(Ordering::Relaxed),
            freeze_failed_drops_total: self.freeze_failed_drops_total.load(Ordering::Relaxed),
            discontinuities_total: self.discontinuities_total.load(Ordering::Relaxed),
            source_identity_discontinuities_total: self
                .source_identity_discontinuities_total
                .load(Ordering::Relaxed),
            sequence_discontinuities_total: self
                .sequence_discontinuities_total
                .load(Ordering::Relaxed),
            timestamp_discontinuities_total: self
                .timestamp_discontinuities_total
                .load(Ordering::Relaxed),
            lineage_epoch_discontinuities_total: self
                .lineage_epoch_discontinuities_total
                .load(Ordering::Relaxed),
            manually_reported_discontinuities_total: self
                .manually_reported_discontinuities_total
                .load(Ordering::Relaxed),
            enqueue_to_receive_samples_total: histogram.iter().sum(),
            enqueue_to_receive_invalid_order_total: self
                .enqueue_to_receive_invalid_order_total
                .load(Ordering::Relaxed),
            enqueue_to_receive_p50_ns: histogram_percentile_ns(&histogram, 50),
            enqueue_to_receive_p95_ns: histogram_percentile_ns(&histogram, 95),
            enqueue_to_receive_p99_ns: histogram_percentile_ns(&histogram, 99),
            enqueue_to_receive_max_ns: self.enqueue_to_receive_max_ns.load(Ordering::Relaxed),
            source_timestamp_to_receive_samples_total: source_timestamp_to_receive_histogram
                .iter()
                .sum(),
            source_timestamp_to_receive_missing_total: self
                .source_timestamp_to_receive_missing_total
                .load(Ordering::Relaxed),
            source_timestamp_to_receive_future_total: self
                .source_timestamp_to_receive_future_total
                .load(Ordering::Relaxed),
            source_timestamp_to_receive_p50_ns: histogram_percentile_ns(
                &source_timestamp_to_receive_histogram,
                50,
            ),
            source_timestamp_to_receive_p95_ns: histogram_percentile_ns(
                &source_timestamp_to_receive_histogram,
                95,
            ),
            source_timestamp_to_receive_p99_ns: histogram_percentile_ns(
                &source_timestamp_to_receive_histogram,
                99,
            ),
            source_timestamp_to_receive_max_ns: self
                .source_timestamp_to_receive_max_ns
                .load(Ordering::Relaxed),
            worker_failures_total: self.worker_failures_total.load(Ordering::Relaxed),
            shutdown_discarded_total: self.shutdown_discarded_total.load(Ordering::Relaxed),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum EdgeDropReason {
    ReceiverUnavailable,
    QueueFull,
    SharedReferenceExhausted,
    BranchPoolExhausted,
    InvalidCopyPolicy,
    FreezeFailed,
}

fn latency_bucket(latency_ns: u64) -> usize {
    if latency_ns == 0 {
        0
    } else {
        (u64::BITS - latency_ns.leading_zeros()) as usize
    }
    .min(LATENCY_HISTOGRAM_BUCKETS - 1)
}

fn histogram_percentile_ns(histogram: &[u64; LATENCY_HISTOGRAM_BUCKETS], percentile: u64) -> u64 {
    let total: u64 = histogram.iter().sum();
    if total == 0 {
        return 0;
    }
    let target = total.saturating_mul(percentile).div_ceil(100);
    let mut cumulative = 0u64;
    for (index, count) in histogram.iter().enumerate() {
        cumulative = cumulative.saturating_add(*count);
        if cumulative >= target {
            return if index == 0 {
                0
            } else {
                1u64.checked_shl(index as u32).unwrap_or(u64::MAX)
            };
        }
    }
    u64::MAX
}

struct EdgeSender {
    producer: Producer<QueuedPlanEdgeFrame>,
    alive: Arc<AtomicBool>,
    telemetry: Arc<EdgeTelemetry>,
}

impl EdgeSender {
    fn is_full(&self) -> bool {
        self.producer.is_full()
    }

    fn send(&mut self, frame: PlanEdgeFrame, enqueued_at_ns: u64) -> bool {
        if !self.alive.load(Ordering::Acquire) {
            self.telemetry
                .observe_drop(EdgeDropReason::ReceiverUnavailable);
            return false;
        }
        let queued = QueuedPlanEdgeFrame {
            frame,
            enqueued_at_ns,
        };
        match self.producer.push(queued) {
            Ok(()) => {
                self.telemetry.observe_enqueue();
                true
            }
            Err(rtrb::PushError::Full(_queued)) => {
                self.telemetry.observe_drop(EdgeDropReason::QueueFull);
                false
            }
        }
    }
}

pub struct PlanEdgeReceiver {
    edge_id: EdgeId,
    #[cfg(any(test, feature = "internal-testing"))]
    from: OutputPortRef,
    to: InputPortRef,
    consumer: Consumer<QueuedPlanEdgeFrame>,
    alive: Arc<AtomicBool>,
    telemetry: Arc<EdgeTelemetry>,
    continuity: Option<FrameContinuity>,
}

#[derive(Debug, Clone, Copy)]
struct FrameContinuity {
    source_id: crate::frame::SourceId,
    next_sequence_number: u64,
    next_timestamp_ns: u64,
    discontinuity_epoch: Option<u64>,
}

impl PlanEdgeReceiver {
    pub fn edge_id(&self) -> EdgeId {
        self.edge_id
    }

    #[cfg(any(test, feature = "internal-testing"))]
    pub fn from(&self) -> &OutputPortRef {
        &self.from
    }

    pub fn to(&self) -> &InputPortRef {
        &self.to
    }

    /// Returns a read-only handle to this edge's authoritative live telemetry.
    ///
    /// Call this before moving the receiver into its destination worker when
    /// another owner must observe queue, drop, discontinuity, or shutdown
    /// state.
    pub fn observation_handle(&self) -> PlanEdgeObservationHandle {
        PlanEdgeObservationHandle {
            telemetry: Arc::clone(&self.telemetry),
        }
    }

    pub(crate) fn recv_at(&mut self, delivered_at_ns: u64) -> Option<PlanEdgeFrame> {
        loop {
            let queued = self.consumer.pop().ok()?;
            if queued.frame.should_discard() {
                self.observe_continuity(&queued.frame);
                self.telemetry.observe_output_discard();
                continue;
            }
            return self
                .observe_received(queued, delivered_at_ns)
                .map(PlanEdgeReceipt::into_frame);
        }
    }

    /// Pops one queued frame before sampling the monotonic process clock.
    ///
    /// Callers that sample time before attempting the pop can race a producer:
    /// an empty queue may receive a new frame between the timestamp read and the
    /// pop, manufacturing a receive-before-enqueue observation. Runtime
    /// destination workers must use this method. The internal `recv_at` method
    /// remains only for deterministic schedulers and runtime tests that already
    /// own a valid timestamp.
    pub fn try_recv(&mut self) -> Option<PlanEdgeFrame> {
        self.try_recv_receipt().map(PlanEdgeReceipt::into_frame)
    }

    pub(crate) fn try_recv_receipt(&mut self) -> Option<PlanEdgeReceipt> {
        self.recv_receipt_with_clock(crate::timing::monotonic_timestamp_ns)
    }

    pub fn is_abandoned(&self) -> bool {
        self.consumer.is_abandoned()
    }

    fn recv_receipt_with_clock(
        &mut self,
        mut clock: impl FnMut() -> u64,
    ) -> Option<PlanEdgeReceipt> {
        loop {
            let queued = self.consumer.pop().ok()?;
            if queued.frame.should_discard() {
                self.observe_continuity(&queued.frame);
                self.telemetry.observe_output_discard();
                continue;
            }
            return self.observe_received(queued, clock());
        }
    }

    fn observe_received(
        &mut self,
        queued: QueuedPlanEdgeFrame,
        delivered_at_ns: u64,
    ) -> Option<PlanEdgeReceipt> {
        self.observe_continuity(&queued.frame);
        self.telemetry.observe_delivery(&queued, delivered_at_ns);
        Some(PlanEdgeReceipt {
            frame: queued.frame,
            enqueued_at_ns: queued.enqueued_at_ns,
            received_at_ns: delivered_at_ns,
        })
    }

    fn observe_continuity(&mut self, frame: &PlanEdgeFrame) {
        let source_id = frame.source_id();
        if let Some(previous) = self.continuity {
            if source_id != previous.source_id {
                self.telemetry
                    .source_identity_discontinuities_total
                    .fetch_add(1, Ordering::Relaxed);
                self.telemetry
                    .discontinuities_total
                    .fetch_add(1, Ordering::Relaxed);
            } else {
                if frame.sequence_number() != previous.next_sequence_number {
                    self.telemetry
                        .sequence_discontinuities_total
                        .fetch_add(1, Ordering::Relaxed);
                    self.telemetry
                        .discontinuities_total
                        .fetch_add(1, Ordering::Relaxed);
                }
                if frame.timestamp_ns().abs_diff(previous.next_timestamp_ns)
                    > TIMESTAMP_CONTINUITY_TOLERANCE_NS
                {
                    self.telemetry
                        .timestamp_discontinuities_total
                        .fetch_add(1, Ordering::Relaxed);
                    self.telemetry
                        .discontinuities_total
                        .fetch_add(1, Ordering::Relaxed);
                }
                if let Some(previous_epoch) = previous.discontinuity_epoch {
                    let lineage = frame.lineage();
                    if lineage.discontinuity_epoch != previous_epoch {
                        self.telemetry
                            .lineage_epoch_discontinuities_total
                            .fetch_add(1, Ordering::Relaxed);
                        self.telemetry
                            .discontinuities_total
                            .fetch_add(1, Ordering::Relaxed);
                    }
                }
            }
        }
        let channels = usize::from(frame.channels()).max(1);
        let samples_per_channel = frame.samples().len() / channels;
        let duration_ns = u64::try_from(samples_per_channel)
            .unwrap_or(u64::MAX)
            .saturating_mul(1_000_000_000)
            .checked_div(u64::from(frame.sample_rate_hz()))
            .unwrap_or(0);
        self.continuity = Some(FrameContinuity {
            source_id,
            next_sequence_number: frame.sequence_number().saturating_add(1),
            next_timestamp_ns: frame.timestamp_ns().saturating_add(duration_ns),
            discontinuity_epoch: Some(frame.lineage().discontinuity_epoch),
        });
    }

    pub fn observations(&self) -> EdgeObservations {
        self.telemetry.snapshot()
    }

    pub fn mark_discontinuity(&self) {
        self.telemetry
            .manually_reported_discontinuities_total
            .fetch_add(1, Ordering::Relaxed);
        self.telemetry
            .discontinuities_total
            .fetch_add(1, Ordering::Relaxed);
    }

    pub fn mark_worker_failure(&self) {
        self.telemetry
            .worker_failures_total
            .fetch_add(1, Ordering::Relaxed);
    }
}

impl Drop for PlanEdgeReceiver {
    fn drop(&mut self) {
        self.alive.store(false, Ordering::Release);
        while self.consumer.pop().is_ok() {
            self.telemetry
                .shutdown_discarded_total
                .fetch_add(1, Ordering::Relaxed);
        }
    }
}

struct RoutedEdge {
    #[cfg(any(test, feature = "internal-testing"))]
    edge_id: EdgeId,
    from: OutputPortRef,
    copy_policy: CopyPolicy,
    branch_pool: Option<Arc<AudioBufferPool>>,
    sender: EdgeSender,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct DispatchSummary {
    pub attempted_edges: u64,
    pub enqueued_edges: u64,
    pub dropped_edges: u64,
    pub copy_pool_exhausted_edges: u64,
    pub freeze_failed_edges: u64,
}

pub struct PlanEdgeRouter {
    edges: Vec<RoutedEdge>,
}

impl PlanEdgeRouter {
    pub fn new(
        plan: &RuntimePlan,
        ir: &GraphIr,
    ) -> Result<(Self, Vec<PlanEdgeReceiver>), PlanRouterError> {
        let mut edges = Vec::with_capacity(ir.edges.len());
        let mut receivers = Vec::with_capacity(ir.edges.len());
        for resolved in &ir.edges {
            let source_is_audio = ir
                .node(resolved.spec.from.node)
                .and_then(|node| {
                    node.descriptor
                        .outputs
                        .iter()
                        .find(|port| port.name == resolved.spec.from.port)
                })
                .is_some_and(|port| port.signal.class.is_audio());
            if !matches!(resolved.media, MediaCaps::Audio(_) | MediaCaps::Any) && !source_is_audio {
                continue;
            }
            let edge_id = resolved.spec.id;
            let buffer = plan
                .memory_plan
                .edge_buffer(edge_id)
                .ok_or(PlanRouterError::MissingMemoryPlan { edge_id })?;
            if buffer.capacity_frames == 0 {
                return Err(PlanRouterError::ZeroCapacity { edge_id });
            }
            if buffer.bytes_per_frame == 0 || buffer.bytes_per_frame % size_of::<f32>() != 0 {
                return Err(PlanRouterError::InvalidFrameBytes {
                    edge_id,
                    bytes_per_frame: buffer.bytes_per_frame,
                });
            }
            let (producer, consumer) = RingBuffer::new(buffer.capacity_frames);
            let alive = Arc::new(AtomicBool::new(true));
            let telemetry = Arc::new(EdgeTelemetry::new(buffer.capacity_frames));
            let branch_pool = (buffer.copy_policy == CopyPolicy::CopyToBranchPool).then(|| {
                AudioBufferPool::new(
                    buffer.branch_copy_pool_capacity_frames(),
                    buffer.bytes_per_frame / size_of::<f32>(),
                )
            });
            edges.push(RoutedEdge {
                #[cfg(any(test, feature = "internal-testing"))]
                edge_id,
                from: resolved.spec.from.clone(),
                copy_policy: buffer.copy_policy,
                branch_pool,
                sender: EdgeSender {
                    producer,
                    alive: Arc::clone(&alive),
                    telemetry: Arc::clone(&telemetry),
                },
            });
            receivers.push(PlanEdgeReceiver {
                edge_id,
                #[cfg(any(test, feature = "internal-testing"))]
                from: resolved.spec.from.clone(),
                to: resolved.spec.to.clone(),
                consumer,
                alive,
                telemetry,
                continuity: None,
            });
        }
        Ok((Self { edges }, receivers))
    }

    pub fn dispatch_from(
        &mut self,
        node_id: NodeId,
        output_port: &str,
        frame: LineagedAudioFrame,
        enqueued_at_ns: u64,
    ) -> DispatchSummary {
        let matching_count = self
            .edges
            .iter()
            .filter(|edge| edge.from.node == node_id && edge.from.port == output_port)
            .count();
        let mut summary = DispatchSummary {
            attempted_edges: matching_count as u64,
            ..DispatchSummary::default()
        };
        if matching_count == 0 {
            return summary;
        }
        let exclusive_index = (matching_count == 1).then(|| {
            self.edges.iter().position(|edge| {
                edge.from.node == node_id
                    && edge.from.port == output_port
                    && edge.copy_policy == CopyPolicy::MoveExclusive
            })
        });
        if let Some(Some(index)) = exclusive_index {
            if self.edges[index]
                .sender
                .send(PlanEdgeFrame::Exclusive(frame), enqueued_at_ns)
            {
                summary.enqueued_edges = 1;
            } else {
                summary.dropped_edges = 1;
            }
            return summary;
        }

        let Some(shared) = frame.freeze() else {
            summary.dropped_edges = summary.attempted_edges;
            summary.freeze_failed_edges = summary.attempted_edges;
            for edge in &self.edges {
                if edge.from.node == node_id && edge.from.port == output_port {
                    edge.sender
                        .telemetry
                        .observe_drop(EdgeDropReason::FreezeFailed);
                }
            }
            return summary;
        };
        for edge in &mut self.edges {
            if edge.from.node != node_id || edge.from.port != output_port {
                continue;
            }
            let branch_frame = match edge.copy_policy {
                CopyPolicy::ShareReadOnly => {
                    let Some(branch_frame) = shared.try_clone() else {
                        summary.dropped_edges = summary.dropped_edges.saturating_add(1);
                        summary.freeze_failed_edges = summary.freeze_failed_edges.saturating_add(1);
                        edge.sender
                            .telemetry
                            .observe_drop(EdgeDropReason::SharedReferenceExhausted);
                        continue;
                    };
                    PlanEdgeFrame::Shared(branch_frame)
                }
                CopyPolicy::CopyToBranchPool => {
                    let Some(branch_frame) = edge
                        .branch_pool
                        .as_ref()
                        .and_then(|pool| shared.copy_to_pool(pool))
                    else {
                        summary.dropped_edges = summary.dropped_edges.saturating_add(1);
                        let reason = if edge.sender.is_full() {
                            EdgeDropReason::QueueFull
                        } else {
                            summary.copy_pool_exhausted_edges =
                                summary.copy_pool_exhausted_edges.saturating_add(1);
                            EdgeDropReason::BranchPoolExhausted
                        };
                        edge.sender.telemetry.observe_drop(reason);
                        continue;
                    };
                    PlanEdgeFrame::Exclusive(branch_frame)
                }
                CopyPolicy::MoveExclusive => {
                    summary.dropped_edges = summary.dropped_edges.saturating_add(1);
                    summary.freeze_failed_edges = summary.freeze_failed_edges.saturating_add(1);
                    edge.sender
                        .telemetry
                        .observe_drop(EdgeDropReason::InvalidCopyPolicy);
                    continue;
                }
            };
            if edge.sender.send(branch_frame, enqueued_at_ns) {
                summary.enqueued_edges = summary.enqueued_edges.saturating_add(1);
            } else {
                summary.dropped_edges = summary.dropped_edges.saturating_add(1);
            }
        }
        summary
    }

    #[cfg(any(test, feature = "internal-testing"))]
    pub fn observations(&self, edge_id: EdgeId) -> Option<EdgeObservations> {
        self.edges
            .iter()
            .find(|edge| edge.edge_id == edge_id)
            .map(|edge| edge.sender.telemetry.snapshot())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frame::{AudioFrame, ClockDomainId, SessionId, SourceId, StemId, StreamId};
    use crate::graph::compile::Compiler;
    use crate::graph::compile::RuntimePlanner;
    use crate::graph::dsl::Pipeline;
    use crate::graph::node::NodeConfig;
    use crate::graph::register_builtins;
    use crate::graph::registry::{NodeDefinition, NodeRegistry};
    use crate::graph::{
        AudioCaps, ChannelLayout, ConfigError, EdgeContract, ExecutionPartition, MediaCaps,
        Multiplicity, NodeDescriptor, NodeTypeId, PortDirection, PortSpec, SafetyContract,
        SignalSpec, TextFormat,
    };

    struct Definition(NodeDescriptor);

    impl NodeDefinition for Definition {
        fn descriptor(&self) -> NodeDescriptor {
            self.0.clone()
        }

        fn validate_config(&self, _config: &NodeConfig) -> Result<(), ConfigError> {
            Ok(())
        }
    }

    fn typed_port(name: &str, direction: PortDirection, signal: SignalSpec) -> PortSpec {
        let media = if signal.class.is_audio() {
            MediaCaps::Audio(AudioCaps {
                sample_rate_hz: None,
                frame_samples: None,
                channel_layout: ChannelLayout::Any,
                format: crate::frame::SampleFormat::F32Interleaved,
            })
        } else {
            MediaCaps::Text
        };
        PortSpec {
            name: name.to_owned(),
            direction,
            signal,
            media,
            multiplicity: Multiplicity::One,
            required: true,
        }
    }

    fn registry() -> NodeRegistry {
        let mut registry = NodeRegistry::new();
        register_builtins(&mut registry).unwrap();
        registry
    }

    fn frame(
        pool: &Arc<AudioBufferPool>,
        source_id: u64,
        sequence_number: u64,
    ) -> LineagedAudioFrame {
        lineaged_frame(
            pool,
            source_id,
            sequence_number,
            sequence_number.saturating_mul(20_000_000),
            0,
        )
    }

    fn lineaged_frame(
        pool: &Arc<AudioBufferPool>,
        source_id: u64,
        sequence_number: u64,
        timestamp_ns: u64,
        discontinuity_epoch: u64,
    ) -> LineagedAudioFrame {
        lineaged_frame_with_clock(
            pool,
            source_id,
            sequence_number,
            timestamp_ns,
            discontinuity_epoch,
            crate::timing::PROCESS_MONOTONIC_CLOCK_DOMAIN_ID,
        )
    }

    fn lineaged_frame_with_clock(
        pool: &Arc<AudioBufferPool>,
        source_id: u64,
        sequence_number: u64,
        timestamp_ns: u64,
        discontinuity_epoch: u64,
        clock_id: ClockDomainId,
    ) -> LineagedAudioFrame {
        LineagedAudioFrame::new(
            {
                let mut buffer = pool.acquire().unwrap();
                buffer
                    .try_copy_from_slice(&[source_id as f32, sequence_number as f32])
                    .unwrap();
                AudioFrame::new(
                    StreamId(source_id),
                    SourceId(source_id),
                    sequence_number,
                    timestamp_ns,
                    1,
                    buffer,
                )
            },
            FrameLineage {
                session_id: SessionId(42),
                source_id: SourceId(source_id),
                stem_id: StemId(8),
                clock_id,
                sequence_num: sequence_number,
                timestamp_start_ns: timestamp_ns,
                duration_ns: 41_666,
                source_generation: 10,
                discontinuity_epoch,
                permission_epoch: 11,
            },
        )
        .unwrap()
    }

    #[test]
    fn given_compiled_text_edge_when_router_builds_then_only_audio_edge_gets_audio_receiver() {
        let mut registry = registry();
        registry
            .register_definition(Arc::new(Definition(NodeDescriptor {
                type_id: NodeTypeId::from("test.stt"),
                display_name: "test stt",
                inputs: vec![typed_port(
                    "audio",
                    PortDirection::Input,
                    SignalSpec::audio(),
                )],
                outputs: vec![typed_port(
                    "transcript",
                    PortDirection::Output,
                    SignalSpec::text(TextFormat::Utf8).with_role("transcript"),
                )],
                execution: ExecutionPartition::AsyncWorker,
                safety: SafetyContract::AllocationAllowed,
                stateful: true,
            })))
            .unwrap();
        registry
            .register_definition(Arc::new(Definition(NodeDescriptor {
                type_id: NodeTypeId::from("test.text-sink"),
                display_name: "test text sink",
                inputs: vec![typed_port(
                    "in",
                    PortDirection::Input,
                    SignalSpec::text(TextFormat::Utf8),
                )],
                outputs: Vec::new(),
                execution: ExecutionPartition::External,
                safety: SafetyContract::ExternalService,
                stateful: true,
            })))
            .unwrap();
        let mut graph = Pipeline::new();
        let source = graph.add_node("passthrough", NodeConfig::new());
        let operator = graph.add_node("test.stt", NodeConfig::new());
        let sink = graph.add_node("test.text-sink", NodeConfig::new());
        let mut input_contract = EdgeContract::realtime_audio();
        input_contract.copy_policy = CopyPolicy::CopyToBranchPool;
        let mut output_contract = EdgeContract::bounded_async();
        output_contract.media = MediaCaps::Text;
        graph.connect_with(source.out("out"), operator.in_("audio"), input_contract);
        graph.connect_with(operator.out("transcript"), sink.in_("in"), output_contract);
        let ir = Compiler::new()
            .compile(graph.into_spec(), &registry)
            .unwrap();
        let plan = RuntimePlanner::new().plan(&ir).unwrap();

        let (_router, receivers) = PlanEdgeRouter::new(&plan, &ir).unwrap();

        assert_eq!(receivers.len(), 1);
        assert_eq!(plan.memory_plan.edge_buffers.len(), 1);
        assert_eq!(plan.typed_edges.len(), 1);
        assert_eq!(plan.typed_edges[0].media, MediaCaps::Text);
        assert_eq!(plan.typed_edges[0].contract, output_contract);
    }

    #[test]
    fn given_one_source_with_three_edges_when_dispatched_then_every_edge_receives_identified_frame()
    {
        // Given
        let registry = registry();
        let mut graph = Pipeline::new();
        let source = graph.add_node("passthrough", NodeConfig::new());
        let first = graph.add_node("passthrough", NodeConfig::new());
        let second = graph.add_node("passthrough", NodeConfig::new());
        let third = graph.add_node("passthrough", NodeConfig::new());
        graph.connect(source.out("out"), first.in_("in"));
        graph.connect(source.out("out"), second.in_("in"));
        graph.connect(source.out("out"), third.in_("in"));
        let ir = Compiler::new()
            .compile(graph.into_spec(), &registry)
            .unwrap();
        let plan = RuntimePlanner::new().plan(&ir).unwrap();
        let (mut router, mut receivers) = PlanEdgeRouter::new(&plan, &ir).unwrap();
        let pool = AudioBufferPool::new(1, 2);

        // When
        let summary = router.dispatch_from(source.id(), "out", frame(&pool, 7, 11), 100);

        // Then
        assert_eq!(summary.enqueued_edges, 3);
        for receiver in &mut receivers {
            let received = receiver.recv_at(150).unwrap();
            assert_eq!(received.source_id(), SourceId(7));
            assert_eq!(received.sequence_number(), 11);
        }
    }

    #[test]
    fn given_replaced_output_in_edge_when_received_then_inactive_frame_is_discarded() {
        let registry = registry();
        let mut graph = Pipeline::new();
        let source = graph.add_node("passthrough", NodeConfig::new());
        let sink = graph.add_node("passthrough", NodeConfig::new());
        graph.connect(source.out("out"), sink.in_("in"));
        let ir = Compiler::new()
            .compile(graph.into_spec(), &registry)
            .unwrap();
        let plan = RuntimePlanner::new().plan(&ir).unwrap();
        let (mut router, mut receivers) = PlanEdgeRouter::new(&plan, &ir).unwrap();
        let pool = AudioBufferPool::new(2, 2);
        let output_state = crate::frame::OutputGenerationState::new();
        let first = output_state.begin().unwrap();

        router.dispatch_from(
            source.id(),
            "out",
            frame(&pool, 7, 1).with_output_generation(Some(first)),
            100,
        );
        let replacement = output_state.begin().unwrap();
        router.dispatch_from(
            source.id(),
            "out",
            frame(&pool, 7, 2).with_output_generation(Some(replacement.clone())),
            101,
        );

        let output_observations = receivers[0].observation_handle();
        let received = receivers[0].recv_at(102).unwrap();
        let observations = receivers[0].observations();
        assert_eq!(received.sequence_number(), 2);
        assert_eq!(
            received
                .output_generation()
                .map(crate::frame::OutputGeneration::id),
            Some(replacement.id())
        );
        assert_eq!(observations.frames_delivered_total, 1);
        assert_eq!(output_observations.discarded_output_frames_total(), 1);
        assert_eq!(observations.queue_depth_frames, 0);
    }

    #[test]
    fn given_lineaged_source_fan_out_when_branch_frames_are_copied_then_exact_lineage_is_preserved()
    {
        let registry = registry();
        let mut graph = Pipeline::new();
        let source = graph.add_node("passthrough", NodeConfig::new());
        for _ in 0..3 {
            let sink = graph.add_node("passthrough", NodeConfig::new());
            graph.connect(source.out("out"), sink.in_("in"));
        }
        let ir = Compiler::new()
            .compile(graph.into_spec(), &registry)
            .unwrap();
        let plan = RuntimePlanner::new().plan(&ir).unwrap();
        let (mut router, mut receivers) = PlanEdgeRouter::new(&plan, &ir).unwrap();
        let pool = AudioBufferPool::new(1, 2);
        let frame = lineaged_frame(&pool, 7, 11, 100, 12);
        let expected_lineage = frame.lineage();

        let summary = router.dispatch_from(source.id(), "out", frame, 101);
        let mut branch_buffer_addresses = Vec::new();
        for receiver in &mut receivers {
            let received = receiver.recv_at(102).unwrap();
            assert_eq!(received.lineage(), expected_lineage);
            match received {
                PlanEdgeFrame::Exclusive(frame) => {
                    branch_buffer_addresses.push(frame.frame().buffer.as_slice().as_ptr());
                }
                PlanEdgeFrame::Shared(frame) => {
                    branch_buffer_addresses.push(frame.frame().buffer.as_slice().as_ptr());
                }
            }
        }

        assert_eq!(summary.enqueued_edges, 3);
        assert_eq!(summary.dropped_edges, 0);
        branch_buffer_addresses.sort_unstable();
        branch_buffer_addresses.dedup();
        assert_eq!(branch_buffer_addresses.len(), 3);
    }

    #[test]
    fn given_lineage_discontinuity_epoch_change_when_received_then_declared_discontinuity_is_counted(
    ) {
        let registry = registry();
        let mut graph = Pipeline::new();
        let source = graph.add_node("passthrough", NodeConfig::new());
        let sink = graph.add_node("passthrough", NodeConfig::new());
        graph.connect(source.out("out"), sink.in_("in"));
        let ir = Compiler::new()
            .compile(graph.into_spec(), &registry)
            .unwrap();
        let plan = RuntimePlanner::new().plan(&ir).unwrap();
        let (mut router, mut receivers) = PlanEdgeRouter::new(&plan, &ir).unwrap();
        let pool = AudioBufferPool::new(1, 2);

        router.dispatch_from(source.id(), "out", lineaged_frame(&pool, 7, 1, 100, 3), 101);
        receivers[0].recv_at(102).unwrap();
        router.dispatch_from(
            source.id(),
            "out",
            lineaged_frame(&pool, 7, 2, 41_766, 4),
            41_767,
        );
        receivers[0].recv_at(41_768).unwrap();
        let observations = receivers[0].observations();

        assert_eq!(observations.sequence_discontinuities_total, 0);
        assert_eq!(observations.timestamp_discontinuities_total, 0);
        assert_eq!(observations.lineage_epoch_discontinuities_total, 1);
        assert_eq!(observations.discontinuities_total, 1);
    }

    #[test]
    fn given_timestamped_frame_when_delivered_then_both_latency_boundaries_have_sample_coverage() {
        let registry = registry();
        let mut graph = Pipeline::new();
        let source = graph.add_node("passthrough", NodeConfig::new());
        let sink = graph.add_node("passthrough", NodeConfig::new());
        graph.connect(source.out("out"), sink.in_("in"));
        let ir = Compiler::new()
            .compile(graph.into_spec(), &registry)
            .unwrap();
        let plan = RuntimePlanner::new().plan(&ir).unwrap();
        let (mut router, mut receivers) = PlanEdgeRouter::new(&plan, &ir).unwrap();
        let pool = AudioBufferPool::new(1, 2);

        router.dispatch_from(
            source.id(),
            "out",
            lineaged_frame(&pool, 7, 1, 20_000_000, 0),
            20_000_010,
        );
        receivers[0].recv_at(20_000_100).unwrap();
        let observations = receivers[0].observations();

        assert_eq!(observations.frames_delivered_total, 1);
        assert_eq!(observations.enqueue_to_receive_samples_total, 1);
        assert_eq!(observations.enqueue_to_receive_invalid_order_total, 0);
        assert_eq!(observations.enqueue_to_receive_max_ns, 90);
        assert_eq!(observations.source_timestamp_to_receive_samples_total, 1);
        assert_eq!(observations.source_timestamp_to_receive_max_ns, 100);
        assert_eq!(observations.source_timestamp_to_receive_missing_total, 0);
        assert_eq!(observations.source_timestamp_to_receive_future_total, 0);
    }

    #[test]
    fn given_foreign_clock_timestamp_when_delivered_then_source_latency_is_not_fabricated() {
        let telemetry = EdgeTelemetry::new(1);
        let pool = AudioBufferPool::new(1, 2);
        let frame = lineaged_frame_with_clock(&pool, 7, 1, 100, 0, ClockDomainId(99));
        let queued = QueuedPlanEdgeFrame {
            frame: PlanEdgeFrame::Exclusive(frame),
            enqueued_at_ns: 150,
        };

        telemetry.observe_delivery(&queued, 200);
        let observations = telemetry.snapshot();

        assert_eq!(observations.source_timestamp_to_receive_samples_total, 0);
        assert_eq!(observations.source_timestamp_to_receive_missing_total, 1);
        assert_eq!(observations.source_timestamp_to_receive_future_total, 0);
    }

    #[test]
    fn given_receive_before_enqueue_when_observed_then_latency_sample_is_rejected() {
        let telemetry = EdgeTelemetry::new(1);
        let pool = AudioBufferPool::new(1, 2);
        let queued = QueuedPlanEdgeFrame {
            frame: PlanEdgeFrame::Exclusive(frame(&pool, 7, 1)),
            enqueued_at_ns: 200,
        };

        telemetry.observe_delivery(&queued, 100);
        let observations = telemetry.snapshot();

        assert_eq!(observations.frames_delivered_total, 1);
        assert_eq!(observations.enqueue_to_receive_samples_total, 0);
        assert_eq!(observations.enqueue_to_receive_invalid_order_total, 1);
    }

    #[test]
    fn given_queued_frame_when_clocked_receive_runs_then_clock_is_sampled_after_pop() {
        let registry = registry();
        let mut graph = Pipeline::new();
        let source = graph.add_node("passthrough", NodeConfig::new());
        let sink = graph.add_node("passthrough", NodeConfig::new());
        graph.connect(source.out("out"), sink.in_("in"));
        let ir = Compiler::new()
            .compile(graph.into_spec(), &registry)
            .unwrap();
        let plan = RuntimePlanner::new().plan(&ir).unwrap();
        let (mut router, mut receivers) = PlanEdgeRouter::new(&plan, &ir).unwrap();
        let pool = AudioBufferPool::new(1, 2);
        router.dispatch_from(source.id(), "out", frame(&pool, 7, 1), 100);

        let receipt = receivers[0].recv_receipt_with_clock(|| 150).unwrap();
        let observations = receivers[0].observations();

        assert_eq!(receipt.enqueued_at_ns(), 100);
        assert_eq!(receipt.received_at_ns(), 150);
        assert_eq!(receipt.into_frame().sequence_number(), 1);
        assert_eq!(observations.enqueue_to_receive_samples_total, 1);
        assert_eq!(observations.enqueue_to_receive_invalid_order_total, 0);
        assert_eq!(observations.enqueue_to_receive_max_ns, 50);
    }

    #[test]
    fn given_enqueued_and_dropped_frames_when_observed_then_drop_rate_uses_all_attempts() {
        let observations = EdgeObservations {
            frames_enqueued_total: 3,
            frames_dropped_total: 1,
            ..EdgeObservations::default()
        };

        assert_eq!(observations.frames_attempted_total(), 4);
        assert!((observations.drop_rate_pct() - 25.0).abs() < f64::EPSILON);
    }

    #[test]
    fn given_sequence_and_timestamp_gap_when_received_then_typed_discontinuities_are_counted() {
        let registry = registry();
        let mut graph = Pipeline::new();
        let source = graph.add_node("passthrough", NodeConfig::new());
        let sink = graph.add_node("passthrough", NodeConfig::new());
        graph.connect(source.out("out"), sink.in_("in"));
        let ir = Compiler::new()
            .compile(graph.into_spec(), &registry)
            .unwrap();
        let plan = RuntimePlanner::new().plan(&ir).unwrap();
        let (mut router, mut receivers) = PlanEdgeRouter::new(&plan, &ir).unwrap();
        let pool = AudioBufferPool::new(1, 2);
        let make_frame = |sequence_number, timestamp_ns| {
            lineaged_frame(&pool, 1, sequence_number, timestamp_ns, 0)
        };

        router.dispatch_from(source.id(), "out", make_frame(1, 20_000_000), 20_000_001);
        receivers[0].recv_at(20_000_002).unwrap();
        router.dispatch_from(source.id(), "out", make_frame(3, 60_000_000), 60_000_001);
        receivers[0].recv_at(60_000_002).unwrap();
        let observations = receivers[0].observations();

        assert_eq!(observations.sequence_discontinuities_total, 1);
        assert_eq!(observations.timestamp_discontinuities_total, 1);
        assert_eq!(observations.discontinuities_total, 2);
    }

    #[test]
    fn given_two_sources_with_six_edges_when_dispatched_then_source_identity_stays_separate() {
        // Given
        let registry = registry();
        let mut graph = Pipeline::new();
        let first_source = graph.add_node("passthrough", NodeConfig::new());
        let second_source = graph.add_node("passthrough", NodeConfig::new());
        let first_source_id = first_source.id();
        let second_source_id = second_source.id();
        for source in [&first_source, &second_source] {
            for _ in 0..3 {
                let sink = graph.add_node("passthrough", NodeConfig::new());
                graph.connect(source.out("out"), sink.in_("in"));
            }
        }
        let ir = Compiler::new()
            .compile(graph.into_spec(), &registry)
            .unwrap();
        let plan = RuntimePlanner::new().plan(&ir).unwrap();
        let (mut router, mut receivers) = PlanEdgeRouter::new(&plan, &ir).unwrap();
        let first_pool = AudioBufferPool::new(1, 2);
        let second_pool = AudioBufferPool::new(1, 2);

        // When
        router.dispatch_from(first_source_id, "out", frame(&first_pool, 1, 1), 10);
        router.dispatch_from(second_source_id, "out", frame(&second_pool, 2, 1), 10);

        // Then
        let mut first_count = 0;
        let mut second_count = 0;
        for receiver in &mut receivers {
            match receiver.recv_at(20).unwrap().source_id() {
                SourceId(1) => first_count += 1,
                SourceId(2) => second_count += 1,
                source_id => panic!("unexpected source {source_id:?}"),
            }
        }
        assert_eq!(first_count, 3);
        assert_eq!(second_count, 3);
    }

    #[test]
    fn given_slow_full_branch_when_more_frames_dispatched_then_other_branch_continues() {
        // Given
        let registry = registry();
        let mut graph = Pipeline::new();
        let source = graph.add_node("passthrough", NodeConfig::new());
        let slow = graph.add_node("passthrough", NodeConfig::new());
        let fast = graph.add_node("passthrough", NodeConfig::new());
        let slow_edge = graph.connect(source.out("out"), slow.in_("in"));
        let fast_edge = graph.connect(source.out("out"), fast.in_("in"));
        let ir = Compiler::new()
            .compile(graph.into_spec(), &registry)
            .unwrap();
        let mut plan = RuntimePlanner::new().plan(&ir).unwrap();
        plan.memory_plan
            .edge_buffers
            .iter_mut()
            .for_each(|buffer| buffer.capacity_frames = 1);
        let (mut router, mut receivers) = PlanEdgeRouter::new(&plan, &ir).unwrap();
        let slow_index = receivers
            .iter()
            .position(|receiver| receiver.edge_id() == slow_edge)
            .unwrap();
        let fast_index = receivers
            .iter()
            .position(|receiver| receiver.edge_id() == fast_edge)
            .unwrap();
        let pool = AudioBufferPool::new(2, 2);
        router.dispatch_from(source.id(), "out", frame(&pool, 1, 1), 1);
        receivers[fast_index].recv_at(2).unwrap();

        // When
        let summary = router.dispatch_from(source.id(), "out", frame(&pool, 1, 2), 3);

        // Then
        assert_eq!(summary.enqueued_edges, 1);
        assert_eq!(summary.dropped_edges, 1);
        assert_eq!(
            receivers[fast_index].recv_at(4).unwrap().sequence_number(),
            2
        );
        let slow_observations = router.observations(slow_edge).unwrap();
        assert_eq!(slow_observations.overruns_total, 1);
        assert_eq!(slow_observations.queue_full_drops_total, 1);
        assert_eq!(slow_observations.branch_pool_exhausted_drops_total, 0);
        assert_eq!(
            router.observations(fast_edge).unwrap().frames_dropped_total,
            0
        );
        assert_eq!(receivers[slow_index].observations().queue_depth_frames, 1);
    }

    #[test]
    fn given_observation_handle_when_producer_fills_edge_then_live_queue_and_drop_are_visible() {
        // Given
        let registry = registry();
        let mut graph = Pipeline::new();
        let source = graph.add_node("passthrough", NodeConfig::new());
        let sink = graph.add_node("passthrough", NodeConfig::new());
        graph.connect(source.out("out"), sink.in_("in"));
        let ir = Compiler::new()
            .compile(graph.into_spec(), &registry)
            .unwrap();
        let mut plan = RuntimePlanner::new().plan(&ir).unwrap();
        plan.memory_plan
            .edge_buffers
            .iter_mut()
            .for_each(|buffer| buffer.capacity_frames = 1);
        let (mut router, receivers) = PlanEdgeRouter::new(&plan, &ir).unwrap();
        let observations = receivers[0].observation_handle();
        let pool = AudioBufferPool::new(2, 2);

        // When
        let first = router.dispatch_from(source.id(), "out", frame(&pool, 1, 1), 1);
        let second = router.dispatch_from(source.id(), "out", frame(&pool, 1, 2), 2);

        // Then
        assert_eq!(first.enqueued_edges, 1);
        assert_eq!(second.dropped_edges, 1);
        let snapshot = observations.observations();
        assert_eq!(snapshot.queue_depth_frames, 1);
        assert_eq!(snapshot.queue_peak_frames, 1);
        assert_eq!(snapshot.frames_enqueued_total, 1);
        assert_eq!(snapshot.frames_dropped_total, 1);
        assert_eq!(snapshot.queue_full_drops_total, 1);
        assert_eq!(snapshot.overruns_total, 1);
    }

    #[test]
    fn given_observation_handle_when_consumer_detects_gap_then_live_discontinuity_is_visible() {
        // Given
        let registry = registry();
        let mut graph = Pipeline::new();
        let source = graph.add_node("passthrough", NodeConfig::new());
        let sink = graph.add_node("passthrough", NodeConfig::new());
        graph.connect(source.out("out"), sink.in_("in"));
        let ir = Compiler::new()
            .compile(graph.into_spec(), &registry)
            .unwrap();
        let plan = RuntimePlanner::new().plan(&ir).unwrap();
        let (mut router, mut receivers) = PlanEdgeRouter::new(&plan, &ir).unwrap();
        let observations = receivers[0].observation_handle();
        let pool = AudioBufferPool::new(1, 2);

        // When
        router.dispatch_from(source.id(), "out", frame(&pool, 1, 1), 20_000_001);
        receivers[0].recv_at(20_000_002).unwrap();
        router.dispatch_from(source.id(), "out", frame(&pool, 1, 3), 60_000_001);
        receivers[0].recv_at(60_000_002).unwrap();

        // Then
        let snapshot = observations.observations();
        assert_eq!(snapshot.sequence_discontinuities_total, 1);
        assert_eq!(snapshot.timestamp_discontinuities_total, 1);
        assert_eq!(snapshot.discontinuities_total, 2);
    }

    #[test]
    fn given_observation_handle_when_receiver_drops_then_shutdown_snapshot_remains_available() {
        // Given
        let registry = registry();
        let mut graph = Pipeline::new();
        let source = graph.add_node("passthrough", NodeConfig::new());
        let sink = graph.add_node("passthrough", NodeConfig::new());
        graph.connect(source.out("out"), sink.in_("in"));
        let ir = Compiler::new()
            .compile(graph.into_spec(), &registry)
            .unwrap();
        let plan = RuntimePlanner::new().plan(&ir).unwrap();
        let (mut router, mut receivers) = PlanEdgeRouter::new(&plan, &ir).unwrap();
        let receiver = receivers.pop().unwrap();
        let observations = receiver.observation_handle();
        let cloned_observations = observations.clone();
        let pool = AudioBufferPool::new(1, 2);
        router.dispatch_from(source.id(), "out", frame(&pool, 1, 1), 1);

        // When
        drop(receiver);
        drop(router);

        // Then
        let snapshot = observations.observations();
        assert_eq!(snapshot.queue_depth_frames, 0);
        assert_eq!(snapshot.frames_enqueued_total, 1);
        assert_eq!(snapshot.frames_delivered_total, 0);
        assert_eq!(snapshot.shutdown_discarded_total, 1);
        assert_eq!(cloned_observations.observations(), snapshot);
    }

    #[test]
    fn given_receiver_holds_popped_frame_when_queue_has_room_then_next_copy_is_enqueued() {
        // Given a one-frame queue and its separately planned receiver in-flight slot.
        let registry = registry();
        let mut graph = Pipeline::new();
        let source = graph.add_node("passthrough", NodeConfig::new());
        let sink = graph.add_node("passthrough", NodeConfig::new());
        let mut contract = EdgeContract::realtime_audio();
        contract.copy_policy = CopyPolicy::CopyToBranchPool;
        graph.connect_with(source.out("out"), sink.in_("in"), contract);
        let ir = Compiler::new()
            .compile(graph.into_spec(), &registry)
            .unwrap();
        let mut plan = RuntimePlanner::new().plan(&ir).unwrap();
        plan.memory_plan
            .edge_buffers
            .iter_mut()
            .for_each(|buffer| buffer.capacity_frames = 1);
        let (mut router, mut receivers) = PlanEdgeRouter::new(&plan, &ir).unwrap();
        let pool = AudioBufferPool::new(3, 2);
        router.dispatch_from(source.id(), "out", frame(&pool, 1, 1), 1);
        let in_flight = receivers[0].recv_at(2).unwrap();

        // When the producer sends while the consumer still owns the popped frame.
        let summary = router.dispatch_from(source.id(), "out", frame(&pool, 1, 2), 3);

        // Then ownership headroom prevents false pool exhaustion.
        assert_eq!(summary.enqueued_edges, 1);
        assert_eq!(summary.dropped_edges, 0);
        assert_eq!(
            router
                .observations(receivers[0].edge_id())
                .unwrap()
                .branch_pool_exhausted_drops_total,
            0
        );

        let saturated = router.dispatch_from(source.id(), "out", frame(&pool, 1, 3), 4);
        let saturated_observations = router.observations(receivers[0].edge_id()).unwrap();
        assert_eq!(saturated.dropped_edges, 1);
        assert_eq!(saturated.copy_pool_exhausted_edges, 0);
        assert_eq!(saturated_observations.queue_full_drops_total, 1);
        assert_eq!(saturated_observations.branch_pool_exhausted_drops_total, 0);
        drop(in_flight);
        assert_eq!(receivers[0].recv_at(5).unwrap().sequence_number(), 2);
    }

    #[test]
    fn given_failed_branch_when_receiver_drops_then_unrelated_branch_continues() {
        // Given
        let registry = registry();
        let mut graph = Pipeline::new();
        let source = graph.add_node("passthrough", NodeConfig::new());
        let failed = graph.add_node("passthrough", NodeConfig::new());
        let healthy = graph.add_node("passthrough", NodeConfig::new());
        let failed_edge = graph.connect(source.out("out"), failed.in_("in"));
        let healthy_edge = graph.connect(source.out("out"), healthy.in_("in"));
        let ir = Compiler::new()
            .compile(graph.into_spec(), &registry)
            .unwrap();
        let plan = RuntimePlanner::new().plan(&ir).unwrap();
        let (mut router, mut receivers) = PlanEdgeRouter::new(&plan, &ir).unwrap();
        let failed_index = receivers
            .iter()
            .position(|receiver| receiver.edge_id() == failed_edge)
            .unwrap();
        receivers[failed_index].mark_worker_failure();
        let failed_receiver = receivers.swap_remove(failed_index);
        drop(failed_receiver);
        let pool = AudioBufferPool::new(1, 2);

        // When
        let summary = router.dispatch_from(source.id(), "out", frame(&pool, 1, 1), 10);

        // Then
        assert_eq!(summary.enqueued_edges, 1);
        assert_eq!(summary.dropped_edges, 1);
        let healthy_receiver = receivers
            .iter_mut()
            .find(|receiver| receiver.edge_id() == healthy_edge)
            .unwrap();
        assert_eq!(healthy_receiver.recv_at(20).unwrap().sequence_number(), 1);
        assert_eq!(
            router
                .observations(failed_edge)
                .unwrap()
                .worker_failures_total,
            1
        );
        assert_eq!(
            router
                .observations(failed_edge)
                .unwrap()
                .receiver_unavailable_drops_total,
            1
        );
    }

    #[test]
    fn given_shutdown_with_queued_shared_frames_when_receivers_drop_then_pool_slots_are_released() {
        // Given
        let registry = registry();
        let mut graph = Pipeline::new();
        let source = graph.add_node("passthrough", NodeConfig::new());
        let first = graph.add_node("passthrough", NodeConfig::new());
        let second = graph.add_node("passthrough", NodeConfig::new());
        graph.connect(source.out("out"), first.in_("in"));
        graph.connect(source.out("out"), second.in_("in"));
        let ir = Compiler::new()
            .compile(graph.into_spec(), &registry)
            .unwrap();
        let mut plan = RuntimePlanner::new().plan(&ir).unwrap();
        plan.memory_plan
            .edge_buffers
            .iter_mut()
            .for_each(|buffer| buffer.capacity_frames = 1);
        let (mut router, receivers) = PlanEdgeRouter::new(&plan, &ir).unwrap();
        let pool = AudioBufferPool::new(1, 2);
        router.dispatch_from(source.id(), "out", frame(&pool, 1, 1), 1);
        let branch_pools: Vec<_> = router
            .edges
            .iter()
            .filter_map(|edge| edge.branch_pool.as_ref().cloned())
            .collect();
        let receiver_in_flight_reservations: Vec<_> = branch_pools
            .iter()
            .map(|branch_pool| branch_pool.acquire().unwrap())
            .collect();
        assert!(branch_pools
            .iter()
            .all(|branch_pool| branch_pool.acquire().is_none()));

        // When
        drop(receivers);

        // Then
        assert!(branch_pools
            .iter()
            .all(|branch_pool| branch_pool.acquire().is_some()));
        drop(receiver_in_flight_reservations);
    }
}
