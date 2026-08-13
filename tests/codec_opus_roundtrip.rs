use pocketstation::codec::{OpusDecoder, OpusEncoder, OPUS_MAX_PACKET_BYTES};
use pocketstation::internal::frame::{
    AudioBufferPool, AudioFrame, SampleFormat, SampleSpec, SourceId, StreamId, POOL_SLOT_SAMPLES,
};

#[test]
fn given_sine_frame_when_codec_roundtrip_runs_then_sample_count_is_preserved() {
    let pool = AudioBufferPool::new(4, POOL_SLOT_SAMPLES);
    let mut h = pool.acquire().unwrap();
    for (sample_index, sample) in h.as_mut_slice().iter_mut().enumerate() {
        let phase = sample_index as f32 * std::f32::consts::TAU * 440.0 / 48_000.0;
        *sample = phase.sin() * 0.25;
    }
    let frame = AudioFrame::try_new(
        StreamId::new(1),
        SourceId::new(1),
        0,
        0,
        SampleSpec::new(48_000, 1, SampleFormat::F32Interleaved),
        h,
    )
    .expect("valid codec frame");
    let mut enc = OpusEncoder::new().expect("OpusEncoder::new failed");
    let mut dec = OpusDecoder::new().expect("OpusDecoder::new failed");
    let mut encoded = Vec::with_capacity(OPUS_MAX_PACKET_BYTES);
    enc.encode_into(frame.samples(), &mut encoded)
        .expect("encode failed");
    let mut decoded = Vec::with_capacity(POOL_SLOT_SAMPLES);
    dec.decode_into(&encoded, &mut decoded, false)
        .expect("decode failed");
    assert_eq!(decoded.len(), POOL_SLOT_SAMPLES);
}
