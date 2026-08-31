/// Fixed audio cadence used by capture and Session routing.
///
/// Twenty milliseconds remains the general-purpose default. Ten milliseconds
/// is available for latency-sensitive voice paths that accept the higher
/// callback, routing, encoding, and packet rate.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub enum AudioFrameDuration {
    Ms10,
    #[default]
    Ms20,
}

impl AudioFrameDuration {
    pub const fn milliseconds(self) -> u16 {
        match self {
            Self::Ms10 => 10,
            Self::Ms20 => 20,
        }
    }

    pub fn samples_per_channel(self, sample_rate_hz: u32) -> usize {
        usize::try_from(sample_rate_hz)
            .unwrap_or(usize::MAX)
            .saturating_mul(usize::from(self.milliseconds()))
            / 1_000
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_48khz_audio_when_duration_is_resolved_then_frame_sizes_are_exact() {
        assert_eq!(AudioFrameDuration::Ms10.samples_per_channel(48_000), 480);
        assert_eq!(AudioFrameDuration::Ms20.samples_per_channel(48_000), 960);
    }
}
