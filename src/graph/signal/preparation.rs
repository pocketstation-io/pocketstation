//! Signal-shaped asynchronous Operator preparation contracts.

use std::future::Future;
use std::pin::Pin;

use crate::graph::node::{NodeError, PortPrepareContext};
use crate::graph::{ExecutionPartition, PortDirection};

#[doc = "Names the future returned by async node operations."]
pub type AsyncNodeFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

/// Exact bounded graph edge supplied to an asynchronous Operator at prepare time.
///
/// This contract is signal-shaped, not audio-shaped. An audio edge carries
/// audio `MediaCaps`; text, event, metrics, control, binary, and custom edges
/// carry their own media and never receive a fabricated `SampleSpec`.
/// Compatibility name for the graph-wide port preparation authority.
/// New code should use `PortPrepareContext` directly.
pub type AsyncOperatorEdgePrepareContext = PortPrepareContext;

/// Complete graph-owned preparation contract for one asynchronous Operator.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AsyncOperatorPrepareContext {
    execution_partition: ExecutionPartition,
    inputs: Vec<AsyncOperatorEdgePrepareContext>,
    outputs: Vec<AsyncOperatorEdgePrepareContext>,
}

impl AsyncOperatorPrepareContext {
    #[doc = "Creates a new `AsyncOperatorPrepareContext`."]
    pub fn new(
        execution_partition: ExecutionPartition,
        edges: Vec<AsyncOperatorEdgePrepareContext>,
    ) -> Result<Self, NodeError> {
        if execution_partition.requires_realtime_safety() {
            return Err(NodeError::Prepare(
                "async operator cannot prepare in a realtime partition".to_owned(),
            ));
        }
        let mut inputs = Vec::new();
        let mut outputs = Vec::new();
        for edge in edges {
            match edge.direction() {
                PortDirection::Input => inputs.push(edge),
                PortDirection::Output => outputs.push(edge),
            }
        }
        if inputs.is_empty() || outputs.is_empty() {
            return Err(NodeError::Prepare(
                "async operator prepare requires bounded input and output edges".to_owned(),
            ));
        }
        Ok(Self {
            execution_partition,
            inputs,
            outputs,
        })
    }

    #[doc = "Returns the execution partition held by `AsyncOperatorPrepareContext`."]
    pub const fn execution_partition(&self) -> ExecutionPartition {
        self.execution_partition
    }

    #[doc = "Returns the inputs held by `AsyncOperatorPrepareContext`."]
    pub fn inputs(&self) -> &[AsyncOperatorEdgePrepareContext] {
        &self.inputs
    }

    #[doc = "Returns the outputs held by `AsyncOperatorPrepareContext`."]
    pub fn outputs(&self) -> &[AsyncOperatorEdgePrepareContext] {
        &self.outputs
    }
}
