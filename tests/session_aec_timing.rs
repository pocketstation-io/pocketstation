#![cfg(feature = "aec")]

use pocketstation::{
    AudioInputConfig, EchoCancellationState, EchoCancelledAudio, PlaybackReference, RunningSession,
    SampleFormat, SampleSpec, Session,
};
use std::{
    thread,
    time::{Duration, Instant},
};

#[path = "support/aec_source.rs"]
mod aec_source;
use aec_source::{AecSource, InputFrame, CLOCK_EPOCH_NS, FRAME_DURATION_NS, FRAME_SAMPLES};

#[test]
fn given_independent_source_start_times_when_equal_length_streams_end_then_no_extra_reference_frame_is_required(
) {
    let session = Session::new();
    let spec = SampleSpec::new(48_000, 1, SampleFormat::F32Interleaved);
    let config = AudioInputConfig::new(spec, 8, 960).unwrap();
    let mut reference = session.audio_input(config).unwrap();
    let mut microphone = session.audio_input(config).unwrap();
    let clean = session
        .echo_cancel(
            microphone.output(),
            PlaybackReference::rendered_audio(reference.output()),
        )
        .unwrap();
    clean.audio().send(session.polled_audio().unwrap()).unwrap();
    let mut running = session.start().unwrap();

    // First writes establish independent source clocks. Each stream contains
    // exactly forty frames, so ordinary EOF cannot rely on future reference.
    reference.try_write(&[0.0; 960]).unwrap();
    thread::sleep(Duration::from_millis(7));
    for index in 0..40 {
        microphone.try_write(&[0.05; 960]).unwrap();
        let deadline = Instant::now() + Duration::from_secs(2);
        loop {
            if let Ok(batch) = running.try_poll_audio() {
                if let Some(frame) = batch.frame(0) {
                    assert_eq!(frame.samples().len(), 960);
                    assert_eq!(frame.lineage().stem_id(), clean.audio().id());
                    assert!(frame.samples().iter().all(|sample| sample.is_finite()));
                    break;
                }
            }
            assert!(Instant::now() < deadline, "{:?}", clean.observations());
            thread::sleep(Duration::from_millis(1));
        }
        let observations = clean.observations();
        assert_eq!(observations.processed_microphone_frames_total, index + 1);
        assert!(observations.latest_reference_age_ns > 1_000_000);
        assert!(observations.latest_reference_age_ns < 80_000_000);
        assert_eq!(observations.resets_total, 0);
        assert_eq!(observations.discarded_microphone_frames_total, 0);
        if index < 39 {
            reference.try_write(&[0.0; 960]).unwrap();
        }
    }
    microphone.close();
    reference.close();
    assert!(running.stop().is_success());
    assert_eq!(clean.observations().state, EchoCancellationState::Stopped);
    assert_eq!(clean.observations().analyzed_reference_frames_total, 40);
}

fn fixture() -> (AecSource, AecSource, EchoCancelledAudio, RunningSession) {
    let session = Session::new();
    let reference = AecSource::declare(&session, "reference");
    let microphone = AecSource::declare(&session, "microphone");
    let clean = session
        .echo_cancel(
            &microphone.output,
            PlaybackReference::rendered_audio(&reference.output),
        )
        .unwrap();
    clean.audio().send(session.polled_audio().unwrap()).unwrap();
    let running = session.start().unwrap();
    (reference, microphone, clean, running)
}

fn receive_frame(
    running: &mut RunningSession,
    clean: &EchoCancelledAudio,
    timestamp_ns: u64,
) -> Vec<f32> {
    let deadline = Instant::now() + Duration::from_secs(2);
    loop {
        if let Ok(batch) = running.try_poll_audio() {
            if let Some(frame) = batch.frame(0) {
                assert_eq!(
                    batch.len(),
                    1,
                    "one processed frame per submitted microphone"
                );
                assert_eq!(frame.lineage().stem_id(), clean.audio().id());
                assert_eq!(frame.lineage().timestamp_start_ns(), timestamp_ns);
                assert_eq!(frame.samples().len(), FRAME_SAMPLES);
                assert!(frame.samples().iter().all(|sample| sample.is_finite()));
                return frame.samples().to_vec();
            }
        }
        let observations = clean.observations();
        assert_ne!(
            observations.state,
            EchoCancellationState::Failed,
            "{observations:?}"
        );
        assert!(Instant::now() < deadline, "{observations:?}");
        thread::sleep(Duration::from_millis(1));
    }
}

fn submit_reference(reference: &AecSource, clean: &EchoCancelledAudio, frame: InputFrame) {
    let before = clean.observations();
    reference.send(frame);
    // Source workers are independent. Wait for admission so these cases vary
    // source clocks without also introducing uncontrolled callback reordering.
    let deadline = Instant::now() + Duration::from_secs(2);
    loop {
        let current = clean.observations();
        if current.reference_queue_depth_frames > before.reference_queue_depth_frames {
            break;
        }
        assert_ne!(current.state, EchoCancellationState::Failed, "{current:?}");
        assert!(
            Instant::now() < deadline,
            "reference not admitted: {current:?}"
        );
        thread::sleep(Duration::from_millis(1));
    }
}

fn finish(
    reference: &mut AecSource,
    microphone: &mut AecSource,
    clean: &EchoCancelledAudio,
    running: &mut RunningSession,
) {
    microphone.close();
    reference.close();
    // Session owns routing queues until stop. Its orderly stop drops producers,
    // drains accepted operator input, flushes, and closes (cancel would discard).
    assert!(running.stop().is_success());
    let operators = running.operator_metrics();
    assert_eq!(operators.len(), 1);
    assert_eq!(operators[0].worker.graceful_finish_total, 1);
    assert_eq!(operators[0].worker.cancellation_total, 0);
    let observations = clean.observations();
    assert_eq!(observations.state, EchoCancellationState::Stopped);
    assert!(observations.last_error.is_none(), "{observations:?}");
}

#[test]
fn given_native_timestamp_jitter_when_samples_remain_contiguous_then_aec_does_not_reset() {
    let (mut reference, mut microphone, clean, mut running) = fixture();
    for index in 0..80 {
        let nominal_ns = CLOCK_EPOCH_NS + index * FRAME_DURATION_NS;
        let reference_ns = nominal_ns + if index % 2 == 0 { 100_000 } else { 0 };
        let microphone_ns = nominal_ns + 7_000_000 + if index % 2 == 0 { 0 } else { 100_000 };
        let mut reference_frame = InputFrame::new(index, reference_ns, vec![0.0; FRAME_SAMPLES]);
        reference_frame.source_generation = 7;
        reference_frame.discontinuity_epoch = 3;
        submit_reference(&reference, &clean, reference_frame);
        let mut microphone_frame = InputFrame::new(index, microphone_ns, vec![0.05; FRAME_SAMPLES]);
        microphone_frame.source_generation = 4;
        microphone_frame.discontinuity_epoch = 11;
        microphone.send(microphone_frame);
        receive_frame(&mut running, &clean, microphone_ns);
        let observations = clean.observations();
        assert_eq!(observations.resets_total, 0, "{observations:?}");
        assert_eq!(observations.processed_microphone_frames_total, index + 1);
        assert!(observations.microphone_queue_depth_frames <= observations.queue_capacity_frames);
        assert!(observations.reference_queue_depth_frames <= observations.queue_capacity_frames);
    }
    finish(&mut reference, &mut microphone, &clean, &mut running);
    let observations = clean.observations();
    assert_eq!(observations.maximum_cadence_error_ns, 100_000);
    assert_eq!(observations.analyzed_reference_frames_total, 80);
    assert_eq!(observations.discarded_microphone_frames_total, 0);
    assert_eq!(observations.discarded_reference_frames_total, 0);
}

#[test]
fn given_declared_sequence_gap_and_source_recovery_when_capture_resumes_then_each_resets_once() {
    let (mut reference, mut microphone, clean, mut running) = fixture();
    for index in 0..80 {
        let timestamp_ns = CLOCK_EPOCH_NS + index * FRAME_DURATION_NS;
        let mut frame = InputFrame::new(
            index + u64::from(index >= 20),
            timestamp_ns,
            vec![0.05; FRAME_SAMPLES],
        );
        frame.discontinuity_epoch = u64::from(index >= 20) + u64::from(index >= 40);
        frame.source_generation = if index >= 40 { 2 } else { 1 };
        let reference_frame = InputFrame::new(index, timestamp_ns, vec![0.0; FRAME_SAMPLES]);
        if index == 20 || index == 40 {
            microphone.send(frame);
            // Deliver the new reference after the declared reset; this isolates
            // recovery from the separate question of queued pre-reset loss.
            let expected_resets = if index == 20 { 1 } else { 2 };
            let deadline = Instant::now() + Duration::from_secs(2);
            while clean.observations().resets_total < expected_resets {
                assert!(Instant::now() < deadline, "{:?}", clean.observations());
                thread::sleep(Duration::from_millis(1));
            }
            reference.send(reference_frame);
        } else {
            submit_reference(&reference, &clean, reference_frame);
            microphone.send(frame);
        }
        receive_frame(&mut running, &clean, timestamp_ns);
    }
    finish(&mut reference, &mut microphone, &clean, &mut running);
    let observations = clean.observations();
    assert_eq!(observations.resets_total, 2);
    assert_eq!(observations.processing_generation, 3);
    assert_eq!(observations.processed_microphone_frames_total, 80);
    assert_eq!(observations.analyzed_reference_frames_total, 80);
    assert_eq!(observations.discarded_microphone_frames_total, 0);
    assert_eq!(observations.discarded_reference_frames_total, 0);
}

fn sample_at(samples: &[f32], position_samples: f64) -> f32 {
    if position_samples < 0.0 {
        return 0.0;
    }
    let lower = position_samples.floor() as usize;
    let fraction = (position_samples - lower as f64) as f32;
    let first = samples.get(lower).copied().unwrap_or(0.0);
    let second = samples.get(lower + 1).copied().unwrap_or(0.0);
    first + fraction * (second - first)
}

fn voice_at(time_s: f64) -> f32 {
    let phase_rad = std::f64::consts::TAU * (173.0 * time_s - 43.0 / 1.7 * (1.7 * time_s).cos());
    let amplitude = 0.55 + 0.45 * (4.3 * time_s).sin().powi(2);
    (amplitude
        * (0.14 * phase_rad.sin()
            + 0.07 * (2.0 * phase_rad).sin()
            + 0.035 * (3.0 * phase_rad).sin())) as f32
}

fn voice_metrics(desired: &[f32], actual: &[f32]) -> (usize, f64, f64, f64) {
    let start_sample = 10 * 48_000;
    let comparison_samples = 12_000;
    let target = &desired[start_sample..start_sample + comparison_samples];
    let target_power = target
        .iter()
        .map(|sample| f64::from(*sample).powi(2))
        .sum::<f64>();
    let mut best = (0, 0.0, -1.0, 0.0);
    for delay_samples in 0..=960 {
        let output = &actual
            [start_sample + delay_samples..start_sample + delay_samples + comparison_samples];
        let output_power = output
            .iter()
            .map(|sample| f64::from(*sample).powi(2))
            .sum::<f64>();
        let dot = target
            .iter()
            .zip(output)
            .map(|(a, b)| f64::from(*a) * f64::from(*b))
            .sum::<f64>();
        let correlation = dot / (target_power * output_power).sqrt().max(1e-12);
        if correlation > best.2 {
            let error_power = target
                .iter()
                .zip(output)
                .map(|(a, b)| (f64::from(*a) - f64::from(*b)).powi(2))
                .sum::<f64>();
            best = (
                delay_samples,
                10.0 * (output_power.max(1e-12) / target_power).log10(),
                correlation,
                10.0 * (target_power / error_power.max(1e-12)).log10(),
            );
        }
    }
    best
}

#[test]
fn given_physical_time_echo_at_opposite_clock_drifts_when_session_runs_then_echo_and_voice_meet_gates(
) {
    // Two twelve-second synthetic captures. PCM represents different microphone
    // sample clocks; changing only metadata would not test acoustic drift.
    for drift_ppm in [-100.0, 100.0] {
        let (mut reference, mut microphone, clean, mut running) = fixture();
        let sample_count = 12 * 48_000;
        let microphone_rate_hz = 48_000.0 * (1.0 + drift_ppm / 1_000_000.0);
        let mut render = Vec::with_capacity(sample_count);
        let (mut random, mut filtered) = (0x92aec138_u32, 0.0_f32);
        for _ in 0..sample_count {
            random ^= random << 13;
            random ^= random >> 17;
            random ^= random << 5;
            filtered =
                0.65 * filtered + 0.35 * (random as f64 / u32::MAX as f64 * 2.0 - 1.0) as f32;
            render.push(filtered * 0.35);
        }
        let mut desired = Vec::with_capacity(sample_count);
        let mut microphone_pcm = Vec::with_capacity(sample_count);
        for index in 0..sample_count {
            let time_s = index as f64 / microphone_rate_hz;
            let voice = if time_s >= 6.0 { voice_at(time_s) } else { 0.0 };
            desired.push(voice);
            microphone_pcm.push(voice + 0.6 * sample_at(&render, time_s * 48_000.0 - 960.0));
        }
        let mut actual = Vec::with_capacity(sample_count);
        for (index, start_sample) in (0..sample_count).step_by(FRAME_SAMPLES).enumerate() {
            let microphone_ns = CLOCK_EPOCH_NS
                + (start_sample as f64 / microphone_rate_hz * 1_000_000_000.0).round() as u64;
            submit_reference(
                &reference,
                &clean,
                InputFrame::new(
                    index as u64,
                    CLOCK_EPOCH_NS + index as u64 * FRAME_DURATION_NS,
                    render[start_sample..start_sample + FRAME_SAMPLES].to_vec(),
                ),
            );
            microphone.send(InputFrame::new(
                index as u64,
                microphone_ns,
                microphone_pcm[start_sample..start_sample + FRAME_SAMPLES].to_vec(),
            ));
            actual.extend(receive_frame(&mut running, &clean, microphone_ns));
            assert_eq!(clean.observations().resets_total, 0);
        }
        finish(&mut reference, &mut microphone, &clean, &mut running);
        assert_eq!(actual.len(), sample_count);
        let observed = clean.observations();
        assert_eq!(
            observed.analyzed_reference_frames_total,
            (sample_count / FRAME_SAMPLES) as u64
        );
        assert_eq!(observed.discarded_microphone_frames_total, 0);
        assert_eq!(observed.discarded_reference_frames_total, 0);
        assert!((1_999..=2_001).contains(&observed.maximum_cadence_error_ns));
        let input_power = microphone_pcm[4 * 48_000..5 * 48_000]
            .iter()
            .map(|x| f64::from(*x).powi(2))
            .sum::<f64>();
        let output_power = actual[4 * 48_000..5 * 48_000]
            .iter()
            .map(|x| f64::from(*x).powi(2))
            .sum::<f64>();
        let reduction_db = 10.0 * (input_power / output_power.max(1e-12)).log10();
        let (delay_samples, gain_db, correlation, snr_db) = voice_metrics(&desired, &actual);
        println!("AEC drift={drift_ppm}ppm: echo={reduction_db:.3}dB delay={delay_samples}samples gain={gain_db:.3}dB correlation={correlation:.4} snr={snr_db:.3}dB");
        assert!(reduction_db >= 10.0, "echo reduction {reduction_db} dB");
        assert!((-3.0..=3.0).contains(&gain_db), "voice gain {gain_db} dB");
        assert!(correlation >= 0.8, "voice correlation {correlation}");
        assert!(snr_db >= 6.0, "voice SNR {snr_db} dB");
        assert!(delay_samples < 960, "voice delay search exhausted");
    }
}
