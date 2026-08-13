//! Explicit capture authorization evidence and open outcomes.

use serde::Serialize;

use super::events::SourceLifecycleEventKind;
use super::identity::{CaptureSource, SourceKind, SourceState};
use super::selection::CaptureMode;
use super::timeline::monotonic_timestamp_ns;

/// Point-in-time authorization evidence for opening one exact capture source.
///
/// This is control-plane evidence, not callback state. Backends must use
/// `NotObservable` when the operating system does not expose an authoritative
/// permission or policy result; a successful stream open is recorded
/// separately and must not be relabeled as an OS permission preflight.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CaptureAuthorizationSnapshot {
    pub capability: CaptureCapabilityState,
    pub os_permission: PermissionObservation,
    pub application_policy: ApplicationPolicyObservation,
    pub session_grant: CaptureSessionGrant,
    pub capture_scope: CaptureScope,
    pub identity_strength: SourceIdentityStrength,
    pub permission_epoch: PermissionEpoch,
    pub observed_at_ns: u64,
    pub open_outcome: CaptureOpenOutcome,
}

impl CaptureAuthorizationSnapshot {
    /// Records the evidence available after an explicitly selected source opens.
    pub fn after_successful_open(
        source: &CaptureSource,
        session_grant: CaptureSessionGrant,
        permission_epoch: PermissionEpoch,
    ) -> Self {
        Self::from_open_outcome(
            source,
            session_grant,
            permission_epoch,
            CaptureOpenOutcome::Succeeded,
        )
    }

    /// Records a backend open failure without guessing that permission was denied.
    pub fn after_failed_open(
        source: &CaptureSource,
        session_grant: CaptureSessionGrant,
        permission_epoch: PermissionEpoch,
    ) -> Self {
        Self::from_open_outcome(
            source,
            session_grant,
            permission_epoch,
            CaptureOpenOutcome::BackendFailed,
        )
    }

    /// Records a source that was resolved but not opened because another
    /// required source failed first.
    pub fn before_open(
        source: &CaptureSource,
        session_grant: CaptureSessionGrant,
        permission_epoch: PermissionEpoch,
    ) -> Self {
        Self::from_open_outcome(
            source,
            session_grant,
            permission_epoch,
            CaptureOpenOutcome::NotAttempted,
        )
    }

    /// Records platform authorization observations without inferring them from
    /// a generic backend result. Callers must pass `NotObservable` when their
    /// platform has no authoritative query for the requested capture class.
    pub fn from_open_observations(
        source: &CaptureSource,
        session_grant: CaptureSessionGrant,
        permission_epoch: PermissionEpoch,
        os_permission: PermissionObservation,
        application_policy: ApplicationPolicyObservation,
        open_outcome: CaptureOpenOutcome,
    ) -> Self {
        let mut snapshot =
            Self::from_open_outcome(source, session_grant, permission_epoch, open_outcome);
        snapshot.os_permission = os_permission;
        snapshot.application_policy = application_policy;
        snapshot
    }

    fn from_open_outcome(
        source: &CaptureSource,
        session_grant: CaptureSessionGrant,
        permission_epoch: PermissionEpoch,
        open_outcome: CaptureOpenOutcome,
    ) -> Self {
        let (capture_scope, identity_strength, application_policy) = match source.stable_id.kind {
            SourceKind::Application => (
                CaptureScope::ExactApplication {
                    stable_id: source.stable_id.stable_key.clone(),
                },
                source.identity_strength(),
                ApplicationPolicyObservation::NotObservable,
            ),
            SourceKind::InputDevice => (
                CaptureScope::ExactInputDevice {
                    stable_id: source.stable_id.stable_key.clone(),
                },
                source.identity_strength(),
                ApplicationPolicyObservation::NotApplicable,
            ),
            SourceKind::OutputDevice => (
                CaptureScope::ExactOutputDevice {
                    stable_id: source.stable_id.stable_key.clone(),
                },
                source.identity_strength(),
                ApplicationPolicyObservation::NotApplicable,
            ),
            SourceKind::SystemMix => (
                CaptureScope::SystemMix,
                SourceIdentityStrength::PlatformStableId,
                ApplicationPolicyObservation::NotApplicable,
            ),
        };
        Self {
            capability: if source.state == SourceState::Unavailable {
                CaptureCapabilityState::Unavailable
            } else {
                CaptureCapabilityState::Available
            },
            os_permission: PermissionObservation::NotObservable,
            application_policy,
            session_grant,
            capture_scope,
            identity_strength,
            permission_epoch,
            observed_at_ns: monotonic_timestamp_ns(),
            open_outcome,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum CaptureCapabilityState {
    Available,
    Unavailable,
    Unsupported,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum PermissionObservation {
    Allowed,
    Denied,
    Restricted,
    NotDetermined,
    Revoked,
    NotObservable,
    NotApplicable,
}

/// One authoritative authorization-state transition observed by the host.
///
/// PocketStation classifies only observations supplied by the platform owner;
/// it never converts a generic backend error into permission state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CapturePermissionTransition {
    pub kind: SourceLifecycleEventKind,
    pub previous: PermissionObservation,
    pub current: PermissionObservation,
    pub permission_epoch: PermissionEpoch,
}

/// Control-plane owner for one source's observed authorization epoch.
///
/// Hosts may poll the platform at their preferred control-plane cadence and
/// feed the authoritative result here. Equal observations produce no event;
/// leaving `Allowed` is a revocation and every other change is a permission
/// change. This type performs no OS query and is never used on an audio
/// callback.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CapturePermissionLifecycle {
    current: PermissionObservation,
    permission_epoch: PermissionEpoch,
}

impl CapturePermissionLifecycle {
    pub const fn new(current: PermissionObservation) -> Self {
        Self {
            current,
            permission_epoch: PermissionEpoch::INITIAL,
        }
    }

    pub const fn current(self) -> PermissionObservation {
        self.current
    }

    pub const fn permission_epoch(self) -> PermissionEpoch {
        self.permission_epoch
    }

    pub fn observe(
        &mut self,
        current: PermissionObservation,
    ) -> Option<CapturePermissionTransition> {
        let previous = self.current;
        if previous == current {
            return None;
        }
        self.current = current;
        self.permission_epoch = self.permission_epoch.next();
        Some(CapturePermissionTransition {
            kind: if previous == PermissionObservation::Allowed
                && current != PermissionObservation::Allowed
            {
                SourceLifecycleEventKind::PermissionRevoked
            } else {
                SourceLifecycleEventKind::PermissionChanged
            },
            previous,
            current,
            permission_epoch: self.permission_epoch,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum ApplicationPolicyObservation {
    Allowed,
    Denied,
    NotObservable,
    NotApplicable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum CaptureSessionGrant {
    GrantedByExplicitSelection,
    Denied,
    NotEvaluated,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum CaptureScope {
    ExactApplication { stable_id: String },
    ExactInputDevice { stable_id: String },
    ExactOutputDevice { stable_id: String },
    SystemMix,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum SourceIdentityStrength {
    ApplicationIdAndProcessId,
    StableApplicationId,
    ProcessId,
    StableDeviceUid,
    PlatformStableId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(transparent)]
pub struct PermissionEpoch(pub u64);

impl PermissionEpoch {
    pub const INITIAL: Self = Self(1);

    /// Advances the local evidence epoch after an observed authorization change
    /// or an explicit source reopen.
    pub fn next(self) -> Self {
        Self(self.0.saturating_add(1))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum CaptureOpenOutcome {
    NotAttempted,
    Succeeded,
    PermissionDenied,
    SourceUnavailable,
    BackendFailed,
}

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum CaptureError {
    #[error("system audio loopback is not supported on this platform")]
    NotSupported,
    #[error("loopback backend error: {0}")]
    BackendInit(String),
    #[error("capture backend setup required for {backend}: {action}")]
    BackendSetupRequired {
        backend: &'static str,
        action: &'static str,
    },
    #[error("capture permission denied while {operation}")]
    PermissionDenied { operation: &'static str },
    #[error("capture backend failed while {operation}: status {status_code}")]
    BackendStatus {
        operation: &'static str,
        status_code: i32,
    },
    #[error("selected capture source is unavailable: {stable_key}")]
    SourceUnavailable { stable_key: String },
    #[error("capture mode not supported on this backend: {0:?}")]
    ModeUnsupported(CaptureMode),
    #[error("captured-frame stream capacity must be greater than zero")]
    InvalidStreamCapacity,
    #[error("source-runtime event channel capacity must be greater than zero")]
    InvalidRuntimeEventCapacity,
    #[error("capture worker panicked while joining: {worker}")]
    CaptureWorkerPanicked { worker: &'static str },
}
