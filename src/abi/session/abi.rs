use core::mem::size_of;

pub const PKS_SESSION_ABI_MAJOR: u16 = 1;
pub const PKS_SESSION_ABI_MINOR: u16 = 1;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PksSessionAbiVersion {
    pub struct_size_bytes: u32,
    pub abi_major: u16,
    pub abi_minor: u16,
}

impl PksSessionAbiVersion {
    pub const fn current() -> Self {
        Self {
            struct_size_bytes: size_of::<Self>() as u32,
            abi_major: PKS_SESSION_ABI_MAJOR,
            abi_minor: PKS_SESSION_ABI_MINOR,
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PksSessionHandle {
    pub kind: PksSessionHandleKind,
    pub slot_index: u32,
    pub generation: u64,
    pub scope_id: u64,
}

impl PksSessionHandle {
    #[allow(dead_code)]
    pub const fn invalid() -> Self {
        Self {
            kind: PksSessionHandleKind::Invalid,
            slot_index: u32::MAX,
            generation: 0,
            scope_id: 0,
        }
    }
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PksSessionHandleKind {
    #[allow(dead_code)]
    Invalid = 0,
    Engine = 1,
    Session = 2,
    AudioBatch = 3,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PksSessionStatus {
    pub code: u32,
    pub detail: u32,
}

impl PksSessionStatus {
    pub const fn ok() -> Self {
        Self {
            code: PksSessionStatusCode::Ok as u32,
            detail: 0,
        }
    }

    pub const fn new(code: PksSessionStatusCode, detail: u32) -> Self {
        Self {
            code: code as u32,
            detail,
        }
    }
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PksSessionStatusCode {
    Ok = 0,
    NullArgument = 1,
    #[allow(dead_code)]
    BufferTooSmall = 2,
    UnsupportedAbiMajor = 3,
    InvalidStructSize = 4,
    InvalidHandle = 5,
    StaleHandle = 6,
    NoCapacity = 7,
    InternalPanic = 8,
    MisalignedPointer = 9,
    InvalidArgument = 10,
    ForeignHandle = 11,
    InvalidLifecycleState = 12,
    WouldBlock = 13,
    BackendFailure = 14,
    Cancelled = 15,
    IndexOutOfRange = 16,
    UnsupportedAbiMinor = 17,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PksSessionUtf8 {
    pub data: *const u8,
    pub len_bytes: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PksSessionEngineConfig {
    pub struct_size_bytes: u32,
    pub abi_major: u16,
    pub abi_minor: u16,
    pub source_queue_capacity_frames: u32,
    pub capture_frame_capacity_frames: u32,
    pub capture_runtime_event_capacity_count: u32,
    pub runtime_work_budget_frames: u32,
    pub runtime_idle_poll_ms: u64,
    pub runtime_ready_timeout_ms: u64,
    pub session_event_capacity_count: u32,
    pub audio_queue_capacity_frames: u32,
    pub audio_max_batch_frames: u32,
    pub audio_max_outstanding_leases: u32,
}

impl Default for PksSessionEngineConfig {
    fn default() -> Self {
        Self {
            struct_size_bytes: size_of::<Self>() as u32,
            abi_major: PKS_SESSION_ABI_MAJOR,
            abi_minor: PKS_SESSION_ABI_MINOR,
            source_queue_capacity_frames: 32,
            capture_frame_capacity_frames: 32,
            capture_runtime_event_capacity_count: 8,
            runtime_work_budget_frames: 64,
            runtime_idle_poll_ms: 1,
            runtime_ready_timeout_ms: 1_000,
            session_event_capacity_count: 32,
            audio_queue_capacity_frames: 32,
            audio_max_batch_frames: 8,
            audio_max_outstanding_leases: 4,
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PksSessionAppMicDeclaration {
    pub struct_size_bytes: u32,
    pub abi_major: u16,
    pub abi_minor: u16,
    pub application_name: PksSessionUtf8,
}

impl PksSessionAppMicDeclaration {
    #[allow(dead_code)]
    pub const fn new(application_name: PksSessionUtf8) -> Self {
        Self {
            struct_size_bytes: size_of::<Self>() as u32,
            abi_major: PKS_SESSION_ABI_MAJOR,
            abi_minor: PKS_SESSION_ABI_MINOR,
            application_name,
        }
    }
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PksSessionLifecycleState {
    Draft = 0,
    Compiled = 1,
    Running = 2,
    Stopped = 3,
    Failed = 4,
    Starting = 5,
    Stopping = 6,
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PksSessionEventKind {
    Lifecycle = 0,
    SourceFailure = 1,
    EndpointFailure = 2,
    RollbackFailure = 3,
    FinalizationFailure = 4,
    Terminal = 5,
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PksSessionSampleFormat {
    F32Interleaved = 1,
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PksSessionEndpointObservationStage {
    Unavailable = 0,
    Live = 1,
    Finalized = 2,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PksSessionEvent {
    pub struct_size_bytes: u32,
    pub abi_major: u16,
    pub abi_minor: u16,
    pub kind: u32,
    pub lifecycle_state: u32,
    pub reserved: u32,
    pub session_id: u64,
    pub stem_id: u64,
    pub endpoint_id: u64,
    pub route_id: u64,
    pub failures_total: u64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PksSessionMetricsSnapshot {
    pub struct_size_bytes: u32,
    pub abi_major: u16,
    pub abi_minor: u16,
    pub reserved: u32,
    pub event_capacity_count: u64,
    pub event_depth_count: u64,
    pub event_peak_depth_count: u64,
    pub events_enqueued_total: u64,
    pub events_dropped_total: u64,
    pub event_receiver_closed_total: u64,
    pub audio_queue_capacity_frames: u64,
    pub audio_queue_depth_frames: u64,
    pub audio_queue_peak_frames: u64,
    pub audio_frames_received_total: u64,
    pub audio_frames_delivered_total: u64,
    pub audio_queue_full_drops_total: u64,
    pub audio_invalid_ownership_drops_total: u64,
    pub audio_lease_capacity_count: u64,
    pub audio_outstanding_leases_count: u64,
    pub audio_lease_exhausted_total: u64,
    pub audio_batches_polled_total: u64,
    pub audio_frames_polled_total: u64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PksSessionSourceMetrics {
    pub struct_size_bytes: u32,
    pub abi_major: u16,
    pub abi_minor: u16,
    pub stem_id: u64,
    pub capture_callback_buffers_total: u64,
    pub capture_frames_enqueued_total: u64,
    pub capture_pool_exhausted_total: u64,
    pub capture_dispatch_queue_full_total: u64,
    pub capture_invalid_buffer_total: u64,
    pub capture_oversized_buffer_total: u64,
    pub capture_stream_errors_total: u64,
    pub capture_timestamp_epoch_clamps_total: u64,
    pub capture_stream_delivered_frames_total: u64,
    pub capture_stream_dropped_newest_frames_total: u64,
    pub capture_runtime_events_enqueued_total: u64,
    pub capture_runtime_events_dropped_total: u64,
    pub ingress_queue_capacity_frames: u64,
    pub ingress_queue_depth_frames: u64,
    pub ingress_queue_peak_frames: u64,
    pub ingress_frames_enqueued_total: u64,
    pub ingress_frames_delivered_total: u64,
    pub ingress_frames_rejected_full_total: u64,
    pub ingress_frames_rejected_cancelled_total: u64,
    pub ingress_frames_discarded_total: u64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PksSessionRouteMetrics {
    pub struct_size_bytes: u32,
    pub abi_major: u16,
    pub abi_minor: u16,
    pub route_id: u64,
    pub endpoint_id: u64,
    pub edge_queue_capacity_frames: u64,
    pub edge_queue_depth_frames: u64,
    pub edge_queue_peak_frames: u64,
    pub edge_frames_enqueued_total: u64,
    pub edge_frames_delivered_total: u64,
    pub edge_frames_dropped_total: u64,
    pub edge_overruns_total: u64,
    pub edge_receiver_unavailable_drops_total: u64,
    pub edge_queue_full_drops_total: u64,
    pub edge_shared_reference_exhausted_drops_total: u64,
    pub edge_branch_pool_exhausted_drops_total: u64,
    pub edge_invalid_copy_policy_drops_total: u64,
    pub edge_freeze_failed_drops_total: u64,
    pub edge_discontinuities_total: u64,
    pub edge_source_identity_discontinuities_total: u64,
    pub edge_sequence_discontinuities_total: u64,
    pub edge_timestamp_discontinuities_total: u64,
    pub edge_lineage_epoch_discontinuities_total: u64,
    pub edge_manually_reported_discontinuities_total: u64,
    pub edge_enqueue_to_receive_samples_total: u64,
    pub edge_enqueue_to_receive_invalid_order_total: u64,
    pub edge_enqueue_to_receive_p50_ns: u64,
    pub edge_enqueue_to_receive_p95_ns: u64,
    pub edge_enqueue_to_receive_p99_ns: u64,
    pub edge_enqueue_to_receive_max_ns: u64,
    pub edge_source_timestamp_to_receive_samples_total: u64,
    pub edge_source_timestamp_to_receive_missing_total: u64,
    pub edge_source_timestamp_to_receive_future_total: u64,
    pub edge_source_timestamp_to_receive_p50_ns: u64,
    pub edge_source_timestamp_to_receive_p95_ns: u64,
    pub edge_source_timestamp_to_receive_p99_ns: u64,
    pub edge_source_timestamp_to_receive_max_ns: u64,
    pub edge_worker_failures_total: u64,
    pub edge_shutdown_discarded_total: u64,
    pub endpoint_frames_received_total: u64,
    pub endpoint_frames_delivered_total: u64,
    pub endpoint_frames_dropped_total: u64,
    pub endpoint_discontinuities_total: u64,
    pub endpoint_failures_total: u64,
    pub endpoint_observation_stage: u32,
    pub reserved: u32,
    pub endpoint_finalization_failures_total: u64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PksSessionAudioBatch {
    pub struct_size_bytes: u32,
    pub abi_major: u16,
    pub abi_minor: u16,
    pub frame_count: u32,
    pub reserved: u32,
    pub handle: PksSessionHandle,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PksSessionAudioFrame {
    pub struct_size_bytes: u32,
    pub abi_major: u16,
    pub abi_minor: u16,
    pub sample_format: PksSessionSampleFormat,
    pub sample_rate_hz: u32,
    pub channel_count: u32,
    pub reserved: u32,
    pub session_id: u64,
    pub source_id: u64,
    pub stem_id: u64,
    pub clock_id: u64,
    pub sequence_num: u64,
    pub timestamp_start_ns: u64,
    pub duration_ns: u64,
    pub source_generation: u32,
    pub discontinuity_epoch: u64,
    pub permission_epoch: u64,
    pub endpoint_id: u64,
    pub connector_id: u64,
    pub route_id: u64,
    pub samples: *const f32,
    pub sample_count: u32,
    pub reserved_tail: u32,
}
