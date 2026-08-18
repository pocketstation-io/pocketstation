use std::sync::Arc;
use std::time::Duration;

use crate::{
    EndpointAudioFrame, EndpointId, EndpointPortInput, EndpointPreparationGroup, EndpointReceiver,
    EndpointShutdownMode, MediaCaps, RouteId, SignalEnvelope, SignalSpec,
};

use super::{ConnectorContext, ConnectorRunOutcome, ConnectorWorker};
use crate::connector::{ConnectorError, ResolvedConnectorConfiguration};

const DRIVER_IDLE_WAIT: Duration = Duration::from_millis(1);

/// Immutable Session and graph metadata for one connector input.
#[derive(Debug, Clone)]
pub struct ConnectorInputDescriptor {
    endpoint_id: EndpointId,
    route_id: RouteId,
    port_name: String,
    signal: SignalSpec,
    media: MediaCaps,
    edge_contract: crate::EdgeContract,
    configuration: ResolvedConnectorConfiguration,
}

impl ConnectorInputDescriptor {
    pub const fn endpoint_id(&self) -> EndpointId {
        self.endpoint_id
    }

    pub const fn route_id(&self) -> RouteId {
        self.route_id
    }

    pub fn port_name(&self) -> &str {
        &self.port_name
    }

    pub const fn signal_spec(&self) -> &SignalSpec {
        &self.signal
    }

    pub const fn media(&self) -> MediaCaps {
        self.media
    }

    pub const fn edge_contract(&self) -> crate::EdgeContract {
        self.edge_contract
    }

    pub const fn configuration(&self) -> &ResolvedConnectorConfiguration {
        &self.configuration
    }
}

/// One bounded item delivered by Core to a connector driver.
pub enum ConnectorItem<'a> {
    Audio {
        input: &'a ConnectorInputDescriptor,
        frame: EndpointAudioFrame,
    },
    Signal {
        input: &'a ConnectorInputDescriptor,
        signal: Arc<SignalEnvelope>,
    },
}

impl ConnectorItem<'_> {
    pub const fn input(&self) -> &ConnectorInputDescriptor {
        match self {
            Self::Audio { input, .. } | Self::Signal { input, .. } => input,
        }
    }
}

/// Explicit delivery result used for Core-owned accounting.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectorDeliveryOutcome {
    Delivered,
    Dropped,
}

/// Provider-specific behavior executed on Core's bounded connector worker.
///
/// Calls occur outside realtime partitions. Implementations must keep every
/// provider operation finite and return structured errors rather than panic.
pub trait ConnectorDriver: Send + 'static {
    fn start(&mut self, context: &ConnectorContext) -> Result<(), ConnectorError> {
        let _ = context.set_ready();
        Ok(())
    }

    fn deliver(
        &mut self,
        item: ConnectorItem<'_>,
        context: &ConnectorContext,
    ) -> Result<ConnectorDeliveryOutcome, ConnectorError>;

    fn idle(&mut self, _context: &ConnectorContext) -> Result<(), ConnectorError> {
        Ok(())
    }

    fn shutdown(
        &mut self,
        _mode: EndpointShutdownMode,
        _context: &ConnectorContext,
    ) -> Result<(), ConnectorError> {
        Ok(())
    }

    fn cancel_preparation(self: Box<Self>) -> Result<(), ConnectorError> {
        drop(self);
        Ok(())
    }
}

/// Prepares provider state while Core retains receiver and lifecycle authority.
pub trait ConnectorDriverFactory: Send + Sync {
    fn preparation_group(
        &self,
        route_id: RouteId,
        _configuration: &ResolvedConnectorConfiguration,
    ) -> Result<EndpointPreparationGroup, ConnectorError> {
        Ok(EndpointPreparationGroup::Route(route_id))
    }

    fn prepare(
        &self,
        inputs: &[ConnectorInputDescriptor],
    ) -> Result<Box<dyn ConnectorDriver>, ConnectorError>;
}

pub(super) fn prepare_connector_driver(
    factory: &dyn ConnectorDriverFactory,
    inputs: Vec<EndpointPortInput>,
    configurations: Vec<ResolvedConnectorConfiguration>,
) -> Result<Box<dyn ConnectorWorker>, ConnectorError> {
    if inputs.len() != configurations.len() {
        return Err(super::supervisor::internal_connector_error(
            "core.configuration_count_mismatch",
            crate::connector::ConnectorErrorStage::Prepare,
            "connector input and resolved configuration counts differ",
        ));
    }

    let mut driver_inputs = Vec::with_capacity(inputs.len());
    for (input, configuration) in inputs.into_iter().zip(configurations) {
        let descriptor = ConnectorInputDescriptor {
            endpoint_id: input.context().endpoint_id(),
            route_id: input.context().route_context().route_id(),
            port_name: input.port_name().to_owned(),
            signal: input.signal_spec().clone(),
            media: *input.media(),
            edge_contract: *input.edge_contract(),
            configuration,
        };
        let (receiver, _) = input.into_parts();
        driver_inputs.push(DriverInput {
            descriptor,
            receiver,
            last_discontinuity_epoch: None,
        });
    }
    let descriptors = driver_inputs
        .iter()
        .map(|input| input.descriptor.clone())
        .collect::<Vec<_>>();
    let driver = factory.prepare(&descriptors)?;
    Ok(Box::new(PollingConnectorWorker {
        driver: Some(driver),
        inputs: driver_inputs,
    }))
}

struct DriverInput {
    descriptor: ConnectorInputDescriptor,
    receiver: EndpointReceiver,
    last_discontinuity_epoch: Option<u64>,
}

struct PollingConnectorWorker {
    driver: Option<Box<dyn ConnectorDriver>>,
    inputs: Vec<DriverInput>,
}

impl ConnectorWorker for PollingConnectorWorker {
    fn run(mut self: Box<Self>, context: ConnectorContext) -> ConnectorRunOutcome {
        let Some(mut driver) = self.driver.take() else {
            return ConnectorRunOutcome::failure(super::supervisor::internal_connector_error(
                "core.connector_driver_missing",
                crate::connector::ConnectorErrorStage::Startup,
                "connector driver ownership is unavailable",
            ));
        };
        if let Err(error) = driver.start(&context) {
            return ConnectorRunOutcome::failure(error);
        }

        loop {
            if context.is_abort_requested() {
                break;
            }
            let mut progressed = false;
            for input in &mut self.inputs {
                let item = match &mut input.receiver {
                    EndpointReceiver::Audio { receiver, .. } => receiver.try_recv().map(|frame| {
                        record_discontinuity(
                            &context,
                            &mut input.last_discontinuity_epoch,
                            frame.lineage().discontinuity_epoch(),
                        );
                        ConnectorItem::Audio {
                            input: &input.descriptor,
                            frame,
                        }
                    }),
                    EndpointReceiver::Signal(receiver) => receiver.try_recv().map(|signal| {
                        if let Some(lineage) = signal.lineage() {
                            record_discontinuity(
                                &context,
                                &mut input.last_discontinuity_epoch,
                                lineage.discontinuity_epoch(),
                            );
                        }
                        ConnectorItem::Signal {
                            input: &input.descriptor,
                            signal,
                        }
                    }),
                };
                let Some(item) = item else {
                    continue;
                };
                progressed = true;
                context.record_frame_received(1);
                match driver.deliver(item, &context) {
                    Ok(ConnectorDeliveryOutcome::Delivered) => {
                        context.record_frame_delivered(1);
                    }
                    Ok(ConnectorDeliveryOutcome::Dropped) => {
                        context.record_frame_dropped(1);
                    }
                    Err(error) => return ConnectorRunOutcome::failure(error),
                }
            }

            if progressed {
                continue;
            }
            if context.shutdown_mode() == Some(EndpointShutdownMode::Drain) {
                break;
            }
            if let Err(error) = driver.idle(&context) {
                return ConnectorRunOutcome::failure(error);
            }
            let _ = context.wait_for_stop(DRIVER_IDLE_WAIT);
        }

        let mode = context
            .shutdown_mode()
            .unwrap_or(EndpointShutdownMode::Abort);
        match driver.shutdown(mode, &context) {
            Ok(()) => ConnectorRunOutcome::success(),
            Err(error) => ConnectorRunOutcome::failure(error),
        }
    }

    fn cancel_preparation(mut self: Box<Self>) -> Result<(), ConnectorError> {
        self.driver
            .take()
            .map_or(Ok(()), ConnectorDriver::cancel_preparation)
    }
}

fn record_discontinuity(context: &ConnectorContext, previous: &mut Option<u64>, current: u64) {
    if previous.is_some_and(|value| value != current) {
        context.record_discontinuity(1);
    }
    *previous = Some(current);
}
