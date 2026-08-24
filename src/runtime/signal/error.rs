//! Failures produced by async operator preparation, execution, and shutdown.

use crate::graph::NodeError;

#[derive(Debug, thiserror::Error)]
#[doc = "Classifies failures surfaced by async operator worker operations."]
pub enum AsyncOperatorWorkerError {
    #[error("operator prepare failed: {0}")]
    #[doc = "Classifies a failure at the prepare stage or component of `AsyncOperatorWorkerError`."]
    Prepare(NodeError),
    #[error("operator prepare exceeded {timeout_ms} ms")]
    #[doc = "Reports that prepare exceeded its deadline."]
    PrepareTimeout {
        #[doc = "Stores the timeout value for `PrepareTimeout`, in milliseconds."]
        timeout_ms: u32,
    },
    #[error("operator process failed: {0}")]
    #[doc = "Classifies a failure at the process stage or component of `AsyncOperatorWorkerError`."]
    Process(NodeError),
    #[error("operator process exceeded {timeout_ms} ms")]
    #[doc = "Reports that the operation exceeded its deadline."]
    Timeout {
        #[doc = "Stores the timeout value for `Timeout`, in milliseconds."]
        timeout_ms: u32,
    },
    #[error("operator close failed: {0}")]
    #[doc = "Classifies a failure at the close stage or component of `AsyncOperatorWorkerError`."]
    Close(NodeError),
    #[error("operator close exceeded {timeout_ms} ms")]
    #[doc = "Reports that close exceeded its deadline."]
    CloseTimeout {
        #[doc = "Stores the timeout value for `CloseTimeout`, in milliseconds."]
        timeout_ms: u32,
    },
    #[error("operator cancellation cleanup failed: {0}")]
    #[doc = "Classifies a failure at the cancel stage or component of `AsyncOperatorWorkerError`."]
    Cancel(NodeError),
    #[error("operator cancellation cleanup exceeded {timeout_ms} ms")]
    #[doc = "Reports that cancel exceeded its deadline."]
    CancelTimeout {
        #[doc = "Stores the timeout value for `CancelTimeout`, in milliseconds."]
        timeout_ms: u32,
    },
    #[error("async operator output has no derived lineage")]
    #[doc = "Reports that the required derived lineage is missing."]
    MissingDerivedLineage,
    #[error("async operator output lineage does not match its registered manifest")]
    #[doc = "Reports that derived lineage does not match the expected contract."]
    DerivedLineageMismatch,
    #[error("async operator output signal does not match its registered output port")]
    #[doc = "Reports that output signal does not match the expected contract."]
    OutputSignalMismatch,
    #[error("async operator output role is not declared by its registered manifest")]
    #[doc = "Reports that output role was emitted or requested without a declaration."]
    UndeclaredOutputRole,
    #[error("async operator manifest has no output port at runtime")]
    #[doc = "Reports that the required output contract is missing."]
    MissingOutputContract,
    #[error("async operator input port '{port_name}' is not declared by its manifest")]
    #[doc = "Reports that the referenced input port is not declared or registered."]
    UnknownInputPort {
        #[doc = "Stores the human-readable port used to identify `UnknownInputPort`."]
        port_name: String,
    },
    #[error("async operator output matches multiple declared output ports")]
    #[doc = "Reports that output port resolves to more than one candidate."]
    AmbiguousOutputPort,
    #[error("terminal output was rejected by full output branch {branch_index}")]
    #[doc = "Reports that terminal output was dropped before delivery completed."]
    TerminalOutputDropped {
        #[doc = "Identifies the branch index position within `TerminalOutputDropped`."]
        branch_index: usize,
    },
    #[error(
        "operator output branch {branch_index} rejected {payload_bytes} payload bytes; maximum is {max_payload_bytes}"
    )]
    #[doc = "Reports that output payload exceeds the supported size limit."]
    OutputPayloadTooLarge {
        #[doc = "Identifies the branch index position within `OutputPayloadTooLarge`."]
        branch_index: usize,
        #[doc = "Stores the payload size for `OutputPayloadTooLarge`, in bytes."]
        payload_bytes: usize,
        #[doc = "Limits payload storage for `OutputPayloadTooLarge`, in bytes."]
        max_payload_bytes: usize,
    },
    #[error("operator worker task failed: {0}")]
    #[doc = "Classifies a failure at the join stage or component of `AsyncOperatorWorkerError`."]
    Join(#[from] tokio::task::JoinError),
    #[error("compiled async input requires a lineaged exclusive plan-edge frame, received {kind}")]
    #[doc = "Reports that the supplied plan input is invalid."]
    InvalidPlanInput {
        #[doc = "Records the kind selected for `InvalidPlanInput`."]
        kind: &'static str,
    },
    #[error("compiled async input lineage does not match its Session stem contract")]
    #[doc = "Reports that plan input lineage does not match the expected contract."]
    PlanInputLineageMismatch,
    #[error("shared audio typed input requires an exclusive generated-audio branch")]
    #[doc = "Classifies a failure at the shared audio typed input stage or component of `AsyncOperatorWorkerError`."]
    SharedAudioTypedInput,
}
