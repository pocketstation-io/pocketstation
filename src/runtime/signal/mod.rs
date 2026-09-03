//! Bounded asynchronous signal execution lane.

mod edge;
mod error;
mod io;
mod observations;
mod operator;

pub(crate) use edge::{SignalEdge, SignalEdgeReceiver, SignalEdgeSender};
#[cfg(any(test, feature = "internal-testing"))]
pub use edge::{SignalEdgeSendError, TypedEdgePublishReport};
pub use edge::{
    TypedEdgeBranchSpec, TypedEdgeBuildError, TypedEdgeFanout, TypedEdgeObservationHandle,
    TypedEdgeObservations, TypedEdgePublishError, TypedEdgeReceiver,
};
#[cfg(any(test, feature = "internal-testing"))]
pub use error::AsyncOperatorWorkerError;
#[cfg(any(test, feature = "internal-testing"))]
pub use io::AsyncOperatorInputAccessError;
pub use io::AsyncOperatorOutputObservations;
#[cfg(any(test, feature = "internal-testing"))]
pub use io::{AsyncOperatorInput, AsyncOperatorNamedOutput};
pub use io::{
    AsyncOperatorNamedOutputBranchSpec, AsyncOperatorOutput, AsyncOperatorOutputBranchSpec,
    AsyncOperatorOutputObservationHandle, AsyncOperatorTypedInput,
};
pub use observations::AsyncOperatorObservationHandle;
pub use observations::AsyncOperatorObservations;
pub(crate) use operator::SessionOperatorInput;
pub use operator::{AsyncOperatorWorker, CompiledOperatorInputDetails};
