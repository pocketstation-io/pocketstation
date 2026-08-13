//! Immutable compiled Session plus its typed runtime bindings.

use crate::graph::ir::GraphIr;
use crate::session::SessionSpec;

#[cfg(any(test, feature = "internal-testing"))]
use crate::session::declaration::{EndpointSpec, SourceInstanceSpec, StemSpec};
#[cfg(any(test, feature = "internal-testing"))]
use crate::session::SessionId;

use super::CompiledSessionBindings;

pub struct CompiledSession {
    pub(super) spec: SessionSpec,
    pub(super) graph_ir: GraphIr,
    pub(super) runtime_plan: crate::graph::plan::RuntimePlan,
    pub(super) bindings: CompiledSessionBindings,
}

impl CompiledSession {
    #[cfg(any(test, feature = "internal-testing"))]
    pub const fn session_id(&self) -> SessionId {
        self.spec.session_id()
    }

    #[cfg(any(test, feature = "internal-testing"))]
    pub fn spec(&self) -> &SessionSpec {
        &self.spec
    }

    #[cfg(any(test, feature = "internal-testing"))]
    pub fn source_declarations(&self) -> &[StemSpec] {
        self.spec.stems()
    }

    #[cfg(any(test, feature = "internal-testing"))]
    pub fn external_source_declarations(&self) -> &[SourceInstanceSpec] {
        self.spec.source_instances()
    }

    #[cfg(any(test, feature = "internal-testing"))]
    pub fn endpoint_declarations(&self) -> &[EndpointSpec] {
        self.spec.endpoints()
    }

    #[cfg(any(test, feature = "internal-testing"))]
    pub fn node_count(&self) -> usize {
        self.graph_ir.node_count()
    }

    #[cfg(any(test, feature = "internal-testing"))]
    pub fn edge_count(&self) -> usize {
        self.graph_ir.edge_count()
    }

    #[cfg(any(test, feature = "internal-testing"))]
    pub fn planned_edge_count(&self) -> usize {
        self.runtime_plan.edge_count
    }

    #[cfg(test)]
    pub(crate) fn planned_source_output_count(&self) -> usize {
        self.runtime_plan.source_outputs.len()
    }

    #[cfg(test)]
    pub(crate) fn planned_typed_edge_count(&self) -> usize {
        self.runtime_plan.typed_edges.len()
    }

    #[cfg(test)]
    pub(crate) fn planned_audio_edge_count(&self) -> usize {
        self.runtime_plan.memory_plan.edge_buffers.len()
    }

    pub(crate) fn into_runtime_parts(
        self,
    ) -> (
        SessionSpec,
        GraphIr,
        crate::graph::plan::RuntimePlan,
        CompiledSessionBindings,
    ) {
        (self.spec, self.graph_ir, self.runtime_plan, self.bindings)
    }

    #[cfg(test)]
    pub(crate) fn graph_ir(&self) -> &GraphIr {
        &self.graph_ir
    }

    #[cfg(test)]
    pub(crate) fn graph_ir_mut(&mut self) -> &mut GraphIr {
        &mut self.graph_ir
    }

    #[cfg(test)]
    pub(crate) fn bindings(&self) -> &CompiledSessionBindings {
        &self.bindings
    }

    #[cfg(test)]
    pub(crate) fn bindings_mut(&mut self) -> &mut CompiledSessionBindings {
        &mut self.bindings
    }
}
