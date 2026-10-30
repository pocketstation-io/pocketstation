//! Immutable observed process instances for exact, bounded native capture.

use crate::frame::Platform;
use crate::{CaptureError, ProcessId, SessionError, SourceKind, StableSourceId};

/// Maximum number of independently observed processes in one exact capture.
pub const MAX_APPLICATION_PROCESS_SET_MEMBERS: usize = 32;

/// A process identity observed before declaration, including its start time.
///
/// A PID and application identity alone cannot distinguish a reused PID. The
/// original start observation is retained until native admission and is never
/// replaced with an observation made while opening the tap.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObservedProcessInstance {
    process_id: ProcessId,
    stable_id: StableSourceId,
    process_start_time_ns: u64,
}

impl ObservedProcessInstance {
    pub fn new(
        process_id: ProcessId,
        stable_id: StableSourceId,
        process_start_time_ns: u64,
    ) -> Result<Self, ProcessSetError> {
        let reason = if process_id.get() == 0 || process_id.get() > i32::MAX as u32 {
            Some("process ID must fit the positive native process ID range")
        } else if stable_id.kind != SourceKind::Application {
            Some("identity must describe an application")
        } else if stable_id.stable_key.trim().is_empty() {
            Some("application identity cannot be empty")
        } else if process_start_time_ns == 0 {
            Some("the original process start observation is unavailable")
        } else {
            None
        };
        if let Some(reason) = reason {
            return Err(ProcessSetError::InvalidMember {
                process_id: process_id.get(),
                reason,
            });
        }
        Ok(Self {
            process_id,
            stable_id,
            process_start_time_ns,
        })
    }

    pub const fn process_id(&self) -> ProcessId {
        self.process_id
    }
    pub const fn stable_id(&self) -> &StableSourceId {
        &self.stable_id
    }
    pub const fn process_start_time_ns(&self) -> u64 {
        self.process_start_time_ns
    }
}

/// A frozen, canonical selection; native opening must admit every member.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApplicationProcessSet {
    members: Vec<ObservedProcessInstance>,
}

impl ApplicationProcessSet {
    pub fn new(mut members: Vec<ObservedProcessInstance>) -> Result<Self, ProcessSetError> {
        if members.is_empty() {
            return Err(ProcessSetError::Empty);
        }
        if members.len() > MAX_APPLICATION_PROCESS_SET_MEMBERS {
            return Err(ProcessSetError::TooManyMembers {
                actual_count: members.len(),
            });
        }
        let platform = members[0].stable_id.platform;
        if members
            .iter()
            .any(|member| member.stable_id.platform != platform)
        {
            return Err(ProcessSetError::MixedPlatforms);
        }
        members.sort_unstable_by_key(|member| member.process_id.get());
        if let Some(pair) = members
            .windows(2)
            .find(|pair| pair[0].process_id == pair[1].process_id)
        {
            return Err(ProcessSetError::DuplicateProcess {
                process_id: pair[0].process_id.get(),
            });
        }
        Ok(Self { members })
    }

    pub fn members(&self) -> &[ObservedProcessInstance] {
        &self.members
    }
    pub fn platform(&self) -> Platform {
        self.members[0].stable_id.platform
    }

    #[cfg(any(test, all(target_os = "macos", feature = "coreaudio-capture")))]
    pub(crate) fn stable_id(&self) -> StableSourceId {
        use std::fmt::Write;
        let mut key = String::from("exact-process-set.v1:");
        for member in &self.members {
            // Length-prefixed identity prevents delimiter ambiguity. This is a
            // logical capture identity, never a substitute for native checks.
            let _ = write!(
                key,
                "{}:{}:{}:{};",
                member.process_id.get(),
                member.process_start_time_ns,
                member.stable_id.stable_key.len(),
                member.stable_id.stable_key
            );
        }
        StableSourceId::new(self.platform(), SourceKind::Application, key)
    }
}

/// Control-path failures for declaring and admitting an exact process set.
#[non_exhaustive]
#[derive(Debug, thiserror::Error)]
pub enum ProcessSetError {
    #[error("an exact process set cannot be empty")]
    Empty,
    #[error("exact process set has {actual_count} members; maximum is 32")]
    TooManyMembers { actual_count: usize },
    #[error("invalid process {process_id}: {reason}")]
    InvalidMember {
        process_id: u32,
        reason: &'static str,
    },
    #[error("process {process_id} appears more than once in the exact set")]
    DuplicateProcess { process_id: u32 },
    #[error("an exact process set must belong to one platform")]
    MixedPlatforms,
    #[error("exact process-set capture is unavailable on this platform or build")]
    Unsupported,
    #[error("exact process-set declaration failed: {0}")]
    Declaration(#[from] SessionError),
    #[error("exact process-set registration failed: {0}")]
    Registration(String),
    #[error("exact process-set capture failed: {0}")]
    Capture(#[from] CaptureError),
}

impl ProcessSetError {
    pub fn code(&self) -> &'static str {
        match self {
            Self::Unsupported => "capture.unsupported",
            Self::Capture(error) => match error {
                CaptureError::NotSupported | CaptureError::ModeUnsupported(_) => {
                    "capture.unsupported"
                }
                CaptureError::PermissionDenied { .. } => "capture.permission_denied",
                CaptureError::SourceUnavailable { .. } => "capture.source_unavailable",
                _ => "capture.backend_failed",
            },
            Self::Declaration(_) | Self::Registration(_) => "session.invalid_process_set",
            _ => "session.invalid_process_set",
        }
    }
}

/// Discovers process instances with usable original start observations.
///
/// macOS 26+ with native capture is required so bundle restoration can be
/// explicitly disabled. Unobservable/churned rows are omitted. This list does
/// not establish complete membership of a browser root: callers must join it
/// against their complete intended inventory and reject missing members.
pub fn discover_process_instances() -> Result<Vec<ObservedProcessInstance>, ProcessSetError> {
    #[cfg(all(target_os = "macos", feature = "coreaudio-capture"))]
    {
        super::platform::macos::macos_tap::discover_process_instances_native()
    }
    #[cfg(not(all(target_os = "macos", feature = "coreaudio-capture")))]
    {
        Err(ProcessSetError::Unsupported)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn member(pid: u32, key: &str, start: u64) -> ObservedProcessInstance {
        ObservedProcessInstance::new(
            ProcessId::new(pid),
            StableSourceId::new(Platform::Macos, SourceKind::Application, key),
            start,
        )
        .unwrap()
    }
    #[test]
    fn given_permuted_members_when_declared_then_scope_identity_is_canonical() {
        let left = ApplicationProcessSet::new(vec![member(2, "helper", 20), member(1, "root", 10)])
            .unwrap();
        let right =
            ApplicationProcessSet::new(vec![member(1, "root", 10), member(2, "helper", 20)])
                .unwrap();
        assert_eq!(left, right);
        assert_eq!(left.stable_id(), right.stable_id());
        assert_ne!(
            left.stable_id(),
            ApplicationProcessSet::new(vec![member(1, "root", 11), member(2, "helper", 20)])
                .unwrap()
                .stable_id()
        );
    }
    #[test]
    fn given_duplicate_or_unobserved_members_when_declared_then_scope_is_rejected() {
        assert!(matches!(
            ApplicationProcessSet::new(vec![member(1, "root", 10), member(1, "other", 20)]),
            Err(ProcessSetError::DuplicateProcess { .. })
        ));
        assert!(matches!(
            ApplicationProcessSet::new(vec![]),
            Err(ProcessSetError::Empty)
        ));
        assert!(ObservedProcessInstance::new(
            ProcessId::new(1),
            StableSourceId::new(Platform::Macos, SourceKind::Application, "root"),
            0
        )
        .is_err());
    }
}
