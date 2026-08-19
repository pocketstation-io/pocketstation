#![allow(dead_code)]

use std::ffi::c_void;
use std::fs::OpenOptions;
use std::io::Write;
use std::mem::size_of;

const ABI_MAJOR: u16 = 1;
const ABI_MINOR: u16 = 2;
const STATUS_OK: u32 = 0;
const STATUS_INVALID_ARGUMENT: u32 = 10;
const KIND_SOURCE: u32 = 1;
const KIND_OPERATOR: u32 = 2;
const KIND_ENDPOINT: u32 = 3;
const PORT_INPUT: u32 = 1;
const PORT_OUTPUT: u32 = 2;
const END_OF_STREAM: u32 = 1;

#[repr(C)]
#[derive(Clone, Copy)]
struct Status {
    code: u32,
    detail: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct Utf8 {
    data: *const u8,
    len_bytes: u32,
}

unsafe impl Sync for Utf8 {}

#[repr(C)]
#[derive(Clone, Copy)]
struct Descriptor {
    struct_size_bytes: u32,
    abi_major: u16,
    abi_minor: u16,
    kind: u32,
    revision: u32,
    generation: u32,
    port_count: u32,
    extension_id: Utf8,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct Port {
    struct_size_bytes: u32,
    abi_major: u16,
    abi_minor: u16,
    direction: u32,
    required: u32,
    name: Utf8,
    signal_id: Utf8,
    semantic_role: Utf8,
    schema: Utf8,
}

unsafe impl Sync for Port {}

#[repr(C)]
struct SignalView {
    struct_size_bytes: u32,
    abi_major: u16,
    abi_minor: u16,
    data: *const u8,
    len_bytes: u32,
    flags: u32,
    observed_timestamp_ns: u64,
    source_timestamp_ns: u64,
    duration_ns: u64,
    sequence_number: u64,
}

#[repr(C)]
struct SignalBuffer {
    struct_size_bytes: u32,
    abi_major: u16,
    abi_minor: u16,
    data: *mut u8,
    capacity_bytes: u32,
    len_bytes: u32,
    flags: u32,
    observed_timestamp_ns: u64,
    source_timestamp_ns: u64,
    duration_ns: u64,
}

type Validate = Option<unsafe extern "C-unwind" fn(*mut c_void, Utf8) -> Status>;
type Create =
    Option<unsafe extern "C-unwind" fn(*mut c_void, Utf8, *mut *mut c_void) -> Status>;
type Prepare = Option<unsafe extern "C-unwind" fn(*mut c_void) -> Status>;
type SourceNext = Option<
    unsafe extern "C-unwind" fn(*mut c_void, u32, *mut SignalBuffer) -> Status,
>;
type OperatorProcess = Option<
    unsafe extern "C-unwind" fn(*mut c_void, *const SignalView, *mut SignalBuffer) -> Status,
>;
type EndpointConsume =
    Option<unsafe extern "C-unwind" fn(*mut c_void, *const SignalView) -> Status>;
type Lifecycle = Option<unsafe extern "C-unwind" fn(*mut c_void) -> Status>;
type Destroy = Option<unsafe extern "C-unwind" fn(*mut c_void)>;

#[repr(C)]
#[derive(Clone, Copy)]
struct Callbacks {
    struct_size_bytes: u32,
    abi_major: u16,
    abi_minor: u16,
    registration_context: *mut c_void,
    max_payload_bytes: u32,
    reserved: u32,
    validate_configuration: Validate,
    create: Create,
    prepare: Prepare,
    source_next: SourceNext,
    operator_process: OperatorProcess,
    endpoint_consume: EndpointConsume,
    request_stop: Lifecycle,
    finish: Lifecycle,
    destroy_instance: Destroy,
    destroy_registration: Destroy,
}

type Acquire = Option<
    unsafe extern "C-unwind" fn(
        *mut c_void,
        u32,
        *mut Descriptor,
        *mut *const Port,
        *mut u32,
        *mut Callbacks,
    ) -> Status,
>;

#[repr(C)]
struct ExtensionLibrary {
    struct_size_bytes: u32,
    abi_major: u16,
    abi_minor: u16,
    registration_count: u32,
    reserved: u32,
    library_context: *mut c_void,
    acquire_registration: Acquire,
}

struct RegistrationContext {
    kind: u32,
}

struct InstanceContext {
    kind: u32,
    emitted: bool,
}

const fn utf8(bytes: &'static [u8]) -> Utf8 {
    Utf8 {
        data: bytes.as_ptr(),
        len_bytes: bytes.len() as u32,
    }
}

const SOURCE_ID: &[u8] = b"dev.pocketstation.source.fixture.v1";
const OPERATOR_ID: &[u8] = b"dev.pocketstation.fixture.operator.v1";
const ENDPOINT_ID: &[u8] = b"dev.pocketstation.fixture.endpoint.v1";
const SIGNAL_ID: &[u8] = b"dev.pocketstation.fixture.signal.v1";
const SCHEMA: &[u8] = b"urn:pocketstation:fixture:native-extension:v1";
const EMPTY: &[u8] = b"";

static SOURCE_PORTS: [Port; 1] = [Port {
    struct_size_bytes: size_of::<Port>() as u32,
    abi_major: ABI_MAJOR,
    abi_minor: ABI_MINOR,
    direction: PORT_OUTPUT,
    required: 1,
    name: utf8(b"out"),
    signal_id: utf8(SIGNAL_ID),
    semantic_role: utf8(EMPTY),
    schema: utf8(SCHEMA),
}];

static OPERATOR_PORTS: [Port; 2] = [
    Port {
        struct_size_bytes: size_of::<Port>() as u32,
        abi_major: ABI_MAJOR,
        abi_minor: ABI_MINOR,
        direction: PORT_INPUT,
        required: 1,
        name: utf8(b"in"),
        signal_id: utf8(SIGNAL_ID),
        semantic_role: utf8(EMPTY),
        schema: utf8(SCHEMA),
    },
    Port {
        struct_size_bytes: size_of::<Port>() as u32,
        abi_major: ABI_MAJOR,
        abi_minor: ABI_MINOR,
        direction: PORT_OUTPUT,
        required: 1,
        name: utf8(b"out"),
        signal_id: utf8(SIGNAL_ID),
        semantic_role: utf8(EMPTY),
        schema: utf8(SCHEMA),
    },
];

static ENDPOINT_PORTS: [Port; 1] = [Port {
    struct_size_bytes: size_of::<Port>() as u32,
    abi_major: ABI_MAJOR,
    abi_minor: ABI_MINOR,
    direction: PORT_INPUT,
    required: 1,
    name: utf8(b"in"),
    signal_id: utf8(SIGNAL_ID),
    semantic_role: utf8(EMPTY),
    schema: utf8(SCHEMA),
}];

fn status(code: u32) -> Status {
    Status { code, detail: 0 }
}

fn marker(line: &str) {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(env!("PKS_FIXTURE_MARKER"))
        .expect("open fixture marker");
    writeln!(file, "{line}").expect("write fixture marker");
}

unsafe extern "C-unwind" fn validate(_context: *mut c_void, _configuration: Utf8) -> Status {
    status(STATUS_OK)
}

unsafe extern "C-unwind" fn create(
    registration_context: *mut c_void,
    _configuration: Utf8,
    output_instance: *mut *mut c_void,
) -> Status {
    if registration_context.is_null() || output_instance.is_null() {
        return status(STATUS_INVALID_ARGUMENT);
    }
    // SAFETY: the host retains the registration context until destruction.
    let registration = unsafe { &*(registration_context as *const RegistrationContext) };
    let instance = Box::new(InstanceContext {
        kind: registration.kind,
        emitted: false,
    });
    // SAFETY: validated writable output; ownership transfers to the host.
    unsafe { output_instance.write(Box::into_raw(instance).cast()) };
    status(STATUS_OK)
}

unsafe extern "C-unwind" fn prepare(_context: *mut c_void) -> Status {
    status(STATUS_OK)
}

unsafe extern "C-unwind" fn source_next(
    context: *mut c_void,
    _cancelled: u32,
    output: *mut SignalBuffer,
) -> Status {
    if context.is_null() || output.is_null() {
        return status(STATUS_INVALID_ARGUMENT);
    }
    // SAFETY: host supplies the retained instance and writable output record.
    let instance = unsafe { &mut *(context as *mut InstanceContext) };
    let output = unsafe { &mut *output };
    if instance.emitted {
        output.flags = END_OF_STREAM;
        output.len_bytes = 0;
        return status(STATUS_OK);
    }
    let payload = b"hello";
    if output.capacity_bytes < payload.len() as u32 || output.data.is_null() {
        return status(STATUS_INVALID_ARGUMENT);
    }
    // SAFETY: the host declared at least payload.len() writable bytes.
    unsafe { std::ptr::copy_nonoverlapping(payload.as_ptr(), output.data, payload.len()) };
    output.len_bytes = payload.len() as u32;
    instance.emitted = true;
    status(STATUS_OK)
}

unsafe extern "C-unwind" fn operator_process(
    _context: *mut c_void,
    input: *const SignalView,
    output: *mut SignalBuffer,
) -> Status {
    if input.is_null() || output.is_null() {
        return status(STATUS_INVALID_ARGUMENT);
    }
    // SAFETY: both views are valid for the callback duration.
    let input = unsafe { &*input };
    let output = unsafe { &mut *output };
    if input.len_bytes > output.capacity_bytes || (input.len_bytes != 0 && input.data.is_null()) {
        return status(STATUS_INVALID_ARGUMENT);
    }
    // SAFETY: validated input and output lengths above.
    unsafe {
        std::ptr::copy_nonoverlapping(input.data, output.data, input.len_bytes as usize);
    }
    output.len_bytes = input.len_bytes;
    status(STATUS_OK)
}

unsafe extern "C-unwind" fn endpoint_consume(
    _context: *mut c_void,
    input: *const SignalView,
) -> Status {
    if input.is_null() {
        return status(STATUS_INVALID_ARGUMENT);
    }
    // SAFETY: input view and bytes remain readable for this call.
    let input = unsafe { &*input };
    let bytes = unsafe { std::slice::from_raw_parts(input.data, input.len_bytes as usize) };
    marker(&format!("consume:{}", String::from_utf8_lossy(bytes)));
    status(STATUS_OK)
}

unsafe extern "C-unwind" fn lifecycle(_context: *mut c_void) -> Status {
    status(STATUS_OK)
}

unsafe extern "C-unwind" fn destroy_instance(context: *mut c_void) {
    if !context.is_null() {
        // SAFETY: final exactly-once callback returns Box ownership.
        let instance = unsafe { Box::from_raw(context as *mut InstanceContext) };
        marker(&format!("destroy_instance:{}", instance.kind));
    }
}

unsafe extern "C-unwind" fn destroy_registration(context: *mut c_void) {
    if !context.is_null() {
        // SAFETY: final exactly-once callback returns Box ownership.
        let registration = unsafe { Box::from_raw(context as *mut RegistrationContext) };
        marker(&format!("destroy_registration:{}", registration.kind));
    }
}

fn callbacks(kind: u32) -> Callbacks {
    Callbacks {
        struct_size_bytes: size_of::<Callbacks>() as u32,
        abi_major: ABI_MAJOR,
        abi_minor: ABI_MINOR,
        registration_context: Box::into_raw(Box::new(RegistrationContext { kind })).cast(),
        max_payload_bytes: 1_024,
        reserved: 0,
        validate_configuration: Some(validate),
        create: Some(create),
        prepare: Some(prepare),
        source_next: (kind == KIND_SOURCE).then_some(source_next),
        operator_process: (kind == KIND_OPERATOR).then_some(operator_process),
        endpoint_consume: (kind == KIND_ENDPOINT).then_some(endpoint_consume),
        request_stop: Some(lifecycle),
        finish: Some(lifecycle),
        destroy_instance: Some(destroy_instance),
        destroy_registration: Some(destroy_registration),
    }
}

unsafe extern "C-unwind" fn acquire(
    _library_context: *mut c_void,
    index: u32,
    output_descriptor: *mut Descriptor,
    output_ports: *mut *const Port,
    output_port_count: *mut u32,
    output_callbacks: *mut Callbacks,
) -> Status {
    if output_descriptor.is_null()
        || output_ports.is_null()
        || output_port_count.is_null()
        || output_callbacks.is_null()
    {
        return status(STATUS_INVALID_ARGUMENT);
    }
    let (kind, id, ports): (u32, &[u8], &[Port]) = match index {
        0 => (KIND_SOURCE, SOURCE_ID, &SOURCE_PORTS),
        1 => (KIND_OPERATOR, OPERATOR_ID, &OPERATOR_PORTS),
        2 => (KIND_ENDPOINT, ENDPOINT_ID, &ENDPOINT_PORTS),
        _ => return status(STATUS_INVALID_ARGUMENT),
    };
    let descriptor = Descriptor {
        struct_size_bytes: size_of::<Descriptor>() as u32,
        abi_major: ABI_MAJOR,
        abi_minor: ABI_MINOR,
        kind,
        revision: if cfg!(invalid_registration) { 0 } else { 1 },
        generation: 1,
        port_count: ports.len() as u32,
        extension_id: utf8(id),
    };
    // SAFETY: host supplied writable output records for this acquisition.
    unsafe {
        output_descriptor.write(descriptor);
        output_ports.write(ports.as_ptr());
        output_port_count.write(ports.len() as u32);
        output_callbacks.write(callbacks(kind));
    }
    status(STATUS_OK)
}

#[cfg(not(no_entrypoint))]
#[no_mangle]
unsafe extern "C-unwind" fn pks_extension_library_v1(output: *mut ExtensionLibrary) -> Status {
    if output.is_null() {
        return status(STATUS_INVALID_ARGUMENT);
    }
    // SAFETY: host supplies one writable current-version descriptor.
    unsafe {
        output.write(ExtensionLibrary {
            struct_size_bytes: size_of::<ExtensionLibrary>() as u32,
            abi_major: if cfg!(unsupported_abi) { 99 } else { ABI_MAJOR },
            abi_minor: ABI_MINOR,
            registration_count: if cfg!(invalid_registration) { 1 } else { 3 },
            reserved: 0,
            library_context: std::ptr::null_mut(),
            acquire_registration: Some(acquire),
        })
    };
    status(STATUS_OK)
}
