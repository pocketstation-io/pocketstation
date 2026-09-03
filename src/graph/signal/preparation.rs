//! Preparation data for asynchronous Operators.

use std::future::Future;
use std::pin::Pin;

use crate::graph::node::{NodeError, PortPrepareContext};
use crate::graph::{ExecutionPartition, PortDirection};

pub type AsyncNodeFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

/// Complete graph-owned preparation data for one asynchronous Operator.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AsyncOperatorPrepareContext {
    execution_partition: ExecutionPartition,
    inputs: Vec<PortPrepareContext>,
    outputs: Vec<PortPrepareContext>,
}

impl AsyncOperatorPrepareContext {
    pub fn new(
        execution_partition: ExecutionPartition,
        edges: Vec<PortPrepareContext>,
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

    pub const fn execution_partition(&self) -> ExecutionPartition {
        self.execution_partition
    }

    pub fn inputs(&self) -> &[PortPrepareContext] {
        &self.inputs
    }

    pub fn outputs(&self) -> &[PortPrepareContext] {
        &self.outputs
    }
}
