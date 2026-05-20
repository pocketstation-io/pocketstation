use pocketstation_audio::*;

#[test]
fn sine_to_mock_codec_roundtrip() {
    let pool = AudioBufferPool::new(4, DEFAULT_SLOT_SAMPLES_MONO_20MS);
    let mut h = pool.acquire().unwrap();
    fill_sine(h.as_mut_slice(), 48_000, 440.0, 0);
    let frame = AudioFrame::new(StreamId(1), SourceId(1), 0, 0, 1, h);
    let mut enc = MockOpusEncoder;
    let mut dec = MockOpusDecoder;
    let encoded = enc.encode(&frame);
    let decoded = dec.decode_to_vec(&encoded);
    assert_eq!(decoded.len(), DEFAULT_SLOT_SAMPLES_MONO_20MS);
}
