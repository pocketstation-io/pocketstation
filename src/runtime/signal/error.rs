//! Failures produced by async operator preparation, execution, and shutdown.

use crate::graph::NodeError;

#[derive(Debug, thiserror::Error)]
pub enum AsyncOperatorWorkerError {
    #[error("operator prepare failed: {0}")]
    Prepare(NodeError),
    #[error("operator prepare exceeded {timeout_ms} ms")]
    PrepareTimeout { timeout_ms: u32 },
    #[error("operator process failed: {0}")]
    Process(NodeError),
    #[error("operator process exceeded {timeout_ms} ms")]
    Timeout { timeout_ms: u32 },
    #[error("operator close failed: {0}")]
    Close(NodeError),
    #[error("operator close exceeded {timeout_ms} ms")]
    CloseTimeout { timeout_ms: u32 },
    #[error("operator cancellation cleanup failed: {0}")]
    Cancel(NodeError),
    #[error("operator cancellation cleanup exceeded {timeout_ms} ms")]
    CancelTimeout { timeout_ms: u32 },
    #[error("async operator output has no derived lineage")]
    MissingDerivedLineage,
    #[error("async operator output lineage does not match its registered manifest")]
    DerivedLineageMismatch,
    #[error("async operator output signal does not match its registered output port")]
    OutputSignalMismatch,
    #[error("async operator output role is not declared by its registered manifest")]
    UndeclaredOutputRole,
    #[error("async operator manifest has no output port at runtime")]
    MissingOutputRoute,
    #[error("async operator input port '{port_name}' is not declared by its manifest")]
    UnknownInputPort { port_name: String },
    #[error("async operator output matches multiple declared output ports")]
    AmbiguousOutputPort,
    #[error("terminal output was rejected by full output branch {branch_index}")]
    TerminalOutputDropped { branch_index: usize },
    #[error(
        "operator output branch {branch_index} rejected {payload_bytes} payload bytes; maximum is {max_payload_bytes}"
    )]
    OutputPayloadTooLarge {
        branch_index: usize,
        payload_bytes: usize,
        max_payload_bytes: usize,
    },
    #[error("operator worker task failed: {0}")]
    Join(#[from] tokio::task::JoinError),
    #[error("compiled async input requires a lineaged exclusive plan-edge frame, received {kind}")]
    InvalidPlanInput { kind: &'static str },
    #[error("compiled async input lineage does not match its declared Session stem")]
    PlanInputLineageMismatch,
    #[error("shared audio typed input requires an exclusive generated-audio branch")]
    SharedAudioTypedInput,
}
