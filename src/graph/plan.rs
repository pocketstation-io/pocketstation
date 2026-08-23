//! RuntimePlan — the lowered, execution-ready shape the planner emits from a
//! verified `GraphIr`. It carries the execution partitions, per-edge buffer
//! sizing, fan-out/fan-in groupings, and stable metric handles the runtime
//! (Wave 6) consumes. No node is executed here; this is the plan, not the run.

use crate::graph::partition::ExecutionPartition;
use crate::graph::ports::{CopyPolicy, EdgeContract, MediaCaps};
use crate::graph::signal::SignalSpec;
use crate::graph::spec::{EdgeId, InputPortRef, NodeId, OutputPortRef};

#[doc = "Defines the public frame bytes mono 48 k value."]
pub const FRAME_BYTES_MONO_48K: usize = 960 * 4; // 20ms × 48kHz mono = 960 f32 × 4 bytes (ADR-012/013)
#[doc = "Defines the public edge ring capacity frames value."]
pub const EDGE_RING_CAPACITY_FRAMES: usize = 8; // 8 × 20ms = 160ms de-jitter headroom (ADR-010)
/// A sequential edge receiver may retain the frame it just popped while it
/// processes that frame. Copy-pool sizing must cover that owned frame in
/// addition to every frame that can still be queued.
pub const EDGE_RECEIVER_MAX_IN_FLIGHT_FRAMES: usize = 1;
#[doc = "Sets the maximum supported edge ring capacity frames."]
pub const MAX_EDGE_RING_CAPACITY_FRAMES: usize =
    crate::frame::POOL_MAX_SLOTS - EDGE_RECEIVER_MAX_IN_FLIGHT_FRAMES;

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
#[doc = "Classifies failures reported as plan error."]
pub enum PlanError {
    #[error("input port '{port}' on node {node} has multiplicity One but receives multiple edges")]
    #[doc = "Reports fan in on single port."]
    FanInOnSinglePort {
        #[doc = "Stores the node used by `FanInOnSinglePort`."]
        node: u32,
        #[doc = "Stores the port used by `FanInOnSinglePort`."]
        port: String,
    },
    #[error("output port '{port}' on node {node} uses MoveExclusive in a fan-out group")]
    #[doc = "Reports move exclusive fan out."]
    MoveExclusiveFanOut {
        #[doc = "Stores the node used by `MoveExclusiveFanOut`."]
        node: u32,
        #[doc = "Stores the port used by `MoveExclusiveFanOut`."]
        port: String,
    },
    #[error("compiled edge {edge:?} is missing its negotiated contract")]
    #[doc = "Reports missing edge contract."]
    MissingEdgeContract {
        #[doc = "Stores the edge used by `MissingEdgeContract`."]
        edge: EdgeId,
    },
    #[error("compiled edge {edge:?} is missing its declared output signal")]
    #[doc = "Reports missing output signal."]
    MissingOutputSignal {
        #[doc = "Stores the edge used by `MissingOutputSignal`."]
        edge: EdgeId,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[doc = "Uniquely identifies edge metric within its PocketStation ownership scope."]
pub struct EdgeMetricId(pub u32);

#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "Records the compiled execution and resource plan for edge buffer."]
pub struct EdgeBufferPlan {
    #[doc = "Stores the edge used by `EdgeBufferPlan`."]
    pub edge: EdgeId,
    #[doc = "Sets the capacity frames available to `EdgeBufferPlan`."]
    pub capacity_frames: usize,
    #[doc = "Stores the bytes per frame used by `EdgeBufferPlan`."]
    pub bytes_per_frame: usize,
    #[doc = "Stores the copy policy used by `EdgeBufferPlan`."]
    pub copy_policy: CopyPolicy,
}

impl EdgeBufferPlan {
    #[doc = "Returns the total bytes held by `EdgeBufferPlan`."]
    pub fn total_bytes(&self) -> usize {
        self.capacity_frames * self.bytes_per_frame
    }

    #[doc = "Returns the branch copy pool capacity frames held by `EdgeBufferPlan`."]
    pub fn branch_copy_pool_capacity_frames(&self) -> usize {
        if self.copy_policy == CopyPolicy::CopyToBranchPool {
            self.capacity_frames
                .saturating_add(EDGE_RECEIVER_MAX_IN_FLIGHT_FRAMES)
        } else {
            0
        }
    }

    #[doc = "Returns the branch copy pool bytes held by `EdgeBufferPlan`."]
    pub fn branch_copy_pool_bytes(&self) -> usize {
        self.branch_copy_pool_capacity_frames()
            .saturating_mul(self.bytes_per_frame)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "Records the compiled execution and resource plan for memory."]
pub struct MemoryPlan {
    #[doc = "Stores the realtime pool size for `MemoryPlan`, in bytes."]
    pub realtime_pool_bytes: usize,
    #[doc = "Stores the branch copy pool size for `MemoryPlan`, in bytes."]
    pub branch_copy_pool_bytes: usize,
    #[doc = "Stores the edge buffers used by `MemoryPlan`."]
    pub edge_buffers: Vec<EdgeBufferPlan>,
}

impl MemoryPlan {
    #[doc = "Returns the edge buffer held by `MemoryPlan`."]
    pub fn edge_buffer(&self, edge: EdgeId) -> Option<&EdgeBufferPlan> {
        self.edge_buffers.iter().find(|plan| plan.edge == edge)
    }
}

/// A group of nodes assigned to the same execution partition in a compiled plan.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PartitionGroup {
    #[doc = "Stores the execution used by `PartitionGroup`."]
    pub execution: ExecutionPartition,
    #[doc = "Stores the nodes used by `PartitionGroup`."]
    pub nodes: Vec<NodeId>, // global topo order, restricted to this partition
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "Groups the compiled edges that share one output port as their origin."]
pub struct FanOutGroup {
    #[doc = "Identifies the origin represented by `FanOutGroup`."]
    pub from: OutputPortRef,
    #[doc = "Stores the targets used by `FanOutGroup`."]
    pub targets: Vec<EdgeId>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "Groups the compiled edges mixed into one input port."]
pub struct FanInGroup {
    #[doc = "Stores the into used by `FanInGroup`."]
    pub into: InputPortRef,
    #[doc = "Stores the sources used by `FanInGroup`."]
    pub sources: Vec<EdgeId>, // runtime mixes these into the single input port
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "Records the compiled execution and resource plan for typed edge."]
pub struct TypedEdgePlan {
    #[doc = "Stores the edge used by `TypedEdgePlan`."]
    pub edge: EdgeId,
    #[doc = "Identifies the origin represented by `TypedEdgePlan`."]
    pub from: OutputPortRef,
    #[doc = "Identifies the destination represented by `TypedEdgePlan`."]
    pub to: InputPortRef,
    #[doc = "Stores the signal used by `TypedEdgePlan`."]
    pub signal: SignalSpec,
    #[doc = "Stores the media used by `TypedEdgePlan`."]
    pub media: MediaCaps,
    #[doc = "Stores the contract used by `TypedEdgePlan`."]
    pub contract: EdgeContract,
    #[doc = "Sets the capacity signals available to `TypedEdgePlan`."]
    pub capacity_signals: usize,
    #[doc = "Identifies the metric identifier recorded by `TypedEdgePlan`."]
    pub metric_id: EdgeMetricId,
}

/// One connected output of a graph root that runtime preparation must feed.
///
/// Each referenced edge has its bounded capacity in either `typed_edges` or
/// `memory_plan.edge_buffers`; this record supplies the source/output identity
/// without introducing a signal-specific queue.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceOutputPlan {
    #[doc = "Identifies the origin represented by `SourceOutputPlan`."]
    pub from: OutputPortRef,
    #[doc = "Stores the signal used by `SourceOutputPlan`."]
    pub signal: SignalSpec,
    #[doc = "Stores the media used by `SourceOutputPlan`."]
    pub media: MediaCaps,
    #[doc = "Stores the branch edges used by `SourceOutputPlan`."]
    pub branch_edges: Vec<EdgeId>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "Records the compiled execution and resource plan for runtime."]
pub struct RuntimePlan {
    #[doc = "Stores the node order used by `RuntimePlan`."]
    pub node_order: Vec<NodeId>,
    #[doc = "Stores the partitions used by `RuntimePlan`."]
    pub partitions: Vec<PartitionGroup>,
    #[doc = "Stores the memory plan used by `RuntimePlan`."]
    pub memory_plan: MemoryPlan,
    #[doc = "Stores the edge metrics used by `RuntimePlan`."]
    pub edge_metrics: Vec<(EdgeId, EdgeMetricId)>,
    #[doc = "Stores the fan out used by `RuntimePlan`."]
    pub fan_out: Vec<FanOutGroup>,
    #[doc = "Stores the fan in used by `RuntimePlan`."]
    pub fan_in: Vec<FanInGroup>,
    #[doc = "Stores the typed edges used by `RuntimePlan`."]
    pub typed_edges: Vec<TypedEdgePlan>,
    #[doc = "Stores the source outputs used by `RuntimePlan`."]
    pub source_outputs: Vec<SourceOutputPlan>,
    #[doc = "Stores the number of edge represented by `RuntimePlan`."]
    pub edge_count: usize,
}

impl RuntimePlan {
    #[cfg(any(test, feature = "internal-testing"))]
    #[doc = "Returns the node count held by `RuntimePlan`."]
    pub fn node_count(&self) -> usize {
        self.node_order.len()
    }

    #[cfg(any(test, feature = "internal-testing"))]
    #[doc = "Returns the partition held by `RuntimePlan`."]
    pub fn partition(&self, ep: ExecutionPartition) -> Option<&PartitionGroup> {
        self.partitions.iter().find(|group| group.execution == ep)
    }

    #[cfg(any(test, feature = "internal-testing"))]
    #[doc = "Returns the metric identifier held by `RuntimePlan`."]
    pub fn metric_id(&self, edge: EdgeId) -> Option<EdgeMetricId> {
        self.edge_metrics
            .iter()
            .find(|(id, _)| *id == edge)
            .map(|(_, metric)| *metric)
    }

    #[doc = "Returns the typed edge held by `RuntimePlan`."]
    pub fn typed_edge(&self, edge: EdgeId) -> Option<&TypedEdgePlan> {
        self.typed_edges.iter().find(|plan| plan.edge == edge)
    }
}
