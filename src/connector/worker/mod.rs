mod coordination;
mod driver;
mod endpoint_adapter;
mod supervisor;

use crate::{EndpointPortInput, EndpointPreparationGroup};

use super::ConnectorError;

pub use coordination::ConnectorContext;
pub use driver::{
    ConnectorDeliveryOutcome, ConnectorDriver, ConnectorDriverFactory, ConnectorInputDescriptor,
    ConnectorItem,
};
pub(crate) use endpoint_adapter::{connector_driver_endpoint_factory, connector_endpoint_factory};

#[doc = "Defines the implementation contract for connector."]
pub trait ConnectorFactory: Send + Sync {
    #[doc = "Returns the preparation group associated with `ConnectorFactory`."]
    fn preparation_group(
        &self,
        route_id: crate::RouteId,
        _configuration: &crate::graph::NodeConfig,
    ) -> Result<EndpointPreparationGroup, ConnectorError> {
        Ok(EndpointPreparationGroup::Route(route_id))
    }

    #[doc = "Prepares resources required by `ConnectorFactory`."]
    fn prepare(
        &self,
        inputs: Vec<EndpointPortInput>,
    ) -> Result<Box<dyn ConnectorWorker>, ConnectorError>;
}

#[doc = "Defines the implementation contract for connector worker."]
pub trait ConnectorWorker: Send + 'static {
    #[doc = "Runs `ConnectorWorker` until completion or cancellation."]
    fn run(self: Box<Self>, context: ConnectorContext) -> ConnectorRunOutcome;

    #[doc = "Cancels preparation for `ConnectorWorker`."]
    fn cancel_preparation(self: Box<Self>) -> Result<(), ConnectorError> {
        drop(self);
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "Reports the structured connector run outcome."]
pub struct ConnectorRunOutcome {
    result: Result<(), ConnectorError>,
}

impl ConnectorRunOutcome {
    #[doc = "Creates a new `ConnectorRunOutcome`."]
    pub const fn new(result: Result<(), ConnectorError>) -> Self {
        Self { result }
    }

    #[doc = "Returns whether `ConnectorRunOutcome` completed successfully."]
    pub const fn success() -> Self {
        Self::new(Ok(()))
    }

    #[doc = "Returns the failure associated with `ConnectorRunOutcome`."]
    pub fn failure(error: ConnectorError) -> Self {
        Self::new(Err(error))
    }

    #[doc = "Returns the result represented by `ConnectorRunOutcome`."]
    pub fn result(&self) -> &Result<(), ConnectorError> {
        &self.result
    }

    pub(crate) fn into_result(self) -> Result<(), ConnectorError> {
        self.result
    }
}
