//! Bounded ABI projection of Core's existing observed-process capture primitive.

mod declaration;
mod discovery;
mod observations;
pub mod records;

#[cfg(test)]
mod tests;

use std::mem::{align_of, size_of};

use super::abi::{PksSessionAbiVersion, PKS_SESSION_ABI_MAJOR, PKS_SESSION_ABI_MINOR};
use super::error::AbiError;

const MAX_STABLE_KEY_BYTES: usize = 4_096;
const MAX_RECORD_STRIDE_BYTES: usize = 4_096;

/// Reads only the initialized common header, including for output-only records.
///
/// # Safety
/// The pointer must address a readable header and the declared writable/readable
/// record extent for the calling operation. No caller memory is retained.
unsafe fn validate_header<T>(pointer: *const T) -> Result<PksSessionAbiVersion, AbiError> {
    super::guard_pointer_alignment(pointer)?;
    // SAFETY: The entry point requires an initialized, readable eight-byte header.
    let header = unsafe { pointer.cast::<PksSessionAbiVersion>().read() };
    super::guard_abi_version(header.abi_major, header.abi_minor)?;
    if header.abi_minor < 2 {
        return Err(AbiError::UnsupportedAbiMinor);
    }
    super::guard_record_size(header.struct_size_bytes, size_of::<T>())?;
    Ok(header)
}

fn checked_range(pointer: usize, size_bytes: usize) -> Result<Option<(usize, usize)>, AbiError> {
    if size_bytes == 0 {
        return Ok(None);
    }
    if pointer == 0 {
        return Err(AbiError::NullArgument);
    }
    if size_bytes > isize::MAX as usize {
        return Err(AbiError::InvalidArgument);
    }
    let end = pointer
        .checked_add(size_bytes)
        .ok_or(AbiError::InvalidArgument)?;
    Ok(Some((pointer, end)))
}

fn check_disjoint(ranges: &[Option<(usize, usize)>]) -> Result<(), AbiError> {
    for (index, range) in ranges.iter().enumerate() {
        if let Some((start, end)) = range {
            for (other_start, other_end) in ranges.iter().skip(index + 1).flatten() {
                if start < other_end && other_start < end {
                    return Err(AbiError::InvalidArgument);
                }
            }
        }
    }
    Ok(())
}

fn checked_span<T>(
    pointer: *const T,
    member_count: usize,
    stride_bytes: usize,
) -> Result<usize, AbiError> {
    if member_count == 0 {
        return Ok(0);
    }
    super::guard_pointer_alignment(pointer)?;
    if stride_bytes < size_of::<T>()
        || stride_bytes > MAX_RECORD_STRIDE_BYTES
        || !stride_bytes.is_multiple_of(align_of::<T>())
    {
        return Err(AbiError::InvalidArgument);
    }
    let size_bytes = (member_count - 1)
        .checked_mul(stride_bytes)
        .and_then(|offset_bytes| offset_bytes.checked_add(size_of::<T>()))
        .ok_or(AbiError::InvalidArgument)?;
    checked_range(pointer as usize, size_bytes)?;
    Ok(size_bytes)
}

fn version_header(size_bytes: usize) -> (u32, u16, u16) {
    (
        size_bytes as u32,
        PKS_SESSION_ABI_MAJOR,
        PKS_SESSION_ABI_MINOR,
    )
}

pub(super) fn process_error(error: crate::ProcessSetError) -> AbiError {
    use records::*;
    match error.code() {
        "session.invalid_process_set" => AbiError::InvalidProcessSet,
        "capture.unsupported" => AbiError::ProcessSetFailure {
            detail_code: PROCESS_SET_UNSUPPORTED,
        },
        "capture.source_unavailable" => AbiError::ProcessSetFailure {
            detail_code: PROCESS_SET_SOURCE_UNAVAILABLE,
        },
        "capture.permission_denied" => AbiError::ProcessSetFailure {
            detail_code: PROCESS_SET_PERMISSION_DENIED,
        },
        "capture.cleanup_unconfirmed" => AbiError::ProcessSetFailure {
            detail_code: PROCESS_SET_CLEANUP_UNCONFIRMED,
        },
        _ => AbiError::ProcessSetFailure {
            detail_code: PROCESS_SET_BACKEND_FAILED,
        },
    }
}

fn platform_to_code(platform: crate::Platform) -> u32 {
    match platform {
        crate::Platform::Macos => 1,
        crate::Platform::Windows => 2,
        crate::Platform::Linux => 3,
        crate::Platform::Ios => 4,
        crate::Platform::Android => 5,
        crate::Platform::Web => 6,
        crate::Platform::Unknown => 7,
    }
}

fn platform_from_code(code: u32) -> Result<crate::Platform, AbiError> {
    match code {
        1 => Ok(crate::Platform::Macos),
        2 => Ok(crate::Platform::Windows),
        3 => Ok(crate::Platform::Linux),
        4 => Ok(crate::Platform::Ios),
        5 => Ok(crate::Platform::Android),
        6 => Ok(crate::Platform::Web),
        7 => Ok(crate::Platform::Unknown),
        _ => Err(AbiError::InvalidProcessSet),
    }
}
