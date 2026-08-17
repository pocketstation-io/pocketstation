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
    PreparationCancellation,
    StartGateIsolation,
    ReadinessDeadline,
    ServiceStatusReporting,
    SaturationAccounting,
    StopRequest,
    JoinedShutdown,
    WorkerFailurePropagation,
    WorkerPanicContainment,
}

pub const REQUIRED_CONNECTOR_CONFORMANCE_CASES: &[ConnectorConformanceCase] = &[
    ConnectorConformanceCase::ManifestValidation,
    ConnectorConformanceCase::ConfigurationRejection,
    ConnectorConformanceCase::SecretRedaction,
    ConnectorConformanceCase::PreparationRollback,
    ConnectorConformanceCase::PreparationCancellation,
    ConnectorConformanceCase::StartGateIsolation,
    ConnectorConformanceCase::ReadinessDeadline,
    ConnectorConformanceCase::ServiceStatusReporting,
    ConnectorConformanceCase::SaturationAccounting,
    ConnectorConformanceCase::StopRequest,
    ConnectorConformanceCase::JoinedShutdown,
    ConnectorConformanceCase::WorkerFailurePropagation,
    ConnectorConformanceCase::WorkerPanicContainment,
];

pub trait ConnectorConformanceCaseRunner {
    type Error: std::fmt::Display;

    fn run_case(&mut self, case: ConnectorConformanceCase) -> Result<(), Self::Error>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConnectorConformanceFailure {
    pub case: ConnectorConformanceCase,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConnectorConformanceReport {
    executed: Vec<ConnectorConformanceCase>,
    failures: Vec<ConnectorConformanceFailure>,
}

impl ConnectorConformanceReport {
    pub fn executed(&self) -> &[ConnectorConformanceCase] {
        &self.executed
    }

    pub fn failures(&self) -> &[ConnectorConformanceFailure] {
        &self.failures
    }

    pub fn is_success(&self) -> bool {
        self.executed.as_slice() == REQUIRED_CONNECTOR_CONFORMANCE_CASES && self.failures.is_empty()
    }

    pub fn require_success(self) -> Result<(), ConnectorConformanceReportError> {
        if self.is_success() {
            Ok(())
        } else {
            Err(ConnectorConformanceReportError {
                failure_count: self.failures.len(),
                executed_count: self.executed.len(),
                report: self,
            })
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("connector conformance failed in {failure_count} of {executed_count} required cases")]
pub struct ConnectorConformanceReportError {
    failure_count: usize,
    executed_count: usize,
    report: ConnectorConformanceReport,
}

impl ConnectorConformanceReportError {
    pub fn report(&self) -> &ConnectorConformanceReport {
        &self.report
    }
}

pub fn run_required_connector_conformance<R>(runner: &mut R) -> ConnectorConformanceReport
where
    R: ConnectorConformanceCaseRunner,
{
    let mut executed = Vec::with_capacity(REQUIRED_CONNECTOR_CONFORMANCE_CASES.len());
    let mut failures = Vec::new();
    for case in REQUIRED_CONNECTOR_CONFORMANCE_CASES.iter().copied() {
        executed.push(case);
        if let Err(error) = runner.run_case(case) {
            failures.push(ConnectorConformanceFailure {
                case,
                message: bounded_message(error.to_string()),
            });
        }
    }
    ConnectorConformanceReport { executed, failures }
}

fn bounded_message(mut message: String) -> String {
    const MAX_BYTES: usize = 4_096;
    if message.len() <= MAX_BYTES {
        return message;
    }
    let mut boundary = MAX_BYTES;
    while boundary > 0 && !message.is_char_boundary(boundary) {
        boundary -= 1;
    }
    message.truncate(boundary);
    message
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Runner {
        failed: Option<ConnectorConformanceCase>,
    }

    impl ConnectorConformanceCaseRunner for Runner {
        type Error = &'static str;

        fn run_case(&mut self, case: ConnectorConformanceCase) -> Result<(), Self::Error> {
            if self.failed == Some(case) {
                Err("injected failure")
            } else {
                Ok(())
            }
        }
    }

    #[test]
    fn given_required_suite_when_executed_then_every_case_and_failure_are_reported() {
        let mut passing = Runner { failed: None };
        let report = run_required_connector_conformance(&mut passing);
        assert!(report.is_success());
        assert_eq!(report.executed(), REQUIRED_CONNECTOR_CONFORMANCE_CASES);

        let mut failing = Runner {
            failed: Some(ConnectorConformanceCase::WorkerPanicContainment),
        };
        let error = run_required_connector_conformance(&mut failing)
            .require_success()
            .expect_err("failure must remain visible");
        assert_eq!(error.report().failures().len(), 1);
        assert_eq!(
            error.report().failures()[0].case,
            ConnectorConformanceCase::WorkerPanicContainment
        );
    }
}
