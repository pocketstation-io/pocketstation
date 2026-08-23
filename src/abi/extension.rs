use std::mem::{align_of, size_of};
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::slice;

use crate::abi::session::{PksSessionStatus, PksSessionStatusCode, PksSessionUtf8};

#[doc = "Defines the major version of extension ABI."]
pub const PKS_EXTENSION_ABI_MAJOR: u16 = 1;
#[doc = "Defines the minor version of extension ABI."]
pub const PKS_EXTENSION_ABI_MINOR: u16 = 2;
const MAX_PORTS: u32 = 64;
const MAX_IDENTIFIER_BYTES: u32 = 1_024;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[doc = "Carries the major and minor native-extension ABI versions checked during loading."]
pub struct PksExtensionAbiVersion {
    #[doc = "Stores the byte size of the `PksExtensionAbiVersion` ABI structure."]
    pub struct_size_bytes: u32,
    #[doc = "Stores the major ABI version expected by `PksExtensionAbiVersion`."]
    pub abi_major: u16,
    #[doc = "Stores the minor ABI version expected by `PksExtensionAbiVersion`."]
    pub abi_minor: u16,
}

impl PksExtensionAbiVersion {
    const fn current() -> Self {
        Self {
            struct_size_bytes: size_of::<Self>() as u32,
            abi_major: PKS_EXTENSION_ABI_MAJOR,
            abi_minor: PKS_EXTENSION_ABI_MINOR,
        }
    }
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[doc = "Selects the extension kind used by PocketStation."]
pub enum PksExtensionKind {
    #[doc = "Selects source behavior for `PksExtensionKind`."]
    Source = 1,
    #[doc = "Selects operator behavior for `PksExtensionKind`."]
    Operator = 2,
    #[doc = "Selects endpoint behavior for `PksExtensionKind`."]
    Endpoint = 3,
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[doc = "Selects the extension port direction used by PocketStation."]
pub enum PksExtensionPortDirection {
    #[doc = "Selects input behavior for `PksExtensionPortDirection`."]
    Input = 1,
    #[doc = "Selects output behavior for `PksExtensionPortDirection`."]
    Output = 2,
}

#[repr(C)]
#[derive(Clone, Copy)]
#[doc = "Describes the extension descriptor contract."]
pub struct PksExtensionDescriptor {
    #[doc = "Stores the byte size of the `PksExtensionDescriptor` ABI structure."]
    pub struct_size_bytes: u32,
    #[doc = "Stores the major ABI version expected by `PksExtensionDescriptor`."]
    pub abi_major: u16,
    #[doc = "Stores the minor ABI version expected by `PksExtensionDescriptor`."]
    pub abi_minor: u16,
    #[doc = "Stores the kind used by `PksExtensionDescriptor`."]
    pub kind: u32,
    #[doc = "Stores the revision used by `PksExtensionDescriptor`."]
    pub revision: u32,
    #[doc = "Stores the generation used by `PksExtensionDescriptor`."]
    pub generation: u32,
    #[doc = "Stores the number of port represented by `PksExtensionDescriptor`."]
    pub port_count: u32,
    #[doc = "Identifies the extension identifier recorded by `PksExtensionDescriptor`."]
    pub extension_id: PksSessionUtf8,
}

#[repr(C)]
#[derive(Clone, Copy)]
#[doc = "Describes one native-extension port across the C ABI, including direction and signal metadata."]
pub struct PksExtensionPort {
    #[doc = "Stores the byte size of the `PksExtensionPort` ABI structure."]
    pub struct_size_bytes: u32,
    #[doc = "Stores the major ABI version expected by `PksExtensionPort`."]
    pub abi_major: u16,
    #[doc = "Stores the minor ABI version expected by `PksExtensionPort`."]
    pub abi_minor: u16,
    #[doc = "Stores the direction used by `PksExtensionPort`."]
    pub direction: u32,
    #[doc = "Indicates whether required applies to `PksExtensionPort`."]
    pub required: u32,
    #[doc = "Stores the name used by `PksExtensionPort`."]
    pub name: PksSessionUtf8,
    #[doc = "Identifies the signal identifier recorded by `PksExtensionPort`."]
    pub signal_id: PksSessionUtf8,
    #[doc = "Stores the semantic role used by `PksExtensionPort`."]
    pub semantic_role: PksSessionUtf8,
    #[doc = "Stores the schema used by `PksExtensionPort`."]
    pub schema: PksSessionUtf8,
}

#[unsafe(no_mangle)]
/// Returns the version of the language-neutral extension descriptor ABI.
///
/// # Safety
/// `output_version` must point to one writable, aligned record.
pub unsafe extern "C" fn pks_extension_abi_get_version(
    output_version: *mut PksExtensionAbiVersion,
) -> PksSessionStatus {
    extension_call(|| {
        guard_mut_pointer(output_version)?;
        // SAFETY: validated against the documented caller contract above.
        unsafe { output_version.write(PksExtensionAbiVersion::current()) };
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn pks_extension_abi_is_compatible(
    requested_abi_major: u16,
    requested_abi_minor: u16,
    requested_struct_size_bytes: u32,
) -> PksSessionStatus {
    extension_call(|| {
        guard_version(requested_abi_major, requested_abi_minor)?;
        guard_size(
            requested_struct_size_bytes,
            size_of::<PksExtensionAbiVersion>(),
        )
    })
}

#[unsafe(no_mangle)]
/// Validates a complete source, operator, or endpoint descriptor without
/// retaining caller memory.
///
/// # Safety
/// `descriptor` must point to one readable aligned descriptor. When
/// `port_count` is non-zero, `ports` must point to that many readable aligned
/// port records for this call.
pub unsafe extern "C" fn pks_extension_descriptor_validate(
    descriptor: *const PksExtensionDescriptor,
    ports: *const PksExtensionPort,
    port_count: u32,
) -> PksSessionStatus {
    extension_call(|| {
        guard_pointer(descriptor)?;
        // SAFETY: pointer and alignment were validated; size is checked before
        // reading fields beyond the fixed versioned prefix.
        let descriptor = unsafe { descriptor.read() };
        guard_version(descriptor.abi_major, descriptor.abi_minor)?;
        guard_size(
            descriptor.struct_size_bytes,
            size_of::<PksExtensionDescriptor>(),
        )?;
        if descriptor.revision == 0
            || descriptor.generation == 0
            || descriptor.port_count != port_count
            || port_count > MAX_PORTS
            || !matches!(
                descriptor.kind,
                value if value == PksExtensionKind::Source as u32
                    || value == PksExtensionKind::Operator as u32
                    || value == PksExtensionKind::Endpoint as u32
            )
        {
            return Err(ExtensionAbiError::InvalidArgument);
        }
        validate_text(descriptor.extension_id, false)?;
        if port_count == 0 {
            return Err(ExtensionAbiError::InvalidArgument);
        }
        guard_pointer(ports)?;
        // SAFETY: the caller contract supplies exactly port_count records and
        // the count is bounded before constructing this view.
        let ports = unsafe { slice::from_raw_parts(ports, port_count as usize) };
        let mut input_count = 0u32;
        let mut output_count = 0u32;
        for (index, port) in ports.iter().enumerate() {
            guard_version(port.abi_major, port.abi_minor)?;
            guard_size(port.struct_size_bytes, size_of::<PksExtensionPort>())?;
            if port.required > 1 {
                return Err(ExtensionAbiError::InvalidArgument);
            }
            match port.direction {
                value if value == PksExtensionPortDirection::Input as u32 => {
                    input_count = input_count.saturating_add(1);
                }
                value if value == PksExtensionPortDirection::Output as u32 => {
                    output_count = output_count.saturating_add(1);
                }
                _ => return Err(ExtensionAbiError::InvalidArgument),
            }
            let name = validate_text(port.name, false)?;
            validate_text(port.signal_id, false)?;
            validate_text(port.semantic_role, true)?;
            validate_text(port.schema, true)?;
            for previous in &ports[..index] {
                if bytes(previous.name)? == name {
                    return Err(ExtensionAbiError::InvalidArgument);
                }
            }
        }
        match descriptor.kind {
            value if value == PksExtensionKind::Source as u32 && output_count == 0 => {
                Err(ExtensionAbiError::InvalidArgument)
            }
            value
                if value == PksExtensionKind::Operator as u32
                    && (input_count == 0 || output_count == 0) =>
            {
                Err(ExtensionAbiError::InvalidArgument)
            }
            value if value == PksExtensionKind::Endpoint as u32 && input_count == 0 => {
                Err(ExtensionAbiError::InvalidArgument)
            }
            _ => Ok(()),
        }
    })
}

fn extension_call(operation: impl FnOnce() -> Result<(), ExtensionAbiError>) -> PksSessionStatus {
    match catch_unwind(AssertUnwindSafe(operation)) {
        Ok(Ok(())) => PksSessionStatus::ok(),
        Ok(Err(error)) => error.status(),
        Err(_) => PksSessionStatus::new(PksSessionStatusCode::InternalPanic, 0),
    }
}

fn guard_version(major: u16, minor: u16) -> Result<(), ExtensionAbiError> {
    if major != PKS_EXTENSION_ABI_MAJOR {
        Err(ExtensionAbiError::UnsupportedMajor)
    } else if minor > PKS_EXTENSION_ABI_MINOR {
        Err(ExtensionAbiError::UnsupportedMinor)
    } else {
        Ok(())
    }
}

fn guard_size(actual: u32, required: usize) -> Result<(), ExtensionAbiError> {
    if actual < required as u32 {
        Err(ExtensionAbiError::InvalidSize)
    } else {
        Ok(())
    }
}

fn guard_pointer<T>(pointer: *const T) -> Result<(), ExtensionAbiError> {
    if pointer.is_null() {
        Err(ExtensionAbiError::Null)
    } else if !(pointer as usize).is_multiple_of(align_of::<T>()) {
        Err(ExtensionAbiError::Misaligned)
    } else {
        Ok(())
    }
}

fn guard_mut_pointer<T>(pointer: *mut T) -> Result<(), ExtensionAbiError> {
    guard_pointer(pointer.cast_const())
}

fn validate_text(view: PksSessionUtf8, empty_allowed: bool) -> Result<Vec<u8>, ExtensionAbiError> {
    let value = bytes(view)?;
    if (!empty_allowed && value.is_empty()) || std::str::from_utf8(&value).is_err() {
        return Err(ExtensionAbiError::InvalidArgument);
    }
    Ok(value)
}

fn bytes(view: PksSessionUtf8) -> Result<Vec<u8>, ExtensionAbiError> {
    if view.len_bytes > MAX_IDENTIFIER_BYTES {
        return Err(ExtensionAbiError::InvalidArgument);
    }
    if view.len_bytes == 0 {
        return Ok(Vec::new());
    }
    guard_pointer(view.data)?;
    // SAFETY: the C contract keeps the declared bytes readable for this call.
    let value = unsafe { slice::from_raw_parts(view.data, view.len_bytes as usize) };
    Ok(value.to_vec())
}

#[derive(Clone, Copy)]
enum ExtensionAbiError {
    Null,
    Misaligned,
    UnsupportedMajor,
    UnsupportedMinor,
    InvalidSize,
    InvalidArgument,
}

impl ExtensionAbiError {
    const fn status(self) -> PksSessionStatus {
        let code = match self {
            Self::Null => PksSessionStatusCode::NullArgument,
            Self::Misaligned => PksSessionStatusCode::MisalignedPointer,
            Self::UnsupportedMajor => PksSessionStatusCode::UnsupportedAbiMajor,
            Self::UnsupportedMinor => PksSessionStatusCode::UnsupportedAbiMinor,
            Self::InvalidSize => PksSessionStatusCode::InvalidStructSize,
            Self::InvalidArgument => PksSessionStatusCode::InvalidArgument,
        };
        PksSessionStatus::new(code, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn text(value: &'static [u8]) -> PksSessionUtf8 {
        PksSessionUtf8 {
            data: value.as_ptr(),
            len_bytes: value.len() as u32,
        }
    }

    #[test]
    fn given_core_extension_c_descriptor_when_validated_then_version_and_ports_pass() {
        let port = PksExtensionPort {
            struct_size_bytes: size_of::<PksExtensionPort>() as u32,
            abi_major: PKS_EXTENSION_ABI_MAJOR,
            abi_minor: PKS_EXTENSION_ABI_MINOR,
            direction: PksExtensionPortDirection::Output as u32,
            required: 1,
            name: text(b"out"),
            signal_id: text(b"dev.pocketstation.fixture.v1"),
            semantic_role: text(b""),
            schema: text(b"urn:pocketstation:fixture:v1"),
        };
        let descriptor = PksExtensionDescriptor {
            struct_size_bytes: size_of::<PksExtensionDescriptor>() as u32,
            abi_major: PKS_EXTENSION_ABI_MAJOR,
            abi_minor: PKS_EXTENSION_ABI_MINOR,
            kind: PksExtensionKind::Source as u32,
            revision: 1,
            generation: 1,
            port_count: 1,
            extension_id: text(b"dev.pocketstation.source.fixture.v1"),
        };
        // SAFETY: test records and their byte views remain live for the call.
        let status = unsafe { pks_extension_descriptor_validate(&descriptor, &port, 1) };
        assert_eq!(status, PksSessionStatus::ok());
    }
}
