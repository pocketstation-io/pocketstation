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

use rtrb::{Producer, RingBuffer};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::mpsc;
use std::sync::{Arc, Once};
use std::thread;
use std::time::Duration;

use pipewire as pw;
use pks_frame::{
    AudioBufferHandle, AudioBufferPool, AudioFrame, AudioSourceTag, EncryptionMode, SourceId,
    StreamId, POOL_SLOT_SAMPLES, SAMPLE_RATE_HZ,
};
use pw::properties::properties;
use pw::spa;
use pw::spa::pod::Pod;
use spa::param::audio::AudioFormat;

use pks_capture::{
    monotonic_timestamp_ns, CaptureError as LoopbackError, CaptureMode, CaptureObservationCounters,
    CaptureObservations, CaptureSource,
};

/// Stereo capture.
const CAPTURE_CHANNEL_COUNT: u8 = 2;

/// Stereo 20 ms frame at 48 kHz = 960 mono samples * 2 channels.
const CAPTURE_FRAME_SAMPLES: usize = POOL_SLOT_SAMPLES * CAPTURE_CHANNEL_COUNT as usize;

/// Maximum interleaved sample count accepted from one PipeWire callback.
/// OS graph quantums are independent of PocketStation's 10/20 ms transport
/// frames; a 2048-frame stereo quantum contains 4096 samples and must reach the
/// downstream profile accumulator without truncation.
const PIPEWIRE_CALLBACK_MAX_SAMPLES: usize = 4096;

/// Pool depth: 8 frames absorb callback jitter without unbounded growth.
const CAPTURE_POOL_CAPACITY_FRAMES: usize = 8;

/// SPSC ring depth between the PW RT process callback and the dispatch thread.
const DISPATCH_QUEUE_CAPACITY_FRAMES: usize = 16;

/// PipeWire node latency: 128 frames at 48 kHz (~2.67 ms quantum).
const PW_NODE_LATENCY: &str = "128/48000";

/// Timer interval for polling the stop signal from within the PipeWire MainLoop.
const PW_STOP_POLL_MS: u64 = 50;

/// Maximum time the public constructor waits for PipeWire to confirm that the
/// stream reached a connected state.
const PW_OPEN_TIMEOUT: Duration = Duration::from_secs(5);

/// PipeWire's canonical exact-target property. `node.target` is obsolete and
/// can be ignored by current WirePlumber linking policy.
const PW_TARGET_OBJECT: &str = "target.object";

/// Exact capture must fail if the selected target disappears. Falling back to
/// a default device would silently widen an application or device selection.
const PW_NODE_DONT_FALLBACK: &str = "node.dont-fallback";

/// Internal capture streams must not inherit a previous WirePlumber stream's
/// volume, mute, or target state merely because they reuse a node name.
const PW_STATE_RESTORE_PROPS: &str = "state.restore-props";
const PW_STATE_RESTORE_TARGET: &str = "state.restore-target";

/// Exact capture stays attached to the resolved live target for its lifetime.
const PW_NODE_DONT_MOVE: &str = "node.dont-move";
const PW_NODE_DONT_RECONNECT: &str = "node.dont-reconnect";

/// ALSA loopback capture device (snd-aloop module, subdevice 0 of card 1).
/// Requires: sudo modprobe snd-aloop
const ALSA_LOOPBACK_DEVICE: &str = "hw:Loopback,1,0";

/// Return the valid F32 samples from one mapped SPA data plane. PipeWire may
/// place the chunk at a non-zero offset inside the mapped buffer.
fn pipewire_f32_plane(data: &mut spa::buffer::Data) -> Option<&[f32]> {
    let byte_offset = data.chunk().offset() as usize;
    let byte_count = data.chunk().size() as usize;
    let byte_end = byte_offset.checked_add(byte_count)?;
    let buffer = data.data()?;
    let source = buffer.get(byte_offset..byte_end)?;
    // SAFETY: all bit patterns are valid f32 values. Prefix/suffix checks below
    // reject an unaligned or partial sample without panicking.
    let (prefix, samples, suffix) = unsafe { source.align_to::<f32>() };
    if !prefix.is_empty() || !suffix.is_empty() {
        return None;
    }
    Some(samples)
}

/// Copy PipeWire audio into one interleaved pool slot. A stream may expose one
/// interleaved SPA data buffer or one data plane per channel. Both layouts are
/// legal even when the negotiated sample format is F32LE.
fn copy_pipewire_f32_samples(
    datas: &mut [spa::buffer::Data],
    destination: &mut [f32],
    channel_count: usize,
) -> usize {
    if datas.len() == 1 {
        let Some(source) = pipewire_f32_plane(&mut datas[0]) else {
            return 0;
        };
        let sample_count = source.len().min(destination.len());
        destination[..sample_count].copy_from_slice(&source[..sample_count]);
        return sample_count;
    }

    if channel_count == 2 && datas.len() >= 2 {
        let (left_data, remaining_data) = datas.split_at_mut(1);
        let Some(left) = pipewire_f32_plane(&mut left_data[0]) else {
            return 0;
        };
        let Some(right) = pipewire_f32_plane(&mut remaining_data[0]) else {
            return 0;
        };
        let frame_count = left
            .len()
            .min(right.len())
            .min(destination.len() / channel_count);
        for (destination_frame, (&left_sample, &right_sample)) in destination
            .chunks_exact_mut(channel_count)
            .zip(left.iter().zip(right.iter()))
            .take(frame_count)
        {
            destination_frame[0] = left_sample;
            destination_frame[1] = right_sample;
        }
        return frame_count * channel_count;
    }

    0
}

fn acquire_capture_buffer(
    pool: &Arc<AudioBufferPool>,
    counters: &CaptureObservationCounters,
) -> Option<AudioBufferHandle> {
    let handle = pool.acquire();
    if handle.is_none() {
        counters.observe_pool_exhaustion();
    }
    handle
}

fn enqueue_capture_frame(
    producer: &mut Producer<AudioFrame>,
    frame: AudioFrame,
    counters: &CaptureObservationCounters,
) -> bool {
    if producer.push(frame).is_err() {
        counters.observe_dispatch_queue_full();
        false
    } else {
        counters.observe_enqueued_frame();
        true
    }
}

fn source_unavailable(stable_key: &str) -> LoopbackError {
    LoopbackError::SourceUnavailable {
        stable_key: stable_key.to_owned(),
    }
}

pub struct SystemLoopbackSource {
    capture_thread: Option<thread::JoinHandle<()>>,
    dispatch_thread: Option<thread::JoinHandle<()>>,
    stop_tx: mpsc::SyncSender<()>,
    counters: CaptureObservationCounters,
}

/// Linux desktop capture dispatch. The loopback type remains semantically
/// limited to output capture while this façade also owns physical input.
pub struct DesktopCaptureSource {
    source: SystemLoopbackSource,
}

impl DesktopCaptureSource {
    pub fn capture_mode<F>(mode: CaptureMode, callback: F) -> Result<Self, LoopbackError>
    where
        F: FnMut(AudioFrame) + Send + 'static,
    {
        SystemLoopbackSource::capture_mode(mode, callback).map(|source| Self { source })
    }

    pub fn observations(&self) -> CaptureObservations {
        self.source.observations()
    }
}

impl SystemLoopbackSource {
    /// Start capturing the system-wide audio mix.
    pub fn capture<F>(callback: F) -> Result<Self, LoopbackError>
    where
        F: FnMut(AudioFrame) + Send + 'static,
    {
        Self::capture_mode(CaptureMode::SystemMix, callback)
    }

    /// Start capturing in the given mode.
    pub fn capture_mode<F>(mode: CaptureMode, callback: F) -> Result<Self, LoopbackError>
    where
        F: FnMut(AudioFrame) + Send + 'static,
    {
        match &mode {
            CaptureMode::Process(pid) => {
                if !pipewire_available() {
                    return Err(LoopbackError::ModeUnsupported(mode));
                }
                let nodes = enumerate_pipewire_nodes();
                let target_pid = *pid;
                match nodes.iter().find(|s| s.process_id == Some(target_pid)) {
                    Some(src) => run_pipewire_targeted(
                        pipewire_node_target(src)?,
                        src.stable_id.to_frame_source_id().0,
                        mode,
                        callback,
                    ),
                    None => Err(LoopbackError::BackendInit(format!(
                        "BLOCKED_WITH_EVIDENCE: PipeWire per-app source capture requires PipeWire node enumeration and link; no node found for '{target_pid}'"
                    ))),
                }
            }
            CaptureMode::ExactApplication {
                process_id,
                stable_id,
            } => {
                if !pipewire_available() {
                    return Err(LoopbackError::ModeUnsupported(mode));
                }
                let nodes = enumerate_pipewire_nodes();
                match nodes.iter().find(|source| {
                    source.process_id == Some(*process_id) && &source.stable_id == stable_id
                }) {
                    Some(source) => run_pipewire_targeted(
                        pipewire_node_target(source)?,
                        stable_id.to_frame_source_id().0,
                        mode,
                        callback,
                    ),
                    None => Err(source_unavailable(&stable_id.stable_key)),
                }
            }
            CaptureMode::ExactApplicationStable { stable_id } => {
                if !pipewire_available() {
                    return Err(LoopbackError::ModeUnsupported(mode));
                }
                let nodes = enumerate_pipewire_nodes();
                match nodes.iter().find(|source| &source.stable_id == stable_id) {
                    Some(source) => run_pipewire_targeted(
                        pipewire_node_target(source)?,
                        stable_id.to_frame_source_id().0,
                        mode,
                        callback,
                    ),
                    None => Err(source_unavailable(&stable_id.stable_key)),
                }
            }
            CaptureMode::Application(name) => {
                if !pipewire_available() {
                    return Err(LoopbackError::ModeUnsupported(mode));
                }
                let nodes = enumerate_pipewire_nodes();
                let name_lower = name.to_ascii_lowercase();
                let name_clone = name.clone();
                match nodes.iter().find(|source| {
                    source.name.to_ascii_lowercase() == name_lower
                        || source
                            .app_id
                            .as_deref()
                            .is_some_and(|app_id| app_id.eq_ignore_ascii_case(&name))
                }) {
                    Some(src) => run_pipewire_targeted(
                        pipewire_node_target(src)?,
                        src.stable_id.to_frame_source_id().0,
                        mode,
                        callback,
                    ),
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
            CaptureMode::InputDevice(selector) => {
                if !pipewire_available() {
                    return Err(LoopbackError::ModeUnsupported(mode));
                }
                let nodes = enumerate_pipewire_nodes();
                let source = match selector {
                    pks_capture::InputDeviceSelector::Default => nodes.iter().find(|source| {
                        source.stable_id.kind == pks_capture::SourceKind::InputDevice
                    }),
                    pks_capture::InputDeviceSelector::StableId(device_uid) => {
                        nodes.iter().find(|source| {
                            source.stable_id.kind == pks_capture::SourceKind::InputDevice
                                && source.device_uid.as_deref() == Some(device_uid.as_str())
                        })
                    }
                };
                match source {
                    Some(source) => run_pipewire_targeted(
                        pipewire_node_target(source)?,
                        source.stable_id.to_frame_source_id().0,
                        mode,
                        callback,
                    ),
                    None => match selector {
                        pks_capture::InputDeviceSelector::StableId(device_uid) => {
                            Err(source_unavailable(device_uid))
                        }
                        pks_capture::InputDeviceSelector::Default => {
                            Err(LoopbackError::BackendInit(
                                "requested default PipeWire input device is unavailable".to_owned(),
                            ))
                        }
                    },
                }
            }
        }
    }

    pub fn observations(&self) -> CaptureObservations {
        self.counters.snapshot()
    }
}

fn pipewire_node_target(source: &CaptureSource) -> Result<String, LoopbackError> {
    source.device_uid.clone().ok_or_else(|| {
        LoopbackError::BackendInit(format!(
            "PipeWire source '{}' has no stable node target",
            source.name
        ))
    })
}

/// Drop contract — control thread only: signal once, join both owned workers,
/// never execute from a capture callback or realtime partition.
impl Drop for SystemLoopbackSource {
    fn drop(&mut self) {
        let _ = self.stop_tx.try_send(());
        if let Some(capture_thread) = self.capture_thread.take() {
            let _ = capture_thread.join();
        }
        if let Some(dispatch_thread) = self.dispatch_thread.take() {
            let _ = dispatch_thread.join();
        }
    }
}

fn initialize_pipewire_once() {
    static PIPEWIRE_INIT: Once = Once::new();
    PIPEWIRE_INIT.call_once(pw::init);
}

/// Returns `true` if the PipeWire socket is present in `$XDG_RUNTIME_DIR`.
///
/// Uses a single `stat(2)` call -- no I/O, no allocation.
fn pipewire_available() -> bool {
    // SAFETY: getuid has no pointer arguments or ownership transfer and cannot
    // invalidate Rust memory.
    let runtime_dir = std::env::var("XDG_RUNTIME_DIR")
        .unwrap_or_else(|_| format!("/run/user/{}", unsafe { libc::getuid() }));
    std::path::Path::new(&runtime_dir)
        .join("pipewire-0")
        .exists()
}

fn run_pipewire<F>(
    _mode: CaptureMode,
    mut callback: F,
) -> Result<SystemLoopbackSource, LoopbackError>
where
    F: FnMut(AudioFrame) + Send + 'static,
{
    let (stop_tx, stop_rx) = mpsc::sync_channel::<()>(1);
    let (open_tx, open_rx) = mpsc::sync_channel::<Result<(), String>>(1);
    let (frame_producer, mut frame_consumer) =
        RingBuffer::<AudioFrame>::new(DISPATCH_QUEUE_CAPACITY_FRAMES);

    let pool = AudioBufferPool::new(CAPTURE_POOL_CAPACITY_FRAMES, PIPEWIRE_CALLBACK_MAX_SAMPLES);
    let seq = Arc::new(AtomicU64::new(0));
    let counters = CaptureObservationCounters::default();
    let capture_counters = counters.clone();

    // Capture thread: owns the PipeWire MainLoop for its lifetime.
    let capture_thread = thread::Builder::new()
        .name("pks-pipewire-capture".into())
        .spawn(move || {
            initialize_pipewire_once();
            let mainloop = match pw::main_loop::MainLoopRc::new(None) {
                Ok(ml) => ml,
                Err(e) => {
                    let _ = open_tx.send(Err(format!("PipeWire main loop: {e}")));
                    return;
                }
            };
            let context = match pw::context::ContextRc::new(&mainloop, None) {
                Ok(ctx) => ctx,
                Err(e) => {
                    let _ = open_tx.send(Err(format!("PipeWire context: {e}")));
                    return;
                }
            };
            let core = match context.connect_rc(None) {
                Ok(c) => c,
                Err(e) => {
                    let _ = open_tx.send(Err(format!("PipeWire core connection: {e}")));
                    return;
                }
            };

            // STREAM_CAPTURE_SINK captures the monitor port of the default sink
            // (system-wide output mix).
            let stream_props = properties! {
                *pw::keys::NODE_NAME => "pks-system-mix-capture",
                *pw::keys::NODE_LATENCY => PW_NODE_LATENCY,
                *pw::keys::MEDIA_TYPE => "Audio",
                *pw::keys::MEDIA_CATEGORY => "Capture",
                *pw::keys::MEDIA_ROLE => "Music",
                *pw::keys::STREAM_CAPTURE_SINK => "true",
                PW_STATE_RESTORE_PROPS => "false",
                PW_STATE_RESTORE_TARGET => "false",
            };

            let stream = match pw::stream::StreamRc::new(core, "pks-loopback", stream_props) {
                Ok(s) => s,
                Err(e) => {
                    let _ = open_tx.send(Err(format!("PipeWire stream creation: {e}")));
                    return;
                }
            };

            let mut frame_producer = frame_producer;
            let pool_cb = pool.clone();
            let seq_cb = seq.clone();
            let process_counters = capture_counters.clone();

            let state_open_tx = open_tx.clone();
            let state_counters = capture_counters.clone();
            let stream_subscription = match stream
                .add_local_listener_with_user_data(())
                .state_changed(move |_stream, _user, _old, new| match new {
                    pw::stream::StreamState::Paused | pw::stream::StreamState::Streaming => {
                        let _ = state_open_tx.try_send(Ok(()));
                    }
                    pw::stream::StreamState::Error(error) => {
                        state_counters.observe_stream_error();
                        let _ =
                            state_open_tx.try_send(Err(format!("PipeWire stream state: {error}")));
                    }
                    _ => {}
                })
                .param_changed(|_stream, _user, _id, _param| {})
                .process(move |stream, _user| {
                    let mut buf = match stream.dequeue_buffer() {
                        Some(b) => b,
                        None => {
                            process_counters.observe_invalid_buffer();
                            return;
                        }
                    };
                    process_counters.observe_callback_buffer();
                    let datas = buf.datas_mut();
                    if datas.is_empty() {
                        process_counters.observe_invalid_buffer();
                        return;
                    }
                    let mut handle = match acquire_capture_buffer(&pool_cb, &process_counters) {
                        Some(h) => h,
                        None => return,
                    };
                    let dst = handle.as_mut_slice();
                    let copy_count =
                        copy_pipewire_f32_samples(datas, dst, CAPTURE_CHANNEL_COUNT as usize);
                    if copy_count == 0 {
                        process_counters.observe_invalid_buffer();
                        return;
                    }
                    handle.set_len(copy_count);

                    let s = seq_cb.fetch_add(1, Ordering::Relaxed);
                    let ts_ns = monotonic_timestamp_ns();

                    let mut frame = AudioFrame::new(
                        StreamId(0),
                        SourceId(0),
                        s,
                        ts_ns,
                        CAPTURE_CHANNEL_COUNT,
                        handle,
                    );
                    frame.source_tag = AudioSourceTag::Captured;
                    frame.encryption_mode = EncryptionMode::None;
                    frame.sample_rate_hz = SAMPLE_RATE_HZ;

                    enqueue_capture_frame(&mut frame_producer, frame, &process_counters);
                })
                .register()
            {
                Ok(stream_subscription) => stream_subscription,
                Err(error) => {
                    let _ = open_tx.send(Err(format!("PipeWire stream subscription: {error}")));
                    return;
                }
            };

            // Build audio format params.
            let mut audio_info = spa::param::audio::AudioInfoRaw::new();
            audio_info.set_format(AudioFormat::F32LE);
            audio_info.set_rate(SAMPLE_RATE_HZ);
            audio_info.set_channels(CAPTURE_CHANNEL_COUNT as u32);
            let obj = match pw::spa::pod::serialize::PodSerializer::serialize(
                std::io::Cursor::new(Vec::new()),
                &pw::spa::pod::Value::Object(pw::spa::pod::Object {
                    type_: pw::spa::utils::SpaTypes::ObjectParamFormat.as_raw(),
                    id: pw::spa::param::ParamType::EnumFormat.as_raw(),
                    properties: audio_info.into(),
                }),
            ) {
                Ok((cursor, _)) => cursor.into_inner(),
                Err(error) => {
                    let _ = open_tx.send(Err(format!("PipeWire format serialization: {error}")));
                    return;
                }
            };
            let Some(param) = Pod::from_bytes(&obj) else {
                let _ = open_tx.send(Err("PipeWire rejected the capture format pod".to_owned()));
                return;
            };

            let flags = pw::stream::StreamFlags::AUTOCONNECT
                | pw::stream::StreamFlags::MAP_BUFFERS
                | pw::stream::StreamFlags::RT_PROCESS;

            if let Err(e) =
                stream.connect(pw::spa::utils::Direction::Input, None, flags, &mut [param])
            {
                let _ = open_tx.send(Err(format!("PipeWire stream connection: {e}")));
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
            drop(timer);
            drop(stream_subscription);
            drop(stream);
            drop(context);
            drop(mainloop);
        })
        .map_err(|e| LoopbackError::BackendInit(format!("capture thread spawn: {e}")))?;

    // Dispatch thread: polls the SPSC ring and calls the user callback.
    // Sleeps 1 ms when the ring is empty to avoid busy-waiting; exits when
    // the producer side is dropped (capture thread exited).
    let dispatch_thread = match thread::Builder::new()
        .name("pks-pipewire-dispatch".into())
        .spawn(move || loop {
            while let Ok(frame) = frame_consumer.pop() {
                callback(frame);
            }
            if frame_consumer.is_abandoned() {
                break;
            }
            thread::sleep(Duration::from_millis(1));
        }) {
        Ok(thread) => thread,
        Err(error) => {
            let _ = stop_tx.try_send(());
            let _ = capture_thread.join();
            return Err(LoopbackError::BackendInit(format!(
                "dispatch thread spawn: {error}"
            )));
        }
    };

    match open_rx.recv_timeout(PW_OPEN_TIMEOUT) {
        Ok(Ok(())) => {}
        Ok(Err(error)) => {
            let _ = stop_tx.try_send(());
            let _ = capture_thread.join();
            let _ = dispatch_thread.join();
            return Err(LoopbackError::BackendInit(error));
        }
        Err(error) => {
            let _ = stop_tx.try_send(());
            let _ = capture_thread.join();
            let _ = dispatch_thread.join();
            return Err(LoopbackError::BackendInit(format!(
                "PipeWire stream did not open within {} ms: {error}",
                PW_OPEN_TIMEOUT.as_millis()
            )));
        }
    }

    Ok(SystemLoopbackSource {
        capture_thread: Some(capture_thread),
        dispatch_thread: Some(dispatch_thread),
        stop_tx,
        counters,
    })
}

/// Enumerate all audio capture sources visible on this Linux system.
///
/// Always returns at least one entry (the system-wide mix at id=0).
/// If PipeWire is available, per-application node sources are appended.
pub fn discover_sources_linux() -> Vec<pks_capture::CaptureSource> {
    use pks_capture::{CaptureSource, SourceKind, SourceState, StableSourceId};
    use pks_frame::Platform;

    let system_mix = CaptureSource {
        stable_id: StableSourceId::new(Platform::Linux, SourceKind::SystemMix, "system:mix"),
        name: "System Mix".to_owned(),
        process_id: None,
        app_id: None,
        device_uid: None,
        state: SourceState::Available,
        sample_rate_hz: 48_000,
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
fn enumerate_pipewire_nodes() -> Vec<pks_capture::CaptureSource> {
    use pks_capture::{CaptureSource, SourceKind, SourceState, StableSourceId};
    use pks_frame::Platform;
    use std::cell::RefCell;
    use std::rc::Rc;
    use std::sync::mpsc as smpsc;

    let (tx, rx) = smpsc::channel::<Vec<CaptureSource>>();

    let join = thread::Builder::new()
        .name("pks-pw-discovery".into())
        .spawn(move || {
            initialize_pipewire_once();

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
                    let app_id = (kind == SourceKind::Application)
                        .then(|| node_name.clone())
                        .flatten();
                    // An application node name is stable enough for public
                    // identity but not for an exact live link. WirePlumber can
                    // retain route state for a reused name across rapid
                    // capture restarts. Resolve applications to the current
                    // PipeWire object serial while retaining node names as
                    // device identities for physical input/output endpoints.
                    let node_target = if kind == SourceKind::Application {
                        props
                            .get("object.serial")
                            .map(str::to_owned)
                            .or_else(|| node_name.clone())
                    } else {
                        node_name.clone()
                    };
                    collected_for_reg.borrow_mut().push(CaptureSource {
                        stable_id: StableSourceId::new(Platform::Linux, kind, stable_key),
                        name,
                        process_id: pid,
                        app_id,
                        device_uid: node_target,
                        state: SourceState::Available,
                        sample_rate_hz: 48_000,
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
        });

    let Ok(join) = join else { return Vec::new() };
    let sources = rx
        .recv_timeout(Duration::from_millis(400))
        .unwrap_or_default();
    let _ = join.join();
    sources
}

/// Like `run_pipewire` but routes the capture stream to a specific PipeWire
/// object instead of the default sink monitor.
fn run_pipewire_targeted<F>(
    node_target: String,
    source_id: u64,
    _mode: CaptureMode,
    mut callback: F,
) -> Result<SystemLoopbackSource, LoopbackError>
where
    F: FnMut(AudioFrame) + Send + 'static,
{
    let (stop_tx, stop_rx) = mpsc::sync_channel::<()>(1);
    let (open_tx, open_rx) = mpsc::sync_channel::<Result<(), String>>(1);
    let (frame_producer, mut frame_consumer) =
        RingBuffer::<AudioFrame>::new(DISPATCH_QUEUE_CAPACITY_FRAMES);

    let pool = AudioBufferPool::new(CAPTURE_POOL_CAPACITY_FRAMES, PIPEWIRE_CALLBACK_MAX_SAMPLES);
    let seq = Arc::new(AtomicU64::new(0));
    let counters = CaptureObservationCounters::default();
    let capture_counters = counters.clone();

    let capture_thread = thread::Builder::new()
        .name("pks-pipewire-capture-targeted".into())
        .spawn(move || {
            initialize_pipewire_once();
            let mainloop = match pw::main_loop::MainLoopRc::new(None) {
                Ok(ml) => ml,
                Err(e) => {
                    let _ = open_tx.send(Err(format!("PipeWire main loop: {e}")));
                    return;
                }
            };
            let context = match pw::context::ContextRc::new(&mainloop, None) {
                Ok(ctx) => ctx,
                Err(e) => {
                    let _ = open_tx.send(Err(format!("PipeWire context: {e}")));
                    return;
                }
            };
            let core = match context.connect_rc(None) {
                Ok(c) => c,
                Err(e) => {
                    let _ = open_tx.send(Err(format!("PipeWire core connection: {e}")));
                    return;
                }
            };

            // Route to a specific node rather than the default sink monitor.
            // STREAM_CAPTURE_SINK is intentionally omitted.
            let stream_props = properties! {
                *pw::keys::NODE_NAME => "pks-targeted-capture",
                *pw::keys::MEDIA_TYPE => "Audio",
                *pw::keys::MEDIA_CATEGORY => "Capture",
                *pw::keys::MEDIA_ROLE => "Music",
                PW_TARGET_OBJECT => node_target.as_str(),
                PW_NODE_DONT_FALLBACK => "true",
                PW_NODE_DONT_MOVE => "true",
                PW_NODE_DONT_RECONNECT => "true",
                PW_STATE_RESTORE_PROPS => "false",
                PW_STATE_RESTORE_TARGET => "false",
            };

            let stream =
                match pw::stream::StreamRc::new(core, "pks-loopback-targeted", stream_props) {
                    Ok(s) => s,
                    Err(e) => {
                        let _ = open_tx.send(Err(format!("PipeWire stream creation: {e}")));
                        return;
                    }
                };

            let mut frame_producer = frame_producer;
            let pool_cb = pool.clone();
            let seq_cb = seq.clone();
            let process_counters = capture_counters.clone();

            let state_open_tx = open_tx.clone();
            let state_counters = capture_counters.clone();
            let stream_subscription = match stream
                .add_local_listener_with_user_data(())
                .state_changed(move |_stream, _user, _old, new| match new {
                    pw::stream::StreamState::Paused | pw::stream::StreamState::Streaming => {
                        let _ = state_open_tx.try_send(Ok(()));
                    }
                    pw::stream::StreamState::Error(error) => {
                        state_counters.observe_stream_error();
                        let _ =
                            state_open_tx.try_send(Err(format!("PipeWire stream state: {error}")));
                    }
                    _ => {}
                })
                .param_changed(|_stream, _user, _id, _param| {})
                .process(move |stream, _user| {
                    let mut buf = match stream.dequeue_buffer() {
                        Some(b) => b,
                        None => {
                            process_counters.observe_invalid_buffer();
                            return;
                        }
                    };
                    process_counters.observe_callback_buffer();
                    let datas = buf.datas_mut();
                    if datas.is_empty() {
                        process_counters.observe_invalid_buffer();
                        return;
                    }
                    let mut handle = match acquire_capture_buffer(&pool_cb, &process_counters) {
                        Some(h) => h,
                        None => return,
                    };
                    let dst = handle.as_mut_slice();
                    let copy_count =
                        copy_pipewire_f32_samples(datas, dst, CAPTURE_CHANNEL_COUNT as usize);
                    if copy_count == 0 {
                        process_counters.observe_invalid_buffer();
                        return;
                    }
                    handle.set_len(copy_count);

                    let s = seq_cb.fetch_add(1, Ordering::Relaxed);
                    let ts_ns = monotonic_timestamp_ns();

                    let mut frame = AudioFrame::new(
                        StreamId(0),
                        SourceId(source_id),
                        s,
                        ts_ns,
                        CAPTURE_CHANNEL_COUNT,
                        handle,
                    );
                    frame.source_tag = AudioSourceTag::Captured;
                    frame.encryption_mode = EncryptionMode::None;
                    frame.sample_rate_hz = SAMPLE_RATE_HZ;

                    enqueue_capture_frame(&mut frame_producer, frame, &process_counters);
                })
                .register()
            {
                Ok(stream_subscription) => stream_subscription,
                Err(error) => {
                    let _ = open_tx.send(Err(format!("PipeWire stream subscription: {error}")));
                    return;
                }
            };

            let mut audio_info = spa::param::audio::AudioInfoRaw::new();
            audio_info.set_format(AudioFormat::F32LE);
            audio_info.set_rate(SAMPLE_RATE_HZ);
            audio_info.set_channels(CAPTURE_CHANNEL_COUNT as u32);
            let obj = match pw::spa::pod::serialize::PodSerializer::serialize(
                std::io::Cursor::new(Vec::new()),
                &pw::spa::pod::Value::Object(pw::spa::pod::Object {
                    type_: pw::spa::utils::SpaTypes::ObjectParamFormat.as_raw(),
                    id: pw::spa::param::ParamType::EnumFormat.as_raw(),
                    properties: audio_info.into(),
                }),
            ) {
                Ok((cursor, _)) => cursor.into_inner(),
                Err(error) => {
                    let _ = open_tx.send(Err(format!("PipeWire format serialization: {error}")));
                    return;
                }
            };
            let Some(param) = Pod::from_bytes(&obj) else {
                let _ = open_tx.send(Err("PipeWire rejected the capture format pod".to_owned()));
                return;
            };

            let flags = pw::stream::StreamFlags::AUTOCONNECT
                | pw::stream::StreamFlags::MAP_BUFFERS
                | pw::stream::StreamFlags::RT_PROCESS;

            if let Err(e) =
                stream.connect(pw::spa::utils::Direction::Input, None, flags, &mut [param])
            {
                let _ = open_tx.send(Err(format!("PipeWire stream connection: {e}")));
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
            drop(timer);
            drop(stream_subscription);
            drop(stream);
            drop(context);
            drop(mainloop);
        })
        .map_err(|e| LoopbackError::BackendInit(format!("capture thread spawn: {e}")))?;

    let dispatch_thread = match thread::Builder::new()
        .name("pks-pipewire-dispatch-targeted".into())
        .spawn(move || loop {
            while let Ok(frame) = frame_consumer.pop() {
                callback(frame);
            }
            if frame_consumer.is_abandoned() {
                break;
            }
            thread::sleep(Duration::from_millis(1));
        }) {
        Ok(thread) => thread,
        Err(error) => {
            let _ = stop_tx.try_send(());
            let _ = capture_thread.join();
            return Err(LoopbackError::BackendInit(format!(
                "dispatch thread spawn: {error}"
            )));
        }
    };

    match open_rx.recv_timeout(PW_OPEN_TIMEOUT) {
        Ok(Ok(())) => {}
        Ok(Err(error)) => {
            let _ = stop_tx.try_send(());
            let _ = capture_thread.join();
            let _ = dispatch_thread.join();
            return Err(LoopbackError::BackendInit(error));
        }
        Err(error) => {
            let _ = stop_tx.try_send(());
            let _ = capture_thread.join();
            let _ = dispatch_thread.join();
            return Err(LoopbackError::BackendInit(format!(
                "PipeWire stream did not open within {} ms: {error}",
                PW_OPEN_TIMEOUT.as_millis()
            )));
        }
    }

    Ok(SystemLoopbackSource {
        capture_thread: Some(capture_thread),
        dispatch_thread: Some(dispatch_thread),
        stop_tx,
        counters,
    })
}

fn run_alsa<F>(mut callback: F) -> Result<SystemLoopbackSource, LoopbackError>
where
    F: FnMut(AudioFrame) + Send + 'static,
{
    use alsa::pcm::{Access, Format, HwParams, PCM};
    use alsa::Direction;

    let (stop_tx, stop_rx) = mpsc::sync_channel::<()>(1);
    let (open_tx, open_rx) = mpsc::sync_channel::<Result<(), String>>(1);
    let pool = AudioBufferPool::new(CAPTURE_POOL_CAPACITY_FRAMES, CAPTURE_FRAME_SAMPLES);
    let seq = Arc::new(AtomicU64::new(0));
    let counters = CaptureObservationCounters::default();
    let capture_counters = counters.clone();

    let capture_thread = thread::Builder::new()
        .name("pks-alsa-capture".into())
        .spawn(move || {
            let pcm = match PCM::new(ALSA_LOOPBACK_DEVICE, Direction::Capture, true) {
                Ok(p) => p,
                Err(e) => {
                    let _ = open_tx.send(Err(format!("ALSA loopback open: {e}")));
                    return;
                }
            };

            {
                let hwp = match HwParams::any(&pcm) {
                    Ok(p) => p,
                    Err(e) => {
                        let _ = open_tx.send(Err(format!("ALSA hardware parameters: {e}")));
                        return;
                    }
                };
                let _ = hwp.set_channels(CAPTURE_CHANNEL_COUNT as u32);
                let _ = hwp.set_rate(SAMPLE_RATE_HZ, alsa::ValueOr::Nearest);
                let _ = hwp.set_format(Format::float());
                let _ = hwp.set_access(Access::RWInterleaved);
                if let Err(e) = pcm.hw_params(&hwp) {
                    let _ = open_tx.send(Err(format!("ALSA apply hardware parameters: {e}")));
                    return;
                }
            }

            if let Err(e) = pcm.start() {
                let _ = open_tx.send(Err(format!("ALSA capture start: {e}")));
                return;
            }

            let io = match pcm.io_f32() {
                Ok(io) => io,
                Err(error) => {
                    let _ = open_tx.send(Err(format!("ALSA f32 capture access: {error}")));
                    return;
                }
            };
            let _ = open_tx.send(Ok(()));
            let mut buf = vec![0f32; CAPTURE_FRAME_SAMPLES];

            loop {
                if stop_rx.try_recv().is_ok() {
                    break;
                }
                match io.readi(&mut buf) {
                    Ok(0) => {
                        if stop_rx.try_recv().is_ok() {
                            break;
                        }
                        thread::sleep(Duration::from_millis(1));
                        continue;
                    }
                    Err(_) => {
                        capture_counters.observe_stream_error();
                        if stop_rx.try_recv().is_ok() {
                            break;
                        }
                        thread::sleep(Duration::from_millis(1));
                        continue;
                    }
                    Ok(frames_read) => {
                        capture_counters.observe_callback_buffer();
                        let sample_count = frames_read * CAPTURE_CHANNEL_COUNT as usize;
                        let mut handle = match acquire_capture_buffer(&pool, &capture_counters) {
                            Some(h) => h,
                            None => continue,
                        };
                        let dst = handle.as_mut_slice();
                        if sample_count > dst.len() {
                            capture_counters.observe_oversized_buffer();
                            continue;
                        }
                        dst[..sample_count].copy_from_slice(&buf[..sample_count]);
                        handle.set_len(sample_count);

                        let s = seq.fetch_add(1, Ordering::Relaxed);
                        let ts_ns = monotonic_timestamp_ns();

                        let mut frame = AudioFrame::new(
                            StreamId(0),
                            SourceId(0),
                            s,
                            ts_ns,
                            CAPTURE_CHANNEL_COUNT,
                            handle,
                        );
                        frame.source_tag = AudioSourceTag::Captured;
                        frame.encryption_mode = EncryptionMode::None;
                        frame.sample_rate_hz = SAMPLE_RATE_HZ;
                        capture_counters.observe_enqueued_frame();
                        callback(frame);
                    }
                }
            }
            let _ = pcm.drop();
        })
        .map_err(|e| LoopbackError::BackendInit(format!("alsa thread spawn: {e}")))?;

    // ALSA path calls callback inline; use a dummy dispatch thread for uniform struct layout.
    let (_dummy_tx, dummy_rx) = mpsc::sync_channel::<AudioFrame>(1);
    let dispatch_thread = match thread::Builder::new()
        .name("pks-alsa-dispatch".into())
        .spawn(move || while dummy_rx.recv().is_ok() {})
    {
        Ok(thread) => thread,
        Err(error) => {
            let _ = stop_tx.try_send(());
            let _ = capture_thread.join();
            return Err(LoopbackError::BackendInit(format!(
                "alsa dispatch spawn: {error}"
            )));
        }
    };

    match open_rx.recv_timeout(PW_OPEN_TIMEOUT) {
        Ok(Ok(())) => {}
        Ok(Err(error)) => {
            let _ = stop_tx.try_send(());
            let _ = capture_thread.join();
            let _ = dispatch_thread.join();
            return Err(LoopbackError::BackendInit(error));
        }
        Err(error) => {
            let _ = stop_tx.try_send(());
            let _ = capture_thread.join();
            let _ = dispatch_thread.join();
            return Err(LoopbackError::BackendInit(format!(
                "ALSA loopback did not open within {} ms: {error}",
                PW_OPEN_TIMEOUT.as_millis()
            )));
        }
    }

    Ok(SystemLoopbackSource {
        capture_thread: Some(capture_thread),
        dispatch_thread: Some(dispatch_thread),
        stop_tx,
        counters,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use pks_capture::{CaptureError, SourceKind, StableSourceId};
    use pks_frame::Platform;

    fn frame_from_pool(pool: &Arc<AudioBufferPool>, sequence: u64) -> AudioFrame {
        let handle = pool.acquire().expect("test pool must have a free slot");
        AudioFrame::new(StreamId(7), SourceId(11), sequence, 13, 1, handle)
    }

    #[test]
    fn given_exhausted_capture_pool_when_acquiring_then_failure_is_observed_once() {
        let pool = AudioBufferPool::new(1, 1);
        let counters = CaptureObservationCounters::default();
        let held = acquire_capture_buffer(&pool, &counters)
            .expect("first acquisition must reserve the only slot");

        assert!(acquire_capture_buffer(&pool, &counters).is_none());
        assert_eq!(counters.snapshot().pool_exhausted_total, 1);
        assert_eq!(pool.acquire_failures(), 1);

        drop(held);
        assert_eq!(pool.available_slots(), 1);
    }

    #[test]
    fn given_full_dispatch_ring_when_producer_pushes_then_failure_is_observed_once() {
        let pool = AudioBufferPool::new(2, 1);
        let counters = CaptureObservationCounters::default();
        let (mut producer, mut consumer) = RingBuffer::new(1);

        assert!(enqueue_capture_frame(
            &mut producer,
            frame_from_pool(&pool, 0),
            &counters,
        ));
        assert!(!enqueue_capture_frame(
            &mut producer,
            frame_from_pool(&pool, 1),
            &counters,
        ));

        let observations = counters.snapshot();
        assert_eq!(observations.frames_enqueued_total, 1);
        assert_eq!(observations.dispatch_queue_full_total, 1);
        drop(consumer.pop().expect("first frame must remain queued"));
        assert_eq!(pool.available_slots(), 2);
    }

    #[test]
    fn given_missing_exact_source_when_classified_then_stable_key_is_preserved() {
        let stable_id =
            StableSourceId::new(Platform::Linux, SourceKind::Application, "pw-node:meeting");

        assert_eq!(
            source_unavailable(&stable_id.stable_key),
            CaptureError::SourceUnavailable {
                stable_key: "pw-node:meeting".to_owned(),
            }
        );
    }

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

    #[test]
    fn given_exact_stable_application_when_pipewire_unavailable_then_mode_is_not_weakened() {
        if pipewire_available() {
            return;
        }
        let stable_id =
            StableSourceId::new(Platform::Linux, SourceKind::Application, "pw-node:meeting");
        let result = SystemLoopbackSource::capture_mode(
            CaptureMode::ExactApplicationStable { stable_id },
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
            Err(CaptureError::InvalidStreamCapacity) => {
                panic!("capture mode does not configure stream capacity")
            }
            Err(error) => panic!("unexpected error for nonexistent PID 0: {error}"),
            Ok(_) => panic!("expected Err for nonexistent PID 0, got Ok"),
        }
    }
}
