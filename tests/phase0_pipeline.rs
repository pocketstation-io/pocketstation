use pocketstation_audio::*;

#[test]
fn sine_to_mock_codec_roundtrip() {
    let pool = AudioBufferPool::new(4, DEFAULT_SLOT_SAMPLES_MONO_20MS);
    let mut h = pool.acquire().unwrap();
    fill_sine(h.as_mut_slice(), 48_000, 440.0, 0);
    let frame = AudioFrame::new(StreamId(1), SourceId(1), 0, 0, 1, h);
    let mut enc = OpusEncoder::new().expect("OpusEncoder::new failed");
    let mut dec = OpusDecoder::new().expect("OpusDecoder::new failed");
    let encoded = enc.encode(&frame).expect("encode failed");
    let decoded = dec.decode_to_vec(&encoded).expect("decode_to_vec failed");
    assert_eq!(decoded.len(), DEFAULT_SLOT_SAMPLES_MONO_20MS);
}
