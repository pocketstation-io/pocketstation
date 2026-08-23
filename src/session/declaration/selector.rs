use crate::capture::{SourceKind, StableSourceId};

use crate::session::SessionError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[doc = "Uniquely identifies process."]
pub struct ProcessId(u32);

impl ProcessId {
    #[doc = "Creates a new `ProcessId`."]
    pub const fn new(process_id: u32) -> Self {
        Self(process_id)
    }

    #[doc = "Returns the value held by `ProcessId`."]
    pub const fn get(self) -> u32 {
        self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[doc = "Uniquely identifies device."]
pub struct DeviceId(String);

impl DeviceId {
    #[doc = "Creates a new `DeviceId`."]
    pub fn new(device_id: impl Into<String>) -> Self {
        Self(device_id.into())
    }

    #[doc = "Returns the stable string representation of `DeviceId`."]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "Enumerates the supported application selector cases."]
pub enum ApplicationSelector {
    #[doc = "Selects bundle identifier behavior for `ApplicationSelector`."]
    BundleId(String),
    #[doc = "Selects process identifier behavior for `ApplicationSelector`."]
    ProcessId(ProcessId),
    #[doc = "Selects process instance behavior for `ApplicationSelector`."]
    ProcessInstance {
        #[doc = "Identifies the process identifier recorded by `ProcessInstance`."]
        process_id: ProcessId,
        #[doc = "Identifies the stable identifier recorded by `ProcessInstance`."]
        stable_id: StableSourceId,
    },
    #[doc = "Selects stable identifier behavior for `ApplicationSelector`."]
    StableId(StableSourceId),
    #[doc = "Selects name behavior for `ApplicationSelector`."]
    Name(String),
}

impl ApplicationSelector {
    #[doc = "Returns the bundle identifier held by `ApplicationSelector`."]
    pub fn bundle_id(bundle_id: impl Into<String>) -> Self {
        Self::BundleId(bundle_id.into())
    }

    #[doc = "Returns the process identifier held by `ApplicationSelector`."]
    pub const fn process_id(process_id: ProcessId) -> Self {
        Self::ProcessId(process_id)
    }

    #[doc = "Creates `ApplicationSelector` for one exact process instance."]
    pub fn process_instance(process_id: ProcessId, stable_id: StableSourceId) -> Self {
        Self::ProcessInstance {
            process_id,
            stable_id,
        }
    }

    #[doc = "Returns the stable identifier held by `ApplicationSelector`."]
    pub fn stable_id(source_id: StableSourceId) -> Self {
        Self::StableId(source_id)
    }

    #[doc = "Returns the name held by `ApplicationSelector`."]
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
#[doc = "Enumerates the supported device selector cases."]
pub enum DeviceSelector {
    #[doc = "Selects default behavior for `DeviceSelector`."]
    Default,
    #[doc = "Selects id behavior for `DeviceSelector`."]
    Id(DeviceId),
}

impl DeviceSelector {
    #[doc = "Returns the default `DeviceSelector` value."]
    pub const fn default() -> Self {
        Self::Default
    }

    #[doc = "Returns the id held by `DeviceSelector`."]
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
#[doc = "Enumerates the supported source cases."]
pub enum Source {
    #[doc = "Represents the application case of `Source`."]
    Application(ApplicationSelector),
    #[doc = "Represents the microphone case of `Source`."]
    Microphone(DeviceSelector),
}

impl Source {
    #[doc = "Returns the application held by `Source`."]
    pub fn application(selector: ApplicationSelector) -> Self {
        Self::Application(selector)
    }

    #[doc = "Creates `Source` for the selected microphone device."]
    pub fn microphone(selector: DeviceSelector) -> Self {
        Self::Microphone(selector)
    }

    #[doc = "Creates `Source` for the host default microphone."]
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
