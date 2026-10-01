#![cfg(feature = "aec")]

use pocketstation::{
    AudioFrameDuration, EchoCancellationState, PlaybackReference, SampleFormat, SampleSpec, Session,
};
use std::{
    thread,
    time::{Duration, Instant},
};
#[path = "support/aec_source.rs"]
mod aec_source;
use aec_source::{AecSource, InputFrame, CLOCK_EPOCH_NS, FRAME_DURATION_NS, FRAME_SAMPLES};

fn input(sequence: u64, timestamp_ns: u64, samples: Vec<f32>) -> InputFrame {
    if samples.len() == FRAME_SAMPLES {
        InputFrame::new(sequence, timestamp_ns, samples)
    } else {
        InputFrame {
            samples,
            timestamp_ns,
            sequence_number: sequence,
            source_generation: 1,
            discontinuity_epoch: 0,
        }
    }
}

fn source(session: &Session, label: &str, channels: u8, samples: usize) -> AecSource {
    if channels == 1 && samples == FRAME_SAMPLES {
        AecSource::declare(session, label)
    } else {
        AecSource::declare_format(
            session,
            label,
            SampleSpec::new(48_000, channels, SampleFormat::F32Interleaved),
            samples,
        )
    }
}

#[test]
fn given_independent_channel_layouts_when_real_aec_runs_then_reference_is_preserved_and_echo_reduced(
) {
    // Explicit source timestamps and actual APM, without representing synthetic
    // echo as a physical speaker or device qualification.
    for duration in [AudioFrameDuration::Ms10, AudioFrameDuration::Ms20] {
        for (microphone_channels, reference_channels) in [(1, 2), (2, 1)] {
            let count = usize::from(duration.milliseconds()) * 48;
            let duration_ns = FRAME_DURATION_NS * u64::from(duration.milliseconds()) / 20;
            let session = Session::builder()
                .sample_spec(SampleSpec::new(
                    48_000,
                    reference_channels,
                    SampleFormat::F32Interleaved,
                ))
                .audio_frame_duration(duration)
                .build();
            let mut reference = source(&session, "reference", reference_channels, count);
            let mut microphone = source(&session, "microphone", microphone_channels, count);
            let processed = session
                .echo_cancel(
                    &microphone.output,
                    PlaybackReference::rendered_audio(&reference.output),
                )
                .unwrap();
            let endpoint = session.polled_audio().unwrap();
            reference.output.send(endpoint).unwrap();
            processed.audio().send(endpoint).unwrap();
            let mut running = session.start().unwrap();
            let mut random = 0x4178a237_u32;
            let frames = 4000 / u32::from(duration.milliseconds());
            let mut history = vec![0.0_f32; 240];
            let mut input_power = 0.0_f64;
            let mut output_power = 0.0_f64;
            for sequence in 0..frames {
                let mut render = Vec::with_capacity(count * usize::from(reference_channels));
                let mut capture = Vec::with_capacity(count * usize::from(microphone_channels));
                for sample in 0..count {
                    random ^= random << 13;
                    random ^= random >> 17;
                    random ^= random << 5;
                    let x = (random as f32 / u32::MAX as f32 - 0.5) * 0.4;
                    render.push(x);
                    if reference_channels == 2 {
                        render.push(-x);
                    }
                    let cursor = (sequence as usize * count + sample) % history.len();
                    let echo = history[cursor] * 0.65;
                    history[cursor] = x;
                    capture.push(echo);
                    if microphone_channels == 2 {
                        capture.push(echo * 0.7);
                    }
                }
                let timestamp = CLOCK_EPOCH_NS + u64::from(sequence) * duration_ns;
                reference.send(input(u64::from(sequence), timestamp, render.clone()));
                let deadline = Instant::now() + Duration::from_secs(2);
                while processed.observations().reference_queue_depth_frames == 0 {
                    assert!(
                        Instant::now() < deadline,
                        "reference admission: {:?}",
                        processed.observations()
                    );
                    thread::sleep(Duration::from_millis(1));
                }
                if sequence >= frames / 2 {
                    input_power += capture.iter().map(|&x| f64::from(x).powi(2)).sum::<f64>();
                }
                microphone.send(input(u64::from(sequence), timestamp, capture));
                let mut saw_raw = false;
                let mut saw_processed = false;
                while !saw_raw || !saw_processed {
                    if let Ok(batch) = running.try_poll_audio() {
                        for index in 0..batch.len() {
                            let frame = batch.frame(index).unwrap();
                            if frame.lineage().stem_id() == processed.audio().id() {
                                assert!(!saw_processed);
                                assert_eq!(frame.channels(), microphone_channels);
                                assert_eq!(
                                    frame.samples().len(),
                                    count * usize::from(microphone_channels)
                                );
                                assert!(frame.samples().iter().all(|x| x.is_finite()));
                                assert_eq!(
                                    frame.processing().unwrap().input_sequence_number,
                                    u64::from(sequence)
                                );
                                if sequence >= frames / 2 {
                                    output_power += frame
                                        .samples()
                                        .iter()
                                        .map(|&x| f64::from(x).powi(2))
                                        .sum::<f64>();
                                }
                                saw_processed = true;
                            } else {
                                assert!(!saw_raw);
                                assert_eq!(frame.channels(), reference_channels);
                                assert_eq!(frame.samples(), render);
                                assert!(frame.processing().is_none());
                                saw_raw = true;
                            }
                        }
                    }
                    assert!(
                        Instant::now() < deadline,
                        "delivery: {:?}",
                        processed.observations()
                    );
                    thread::sleep(Duration::from_millis(1));
                }
            }
            reference.close();
            microphone.close();
            assert!(running.stop().is_success());
            let observations = processed.observations();
            assert_eq!(
                observations.analyzed_reference_frames_total,
                u64::from(frames)
            );
            assert_eq!(
                observations.processed_microphone_frames_total,
                u64::from(frames)
            );
            assert_eq!(observations.resets_total, 0);
            assert_eq!(observations.state, EchoCancellationState::Stopped);
            let reduction_db = 10.0 * (input_power / output_power.max(1e-15)).log10();
            println!("mixed channels mic={microphone_channels} ref={reference_channels} duration={duration:?} reduction_db={reduction_db}");
            assert!(
                reduction_db > 10.0,
                "insufficient actual echo reduction: {reduction_db} dB"
            );
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum ReferenceSignal {
    Silent,
    AntiPhase,
    Independent,
}

#[test]
fn given_mixed_layouts_when_near_end_speech_overlaps_playback_then_voice_survives() {
    for duration in [AudioFrameDuration::Ms10, AudioFrameDuration::Ms20] {
        for reference_signal in [
            ReferenceSignal::Silent,
            ReferenceSignal::AntiPhase,
            ReferenceSignal::Independent,
        ] {
            let count = usize::from(duration.milliseconds()) * 48;
            let duration_ns = u64::from(duration.milliseconds()) * 1_000_000;
            let sample_count = 48_000 * 8;
            let session = Session::builder().audio_frame_duration(duration).build();
            let mut reference = source(&session, "reference", 2, count);
            let mut microphone = source(&session, "microphone", 1, count);
            let processed = session
                .echo_cancel(
                    &microphone.output,
                    PlaybackReference::rendered_audio(&reference.output),
                )
                .unwrap();
            processed
                .audio()
                .send(session.polled_audio().unwrap())
                .unwrap();
            let mut running = session.start().unwrap();
            let mut desired = Vec::with_capacity(sample_count);
            let mut render = Vec::with_capacity(sample_count);
            let seed = match reference_signal {
                ReferenceSignal::Independent => 0x9276ab53_u32,
                _ => 0x13aec138_u32,
            };
            let (mut random, mut phase, mut filtered) = (seed, 0.0_f32, 0.0_f32);
            let (mut right_random, mut right_filtered) = (0x76ca1973_u32, 0.0_f32);
            let mut right = Vec::with_capacity(sample_count);
            for index in 0..sample_count {
                random ^= random << 13;
                random ^= random >> 17;
                random ^= random << 5;
                filtered =
                    0.65 * filtered + 0.35 * (random as f64 / u32::MAX as f64 * 2.0 - 1.0) as f32;
                render.push(filtered * 0.35);
                right_random ^= right_random << 13;
                right_random ^= right_random >> 17;
                right_random ^= right_random << 5;
                right_filtered = 0.42 * right_filtered
                    + 0.58 * (right_random as f64 / u32::MAX as f64 * 2.0 - 1.0) as f32;
                right.push(right_filtered * 0.35);
                let time_s = index as f32 / 48_000.0;
                let frequency_hz = 173.0 + 43.0 * (time_s * 1.7).sin();
                phase = (phase + std::f32::consts::TAU * frequency_hz / 48_000.0)
                    % std::f32::consts::TAU;
                let envelope = 0.55 + 0.45 * (time_s * 4.3).sin().powi(2);
                desired.push(
                    envelope
                        * (0.14 * phase.sin()
                            + 0.07 * (2.0 * phase).sin()
                            + 0.035 * (3.0 * phase).sin()),
                );
            }
            let mut actual = Vec::with_capacity(sample_count);
            for start in (0..sample_count).step_by(count) {
                let sequence = (start / count) as u64;
                let timestamp = CLOCK_EPOCH_NS + sequence * duration_ns;
                let render_pcm = (start..start + count)
                    .flat_map(|index| match reference_signal {
                        ReferenceSignal::Silent => [0.0, 0.0],
                        ReferenceSignal::AntiPhase => [render[index], -render[index]],
                        ReferenceSignal::Independent => [render[index], right[index]],
                    })
                    .collect();
                let microphone_pcm = (start..start + count)
                    .map(|index| {
                        let echo = match reference_signal {
                            ReferenceSignal::Silent => 0.0,
                            ReferenceSignal::AntiPhase => index
                                .checked_sub(960)
                                .map_or(0.0, |past| render[past] * 0.6),
                            ReferenceSignal::Independent => {
                                index
                                    .checked_sub(960)
                                    .map_or(0.0, |past| render[past] * 0.6)
                                    + index
                                        .checked_sub(1440)
                                        .map_or(0.0, |past| right[past] * 0.4)
                            }
                        };
                        desired[index] + echo
                    })
                    .collect();
                reference.send(input(sequence, timestamp, render_pcm));
                let deadline = Instant::now() + Duration::from_secs(2);
                while processed.observations().reference_queue_depth_frames == 0 {
                    assert!(Instant::now() < deadline, "{:?}", processed.observations());
                    thread::sleep(Duration::from_millis(1));
                }
                microphone.send(input(sequence, timestamp, microphone_pcm));
                loop {
                    if let Ok(batch) = running.try_poll_audio() {
                        if let Some(frame) = batch.frame(0) {
                            assert_eq!(frame.channels(), 1);
                            assert_eq!(frame.samples().len(), count);
                            actual.extend_from_slice(frame.samples());
                            break;
                        }
                    }
                    assert!(Instant::now() < deadline, "{:?}", processed.observations());
                    thread::sleep(Duration::from_millis(1));
                }
            }
            reference.close();
            microphone.close();
            assert!(running.stop().is_success());
            assert_eq!(actual.len(), sample_count);
            let start = 48_000 * 6;
            let target = &desired[start..start + 12_000];
            let target_power = target.iter().map(|&x| f64::from(x).powi(2)).sum::<f64>();
            let (mut correlation, mut lag, mut gain_db, mut snr_db) = (-1.0, 0, 0.0, 0.0);
            for delay in 0..=960 {
                let output = &actual[start + delay..start + delay + target.len()];
                let output_power = output.iter().map(|&x| f64::from(x).powi(2)).sum::<f64>();
                let dot = target
                    .iter()
                    .zip(output)
                    .map(|(&a, &b)| f64::from(a) * f64::from(b))
                    .sum::<f64>();
                let candidate = dot / (target_power * output_power).sqrt().max(1e-12);
                if candidate > correlation {
                    let error = target
                        .iter()
                        .zip(output)
                        .map(|(&a, &b)| (f64::from(a) - f64::from(b)).powi(2))
                        .sum::<f64>();
                    correlation = candidate;
                    lag = delay;
                    gain_db = 10.0 * (output_power.max(1e-12) / target_power).log10();
                    snr_db = 10.0 * (target_power / error.max(1e-12)).log10();
                }
            }
            println!("mixed near-end {duration:?} reference={reference_signal:?}: lag={lag} gain_db={gain_db} correlation={correlation} snr_db={snr_db}");
            // Identical acceptance to the existing equal-layout near-end gate.
            assert!((-3.0..=3.0).contains(&gain_db));
            assert!(correlation >= 0.8);
            assert!(snr_db >= 6.0);
            assert!(lag < 960);
        }
    }
}

#[test]
fn given_mixed_layouts_when_first_and_last_samples_are_transients_then_startup_and_tail_survive() {
    for duration in [AudioFrameDuration::Ms10, AudioFrameDuration::Ms20] {
        let count = usize::from(duration.milliseconds()) * 48;
        let session = Session::builder().audio_frame_duration(duration).build();
        let mut reference = source(&session, "reference", 2, count);
        let mut microphone = source(&session, "microphone", 1, count);
        let processed = session
            .echo_cancel(
                &microphone.output,
                PlaybackReference::rendered_audio(&reference.output),
            )
            .unwrap();
        processed
            .audio()
            .send(session.polled_audio().unwrap())
            .unwrap();
        let mut running = session.start().unwrap();
        let mut capture = vec![0.0; count];
        capture[0] = 0.35;
        capture[count - 1] = 0.25;
        reference.send(input(0, CLOCK_EPOCH_NS, vec![0.0; 2 * count]));
        microphone.send(input(0, CLOCK_EPOCH_NS, capture));
        let deadline = Instant::now() + Duration::from_secs(2);
        let mut actual = Vec::new();
        loop {
            if let Ok(batch) = running.try_poll_audio() {
                if let Some(frame) = batch.frame(0) {
                    assert_eq!(frame.channels(), 1);
                    assert!(!frame.processing().unwrap().is_tail());
                    actual.extend_from_slice(frame.samples());
                    break;
                }
            }
            assert!(Instant::now() < deadline, "{:?}", processed.observations());
            thread::sleep(Duration::from_millis(1));
        }
        reference.close();
        microphone.close();
        assert!(running.stop().is_success());
        let receipt = running.audio_receipt();
        while let Ok(batch) = receipt.try_poll() {
            for index in 0..batch.len() {
                let frame = batch.frame(index).unwrap();
                assert_eq!(frame.channels(), 1);
                let metadata = frame.processing().unwrap();
                assert!(metadata.is_tail());
                assert_eq!(metadata.input_sequence_number, 0);
                assert_eq!(metadata.input_timestamp_ns, CLOCK_EPOCH_NS);
                actual.extend_from_slice(frame.samples());
            }
        }
        assert_eq!(actual.len(), count + 1920);
        // No delay cropping: keep both complete impulse responses around the
        // qualified engine's causal delay, including output after source EOF.
        assert!(
            actual[380..480].iter().any(|x| x.abs() > 0.1),
            "first transient lost"
        );
        assert!(
            actual[count + 380..count + 480]
                .iter()
                .any(|x| x.abs() > 0.07),
            "final transient lost"
        );
        let observations = processed.observations();
        assert_eq!(observations.processed_microphone_frames_total, 1);
        assert_eq!(observations.analyzed_reference_frames_total, 1);
        assert_eq!(observations.tail_padding_samples_total, 1920);
    }
}
