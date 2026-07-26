//! pocketstation-runtime — executes a compiled `RuntimePlan` from pocketstation-graph.
//!
//! The planner emits a `RuntimePlan`; this crate runs it. `PlanScheduler`
//! instantiates realtime nodes into a `RealtimePlanExecutor`, routes only along
//! compiled edges, and hands non-realtime work to independent bounded edge
//! receivers. The older linear `RealtimeExecutor` remains available for focused
//! compatibility tests.

pub mod async_bridge;
pub mod edge_channel;
pub mod executor;
pub mod plan_executor;
pub mod plan_router;
pub mod plan_runner;
pub mod scheduler;
pub mod shared_edge_channel;

pub use async_bridge::{AsyncBridge, AsyncBridgeReceiver, AsyncBridgeSender};
pub use edge_channel::{EdgeChannel, EdgeReceiver, EdgeSender, EdgeTelemetrySnapshot};
pub use executor::{ExecError, RealtimeExecutor, RunMetrics};
pub use plan_executor::{PlanExecutionSummary, RealtimePlanExecutor};
pub use plan_router::{
    DispatchSummary, EdgeObservations, PlanEdgeFrame, PlanEdgeReceiver, PlanEdgeRouter,
    PlanRouterError,
};
pub use plan_runner::{
    plan_source_channel, PlanRunnerCancellation, PlanRunnerDrainPolicy, PlanRunnerError,
    PlanRunnerFinishSummary, PlanRunnerStepSummary, PlanSourceInput, PlanSourceInputObservations,
    PlanSourceSendError, PlanSourceSendOutcome, PlanSourceSender, RealtimePlanRunner,
};
pub use scheduler::PlanScheduler;
pub use shared_edge_channel::{
    SharedEdgeChannel, SharedEdgeReceiver, SharedEdgeSender, SharedEdgeTelemetrySnapshot,
};
