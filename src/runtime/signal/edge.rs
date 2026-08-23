use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, OnceLock};
use std::thread::Thread;

use rtrb::{Consumer, Producer, RingBuffer};

#[cfg(any(test, feature = "internal-testing"))]
use crate::frame::AudioFrame;
#[cfg(any(test, feature = "internal-testing"))]
use crate::graph::SignalPayload;
use crate::graph::{EdgeContract, LossPolicy, SignalEnvelope, SignalEnvelopeError};

/// Keeps per-branch envelope ownership in the same finite operating range as
/// the realtime frame pools. Payload bytes have their own independent limit.
pub const MAX_TYPED_EDGE_CAPACITY_SIGNALS: usize = 64;

#[derive(Default)]
struct SignalEdgeObservationState {
    capacity_signals: u64,
    max_payload_bytes: u64,
    maximum_buffered_payload_bytes: u64,
    depth_signals: AtomicU64,
    peak_depth_signals: AtomicU64,
    enqueued_total: AtomicU64,
    received_total: AtomicU64,
    delivered_total: AtomicU64,
    dropped_total: AtomicU64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "Reports the signal edge observations collected at an observation boundary."]
pub struct SignalEdgeObservations {
    #[doc = "Sets the capacity signals available to `SignalEdgeObservations`."]
    pub capacity_signals: u64,
    #[doc = "Limits payload storage for `SignalEdgeObservations`, in bytes."]
    pub max_payload_bytes: u64,
    #[doc = "Stores the maximum buffered payload size for `SignalEdgeObservations`, in bytes."]
    pub maximum_buffered_payload_bytes: u64,
    #[doc = "Reports the depth signals observed by `SignalEdgeObservations`."]
    pub depth_signals: u64,
    #[doc = "Reports the peak depth signals observed by `SignalEdgeObservations`."]
    pub peak_depth_signals: u64,
    #[doc = "Counts the total number of enqueued observed by `SignalEdgeObservations`."]
    pub enqueued_total: u64,
    #[doc = "Counts the total number of received observed by `SignalEdgeObservations`."]
    pub received_total: u64,
    /// Compatibility alias for `enqueued_total`.
    ///
    /// A queue insertion is delivery to the branch, not consumption by its
    /// receiver. New accounting should use `enqueued_total` and
    /// `received_total` explicitly.
    pub delivered_total: u64,
    #[doc = "Counts the total number of dropped observed by `SignalEdgeObservations`."]
    pub dropped_total: u64,
}

#[derive(Clone)]
pub struct SignalEdgeObservationHandle {
    state: Arc<SignalEdgeObservationState>,
}

impl SignalEdgeObservationHandle {
    pub fn snapshot(&self) -> SignalEdgeObservations {
        SignalEdgeObservations {
            capacity_signals: self.state.capacity_signals,
            max_payload_bytes: self.state.max_payload_bytes,
            maximum_buffered_payload_bytes: self.state.maximum_buffered_payload_bytes,
            depth_signals: self.state.depth_signals.load(Ordering::Relaxed),
            peak_depth_signals: self.state.peak_depth_signals.load(Ordering::Relaxed),
            enqueued_total: self.state.enqueued_total.load(Ordering::Relaxed),
            received_total: self.state.received_total.load(Ordering::Relaxed),
            delivered_total: self.state.delivered_total.load(Ordering::Relaxed),
            dropped_total: self.state.dropped_total.load(Ordering::Relaxed),
        }
    }
}

/// The single bounded SPSC authority for asynchronous signals.
///
/// The item type expresses ownership only: callback bridges carry an owned
/// `SignalEnvelope`, while fan-out branches carry `Arc<SignalEnvelope>`. Both
/// modes share the exact same capacity, saturation, and observation logic.
pub struct SignalEdge;

impl SignalEdge {
    pub(crate) fn bounded<Item>(
        capacity_signals: usize,
    ) -> (SignalEdgeSender<Item>, SignalEdgeReceiver<Item>) {
        Self::bounded_with_payload_limit(capacity_signals, 0)
    }

    fn bounded_with_payload_limit<Item>(
        capacity_signals: usize,
        max_payload_bytes: usize,
    ) -> (SignalEdgeSender<Item>, SignalEdgeReceiver<Item>) {
        let (producer, consumer) = RingBuffer::new(capacity_signals);
        let state = Arc::new(SignalEdgeObservationState {
            capacity_signals: capacity_signals as u64,
            max_payload_bytes: max_payload_bytes as u64,
            maximum_buffered_payload_bytes: capacity_signals.saturating_mul(max_payload_bytes)
                as u64,
            ..SignalEdgeObservationState::default()
        });
        let receiver_thread = Arc::new(OnceLock::new());
        (
            SignalEdgeSender {
                producer,
                state: Arc::clone(&state),
                receiver_thread: Arc::clone(&receiver_thread),
            },
            SignalEdgeReceiver {
                consumer,
                state,
                receiver_thread,
            },
        )
    }
}

pub struct SignalEdgeSender<Item> {
    producer: Producer<Item>,
    state: Arc<SignalEdgeObservationState>,
    receiver_thread: Arc<OnceLock<Thread>>,
}

#[derive(Debug)]
pub struct SignalEdgeSendError<Item> {
    rejected: Item,
}

impl<Item> SignalEdgeSendError<Item> {
    pub fn into_rejected(self) -> Item {
        self.rejected
    }
}

impl<Item> SignalEdgeSender<Item> {
    pub fn try_send(&mut self, item: Item) -> Result<(), SignalEdgeSendError<Item>> {
        let depth = self
            .state
            .depth_signals
            .fetch_add(1, Ordering::Relaxed)
            .saturating_add(1);
        match self.producer.push(item) {
            Ok(()) => {
                self.state.enqueued_total.fetch_add(1, Ordering::Relaxed);
                self.state.delivered_total.fetch_add(1, Ordering::Relaxed);
                self.state
                    .peak_depth_signals
                    .fetch_max(depth, Ordering::Relaxed);
                if let Some(receiver_thread) = self.receiver_thread.get() {
                    receiver_thread.unpark();
                }
                Ok(())
            }
            Err(rtrb::PushError::Full(item)) => {
                self.state.depth_signals.fetch_sub(1, Ordering::Relaxed);
                self.state.dropped_total.fetch_add(1, Ordering::Relaxed);
                Err(SignalEdgeSendError { rejected: item })
            }
        }
    }

    pub fn is_full(&self) -> bool {
        self.producer.is_full()
    }

    pub(crate) fn is_abandoned(&self) -> bool {
        self.producer.is_abandoned()
    }

    #[cfg(test)]
    pub fn dropped_count(&self) -> u64 {
        self.state.dropped_total.load(Ordering::Relaxed)
    }
}

impl SignalEdgeSender<SignalEnvelope> {
    #[cfg(any(test, feature = "internal-testing"))]
    #[expect(
        clippy::result_large_err,
        reason = "full queues return the rejected envelope inline so saturation never allocates"
    )]
    pub fn send(
        &mut self,
        envelope: SignalEnvelope,
    ) -> Result<(), SignalEdgeSendError<SignalEnvelope>> {
        self.try_send(envelope)
    }

    #[cfg(any(test, feature = "internal-testing"))]
    pub fn send_audio(
        &mut self,
        mut frame: AudioFrame,
        sequence_number: u64,
        timestamp_ns: u64,
    ) -> Result<(), AudioFrame> {
        frame.sequence_number = sequence_number;
        frame.timestamp_ns = timestamp_ns;
        let envelope = SignalEnvelope::from_audio(frame, None);
        match self.try_send(envelope) {
            Ok(()) => Ok(()),
            Err(error) => {
                let SignalPayload::Audio(frame) = error.into_rejected().payload else {
                    return Ok(());
                };
                Err(frame)
            }
        }
    }
}

pub struct SignalEdgeReceiver<Item> {
    consumer: Consumer<Item>,
    state: Arc<SignalEdgeObservationState>,
    receiver_thread: Arc<OnceLock<Thread>>,
}

impl<Item> SignalEdgeReceiver<Item> {
    /// Registers the current non-realtime consumer thread for producer wakeups.
    ///
    /// Registration is one-shot and allocation-free after setup. Producers
    /// that feed realtime callbacks must not register a thread here; typed
    /// signal workers and generated-audio bridges execute off callback threads.
    pub(crate) fn register_current_thread(&self) -> bool {
        self.receiver_thread.set(std::thread::current()).is_ok()
    }

    pub fn recv(&mut self) -> Option<Item> {
        let item = self.consumer.pop().ok()?;
        self.state.depth_signals.fetch_sub(1, Ordering::Relaxed);
        self.state.received_total.fetch_add(1, Ordering::Relaxed);
        Some(item)
    }

    #[cfg(any(test, feature = "internal-testing"))]
    pub fn observations(&self) -> SignalEdgeObservations {
        self.observation_handle().snapshot()
    }

    pub fn observation_handle(&self) -> SignalEdgeObservationHandle {
        SignalEdgeObservationHandle {
            state: Arc::clone(&self.state),
        }
    }

    pub fn is_abandoned(&self) -> bool {
        self.consumer.is_abandoned()
    }
}

pub type TypedEdgeReceiver = SignalEdgeReceiver<Arc<SignalEnvelope>>;
pub type TypedEdgeObservations = SignalEdgeObservations;
pub type TypedEdgeObservationHandle = SignalEdgeObservationHandle;

#[derive(Debug, Clone, Copy)]
pub struct TypedEdgeBranchSpec {
    pub capacity_signals: usize,
    pub edge_contract: EdgeContract,
}

struct TypedEdgeBranchSender {
    sender: SignalEdgeSender<Arc<SignalEnvelope>>,
    loss: LossPolicy,
    max_payload_bytes: usize,
}

pub struct TypedEdgeFanout {
    branches: Vec<TypedEdgeBranchSender>,
}

impl TypedEdgeFanout {
    pub fn new(
        branch_specs: &[TypedEdgeBranchSpec],
    ) -> Result<(Self, Vec<TypedEdgeReceiver>), TypedEdgeBuildError> {
        if branch_specs.is_empty() {
            return Err(TypedEdgeBuildError::NoBranches);
        }
        let mut branches = Vec::with_capacity(branch_specs.len());
        let mut receivers = Vec::with_capacity(branch_specs.len());
        for specification in branch_specs {
            if specification.capacity_signals == 0 {
                return Err(TypedEdgeBuildError::ZeroCapacity);
            }
            if specification.capacity_signals > MAX_TYPED_EDGE_CAPACITY_SIGNALS {
                return Err(TypedEdgeBuildError::CapacityTooLarge {
                    capacity_signals: specification.capacity_signals,
                    maximum: MAX_TYPED_EDGE_CAPACITY_SIGNALS,
                });
            }
            let Some(max_payload_bytes) = specification.edge_contract.max_payload_bytes() else {
                return Err(TypedEdgeBuildError::MissingPayloadLimit);
            };
            if max_payload_bytes == 0 {
                return Err(TypedEdgeBuildError::ZeroPayloadLimit);
            }
            if max_payload_bytes > crate::graph::MAX_ASYNC_PAYLOAD_BYTES {
                return Err(TypedEdgeBuildError::PayloadLimitTooLarge {
                    max_payload_bytes,
                    maximum: crate::graph::MAX_ASYNC_PAYLOAD_BYTES,
                });
            }
            let (sender, receiver) = SignalEdge::bounded_with_payload_limit(
                specification.capacity_signals,
                max_payload_bytes,
            );
            branches.push(TypedEdgeBranchSender {
                sender,
                loss: specification.edge_contract.loss,
                max_payload_bytes,
            });
            receivers.push(receiver);
        }
        Ok((Self { branches }, receivers))
    }

    pub fn publish(
        &mut self,
        envelope: SignalEnvelope,
        terminal: bool,
    ) -> Result<TypedEdgePublishReport, TypedEdgePublishError> {
        envelope
            .validate()
            .map_err(TypedEdgePublishError::InvalidEnvelope)?;
        let payload_bytes = envelope.payload_size_bytes();
        if let Some((branch_index, branch)) = self
            .branches
            .iter()
            .enumerate()
            .find(|(_, branch)| payload_bytes > branch.max_payload_bytes)
        {
            return Err(TypedEdgePublishError::PayloadTooLarge {
                branch_index,
                payload_bytes,
                max_payload_bytes: branch.max_payload_bytes,
            });
        }
        if terminal {
            if let Some(branch_index) = self.branches.iter().position(|branch| {
                branch.loss == LossPolicy::MustDeliverOrFail && branch.sender.is_full()
            }) {
                self.branches[branch_index]
                    .sender
                    .state
                    .dropped_total
                    .fetch_add(1, Ordering::Relaxed);
                return Err(TypedEdgePublishError::RequiredBranchFull { branch_index });
            }
        }
        let shared = Arc::new(envelope);
        let mut report = TypedEdgePublishReport::default();
        let Some((final_branch, preceding_branches)) = self.branches.split_last_mut() else {
            return Err(TypedEdgePublishError::NoBranches);
        };
        for (branch_index, branch) in preceding_branches.iter_mut().enumerate() {
            match branch.sender.try_send(Arc::clone(&shared)) {
                Ok(()) => {
                    report.delivered_total = report.delivered_total.saturating_add(1);
                }
                Err(error) => {
                    drop(error.into_rejected());
                    report.dropped_total = report.dropped_total.saturating_add(1);
                    if terminal && branch.loss == LossPolicy::MustDeliverOrFail {
                        return Err(TypedEdgePublishError::RequiredBranchFull { branch_index });
                    }
                }
            }
        }
        let final_branch_index = preceding_branches.len();
        match final_branch.sender.try_send(shared) {
            Ok(()) => {
                report.delivered_total = report.delivered_total.saturating_add(1);
            }
            Err(error) => {
                drop(error.into_rejected());
                report.dropped_total = report.dropped_total.saturating_add(1);
                if terminal && final_branch.loss == LossPolicy::MustDeliverOrFail {
                    return Err(TypedEdgePublishError::RequiredBranchFull {
                        branch_index: final_branch_index,
                    });
                }
            }
        }
        Ok(report)
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct TypedEdgePublishReport {
    pub delivered_total: u64,
    pub dropped_total: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum TypedEdgeBuildError {
    #[error("typed edge fanout requires at least one branch")]
    NoBranches,
    #[error("typed edge branch capacity must be greater than zero")]
    ZeroCapacity,
    #[error("typed edge branch capacity {capacity_signals} exceeds maximum {maximum}")]
    CapacityTooLarge {
        capacity_signals: usize,
        maximum: usize,
    },
    #[error("typed edge branch requires an explicit maximum payload size")]
    MissingPayloadLimit,
    #[error("typed edge branch maximum payload size must be greater than zero")]
    ZeroPayloadLimit,
    #[error("typed edge branch payload limit {max_payload_bytes} exceeds maximum {maximum}")]
    PayloadLimitTooLarge {
        max_payload_bytes: usize,
        maximum: usize,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum TypedEdgePublishError {
    #[error("typed edge fanout has no branches")]
    NoBranches,
    #[error("typed edge rejected an invalid signal envelope: {0}")]
    InvalidEnvelope(SignalEnvelopeError),
    #[error(
        "typed edge branch {branch_index} rejected {payload_bytes} payload bytes; maximum is {max_payload_bytes}"
    )]
    PayloadTooLarge {
        branch_index: usize,
        payload_bytes: usize,
        max_payload_bytes: usize,
    },
    #[error("required typed edge branch {branch_index} is full")]
    RequiredBranchFull { branch_index: usize },
}

#[cfg(test)]
mod tests {
    use std::sync::mpsc;
    use std::time::Duration;

    use super::*;
    use crate::frame::{AudioBufferPool, ClockDomainId, SessionId, SourceId, StreamId};
    use crate::graph::{EventFormat, SignalLineage, SignalSpec, SignalTiming};

    fn envelope(payload: SignalPayload, spec: SignalSpec, sequence: u64) -> SignalEnvelope {
        SignalEnvelope::untracked(payload, spec, sequence).with_lineage(
            SignalLineage {
                session_id: SessionId(1),
                stream_id: StreamId(2),
                source_id: SourceId(3),
                clock_id: ClockDomainId(4),
                sequence_number: sequence,
                source_generation: 1,
                discontinuity_epoch: 0,
                policy_epoch: 1,
            },
            SignalTiming {
                source_timestamp_ns: Some(sequence),
                observed_timestamp_ns: sequence,
                session_timestamp_ns: Some(sequence),
                duration_ns: None,
            },
        )
    }

    fn frame_with_samples(samples: &[f32]) -> AudioFrame {
        let pool = AudioBufferPool::new(1, samples.len());
        let mut handle = pool.acquire().unwrap();
        handle.try_copy_from_slice(samples).unwrap();
        AudioFrame::new(StreamId(0), SourceId(0), 3, 5, 1, handle)
    }

    #[test]
    fn given_full_owned_signal_edge_when_audio_sent_then_frame_returns_without_allocation() {
        let (mut sender, _receiver) = SignalEdge::bounded(1);
        sender.send_audio(frame_with_samples(&[1.0]), 0, 0).unwrap();
        let rejected = sender.send_audio(frame_with_samples(&[2.0]), 1, 1);
        assert_eq!(rejected.unwrap_err().buffer.as_slice(), &[2.0]);
        assert_eq!(sender.dropped_count(), 1);
    }

    #[test]
    fn given_registered_signal_consumer_when_item_enqueued_then_parked_thread_is_woken() {
        let (ready_tx, ready_rx) = mpsc::sync_channel(1);
        let (received_tx, received_rx) = mpsc::sync_channel(1);
        let (mut sender, mut receiver) = SignalEdge::bounded(1);
        let consumer = std::thread::spawn(move || {
            assert!(receiver.register_current_thread());
            ready_tx.send(()).unwrap();
            loop {
                if let Some(value) = receiver.recv() {
                    received_tx.send(value).unwrap();
                    return;
                }
                std::thread::park();
            }
        });
        ready_rx.recv_timeout(Duration::from_secs(1)).unwrap();

        sender.try_send(7_u64).unwrap();

        assert_eq!(received_rx.recv_timeout(Duration::from_secs(1)).unwrap(), 7);
        consumer.join().unwrap();
    }

    #[test]
    fn given_independent_shared_branches_when_one_saturates_then_other_continues() {
        let contract = EdgeContract::bounded_async();
        let (mut fanout, mut receivers) = TypedEdgeFanout::new(&[
            TypedEdgeBranchSpec {
                capacity_signals: 1,
                edge_contract: contract,
            },
            TypedEdgeBranchSpec {
                capacity_signals: 2,
                edge_contract: contract,
            },
        ])
        .unwrap();
        let event_envelope = |sequence| {
            envelope(
                SignalPayload::Bytes(vec![sequence as u8]),
                SignalSpec::event(EventFormat::Json),
                sequence,
            )
        };
        fanout.publish(event_envelope(1), false).unwrap();
        let report = fanout.publish(event_envelope(2), false).unwrap();
        assert_eq!(report.delivered_total, 1);
        assert_eq!(report.dropped_total, 1);
        assert_eq!(receivers[0].observations().dropped_total, 1);
        assert_eq!(receivers[0].observations().capacity_signals, 1);
        assert_eq!(
            receivers[0].observations().max_payload_bytes,
            crate::graph::DEFAULT_ASYNC_MAX_PAYLOAD_BYTES as u64
        );
        assert_eq!(
            receivers[1].observations().maximum_buffered_payload_bytes,
            (2 * crate::graph::DEFAULT_ASYNC_MAX_PAYLOAD_BYTES) as u64
        );
        assert_eq!(receivers[0].observations().depth_signals, 1);
        assert_eq!(receivers[1].observations().peak_depth_signals, 2);
        assert_eq!(receivers[1].recv().unwrap().sequence_number(), Some(1));
        assert_eq!(receivers[1].recv().unwrap().sequence_number(), Some(2));
        assert_eq!(receivers[1].observations().depth_signals, 0);
        assert_eq!(receivers[1].observations().received_total, 2);
    }

    #[test]
    fn given_payload_above_branch_limit_when_published_then_all_branches_reject_before_fanout() {
        let contract = EdgeContract::bounded_async().with_max_payload_bytes(4);
        let (mut fanout, receivers) = TypedEdgeFanout::new(&[
            TypedEdgeBranchSpec {
                capacity_signals: 1,
                edge_contract: contract,
            },
            TypedEdgeBranchSpec {
                capacity_signals: 2,
                edge_contract: contract,
            },
        ])
        .unwrap();

        let error = fanout
            .publish(
                envelope(
                    SignalPayload::Bytes(vec![0; 5]),
                    SignalSpec::event(EventFormat::Json),
                    1,
                ),
                false,
            )
            .unwrap_err();

        assert_eq!(
            error,
            TypedEdgePublishError::PayloadTooLarge {
                branch_index: 0,
                payload_bytes: 5,
                max_payload_bytes: 4,
            }
        );
        assert!(receivers
            .iter()
            .all(|receiver| receiver.observations().depth_signals == 0));
    }

    #[test]
    fn given_capacity_above_global_bound_when_fanout_built_then_setup_fails() {
        let result = TypedEdgeFanout::new(&[TypedEdgeBranchSpec {
            capacity_signals: MAX_TYPED_EDGE_CAPACITY_SIGNALS + 1,
            edge_contract: EdgeContract::bounded_async(),
        }]);

        assert!(matches!(
            result,
            Err(TypedEdgeBuildError::CapacityTooLarge {
                capacity_signals,
                maximum: MAX_TYPED_EDGE_CAPACITY_SIGNALS,
            }) if capacity_signals == MAX_TYPED_EDGE_CAPACITY_SIGNALS + 1
        ));
    }

    #[test]
    fn given_missing_or_zero_payload_limit_when_fanout_built_then_setup_fails() {
        let missing = TypedEdgeFanout::new(&[TypedEdgeBranchSpec {
            capacity_signals: 1,
            edge_contract: EdgeContract::realtime_audio(),
        }]);
        let zero = TypedEdgeFanout::new(&[TypedEdgeBranchSpec {
            capacity_signals: 1,
            edge_contract: EdgeContract::bounded_async().with_max_payload_bytes(0),
        }]);

        assert!(matches!(
            missing,
            Err(TypedEdgeBuildError::MissingPayloadLimit)
        ));
        assert!(matches!(zero, Err(TypedEdgeBuildError::ZeroPayloadLimit)));
    }

    #[test]
    fn given_payload_limit_above_global_bound_when_fanout_built_then_setup_fails() {
        let result = TypedEdgeFanout::new(&[TypedEdgeBranchSpec {
            capacity_signals: 1,
            edge_contract: EdgeContract::bounded_async()
                .with_max_payload_bytes(crate::graph::MAX_ASYNC_PAYLOAD_BYTES + 1),
        }]);

        assert!(matches!(
            result,
            Err(TypedEdgeBuildError::PayloadLimitTooLarge {
                max_payload_bytes,
                maximum: crate::graph::MAX_ASYNC_PAYLOAD_BYTES,
            }) if max_payload_bytes == crate::graph::MAX_ASYNC_PAYLOAD_BYTES + 1
        ));
    }

    #[test]
    fn given_one_branch_when_signal_published_then_receiver_has_exclusive_ownership() {
        let (mut fanout, mut receivers) = TypedEdgeFanout::new(&[TypedEdgeBranchSpec {
            capacity_signals: 1,
            edge_contract: EdgeContract::bounded_async(),
        }])
        .unwrap();

        fanout
            .publish(
                envelope(
                    SignalPayload::Bytes(vec![1]),
                    SignalSpec::event(EventFormat::Json),
                    1,
                ),
                false,
            )
            .unwrap();

        let received = receivers[0].recv().unwrap();
        assert_eq!(Arc::strong_count(&received), 1);
    }
}
