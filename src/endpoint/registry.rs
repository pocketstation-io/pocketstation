use std::collections::HashMap;
use std::sync::Arc;

use crate::graph::NodeTypeId;
use crate::runtime::{AsyncOperatorOutput, PlanEdgeReceiver};

use crate::endpoint::{
    EndpointFailure, EndpointPrepareContext, OperatorId, PreparedEndpoint, PreparedEndpointDriver,
};

pub struct EndpointDriverInput {
    receiver: PlanEdgeReceiver,
    context: EndpointPrepareContext,
}

impl EndpointDriverInput {
    pub fn new(receiver: PlanEdgeReceiver, context: EndpointPrepareContext) -> Self {
        Self { receiver, context }
    }

    pub const fn context(&self) -> &EndpointPrepareContext {
        &self.context
    }

    pub fn into_parts(self) -> (PlanEdgeReceiver, EndpointPrepareContext) {
        (self.receiver, self.context)
    }
}

pub struct DerivedEndpointDriverInput {
    receiver: AsyncOperatorOutput,
    context: EndpointPrepareContext,
}

impl DerivedEndpointDriverInput {
    pub fn new(receiver: AsyncOperatorOutput, context: EndpointPrepareContext) -> Self {
        Self { receiver, context }
    }

    pub const fn context(&self) -> &EndpointPrepareContext {
        &self.context
    }

    pub fn into_parts(self) -> (AsyncOperatorOutput, EndpointPrepareContext) {
        (self.receiver, self.context)
    }
}

pub trait EndpointDriverFactory: Send + Sync {
    fn prepare(
        &self,
        inputs: Vec<EndpointDriverInput>,
    ) -> Result<Box<dyn PreparedEndpointDriver>, EndpointFailure>;

    fn prepare_derived(
        &self,
        _inputs: Vec<DerivedEndpointDriverInput>,
    ) -> Result<Box<dyn PreparedEndpointDriver>, EndpointFailure> {
        Err(EndpointFailure::new(
            crate::endpoint::EndpointFailureStage::Prepare,
            "endpoint driver does not accept typed derived signals",
        ))
    }
}

struct RegisteredEndpointDriver {
    node_type_id: NodeTypeId,
    factory: Arc<dyn EndpointDriverFactory>,
}

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum EndpointDriverRegistryError {
    #[error("endpoint operator id cannot be empty")]
    EmptyOperatorId,
    #[error("endpoint node type id cannot be empty")]
    EmptyNodeTypeId,
    #[error(
        "endpoint driver already registered for operator '{operator_id}' and node type '{node_type_id}'"
    )]
    Duplicate {
        operator_id: String,
        node_type_id: String,
    },
    #[error(
        "endpoint operator '{operator_id}' is already mapped to node type '{registered_node_type_id}', not '{requested_node_type_id}'"
    )]
    OperatorNodeTypeConflict {
        operator_id: String,
        registered_node_type_id: String,
        requested_node_type_id: String,
    },
}

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum EndpointPrepareError {
    #[error("endpoint driver batch cannot be empty")]
    EmptyBatch,
    #[error(
        "no endpoint driver registered for operator '{operator_id}' and node type '{node_type_id}'"
    )]
    NotRegistered {
        operator_id: String,
        node_type_id: String,
    },
    #[error(transparent)]
    Driver(#[from] EndpointFailure),
}

#[derive(Default)]
pub struct EndpointDriverRegistry {
    registrations: HashMap<OperatorId, RegisteredEndpointDriver>,
}

impl EndpointDriverRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(
        &mut self,
        operator_id: OperatorId,
        node_type_id: NodeTypeId,
        factory: Arc<dyn EndpointDriverFactory>,
    ) -> Result<(), EndpointDriverRegistryError> {
        if operator_id.as_str().trim().is_empty() {
            return Err(EndpointDriverRegistryError::EmptyOperatorId);
        }
        if node_type_id.as_str().trim().is_empty() {
            return Err(EndpointDriverRegistryError::EmptyNodeTypeId);
        }
        if let Some(registered) = self.registrations.get(&operator_id) {
            if registered.node_type_id == node_type_id {
                return Err(EndpointDriverRegistryError::Duplicate {
                    operator_id: operator_id.as_str().to_owned(),
                    node_type_id: node_type_id.as_str().to_owned(),
                });
            }
            return Err(EndpointDriverRegistryError::OperatorNodeTypeConflict {
                operator_id: operator_id.as_str().to_owned(),
                registered_node_type_id: registered.node_type_id.as_str().to_owned(),
                requested_node_type_id: node_type_id.as_str().to_owned(),
            });
        }
        self.registrations.insert(
            operator_id,
            RegisteredEndpointDriver {
                node_type_id,
                factory,
            },
        );
        Ok(())
    }

    pub fn contains(&self, operator_id: &OperatorId, node_type_id: &NodeTypeId) -> bool {
        self.node_type_id(operator_id) == Some(node_type_id)
    }

    pub fn node_type_id(&self, operator_id: &OperatorId) -> Option<&NodeTypeId> {
        self.registrations
            .get(operator_id)
            .map(|registration| &registration.node_type_id)
    }

    pub fn prepare(
        &self,
        operator_id: &OperatorId,
        node_type_id: &NodeTypeId,
        receiver: PlanEdgeReceiver,
        context: &EndpointPrepareContext,
    ) -> Result<PreparedEndpoint, EndpointPrepareError> {
        self.prepare_batch(
            operator_id,
            node_type_id,
            vec![EndpointDriverInput::new(receiver, context.clone())],
        )
    }

    pub fn prepare_batch(
        &self,
        operator_id: &OperatorId,
        node_type_id: &NodeTypeId,
        inputs: Vec<EndpointDriverInput>,
    ) -> Result<PreparedEndpoint, EndpointPrepareError> {
        if inputs.is_empty() {
            return Err(EndpointPrepareError::EmptyBatch);
        }
        let registration = self
            .registrations
            .get(operator_id)
            .filter(|registration| &registration.node_type_id == node_type_id)
            .ok_or_else(|| EndpointPrepareError::NotRegistered {
                operator_id: operator_id.as_str().to_owned(),
                node_type_id: node_type_id.as_str().to_owned(),
            })?;
        Ok(PreparedEndpoint {
            driver: registration.factory.prepare(inputs)?,
        })
    }

    pub fn prepare_derived_batch(
        &self,
        operator_id: &OperatorId,
        node_type_id: &NodeTypeId,
        inputs: Vec<DerivedEndpointDriverInput>,
    ) -> Result<PreparedEndpoint, EndpointPrepareError> {
        if inputs.is_empty() {
            return Err(EndpointPrepareError::EmptyBatch);
        }
        let registration = self
            .registrations
            .get(operator_id)
            .filter(|registration| &registration.node_type_id == node_type_id)
            .ok_or_else(|| EndpointPrepareError::NotRegistered {
                operator_id: operator_id.as_str().to_owned(),
                node_type_id: node_type_id.as_str().to_owned(),
            })?;
        Ok(PreparedEndpoint {
            driver: registration.factory.prepare_derived(inputs)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};

    use crate::frame::{EndpointId, SampleFormat, SampleSpec, SessionId};
    use crate::graph::compiler::Compiler;
    use crate::graph::dsl::Pipeline;
    use crate::graph::node::{NodeConfig, PrepareContext};
    use crate::graph::planner::RuntimePlanner;
    use crate::graph::register_builtins;
    use crate::graph::registry::NodeRegistry;
    use crate::runtime::PlanEdgeRouter;

    use super::*;
    use crate::endpoint::{
        endpoint_start_gate, EndpointCancellationOutcome, EndpointDriverFinalization,
        EndpointDriverObservations, EndpointFailureStage, EndpointFinalizationOutcome,
        EndpointStartFailureCause, EndpointStartGate, RunningEndpointDriver,
    };

    struct TestDriverControl {
        prepare_calls_total: AtomicU64,
        cancel_calls_total: AtomicU64,
        start_calls_total: AtomicU64,
        request_stop_calls_total: AtomicU64,
        join_finalize_calls_total: AtomicU64,
        fail_prepare: AtomicBool,
        fail_request_stop: AtomicBool,
        fail_join_finalize: AtomicBool,
    }

    impl TestDriverControl {
        fn new() -> Arc<Self> {
            Arc::new(Self {
                prepare_calls_total: AtomicU64::new(0),
                cancel_calls_total: AtomicU64::new(0),
                start_calls_total: AtomicU64::new(0),
                request_stop_calls_total: AtomicU64::new(0),
                join_finalize_calls_total: AtomicU64::new(0),
                fail_prepare: AtomicBool::new(false),
                fail_request_stop: AtomicBool::new(false),
                fail_join_finalize: AtomicBool::new(false),
            })
        }
    }

    struct TestDriverFactory {
        control: Arc<TestDriverControl>,
    }

    struct TestPreparedDriver {
        control: Arc<TestDriverControl>,
        _receiver: PlanEdgeReceiver,
    }

    struct TestRunningDriver {
        control: Arc<TestDriverControl>,
        _receiver: PlanEdgeReceiver,
        start_gate: Arc<EndpointStartGate>,
    }

    impl EndpointDriverFactory for TestDriverFactory {
        fn prepare(
            &self,
            mut inputs: Vec<EndpointDriverInput>,
        ) -> Result<Box<dyn PreparedEndpointDriver>, EndpointFailure> {
            self.control
                .prepare_calls_total
                .fetch_add(1, Ordering::Relaxed);
            if self.control.fail_prepare.load(Ordering::Acquire) {
                return Err(EndpointFailure::new(
                    EndpointFailureStage::Prepare,
                    "test prepare failure",
                ));
            }
            let (receiver, _context) = inputs
                .pop()
                .expect("single-input registry path must supply one input")
                .into_parts();
            Ok(Box::new(TestPreparedDriver {
                control: Arc::clone(&self.control),
                _receiver: receiver,
            }))
        }
    }

    impl PreparedEndpointDriver for TestPreparedDriver {
        fn start(
            self: Box<Self>,
            start_gate: Arc<EndpointStartGate>,
        ) -> Result<Box<dyn RunningEndpointDriver>, EndpointFailure> {
            self.control
                .start_calls_total
                .fetch_add(1, Ordering::Relaxed);
            let Self { control, _receiver } = *self;
            Ok(Box::new(TestRunningDriver {
                control,
                _receiver,
                start_gate,
            }))
        }

        fn cancel_preparation(self: Box<Self>) -> EndpointCancellationOutcome {
            self.control
                .cancel_calls_total
                .fetch_add(1, Ordering::Relaxed);
            EndpointCancellationOutcome {
                observations: observations(),
                result: Ok(()),
            }
        }
    }

    impl RunningEndpointDriver for TestRunningDriver {
        fn observations(&self) -> EndpointDriverObservations {
            if self.start_gate.is_open() {
                observations()
            } else {
                EndpointDriverObservations::default()
            }
        }

        fn request_stop(&mut self) -> Result<(), EndpointFailure> {
            self.control
                .request_stop_calls_total
                .fetch_add(1, Ordering::Relaxed);
            if self.control.fail_request_stop.load(Ordering::Acquire) {
                Err(EndpointFailure::new(
                    EndpointFailureStage::RequestStop,
                    "test stop failure",
                ))
            } else {
                Ok(())
            }
        }

        fn join_and_finalize(self: Box<Self>) -> EndpointDriverFinalization {
            self.control
                .join_finalize_calls_total
                .fetch_add(1, Ordering::Relaxed);
            let result = if self.control.fail_join_finalize.load(Ordering::Acquire) {
                Err(EndpointFailure::new(
                    EndpointFailureStage::JoinFinalize,
                    "test join/finalize failure",
                ))
            } else {
                Ok(())
            };
            EndpointDriverFinalization {
                observations: observations(),
                result,
            }
        }
    }

    fn observations() -> EndpointDriverObservations {
        EndpointDriverObservations {
            frames_received_total: 5,
            frames_delivered_total: 4,
            frames_dropped_total: 1,
            discontinuities_total: 2,
            failures_total: 3,
        }
    }

    fn receiver() -> PlanEdgeReceiver {
        let mut node_registry = NodeRegistry::new();
        register_builtins(&mut node_registry).unwrap();
        let mut pipeline = Pipeline::new();
        let source = pipeline.add_node("passthrough", NodeConfig::new());
        let endpoint = pipeline.add_node("passthrough", NodeConfig::new());
        pipeline.connect(source.out("out"), endpoint.in_("in"));
        let ir = Compiler::new()
            .compile(pipeline.into_spec(), &node_registry)
            .unwrap();
        let plan = RuntimePlanner::new().plan(&ir).unwrap();
        let (_router, mut receivers) = PlanEdgeRouter::new(&plan, &ir).unwrap();
        receivers.pop().unwrap()
    }

    fn context() -> EndpointPrepareContext {
        EndpointPrepareContext::new(
            SessionId(7),
            EndpointId(8),
            NodeConfig::new().with("target", "test"),
            PrepareContext::new(SampleSpec::new(48_000, 1, SampleFormat::F32Interleaved)),
        )
    }

    fn registry_with(
        operator_id: &OperatorId,
        node_type_id: &NodeTypeId,
        control: &Arc<TestDriverControl>,
    ) -> EndpointDriverRegistry {
        let mut registry = EndpointDriverRegistry::new();
        registry
            .register(
                operator_id.clone(),
                node_type_id.clone(),
                Arc::new(TestDriverFactory {
                    control: Arc::clone(control),
                }),
            )
            .unwrap();
        registry
    }

    #[test]
    fn given_prior_preparation_when_next_endpoint_fails_then_prior_endpoint_rolls_back_explicitly()
    {
        let operator_id = OperatorId::new("connector.test");
        let node_type_id = NodeTypeId::from("endpoint.connector");
        let first_control = TestDriverControl::new();
        let failing_control = TestDriverControl::new();
        failing_control.fail_prepare.store(true, Ordering::Release);
        let first_registry = registry_with(&operator_id, &node_type_id, &first_control);
        let failing_registry = registry_with(&operator_id, &node_type_id, &failing_control);
        let first = first_registry
            .prepare(&operator_id, &node_type_id, receiver(), &context())
            .unwrap();

        let failure = failing_registry.prepare(&operator_id, &node_type_id, receiver(), &context());
        let rollback = first.cancel_preparation();

        assert!(matches!(
            failure,
            Err(EndpointPrepareError::Driver(EndpointFailure { .. }))
        ));
        assert!(rollback.result.is_ok());
        assert_eq!(rollback.observations, observations());
        assert_eq!(first_control.cancel_calls_total.load(Ordering::Relaxed), 1);
    }

    #[test]
    fn given_closed_start_gate_when_endpoint_starts_then_delivery_waits_until_session_opens_gate() {
        let operator_id = OperatorId::new("connector.test");
        let node_type_id = NodeTypeId::from("endpoint.connector");
        let control = TestDriverControl::new();
        let registry = registry_with(&operator_id, &node_type_id, &control);
        let prepared = registry
            .prepare(&operator_id, &node_type_id, receiver(), &context())
            .unwrap();
        let (gate_controller, gate) = endpoint_start_gate();

        let running = prepared.start(Arc::clone(&gate)).unwrap();

        assert_eq!(control.start_calls_total.load(Ordering::Relaxed), 1);
        assert_eq!(
            running.observations(),
            EndpointDriverObservations::default()
        );
        assert!(gate_controller.open());
        assert_eq!(running.observations(), observations());
    }

    #[test]
    fn given_already_open_start_gate_when_endpoint_start_requested_then_start_fails_recoverably() {
        let operator_id = OperatorId::new("connector.test");
        let node_type_id = NodeTypeId::from("endpoint.connector");
        let control = TestDriverControl::new();
        let registry = registry_with(&operator_id, &node_type_id, &control);
        let prepared = registry
            .prepare(&operator_id, &node_type_id, receiver(), &context())
            .unwrap();
        let (gate_controller, gate) = endpoint_start_gate();
        gate_controller.open();

        let failure = match prepared.start(gate) {
            Ok(_) => panic!("an already-open gate must reject endpoint start"),
            Err(failure) => failure,
        };

        assert_eq!(failure.cause(), &EndpointStartFailureCause::GateAlreadyOpen);
        assert_eq!(control.start_calls_total.load(Ordering::Relaxed), 0);
        assert!(failure.into_prepared().is_some());
    }

    #[test]
    fn given_stop_and_join_failures_when_finalized_then_both_failures_and_observations_remain_true()
    {
        let operator_id = OperatorId::new("connector.test");
        let node_type_id = NodeTypeId::from("endpoint.connector");
        let control = TestDriverControl::new();
        control.fail_request_stop.store(true, Ordering::Release);
        control.fail_join_finalize.store(true, Ordering::Release);
        let registry = registry_with(&operator_id, &node_type_id, &control);
        let prepared = registry
            .prepare(&operator_id, &node_type_id, receiver(), &context())
            .unwrap();
        let (gate_controller, gate) = endpoint_start_gate();
        let mut running = prepared.start(gate).unwrap();
        gate_controller.open();

        assert_eq!(running.observations(), observations());
        assert!(running.request_stop().is_err());
        assert!(running.request_stop().is_err());
        let outcome: EndpointFinalizationOutcome = running.join_and_finalize();

        assert!(!outcome.is_success());
        assert_eq!(outcome.observations, observations());
        assert_eq!(
            outcome.request_stop_result.unwrap_err().stage(),
            EndpointFailureStage::RequestStop
        );
        assert_eq!(
            outcome.join_finalize_result.unwrap_err().stage(),
            EndpointFailureStage::JoinFinalize
        );
        assert_eq!(control.request_stop_calls_total.load(Ordering::Relaxed), 1);
        assert_eq!(control.join_finalize_calls_total.load(Ordering::Relaxed), 1);
    }

    #[test]
    fn given_exact_operator_and_node_pair_when_resolved_then_other_pair_is_not_substituted() {
        let operator_id = OperatorId::new("connector.test");
        let node_type_id = NodeTypeId::from("endpoint.connector");
        let control = TestDriverControl::new();
        let registry = registry_with(&operator_id, &node_type_id, &control);

        assert!(registry.contains(&operator_id, &node_type_id));
        assert!(!registry.contains(&OperatorId::new("connector.other"), &node_type_id));
        assert!(matches!(
            registry.prepare(
                &OperatorId::new("connector.other"),
                &node_type_id,
                receiver(),
                &context()
            ),
            Err(EndpointPrepareError::NotRegistered { .. })
        ));
    }

    #[test]
    fn given_registered_operator_when_other_node_is_requested_then_conflict_preserves_first_binding(
    ) {
        let operator_id = OperatorId::new("connector.test");
        let registered_node_type_id = NodeTypeId::from("endpoint.connector");
        let requested_node_type_id = NodeTypeId::from("endpoint.browser");
        let first_control = TestDriverControl::new();
        let second_control = TestDriverControl::new();
        let mut registry = registry_with(&operator_id, &registered_node_type_id, &first_control);

        let result = registry.register(
            operator_id.clone(),
            requested_node_type_id.clone(),
            Arc::new(TestDriverFactory {
                control: second_control,
            }),
        );

        assert_eq!(
            result,
            Err(EndpointDriverRegistryError::OperatorNodeTypeConflict {
                operator_id: operator_id.as_str().to_owned(),
                registered_node_type_id: registered_node_type_id.as_str().to_owned(),
                requested_node_type_id: requested_node_type_id.as_str().to_owned(),
            })
        );
        assert_eq!(
            registry.node_type_id(&operator_id),
            Some(&registered_node_type_id)
        );
        assert!(registry.contains(&operator_id, &registered_node_type_id));
        assert!(!registry.contains(&operator_id, &requested_node_type_id));
    }
}
