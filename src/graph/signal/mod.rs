//! Signal vocabulary, envelopes, lineage, and asynchronous Operator APIs.

mod continuity;
mod envelope;
mod lineage;
mod operator;
mod payload;
mod preparation;
mod spec;
mod timing;

pub use continuity::{SignalContinuityError, SignalContinuityObservation, SignalContinuityTracker};
pub use envelope::{SignalEnvelope, SignalEnvelopeError};
pub use lineage::{SignalDerivation, SignalDerivationError, SignalLineage, SignalLineageError};
pub use operator::{
    AsyncNode, AsyncOperatorFactory, AsyncOperatorManifest, AsyncOperatorManifestError,
    OperatorCancellationPolicy, OperatorDeadlinePolicy, OperatorFailurePolicy,
    OperatorOutputRolePolicy, OperatorPermissionPolicy,
};
pub use payload::SignalPayload;
pub use preparation::{AsyncNodeFuture, AsyncOperatorPrepareContext};
pub use spec::{
    BinaryFormat, Codec, EventFormat, SchemaRef, SemanticRole, SignalClass, SignalId, SignalSpec,
    SignalSpecError, TextFormat,
};
pub use timing::{SignalTiming, SignalTimingError};
