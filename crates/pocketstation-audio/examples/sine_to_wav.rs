use pocketstation_audio::*;

fn main() {
    let pool = AudioBufferPool::new(64, DEFAULT_SLOT_SAMPLES_MONO_20MS);
    // Ring sized to match the pool so all 50 frames fit without backpressure.
    let (mut prod, mut cons) = frame_bus(64);
    let mut encoder = MockOpusEncoder::default();
    let mut decoder = MockOpusDecoder::default();
    let mut decoded_all = Vec::new();

    for seq in 0..50u64 {
        // 1 second at 20 ms frames
        let mut handle = pool.acquire().expect("pool exhausted in example");
        fill_sine(handle.as_mut_slice(), DEFAULT_SAMPLE_RATE, 440.0, seq * 960);
        let frame = AudioFrame::new(StreamId(1), SourceId(1), seq, seq * 20_000_000, 1, handle);
        // push_drop_newest only drops if the ring is full; ring is sized to
        // prevent that in this single-threaded example.
        let _ = prod.push_drop_newest(frame);
    }

    while let Some(frame) = cons.pop() {
        let encoded = encoder.encode(&frame);
        let decoded = decoder.decode_to_vec(&encoded);
        decoded_all.extend_from_slice(&decoded);
    }

    write_wav_mono_48k("pocketstation-sine.wav", &decoded_all).expect("write wav");
    println!(
        "wrote pocketstation-sine.wav with {} samples",
        decoded_all.len()
    );
}
