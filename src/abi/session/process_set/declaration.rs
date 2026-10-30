use std::mem::size_of;
use std::slice;

use super::records::{PksSessionProcessInstance, PksSessionProcessSetDeclaration};
use super::{
    check_disjoint, checked_range, checked_span, platform_from_code, process_error, validate_header,
};
use crate::abi::session::{
    abi_call, runtime_state, write_record, AbiError, PksSessionHandle, PksSessionStatus,
};

/// Copies an exact 1–32-member original observation set into a new Session.
/// This selected-only constructor uses 48 kHz stereo/20 ms and opens no mic.
/// Compile/start remain explicit; declaration does not start playback/capture.
///
/// # Safety
/// Input records/UTF-8 views must remain initialized, readable and immutable for
/// this call. The output handle must be writable/aligned and disjoint from all
/// input regions. Caller-owned discovery key bytes must remain alive until this
/// function returns; all selected values are copied before returning.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn pks_session_create_process_set(
    engine: PksSessionHandle,
    declaration: *const PksSessionProcessSetDeclaration,
    output_session: *mut PksSessionHandle,
) -> PksSessionStatus {
    abi_call(|| {
        super::super::guard_pointer_alignment(output_session.cast_const())?;
        // SAFETY: Forward the documented initialized bounded declaration contract.
        let selected = unsafe { copy_selection(declaration, output_session) }?;
        let session = runtime_state().create_process_set_session(engine, selected)?;
        // SAFETY: Output was validated and all borrowed inputs have been copied.
        unsafe { write_record(output_session, session) }?;
        Ok(PksSessionStatus::ok())
    })
}

pub(super) unsafe fn copy_selection(
    declaration: *const PksSessionProcessSetDeclaration,
    output_session: *mut PksSessionHandle,
) -> Result<crate::ApplicationProcessSet, AbiError> {
    // SAFETY: Only the initialized header is read before its size is accepted.
    let declaration_header = unsafe { validate_header(declaration) }?;
    // SAFETY: Header validation establishes the complete readable input record.
    let input = unsafe { declaration.read() };
    let member_count = input.member_count as usize;
    if !(1..=crate::MAX_APPLICATION_PROCESS_SET_MEMBERS).contains(&member_count) {
        return Err(AbiError::InvalidProcessSet);
    }
    let stride_bytes = input.member_stride_bytes as usize;
    let span_bytes = checked_span(input.members, member_count, stride_bytes)?;
    let output_range = checked_range(output_session as usize, size_of::<PksSessionHandle>())?;
    check_disjoint(&[
        output_range,
        checked_range(
            declaration as usize,
            declaration_header.struct_size_bytes as usize,
        )?,
        checked_range(input.members as usize, span_bytes)?,
    ])?;
    let mut copied = Vec::with_capacity(member_count);
    for member_index in 0..member_count {
        // SAFETY: checked_span established all bounded aligned row addresses.
        let pointer = unsafe {
            input
                .members
                .cast::<u8>()
                .add(member_index * stride_bytes)
                .cast::<PksSessionProcessInstance>()
        };
        // SAFETY: Each caller row has an initialized readable version header.
        let header = unsafe { validate_header(pointer) }?;
        if header.struct_size_bytes as usize > stride_bytes {
            return Err(AbiError::InvalidArgument);
        }
        check_disjoint(&[
            output_range,
            checked_range(pointer as usize, header.struct_size_bytes as usize)?,
        ])?;
        // SAFETY: The checked row header declares a complete initialized member.
        let member = unsafe { pointer.read() };
        let key_len_bytes = member.stable_key.len_bytes as usize;
        if key_len_bytes == 0 || key_len_bytes > super::MAX_STABLE_KEY_BYTES {
            return Err(AbiError::InvalidProcessSet);
        }
        check_disjoint(&[
            output_range,
            checked_range(member.stable_key.data as usize, key_len_bytes)?,
        ])?;
        // SAFETY: The caller provides exactly the validated readable key region;
        // it is copied before any output or Session mutation, never retained.
        let key = unsafe { slice::from_raw_parts(member.stable_key.data, key_len_bytes) };
        let key = std::str::from_utf8(key)
            .map_err(|_| AbiError::InvalidProcessSet)?
            .to_owned();
        copied.push(
            crate::ObservedProcessInstance::new(
                crate::ProcessId::new(member.process_id),
                crate::StableSourceId::new(
                    platform_from_code(member.platform)?,
                    crate::SourceKind::Application,
                    key,
                ),
                member.process_start_time_ns,
            )
            .map_err(process_error)?,
        );
    }
    crate::ApplicationProcessSet::new(copied).map_err(process_error)
}
