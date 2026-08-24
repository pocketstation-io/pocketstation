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
    #[doc = "Classifies the loaded native extension as source."]
    Source,
    #[doc = "Classifies the loaded native extension as operator."]
    Operator,
    #[doc = "Classifies the loaded native extension as endpoint."]
    Endpoint,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "Identifies one node registration imported transactionally from a native extension."]
pub struct NativeExtensionRegistration {
    pub(crate) id: String,
    pub(crate) kind: NativeExtensionKind,
    pub(crate) revision: u32,
    pub(crate) generation: u32,
}

impl NativeExtensionRegistration {
    #[doc = "Returns the id held by `NativeExtensionRegistration`."]
    pub fn id(&self) -> &str {
        &self.id
    }

    #[doc = "Returns the kind represented by `NativeExtensionRegistration`."]
    pub const fn kind(&self) -> NativeExtensionKind {
        self.kind
    }

    #[doc = "Returns the revision held by `NativeExtensionRegistration`."]
    pub const fn revision(&self) -> u32 {
        self.revision
    }

    #[doc = "Returns the generation held by `NativeExtensionRegistration`."]
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
    #[doc = "Returns the canonical path held by `NativeExtensionLibrary`."]
    pub fn canonical_path(&self) -> &Path {
        &self.canonical_path
    }

    #[doc = "Returns the registrations held by `NativeExtensionLibrary`."]
    pub fn registrations(&self) -> &[NativeExtensionRegistration] {
        &self.registrations
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "Provides stable categories for native-extension load and validation failures."]
pub enum NativeExtensionLibraryErrorCode {
    #[doc = "Classifies a failure at the path not absolute stage or component of `NativeExtensionLibraryErrorCode`."]
    PathNotAbsolute,
    #[doc = "Reports that path canonicalization failed."]
    PathCanonicalizationFailed,
    #[doc = "Classifies a failure at the path not file stage or component of `NativeExtensionLibraryErrorCode`."]
    PathNotFile,
    #[doc = "Reports that library load failed."]
    LibraryLoadFailed,
    #[doc = "Classifies a failure at the entrypoint missing stage or component of `NativeExtensionLibraryErrorCode`."]
    EntrypointMissing,
    #[doc = "Reports that entrypoint panicked while the operation was active."]
    EntrypointPanicked,
    #[doc = "Reports that entrypoint failed."]
    EntrypointFailed,
    #[doc = "Reports that the requested ABI major is unsupported."]
    UnsupportedAbiMajor,
    #[doc = "Reports that the requested ABI minor is unsupported."]
    UnsupportedAbiMinor,
    #[doc = "Reports that the supplied library descriptor is invalid."]
    InvalidLibraryDescriptor,
    #[doc = "Reports that registration acquisition panicked while the operation was active."]
    RegistrationAcquisitionPanicked,
    #[doc = "Reports that registration acquisition failed."]
    RegistrationAcquisitionFailed,
    #[doc = "Reports that the supplied registration is invalid."]
    InvalidRegistration,
    #[doc = "Reports that registration duplicates an existing declaration or record."]
    DuplicateRegistration,
    #[doc = "Reports that registration state is unavailable."]
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

    #[doc = "Returns the diagnostic message reported by `NativeExtensionLibraryError`."]
    pub fn message(&self) -> &str {
        &self.message
    }

    #[doc = "Returns the path held by `NativeExtensionLibraryError`."]
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
