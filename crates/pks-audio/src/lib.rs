pub mod ffi;
pub use ffi::{pks_encode_opus, pks_opus_encoder_create, pks_opus_encoder_destroy, PksOpusEncoder};

pub use pks_codec::*;
pub use pks_frame::*;
pub use pks_metrics::*;
pub use pks_pipeline::*;

// frame owns the canonical graph-level EncodedFrame; codec's leaner Opus-packet
// type stays reachable as pks_codec::EncodedFrame. Explicit re-export
// disambiguates the two glob re-exports above.
pub use pks_frame::EncodedFrame;

// Graph compiler + runtime executor (targeted, not glob — avoids re-export clashes).
pub use pks_runtime::{
    EdgeChannel, EdgeReceiver, EdgeSender, ExecError, PlanScheduler, RealtimeExecutor, RunMetrics,
};

pub use pks_capture::{
    capture_system_audio, capture_with_mode, discover_sources, open_best_source, AdapterError,
    AudioOutputDescriptor, AudioOutputSink, AudioSourceDescriptor, AudioSourceStream, CaptureError,
    CaptureMode, CaptureSource, LatencyClass, LoopbackError, OutputRequest, PlatformAdapter,
    PlatformId, ReliabilityClass, SourceCapability, SourceKind, SourcePreference, SourceRequest,
    SourceState, StableSourceId, SystemLoopbackSource,
};

// Re-export capture's OutputTarget.
pub use pks_capture::OutputTarget;

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
pub use pks_capture_macos::{asp_is_installed, tap_available};

use std::f32::consts::PI;

pub fn fill_sine(buffer: &mut [f32], sample_rate_hz: u32, freq_hz: f32, start_sample: u64) {
    for (i, s) in buffer.iter_mut().enumerate() {
        let t = (start_sample + i as u64) as f32 / sample_rate_hz as f32;
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
