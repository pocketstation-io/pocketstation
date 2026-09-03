//! Origin-independent endpoint preparation, grouping, and rollback.

use crate::endpoint::{
    EndpointDriverRegistry, EndpointPortInput, EndpointPrepareContext, EndpointRouteContext,
    PreparedEndpoint, SessionTimelineOrigin,
};
use crate::frame::{EndpointId, RouteId};
use crate::runtime::AsyncOperatorOutputObservationHandle;
use crate::session::prepare::{PreparedWorkerMapping, PreparedWorkerOrigin};
use crate::session::{
    SessionComponentId, SessionControlFailure, SessionRollbackFailure, SessionRollbackStage,
    SessionSpec, SessionStartError,
};

use super::rollback::StartupRollback;
use super::telemetry::DerivedRouteObservationBinding;

pub(super) struct PreparedEndpointBinding {
    pub(super) identities: Vec<(RouteId, EndpointId)>,
    pub(super) endpoint: PreparedEndpoint,
}

pub(super) struct PendingEndpointPortInput {
    pub(super) route_id: RouteId,
    pub(super) endpoint_id: EndpointId,
    pub(super) input: EndpointPortInput,
    pub(super) signal_observation: Option<AsyncOperatorOutputObservationHandle>,
}

pub(super) type EndpointBatchPreparation = (
    Vec<PreparedEndpointBinding>,
    Vec<DerivedRouteObservationBinding>,
);

pub(super) fn rollback_prepared_endpoints(
    endpoints: Vec<PreparedEndpointBinding>,
) -> StartupRollback {
    let mut rollback = StartupRollback::default();
    for binding in endpoints.into_iter().rev() {
        let outcome = binding.endpoint.cancel_preparation();
        if let Err(error) = outcome.result {
            for (route_id, endpoint_id) in binding.identities {
                rollback.failures.push(SessionRollbackFailure::new(
                    SessionRollbackStage::CancelEndpointPreparation,
                    SessionControlFailure::new(
                        SessionComponentId::Endpoint {
                            route_id,
                            endpoint_id,
                        },
                        "cancel_endpoint_preparation",
                        error.to_string(),
                    ),
                ));
            }
        }
    }
    rollback
}

pub(super) fn prepare_endpoint_batches(
    spec: &SessionSpec,
    mut pending: Vec<PendingEndpointPortInput>,
    endpoint_registry: &EndpointDriverRegistry,
) -> Result<EndpointBatchPreparation, (SessionStartError, Vec<SessionRollbackFailure>)> {
    let mut endpoints = Vec::with_capacity(pending.len());
    let mut signal_observations = Vec::new();
    while !pending.is_empty() {
        let first = pending.remove(0);
        let endpoint = spec
            .endpoints()
            .iter()
            .find(|endpoint| endpoint.id() == first.endpoint_id)
            .ok_or_else(|| {
                (
                    SessionStartError::MissingEndpointDeclaration {
                        endpoint_id: first.endpoint_id,
                    },
                    Vec::new(),
                )
            })?;
        let preparation_group = endpoint_registry
            .preparation_group(
                endpoint.operator_id(),
                endpoint.node_type_id(),
                first.route_id,
                first.input.context().node_configuration(),
            )
            .map_err(|source| {
                let rollback = rollback_prepared_endpoints(std::mem::take(&mut endpoints));
                (
                    SessionStartError::EndpointPrepare {
                        source,
                        rollback_failures_total: rollback.failures_total(),
                    },
                    rollback.failures,
                )
            })?;
        let mut grouped = vec![first];
        let mut index = 0;
        while index < pending.len() {
            let candidate = &pending[index];
            let Some(candidate_endpoint) = spec
                .endpoints()
                .iter()
                .find(|endpoint| endpoint.id() == candidate.endpoint_id)
            else {
                return Err((
                    SessionStartError::MissingEndpointDeclaration {
                        endpoint_id: candidate.endpoint_id,
                    },
                    Vec::new(),
                ));
            };
            let same_registration = candidate_endpoint.operator_id() == endpoint.operator_id()
                && candidate_endpoint.node_type_id() == endpoint.node_type_id();
            let same_group = same_registration
                && endpoint_registry
                    .preparation_group(
                        candidate_endpoint.operator_id(),
                        candidate_endpoint.node_type_id(),
                        candidate.route_id,
                        candidate.input.context().node_configuration(),
                    )
                    .map_err(|source| {
                        let rollback = rollback_prepared_endpoints(std::mem::take(&mut endpoints));
                        (
                            SessionStartError::EndpointPrepare {
                                source,
                                rollback_failures_total: rollback.failures_total(),
                            },
                            rollback.failures,
                        )
                    })?
                    == preparation_group;
            if same_group {
                grouped.push(pending.remove(index));
            } else {
                index += 1;
            }
        }
        let identities = grouped
            .iter()
            .map(|input| (input.route_id, input.endpoint_id))
            .collect();
        let inputs = grouped
            .into_iter()
            .map(|input| {
                if let Some(output) = input.signal_observation {
                    signal_observations.push(DerivedRouteObservationBinding {
                        route_id: input.route_id,
                        endpoint_id: input.endpoint_id,
                        output,
                    });
                }
                input.input
            })
            .collect();
        match endpoint_registry.prepare_batch(
            endpoint.operator_id(),
            endpoint.node_type_id(),
            inputs,
        ) {
            Ok(endpoint) => endpoints.push(PreparedEndpointBinding {
                identities,
                endpoint,
            }),
            Err(source) => {
                let rollback = rollback_prepared_endpoints(endpoints);
                return Err((
                    SessionStartError::EndpointPrepare {
                        source,
                        rollback_failures_total: rollback.failures_total(),
                    },
                    rollback.failures,
                ));
            }
        }
    }
    Ok((endpoints, signal_observations))
}

pub(super) fn prepare_endpoints(
    spec: &SessionSpec,
    worker_mappings: Vec<PreparedWorkerMapping>,
    endpoint_registry: &EndpointDriverRegistry,
    session_timeline_origin: SessionTimelineOrigin,
) -> Result<Vec<PreparedEndpointBinding>, (SessionStartError, Vec<SessionRollbackFailure>)> {
    let session_id = spec.session_id();
    let mut pending = Vec::with_capacity(worker_mappings.len());
    for mapping in worker_mappings {
        let endpoint = spec
            .endpoints()
            .iter()
            .find(|endpoint| endpoint.id() == mapping.endpoint_id)
            .ok_or_else(|| {
                (
                    SessionStartError::MissingEndpointDeclaration {
                        endpoint_id: mapping.endpoint_id,
                    },
                    Vec::new(),
                )
            })?;
        if let PreparedWorkerOrigin::Stem(stem_id) = mapping.origin {
            if !spec
                .stems()
                .iter()
                .any(|candidate| candidate.id() == stem_id)
            {
                return Err((SessionStartError::UnsupportedSourceTopology, Vec::new()));
            }
        }
        let route_context = match mapping.origin {
            PreparedWorkerOrigin::Stem(stem_id) => {
                EndpointRouteContext::from_stem(mapping.route_id, stem_id)
            }
            PreparedWorkerOrigin::SignalIngress {
                stem_id,
                source_id,
                stream_id,
            } => EndpointRouteContext::from_source(
                mapping.route_id,
                source_id,
                stream_id,
                Some(stem_id),
            ),
        };
        let context = EndpointPrepareContext::new(
            session_id,
            endpoint.id(),
            route_context,
            session_timeline_origin,
            mapping.node_configuration,
        )
        .with_connector_id(mapping.connector_id);
        pending.push(PendingEndpointPortInput {
            route_id: mapping.route_id,
            endpoint_id: mapping.endpoint_id,
            input: EndpointPortInput::audio(
                mapping.input_port,
                mapping.signal_spec,
                mapping.media,
                mapping.route_settings,
                mapping.receiver,
                mapping.prepare_context,
                context,
            ),
            signal_observation: None,
        });
    }
    let (endpoints, signal_observations) =
        prepare_endpoint_batches(spec, pending, endpoint_registry)?;
    debug_assert!(signal_observations.is_empty());
    Ok(endpoints)
}
