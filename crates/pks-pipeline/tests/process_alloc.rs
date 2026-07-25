use assert_no_alloc::{assert_no_alloc, AllocDisabler};
use pks_frame::{AudioBufferPool, AudioFrame, SourceId, StreamId};
use pks_pipeline::{AudioProcessorNode, ResampleNode};

#[global_allocator]
static ALLOCATOR: AllocDisabler = AllocDisabler;

#[test]
fn given_preallocated_resampler_when_frame_is_processed_then_no_heap_allocation_occurs() {
    let pool = AudioBufferPool::new(1, 960);
    let mut buffer = pool.acquire().expect("setup pool must contain one slot");
    buffer.as_mut_slice().fill(0.25);
    buffer.set_len(960);
    let frame = AudioFrame::new(StreamId(1), SourceId(2), 3, 4, 1, buffer);
    let mut resampler = ResampleNode::new(48_000, 24_000, 1);

    let output =
        assert_no_alloc(|| resampler.process(frame)).expect("downsample must emit a frame");

    assert_eq!(output.buffer.len(), 480);
    assert_eq!(output.sample_rate_hz, 24_000);
}
