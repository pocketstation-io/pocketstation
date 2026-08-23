//! RuntimePlan — the lowered, execution-ready shape the planner emits from a
//! verified `GraphIr`. It carries the execution partitions, per-edge buffer
//! sizing, fan-out/fan-in groupings, and stable metric handles the runtime
//! (Wave 6) consumes. No node is executed here; this is the plan, not the run.

use crate::graph::partition::ExecutionPartition;
use crate::graph::ports::{CopyPolicy, EdgeContract, MediaCaps};
use crate::graph::signal::SignalSpec;
use crate::graph::spec::{EdgeId, InputPortRef, NodeId, OutputPortRef};

pub const FRAME_BYTES_MONO_48K: usize = 960 * 4; // 20ms × 48kHz mono = 960 f32 × 4 bytes (ADR-012/013)
pub const EDGE_RING_CAPACITY_FRAMES: usize = 8; // 8 × 20ms = 160ms de-jitter headroom (ADR-010)
/// A sequential edge receiver may retain the frame it just popped while it
/// processes that frame. Copy-pool sizing must cover that owned frame in
/// addition to every frame that can still be queued.
pub const EDGE_RECEIVER_MAX_IN_FLIGHT_FRAMES: usize = 1;
pub const MAX_EDGE_RING_CAPACITY_FRAMES: usize =
    crate::frame::POOL_MAX_SLOTS - EDGE_RECEIVER_MAX_IN_FLIGHT_FRAMES;

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
#[doc = "Classifies failures reported as plan error."]
pub enum PlanError {
    #[error("input port '{port}' on node {node} has multiplicity One but receives multiple edges")]
    #[doc = "Reports fan in on single port."]
    FanInOnSinglePort {
        #[doc = "Stores the node associated with `FanInOnSinglePort`."]
        node: u32,
        #[doc = "Stores the port associated with `FanInOnSinglePort`."]
        port: String,
    },
    #[error("output port '{port}' on node {node} uses MoveExclusive in a fan-out group")]
    #[doc = "Reports move exclusive fan out."]
    MoveExclusiveFanOut {
        #[doc = "Stores the node associated with `MoveExclusiveFanOut`."]
        node: u32,
        #[doc = "Stores the port associated with `MoveExclusiveFanOut`."]
        port: String,
    },
    #[error("compiled edge {edge:?} is missing its negotiated contract")]
    #[doc = "Reports missing edge contract."]
    MissingEdgeContract {
        #[doc = "Stores the edge associated with `MissingEdgeContract`."]
        edge: EdgeId,
    },
    #[error("compiled edge {edge:?} is missing its declared output signal")]
    #[doc = "Reports missing output signal."]
    MissingOutputSignal {
        #[doc = "Stores the edge associated with `MissingOutputSignal`."]
        edge: EdgeId,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EdgeMetricId(pub u32);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EdgeBufferPlan {
    pub edge: EdgeId,
    pub capacity_frames: usize,
    pub bytes_per_frame: usize,
    pub copy_policy: CopyPolicy,
}

impl EdgeBufferPlan {
    pub fn total_bytes(&self) -> usize {
        self.capacity_frames * self.bytes_per_frame
    }

    pub fn branch_copy_pool_capacity_frames(&self) -> usize {
        if self.copy_policy == CopyPolicy::CopyToBranchPool {
            self.capacity_frames
                .saturating_add(EDGE_RECEIVER_MAX_IN_FLIGHT_FRAMES)
        } else {
            0
        }
    }

    pub fn branch_copy_pool_bytes(&self) -> usize {
        self.branch_copy_pool_capacity_frames()
            .saturating_mul(self.bytes_per_frame)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MemoryPlan {
    pub realtime_pool_bytes: usize,
    pub branch_copy_pool_bytes: usize,
    pub edge_buffers: Vec<EdgeBufferPlan>,
}

impl MemoryPlan {
    pub fn edge_buffer(&self, edge: EdgeId) -> Option<&EdgeBufferPlan> {
        self.edge_buffers.iter().find(|plan| plan.edge == edge)
    }
}

/// A group of nodes assigned to the same execution partition in a compiled plan.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PartitionGroup {
    pub execution: ExecutionPartition,
    pub nodes: Vec<NodeId>, // global topo order, restricted to this partition
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FanOutGroup {
    pub from: OutputPortRef,
    pub targets: Vec<EdgeId>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FanInGroup {
    pub into: InputPortRef,
    pub sources: Vec<EdgeId>, // runtime mixes these into the single input port
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypedEdgePlan {
    pub edge: EdgeId,
    pub from: OutputPortRef,
    pub to: InputPortRef,
    pub signal: SignalSpec,
    pub media: MediaCaps,
    pub contract: EdgeContract,
    pub capacity_signals: usize,
    pub metric_id: EdgeMetricId,
}

/// One connected output of a graph root that runtime preparation must feed.
///
/// Each referenced edge has its bounded capacity in either `typed_edges` or
/// `memory_plan.edge_buffers`; this record supplies the source/output identity
/// without introducing a signal-specific queue.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceOutputPlan {
    pub from: OutputPortRef,
    pub signal: SignalSpec,
    pub media: MediaCaps,
    pub branch_edges: Vec<EdgeId>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimePlan {
    pub node_order: Vec<NodeId>,
    pub partitions: Vec<PartitionGroup>,
    pub memory_plan: MemoryPlan,
    pub edge_metrics: Vec<(EdgeId, EdgeMetricId)>,
    pub fan_out: Vec<FanOutGroup>,
    pub fan_in: Vec<FanInGroup>,
    pub typed_edges: Vec<TypedEdgePlan>,
    pub source_outputs: Vec<SourceOutputPlan>,
    pub edge_count: usize,
}

impl RuntimePlan {
    #[cfg(any(test, feature = "internal-testing"))]
    pub fn node_count(&self) -> usize {
        self.node_order.len()
    }

    #[cfg(any(test, feature = "internal-testing"))]
    pub fn partition(&self, ep: ExecutionPartition) -> Option<&PartitionGroup> {
        self.partitions.iter().find(|group| group.execution == ep)
    }

    #[cfg(any(test, feature = "internal-testing"))]
    pub fn metric_id(&self, edge: EdgeId) -> Option<EdgeMetricId> {
        self.edge_metrics
            .iter()
            .find(|(id, _)| *id == edge)
            .map(|(_, metric)| *metric)
    }

    pub fn typed_edge(&self, edge: EdgeId) -> Option<&TypedEdgePlan> {
        self.typed_edges.iter().find(|plan| plan.edge == edge)
    }
}
