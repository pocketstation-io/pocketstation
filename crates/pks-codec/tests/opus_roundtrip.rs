use pks_codec::{OpusDecoder, OpusEncoder};
use pks_frame::{AudioBufferPool, AudioFrame, SourceId, StreamId, POOL_SLOT_SAMPLES};

#[test]
fn given_sine_frame_when_codec_roundtrip_runs_then_sample_count_is_preserved() {
    let pool = AudioBufferPool::new(4, POOL_SLOT_SAMPLES);
    let mut h = pool.acquire().unwrap();
    for (sample_index, sample) in h.as_mut_slice().iter_mut().enumerate() {
        let phase = sample_index as f32 * std::f32::consts::TAU * 440.0 / 48_000.0;
        *sample = phase.sin() * 0.25;
    }
    let frame = AudioFrame::new(StreamId(1), SourceId(1), 0, 0, 1, h);
    let mut enc = OpusEncoder::new().expect("OpusEncoder::new failed");
    let mut dec = OpusDecoder::new().expect("OpusDecoder::new failed");
    let encoded = enc.encode(&frame).expect("encode failed");
    let decoded = dec.decode_to_vec(&encoded).expect("decode_to_vec failed");
    assert_eq!(decoded.len(), POOL_SLOT_SAMPLES);
}
