//! Loading, validation, registration, and ownership for native extension
//! libraries that implement the versioned PocketStation extension ABI.

mod library;

use std::path::{Path, PathBuf};

pub use crate::abi::executable_extension::{
    PksExtensionAcquireRegistrationCallback, PksExtensionCallbacks, PksExtensionCreateCallback,
    PksExtensionDestroyCallback, PksExtensionEndpointConsumeCallback, PksExtensionFinishCallback,
    PksExtensionLibrary, PksExtensionLibraryEntrypoint, PksExtensionOperatorProcessCallback,
    PksExtensionPipelineDeclaration, PksExtensionPrepareCallback, PksExtensionSignalBuffer,
    PksExtensionSignalView, PksExtensionSourceNextCallback, PksExtensionStopCallback,
    PksExtensionValidateConfigurationCallback,
};
pub use crate::abi::extension::{
    PksExtensionAbiVersion, PksExtensionDescriptor, PksExtensionKind, PksExtensionPort,
    PksExtensionPortDirection, PKS_EXTENSION_ABI_MAJOR, PKS_EXTENSION_ABI_MINOR,
};
pub use crate::abi::session::{PksSessionStatus, PksSessionStatusCode, PksSessionUtf8};

pub(crate) use library::load_native_extension_library;

/// Exact exported symbol required from a native Extension ABI v1 dynamic
/// library. The suffix follows the ABI major; compatible minor revisions use
/// the same entrypoint.
pub const EXTENSION_LIBRARY_ENTRYPOINT_V1: &str = "pks_extension_library_v1";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "Selects the native extension kind used by PocketStation."]
pub enum NativeExtensionKind {
    #[doc = "Selects source behavior for `NativeExtensionKind`."]
    Source,
    #[doc = "Selects operator behavior for `NativeExtensionKind`."]
    Operator,
    #[doc = "Selects endpoint behavior for `NativeExtensionKind`."]
    Endpoint,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "Represents native extension registration in the PocketStation API."]
pub struct NativeExtensionRegistration {
    pub(crate) id: String,
    pub(crate) kind: NativeExtensionKind,
    pub(crate) revision: u32,
    pub(crate) generation: u32,
}

impl NativeExtensionRegistration {
    #[doc = "Returns the id associated with `NativeExtensionRegistration`."]
    pub fn id(&self) -> &str {
        &self.id
    }

    #[doc = "Returns the kind represented by `NativeExtensionRegistration`."]
    pub const fn kind(&self) -> NativeExtensionKind {
        self.kind
    }

    #[doc = "Returns the revision associated with `NativeExtensionRegistration`."]
    pub const fn revision(&self) -> u32 {
        self.revision
    }

    #[doc = "Returns the generation associated with `NativeExtensionRegistration`."]
    pub const fn generation(&self) -> u32 {
        self.generation
    }
}

/// Immutable receipt for registrations imported into one Session. Executable
/// code ownership remains internal to the registered factories and drivers.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NativeExtensionLibrary {
    pub(crate) canonical_path: PathBuf,
    pub(crate) registrations: Vec<NativeExtensionRegistration>,
}

impl NativeExtensionLibrary {
    #[doc = "Returns the canonical path associated with `NativeExtensionLibrary`."]
    pub fn canonical_path(&self) -> &Path {
        &self.canonical_path
    }

    #[doc = "Returns the registrations associated with `NativeExtensionLibrary`."]
    pub fn registrations(&self) -> &[NativeExtensionRegistration] {
        &self.registrations
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "Enumerates the supported native extension library error code cases."]
pub enum NativeExtensionLibraryErrorCode {
    #[doc = "Reports path not absolute."]
    PathNotAbsolute,
    #[doc = "Reports path canonicalization failed."]
    PathCanonicalizationFailed,
    #[doc = "Reports path not file."]
    PathNotFile,
    #[doc = "Reports library load failed."]
    LibraryLoadFailed,
    #[doc = "Reports entrypoint missing."]
    EntrypointMissing,
    #[doc = "Reports entrypoint panicked."]
    EntrypointPanicked,
    #[doc = "Reports entrypoint failed."]
    EntrypointFailed,
    #[doc = "Reports unsupported ABI major."]
    UnsupportedAbiMajor,
    #[doc = "Reports unsupported ABI minor."]
    UnsupportedAbiMinor,
    #[doc = "Reports invalid library descriptor."]
    InvalidLibraryDescriptor,
    #[doc = "Reports registration acquisition panicked."]
    RegistrationAcquisitionPanicked,
    #[doc = "Reports registration acquisition failed."]
    RegistrationAcquisitionFailed,
    #[doc = "Reports invalid registration."]
    InvalidRegistration,
    #[doc = "Reports duplicate registration."]
    DuplicateRegistration,
    #[doc = "Reports registration state unavailable."]
    RegistrationStateUnavailable,
}

impl NativeExtensionLibraryErrorCode {
    #[doc = "Returns the stable string representation of `NativeExtensionLibraryErrorCode`."]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PathNotAbsolute => "NATIVE_EXTENSION_PATH_NOT_ABSOLUTE",
            Self::PathCanonicalizationFailed => "NATIVE_EXTENSION_PATH_CANONICALIZATION_FAILED",
            Self::PathNotFile => "NATIVE_EXTENSION_PATH_NOT_FILE",
            Self::LibraryLoadFailed => "NATIVE_EXTENSION_LIBRARY_LOAD_FAILED",
            Self::EntrypointMissing => "NATIVE_EXTENSION_ENTRYPOINT_MISSING",
            Self::EntrypointPanicked => "NATIVE_EXTENSION_ENTRYPOINT_PANICKED",
            Self::EntrypointFailed => "NATIVE_EXTENSION_ENTRYPOINT_FAILED",
            Self::UnsupportedAbiMajor => "NATIVE_EXTENSION_UNSUPPORTED_ABI_MAJOR",
            Self::UnsupportedAbiMinor => "NATIVE_EXTENSION_UNSUPPORTED_ABI_MINOR",
            Self::InvalidLibraryDescriptor => "NATIVE_EXTENSION_INVALID_LIBRARY_DESCRIPTOR",
            Self::RegistrationAcquisitionPanicked => {
                "NATIVE_EXTENSION_REGISTRATION_ACQUISITION_PANICKED"
            }
            Self::RegistrationAcquisitionFailed => {
                "NATIVE_EXTENSION_REGISTRATION_ACQUISITION_FAILED"
            }
            Self::InvalidRegistration => "NATIVE_EXTENSION_INVALID_REGISTRATION",
            Self::DuplicateRegistration => "NATIVE_EXTENSION_DUPLICATE_REGISTRATION",
            Self::RegistrationStateUnavailable => "NATIVE_EXTENSION_REGISTRATION_STATE_UNAVAILABLE",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("{code}: {message}", code = .code.as_str())]
#[doc = "Reports a native extension library error."]
pub struct NativeExtensionLibraryError {
    code: NativeExtensionLibraryErrorCode,
    message: String,
    path: Option<PathBuf>,
}

impl NativeExtensionLibraryError {
    #[doc = "Returns the stable error or status code represented by `NativeExtensionLibraryError`."]
    pub const fn code(&self) -> NativeExtensionLibraryErrorCode {
        self.code
    }

    #[doc = "Returns the diagnostic message associated with `NativeExtensionLibraryError`."]
    pub fn message(&self) -> &str {
        &self.message
    }

    #[doc = "Returns the path associated with `NativeExtensionLibraryError`."]
    pub fn path(&self) -> Option<&Path> {
        self.path.as_deref()
    }

    pub(crate) fn registration_state_unavailable(registry: &'static str) -> Self {
        Self::new(
            NativeExtensionLibraryErrorCode::RegistrationStateUnavailable,
            format!("{registry} registration state is unavailable"),
            None,
        )
    }

    pub(crate) fn duplicate_registration(id: &str) -> Self {
        Self::new(
            NativeExtensionLibraryErrorCode::DuplicateRegistration,
            format!("registration id {id:?} already exists in this Session"),
            None,
        )
    }

    pub(crate) fn new(
        code: NativeExtensionLibraryErrorCode,
        message: impl Into<String>,
        path: Option<PathBuf>,
    ) -> Self {
        Self {
            code,
            message: message.into(),
            path,
        }
    }
}
