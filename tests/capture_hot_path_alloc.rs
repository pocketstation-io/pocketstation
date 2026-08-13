use assert_no_alloc::{assert_no_alloc, AllocDisabler};
use pocketstation::internal::capture::{
    capture_delivery_start_gate, captured_frame_stream, captured_frame_stream_with_start_gate,
    CapturedFrameDelivery,
};
use pocketstation::internal::frame::{
    AudioBufferPool, AudioFrame, SampleFormat, SampleSpec, SourceId, StreamId,
};

#[global_allocator]
static ALLOCATOR: AllocDisabler = AllocDisabler;

const SPEC: SampleSpec = SampleSpec {
    sample_rate_hz: 48_000,
    channels: 1,
    format: SampleFormat::F32Interleaved,
};

fn frame(pool: &std::sync::Arc<AudioBufferPool>, sequence_number: u64) -> AudioFrame {
    AudioFrame::try_new(
        StreamId::new(1),
        SourceId::new(2),
        sequence_number,
        sequence_number * 20_000_000,
        SPEC,
        pool.acquire().expect("prepared pool capacity"),
    )
    .expect("prepared frame shape")
}

#[test]
fn given_preallocated_capture_handoff_when_frames_flow_then_no_heap_allocation_occurs() {
    let pool = AudioBufferPool::new(2, 960);
    let (mut sender, mut stream) = captured_frame_stream(1).expect("prepared capture handoff");

    assert_no_alloc(|| {
        for sequence_number in 0..100 {
            assert_eq!(
                sender.try_send(frame(&pool, sequence_number)),
                CapturedFrameDelivery::Delivered
            );
            drop(stream.try_next().expect("enqueued capture frame"));
        }
    });

    assert_eq!(pool.available_slots(), 2);
}

#[test]
fn given_full_capture_handoff_when_newest_is_dropped_then_no_heap_allocation_occurs() {
    let pool = AudioBufferPool::new(2, 960);
    let (mut sender, mut stream) = captured_frame_stream(1).expect("prepared capture handoff");
    assert_eq!(
        sender.try_send(frame(&pool, 1)),
        CapturedFrameDelivery::Delivered
    );

    assert_no_alloc(|| {
        assert_eq!(
            sender.try_send(frame(&pool, 2)),
            CapturedFrameDelivery::DroppedNewest
        );
    });

    drop(stream.try_next().expect("oldest frame remains queued"));
    assert_eq!(pool.available_slots(), 2);
}

#[test]
fn given_closed_capture_start_gate_when_frame_arrives_then_discard_is_allocation_free() {
    let pool = AudioBufferPool::new(1, 960);
    let (_controller, gate) = capture_delivery_start_gate();
    let (mut sender, mut stream) =
        captured_frame_stream_with_start_gate(1, gate).expect("prepared gated handoff");

    assert_no_alloc(|| {
        assert_eq!(
            sender.try_send(frame(&pool, 1)),
            CapturedFrameDelivery::DiscardedBeforeStart
        );
    });

    assert!(stream.try_next().is_none());
    assert_eq!(pool.available_slots(), 1);
}

#[test]
fn given_exhausted_audio_pool_when_acquire_is_rejected_then_no_heap_allocation_occurs() {
    let pool = AudioBufferPool::new(1, 960);
    let held = pool.acquire().expect("first prepared slot");

    assert_no_alloc(|| {
        assert!(pool.acquire().is_none());
    });

    drop(held);
    assert_eq!(pool.available_slots(), 1);
    assert_eq!(pool.acquire_failures(), 1);
}
