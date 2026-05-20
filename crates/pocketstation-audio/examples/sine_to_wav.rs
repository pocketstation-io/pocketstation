use pocketstation_audio::*;

fn main() {
    let pool = AudioBufferPool::new(64, DEFAULT_SLOT_SAMPLES_MONO_20MS);
    let (mut prod, mut cons) = frame_bus(8);
    let mut encoder = MockOpusEncoder;
    let mut decoder = MockOpusDecoder;
    let mut decoded_all = Vec::new();

    for seq in 0..50u64 { // 1 second at 20ms
        let mut handle = pool.acquire().expect("pool exhausted in example");
        fill_sine(handle.as_mut_slice(), 48_000, 440.0, seq * 960);
        let frame = AudioFrame::new(StreamId(1), SourceId(1), seq, seq * 20_000_000, 1, handle);
        prod.push_drop_newest(frame).expect("ring full in example");
    }

    while let Some(frame) = cons.pop() {
        let encoded = encoder.encode(&frame);
        let decoded = decoder.decode_to_vec(&encoded);
        decoded_all.extend_from_slice(&decoded);
    }

    write_wav_mono_48k("pocketstation-sine.wav", &decoded_all).expect("write wav");
    println!("wrote pocketstation-sine.wav with {} samples", decoded_all.len());
}
