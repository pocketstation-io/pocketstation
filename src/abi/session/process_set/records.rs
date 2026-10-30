//! Additive ABI 1.2 records. Existing ABI records and discriminants are unchanged.

use crate::abi::session::abi::PksSessionUtf8;

/// Detail codes are scoped to the existing BackendFailure/InvalidArgument status.
pub const PROCESS_SET_UNSUPPORTED: u32 = 1;
pub const PROCESS_SET_INVALID: u32 = 2;
pub const PROCESS_SET_SOURCE_UNAVAILABLE: u32 = 3;
pub const PROCESS_SET_PERMISSION_DENIED: u32 = 4;
pub const PROCESS_SET_CLEANUP_UNCONFIRMED: u32 = 5;
pub const PROCESS_SET_BACKEND_FAILED: u32 = 6;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct PksSessionProcessInstance {
    pub struct_size_bytes: u32,
    pub abi_major: u16,
    pub abi_minor: u16,
    pub process_id: u32,
    pub platform: u32,
    pub process_start_time_ns: u64,
    pub stable_key: PksSessionUtf8,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct PksSessionProcessDiscovery {
    pub struct_size_bytes: u32,
    pub abi_major: u16,
    pub abi_minor: u16,
    pub members: *mut PksSessionProcessInstance,
    pub member_capacity_count: u32,
    pub member_stride_bytes: u32,
    pub key_bytes: *mut u8,
    pub key_capacity_bytes: u32,
    pub required_member_count: u32,
    pub required_key_bytes: u32,
    pub reserved_zero: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct PksSessionProcessSetDeclaration {
    pub struct_size_bytes: u32,
    pub abi_major: u16,
    pub abi_minor: u16,
    pub members: *const PksSessionProcessInstance,
    pub member_count: u32,
    pub member_stride_bytes: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct PksSessionExternalSourceMetrics {
    pub struct_size_bytes: u32,
    pub abi_major: u16,
    pub abi_minor: u16,
    pub source_instance_id: u64,
    pub source_id: u64,
    pub emitted_total: u64,
    pub dropped_total: u64,
    pub failure_total: u64,
    pub cancellation_total: u64,
    pub discontinuity_total: u64,
    pub recovery_total: u64,
    pub policy_change_total: u64,
    pub ready: u32,
    pub joined: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct PksSessionStopOutcome {
    pub struct_size_bytes: u32,
    pub abi_major: u16,
    pub abi_minor: u16,
    pub success: u32,
    pub runtime_worker_panicked: u32,
    pub capture_finalization_failures_total: u64,
    pub operator_finalization_failures_total: u64,
    pub endpoint_finalization_failures_total: u64,
    pub runtime_failures_total: u64,
    pub lineage_failures_total: u64,
    pub source_send_rejections_total: u64,
    pub runtime_events_total: u64,
    pub cleanup_unconfirmed: u32,
    pub reserved_zero: u32,
}
