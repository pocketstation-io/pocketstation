//! Real Opus encode, decode, and packet-loss concealment primitives.
//!
//! Provider policy, transport pacing, and RTP continuity do not belong here.
//! This module owns only codec configuration and deterministic libopus calls.

mod constants;
mod decoder;
mod encoder;
mod profile;

pub use constants::{
    OPUS_FRAME_SAMPLES, OPUS_MAX_PACKET_BYTES, OPUS_SAMPLE_RATE_HZ, VOICE_AGENT_FRAME_SAMPLES,
};
pub use decoder::{OpusDecodeError, OpusDecoder};
pub use encoder::{
    OpusApplication, OpusChannels, OpusConfig, OpusEncodeError, OpusEncoder, OpusFrameDuration,
    OpusSampleRate,
};
pub use profile::StreamProfile;
