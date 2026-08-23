//! Named transport encoding profiles resolved to concrete Opus configuration.

use crate::codec::{OpusApplication, OpusChannels, OpusConfig, OpusFrameDuration, OpusSampleRate};

const VOICE_BITRATE_KBPS: u32 = 48;
const MUSIC_STEREO_BITRATE_KBPS: u32 = 160;
const MUSIC_STEREO_10MS_BITRATE_KBPS: u32 = 192;
const HIFI_STEREO_BITRATE_KBPS: u32 = 224;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "Enumerates the supported stream profile cases."]
pub enum StreamProfile {
    #[doc = "Represents the voice mono20ms case of `StreamProfile`."]
    VoiceMono20ms,
    #[doc = "Represents the voice agent mono10ms case of `StreamProfile`."]
    VoiceAgentMono10ms,
    #[doc = "Represents the music stereo20ms case of `StreamProfile`."]
    MusicStereo20ms,
    #[doc = "Represents the music stereo10ms case of `StreamProfile`."]
    MusicStereo10ms,
    #[doc = "Represents the broadcast stereo20ms case of `StreamProfile`."]
    BroadcastStereo20ms,
    #[doc = "Represents the hifi stereo20ms case of `StreamProfile`."]
    HifiStereo20ms,
}

impl StreamProfile {
    #[doc = "Returns the channel count represented by `StreamProfile`."]
    pub const fn channels(self) -> OpusChannels {
        match self {
            Self::VoiceMono20ms | Self::VoiceAgentMono10ms => OpusChannels::Mono,
            Self::MusicStereo20ms
            | Self::MusicStereo10ms
            | Self::BroadcastStereo20ms
            | Self::HifiStereo20ms => OpusChannels::Stereo,
        }
    }

    #[doc = "Returns the frame duration held by `StreamProfile`."]
    pub const fn frame_duration(self) -> OpusFrameDuration {
        match self {
            Self::VoiceAgentMono10ms | Self::MusicStereo10ms => OpusFrameDuration::Ms10,
            Self::VoiceMono20ms
            | Self::MusicStereo20ms
            | Self::BroadcastStereo20ms
            | Self::HifiStereo20ms => OpusFrameDuration::Ms20,
        }
    }

    #[doc = "Returns the application held by `StreamProfile`."]
    pub const fn application(self) -> OpusApplication {
        match self {
            Self::VoiceMono20ms => OpusApplication::Voip,
            Self::VoiceAgentMono10ms | Self::MusicStereo10ms => OpusApplication::LowDelay,
            Self::MusicStereo20ms | Self::BroadcastStereo20ms | Self::HifiStereo20ms => {
                OpusApplication::Audio
            }
        }
    }

    #[doc = "Returns the bitrate kbps held by `StreamProfile`."]
    pub const fn bitrate_kbps(self) -> u32 {
        match self {
            Self::VoiceMono20ms | Self::VoiceAgentMono10ms => VOICE_BITRATE_KBPS,
            Self::MusicStereo20ms | Self::BroadcastStereo20ms => MUSIC_STEREO_BITRATE_KBPS,
            Self::MusicStereo10ms => MUSIC_STEREO_10MS_BITRATE_KBPS,
            Self::HifiStereo20ms => HIFI_STEREO_BITRATE_KBPS,
        }
    }

    #[doc = "Returns the frame milliseconds held by `StreamProfile`."]
    pub const fn frame_ms(self) -> u16 {
        match self.frame_duration() {
            OpusFrameDuration::Ms10 => 10,
            OpusFrameDuration::Ms20 => 20,
            OpusFrameDuration::Ms40 => 40,
            OpusFrameDuration::Ms60 => 60,
        }
    }

    #[doc = "Returns whether stereo applies to `StreamProfile`."]
    pub const fn is_stereo(self) -> bool {
        matches!(self.channels(), OpusChannels::Stereo)
    }

    #[doc = "Returns the opus config held by `StreamProfile`."]
    pub const fn opus_config(self) -> OpusConfig {
        OpusConfig {
            sample_rate: OpusSampleRate::Hz48000,
            channels: self.channels(),
            frame_duration: self.frame_duration(),
            application: self.application(),
            bitrate_kbps: Some(self.bitrate_kbps()),
            complexity: match self {
                Self::VoiceAgentMono10ms => 5,
                Self::VoiceMono20ms => 9,
                _ => 10,
            },
            dtx: false,
            fec: matches!(self, Self::VoiceMono20ms),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_stereo_profiles_when_resolved_then_channels_remain_stereo() {
        for profile in [
            StreamProfile::MusicStereo20ms,
            StreamProfile::MusicStereo10ms,
            StreamProfile::BroadcastStereo20ms,
            StreamProfile::HifiStereo20ms,
        ] {
            assert_eq!(profile.opus_config().channels, OpusChannels::Stereo);
        }
    }
}
