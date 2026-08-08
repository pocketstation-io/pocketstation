//! pocketstation-runtime — executes a compiled `RuntimePlan` from pocketstation-graph.
//!
//! The planner emits a `RuntimePlan`; this crate runs it. `RealtimePlanRunner`
//! drives the source-aware `RealtimePlanExecutor`, routes only along compiled
//! edges, and hands non-realtime work to independent bounded edge receivers.

pub mod async_bridge;
pub mod async_host;
pub mod async_operator;
pub mod generated_audio_bridge;
pub mod metrics;
pub mod nodes;
pub mod plan_executor;
pub mod plan_router;
pub mod plan_runner;
pub mod sidecar_protocol;
pub mod typed_edge;

pub use async_bridge::{AsyncBridge, AsyncBridgeReceiver, AsyncBridgeSendError, AsyncBridgeSender};
pub use async_host::{AsyncRuntimeHost, AsyncRuntimeHostError};
pub use async_operator::{
    AsyncOperatorInput, AsyncOperatorInputAccessError, AsyncOperatorNamedOutput,
    AsyncOperatorNamedOutputBranchSpec, AsyncOperatorObservationHandle, AsyncOperatorObservations,
    AsyncOperatorOutput, AsyncOperatorOutputBranchSpec, AsyncOperatorOutputObservationHandle,
    AsyncOperatorOutputObservations, AsyncOperatorTypedInput, AsyncOperatorWorker,
    AsyncOperatorWorkerError, CompiledOperatorInputContract,
};
pub use generated_audio_bridge::{
    GeneratedAudioBridge, GeneratedAudioBridgeObservationHandle, GeneratedAudioBridgeObservations,
    GeneratedAudioBridgeSpec, GeneratedAudioBridgeStartError,
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
pub use sidecar_protocol::{
    SidecarMessage, SidecarMessageKind, SidecarProtocolError, SidecarProtocolLimits,
    SIDECAR_PROTOCOL_MAJOR, SIDECAR_PROTOCOL_MINOR,
};
pub use typed_edge::{
    TypedEdgeBranchSpec, TypedEdgeBuildError, TypedEdgeFanout, TypedEdgeObservationHandle,
    TypedEdgeObservations, TypedEdgePublishError, TypedEdgePublishReport, TypedEdgeReceiver,
};
