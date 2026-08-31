const MACOS_INPUT: &str = include_str!("../src/capture/platform/macos/input.rs");
const LINUX_CAPTURE: &str = include_str!("../src/capture/platform/linux/pipewire.rs");
const WINDOWS_CAPTURE: &str = include_str!("../src/capture/platform/windows/windows.rs");
const MACOS_NATIVE_TAP: &str = include_str!("../native/macos/asp/source_discovery.m");
const MACOS_ASP_RING: &str = include_str!("../native/macos/asp/SharedRing.h");
const FRAME_OWNERSHIP: &str = include_str!("../src/frame/pool.rs");
const PLAN_ROUTER: &str = include_str!("../src/runtime/audio/router.rs");

const FORBIDDEN_REALTIME_TOKENS: &[&str] = &[
    "Vec::",
    "vec![",
    "Box::",
    "String::",
    ".to_owned()",
    "format!",
    "malloc(",
    "calloc(",
    "realloc(",
    "free(",
    ".lock(",
    "Mutex",
    "Condvar",
    "recv(",
    "recv_timeout(",
    "sleep(",
    "yield_now(",
    "park(",
    ".await",
    "spawn(",
    "println!",
    "eprintln!",
    "dbg!",
    "trace!",
    "debug!",
    "info!",
    "warn!",
    "error!",
    "printf(",
    "NSLog",
    "panic!",
    "unwrap(",
    "expect(",
    "assert!(",
];

fn fragment_between<'a>(source: &'a str, start: &str, end: &str) -> &'a str {
    let start_index = source
        .find(start)
        .unwrap_or_else(|| panic!("missing realtime boundary start marker: {start}"));
    let remainder = &source[start_index..];
    let end_index = remainder
        .find(end)
        .unwrap_or_else(|| panic!("missing realtime boundary end marker: {end}"));
    &remainder[..end_index]
}

fn fragments_between_all<'a>(source: &'a str, start: &str, end: &str) -> Vec<&'a str> {
    let mut fragments = Vec::new();
    let mut remainder = source;
    while let Some(start_index) = remainder.find(start) {
        let candidate = &remainder[start_index..];
        let end_index = candidate
            .find(end)
            .unwrap_or_else(|| panic!("missing realtime boundary end marker: {end}"));
        fragments.push(&candidate[..end_index]);
        remainder = &candidate[end_index + end.len()..];
    }
    assert!(!fragments.is_empty(), "missing realtime boundary: {start}");
    fragments
}

fn assert_realtime_fragment(name: &str, fragment: &str, required_tokens: &[&str]) {
    for forbidden in FORBIDDEN_REALTIME_TOKENS {
        assert!(
            !fragment.contains(forbidden),
            "{name} contains forbidden realtime token {forbidden:?}"
        );
    }
    for required in required_tokens {
        assert!(
            fragment.contains(required),
            "{name} lost required bounded primitive {required:?}"
        );
    }
}

#[test]
fn given_macos_input_callback_when_source_changes_then_realtime_contract_remains_explicit() {
    let callback = fragment_between(
        MACOS_INPUT,
        "let data_callback = move |data:",
        "let error_counters =",
    );
    assert_realtime_fragment(
        "macOS CPAL input callback",
        callback,
        &["callback_pool.acquire()", "producer.push(frame)"],
    );
}

#[test]
fn given_pipewire_process_callbacks_when_source_changes_then_realtime_contract_remains_explicit() {
    let process_audio = fragment_between(
        LINUX_CAPTURE,
        "fn process_pipewire_audio(",
        "fn capture_channel_count(",
    );
    assert_realtime_fragment(
        "PipeWire bounded audio processing",
        process_audio,
        &["acquire_capture_buffer", "enqueue_capture_frame"],
    );

    let callbacks = fragments_between_all(
        LINUX_CAPTURE,
        ".process(move |stream, state| {",
        ".register()",
    );
    assert_eq!(
        callbacks.len(),
        2,
        "expected monitor and microphone callbacks"
    );
    for (index, callback) in callbacks.into_iter().enumerate() {
        assert_realtime_fragment(
            &format!("PipeWire process callback {index}"),
            callback,
            &["process_pipewire_audio"],
        );
    }
}

#[test]
fn given_wasapi_packet_delivery_when_source_changes_then_bounded_worker_contract_remains_explicit()
{
    let capture_handoff = fragment_between(
        WINDOWS_CAPTURE,
        "let capture_callback = move |frame| {",
        "let result = match resolved_mode",
    );
    assert_realtime_fragment(
        "WASAPI capture-worker handoff",
        capture_handoff,
        &["frame_producer.push(frame)", "observe_dispatch_queue_full"],
    );

    let delivery = fragment_between(
        WINDOWS_CAPTURE,
        "fn deliver_packet(",
        "struct CaptureLoopState",
    );
    assert_realtime_fragment(
        "WASAPI packet delivery",
        delivery,
        &[
            "frame_normalizer.push(",
            "pool.acquire()",
            "handle.try_copy_from_slice(samples)",
            "callback(frame)",
        ],
    );
    assert!(
        !delivery.contains("monotonic_timestamp_ns()"),
        "WASAPI packet delivery must receive the native first-sample timestamp instead of stamping callback arrival"
    );
    for required_timestamp_boundary in [
        "info.timestamp",
        "qpc_position",
        "qpc_timestamp_ns",
        "timestamp_mapping",
        ".to_monotonic_ns",
    ] {
        assert!(
            WINDOWS_CAPTURE.contains(required_timestamp_boundary),
            "WASAPI capture lost native timestamp boundary {required_timestamp_boundary:?}"
        );
    }
    assert!(
        WINDOWS_CAPTURE.matches("plan_packet_read(").count() >= 4,
        "every WASAPI packet path must validate native frame counts before reading fixed scratch"
    );
}

#[test]
fn given_macos_native_callbacks_when_source_changes_then_no_control_plane_work_enters_them() {
    let process_tap = fragment_between(
        MACOS_NATIVE_TAP,
        "static OSStatus tap_io_proc(",
        "// ─── Helper: look up AudioObjectID",
    );
    assert_realtime_fragment(
        "macOS process-tap IO callback",
        process_tap,
        &["TAP_RING_FRAMES", "atomic_store_explicit", "drop_count"],
    );
    assert!(
        !process_tap.contains("sqrtf("),
        "RMS diagnostics belong on the reader worker"
    );

    let asp_write = fragment_between(
        MACOS_ASP_RING,
        "static inline int pks_ring_try_write(",
        "/* Total shared memory size.",
    );
    assert_realtime_fragment(
        "macOS ASP IO callback ring write",
        asp_write,
        &["PKS_RING_FRAMES", "memory_order_release", "drop_count"],
    );
}

#[test]
fn given_hot_ownership_drops_when_source_changes_then_cleanup_remains_bounded_and_nonblocking() {
    let exclusive_drop = fragment_between(
        FRAME_OWNERSHIP,
        "impl Drop for AudioBufferHandle",
        "impl fmt::Debug for AudioBufferHandle",
    );
    assert_realtime_fragment(
        "exclusive audio-buffer Drop",
        exclusive_drop,
        &["self.pool.release(self.index)"],
    );

    let shared_drop = fragment_between(
        FRAME_OWNERSHIP,
        "impl Drop for SharedAudioBufferHandle",
        "impl fmt::Debug for SharedAudioBufferHandle",
    );
    assert_realtime_fragment(
        "shared audio-buffer Drop",
        shared_drop,
        &["self.pool.release_shared(self.index)"],
    );

    let edge_drop = fragment_between(
        PLAN_ROUTER,
        "impl Drop for PlanEdgeReceiver",
        "struct RoutedEdge",
    );
    assert_realtime_fragment(
        "bounded realtime edge receiver Drop",
        edge_drop,
        &[
            "self.alive.store",
            "self.consumer.pop()",
            "shutdown_discarded_total",
        ],
    );
}
