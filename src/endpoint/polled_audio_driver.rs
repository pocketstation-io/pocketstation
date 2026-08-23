use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Condvar, Mutex};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

use crate::endpoint::{
    EndpointAudioReceiver, EndpointCancellationOutcome, EndpointDriverFactory,
    EndpointDriverFinalization, EndpointDriverObservations, EndpointFailure, EndpointFailureStage,
    EndpointPortInput, EndpointReceiver, EndpointStartGate, PreparedEndpointDriver,
    RunningEndpointDriver,
};
use crate::frame::{ConnectorId, EndpointId, FrameLineage, LineagedAudioFrame, RouteId, StreamId};
use crate::runtime::{PlanEdgeFrame, PlanEdgeObservationHandle};
use rtrb::{Consumer, Producer, PushError, RingBuffer};

const WORK_BUDGET_FRAMES: usize = 64;
const IDLE_POLL_INTERVAL: Duration = Duration::from_millis(1);
const MAX_QUEUE_CAPACITY_FRAMES: usize = 4_096;
const MAX_BATCH_CAPACITY_FRAMES: usize = 256;
const MAX_OUTSTANDING_LEASES: usize = 64;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "Configures polled audio endpoint behavior at its owning API boundary."]
pub struct PolledAudioEndpointConfig {
    #[doc = "Sets the queue capacity frames available to `PolledAudioEndpointConfig`."]
    pub queue_capacity_frames: usize,
    #[doc = "Stores the max batch frames used by `PolledAudioEndpointConfig`."]
    pub max_batch_frames: usize,
    #[doc = "Stores the max outstanding leases used by `PolledAudioEndpointConfig`."]
    pub max_outstanding_leases: usize,
}

impl Default for PolledAudioEndpointConfig {
    #[doc = "Returns the default `PolledAudioEndpointConfig` value."]
    fn default() -> Self {
        Self {
            queue_capacity_frames: 32,
            max_batch_frames: 8,
            max_outstanding_leases: 4,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
#[doc = "Classifies failures reported as polled audio endpoint config error."]
pub enum PolledAudioEndpointConfigError {
    #[error("polled-audio queue capacity must be greater than zero frames")]
    #[doc = "Reports zero queue capacity."]
    ZeroQueueCapacity,
    #[error("polled-audio batch capacity must be greater than zero frames")]
    #[doc = "Reports zero batch capacity."]
    ZeroBatchCapacity,
    #[error("polled-audio lease capacity must be greater than zero")]
    #[doc = "Reports zero lease capacity."]
    ZeroLeaseCapacity,
    #[error("polled-audio queue capacity exceeds {MAX_QUEUE_CAPACITY_FRAMES} frames")]
    #[doc = "Reports queue capacity too large."]
    QueueCapacityTooLarge,
    #[error("polled-audio batch capacity exceeds {MAX_BATCH_CAPACITY_FRAMES} frames")]
    #[doc = "Reports batch capacity too large."]
    BatchCapacityTooLarge,
    #[error("polled-audio lease capacity exceeds {MAX_OUTSTANDING_LEASES}")]
    #[doc = "Reports lease capacity too large."]
    LeaseCapacityTooLarge,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[doc = "Reports the polled audio observations collected at an observation boundary."]
pub struct PolledAudioObservations {
    #[doc = "Stores the registered endpoints used by `PolledAudioObservations`."]
    pub registered_endpoints: u64,
    #[doc = "Sets the queue capacity frames available to `PolledAudioObservations`."]
    pub queue_capacity_frames: u64,
    #[doc = "Reports the queue depth frames observed by `PolledAudioObservations`."]
    pub queue_depth_frames: u64,
    #[doc = "Reports the queue peak frames observed by `PolledAudioObservations`."]
    pub queue_peak_frames: u64,
    #[doc = "Counts the total number of queue depth invariant failures observed by `PolledAudioObservations`."]
    pub queue_depth_invariant_failures_total: u64,
    #[doc = "Counts the total number of frames received observed by `PolledAudioObservations`."]
    pub frames_received_total: u64,
    #[doc = "Counts the total number of frames delivered observed by `PolledAudioObservations`."]
    pub frames_delivered_total: u64,
    #[doc = "Counts the total number of queue full drops observed by `PolledAudioObservations`."]
    pub queue_full_drops_total: u64,
    #[doc = "Counts the total number of invalid ownership drops observed by `PolledAudioObservations`."]
    pub invalid_ownership_drops_total: u64,
    #[doc = "Sets the lease capacity count available to `PolledAudioObservations`."]
    pub lease_capacity_count: u64,
    #[doc = "Stores the outstanding leases used by `PolledAudioObservations`."]
    pub outstanding_leases: u64,
    #[doc = "Counts the total number of lease exhausted observed by `PolledAudioObservations`."]
    pub lease_exhausted_total: u64,
    #[doc = "Counts the total number of batches polled observed by `PolledAudioObservations`."]
    pub batches_polled_total: u64,
    #[doc = "Counts the total number of frames polled observed by `PolledAudioObservations`."]
    pub frames_polled_total: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
#[doc = "Classifies failures reported as polled audio poll error."]
pub enum PolledAudioPollError {
    #[error("polled-audio queue is empty")]
    #[doc = "Represents an empty value or collection."]
    Empty,
    #[error("polled-audio lease capacity is exhausted")]
    #[doc = "Reports lease capacity exhausted."]
    LeaseCapacityExhausted,
    #[error("polled-audio receipt state is poisoned")]
    #[doc = "Reports state poisoned."]
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
#[doc = "Retains the identity and observation access returned for polled audio."]
pub struct PolledAudioReceipt {
    shared: Arc<ReceiptShared>,
}

impl PolledAudioReceipt {
    #[doc = "Attempts to poll through `PolledAudioReceipt`."]
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
                let Ok(mut frame) = consumer.pop() else {
                    break;
                };
                frame.polled_at_ns = crate::timing::monotonic_timestamp_ns();
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

    #[doc = "Returns the observations exposed by `PolledAudioReceipt`."]
    pub fn observations(&self) -> PolledAudioObservations {
        self.shared.observations()
    }

    /// Waits for a batch until the finite deadline expires.
    ///
    /// The wait owns no additional audio queue. Producers wake this receipt
    /// after publishing into the existing bounded endpoint rings.
    pub fn wait_poll(
        &self,
        timeout: Duration,
    ) -> Result<Option<PolledAudioBatchLease>, PolledAudioPollError> {
        let deadline = Instant::now().checked_add(timeout);
        loop {
            let generation = self.shared.wake_generation.load(Ordering::Acquire);
            match self.try_poll() {
                Ok(batch) => return Ok(Some(batch)),
                Err(PolledAudioPollError::Empty) => {}
                Err(error) => return Err(error),
            }
            let Some(deadline) = deadline else {
                return Ok(None);
            };
            let now = Instant::now();
            if now >= deadline {
                return Ok(None);
            }
            let state = self
                .shared
                .state
                .lock()
                .map_err(|_| PolledAudioPollError::StatePoisoned)?;
            if self.shared.wake_generation.load(Ordering::Acquire) != generation
                || self.shared.queue_depth_frames.load(Ordering::Acquire) > 0
            {
                drop(state);
                continue;
            }
            let (_state, wait) = self
                .shared
                .available
                .wait_timeout(state, deadline.saturating_duration_since(now))
                .map_err(|_| PolledAudioPollError::StatePoisoned)?;
            if wait.timed_out() {
                return Ok(None);
            }
        }
    }
}

#[doc = "Owns bounded access to polled audio batch."]
pub struct PolledAudioBatchLease {
    shared: Arc<ReceiptShared>,
    frames: Option<Vec<DeliveredAudioFrame>>,
}

impl PolledAudioBatchLease {
    #[doc = "Returns the number of values held by `PolledAudioBatchLease`."]
    pub fn len(&self) -> usize {
        self.frames.as_ref().map_or(0, Vec::len)
    }

    #[doc = "Returns whether `PolledAudioBatchLease` contains no values."]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[doc = "Returns the frame held by `PolledAudioBatchLease`."]
    pub fn frame(&self, index: usize) -> Option<PolledAudioFrame<'_>> {
        self.frames
            .as_ref()?
            .get(index)
            .map(|delivered| PolledAudioFrame { delivered })
    }
}

impl Drop for PolledAudioBatchLease {
    #[doc = "Releases resources owned by `PolledAudioBatchLease`."]
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
#[doc = "Carries one polled audio payload together with its declared metadata."]
pub struct PolledAudioFrame<'lease> {
    delivered: &'lease DeliveredAudioFrame,
}

impl<'lease> PolledAudioFrame<'lease> {
    #[doc = "Returns the frame lineage carried by `PolledAudioFrame`."]
    pub fn lineage(self) -> FrameLineage {
        self.delivered.frame.lineage()
    }

    #[doc = "Returns the endpoint identifier held by `PolledAudioFrame`."]
    pub fn endpoint_id(self) -> EndpointId {
        self.delivered.endpoint_id
    }

    #[doc = "Returns the route identifier held by `PolledAudioFrame`."]
    pub fn route_id(self) -> RouteId {
        self.delivered.route_id
    }

    #[doc = "Returns the route enqueued at nanoseconds held by `PolledAudioFrame`."]
    pub fn route_enqueued_at_ns(self) -> u64 {
        self.delivered.route_enqueued_at_ns
    }

    #[doc = "Returns the route received at nanoseconds held by `PolledAudioFrame`."]
    pub fn route_received_at_ns(self) -> u64 {
        self.delivered.route_received_at_ns
    }

    #[doc = "Returns the endpoint enqueued at nanoseconds held by `PolledAudioFrame`."]
    pub fn endpoint_enqueued_at_ns(self) -> u64 {
        self.delivered.endpoint_enqueued_at_ns
    }

    #[doc = "Returns the polled at nanoseconds held by `PolledAudioFrame`."]
    pub fn polled_at_ns(self) -> u64 {
        self.delivered.polled_at_ns
    }

    #[doc = "Returns the stream identifier held by `PolledAudioFrame`."]
    pub fn stream_id(self) -> StreamId {
        self.delivered.frame.frame().stream_id()
    }

    #[doc = "Returns the connector identifier held by `PolledAudioFrame`."]
    pub fn connector_id(self) -> ConnectorId {
        self.delivered.connector_id
    }

    #[doc = "Returns the sample rate hertz held by `PolledAudioFrame`."]
    pub fn sample_rate_hz(self) -> u32 {
        self.delivered.frame.frame().sample_rate_hz
    }

    #[doc = "Returns the channel count represented by `PolledAudioFrame`."]
    pub fn channels(self) -> u8 {
        self.delivered.frame.frame().channels
    }

    #[doc = "Returns the audio samples held by `PolledAudioFrame`."]
    pub fn samples(self) -> &'lease [f32] {
        self.delivered.frame.frame().buffer.as_slice()
    }
}

#[derive(Debug)]
struct DeliveredAudioFrame {
    endpoint_id: EndpointId,
    connector_id: ConnectorId,
    route_id: RouteId,
    route_enqueued_at_ns: u64,
    route_received_at_ns: u64,
    endpoint_enqueued_at_ns: u64,
    polled_at_ns: u64,
    frame: LineagedAudioFrame,
}

struct ReceiptState {
    consumers: Vec<Option<Consumer<DeliveredAudioFrame>>>,
    recycled_batches: Vec<Vec<DeliveredAudioFrame>>,
    next_consumer: usize,
}

struct ReceiptShared {
    state: Mutex<ReceiptState>,
    available: Condvar,
    wake_generation: AtomicU64,
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
            available: Condvar::new(),
            wake_generation: AtomicU64::new(0),
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
        self.wake_generation.fetch_add(1, Ordering::Release);
        self.available.notify_all();
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
        self.wake_generation.fetch_add(1, Ordering::Release);
        self.available.notify_one();
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
        mut inputs: Vec<EndpointPortInput>,
    ) -> Result<Box<dyn PreparedEndpointDriver>, EndpointFailure> {
        if inputs.len() != 1 {
            return Err(prepare_failure(
                "one polled-audio endpoint requires exactly one input",
            ));
        }
        let Some(input) = inputs.pop() else {
            return Err(prepare_failure(
                "one polled-audio endpoint requires exactly one input",
            ));
        };
        let (receiver, context) = input.into_parts();
        let EndpointReceiver::Audio { receiver, .. } = receiver else {
            return Err(prepare_failure(
                "polled-audio endpoint accepts only realtime audio inputs",
            ));
        };
        let connector_id = context.connector_id().ok_or_else(|| {
            prepare_failure("polled-audio endpoint is missing its typed connector binding")
        })?;
        let route_id = context.route_context().route_id();
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
    receiver: EndpointAudioReceiver,
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
        let join_result = match self.worker.take() {
            Some(worker) => worker.join().map_err(|_| {
                EndpointFailure::new(
                    EndpointFailureStage::JoinFinalize,
                    "polled-audio worker panicked",
                )
            }),
            None => Err(EndpointFailure::new(
                EndpointFailureStage::JoinFinalize,
                "polled-audio worker ownership was already consumed",
            )),
        };
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
    receiver: &mut EndpointAudioReceiver,
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
    frame: crate::endpoint::EndpointAudioFrame,
    endpoint_id: EndpointId,
    connector_id: ConnectorId,
    route_id: RouteId,
    shared: &ReceiptShared,
    observations: &WorkerObservations,
) -> Option<DeliveredAudioFrame> {
    let route_enqueued_at_ns = frame.route_enqueued_at_ns();
    let route_received_at_ns = frame.route_received_at_ns();
    let PlanEdgeFrame::Exclusive(frame) = frame.into_inner() else {
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
        route_enqueued_at_ns,
        route_received_at_ns,
        endpoint_enqueued_at_ns: crate::timing::monotonic_timestamp_ns(),
        polled_at_ns: 0,
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
mod tests;
