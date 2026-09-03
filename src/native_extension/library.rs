use std::collections::BTreeSet;
use std::path::Path;
use std::sync::Arc;

use libloading::Library;

use super::{
    NativeExtensionKind, NativeExtensionLibrary, NativeExtensionLibraryError,
    NativeExtensionLibraryErrorCode, NativeExtensionRegistration, PksExtensionCallbacks,
    PksExtensionDescriptor, PksExtensionKind, PksExtensionLibrary, PksExtensionLibraryEntrypoint,
    PksSessionStatus, PksSessionStatusCode, EXTENSION_LIBRARY_ENTRYPOINT_V1,
    PKS_EXTENSION_ABI_MAJOR, PKS_EXTENSION_ABI_MINOR,
};
use crate::abi::executable_extension::{
    build_registration_with_library, destroy_acquired_registration,
    ExecutableExtensionRegistration, MAX_LIBRARY_REGISTRATIONS,
};

const EXTENSION_LIBRARY_ENTRYPOINT_V1_NUL: &[u8] = b"pks_extension_library_v1\0";

pub(crate) struct LoadedNativeExtensionLibrary {
    pub(crate) receipt: NativeExtensionLibrary,
    pub(crate) registrations: Vec<ExecutableExtensionRegistration>,
}

/// Loads executable native code selected by the public Session trust boundary.
///
/// # Safety
///
/// The caller must guarantee that the library is trusted and that any exported
/// Extension ABI records, pointers, callbacks, and contexts satisfy their
/// documented validity and lifetime requirements.
pub(crate) unsafe fn load_native_extension_library(
    path: &Path,
) -> Result<LoadedNativeExtensionLibrary, NativeExtensionLibraryError> {
    if !path.is_absolute() {
        return Err(NativeExtensionLibraryError::new(
            NativeExtensionLibraryErrorCode::PathNotAbsolute,
            "native extension libraries require an absolute path",
            Some(path.to_path_buf()),
        ));
    }
    let canonical_path = path.canonicalize().map_err(|error| {
        NativeExtensionLibraryError::new(
            NativeExtensionLibraryErrorCode::PathCanonicalizationFailed,
            format!("failed to canonicalize native extension path: {error}"),
            Some(path.to_path_buf()),
        )
    })?;
    let metadata = canonical_path.metadata().map_err(|error| {
        NativeExtensionLibraryError::new(
            NativeExtensionLibraryErrorCode::PathNotFile,
            format!("failed to inspect native extension path: {error}"),
            Some(canonical_path.clone()),
        )
    })?;
    if !metadata.is_file() {
        return Err(NativeExtensionLibraryError::new(
            NativeExtensionLibraryErrorCode::PathNotFile,
            "native extension path is not a regular file",
            Some(canonical_path),
        ));
    }

    // SAFETY: the caller selected an absolute path, it has been canonicalized,
    // and no ambient loader search is used. Executing a supplied native library
    // remains a trusted setup-time operation by API design.
    let library = Arc::new(unsafe { Library::new(&canonical_path) }.map_err(|error| {
        NativeExtensionLibraryError::new(
            NativeExtensionLibraryErrorCode::LibraryLoadFailed,
            format!("failed to load native extension library: {error}"),
            Some(canonical_path.clone()),
        )
    })?);
    // SAFETY: symbol bytes are NUL terminated and the Extension ABI fixes the
    // exact C-unwind function signature for this ABI major.
    let entrypoint = unsafe {
        *library
            .get::<PksExtensionLibraryEntrypoint>(EXTENSION_LIBRARY_ENTRYPOINT_V1_NUL)
            .map_err(|error| {
                NativeExtensionLibraryError::new(
                    NativeExtensionLibraryErrorCode::EntrypointMissing,
                    format!(
                        "required entrypoint {EXTENSION_LIBRARY_ENTRYPOINT_V1:?} is missing: {error}"
                    ),
                    Some(canonical_path.clone()),
                )
            })?
    };

    let mut descriptor = PksExtensionLibrary {
        struct_size_bytes: std::mem::size_of::<PksExtensionLibrary>() as u32,
        abi_major: PKS_EXTENSION_ABI_MAJOR,
        abi_minor: PKS_EXTENSION_ABI_MINOR,
        registration_count: 0,
        reserved: 0,
        library_context: std::ptr::null_mut(),
        acquire_registration: None,
    };
    let status = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        // SAFETY: descriptor is writable for this call and the library lease
        // keeps the resolved entrypoint executable.
        unsafe { entrypoint(&mut descriptor) }
    }))
    .map_err(|_| {
        NativeExtensionLibraryError::new(
            NativeExtensionLibraryErrorCode::EntrypointPanicked,
            "native extension entrypoint unwound across the host boundary",
            Some(canonical_path.clone()),
        )
    })?;
    status_ok(status).map_err(|message| {
        NativeExtensionLibraryError::new(
            NativeExtensionLibraryErrorCode::EntrypointFailed,
            format!("native extension entrypoint failed: {message}"),
            Some(canonical_path.clone()),
        )
    })?;
    validate_library_descriptor(&descriptor, &canonical_path)?;

    let acquire = descriptor
        .acquire_registration
        .expect("validated acquisition callback");
    let mut registrations = Vec::with_capacity(descriptor.registration_count as usize);
    let mut registration_receipts = Vec::with_capacity(descriptor.registration_count as usize);
    let mut ids = BTreeSet::new();

    for registration_index in 0..descriptor.registration_count {
        // SAFETY: all-zero is a valid initial representation for this C record:
        // function pointer Options and raw pointers use null as their empty
        // value, while numeric fields are subsequently validated before use.
        let mut extension_descriptor: PksExtensionDescriptor = unsafe { std::mem::zeroed() };
        // SAFETY: the callback record has the same zero-valid field classes as
        // the descriptor and is validated before any callback is invoked.
        let mut callbacks: PksExtensionCallbacks = unsafe { std::mem::zeroed() };
        let mut ports = std::ptr::null();
        let mut port_count = 0u32;
        let status = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            // SAFETY: all outputs address writable local records, the library
            // context and callback are retained by the library lease.
            unsafe {
                acquire(
                    descriptor.library_context,
                    registration_index,
                    &mut extension_descriptor,
                    &mut ports,
                    &mut port_count,
                    &mut callbacks,
                )
            }
        }))
        .map_err(|_| {
            NativeExtensionLibraryError::new(
                NativeExtensionLibraryErrorCode::RegistrationAcquisitionPanicked,
                format!("registration acquisition {registration_index} unwound"),
                Some(canonical_path.clone()),
            )
        })?;
        status_ok(status).map_err(|message| {
            NativeExtensionLibraryError::new(
                NativeExtensionLibraryErrorCode::RegistrationAcquisitionFailed,
                format!("registration acquisition {registration_index} failed: {message}"),
                Some(canonical_path.clone()),
            )
        })?;

        if port_count > 64 || (port_count != 0 && ports.is_null()) {
            destroy_acquired_registration(&callbacks);
            return Err(NativeExtensionLibraryError::new(
                NativeExtensionLibraryErrorCode::InvalidRegistration,
                format!("registration {registration_index} returned an invalid port array"),
                Some(canonical_path.clone()),
            ));
        }
        let ports = if port_count == 0 {
            &[]
        } else {
            // SAFETY: the provider contract keeps exactly port_count records
            // readable until this acquisition is synchronously copied. Count
            // and nullness are bounded above.
            unsafe { std::slice::from_raw_parts(ports, port_count as usize) }
        };
        let registration = build_registration_with_library(
            extension_descriptor,
            ports,
            callbacks,
            Some(Arc::clone(&library)),
            true,
        )
        .map_err(|error| {
            NativeExtensionLibraryError::new(
                NativeExtensionLibraryErrorCode::InvalidRegistration,
                format!(
                    "registration {registration_index} is invalid: {}",
                    error.code()
                ),
                Some(canonical_path.clone()),
            )
        })?;
        if !ids.insert(registration.id().to_owned()) {
            return Err(NativeExtensionLibraryError::new(
                NativeExtensionLibraryErrorCode::DuplicateRegistration,
                format!(
                    "library returned duplicate registration id {:?}",
                    registration.id()
                ),
                Some(canonical_path.clone()),
            ));
        }
        registration_receipts.push(NativeExtensionRegistration {
            id: registration.id().to_owned(),
            kind: match registration.kind() {
                PksExtensionKind::Source => NativeExtensionKind::Source,
                PksExtensionKind::Operator => NativeExtensionKind::Operator,
                PksExtensionKind::Endpoint => NativeExtensionKind::Endpoint,
            },
            revision: extension_descriptor.revision,
            generation: extension_descriptor.generation,
        });
        registrations.push(registration);
    }

    Ok(LoadedNativeExtensionLibrary {
        receipt: NativeExtensionLibrary {
            canonical_path,
            registrations: registration_receipts,
        },
        registrations,
    })
}

fn validate_library_descriptor(
    descriptor: &PksExtensionLibrary,
    canonical_path: &Path,
) -> Result<(), NativeExtensionLibraryError> {
    if descriptor.abi_major != PKS_EXTENSION_ABI_MAJOR {
        return Err(NativeExtensionLibraryError::new(
            NativeExtensionLibraryErrorCode::UnsupportedAbiMajor,
            format!("unsupported Extension ABI major {}", descriptor.abi_major),
            Some(canonical_path.to_path_buf()),
        ));
    }
    if descriptor.abi_minor < 2 || descriptor.abi_minor > PKS_EXTENSION_ABI_MINOR {
        return Err(NativeExtensionLibraryError::new(
            NativeExtensionLibraryErrorCode::UnsupportedAbiMinor,
            format!("unsupported Extension ABI minor {}", descriptor.abi_minor),
            Some(canonical_path.to_path_buf()),
        ));
    }
    if descriptor.struct_size_bytes < std::mem::size_of::<PksExtensionLibrary>() as u32
        || descriptor.registration_count == 0
        || descriptor.registration_count > MAX_LIBRARY_REGISTRATIONS
        || descriptor.reserved != 0
        || descriptor.acquire_registration.is_none()
    {
        return Err(NativeExtensionLibraryError::new(
            NativeExtensionLibraryErrorCode::InvalidLibraryDescriptor,
            "native extension library descriptor is malformed or exceeds bounded capacity",
            Some(canonical_path.to_path_buf()),
        ));
    }
    Ok(())
}

fn status_ok(status: PksSessionStatus) -> Result<(), String> {
    if status.code == PksSessionStatusCode::Ok as u32 {
        Ok(())
    } else {
        Err(format!("status={} detail={}", status.code, status.detail))
    }
}
