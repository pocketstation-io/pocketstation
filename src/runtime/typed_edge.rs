use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

use rtrb::{Consumer, Producer, RingBuffer};

use crate::graph::{EdgeContract, LossPolicy, SignalEnvelope, SignalEnvelopeError};

#[derive(Debug, Clone, Copy)]
pub struct TypedEdgeBranchSpec {
    pub capacity_signals: usize,
    pub edge_contract: EdgeContract,
}

#[derive(Default)]
struct TypedEdgeObservationState {
    delivered_total: AtomicU64,
    dropped_total: AtomicU64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TypedEdgeObservations {
    pub delivered_total: u64,
    pub dropped_total: u64,
}

#[derive(Clone)]
pub struct TypedEdgeObservationHandle {
    state: Arc<TypedEdgeObservationState>,
}

impl TypedEdgeObservationHandle {
    pub fn snapshot(&self) -> TypedEdgeObservations {
        TypedEdgeObservations {
            delivered_total: self.state.delivered_total.load(Ordering::Relaxed),
            dropped_total: self.state.dropped_total.load(Ordering::Relaxed),
        }
    }
}

pub struct TypedEdgeReceiver {
    receiver: Consumer<Arc<SignalEnvelope>>,
    observations: TypedEdgeObservationHandle,
}

impl TypedEdgeReceiver {
    pub fn recv(&mut self) -> Option<Arc<SignalEnvelope>> {
        self.receiver.pop().ok()
    }

    pub fn observations(&self) -> TypedEdgeObservations {
        self.observations.snapshot()
    }

    pub fn observation_handle(&self) -> TypedEdgeObservationHandle {
        self.observations.clone()
    }

    pub fn is_abandoned(&self) -> bool {
        self.receiver.is_abandoned()
    }
}

struct TypedEdgeBranchSender {
    sender: Producer<Arc<SignalEnvelope>>,
    observations: Arc<TypedEdgeObservationState>,
    loss: LossPolicy,
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
            let (sender, receiver) = RingBuffer::new(specification.capacity_signals);
            let state = Arc::new(TypedEdgeObservationState::default());
            branches.push(TypedEdgeBranchSender {
                sender,
                observations: Arc::clone(&state),
                loss: specification.edge_contract.loss,
            });
            receivers.push(TypedEdgeReceiver {
                receiver,
                observations: TypedEdgeObservationHandle { state },
            });
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
        if terminal {
            if let Some(branch_index) = self.branches.iter().position(|branch| {
                branch.loss == LossPolicy::MustDeliverOrFail && branch.sender.is_full()
            }) {
                self.branches[branch_index]
                    .observations
                    .dropped_total
                    .fetch_add(1, Ordering::Relaxed);
                return Err(TypedEdgePublishError::RequiredBranchFull { branch_index });
            }
        }
        let shared = Arc::new(envelope);
        let mut report = TypedEdgePublishReport::default();
        for (branch_index, branch) in self.branches.iter_mut().enumerate() {
            match branch.sender.push(Arc::clone(&shared)) {
                Ok(()) => {
                    branch
                        .observations
                        .delivered_total
                        .fetch_add(1, Ordering::Relaxed);
                    report.delivered_total = report.delivered_total.saturating_add(1);
                }
                Err(rtrb::PushError::Full(_)) => {
                    branch
                        .observations
                        .dropped_total
                        .fetch_add(1, Ordering::Relaxed);
                    report.dropped_total = report.dropped_total.saturating_add(1);
                    if terminal && branch.loss == LossPolicy::MustDeliverOrFail {
                        return Err(TypedEdgePublishError::RequiredBranchFull { branch_index });
                    }
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
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum TypedEdgePublishError {
    #[error("typed edge rejected an invalid signal envelope: {0}")]
    InvalidEnvelope(SignalEnvelopeError),
    #[error("required typed edge branch {branch_index} is full")]
    RequiredBranchFull { branch_index: usize },
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frame::{ClockDomainId, SessionId, SourceId, StreamId};
    use crate::graph::{EventFormat, SignalLineage, SignalPayload, SignalSpec, SignalTiming};

    fn envelope(payload: SignalPayload, sequence: u64) -> SignalEnvelope {
        SignalEnvelope::untracked(payload, sequence).with_lineage(
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

    #[test]
    fn given_independent_branches_when_one_saturates_then_other_continues() {
        let contract = EdgeContract::typed_default();
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
            envelope(SignalPayload::Event(vec![sequence as u8]), sequence).map_payload(
                SignalPayload::Event(vec![sequence as u8]),
                SignalSpec::event(EventFormat::Json),
            )
        };
        fanout.publish(event_envelope(1), false).unwrap();
        let report = fanout.publish(event_envelope(2), false).unwrap();
        assert_eq!(report.delivered_total, 1);
        assert_eq!(report.dropped_total, 1);
        assert_eq!(receivers[0].observations().dropped_total, 1);
        assert_eq!(receivers[1].recv().unwrap().sequence_number(), Some(1));
        assert_eq!(receivers[1].recv().unwrap().sequence_number(), Some(2));
    }

    #[test]
    fn given_required_terminal_when_branch_is_full_then_publish_fails_closed() {
        let (mut fanout, _receivers) = TypedEdgeFanout::new(&[TypedEdgeBranchSpec {
            capacity_signals: 1,
            edge_contract: EdgeContract::typed_default(),
        }])
        .unwrap();
        let control_envelope = |sequence| envelope(SignalPayload::Control(Vec::new()), sequence);
        fanout.publish(control_envelope(1), false).unwrap();
        assert_eq!(
            fanout.publish(control_envelope(2), true),
            Err(TypedEdgePublishError::RequiredBranchFull { branch_index: 0 })
        );
    }
}
