pub use crate::conformance::{
    observed_connector, session, session_for_saturation, session_with_recording,
    session_with_recording_and_trace, session_with_trace, ObservedEndpointError,
    OBSERVED_CONNECTOR_OPERATOR_ID,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ConnectorConformanceCase {
    ManifestValidation,
    ConfigurationRejection,
    SecretRedaction,
    PreparationRollback,
    StartFailureRollback,
    StartGateIsolation,
    ReadinessTransitions,
    SaturationAccounting,
    Cancellation,
    Stop,
    JoinFinalizationFailure,
    WorkerPanicContainment,
}

pub const REQUIRED_CONNECTOR_CONFORMANCE_CASES: &[ConnectorConformanceCase] = &[
    ConnectorConformanceCase::ManifestValidation,
    ConnectorConformanceCase::ConfigurationRejection,
    ConnectorConformanceCase::SecretRedaction,
    ConnectorConformanceCase::PreparationRollback,
    ConnectorConformanceCase::StartFailureRollback,
    ConnectorConformanceCase::StartGateIsolation,
    ConnectorConformanceCase::ReadinessTransitions,
    ConnectorConformanceCase::SaturationAccounting,
    ConnectorConformanceCase::Cancellation,
    ConnectorConformanceCase::Stop,
    ConnectorConformanceCase::JoinFinalizationFailure,
    ConnectorConformanceCase::WorkerPanicContainment,
];
