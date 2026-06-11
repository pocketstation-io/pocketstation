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

use pocketstation_frame::{
    AudioBufferPool, AudioFrame, AudioSourceTag, EncryptionMode, SourceId, StreamId,
    DEFAULT_SAMPLE_RATE, DEFAULT_SLOT_SAMPLES_MONO_20MS,
};
use wasapi::{AudioClient, DeviceEnumerator, Direction, SampleType, StreamMode, WaveFormat};

use crate::{CaptureMode, LoopbackError};

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

const CAPTURE_CHANNELS: u8 = 2;
const CAPTURE_FRAME_SAMPLES: usize = DEFAULT_SLOT_SAMPLES_MONO_20MS * CAPTURE_CHANNELS as usize;
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
        F: Fn(AudioFrame) + Send + Sync + 'static,
    {
        Self::capture_mode(CaptureMode::SystemMix, callback)
    }

    pub fn capture_mode<F>(mode: CaptureMode, callback: F) -> Result<Self, LoopbackError>
    where
        F: Fn(AudioFrame) + Send + Sync + 'static,
    {
        // S_OK = 0, S_FALSE = 1 (already initialised) -- both success.
        let hr = wasapi::initialize_mta();
        if hr.0 < 0 {
            return Err(LoopbackError::BackendInit(format!(
                "COM MTA initialisation failed: {hr:?}"
            )));
        }

        let (stop_tx, stop_rx) = std::sync::mpsc::sync_channel::<()>(1);
        let pool = AudioBufferPool::new(POOL_DEPTH, CAPTURE_FRAME_SAMPLES);
        let seq = Arc::new(AtomicU64::new(0));

        let thread = std::thread::Builder::new()
            .name("pks-wasapi-capture".into())
            .spawn(move || {
                wasapi::initialize_mta();
                apply_mmcss_audio_thread();

                let result = match mode {
                    CaptureMode::SystemMix => run_system_loopback(pool, seq, callback, stop_rx),
                    CaptureMode::Process(pid) => {
                        run_process_loopback(pid, pool, seq, callback, stop_rx)
                    }
                    other => {
                        eprintln!("pks-wasapi: unsupported capture mode {:?}", other);
                        Ok(())
                    }
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
    use windows::Win32::Media::{Audio::AvSetMmThreadCharacteristicsW, timeBeginPeriod};
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
        DEFAULT_SAMPLE_RATE as usize,
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
    pool: &Arc<AudioBufferPool>,
    seq: &Arc<AtomicU64>,
    callback: &(impl Fn(AudioFrame) + Send + Sync + 'static),
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
    let ts_ns = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos() as u64;
    let mut frame =
        AudioFrame::new(StreamId(0), SourceId(0), s, ts_ns, CAPTURE_CHANNELS, handle);
    frame.source_tag = AudioSourceTag::Captured;
    frame.encryption_mode = EncryptionMode::None;
    frame.sample_rate = DEFAULT_SAMPLE_RATE;
    callback(frame);
}

// ---------------------------------------------------------------------------
// Event-driven capture loop
// ---------------------------------------------------------------------------

fn capture_loop(
    audio_client: &AudioClient,
    capture_client: &wasapi::AudioCaptureClient,
    h_event: &wasapi::Handle,
    pool: Arc<AudioBufferPool>,
    seq: Arc<AtomicU64>,
    callback: impl Fn(AudioFrame) + Send + Sync + 'static,
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
                    let bytes =
                        frames as usize * CAPTURE_CHANNELS as usize * size_of::<f32>();
                    deliver_packet(&raw_buf[..bytes], &pool, &seq, &callback);
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
    callback: impl Fn(AudioFrame) + Send + Sync + 'static,
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
        pool,
        seq,
        callback,
        stop_rx,
    )
}

// ---------------------------------------------------------------------------
// Process-specific loopback (Windows 10 2004+ / build 19041)
// ---------------------------------------------------------------------------

fn run_process_loopback(
    pid: u32,
    pool: Arc<AudioBufferPool>,
    seq: Arc<AtomicU64>,
    callback: impl Fn(AudioFrame) + Send + Sync + 'static,
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
        pool,
        seq,
        callback,
        stop_rx,
    )
}
