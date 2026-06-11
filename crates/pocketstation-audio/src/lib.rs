pub mod ffi;
pub use ffi::{pks_encode_opus, pks_opus_encoder_create, pks_opus_encoder_destroy, PksOpusEncoder};

pub use pocketstation_bus::*;
pub use pocketstation_codec::*;
pub use pocketstation_frame::*;
pub use pocketstation_graph::*;
pub use pocketstation_metrics::*;
// Route re-exports: exclude EncryptionMode which conflicts with pocketstation_codec's re-export.
pub use pocketstation_route::{
    open_best_source, AdapterError, AudioOutputDescriptor, AudioOutputSink, AudioSourceDescriptor,
    AudioSourceStream, LatencyClass, OutputRequest, OutputTarget, PlatformAdapter, PlatformId,
    ReliabilityClass, RouteKind, RoutePlan, SourceCapability, SourcePreference, SourceRequest,
    TransportKind,
};

use std::f32::consts::PI;

pub fn fill_sine(buffer: &mut [f32], sample_rate: u32, freq_hz: f32, start_sample: u64) {
    for (i, s) in buffer.iter_mut().enumerate() {
        let t = (start_sample + i as u64) as f32 / sample_rate as f32;
        *s = (2.0 * PI * freq_hz * t).sin() * 0.25;
    }
}

pub fn write_wav_mono_48k(path: &str, samples: &[f32]) -> Result<(), hound::Error> {
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: 48_000,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut writer = hound::WavWriter::create(path, spec)?;
    for s in samples {
        let v = (s.clamp(-1.0, 1.0) * i16::MAX as f32) as i16;
        writer.write_sample(v)?;
    }
    writer.finalize()?;
    Ok(())
}
