//! CoreAudio process tap backend — macOS 14.4+ (public support claim).
//!
//! Uses `AudioHardwareCreateProcessTap` + `CATapDescription` to capture audio
//! from specific processes or the global system output mix without routing
//! changes, HAL plugin installation, or Screen Recording permission.

use std::ptr::NonNull;
use std::time::Duration;

use crate::frame::{AudioBufferPool, AudioFrame, Platform, StreamId};

use crate::capture::{
    initialize_monotonic_timestamp_domain, monotonic_timestamp_ns, CaptureError as LoopbackError,
    CaptureMode, CaptureObservationCounters, CaptureObservationHandle, CaptureObservations,
    CaptureSource, SourceKind, SourceState, StableSourceId,
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

extern "C" {
    fn pks_process_tap_available() -> i32;
    fn pks_discover_sources(out: *mut RawSourceInfo, max_count: i32) -> i32;
    fn pks_create_process_tap(
        pids: *const i32,
        process_count: i32,
        out_status: *mut i32,
        out_stage: *mut u8,
    ) -> *mut std::ffi::c_void;
    fn pks_tap_start(tap: *mut std::ffi::c_void, out_status: *mut i32, out_stage: *mut u8) -> i32;
    fn pks_destroy_process_tap(tap: *mut std::ffi::c_void);
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
    const MAX: usize = 128;
    // SAFETY: write_bytes zeroes the allocation before set_len, so all MAX
    // elements are initialised.  pks_discover_sources then writes exactly `n`
    // valid entries into the first `n` slots; we truncate to that count.
    let raw: Vec<RawSourceInfo> = unsafe {
        let mut v: Vec<RawSourceInfo> = Vec::with_capacity(MAX);
        std::ptr::write_bytes(v.as_mut_ptr(), 0, MAX);
        v.set_len(MAX);
        let n = pks_discover_sources(v.as_mut_ptr(), MAX as i32);
        v.truncate(n.max(0) as usize);
        v
    };

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
            let native_identity = cstr_to_opt(&r.bundle_id);
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

struct ProcessTap(NonNull<std::ffi::c_void>);

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

fn capture_exact_application_open_audit(
    process_id: u32,
    stable_id: &StableSourceId,
) -> Result<ExactApplicationOpenAudit, LoopbackError> {
    exact_application_open_audit(&discover_sources_native_with_audit(), process_id, stable_id)
        .ok_or_else(|| LoopbackError::SourceUnavailable {
            stable_key: stable_id.stable_key.clone(),
        })
}

fn verify_exact_application_open_audit(
    expected: &ExactApplicationOpenAudit,
) -> Result<(), LoopbackError> {
    let observed = exact_application_open_audit(
        &discover_sources_native_with_audit(),
        expected.process_id,
        &expected.stable_id,
    );
    if observed.as_ref() == Some(expected) {
        Ok(())
    } else {
        Err(LoopbackError::SourceUnavailable {
            stable_key: expected.stable_id.stable_key.clone(),
        })
    }
}

// SAFETY:
// - PksProcessTapHandle is heap-allocated and exclusively owned by this struct.
// - pks_tap_read_frames is called only from the single thread that owns ProcessTap.
// - The IO callback writes to the ring from CoreAudio's RT thread; synchronisation
//   is through the atomic write_head.
// - AudioDeviceStart/Stop are safe to call from any thread.
unsafe impl Send for ProcessTap {}

impl ProcessTap {
    fn global() -> Result<Self, LoopbackError> {
        Self::create(std::ptr::null(), 0)
    }

    fn for_pids(pids: &[i32]) -> Result<Self, LoopbackError> {
        Self::create(pids.as_ptr(), pids.len() as i32)
    }

    fn create(pids: *const i32, process_count: i32) -> Result<Self, LoopbackError> {
        let mut status_code = 0;
        let mut stage_code = 0;
        // SAFETY: pids addresses process_count valid i32 values or is null when
        // process_count is zero; both out-pointers live through this call.
        let handle = unsafe {
            pks_create_process_tap(pids, process_count, &mut status_code, &mut stage_code)
        };
        NonNull::new(handle)
            .map(Self)
            .ok_or_else(|| tap_error(status_code, stage_code))
    }

    fn start(&mut self) -> Result<(), LoopbackError> {
        let mut status_code = 0;
        let mut stage_code = 0;
        // SAFETY: self owns a live tap handle and both out-pointers live through
        // this call.
        if unsafe { pks_tap_start(self.0.as_ptr(), &mut status_code, &mut stage_code) } == 0 {
            Ok(())
        } else {
            Err(tap_error(status_code, stage_code))
        }
    }

    fn sample_rate_hz(&self) -> u32 {
        // SAFETY: self owns a live tap handle for the duration of the call.
        unsafe { pks_tap_sample_rate(self.0.as_ptr()) }
    }

    fn channel_count(&self) -> u32 {
        // SAFETY: self owns a live tap handle for the duration of the call.
        unsafe { pks_tap_channels(self.0.as_ptr()) }
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
                self.0.as_ptr(),
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
        unsafe { pks_tap_drop_count(self.0.as_ptr()) }
    }
}

/// Drop contract — this is a control-thread-only owner:
///   destroy exactly once · panic-free · no Rust allocation · no Rust logging
impl Drop for ProcessTap {
    fn drop(&mut self) {
        // SAFETY: self exclusively owns the tap handle and destroys it once.
        unsafe {
            pks_destroy_process_tap(self.0.as_ptr());
        }
    }
}

// Process-tap callbacks can arrive as 10 ms buffers while the public pipeline
// consumes 20 ms frames. Keep bounded ownership for a full downstream burst;
// empty pool slots add memory headroom, not playout latency.
const POOL_CAPACITY_FRAMES: usize = 32;

/// Captures system audio via CoreAudio process tap (macOS 14.2+).
pub struct TapLoopbackSource {
    reader_thread: Option<std::thread::JoinHandle<()>>,
    pub(crate) stop_tx: std::sync::mpsc::SyncSender<()>,
    counters: CaptureObservationCounters,
    source_id: crate::frame::SourceId,
}

impl TapLoopbackSource {
    pub(crate) fn capture_mode_with_runtime_event_sender<F>(
        mode: CaptureMode,
        mut callback: F,
        runtime_event_sender: Option<crate::capture::SourceRuntimeEventSender>,
    ) -> Result<Self, LoopbackError>
    where
        F: FnMut(AudioFrame) + Send + 'static,
    {
        if !tap_available() {
            return Err(LoopbackError::BackendInit(
                "CoreAudio process tap requires macOS 14.4 or later".into(),
            ));
        }

        let exact_application_open_audit = if let CaptureMode::ExactApplication {
            process_id,
            stable_id,
        } = &mode
        {
            Some(capture_exact_application_open_audit(
                *process_id,
                stable_id,
            )?)
        } else {
            None
        };

        let (mut tap, stable_id) = match &mode {
            CaptureMode::SystemMix => (ProcessTap::global()?, stable_source_id(&mode)?),
            CaptureMode::Process(pid) => (
                ProcessTap::for_pids(&[*pid as i32])?,
                stable_source_id(&mode)?,
            ),
            CaptureMode::ExactApplication { process_id, .. } => (
                ProcessTap::for_pids(&[*process_id as i32])?,
                stable_source_id(&mode)?,
            ),
            CaptureMode::ExactApplicationStable { stable_id } => {
                let sources = discover_sources_native();
                let selected = select_stable_application_capture(&sources, stable_id)?;
                (
                    ProcessTap::for_pids(&selected.process_ids)?,
                    selected.stable_id,
                )
            }
            CaptureMode::Application(application) => {
                let sources = discover_sources_native();
                let selected = select_application_capture(&sources, application)?;
                if std::env::var_os("PKS_TAP_DIAG").is_some() {
                    eprintln!(
                        "tap_diag: application={} sources={} process_ids={:?}",
                        application,
                        sources.len(),
                        selected.process_ids
                    );
                }
                (
                    ProcessTap::for_pids(&selected.process_ids)?,
                    selected.stable_id,
                )
            }
            CaptureMode::InputDevice(_) => {
                return Err(LoopbackError::ModeUnsupported(mode));
            }
        };

        tap.start()?;
        if let Some(expected) = exact_application_open_audit.as_ref() {
            verify_exact_application_open_audit(expected)?;
        }

        let sample_rate_hz = tap.sample_rate_hz();
        if sample_rate_hz == 0 {
            return Err(LoopbackError::BackendInit(
                "tap reported a zero sample rate".to_owned(),
            ));
        }
        let channel_count = tap.channel_count() as u8;
        initialize_monotonic_timestamp_domain();
        let host_time_before_ns = ProcessTap::current_host_time_ns();
        let process_time_ns = monotonic_timestamp_ns();
        let host_time_after_ns = ProcessTap::current_host_time_ns();
        if host_time_before_ns == 0 || host_time_after_ns < host_time_before_ns {
            return Err(LoopbackError::BackendInit(
                "CoreAudio host-time mapping is unavailable".to_owned(),
            ));
        }
        let host_time_midpoint_ns =
            host_time_before_ns.saturating_add((host_time_after_ns - host_time_before_ns) / 2);
        let host_to_process = TimelineMapping::new(host_time_midpoint_ns, process_time_ns);
        let callback_frame_count: u32 = sample_rate_hz / 50; // 20 ms
        let buffer_capacity_samples = callback_frame_count as usize * channel_count as usize;
        let pool = AudioBufferPool::new(POOL_CAPACITY_FRAMES, buffer_capacity_samples);
        let (stop_tx, stop_rx) = std::sync::mpsc::sync_channel::<()>(1);
        let counters = CaptureObservationCounters::default();
        let capture_counters = counters.clone();

        let source_id = stable_id.source_id();
        let failure_counters = counters.clone();

        let thread = std::thread::Builder::new()
            .name("pks-tap-reader".into())
            .spawn(move || {
                let worker = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    let mut sequence_num: u64 = 0;
                    let mut buffer = vec![0.0f32; buffer_capacity_samples];
                    let mut observed_drop_count = tap.drop_count();
                    loop {
                        if stop_rx.try_recv().is_ok() {
                            break;
                        }
                        let batch = tap.read_frames(&mut buffer, callback_frame_count);
                        let frame_count = batch.frame_count;
                        let drop_count = tap.drop_count();
                        capture_counters.observe_dispatch_queue_full_frames(
                            drop_count.saturating_sub(observed_drop_count),
                        );
                        observed_drop_count = drop_count;
                        if frame_count == 0 {
                            std::thread::sleep(Duration::from_millis(1));
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
                        let frame_sequence_number = sequence_num;
                        sequence_num = sequence_num.saturating_add(1);
                        let mut handle = match pool.acquire() {
                            Some(h) => h,
                            None => {
                                capture_counters.observe_pool_exhaustion();
                                continue;
                            }
                        };
                        let dst = handle.as_mut_slice();
                        let sample_count = frame_count as usize * channel_count as usize;
                        if sample_count > dst.len() {
                            capture_counters.observe_oversized_buffer();
                            continue;
                        }
                        dst[..sample_count].copy_from_slice(&buffer[..sample_count]);
                        if handle.try_set_len(sample_count).is_err() {
                            capture_counters.observe_oversized_buffer();
                            continue;
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
                    }
                }));
                if let Err(payload) = worker {
                    failure_counters.observe_stream_error();
                    if let Some(sender) = runtime_event_sender.as_ref() {
                        let _ = crate::capture::publish_backend_failure(
                            sender,
                            stable_id,
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
            .map_err(|e| LoopbackError::BackendInit(format!("thread spawn: {e}")))?;

        Ok(Self {
            reader_thread: Some(thread),
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
        self.reader_thread.take().map_or(Ok(()), |thread| {
            crate::capture::join_capture_worker(thread, "macOS tap reader")
        })
    }
}

/// Drop contract — control thread only: signal and join the owned reader.
impl Drop for TapLoopbackSource {
    fn drop(&mut self) {
        let _ = self.stop_reader();
    }
}

#[cfg(test)]
mod tests {
    use super::{
        exact_application_open_audit, process_timestamp_ns, select_application_capture,
        select_stable_application_capture, source_host_timestamp_ns, stable_source_id, tap_error,
        AuditedCaptureSource, ExactApplicationOpenAudit, ProcessTapReadBatch,
        CORE_AUDIO_PERMISSION_DENIED_STATUS,
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
    fn given_core_audio_permission_status_when_mapped_then_denial_remains_typed() {
        assert_eq!(
            tap_error(CORE_AUDIO_PERMISSION_DENIED_STATUS, 2),
            CaptureError::PermissionDenied {
                operation: "creating the CoreAudio process tap"
            }
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
}
