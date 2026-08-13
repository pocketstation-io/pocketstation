//! Allocation-free realtime audio execution lane.

mod executor;
mod router;
mod runner;

pub use executor::{ExecError, PlanExecutionSummary, RealtimePlanExecutor};
pub use router::EdgeObservations;
#[cfg(any(test, feature = "internal-testing"))]
pub use router::PlanRouterError;
pub use router::{DispatchSummary, PlanEdgeRouter};
pub use router::{PlanEdgeFrame, PlanEdgeObservationHandle, PlanEdgeReceiver};
#[cfg(any(test, feature = "internal-testing"))]
pub use runner::PlanRunnerStepSummary;
pub use runner::PlanSourceInputObservations;
pub use runner::PlanSourceSendError;
pub use runner::{
    plan_source_channel, PlanRunnerCancellation, PlanRunnerDrainPolicy, PlanRunnerError,
    PlanRunnerFinishSummary, PlanSourceInput, PlanSourceObservationHandle, PlanSourceSendOutcome,
    PlanSourceSender, RealtimePlanRunner,
};
