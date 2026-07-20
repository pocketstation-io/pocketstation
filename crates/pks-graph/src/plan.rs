//! RuntimePlan — the lowered, execution-ready shape the planner emits from a
//! verified `GraphIr`. It carries the execution partitions, per-edge buffer
//! sizing, fan-out/fan-in groupings, and stable metric handles the runtime
//! (Wave 6) consumes. No node is executed here; this is the plan, not the run.

use crate::partition::ExecutionPartition;
use crate::spec::{EdgeId, InputPortRef, NodeId, OutputPortRef};
use pks_caps::CopyPolicy;

pub const FRAME_BYTES_MONO_48K: usize = 960 * 4; // 20ms × 48kHz mono = 960 f32 × 4 bytes (ADR-012/013)
pub const EDGE_RING_CAPACITY_FRAMES: usize = 8; // 8 × 20ms = 160ms de-jitter headroom (ADR-010)
pub const MAX_EDGE_RING_CAPACITY_FRAMES: usize = 64;

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum PlanError {
    #[error("input port '{port}' on node {node} has multiplicity One but receives multiple edges")]
    FanInOnSinglePort { node: u32, port: String },
    #[error("output port '{port}' on node {node} uses MoveExclusive in a fan-out group")]
    MoveExclusiveFanOut { node: u32, port: String },
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
pub struct RuntimePlan {
    pub node_order: Vec<NodeId>,
    pub partitions: Vec<PartitionGroup>,
    pub memory_plan: MemoryPlan,
    pub edge_metrics: Vec<(EdgeId, EdgeMetricId)>,
    pub fan_out: Vec<FanOutGroup>,
    pub fan_in: Vec<FanInGroup>,
    pub edge_count: usize,
}

impl RuntimePlan {
    pub fn node_count(&self) -> usize {
        self.node_order.len()
    }

    pub fn partition(&self, ep: ExecutionPartition) -> Option<&PartitionGroup> {
        self.partitions.iter().find(|group| group.execution == ep)
    }

    pub fn metric_id(&self, edge: EdgeId) -> Option<EdgeMetricId> {
        self.edge_metrics
            .iter()
            .find(|(id, _)| *id == edge)
            .map(|(_, metric)| *metric)
    }
}
