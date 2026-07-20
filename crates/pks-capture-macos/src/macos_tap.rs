//! CoreAudio process tap backend — macOS 14.4+ (public support claim).
//!
//! Uses `AudioHardwareCreateProcessTap` + `CATapDescription` to capture audio
//! from specific processes or the global system output mix without routing
//! changes, HAL plugin installation, or Screen Recording permission.

use std::ptr::NonNull;
use std::time::Duration;

use pks_frame::{AudioBufferPool, AudioFrame, AudioSourceTag, EncryptionMode, Platform, StreamId};

use pks_capture::{
    CaptureError as LoopbackError, CaptureMode, CaptureSampleTimeline, CaptureSource, SourceKind,
    SourceState, StableSourceId,
};

// ---------------------------------------------------------------------------
// FFI — mirrors source_discovery.h
// ---------------------------------------------------------------------------

#[repr(C)]
struct RawSourceInfo {
    audio_object_id: u32,
    pid: i32,
    bundle_id: [u8; 256],
    name: [u8; 256],
    kind: u8,
    state: u8,
    sample_rate: u32,
    channels: u16,
}

extern "C" {
    fn pks_process_tap_available() -> i32;
    fn pks_discover_sources(out: *mut RawSourceInfo, max: i32) -> i32;
    fn pks_create_process_tap(
        pids: *const i32,
        pid_count: i32,
        out_status: *mut i32,
        out_stage: *mut u8,
    ) -> *mut std::ffi::c_void;
    fn pks_tap_start(tap: *mut std::ffi::c_void, out_status: *mut i32, out_stage: *mut u8) -> i32;
    fn pks_destroy_process_tap(tap: *mut std::ffi::c_void);
    fn pks_tap_read_frames(tap: *mut std::ffi::c_void, out: *mut f32, frame_count: u32) -> u32;
    fn pks_tap_sample_rate(tap: *const std::ffi::c_void) -> u32;
    fn pks_tap_channels(tap: *const std::ffi::c_void) -> u32;
    // Reserved for future audio-level diagnostic (capture-path signal-level probe).
    #[allow(dead_code)]
    fn pks_tap_level(tap: *const std::ffi::c_void) -> f32;
}

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

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
    unsafe { pks_process_tap_available() != 0 }
}

/// Enumerate all running processes that have audio output.
/// Returns an empty `Vec` on macOS < 14.4 (public support floor) or on non-macOS platforms.
pub fn discover_sources_native() -> Vec<CaptureSource> {
    const MAX: usize = 128;
    // Safety: write_bytes zeroes the allocation before set_len, so all MAX
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
            let pid = if r.pid > 0 { Some(r.pid as u32) } else { None };
            let app_id = cstr_to_opt(&r.bundle_id);
            let source_kind = match r.kind {
                1 => SourceKind::InputDevice,
                2 => SourceKind::OutputDevice,
                3 => SourceKind::SystemMix,
                _ => SourceKind::Application,
            };
            let stable_key = app_id
                .as_deref()
                .map(|id| id.to_owned())
                .unwrap_or_else(|| format!("pid:{}", r.pid));
            CaptureSource {
                stable_id: StableSourceId::new(Platform::Macos, source_kind, stable_key),
                name: cstr_to_string(&r.name).unwrap_or_else(|| format!("pid:{}", r.pid)),
                process_id: pid,
                app_id,
                device_uid: None,
                state: match r.state {
                    1 => SourceState::Playing,
                    2 => SourceState::Silent,
                    3 => SourceState::Unavailable,
                    _ => SourceState::Available,
                },
                sample_rate_hz: r.sample_rate,
                channels: r.channels,
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

// ---------------------------------------------------------------------------
// ProcessTap — RAII handle around pks_create_process_tap
// ---------------------------------------------------------------------------

struct ProcessTap(NonNull<std::ffi::c_void>);

const CORE_AUDIO_PERMISSION_DENIED_STATUS: i32 = i32::from_be_bytes(*b"!hog");

fn tap_operation(stage: u8) -> &'static str {
    match stage {
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

fn tap_error(status: i32, stage: u8) -> LoopbackError {
    let operation = tap_operation(stage);
    if status == CORE_AUDIO_PERMISSION_DENIED_STATUS {
        LoopbackError::PermissionDenied { operation }
    } else {
        LoopbackError::BackendStatus { operation, status }
    }
}

// Safety:
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

    fn create(pids: *const i32, pid_count: i32) -> Result<Self, LoopbackError> {
        let mut status = 0;
        let mut stage = 0;
        let handle = unsafe { pks_create_process_tap(pids, pid_count, &mut status, &mut stage) };
        NonNull::new(handle)
            .map(Self)
            .ok_or_else(|| tap_error(status, stage))
    }

    fn start(&mut self) -> Result<(), LoopbackError> {
        let mut status = 0;
        let mut stage = 0;
        if unsafe { pks_tap_start(self.0.as_ptr(), &mut status, &mut stage) } == 0 {
            Ok(())
        } else {
            Err(tap_error(status, stage))
        }
    }

    fn sample_rate(&self) -> u32 {
        unsafe { pks_tap_sample_rate(self.0.as_ptr()) }
    }

    fn channels(&self) -> u32 {
        unsafe { pks_tap_channels(self.0.as_ptr()) }
    }

    fn read_frames(&mut self, out: &mut [f32], frame_count: u32) -> u32 {
        let required = frame_count as usize * self.channels() as usize;
        if out.len() < required {
            return 0;
        }
        unsafe { pks_tap_read_frames(self.0.as_ptr(), out.as_mut_ptr(), frame_count) }
    }

    fn level(&self) -> f32 {
        unsafe { pks_tap_level(self.0.as_ptr()) }
    }
}

impl Drop for ProcessTap {
    fn drop(&mut self) {
        unsafe {
            pks_destroy_process_tap(self.0.as_ptr());
        }
    }
}

// ---------------------------------------------------------------------------
// TapLoopbackSource — public capture source using the process tap
// ---------------------------------------------------------------------------

const POOL_DEPTH: usize = 8;

/// Captures system audio via CoreAudio process tap (macOS 14.2+).
pub struct TapLoopbackSource {
    pub(crate) _thread: std::thread::JoinHandle<()>,
    pub(crate) stop_tx: std::sync::mpsc::SyncSender<()>,
}

impl TapLoopbackSource {
    pub fn capture_mode<F>(mode: CaptureMode, mut callback: F) -> Result<Self, LoopbackError>
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

        let sample_rate = tap.sample_rate();
        let sample_rate_nonzero = std::num::NonZeroU32::new(sample_rate).ok_or_else(|| {
            LoopbackError::BackendInit("tap reported a zero sample rate".to_owned())
        })?;
        let channels = tap.channels() as u8;
        let frames_per_cb: u32 = sample_rate / 50; // 20 ms
        let buf_samples = frames_per_cb as usize * channels as usize;
        let pool = std::sync::Arc::new(AudioBufferPool::new(POOL_DEPTH, buf_samples));
        let (stop_tx, stop_rx) = std::sync::mpsc::sync_channel::<()>(1);

        // Compute the stable SourceId from the capture mode before the thread
        // is spawned so that every AudioFrame carries the correct source identity.
        let frame_source_id = match &mode {
            CaptureMode::SystemMix => {
                StableSourceId::new(Platform::Macos, SourceKind::SystemMix, "system:mix")
                    .to_frame_source_id()
            }
            CaptureMode::Process(pid) => StableSourceId::new(
                Platform::Macos,
                SourceKind::Application,
                format!("pid:{pid}"),
            )
            .to_frame_source_id(),
            CaptureMode::Application(bundle_id) => {
                StableSourceId::new(Platform::Macos, SourceKind::Application, bundle_id.clone())
                    .to_frame_source_id()
            }
            CaptureMode::InputDevice(_) => {
                return Err(LoopbackError::ModeUnsupported(mode));
            }
        };

        let thread = std::thread::Builder::new()
            .name("pks-tap-reader".into())
            .spawn(move || {
                let mut seq: u64 = 0;
                let mut timeline = CaptureSampleTimeline::new(sample_rate_nonzero);
                let mut buf = vec![0.0f32; buf_samples];
                let diag = std::env::var_os("PKS_TAP_DIAG").is_some();
                let mut diag_samples: u64 = 0;
                let mut diag_nonzero_frames: u64 = 0;
                let mut diag_peak: f32 = 0.0;
                let mut diag_last = std::time::Instant::now();
                loop {
                    if stop_rx.try_recv().is_ok() {
                        break;
                    }
                    let read = tap.read_frames(&mut buf, frames_per_cb);
                    if read == 0 {
                        std::thread::sleep(Duration::from_millis(1));
                        continue;
                    }
                    let timestamp_ns = timeline.advance(u64::from(read));
                    let frame_sequence_number = seq;
                    seq = seq.saturating_add(1);
                    let mut handle = match pool.acquire() {
                        Some(h) => h,
                        None => continue,
                    };
                    let dst = handle.as_mut_slice();
                    let samples = read as usize * channels as usize;
                    let copy_len = samples.min(dst.len());
                    dst[..copy_len].copy_from_slice(&buf[..copy_len]);
                    handle.set_len(copy_len);
                    if diag {
                        diag_samples += copy_len as u64;
                        let chunk_peak = buf[..copy_len]
                            .iter()
                            .fold(0.0_f32, |acc, sample| acc.max(sample.abs()));
                        if chunk_peak > 1e-9 {
                            diag_nonzero_frames += 1;
                        }
                        diag_peak = diag_peak.max(chunk_peak);
                        if diag_last.elapsed() >= Duration::from_secs(1) {
                            eprintln!(
                                "tap_diag: read_samples={} nonzero_chunks={} peak={:.6} tap_rms={:.6}",
                                diag_samples,
                                diag_nonzero_frames,
                                diag_peak,
                                tap.level()
                            );
                            diag_samples = 0;
                            diag_nonzero_frames = 0;
                            diag_peak = 0.0;
                            diag_last = std::time::Instant::now();
                        }
                    }
                    let mut frame = AudioFrame::new(
                        StreamId(0),
                        frame_source_id,
                        frame_sequence_number,
                        timestamp_ns,
                        channels,
                        handle,
                    );
                    frame.source_tag = AudioSourceTag::Captured;
                    frame.encryption_mode = EncryptionMode::None;
                    frame.sample_rate_hz = sample_rate;
                    callback(frame);
                }
            })
            .map_err(|e| LoopbackError::BackendInit(format!("thread spawn: {e}")))?;

        Ok(Self {
            _thread: thread,
            stop_tx,
        })
    }
}

impl Drop for TapLoopbackSource {
    fn drop(&mut self) {
        let _ = self.stop_tx.try_send(());
    }
}

#[cfg(test)]
mod tests {
    use super::{tap_error, CORE_AUDIO_PERMISSION_DENIED_STATUS};
    use pks_capture::CaptureError;

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
                status: -50
            }
        );
    }
}
