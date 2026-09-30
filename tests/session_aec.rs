#![cfg(feature = "echo-cancellation")]

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
