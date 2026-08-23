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
    #[doc = "Stores the capability used by `CaptureAuthorizationSnapshot`."]
    pub capability: CaptureCapabilityState,
    #[doc = "Stores the os permission used by `CaptureAuthorizationSnapshot`."]
    pub os_permission: PermissionObservation,
    #[doc = "Stores the application policy used by `CaptureAuthorizationSnapshot`."]
    pub application_policy: ApplicationPolicyObservation,
    #[doc = "Stores the session grant used by `CaptureAuthorizationSnapshot`."]
    pub session_grant: CaptureSessionGrant,
    #[doc = "Stores the capture scope used by `CaptureAuthorizationSnapshot`."]
    pub capture_scope: CaptureScope,
    #[doc = "Stores the identity strength used by `CaptureAuthorizationSnapshot`."]
    pub identity_strength: SourceIdentityStrength,
    #[doc = "Stores the permission epoch used by `CaptureAuthorizationSnapshot`."]
    pub permission_epoch: PermissionEpoch,
    #[doc = "Stores the observed at value for `CaptureAuthorizationSnapshot`, in nanoseconds."]
    pub observed_at_ns: u64,
    #[doc = "Stores the open outcome used by `CaptureAuthorizationSnapshot`."]
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
#[doc = "Selects the capture capability state used by PocketStation."]
pub enum CaptureCapabilityState {
    #[doc = "Indicates the available state for `CaptureCapabilityState`."]
    Available,
    #[doc = "Reports that the requested resource is unavailable."]
    Unavailable,
    #[doc = "Reports that the requested operation is unsupported."]
    Unsupported,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
#[doc = "Classifies the observable permission observation."]
pub enum PermissionObservation {
    #[doc = "Represents the allowed case of `PermissionObservation`."]
    Allowed,
    #[doc = "Represents the denied case of `PermissionObservation`."]
    Denied,
    #[doc = "Represents the restricted case of `PermissionObservation`."]
    Restricted,
    #[doc = "Represents the not determined case of `PermissionObservation`."]
    NotDetermined,
    #[doc = "Represents the revoked case of `PermissionObservation`."]
    Revoked,
    #[doc = "Represents the not observable case of `PermissionObservation`."]
    NotObservable,
    #[doc = "Represents the not applicable case of `PermissionObservation`."]
    NotApplicable,
}

/// One authoritative authorization-state transition observed by the host.
///
/// PocketStation classifies only observations supplied by the platform owner;
/// it never converts a generic backend error into permission state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CapturePermissionTransition {
    #[doc = "Stores the kind used by `CapturePermissionTransition`."]
    pub kind: SourceLifecycleEventKind,
    #[doc = "Stores the previous used by `CapturePermissionTransition`."]
    pub previous: PermissionObservation,
    #[doc = "Stores the current used by `CapturePermissionTransition`."]
    pub current: PermissionObservation,
    #[doc = "Stores the permission epoch used by `CapturePermissionTransition`."]
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
    #[doc = "Creates a new `CapturePermissionLifecycle`."]
    pub const fn new(current: PermissionObservation) -> Self {
        Self {
            current,
            permission_epoch: PermissionEpoch::INITIAL,
        }
    }

    #[doc = "Returns the current value observed by `CapturePermissionLifecycle`."]
    pub const fn current(self) -> PermissionObservation {
        self.current
    }

    #[doc = "Returns the permission epoch held by `CapturePermissionLifecycle`."]
    pub const fn permission_epoch(self) -> PermissionEpoch {
        self.permission_epoch
    }

    #[doc = "Returns the current observation exposed by `CapturePermissionLifecycle`."]
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
#[doc = "Classifies the observable application policy observation."]
pub enum ApplicationPolicyObservation {
    #[doc = "Selects allowed behavior for `ApplicationPolicyObservation`."]
    Allowed,
    #[doc = "Selects denied behavior for `ApplicationPolicyObservation`."]
    Denied,
    #[doc = "Selects not observable behavior for `ApplicationPolicyObservation`."]
    NotObservable,
    #[doc = "Selects not applicable behavior for `ApplicationPolicyObservation`."]
    NotApplicable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
#[doc = "Enumerates the supported capture session grant cases."]
pub enum CaptureSessionGrant {
    #[doc = "Represents the granted by explicit selection case of `CaptureSessionGrant`."]
    GrantedByExplicitSelection,
    #[doc = "Represents the denied case of `CaptureSessionGrant`."]
    Denied,
    #[doc = "Represents the not evaluated case of `CaptureSessionGrant`."]
    NotEvaluated,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(tag = "kind", rename_all = "kebab-case")]
#[doc = "Selects the capture scope used by PocketStation."]
pub enum CaptureScope {
    #[doc = "Selects exact application behavior for `CaptureScope`."]
    ExactApplication {
        #[doc = "Identifies the stable identifier recorded by `ExactApplication`."]
        stable_id: String,
    },
    #[doc = "Selects exact input device behavior for `CaptureScope`."]
    ExactInputDevice {
        #[doc = "Identifies the stable identifier recorded by `ExactInputDevice`."]
        stable_id: String,
    },
    #[doc = "Selects exact output device behavior for `CaptureScope`."]
    ExactOutputDevice {
        #[doc = "Identifies the stable identifier recorded by `ExactOutputDevice`."]
        stable_id: String,
    },
    #[doc = "Selects system mix behavior for `CaptureScope`."]
    SystemMix,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
#[doc = "Enumerates the supported source identity strength cases."]
pub enum SourceIdentityStrength {
    #[doc = "Represents the application id and process identifier case of `SourceIdentityStrength`."]
    ApplicationIdAndProcessId,
    #[doc = "Represents the stable application identifier case of `SourceIdentityStrength`."]
    StableApplicationId,
    #[doc = "Represents the process identifier case of `SourceIdentityStrength`."]
    ProcessId,
    #[doc = "Represents the stable device uid case of `SourceIdentityStrength`."]
    StableDeviceUid,
    #[doc = "Represents the platform stable identifier case of `SourceIdentityStrength`."]
    PlatformStableId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(transparent)]
#[doc = "Identifies the permission-observation generation attached to captured lineage."]
pub struct PermissionEpoch(pub u64);

impl PermissionEpoch {
    #[doc = "Provides the initial value for `PermissionEpoch`."]
    pub const INITIAL: Self = Self(1);

    /// Advances the local evidence epoch after an observed authorization change
    /// or an explicit source reopen.
    pub fn next(self) -> Self {
        Self(self.0.saturating_add(1))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
#[doc = "Classifies the observable capture open outcome."]
pub enum CaptureOpenOutcome {
    #[doc = "Indicates the not attempted state for `CaptureOpenOutcome`."]
    NotAttempted,
    #[doc = "Indicates the succeeded state for `CaptureOpenOutcome`."]
    Succeeded,
    #[doc = "Reports that the required permission was denied."]
    PermissionDenied,
    #[doc = "Indicates the source unavailable state for `CaptureOpenOutcome`."]
    SourceUnavailable,
    #[doc = "Indicates the backend failed state for `CaptureOpenOutcome`."]
    BackendFailed,
}

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
#[doc = "Classifies failures reported as capture error."]
pub enum CaptureError {
    #[error("system audio loopback is not supported on this platform")]
    #[doc = "Reports not supported."]
    NotSupported,
    #[error("loopback backend error: {0}")]
    #[doc = "Reports backend init."]
    BackendInit(String),
    #[error("capture backend setup required for {backend}: {action}")]
    #[doc = "Reports backend setup required."]
    BackendSetupRequired {
        #[doc = "Stores the backend used by `BackendSetupRequired`."]
        backend: &'static str,
        #[doc = "Stores the action used by `BackendSetupRequired`."]
        action: &'static str,
    },
    #[error("capture permission denied while {operation}")]
    #[doc = "Reports that the required permission was denied."]
    PermissionDenied {
        #[doc = "Stores the operation used by `PermissionDenied`."]
        operation: &'static str,
    },
    #[error("capture backend failed while {operation}: status {status_code}")]
    #[doc = "Reports backend status."]
    BackendStatus {
        #[doc = "Stores the operation used by `BackendStatus`."]
        operation: &'static str,
        #[doc = "Stores the status code used by `BackendStatus`."]
        status_code: i32,
    },
    #[error("selected capture source is unavailable: {stable_key}")]
    #[doc = "Reports source unavailable."]
    SourceUnavailable {
        #[doc = "Stores the stable key used by `SourceUnavailable`."]
        stable_key: String,
    },
    #[error("capture mode not supported on this backend: {0:?}")]
    #[doc = "Reports mode unsupported."]
    ModeUnsupported(CaptureMode),
    #[error("captured-frame stream capacity must be greater than zero")]
    #[doc = "Reports invalid stream capacity."]
    InvalidStreamCapacity,
    #[error("source-runtime event channel capacity must be greater than zero")]
    #[doc = "Reports invalid runtime event capacity."]
    InvalidRuntimeEventCapacity,
    #[error("capture worker panicked while joining: {worker}")]
    #[doc = "Reports capture worker panicked."]
    CaptureWorkerPanicked {
        #[doc = "Stores the worker used by `CaptureWorkerPanicked`."]
        worker: &'static str,
    },
}
