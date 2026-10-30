use std::mem::size_of;
use std::ptr;

use super::records::{PksSessionProcessDiscovery, PksSessionProcessInstance};
use super::{
    check_disjoint, checked_range, checked_span, process_error, validate_header, version_header,
};
use crate::abi::session::{abi_call, AbiError, PksSessionStatus, PksSessionUtf8};

/// Copies one coherent native discovery into caller-owned record/key buffers.
/// A size-query retry performs a fresh discovery; no cross-call snapshot is implied.
/// No audio opens. On NoCapacity only the required counts are written.
///
/// # Safety
/// `output` must address an initialized ABI 1.2 descriptor. Nonempty record and
/// key buffers must be valid writable, disjoint regions, also disjoint from the
/// descriptor. Before an adequate call, each required row has an initialized
/// version/size header; its remaining fields need not be initialized. Key views
/// remain valid only while the caller retains its arena without mutation.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn pks_session_discover_process_instances(
    output: *mut PksSessionProcessDiscovery,
) -> PksSessionStatus {
    abi_call(|| {
        // SAFETY: Forward the documented initialized descriptor header contract.
        unsafe { validate_header(output.cast_const()) }?;
        // SAFETY: These are initialized input fields; output-only counts are not read.
        let (members, capacity_count, stride_bytes, key_bytes, key_capacity_bytes, reserved) = unsafe {
            (
                ptr::addr_of!((*output).members).read(),
                ptr::addr_of!((*output).member_capacity_count).read() as usize,
                ptr::addr_of!((*output).member_stride_bytes).read() as usize,
                ptr::addr_of!((*output).key_bytes).read(),
                ptr::addr_of!((*output).key_capacity_bytes).read() as usize,
                ptr::addr_of!((*output).reserved_zero).read(),
            )
        };
        if reserved != 0 {
            return Err(AbiError::InvalidArgument);
        }
        let member_span_bytes = checked_span(members.cast_const(), capacity_count, stride_bytes)?;
        check_disjoint(&[
            checked_range(output as usize, size_of::<PksSessionProcessDiscovery>())?,
            checked_range(members as usize, member_span_bytes)?,
            checked_range(key_bytes as usize, key_capacity_bytes)?,
        ])?;
        let observed = crate::discover_process_instances().map_err(process_error)?;
        // SAFETY: Descriptor/ranges have been checked and the caller supplies
        // initialized row headers for an adequate result buffer.
        unsafe {
            write_discovery(
                output,
                members,
                capacity_count,
                stride_bytes,
                key_bytes,
                key_capacity_bytes,
                &observed,
            )
        }
    })
}

/// # Safety
/// The descriptor and disjoint arenas must satisfy the exported operation's
/// validated writable extents. Adequate rows require initialized headers.
pub(super) unsafe fn write_discovery(
    output: *mut PksSessionProcessDiscovery,
    members: *mut PksSessionProcessInstance,
    capacity_count: usize,
    stride_bytes: usize,
    key_bytes: *mut u8,
    key_capacity_bytes: usize,
    observed: &[crate::ObservedProcessInstance],
) -> Result<PksSessionStatus, AbiError> {
    let required_member_count = u32::try_from(observed.len()).map_err(|_| AbiError::NoCapacity)?;
    let required_key_bytes = observed.iter().try_fold(0_usize, |size_bytes, member| {
        let key_len_bytes = member.stable_id().stable_key.len();
        if key_len_bytes > super::MAX_STABLE_KEY_BYTES {
            return Err(AbiError::InvalidProcessSet);
        }
        size_bytes
            .checked_add(key_len_bytes)
            .ok_or(AbiError::NoCapacity)
    })?;
    let required_key_bytes_u32 =
        u32::try_from(required_key_bytes).map_err(|_| AbiError::NoCapacity)?;
    // SAFETY: The caller operation validated the full writable descriptor and
    // disjoint regions. Only initialized output counts are assigned here.
    unsafe {
        ptr::addr_of_mut!((*output).required_member_count).write(required_member_count);
        ptr::addr_of_mut!((*output).required_key_bytes).write(required_key_bytes_u32);
    }
    if capacity_count < observed.len() || key_capacity_bytes < required_key_bytes {
        return Err(AbiError::NoCapacity);
    }
    // Validate every row header before modifying any row or key byte.
    for member_index in 0..observed.len() {
        // SAFETY: checked_span established bounded aligned row arithmetic.
        let row = unsafe {
            members
                .cast::<u8>()
                .add(member_index * stride_bytes)
                .cast::<PksSessionProcessInstance>()
        };
        // SAFETY: Adequate calls require every row's initialized readable header.
        let header = unsafe { validate_header(row.cast_const()) }?;
        if header.struct_size_bytes as usize > stride_bytes {
            return Err(AbiError::InvalidArgument);
        }
    }
    let mut offset_bytes = 0;
    for (member_index, member) in observed.iter().enumerate() {
        let key = member.stable_id().stable_key.as_bytes();
        let (struct_size_bytes, abi_major, abi_minor) =
            version_header(size_of::<PksSessionProcessInstance>());
        // SAFETY: All rows/arenas were capacity, alignment, version and overlap
        // checked before any write. Every key is copied; no Rust view is returned.
        unsafe {
            let key_output = key_bytes.add(offset_bytes);
            ptr::copy_nonoverlapping(key.as_ptr(), key_output, key.len());
            let row = members
                .cast::<u8>()
                .add(member_index * stride_bytes)
                .cast::<PksSessionProcessInstance>();
            row.write(PksSessionProcessInstance {
                struct_size_bytes,
                abi_major,
                abi_minor,
                process_id: member.process_id().get(),
                platform: super::platform_to_code(member.stable_id().platform),
                process_start_time_ns: member.process_start_time_ns(),
                stable_key: PksSessionUtf8 {
                    data: key_output,
                    len_bytes: key.len() as u32,
                },
            });
        }
        offset_bytes += key.len();
    }
    Ok(PksSessionStatus::ok())
}
