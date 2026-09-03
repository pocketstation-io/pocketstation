//! Operator placement and execution safety.
//!
//! These are separate types because they answer different questions:
//!   `ExecutionPartition` — where does this operator run?
//!   `ExecutionSafety`    — what may this operator do while it runs?
//!
//! The compiler enforces: `ExecutionSafety::RealtimeSafe` is only valid on
//! `AudioCallback` or `RealtimeCpu` partitions.  Any edge that crosses partition
//! boundaries gets a compiler-inserted `Bridge`; no cross-partition calls on the
//! hot path.

/// WHERE an operator runs.
///
/// Determines which executor thread pool, scheduling domain, and OS resource
/// context the operator is placed in.  Two operators can only share a ring-buffer
/// edge without a Bridge if their partitions are compatible.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ExecutionPartition {
    /// Platform OS audio callback — the strictest domain.
    ///
    /// The OS calls this on a high-priority real-time thread.  Must be
    /// alloc-free, lock-free, blocking-free, and log-free (LAW 15).
    /// Only `ExecutionSafety::RealtimeSafe` operators may live here.
    AudioCallback,

    /// Dedicated real-time processing thread.
    ///
    /// Not the OS audio callback, but still real-time-safe: no alloc, no locks,
    /// no blocking.  DSP transforms, mixers, encoders, VAD.
    RealtimeCpu,

    /// Tokio async task.
    ///
    /// May allocate, await, and perform I/O.  Never blocks the OS thread.
    /// Network sockets, relay transport, async codec pipelines.
    AsyncWorker,

    /// `spawn_blocking` thread.
    ///
    /// Allowed to block the OS thread.  Disk writes, database queries,
    /// heavy CPU work that cannot be made async cheaply.
    BlockingWorker,

    /// Remote service — always async, always network-required.
    ///
    /// The operator implementation lives outside this process (cloud API,
    /// sidecar, remote GPU).  Latency is unbounded; always use a bounded queue.
    External,
}

impl ExecutionPartition {
    /// Returns `true` if the partition requires strict real-time safety.
    ///
    /// Operators in real-time partitions must use `ExecutionSafety::RealtimeSafe`.
    pub fn requires_realtime_safety(self) -> bool {
        matches!(self, Self::AudioCallback | Self::RealtimeCpu)
    }

    /// Priority rank for scheduling: lower = higher priority.
    pub fn rank(self) -> u8 {
        match self {
            Self::AudioCallback => 0,
            Self::RealtimeCpu => 1,
            Self::AsyncWorker => 2,
            Self::BlockingWorker => 3,
            Self::External => 4,
        }
    }

    /// Returns `true` if crossing from `self` to `other` requires a compiler-inserted Bridge.
    pub fn needs_bridge_to(self, other: ExecutionPartition) -> bool {
        self != other
    }
}

/// States what an Operator may do while it runs.
///
/// Separate from `ExecutionPartition` — an operator can declare its safety
/// requirements independently of where it wants to run. The compiler validates
/// the combination before the Session starts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ExecutionSafety {
    /// No heap allocation, no locking, no blocking, no logging.
    ///
    /// The only valid setting for `AudioCallback` and `RealtimeCpu` partitions.
    /// Verified by CI via the alloc-check integration test.
    RealtimeSafe,

    /// May heap-allocate but must not block or make network calls.
    AllocationAllowed,

    /// May block the current OS thread.
    BlockingAllowed,

    /// May make network calls (implies async + allocation allowed).
    NetworkAllowed,

    /// Backed by a remote service; all calls are async network operations.
    ExternalService,
}

impl ExecutionSafety {
    /// Returns `true` if this behavior is safe on the given partition.
    ///
    /// The compiler calls this during graph validation to reject unsafe combinations
    /// before any code runs.
    pub fn is_valid_for(self, partition: ExecutionPartition) -> bool {
        match (partition, self) {
            (ExecutionPartition::AudioCallback, ExecutionSafety::RealtimeSafe) => true,
            (ExecutionPartition::AudioCallback, _) => false,
            (ExecutionPartition::RealtimeCpu, ExecutionSafety::RealtimeSafe) => true,
            (ExecutionPartition::RealtimeCpu, _) => false,
            (ExecutionPartition::AsyncWorker, ExecutionSafety::NetworkAllowed) => true,
            (ExecutionPartition::AsyncWorker, ExecutionSafety::AllocationAllowed) => true,
            (ExecutionPartition::AsyncWorker, ExecutionSafety::ExternalService) => true,
            (ExecutionPartition::AsyncWorker, _) => false,
            (ExecutionPartition::BlockingWorker, ExecutionSafety::BlockingAllowed) => true,
            (ExecutionPartition::BlockingWorker, ExecutionSafety::AllocationAllowed) => true,
            (ExecutionPartition::BlockingWorker, _) => false,
            (ExecutionPartition::External, ExecutionSafety::ExternalService) => true,
            (ExecutionPartition::External, _) => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_audio_callback_partition_when_requires_realtime_safety_then_true() {
        assert!(ExecutionPartition::AudioCallback.requires_realtime_safety());
        assert!(ExecutionPartition::RealtimeCpu.requires_realtime_safety());
    }

    #[test]
    fn given_non_realtime_partitions_when_requires_realtime_safety_then_false() {
        assert!(!ExecutionPartition::AsyncWorker.requires_realtime_safety());
        assert!(!ExecutionPartition::BlockingWorker.requires_realtime_safety());
        assert!(!ExecutionPartition::External.requires_realtime_safety());
    }

    #[test]
    fn given_partitions_when_ranked_then_audio_callback_is_lowest() {
        assert_eq!(ExecutionPartition::AudioCallback.rank(), 0);
        assert!(ExecutionPartition::AudioCallback.rank() < ExecutionPartition::RealtimeCpu.rank());
        assert!(ExecutionPartition::RealtimeCpu.rank() < ExecutionPartition::AsyncWorker.rank());
        assert!(ExecutionPartition::AsyncWorker.rank() < ExecutionPartition::BlockingWorker.rank());
        assert!(ExecutionPartition::BlockingWorker.rank() < ExecutionPartition::External.rank());
    }

    #[test]
    fn given_network_execution_safety_when_compared_then_it_is_preserved() {
        let safety = ExecutionSafety::NetworkAllowed;

        assert_eq!(safety, ExecutionSafety::NetworkAllowed);
    }

    #[test]
    fn given_same_partition_when_needs_bridge_then_false() {
        assert!(!ExecutionPartition::RealtimeCpu.needs_bridge_to(ExecutionPartition::RealtimeCpu));
        assert!(!ExecutionPartition::AsyncWorker.needs_bridge_to(ExecutionPartition::AsyncWorker));
    }

    #[test]
    fn given_different_partitions_when_needs_bridge_then_true() {
        assert!(ExecutionPartition::RealtimeCpu.needs_bridge_to(ExecutionPartition::AsyncWorker));
        assert!(ExecutionPartition::AudioCallback.needs_bridge_to(ExecutionPartition::External));
    }

    #[test]
    fn given_realtime_safe_contract_when_valid_for_audio_callback_then_true() {
        assert!(ExecutionSafety::RealtimeSafe.is_valid_for(ExecutionPartition::AudioCallback));
        assert!(ExecutionSafety::RealtimeSafe.is_valid_for(ExecutionPartition::RealtimeCpu));
    }

    #[test]
    fn given_allocation_allowed_contract_when_valid_for_audio_callback_then_false() {
        assert!(!ExecutionSafety::AllocationAllowed.is_valid_for(ExecutionPartition::AudioCallback));
        assert!(!ExecutionSafety::AllocationAllowed.is_valid_for(ExecutionPartition::RealtimeCpu));
    }

    #[test]
    fn given_network_allowed_contract_when_valid_for_async_worker_then_true() {
        assert!(ExecutionSafety::NetworkAllowed.is_valid_for(ExecutionPartition::AsyncWorker));
    }

    #[test]
    fn given_network_allowed_contract_when_valid_for_blocking_worker_then_false() {
        assert!(!ExecutionSafety::NetworkAllowed.is_valid_for(ExecutionPartition::BlockingWorker));
    }

    #[test]
    fn given_external_service_contract_when_valid_for_external_partition_then_true() {
        assert!(ExecutionSafety::ExternalService.is_valid_for(ExecutionPartition::External));
    }

    #[test]
    fn given_blocking_allowed_contract_when_valid_for_blocking_worker_then_true() {
        assert!(ExecutionSafety::BlockingAllowed.is_valid_for(ExecutionPartition::BlockingWorker));
    }

    #[test]
    fn given_blocking_allowed_contract_when_valid_for_async_worker_then_false() {
        assert!(!ExecutionSafety::BlockingAllowed.is_valid_for(ExecutionPartition::AsyncWorker));
    }
}
