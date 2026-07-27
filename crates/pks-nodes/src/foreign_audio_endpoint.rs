use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::Duration;

use pks_endpoint::{
    EndpointCancellationOutcome, EndpointDriverFactory, EndpointDriverFinalization,
    EndpointDriverInput, EndpointDriverObservations, EndpointFailure, EndpointFailureStage,
    EndpointStartGate, PreparedEndpointDriver, RunningEndpointDriver,
};
use pks_frame::{ConnectorId, EndpointId, FrameLineage, LineagedAudioFrame, RouteId};
use pks_runtime::{PlanEdgeFrame, PlanEdgeObservationHandle, PlanEdgeReceiver};
use rtrb::{Consumer, Producer, PushError, RingBuffer};

const WORK_BUDGET_FRAMES: usize = 64;
const IDLE_POLL_INTERVAL: Duration = Duration::from_millis(1);
const MAX_QUEUE_CAPACITY_FRAMES: usize = 4_096;
const MAX_BATCH_CAPACITY_FRAMES: usize = 256;
const MAX_OUTSTANDING_LEASES: usize = 64;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PolledAudioEndpointConfig {
    pub queue_capacity_frames: usize,
    pub max_batch_frames: usize,
    pub max_outstanding_leases: usize,
}

impl Default for PolledAudioEndpointConfig {
    fn default() -> Self {
        Self {
            queue_capacity_frames: 32,
            max_batch_frames: 8,
            max_outstanding_leases: 4,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum PolledAudioEndpointConfigError {
    #[error("polled-audio queue capacity must be greater than zero frames")]
    ZeroQueueCapacity,
    #[error("polled-audio batch capacity must be greater than zero frames")]
    ZeroBatchCapacity,
    #[error("polled-audio lease capacity must be greater than zero")]
    ZeroLeaseCapacity,
    #[error("polled-audio queue capacity exceeds {MAX_QUEUE_CAPACITY_FRAMES} frames")]
    QueueCapacityTooLarge,
    #[error("polled-audio batch capacity exceeds {MAX_BATCH_CAPACITY_FRAMES} frames")]
    BatchCapacityTooLarge,
    #[error("polled-audio lease capacity exceeds {MAX_OUTSTANDING_LEASES}")]
    LeaseCapacityTooLarge,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct PolledAudioObservations {
    pub registered_endpoints: u64,
    pub queue_capacity_frames: u64,
    pub queue_depth_frames: u64,
    pub queue_peak_frames: u64,
    pub queue_depth_invariant_failures_total: u64,
    pub frames_received_total: u64,
    pub frames_delivered_total: u64,
    pub queue_full_drops_total: u64,
    pub invalid_ownership_drops_total: u64,
    pub lease_capacity_count: u64,
    pub outstanding_leases: u64,
    pub lease_exhausted_total: u64,
    pub batches_polled_total: u64,
    pub frames_polled_total: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum PolledAudioPollError {
    #[error("polled-audio queue is empty")]
    Empty,
    #[error("polled-audio lease capacity is exhausted")]
    LeaseCapacityExhausted,
    #[error("polled-audio receipt state is poisoned")]
    StatePoisoned,
}

pub struct PolledAudioEndpointFactory {
    config: PolledAudioEndpointConfig,
    shared: Arc<ReceiptShared>,
}

impl PolledAudioEndpointFactory {
    pub fn new(
        config: PolledAudioEndpointConfig,
    ) -> Result<(Self, PolledAudioReceipt), PolledAudioEndpointConfigError> {
        validate_config(config)?;
        let shared = Arc::new(ReceiptShared::new(config));
        Ok((
            Self {
                config,
                shared: Arc::clone(&shared),
            },
            PolledAudioReceipt { shared },
        ))
    }
}

#[derive(Clone)]
pub struct PolledAudioReceipt {
    shared: Arc<ReceiptShared>,
}

impl PolledAudioReceipt {
    pub fn try_poll(&self) -> Result<PolledAudioBatchLease, PolledAudioPollError> {
        let mut state = self
            .shared
            .state
            .lock()
            .map_err(|_| PolledAudioPollError::StatePoisoned)?;
        let Some(mut frames) = state.recycled_batches.pop() else {
            self.shared
                .lease_exhausted_total
                .fetch_add(1, Ordering::Relaxed);
            return Err(PolledAudioPollError::LeaseCapacityExhausted);
        };

        let consumer_count = state.consumers.len();
        if consumer_count == 0 {
            state.recycled_batches.push(frames);
            return Err(PolledAudioPollError::Empty);
        }

        let mut visited = 0;
        while frames.len() < self.shared.max_batch_frames && visited < consumer_count {
            let index = state.next_consumer % consumer_count;
            state.next_consumer = (index + 1) % consumer_count;
            visited += 1;
            let Some(consumer) = state.consumers[index].as_mut() else {
                continue;
            };
            while frames.len() < self.shared.max_batch_frames {
                let Ok(frame) = consumer.pop() else {
                    break;
                };
                frames.push(frame);
                self.shared.observe_dequeued(1);
            }
        }

        if frames.is_empty() {
            state.recycled_batches.push(frames);
            return Err(PolledAudioPollError::Empty);
        }
        drop(state);

        self.shared
            .outstanding_leases
            .fetch_add(1, Ordering::Relaxed);
        self.shared
            .batches_polled_total
            .fetch_add(1, Ordering::Relaxed);
        self.shared
            .frames_polled_total
            .fetch_add(frames.len() as u64, Ordering::Relaxed);
        Ok(PolledAudioBatchLease {
            shared: Arc::clone(&self.shared),
            frames: Some(frames),
        })
    }

    pub fn observations(&self) -> PolledAudioObservations {
        self.shared.observations()
    }
}

pub struct PolledAudioBatchLease {
    shared: Arc<ReceiptShared>,
    frames: Option<Vec<DeliveredAudioFrame>>,
}

impl PolledAudioBatchLease {
    pub fn len(&self) -> usize {
        self.frames.as_ref().map_or(0, Vec::len)
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn frame(&self, index: usize) -> Option<PolledAudioFrame<'_>> {
        self.frames
            .as_ref()?
            .get(index)
            .map(|delivered| PolledAudioFrame { delivered })
    }
}

impl Drop for PolledAudioBatchLease {
    fn drop(&mut self) {
        let Some(mut frames) = self.frames.take() else {
            return;
        };
        frames.clear();
        self.shared
            .outstanding_leases
            .fetch_sub(1, Ordering::Relaxed);
        if let Ok(mut state) = self.shared.state.lock() {
            state.recycled_batches.push(frames);
        }
    }
}

#[derive(Clone, Copy)]
pub struct PolledAudioFrame<'lease> {
    delivered: &'lease DeliveredAudioFrame,
}

impl<'lease> PolledAudioFrame<'lease> {
    pub fn lineage(self) -> FrameLineage {
        self.delivered.frame.lineage()
    }

    pub fn endpoint_id(self) -> EndpointId {
        self.delivered.endpoint_id
    }

    pub fn route_id(self) -> RouteId {
        self.delivered.route_id
    }

    pub fn connector_id(self) -> ConnectorId {
        self.delivered.connector_id
    }

    pub fn sample_rate_hz(self) -> u32 {
        self.delivered.frame.frame().sample_rate_hz
    }

    pub fn channels(self) -> u8 {
        self.delivered.frame.frame().channels
    }

    pub fn samples(self) -> &'lease [f32] {
        self.delivered.frame.frame().buffer.as_slice()
    }
}

#[derive(Debug)]
struct DeliveredAudioFrame {
    endpoint_id: EndpointId,
    connector_id: ConnectorId,
    route_id: RouteId,
    frame: LineagedAudioFrame,
}

struct ReceiptState {
    consumers: Vec<Option<Consumer<DeliveredAudioFrame>>>,
    recycled_batches: Vec<Vec<DeliveredAudioFrame>>,
    next_consumer: usize,
}

struct ReceiptShared {
    state: Mutex<ReceiptState>,
    max_batch_frames: usize,
    registered_endpoints: AtomicU64,
    queue_capacity_frames: AtomicU64,
    queue_depth_frames: AtomicU64,
    queue_peak_frames: AtomicU64,
    queue_depth_invariant_failures_total: AtomicU64,
    frames_received_total: AtomicU64,
    frames_delivered_total: AtomicU64,
    queue_full_drops_total: AtomicU64,
    invalid_ownership_drops_total: AtomicU64,
    lease_capacity_count: u64,
    outstanding_leases: AtomicU64,
    lease_exhausted_total: AtomicU64,
    batches_polled_total: AtomicU64,
    frames_polled_total: AtomicU64,
}

impl ReceiptShared {
    fn new(config: PolledAudioEndpointConfig) -> Self {
        let mut recycled_batches = Vec::with_capacity(config.max_outstanding_leases);
        for _ in 0..config.max_outstanding_leases {
            recycled_batches.push(Vec::with_capacity(config.max_batch_frames));
        }
        Self {
            state: Mutex::new(ReceiptState {
                consumers: Vec::new(),
                recycled_batches,
                next_consumer: 0,
            }),
            max_batch_frames: config.max_batch_frames,
            registered_endpoints: AtomicU64::new(0),
            queue_capacity_frames: AtomicU64::new(0),
            queue_depth_frames: AtomicU64::new(0),
            queue_peak_frames: AtomicU64::new(0),
            queue_depth_invariant_failures_total: AtomicU64::new(0),
            frames_received_total: AtomicU64::new(0),
            frames_delivered_total: AtomicU64::new(0),
            queue_full_drops_total: AtomicU64::new(0),
            invalid_ownership_drops_total: AtomicU64::new(0),
            lease_capacity_count: config.max_outstanding_leases as u64,
            outstanding_leases: AtomicU64::new(0),
            lease_exhausted_total: AtomicU64::new(0),
            batches_polled_total: AtomicU64::new(0),
            frames_polled_total: AtomicU64::new(0),
        }
    }

    fn register_consumer(
        &self,
        consumer: Consumer<DeliveredAudioFrame>,
        capacity_frames: usize,
    ) -> Result<usize, EndpointFailure> {
        let mut state = self
            .state
            .lock()
            .map_err(|_| prepare_failure("polled-audio receipt state is poisoned"))?;
        let slot = state
            .consumers
            .iter()
            .position(Option::is_none)
            .unwrap_or(state.consumers.len());
        if slot == state.consumers.len() {
            state.consumers.push(Some(consumer));
        } else {
            state.consumers[slot] = Some(consumer);
        }
        self.registered_endpoints.fetch_add(1, Ordering::Relaxed);
        self.queue_capacity_frames
            .fetch_add(capacity_frames as u64, Ordering::Relaxed);
        Ok(slot)
    }

    fn remove_consumer(&self, slot: usize, capacity_frames: usize) -> Result<(), EndpointFailure> {
        let mut state = self.state.lock().map_err(|_| {
            EndpointFailure::new(
                EndpointFailureStage::JoinFinalize,
                "polled-audio receipt state is poisoned",
            )
        })?;
        let mut consumer = state
            .consumers
            .get_mut(slot)
            .and_then(Option::take)
            .ok_or_else(|| {
                EndpointFailure::new(
                    EndpointFailureStage::JoinFinalize,
                    "polled-audio receipt consumer is unavailable",
                )
            })?;
        let mut discarded = 0_u64;
        while consumer.pop().is_ok() {
            discarded = discarded.saturating_add(1);
        }
        self.observe_dequeued(discarded);
        drop(consumer);
        self.registered_endpoints.fetch_sub(1, Ordering::Relaxed);
        self.queue_capacity_frames
            .fetch_sub(capacity_frames as u64, Ordering::Relaxed);
        Ok(())
    }

    fn try_reserve_queue_slot(&self) -> bool {
        let capacity = self.queue_capacity_frames.load(Ordering::Acquire);
        let mut depth = self.queue_depth_frames.load(Ordering::Relaxed);
        loop {
            if depth >= capacity {
                return false;
            }
            match self.queue_depth_frames.compare_exchange_weak(
                depth,
                depth + 1,
                Ordering::AcqRel,
                Ordering::Relaxed,
            ) {
                Ok(_) => return true,
                Err(observed) => depth = observed,
            }
        }
    }

    fn observe_enqueued(&self) {
        self.frames_delivered_total.fetch_add(1, Ordering::Relaxed);
        let depth = self.queue_depth_frames.load(Ordering::Relaxed);
        self.queue_peak_frames.fetch_max(depth, Ordering::Relaxed);
    }

    fn release_queue_reservation(&self) {
        self.observe_dequeued(1);
    }

    fn observe_queue_full_drop(&self) {
        self.queue_full_drops_total.fetch_add(1, Ordering::Relaxed);
    }

    fn observe_dequeued(&self, frame_count: u64) {
        if frame_count == 0 {
            return;
        }
        let result =
            self.queue_depth_frames
                .fetch_update(Ordering::AcqRel, Ordering::Relaxed, |depth| {
                    depth.checked_sub(frame_count)
                });
        if result.is_err() {
            self.queue_depth_invariant_failures_total
                .fetch_add(1, Ordering::Relaxed);
            self.queue_depth_frames.store(0, Ordering::Release);
        }
    }

    fn observations(&self) -> PolledAudioObservations {
        PolledAudioObservations {
            registered_endpoints: self.registered_endpoints.load(Ordering::Relaxed),
            queue_capacity_frames: self.queue_capacity_frames.load(Ordering::Relaxed),
            queue_depth_frames: self.queue_depth_frames.load(Ordering::Relaxed),
            queue_peak_frames: self.queue_peak_frames.load(Ordering::Relaxed),
            queue_depth_invariant_failures_total: self
                .queue_depth_invariant_failures_total
                .load(Ordering::Relaxed),
            frames_received_total: self.frames_received_total.load(Ordering::Relaxed),
            frames_delivered_total: self.frames_delivered_total.load(Ordering::Relaxed),
            queue_full_drops_total: self.queue_full_drops_total.load(Ordering::Relaxed),
            invalid_ownership_drops_total: self
                .invalid_ownership_drops_total
                .load(Ordering::Relaxed),
            lease_capacity_count: self.lease_capacity_count,
            outstanding_leases: self.outstanding_leases.load(Ordering::Relaxed),
            lease_exhausted_total: self.lease_exhausted_total.load(Ordering::Relaxed),
            batches_polled_total: self.batches_polled_total.load(Ordering::Relaxed),
            frames_polled_total: self.frames_polled_total.load(Ordering::Relaxed),
        }
    }
}

impl EndpointDriverFactory for PolledAudioEndpointFactory {
    fn prepare(
        &self,
        mut inputs: Vec<EndpointDriverInput>,
    ) -> Result<Box<dyn PreparedEndpointDriver>, EndpointFailure> {
        if inputs.len() != 1 {
            return Err(prepare_failure(
                "one polled-audio endpoint requires exactly one input",
            ));
        }
        let (receiver, context) = inputs
            .pop()
            .expect("length was checked before removing the input")
            .into_parts();
        let connector_id = parse_u64(&context, "connector_id").map(ConnectorId)?;
        let route_id = parse_u64(&context, "route_id").map(RouteId)?;
        let endpoint_id = context.endpoint_id();
        let edge_observations = receiver.observation_handle();
        let (producer, consumer) = RingBuffer::new(self.config.queue_capacity_frames);
        let consumer_slot = self
            .shared
            .register_consumer(consumer, self.config.queue_capacity_frames)?;
        Ok(Box::new(PreparedPolledAudioEndpoint {
            receiver,
            producer,
            endpoint_id,
            connector_id,
            route_id,
            consumer_slot,
            queue_capacity_frames: self.config.queue_capacity_frames,
            shared: Arc::clone(&self.shared),
            edge_observations,
        }))
    }
}

struct PreparedPolledAudioEndpoint {
    receiver: PlanEdgeReceiver,
    producer: Producer<DeliveredAudioFrame>,
    endpoint_id: EndpointId,
    connector_id: ConnectorId,
    route_id: RouteId,
    consumer_slot: usize,
    queue_capacity_frames: usize,
    shared: Arc<ReceiptShared>,
    edge_observations: PlanEdgeObservationHandle,
}

impl PreparedEndpointDriver for PreparedPolledAudioEndpoint {
    fn start(
        self: Box<Self>,
        start_gate: Arc<EndpointStartGate>,
    ) -> Result<Box<dyn RunningEndpointDriver>, EndpointFailure> {
        let stop = Arc::new(AtomicBool::new(false));
        let observations = Arc::new(WorkerObservations::default());
        let thread_stop = Arc::clone(&stop);
        let thread_observations = Arc::clone(&observations);
        let thread_shared = Arc::clone(&self.shared);
        let endpoint_id = self.endpoint_id;
        let connector_id = self.connector_id;
        let route_id = self.route_id;
        let consumer_slot = self.consumer_slot;
        let queue_capacity_frames = self.queue_capacity_frames;
        let edge_observations = self.edge_observations.clone();
        let mut receiver = self.receiver;
        let mut producer = self.producer;
        let worker = thread::Builder::new()
            .name(format!("pks-polled-audio-{}", endpoint_id.0))
            .spawn(move || {
                run_worker(
                    &mut receiver,
                    &mut producer,
                    endpoint_id,
                    connector_id,
                    route_id,
                    &thread_shared,
                    &thread_observations,
                    &thread_stop,
                    &start_gate,
                )
            })
            .map_err(|error| {
                let _ = self
                    .shared
                    .remove_consumer(consumer_slot, queue_capacity_frames);
                EndpointFailure::new(EndpointFailureStage::Start, error.to_string())
            })?;
        Ok(Box::new(RunningPolledAudioEndpoint {
            stop,
            worker: Some(worker),
            observations,
            shared: self.shared,
            consumer_slot,
            queue_capacity_frames,
            edge_observations,
        }))
    }

    fn cancel_preparation(self: Box<Self>) -> EndpointCancellationOutcome {
        let result = self
            .shared
            .remove_consumer(self.consumer_slot, self.queue_capacity_frames)
            .map_err(|failure| {
                EndpointFailure::new(EndpointFailureStage::CancelPreparation, failure.message())
            });
        EndpointCancellationOutcome {
            observations: EndpointDriverObservations::default(),
            result,
        }
    }
}

struct RunningPolledAudioEndpoint {
    stop: Arc<AtomicBool>,
    worker: Option<JoinHandle<()>>,
    observations: Arc<WorkerObservations>,
    shared: Arc<ReceiptShared>,
    consumer_slot: usize,
    queue_capacity_frames: usize,
    edge_observations: PlanEdgeObservationHandle,
}

impl RunningEndpointDriver for RunningPolledAudioEndpoint {
    fn observations(&self) -> EndpointDriverObservations {
        endpoint_observations(&self.observations, &self.edge_observations)
    }

    fn request_stop(&mut self) -> Result<(), EndpointFailure> {
        self.stop.store(true, Ordering::Release);
        Ok(())
    }

    fn join_and_finalize(mut self: Box<Self>) -> EndpointDriverFinalization {
        self.stop.store(true, Ordering::Release);
        let join_result = self
            .worker
            .take()
            .expect("running polled-audio endpoint owns one worker")
            .join()
            .map_err(|_| {
                EndpointFailure::new(
                    EndpointFailureStage::JoinFinalize,
                    "polled-audio worker panicked",
                )
            });
        let remove_result = self
            .shared
            .remove_consumer(self.consumer_slot, self.queue_capacity_frames);
        let ownership_failures = self
            .observations
            .invalid_ownership_drops_total
            .load(Ordering::Relaxed);
        let result = join_result.and(remove_result).and_then(|()| {
            if ownership_failures == 0 {
                Ok(())
            } else {
                Err(EndpointFailure::new(
                    EndpointFailureStage::JoinFinalize,
                    "polled-audio endpoint received a non-lineaged or non-branch-copy frame",
                ))
            }
        });
        EndpointDriverFinalization {
            observations: endpoint_observations(&self.observations, &self.edge_observations),
            result,
        }
    }
}

#[derive(Default)]
struct WorkerObservations {
    frames_received_total: AtomicU64,
    frames_delivered_total: AtomicU64,
    queue_full_drops_total: AtomicU64,
    invalid_ownership_drops_total: AtomicU64,
}

#[allow(clippy::too_many_arguments)]
fn run_worker(
    receiver: &mut PlanEdgeReceiver,
    producer: &mut Producer<DeliveredAudioFrame>,
    endpoint_id: EndpointId,
    connector_id: ConnectorId,
    route_id: RouteId,
    shared: &ReceiptShared,
    observations: &WorkerObservations,
    stop: &AtomicBool,
    start_gate: &EndpointStartGate,
) {
    while !start_gate.is_open() {
        if stop.load(Ordering::Acquire) {
            return;
        }
        thread::sleep(IDLE_POLL_INTERVAL);
    }
    loop {
        let mut progressed = false;
        for _ in 0..WORK_BUDGET_FRAMES {
            let Some(frame) = receiver.try_recv() else {
                break;
            };
            progressed = true;
            observations
                .frames_received_total
                .fetch_add(1, Ordering::Relaxed);
            shared.frames_received_total.fetch_add(1, Ordering::Relaxed);
            let Some(delivered) = prepare_delivered_frame(
                frame,
                endpoint_id,
                connector_id,
                route_id,
                shared,
                observations,
            ) else {
                receiver.mark_worker_failure();
                continue;
            };
            publish_delivered_frame(producer, delivered, shared, observations);
        }
        if stop.load(Ordering::Acquire) && !progressed {
            return;
        }
        if !progressed {
            thread::sleep(IDLE_POLL_INTERVAL);
        }
    }
}

fn publish_delivered_frame(
    producer: &mut Producer<DeliveredAudioFrame>,
    delivered: DeliveredAudioFrame,
    shared: &ReceiptShared,
    observations: &WorkerObservations,
) {
    if !shared.try_reserve_queue_slot() {
        observations
            .queue_full_drops_total
            .fetch_add(1, Ordering::Relaxed);
        shared.observe_queue_full_drop();
        return;
    }
    match producer.push(delivered) {
        Ok(()) => {
            observations
                .frames_delivered_total
                .fetch_add(1, Ordering::Relaxed);
            shared.observe_enqueued();
        }
        Err(PushError::Full(_frame)) => {
            shared.release_queue_reservation();
            observations
                .queue_full_drops_total
                .fetch_add(1, Ordering::Relaxed);
            shared.observe_queue_full_drop();
        }
    }
}

fn prepare_delivered_frame(
    frame: PlanEdgeFrame,
    endpoint_id: EndpointId,
    connector_id: ConnectorId,
    route_id: RouteId,
    shared: &ReceiptShared,
    observations: &WorkerObservations,
) -> Option<DeliveredAudioFrame> {
    let PlanEdgeFrame::LineagedExclusive(frame) = frame else {
        observations
            .invalid_ownership_drops_total
            .fetch_add(1, Ordering::Relaxed);
        shared
            .invalid_ownership_drops_total
            .fetch_add(1, Ordering::Relaxed);
        return None;
    };
    Some(DeliveredAudioFrame {
        endpoint_id,
        connector_id,
        route_id,
        frame,
    })
}

fn endpoint_observations(
    worker: &WorkerObservations,
    edge: &PlanEdgeObservationHandle,
) -> EndpointDriverObservations {
    let edge = edge.observations();
    let queue_full = worker.queue_full_drops_total.load(Ordering::Relaxed);
    let invalid_ownership = worker.invalid_ownership_drops_total.load(Ordering::Relaxed);
    EndpointDriverObservations {
        frames_received_total: worker.frames_received_total.load(Ordering::Relaxed),
        frames_delivered_total: worker.frames_delivered_total.load(Ordering::Relaxed),
        frames_dropped_total: queue_full.saturating_add(invalid_ownership),
        discontinuities_total: edge.discontinuities_total,
        failures_total: invalid_ownership,
    }
}

fn parse_u64(
    context: &pks_endpoint::EndpointPrepareContext,
    key: &'static str,
) -> Result<u64, EndpointFailure> {
    context
        .node_configuration()
        .get(key)
        .ok_or_else(|| prepare_failure(format!("polled-audio endpoint is missing {key}")))?
        .parse()
        .map_err(|_| prepare_failure(format!("polled-audio endpoint has invalid {key}")))
}

fn prepare_failure(message: impl Into<String>) -> EndpointFailure {
    EndpointFailure::new(EndpointFailureStage::Prepare, message)
}

fn validate_config(
    config: PolledAudioEndpointConfig,
) -> Result<(), PolledAudioEndpointConfigError> {
    if config.queue_capacity_frames == 0 {
        Err(PolledAudioEndpointConfigError::ZeroQueueCapacity)
    } else if config.queue_capacity_frames > MAX_QUEUE_CAPACITY_FRAMES {
        Err(PolledAudioEndpointConfigError::QueueCapacityTooLarge)
    } else if config.max_batch_frames == 0 {
        Err(PolledAudioEndpointConfigError::ZeroBatchCapacity)
    } else if config.max_batch_frames > MAX_BATCH_CAPACITY_FRAMES {
        Err(PolledAudioEndpointConfigError::BatchCapacityTooLarge)
    } else if config.max_outstanding_leases == 0 {
        Err(PolledAudioEndpointConfigError::ZeroLeaseCapacity)
    } else if config.max_outstanding_leases > MAX_OUTSTANDING_LEASES {
        Err(PolledAudioEndpointConfigError::LeaseCapacityTooLarge)
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use pks_frame::{
        AudioBufferPool, AudioFrame, ClockDomainId, SessionId, SourceId, StemId, StreamId,
    };

    use super::*;

    const ENDPOINT_ID: EndpointId = EndpointId(31);
    const CONNECTOR_ID: ConnectorId = ConnectorId(41);
    const ROUTE_ID: RouteId = RouteId(51);

    fn lineaged_frame(
        pool: &Arc<AudioBufferPool>,
        sequence_number: u64,
        sample_linear: f32,
    ) -> LineagedAudioFrame {
        let mut buffer = pool.acquire().expect("test pool slot");
        buffer.copy_from_slice(&[sample_linear, sample_linear, sample_linear, sample_linear]);
        let frame = AudioFrame::new(
            StreamId(1),
            SourceId(2),
            sequence_number,
            sequence_number.saturating_mul(1_000_000),
            1,
            buffer,
        );
        let lineage = FrameLineage {
            session_id: SessionId(3),
            source_id: SourceId(2),
            stem_id: StemId(4),
            clock_id: ClockDomainId(5),
            sequence_num: sequence_number,
            timestamp_start_ns: sequence_number.saturating_mul(1_000_000),
            duration_ns: 83_333,
            source_generation: 1,
            discontinuity_epoch: 0,
            permission_epoch: 1,
        };
        LineagedAudioFrame::new(frame, lineage).expect("valid test lineage")
    }

    fn delivered(frame: LineagedAudioFrame) -> DeliveredAudioFrame {
        DeliveredAudioFrame {
            endpoint_id: ENDPOINT_ID,
            connector_id: CONNECTOR_ID,
            route_id: ROUTE_ID,
            frame,
        }
    }

    #[test]
    fn given_non_branch_or_non_lineaged_variants_when_published_then_each_is_rejected_and_counted()
    {
        let config = PolledAudioEndpointConfig::default();
        let shared = ReceiptShared::new(config);
        let observations = WorkerObservations::default();

        let raw_pool = AudioBufferPool::new(1, 4);
        let raw = lineaged_frame(&raw_pool, 1, 0.1).into_parts().0;
        assert!(prepare_delivered_frame(
            PlanEdgeFrame::Exclusive(raw),
            ENDPOINT_ID,
            CONNECTOR_ID,
            ROUTE_ID,
            &shared,
            &observations,
        )
        .is_none());

        let shared_pool = AudioBufferPool::new(1, 4);
        let shared_raw = lineaged_frame(&shared_pool, 2, 0.2)
            .into_parts()
            .0
            .freeze()
            .expect("shared frame");
        assert!(prepare_delivered_frame(
            PlanEdgeFrame::Shared(shared_raw),
            ENDPOINT_ID,
            CONNECTOR_ID,
            ROUTE_ID,
            &shared,
            &observations,
        )
        .is_none());

        let lineaged_shared_pool = AudioBufferPool::new(1, 4);
        let lineaged_shared = lineaged_frame(&lineaged_shared_pool, 3, 0.3)
            .freeze()
            .expect("shared lineaged frame");
        assert!(prepare_delivered_frame(
            PlanEdgeFrame::LineagedShared(lineaged_shared),
            ENDPOINT_ID,
            CONNECTOR_ID,
            ROUTE_ID,
            &shared,
            &observations,
        )
        .is_none());

        assert_eq!(
            observations
                .invalid_ownership_drops_total
                .load(Ordering::Relaxed),
            3
        );
        assert_eq!(
            shared.invalid_ownership_drops_total.load(Ordering::Relaxed),
            3
        );
    }

    #[test]
    fn given_held_batch_when_polled_then_samples_stay_stable_and_lease_exhaustion_is_counted() {
        let config = PolledAudioEndpointConfig {
            queue_capacity_frames: 2,
            max_batch_frames: 1,
            max_outstanding_leases: 1,
        };
        let shared = Arc::new(ReceiptShared::new(config));
        let receipt = PolledAudioReceipt {
            shared: Arc::clone(&shared),
        };
        let (mut producer, consumer) = RingBuffer::new(config.queue_capacity_frames);
        let consumer_slot = shared
            .register_consumer(consumer, config.queue_capacity_frames)
            .expect("consumer registration");
        let pool = AudioBufferPool::new(2, 4);
        let worker = WorkerObservations::default();
        publish_delivered_frame(
            &mut producer,
            delivered(lineaged_frame(&pool, 1, 0.25)),
            &shared,
            &worker,
        );
        publish_delivered_frame(
            &mut producer,
            delivered(lineaged_frame(&pool, 2, 0.75)),
            &shared,
            &worker,
        );

        let lease = receipt.try_poll().expect("first lease");
        let frame = lease.frame(0).expect("leased frame");
        let samples_pointer = frame.samples().as_ptr();
        assert_eq!(frame.samples(), &[0.25, 0.25, 0.25, 0.25]);
        assert_eq!(frame.samples().as_ptr(), samples_pointer);
        assert!(matches!(
            receipt.try_poll(),
            Err(PolledAudioPollError::LeaseCapacityExhausted)
        ));
        assert_eq!(receipt.observations().outstanding_leases, 1);
        assert_eq!(receipt.observations().lease_exhausted_total, 1);

        drop(lease);
        let second = receipt.try_poll().expect("recycled lease");
        assert_eq!(
            second.frame(0).expect("second frame").samples(),
            &[0.75, 0.75, 0.75, 0.75]
        );
        drop(second);
        drop(producer);
        shared
            .remove_consumer(consumer_slot, config.queue_capacity_frames)
            .expect("consumer removal");
        assert_eq!(receipt.observations().outstanding_leases, 0);
    }

    #[test]
    fn given_concurrent_publish_and_poll_when_observed_then_depth_stays_bounded_and_returns_to_zero(
    ) {
        let config = PolledAudioEndpointConfig {
            queue_capacity_frames: 8,
            max_batch_frames: 4,
            max_outstanding_leases: 2,
        };
        let shared = Arc::new(ReceiptShared::new(config));
        let receipt = PolledAudioReceipt {
            shared: Arc::clone(&shared),
        };
        let (mut producer, consumer) = RingBuffer::new(config.queue_capacity_frames);
        let consumer_slot = shared
            .register_consumer(consumer, config.queue_capacity_frames)
            .expect("consumer registration");
        let producer_shared = Arc::clone(&shared);
        let producer_done = Arc::new(AtomicBool::new(false));
        let thread_done = Arc::clone(&producer_done);
        let worker = Arc::new(WorkerObservations::default());
        let thread_worker = Arc::clone(&worker);
        let producer_thread = thread::spawn(move || {
            let pool = AudioBufferPool::new(16, 4);
            for sequence_number in 0..2_000 {
                let frame = loop {
                    if let Some(buffer) = pool.acquire() {
                        let frame = AudioFrame::new(
                            StreamId(1),
                            SourceId(2),
                            sequence_number,
                            sequence_number.saturating_mul(1_000_000),
                            1,
                            buffer,
                        );
                        let lineage = FrameLineage {
                            session_id: SessionId(3),
                            source_id: SourceId(2),
                            stem_id: StemId(4),
                            clock_id: ClockDomainId(5),
                            sequence_num: sequence_number,
                            timestamp_start_ns: sequence_number.saturating_mul(1_000_000),
                            duration_ns: 83_333,
                            source_generation: 1,
                            discontinuity_epoch: 0,
                            permission_epoch: 1,
                        };
                        break LineagedAudioFrame::new(frame, lineage)
                            .expect("valid concurrent lineage");
                    }
                    thread::yield_now();
                };
                publish_delivered_frame(
                    &mut producer,
                    delivered(frame),
                    &producer_shared,
                    &thread_worker,
                );
                if sequence_number % 7 == 0 {
                    thread::yield_now();
                }
            }
            thread_done.store(true, Ordering::Release);
        });

        let mut maximum_depth = 0;
        loop {
            match receipt.try_poll() {
                Ok(lease) => drop(lease),
                Err(PolledAudioPollError::Empty) => thread::yield_now(),
                Err(error) => panic!("unexpected concurrent poll result: {error}"),
            }
            let depth = receipt.observations().queue_depth_frames;
            maximum_depth = maximum_depth.max(depth);
            assert!(depth <= config.queue_capacity_frames as u64);
            if producer_done.load(Ordering::Acquire) && depth == 0 {
                break;
            }
        }
        producer_thread.join().expect("producer thread");
        assert!(maximum_depth <= config.queue_capacity_frames as u64);
        assert_eq!(receipt.observations().queue_depth_frames, 0);
        assert_eq!(
            receipt.observations().queue_depth_invariant_failures_total,
            0
        );
        shared
            .remove_consumer(consumer_slot, config.queue_capacity_frames)
            .expect("consumer removal");
    }

    #[test]
    fn given_untrusted_oversized_capacities_when_constructed_then_all_fail_before_allocation() {
        for (config, expected) in [
            (
                PolledAudioEndpointConfig {
                    queue_capacity_frames: MAX_QUEUE_CAPACITY_FRAMES + 1,
                    ..PolledAudioEndpointConfig::default()
                },
                PolledAudioEndpointConfigError::QueueCapacityTooLarge,
            ),
            (
                PolledAudioEndpointConfig {
                    max_batch_frames: MAX_BATCH_CAPACITY_FRAMES + 1,
                    ..PolledAudioEndpointConfig::default()
                },
                PolledAudioEndpointConfigError::BatchCapacityTooLarge,
            ),
            (
                PolledAudioEndpointConfig {
                    max_outstanding_leases: MAX_OUTSTANDING_LEASES + 1,
                    ..PolledAudioEndpointConfig::default()
                },
                PolledAudioEndpointConfigError::LeaseCapacityTooLarge,
            ),
        ] {
            assert_eq!(
                PolledAudioEndpointFactory::new(config)
                    .err()
                    .expect("oversized config must fail"),
                expected
            );
        }
    }

    #[test]
    fn given_impossible_dequeue_when_observed_then_depth_saturates_and_failure_is_explicit() {
        let shared = ReceiptShared::new(PolledAudioEndpointConfig::default());

        shared.observe_dequeued(1);

        assert_eq!(shared.observations().queue_depth_frames, 0);
        assert_eq!(
            shared.observations().queue_depth_invariant_failures_total,
            1
        );
    }
}
