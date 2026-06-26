pub mod ffi;
pub use ffi::{pks_encode_opus, pks_opus_encoder_create, pks_opus_encoder_destroy, PksOpusEncoder};

pub use pocketstation_codec::*;
pub use pocketstation_frame::*;
pub use pocketstation_metrics::*;
pub use pocketstation_pipeline::*;

pub use pocketstation_capture::{
    capture_system_audio, capture_with_mode, discover_sources, open_best_source, AdapterError,
    AudioOutputDescriptor, AudioOutputSink, AudioSourceDescriptor, AudioSourceStream, CaptureError,
    CaptureMode, CaptureSource, LatencyClass, LoopbackError, OutputRequest, PlatformAdapter,
    PlatformId, ReliabilityClass, SourceCapability, SourceKind, SourcePreference, SourceRequest,
    SourceState, StableSourceId, SystemLoopbackSource,
};

// Re-export capture's OutputTarget.
pub use pocketstation_capture::OutputTarget;

/// Transport mechanism used to carry encoded audio frames.
/// Not dispatched on until a Rust transport layer exists (AUDIO-025).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TransportKind {
    Local,
    WebRtc,
    RtpUdp,
    File,
}

/// Encryption mode applied at the transport or frame layer (AUDIO-025).
/// SFrameE2EE follows RFC 9605. EnterpriseKeyManager is deferred to Phase 5.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RouteEncryptionMode {
    TransportOnly,
    SFrameE2EE,
    EnterpriseKeyManager,
}

// Platform-specific re-exports.
#[cfg(target_os = "macos")]
pub use pocketstation_capture_macos::{asp_is_installed, tap_available};

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
