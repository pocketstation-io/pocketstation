//! Qualification probe for the locked APM configuration, not physical AEC proof.
//! The nominal delay is structural; measured waveform peaks may differ because
//! WebRTC's three-band analysis/synthesis is not perfect reconstruction.
#![cfg(feature = "aec")]

use webrtc_audio_processing::{config::EchoCanceller, Config, Processor};

const NATIVE_BLOCK_SAMPLES: usize = 480;
const NOMINAL_DELAY_SAMPLES: usize = (64 + 64) * 3 + 24 + 24;

#[derive(Clone, Copy, Debug)]
enum Continuation {
    CaptureOnlyDrain,
    SilentRenderAndCapture,
}

#[derive(Clone, Copy, Debug)]
enum Initialization {
    Cold,
    SilentCycles(usize),
    ConfigureThenReset,
}

const INITIALIZATIONS: [Initialization; 7] = [
    Initialization::Cold,
    Initialization::SilentCycles(1),
    Initialization::SilentCycles(2),
    Initialization::SilentCycles(3),
    Initialization::SilentCycles(4),
    Initialization::SilentCycles(20),
    Initialization::ConfigureThenReset,
];

fn processor(channels: usize) -> Processor {
    let processor = if channels > 1 {
        let mut config = webrtc_audio_processing::experimental::EchoCanceller3Config::default();
        config.multi_channel.detect_stereo_content = false;
        assert!(config.validate());
        Processor::with_aec3_config(48_000, config).unwrap()
    } else {
        Processor::new(48_000).unwrap()
    };
    processor.set_config(Config {
        echo_canceller: Some(EchoCanceller::Full {
            stream_delay_ms: None,
        }),
        pipeline: webrtc_audio_processing::config::Pipeline {
            multi_channel_render: channels > 1,
            multi_channel_capture: channels > 1,
            ..Default::default()
        },
        ..Default::default()
    });
    processor
}

fn render_silence(processor: &Processor, channels: usize) {
    let reference = [[0.0_f32; NATIVE_BLOCK_SAMPLES]; 2];
    processor
        .analyze_render_frame(
            reference[..channels]
                .iter()
                .map(|channel| channel.as_slice()),
        )
        .unwrap();
}

fn capture_block(processor: &Processor, input: &[f32], channels: usize, output: &mut Vec<f32>) {
    assert_eq!(input.len(), NATIVE_BLOCK_SAMPLES * channels);
    let mut capture = [[0.0_f32; NATIVE_BLOCK_SAMPLES]; 2];
    for (sample, values) in input.chunks_exact(channels).enumerate() {
        for channel in 0..channels {
            capture[channel][sample] = values[channel];
        }
    }
    processor
        .process_capture_frame(
            capture[..channels]
                .iter_mut()
                .map(|channel| channel.as_mut_slice()),
        )
        .unwrap();
    for sample in 0..NATIVE_BLOCK_SAMPLES {
        for values in capture.iter().take(channels) {
            assert!(values[sample].is_finite());
            output.push(values[sample]);
        }
    }
}

fn run(
    input: &[f32],
    channels: usize,
    outer_frame_samples: usize,
    initialization: Initialization,
    continuation: Continuation,
) -> Vec<f32> {
    assert_eq!(input.len() % (outer_frame_samples * channels), 0);
    let processor = processor(channels);
    let silence = vec![0.0; NATIVE_BLOCK_SAMPLES * channels];
    let mut warm_output = Vec::with_capacity(NATIVE_BLOCK_SAMPLES * channels);
    let initialization_blocks = match initialization {
        Initialization::Cold => 0,
        Initialization::SilentCycles(blocks) => blocks,
        Initialization::ConfigureThenReset => 1,
    };
    for _ in 0..initialization_blocks {
        render_silence(&processor, channels);
        warm_output.clear();
        capture_block(&processor, &silence, channels, &mut warm_output);
    }
    if let Initialization::ConfigureThenReset = initialization {
        // Resolve both native stream formats, then discard all initialization
        // audio/adaptation history while retaining those configured formats.
        processor.reinitialize();
    }
    let mut output = Vec::with_capacity(input.len() + 2 * NATIVE_BLOCK_SAMPLES * channels);
    for frame in input.chunks_exact(outer_frame_samples * channels) {
        // Match Core's outer-frame admission: actual render blocks first,
        // then the corresponding microphone blocks. No synthetic echo path.
        for _ in 0..outer_frame_samples / NATIVE_BLOCK_SAMPLES {
            render_silence(&processor, channels);
        }
        for block in frame.chunks_exact(NATIVE_BLOCK_SAMPLES * channels) {
            capture_block(&processor, block, channels, &mut output);
        }
    }
    // Two blocks expose the full measured response. The proposed bounded drain
    // may consume only the first block; the second must not be needed to retain
    // the original source-length interval after nominal delay compensation.
    for _ in 0..2 {
        match continuation {
            Continuation::CaptureOnlyDrain => {}
            Continuation::SilentRenderAndCapture => render_silence(&processor, channels),
        }
        capture_block(&processor, &silence, channels, &mut output);
    }
    output
}

fn energy(samples: impl Iterator<Item = f32>) -> f64 {
    samples.map(|sample| f64::from(sample).powi(2)).sum()
}

fn compare_tail_to_uninterrupted_baseline(
    input: &[f32],
    drained: &[f32],
    baseline: &[f32],
    channels: usize,
) -> (f64, f64) {
    let source_samples = input.len() / channels;
    let compensated_end = (NOMINAL_DELAY_SAMPLES + source_samples) * channels;
    let one_block_end = input.len() + NATIVE_BLOCK_SAMPLES * channels;
    assert!(compensated_end <= one_block_end);
    assert_eq!(drained.len(), baseline.len());
    assert_eq!(
        drained.len(),
        input.len() + 2 * NATIVE_BLOCK_SAMPLES * channels
    );
    let drain = &drained[NOMINAL_DELAY_SAMPLES * channels..compensated_end];
    let baseline = &baseline[NOMINAL_DELAY_SAMPLES * channels..compensated_end];
    assert_eq!(
        drain.len(),
        input.len(),
        "DSP padding is not a source frame"
    );
    let error_power = drain
        .iter()
        .zip(baseline)
        .map(|(a, b)| (f64::from(*a) - f64::from(*b)).powi(2))
        .sum::<f64>();
    let baseline_power = energy(baseline.iter().copied());
    let relative_error = (error_power / baseline_power.max(1e-20)).sqrt();
    let recovered_tail_energy = energy(drained[input.len()..compensated_end].iter().copied());
    // This checks the bounded EOF strategy against an independently constructed
    // uninterrupted engine receiving actual silent render after the input ends.
    // It does not assert perfect reconstruction or an exact waveform delay.
    assert!(
        relative_error <= 1e-4,
        "capture-only EOF altered retained output: relative error {relative_error}"
    );
    assert!(
        recovered_tail_energy > 1e-6,
        "the final excitation was not recovered by draining"
    );
    (relative_error, recovered_tail_energy)
}

#[test]
fn given_locked_apm_when_impulses_cross_phase_and_block_boundaries_then_delay_and_tail_are_measured(
) {
    // Covers all three polyphases and 64/80/160-sample subband boundaries.
    let positions_samples = [0, 1, 2, 191, 192, 239, 240, 479];
    for outer_frame_samples in [480, 960] {
        for channels in [1, 2] {
            // Internal silence initializes the engine, never captured sources.
            for initialization in INITIALIZATIONS {
                for position_samples in positions_samples {
                    let source_samples = outer_frame_samples * 4;
                    let mut input = vec![0.0; source_samples * channels];
                    for channel in 0..channels {
                        let gain = if channel == 0 { 0.35 } else { -0.21 };
                        input[position_samples * channels + channel] = gain;
                        input[(source_samples - 49) * channels + channel] = -gain;
                    }
                    let drained = run(
                        &input,
                        channels,
                        outer_frame_samples,
                        initialization,
                        Continuation::CaptureOnlyDrain,
                    );
                    let baseline = run(
                        &input,
                        channels,
                        outer_frame_samples,
                        initialization,
                        Continuation::SilentRenderAndCapture,
                    );
                    let (relative_error, tail_energy) = compare_tail_to_uninterrupted_baseline(
                        &input, &drained, &baseline, channels,
                    );
                    for channel in 0..channels {
                        let peak_sample = (position_samples..position_samples + 960)
                            .max_by(|a, b| {
                                drained[a * channels + channel]
                                    .abs()
                                    .total_cmp(&drained[b * channels + channel].abs())
                            })
                            .unwrap();
                        let peak_delay_samples = peak_sample - position_samples;
                        let residual_samples =
                            peak_delay_samples as i64 - NOMINAL_DELAY_SAMPLES as i64;
                        println!("APM impulse outer={outer_frame_samples}samples channels={channels} channel={channel} init={initialization:?} position={position_samples}samples nominal={NOMINAL_DELAY_SAMPLES}samples peak={peak_delay_samples}samples residual={residual_samples}samples tail_energy={tail_energy:.9} baseline_relative_error={relative_error:.9}");
                        assert!(
                            peak_delay_samples < NATIVE_BLOCK_SAMPLES,
                            "one-block drain bound exceeded"
                        );
                        if !matches!(initialization, Initialization::Cold) {
                            assert!(
                                residual_samples.abs() <= 4,
                                "initialization did not stabilize the first impulse"
                            );
                        }
                    }
                }
            }
        }
    }
}

#[test]
fn given_first_and_final_voice_when_formats_are_initialized_then_speech_and_tail_are_retained() {
    for outer_frame_samples in [480, 960] {
        for channels in [1, 2] {
            for initialization in INITIALIZATIONS {
                let source_samples = outer_frame_samples * 8;
                let mut input = vec![0.0; source_samples * channels];
                let final_start_sample = source_samples - 1_440;
                for start_sample in [0, final_start_sample] {
                    for index in start_sample..start_sample + 1_440 {
                        let phase = std::f64::consts::TAU * 173.0 * (index - start_sample) as f64
                            / 48_000.0;
                        let value = (0.14 * phase.sin()
                            + 0.07 * (2.0 * phase).sin()
                            + 0.035 * (3.0 * phase).sin())
                            as f32;
                        for channel in 0..channels {
                            input[index * channels + channel] =
                                if channel == 0 { value } else { -value * 0.6 };
                        }
                    }
                }
                let drained = run(
                    &input,
                    channels,
                    outer_frame_samples,
                    initialization,
                    Continuation::CaptureOnlyDrain,
                );
                let baseline = run(
                    &input,
                    channels,
                    outer_frame_samples,
                    initialization,
                    Continuation::SilentRenderAndCapture,
                );
                let (relative_error, tail_energy) =
                    compare_tail_to_uninterrupted_baseline(&input, &drained, &baseline, channels);
                for (name, start_sample, length_samples) in
                    [("first", 0, 192), ("final", final_start_sample + 240, 960)]
                {
                    let target =
                        &input[start_sample * channels..(start_sample + length_samples) * channels];
                    let target_power = energy(target.iter().copied());
                    let mut best = (0, -1.0_f64);
                    let (mut nominal_correlation, mut nominal_gain_db) = (0.0, 0.0);
                    for lag_samples in 180..=470 {
                        let output = &drained[(start_sample + lag_samples) * channels
                            ..(start_sample + length_samples + lag_samples) * channels];
                        let output_power = energy(output.iter().copied());
                        let dot = target
                            .iter()
                            .zip(output)
                            .map(|(a, b)| f64::from(*a) * f64::from(*b))
                            .sum::<f64>();
                        let correlation = dot / (target_power * output_power).sqrt().max(1e-20);
                        if correlation > best.1 {
                            best = (lag_samples, correlation);
                        }
                        if lag_samples == NOMINAL_DELAY_SAMPLES {
                            nominal_correlation = correlation;
                            nominal_gain_db =
                                10.0 * (output_power / target_power).max(1e-20).log10();
                        }
                    }
                    println!("APM {name} voice outer={outer_frame_samples}samples channels={channels} init={initialization:?} source={source_samples}samples nominal={NOMINAL_DELAY_SAMPLES}samples peak={}samples residual={}samples correlation={:.6} nominal_correlation={nominal_correlation:.6} nominal_gain={nominal_gain_db:.6}dB tail_energy={tail_energy:.9} baseline_relative_error={relative_error:.9}", best.0, best.0 as i64 - NOMINAL_DELAY_SAMPLES as i64, best.1);
                    assert!(best.0 > 180 && best.0 < 470, "voice lag search exhausted");
                    if !matches!(initialization, Initialization::Cold) || name == "final" {
                        assert!(
                            best.0.abs_diff(NOMINAL_DELAY_SAMPLES) <= 4,
                            "voice delay did not stabilize"
                        );
                        assert!(
                            nominal_correlation >= 0.8,
                            "nominal trimming damaged the first/last voice"
                        );
                        assert!(
                            (-3.0..=3.0).contains(&nominal_gain_db),
                            "nominal trimming damaged voice gain"
                        );
                    }
                }
            }
        }
    }
}
