//! Windows WASAPI loopback capture backend.
//!
//! Supports two modes:
//!
//! - `CaptureMode::SystemMix` -- captures the system-wide audio mix using
//!   `AUDCLNT_STREAMFLAGS_LOOPBACK` on the default render endpoint.
//!   Available on Windows Vista+.
//!
//! - `CaptureMode::Process(pid)` -- captures audio from a single process
//!   using `ActivateAudioInterfaceAsync` with process-loopback params.
//!   Available on Windows 10 2004+ (build 19041).
//!
//! # Hot-path invariants
//!
//! No allocation, locking, logging, or panicking on the audio delivery path.
//! Pool slot acquisition is lock-free (atomic CAS in `AudioBufferPool::acquire`).
//!
//! # Known WASAPI bug -- process-loopback period
//!
//! When an `AudioClient` is created via `new_application_loopback_client`,
//! `get_device_period` returns `Not implemented` and `get_buffer_size` returns
//! garbage (e.g. 3 131 961 357).  The period passed to `initialize_client`
//! is documented to be irrelevant in this mode, so we use
//! `WASAPI_PROCESS_LOOPBACK_PERIOD_100NS` (10 ms in 100-ns units) as a safe
//! placeholder that avoids passing zero.
//! Reference: wasapi crate 0.23 doc comment on `new_application_loopback_client`.

use std::mem::size_of;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

use pks_frame::{
    AudioBufferPool, AudioFrame, AudioSourceTag, EncryptionMode, SourceId, StreamId,
    POOL_SLOT_SAMPLES, SAMPLE_RATE_HZ,
};
use wasapi::{AudioClient, DeviceEnumerator, Direction, SampleType, StreamMode, WaveFormat};

use pks_capture::{monotonic_timestamp_ns, CaptureError as LoopbackError, CaptureMode};

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

const CAPTURE_CHANNELS: u8 = 2;
const CAPTURE_FRAME_SAMPLES: usize = POOL_SLOT_SAMPLES * CAPTURE_CHANNELS as usize;
const POOL_DEPTH: usize = 8;

/// Buffer duration hint (20 ms in 100-ns units).  Ignored for loopback modes.
const BUFFER_DURATION_100NS: i64 = 200_000;

/// Hardcoded period for process-loopback mode (10 ms in 100-ns units).
///
/// `get_device_period` returns `Not implemented` in process-loopback mode.
/// The period value is also documented as irrelevant.  We use this non-zero
/// constant as a safe placeholder.
pub(crate) const WASAPI_PROCESS_LOOPBACK_PERIOD_100NS: i64 = 100_000;

/// `wait_for_event` timeout in milliseconds.
const WAIT_TIMEOUT_MS: u32 = 200;

// ---------------------------------------------------------------------------
// Public struct
// ---------------------------------------------------------------------------

/// Manages a WASAPI loopback capture session.
///
/// Drop this value to stop capture.
pub struct SystemLoopbackSource {
    _thread: std::thread::JoinHandle<()>,
    stop_tx: std::sync::mpsc::SyncSender<()>,
}

impl SystemLoopbackSource {
    pub fn capture<F>(callback: F) -> Result<Self, LoopbackError>
    where
        F: FnMut(AudioFrame) + Send + 'static,
    {
        Self::capture_mode(CaptureMode::SystemMix, callback)
    }

    pub fn capture_mode<F>(mode: CaptureMode, callback: F) -> Result<Self, LoopbackError>
    where
        F: FnMut(AudioFrame) + Send + 'static,
    {
        // S_OK = 0, S_FALSE = 1 (already initialised) -- both success.
        let hr = wasapi::initialize_mta();
        if hr.0 < 0 {
            return Err(LoopbackError::BackendInit(format!(
                "COM MTA initialisation failed: {hr:?}"
            )));
        }

        // Resolve Application(name) → Process(pid) before spawning the thread.
        let resolved_mode = match mode {
            CaptureMode::Application(ref name) => {
                let sources = discover_sources_windows();
                let name_lower = name.to_ascii_lowercase();
                let matched_pid = sources.iter().find(|s| {
                    s.name.to_ascii_lowercase() == name_lower
                        || s.app_id.as_deref().map(|id| id.to_ascii_lowercase())
                            == Some(name_lower.clone())
                });
                match matched_pid {
                    Some(src) => match src.process_id {
                        Some(pid) => CaptureMode::ExactApplication {
                            process_id: pid,
                            stable_id: src.stable_id.clone(),
                        },
                        None => {
                            return Err(LoopbackError::BackendInit(format!(
                                "no audio session found for '{name}' — run `pks sources list`"
                            )));
                        }
                    },
                    None => {
                        return Err(LoopbackError::BackendInit(format!(
                            "no audio session found for '{name}' — run `pks sources list`"
                        )));
                    }
                }
            }
            other => other,
        };

        let (stop_tx, stop_rx) = std::sync::mpsc::sync_channel::<()>(1);
        let pool = AudioBufferPool::new(POOL_DEPTH, CAPTURE_FRAME_SAMPLES);
        let seq = Arc::new(AtomicU64::new(0));

        let thread = std::thread::Builder::new()
            .name("pks-wasapi-capture".into())
            .spawn(move || {
                wasapi::initialize_mta();
                apply_mmcss_audio_thread();

                let result = match resolved_mode {
                    CaptureMode::SystemMix => run_system_loopback(pool, seq, callback, stop_rx),
                    CaptureMode::Process(pid) => {
                        run_process_loopback(pid, SourceId(0), pool, seq, callback, stop_rx)
                    }
                    CaptureMode::ExactApplication {
                        process_id,
                        stable_id,
                    } => run_process_loopback(
                        process_id,
                        stable_id.to_frame_source_id(),
                        pool,
                        seq,
                        callback,
                        stop_rx,
                    ),
                    // Application(_) is resolved to Process before thread spawn.
                    other => Err(LoopbackError::ModeUnsupported(other)),
                };
                if let Err(e) = result {
                    eprintln!("pks-wasapi capture error: {e}");
                }
            })
            .map_err(|e| LoopbackError::BackendInit(e.to_string()))?;

        Ok(Self {
            _thread: thread,
            stop_tx,
        })
    }
}

impl Drop for SystemLoopbackSource {
    fn drop(&mut self) {
        let _ = self.stop_tx.try_send(());
    }
}

// ---------------------------------------------------------------------------
// MMCSS helper
// ---------------------------------------------------------------------------

fn apply_mmcss_audio_thread() {
    use windows::Win32::Media::{timeBeginPeriod, Audio::AvSetMmThreadCharacteristicsW};
    use windows_core::w;

    // SAFETY: both API calls are safe to call from any thread.
    // Failure is non-fatal -- capture continues at normal priority.
    unsafe {
        timeBeginPeriod(1);
        let mut task_index: u32 = 0;
        let _ = AvSetMmThreadCharacteristicsW(w!("Audio"), &mut task_index);
    }
}

// ---------------------------------------------------------------------------
// Shared wave format
// ---------------------------------------------------------------------------

fn target_wave_format() -> WaveFormat {
    WaveFormat::new(
        32,
        32,
        &SampleType::Float,
        SAMPLE_RATE_HZ as usize,
        CAPTURE_CHANNELS as usize,
        None,
    )
}

// ---------------------------------------------------------------------------
// Frame delivery helper (hot path -- no allocation)
// ---------------------------------------------------------------------------

#[inline(always)]
fn deliver_packet(
    raw: &[u8],
    source_id: SourceId,
    pool: &Arc<AudioBufferPool>,
    seq: &Arc<AtomicU64>,
    callback: &mut (impl FnMut(AudioFrame) + Send + 'static),
) {
    let n_samples = raw.len() / size_of::<f32>();
    if n_samples == 0 {
        return;
    }
    let mut handle = match pool.acquire() {
        Some(h) => h,
        None => return,
    };
    let dst = handle.as_mut_slice();
    let copy_count = n_samples.min(dst.len());
    // SAFETY: `raw` is a valid WASAPI buffer; 4-byte groups are f32.
    let src_ptr = raw.as_ptr() as *const f32;
    for i in 0..copy_count {
        dst[i] = unsafe { src_ptr.add(i).read_unaligned() };
    }
    handle.set_len(copy_count);

    let s = seq.fetch_add(1, Ordering::Relaxed);
    let ts_ns = monotonic_timestamp_ns();
    let mut frame = AudioFrame::new(StreamId(0), source_id, s, ts_ns, CAPTURE_CHANNELS, handle);
    frame.source_tag = AudioSourceTag::Captured;
    frame.encryption_mode = EncryptionMode::None;
    frame.sample_rate = SAMPLE_RATE_HZ;
    callback(frame);
}

// ---------------------------------------------------------------------------
// Event-driven capture loop
// ---------------------------------------------------------------------------

fn capture_loop(
    audio_client: &AudioClient,
    capture_client: &wasapi::AudioCaptureClient,
    h_event: &wasapi::Handle,
    source_id: SourceId,
    pool: Arc<AudioBufferPool>,
    seq: Arc<AtomicU64>,
    mut callback: impl FnMut(AudioFrame) + Send + 'static,
    stop_rx: std::sync::mpsc::Receiver<()>,
) -> Result<(), LoopbackError> {
    const BUF_BYTES: usize = CAPTURE_FRAME_SAMPLES * size_of::<f32>() * 2;
    let mut raw_buf = [0u8; BUF_BYTES];

    loop {
        if stop_rx.try_recv().is_ok() {
            break;
        }
        match h_event.wait_for_event(WAIT_TIMEOUT_MS) {
            Ok(()) => {}
            Err(_) => {
                if stop_rx.try_recv().is_ok() {
                    break;
                }
                continue;
            }
        }
        // Drain all available packets.
        loop {
            let next = match capture_client.get_next_packet_size() {
                Ok(Some(n)) => n,
                Ok(None) => 0,
                Err(_) => break,
            };
            if next == 0 {
                break;
            }
            match capture_client.read_from_device(&mut raw_buf) {
                Ok((frames, info)) => {
                    if frames == 0 || info.flags.silent {
                        continue;
                    }
                    let bytes = frames as usize * CAPTURE_CHANNELS as usize * size_of::<f32>();
                    deliver_packet(&raw_buf[..bytes], source_id, &pool, &seq, &mut callback);
                }
                Err(_) => break,
            }
        }
    }
    let _ = audio_client.stop_stream();
    Ok(())
}

// ---------------------------------------------------------------------------
// System-wide loopback
// ---------------------------------------------------------------------------

fn run_system_loopback(
    pool: Arc<AudioBufferPool>,
    seq: Arc<AtomicU64>,
    callback: impl FnMut(AudioFrame) + Send + 'static,
    stop_rx: std::sync::mpsc::Receiver<()>,
) -> Result<(), LoopbackError> {
    let enumerator =
        DeviceEnumerator::new().map_err(|e| LoopbackError::BackendInit(e.to_string()))?;
    let device = enumerator
        .get_default_device(&Direction::Render)
        .map_err(|e| LoopbackError::BackendInit(e.to_string()))?;
    let mut audio_client = device
        .get_iaudioclient()
        .map_err(|e| LoopbackError::BackendInit(e.to_string()))?;
    let wave_fmt = target_wave_format();
    audio_client
        .initialize_client(
            &wave_fmt,
            &Direction::Capture, // wasapi adds LOOPBACK flag automatically
            &StreamMode::EventsShared {
                autoconvert: true,
                buffer_duration_hns: BUFFER_DURATION_100NS,
            },
        )
        .map_err(|e| LoopbackError::BackendInit(e.to_string()))?;
    let h_event = audio_client
        .set_get_eventhandle()
        .map_err(|e| LoopbackError::BackendInit(e.to_string()))?;
    let capture_client = audio_client
        .get_audiocaptureclient()
        .map_err(|e| LoopbackError::BackendInit(e.to_string()))?;
    audio_client
        .start_stream()
        .map_err(|e| LoopbackError::BackendInit(e.to_string()))?;
    capture_loop(
        &audio_client,
        &capture_client,
        &h_event,
        SourceId(0),
        pool,
        seq,
        callback,
        stop_rx,
    )
}

// ---------------------------------------------------------------------------
// Source discovery
// ---------------------------------------------------------------------------

/// Enumerate all audio capture sources visible on this Windows system.
///
/// Always returns at least one entry (the system-wide mix at id=0).
/// Per-process application sources are appended via WASAPI session enumeration.
pub(crate) fn discover_sources_windows() -> Vec<pks_capture::CaptureSource> {
    use pks_capture::{CaptureSource, SourceKind, SourceState, StableSourceId};
    use pks_frame::Platform;

    let system_mix = CaptureSource {
        stable_id: StableSourceId::new(Platform::Windows, SourceKind::SystemMix, "system:mix"),
        name: "System Mix".to_owned(),
        process_id: None,
        app_id: None,
        device_uid: None,
        state: SourceState::Available,
        sample_rate: 48_000,
        channels: 2,
    };

    let hr = wasapi::initialize_mta();
    if hr.0 < 0 {
        return vec![system_mix];
    }

    let mut sources = vec![system_mix];
    // SAFETY: COM is initialised above; all COM objects are released before return.
    let app_sources = unsafe { enumerate_wasapi_sessions() };
    sources.extend(app_sources);
    sources
}

/// Enumerate active WASAPI audio sessions on the default render endpoint.
///
/// Returns per-process `Application` sources.  On any error, returns an empty
/// `Vec` — missing sessions are not fatal.
///
/// # Safety
///
/// Caller must have initialised COM (MTA) before calling this function.
unsafe fn enumerate_wasapi_sessions() -> Vec<pks_capture::CaptureSource> {
    use pks_capture::{CaptureSource, SourceKind, SourceState, StableSourceId};
    use pks_frame::Platform;
    use windows::Win32::Foundation::CloseHandle;
    use windows::Win32::Media::Audio::{
        eMultimedia, eRender, AudioSessionStateActive, IAudioSessionControl, IAudioSessionControl2,
        IAudioSessionEnumerator, IAudioSessionManager2, IMMDeviceEnumerator, MMDeviceEnumerator,
    };
    use windows::Win32::System::Com::{CoCreateInstance, CLSCTX_ALL};
    use windows::Win32::System::Threading::{
        OpenProcess, QueryFullProcessImageNameW, PROCESS_NAME_WIN32,
        PROCESS_QUERY_LIMITED_INFORMATION,
    };
    use windows_core::PWSTR;

    let current_pid = std::process::id();

    let enumerator: IMMDeviceEnumerator =
        match CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_ALL) {
            Ok(e) => e,
            Err(_) => return Vec::new(),
        };

    let device = match enumerator.GetDefaultAudioEndpoint(eRender, eMultimedia) {
        Ok(d) => d,
        Err(_) => return Vec::new(),
    };

    let session_manager: IAudioSessionManager2 = match device.Activate(CLSCTX_ALL, None) {
        Ok(sm) => sm,
        Err(_) => return Vec::new(),
    };

    let session_enum: IAudioSessionEnumerator = match session_manager.GetSessionEnumerator() {
        Ok(se) => se,
        Err(_) => return Vec::new(),
    };

    let count = match session_enum.GetCount() {
        Ok(c) => c,
        Err(_) => return Vec::new(),
    };

    let mut sources: Vec<CaptureSource> = Vec::with_capacity(count as usize);

    for i in 0..count {
        let ctrl: IAudioSessionControl = match session_enum.GetSession(i) {
            Ok(c) => c,
            Err(_) => continue,
        };
        let ctrl2: IAudioSessionControl2 = match ctrl.cast() {
            Ok(c) => c,
            Err(_) => continue,
        };

        let pid = match ctrl2.GetProcessId() {
            Ok(p) => p,
            Err(_) => continue,
        };

        // Skip system/idle sessions and our own process.
        if pid == 0 || pid == current_pid {
            continue;
        }

        let state = match ctrl.GetState() {
            Ok(s) => s,
            Err(_) => continue,
        };
        let source_state = if state == AudioSessionStateActive {
            SourceState::Playing
        } else {
            SourceState::Silent
        };

        // Resolve process name from PID.
        let name = match OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, false, pid) {
            Ok(handle) => {
                let mut buf = vec![0u16; 260];
                let mut len = buf.len() as u32;
                let name_result = QueryFullProcessImageNameW(
                    handle,
                    PROCESS_NAME_WIN32,
                    PWSTR(buf.as_mut_ptr()),
                    &mut len,
                );
                let _ = CloseHandle(handle);
                match name_result {
                    Ok(()) => {
                        let path = String::from_utf16_lossy(&buf[..len as usize]);
                        std::path::Path::new(&path)
                            .file_stem()
                            .and_then(|s| s.to_str())
                            .unwrap_or(&path)
                            .to_owned()
                    }
                    Err(_) => format!("pid-{pid}"),
                }
            }
            Err(_) => format!("pid-{pid}"),
        };

        sources.push(CaptureSource {
            stable_id: StableSourceId::new(
                Platform::Windows,
                SourceKind::Application,
                format!("wasapi:pid:{pid}"),
            ),
            name: name.clone(),
            process_id: Some(pid),
            app_id: Some(name),
            device_uid: None,
            state: source_state,
            sample_rate: 48_000,
            channels: 2,
        });
    }

    sources
}

// ---------------------------------------------------------------------------
// Process-specific loopback (Windows 10 2004+ / build 19041)
// ---------------------------------------------------------------------------

fn run_process_loopback(
    pid: u32,
    source_id: SourceId,
    pool: Arc<AudioBufferPool>,
    seq: Arc<AtomicU64>,
    callback: impl FnMut(AudioFrame) + Send + 'static,
    stop_rx: std::sync::mpsc::Receiver<()>,
) -> Result<(), LoopbackError> {
    // include_tree = false: capture only `pid`, not its children.
    let mut audio_client = AudioClient::new_application_loopback_client(pid, false)
        .map_err(|e| LoopbackError::BackendInit(e.to_string()))?;
    let wave_fmt = target_wave_format();
    // Period is irrelevant in process-loopback mode; use a safe non-zero value.
    audio_client
        .initialize_client(
            &wave_fmt,
            &Direction::Capture,
            &StreamMode::EventsShared {
                autoconvert: true,
                buffer_duration_hns: WASAPI_PROCESS_LOOPBACK_PERIOD_100NS,
            },
        )
        .map_err(|e| LoopbackError::BackendInit(e.to_string()))?;
    let h_event = audio_client
        .set_get_eventhandle()
        .map_err(|e| LoopbackError::BackendInit(e.to_string()))?;
    let capture_client = audio_client
        .get_audiocaptureclient()
        .map_err(|e| LoopbackError::BackendInit(e.to_string()))?;
    audio_client
        .start_stream()
        .map_err(|e| LoopbackError::BackendInit(e.to_string()))?;
    capture_loop(
        &audio_client,
        &capture_client,
        &h_event,
        source_id,
        pool,
        seq,
        callback,
        stop_rx,
    )
}
