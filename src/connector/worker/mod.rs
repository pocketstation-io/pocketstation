mod context;
mod driver;
mod endpoint_adapter;
mod supervisor;

use crate::{EndpointPortInput, EndpointPreparationGroup};

use super::ConnectorError;

pub use context::ConnectorContext;
pub use driver::{
    ConnectorDeliveryOutcome, ConnectorDriver, ConnectorDriverFactory, ConnectorInputDescriptor,
    ConnectorItem,
};
pub(crate) use endpoint_adapter::{connector_driver_endpoint_factory, connector_endpoint_factory};

pub trait ConnectorFactory: Send + Sync {
    fn preparation_group(
        &self,
        route_id: crate::RouteId,
        _configuration: &crate::graph::NodeConfig,
    ) -> Result<EndpointPreparationGroup, ConnectorError> {
        Ok(EndpointPreparationGroup::Route(route_id))
    }

    fn prepare(
        &self,
        inputs: Vec<EndpointPortInput>,
    ) -> Result<Box<dyn ConnectorWorker>, ConnectorError>;
}

pub trait ConnectorWorker: Send + 'static {
    fn run(self: Box<Self>, context: ConnectorContext) -> ConnectorRunOutcome;

    fn cancel_preparation(self: Box<Self>) -> Result<(), ConnectorError> {
        drop(self);
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConnectorRunOutcome {
    result: Result<(), ConnectorError>,
}

impl ConnectorRunOutcome {
    pub const fn new(result: Result<(), ConnectorError>) -> Self {
        Self { result }
    }

    pub const fn success() -> Self {
        Self::new(Ok(()))
    }

    pub fn failure(error: ConnectorError) -> Self {
        Self::new(Err(error))
    }

    pub fn result(&self) -> &Result<(), ConnectorError> {
        &self.result
    }

    pub(crate) fn into_result(self) -> Result<(), ConnectorError> {
        self.result
    }
}
