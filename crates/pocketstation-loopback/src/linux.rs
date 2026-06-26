//! Linux system audio loopback via PipeWire (primary) with snd-aloop ALSA fallback.
//!
//! ## PipeWire path
//!
//! A dedicated OS thread named `"pks-pipewire-capture"` owns the PipeWire
//! `MainLoop` for its entire lifetime.  PipeWire objects are not `Send`/`Sync`,
//! so all creation happens inside that thread.
//!
//! Audio data flows from the PipeWire process callback through a bounded
//! `std::sync::mpsc::sync_channel` to a second dispatcher thread
//! (`"pks-pipewire-dispatch"`), which invokes the user callback without
//! holding any PipeWire lock.
//!
//! ## Hot-path rule
//!
//! The PipeWire process callback does only lock-free work:
//! - `AudioBufferPool::acquire()` - lock-free CAS
//! - byte copy from PipeWire buffer into pool slot
//! - `mpsc::SyncSender::try_send` - non-blocking send, drops frame when full

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::mpsc;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use pipewire as pw;
use pocketstation_frame::{
    AudioBufferPool, AudioFrame, AudioSourceTag, EncryptionMode, SourceId, StreamId,
    DEFAULT_SAMPLE_RATE, DEFAULT_SLOT_SAMPLES_MONO_20MS,
};
use pw::properties::properties;
use pw::spa;
use pw::spa::pod::Pod;
use spa::param::audio::AudioFormat;

use crate::{CaptureMode, LoopbackError};

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

/// Stereo capture.
const CAPTURE_CHANNELS: u8 = 2;

/// Stereo 20 ms frame at 48 kHz = 960 mono samples * 2 channels.
const CAPTURE_FRAME_SAMPLES: usize = DEFAULT_SLOT_SAMPLES_MONO_20MS * CAPTURE_CHANNELS as usize;

/// Pool depth: 8 frames absorb callback jitter without unbounded growth.
const POOL_DEPTH: usize = 8;

/// Bounded channel depth between the PW process callback and the dispatch thread.
const PW_CHANNEL_DEPTH: usize = 10;

/// PipeWire node latency: 128 frames at 48 kHz (~2.67 ms quantum).
const PW_NODE_LATENCY: &str = "128/48000";

/// Timer interval for polling the stop signal from within the PipeWire MainLoop.
const PW_STOP_POLL_MS: u64 = 50;

/// ALSA loopback capture device (snd-aloop module, subdevice 0 of card 1).
/// Requires: sudo modprobe snd-aloop
const ALSA_LOOPBACK_DEVICE: &str = "hw:Loopback,1,0";

// ---------------------------------------------------------------------------
// Public struct
// ---------------------------------------------------------------------------

/// Manages a Linux loopback capture session.
///
/// Drop this value to stop capture.
#[derive(Debug)]
pub struct SystemLoopbackSource {
    _capture_thread: thread::JoinHandle<()>,
    _dispatch_thread: thread::JoinHandle<()>,
    stop_tx: mpsc::SyncSender<()>,
}

impl SystemLoopbackSource {
    /// Start capturing the system-wide audio mix.
    pub fn capture<F>(callback: F) -> Result<Self, LoopbackError>
    where
        F: Fn(AudioFrame) + Send + Sync + 'static,
    {
        Self::capture_mode(CaptureMode::SystemMix, callback)
    }

    /// Start capturing in the given mode.
    pub fn capture_mode<F>(mode: CaptureMode, callback: F) -> Result<Self, LoopbackError>
    where
        F: Fn(AudioFrame) + Send + Sync + 'static,
    {
        match &mode {
            CaptureMode::Process(_) | CaptureMode::Application(_) => {
                return Err(LoopbackError::ModeUnsupported(mode));
            }
            _ => {}
        }
        if pipewire_available() {
            run_pipewire(mode, callback)
        } else {
            run_alsa(callback)
        }
    }
}

impl Drop for SystemLoopbackSource {
    fn drop(&mut self) {
        let _ = self.stop_tx.try_send(());
    }
}

// ---------------------------------------------------------------------------
// PipeWire availability probe
// ---------------------------------------------------------------------------

/// Returns `true` if the PipeWire socket is present in `$XDG_RUNTIME_DIR`.
///
/// Uses a single `stat(2)` call -- no I/O, no allocation.
fn pipewire_available() -> bool {
    let runtime_dir = std::env::var("XDG_RUNTIME_DIR")
        .unwrap_or_else(|_| format!("/run/user/{}", unsafe { libc::getuid() }));
    std::path::Path::new(&runtime_dir)
        .join("pipewire-0")
        .exists()
}

// ---------------------------------------------------------------------------
// PipeWire implementation
// ---------------------------------------------------------------------------

fn run_pipewire<F>(_mode: CaptureMode, callback: F) -> Result<SystemLoopbackSource, LoopbackError>
where
    F: Fn(AudioFrame) + Send + Sync + 'static,
{
    let (stop_tx, stop_rx) = mpsc::sync_channel::<()>(1);
    let (frame_tx, frame_rx) = mpsc::sync_channel::<AudioFrame>(PW_CHANNEL_DEPTH);

    let pool = AudioBufferPool::new(POOL_DEPTH, CAPTURE_FRAME_SAMPLES);
    let seq = Arc::new(AtomicU64::new(0));

    // Capture thread: owns the PipeWire MainLoop for its lifetime.
    let capture_thread = thread::Builder::new()
        .name("pks-pipewire-capture".into())
        .spawn(move || {
            pw::init();
            let mainloop = match pw::main_loop::MainLoopRc::new(None) {
                Ok(ml) => ml,
                Err(e) => {
                    eprintln!("pks-pipewire: MainLoop::new failed: {e}");
                    return;
                }
            };
            let context = match pw::context::ContextRc::new(&mainloop, None) {
                Ok(ctx) => ctx,
                Err(e) => {
                    eprintln!("pks-pipewire: Context::new failed: {e}");
                    return;
                }
            };
            let core = match context.connect_rc(None) {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("pks-pipewire: Context::connect failed: {e}");
                    return;
                }
            };

            // STREAM_CAPTURE_SINK captures the monitor port of the default sink
            // (system-wide output mix).
            let stream_props = properties! {
                *pw::keys::NODE_NAME => "pks-loopback-capture",
                *pw::keys::NODE_LATENCY => PW_NODE_LATENCY,
                *pw::keys::STREAM_CAPTURE_SINK => "true",
            };

            let stream = match pw::stream::StreamRc::new(core, "pks-loopback", stream_props) {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("pks-pipewire: Stream::new failed: {e}");
                    return;
                }
            };

            let frame_tx_cb = frame_tx.clone();
            let pool_cb = pool.clone();
            let seq_cb = seq.clone();

            let _listener = stream
                .add_local_listener_with_user_data(())
                .param_changed(|_stream, _user, _id, _param| {})
                .process(move |stream, _user| {
                    let mut buf = match stream.dequeue_buffer() {
                        Some(b) => b,
                        None => return,
                    };
                    let datas = buf.datas_mut();
                    if datas.is_empty() {
                        return;
                    }
                    let chunk = datas[0].chunk();
                    let byte_count = chunk.size() as usize;
                    if byte_count == 0 {
                        return;
                    }
                    let src_ptr = match datas[0].data() {
                        Some(d) => d.as_ptr(),
                        None => return,
                    };
                    let n_samples = byte_count / std::mem::size_of::<f32>();

                    let mut handle = match pool_cb.acquire() {
                        Some(h) => h,
                        None => return,
                    };
                    let dst = handle.as_mut_slice();
                    let copy_count = n_samples.min(dst.len());
                    // SAFETY: src_ptr points into a valid PipeWire buffer.
                    unsafe {
                        std::ptr::copy_nonoverlapping(
                            src_ptr as *const f32,
                            dst.as_mut_ptr(),
                            copy_count,
                        );
                    }
                    handle.set_len(copy_count);

                    let s = seq_cb.fetch_add(1, Ordering::Relaxed);
                    let ts_ns = std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap_or_default()
                        .as_nanos() as u64;

                    let mut frame = AudioFrame::new(
                        StreamId(0),
                        SourceId(0),
                        s,
                        ts_ns,
                        CAPTURE_CHANNELS,
                        handle,
                    );
                    frame.source_tag = AudioSourceTag::Captured;
                    frame.encryption_mode = EncryptionMode::None;
                    frame.sample_rate = DEFAULT_SAMPLE_RATE;

                    let _ = frame_tx_cb.try_send(frame);
                })
                .register()
                .expect("listener registration must not fail");

            // Build audio format params.
            let mut audio_info = spa::param::audio::AudioInfoRaw::new();
            audio_info.set_format(AudioFormat::F32LE);
            audio_info.set_rate(DEFAULT_SAMPLE_RATE);
            audio_info.set_channels(CAPTURE_CHANNELS as u32);
            let obj = pw::spa::pod::serialize::PodSerializer::serialize(
                std::io::Cursor::new(Vec::new()),
                &pw::spa::pod::Value::Object(pw::spa::pod::Object {
                    type_: pw::spa::utils::SpaTypes::ObjectParamFormat.as_raw(),
                    id: pw::spa::param::ParamType::EnumFormat.as_raw(),
                    properties: audio_info.into(),
                }),
            )
            .unwrap()
            .0
            .into_inner();
            let param = Pod::from_bytes(&obj).unwrap();

            let flags = pw::stream::StreamFlags::AUTOCONNECT
                | pw::stream::StreamFlags::MAP_BUFFERS
                | pw::stream::StreamFlags::RT_PROCESS;

            if let Err(e) =
                stream.connect(pw::spa::utils::Direction::Input, None, flags, &mut [param])
            {
                eprintln!("pks-pipewire: stream.connect failed: {e}");
                return;
            }

            // Attach a timer to poll for stop signal from within the event loop.
            let ml_weak = mainloop.downgrade();
            let timer = mainloop.loop_().add_timer(move |_| {
                if stop_rx.try_recv().is_ok() {
                    if let Some(ml) = ml_weak.upgrade() {
                        ml.quit();
                    }
                }
            });
            timer.update_timer(
                Some(Duration::from_millis(PW_STOP_POLL_MS)),
                Some(Duration::from_millis(PW_STOP_POLL_MS)),
            );

            mainloop.run();
            let _ = stream.disconnect();
            unsafe { pw::deinit() };
        })
        .map_err(|e| LoopbackError::BackendInit(format!("capture thread spawn: {e}")))?;

    // Dispatch thread: pulls frames from the channel and calls the user callback.
    let dispatch_thread = thread::Builder::new()
        .name("pks-pipewire-dispatch".into())
        .spawn(move || {
            while let Ok(frame) = frame_rx.recv() {
                callback(frame);
            }
        })
        .map_err(|e| LoopbackError::BackendInit(format!("dispatch thread spawn: {e}")))?;

    Ok(SystemLoopbackSource {
        _capture_thread: capture_thread,
        _dispatch_thread: dispatch_thread,
        stop_tx,
    })
}

// ---------------------------------------------------------------------------
// ALSA snd-aloop fallback
// ---------------------------------------------------------------------------

fn run_alsa<F>(callback: F) -> Result<SystemLoopbackSource, LoopbackError>
where
    F: Fn(AudioFrame) + Send + Sync + 'static,
{
    use alsa::pcm::{Access, Format, HwParams, PCM};
    use alsa::Direction;

    let (stop_tx, stop_rx) = mpsc::sync_channel::<()>(1);
    let pool = AudioBufferPool::new(POOL_DEPTH, CAPTURE_FRAME_SAMPLES);
    let seq = Arc::new(AtomicU64::new(0));

    let capture_thread = thread::Builder::new()
        .name("pks-alsa-capture".into())
        .spawn(move || {
            let pcm = match PCM::new(ALSA_LOOPBACK_DEVICE, Direction::Capture, false) {
                Ok(p) => p,
                Err(e) => {
                    eprintln!("pks-alsa: PCM::new failed: {e}");
                    return;
                }
            };

            {
                let hwp = match HwParams::any(&pcm) {
                    Ok(p) => p,
                    Err(e) => {
                        eprintln!("pks-alsa: HwParams::any failed: {e}");
                        return;
                    }
                };
                let _ = hwp.set_channels(CAPTURE_CHANNELS as u32);
                let _ = hwp.set_rate(DEFAULT_SAMPLE_RATE, alsa::ValueOr::Nearest);
                let _ = hwp.set_format(Format::float());
                let _ = hwp.set_access(Access::RWInterleaved);
                if let Err(e) = pcm.hw_params(&hwp) {
                    eprintln!("pks-alsa: hw_params failed: {e}");
                    return;
                }
            }

            if let Err(e) = pcm.start() {
                eprintln!("pks-alsa: pcm.start() failed: {e}");
                return;
            }

            let io = pcm.io_f32().expect("io_f32 must succeed after hw_params");
            let mut buf = vec![0f32; CAPTURE_FRAME_SAMPLES];

            loop {
                if stop_rx.try_recv().is_ok() {
                    break;
                }
                match io.readi(&mut buf) {
                    Ok(0) | Err(_) => {
                        if stop_rx.try_recv().is_ok() {
                            break;
                        }
                        continue;
                    }
                    Ok(frames_read) => {
                        let n_samples = frames_read * CAPTURE_CHANNELS as usize;
                        let mut handle = match pool.acquire() {
                            Some(h) => h,
                            None => continue,
                        };
                        let dst = handle.as_mut_slice();
                        let copy_count = n_samples.min(dst.len());
                        dst[..copy_count].copy_from_slice(&buf[..copy_count]);
                        handle.set_len(copy_count);

                        let s = seq.fetch_add(1, Ordering::Relaxed);
                        let ts_ns = std::time::SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH)
                            .unwrap_or_default()
                            .as_nanos() as u64;

                        let mut frame = AudioFrame::new(
                            StreamId(0),
                            SourceId(0),
                            s,
                            ts_ns,
                            CAPTURE_CHANNELS,
                            handle,
                        );
                        frame.source_tag = AudioSourceTag::Captured;
                        frame.encryption_mode = EncryptionMode::None;
                        frame.sample_rate = DEFAULT_SAMPLE_RATE;
                        callback(frame);
                    }
                }
            }
            let _ = pcm.drop();
        })
        .map_err(|e| LoopbackError::BackendInit(format!("alsa thread spawn: {e}")))?;

    // ALSA path calls callback inline; use a dummy dispatch thread for uniform struct layout.
    let (_dummy_tx, dummy_rx) = mpsc::sync_channel::<AudioFrame>(1);
    let dispatch_thread = thread::Builder::new()
        .name("pks-alsa-dispatch".into())
        .spawn(move || while dummy_rx.recv().is_ok() {})
        .map_err(|e| LoopbackError::BackendInit(format!("alsa dispatch spawn: {e}")))?;

    Ok(SystemLoopbackSource {
        _capture_thread: capture_thread,
        _dispatch_thread: dispatch_thread,
        stop_tx,
    })
}
