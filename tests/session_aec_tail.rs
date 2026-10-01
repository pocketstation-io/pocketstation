#![cfg(feature = "aec")]

use pocketstation::{
    AudioFrameDuration, AudioInputConfig, PlaybackReference, SampleFormat, SampleSpec, Session,
};
use std::{
    fs, thread,
    time::{Duration, Instant},
};

#[test]
fn given_last_microphone_transient_when_graceful_finish_then_recording_retains_tail_and_input_origin(
) {
    for (duration, channels) in [
        (AudioFrameDuration::Ms10, 1),
        (AudioFrameDuration::Ms20, 1),
        (AudioFrameDuration::Ms10, 2),
        (AudioFrameDuration::Ms20, 2),
    ] {
        let root = tempfile::tempdir().unwrap();
        let samples = usize::from(duration.milliseconds()) * 48;
        let spec = SampleSpec::new(48_000, channels, SampleFormat::F32Interleaved);
        let session = Session::builder()
            .sample_spec(spec)
            .audio_frame_duration(duration)
            .recording_root(root.path())
            .build();
        let mut reference = session
            .audio_input(AudioInputConfig::new(spec, 8, samples).unwrap())
            .unwrap();
        let mut microphone = session
            .audio_input(AudioInputConfig::new(spec, 8, samples).unwrap())
            .unwrap();
        let input_id = microphone.source().source_id();
        let processed = session
            .echo_cancel(
                microphone.output(),
                PlaybackReference::rendered_audio(reference.output()),
            )
            .unwrap();
        let endpoint = session.polled_audio().unwrap();
        processed.audio().send(endpoint).unwrap();
        processed.audio().record("processed").unwrap();
        microphone.output().record("raw").unwrap();
        reference.output().record("reference").unwrap();
        let mut running = session.start().unwrap();
        // First AND last transient: cropping nominal delay would lose the first;
        // omitting native drain would lose the last. These are actual APM samples.
        let mut input = vec![0.0_f32; samples * usize::from(channels)];
        for channel in 0..usize::from(channels) {
            input[channel] = 0.35;
            input[(samples - 1) * usize::from(channels) + channel] = 0.25;
        }
        reference.try_write(&vec![0.0; input.len()]).unwrap();
        microphone.try_write(&input).unwrap();
        let deadline = Instant::now() + Duration::from_secs(2);
        let (input_sequence, input_time);
        loop {
            if let Ok(batch) = running.try_poll_audio() {
                if let Some(frame) = batch.frame(0) {
                    let metadata = frame
                        .processing()
                        .expect("frame provenance survives audio reentry");
                    assert_eq!(metadata.input_source_id, input_id);
                    assert_ne!(frame.lineage().source_id(), input_id);
                    assert!(!metadata.is_tail());
                    assert_eq!(metadata.nominal_delay_samples, 432);
                    assert_eq!(
                        metadata.input_duration_ns,
                        u64::from(duration.milliseconds()) * 1_000_000
                    );
                    input_sequence = metadata.input_sequence_number;
                    input_time = metadata.input_timestamp_ns;
                    break;
                }
            }
            assert!(Instant::now() < deadline, "{:?}", processed.observations());
            thread::sleep(Duration::from_millis(1));
        }
        reference.close();
        microphone.close();
        let stop = running.stop();
        assert!(stop.is_success(), "{stop:?}");
        let obs = processed.observations();
        let tail_frames = 40 / u64::from(duration.milliseconds());
        assert_eq!(obs.processed_microphone_frames_total, 1);
        assert_eq!(obs.analyzed_reference_frames_total, 1);
        assert_eq!(obs.output_frames_total, 1 + tail_frames);
        assert_eq!(obs.tail_frames_total, tail_frames);
        assert_eq!(obs.tail_padding_samples_total, 1920);
        assert_eq!(obs.discarded_tail_generations_total, 0);
        let receipt = running.audio_receipt();
        let mut delivered_tail = 0;
        while let Ok(batch) = receipt.try_poll() {
            for index in 0..batch.len() {
                let frame = batch.frame(index).unwrap();
                let metadata = frame.processing().unwrap();
                assert!(metadata.is_tail());
                assert_eq!(metadata.input_sequence_number, input_sequence);
                assert_eq!(metadata.input_timestamp_ns, input_time);
                assert_eq!(
                    metadata.tail_offset_samples,
                    delivered_tail * samples as u32
                );
                delivered_tail += 1;
            }
        }
        assert_eq!(u64::from(delivered_tail), tail_frames);
        assert_eq!(receipt.observations().queue_depth_frames, 0);
        assert_eq!(receipt.observations().queue_capacity_frames, 0);
        assert_eq!(receipt.observations().registered_endpoints, 0);
        let directory = fs::read_dir(root.path())
            .unwrap()
            .next()
            .unwrap()
            .unwrap()
            .path();
        let events: Vec<serde_json::Value> =
            fs::read_to_string(directory.join("events/processing-processed.jsonl"))
                .unwrap()
                .lines()
                .map(|line| serde_json::from_str(line).unwrap())
                .collect();
        assert_eq!(events.len() as u64, 1 + tail_frames);
        assert!(!directory.join("events/processing-raw.jsonl").exists());
        for (index, event) in events.iter().enumerate() {
            assert_eq!(event["input_source_id"], input_id.get());
            assert_eq!(event["input_sequence_number"], input_sequence);
            assert_eq!(event["input_timestamp_ns"], input_time);
            assert_eq!(event["output_sequence_number"], index);
            assert_eq!(
                event["output_timestamp_ns"],
                input_time + index as u64 * u64::from(duration.milliseconds()) * 1_000_000
            );
            assert_eq!(
                event["padding_samples"],
                if index == 0 { 0 } else { samples }
            );
            assert_eq!(
                event["tail_offset_samples"],
                index.saturating_sub(1) * samples
            );
        }
        let recorded: Vec<f32> = hound::WavReader::open(directory.join("stems/processed.wav"))
            .unwrap()
            .samples()
            .map(Result::unwrap)
            .collect();
        let raw: Vec<f32> = hound::WavReader::open(directory.join("stems/raw.wav"))
            .unwrap()
            .samples()
            .map(Result::unwrap)
            .collect();
        assert_eq!(raw, input);
        assert_eq!(recorded.len(), (samples + 1920) * usize::from(channels));
        for channel in 0..usize::from(channels) {
            for (start, end) in [(420, 445), (samples + 420, samples + 445)] {
                let peak = (start..end)
                    .map(|sample| recorded[sample * usize::from(channels) + channel].abs())
                    .fold(0.0_f32, f32::max);
                assert!(peak > 0.1, "first/final transient missing: {peak}");
            }
        }
    }
}
