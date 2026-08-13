//! Reverse-order startup rollback accounting.

use crate::session::SessionRollbackFailure;

#[derive(Default)]
pub(super) struct StartupRollback {
    pub(super) failures: Vec<SessionRollbackFailure>,
}

impl StartupRollback {
    pub(super) fn failures_total(&self) -> u64 {
        self.failures.len() as u64
    }

    pub(super) fn append(&mut self, mut other: Self) {
        self.failures.append(&mut other.failures);
    }
}
