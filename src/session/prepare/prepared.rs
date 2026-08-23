//! Setup-time ownership produced from one compiled Session.

#[cfg(any(test, feature = "internal-testing"))]
use crate::frame::{RouteId, SessionId};
use crate::runtime::{PlanRunnerCancellation, PlanSourceInput, RealtimePlanExecutor};
use crate::session::SessionSpec;

use super::{
    PreparedExternalSourceMapping, PreparedOperatorMapping, PreparedSourceMapping,
    PreparedWorkerMapping,
};

/// Setup-time ownership for one compiled Session.
///
/// Preparation instantiates the realtime plan and allocates only bounded
/// channels. It does not open capture, start endpoint workers, spawn a runtime
/// thread, or publish a `Running` lifecycle state.
pub struct PreparedSession {
    pub(crate) spec: SessionSpec,
    pub(crate) executor: RealtimePlanExecutor,
    pub(crate) source_mappings: Vec<PreparedSourceMapping>,
    pub(crate) source_inputs: Vec<PlanSourceInput>,
    pub(crate) worker_mappings: Vec<PreparedWorkerMapping>,
    pub(crate) operator_mappings: Vec<PreparedOperatorMapping>,
    pub(crate) external_source_mappings: Vec<PreparedExternalSourceMapping>,
    pub(crate) cancellation: PlanRunnerCancellation,
}

impl PreparedSession {
    #[cfg(any(test, feature = "internal-testing"))]
    #[doc = "Returns the session identifier held by `PreparedSession`."]
    pub const fn session_id(&self) -> SessionId {
        self.spec.session_id()
    }

    #[cfg(any(test, feature = "internal-testing"))]
    #[doc = "Returns the spec held by `PreparedSession`."]
    pub fn spec(&self) -> &SessionSpec {
        &self.spec
    }

    #[cfg(any(test, feature = "internal-testing"))]
    #[doc = "Returns the source mappings held by `PreparedSession`."]
    pub fn source_mappings(&self) -> &[PreparedSourceMapping] {
        &self.source_mappings
    }

    #[cfg(any(test, feature = "internal-testing"))]
    #[doc = "Returns the source input count held by `PreparedSession`."]
    pub fn source_input_count(&self) -> usize {
        self.source_inputs.len()
    }

    #[cfg(any(test, feature = "internal-testing"))]
    #[doc = "Returns the worker mappings held by `PreparedSession`."]
    pub fn worker_mappings(&self) -> &[PreparedWorkerMapping] {
        &self.worker_mappings
    }

    #[cfg(any(test, feature = "internal-testing"))]
    #[doc = "Returns the operator mappings held by `PreparedSession`."]
    pub fn operator_mappings(&self) -> &[PreparedOperatorMapping] {
        &self.operator_mappings
    }

    #[cfg(any(test, feature = "internal-testing"))]
    #[doc = "Returns the route observations held by `PreparedSession`."]
    pub fn route_observations(
        &self,
        route_id: RouteId,
    ) -> Option<crate::runtime::EdgeObservations> {
        let mapping = self
            .worker_mappings
            .iter()
            .find(|mapping| mapping.route_id == route_id)?;
        self.executor.observations(mapping.receiver.edge_id())
    }

    #[cfg(any(test, feature = "internal-testing"))]
    #[doc = "Returns whether cancellation requested is true for `PreparedSession`."]
    pub fn cancellation_requested(&self) -> bool {
        self.cancellation.is_requested()
    }
}
