use super::records::*;
use super::*;
use crate::abi::session::abi::{PksSessionHandle, PksSessionUtf8};
use std::ptr;

fn member() -> PksSessionProcessInstance {
    PksSessionProcessInstance {
        struct_size_bytes: size_of::<PksSessionProcessInstance>() as u32,
        abi_major: 1,
        abi_minor: 2,
        process_id: 7,
        platform: 1,
        process_start_time_ns: u64::MAX,
        stable_key: PksSessionUtf8 {
            data: b"key".as_ptr(),
            len_bytes: 3,
        },
    }
}
fn declaration(rows: &[PksSessionProcessInstance]) -> PksSessionProcessSetDeclaration {
    PksSessionProcessSetDeclaration {
        struct_size_bytes: size_of::<PksSessionProcessSetDeclaration>() as u32,
        abi_major: 1,
        abi_minor: 2,
        members: rows.as_ptr(),
        member_count: rows.len() as u32,
        member_stride_bytes: size_of::<PksSessionProcessInstance>() as u32,
    }
}
fn selection(
    input: &PksSessionProcessSetDeclaration,
) -> Result<crate::ApplicationProcessSet, AbiError> {
    let mut output = PksSessionHandle::invalid();
    // SAFETY: Tests provide owned readable descriptors/rows/keys, or inputs whose
    // invalid bounds are rejected before any dereference.
    unsafe { declaration::copy_selection(input, &mut output) }
}

#[test]
fn given_original_u64_observation_when_copied_then_full_precision_is_preserved() {
    let rows = [member()];
    let selected = selection(&declaration(&rows)).expect("exact copied selection");
    assert_eq!(selected.members()[0].process_start_time_ns(), u64::MAX);
}
#[test]
fn given_empty_or_oversized_set_when_declared_then_bounds_fail_before_rows_are_read() {
    let mut input = declaration(&[]);
    assert!(matches!(
        selection(&input),
        Err(AbiError::InvalidProcessSet)
    ));
    input.member_count = 33;
    input.members = ptr::null();
    assert!(matches!(
        selection(&input),
        Err(AbiError::InvalidProcessSet)
    ));
}
#[test]
fn given_null_or_invalid_utf8_key_when_declared_then_selection_is_rejected() {
    let mut row = member();
    row.stable_key.data = ptr::null();
    assert!(matches!(
        selection(&declaration(&[row])),
        Err(AbiError::NullArgument)
    ));
    let invalid_key = [255_u8];
    row.stable_key = PksSessionUtf8 {
        data: invalid_key.as_ptr(),
        len_bytes: 1,
    };
    assert!(matches!(
        selection(&declaration(&[row])),
        Err(AbiError::InvalidProcessSet)
    ));
}
#[test]
fn given_invalid_platform_or_zero_start_when_declared_then_typed_validation_fails() {
    for row in [
        PksSessionProcessInstance {
            platform: 99,
            ..member()
        },
        PksSessionProcessInstance {
            process_start_time_ns: 0,
            ..member()
        },
    ] {
        assert!(matches!(
            selection(&declaration(&[row])),
            Err(AbiError::InvalidProcessSet)
        ));
    }
}
#[test]
fn given_short_or_unaligned_stride_when_checked_then_no_row_view_is_created() {
    let row = member();
    for stride_bytes in [0, size_of::<PksSessionProcessInstance>() - 1, 41, 8192] {
        assert!(checked_span(&row, 2, stride_bytes).is_err());
    }
}
#[test]
fn given_extreme_span_or_address_when_checked_then_arithmetic_is_rejected() {
    assert!(checked_span(
        std::ptr::dangling::<PksSessionProcessInstance>(),
        usize::MAX,
        40
    )
    .is_err());
    assert!(checked_span((usize::MAX - 7) as *const PksSessionProcessInstance, 2, 40).is_err());
    assert!(checked_range(8, usize::MAX).is_err());
}
#[test]
fn given_overlapping_output_regions_when_checked_then_aliasing_is_rejected() {
    assert_eq!(
        check_disjoint(&[Some((8, 24)), Some((16, 32))]),
        Err(AbiError::InvalidArgument)
    );
    assert_eq!(check_disjoint(&[Some((8, 24)), Some((24, 32))]), Ok(()));
}
#[test]
fn given_old_unknown_or_short_header_when_checked_then_tail_is_not_read() {
    let mut row = member();
    for (abi_major, abi_minor, expected) in [
        (2, 2, AbiError::UnsupportedAbiMajor),
        (1, 3, AbiError::UnsupportedAbiMinor),
        (1, 1, AbiError::UnsupportedAbiMinor),
    ] {
        row.abi_major = abi_major;
        row.abi_minor = abi_minor;
        // SAFETY: row is an initialized owned record.
        assert_eq!(unsafe { validate_header(&row) }.unwrap_err(), expected);
    }
    row = member();
    row.struct_size_bytes = 8;
    // SAFETY: row is an initialized owned record.
    assert_eq!(
        unsafe { validate_header(&row) }.unwrap_err(),
        AbiError::InvalidStructSize
    );
}
#[test]
fn given_null_zero_capacity_discovery_when_empty_then_query_succeeds_without_arenas() {
    let mut output = PksSessionProcessDiscovery {
        struct_size_bytes: size_of::<PksSessionProcessDiscovery>() as u32,
        abi_major: 1,
        abi_minor: 2,
        members: ptr::null_mut(),
        member_capacity_count: 0,
        member_stride_bytes: 0,
        key_bytes: ptr::null_mut(),
        key_capacity_bytes: 0,
        required_member_count: 99,
        required_key_bytes: 99,
        reserved_zero: 0,
    };
    // SAFETY: Empty discovery writes only the valid descriptor, without arenas.
    assert!(unsafe {
        discovery::write_discovery(&mut output, ptr::null_mut(), 0, 0, ptr::null_mut(), 0, &[])
    }
    .is_ok());
    assert_eq!(
        (output.required_member_count, output.required_key_bytes),
        (0, 0)
    );
}
#[test]
fn given_nonempty_discovery_when_capacity_is_short_then_only_required_sizes_are_written() {
    let observed = crate::ObservedProcessInstance::new(
        crate::ProcessId::new(7),
        crate::StableSourceId::new(
            crate::Platform::Macos,
            crate::SourceKind::Application,
            "key",
        ),
        9,
    )
    .unwrap();
    // SAFETY: This descriptor contains only numeric fields and nullable raw pointers.
    let mut output: PksSessionProcessDiscovery = unsafe { std::mem::zeroed() };
    // SAFETY: Nonempty result with no capacity returns before touching either null arena.
    assert_eq!(
        unsafe {
            discovery::write_discovery(
                &mut output,
                ptr::null_mut(),
                0,
                0,
                ptr::null_mut(),
                0,
                &[observed],
            )
        }
        .unwrap_err(),
        AbiError::NoCapacity
    );
    assert_eq!(
        (output.required_member_count, output.required_key_bytes),
        (1, 3)
    );
}

fn descriptor() -> PksSessionProcessDiscovery {
    PksSessionProcessDiscovery {
        struct_size_bytes: size_of::<PksSessionProcessDiscovery>() as u32,
        abi_major: 1,
        abi_minor: 2,
        members: ptr::null_mut(),
        member_capacity_count: 0,
        member_stride_bytes: 0,
        key_bytes: ptr::null_mut(),
        key_capacity_bytes: 0,
        required_member_count: 0,
        required_key_bytes: 0,
        reserved_zero: 0,
    }
}
fn observed(process_id: u32) -> crate::ObservedProcessInstance {
    crate::ObservedProcessInstance::new(
        crate::ProcessId::new(process_id),
        crate::StableSourceId::new(
            crate::Platform::Macos,
            crate::SourceKind::Application,
            "key",
        ),
        9,
    )
    .unwrap()
}
#[test]
fn given_invalid_discovery_descriptor_when_called_then_validation_precedes_native_discovery() {
    let mut value = descriptor();
    value.reserved_zero = 1;
    // SAFETY: The descriptor is valid memory; invalid reserved values fail before acquisition.
    assert_eq!(
        unsafe { discovery::pks_session_discover_process_instances(&mut value) }.code,
        10
    );
    value.reserved_zero = 0;
    value.key_capacity_bytes = 1;
    // SAFETY: The invalid null arena is rejected before dereferencing or discovery.
    assert_eq!(
        unsafe { discovery::pks_session_discover_process_instances(&mut value) }.code,
        1
    );
    value.key_bytes = (&mut value as *mut PksSessionProcessDiscovery).cast();
    // SAFETY: Overlap is checked before any write into the descriptor or arena.
    assert_eq!(
        unsafe { discovery::pks_session_discover_process_instances(&mut value) }.code,
        10
    );
}
#[test]
fn given_later_invalid_discovery_row_when_copied_then_all_arenas_remain_unchanged() {
    let mut value = descriptor();
    let mut rows = [member(), member()];
    rows[1].abi_minor = 1;
    let mut keys = [0xaa_u8; 6];
    // SAFETY: Both owned arrays have adequate nonoverlapping capacity; row2 has
    // an intentionally invalid initialized header which must fail before writes.
    assert!(unsafe {
        discovery::write_discovery(
            &mut value,
            rows.as_mut_ptr(),
            2,
            size_of::<PksSessionProcessInstance>(),
            keys.as_mut_ptr(),
            keys.len(),
            &[observed(7), observed(8)],
        )
    }
    .is_err());
    assert_eq!(keys, [0xaa; 6]);
    assert_eq!(rows[0].process_start_time_ns, u64::MAX);
}
#[test]
fn given_adequate_discovery_arenas_when_copied_then_keys_are_owned_and_reusable() {
    let mut value = descriptor();
    let mut rows = [member()];
    let mut keys = [0_u8; 3];
    // SAFETY: Descriptor, initialized row and key arena are adequately sized and disjoint.
    unsafe {
        discovery::write_discovery(
            &mut value,
            rows.as_mut_ptr(),
            1,
            size_of::<PksSessionProcessInstance>(),
            keys.as_mut_ptr(),
            keys.len(),
            &[observed(7)],
        )
    }
    .unwrap();
    assert_eq!(rows[0].stable_key.data, keys.as_ptr());
    let selected = selection(&declaration(&rows)).unwrap();
    keys.fill(b'x');
    assert_eq!(selected.members()[0].stable_id().stable_key, "key");
}
#[test]
fn given_output_overlapping_input_when_declared_then_no_handle_write_occurs() {
    let mut rows = [member()];
    let input = declaration(&rows);
    // SAFETY: The aliased output is never written because overlap is validated
    // before state mutation; row memory itself is readable/aligned and initialized.
    assert!(matches!(
        unsafe { declaration::copy_selection(&input, rows.as_mut_ptr().cast()) },
        Err(AbiError::InvalidArgument)
    ));
    assert_eq!(rows[0].process_id, 7);
}

#[test]
fn given_strided_utf8_discovery_when_roundtripped_then_canaries_and_owned_values_are_preserved() {
    #[repr(C)]
    struct PaddedRow {
        member: PksSessionProcessInstance,
        canary: [u8; 8],
    }
    let mut rows = [
        PaddedRow {
            member: member(),
            canary: [0xa5; 8],
        },
        PaddedRow {
            member: member(),
            canary: [0x5a; 8],
        },
    ];
    let observations = ["café", "音"]
        .into_iter()
        .enumerate()
        .map(|(member_index, key)| {
            crate::ObservedProcessInstance::new(
                crate::ProcessId::new(7 + member_index as u32),
                crate::StableSourceId::new(
                    crate::Platform::Macos,
                    crate::SourceKind::Application,
                    key,
                ),
                u64::MAX,
            )
            .unwrap()
        })
        .collect::<Vec<_>>();
    let mut value = descriptor();
    let mut keys = [0_u8; 8];
    // SAFETY: repr(C) row alignment and stride preserve each initialized member
    // header; descriptor/rows/key arena are owned adequate disjoint buffers.
    unsafe {
        discovery::write_discovery(
            &mut value,
            rows.as_mut_ptr().cast(),
            2,
            size_of::<PaddedRow>(),
            keys.as_mut_ptr(),
            keys.len(),
            &observations,
        )
    }
    .unwrap();
    assert_eq!(rows[0].canary, [0xa5; 8]);
    assert_eq!(rows[1].canary, [0x5a; 8]);
    for row in &rows {
        assert!(
            (keys.as_ptr() as usize..keys.as_ptr() as usize + keys.len())
                .contains(&(row.member.stable_key.data as usize))
        );
        assert_eq!(row.member.process_start_time_ns, u64::MAX);
    }
    let mut input = declaration(&[]);
    input.members = rows.as_ptr().cast();
    input.member_count = 2;
    input.member_stride_bytes = size_of::<PaddedRow>() as u32;
    let selected = selection(&input).unwrap();
    keys.fill(0);
    assert_eq!(selected.members()[0].stable_id().stable_key, "café");
    assert_eq!(selected.members()[1].stable_id().stable_key, "音");
}
#[test]
fn given_conflicting_identity_members_when_declared_then_actual_core_validation_rejects_them() {
    let first = member();
    for second in [
        member(),
        PksSessionProcessInstance {
            process_id: 8,
            platform: 2,
            ..member()
        },
        PksSessionProcessInstance {
            process_id: 0,
            ..member()
        },
        PksSessionProcessInstance {
            process_id: i32::MAX as u32 + 1,
            ..member()
        },
        PksSessionProcessInstance {
            process_id: 8,
            stable_key: PksSessionUtf8 {
                data: ptr::null(),
                len_bytes: 4097,
            },
            ..member()
        },
    ] {
        assert!(matches!(
            selection(&declaration(&[first, second])),
            Err(AbiError::InvalidProcessSet)
        ));
    }
}
