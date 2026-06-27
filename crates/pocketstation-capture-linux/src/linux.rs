//! Linux system audio loopback via PipeWire (primary) with snd-aloop ALSA fallback.
//!
//! ## PipeWire path
//!
//! A dedicated OS thread named `"pks-pipewire-capture"` owns the PipeWire
//! `MainLoop` for its entire lifetime.  PipeWire objects are not `Send`/`Sync`,
//! so all creation happens inside that thread.
//!
//! Audio data flows from the PipeWire RT process callback through a lock-free
//! `rtrb::RingBuffer` to a dispatcher thread (`"pks-pipewire-dispatch"`),
//! which invokes the user callback without holding any PipeWire lock.
//!
//! ## Hot-path rule (RT callback — strictly enforced)
//!
//! The PipeWire process callback does only lock-free work:
//! - `AudioBufferPool::acquire()` — lock-free CAS bitset
//! - byte copy from PipeWire buffer into pool slot (ptr::copy_nonoverlapping)
//! - `rtrb::Producer::push()` — wait-free SPSC push, drops frame when ring is full

use rtrb::RingBuffer;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::mpsc;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use pipewire as pw;
use pocketstation_frame::{
    AudioBufferPool, AudioFrame, AudioSourceTag, EncryptionMode, SourceId, StreamId,
    POOL_SLOT_SAMPLES, SAMPLE_RATE_HZ,
};
use pw::properties::properties;
use pw::spa;
use pw::spa::pod::Pod;
use spa::param::audio::AudioFormat;

use pocketstation_capture::{CaptureError as LoopbackError, CaptureMode};

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

/// Stereo capture.
const CAPTURE_CHANNELS: u8 = 2;

/// Stereo 20 ms frame at 48 kHz = 960 mono samples * 2 channels.
const CAPTURE_FRAME_SAMPLES: usize = POOL_SLOT_SAMPLES * CAPTURE_CHANNELS as usize;

/// Pool depth: 8 frames absorb callback jitter without unbounded growth.
const POOL_DEPTH: usize = 8;

/// SPSC ring depth between the PW RT process callback and the dispatch thread.
const PW_RING_DEPTH: usize = 16;

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
            CaptureMode::Process(pid) => {
                if !pipewire_available() {
                    return Err(LoopbackError::ModeUnsupported(mode));
                }
                let nodes = enumerate_pipewire_nodes();
                let target_pid = *pid;
                match nodes.iter().find(|s| s.process_id == Some(target_pid)) {
                    Some(src) => run_pipewire_targeted(src.stable_id.to_frame_source_id().0, mode, callback),
                    None => Err(LoopbackError::BackendInit(format!(
                        "BLOCKED_WITH_EVIDENCE: PipeWire per-app source capture requires PipeWire node enumeration and link; no node found for '{target_pid}'"
                    ))),
                }
            }
            CaptureMode::Application(name) => {
                if !pipewire_available() {
                    return Err(LoopbackError::ModeUnsupported(mode));
                }
                let nodes = enumerate_pipewire_nodes();
                let name_lower = name.to_ascii_lowercase();
                let name_clone = name.clone();
                match nodes.iter().find(|s| s.name.to_ascii_lowercase() == name_lower) {
                    Some(src) => run_pipewire_targeted(src.stable_id.to_frame_source_id().0, mode, callback),
                    None => Err(LoopbackError::BackendInit(format!(
                        "BLOCKED_WITH_EVIDENCE: PipeWire per-app source capture requires PipeWire node enumeration and link; no node found for '{name_clone}'"
                    ))),
                }
            }
            CaptureMode::SystemMix => {
                if pipewire_available() {
                    run_pipewire(mode, callback)
                } else {
                    run_alsa(callback)
                }
            }
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
    let (frame_producer, mut frame_consumer) = RingBuffer::<AudioFrame>::new(PW_RING_DEPTH);

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

            let mut frame_producer = frame_producer;
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
                    frame.sample_rate = SAMPLE_RATE_HZ;

                    // RT-safe: wait-free push; drops frame silently when ring is full.
                    let _ = frame_producer.push(frame);
                })
                .register()
                .expect("listener registration must not fail");

            // Build audio format params.
            let mut audio_info = spa::param::audio::AudioInfoRaw::new();
            audio_info.set_format(AudioFormat::F32LE);
            audio_info.set_rate(SAMPLE_RATE_HZ);
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

    // Dispatch thread: polls the SPSC ring and calls the user callback.
    // Sleeps 1 ms when the ring is empty to avoid busy-waiting; exits when
    // the producer side is dropped (capture thread exited).
    let dispatch_thread = thread::Builder::new()
        .name("pks-pipewire-dispatch".into())
        .spawn(move || loop {
            while let Ok(frame) = frame_consumer.pop() {
                callback(frame);
            }
            if frame_consumer.is_abandoned() {
                break;
            }
            thread::sleep(Duration::from_millis(1));
        })
        .map_err(|e| LoopbackError::BackendInit(format!("dispatch thread spawn: {e}")))?;

    Ok(SystemLoopbackSource {
        _capture_thread: capture_thread,
        _dispatch_thread: dispatch_thread,
        stop_tx,
    })
}

// ---------------------------------------------------------------------------
// Source discovery
// ---------------------------------------------------------------------------

/// Enumerate all audio capture sources visible on this Linux system.
///
/// Always returns at least one entry (the system-wide mix at id=0).
/// If PipeWire is available, per-application node sources are appended.
pub fn discover_sources_linux() -> Vec<pocketstation_capture::CaptureSource> {
    use pocketstation_capture::{CaptureSource, SourceKind, SourceState, StableSourceId};
    use pocketstation_frame::Platform;

    let system_mix = CaptureSource {
        stable_id: StableSourceId::new(Platform::Linux, SourceKind::SystemMix, "system:mix"),
        name: "System Mix".to_owned(),
        process_id: None,
        app_id: None,
        device_uid: None,
        state: SourceState::Available,
        sample_rate: 48_000,
        channels: 2,
    };

    let mut sources = vec![system_mix];
    if pipewire_available() {
        sources.extend(enumerate_pipewire_nodes());
    }
    sources
}

/// Collect PipeWire audio nodes via the registry API.
///
/// Spawns a thread, connects to PipeWire, subscribes to registry globals,
/// and waits for the initial round-trip to complete (or 300 ms timeout).
/// Returns an empty `Vec` on any error.
fn enumerate_pipewire_nodes() -> Vec<pocketstation_capture::CaptureSource> {
    use pocketstation_capture::{CaptureSource, SourceKind, SourceState, StableSourceId};
    use pocketstation_frame::Platform;
    use std::cell::RefCell;
    use std::rc::Rc;
    use std::sync::mpsc as smpsc;

    let (tx, rx) = smpsc::channel::<Vec<CaptureSource>>();

    let join = thread::Builder::new()
        .name("pks-pw-discovery".into())
        .spawn(move || {
            pw::init();

            let mainloop = match pw::main_loop::MainLoopRc::new(None) {
                Ok(ml) => ml,
                Err(_) => {
                    let _ = tx.send(Vec::new());
                    return;
                }
            };
            let context = match pw::context::ContextRc::new(&mainloop, None) {
                Ok(ctx) => ctx,
                Err(_) => {
                    let _ = tx.send(Vec::new());
                    return;
                }
            };
            let core = match context.connect_rc(None) {
                Ok(c) => c,
                Err(_) => {
                    let _ = tx.send(Vec::new());
                    return;
                }
            };

            let registry = match core.get_registry() {
                Ok(r) => r,
                Err(_) => {
                    let _ = tx.send(Vec::new());
                    return;
                }
            };

            let collected: Rc<RefCell<Vec<CaptureSource>>> = Rc::new(RefCell::new(Vec::new()));
            let collected_for_reg = collected.clone();

            let _reg_listener = registry
                .add_listener_local()
                .global(move |global| {
                    let props = match global.props {
                        Some(ref p) => p,
                        None => return,
                    };
                    let media_class = props.get("media.class").unwrap_or("");
                    let (kind, is_audio) = match media_class {
                        "Stream/Output/Audio" => (SourceKind::Application, true),
                        "Audio/Source" => (SourceKind::InputDevice, true),
                        "Audio/Sink" => (SourceKind::OutputDevice, true),
                        _ => (SourceKind::SystemMix, false),
                    };
                    if !is_audio {
                        return;
                    }

                    let name = props
                        .get("application.name")
                        .or_else(|| props.get("node.name"))
                        .unwrap_or("unknown")
                        .to_owned();
                    let node_name = props.get("node.name").map(str::to_owned);
                    let pid = props
                        .get("application.process.id")
                        .and_then(|s| s.parse::<u32>().ok());

                    let stable_key = node_name
                        .as_deref()
                        .map(|n| format!("pw-node:{n}"))
                        .unwrap_or_else(|| format!("pw-id:{}", global.id));
                    collected_for_reg.borrow_mut().push(CaptureSource {
                        stable_id: StableSourceId::new(Platform::Linux, kind, stable_key),
                        name,
                        process_id: pid,
                        app_id: None,
                        device_uid: node_name,
                        state: SourceState::Available,
                        sample_rate: 48_000,
                        channels: 2,
                    });
                })
                .register();

            // Request a round-trip sync so we know when all initial globals have arrived.
            let seq = match core.sync(0) {
                Ok(s) => s,
                Err(_) => {
                    let _ = tx.send(Vec::new());
                    return;
                }
            };

            let ml_for_done = mainloop.downgrade();
            let collected_for_done = collected.clone();
            let tx_clone = tx.clone();

            let _core_listener = core
                .add_listener_local()
                .done(move |id, done_seq| {
                    if id == 0 && done_seq == seq {
                        let sources = collected_for_done.borrow().clone();
                        let _ = tx_clone.send(sources);
                        if let Some(ml) = ml_for_done.upgrade() {
                            ml.quit();
                        }
                    }
                })
                .register();

            // Safety valve: quit after 300 ms even if done never fires.
            let ml_timer = mainloop.downgrade();
            let tx_timer = tx.clone();
            let collected_timer = collected.clone();
            let timer = mainloop.loop_().add_timer(move |_| {
                let sources = collected_timer.borrow().clone();
                let _ = tx_timer.send(sources);
                if let Some(ml) = ml_timer.upgrade() {
                    ml.quit();
                }
            });
            timer.update_timer(Some(Duration::from_millis(300)), None);

            mainloop.run();
            unsafe { pw::deinit() };
        });

    match join {
        Ok(_) => {}
        Err(_) => return Vec::new(),
    }

    rx.recv_timeout(Duration::from_millis(400))
        .unwrap_or_default()
}

// ---------------------------------------------------------------------------
// PipeWire targeted (per-node) capture
// ---------------------------------------------------------------------------

/// Like `run_pipewire` but routes the capture stream to a specific PipeWire
/// node identified by `node_id` instead of the default sink monitor.
fn run_pipewire_targeted<F>(
    node_id: u64,
    _mode: CaptureMode,
    callback: F,
) -> Result<SystemLoopbackSource, LoopbackError>
where
    F: Fn(AudioFrame) + Send + Sync + 'static,
{
    let (stop_tx, stop_rx) = mpsc::sync_channel::<()>(1);
    let (frame_producer, mut frame_consumer) = RingBuffer::<AudioFrame>::new(PW_RING_DEPTH);

    let pool = AudioBufferPool::new(POOL_DEPTH, CAPTURE_FRAME_SAMPLES);
    let seq = Arc::new(AtomicU64::new(0));

    let capture_thread = thread::Builder::new()
        .name("pks-pipewire-capture-targeted".into())
        .spawn(move || {
            pw::init();
            let mainloop = match pw::main_loop::MainLoopRc::new(None) {
                Ok(ml) => ml,
                Err(e) => {
                    eprintln!("pks-pipewire-targeted: MainLoop::new failed: {e}");
                    return;
                }
            };
            let context = match pw::context::ContextRc::new(&mainloop, None) {
                Ok(ctx) => ctx,
                Err(e) => {
                    eprintln!("pks-pipewire-targeted: Context::new failed: {e}");
                    return;
                }
            };
            let core = match context.connect_rc(None) {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("pks-pipewire-targeted: Context::connect failed: {e}");
                    return;
                }
            };

            let node_id_str = node_id.to_string();
            // Route to a specific node rather than the default sink monitor.
            // STREAM_CAPTURE_SINK is intentionally omitted.
            let stream_props = properties! {
                *pw::keys::NODE_NAME => "pks-loopback-capture",
                *pw::keys::NODE_LATENCY => PW_NODE_LATENCY,
                "node.target" => node_id_str.as_str(),
            };

            let stream =
                match pw::stream::StreamRc::new(core, "pks-loopback-targeted", stream_props) {
                    Ok(s) => s,
                    Err(e) => {
                        eprintln!("pks-pipewire-targeted: Stream::new failed: {e}");
                        return;
                    }
                };

            let mut frame_producer = frame_producer;
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
                    frame.sample_rate = SAMPLE_RATE_HZ;

                    // RT-safe: wait-free push; drops frame silently when ring is full.
                    let _ = frame_producer.push(frame);
                })
                .register()
                .expect("listener registration must not fail");

            let mut audio_info = spa::param::audio::AudioInfoRaw::new();
            audio_info.set_format(AudioFormat::F32LE);
            audio_info.set_rate(SAMPLE_RATE_HZ);
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
                eprintln!("pks-pipewire-targeted: stream.connect failed: {e}");
                return;
            }

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

    let dispatch_thread = thread::Builder::new()
        .name("pks-pipewire-dispatch-targeted".into())
        .spawn(move || loop {
            while let Ok(frame) = frame_consumer.pop() {
                callback(frame);
            }
            if frame_consumer.is_abandoned() {
                break;
            }
            thread::sleep(Duration::from_millis(1));
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
                let _ = hwp.set_rate(SAMPLE_RATE_HZ, alsa::ValueOr::Nearest);
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
                        frame.sample_rate = SAMPLE_RATE_HZ;
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

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use pocketstation_capture::CaptureError;

    /// P4 no-fallback: Process(pid) on a system without PipeWire must return
    /// ModeUnsupported, not silently fall back to the system mix.
    #[test]
    fn given_process_mode_when_pipewire_unavailable_then_mode_unsupported_not_system_mix() {
        if pipewire_available() {
            return; // test requires PipeWire to be absent
        }
        let result = SystemLoopbackSource::capture_mode(CaptureMode::Process(99999), |_| {});
        match result {
            Err(CaptureError::ModeUnsupported(_)) => {}
            _ => panic!("expected Err(ModeUnsupported)"),
        }
    }

    /// P4 no-fallback: Application(name) on a system without PipeWire must return
    /// ModeUnsupported, not silently fall back to the system mix.
    #[test]
    fn given_application_mode_when_pipewire_unavailable_then_mode_unsupported_not_system_mix() {
        if pipewire_available() {
            return; // test requires PipeWire to be absent
        }
        let result = SystemLoopbackSource::capture_mode(
            CaptureMode::Application("some-app".to_owned()),
            |_| {},
        );
        match result {
            Err(CaptureError::ModeUnsupported(_)) => {}
            _ => panic!("expected Err(ModeUnsupported)"),
        }
    }

    /// P4 no-fallback: Process(pid) when PipeWire is present but node not found
    /// must return a BackendInit error, not fall back to SystemMix.
    #[test]
    fn given_process_mode_when_node_not_found_then_backend_init_error_not_system_mix() {
        if !pipewire_available() {
            return; // test requires PipeWire
        }
        // PID 0 is the idle process and will never appear as an audio node.
        let result = SystemLoopbackSource::capture_mode(CaptureMode::Process(0), |_| {});
        match result {
            Err(CaptureError::ModeUnsupported(_))
            | Err(CaptureError::BackendInit(_))
            | Err(CaptureError::NotSupported) => {}
            Ok(_) => panic!("expected Err for nonexistent PID 0, got Ok"),
        }
    }
}
