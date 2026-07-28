// Proves each ml node's RuntimeNode frame path produces the same samples as its
// slice DSP core — i.e. the lifecycle wrapper is transform-preserving.

use std::sync::Arc;

use pks_dsp::{AudioWatermark, EchoCanceller, NoiseSuppressor, VadProcessor};
use pks_frame::{AudioBufferPool, AudioFrame, SampleFormat, SampleSpec, SourceId, StreamId};
use pks_graph::node::PrepareContext;
use pks_graph::RuntimeNode;

const FRAME_SAMPLES: usize = 960;

fn sine(amplitude: f32) -> Vec<f32> {
    use std::f32::consts::PI;
    (0..FRAME_SAMPLES)
        .map(|i| amplitude * (2.0 * PI * 440.0 * i as f32 / 48_000.0).sin())
        .collect()
}

fn prepare_cx() -> PrepareContext {
    PrepareContext::new(SampleSpec::new(48_000, 1, SampleFormat::F32Interleaved))
}

fn frame_from(pool: &Arc<AudioBufferPool>, samples: &[f32]) -> AudioFrame {
    let mut handle = pool.acquire().expect("pool slot");
    handle.copy_from_slice(samples);
    AudioFrame::new(StreamId(0), SourceId(0), 0, 0, 1, handle)
}

#[test]
fn given_vad_when_run_via_runtime_node_then_buffer_matches_slice_core() {
    let input = sine(0.5);
    let mut core = VadProcessor::default_config();
    let mut expected = vec![0.0f32; FRAME_SAMPLES];
    core.process_slices(&input, &mut expected);

    let pool = AudioBufferPool::new(2, FRAME_SAMPLES);
    let mut node = VadProcessor::default_config();
    node.prepare(&prepare_cx()).unwrap();
    let out = node.process(frame_from(&pool, &input)).unwrap().unwrap();
    assert_eq!(out.buffer.as_slice(), &expected[..]);
}

#[test]
fn given_noise_suppressor_when_run_via_runtime_node_then_buffer_matches_slice_core() {
    let input = sine(0.3);
    let mut core = NoiseSuppressor::default_config();
    let mut expected = vec![0.0f32; FRAME_SAMPLES];
    core.process_slices(&input, &mut expected);

    let pool = AudioBufferPool::new(2, FRAME_SAMPLES);
    let mut node = NoiseSuppressor::default_config();
    node.prepare(&prepare_cx()).unwrap();
    let out = node.process(frame_from(&pool, &input)).unwrap().unwrap();
    assert_eq!(out.buffer.as_slice(), &expected[..]);
}

#[test]
fn given_watermark_when_run_via_runtime_node_then_buffer_matches_slice_core() {
    let input = sine(0.4);
    let mut core = AudioWatermark::new(0xC0FFEE, 0.01, 0.1);
    let mut expected = vec![0.0f32; FRAME_SAMPLES];
    core.process_slices(&input, &mut expected);

    let pool = AudioBufferPool::new(2, FRAME_SAMPLES);
    let mut node = AudioWatermark::new(0xC0FFEE, 0.01, 0.1);
    node.prepare(&prepare_cx()).unwrap();
    let out = node.process(frame_from(&pool, &input)).unwrap().unwrap();
    assert_eq!(out.buffer.as_slice(), &expected[..]);
}

#[test]
fn given_echo_canceller_without_reference_when_run_via_runtime_node_then_passes_input_through() {
    let input = sine(0.5);
    let pool = AudioBufferPool::new(2, FRAME_SAMPLES);
    let mut node = EchoCanceller::default_config();
    node.prepare(&prepare_cx()).unwrap();
    let out = node.process(frame_from(&pool, &input)).unwrap().unwrap();
    assert_eq!(out.buffer.as_slice(), &input[..]);
}
