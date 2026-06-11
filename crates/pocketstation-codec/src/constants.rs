/// 48 000 Hz, mono, VOIP application profile (AUDIO-012 default).
pub const OPUS_SAMPLE_RATE: u32 = 48_000;

/// 20 ms frame = 960 samples at 48 kHz (AUDIO-012).
pub const OPUS_FRAME_SAMPLES: usize = 960;

/// 10 ms frame = 480 samples at 48 kHz (voice-agent low-latency mode, RFC 6716 §3.1).
/// Used with [`OpusEncoder::voice_agent`] and OPUS_APPLICATION_RESTRICTED_LOWDELAY.
pub const VOICE_AGENT_FRAME_SAMPLES: usize = 480;

/// Maximum number of bytes the Opus encoder can emit per 20 ms frame.
/// libopus guarantees this upper bound.
pub const OPUS_MAX_PACKET_BYTES: usize = 4_000;

/// Scale factor for f32 ↔ i16 conversion.
pub(crate) const I16_SCALE: f32 = i16::MAX as f32;
