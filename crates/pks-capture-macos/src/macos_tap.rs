//! CoreAudio process tap backend — macOS 14.4+ (public support claim).
//!
//! Uses `AudioHardwareCreateProcessTap` + `CATapDescription` to capture audio
//! from specific processes or the global system output mix without routing
//! changes, HAL plugin installation, or Screen Recording permission.

use std::ptr::NonNull;
use std::time::Duration;

use pks_frame::{AudioBufferPool, AudioFrame, AudioSourceTag, EncryptionMode, Platform, StreamId};

use pks_capture::{
    CaptureError as LoopbackError, CaptureMode, CaptureObservationCounters,
    CaptureObservationHandle, CaptureObservations, CaptureSampleTimeline, CaptureSource,
    SourceKind, SourceState, StableSourceId,
};

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
    fn pks_tap_read_frames(tap: *mut std::ffi::c_void, out: *mut f32, frame_count: u32) -> u32;
    fn pks_tap_drop_count(tap: *const std::ffi::c_void) -> u64;
    fn pks_tap_sample_rate(tap: *const std::ffi::c_void) -> u32;
    fn pks_tap_channels(tap: *const std::ffi::c_void) -> u32;
    // Reserved for future audio-level diagnostic (capture-path signal-level probe).
    #[allow(dead_code)]
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
    unsafe { pks_process_tap_available() != 0 }
}

/// Enumerate all running processes that have audio output.
/// Returns an empty `Vec` on macOS < 14.4 (public support floor) or on non-macOS platforms.
pub fn discover_sources_native() -> Vec<CaptureSource> {
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
            let app_id = cstr_to_opt(&r.bundle_id);
            let source_kind = match r.source_kind_code {
                1 => SourceKind::InputDevice,
                2 => SourceKind::OutputDevice,
                3 => SourceKind::SystemMix,
                _ => SourceKind::Application,
            };
            let stable_key = app_id
                .as_deref()
                .map(|id| id.to_owned())
                .unwrap_or_else(|| format!("pid:{}", r.process_id));
            CaptureSource {
                stable_id: StableSourceId::new(Platform::Macos, source_kind, stable_key),
                name: cstr_to_string(&r.name).unwrap_or_else(|| format!("pid:{}", r.process_id)),
                process_id,
                app_id,
                device_uid: None,
                state: match r.source_state_code {
                    1 => SourceState::Playing,
                    2 => SourceState::Silent,
                    3 => SourceState::Unavailable,
                    _ => SourceState::Available,
                },
                sample_rate_hz: r.sample_rate_hz,
                channels: r.channel_count,
            }
        })
        .collect()
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

fn frame_source_id(mode: &CaptureMode) -> Result<pks_frame::SourceId, LoopbackError> {
    stable_source_id(mode).map(|stable_id| stable_id.to_frame_source_id())
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

    fn read_frames(&mut self, out: &mut [f32], frame_count: u32) -> u32 {
        let required_samples = frame_count as usize * self.channel_count() as usize;
        if out.len() < required_samples {
            return 0;
        }
        // SAFETY: self owns the live tap, and out contains at least
        // frame_count * channel_count writable f32 samples.
        unsafe { pks_tap_read_frames(self.0.as_ptr(), out.as_mut_ptr(), frame_count) }
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
}

impl TapLoopbackSource {
    pub fn capture_mode<F>(mode: CaptureMode, callback: F) -> Result<Self, LoopbackError>
    where
        F: FnMut(AudioFrame) + Send + 'static,
    {
        Self::capture_mode_with_runtime_event_sender(mode, callback, None)
    }

    pub(crate) fn capture_mode_with_runtime_event_sender<F>(
        mode: CaptureMode,
        mut callback: F,
        runtime_event_sender: Option<pks_capture::SourceRuntimeEventSender>,
    ) -> Result<Self, LoopbackError>
    where
        F: FnMut(AudioFrame) + Send + 'static,
    {
        if !tap_available() {
            return Err(LoopbackError::BackendInit(
                "CoreAudio process tap requires macOS 14.4 or later".into(),
            ));
        }

        let mut tap = match &mode {
            CaptureMode::SystemMix => ProcessTap::global()?,
            CaptureMode::Process(pid) => ProcessTap::for_pids(&[*pid as i32])?,
            CaptureMode::ExactApplication { process_id, .. } => {
                ProcessTap::for_pids(&[*process_id as i32])?
            }
            CaptureMode::ExactApplicationStable { .. } => {
                return Err(LoopbackError::ModeUnsupported(mode.clone()));
            }
            CaptureMode::Application(bundle_id) => {
                let sources = discover_sources_native();
                let pids: Vec<i32> = sources
                    .iter()
                    .filter(|s| s.app_id.as_deref() == Some(bundle_id.as_str()))
                    .filter_map(|s| s.process_id.map(|p| p as i32))
                    .collect();
                if std::env::var_os("PKS_TAP_DIAG").is_some() {
                    eprintln!(
                        "tap_diag: app_source_lookup bundle_id={} sources={} pids={:?}",
                        bundle_id,
                        sources.len(),
                        pids
                    );
                }
                if pids.is_empty() {
                    return Err(LoopbackError::BackendInit(format!(
                        "no running audio process found for bundle ID: {bundle_id}"
                    )));
                }
                ProcessTap::for_pids(&pids)?
            }
            CaptureMode::InputDevice(_) => {
                return Err(LoopbackError::ModeUnsupported(mode));
            }
        };

        tap.start()?;

        let sample_rate_hz = tap.sample_rate_hz();
        let sample_rate_nonzero = std::num::NonZeroU32::new(sample_rate_hz).ok_or_else(|| {
            LoopbackError::BackendInit("tap reported a zero sample rate".to_owned())
        })?;
        let channel_count = tap.channel_count() as u8;
        let callback_frame_count: u32 = sample_rate_hz / 50; // 20 ms
        let buffer_capacity_samples = callback_frame_count as usize * channel_count as usize;
        let pool = std::sync::Arc::new(AudioBufferPool::new(
            POOL_CAPACITY_FRAMES,
            buffer_capacity_samples,
        ));
        let (stop_tx, stop_rx) = std::sync::mpsc::sync_channel::<()>(1);
        let counters = CaptureObservationCounters::default();
        let capture_counters = counters.clone();

        // Compute the stable SourceId from the capture mode before the thread
        // is spawned so that every AudioFrame carries the correct source identity.
        let frame_source_id = frame_source_id(&mode)?;
        let stable_id = stable_source_id(&mode)?;
        let failure_counters = counters.clone();

        let thread = std::thread::Builder::new()
            .name("pks-tap-reader".into())
            .spawn(move || {
                let worker = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    let mut sequence_num: u64 = 0;
                    let mut timeline = CaptureSampleTimeline::new(sample_rate_nonzero);
                    let mut buffer = vec![0.0f32; buffer_capacity_samples];
                    let mut observed_drop_count = tap.drop_count();
                    loop {
                        if stop_rx.try_recv().is_ok() {
                            break;
                        }
                        let frame_count = tap.read_frames(&mut buffer, callback_frame_count);
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
                        let timestamp_ns = timeline.advance(u64::from(frame_count));
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
                        handle.set_len(sample_count);
                        let mut frame = AudioFrame::new(
                            StreamId(0),
                            frame_source_id,
                            frame_sequence_number,
                            timestamp_ns,
                            channel_count,
                            handle,
                        );
                        frame.source_tag = AudioSourceTag::Captured;
                        frame.encryption_mode = EncryptionMode::None;
                        frame.sample_rate_hz = sample_rate_hz;
                        capture_counters.observe_enqueued_frame();
                        callback(frame);
                    }
                }));
                if let Err(payload) = worker {
                    failure_counters.observe_stream_error();
                    if let Some(sender) = runtime_event_sender.as_ref() {
                        let _ = pks_capture::publish_backend_failure(
                            sender,
                            stable_id,
                            pks_capture::SourceGeneration::INITIAL,
                            "macOS tap reader",
                            pks_capture::CaptureRuntimeFailureClass::BackendClass {
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
        })
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
            pks_capture::join_capture_worker(thread, "macOS tap reader")
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
    use super::{frame_source_id, tap_error, CORE_AUDIO_PERMISSION_DENIED_STATUS};
    use pks_capture::{CaptureError, CaptureMode, SourceKind, StableSourceId};
    use pks_frame::Platform;

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
        let expected = stable_id.to_frame_source_id();

        let observed = frame_source_id(&CaptureMode::ExactApplication {
            process_id: 42,
            stable_id,
        })
        .unwrap();

        assert_eq!(observed, expected);
    }
}
