//! Executable, typed-signal-only C extension projection.
//!
//! Foreign callbacks are deliberately admitted only to blocking/async Session
//! workers. PCM audio remains on the native fixed-capacity realtime lane and
//! cannot be registered through this ABI version.

use std::ffi::c_void;
use std::mem::{align_of, size_of};
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::slice;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;

use crate::abi::extension::{
    pks_extension_descriptor_validate, PksExtensionDescriptor, PksExtensionKind, PksExtensionPort,
    PksExtensionPortDirection, PKS_EXTENSION_ABI_MAJOR, PKS_EXTENSION_ABI_MINOR,
};
use crate::abi::session::{
    self, PksSessionHandle, PksSessionStatus, PksSessionStatusCode, PksSessionUtf8,
};
use crate::endpoint::{
    EndpointCancellationOutcome, EndpointDriverFactory, EndpointDriverFinalization,
    EndpointDriverObservations, EndpointFailure, EndpointFailureStage, EndpointPortInput,
    EndpointReceiver, EndpointStartGate, PreparedEndpointDriver, RunningEndpointDriver,
};
use crate::frame::{ClockDomainId, ConnectorId};
use crate::graph::{
    AsyncNode, AsyncNodeFuture, AsyncOperatorFactory, AsyncOperatorManifest,
    AsyncOperatorPrepareContext, BackpressurePolicy, BinaryFormat, ConfigError, CopyPolicy,
    EdgeContract, ExecutionPartition, MediaCaps, Multiplicity, NodeConfig, NodeDefinition,
    NodeDescriptor, NodeError, NodeTypeId, OperatorCancellationPolicy, OperatorDeadlinePolicy,
    OperatorFailurePolicy, OperatorId, OperatorOutputRolePolicy, OperatorPermissionPolicy,
    PortDirection, PortSpec, SafetyContract, SignalDerivation, SignalEnvelope, SignalLineage,
    SignalPayload, SignalSpec, SignalTiming,
};
use crate::session::{SourceConfiguration, SourceSessionContext, SourceTypeId};
use crate::{
    SourceCancellation, SourceDriver, SourceDriverError, SourceEmission, SourceFactory,
    SourceManifest, SourcePrepareContext,
};

const MAX_CALLBACK_PAYLOAD_BYTES: u32 = 1_048_576;
pub(crate) const MAX_LIBRARY_REGISTRATIONS: u32 = 64;
const SIGNAL_FLAG_END_OF_STREAM: u32 = 1;
const SIGNAL_FLAG_TERMINAL: u32 = 2;

#[doc = "Defines the optional C callback used to prepare an extension instance; pointer validity and ownership follow the extension ABI contract."]
pub type PksExtensionPrepareCallback =
    Option<unsafe extern "C-unwind" fn(context: *mut c_void) -> PksSessionStatus>;
#[doc = "Defines the optional C callback used to validate extension configuration; pointer validity and ownership follow the extension ABI contract."]
pub type PksExtensionValidateConfigurationCallback = Option<
    unsafe extern "C-unwind" fn(
        registration_context: *mut c_void,
        configuration: PksSessionUtf8,
    ) -> PksSessionStatus,
>;
#[doc = "Defines the optional C callback used to create an extension instance; pointer validity and ownership follow the extension ABI contract."]
pub type PksExtensionCreateCallback = Option<
    unsafe extern "C-unwind" fn(
        registration_context: *mut c_void,
        configuration: PksSessionUtf8,
        output_instance_context: *mut *mut c_void,
    ) -> PksSessionStatus,
>;
#[doc = "Defines the optional C callback used to produce the next source signal; pointer validity and ownership follow the extension ABI contract."]
pub type PksExtensionSourceNextCallback = Option<
    unsafe extern "C-unwind" fn(
        context: *mut c_void,
        cancellation_requested: u32,
        output: *mut PksExtensionSignalBuffer,
    ) -> PksSessionStatus,
>;
#[doc = "Defines the optional C callback used to process an operator input; pointer validity and ownership follow the extension ABI contract."]
pub type PksExtensionOperatorProcessCallback = Option<
    unsafe extern "C-unwind" fn(
        context: *mut c_void,
        input: *const PksExtensionSignalView,
        output: *mut PksExtensionSignalBuffer,
    ) -> PksSessionStatus,
>;
#[doc = "Defines the optional C callback used to consume an endpoint input; pointer validity and ownership follow the extension ABI contract."]
pub type PksExtensionEndpointConsumeCallback = Option<
    unsafe extern "C-unwind" fn(
        context: *mut c_void,
        input: *const PksExtensionSignalView,
    ) -> PksSessionStatus,
>;
#[doc = "Defines the optional C callback used to request an extension instance to stop; pointer validity and ownership follow the extension ABI contract."]
pub type PksExtensionStopCallback =
    Option<unsafe extern "C-unwind" fn(context: *mut c_void) -> PksSessionStatus>;
#[doc = "Defines the optional C callback used to finish extension work; pointer validity and ownership follow the extension ABI contract."]
pub type PksExtensionFinishCallback =
    Option<unsafe extern "C-unwind" fn(context: *mut c_void) -> PksSessionStatus>;
#[doc = "Defines the optional C callback used to destroy extension-owned context; pointer validity and ownership follow the extension ABI contract."]
pub type PksExtensionDestroyCallback = Option<unsafe extern "C-unwind" fn(context: *mut c_void)>;

#[repr(C)]
#[derive(Clone, Copy)]
#[doc = "Defines the optional function table through which a native extension prepares, runs, stops, and releases instances."]
pub struct PksExtensionCallbacks {
    #[doc = "Stores the byte size of the `PksExtensionCallbacks` ABI structure."]
    pub struct_size_bytes: u32,
    #[doc = "Stores the major ABI version expected by `PksExtensionCallbacks`."]
    pub abi_major: u16,
    #[doc = "Stores the minor ABI version expected by `PksExtensionCallbacks`."]
    pub abi_minor: u16,
    #[doc = "Carries the opaque registration context used by `PksExtensionCallbacks` callbacks."]
    pub registration_context: *mut c_void,
    #[doc = "Limits payload storage for `PksExtensionCallbacks`, in bytes."]
    pub max_payload_bytes: u32,
    #[doc = "Reserves storage for forward-compatible evolution of `PksExtensionCallbacks`."]
    pub reserved: u32,
    #[doc = "Provides the validate configuration callback used by `PksExtensionCallbacks`."]
    pub validate_configuration: PksExtensionValidateConfigurationCallback,
    #[doc = "Provides the create callback used by `PksExtensionCallbacks`."]
    pub create: PksExtensionCreateCallback,
    #[doc = "Provides the prepare callback used by `PksExtensionCallbacks`."]
    pub prepare: PksExtensionPrepareCallback,
    #[doc = "Provides the source next callback used by `PksExtensionCallbacks`."]
    pub source_next: PksExtensionSourceNextCallback,
    #[doc = "Provides the operator process callback used by `PksExtensionCallbacks`."]
    pub operator_process: PksExtensionOperatorProcessCallback,
    #[doc = "Provides the endpoint consume callback used by `PksExtensionCallbacks`."]
    pub endpoint_consume: PksExtensionEndpointConsumeCallback,
    #[doc = "Provides the request stop callback used by `PksExtensionCallbacks`."]
    pub request_stop: PksExtensionStopCallback,
    #[doc = "Provides the finish callback used by `PksExtensionCallbacks`."]
    pub finish: PksExtensionFinishCallback,
    #[doc = "Provides the destroy instance callback used by `PksExtensionCallbacks`."]
    pub destroy_instance: PksExtensionDestroyCallback,
    #[doc = "Provides the destroy registration callback used by `PksExtensionCallbacks`."]
    pub destroy_registration: PksExtensionDestroyCallback,
}

#[doc = "Defines the optional C callback used to acquire an extension registration; pointer validity and ownership follow the extension ABI contract."]
pub type PksExtensionAcquireRegistrationCallback = Option<
    unsafe extern "C-unwind" fn(
        library_context: *mut c_void,
        registration_index: u32,
        output_descriptor: *mut PksExtensionDescriptor,
        output_ports: *mut *const PksExtensionPort,
        output_port_count: *mut u32,
        output_callbacks: *mut PksExtensionCallbacks,
    ) -> PksSessionStatus,
>;

#[repr(C)]
#[derive(Clone, Copy)]
#[doc = "Owns a loaded native-extension library and the registrations imported from its validated descriptor."]
pub struct PksExtensionLibrary {
    #[doc = "Stores the byte size of the `PksExtensionLibrary` ABI structure."]
    pub struct_size_bytes: u32,
    #[doc = "Stores the major ABI version expected by `PksExtensionLibrary`."]
    pub abi_major: u16,
    #[doc = "Stores the minor ABI version expected by `PksExtensionLibrary`."]
    pub abi_minor: u16,
    #[doc = "Stores the number of registration represented by `PksExtensionLibrary`."]
    pub registration_count: u32,
    #[doc = "Reserves storage for forward-compatible evolution of `PksExtensionLibrary`."]
    pub reserved: u32,
    #[doc = "Carries the opaque library context used by `PksExtensionLibrary` callbacks."]
    pub library_context: *mut c_void,
    #[doc = "Provides the acquire registration callback used by `PksExtensionLibrary`."]
    pub acquire_registration: PksExtensionAcquireRegistrationCallback,
}

#[doc = "Names the extension library entrypoint type used by the public API."]
pub type PksExtensionLibraryEntrypoint =
    unsafe extern "C-unwind" fn(output_library: *mut PksExtensionLibrary) -> PksSessionStatus;

#[repr(C)]
#[derive(Clone, Copy)]
#[doc = "Borrows one signal payload and metadata for delivery into a native-extension callback."]
pub struct PksExtensionSignalView {
    #[doc = "Stores the byte size of the `PksExtensionSignalView` ABI structure."]
    pub struct_size_bytes: u32,
    #[doc = "Stores the major ABI version expected by `PksExtensionSignalView`."]
    pub abi_major: u16,
    #[doc = "Stores the minor ABI version expected by `PksExtensionSignalView`."]
    pub abi_minor: u16,
    #[doc = "Carries the data owned or referenced by `PksExtensionSignalView`."]
    pub data: *const u8,
    #[doc = "Stores the len size for `PksExtensionSignalView`, in bytes."]
    pub len_bytes: u32,
    #[doc = "Carries the bit flags defined by `PksExtensionSignalView`."]
    pub flags: u32,
    #[doc = "Stores the observed timestamp value for `PksExtensionSignalView`, in nanoseconds."]
    pub observed_timestamp_ns: u64,
    #[doc = "Stores the source timestamp value for `PksExtensionSignalView`, in nanoseconds."]
    pub source_timestamp_ns: u64,
    #[doc = "Stores the duration value for `PksExtensionSignalView`, in nanoseconds."]
    pub duration_ns: u64,
    #[doc = "Stores the sequence number used by `PksExtensionSignalView`."]
    pub sequence_number: u64,
}

#[repr(C)]
#[derive(Clone, Copy)]
#[doc = "Provides bounded extension-owned storage for a signal returned through the native ABI."]
pub struct PksExtensionSignalBuffer {
    #[doc = "Stores the byte size of the `PksExtensionSignalBuffer` ABI structure."]
    pub struct_size_bytes: u32,
    #[doc = "Stores the major ABI version expected by `PksExtensionSignalBuffer`."]
    pub abi_major: u16,
    #[doc = "Stores the minor ABI version expected by `PksExtensionSignalBuffer`."]
    pub abi_minor: u16,
    #[doc = "Carries the data owned or referenced by `PksExtensionSignalBuffer`."]
    pub data: *mut u8,
    #[doc = "Stores the capacity size for `PksExtensionSignalBuffer`, in bytes."]
    pub capacity_bytes: u32,
    #[doc = "Stores the len size for `PksExtensionSignalBuffer`, in bytes."]
    pub len_bytes: u32,
    #[doc = "Carries the bit flags defined by `PksExtensionSignalBuffer`."]
    pub flags: u32,
    #[doc = "Stores the observed timestamp value for `PksExtensionSignalBuffer`, in nanoseconds."]
    pub observed_timestamp_ns: u64,
    #[doc = "Stores the source timestamp value for `PksExtensionSignalBuffer`, in nanoseconds."]
    pub source_timestamp_ns: u64,
    #[doc = "Stores the duration value for `PksExtensionSignalBuffer`, in nanoseconds."]
    pub duration_ns: u64,
}

#[repr(C)]
#[derive(Clone, Copy)]
#[doc = "Describes the extension pipeline declaration contract."]
pub struct PksExtensionPipelineDeclaration {
    #[doc = "Stores the byte size of the `PksExtensionPipelineDeclaration` ABI structure."]
    pub struct_size_bytes: u32,
    #[doc = "Stores the major ABI version expected by `PksExtensionPipelineDeclaration`."]
    pub abi_major: u16,
    #[doc = "Stores the minor ABI version expected by `PksExtensionPipelineDeclaration`."]
    pub abi_minor: u16,
    #[doc = "Identifies the source identifier recorded by `PksExtensionPipelineDeclaration`."]
    pub source_id: PksSessionUtf8,
    #[doc = "Stores the source output port used by `PksExtensionPipelineDeclaration`."]
    pub source_output_port: PksSessionUtf8,
    #[doc = "Identifies the operator identifier recorded by `PksExtensionPipelineDeclaration`."]
    pub operator_id: PksSessionUtf8,
    #[doc = "Stores the operator input port used by `PksExtensionPipelineDeclaration`."]
    pub operator_input_port: PksSessionUtf8,
    #[doc = "Stores the operator output port used by `PksExtensionPipelineDeclaration`."]
    pub operator_output_port: PksSessionUtf8,
    #[doc = "Identifies the endpoint identifier recorded by `PksExtensionPipelineDeclaration`."]
    pub endpoint_id: PksSessionUtf8,
    #[doc = "Stores the endpoint input port used by `PksExtensionPipelineDeclaration`."]
    pub endpoint_input_port: PksSessionUtf8,
}

/// Aggregate bounded-runtime observations for one executable C pipeline.
///
/// ABI v1 creates exactly one source, operator, and endpoint in a Session. The
/// count fields keep that constraint explicit while the remaining fields
/// expose the native Session counter authorities rather than callback-owned
/// shadow metrics.
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PksExtensionMetricsSnapshot {
    pub struct_size_bytes: u32,
    pub abi_major: u16,
    pub abi_minor: u16,
    pub external_source_count: u32,
    pub operator_count: u32,
    pub derived_route_count: u32,
    pub reserved: u32,
    pub maximum_buffered_payload_bytes: u64,
    pub source_emitted_total: u64,
    pub source_dropped_total: u64,
    pub source_failure_total: u64,
    pub source_cancellation_total: u64,
    pub operator_input_capacity_signals: u64,
    pub operator_input_depth_signals: u64,
    pub operator_input_peak_signals: u64,
    pub operator_input_enqueued_total: u64,
    pub operator_input_dropped_total: u64,
    pub operator_processed_total: u64,
    pub operator_output_emitted_total: u64,
    pub operator_output_dropped_total: u64,
    pub operator_process_failure_total: u64,
    pub operator_timeout_total: u64,
    pub operator_cancellation_total: u64,
    pub route_capacity_signals: u64,
    pub route_depth_signals: u64,
    pub route_peak_signals: u64,
    pub route_enqueued_total: u64,
    pub route_received_total: u64,
    pub route_dropped_total: u64,
    pub endpoint_received_total: u64,
    pub endpoint_delivered_total: u64,
    pub endpoint_dropped_total: u64,
    pub endpoint_failure_total: u64,
}

#[derive(Clone)]
pub(crate) enum ExecutableExtensionRegistration {
    Source {
        id: String,
        factory: Arc<dyn SourceFactory>,
    },
    Operator {
        id: String,
        factory: Arc<dyn AsyncOperatorFactory>,
    },
    Endpoint {
        id: String,
        definition: Arc<dyn NodeDefinition>,
        factory: Arc<dyn EndpointDriverFactory>,
    },
}

impl ExecutableExtensionRegistration {
    pub(crate) fn id(&self) -> &str {
        match self {
            Self::Source { id, .. } | Self::Operator { id, .. } | Self::Endpoint { id, .. } => id,
        }
    }

    pub(crate) const fn kind(&self) -> PksExtensionKind {
        match self {
            Self::Source { .. } => PksExtensionKind::Source,
            Self::Operator { .. } => PksExtensionKind::Operator,
            Self::Endpoint { .. } => PksExtensionKind::Endpoint,
        }
    }
}

#[derive(Clone)]
pub(crate) struct ExecutableExtensionPipeline {
    pub source_id: String,
    pub source_output_port: String,
    pub operator_id: String,
    pub operator_input_port: String,
    pub operator_output_port: String,
    pub endpoint_id: String,
    pub endpoint_input_port: String,
}

#[unsafe(no_mangle)]
/// Retains one executable C extension on an engine before a Session exists.
///
/// All descriptor and port text is copied. The callback context remains owned
/// by the caller until PocketStation invokes `destroy` exactly once.
///
/// # Safety
/// The descriptor, ports, and callbacks must be readable and aligned for this
/// call. Callback function pointers and their context must remain valid until
/// the registered context is destroyed.
pub unsafe extern "C" fn pks_session_engine_register_extension(
    engine: PksSessionHandle,
    descriptor: *const PksExtensionDescriptor,
    ports: *const PksExtensionPort,
    port_count: u32,
    callbacks: *const PksExtensionCallbacks,
) -> PksSessionStatus {
    extension_call(|| {
        guard_pointer(descriptor)?;
        guard_pointer(callbacks)?;
        // SAFETY: validated pointers are readable for this call.
        let descriptor_value = unsafe { descriptor.read() };
        // SAFETY: the descriptor validator applies all descriptor/port bounds.
        let status = unsafe { pks_extension_descriptor_validate(descriptor, ports, port_count) };
        status_result(status)?;
        // SAFETY: port_count was bounded by descriptor validation.
        let ports = unsafe { slice::from_raw_parts(ports, port_count as usize) };
        // SAFETY: validated pointer is readable for this call.
        let callbacks = unsafe { callbacks.read() };
        let registration = build_registration(descriptor_value, ports, callbacks)?;
        session::register_executable_extension(engine, registration)
            .map_err(AbiExtensionError::Session)
    })
}

#[unsafe(no_mangle)]
/// Declares one typed source -> operator -> endpoint Session from registrations.
///
/// # Safety
/// `declaration` and `output_session` must be readable/writable aligned records;
/// all text views are copied before return.
pub unsafe extern "C" fn pks_session_create_extension_pipeline(
    engine: PksSessionHandle,
    declaration: *const PksExtensionPipelineDeclaration,
    output_session: *mut PksSessionHandle,
) -> PksSessionStatus {
    extension_call(|| {
        guard_pointer(declaration)?;
        guard_mut_pointer(output_session)?;
        // SAFETY: validated pointer is readable for this call.
        let declaration = unsafe { declaration.read() };
        guard_versioned(
            declaration.struct_size_bytes,
            declaration.abi_major,
            declaration.abi_minor,
            size_of::<PksExtensionPipelineDeclaration>(),
        )?;
        let pipeline = ExecutableExtensionPipeline {
            source_id: copy_text(declaration.source_id, false)?,
            source_output_port: copy_text(declaration.source_output_port, false)?,
            operator_id: copy_text(declaration.operator_id, false)?,
            operator_input_port: copy_text(declaration.operator_input_port, false)?,
            operator_output_port: copy_text(declaration.operator_output_port, false)?,
            endpoint_id: copy_text(declaration.endpoint_id, false)?,
            endpoint_input_port: copy_text(declaration.endpoint_input_port, false)?,
        };
        let handle = session::create_executable_extension_session(engine, pipeline)
            .map_err(AbiExtensionError::Session)?;
        // SAFETY: validated output pointer is writable for one record.
        unsafe { output_session.write(handle) };
        Ok(())
    })
}

#[unsafe(no_mangle)]
/// Copies native Session observations for one executable extension pipeline.
///
/// # Safety
/// `output_metrics` must address one writable, aligned current-version record.
pub unsafe extern "C" fn pks_session_extension_metrics_poll(
    engine: PksSessionHandle,
    session_handle: PksSessionHandle,
    output_metrics: *mut PksExtensionMetricsSnapshot,
) -> PksSessionStatus {
    extension_call(|| {
        guard_mut_pointer(output_metrics)?;
        let metrics = session::executable_extension_metrics(engine, session_handle)
            .map_err(AbiExtensionError::Session)?;
        let mut output = PksExtensionMetricsSnapshot {
            struct_size_bytes: size_of::<PksExtensionMetricsSnapshot>() as u32,
            abi_major: PKS_EXTENSION_ABI_MAJOR,
            abi_minor: PKS_EXTENSION_ABI_MINOR,
            external_source_count: u32::try_from(metrics.external_source_count())
                .map_err(|_| AbiExtensionError::InvalidArgument)?,
            operator_count: u32::try_from(metrics.operator_count())
                .map_err(|_| AbiExtensionError::InvalidArgument)?,
            derived_route_count: u32::try_from(metrics.derived_route_count())
                .map_err(|_| AbiExtensionError::InvalidArgument)?,
            ..PksExtensionMetricsSnapshot::default()
        };
        for index in 0..metrics.external_source_count() {
            let source = metrics
                .external_source(index)
                .expect("bounded index from external_source_count");
            output.source_emitted_total = output
                .source_emitted_total
                .saturating_add(source.runtime.emitted_total);
            output.source_dropped_total = output
                .source_dropped_total
                .saturating_add(source.runtime.dropped_total);
            output.source_failure_total = output
                .source_failure_total
                .saturating_add(source.runtime.failure_total);
            output.source_cancellation_total = output
                .source_cancellation_total
                .saturating_add(source.runtime.cancellation_total);
        }
        for index in 0..metrics.operator_count() {
            let operator = metrics
                .operator(index)
                .expect("bounded index from operator_count");
            output.operator_input_capacity_signals = output
                .operator_input_capacity_signals
                .saturating_add(operator.input_edge.queue_capacity_frames);
            output.operator_input_depth_signals = output
                .operator_input_depth_signals
                .saturating_add(operator.input_edge.queue_depth_frames);
            output.operator_input_peak_signals = output
                .operator_input_peak_signals
                .saturating_add(operator.input_edge.queue_peak_frames);
            output.operator_input_enqueued_total = output
                .operator_input_enqueued_total
                .saturating_add(operator.input_edge.frames_enqueued_total);
            output.operator_input_dropped_total = output
                .operator_input_dropped_total
                .saturating_add(operator.input_edge.frames_dropped_total);
            output.operator_processed_total = output
                .operator_processed_total
                .saturating_add(operator.worker.processed_total);
            output.operator_output_emitted_total = output
                .operator_output_emitted_total
                .saturating_add(operator.worker.output_emitted_total);
            output.operator_output_dropped_total = output
                .operator_output_dropped_total
                .saturating_add(operator.worker.output_dropped_total);
            output.operator_process_failure_total = output
                .operator_process_failure_total
                .saturating_add(operator.worker.process_failure_total);
            output.operator_timeout_total = output
                .operator_timeout_total
                .saturating_add(operator.worker.timeout_total);
            output.operator_cancellation_total = output
                .operator_cancellation_total
                .saturating_add(operator.worker.cancellation_total);
        }
        for index in 0..metrics.derived_route_count() {
            let route = metrics
                .derived_route(index)
                .expect("bounded index from derived_route_count");
            output.maximum_buffered_payload_bytes = output
                .maximum_buffered_payload_bytes
                .saturating_add(route.output.maximum_buffered_payload_bytes);
            output.route_capacity_signals = output
                .route_capacity_signals
                .saturating_add(route.output.capacity_signals);
            output.route_depth_signals = output
                .route_depth_signals
                .saturating_add(route.output.depth_signals);
            output.route_peak_signals = output
                .route_peak_signals
                .saturating_add(route.output.peak_depth_signals);
            output.route_enqueued_total = output
                .route_enqueued_total
                .saturating_add(route.output.enqueued_total);
            output.route_received_total = output
                .route_received_total
                .saturating_add(route.output.received_total);
            output.route_dropped_total = output
                .route_dropped_total
                .saturating_add(route.output.dropped_total);
            if let Some(endpoint) = route.endpoint {
                output.endpoint_received_total = output
                    .endpoint_received_total
                    .saturating_add(endpoint.frames_received_total);
                output.endpoint_delivered_total = output
                    .endpoint_delivered_total
                    .saturating_add(endpoint.frames_delivered_total);
                output.endpoint_dropped_total = output
                    .endpoint_dropped_total
                    .saturating_add(endpoint.frames_dropped_total);
                output.endpoint_failure_total = output
                    .endpoint_failure_total
                    .saturating_add(endpoint.failures_total);
            }
        }
        // SAFETY: output_metrics was validated writable and aligned above.
        unsafe { output_metrics.write(output) };
        Ok(())
    })
}

#[derive(Clone)]
struct ForeignCallbacks {
    callbacks: PksExtensionCallbacks,
    instance_context: *mut c_void,
    prepared: Arc<AtomicBool>,
    stopped: Arc<AtomicBool>,
    finished: Arc<AtomicBool>,
    instance_destroyed: Arc<AtomicBool>,
    registration_destroyed: Arc<AtomicBool>,
    call_lock: Arc<Mutex<()>>,
    _code_lease: Option<Arc<libloading::Library>>,
}

// SAFETY: callbacks are serialized by call_lock. The C registration contract
// requires its context and function pointers to remain valid until destroy.
unsafe impl Send for ForeignCallbacks {}
unsafe impl Sync for ForeignCallbacks {}

impl ForeignCallbacks {
    fn new_validated(
        callbacks: PksExtensionCallbacks,
        code_lease: Option<Arc<libloading::Library>>,
    ) -> Result<Self, AbiExtensionError> {
        let empty_configuration = PksSessionUtf8 {
            data: std::ptr::NonNull::<u8>::dangling().as_ptr().cast_const(),
            len_bytes: 0,
        };
        let validate = callbacks
            .validate_configuration
            .expect("validated configuration callback");
        // SAFETY: registration_context is retained until final destruction;
        // the empty configuration has no readable payload requirement.
        if invoke_status_callback(|| unsafe {
            validate(callbacks.registration_context, empty_configuration)
        })
        .is_err()
        {
            destroy_unretained_registration(&callbacks, std::ptr::null_mut());
            return Err(AbiExtensionError::InvalidArgument);
        }
        let create = callbacks.create.expect("validated create callback");
        let mut instance_context = std::ptr::null_mut();
        // SAFETY: output pointer is valid for this call and the registration
        // context is retained until its terminal destroy callback.
        if invoke_status_callback(|| unsafe {
            create(
                callbacks.registration_context,
                empty_configuration,
                &mut instance_context,
            )
        })
        .is_err()
            || instance_context.is_null()
        {
            destroy_unretained_registration(&callbacks, instance_context);
            return Err(AbiExtensionError::InvalidArgument);
        }
        Ok(Self {
            callbacks,
            instance_context,
            prepared: Arc::new(AtomicBool::new(false)),
            stopped: Arc::new(AtomicBool::new(false)),
            finished: Arc::new(AtomicBool::new(false)),
            instance_destroyed: Arc::new(AtomicBool::new(false)),
            registration_destroyed: Arc::new(AtomicBool::new(false)),
            call_lock: Arc::new(Mutex::new(())),
            _code_lease: code_lease,
        })
    }

    fn max_payload_bytes(&self) -> usize {
        self.callbacks.max_payload_bytes as usize
    }

    fn prepare(&self) -> Result<(), String> {
        if self.prepared.swap(true, Ordering::AcqRel) {
            return Err("foreign extension prepare invoked more than once".to_owned());
        }
        let _guard = self
            .call_lock
            .lock()
            .map_err(|_| "foreign extension callback lock poisoned".to_owned())?;
        let callback = self.callbacks.prepare.expect("validated prepare callback");
        // SAFETY: registration retains the context through destroy and calls
        // are serialized on a non-realtime worker/control thread.
        invoke_status_callback(|| unsafe { callback(self.instance_context) })
    }

    fn stop(&self) -> Result<(), String> {
        if self.stopped.swap(true, Ordering::AcqRel) {
            return Ok(());
        }
        let _guard = self
            .call_lock
            .lock()
            .map_err(|_| "foreign extension callback lock poisoned".to_owned())?;
        let callback = self
            .callbacks
            .request_stop
            .expect("validated stop callback");
        // SAFETY: same retained/serialized callback contract as prepare.
        invoke_status_callback(|| unsafe { callback(self.instance_context) })
    }

    fn finish(&self) -> Result<(), String> {
        if self.finished.swap(true, Ordering::AcqRel) {
            return Ok(());
        }
        let _guard = self
            .call_lock
            .lock()
            .map_err(|_| "foreign extension callback lock poisoned".to_owned())?;
        let callback = self.callbacks.finish.expect("validated finish callback");
        // SAFETY: finish is serialized after request_stop/worker completion.
        invoke_status_callback(|| unsafe { callback(self.instance_context) })
    }

    fn stop_and_finish(&self) -> Result<(), String> {
        let stop_result = self.stop();
        let finish_result = self.finish();
        stop_result.and(finish_result)
    }

    fn source_next(&self, cancellation_requested: bool) -> Result<Option<ForeignOutput>, String> {
        let callback = self
            .callbacks
            .source_next
            .ok_or_else(|| "source_next callback is missing".to_owned())?;
        let mut storage = vec![0_u8; self.max_payload_bytes()];
        let mut output = output_buffer(&mut storage);
        let _guard = self
            .call_lock
            .lock()
            .map_err(|_| "foreign extension callback lock poisoned".to_owned())?;
        // SAFETY: storage is writable for capacity_bytes and callback calls are serialized.
        invoke_status_callback(|| unsafe {
            callback(
                self.instance_context,
                u32::from(cancellation_requested),
                &mut output,
            )
        })?;
        parse_output(output, storage)
    }

    fn operator_process(&self, input: &SignalEnvelope) -> Result<ForeignOutput, String> {
        let callback = self
            .callbacks
            .operator_process
            .ok_or_else(|| "operator_process callback is missing".to_owned())?;
        let input_bytes = payload_bytes(input.payload())?;
        let input_view = signal_view(input, input_bytes)?;
        let mut storage = vec![0_u8; self.max_payload_bytes()];
        let mut output = output_buffer(&mut storage);
        let _guard = self
            .call_lock
            .lock()
            .map_err(|_| "foreign extension callback lock poisoned".to_owned())?;
        // SAFETY: both records and their borrowed buffers remain valid for this call.
        invoke_status_callback(|| unsafe {
            callback(self.instance_context, &input_view, &mut output)
        })?;
        parse_output(output, storage)?
            .ok_or_else(|| "operator returned end-of-stream instead of one output".to_owned())
    }

    fn endpoint_consume(&self, input: &SignalEnvelope) -> Result<(), String> {
        let callback = self
            .callbacks
            .endpoint_consume
            .ok_or_else(|| "endpoint_consume callback is missing".to_owned())?;
        let input_bytes = payload_bytes(input.payload())?;
        let input_view = signal_view(input, input_bytes)?;
        let _guard = self
            .call_lock
            .lock()
            .map_err(|_| "foreign extension callback lock poisoned".to_owned())?;
        // SAFETY: record and borrowed payload remain valid for this serialized call.
        invoke_status_callback(|| unsafe { callback(self.instance_context, &input_view) })
    }
}

impl Drop for ForeignCallbacks {
    fn drop(&mut self) {
        if Arc::strong_count(&self.instance_destroyed) != 1
            || self.instance_destroyed.swap(true, Ordering::AcqRel)
        {
            return;
        }
        let Ok(_guard) = self.call_lock.lock() else {
            return;
        };
        if let Some(destroy) = self.callbacks.destroy_instance {
            // SAFETY: this is the exactly-once terminal callback after all
            // other clones and in-flight serialized calls have ended.
            let _ = catch_unwind(AssertUnwindSafe(|| unsafe {
                destroy(self.instance_context)
            }));
        }
        if !self.registration_destroyed.swap(true, Ordering::AcqRel) {
            if let Some(destroy) = self.callbacks.destroy_registration {
                // SAFETY: the instance is destroyed and no registration user
                // remains when the final callback owner reaches this point.
                let _ = catch_unwind(AssertUnwindSafe(|| unsafe {
                    destroy(self.callbacks.registration_context)
                }));
            }
        }
    }
}

fn destroy_unretained_registration(
    callbacks: &PksExtensionCallbacks,
    instance_context: *mut c_void,
) {
    if !instance_context.is_null() {
        if let Some(destroy) = callbacks.destroy_instance {
            // SAFETY: create returned this unretained instance context and no
            // adapter object can concurrently access it on this error path.
            let _ = catch_unwind(AssertUnwindSafe(|| unsafe { destroy(instance_context) }));
        }
    }
    if !callbacks.registration_context.is_null() {
        let Some(destroy) = callbacks.destroy_registration else {
            return;
        };
        // SAFETY: registration failed before retention, so ownership returns
        // exactly once through this terminal callback.
        let _ = catch_unwind(AssertUnwindSafe(|| unsafe {
            destroy(callbacks.registration_context)
        }));
    }
}

pub(crate) fn destroy_acquired_registration(callbacks: &PksExtensionCallbacks) {
    destroy_unretained_registration(callbacks, std::ptr::null_mut());
}

struct ForeignOutput {
    bytes: Vec<u8>,
    observed_timestamp_ns: u64,
    source_timestamp_ns: Option<u64>,
    duration_ns: Option<u64>,
    terminal: bool,
}

struct ForeignSourceFactory {
    manifest: SourceManifest,
    callbacks: ForeignCallbacks,
}

impl SourceFactory for ForeignSourceFactory {
    fn manifest(&self) -> &SourceManifest {
        &self.manifest
    }

    fn validate_config(&self, _configuration: &SourceConfiguration) -> Result<(), ConfigError> {
        Ok(())
    }

    fn create(
        &self,
        _configuration: &SourceConfiguration,
    ) -> Result<Box<dyn SourceDriver>, SourceDriverError> {
        Ok(Box::new(ForeignSourceDriver {
            callbacks: self.callbacks.clone(),
            session: None,
            output_port: self.manifest.outputs()[0].name().to_owned(),
            signal: self.manifest.outputs()[0].signal().clone(),
            sequence: 0,
        }))
    }
}

struct ForeignSourceDriver {
    callbacks: ForeignCallbacks,
    session: Option<SourceSessionContext>,
    output_port: String,
    signal: SignalSpec,
    sequence: u64,
}

impl SourceDriver for ForeignSourceDriver {
    fn prepare(&mut self, context: &SourcePrepareContext) -> Result<(), SourceDriverError> {
        self.session = context.session.clone();
        self.callbacks.prepare().map_err(SourceDriverError::Failed)
    }

    fn next(
        &mut self,
        cancellation: &SourceCancellation,
    ) -> Result<Option<SourceEmission>, SourceDriverError> {
        let Some(output) = self
            .callbacks
            .source_next(cancellation.is_cancelled())
            .map_err(SourceDriverError::Failed)?
        else {
            return Ok(None);
        };
        self.sequence = self.sequence.saturating_add(1);
        let session = self.session.as_ref().ok_or_else(|| {
            SourceDriverError::Failed("foreign source missing Session prepare context".to_owned())
        })?;
        let identity = session.output(&self.output_port).ok_or_else(|| {
            SourceDriverError::Failed("foreign source output identity is missing".to_owned())
        })?;
        let observed = if output.observed_timestamp_ns == 0 {
            crate::timing::monotonic_timestamp_ns()
        } else {
            output.observed_timestamp_ns
        };
        let timing = SignalTiming::try_new(
            output.source_timestamp_ns,
            observed,
            output.source_timestamp_ns,
            output.duration_ns,
        )
        .map_err(|error| SourceDriverError::Failed(error.to_string()))?;
        let lineage = SignalLineage::try_new(
            session.session_id,
            identity.stream_id,
            session.source_id,
            ClockDomainId::new(1),
            self.sequence,
            1,
            0,
            0,
        )
        .map_err(|error| SourceDriverError::Failed(error.to_string()))?;
        let envelope = SignalEnvelope::untracked(
            SignalPayload::Bytes(output.bytes),
            self.signal.clone(),
            observed,
        )
        .with_lineage(lineage, timing);
        Ok(Some(SourceEmission {
            output_port: self.output_port.clone(),
            envelope,
            terminal: output.terminal,
        }))
    }

    fn close(&mut self) -> Result<(), SourceDriverError> {
        self.callbacks
            .stop_and_finish()
            .map_err(SourceDriverError::Failed)
    }
}

struct ForeignOperatorFactory {
    manifest: AsyncOperatorManifest,
    callbacks: ForeignCallbacks,
}

impl AsyncOperatorFactory for ForeignOperatorFactory {
    fn manifest(&self) -> &AsyncOperatorManifest {
        &self.manifest
    }

    fn validate_config(&self, _configuration: &NodeConfig) -> Result<(), ConfigError> {
        Ok(())
    }

    fn create(&self, _configuration: &NodeConfig) -> Result<Box<dyn AsyncNode>, NodeError> {
        Ok(Box::new(ForeignOperator {
            callbacks: self.callbacks.clone(),
            output_signal: self
                .manifest
                .output_ports()
                .next()
                .expect("validated output")
                .signal()
                .clone(),
            operator_id: self.manifest.operator_id().clone(),
            revision: self.manifest.revision(),
            generation: self.manifest.generation(),
        }))
    }
}

struct ForeignOperator {
    callbacks: ForeignCallbacks,
    output_signal: SignalSpec,
    operator_id: OperatorId,
    revision: u32,
    generation: u32,
}

impl AsyncNode for ForeignOperator {
    fn prepare<'a>(
        &'a mut self,
        _context: &'a AsyncOperatorPrepareContext,
    ) -> AsyncNodeFuture<'a, Result<(), NodeError>> {
        Box::pin(async move { self.callbacks.prepare().map_err(NodeError::Prepare) })
    }

    fn process<'a>(
        &'a mut self,
        input: SignalEnvelope,
    ) -> AsyncNodeFuture<'a, Result<Vec<SignalEnvelope>, NodeError>> {
        Box::pin(async move {
            let lineage = input.lineage().ok_or_else(|| {
                NodeError::Process("foreign operator input lacks lineage".to_owned())
            })?;
            let timing = input.timing();
            let output = self
                .callbacks
                .operator_process(&input)
                .map_err(NodeError::Process)?;
            let observed = if output.observed_timestamp_ns == 0 {
                crate::timing::monotonic_timestamp_ns()
            } else {
                output.observed_timestamp_ns
            };
            let output_timing = SignalTiming::try_new(
                output.source_timestamp_ns.or(timing.source_timestamp_ns()),
                observed,
                timing.session_timestamp_ns(),
                output.duration_ns.or(timing.duration_ns()),
            )
            .map_err(|error| NodeError::Process(error.to_string()))?;
            let derivation = SignalDerivation::new(
                lineage,
                timing,
                self.operator_id.clone(),
                self.revision,
                self.generation,
                None::<ConnectorId>,
            )
            .map_err(|error| NodeError::Process(error.to_string()))?;
            Ok(vec![SignalEnvelope::untracked(
                SignalPayload::Bytes(output.bytes),
                self.output_signal.clone(),
                observed,
            )
            .with_lineage(lineage, output_timing)
            .with_derivation(derivation)])
        })
    }

    fn close<'a>(&'a mut self) -> AsyncNodeFuture<'a, Result<(), NodeError>> {
        Box::pin(async move { self.callbacks.stop_and_finish().map_err(NodeError::Process) })
    }
}

struct ForeignEndpointDefinition {
    descriptor: NodeDescriptor,
}

impl NodeDefinition for ForeignEndpointDefinition {
    fn descriptor(&self) -> NodeDescriptor {
        self.descriptor.clone()
    }

    fn validate_config(&self, _configuration: &NodeConfig) -> Result<(), ConfigError> {
        Ok(())
    }
}

struct ForeignEndpointFactory {
    callbacks: ForeignCallbacks,
}

impl EndpointDriverFactory for ForeignEndpointFactory {
    fn prepare(
        &self,
        mut inputs: Vec<EndpointPortInput>,
    ) -> Result<Box<dyn PreparedEndpointDriver>, EndpointFailure> {
        if inputs.len() != 1 {
            return Err(endpoint_failure(
                EndpointFailureStage::Prepare,
                "C endpoint v1 requires exactly one typed signal input",
            ));
        }
        self.callbacks
            .prepare()
            .map_err(|message| endpoint_failure(EndpointFailureStage::Prepare, message))?;
        let input = inputs.pop().expect("one checked endpoint input");
        let (receiver, _) = input.into_parts();
        let EndpointReceiver::Signal(receiver) = receiver else {
            return Err(endpoint_failure(
                EndpointFailureStage::Prepare,
                "C endpoint v1 rejects realtime audio receivers",
            ));
        };
        Ok(Box::new(ForeignPreparedEndpoint {
            callbacks: self.callbacks.clone(),
            receiver,
        }))
    }
}

struct ForeignPreparedEndpoint {
    callbacks: ForeignCallbacks,
    receiver: crate::endpoint::EndpointSignalReceiver,
}

impl PreparedEndpointDriver for ForeignPreparedEndpoint {
    fn start(
        self: Box<Self>,
        start_gate: Arc<EndpointStartGate>,
    ) -> Result<Box<dyn RunningEndpointDriver>, EndpointFailure> {
        let stop = Arc::new(AtomicBool::new(false));
        let worker_stop = Arc::clone(&stop);
        let callbacks = self.callbacks.clone();
        let mut receiver = self.receiver;
        let observations = Arc::new(ForeignEndpointObservations::default());
        let worker_observations = Arc::clone(&observations);
        let worker = std::thread::Builder::new()
            .name("pks-c-extension-endpoint".to_owned())
            .spawn(move || {
                while !start_gate.is_open() && !worker_stop.load(Ordering::Acquire) {
                    std::thread::sleep(std::time::Duration::from_millis(1));
                }
                while !worker_stop.load(Ordering::Acquire) {
                    if let Some(envelope) = receiver.try_recv() {
                        worker_observations.received.fetch_add(1, Ordering::Relaxed);
                        if let Err(message) = callbacks.endpoint_consume(&envelope) {
                            worker_observations.failures.fetch_add(1, Ordering::Relaxed);
                            return Err(message);
                        }
                        worker_observations
                            .delivered
                            .fetch_add(1, Ordering::Relaxed);
                    } else if receiver.is_abandoned() {
                        break;
                    } else {
                        std::thread::sleep(std::time::Duration::from_millis(1));
                    }
                }
                Ok(())
            })
            .map_err(|error| endpoint_failure(EndpointFailureStage::Start, error.to_string()))?;
        Ok(Box::new(ForeignRunningEndpoint {
            callbacks: self.callbacks.clone(),
            stop,
            worker: Some(worker),
            observations,
        }))
    }

    fn cancel_preparation(self: Box<Self>) -> EndpointCancellationOutcome {
        let result = self
            .callbacks
            .stop_and_finish()
            .map_err(|message| endpoint_failure(EndpointFailureStage::CancelPreparation, message));
        EndpointCancellationOutcome {
            observations: EndpointDriverObservations::default(),
            result,
        }
    }
}

#[derive(Default)]
struct ForeignEndpointObservations {
    received: AtomicU64,
    delivered: AtomicU64,
    failures: AtomicU64,
}

struct ForeignRunningEndpoint {
    callbacks: ForeignCallbacks,
    stop: Arc<AtomicBool>,
    worker: Option<JoinHandle<Result<(), String>>>,
    observations: Arc<ForeignEndpointObservations>,
}

impl RunningEndpointDriver for ForeignRunningEndpoint {
    fn observations(&self) -> EndpointDriverObservations {
        EndpointDriverObservations {
            frames_received_total: self.observations.received.load(Ordering::Relaxed),
            frames_delivered_total: self.observations.delivered.load(Ordering::Relaxed),
            failures_total: self.observations.failures.load(Ordering::Relaxed),
            ..EndpointDriverObservations::default()
        }
    }

    fn request_stop(&mut self) -> Result<(), EndpointFailure> {
        self.stop.store(true, Ordering::Release);
        self.callbacks
            .stop()
            .map_err(|message| endpoint_failure(EndpointFailureStage::RequestStop, message))
    }

    fn join_and_finalize(mut self: Box<Self>) -> EndpointDriverFinalization {
        self.stop.store(true, Ordering::Release);
        let result = self
            .worker
            .take()
            .expect("foreign endpoint worker exists")
            .join()
            .map_err(|_| {
                endpoint_failure(
                    EndpointFailureStage::JoinFinalize,
                    "C endpoint worker panicked",
                )
            })
            .and_then(|result| {
                result.map_err(|message| {
                    endpoint_failure(EndpointFailureStage::JoinFinalize, message)
                })
            })
            .and_then(|()| {
                self.callbacks.finish().map_err(|message| {
                    endpoint_failure(EndpointFailureStage::JoinFinalize, message)
                })
            });
        EndpointDriverFinalization {
            observations: self.observations(),
            result,
        }
    }
}

fn build_registration(
    descriptor: PksExtensionDescriptor,
    ports: &[PksExtensionPort],
    callbacks: PksExtensionCallbacks,
) -> Result<ExecutableExtensionRegistration, AbiExtensionError> {
    build_registration_with_library(descriptor, ports, callbacks, None, false)
}

pub(crate) fn build_registration_with_library(
    descriptor: PksExtensionDescriptor,
    ports: &[PksExtensionPort],
    callbacks: PksExtensionCallbacks,
    code_lease: Option<Arc<libloading::Library>>,
    destroy_registration_on_preparation_error: bool,
) -> Result<ExecutableExtensionRegistration, AbiExtensionError> {
    let port_count = u32::try_from(ports.len()).map_err(|_| AbiExtensionError::InvalidArgument)?;
    // SAFETY: descriptor is one readable local record and ports is a readable,
    // bounded slice for this synchronous validation call.
    let descriptor_status =
        unsafe { pks_extension_descriptor_validate(&descriptor, ports.as_ptr(), port_count) };
    if let Err(error) = status_result(descriptor_status) {
        if destroy_registration_on_preparation_error {
            destroy_unretained_registration(&callbacks, std::ptr::null_mut());
        }
        return Err(error);
    }
    if let Err(error) = validate_callback_shape(&callbacks) {
        if destroy_registration_on_preparation_error {
            destroy_unretained_registration(&callbacks, std::ptr::null_mut());
        }
        return Err(error);
    }

    let prepared = match prepare_registration(descriptor, ports, &callbacks) {
        Ok(prepared) => prepared,
        Err(error) => {
            if destroy_registration_on_preparation_error {
                destroy_unretained_registration(&callbacks, std::ptr::null_mut());
            }
            return Err(error);
        }
    };
    let callbacks = ForeignCallbacks::new_validated(callbacks, code_lease)?;
    Ok(prepared.with_callbacks(callbacks))
}

fn validate_callback_shape(callbacks: &PksExtensionCallbacks) -> Result<(), AbiExtensionError> {
    guard_versioned(
        callbacks.struct_size_bytes,
        callbacks.abi_major,
        callbacks.abi_minor,
        size_of::<PksExtensionCallbacks>(),
    )?;
    if callbacks.registration_context.is_null()
        || callbacks.max_payload_bytes == 0
        || callbacks.max_payload_bytes > MAX_CALLBACK_PAYLOAD_BYTES
        || callbacks.validate_configuration.is_none()
        || callbacks.create.is_none()
        || callbacks.prepare.is_none()
        || callbacks.request_stop.is_none()
        || callbacks.finish.is_none()
        || callbacks.destroy_instance.is_none()
        || callbacks.destroy_registration.is_none()
    {
        Err(AbiExtensionError::InvalidArgument)
    } else {
        Ok(())
    }
}

enum PreparedExtensionRegistration {
    Source {
        id: String,
        manifest: SourceManifest,
    },
    Operator {
        id: String,
        manifest: Box<AsyncOperatorManifest>,
    },
    Endpoint {
        id: String,
        descriptor: NodeDescriptor,
    },
}

impl PreparedExtensionRegistration {
    fn with_callbacks(self, callbacks: ForeignCallbacks) -> ExecutableExtensionRegistration {
        match self {
            Self::Source { id, manifest } => ExecutableExtensionRegistration::Source {
                id,
                factory: Arc::new(ForeignSourceFactory {
                    manifest,
                    callbacks,
                }),
            },
            Self::Operator { id, manifest } => ExecutableExtensionRegistration::Operator {
                id,
                factory: Arc::new(ForeignOperatorFactory {
                    manifest: *manifest,
                    callbacks,
                }),
            },
            Self::Endpoint { id, descriptor } => ExecutableExtensionRegistration::Endpoint {
                id,
                definition: Arc::new(ForeignEndpointDefinition { descriptor }),
                factory: Arc::new(ForeignEndpointFactory { callbacks }),
            },
        }
    }
}

fn prepare_registration(
    descriptor: PksExtensionDescriptor,
    ports: &[PksExtensionPort],
    callbacks: &PksExtensionCallbacks,
) -> Result<PreparedExtensionRegistration, AbiExtensionError> {
    let id = copy_text(descriptor.extension_id, false)?;
    let owned_ports = ports
        .iter()
        .map(owned_port)
        .collect::<Result<Vec<_>, _>>()?;
    match descriptor.kind {
        value if value == PksExtensionKind::Source as u32 => {
            if callbacks.source_next.is_none()
                || callbacks.operator_process.is_some()
                || callbacks.endpoint_consume.is_some()
                || owned_ports.len() != 1
                || owned_ports[0].direction() != PortDirection::Output
            {
                return Err(AbiExtensionError::InvalidArgument);
            }
            let manifest = SourceManifest::new(
                SourceTypeId::new(id.clone()).map_err(|_| AbiExtensionError::InvalidArgument)?,
                descriptor.revision,
                descriptor.generation,
                owned_ports,
                ExecutionPartition::BlockingWorker,
                SafetyContract::BlockingAllowed,
            )
            .map_err(|_| AbiExtensionError::InvalidArgument)?;
            Ok(PreparedExtensionRegistration::Source { id, manifest })
        }
        value if value == PksExtensionKind::Operator as u32 => {
            if callbacks.operator_process.is_none()
                || callbacks.source_next.is_some()
                || callbacks.endpoint_consume.is_some()
                || owned_ports.len() != 2
            {
                return Err(AbiExtensionError::InvalidArgument);
            }
            let inputs = owned_ports
                .iter()
                .filter(|port| port.direction() == PortDirection::Input)
                .cloned()
                .collect::<Vec<_>>();
            let outputs = owned_ports
                .iter()
                .filter(|port| port.direction() == PortDirection::Output)
                .cloned()
                .collect::<Vec<_>>();
            if inputs.len() != 1 || outputs.len() != 1 {
                return Err(AbiExtensionError::InvalidArgument);
            }
            let media = inputs[0].media();
            if !media.is_compatible_with(&outputs[0].media()) {
                return Err(AbiExtensionError::InvalidArgument);
            }
            let node = NodeDescriptor::new(
                NodeTypeId::from(id.as_str()),
                "C extension operator",
                inputs,
                outputs,
                ExecutionPartition::AsyncWorker,
                SafetyContract::AllocationAllowed,
                true,
            )
            .map_err(|_| AbiExtensionError::InvalidArgument)?;
            let input_edge = EdgeContract::bounded_async()
                .with_media(media)
                .with_backpressure(BackpressurePolicy::DropNewest)
                .with_copy_policy(CopyPolicy::CopyToBranchPool)
                .with_max_payload_bytes(callbacks.max_payload_bytes as usize);
            let output_edge = EdgeContract::bounded_async()
                .with_media(media)
                .with_max_payload_bytes(callbacks.max_payload_bytes as usize);
            let manifest = AsyncOperatorManifest::new(
                OperatorId::new(id.clone()),
                descriptor.revision,
                descriptor.generation,
                node,
                input_edge,
                output_edge,
                8,
                OperatorPermissionPolicy {
                    network_allowed: false,
                    filesystem_allowed: false,
                },
                OperatorDeadlinePolicy {
                    process_timeout_ms: 1_000,
                },
                OperatorCancellationPolicy::DiscardQueued,
                OperatorFailurePolicy::StopWorker,
                OperatorOutputRolePolicy::default(),
            )
            .map_err(|_| AbiExtensionError::InvalidArgument)?;
            Ok(PreparedExtensionRegistration::Operator {
                id,
                manifest: Box::new(manifest),
            })
        }
        value if value == PksExtensionKind::Endpoint as u32 => {
            if callbacks.endpoint_consume.is_none()
                || callbacks.source_next.is_some()
                || callbacks.operator_process.is_some()
                || owned_ports.len() != 1
                || owned_ports[0].direction() != PortDirection::Input
            {
                return Err(AbiExtensionError::InvalidArgument);
            }
            let descriptor = NodeDescriptor::new(
                NodeTypeId::from(id.as_str()),
                "C extension endpoint",
                owned_ports,
                Vec::new(),
                ExecutionPartition::External,
                SafetyContract::ExternalService,
                true,
            )
            .map_err(|_| AbiExtensionError::InvalidArgument)?;
            Ok(PreparedExtensionRegistration::Endpoint { id, descriptor })
        }
        _ => Err(AbiExtensionError::InvalidArgument),
    }
}

fn owned_port(port: &PksExtensionPort) -> Result<PortSpec, AbiExtensionError> {
    let direction = match port.direction {
        value if value == PksExtensionPortDirection::Input as u32 => PortDirection::Input,
        value if value == PksExtensionPortDirection::Output as u32 => PortDirection::Output,
        _ => return Err(AbiExtensionError::InvalidArgument),
    };
    let signal_id = copy_text(port.signal_id, false)?;
    if signal_id.starts_with("pks.signal.") {
        return Err(AbiExtensionError::InvalidArgument);
    }
    let schema = copy_text(port.schema, false)?;
    let role = copy_text(port.semantic_role, true)?;
    let mut signal = SignalSpec::custom(signal_id).with_schema(schema);
    if !role.is_empty() {
        signal = signal.with_role(role);
    }
    PortSpec::new(
        copy_text(port.name, false)?,
        direction,
        signal,
        MediaCaps::Binary(BinaryFormat::Raw),
        Multiplicity::Many,
        port.required != 0,
    )
    .map_err(|_| AbiExtensionError::InvalidArgument)
}

fn output_buffer(storage: &mut [u8]) -> PksExtensionSignalBuffer {
    PksExtensionSignalBuffer {
        struct_size_bytes: size_of::<PksExtensionSignalBuffer>() as u32,
        abi_major: PKS_EXTENSION_ABI_MAJOR,
        abi_minor: PKS_EXTENSION_ABI_MINOR,
        data: storage.as_mut_ptr(),
        capacity_bytes: storage.len() as u32,
        len_bytes: 0,
        flags: 0,
        observed_timestamp_ns: 0,
        source_timestamp_ns: 0,
        duration_ns: 0,
    }
}

fn parse_output(
    output: PksExtensionSignalBuffer,
    mut storage: Vec<u8>,
) -> Result<Option<ForeignOutput>, String> {
    if output.struct_size_bytes < size_of::<PksExtensionSignalBuffer>() as u32
        || output.abi_major != PKS_EXTENSION_ABI_MAJOR
        || output.abi_minor > PKS_EXTENSION_ABI_MINOR
        || output.capacity_bytes != storage.len() as u32
        || output.data != storage.as_mut_ptr()
        || output.len_bytes > output.capacity_bytes
        || output.flags & !(SIGNAL_FLAG_END_OF_STREAM | SIGNAL_FLAG_TERMINAL) != 0
    {
        return Err("foreign callback returned an invalid bounded output record".to_owned());
    }
    if output.flags & SIGNAL_FLAG_END_OF_STREAM != 0 {
        if output.len_bytes != 0 {
            return Err("end-of-stream output must have zero bytes".to_owned());
        }
        return Ok(None);
    }
    storage.truncate(output.len_bytes as usize);
    Ok(Some(ForeignOutput {
        bytes: storage,
        observed_timestamp_ns: output.observed_timestamp_ns,
        source_timestamp_ns: (output.source_timestamp_ns != 0)
            .then_some(output.source_timestamp_ns),
        duration_ns: (output.duration_ns != 0).then_some(output.duration_ns),
        terminal: output.flags & SIGNAL_FLAG_TERMINAL != 0,
    }))
}

fn payload_bytes(payload: &SignalPayload) -> Result<&[u8], String> {
    match payload {
        SignalPayload::Bytes(bytes) => Ok(bytes),
        SignalPayload::Text(text) => Ok(text.as_bytes()),
        SignalPayload::Audio(_) => Err("C extension v1 rejects realtime audio payloads".to_owned()),
    }
}

fn signal_view(envelope: &SignalEnvelope, bytes: &[u8]) -> Result<PksExtensionSignalView, String> {
    let len_bytes = u32::try_from(bytes.len())
        .map_err(|_| "foreign input payload exceeds ABI length".to_owned())?;
    let timing = envelope.timing();
    Ok(PksExtensionSignalView {
        struct_size_bytes: size_of::<PksExtensionSignalView>() as u32,
        abi_major: PKS_EXTENSION_ABI_MAJOR,
        abi_minor: PKS_EXTENSION_ABI_MINOR,
        data: bytes.as_ptr(),
        len_bytes,
        flags: 0,
        observed_timestamp_ns: timing.observed_timestamp_ns(),
        source_timestamp_ns: timing.source_timestamp_ns().unwrap_or(0),
        duration_ns: timing.duration_ns().unwrap_or(0),
        sequence_number: envelope.sequence_number().unwrap_or(0),
    })
}

fn callback_result(status: PksSessionStatus) -> Result<(), String> {
    if status.code == PksSessionStatusCode::Ok as u32 {
        Ok(())
    } else {
        Err(format!(
            "foreign callback failed with status {} detail {}",
            status.code, status.detail
        ))
    }
}

fn invoke_status_callback(callback: impl FnOnce() -> PksSessionStatus) -> Result<(), String> {
    match catch_unwind(AssertUnwindSafe(callback)) {
        Ok(status) => callback_result(status),
        Err(_) => Err("foreign extension callback unwound".to_owned()),
    }
}

fn status_result(status: PksSessionStatus) -> Result<(), AbiExtensionError> {
    if status.code == PksSessionStatusCode::Ok as u32 {
        Ok(())
    } else {
        Err(AbiExtensionError::Status(status))
    }
}

fn endpoint_failure(stage: EndpointFailureStage, message: impl Into<String>) -> EndpointFailure {
    EndpointFailure::new(stage, message)
}

fn guard_versioned(
    struct_size_bytes: u32,
    abi_major: u16,
    abi_minor: u16,
    required_size: usize,
) -> Result<(), AbiExtensionError> {
    if abi_major != PKS_EXTENSION_ABI_MAJOR {
        Err(AbiExtensionError::UnsupportedMajor)
    } else if abi_minor > PKS_EXTENSION_ABI_MINOR {
        Err(AbiExtensionError::UnsupportedMinor)
    } else if struct_size_bytes < required_size as u32 {
        Err(AbiExtensionError::InvalidSize)
    } else {
        Ok(())
    }
}

fn guard_pointer<T>(pointer: *const T) -> Result<(), AbiExtensionError> {
    if pointer.is_null() {
        Err(AbiExtensionError::Null)
    } else if !(pointer as usize).is_multiple_of(align_of::<T>()) {
        Err(AbiExtensionError::Misaligned)
    } else {
        Ok(())
    }
}

fn guard_mut_pointer<T>(pointer: *mut T) -> Result<(), AbiExtensionError> {
    guard_pointer(pointer.cast_const())
}

fn copy_text(view: PksSessionUtf8, empty_allowed: bool) -> Result<String, AbiExtensionError> {
    if view.len_bytes == 0 {
        return if empty_allowed {
            Ok(String::new())
        } else {
            Err(AbiExtensionError::InvalidArgument)
        };
    }
    if view.len_bytes > 1_024 {
        return Err(AbiExtensionError::InvalidArgument);
    }
    guard_pointer(view.data)?;
    // SAFETY: caller guarantees readable bytes for this call; data is copied.
    let bytes = unsafe { slice::from_raw_parts(view.data, view.len_bytes as usize) };
    let text = std::str::from_utf8(bytes).map_err(|_| AbiExtensionError::InvalidArgument)?;
    if !empty_allowed && text.trim().is_empty() {
        return Err(AbiExtensionError::InvalidArgument);
    }
    Ok(text.to_owned())
}

fn extension_call(operation: impl FnOnce() -> Result<(), AbiExtensionError>) -> PksSessionStatus {
    match catch_unwind(AssertUnwindSafe(operation)) {
        Ok(Ok(())) => PksSessionStatus::ok(),
        Ok(Err(error)) => error.status(),
        Err(_) => PksSessionStatus::new(PksSessionStatusCode::InternalPanic, 0),
    }
}

#[derive(Debug)]
pub(crate) enum AbiExtensionError {
    Null,
    Misaligned,
    UnsupportedMajor,
    UnsupportedMinor,
    InvalidSize,
    InvalidArgument,
    Status(PksSessionStatus),
    Session(PksSessionStatus),
}

impl AbiExtensionError {
    pub(crate) const fn code(&self) -> &'static str {
        match self {
            Self::Null => "NULL_POINTER",
            Self::Misaligned => "MISALIGNED_POINTER",
            Self::UnsupportedMajor => "UNSUPPORTED_ABI_MAJOR",
            Self::UnsupportedMinor => "UNSUPPORTED_ABI_MINOR",
            Self::InvalidSize => "INVALID_STRUCT_SIZE",
            Self::InvalidArgument => "INVALID_ARGUMENT",
            Self::Status(_) => "CALLBACK_STATUS",
            Self::Session(_) => "SESSION_STATUS",
        }
    }

    fn status(self) -> PksSessionStatus {
        match self {
            Self::Null => PksSessionStatus::new(PksSessionStatusCode::NullArgument, 0),
            Self::Misaligned => PksSessionStatus::new(PksSessionStatusCode::MisalignedPointer, 0),
            Self::UnsupportedMajor => {
                PksSessionStatus::new(PksSessionStatusCode::UnsupportedAbiMajor, 0)
            }
            Self::UnsupportedMinor => {
                PksSessionStatus::new(PksSessionStatusCode::UnsupportedAbiMinor, 0)
            }
            Self::InvalidSize => PksSessionStatus::new(PksSessionStatusCode::InvalidStructSize, 0),
            Self::InvalidArgument => {
                PksSessionStatus::new(PksSessionStatusCode::InvalidArgument, 0)
            }
            Self::Status(status) | Self::Session(status) => status,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn utf8(bytes: &'static [u8]) -> PksSessionUtf8 {
        PksSessionUtf8 {
            data: bytes.as_ptr(),
            len_bytes: bytes.len() as u32,
        }
    }

    #[test]
    fn given_unwinding_foreign_callback_when_invoked_then_unwind_is_contained() {
        let result = invoke_status_callback(|| panic!("foreign panic"));

        assert_eq!(
            result.expect_err("unwind must be contained"),
            "foreign extension callback unwound"
        );
    }

    #[test]
    fn given_oversized_callback_output_when_parsed_then_record_is_rejected() {
        let mut storage = vec![0_u8; 4];
        let mut output = output_buffer(&mut storage);
        output.len_bytes = 5;

        let result = parse_output(output, storage);

        assert!(matches!(
            result,
            Err(message) if message == "foreign callback returned an invalid bounded output record"
        ));
    }

    #[test]
    fn given_reserved_core_signal_when_c_port_owned_then_realtime_masquerade_is_rejected() {
        let port = PksExtensionPort {
            struct_size_bytes: size_of::<PksExtensionPort>() as u32,
            abi_major: PKS_EXTENSION_ABI_MAJOR,
            abi_minor: PKS_EXTENSION_ABI_MINOR,
            direction: PksExtensionPortDirection::Output as u32,
            required: 1,
            name: utf8(b"out"),
            signal_id: utf8(b"pks.signal.pcm-audio.v1"),
            semantic_role: utf8(b""),
            schema: utf8(b"urn:pocketstation:test:v1"),
        };

        assert!(matches!(
            owned_port(&port),
            Err(AbiExtensionError::InvalidArgument)
        ));
    }
}
