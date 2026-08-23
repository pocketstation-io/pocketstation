//! Failures produced by async operator preparation, execution, and shutdown.

use crate::graph::NodeError;

#[derive(Debug, thiserror::Error)]
#[doc = "Classifies failures reported as async operator worker error."]
pub enum AsyncOperatorWorkerError {
    #[error("operator prepare failed: {0}")]
    #[doc = "Reported when the owning operation encounters prepare."]
    Prepare(NodeError),
    #[error("operator prepare exceeded {timeout_ms} ms")]
    #[doc = "Reported when the owning operation encounters prepare timeout."]
    PrepareTimeout {
        #[doc = "Stores the timeout value for `PrepareTimeout`, in milliseconds."]
        timeout_ms: u32,
    },
    #[error("operator process failed: {0}")]
    #[doc = "Reported when the owning operation encounters process."]
    Process(NodeError),
    #[error("operator process exceeded {timeout_ms} ms")]
    #[doc = "Reported when the owning operation encounters timeout."]
    Timeout {
        #[doc = "Stores the timeout value for `Timeout`, in milliseconds."]
        timeout_ms: u32,
    },
    #[error("operator close failed: {0}")]
    #[doc = "Reported when the owning operation encounters close."]
    Close(NodeError),
    #[error("operator close exceeded {timeout_ms} ms")]
    #[doc = "Reported when the owning operation encounters close timeout."]
    CloseTimeout {
        #[doc = "Stores the timeout value for `CloseTimeout`, in milliseconds."]
        timeout_ms: u32,
    },
    #[error("operator cancellation cleanup failed: {0}")]
    #[doc = "Reported when the owning operation encounters cancel."]
    Cancel(NodeError),
    #[error("operator cancellation cleanup exceeded {timeout_ms} ms")]
    #[doc = "Reported when the owning operation encounters cancel timeout."]
    CancelTimeout {
        #[doc = "Stores the timeout value for `CancelTimeout`, in milliseconds."]
        timeout_ms: u32,
    },
    #[error("async operator output has no derived lineage")]
    #[doc = "Reported when the owning operation encounters missing derived lineage."]
    MissingDerivedLineage,
    #[error("async operator output lineage does not match its registered manifest")]
    #[doc = "Reported when the owning operation encounters derived lineage mismatch."]
    DerivedLineageMismatch,
    #[error("async operator output signal does not match its registered output port")]
    #[doc = "Reported when the owning operation encounters output signal mismatch."]
    OutputSignalMismatch,
    #[error("async operator output role is not declared by its registered manifest")]
    #[doc = "Reported when the owning operation encounters undeclared output role."]
    UndeclaredOutputRole,
    #[error("async operator manifest has no output port at runtime")]
    #[doc = "Reported when the owning operation encounters missing output contract."]
    MissingOutputContract,
    #[error("async operator input port '{port_name}' is not declared by its manifest")]
    #[doc = "Reported when the owning operation encounters unknown input port."]
    UnknownInputPort {
        #[doc = "Stores the port name used by `UnknownInputPort`."]
        port_name: String,
    },
    #[error("async operator output matches multiple declared output ports")]
    #[doc = "Reported when the owning operation encounters ambiguous output port."]
    AmbiguousOutputPort,
    #[error("terminal output was rejected by full output branch {branch_index}")]
    #[doc = "Reported when the owning operation encounters terminal output dropped."]
    TerminalOutputDropped {
        #[doc = "Stores the branch index used by `TerminalOutputDropped`."]
        branch_index: usize,
    },
    #[error(
        "operator output branch {branch_index} rejected {payload_bytes} payload bytes; maximum is {max_payload_bytes}"
    )]
    #[doc = "Reported when the owning operation encounters output payload too large."]
    OutputPayloadTooLarge {
        #[doc = "Stores the branch index used by `OutputPayloadTooLarge`."]
        branch_index: usize,
        #[doc = "Stores the payload size for `OutputPayloadTooLarge`, in bytes."]
        payload_bytes: usize,
        #[doc = "Limits payload storage for `OutputPayloadTooLarge`, in bytes."]
        max_payload_bytes: usize,
    },
    #[error("operator worker task failed: {0}")]
    #[doc = "Reported when the owning operation encounters join."]
    Join(#[from] tokio::task::JoinError),
    #[error("compiled async input requires a lineaged exclusive plan-edge frame, received {kind}")]
    #[doc = "Reported when the owning operation encounters invalid plan input."]
    InvalidPlanInput {
        #[doc = "Stores the kind used by `InvalidPlanInput`."]
        kind: &'static str,
    },
    #[error("compiled async input lineage does not match its Session stem contract")]
    #[doc = "Reported when the owning operation encounters plan input lineage mismatch."]
    PlanInputLineageMismatch,
    #[error("shared audio typed input requires an exclusive generated-audio branch")]
    #[doc = "Reported when the owning operation encounters shared audio typed input."]
    SharedAudioTypedInput,
}
