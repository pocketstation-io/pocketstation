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
use std::num::NonZeroU32;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::mpsc;
use std::sync::{Arc, Once};
use std::thread;
use std::time::Duration;

use crate::frame::{
    AudioBufferHandle, AudioBufferPool, AudioFrame, AudioFrameDuration, Platform, SourceId,
    StreamId, SAMPLE_RATE_HZ,
};
use pipewire as pw;
use pw::properties::properties;
use pw::spa;
use pw::spa::pod::Pod;
use spa::param::audio::AudioFormat;

use crate::capture::frame_normalizer::CaptureFrameNormalizer;
use crate::capture::{
    initialize_monotonic_timestamp_domain, monotonic_timestamp_ns, CaptureError as LoopbackError,
    CaptureMode, CaptureObservationCounters, CaptureObservationHandle, CaptureObservations,
    CaptureSampleTimeline, CaptureSource, SourceKind, SourceState, StableSourceId,
};

/// Stereo application/system capture.
const CAPTURE_CHANNEL_COUNT: u8 = 2;
const MICROPHONE_CHANNEL_COUNT: u8 = 1;

/// Maximum interleaved sample count accepted from one PipeWire callback.
/// PipeWire's default graph quantum limit is 8192 sample frames. The callback
/// scratch space is fixed at setup and normalizes that input into PocketStation
/// 20 ms frames without allocating on the realtime thread.
const PIPEWIRE_CALLBACK_MAX_SAMPLES: usize = 8192 * CAPTURE_CHANNEL_COUNT as usize;

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

/// PipeWire's documented exact-target property. `node.target` is obsolete and
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

#[derive(Debug, Clone)]
struct PipeWireDiscoveredNode {
    source: CaptureSource,
    /// Current PipeWire object serial used only to open this live node. It is
    /// deliberately separate from the persistent public source identity.
    target_object: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ApplicationIdentityScope {
    Persistent,
    ProcessLifetime,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PipeWireTimelineMode {
    Undecided,
    SourcePosition,
    CumulativeSamples,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct NegotiatedPipeWireFormat {
    sample_rate_hz: NonZeroU32,
    channel_count: u8,
}

#[derive(Debug)]
struct PipeWireCaptureState {
    expected_channel_count: u8,
    negotiated_format: Option<NegotiatedPipeWireFormat>,
    sample_timeline: Option<CaptureSampleTimeline>,
    timeline_mode: PipeWireTimelineMode,
    connected: bool,
    open_result_sent: bool,
    format_failed: bool,
    callback_samples: [f32; PIPEWIRE_CALLBACK_MAX_SAMPLES],
    frame_normalizer: CaptureFrameNormalizer,
}

impl PipeWireCaptureState {
    fn new(expected_channel_count: u8, frame_samples_per_channel: usize) -> Self {
        Self {
            expected_channel_count,
            negotiated_format: None,
            sample_timeline: None,
            timeline_mode: PipeWireTimelineMode::Undecided,
            connected: false,
            open_result_sent: false,
            format_failed: false,
            callback_samples: [0.0; PIPEWIRE_CALLBACK_MAX_SAMPLES],
            frame_normalizer: CaptureFrameNormalizer::new(
                frame_samples_per_channel,
                expected_channel_count,
                SAMPLE_RATE_HZ,
            ),
        }
    }

    fn set_negotiated_format(
        &mut self,
        negotiated_format: NegotiatedPipeWireFormat,
    ) -> Result<(), &'static str> {
        if let Some(current_format) = self.negotiated_format {
            if current_format != negotiated_format {
                self.format_failed = true;
                return Err("PipeWire changed capture format after negotiation");
            }
            return Ok(());
        }
        if negotiated_format.channel_count != self.expected_channel_count {
            self.format_failed = true;
            return Err("PipeWire negotiated an unexpected channel count");
        }
        if negotiated_format.sample_rate_hz.get() != SAMPLE_RATE_HZ {
            self.format_failed = true;
            return Err("PipeWire negotiated an unexpected sample rate");
        }
        self.sample_timeline = Some(CaptureSampleTimeline::new(negotiated_format.sample_rate_hz));
        self.negotiated_format = Some(negotiated_format);
        Ok(())
    }

    fn advance_sample_timeline(
        &mut self,
        source_sample_frame: Option<u64>,
        sample_frames: u64,
        first_sample_timestamp_ns: u64,
    ) -> Option<(u64, u32, u8)> {
        if self.format_failed {
            return None;
        }
        let negotiated_format = self.negotiated_format?;
        if self.timeline_mode == PipeWireTimelineMode::Undecided {
            self.sample_timeline = Some(CaptureSampleTimeline::anchored(
                negotiated_format.sample_rate_hz,
                first_sample_timestamp_ns,
            ));
        }
        let sample_timeline = self.sample_timeline.as_mut()?;
        let timestamp_ns = match (self.timeline_mode, source_sample_frame) {
            (PipeWireTimelineMode::Undecided, Some(source_sample_frame)) => {
                self.timeline_mode = PipeWireTimelineMode::SourcePosition;
                sample_timeline
                    .advance_from_source_position(source_sample_frame, sample_frames)
                    .ok()?
            }
            (PipeWireTimelineMode::Undecided, None) => {
                self.timeline_mode = PipeWireTimelineMode::CumulativeSamples;
                sample_timeline.advance(sample_frames)
            }
            (PipeWireTimelineMode::SourcePosition, Some(source_sample_frame)) => sample_timeline
                .advance_from_source_position(source_sample_frame, sample_frames)
                .ok()?,
            (PipeWireTimelineMode::SourcePosition, None) => return None,
            (PipeWireTimelineMode::CumulativeSamples, _) => sample_timeline.advance(sample_frames),
        };
        Some((
            timestamp_ns,
            negotiated_format.sample_rate_hz.get(),
            negotiated_format.channel_count,
        ))
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
struct PipeWireCaptureTiming {
    source_sample_frame: Option<u64>,
    capture_delay_ns: u64,
    report_age_ns: u64,
    buffer_timestamp_ns: Option<u64>,
}

fn pipewire_capture_timing(
    stream: &pw::stream::Stream,
    sample_rate_hz: u32,
    buffer_pts_ns: Option<i64>,
    callback_timestamp_ns: u64,
) -> PipeWireCaptureTiming {
    let Ok(stream_time) = stream.time() else {
        return PipeWireCaptureTiming::default();
    };
    let stream_rate = stream_time.rate();
    let source_sample_frame = (stream_rate.num == 1 && stream_rate.denom == sample_rate_hz)
        .then_some(stream_time.ticks());
    let capture_delay_ns = u64::try_from(stream_time.delay())
        .ok()
        .and_then(|delay| pipewire_ticks_to_nanoseconds(delay, stream_rate.num, stream_rate.denom))
        .unwrap_or(0);
    let linux_monotonic_now_ns = linux_monotonic_timestamp_ns();
    let report_age_ns = linux_monotonic_now_ns
        .and_then(|now_ns| pipewire_report_age_ns(stream_time.now(), now_ns))
        .unwrap_or(0);
    let buffer_timestamp_ns = buffer_pts_ns.and_then(|buffer_pts_ns| {
        linux_monotonic_now_ns
            .and_then(|now_ns| pipewire_report_age_ns(buffer_pts_ns, now_ns))
            .map(|buffer_age_ns| callback_timestamp_ns.saturating_sub(buffer_age_ns).max(1))
    });
    PipeWireCaptureTiming {
        source_sample_frame,
        capture_delay_ns,
        report_age_ns,
        buffer_timestamp_ns,
    }
}

fn linux_monotonic_timestamp_ns() -> Option<u64> {
    let mut timestamp = libc::timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    // SAFETY: `timestamp` points to writable storage for one `timespec` and
    // `CLOCK_MONOTONIC` requires no caller-owned resources.
    if unsafe { libc::clock_gettime(libc::CLOCK_MONOTONIC, &mut timestamp) } != 0 {
        return None;
    }
    let seconds = u64::try_from(timestamp.tv_sec).ok()?;
    let nanoseconds = u64::try_from(timestamp.tv_nsec).ok()?;
    seconds
        .checked_mul(1_000_000_000)
        .and_then(|value| value.checked_add(nanoseconds))
}

fn pipewire_report_age_ns(report_timestamp_ns: i64, monotonic_now_ns: u64) -> Option<u64> {
    let report_timestamp_ns = u64::try_from(report_timestamp_ns).ok()?;
    monotonic_now_ns.checked_sub(report_timestamp_ns)
}

fn pipewire_ticks_to_nanoseconds(
    ticks: u64,
    seconds_per_tick_numerator_seconds: u32,
    seconds_per_tick_denominator_ticks: u32,
) -> Option<u64> {
    if seconds_per_tick_numerator_seconds == 0 || seconds_per_tick_denominator_ticks == 0 {
        return None;
    }
    let nanoseconds = u128::from(ticks)
        .checked_mul(u128::from(seconds_per_tick_numerator_seconds))?
        .checked_mul(1_000_000_000)?
        .checked_div(u128::from(seconds_per_tick_denominator_ticks))?;
    u64::try_from(nanoseconds).ok()
}

fn pipewire_first_sample_timestamp_ns(
    callback_timestamp_ns: u64,
    sample_frames: u64,
    sample_rate_hz: u32,
    capture_delay_ns: u64,
    report_age_ns: u64,
) -> u64 {
    let buffer_duration_ns =
        pipewire_ticks_to_nanoseconds(sample_frames, 1, sample_rate_hz).unwrap_or(u64::MAX);
    callback_timestamp_ns
        .saturating_sub(
            buffer_duration_ns
                .saturating_add(capture_delay_ns)
                .saturating_add(report_age_ns),
        )
        .max(1)
}

fn parse_pipewire_negotiated_format(param: &Pod) -> Result<NegotiatedPipeWireFormat, &'static str> {
    let mut audio_info = spa::param::audio::AudioInfoRaw::new();
    audio_info
        .parse(param)
        .map_err(|_| "PipeWire returned an invalid audio format")?;
    if audio_info.format() != AudioFormat::F32LE {
        return Err("PipeWire did not negotiate interleaved f32 audio");
    }
    let sample_rate_hz =
        NonZeroU32::new(audio_info.rate()).ok_or("PipeWire negotiated a zero sample rate")?;
    let channel_count = u8::try_from(audio_info.channels())
        .ok()
        .filter(|channel_count| *channel_count > 0)
        .ok_or("PipeWire negotiated an invalid channel count")?;
    Ok(NegotiatedPipeWireFormat {
        sample_rate_hz,
        channel_count,
    })
}

fn signal_pipewire_open_if_ready(
    state: &mut PipeWireCaptureState,
    open_sender: &mpsc::SyncSender<Result<(), String>>,
) {
    if state.connected
        && state.negotiated_format.is_some()
        && !state.format_failed
        && !state.open_result_sent
        && open_sender.try_send(Ok(())).is_ok()
    {
        state.open_result_sent = true;
    }
}

fn parse_pipewire_sample_rate(audio_rate: Option<&str>, node_rate: Option<&str>) -> u32 {
    audio_rate
        .and_then(|value| value.parse::<u32>().ok())
        .filter(|rate| *rate > 0)
        .or_else(|| {
            node_rate.and_then(|value| {
                let (numerator, denominator) = value.split_once('/')?;
                if numerator != "1" {
                    return None;
                }
                denominator.parse::<u32>().ok().filter(|rate| *rate > 0)
            })
        })
        .unwrap_or(0)
}

fn parse_pipewire_channel_count(audio_channels: Option<&str>) -> u16 {
    audio_channels
        .and_then(|value| value.parse::<u16>().ok())
        .filter(|channels| *channels > 0)
        .unwrap_or(0)
}

fn pipewire_source_name(
    kind: SourceKind,
    application_name: Option<&str>,
    node_description: Option<&str>,
    device_description: Option<&str>,
    node_name: Option<&str>,
) -> String {
    let preferred_name = if kind == SourceKind::Application {
        application_name
            .or(node_description)
            .or(device_description)
            .or(node_name)
    } else {
        node_description
            .or(device_description)
            .or(node_name)
            .or(application_name)
    };
    preferred_name.unwrap_or("unknown").to_owned()
}

fn pipewire_discovered_node(
    props: &spa::utils::dict::DictRef,
    global_id: u32,
) -> Option<PipeWireDiscoveredNode> {
    let kind = match props.get("media.class")? {
        "Stream/Output/Audio" => SourceKind::Application,
        "Audio/Source" => SourceKind::InputDevice,
        "Audio/Sink" => SourceKind::OutputDevice,
        _ => return None,
    };
    let name = pipewire_source_name(
        kind,
        props.get("application.name"),
        props.get("node.description"),
        props.get("device.description"),
        props.get("node.name"),
    );
    let node_name = props.get("node.name").map(str::to_owned);
    let process_id = props
        .get("application.process.id")
        .and_then(|value| value.parse::<u32>().ok());
    let app_id = (kind == SourceKind::Application)
        .then(|| props.get("application.id").map(str::to_owned))
        .flatten();
    let stable_key = if kind == SourceKind::Application {
        pipewire_application_identity(app_id.as_deref(), node_name.as_deref(), global_id).0
    } else {
        node_name
            .as_deref()
            .map(|value| format!("pw-node:{value}"))
            .unwrap_or_else(|| format!("pw-transient:{global_id}"))
    };
    let device_uid = (kind != SourceKind::Application)
        .then(|| node_name.clone())
        .flatten();

    Some(PipeWireDiscoveredNode {
        source: CaptureSource {
            stable_id: StableSourceId::new(Platform::Linux, kind, stable_key),
            name,
            process_id,
            app_id,
            device_uid,
            state: SourceState::Available,
            sample_rate_hz: parse_pipewire_sample_rate(
                props.get("audio.rate"),
                props.get("node.rate"),
            ),
            channels: parse_pipewire_channel_count(props.get("audio.channels")),
        },
        target_object: props.get("object.serial").map(str::to_owned),
    })
}

fn pipewire_application_identity(
    application_id: Option<&str>,
    node_name: Option<&str>,
    global_id: u32,
) -> (String, ApplicationIdentityScope) {
    if let Some(application_id) = application_id.filter(|value| !value.is_empty()) {
        return (
            format!("pw-app:{application_id}"),
            ApplicationIdentityScope::Persistent,
        );
    }
    if let Some(node_name) = node_name.filter(|value| !value.is_empty()) {
        return (
            format!("pw-node:{node_name}"),
            ApplicationIdentityScope::Persistent,
        );
    }
    (
        format!("pw-transient:{global_id}"),
        ApplicationIdentityScope::ProcessLifetime,
    )
}

fn select_unique_exact_application<'a>(
    nodes: &'a [PipeWireDiscoveredNode],
    stable_id: &StableSourceId,
    process_id: Option<u32>,
) -> Result<&'a PipeWireDiscoveredNode, LoopbackError> {
    if stable_id.platform != Platform::Linux || stable_id.kind != SourceKind::Application {
        return Err(source_unavailable(&stable_id.stable_key));
    }

    let mut matching_nodes = nodes.iter().filter(|node| {
        node.source.stable_id == *stable_id
            && process_id.is_none_or(|pid| node.source.process_id == Some(pid))
    });
    let Some(matching_node) = matching_nodes.next() else {
        return Err(source_unavailable(&stable_id.stable_key));
    };
    if matching_nodes.next().is_some() {
        return Err(LoopbackError::BackendInit(format!(
            "ambiguous PipeWire exact-application selector: {}",
            stable_id.stable_key
        )));
    }
    Ok(matching_node)
}

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

fn pipewire_f32_sample_count(
    datas: &mut [spa::buffer::Data],
    channel_count: usize,
) -> Option<usize> {
    if datas.len() == 1 {
        let sample_count = pipewire_f32_plane(&mut datas[0])?.len();
        return (sample_count % channel_count == 0).then_some(sample_count);
    }

    if channel_count == 2 && datas.len() >= 2 {
        let (left_data, remaining_data) = datas.split_at_mut(1);
        let left_frames = pipewire_f32_plane(&mut left_data[0])?.len();
        let right_frames = pipewire_f32_plane(&mut remaining_data[0])?.len();
        return left_frames.min(right_frames).checked_mul(channel_count);
    }

    None
}

fn process_pipewire_audio(
    stream: &pw::stream::Stream,
    state: &mut PipeWireCaptureState,
    pool: &Arc<AudioBufferPool>,
    sequence: &AtomicU64,
    producer: &mut Producer<AudioFrame>,
    counters: &CaptureObservationCounters,
    source_id: SourceId,
) {
    let mut buffer = match stream.dequeue_buffer() {
        Some(buffer) => buffer,
        None => {
            counters.observe_invalid_buffer();
            return;
        }
    };
    counters.observe_callback_buffer();
    let buffer_pts_ns = buffer
        .find_meta::<spa::buffer::meta::MetaHeader>()
        .map(spa::buffer::meta::MetaHeader::pts);
    let datas = buffer.datas_mut();
    if datas.is_empty() {
        counters.observe_invalid_buffer();
        return;
    }
    let channel_count = usize::from(state.expected_channel_count);
    let Some(sample_count) = pipewire_f32_sample_count(datas, channel_count) else {
        counters.observe_invalid_buffer();
        return;
    };
    if sample_count > state.callback_samples.len() {
        counters.observe_oversized_buffer();
        return;
    }
    let copy_count = copy_pipewire_f32_samples(
        datas,
        &mut state.callback_samples[..sample_count],
        channel_count,
    );
    if copy_count != sample_count {
        counters.observe_invalid_buffer();
        return;
    }
    let callback_timestamp_ns = monotonic_timestamp_ns();
    let capture_timing = state
        .negotiated_format
        .map(|format| {
            pipewire_capture_timing(
                stream,
                format.sample_rate_hz.get(),
                buffer_pts_ns,
                callback_timestamp_ns,
            )
        })
        .unwrap_or_default();
    let sample_frames = u64::try_from(sample_count / channel_count).unwrap_or(u64::MAX);
    let first_sample_timestamp_ns = capture_timing.buffer_timestamp_ns.unwrap_or_else(|| {
        pipewire_first_sample_timestamp_ns(
            callback_timestamp_ns,
            sample_frames,
            state
                .negotiated_format
                .map(|format| format.sample_rate_hz.get())
                .unwrap_or(SAMPLE_RATE_HZ),
            capture_timing.capture_delay_ns,
            capture_timing.report_age_ns,
        )
    });
    let capture_format = capture_timing.buffer_timestamp_ns.and_then(|timestamp_ns| {
        (!state.format_failed)
            .then_some(state.negotiated_format)
            .flatten()
            .map(|format| {
                (
                    timestamp_ns,
                    format.sample_rate_hz.get(),
                    format.channel_count,
                )
            })
    });
    let Some((timestamp_ns, sample_rate_hz, output_channel_count)) = capture_format.or_else(|| {
        state.advance_sample_timeline(
            capture_timing.source_sample_frame,
            sample_frames,
            first_sample_timestamp_ns,
        )
    }) else {
        counters.observe_invalid_buffer();
        return;
    };

    let normalized = state.frame_normalizer.push(
        &state.callback_samples[..sample_count],
        timestamp_ns,
        |frame_timestamp_ns, samples| {
            let frame_sequence = sequence.fetch_add(1, Ordering::Relaxed);
            let Some(mut handle) = acquire_capture_buffer(pool, counters) else {
                return;
            };
            if handle.try_copy_from_slice(samples).is_err() {
                counters.observe_oversized_buffer();
                return;
            }
            let mut frame = AudioFrame::new(
                StreamId(0),
                source_id,
                frame_sequence,
                frame_timestamp_ns,
                output_channel_count,
                handle,
            );
            frame.sample_rate_hz = sample_rate_hz;
            enqueue_capture_frame(producer, frame, counters);
        },
    );
    if !normalized {
        counters.observe_invalid_buffer();
    }
}

fn capture_channel_count(mode: &CaptureMode) -> u8 {
    match mode {
        CaptureMode::InputDevice(_) => MICROPHONE_CHANNEL_COUNT,
        CaptureMode::SystemMix
        | CaptureMode::Application(_)
        | CaptureMode::Process(_)
        | CaptureMode::ExactApplication { .. }
        | CaptureMode::ExactApplicationStable { .. } => CAPTURE_CHANNEL_COUNT,
    }
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

fn system_mix_source_id() -> SourceId {
    StableSourceId::new(Platform::Linux, SourceKind::SystemMix, "system:mix").source_id()
}

pub struct SystemLoopbackSource {
    capture_thread: Option<thread::JoinHandle<()>>,
    dispatch_thread: Option<thread::JoinHandle<()>>,
    stop_tx: mpsc::SyncSender<()>,
    counters: CaptureObservationCounters,
    source_id: SourceId,
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
        Self::capture_mode_with_runtime_event_sender(
            mode,
            AudioFrameDuration::default(),
            callback,
            None,
        )
    }

    pub(crate) fn capture_mode_with_runtime_event_sender<F>(
        mode: CaptureMode,
        audio_frame_duration: AudioFrameDuration,
        callback: F,
        runtime_event_sender: Option<crate::capture::SourceRuntimeEventSender>,
    ) -> Result<Self, LoopbackError>
    where
        F: FnMut(AudioFrame) + Send + 'static,
    {
        SystemLoopbackSource::capture_mode_with_runtime_event_sender(
            mode,
            audio_frame_duration,
            callback,
            runtime_event_sender,
        )
        .map(|source| Self { source })
    }

    pub fn observations(&self) -> CaptureObservations {
        self.source.observations()
    }

    pub fn source_id(&self) -> SourceId {
        self.source.source_id()
    }

    pub fn observation_handle(&self) -> CaptureObservationHandle {
        self.source.observation_handle()
    }

    pub fn stop_and_join(self) -> Result<CaptureObservations, LoopbackError> {
        self.source.stop_and_join()
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
        Self::capture_mode_with_runtime_event_sender(
            mode,
            AudioFrameDuration::default(),
            callback,
            None,
        )
    }

    pub(crate) fn capture_mode_with_runtime_event_sender<F>(
        mode: CaptureMode,
        audio_frame_duration: AudioFrameDuration,
        callback: F,
        runtime_event_sender: Option<crate::capture::SourceRuntimeEventSender>,
    ) -> Result<Self, LoopbackError>
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
                match nodes
                    .iter()
                    .find(|node| node.source.process_id == Some(target_pid))
                {
                    Some(node) => run_pipewire_targeted(
                        pipewire_node_target(node)?,
                        node.source.stable_id.source_id(),
                        node.source.stable_id.clone(),
                        mode,
                        audio_frame_duration,
                        callback,
                        runtime_event_sender,
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
                match select_unique_exact_application(&nodes, stable_id, Some(*process_id)) {
                    Ok(node) => run_pipewire_targeted(
                        pipewire_node_target(node)?,
                        stable_id.source_id(),
                        stable_id.clone(),
                        mode,
                        audio_frame_duration,
                        callback,
                        runtime_event_sender,
                    ),
                    Err(error) => Err(error),
                }
            }
            CaptureMode::ExactApplicationStable { stable_id } => {
                if !pipewire_available() {
                    return Err(LoopbackError::ModeUnsupported(mode));
                }
                let nodes = enumerate_pipewire_nodes();
                match select_unique_exact_application(&nodes, stable_id, None) {
                    Ok(node) => run_pipewire_targeted(
                        pipewire_node_target(node)?,
                        stable_id.source_id(),
                        stable_id.clone(),
                        mode,
                        audio_frame_duration,
                        callback,
                        runtime_event_sender,
                    ),
                    Err(error) => Err(error),
                }
            }
            CaptureMode::Application(name) => {
                if !pipewire_available() {
                    return Err(LoopbackError::ModeUnsupported(mode));
                }
                let nodes = enumerate_pipewire_nodes();
                let name_lower = name.to_ascii_lowercase();
                let name_clone = name.clone();
                let mut matches = nodes.iter().filter(|node| {
                    node.source.stable_id.kind == SourceKind::Application
                        && (node.source.name.to_ascii_lowercase() == name_lower
                            || node
                                .source
                                .app_id
                                .as_deref()
                                .is_some_and(|app_id| app_id.eq_ignore_ascii_case(name)))
                });
                match matches.next() {
                    Some(_node) if matches.next().is_some() => Err(LoopbackError::BackendInit(
                        format!(
                            "application '{name}' matches multiple PipeWire audio nodes — select one from source discovery"
                        ),
                    )),
                    Some(node) => run_pipewire_targeted(
                        pipewire_node_target(node)?,
                        node.source.stable_id.source_id(),
                        node.source.stable_id.clone(),
                        mode,
                        audio_frame_duration,
                        callback,
                        runtime_event_sender,
                    ),
                    None => Err(LoopbackError::BackendInit(format!(
                        "BLOCKED_WITH_EVIDENCE: PipeWire per-app source capture requires PipeWire node enumeration and link; no node found for '{name_clone}'"
                    ))),
                }
            }
            CaptureMode::SystemMix => {
                if pipewire_available() {
                    run_pipewire(mode, audio_frame_duration, callback, runtime_event_sender)
                } else {
                    run_alsa(audio_frame_duration, callback, runtime_event_sender)
                }
            }
            CaptureMode::InputDevice(selector) => {
                if !pipewire_available() {
                    return Err(LoopbackError::ModeUnsupported(mode));
                }
                let nodes = enumerate_pipewire_nodes();
                let source = match selector {
                    crate::capture::InputDeviceSelector::Default => nodes.iter().find(|node| {
                        node.source.stable_id.kind == crate::capture::SourceKind::InputDevice
                    }),
                    crate::capture::InputDeviceSelector::StableId(device_uid) => {
                        nodes.iter().find(|node| {
                            node.source.stable_id.kind == crate::capture::SourceKind::InputDevice
                                && node.source.device_uid.as_deref() == Some(device_uid.as_str())
                        })
                    }
                };
                match source {
                    Some(node) => run_pipewire_targeted(
                        pipewire_node_target(node)?,
                        node.source.stable_id.source_id(),
                        node.source.stable_id.clone(),
                        mode,
                        audio_frame_duration,
                        callback,
                        runtime_event_sender,
                    ),
                    None => match selector {
                        crate::capture::InputDeviceSelector::StableId(device_uid) => {
                            Err(source_unavailable(device_uid))
                        }
                        crate::capture::InputDeviceSelector::Default => {
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

    pub fn source_id(&self) -> SourceId {
        self.source_id
    }

    pub fn observation_handle(&self) -> CaptureObservationHandle {
        self.counters.observation_handle()
    }

    pub fn stop_and_join(mut self) -> Result<CaptureObservations, LoopbackError> {
        let counters = self.counters.clone();
        self.stop_workers()?;
        Ok(counters.snapshot())
    }

    fn stop_workers(&mut self) -> Result<(), LoopbackError> {
        let _ = self.stop_tx.try_send(());
        let capture_join = self.capture_thread.take().map_or(Ok(()), |thread| {
            crate::capture::join_capture_worker(thread, "linux capture")
        });
        let dispatch_join = self.dispatch_thread.take().map_or(Ok(()), |thread| {
            crate::capture::join_capture_worker(thread, "linux dispatch")
        });
        capture_join.and(dispatch_join)
    }
}

fn pipewire_node_target(node: &PipeWireDiscoveredNode) -> Result<String, LoopbackError> {
    node.target_object.clone().ok_or_else(|| {
        LoopbackError::BackendInit(format!(
            "PipeWire source '{}' has no current object serial",
            node.source.name
        ))
    })
}

/// Drop contract — control thread only: signal once, join both owned workers,
/// never execute from a capture callback or realtime partition.
impl Drop for SystemLoopbackSource {
    fn drop(&mut self) {
        let _ = self.stop_workers();
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
    audio_frame_duration: AudioFrameDuration,
    mut callback: F,
    runtime_event_sender: Option<crate::capture::SourceRuntimeEventSender>,
) -> Result<SystemLoopbackSource, LoopbackError>
where
    F: FnMut(AudioFrame) + Send + 'static,
{
    initialize_monotonic_timestamp_domain();
    let source_id = system_mix_source_id();
    let (stop_tx, stop_rx) = mpsc::sync_channel::<()>(1);
    let (open_tx, open_rx) = mpsc::sync_channel::<Result<(), String>>(1);
    let (frame_producer, mut frame_consumer) =
        RingBuffer::<AudioFrame>::new(DISPATCH_QUEUE_CAPACITY_FRAMES);

    let frame_samples_per_channel = audio_frame_duration.samples_per_channel(SAMPLE_RATE_HZ);
    let frame_sample_count = frame_samples_per_channel * usize::from(CAPTURE_CHANNEL_COUNT);
    let pool = AudioBufferPool::new(CAPTURE_POOL_CAPACITY_FRAMES, frame_sample_count);
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
            let format_open_tx = open_tx.clone();
            let state_counters = capture_counters.clone();
            let format_counters = capture_counters.clone();
            let mut runtime_failure_event = runtime_event_sender.as_ref().map(|_| {
                crate::capture::SourceRuntimeEvent::BackendFailure {
                    stable_id: StableSourceId::new(
                        Platform::Linux,
                        SourceKind::SystemMix,
                        "system:mix",
                    ),
                    generation: crate::capture::SourceGeneration::INITIAL,
                    failure: crate::capture::CaptureRuntimeFailure {
                        operation: "Linux PipeWire system-mix stream",
                        error_class: crate::capture::CaptureRuntimeFailureClass::BackendClass {
                            class: "pipewire-stream-error".to_owned(),
                        },
                    },
                }
            });
            let stream_subscription = match stream
                .add_local_listener_with_user_data(PipeWireCaptureState::new(
                    CAPTURE_CHANNEL_COUNT,
                    frame_samples_per_channel,
                ))
                .state_changed(move |_stream, state, _old, new| match new {
                    pw::stream::StreamState::Paused | pw::stream::StreamState::Streaming => {
                        state.connected = true;
                        signal_pipewire_open_if_ready(state, &state_open_tx);
                    }
                    pw::stream::StreamState::Error(error) => {
                        state_counters.observe_stream_error();
                        if let (Some(sender), Some(event)) =
                            (runtime_event_sender.as_ref(), runtime_failure_event.take())
                        {
                            let _ = sender.try_send(event);
                        }
                        let _ =
                            state_open_tx.try_send(Err(format!("PipeWire stream state: {error}")));
                    }
                    _ => {}
                })
                .param_changed(move |_stream, state, id, param| {
                    if id != spa::param::ParamType::Format.as_raw() {
                        return;
                    }
                    let Some(param) = param else {
                        return;
                    };
                    let result = parse_pipewire_negotiated_format(param)
                        .and_then(|format| state.set_negotiated_format(format));
                    match result {
                        Ok(()) => signal_pipewire_open_if_ready(state, &format_open_tx),
                        Err(error) => {
                            state.format_failed = true;
                            format_counters.observe_stream_error();
                            let _ = format_open_tx.try_send(Err(error.to_owned()));
                        }
                    }
                })
                .process(move |stream, state| {
                    process_pipewire_audio(
                        stream,
                        state,
                        &pool_cb,
                        &seq_cb,
                        &mut frame_producer,
                        &process_counters,
                        source_id,
                    );
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
        source_id,
    })
}

/// Enumerate all audio capture sources visible on this Linux system.
///
/// Always returns at least one entry (the system-wide mix at id=0).
/// If PipeWire is available, per-application node sources are appended.
pub fn discover_sources_linux() -> Vec<crate::capture::CaptureSource> {
    use crate::capture::{CaptureSource, SourceKind, SourceState, StableSourceId};
    use crate::frame::Platform;

    let system_mix = CaptureSource {
        stable_id: StableSourceId::new(Platform::Linux, SourceKind::SystemMix, "system:mix"),
        name: "System Mix".to_owned(),
        process_id: None,
        app_id: None,
        device_uid: None,
        state: SourceState::Available,
        // The default sink's native format is known only after PipeWire opens
        // the monitor stream. Zero means discovery did not observe it.
        sample_rate_hz: 0,
        channels: 0,
    };

    let mut sources = vec![system_mix];
    if pipewire_available() {
        sources.extend(
            enumerate_pipewire_nodes()
                .into_iter()
                .map(|node| node.source),
        );
    }
    sources
}

/// Collect PipeWire audio nodes via the registry API.
///
/// Spawns a thread, connects to PipeWire, subscribes to registry globals,
/// and waits for the initial round-trip to complete (or 300 ms timeout).
/// Returns an empty `Vec` on any error.
fn enumerate_pipewire_nodes() -> Vec<PipeWireDiscoveredNode> {
    use std::cell::{Cell, RefCell};
    use std::rc::Rc;
    use std::sync::mpsc as smpsc;

    let (tx, rx) = smpsc::channel::<Vec<PipeWireDiscoveredNode>>();

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

            let registry = match core.get_registry_rc() {
                Ok(r) => r,
                Err(_) => {
                    let _ = tx.send(Vec::new());
                    return;
                }
            };

            let collected: Rc<RefCell<Vec<(u32, PipeWireDiscoveredNode)>>> =
                Rc::new(RefCell::new(Vec::new()));
            let collected_for_reg = collected.clone();
            let node_bindings: Rc<RefCell<Vec<(pw::node::Node, pw::node::NodeListener)>>> =
                Rc::new(RefCell::new(Vec::new()));
            let node_bindings_for_reg = node_bindings.clone();
            let registry_weak = registry.downgrade();

            let _reg_listener = registry
                .add_listener_local()
                .global(move |global| {
                    let props = match global.props {
                        Some(ref p) => p,
                        None => return,
                    };
                    let media_class = props.get("media.class").unwrap_or("");
                    if !matches!(
                        media_class,
                        "Stream/Output/Audio" | "Audio/Source" | "Audio/Sink"
                    ) {
                        return;
                    }

                    let Some(registry) = registry_weak.upgrade() else {
                        return;
                    };
                    let Ok(node) = registry.bind::<pw::node::Node, _>(global) else {
                        return;
                    };
                    let global_id = global.id;
                    let collected_for_info = collected_for_reg.clone();
                    let node_listener = node
                        .add_listener_local()
                        .info(move |info| {
                            let Some(props) = info.props() else {
                                return;
                            };
                            let Some(discovered) = pipewire_discovered_node(props, global_id)
                            else {
                                return;
                            };
                            let mut collected = collected_for_info.borrow_mut();
                            if let Some((_, existing)) = collected
                                .iter_mut()
                                .find(|(candidate_id, _)| *candidate_id == global_id)
                            {
                                *existing = discovered;
                            } else {
                                collected.push((global_id, discovered));
                            }
                        })
                        .register();
                    node_bindings_for_reg
                        .borrow_mut()
                        .push((node, node_listener));
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
            let core_weak = core.downgrade();
            let final_seq = Rc::new(Cell::new(None));
            let final_seq_for_done = final_seq.clone();

            let _core_listener = core
                .add_listener_local()
                .done(move |id, done_seq| {
                    if id != 0 {
                        return;
                    }
                    if done_seq == seq {
                        if let Some(core) = core_weak.upgrade() {
                            if let Ok(seq) = core.sync(0) {
                                final_seq_for_done.set(Some(seq));
                                return;
                            }
                        }
                    }
                    if final_seq_for_done
                        .get()
                        .is_some_and(|final_seq| done_seq == final_seq)
                    {
                        let sources = collected_for_done
                            .borrow()
                            .iter()
                            .map(|(_, source)| source.clone())
                            .collect();
                        let _ = tx_clone.send(sources);
                        if let Some(mainloop) = ml_for_done.upgrade() {
                            mainloop.quit();
                        }
                    }
                })
                .register();

            // Safety valve: quit after 300 ms even if done never fires.
            let ml_timer = mainloop.downgrade();
            let tx_timer = tx.clone();
            let collected_timer = collected.clone();
            let timer = mainloop.loop_().add_timer(move |_| {
                let sources = collected_timer
                    .borrow()
                    .iter()
                    .map(|(_, source)| source.clone())
                    .collect();
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
    source_id: SourceId,
    stable_id: StableSourceId,
    mode: CaptureMode,
    audio_frame_duration: AudioFrameDuration,
    mut callback: F,
    runtime_event_sender: Option<crate::capture::SourceRuntimeEventSender>,
) -> Result<SystemLoopbackSource, LoopbackError>
where
    F: FnMut(AudioFrame) + Send + 'static,
{
    initialize_monotonic_timestamp_domain();
    let capture_channel_count = capture_channel_count(&mode);
    let (stop_tx, stop_rx) = mpsc::sync_channel::<()>(1);
    let (open_tx, open_rx) = mpsc::sync_channel::<Result<(), String>>(1);
    let (frame_producer, mut frame_consumer) =
        RingBuffer::<AudioFrame>::new(DISPATCH_QUEUE_CAPACITY_FRAMES);

    let frame_samples_per_channel = audio_frame_duration.samples_per_channel(SAMPLE_RATE_HZ);
    let frame_sample_count = frame_samples_per_channel * usize::from(capture_channel_count);
    let pool = AudioBufferPool::new(CAPTURE_POOL_CAPACITY_FRAMES, frame_sample_count);
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
            let format_open_tx = open_tx.clone();
            let state_counters = capture_counters.clone();
            let format_counters = capture_counters.clone();
            let mut runtime_failure_event = runtime_event_sender.as_ref().map(|_| {
                crate::capture::SourceRuntimeEvent::BackendFailure {
                    stable_id,
                    generation: crate::capture::SourceGeneration::INITIAL,
                    failure: crate::capture::CaptureRuntimeFailure {
                        operation: "Linux PipeWire targeted stream",
                        error_class: crate::capture::CaptureRuntimeFailureClass::BackendClass {
                            class: "pipewire-stream-error".to_owned(),
                        },
                    },
                }
            });
            let stream_subscription = match stream
                .add_local_listener_with_user_data(PipeWireCaptureState::new(
                    capture_channel_count,
                    frame_samples_per_channel,
                ))
                .state_changed(move |_stream, state, _old, new| match new {
                    pw::stream::StreamState::Paused | pw::stream::StreamState::Streaming => {
                        state.connected = true;
                        signal_pipewire_open_if_ready(state, &state_open_tx);
                    }
                    pw::stream::StreamState::Error(error) => {
                        state_counters.observe_stream_error();
                        if let (Some(sender), Some(event)) =
                            (runtime_event_sender.as_ref(), runtime_failure_event.take())
                        {
                            let _ = sender.try_send(event);
                        }
                        let _ =
                            state_open_tx.try_send(Err(format!("PipeWire stream state: {error}")));
                    }
                    _ => {}
                })
                .param_changed(move |_stream, state, id, param| {
                    if id != spa::param::ParamType::Format.as_raw() {
                        return;
                    }
                    let Some(param) = param else {
                        return;
                    };
                    let result = parse_pipewire_negotiated_format(param)
                        .and_then(|format| state.set_negotiated_format(format));
                    match result {
                        Ok(()) => signal_pipewire_open_if_ready(state, &format_open_tx),
                        Err(error) => {
                            state.format_failed = true;
                            format_counters.observe_stream_error();
                            let _ = format_open_tx.try_send(Err(error.to_owned()));
                        }
                    }
                })
                .process(move |stream, state| {
                    process_pipewire_audio(
                        stream,
                        state,
                        &pool_cb,
                        &seq_cb,
                        &mut frame_producer,
                        &process_counters,
                        source_id,
                    );
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
            audio_info.set_channels(capture_channel_count as u32);
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
        source_id,
    })
}

fn run_alsa<F>(
    audio_frame_duration: AudioFrameDuration,
    mut callback: F,
    runtime_event_sender: Option<crate::capture::SourceRuntimeEventSender>,
) -> Result<SystemLoopbackSource, LoopbackError>
where
    F: FnMut(AudioFrame) + Send + 'static,
{
    use alsa::pcm::{Access, Format, HwParams, PCM};
    use alsa::Direction;

    let (stop_tx, stop_rx) = mpsc::sync_channel::<()>(1);
    let source_id = system_mix_source_id();
    let (open_tx, open_rx) = mpsc::sync_channel::<Result<(), String>>(1);
    let frame_samples_per_channel = audio_frame_duration.samples_per_channel(SAMPLE_RATE_HZ);
    let frame_sample_count = frame_samples_per_channel * usize::from(CAPTURE_CHANNEL_COUNT);
    let pool = AudioBufferPool::new(CAPTURE_POOL_CAPACITY_FRAMES, frame_sample_count);
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

            let negotiated_sample_rate_hz = {
                let hwp = match HwParams::any(&pcm) {
                    Ok(p) => p,
                    Err(e) => {
                        let _ = open_tx.send(Err(format!("ALSA hardware parameters: {e}")));
                        return;
                    }
                };
                if let Err(error) = hwp.set_channels(CAPTURE_CHANNEL_COUNT as u32) {
                    let _ = open_tx.send(Err(format!("ALSA channel configuration: {error}")));
                    return;
                }
                if let Err(error) = hwp.set_rate(SAMPLE_RATE_HZ, alsa::ValueOr::Nearest) {
                    let _ = open_tx.send(Err(format!("ALSA sample-rate configuration: {error}")));
                    return;
                }
                if let Err(error) = hwp.set_format(Format::float()) {
                    let _ = open_tx.send(Err(format!("ALSA sample-format configuration: {error}")));
                    return;
                }
                if let Err(error) = hwp.set_access(Access::RWInterleaved) {
                    let _ = open_tx.send(Err(format!("ALSA access configuration: {error}")));
                    return;
                }
                if let Err(e) = pcm.hw_params(&hwp) {
                    let _ = open_tx.send(Err(format!("ALSA apply hardware parameters: {e}")));
                    return;
                }
                let current = match pcm.hw_params_current() {
                    Ok(current) => current,
                    Err(error) => {
                        let _ = open_tx.send(Err(format!("ALSA negotiated parameters: {error}")));
                        return;
                    }
                };
                let channel_count = match current.get_channels() {
                    Ok(channel_count) => channel_count,
                    Err(error) => {
                        let _ =
                            open_tx.send(Err(format!("ALSA negotiated channel count: {error}")));
                        return;
                    }
                };
                if channel_count != u32::from(CAPTURE_CHANNEL_COUNT) {
                    let _ = open_tx.send(Err(format!(
                        "ALSA negotiated {channel_count} channels; expected {}",
                        CAPTURE_CHANNEL_COUNT
                    )));
                    return;
                }
                let sample_format = match current.get_format() {
                    Ok(sample_format) => sample_format,
                    Err(error) => {
                        let _ =
                            open_tx.send(Err(format!("ALSA negotiated sample format: {error}")));
                        return;
                    }
                };
                if sample_format != Format::float() {
                    let _ = open_tx.send(Err(format!(
                        "ALSA negotiated unsupported sample format: {sample_format:?}"
                    )));
                    return;
                }
                match current.get_rate().ok().and_then(NonZeroU32::new) {
                    Some(sample_rate_hz) => sample_rate_hz,
                    None => {
                        let _ =
                            open_tx.send(Err("ALSA negotiated an invalid sample rate".to_owned()));
                        return;
                    }
                }
            };

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
            let mut buf = vec![0f32; frame_sample_count];
            let mut sample_timeline = CaptureSampleTimeline::new(negotiated_sample_rate_hz);
            let mut runtime_failure_event = runtime_event_sender.as_ref().map(|_| {
                crate::capture::SourceRuntimeEvent::BackendFailure {
                    stable_id: StableSourceId::new(
                        Platform::Linux,
                        SourceKind::SystemMix,
                        "system:mix",
                    ),
                    generation: crate::capture::SourceGeneration::INITIAL,
                    failure: crate::capture::CaptureRuntimeFailure {
                        operation: "Linux ALSA capture reader",
                        error_class: crate::capture::CaptureRuntimeFailureClass::BackendClass {
                            class: "alsa-read-error".to_owned(),
                        },
                    },
                }
            });

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
                        if let (Some(sender), Some(event)) =
                            (runtime_event_sender.as_ref(), runtime_failure_event.take())
                        {
                            let _ = sender.try_send(event);
                        }
                        thread::sleep(Duration::from_millis(1));
                        continue;
                    }
                    Ok(frames_read) => {
                        capture_counters.observe_callback_buffer();
                        let sample_count = frames_read * CAPTURE_CHANNEL_COUNT as usize;
                        let timestamp_ns = sample_timeline.advance(frames_read as u64);
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
                        if handle.try_set_len(sample_count).is_err() {
                            capture_counters.observe_oversized_buffer();
                            continue;
                        }

                        let s = seq.fetch_add(1, Ordering::Relaxed);

                        let mut frame = AudioFrame::new(
                            StreamId(0),
                            source_id,
                            s,
                            timestamp_ns,
                            CAPTURE_CHANNEL_COUNT,
                            handle,
                        );
                        frame.sample_rate_hz = negotiated_sample_rate_hz.get();
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
        source_id,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::capture::{CaptureError, SourceKind, SourceState, StableSourceId};
    use crate::frame::Platform;

    fn frame_from_pool(pool: &Arc<AudioBufferPool>, sequence: u64) -> AudioFrame {
        let handle = pool.acquire().expect("test pool must have a free slot");
        AudioFrame::new(StreamId(7), SourceId(11), sequence, 13, 1, handle)
    }

    fn discovered_application(
        stable_key: &str,
        process_id: u32,
        target_object: &str,
    ) -> PipeWireDiscoveredNode {
        PipeWireDiscoveredNode {
            source: CaptureSource {
                stable_id: StableSourceId::new(
                    Platform::Linux,
                    SourceKind::Application,
                    stable_key,
                ),
                name: "Meeting".to_owned(),
                process_id: Some(process_id),
                app_id: Some("org.example.Meeting".to_owned()),
                device_uid: None,
                state: SourceState::Available,
                sample_rate_hz: 48_000,
                channels: 2,
            },
            target_object: Some(target_object.to_owned()),
        }
    }

    #[test]
    fn given_capture_mode_when_channels_selected_then_microphone_is_mono_and_output_is_stereo() {
        assert_eq!(
            capture_channel_count(&CaptureMode::InputDevice(
                crate::capture::InputDeviceSelector::Default
            )),
            MICROPHONE_CHANNEL_COUNT
        );
        assert_eq!(
            capture_channel_count(&CaptureMode::Application("meeting".to_owned())),
            CAPTURE_CHANNEL_COUNT
        );
        assert_eq!(
            capture_channel_count(&CaptureMode::SystemMix),
            CAPTURE_CHANNEL_COUNT
        );
    }

    #[test]
    fn given_pipewire_source_metadata_when_named_then_human_description_precedes_node_name() {
        assert_eq!(
            pipewire_source_name(
                SourceKind::InputDevice,
                None,
                Some("Studio Microphone"),
                None,
                Some("alsa_input.usb-42"),
            ),
            "Studio Microphone"
        );
        assert_eq!(
            pipewire_source_name(
                SourceKind::Application,
                Some("Meeting"),
                Some("Meeting audio output"),
                None,
                Some("meeting-output"),
            ),
            "Meeting"
        );
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

    #[test]
    fn given_pipewire_application_metadata_when_identity_is_derived_then_persistent_fields_win() {
        assert_eq!(
            pipewire_application_identity(Some("org.example.Meeting"), Some("meeting-output"), 91),
            (
                "pw-app:org.example.Meeting".to_owned(),
                ApplicationIdentityScope::Persistent
            )
        );
        assert_eq!(
            pipewire_application_identity(None, Some("meeting-output"), 91),
            (
                "pw-node:meeting-output".to_owned(),
                ApplicationIdentityScope::Persistent
            )
        );
        assert_eq!(
            pipewire_application_identity(None, None, 91),
            (
                "pw-transient:91".to_owned(),
                ApplicationIdentityScope::ProcessLifetime
            )
        );
    }

    #[test]
    fn given_exact_application_selector_when_one_live_node_matches_then_current_target_is_selected()
    {
        let nodes = vec![
            discovered_application("pw-app:org.example.Meeting", 41, "812"),
            discovered_application("pw-app:org.example.Other", 42, "813"),
        ];
        let selector = StableSourceId::new(
            Platform::Linux,
            SourceKind::Application,
            "pw-app:org.example.Meeting",
        );

        let selected = select_unique_exact_application(&nodes, &selector, Some(41))
            .expect("one persistent exact application must resolve");

        assert_eq!(selected.target_object.as_deref(), Some("812"));
        assert!(selected.source.device_uid.is_none());
    }

    #[test]
    fn given_exact_application_selector_when_identity_is_transient_then_live_source_is_selected() {
        let nodes = vec![discovered_application("pw-transient:91", 41, "812")];
        let selector =
            StableSourceId::new(Platform::Linux, SourceKind::Application, "pw-transient:91");

        let selected = select_unique_exact_application(&nodes, &selector, None)
            .expect("an exact transient identity remains valid while its node is live");

        assert_eq!(selected.target_object.as_deref(), Some("812"));
    }

    #[test]
    fn given_process_scoped_exact_selector_when_identity_is_transient_then_matching_pid_is_allowed()
    {
        let nodes = vec![discovered_application("pw-transient:91", 41, "812")];
        let selector =
            StableSourceId::new(Platform::Linux, SourceKind::Application, "pw-transient:91");

        let selected = select_unique_exact_application(&nodes, &selector, Some(41))
            .expect("process-scoped exact selector may use a transient identity");

        assert_eq!(selected.target_object.as_deref(), Some("812"));
    }

    #[test]
    fn given_exact_application_selector_when_multiple_nodes_match_then_selection_is_ambiguous() {
        let nodes = vec![
            discovered_application("pw-app:org.example.Meeting", 41, "812"),
            discovered_application("pw-app:org.example.Meeting", 42, "813"),
        ];
        let selector = StableSourceId::new(
            Platform::Linux,
            SourceKind::Application,
            "pw-app:org.example.Meeting",
        );

        let error = select_unique_exact_application(&nodes, &selector, None)
            .expect_err("persistent selector must not choose an arbitrary live node");

        assert!(
            matches!(error, CaptureError::BackendInit(message) if message.contains("ambiguous"))
        );
    }

    #[test]
    fn given_pipewire_properties_when_native_format_is_reported_then_unknown_is_not_fabricated() {
        assert_eq!(
            parse_pipewire_sample_rate(Some("invalid"), Some("1/48000")),
            48_000
        );
        assert_eq!(
            parse_pipewire_sample_rate(Some("44100"), Some("1/48000")),
            44_100
        );
        assert_eq!(parse_pipewire_sample_rate(None, None), 0);
        assert_eq!(parse_pipewire_channel_count(Some("2")), 2);
        assert_eq!(parse_pipewire_channel_count(None), 0);
    }

    #[test]
    fn given_negotiated_format_when_one_buffer_is_dropped_then_sample_timeline_keeps_its_duration()
    {
        let mut state = PipeWireCaptureState::new(
            CAPTURE_CHANNEL_COUNT,
            AudioFrameDuration::default().samples_per_channel(SAMPLE_RATE_HZ),
        );
        state
            .set_negotiated_format(NegotiatedPipeWireFormat {
                sample_rate_hz: NonZeroU32::new(48_000).expect("test rate is non-zero"),
                channel_count: CAPTURE_CHANNEL_COUNT,
            })
            .expect("test format must negotiate");

        let (dropped_timestamp_ns, _, _) = state
            .advance_sample_timeline(Some(1_000), 480, 2_000_000_000)
            .expect("dropped source buffer must still advance time");
        let (delivered_timestamp_ns, sample_rate_hz, channel_count) = state
            .advance_sample_timeline(Some(1_480), 480, 9_000_000_000)
            .expect("next source buffer must retain cadence");

        assert_eq!(dropped_timestamp_ns, 2_000_000_000);
        assert_eq!(delivered_timestamp_ns - dropped_timestamp_ns, 10_000_000);
        assert_eq!(sample_rate_hz, 48_000);
        assert_eq!(channel_count, CAPTURE_CHANNEL_COUNT);
    }

    #[test]
    fn given_pipewire_callback_when_buffer_contains_multiple_frames_then_timestamp_starts_before_callback(
    ) {
        assert_eq!(
            pipewire_first_sample_timestamp_ns(
                1_000_000_000,
                2_048,
                48_000,
                5_000_000,
                300_000_000,
            ),
            652_333_334
        );
        assert_eq!(
            pipewire_report_age_ns(700_000_000, 1_000_000_000),
            Some(300_000_000)
        );
        assert_eq!(pipewire_report_age_ns(1_100_000_000, 1_000_000_000), None);
        assert_eq!(pipewire_report_age_ns(-1, 1_000_000_000), None);
        assert_eq!(
            pipewire_ticks_to_nanoseconds(960, 1, 48_000),
            Some(20_000_000)
        );
        assert_eq!(pipewire_ticks_to_nanoseconds(960, 0, 48_000), None);
    }

    #[test]
    fn given_negotiated_format_when_channel_count_changes_then_capture_fails_closed() {
        let mut state = PipeWireCaptureState::new(
            MICROPHONE_CHANNEL_COUNT,
            AudioFrameDuration::default().samples_per_channel(SAMPLE_RATE_HZ),
        );

        assert_eq!(
            state.set_negotiated_format(NegotiatedPipeWireFormat {
                sample_rate_hz: NonZeroU32::new(48_000).expect("test rate is non-zero"),
                channel_count: CAPTURE_CHANNEL_COUNT,
            }),
            Err("PipeWire negotiated an unexpected channel count")
        );
        assert!(state
            .advance_sample_timeline(None, 480, 2_000_000_000)
            .is_none());
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
