#![cfg(feature = "aec")]

use pocketstation::{
    AudioFrameDuration, AudioInputConfig, EchoCancellationState, PlaybackReference, SampleFormat,
    SampleSpec, Session,
};
use std::{
    thread,
    time::{Duration, Instant},
};

fn configuration(samples: usize) -> AudioInputConfig {
    AudioInputConfig::new(
        SampleSpec::new(48_000, 1, SampleFormat::F32Interleaved),
        8,
        samples,
    )
    .unwrap()
}

#[test]
fn given_paired_inputs_when_session_executes_then_aec_and_independent_raw_audio_are_delivered() {
    for (duration_ms, channels) in [(10, 1), (20, 1), (10, 2)] {
        let samples = duration_ms as usize * 48;
        let spec = SampleSpec::new(48_000, channels, SampleFormat::F32Interleaved);
        let session = Session::builder()
            .sample_spec(spec)
            .audio_frame_duration(if duration_ms == 10 {
                AudioFrameDuration::Ms10
            } else {
                AudioFrameDuration::Ms20
            })
            .build();
        let mut reference = session
            .audio_input(AudioInputConfig::new(spec, 8, samples).unwrap())
            .unwrap();
        let mut microphone = session
            .audio_input(AudioInputConfig::new(spec, 8, samples).unwrap())
            .unwrap();
        let reference_id = reference.source().source_id();
        let mic_id = microphone.source().source_id();
        let processed = session
            .echo_cancel(
                microphone.output(),
                PlaybackReference::rendered_audio(reference.output()),
            )
            .unwrap();
        let endpoint = session.polled_audio().unwrap();
        reference.output().send(endpoint).unwrap();
        processed.audio().send(endpoint).unwrap();
        let mut running = session.start().unwrap();
        let mut random = 0x917ba33_u32;
        let mut previous = vec![0.0_f32; samples * usize::from(channels)];
        let mut input_power = 0.0_f64;
        let mut output_power = 0.0_f64;
        for index in 0..300 {
            let reference_pcm: Vec<f32> = (0..samples)
                .map(|_| {
                    random ^= random << 13;
                    random ^= random >> 17;
                    random ^= random << 5;
                    (random as f64 / u32::MAX as f64 * 2.0 - 1.0) as f32 * 0.1
                })
                .flat_map(|sample| [sample, -sample].into_iter().take(usize::from(channels)))
                .collect();
            // Opposite-polarity stereo remains audible with unequal speaker
            // transfer functions. Averaging this reference would erase it.
            let mic_pcm: Vec<_> = previous
                .chunks_exact(usize::from(channels))
                .flat_map(|frame| {
                    let echo = frame[0] * 0.6 + frame.get(1).copied().unwrap_or(0.0) * 0.1;
                    std::iter::repeat_n(echo, usize::from(channels))
                })
                .collect();
            // Reverse callback arrival order without changing source-time cadence.
            if index % 2 == 0 {
                reference.try_write(&reference_pcm).unwrap();
                microphone.try_write(&mic_pcm).unwrap();
            } else {
                microphone.try_write(&mic_pcm).unwrap();
                reference.try_write(&reference_pcm).unwrap();
            }
            let deadline = Instant::now() + Duration::from_secs(2);
            let (mut saw_raw, mut saw_aec) = (false, false);
            while !(saw_raw && saw_aec) {
                if let Ok(batch) = running.try_poll_audio() {
                    for i in 0..batch.len() {
                        let frame = batch.frame(i).unwrap();
                        if frame.lineage().source_id() == reference_id {
                            assert!(!saw_raw);
                            assert_eq!(frame.samples(), reference_pcm);
                            saw_raw = true;
                        } else {
                            assert_eq!(frame.lineage().stem_id(), processed.audio().id());
                            assert!(!saw_aec);
                            assert_eq!(frame.samples().len(), samples * usize::from(channels));
                            assert!(frame.samples().iter().all(|x| x.is_finite()));
                            if index >= 200 {
                                output_power += frame
                                    .samples()
                                    .iter()
                                    .map(|x| f64::from(*x).powi(2))
                                    .sum::<f64>();
                            }
                            saw_aec = true;
                        }
                    }
                }
                assert!(
                    Instant::now() < deadline,
                    "Session did not deliver both branches at frame {index}; raw={saw_raw}, aec={saw_aec}, observations={:?}", processed.observations()
                );
                if !(saw_raw && saw_aec) {
                    thread::sleep(Duration::from_millis(1));
                }
            }
            if index >= 200 {
                input_power += mic_pcm.iter().map(|x| f64::from(*x).powi(2)).sum::<f64>();
            }
            previous = reference_pcm;
        }
        assert!(input_power > 1.0);
        assert!(
            output_power < input_power * 0.5,
            "real cancellation was not executed: {output_power}/{input_power}"
        );
        assert_eq!(processed.observations().microphone_source_id, Some(mic_id));
        assert_eq!(
            processed.observations().reference_source_id,
            Some(reference_id)
        );
        reference.close();
        microphone.close();
        assert!(running.stop().is_success());
        assert_eq!(
            processed.observations().state,
            EchoCancellationState::Stopped
        );
    }
}

#[test]
fn given_builtin_aec_when_only_application_audio_is_declared_then_no_microphone_is_required() {
    let session = Session::new();
    let mut application = session.audio_input(configuration(960)).unwrap();
    application
        .output()
        .send(session.polled_audio().unwrap())
        .unwrap();
    let mut running = session.start().unwrap();
    application.try_write(&vec![0.125; 960]).unwrap();
    let deadline = Instant::now() + Duration::from_secs(2);
    loop {
        if let Ok(batch) = running.try_poll_audio() {
            if let Some(frame) = batch.frame(0) {
                assert_eq!(frame.samples(), vec![0.125; 960]);
                break;
            }
        }
        assert!(Instant::now() < deadline);
        thread::sleep(Duration::from_millis(1));
    }
    application.close();
    assert!(running.stop().is_success());
}

#[test]
fn given_foreign_or_identical_inputs_when_declaring_echo_cancellation_then_rejected_without_poisoning_the_session(
) {
    let session = Session::new();
    let other = Session::new();
    let mut mic = session.audio_input(configuration(960)).unwrap();
    let mut reference = session.audio_input(configuration(960)).unwrap();
    let foreign = other.audio_input(configuration(960)).unwrap();
    assert!(session
        .echo_cancel(
            mic.output(),
            PlaybackReference::rendered_audio(foreign.output())
        )
        .is_err());
    assert!(session
        .echo_cancel(
            mic.output(),
            PlaybackReference::rendered_audio(mic.output())
        )
        .is_err());
    let processed = session
        .echo_cancel(
            mic.output(),
            PlaybackReference::rendered_audio(reference.output()),
        )
        .unwrap();
    processed
        .audio()
        .send(session.polled_audio().unwrap())
        .unwrap();
    mic.close();
    reference.close();
    let mut running = session.start().unwrap();
    assert!(running.stop().is_success());
}

#[test]
fn given_missing_reference_when_capacity_is_reached_then_failure_is_retained_and_application_continues(
) {
    let session = Session::new();
    let mut microphone = session.audio_input(configuration(960)).unwrap();
    let mut reference = session.audio_input(configuration(960)).unwrap();
    let mut application = session.audio_input(configuration(960)).unwrap();
    let processed = session
        .echo_cancel(
            microphone.output(),
            PlaybackReference::rendered_audio(reference.output()),
        )
        .unwrap();
    let endpoint = session.polled_audio().unwrap();
    processed.audio().send(endpoint).unwrap();
    application.output().send(endpoint).unwrap();
    let mut running = session.start().unwrap();
    let capacity = processed.observations().queue_capacity_frames;
    // Wait for each accepted input, so this tests the unmatched-reference limit,
    // not an unrelated capture queue overflow.
    for index in 0..=capacity {
        microphone.try_write(&vec![0.1; 960]).unwrap();
        let deadline = Instant::now() + Duration::from_secs(2);
        loop {
            let observed = processed.observations();
            if observed.microphone_queue_depth_frames == index + 1
                || observed.state == EchoCancellationState::Failed
            {
                break;
            }
            assert!(Instant::now() < deadline, "{observed:?}");
            thread::sleep(Duration::from_millis(1));
        }
    }
    let failed = processed.observations();
    assert_eq!(failed.state, EchoCancellationState::Failed);
    assert!(failed
        .last_error
        .as_ref()
        .unwrap()
        .contains("capacity exhausted"));
    assert_eq!(failed.processed_microphone_frames_total, 0);
    assert!(failed.microphone_queue_depth_frames <= capacity);
    application.try_write(&vec![0.25; 960]).unwrap();
    let deadline = Instant::now() + Duration::from_secs(2);
    loop {
        if let Ok(batch) = running.try_poll_audio() {
            if let Some(frame) = batch.frame(0) {
                assert_eq!(
                    frame.lineage().source_id(),
                    application.source().source_id()
                );
                assert_eq!(frame.samples(), vec![0.25; 960]);
                break;
            }
        }
        assert!(Instant::now() < deadline);
        thread::sleep(Duration::from_millis(1));
    }
    microphone.close();
    reference.close();
    application.close();
    assert!(!running.stop().is_success());
    assert_eq!(
        processed.observations().state,
        EchoCancellationState::Failed
    );
    assert_eq!(processed.observations().last_error, failed.last_error);
}

#[test]
fn given_near_end_and_double_talk_when_session_processes_then_voice_level_and_timing_are_measured()
{
    // Synthetic voiced excitation, not physical speech qualification. Preserve
    // the reference-engine thresholds; never normalize gain in the scorer.
    for (duration_ms, channels, echo_gain_linear) in
        [(10, 1, 0.0), (20, 1, 0.0), (10, 2, 0.0), (10, 1, 0.6)]
    {
        let frame_samples = duration_ms as usize * 48;
        let sample_count = 48_000 * 12;
        let spec = SampleSpec::new(48_000, channels, SampleFormat::F32Interleaved);
        let session = Session::builder()
            .sample_spec(spec)
            .audio_frame_duration(if duration_ms == 10 {
                AudioFrameDuration::Ms10
            } else {
                AudioFrameDuration::Ms20
            })
            .build();
        let mut microphone = session
            .audio_input(AudioInputConfig::new(spec, 8, frame_samples).unwrap())
            .unwrap();
        let mut reference = session
            .audio_input(AudioInputConfig::new(spec, 8, frame_samples).unwrap())
            .unwrap();
        let processed = session
            .echo_cancel(
                microphone.output(),
                PlaybackReference::rendered_audio(reference.output()),
            )
            .unwrap();
        processed
            .audio()
            .send(session.polled_audio().unwrap())
            .unwrap();
        let mut running = session.start().unwrap();
        let mut desired = Vec::with_capacity(sample_count);
        let mut render = Vec::with_capacity(sample_count);
        let (mut random, mut phase_rad, mut filtered_linear) = (0x13aec138_u32, 0.0_f32, 0.0_f32);
        for sample_index in 0..sample_count {
            random ^= random << 13;
            random ^= random >> 17;
            random ^= random << 5;
            filtered_linear = 0.65 * filtered_linear
                + 0.35 * (random as f64 / u32::MAX as f64 * 2.0 - 1.0) as f32;
            render.push(filtered_linear * 0.35);
            let time_s = sample_index as f32 / 48_000.0;
            let frequency_hz = 173.0 + 43.0 * (time_s * 1.7).sin();
            phase_rad = (phase_rad + std::f32::consts::TAU * frequency_hz / 48_000.0)
                % std::f32::consts::TAU;
            let envelope_ratio = 0.55 + 0.45 * (time_s * 4.3).sin().powi(2);
            desired.push(
                envelope_ratio
                    * (0.14 * phase_rad.sin()
                        + 0.07 * (2.0 * phase_rad).sin()
                        + 0.035 * (3.0 * phase_rad).sin()),
            );
        }
        let mut actual = Vec::with_capacity(sample_count);
        for start_sample in (0..sample_count).step_by(frame_samples) {
            let render_pcm: Vec<f32> = render[start_sample..start_sample + frame_samples]
                .iter()
                .flat_map(|value| {
                    std::iter::repeat_n(
                        if echo_gain_linear == 0.0 { 0.0 } else { *value },
                        channels as usize,
                    )
                })
                .collect();
            let microphone_pcm: Vec<f32> = (start_sample..start_sample + frame_samples)
                .flat_map(|index| {
                    std::iter::repeat_n(
                        desired[index]
                            + index
                                .checked_sub(960)
                                .map_or(0.0, |past| render[past] * echo_gain_linear),
                        channels as usize,
                    )
                })
                .collect();
            reference.try_write(&render_pcm).unwrap();
            microphone.try_write(&microphone_pcm).unwrap();
            let deadline = Instant::now() + Duration::from_secs(2);
            loop {
                if let Ok(batch) = running.try_poll_audio() {
                    if let Some(frame) = batch.frame(0) {
                        assert_eq!(frame.lineage().stem_id(), processed.audio().id());
                        assert_eq!(frame.samples().len(), frame_samples * channels as usize);
                        actual.extend(
                            frame
                                .samples()
                                .chunks_exact(channels as usize)
                                .map(|frame| frame[0]),
                        );
                        break;
                    }
                }
                assert!(Instant::now() < deadline, "{:?}", processed.observations());
                thread::sleep(Duration::from_millis(1));
            }
        }
        microphone.close();
        reference.close();
        assert!(running.stop().is_success());
        assert_eq!(actual.len(), sample_count);
        // Compare after six seconds of adaptation. A short held-out interval
        // keeps this integration gate fast while retaining sample-level delay.
        let start_sample = 48_000 * 6;
        let comparison_samples = 12_000;
        let target = &desired[start_sample..start_sample + comparison_samples];
        let target_power = target
            .iter()
            .map(|value| f64::from(*value).powi(2))
            .sum::<f64>();
        let (mut best_correlation_ratio, mut best_delay_samples, mut best_gain_db, mut best_snr_db) =
            (-1.0, 0, 0.0, 0.0);
        for delay_samples in 0..=960 {
            let output = &actual
                [start_sample + delay_samples..start_sample + delay_samples + comparison_samples];
            let output_power = output
                .iter()
                .map(|value| f64::from(*value).powi(2))
                .sum::<f64>();
            let dot = target
                .iter()
                .zip(output)
                .map(|(a, b)| f64::from(*a) * f64::from(*b))
                .sum::<f64>();
            let correlation_ratio = dot / (target_power * output_power).sqrt().max(1e-12);
            if correlation_ratio > best_correlation_ratio {
                let error_power = target
                    .iter()
                    .zip(output)
                    .map(|(a, b)| (f64::from(*a) - f64::from(*b)).powi(2))
                    .sum::<f64>();
                best_correlation_ratio = correlation_ratio;
                best_delay_samples = delay_samples;
                best_gain_db = 10.0 * (output_power.max(1e-12) / target_power).log10();
                best_snr_db = 10.0 * (target_power / error_power.max(1e-12)).log10();
            }
        }
        println!("AEC Session: frame={duration_ms}ms channels={channels} echo_gain={echo_gain_linear}; delay={best_delay_samples} samples gain={best_gain_db:.3}dB correlation={best_correlation_ratio:.4} snr={best_snr_db:.3}dB");
        assert!((-3.0..=3.0).contains(&best_gain_db));
        assert!(best_correlation_ratio >= 0.8);
        assert!(best_snr_db >= 6.0);
        assert!(best_delay_samples < 960, "delay search exhausted");
    }
}

#[test]
fn given_first_microphone_transient_when_session_starts_then_formats_are_ready_before_real_audio() {
    for (frame_duration, channels) in [
        (AudioFrameDuration::Ms10, 1),
        (AudioFrameDuration::Ms20, 1),
        (AudioFrameDuration::Ms10, 2),
        (AudioFrameDuration::Ms20, 2),
    ] {
        let samples = usize::from(frame_duration.milliseconds()) * 48;
        let spec = SampleSpec::new(48_000, channels, SampleFormat::F32Interleaved);
        let session = Session::builder()
            .sample_spec(spec)
            .audio_frame_duration(frame_duration)
            .build();
        let config = AudioInputConfig::new(spec, 8, samples).unwrap();
        let mut reference = session.audio_input(config).unwrap();
        let mut microphone = session.audio_input(config).unwrap();
        let processed = session
            .echo_cancel(
                microphone.output(),
                PlaybackReference::rendered_audio(reference.output()),
            )
            .unwrap();
        processed
            .audio()
            .send(session.polled_audio().unwrap())
            .unwrap();
        let mut running = session.start().unwrap();
        let mut impulse = vec![0.0; samples * usize::from(channels)];
        for (channel, value) in impulse.iter_mut().take(usize::from(channels)).enumerate() {
            *value = if channel == 0 { 0.35 } else { -0.21 };
        }
        reference.try_write(&vec![0.0; impulse.len()]).unwrap();
        microphone.try_write(&impulse).unwrap();
        let deadline = Instant::now() + Duration::from_secs(2);
        loop {
            if let Ok(batch) = running.try_poll_audio() {
                if let Some(frame) = batch.frame(0) {
                    for channel in 0..usize::from(channels) {
                        let peak_sample = (0..samples)
                            .max_by(|a, b| {
                                frame.samples()[a * usize::from(channels) + channel]
                                    .abs()
                                    .total_cmp(
                                        &frame.samples()[b * usize::from(channels) + channel].abs(),
                                    )
                            })
                            .unwrap();
                        // A first-capture format reset previously lost the real
                        // reference and changed this peak to 238 samples.
                        assert!(
                            (428..=436).contains(&peak_sample),
                            "first peak {peak_sample}"
                        );
                        assert!(
                            frame.samples()[peak_sample * usize::from(channels) + channel].abs()
                                > 0.1
                        );
                    }
                    break;
                }
            }
            assert!(Instant::now() < deadline, "{:?}", processed.observations());
            thread::sleep(Duration::from_millis(1));
        }
        reference.close();
        microphone.close();
        assert!(running.stop().is_success());
        let observed = processed.observations();
        assert_eq!(observed.processed_microphone_frames_total, 1);
        assert_eq!(observed.analyzed_reference_frames_total, 1);
        assert_eq!(observed.resets_total, 0);
        assert!(observed.last_error.is_none());
    }
}
