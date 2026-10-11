//! CoreAudio process tap backend — macOS 14.4+ (public support claim).
//!
//! Uses `AudioHardwareCreateProcessTap` + `CATapDescription` to capture audio
//! from specific processes or the global system output mix without routing
//! changes, HAL plugin installation, or Screen Recording permission.

use std::ptr::NonNull;
use std::sync::atomic::{AtomicBool, AtomicI32, AtomicU32, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

use crate::capture::frame_normalizer::CaptureFrameNormalizer;
use crate::frame::{AudioBufferPool, AudioFrame, Platform, StreamId};

use crate::capture::{
    initialize_monotonic_timestamp_domain, monotonic_timestamp_ns, CaptureError as LoopbackError,
    CaptureMode, CaptureObservationCounters, CaptureObservationHandle, CaptureObservations,
    CaptureOpenCancellation, CaptureOpenFailure, CaptureSource, NativeCallOperation,
    NativeCallReporter, SourceKind, SourceState, StableSourceId,
};
use crate::timing::TimelineMapping;

#[repr(C)]
struct RawSourceInfo {
    audio_object_id: u32,
    process_id: i32,
    bundle_id: [u8; 256],
    name: [u8; 256],
    source_kind_code: u8,
    source_state_code: u8,
    sample_rate_hz: u32,
    channel_count: u16,
    process_start_time_ns: u64,
}

#[derive(Debug)]
struct AuditedCaptureSource {
    source: CaptureSource,
    process_start_time_ns: u64,
}

#[derive(Debug, PartialEq, Eq)]
struct ApplicationCaptureSelection {
    process_ids: Vec<i32>,
    stable_id: StableSourceId,
}

#[repr(C)]
struct NativeObserver {
    begin: extern "C" fn(*mut std::ffi::c_void, u32) -> u64,
    end: extern "C" fn(*mut std::ffi::c_void, u64, i32, u8),
    context: *mut std::ffi::c_void,
}
extern "C" fn native_call_begin(context: *mut std::ffi::c_void, operation: u32) -> u64 {
    if context.is_null() {
        return 0;
    }
    let Some(operation) = NativeCallOperation::from_code(operation) else {
        return 0;
    };
    // SAFETY: the exclusive reporter is borrowed for this synchronous call.
    // Native code never stores this pointer in a handle, ring or IOProc.
    unsafe { &*context.cast::<NativeCallReporter>() }.begin(operation)
}
extern "C" fn native_call_end(
    context: *mut std::ffi::c_void,
    call: u64,
    status: i32,
    has_status: u8,
) {
    if context.is_null() {
        return;
    }
    // SAFETY: same scoped single-writer reporter as native_call_begin.
    unsafe { &*context.cast::<NativeCallReporter>() }
        .end(call, (has_status != 0).then_some(status));
}
fn native_observer(reporter: Option<&NativeCallReporter>) -> Option<NativeObserver> {
    reporter.map(|reporter| NativeObserver {
        begin: native_call_begin,
        end: native_call_end,
        context: std::ptr::from_ref(reporter).cast_mut().cast(),
    })
}
fn observer_pointer(observer: &Option<NativeObserver>) -> *const NativeObserver {
    observer
        .as_ref()
        .map_or(std::ptr::null(), std::ptr::from_ref)
}

extern "C" {
    fn pks_process_tap_available() -> i32;
    fn pks_process_start_time_ns_observed(process_id: i32, observer: *const NativeObserver) -> u64;
    fn pks_discover_sources_observed(
        out: *mut RawSourceInfo,
        max_count: i32,
        observer: *const NativeObserver,
    ) -> i32;
    fn pks_create_process_tap_observed(
        pids: *const i32,
        process_count: i32,
        out_status: *mut i32,
        out_stage: *mut u8,
        cleanup_status: *mut i32,
        cleanup_stage: *mut u8,
        cancelled: extern "C" fn(*mut std::ffi::c_void) -> i32,
        cancellation: *mut std::ffi::c_void,
        observer: *const NativeObserver,
    ) -> *mut std::ffi::c_void;
    fn pks_tap_start_observed(
        tap: *mut std::ffi::c_void,
        frame_duration_ms: u16,
        cancelled: extern "C" fn(*mut std::ffi::c_void) -> i32,
        cancellation: *mut std::ffi::c_void,
        out_status: *mut i32,
        out_stage: *mut u8,
        observer: *const NativeObserver,
    ) -> i32;
    fn pks_destroy_process_tap_observed(
        tap: *mut std::ffi::c_void,
        status: *mut i32,
        stage: *mut u8,
        observer: *const NativeObserver,
    ) -> i32;
    fn pks_tap_read_frames_timed(
        tap: *mut std::ffi::c_void,
        out: *mut f32,
        frame_count: u32,
        out_source_frame_position_frames: *mut u64,
        out_anchor_frame_position_frames: *mut u64,
        out_anchor_host_time_ns: *mut u64,
    ) -> u32;
    fn pks_tap_current_host_time_ns() -> u64;
    fn pks_tap_drop_count(tap: *const std::ffi::c_void) -> u64;
    fn pks_tap_io_buffer_before_frames(tap: *const std::ffi::c_void) -> u32;
    fn pks_tap_io_buffer_requested_frames(tap: *const std::ffi::c_void) -> u32;
    fn pks_tap_io_buffer_applied_frames(tap: *const std::ffi::c_void) -> u32;
    fn pks_tap_io_buffer_min_frames(tap: *const std::ffi::c_void) -> u32;
    fn pks_tap_io_buffer_max_frames(tap: *const std::ffi::c_void) -> u32;
    fn pks_tap_input_device_latency_frames_observed(
        tap: *const std::ffi::c_void,
        observer: *const NativeObserver,
    ) -> u32;
    fn pks_tap_input_safety_offset_frames_observed(
        tap: *const std::ffi::c_void,
        observer: *const NativeObserver,
    ) -> u32;
    fn pks_tap_input_safety_offset_settable_observed(
        tap: *const std::ffi::c_void,
        observer: *const NativeObserver,
    ) -> u8;
    fn pks_tap_input_stream_latency_frames_observed(
        tap: *const std::ffi::c_void,
        observer: *const NativeObserver,
    ) -> u32;
    fn pks_tap_sample_rate(tap: *const std::ffi::c_void) -> u32;
    fn pks_tap_channels(tap: *const std::ffi::c_void) -> u32;
    fn pks_tap_level(tap: *const std::ffi::c_void) -> f32;
}

/// Returns `true` when the CoreAudio process tap API is available.
///
/// This is a **runtime** availability check, not a compile-time gate.  The
/// call is safe on any macOS version: on macOS < 14.2 the FFI symbol resolves
/// but returns 0 (unavailable), so callers on older systems get a clean `false`
/// rather than a link error or panic.  Code that calls `tap_available()` therefore
/// compiles and runs on all macOS versions and degrades gracefully when the host
/// is below 14.2.
///
/// The underlying API (`AudioHardwareCreateProcessTap` / `CATapDescription`) was
/// introduced in macOS 14.2 but is only publicly claimed to be supported on
/// macOS 14.4+ until runtime tests on 14.2/14.3 validate the earlier versions.
pub fn tap_available() -> bool {
    // SAFETY: The linked shim exposes a zero-argument availability probe with
    // no borrowed memory and no ownership transfer.
    unsafe {
        let _diagnostic_symbol = pks_tap_level;
        pks_process_tap_available() != 0
    }
}

/// Enumerate all running processes that have audio output.
/// Returns an empty `Vec` on macOS < 14.4 (public support floor) or on non-macOS platforms.
pub fn discover_sources_native() -> Vec<CaptureSource> {
    discover_sources_native_with_audit()
        .into_iter()
        .map(|audited| audited.source)
        .collect()
}

fn discover_sources_native_with_audit() -> Vec<AuditedCaptureSource> {
    discover_sources_native_observed(None)
}

fn discover_sources_native_observed(
    reporter: Option<&NativeCallReporter>,
) -> Vec<AuditedCaptureSource> {
    let observer = native_observer(reporter);
    const MAX: usize = 128;
    // SAFETY: write_bytes zeroes the allocation before set_len, so all MAX
    // elements are initialised.  pks_discover_sources then writes exactly `n`
    // valid entries into the first `n` slots; we truncate to that count.
    let raw: Vec<RawSourceInfo> = unsafe {
        let mut v: Vec<RawSourceInfo> = Vec::with_capacity(MAX);
        std::ptr::write_bytes(v.as_mut_ptr(), 0, MAX);
        v.set_len(MAX);
        let n =
            pks_discover_sources_observed(v.as_mut_ptr(), MAX as i32, observer_pointer(&observer));
        v.truncate(n.max(0) as usize);
        v
    };

    decode_discovered_sources(&raw)
}

fn decode_discovered_sources(raw: &[RawSourceInfo]) -> Vec<AuditedCaptureSource> {
    raw.iter()
        .map(|r| {
            let process_id = if r.process_id > 0 {
                Some(r.process_id as u32)
            } else {
                None
            };
            let source_kind = match r.source_kind_code {
                1 => SourceKind::InputDevice,
                2 => SourceKind::OutputDevice,
                3 => SourceKind::SystemMix,
                _ => SourceKind::Application,
            };
            let native_identity = cstr_to_opt(&r.bundle_id).map(|identity| {
                if source_kind == SourceKind::InputDevice {
                    super::input::canonical_input_device_id(&identity)
                } else {
                    identity
                }
            });
            let stable_key = native_identity
                .as_deref()
                .map(|id| id.to_owned())
                .unwrap_or_else(|| {
                    if matches!(
                        source_kind,
                        SourceKind::InputDevice | SourceKind::OutputDevice
                    ) {
                        format!("coreaudio-object:{}", r.audio_object_id)
                    } else {
                        format!("pid:{}", r.process_id)
                    }
                });
            let app_id = (source_kind == SourceKind::Application)
                .then(|| native_identity.clone())
                .flatten();
            let device_uid = matches!(
                source_kind,
                SourceKind::InputDevice | SourceKind::OutputDevice
            )
            .then_some(native_identity)
            .flatten();
            AuditedCaptureSource {
                source: CaptureSource {
                    stable_id: StableSourceId::new(Platform::Macos, source_kind, stable_key),
                    name: cstr_to_string(&r.name)
                        .unwrap_or_else(|| format!("pid:{}", r.process_id)),
                    process_id,
                    app_id,
                    device_uid,
                    state: match r.source_state_code {
                        1 => SourceState::Playing,
                        2 => SourceState::Silent,
                        3 => SourceState::Unavailable,
                        _ => SourceState::Available,
                    },
                    sample_rate_hz: r.sample_rate_hz,
                    channels: r.channel_count,
                },
                process_start_time_ns: r.process_start_time_ns,
            }
        })
        .collect()
}

fn select_application_capture(
    sources: &[CaptureSource],
    application: &str,
) -> Result<ApplicationCaptureSelection, LoopbackError> {
    let mut matches = sources.iter().filter(|source| {
        source.stable_id.kind == SourceKind::Application
            && (source.name.eq_ignore_ascii_case(application)
                || source
                    .app_id
                    .as_deref()
                    .is_some_and(|app_id| app_id.eq_ignore_ascii_case(application)))
    });
    let first = matches.next().ok_or_else(|| {
        LoopbackError::BackendInit(format!(
            "no running audio source found for application '{application}'"
        ))
    })?;
    let stable_id = first.stable_id.clone();
    let mut process_ids = first
        .process_id
        .map(|process_id| process_id as i32)
        .into_iter()
        .collect::<Vec<_>>();

    for source in matches {
        if source.stable_id != stable_id {
            return Err(LoopbackError::BackendInit(format!(
                "application '{application}' matches multiple running audio sources; select one from source discovery"
            )));
        }
        if let Some(process_id) = source.process_id {
            process_ids.push(process_id as i32);
        }
    }

    process_ids.sort_unstable();
    process_ids.dedup();
    if process_ids.is_empty() {
        return Err(LoopbackError::BackendInit(format!(
            "audio source for application '{application}' has no process identity"
        )));
    }

    Ok(ApplicationCaptureSelection {
        process_ids,
        stable_id,
    })
}

fn select_stable_application_capture(
    sources: &[CaptureSource],
    stable_id: &StableSourceId,
) -> Result<ApplicationCaptureSelection, LoopbackError> {
    if stable_id.platform != Platform::Macos || stable_id.kind != SourceKind::Application {
        return Err(LoopbackError::SourceUnavailable {
            stable_key: stable_id.stable_key.clone(),
        });
    }

    let mut process_ids = sources
        .iter()
        .filter(|source| source.stable_id == *stable_id)
        .filter_map(|source| source.process_id)
        .map(|process_id| process_id as i32)
        .collect::<Vec<_>>();
    process_ids.sort_unstable();
    process_ids.dedup();
    if process_ids.is_empty() {
        return Err(LoopbackError::SourceUnavailable {
            stable_key: stable_id.stable_key.clone(),
        });
    }

    Ok(ApplicationCaptureSelection {
        process_ids,
        stable_id: stable_id.clone(),
    })
}

fn cstr_to_string(buf: &[u8]) -> Option<String> {
    let end = buf.iter().position(|&b| b == 0).unwrap_or(buf.len());
    if end == 0 {
        return None;
    }
    Some(String::from_utf8_lossy(&buf[..end]).into_owned())
}

fn cstr_to_opt(buf: &[u8]) -> Option<String> {
    cstr_to_string(buf)
}

const MAX_PROCESS_TAP_OWNERS: u32 = 64;
struct TapAdmission {
    owners: AtomicU32,
    quarantined: AtomicBool,
}
static TAP_ADMISSION: TapAdmission = TapAdmission::new();
impl TapAdmission {
    const fn new() -> Self {
        Self {
            owners: AtomicU32::new(0),
            quarantined: AtomicBool::new(false),
        }
    }
    fn acquire(&self) -> Result<TapLease<'_>, LoopbackError> {
        if self.quarantined.load(Ordering::Acquire) {
            return Err(LoopbackError::BackendSetupRequired { backend: "coreaudio-process-tap",
                action: "native cleanup is unconfirmed; further process taps are unavailable in this process" });
        }
        self.owners
            .fetch_update(Ordering::AcqRel, Ordering::Acquire, |owners| {
                (owners < MAX_PROCESS_TAP_OWNERS).then_some(owners + 1)
            })
            .map_err(|_| LoopbackError::BackendSetupRequired {
                backend: "coreaudio-process-tap",
                action: "stop an owned process tap before opening more than 64 simultaneous taps",
            })?;
        let lease = TapLease {
            admission: self,
            retained: false,
        };
        if self.quarantined.load(Ordering::Acquire) {
            return Err(LoopbackError::BackendSetupRequired { backend: "coreaudio-process-tap",
                action: "native cleanup is unconfirmed; further process taps are unavailable in this process" });
        }
        Ok(lease)
    }
}
struct TapLease<'a> {
    admission: &'a TapAdmission,
    retained: bool,
}
impl TapLease<'_> {
    fn quarantine(&mut self) {
        self.retained = true;
        self.admission.quarantined.store(true, Ordering::Release);
    }
}
impl Drop for TapLease<'_> {
    fn drop(&mut self) {
        if !self.retained {
            self.admission.owners.fetch_sub(1, Ordering::AcqRel);
        }
    }
}
#[derive(Default)]
struct TapCleanupReceipt {
    completed: AtomicBool,
    status: AtomicI32,
    stage: AtomicU32,
}
impl TapCleanupReceipt {
    fn error(&self) -> Option<LoopbackError> {
        if !self.completed.load(Ordering::Acquire) {
            return None;
        }
        let stage = self.stage.load(Ordering::Relaxed) as u8;
        (stage != 0).then(|| tap_error(self.status.load(Ordering::Relaxed), stage))
    }
}
struct ProcessTap {
    handle: NonNull<std::ffi::c_void>,
    lease: TapLease<'static>,
    cleanup_attempted: bool,
    cleanup: Arc<TapCleanupReceipt>,
    native_calls: Option<NativeCallReporter>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct ProcessTapIoBuffer {
    before_frames: u32,
    requested_frames: u32,
    applied_frames: u32,
    minimum_frames: u32,
    maximum_frames: u32,
    input_device_latency_frames: u32,
    input_safety_offset_frames: u32,
    input_safety_offset_settable: bool,
    input_stream_latency_frames: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct ProcessTapReadBatch {
    frame_count: u32,
    source_frame_position_frames: u64,
    anchor_frame_position_frames: u64,
    anchor_host_time_ns: u64,
}

fn source_host_timestamp_ns(batch: ProcessTapReadBatch, sample_rate_hz: u32) -> Option<u64> {
    if batch.frame_count == 0 || batch.anchor_host_time_ns == 0 || sample_rate_hz == 0 {
        return None;
    }
    let frame_delta = i128::from(batch.source_frame_position_frames)
        .checked_sub(i128::from(batch.anchor_frame_position_frames))?;
    let timestamp_delta_ns = frame_delta
        .checked_mul(1_000_000_000)?
        .checked_div(i128::from(sample_rate_hz))?;
    let timestamp_ns = i128::from(batch.anchor_host_time_ns).checked_add(timestamp_delta_ns)?;
    u64::try_from(timestamp_ns).ok().filter(|value| *value != 0)
}

fn process_timestamp_ns(
    batch: ProcessTapReadBatch,
    sample_rate_hz: u32,
    host_to_process: TimelineMapping,
) -> Option<u64> {
    host_to_process.normalize_timestamp_ns(source_host_timestamp_ns(batch, sample_rate_hz)?)
}

const fn process_tap_io_duration_ms(audio_frame_duration: crate::frame::AudioFrameDuration) -> u16 {
    // Keep the native callback at no more than 10 ms. Twenty-millisecond
    // product frames then comprise exactly two native batches instead of
    // straddling the aggregate device's 512-frame default callback cadence.
    match audio_frame_duration {
        crate::frame::AudioFrameDuration::Ms10 | crate::frame::AudioFrameDuration::Ms20 => 10,
    }
}

const CORE_AUDIO_PERMISSION_DENIED_STATUS: i32 = i32::from_be_bytes(*b"!hog");

fn tap_operation(stage_code: u8) -> &'static str {
    match stage_code {
        1 => "resolving the selected process",
        2 => "creating the CoreAudio process tap",
        3 => "reading the CoreAudio process tap identifier",
        4 => "creating the CoreAudio aggregate device",
        5 => "allocating the CoreAudio process tap handle",
        6 => "creating the CoreAudio device callback",
        7 => "starting the CoreAudio aggregate device",
        8 => "checking CoreAudio process tap platform support",
        9 => "stopping the CoreAudio aggregate device",
        10 => "unregistering the CoreAudio device callback",
        11 => "destroying the CoreAudio aggregate device",
        12 => "destroying the CoreAudio process tap",
        13 => "retaining uncertain CoreAudio resource ownership",
        _ => "opening the CoreAudio process tap",
    }
}

fn tap_error(status_code: i32, stage_code: u8) -> LoopbackError {
    let operation = tap_operation(stage_code);
    if status_code == CORE_AUDIO_PERMISSION_DENIED_STATUS {
        LoopbackError::PermissionDenied { operation }
    } else {
        LoopbackError::BackendStatus {
            operation,
            status_code,
        }
    }
}

fn stable_source_id(mode: &CaptureMode) -> Result<StableSourceId, LoopbackError> {
    match mode {
        CaptureMode::SystemMix => Ok(StableSourceId::new(
            Platform::Macos,
            SourceKind::SystemMix,
            "system:mix",
        )),
        CaptureMode::Process(pid) => Ok(StableSourceId::new(
            Platform::Macos,
            SourceKind::Application,
            format!("pid:{pid}"),
        )),
        CaptureMode::ExactApplication { stable_id, .. }
        | CaptureMode::ExactApplicationStable { stable_id } => Ok(stable_id.clone()),
        CaptureMode::Application(bundle_id) => Ok(StableSourceId::new(
            Platform::Macos,
            SourceKind::Application,
            bundle_id.clone(),
        )),
        CaptureMode::InputDevice(_) => Err(LoopbackError::ModeUnsupported(mode.clone())),
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct ExactApplicationOpenAudit {
    process_id: u32,
    stable_id: StableSourceId,
    process_start_time_ns: u64,
}

fn exact_application_open_audit(
    sources: &[AuditedCaptureSource],
    process_id: u32,
    stable_id: &StableSourceId,
) -> Option<ExactApplicationOpenAudit> {
    sources
        .iter()
        .find(|audited| {
            audited.process_start_time_ns != 0
                && audited.source.process_id == Some(process_id)
                && audited.source.stable_id == *stable_id
                && audited.source.stable_id.kind == SourceKind::Application
        })
        .map(|audited| ExactApplicationOpenAudit {
            process_id,
            stable_id: stable_id.clone(),
            process_start_time_ns: audited.process_start_time_ns,
        })
}

fn application_open_audit_for_process(
    sources: &[AuditedCaptureSource],
    process_id: u32,
) -> Option<ExactApplicationOpenAudit> {
    sources
        .iter()
        .find(|audited| {
            audited.process_start_time_ns != 0
                && audited.source.process_id == Some(process_id)
                && audited.source.stable_id.kind == SourceKind::Application
        })
        .map(|audited| ExactApplicationOpenAudit {
            process_id,
            stable_id: audited.source.stable_id.clone(),
            process_start_time_ns: audited.process_start_time_ns,
        })
}

fn capture_application_open_audit_for_process(
    process_id: u32,
    reporter: Option<&NativeCallReporter>,
) -> Result<ExactApplicationOpenAudit, LoopbackError> {
    application_open_audit_for_process(&discover_sources_native_observed(reporter), process_id)
        .ok_or_else(|| LoopbackError::SourceUnavailable {
            stable_key: format!("pid:{process_id}"),
        })
}

fn capture_exact_application_open_audit(
    process_id: u32,
    stable_id: &StableSourceId,
    reporter: Option<&NativeCallReporter>,
) -> Result<ExactApplicationOpenAudit, LoopbackError> {
    exact_application_open_audit(
        &discover_sources_native_observed(reporter),
        process_id,
        stable_id,
    )
    .ok_or_else(|| LoopbackError::SourceUnavailable {
        stable_key: stable_id.stable_key.clone(),
    })
}

fn capture_application_open_audits(
    selection: &ApplicationCaptureSelection,
    reporter: Option<&NativeCallReporter>,
) -> Result<Vec<ExactApplicationOpenAudit>, LoopbackError> {
    let sources = discover_sources_native_observed(reporter);
    selection
        .process_ids
        .iter()
        .map(|process_id| {
            u32::try_from(*process_id)
                .ok()
                .and_then(|process_id| {
                    exact_application_open_audit(&sources, process_id, &selection.stable_id)
                })
                .ok_or_else(|| LoopbackError::SourceUnavailable {
                    stable_key: selection.stable_id.stable_key.clone(),
                })
        })
        .collect()
}

fn selected_application_is_running_with(
    audits: &[ExactApplicationOpenAudit],
    mut process_start_time: impl FnMut(u32) -> u64,
) -> bool {
    audits.is_empty()
        || audits
            .iter()
            .any(|audit| process_start_time(audit.process_id) == audit.process_start_time_ns)
}

fn selected_application_is_running(audits: &[ExactApplicationOpenAudit]) -> bool {
    selected_application_is_running_observed(audits, None)
}
fn selected_application_is_running_observed(
    audits: &[ExactApplicationOpenAudit],
    reporter: Option<&NativeCallReporter>,
) -> bool {
    let observer = native_observer(reporter);
    selected_application_is_running_with(audits, |process_id| {
        let Ok(process_id) = i32::try_from(process_id) else {
            return 0;
        };
        // SAFETY: this control-thread query borrows no memory and returns zero
        // when the selected process instance no longer exists.
        unsafe { pks_process_start_time_ns_observed(process_id, observer_pointer(&observer)) }
    })
}

fn verify_application_open_audits(
    audits: &[ExactApplicationOpenAudit],
    reporter: Option<&NativeCallReporter>,
) -> Result<(), LoopbackError> {
    if selected_application_is_running_observed(audits, reporter) {
        return Ok(());
    }
    Err(LoopbackError::SourceUnavailable {
        stable_key: audits
            .first()
            .map(|audit| audit.stable_id.stable_key.clone())
            .unwrap_or_else(|| "selected-application".to_owned()),
    })
}

// SAFETY:
// - PksProcessTapHandle is heap-allocated and exclusively owned by this struct.
// - pks_tap_read_frames is called only from the single thread that owns ProcessTap.
// - The IO callback writes to the ring from CoreAudio's RT thread; synchronisation
//   is through the atomic write_head.
// - AudioDeviceStart/Stop are safe to call from any thread.
unsafe impl Send for ProcessTap {}

impl ProcessTap {
    fn global(
        cancellation: &CaptureOpenCancellation,
        native_calls: Option<NativeCallReporter>,
    ) -> Result<Self, CaptureOpenFailure> {
        Self::create(std::ptr::null(), 0, cancellation, native_calls)
    }

    fn for_pids(
        pids: &[i32],
        cancellation: &CaptureOpenCancellation,
        native_calls: Option<NativeCallReporter>,
    ) -> Result<Self, CaptureOpenFailure> {
        Self::create(pids.as_ptr(), pids.len() as i32, cancellation, native_calls)
    }

    fn create(
        pids: *const i32,
        process_count: i32,
        cancellation: &CaptureOpenCancellation,
        native_calls: Option<NativeCallReporter>,
    ) -> Result<Self, CaptureOpenFailure> {
        cancellation.check()?;
        let mut lease = TAP_ADMISSION.acquire()?;
        let observer = native_observer(native_calls.as_ref());
        let mut status_code = 0;
        let mut stage_code = 0;
        let mut cleanup_status = 0;
        let mut cleanup_stage = 0;
        // SAFETY: pids addresses process_count valid i32 values or is null when
        // process_count is zero; both out-pointers live through this call.
        let handle = unsafe {
            pks_create_process_tap_observed(
                pids,
                process_count,
                &mut status_code,
                &mut stage_code,
                &mut cleanup_status,
                &mut cleanup_stage,
                capture_open_cancelled,
                std::ptr::from_ref(cancellation).cast_mut().cast(),
                observer_pointer(&observer),
            )
        };
        match NonNull::new(handle) {
            Some(handle) => Ok(Self {
                handle,
                lease,
                cleanup_attempted: false,
                cleanup: Arc::new(TapCleanupReceipt::default()),
                native_calls,
            }),
            None => {
                if cleanup_stage != 0 {
                    lease.quarantine();
                }
                let cleanup_error =
                    (cleanup_stage != 0).then(|| tap_error(cleanup_status, cleanup_stage));
                if status_code == 0 && stage_code == 0 && cancellation.is_requested() {
                    Err(CaptureOpenFailure::Cancelled { cleanup_error })
                } else {
                    Err(CaptureOpenFailure::Backend {
                        source: tap_error(status_code, stage_code),
                        cleanup_error,
                    })
                }
            }
        }
    }

    fn start(
        &mut self,
        audio_frame_duration: crate::frame::AudioFrameDuration,
        cancellation: &CaptureOpenCancellation,
    ) -> Result<(), CaptureOpenFailure> {
        let observer = native_observer(self.native_calls.as_ref());
        let mut status_code = 0;
        let mut stage_code = 0;
        // SAFETY: self owns a live tap handle and both out-pointers live through
        // this call.
        let result = unsafe {
            pks_tap_start_observed(
                self.handle.as_ptr(),
                process_tap_io_duration_ms(audio_frame_duration),
                capture_open_cancelled,
                std::ptr::from_ref(cancellation).cast_mut().cast(),
                &mut status_code,
                &mut stage_code,
                observer_pointer(&observer),
            )
        };
        match result {
            0 => Ok(()),
            -2 => Err(CaptureOpenFailure::Cancelled {
                cleanup_error: None,
            }),
            _ => Err(tap_error(status_code, stage_code).into()),
        }
    }

    fn close(&mut self) -> Result<(), LoopbackError> {
        if !self.cleanup_attempted {
            self.cleanup_attempted = true;
            let observer = native_observer(self.native_calls.as_ref());
            let mut status = 0;
            let mut stage = 0;
            // SAFETY: exclusive owner; this call either frees on confirmed cleanup
            // or retains native callback context. No handle access follows it.
            let result = unsafe {
                pks_destroy_process_tap_observed(
                    self.handle.as_ptr(),
                    &mut status,
                    &mut stage,
                    observer_pointer(&observer),
                )
            };
            if result != 0 {
                self.lease.quarantine();
            }
            self.cleanup.status.store(status, Ordering::Relaxed);
            self.cleanup
                .stage
                .store(u32::from(stage), Ordering::Relaxed);
            self.cleanup.completed.store(true, Ordering::Release);
        }
        self.cleanup.error().map_or(Ok(()), Err)
    }
    fn fail_open(mut self, failure: CaptureOpenFailure) -> CaptureOpenFailure {
        failure.with_cleanup(self.close().err())
    }

    fn io_buffer(&self) -> ProcessTapIoBuffer {
        let observer = native_observer(self.native_calls.as_ref());
        // SAFETY: self owns a live tap handle for the duration of these reads.
        unsafe {
            ProcessTapIoBuffer {
                before_frames: pks_tap_io_buffer_before_frames(self.handle.as_ptr()),
                requested_frames: pks_tap_io_buffer_requested_frames(self.handle.as_ptr()),
                applied_frames: pks_tap_io_buffer_applied_frames(self.handle.as_ptr()),
                minimum_frames: pks_tap_io_buffer_min_frames(self.handle.as_ptr()),
                maximum_frames: pks_tap_io_buffer_max_frames(self.handle.as_ptr()),
                input_device_latency_frames: pks_tap_input_device_latency_frames_observed(
                    self.handle.as_ptr(),
                    observer_pointer(&observer),
                ),
                input_safety_offset_frames: pks_tap_input_safety_offset_frames_observed(
                    self.handle.as_ptr(),
                    observer_pointer(&observer),
                ),
                input_safety_offset_settable: pks_tap_input_safety_offset_settable_observed(
                    self.handle.as_ptr(),
                    observer_pointer(&observer),
                ) != 0,
                input_stream_latency_frames: pks_tap_input_stream_latency_frames_observed(
                    self.handle.as_ptr(),
                    observer_pointer(&observer),
                ),
            }
        }
    }

    fn sample_rate_hz(&self) -> u32 {
        // SAFETY: self owns a live tap handle for the duration of the call.
        unsafe { pks_tap_sample_rate(self.handle.as_ptr()) }
    }

    fn channel_count(&self) -> u32 {
        // SAFETY: self owns a live tap handle for the duration of the call.
        unsafe { pks_tap_channels(self.handle.as_ptr()) }
    }

    fn read_frames(&mut self, out: &mut [f32], frame_count: u32) -> ProcessTapReadBatch {
        let required_samples = frame_count as usize * self.channel_count() as usize;
        if out.len() < required_samples {
            return ProcessTapReadBatch {
                frame_count: 0,
                source_frame_position_frames: 0,
                anchor_frame_position_frames: 0,
                anchor_host_time_ns: 0,
            };
        }
        let mut source_frame_position_frames = 0;
        let mut anchor_frame_position_frames = 0;
        let mut anchor_host_time_ns = 0;
        // SAFETY: self owns the live tap, and out contains at least
        // frame_count * channel_count writable f32 samples. All out-pointers
        // refer to live u64 values for the duration of the call.
        let read_frame_count = unsafe {
            pks_tap_read_frames_timed(
                self.handle.as_ptr(),
                out.as_mut_ptr(),
                frame_count,
                &mut source_frame_position_frames,
                &mut anchor_frame_position_frames,
                &mut anchor_host_time_ns,
            )
        };
        ProcessTapReadBatch {
            frame_count: read_frame_count,
            source_frame_position_frames,
            anchor_frame_position_frames,
            anchor_host_time_ns,
        }
    }

    fn current_host_time_ns() -> u64 {
        // SAFETY: this reads the platform monotonic clock and owns no memory.
        unsafe { pks_tap_current_host_time_ns() }
    }

    fn drop_count(&self) -> u64 {
        // SAFETY: self owns a live tap handle for the duration of the call.
        unsafe { pks_tap_drop_count(self.handle.as_ptr()) }
    }
}

/// Drop contract — this is a control-thread-only owner:
///   destroy exactly once · panic-free · no Rust allocation · no Rust logging
impl Drop for ProcessTap {
    fn drop(&mut self) {
        let _ = self.close();
    }
}
extern "C" fn capture_open_cancelled(context: *mut std::ffi::c_void) -> i32 {
    // SAFETY: start_cancellable borrows this token synchronously; native code
    // never stores the pointer or calls it from the audio callback.
    let cancellation = unsafe { &*context.cast::<CaptureOpenCancellation>() };
    i32::from(cancellation.is_requested())
}

// Process-tap callbacks can arrive as 10 ms buffers while the public pipeline
// consumes 20 ms frames. Keep bounded ownership for a full downstream burst;
// empty pool slots add memory headroom, not playout latency.
const POOL_CAPACITY_FRAMES: usize = 32;
const PROCESS_LIFETIME_POLL_INTERVAL: Duration = Duration::from_millis(100);
const EMPTY_RING_POLL_INTERVAL: Duration = Duration::from_micros(500);

/// Captures system audio via CoreAudio process tap (macOS 14.2+).
pub struct TapLoopbackSource {
    reader_thread: Option<std::thread::JoinHandle<()>>,
    cleanup: Arc<TapCleanupReceipt>,
    pub(crate) stop_tx: std::sync::mpsc::SyncSender<()>,
    counters: CaptureObservationCounters,
    source_id: crate::frame::SourceId,
}

impl TapLoopbackSource {
    pub(crate) fn capture_mode_cancellable<F>(
        mode: CaptureMode,
        audio_frame_duration: crate::frame::AudioFrameDuration,
        mut callback: F,
        runtime_event_sender: Option<crate::capture::SourceRuntimeEventSender>,
        cancellation: &CaptureOpenCancellation,
    ) -> Result<Self, CaptureOpenFailure>
    where
        F: FnMut(AudioFrame) + Send + 'static,
    {
        cancellation.check()?;
        if !tap_available() {
            return Err(LoopbackError::BackendInit(
                "CoreAudio process tap requires macOS 14.4 or later".into(),
            )
            .into());
        }

        let native_calls = cancellation.native_call_reporter();
        let (mut tap, stable_id, application_open_audits) = match &mode {
            CaptureMode::SystemMix => (
                ProcessTap::global(cancellation, native_calls)?,
                stable_source_id(&mode)?,
                Vec::new(),
            ),
            CaptureMode::Process(pid) => {
                let audit =
                    capture_application_open_audit_for_process(*pid, native_calls.as_ref())?;
                let stable_id = audit.stable_id.clone();
                (
                    ProcessTap::for_pids(&[*pid as i32], cancellation, native_calls)?,
                    stable_id,
                    vec![audit],
                )
            }
            CaptureMode::ExactApplication {
                process_id,
                stable_id,
            } => {
                let audit = capture_exact_application_open_audit(
                    *process_id,
                    stable_id,
                    native_calls.as_ref(),
                )?;
                (
                    ProcessTap::for_pids(&[*process_id as i32], cancellation, native_calls)?,
                    stable_id.clone(),
                    vec![audit],
                )
            }
            CaptureMode::ExactApplicationStable { stable_id } => {
                let sources = discover_sources_native_observed(native_calls.as_ref())
                    .into_iter()
                    .map(|audited| audited.source)
                    .collect::<Vec<_>>();
                let selected = select_stable_application_capture(&sources, stable_id)?;
                let audits = capture_application_open_audits(&selected, native_calls.as_ref())?;
                (
                    ProcessTap::for_pids(&selected.process_ids, cancellation, native_calls)?,
                    selected.stable_id,
                    audits,
                )
            }
            CaptureMode::Application(application) => {
                let sources = discover_sources_native_observed(native_calls.as_ref())
                    .into_iter()
                    .map(|audited| audited.source)
                    .collect::<Vec<_>>();
                let selected = select_application_capture(&sources, application)?;
                if std::env::var_os("PKS_TAP_DIAG").is_some() {
                    eprintln!(
                        "tap_diag: application={} sources={} process_ids={:?}",
                        application,
                        sources.len(),
                        selected.process_ids
                    );
                }
                let audits = capture_application_open_audits(&selected, native_calls.as_ref())?;
                (
                    ProcessTap::for_pids(&selected.process_ids, cancellation, native_calls)?,
                    selected.stable_id,
                    audits,
                )
            }
            CaptureMode::InputDevice(_) => {
                return Err(LoopbackError::ModeUnsupported(mode).into());
            }
        };

        if let Err(failure) = tap.start(audio_frame_duration, cancellation) {
            return Err(tap.fail_open(failure));
        }
        if let Err(error) =
            verify_application_open_audits(&application_open_audits, tap.native_calls.as_ref())
        {
            return Err(tap.fail_open(error.into()));
        }

        let sample_rate_hz = tap.sample_rate_hz();
        if sample_rate_hz == 0 {
            return Err(tap.fail_open(
                LoopbackError::BackendInit("tap reported a zero sample rate".to_owned()).into(),
            ));
        }
        let channel_count = tap.channel_count() as u8;
        if std::env::var_os("PKS_TAP_DIAG").is_some() {
            let io_buffer = tap.io_buffer();
            eprintln!(
                "tap_diag: io_buffer_before_frames={} io_buffer_requested_frames={} io_buffer_applied_frames={} io_buffer_min_frames={} io_buffer_max_frames={} input_device_latency_frames={} input_safety_offset_frames={} input_safety_offset_settable={} input_stream_latency_frames={}",
                io_buffer.before_frames,
                io_buffer.requested_frames,
                io_buffer.applied_frames,
                io_buffer.minimum_frames,
                io_buffer.maximum_frames,
                io_buffer.input_device_latency_frames,
                io_buffer.input_safety_offset_frames,
                io_buffer.input_safety_offset_settable,
                io_buffer.input_stream_latency_frames,
            );
        }
        initialize_monotonic_timestamp_domain();
        let host_time_before_ns = ProcessTap::current_host_time_ns();
        let process_time_ns = monotonic_timestamp_ns();
        let host_time_after_ns = ProcessTap::current_host_time_ns();
        if host_time_before_ns == 0 || host_time_after_ns < host_time_before_ns {
            return Err(tap.fail_open(
                LoopbackError::BackendInit("CoreAudio host-time mapping is unavailable".to_owned())
                    .into(),
            ));
        }
        let host_time_midpoint_ns =
            host_time_before_ns.saturating_add((host_time_after_ns - host_time_before_ns) / 2);
        let host_to_process = TimelineMapping::new(host_time_midpoint_ns, process_time_ns);
        let callback_frame_count =
            u32::try_from(audio_frame_duration.samples_per_channel(sample_rate_hz))
                .unwrap_or(u32::MAX)
                .max(1);
        let buffer_capacity_samples = callback_frame_count as usize * channel_count as usize;
        let mut frame_normalizer = CaptureFrameNormalizer::new(
            callback_frame_count as usize,
            channel_count,
            sample_rate_hz,
        );
        let pool = AudioBufferPool::new(POOL_CAPACITY_FRAMES, buffer_capacity_samples);
        let (stop_tx, stop_rx) = std::sync::mpsc::sync_channel::<()>(1);
        let counters = CaptureObservationCounters::default();
        let capture_counters = counters.clone();

        let source_id = stable_id.source_id();
        let failure_counters = counters.clone();
        let runtime_stable_id = stable_id.clone();

        if let Err(failure) = cancellation.check() {
            return Err(tap.fail_open(failure));
        }
        let cleanup = Arc::clone(&tap.cleanup);
        let thread = std::thread::Builder::new()
            .name("pks-tap-reader".into())
            .spawn(move || {
                let worker = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    let mut sequence_num: u64 = 0;
                    let mut buffer = vec![0.0f32; buffer_capacity_samples];
                    let mut observed_drop_count = tap.drop_count();
                    let mut next_process_lifetime_check =
                        Instant::now() + PROCESS_LIFETIME_POLL_INTERVAL;
                    loop {
                        if stop_rx.try_recv().is_ok() {
                            break;
                        }
                        if Instant::now() >= next_process_lifetime_check {
                            if !selected_application_is_running(&application_open_audits) {
                                if let Some(sender) = runtime_event_sender.as_ref() {
                                    let _ = sender.try_send(
                                        crate::capture::SourceRuntimeEvent::SourceUnavailable {
                                            stable_id: runtime_stable_id.clone(),
                                            generation: crate::capture::SourceGeneration::INITIAL,
                                            recovery_requirement: crate::capture::SourceRecoveryRequirement::ExplicitRediscoveryAndNewSession,
                                            failure: crate::capture::CaptureRuntimeFailure {
                                                operation: "observe selected application lifetime",
                                                error_class: crate::capture::CaptureRuntimeFailureClass::SourceInstanceExited,
                                            },
                                        },
                                    );
                                }
                                break;
                            }
                            next_process_lifetime_check =
                                Instant::now() + PROCESS_LIFETIME_POLL_INTERVAL;
                        }
                        let batch = tap.read_frames(&mut buffer, callback_frame_count);
                        let frame_count = batch.frame_count;
                        let drop_count = tap.drop_count();
                        capture_counters.observe_dispatch_queue_full_frames(
                            drop_count.saturating_sub(observed_drop_count),
                        );
                        observed_drop_count = drop_count;
                        if frame_count == 0 {
                            std::thread::sleep(EMPTY_RING_POLL_INTERVAL);
                            continue;
                        }
                        capture_counters.observe_callback_buffer();
                        let Some(timestamp_ns) =
                            process_timestamp_ns(batch, sample_rate_hz, host_to_process)
                        else {
                            capture_counters.observe_stream_error();
                            if let Some(sender) = runtime_event_sender.as_ref() {
                                let _ = crate::capture::publish_backend_failure(
                                    sender,
                                    stable_id.clone(),
                                    crate::capture::SourceGeneration::INITIAL,
                                    "macOS tap reader",
                                    crate::capture::CaptureRuntimeFailureClass::BackendClass {
                                        class: "native-host-timeline-unavailable".to_owned(),
                                    },
                                );
                            }
                            break;
                        };
                        let sample_count = frame_count as usize * channel_count as usize;
                        if sample_count > buffer.len() {
                            capture_counters.observe_oversized_buffer();
                            continue;
                        }
                        let normalized = frame_normalizer.push(
                            &buffer[..sample_count],
                            timestamp_ns,
                            |timestamp_ns, samples| {
                                let frame_sequence_number = sequence_num;
                                sequence_num = sequence_num.saturating_add(1);
                                let Some(mut handle) = pool.acquire() else {
                                    capture_counters.observe_pool_exhaustion();
                                    return;
                                };
                                if handle.try_copy_from_slice(samples).is_err() {
                                    capture_counters.observe_oversized_buffer();
                                    return;
                                }
                                let mut frame = AudioFrame::new(
                                    StreamId(0),
                                    source_id,
                                    frame_sequence_number,
                                    timestamp_ns,
                                    channel_count,
                                    handle,
                                );
                                frame.sample_rate_hz = sample_rate_hz;
                                capture_counters.observe_enqueued_frame();
                                callback(frame);
                            },
                        );
                        if !normalized {
                            capture_counters.observe_invalid_buffer();
                        }
                    }
                }));
                if let Err(payload) = worker {
                    failure_counters.observe_stream_error();
                    if let Some(sender) = runtime_event_sender.as_ref() {
                        let _ = crate::capture::publish_backend_failure(
                            sender,
                            runtime_stable_id,
                            crate::capture::SourceGeneration::INITIAL,
                            "macOS tap reader",
                            crate::capture::CaptureRuntimeFailureClass::BackendClass {
                                class: "reader-panicked".to_owned(),
                            },
                        );
                    }
                    std::panic::resume_unwind(payload);
                }
            })
            .map_err(|e| CaptureOpenFailure::Backend {
                source: LoopbackError::BackendInit(format!("thread spawn: {e}")),
                cleanup_error: cleanup.error(),
            })?;

        Ok(Self {
            reader_thread: Some(thread),
            cleanup,
            stop_tx,
            counters,
            source_id,
        })
    }

    pub fn source_id(&self) -> crate::frame::SourceId {
        self.source_id
    }

    pub fn observations(&self) -> CaptureObservations {
        self.counters.snapshot()
    }

    pub fn observation_handle(&self) -> CaptureObservationHandle {
        self.counters.observation_handle()
    }

    pub(crate) fn stop_and_join(&mut self) -> Result<CaptureObservations, LoopbackError> {
        let counters = self.counters.clone();
        self.stop_reader()?;
        Ok(counters.snapshot())
    }

    fn stop_reader(&mut self) -> Result<(), LoopbackError> {
        let _ = self.stop_tx.try_send(());
        let joined = self.reader_thread.take().map_or(Ok(()), |thread| {
            crate::capture::join_capture_worker(thread, "macOS tap reader")
        });
        // Native ownership uncertainty takes precedence over a reader panic:
        // the caller must not infer that joining reclaimed callback context.
        match self.cleanup.error() {
            Some(error) => Err(error),
            None => joined,
        }
    }
}

/// Drop contract — control thread only: signal and join the owned reader.
impl Drop for TapLoopbackSource {
    fn drop(&mut self) {
        let _ = self.stop_reader();
    }
}

/// Fault operations call the production C ownership controller, never HAL.
#[cfg(test)]
pub(crate) mod late_cancellation_tests {
    use super::{tap_error, TapAdmission, MAX_PROCESS_TAP_OWNERS};
    use crate::capture::{CaptureOpenCancellation, CaptureOpenFailure};
    use std::ffi::c_void;
    use std::sync::atomic::Ordering;
    use std::sync::{mpsc, Arc, Barrier};
    use std::time::Duration;

    #[repr(C)]
    #[derive(Default)]
    struct Control {
        registered: u8,
        start_attempted: u8,
        started: u8,
        cleanup_attempted: u8,
        retained: u8,
        aggregate_owned: u8,
        tap_owned: u8,
        registration_uncertain: u8,
        cleanup_status: i32,
        cleanup_stage: u8,
        reserved: [u8; 3],
    }
    #[repr(C)]
    struct Operations {
        register_io: extern "C" fn(*mut c_void) -> i32,
        start: extern "C" fn(*mut c_void) -> i32,
        stop: extern "C" fn(*mut c_void) -> i32,
        unregister_io: extern "C" fn(*mut c_void) -> i32,
        destroy_aggregate: extern "C" fn(*mut c_void) -> i32,
        destroy_tap: extern "C" fn(*mut c_void) -> i32,
    }
    extern "C" {
        fn pks_tap_control_start_observed(
            state: *mut Control,
            ops: *const Operations,
            context: *mut c_void,
            cancelled: extern "C" fn(*mut c_void) -> i32,
            cancellation: *mut c_void,
            status: *mut i32,
            stage: *mut u8,
            observer: *const super::NativeObserver,
        ) -> i32;
        fn pks_tap_control_cleanup_observed(
            state: *mut Control,
            ops: *const Operations,
            context: *mut c_void,
            status: *mut i32,
            stage: *mut u8,
            observer: *const super::NativeObserver,
        ) -> i32;
    }
    #[derive(Clone, Copy, Default)]
    pub(crate) struct Fault {
        pub register_status: i32,
        pub start_status: i32,
        pub cleanup_stage: u8,
        pub cancel_at: u8,
        pub known_registration_rejection: bool,
        pub registered_on_error: bool,
        pub pause_stage: u8,
    }
    pub(crate) type RegistrationBarrier = (mpsc::SyncSender<()>, mpsc::Receiver<()>);
    struct Script {
        state: *mut Control,
        fault: Fault,
        cancellation: CaptureOpenCancellation,
        trace: Vec<&'static str>,
        barrier: Option<RegistrationBarrier>,
    }
    fn operation(context: *mut c_void, stage: u8, name: &'static str) -> i32 {
        // SAFETY: run_fixture owns Script and Control for the synchronous C
        // call; callbacks are serialized and no pointer escapes the call.
        let script = unsafe { &mut *context.cast::<Script>() };
        script.trace.push(name);
        let paused_stage = if script.fault.pause_stage == 0 {
            6
        } else {
            script.fault.pause_stage
        };
        if stage == paused_stage {
            if let Some((entered, release)) = script.barrier.take() {
                if entered.send(()).is_err()
                    || release.recv_timeout(Duration::from_secs(5)).is_err()
                {
                    return -999;
                }
            }
        }
        if stage == 6 {
            if script.fault.cancel_at == 2 {
                script.cancellation.request();
            }
            if script.fault.known_registration_rejection {
                // SAFETY: same exclusive, live C state as described above.
                unsafe {
                    (*script.state).registration_uncertain = 0;
                }
            }
            if script.fault.registered_on_error {
                // SAFETY: model the real operation returning a non-null ID.
                unsafe {
                    (*script.state).registered = 1;
                    (*script.state).registration_uncertain = 0;
                }
            }
            script.fault.register_status
        } else if stage == 7 {
            if script.fault.cancel_at == 3 {
                script.cancellation.request();
            }
            script.fault.start_status
        } else if stage == script.fault.cleanup_stage {
            -700 - i32::from(stage)
        } else {
            0
        }
    }
    extern "C" fn register(context: *mut c_void) -> i32 {
        operation(context, 6, "register")
    }
    extern "C" fn start(context: *mut c_void) -> i32 {
        operation(context, 7, "start")
    }
    extern "C" fn stop(context: *mut c_void) -> i32 {
        operation(context, 9, "stop")
    }
    extern "C" fn unregister(context: *mut c_void) -> i32 {
        operation(context, 10, "unregister")
    }
    extern "C" fn aggregate(context: *mut c_void) -> i32 {
        operation(context, 11, "aggregate")
    }
    extern "C" fn tap(context: *mut c_void) -> i32 {
        operation(context, 12, "tap")
    }
    const OPS: Operations = Operations {
        register_io: register,
        start,
        stop,
        unregister_io: unregister,
        destroy_aggregate: aggregate,
        destroy_tap: tap,
    };

    pub(crate) struct Receipt {
        pub failure: Option<CaptureOpenFailure>,
        pub trace: Vec<&'static str>,
        pub start_result: i32,
        pub primary_status: i32,
        pub primary_stage: u8,
        pub cleanup_status: i32,
        pub cleanup_stage: u8,
        pub retained: bool,
        pub repeat_before_cleanup_operations: usize,
    }
    pub(crate) fn run_fixture(
        cancellation: &CaptureOpenCancellation,
        fault: Fault,
        barrier: Option<RegistrationBarrier>,
    ) -> Receipt {
        let reporter = cancellation.native_call_reporter();
        let observer = super::native_observer(reporter.as_ref());
        let mut state = Control {
            aggregate_owned: 1,
            tap_owned: 1,
            ..Control::default()
        };
        let mut script = Script {
            state: &mut state,
            fault,
            cancellation: cancellation.clone(),
            trace: Vec::with_capacity(16),
            barrier,
        };
        if fault.cancel_at == 1 {
            cancellation.request();
        }
        let mut status = 0;
        let mut stage = 0;
        // SAFETY: header-compatible layouts and synchronous operation table;
        // no native operation is reachable through this injected table.
        let result = unsafe {
            pks_tap_control_start_observed(
                &mut state,
                &OPS,
                std::ptr::from_mut(&mut script).cast(),
                super::capture_open_cancelled,
                std::ptr::from_ref(cancellation).cast_mut().cast(),
                &mut status,
                &mut stage,
                super::observer_pointer(&observer),
            )
        };
        let before = script.trace.len();
        if state.registration_uncertain != 0 {
            let mut retry_status = 0;
            let mut retry_stage = 0;
            // SAFETY: same synchronous fixture; unknown registration must refuse retry.
            let retry = unsafe {
                pks_tap_control_start_observed(
                    &mut state,
                    &OPS,
                    std::ptr::from_mut(&mut script).cast(),
                    super::capture_open_cancelled,
                    std::ptr::from_ref(cancellation).cast_mut().cast(),
                    &mut retry_status,
                    &mut retry_stage,
                    super::observer_pointer(&observer),
                )
            };
            assert_eq!(retry, -1);
        }
        let repeated = script.trace.len() - before;
        let mut cleanup_status = 0;
        let mut cleanup_stage = 0;
        // SAFETY: same live state/context; cleanup does not free fixture storage.
        let cleanup_result = unsafe {
            pks_tap_control_cleanup_observed(
                &mut state,
                &OPS,
                std::ptr::from_mut(&mut script).cast(),
                &mut cleanup_status,
                &mut cleanup_stage,
                super::observer_pointer(&observer),
            )
        };
        let before_retry = script.trace.len();
        // SAFETY: same state; production requires repeated cleanup to be a no-op.
        let repeated_cleanup = unsafe {
            pks_tap_control_cleanup_observed(
                &mut state,
                &OPS,
                std::ptr::from_mut(&mut script).cast(),
                &mut cleanup_status,
                &mut cleanup_stage,
                super::observer_pointer(&observer),
            )
        };
        assert_eq!(cleanup_result, repeated_cleanup);
        let mut retry_status = 0;
        let mut retry_stage = 0;
        // SAFETY: same fixture; post-cleanup start is prohibited for this owner.
        let retry = unsafe {
            pks_tap_control_start_observed(
                &mut state,
                &OPS,
                std::ptr::from_mut(&mut script).cast(),
                super::capture_open_cancelled,
                std::ptr::from_ref(cancellation).cast_mut().cast(),
                &mut retry_status,
                &mut retry_stage,
                super::observer_pointer(&observer),
            )
        };
        assert_eq!(retry, -1);
        assert_eq!(script.trace.len(), before_retry);
        if cleanup_result == 0 {
            script.trace.push("release");
        }
        let cleanup_error = (cleanup_result != 0).then(|| tap_error(cleanup_status, cleanup_stage));
        let failure = match result {
            -2 => Some(CaptureOpenFailure::Cancelled { cleanup_error }),
            -1 => Some(CaptureOpenFailure::Backend {
                source: tap_error(status, stage),
                cleanup_error,
            }),
            _ => cleanup_error.map(|source| CaptureOpenFailure::Backend {
                source,
                cleanup_error: None,
            }),
        };
        Receipt {
            failure,
            trace: script.trace,
            start_result: result,
            primary_status: status,
            primary_stage: stage,
            cleanup_status,
            cleanup_stage,
            retained: state.retained != 0,
            repeat_before_cleanup_operations: repeated,
        }
    }
    fn snapshot_json(snapshot: crate::capture::NativeCallObservations) -> serde_json::Value {
        let spans = |values: &[Option<crate::capture::NativeCallSpan>]| {
            values.iter().flatten().map(|span| serde_json::json!({
                "open_ordinal":span.open_ordinal, "call_ordinal":span.call_ordinal,
                "operation":format!("{:?}",span.operation), "started_at_ns":span.started_at_ns,
                "returned_at_ns":span.returned_at_ns, "status_code":span.status_code,
            })).collect::<Vec<_>>()
        };
        serde_json::json!({"observed_at_ns":snapshot.observed_at_ns,
            "current":spans(&snapshot.current), "completed":spans(&snapshot.completed),
            "busy_slots":snapshot.busy_slots,"omitted_opens_total":snapshot.omitted_opens_total,
            "omitted_calls_total":snapshot.omitted_calls_total,"completed_calls_total":snapshot.completed_calls_total,
            "truncated_completed_total":snapshot.truncated_completed_total,"counters_inexact":snapshot.counters_inexact})
    }

    #[test]
    fn given_paused_native_controller_when_observed_then_registration_start_and_cleanup_are_visible_without_preemption(
    ) {
        use crate::capture::NativeCallOperation::*;
        let mut evidence = Vec::new();
        for (stage, expected) in [
            (6, RegisterIoProc),
            (7, StartDevice),
            (9, StopDevice),
            (10, UnregisterIoProc),
            (11, DestroyAggregateDevice),
            (12, DestroyProcessTap),
        ] {
            let token = crate::session::SessionStartCancellation::observed();
            let worker_token = token.clone();
            let (entered_tx, entered_rx) = mpsc::sync_channel(1);
            let (release_tx, release_rx) = mpsc::sync_channel(1);
            let fault = Fault {
                pause_stage: stage,
                cleanup_stage: if stage >= 9 { stage } else { 0 },
                cancel_at: if stage >= 9 { 3 } else { 0 },
                ..Fault::default()
            };
            let worker = std::thread::spawn(move || {
                run_fixture(
                    &worker_token.capture_open_cancellation(),
                    fault,
                    Some((entered_tx, release_rx)),
                )
            });
            entered_rx
                .recv_timeout(Duration::from_secs(5))
                .expect("injected operation entered");
            let snapshot = token.native_call_observations().unwrap();
            let current = snapshot
                .current
                .iter()
                .flatten()
                .copied()
                .collect::<Vec<_>>();
            assert_eq!(current.len(), 1);
            assert_eq!(current[0].operation, expected);
            assert!(current[0].started_at_ns > 0);
            assert_eq!(current[0].returned_at_ns, None);
            assert_eq!(current[0].status_code, None);
            token.request();
            assert_eq!(
                token.native_call_observations().unwrap().current,
                snapshot.current
            );
            release_tx.send(()).expect("release injected operation");
            let receipt = worker.join().expect("controller worker");
            let returned = token.native_call_observations().unwrap();
            assert!(returned.current.iter().all(Option::is_none));
            let completed = returned
                .completed
                .iter()
                .flatten()
                .find(|span| span.call_ordinal == current[0].call_ordinal)
                .unwrap();
            assert_eq!(completed.operation, expected);
            assert!(completed.returned_at_ns.unwrap() >= completed.started_at_ns);
            assert_eq!(
                completed.status_code,
                Some(if stage >= 9 {
                    -700 - i32::from(stage)
                } else {
                    0
                })
            );
            assert_eq!(receipt.retained, stage >= 9);
            assert!(!receipt.trace.contains(&"start") || stage != 6);
            evidence.push(serde_json::json!({"paused_stage":stage,"cancellation_requested":token.is_requested(),
                "in_flight":snapshot_json(snapshot),"returned":snapshot_json(returned),
                "trace":receipt.trace,"primary_status":receipt.primary_status,"primary_stage":receipt.primary_stage,
                "cleanup_status":receipt.cleanup_status,"cleanup_stage":receipt.cleanup_stage,"retained":receipt.retained}));
        }
        if let Ok(path) = std::env::var("PKS_NATIVE_CALL_RECEIPTS") {
            std::fs::write(path, serde_json::to_vec_pretty(&evidence).unwrap())
                .expect("test-only observation receipts");
        }
    }

    #[test]
    fn given_observed_controller_when_faults_match_default_then_operation_order_and_ownership_are_unchanged(
    ) {
        for fault in [
            Fault::default(),
            Fault {
                cancel_at: 2,
                ..Fault::default()
            },
            Fault {
                start_status: 0x10004003,
                cleanup_stage: 9,
                ..Fault::default()
            },
            Fault {
                register_status: 0x10004003,
                ..Fault::default()
            },
        ] {
            let ordinary = run_fixture(&CaptureOpenCancellation::default(), fault, None);
            let token = crate::session::SessionStartCancellation::observed();
            let observed = run_fixture(&token.capture_open_cancellation(), fault, None);
            assert_eq!(ordinary.trace, observed.trace);
            assert_eq!(
                (
                    ordinary.start_result,
                    ordinary.primary_status,
                    ordinary.primary_stage,
                    ordinary.cleanup_status,
                    ordinary.cleanup_stage,
                    ordinary.retained
                ),
                (
                    observed.start_result,
                    observed.primary_status,
                    observed.primary_stage,
                    observed.cleanup_status,
                    observed.cleanup_stage,
                    observed.retained
                )
            );
        }
    }

    #[test]
    fn given_native_faults_when_late_cancellation_cleanup_runs_then_ownership_is_retained_exactly()
    {
        let cases = [
            (
                "cancel_before_register",
                Fault {
                    cancel_at: 1,
                    ..Fault::default()
                },
                vec!["aggregate", "tap", "release"],
            ),
            (
                "cancel_after_register",
                Fault {
                    cancel_at: 2,
                    ..Fault::default()
                },
                vec!["register", "unregister", "aggregate", "tap", "release"],
            ),
            (
                "cancel_after_start",
                Fault {
                    cancel_at: 3,
                    ..Fault::default()
                },
                vec![
                    "register",
                    "start",
                    "stop",
                    "unregister",
                    "aggregate",
                    "tap",
                    "release",
                ],
            ),
            (
                "start_rejected",
                Fault {
                    start_status: 0x10004003,
                    ..Fault::default()
                },
                vec![
                    "register",
                    "start",
                    "stop",
                    "unregister",
                    "aggregate",
                    "tap",
                    "release",
                ],
            ),
            (
                "register_rejected_known",
                Fault {
                    register_status: -50,
                    known_registration_rejection: true,
                    ..Fault::default()
                },
                vec!["register", "aggregate", "tap", "release"],
            ),
            (
                "register_timeout_unknown",
                Fault {
                    register_status: 0x10004003,
                    ..Fault::default()
                },
                vec!["register"],
            ),
            (
                "register_error_with_id",
                Fault {
                    register_status: -50,
                    registered_on_error: true,
                    ..Fault::default()
                },
                vec!["register", "unregister", "aggregate", "tap", "release"],
            ),
            (
                "stop_uncertain",
                Fault {
                    cancel_at: 3,
                    cleanup_stage: 9,
                    ..Fault::default()
                },
                vec!["register", "start", "stop"],
            ),
            (
                "unregister_uncertain",
                Fault {
                    cancel_at: 2,
                    cleanup_stage: 10,
                    ..Fault::default()
                },
                vec!["register", "unregister"],
            ),
            (
                "aggregate_uncertain",
                Fault {
                    cancel_at: 2,
                    cleanup_stage: 11,
                    ..Fault::default()
                },
                vec!["register", "unregister", "aggregate"],
            ),
            (
                "tap_uncertain",
                Fault {
                    cancel_at: 2,
                    cleanup_stage: 12,
                    ..Fault::default()
                },
                vec!["register", "unregister", "aggregate", "tap"],
            ),
            (
                "completed",
                Fault::default(),
                vec![
                    "register",
                    "start",
                    "stop",
                    "unregister",
                    "aggregate",
                    "tap",
                    "release",
                ],
            ),
        ];
        let mut receipts = Vec::new();
        for (case, fault, expected) in cases {
            let receipt = run_fixture(&CaptureOpenCancellation::default(), fault, None);
            assert_eq!(receipt.trace, expected, "{case}");
            assert_eq!(receipt.repeat_before_cleanup_operations, 0, "{case}");
            assert_eq!(receipt.retained, receipt.cleanup_stage != 0, "{case}");
            if case == "register_timeout_unknown" {
                assert_eq!(receipt.cleanup_status, 0x10004003);
                assert_eq!(receipt.cleanup_stage, 13);
            }
            receipts.push(serde_json::json!({"case":case,"trace":receipt.trace,
                "start_result":receipt.start_result,"primary_status":receipt.primary_status,
                "primary_stage":receipt.primary_stage,"cleanup_status":receipt.cleanup_status,
                "cleanup_stage":receipt.cleanup_stage,"context_retained":receipt.retained,
                "repeat_before_cleanup_operations":receipt.repeat_before_cleanup_operations,
                "repeat_after_cleanup_operations":0,"classification":"MOCKED"}));
        }
        if let Some(path) = std::env::var_os("PKS_TEST_TAP_CONTROL_RECEIPTS") {
            std::fs::write(
                path,
                serde_json::to_vec_pretty(&receipts).expect("receipt JSON"),
            )
            .expect("write private receipt");
        }
    }
    #[test]
    fn given_native_lease_budget_when_cleanup_is_uncertain_then_late_cancellation_admission_is_quarantined(
    ) {
        let admission = TapAdmission::new();
        let mut leases = (0..MAX_PROCESS_TAP_OWNERS)
            .map(|_| admission.acquire().expect("available lease"))
            .collect::<Vec<_>>();
        assert!(admission.acquire().is_err());
        drop(leases.pop());
        let mut lease = admission
            .acquire()
            .expect("confirmed cleanup releases capacity");
        lease.quarantine();
        drop(lease);
        drop(leases);
        assert_eq!(admission.owners.load(Ordering::Acquire), 1);
        assert!(admission.acquire().is_err());
    }
    #[test]
    fn given_concurrent_native_opens_when_late_cancellation_budget_is_full_then_admission_stays_finite(
    ) {
        let admission = Arc::new(TapAdmission::new());
        let barrier = Arc::new(Barrier::new(81));
        std::thread::scope(|scope| {
            let mut workers = Vec::new();
            for _ in 0..80 {
                let admission = Arc::clone(&admission);
                let barrier = Arc::clone(&barrier);
                workers.push(scope.spawn(move || {
                    let lease = admission.acquire();
                    barrier.wait();
                    barrier.wait();
                    lease.is_ok()
                }));
            }
            barrier.wait();
            assert_eq!(
                admission.owners.load(Ordering::Acquire),
                MAX_PROCESS_TAP_OWNERS
            );
            barrier.wait();
            let admitted = workers
                .into_iter()
                .map(|worker| worker.join().expect("worker"))
                .filter(|admitted| *admitted)
                .count();
            assert_eq!(admitted, MAX_PROCESS_TAP_OWNERS as usize);
        });
        assert_eq!(admission.owners.load(Ordering::Acquire), 0);
    }

    #[test]
    fn given_pre_admitted_owners_when_both_cleanup_results_are_uncertain_then_late_cancellation_retains_each_lease(
    ) {
        let admission = TapAdmission::new();
        let mut first = admission.acquire().expect("first admitted owner");
        let mut second = admission.acquire().expect("second admitted owner");
        first.quarantine();
        assert!(admission.acquire().is_err());
        second.quarantine();
        drop(first);
        drop(second);
        assert_eq!(admission.owners.load(Ordering::Acquire), 2);
        assert!(admission.acquire().is_err());
    }
}

#[cfg(test)]
mod tests {
    use super::{
        application_open_audit_for_process, exact_application_open_audit,
        process_tap_io_duration_ms, process_timestamp_ns, select_application_capture,
        select_stable_application_capture, selected_application_is_running_with,
        source_host_timestamp_ns, stable_source_id, tap_error, AuditedCaptureSource,
        ExactApplicationOpenAudit, ProcessTapReadBatch, CORE_AUDIO_PERMISSION_DENIED_STATUS,
    };
    use crate::capture::{
        CaptureError, CaptureMode, CaptureSource, SourceKind, SourceState, StableSourceId,
    };
    use crate::frame::Platform;
    use crate::timing::TimelineMapping;

    fn audited_application(
        stable_id: StableSourceId,
        process_id: u32,
        process_start_time_ns: u64,
    ) -> AuditedCaptureSource {
        let app_id = stable_id.stable_key.clone();
        AuditedCaptureSource {
            source: CaptureSource {
                stable_id,
                name: "Application".to_owned(),
                process_id: Some(process_id),
                app_id: Some(app_id),
                device_uid: None,
                state: SourceState::Playing,
                sample_rate_hz: 48_000,
                channels: 2,
            },
            process_start_time_ns,
        }
    }

    fn application_source(name: &str, app_id: &str, process_id: Option<u32>) -> CaptureSource {
        CaptureSource {
            stable_id: StableSourceId::new(Platform::Macos, SourceKind::Application, app_id),
            name: name.to_owned(),
            process_id,
            app_id: Some(app_id.to_owned()),
            device_uid: None,
            state: SourceState::Playing,
            sample_rate_hz: 48_000,
            channels: 2,
        }
    }

    #[test]
    fn given_native_input_uid_when_discovered_then_identity_matches_canonical_opener() {
        let mut raw = super::RawSourceInfo {
            audio_object_id: 7,
            process_id: 0,
            bundle_id: [0; 256],
            name: [0; 256],
            source_kind_code: 1,
            source_state_code: 0,
            sample_rate_hz: 48_000,
            channel_count: 1,
            process_start_time_ns: 0,
        };
        let uid = b"BuiltInMicrophoneDevice";
        raw.bundle_id[..uid.len()].copy_from_slice(uid);
        let sources = super::decode_discovered_sources(&[raw]);
        assert_eq!(sources.len(), 1);
        let source = &sources[0].source;
        let canonical = super::super::input::canonical_input_device_id("BuiltInMicrophoneDevice");
        assert_eq!(source.stable_id.stable_key, canonical);
        assert_eq!(source.device_uid.as_deref(), Some(canonical.as_str()));
        assert_eq!(
            source.stable_id.source_id(),
            StableSourceId::new(Platform::Macos, SourceKind::InputDevice, canonical).source_id()
        );
        assert_eq!(source.state, SourceState::Available);
        assert_eq!(source.channels, 1);
    }

    #[test]
    fn given_core_audio_permission_status_when_mapped_then_denial_remains_typed() {
        assert_eq!(
            tap_error(CORE_AUDIO_PERMISSION_DENIED_STATUS, 2),
            CaptureError::PermissionDenied {
                operation: "creating the CoreAudio process tap"
            }
        );
    }

    #[test]
    fn given_supported_product_cadence_when_opening_tap_then_native_io_is_at_most_ten_ms() {
        assert_eq!(
            process_tap_io_duration_ms(crate::frame::AudioFrameDuration::Ms10),
            10
        );
        assert_eq!(
            process_tap_io_duration_ms(crate::frame::AudioFrameDuration::Ms20),
            10
        );
    }

    #[test]
    fn given_reader_position_before_native_anchor_when_mapped_then_sample_delta_is_preserved() {
        let batch = ProcessTapReadBatch {
            frame_count: 480,
            source_frame_position_frames: 48_000,
            anchor_frame_position_frames: 48_480,
            anchor_host_time_ns: 2_000_000_000,
        };

        assert_eq!(source_host_timestamp_ns(batch, 48_000), Some(1_990_000_000));
    }

    #[test]
    fn given_native_host_time_when_normalized_then_process_clock_boundary_is_comparable() {
        let batch = ProcessTapReadBatch {
            frame_count: 960,
            source_frame_position_frames: 96_000,
            anchor_frame_position_frames: 96_000,
            anchor_host_time_ns: 9_000_000_000,
        };
        let mapping = TimelineMapping::new(8_500_000_000, 500_000_000);

        assert_eq!(
            process_timestamp_ns(batch, 48_000, mapping),
            Some(1_000_000_000)
        );
    }

    #[test]
    fn given_other_core_audio_status_when_mapped_then_raw_status_is_preserved() {
        assert_eq!(
            tap_error(-50, 7),
            CaptureError::BackendStatus {
                operation: "starting the CoreAudio aggregate device",
                status_code: -50
            }
        );
    }

    #[test]
    fn given_exact_application_target_when_framed_then_stable_identity_is_preserved() {
        let stable_id =
            StableSourceId::new(Platform::Macos, SourceKind::Application, "com.acme.meeting");
        let expected = stable_id.source_id();

        let observed = stable_source_id(&CaptureMode::ExactApplication {
            process_id: 42,
            stable_id,
        })
        .unwrap()
        .source_id();

        assert_eq!(observed, expected);
    }

    #[test]
    fn given_process_id_when_resolved_then_discovered_application_identity_is_preserved() {
        let stable_id =
            StableSourceId::new(Platform::Macos, SourceKind::Application, "com.acme.meeting");
        let sources = vec![audited_application(stable_id.clone(), 42, 73)];

        let audit = application_open_audit_for_process(&sources, 42).unwrap();

        assert_eq!(audit.process_id, 42);
        assert_eq!(audit.stable_id, stable_id);
        assert_eq!(audit.process_start_time_ns, 73);
    }

    #[test]
    fn given_display_name_when_application_selected_then_all_matching_processes_are_captured() {
        let sources = vec![
            application_source("Brave Browser", "com.brave.Browser", Some(42)),
            application_source("Brave Browser", "com.brave.Browser", Some(43)),
        ];

        let selected = select_application_capture(&sources, "Brave Browser").unwrap();

        assert_eq!(selected.process_ids, vec![42, 43]);
        assert_eq!(selected.stable_id.stable_key, "com.brave.Browser");
    }

    #[test]
    fn given_bundle_id_when_application_selected_then_discovered_identity_is_preserved() {
        let sources = vec![application_source(
            "Brave Browser",
            "com.brave.Browser",
            Some(42),
        )];

        let selected = select_application_capture(&sources, "com.brave.Browser").unwrap();

        assert_eq!(selected.process_ids, vec![42]);
        assert_eq!(selected.stable_id.stable_key, "com.brave.Browser");
    }

    #[test]
    fn given_stable_application_identity_when_selected_then_all_live_processes_are_captured() {
        let stable_id = StableSourceId::new(
            Platform::Macos,
            SourceKind::Application,
            "com.brave.Browser",
        );
        let sources = vec![
            application_source("Brave Browser", "com.brave.Browser", Some(42)),
            application_source("Brave Browser", "com.brave.Browser", Some(43)),
        ];

        let selected = select_stable_application_capture(&sources, &stable_id).unwrap();

        assert_eq!(selected.process_ids, vec![42, 43]);
        assert_eq!(selected.stable_id, stable_id);
    }

    #[test]
    fn given_foreign_platform_identity_when_selected_then_capture_fails_closed() {
        let stable_id = StableSourceId::new(
            Platform::Windows,
            SourceKind::Application,
            "com.brave.Browser",
        );
        let sources = vec![application_source(
            "Brave Browser",
            "com.brave.Browser",
            Some(42),
        )];

        assert!(matches!(
            select_stable_application_capture(&sources, &stable_id),
            Err(CaptureError::SourceUnavailable { stable_key })
                if stable_key == "com.brave.Browser"
        ));
    }

    #[test]
    fn given_ambiguous_display_name_when_application_selected_then_capture_fails_closed() {
        let sources = vec![
            application_source("Meeting", "com.acme.meeting", Some(42)),
            application_source("Meeting", "com.other.meeting", Some(43)),
        ];

        let error = select_application_capture(&sources, "Meeting").unwrap_err();

        assert!(error.to_string().contains("multiple running audio sources"));
    }

    #[test]
    fn given_application_without_process_identity_when_selected_then_capture_fails_closed() {
        let sources = vec![application_source("Meeting", "com.acme.meeting", None)];

        let error = select_application_capture(&sources, "Meeting").unwrap_err();

        assert!(error.to_string().contains("has no process identity"));
    }

    #[test]
    fn given_reused_pid_with_different_application_when_verified_then_target_is_rejected() {
        let selected =
            StableSourceId::new(Platform::Macos, SourceKind::Application, "com.acme.meeting");
        let replacement =
            StableSourceId::new(Platform::Macos, SourceKind::Application, "com.other.player");
        let sources = vec![audited_application(replacement, 42, 200)];

        assert_eq!(exact_application_open_audit(&sources, 42, &selected), None);
    }

    #[test]
    fn given_same_pid_and_application_when_verified_then_target_is_retained() {
        let selected =
            StableSourceId::new(Platform::Macos, SourceKind::Application, "com.acme.meeting");
        let sources = vec![audited_application(selected.clone(), 42, 100)];

        assert_eq!(
            exact_application_open_audit(&sources, 42, &selected),
            Some(ExactApplicationOpenAudit {
                process_id: 42,
                stable_id: selected,
                process_start_time_ns: 100,
            })
        );
    }

    #[test]
    fn given_same_pid_and_application_with_new_creation_when_audited_then_reuse_is_detected() {
        let selected =
            StableSourceId::new(Platform::Macos, SourceKind::Application, "com.acme.meeting");
        let before = ExactApplicationOpenAudit {
            process_id: 42,
            stable_id: selected.clone(),
            process_start_time_ns: 100,
        };
        let replacement = vec![audited_application(selected.clone(), 42, 200)];

        assert_ne!(
            exact_application_open_audit(&replacement, 42, &selected),
            Some(before)
        );
    }

    #[test]
    fn given_missing_creation_time_when_audited_then_exact_open_fails_closed() {
        let selected =
            StableSourceId::new(Platform::Macos, SourceKind::Application, "com.acme.meeting");
        let sources = vec![audited_application(selected.clone(), 42, 0)];

        assert_eq!(exact_application_open_audit(&sources, 42, &selected), None);
    }

    #[test]
    fn given_one_selected_process_still_running_when_checked_then_application_remains_available() {
        let stable_id =
            StableSourceId::new(Platform::Macos, SourceKind::Application, "com.acme.meeting");
        let audits = vec![
            ExactApplicationOpenAudit {
                process_id: 42,
                stable_id: stable_id.clone(),
                process_start_time_ns: 100,
            },
            ExactApplicationOpenAudit {
                process_id: 43,
                stable_id,
                process_start_time_ns: 200,
            },
        ];

        assert!(selected_application_is_running_with(
            &audits,
            |process_id| {
                if process_id == 43 {
                    200
                } else {
                    0
                }
            }
        ));
    }

    #[test]
    fn given_selected_pid_reused_after_exit_when_checked_then_application_is_unavailable() {
        let audits = vec![ExactApplicationOpenAudit {
            process_id: 42,
            stable_id: StableSourceId::new(
                Platform::Macos,
                SourceKind::Application,
                "com.acme.meeting",
            ),
            process_start_time_ns: 100,
        }];

        assert!(!selected_application_is_running_with(&audits, |_| 200));
    }
}
