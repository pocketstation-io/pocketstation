use std::panic::{catch_unwind, AssertUnwindSafe};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};

use crate::endpoint::EndpointDriverObservationHandle;
use crate::{
    EndpointCancellationOutcome, EndpointDriverFactory, EndpointDriverFinalization,
    EndpointDriverObservations, EndpointFailure, EndpointFailureStage, EndpointPortInput,
    EndpointPreparationGroup, EndpointShutdownMode, EndpointStartGate, PreparedEndpointDriver,
    RunningEndpointDriver,
};

use super::coordination::{ConnectorStopToken, ReadinessProbeState};
use super::driver::{prepare_connector_driver, ConnectorDriverFactory};
use super::supervisor::{
    internal_connector_error, supervise_startup_readiness, wait_for_start_gate,
    ConnectorWorkerState,
};
use super::{ConnectorContext, ConnectorFactory, ConnectorWorker};
use crate::connector::{
    ConnectorError, ConnectorErrorStage, ConnectorManifest, ConnectorObservationHandle,
    ConnectorObservationStore, ConnectorReadinessPolicy,
};

pub(crate) fn connector_endpoint_factory(
    factory: Arc<dyn ConnectorFactory>,
    observations: ConnectorObservationStore,
    readiness_policy: ConnectorReadinessPolicy,
) -> Arc<dyn EndpointDriverFactory> {
    Arc::new(ConnectorEndpointAdapter {
        factory,
        observations,
        readiness_policy,
    })
}

pub(crate) fn connector_driver_endpoint_factory(
    factory: Arc<dyn ConnectorDriverFactory>,
    observations: ConnectorObservationStore,
    manifest: Arc<ConnectorManifest>,
) -> Arc<dyn EndpointDriverFactory> {
    Arc::new(ConnectorDriverEndpointAdapter {
        factory,
        observations,
        manifest,
    })
}

struct ConnectorDriverEndpointAdapter {
    factory: Arc<dyn ConnectorDriverFactory>,
    observations: ConnectorObservationStore,
    manifest: Arc<ConnectorManifest>,
}

impl EndpointDriverFactory for ConnectorDriverEndpointAdapter {
    fn preparation_group(
        &self,
        route_id: crate::RouteId,
        configuration: &crate::graph::NodeConfig,
    ) -> Result<EndpointPreparationGroup, EndpointFailure> {
        let configuration = self
            .manifest
            .configuration()
            .resolve_node_config(configuration)
            .map_err(configuration_endpoint_failure)?;
        self.factory
            .preparation_group(route_id, &configuration)
            .map_err(ConnectorError::into_endpoint_failure)
    }

    fn prepare(
        &self,
        inputs: Vec<EndpointPortInput>,
    ) -> Result<Box<dyn PreparedEndpointDriver>, EndpointFailure> {
        let endpoint_ids = endpoint_ids(&inputs)?;
        let connector_observations = ConnectorObservationHandle::new();
        let endpoint_observations = EndpointDriverObservationHandle::default();
        self.observations.install(
            endpoint_ids,
            connector_observations.clone(),
            endpoint_observations.clone(),
        );
        let configurations = inputs
            .iter()
            .map(|input| {
                self.manifest
                    .configuration()
                    .resolve_node_config(input.context().node_configuration())
            })
            .collect::<Result<Vec<_>, _>>()
            .map_err(configuration_endpoint_failure)?;
        match prepare_connector_driver(self.factory.as_ref(), inputs, configurations) {
            Ok(worker) => Ok(Box::new(PreparedConnectorWorker {
                worker: Some(worker),
                state: ConnectorWorkerState::new(connector_observations, endpoint_observations),
                readiness_policy: self.manifest.readiness(),
            })),
            Err(error) => {
                let state =
                    ConnectorWorkerState::new(connector_observations, endpoint_observations);
                state.record_terminal(error.clone());
                Err(error.into_endpoint_failure())
            }
        }
    }
}

fn configuration_endpoint_failure(
    error: crate::connector::ConnectorConfigurationError,
) -> EndpointFailure {
    EndpointFailure::new(
        EndpointFailureStage::Prepare,
        format!("{}: {error}", error.code().as_str()),
    )
}

struct ConnectorEndpointAdapter {
    factory: Arc<dyn ConnectorFactory>,
    observations: ConnectorObservationStore,
    readiness_policy: ConnectorReadinessPolicy,
}

impl EndpointDriverFactory for ConnectorEndpointAdapter {
    fn preparation_group(
        &self,
        route_id: crate::RouteId,
        configuration: &crate::graph::NodeConfig,
    ) -> Result<EndpointPreparationGroup, EndpointFailure> {
        self.factory
            .preparation_group(route_id, configuration)
            .map_err(ConnectorError::into_endpoint_failure)
    }

    fn prepare(
        &self,
        inputs: Vec<EndpointPortInput>,
    ) -> Result<Box<dyn PreparedEndpointDriver>, EndpointFailure> {
        let endpoint_ids = endpoint_ids(&inputs)?;
        let connector_observations = ConnectorObservationHandle::new();
        let endpoint_observations = EndpointDriverObservationHandle::default();
        self.observations.install(
            endpoint_ids,
            connector_observations.clone(),
            endpoint_observations.clone(),
        );
        match self.factory.prepare(inputs) {
            Ok(worker) => Ok(Box::new(PreparedConnectorWorker {
                worker: Some(worker),
                state: ConnectorWorkerState::new(connector_observations, endpoint_observations),
                readiness_policy: self.readiness_policy,
            })),
            Err(error) => {
                let state =
                    ConnectorWorkerState::new(connector_observations, endpoint_observations);
                state.record_terminal(error.clone());
                Err(error.into_endpoint_failure())
            }
        }
    }
}

fn endpoint_ids(inputs: &[EndpointPortInput]) -> Result<Arc<[crate::EndpointId]>, EndpointFailure> {
    let Some(first) = inputs.first() else {
        return Err(EndpointFailure::new(
            EndpointFailureStage::Prepare,
            "connector input batch cannot be empty",
        ));
    };
    let session_id = first.context().session_id();
    let mut endpoint_ids = Vec::with_capacity(inputs.len());
    for input in inputs {
        if input.context().session_id() != session_id {
            return Err(EndpointFailure::new(
                EndpointFailureStage::Prepare,
                "connector input batch spans multiple Sessions",
            ));
        }
        let endpoint_id = input.context().endpoint_id();
        if !endpoint_ids.contains(&endpoint_id) {
            endpoint_ids.push(endpoint_id);
        }
    }
    Ok(endpoint_ids.into())
}

struct PreparedConnectorWorker {
    worker: Option<Box<dyn ConnectorWorker>>,
    state: ConnectorWorkerState,
    readiness_policy: ConnectorReadinessPolicy,
}

impl PreparedEndpointDriver for PreparedConnectorWorker {
    fn start(
        mut self: Box<Self>,
        start_gate: Arc<EndpointStartGate>,
    ) -> Result<Box<dyn RunningEndpointDriver>, EndpointFailure> {
        let worker = self.worker.take().ok_or_else(|| {
            EndpointFailure::new(
                EndpointFailureStage::Start,
                "connector worker ownership was already consumed",
            )
        })?;
        let worker_slot = Arc::new(Mutex::new(Some(worker)));
        let stop = ConnectorStopToken::new();
        let readiness_probes = Arc::new(Mutex::new(ReadinessProbeState::default()));

        let worker_handle = spawn_worker(
            Arc::clone(&worker_slot),
            Arc::clone(&start_gate),
            stop.clone(),
            self.state.clone(),
            self.readiness_policy,
            Arc::clone(&readiness_probes),
        );
        let worker_handle = match worker_handle {
            Ok(handle) => handle,
            Err(error) => {
                if let Some(worker) = worker_slot
                    .lock()
                    .unwrap_or_else(|poisoned| poisoned.into_inner())
                    .take()
                {
                    let _ = cancel_worker(worker, &self.state);
                }
                self.state.record_terminal(error.clone());
                return Err(error.into_endpoint_failure());
            }
        };

        let watchdog_stop = stop.clone();
        let watchdog_state = self.state.clone();
        let watchdog_gate = Arc::clone(&start_gate);
        let readiness_policy = self.readiness_policy;
        let watchdog_handle = thread::Builder::new()
            .name("pks-connector-readiness".to_owned())
            .spawn(move || {
                supervise_startup_readiness(
                    &watchdog_stop,
                    &watchdog_state,
                    &watchdog_gate,
                    readiness_policy,
                );
            });
        let watchdog_handle = match watchdog_handle {
            Ok(handle) => handle,
            Err(error) => {
                stop.request(EndpointShutdownMode::Abort);
                let _ = worker_handle.join();
                let error = internal_connector_error(
                    "core.readiness_supervisor_spawn",
                    ConnectorErrorStage::Startup,
                    error.to_string(),
                );
                self.state.record_terminal(error.clone());
                return Err(error.into_endpoint_failure());
            }
        };

        Ok(Box::new(RunningConnectorWorker {
            stop,
            state: self.state.clone(),
            worker_handle: Some(worker_handle),
            watchdog_handle: Some(watchdog_handle),
        }))
    }

    fn cancel_preparation(mut self: Box<Self>) -> EndpointCancellationOutcome {
        self.state.mark_stopping();
        let result = self.worker.take().map_or(Ok(()), |worker| {
            cancel_worker(worker, &self.state).map_err(ConnectorError::into_endpoint_failure)
        });
        EndpointCancellationOutcome {
            observations: self.state.endpoint().snapshot(),
            result,
        }
    }
}

impl Drop for PreparedConnectorWorker {
    fn drop(&mut self) {
        if let Some(worker) = self.worker.take() {
            let _ = cancel_worker(worker, &self.state);
        }
    }
}

fn spawn_worker(
    worker_slot: Arc<Mutex<Option<Box<dyn ConnectorWorker>>>>,
    start_gate: Arc<EndpointStartGate>,
    stop: ConnectorStopToken,
    state: ConnectorWorkerState,
    readiness_policy: ConnectorReadinessPolicy,
    readiness_probes: Arc<Mutex<ReadinessProbeState>>,
) -> Result<JoinHandle<()>, ConnectorError> {
    thread::Builder::new()
        .name("pks-connector-worker".to_owned())
        .spawn(move || {
            let worker = worker_slot
                .lock()
                .unwrap_or_else(|poisoned| poisoned.into_inner())
                .take();
            let Some(worker) = worker else {
                state.record_terminal(internal_connector_error(
                    "core.worker_missing",
                    ConnectorErrorStage::Startup,
                    "connector worker ownership was unavailable",
                ));
                stop.request(EndpointShutdownMode::Abort);
                return;
            };
            if !wait_for_start_gate(&stop, &start_gate) {
                let _ = cancel_worker(worker, &state);
                return;
            }
            let context = ConnectorContext {
                stop: stop.clone(),
                state: state.clone(),
                readiness_policy,
                readiness_probes,
            };
            let outcome = catch_unwind(AssertUnwindSafe(|| worker.run(context)));
            match outcome {
                Ok(outcome) => match outcome.into_result() {
                    Ok(()) if stop.is_requested() => {}
                    Ok(()) => state.record_terminal(internal_connector_error(
                        "core.worker_exited",
                        ConnectorErrorStage::Delivery,
                        "connector worker exited before stop was requested",
                    )),
                    Err(error) => state.record_terminal(error),
                },
                Err(_) => state.record_terminal(internal_connector_error(
                    "core.worker_panic",
                    ConnectorErrorStage::Join,
                    "connector worker panicked",
                )),
            }
            stop.request(EndpointShutdownMode::Abort);
        })
        .map_err(|error| {
            internal_connector_error(
                "core.worker_spawn",
                ConnectorErrorStage::Startup,
                error.to_string(),
            )
        })
}

fn cancel_worker(
    worker: Box<dyn ConnectorWorker>,
    state: &ConnectorWorkerState,
) -> Result<(), ConnectorError> {
    match catch_unwind(AssertUnwindSafe(|| worker.cancel_preparation())) {
        Ok(result) => {
            if let Err(error) = &result {
                state.record_terminal(error.clone());
            }
            result
        }
        Err(_) => {
            let error = internal_connector_error(
                "core.preparation_cancel_panic",
                ConnectorErrorStage::Prepare,
                "connector preparation cancellation panicked",
            );
            state.record_terminal(error.clone());
            Err(error)
        }
    }
}

struct RunningConnectorWorker {
    stop: ConnectorStopToken,
    state: ConnectorWorkerState,
    worker_handle: Option<JoinHandle<()>>,
    watchdog_handle: Option<JoinHandle<()>>,
}

impl RunningConnectorWorker {
    fn stop_and_join(&mut self) {
        self.state.mark_stopping();
        if !self.stop.is_requested() {
            self.stop.request(EndpointShutdownMode::Abort);
        }
        if let Some(watchdog) = self.watchdog_handle.take() {
            if watchdog.join().is_err() {
                self.state.record_terminal(internal_connector_error(
                    "core.readiness_supervisor_panic",
                    ConnectorErrorStage::Join,
                    "connector readiness supervisor panicked",
                ));
            }
        }
        if let Some(worker) = self.worker_handle.take() {
            if worker.join().is_err() {
                self.state.record_terminal(internal_connector_error(
                    "core.worker_join_panic",
                    ConnectorErrorStage::Join,
                    "connector worker thread panicked outside its contained run callback",
                ));
            }
        }
    }
}

impl RunningEndpointDriver for RunningConnectorWorker {
    fn observations(&self) -> EndpointDriverObservations {
        self.state.endpoint().snapshot()
    }

    fn request_stop(&mut self) -> Result<(), EndpointFailure> {
        self.request_shutdown(EndpointShutdownMode::Drain)
    }

    fn request_shutdown(&mut self, mode: EndpointShutdownMode) -> Result<(), EndpointFailure> {
        self.state.mark_stopping();
        self.stop.request(mode);
        Ok(())
    }

    fn join_and_finalize(mut self: Box<Self>) -> EndpointDriverFinalization {
        self.stop_and_join();
        EndpointDriverFinalization {
            observations: self.state.endpoint().snapshot(),
            result: self
                .state
                .terminal_error()
                .map_or(Ok(()), |error| Err(error.into_endpoint_failure())),
        }
    }
}

impl Drop for RunningConnectorWorker {
    fn drop(&mut self) {
        self.stop_and_join();
    }
}
