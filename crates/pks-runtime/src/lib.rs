//! pocketstation-runtime — executes a compiled `RuntimePlan` from pocketstation-graph.
//!
//! The planner emits a `RuntimePlan`; this crate runs it. `PlanScheduler`
//! instantiates the plan's realtime nodes into a `RealtimeExecutor`, which drives
//! frames through them in topological order. Cross-partition handoff (async,
//! model, network, recorder) rides bounded `EdgeChannel`s; executing those
//! partitions is a later wave and is not implemented here.

pub mod edge_channel;
pub mod executor;
pub mod scheduler;

pub use edge_channel::{EdgeChannel, EdgeReceiver, EdgeSender};
pub use executor::{ExecError, RealtimeExecutor, RunMetrics};
pub use scheduler::PlanScheduler;
