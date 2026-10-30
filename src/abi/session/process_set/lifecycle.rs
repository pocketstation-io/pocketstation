//! Retained ownership after uncertain native closure; no realtime work occurs here.

use super::*;
use crate::abi::session::process_set::records::PROCESS_SET_CLEANUP_UNCONFIRMED;

impl EngineRuntime {
    pub(super) fn check_open(&self) -> Result<(), AbiError> {
        if self.closing_requested.load(Ordering::Acquire) {
            Err(AbiError::InvalidLifecycleState)
        } else {
            Ok(())
        }
    }
}

impl SessionRuntime {
    pub(super) fn factory_cleanup_unconfirmed(&self) -> bool {
        self.process_set_factory
            .as_ref()
            .is_some_and(|factory| factory.cleanup_unconfirmed())
    }

    fn cleanup_unconfirmed(&self, object: &SessionObject) -> bool {
        if self.startup_cleanup_unconfirmed || self.factory_cleanup_unconfirmed() {
            return true;
        }
        match object {
            SessionObject::Transitioning => true,
            SessionObject::Draft(_) | SessionObject::Compiled(_) | SessionObject::Failed { .. } => {
                false
            }
            SessionObject::Running { session, .. } => {
                let Some(outcome) = self.last_stop_outcome else {
                    return true;
                };
                outcome.runtime_worker_panicked()
                    || outcome.capture_finalization_failures_total() != 0
                    || outcome.operator_finalization_failures_total() != 0
                    || outcome.endpoint_finalization_failures_total() != 0
                    || session.external_source_metrics().iter().any(|source| !source.runtime.joined)
                    // Generic extensions provide no native-closure receipt. The
                    // exact-set factory supplies one, so confirmed source loss
                    // remains distinct from unknown resource finalization.
                    || (outcome.runtime_failures_total() != 0 && self.process_set_factory.is_none())
            }
        }
    }

    pub fn stop_outcome(
        &self,
        handle: PksSessionHandle,
    ) -> Result<(crate::session::SessionStopOutcome, bool), AbiError> {
        let object = self.sessions.get(handle)?;
        if matches!(object, SessionObject::Running { .. }) {
            let outcome = self
                .last_stop_outcome
                .ok_or(AbiError::InvalidLifecycleState)?;
            Ok((outcome, self.cleanup_unconfirmed(object)))
        } else {
            Err(AbiError::InvalidLifecycleState)
        }
    }

    pub fn destroy_session(&mut self, handle: PksSessionHandle) -> Result<(), AbiError> {
        let object = self.sessions.get_mut(handle)?;
        if matches!(object, SessionObject::Running { .. }) {
            // stop() is idempotent and caches its actual outcome. Repeating it
            // cannot erase an unconfirmed close or manufacture native recovery.
            let _ = object.request_stop(&mut self.last_stop_outcome);
        }
        if self.cleanup_unconfirmed(self.sessions.get(handle)?) {
            return Err(AbiError::ProcessSetFailure {
                detail_code: PROCESS_SET_CLEANUP_UNCONFIRMED,
            });
        }
        self.sessions.remove(handle)?;
        Ok(())
    }

    pub(super) fn shutdown(&mut self) -> Result<(), AbiError> {
        let stop_outcome = &mut self.last_stop_outcome;
        self.sessions.for_each_mut(|object| {
            if matches!(object, SessionObject::Running { .. }) {
                let _ = object.request_stop(stop_outcome);
            }
        });
        let mut uncertain = self.startup_cleanup_unconfirmed || self.factory_cleanup_unconfirmed();
        // Snapshot shared closure facts before visiting the handle table mutably.
        let process_set = self.process_set_factory.is_some();
        let stop_outcome = self.last_stop_outcome;
        self.sessions.for_each_mut(|object| {
            if let SessionObject::Running { session, .. } = object {
                uncertain |= stop_outcome.as_ref().is_none_or(|outcome| {
                    outcome.runtime_worker_panicked()
                        || outcome.capture_finalization_failures_total() != 0
                        || outcome.operator_finalization_failures_total() != 0
                        || outcome.endpoint_finalization_failures_total() != 0
                        || (!process_set && outcome.runtime_failures_total() != 0)
                }) || session
                    .external_source_metrics()
                    .iter()
                    .any(|source| !source.runtime.joined);
            } else if matches!(object, SessionObject::Transitioning) {
                uncertain = true;
            }
        });
        if uncertain {
            Err(AbiError::ProcessSetFailure {
                detail_code: PROCESS_SET_CLEANUP_UNCONFIRMED,
            })
        } else {
            Ok(())
        }
    }
}
