//! StreamProfile — the named encode profiles a route is published with.
//! Each profile resolves to a concrete `OpusConfig`; this is the single place
//! that decides channels, frame duration, application mode, and bitrate so that
//! "is this stream stereo?" is never guessed (PocketStation Corrected Audit §4).

use media_clock::Contract;

use crate::encoder::{
    OpusApplication, OpusChannels, OpusConfig, OpusFrameDuration, OpusSampleRate,
};

const VOICE_BITRATE_KBPS: u32 = 48; // 32–64 kbps band (conferencing / agent uplink)
const MUSIC_STEREO_BITRATE_KBPS: u32 = 160; // 128–192 kbps band (Spotify / broadcast)
const MUSIC_STEREO_10MS_BITRATE_KBPS: u32 = 192; // 160–256 kbps; low-latency monitor
const HIFI_STEREO_BITRATE_KBPS: u32 = 224; // 192–256 kbps band (high quality)

const VOICE_AGENT_COMPLEXITY: u8 = 5; // low CPU for 10 ms realtime agent path
const VOICE_COMPLEXITY: u8 = 9;
const STEREO_COMPLEXITY: u8 = 10; // music/broadcast quality

/// The publish-time stream profiles. A mono profile must never be the silent
/// default for a stereo source: choose the profile explicitly per route.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StreamProfile {
    VoiceMono20ms,
    VoiceAgentMono10ms,
    MusicStereo20ms,
    MusicStereo10ms,
    BroadcastStereo20ms,
    HifiStereo20ms,
}

impl StreamProfile {
    pub fn channels(self) -> OpusChannels {
        match self {
            Self::VoiceMono20ms | Self::VoiceAgentMono10ms => OpusChannels::Mono,
            Self::MusicStereo20ms
            | Self::MusicStereo10ms
            | Self::BroadcastStereo20ms
            | Self::HifiStereo20ms => OpusChannels::Stereo,
        }
    }

    pub fn frame_duration(self) -> OpusFrameDuration {
        match self {
            Self::VoiceAgentMono10ms | Self::MusicStereo10ms => OpusFrameDuration::Ms10,
            Self::VoiceMono20ms
            | Self::MusicStereo20ms
            | Self::BroadcastStereo20ms
            | Self::HifiStereo20ms => OpusFrameDuration::Ms20,
        }
    }

    pub fn application(self) -> OpusApplication {
        match self {
            Self::VoiceMono20ms => OpusApplication::Voip,
            // 10 ms low-latency profiles use RESTRICTED_LOWDELAY; the music/10ms
            // application is benchmarked against Audio in Wave 11.
            Self::VoiceAgentMono10ms | Self::MusicStereo10ms => OpusApplication::LowDelay,
            Self::MusicStereo20ms | Self::BroadcastStereo20ms | Self::HifiStereo20ms => {
                OpusApplication::Audio
            }
        }
    }

    pub fn bitrate_kbps(self) -> u32 {
        match self {
            Self::VoiceMono20ms | Self::VoiceAgentMono10ms => VOICE_BITRATE_KBPS,
            Self::MusicStereo20ms | Self::BroadcastStereo20ms => MUSIC_STEREO_BITRATE_KBPS,
            Self::MusicStereo10ms => MUSIC_STEREO_10MS_BITRATE_KBPS,
            Self::HifiStereo20ms => HIFI_STEREO_BITRATE_KBPS,
        }
    }

    pub fn frame_ms(self) -> u16 {
        match self.frame_duration() {
            OpusFrameDuration::Ms10 => 10,
            OpusFrameDuration::Ms20 => 20,
            OpusFrameDuration::Ms40 => 40,
            OpusFrameDuration::Ms60 => 60,
        }
    }

    pub fn is_stereo(self) -> bool {
        matches!(self.channels(), OpusChannels::Stereo)
    }

    /// Map this stream profile to a MediaClock Contract.
    ///
    /// Voice profiles map to Direct (AI ingest) or Interactive (agent output).
    /// Music/broadcast/hifi profiles map to Fidelity or Continuity.
    pub fn contract(self) -> Contract {
        match self {
            Self::VoiceAgentMono10ms => Contract::direct(),
            Self::VoiceMono20ms => Contract::interactive(),
            Self::MusicStereo20ms | Self::MusicStereo10ms | Self::HifiStereo20ms => {
                Contract::fidelity()
            }
            Self::BroadcastStereo20ms => Contract::continuity(),
        }
    }

    fn complexity(self) -> u8 {
        match self {
            Self::VoiceAgentMono10ms => VOICE_AGENT_COMPLEXITY,
            Self::VoiceMono20ms => VOICE_COMPLEXITY,
            _ => STEREO_COMPLEXITY,
        }
    }

    fn fec(self) -> bool {
        matches!(self, Self::VoiceMono20ms) // in-band FEC for conferencing loss recovery
    }

    pub fn opus_config(self) -> OpusConfig {
        OpusConfig {
            sample_rate: OpusSampleRate::Hz48000,
            channels: self.channels(),
            frame_duration: self.frame_duration(),
            application: self.application(),
            bitrate_kbps: Some(self.bitrate_kbps()),
            complexity: self.complexity(),
            dtx: false,
            fec: self.fec(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_voice_agent_profile_when_resolved_then_mono_10ms_lowdelay() {
        let profile = StreamProfile::VoiceAgentMono10ms;
        assert_eq!(profile.channels(), OpusChannels::Mono);
        assert_eq!(profile.frame_ms(), 10);
        assert_eq!(profile.application(), OpusApplication::LowDelay);
        assert!(!profile.is_stereo());
    }

    #[test]
    fn given_music_stereo_profile_when_resolved_then_stereo_20ms_audio() {
        let profile = StreamProfile::MusicStereo20ms;
        assert_eq!(profile.channels(), OpusChannels::Stereo);
        assert_eq!(profile.frame_ms(), 20);
        assert_eq!(profile.application(), OpusApplication::Audio);
        assert!(profile.is_stereo());
    }

    #[test]
    fn given_every_stereo_profile_when_resolved_then_opus_config_keeps_two_channels() {
        for profile in [
            StreamProfile::MusicStereo20ms,
            StreamProfile::MusicStereo10ms,
            StreamProfile::BroadcastStereo20ms,
            StreamProfile::HifiStereo20ms,
        ] {
            let config = profile.opus_config();
            assert_eq!(
                config.channels,
                OpusChannels::Stereo,
                "{profile:?} lost stereo"
            );
            assert_eq!(config.bitrate_kbps, Some(profile.bitrate_kbps()));
        }
    }

    #[test]
    fn given_hifi_profile_when_resolved_then_highest_bitrate_band() {
        assert!(
            StreamProfile::HifiStereo20ms.bitrate_kbps()
                > StreamProfile::MusicStereo20ms.bitrate_kbps()
        );
    }

    #[test]
    fn given_voice_agent_profile_when_contract_then_direct() {
        assert_eq!(
            StreamProfile::VoiceAgentMono10ms.contract(),
            Contract::direct()
        );
    }

    #[test]
    fn given_voice_mono_profile_when_contract_then_interactive() {
        assert_eq!(
            StreamProfile::VoiceMono20ms.contract(),
            Contract::interactive()
        );
    }

    #[test]
    fn given_broadcast_profile_when_contract_then_continuity() {
        assert_eq!(
            StreamProfile::BroadcastStereo20ms.contract(),
            Contract::continuity()
        );
    }

    #[test]
    fn given_music_profiles_when_contract_then_fidelity() {
        assert_eq!(
            StreamProfile::MusicStereo20ms.contract(),
            Contract::fidelity()
        );
        assert_eq!(
            StreamProfile::HifiStereo20ms.contract(),
            Contract::fidelity()
        );
    }
}
