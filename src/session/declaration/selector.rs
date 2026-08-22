use crate::capture::{SourceKind, StableSourceId};

use crate::session::SessionError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ProcessId(u32);

impl ProcessId {
    pub const fn new(process_id: u32) -> Self {
        Self(process_id)
    }

    pub const fn get(self) -> u32 {
        self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DeviceId(String);

impl DeviceId {
    pub fn new(device_id: impl Into<String>) -> Self {
        Self(device_id.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ApplicationSelector {
    BundleId(String),
    ProcessId(ProcessId),
    ProcessInstance {
        process_id: ProcessId,
        stable_id: StableSourceId,
    },
    StableId(StableSourceId),
    Name(String),
}

impl ApplicationSelector {
    pub fn bundle_id(bundle_id: impl Into<String>) -> Self {
        Self::BundleId(bundle_id.into())
    }

    pub const fn process_id(process_id: ProcessId) -> Self {
        Self::ProcessId(process_id)
    }

    pub fn process_instance(process_id: ProcessId, stable_id: StableSourceId) -> Self {
        Self::ProcessInstance {
            process_id,
            stable_id,
        }
    }

    pub fn stable_id(source_id: StableSourceId) -> Self {
        Self::StableId(source_id)
    }

    pub fn name(name: impl Into<String>) -> Self {
        Self::Name(name.into())
    }

    pub(crate) fn validate(&self) -> Result<(), SessionError> {
        match self {
            Self::BundleId(bundle_id) if bundle_id.trim().is_empty() => {
                Err(SessionError::InvalidSelector {
                    reason: "application bundle id cannot be empty".to_owned(),
                })
            }
            Self::ProcessId(process_id) if process_id.get() == 0 => {
                Err(SessionError::InvalidSelector {
                    reason: "application process id must be non-zero".to_owned(),
                })
            }
            Self::ProcessInstance { process_id, .. } if process_id.get() == 0 => {
                Err(SessionError::InvalidSelector {
                    reason: "application process instance id must be non-zero".to_owned(),
                })
            }
            Self::ProcessInstance { stable_id, .. } | Self::StableId(stable_id)
                if stable_id.kind != SourceKind::Application =>
            {
                Err(SessionError::InvalidSelector {
                    reason: "application stable id must identify an application".to_owned(),
                })
            }
            Self::ProcessInstance { stable_id, .. } | Self::StableId(stable_id)
                if stable_id.stable_key.trim().is_empty() =>
            {
                Err(SessionError::InvalidSelector {
                    reason: "application stable id cannot be empty".to_owned(),
                })
            }
            Self::Name(name) if name.trim().is_empty() => Err(SessionError::InvalidSelector {
                reason: "application name cannot be empty".to_owned(),
            }),
            _ => Ok(()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DeviceSelector {
    Default,
    Id(DeviceId),
}

impl DeviceSelector {
    pub const fn default() -> Self {
        Self::Default
    }

    pub fn id(device_id: DeviceId) -> Self {
        Self::Id(device_id)
    }

    pub(crate) fn validate(&self) -> Result<(), SessionError> {
        match self {
            Self::Id(device_id) if device_id.as_str().trim().is_empty() => {
                Err(SessionError::InvalidSelector {
                    reason: "microphone device id cannot be empty".to_owned(),
                })
            }
            _ => Ok(()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Source {
    Application(ApplicationSelector),
    Microphone(DeviceSelector),
}

impl Source {
    pub fn application(selector: ApplicationSelector) -> Self {
        Self::Application(selector)
    }

    pub fn microphone(selector: DeviceSelector) -> Self {
        Self::Microphone(selector)
    }

    pub const fn microphone_default() -> Self {
        Self::Microphone(DeviceSelector::default())
    }

    pub(crate) fn validate(&self) -> Result<(), SessionError> {
        match self {
            Self::Application(selector) => selector.validate(),
            Self::Microphone(selector) => selector.validate(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::capture::{SourceKind, StableSourceId};
    use crate::frame::Platform;

    use super::{ApplicationSelector, ProcessId};
    use crate::session::SessionError;

    #[test]
    fn given_zero_pid_when_process_instance_validated_then_selector_is_rejected() {
        let selector = ApplicationSelector::process_instance(
            ProcessId::new(0),
            StableSourceId::new(
                Platform::Windows,
                SourceKind::Application,
                "wasapi:pid:0:creation-100ns:133801234567890000",
            ),
        );

        assert!(matches!(
            selector.validate(),
            Err(SessionError::InvalidSelector { .. })
        ));
    }

    #[test]
    fn given_non_application_identity_when_process_instance_validated_then_selector_is_rejected() {
        let selector = ApplicationSelector::process_instance(
            ProcessId::new(42),
            StableSourceId::new(
                Platform::Windows,
                SourceKind::InputDevice,
                "wasapi:pid:42:creation-100ns:133801234567890000",
            ),
        );

        assert!(matches!(
            selector.validate(),
            Err(SessionError::InvalidSelector { .. })
        ));
    }

    #[test]
    fn given_empty_stable_key_when_process_instance_validated_then_selector_is_rejected() {
        let selector = ApplicationSelector::process_instance(
            ProcessId::new(42),
            StableSourceId::new(Platform::Windows, SourceKind::Application, " "),
        );

        assert!(matches!(
            selector.validate(),
            Err(SessionError::InvalidSelector { .. })
        ));
    }
}
