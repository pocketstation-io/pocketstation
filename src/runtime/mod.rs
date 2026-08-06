//! pocketstation-runtime — executes a compiled `RuntimePlan` from pocketstation-graph.
//!
//! The planner emits a `RuntimePlan`; this crate runs it. `RealtimePlanRunner`
//! drives the source-aware `RealtimePlanExecutor`, routes only along compiled
//! edges, and hands non-realtime work to independent bounded edge receivers.

pub mod async_bridge;
pub mod async_host;
pub mod async_operator;
pub mod metrics;
pub mod nodes;
pub mod plan_executor;
pub mod plan_router;
pub mod plan_runner;

pub use async_bridge::{AsyncBridge, AsyncBridgeReceiver, AsyncBridgeSendError, AsyncBridgeSender};
pub use async_host::{AsyncRuntimeHost, AsyncRuntimeHostError};
pub use async_operator::{
    AsyncOperatorInput, AsyncOperatorInputAccessError, AsyncOperatorObservationHandle,
    AsyncOperatorObservations, AsyncOperatorOutput, AsyncOperatorOutputBranchSpec,
    AsyncOperatorOutputObservationHandle, AsyncOperatorOutputObservations, AsyncOperatorWorker,
    AsyncOperatorWorkerError, CompiledOperatorInputContract,
};
pub use metrics::{
    BusMetrics, Counter, EdgeMetrics, EdgeObservation, Gauge, SimpleHistogram, SILENCE_FLOOR_DBFS,
};
pub use plan_executor::{ExecError, PlanExecutionSummary, RealtimePlanExecutor};
pub use plan_router::{
    DispatchSummary, EdgeObservations, PlanEdgeFrame, PlanEdgeObservationHandle, PlanEdgeReceiver,
    PlanEdgeRouter, PlanRouterError,
};
pub use plan_runner::{
    plan_source_channel, PlanRunnerCancellation, PlanRunnerDrainPolicy, PlanRunnerError,
    PlanRunnerFinishSummary, PlanRunnerStepSummary, PlanSourceInput, PlanSourceInputObservations,
    PlanSourceObservationHandle, PlanSourceSendError, PlanSourceSendOutcome, PlanSourceSender,
    RealtimePlanRunner,
};
