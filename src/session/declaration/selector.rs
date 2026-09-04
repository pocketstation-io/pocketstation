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
    /// Selects an application by its native application identifier.
    ///
    /// Application identifiers are platform-specific. Use a discovered
    /// [`StableSourceId`] when the selection must be reused without repeating
    /// a display-name lookup.
    pub fn bundle_id(bundle_id: impl Into<String>) -> Self {
        Self::BundleId(bundle_id.into())
    }

    /// Selects one currently running process.
    ///
    /// Process identifiers are temporary and can be reused after a process
    /// exits. Prefer a discovered [`StableSourceId`] when one is available.
    pub const fn process_id(process_id: ProcessId) -> Self {
        Self::ProcessId(process_id)
    }

    pub fn process_instance(process_id: ProcessId, stable_id: StableSourceId) -> Self {
        Self::ProcessInstance {
            process_id,
            stable_id,
        }
    }

    /// Selects the live application represented by a discovered identity.
    ///
    /// The identity's persistence is reported by
    /// [`crate::CaptureSource::selector_persistence_scope`]. Some platforms
    /// provide an application identity that survives restarts; others can
    /// provide only an identity for the current process or audio node.
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

impl From<&str> for ApplicationSelector {
    fn from(name: &str) -> Self {
        Self::name(name)
    }
}

impl From<String> for ApplicationSelector {
    fn from(name: String) -> Self {
        Self::name(name)
    }
}

impl From<&String> for ApplicationSelector {
    fn from(name: &String) -> Self {
        Self::name(name)
    }
}

impl From<ProcessId> for ApplicationSelector {
    fn from(process_id: ProcessId) -> Self {
        Self::process_id(process_id)
    }
}

impl From<StableSourceId> for ApplicationSelector {
    fn from(stable_id: StableSourceId) -> Self {
        Self::stable_id(stable_id)
    }
}

impl From<&StableSourceId> for ApplicationSelector {
    fn from(stable_id: &StableSourceId) -> Self {
        Self::stable_id(stable_id.clone())
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
    SystemAudio,
    Microphone(DeviceSelector),
}

impl Source {
    /// Selects one running desktop application.
    ///
    /// A string matches an exact display name or native application ID,
    /// ignoring ASCII case. Selection fails before capture begins when no
    /// application matches or when the value identifies more than one
    /// application. PocketStation never guesses or selects the first match.
    ///
    /// Use [`ProcessId`] for a temporary process selection or a discovered
    /// [`StableSourceId`] for exact identity selection.
    ///
    /// ```
    /// use pocketstation::{ProcessId, Source};
    ///
    /// let by_name = Source::application("Zoom");
    /// let by_application_id = Source::application("us.zoom.xos");
    /// let by_process = Source::application(ProcessId::new(1234));
    /// # let _ = (by_name, by_application_id, by_process);
    /// ```
    pub fn application(selector: impl Into<ApplicationSelector>) -> Self {
        Self::Application(selector.into())
    }

    /// Captures the audio playing through the host's output devices.
    ///
    /// This selects the complete system mix. Use [`Self::application`] when
    /// only one running application should be captured.
    pub const fn system_audio() -> Self {
        Self::SystemAudio
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
            Self::SystemAudio => Ok(()),
            Self::Microphone(selector) => selector.validate(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::capture::{SourceKind, StableSourceId};
    use crate::frame::Platform;

    use super::{ApplicationSelector, ProcessId, Source};
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

    #[test]
    fn given_application_name_when_source_declared_then_name_selector_is_inferred() {
        assert_eq!(
            Source::application("Zoom"),
            Source::Application(ApplicationSelector::Name("Zoom".to_owned()))
        );
    }

    #[test]
    fn given_owned_application_name_when_source_declared_then_name_selector_is_inferred() {
        assert_eq!(
            Source::application("Zoom".to_owned()),
            Source::Application(ApplicationSelector::Name("Zoom".to_owned()))
        );
    }

    #[test]
    fn given_borrowed_owned_name_when_source_declared_then_name_selector_is_inferred() {
        let name = "Zoom".to_owned();

        assert_eq!(
            Source::application(&name),
            Source::Application(ApplicationSelector::Name("Zoom".to_owned()))
        );
    }

    #[test]
    fn given_process_id_when_source_declared_then_process_selector_is_inferred() {
        assert_eq!(
            Source::application(ProcessId::new(42)),
            Source::Application(ApplicationSelector::ProcessId(ProcessId::new(42)))
        );
    }

    #[test]
    fn given_stable_identity_when_source_declared_then_exact_selector_is_inferred() {
        let stable_id =
            StableSourceId::new(Platform::Macos, SourceKind::Application, "com.acme.meeting");

        assert_eq!(
            Source::application(&stable_id),
            Source::Application(ApplicationSelector::StableId(stable_id))
        );
    }

    #[test]
    fn given_system_audio_when_source_declared_then_no_selector_is_required() {
        assert_eq!(Source::system_audio(), Source::SystemAudio);
        assert!(Source::system_audio().validate().is_ok());
    }
}
