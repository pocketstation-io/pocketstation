// Phase 3, AUDIO-012: real libopus bindings replace the Phase 0 mock.
// Dependency approved: opus = "0.3" wraps libopus via libopus-sys. Chosen over
// audiopus because it is the de-facto Rust binding (most downloads, active
// maintenance, matches the workspace dep already declared).

mod constants;
mod decoder;
mod encoder;
mod profile;

#[deprecated(since = "0.1.0", note = "use OPUS_SAMPLE_RATE_HZ")]
pub use constants::OPUS_SAMPLE_RATE_HZ as OPUS_SAMPLE_RATE;
pub use constants::{
    OPUS_FRAME_SAMPLES, OPUS_MAX_PACKET_BYTES, OPUS_SAMPLE_RATE_HZ, VOICE_AGENT_FRAME_SAMPLES,
};
pub use decoder::OpusDecoder;
pub use encoder::{
    EncodedFrame, OpusApplication, OpusChannels, OpusConfig, OpusEncodeError, OpusEncoder,
    OpusFrameDuration, OpusSampleRate,
};
pub use profile::StreamProfile;

#[cfg(any(test, feature = "test-helpers"))]
pub use decoder::MockOpusDecoder;
#[cfg(any(test, feature = "test-helpers"))]
pub use encoder::MockOpusEncoder;

#[cfg(test)]
mod compatibility_tests {
    #[test]
    #[allow(deprecated)]
    fn given_legacy_sample_rate_alias_when_read_then_value_matches_unit_named_constant() {
        assert_eq!(super::OPUS_SAMPLE_RATE, super::OPUS_SAMPLE_RATE_HZ);
    }
}
