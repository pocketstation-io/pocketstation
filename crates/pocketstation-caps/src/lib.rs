//! pocketstation-caps — typed media contracts for the audio graph.
//! Negotiation rules here decide which node ports may connect and what an
//! edge between them guarantees at runtime.

use pocketstation_frame::{EncryptionMode, SampleFormat};

const VOICE_SAMPLE_RATE_HZ: u32 = 48_000; // internal canonical rate (ADR-013)
const VOICE_FRAME_SAMPLES: usize = 960; // 20ms × 48kHz (ADR-012)
const VOICE_LATENCY_BUDGET_MS: u32 = 170; // mouth-to-ear target (ADR-010)
const VOICE_JITTER_BUDGET_MS: u32 = 60; // de-jitter headroom (ADR-010)

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MediaKind {
    AudioPcm,
    AudioEncoded,
    Text,
    Event,
    Control,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChannelLayout {
    Mono,
    Stereo,
    Any, // wildcard; matches any concrete layout during negotiation
}

impl ChannelLayout {
    pub fn channel_count(self) -> Option<u8> {
        match self {
            Self::Mono => Some(1),
            Self::Stereo => Some(2),
            Self::Any => None,
        }
    }

    pub fn is_compatible_with(self, other: ChannelLayout) -> bool {
        matches!(self, Self::Any) || matches!(other, Self::Any) || self == other
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AudioCaps {
    pub sample_rate_hz: Option<u32>,  // None = any rate accepted
    pub frame_samples: Option<usize>, // None = any frame length accepted
    pub channel_layout: ChannelLayout,
    pub format: SampleFormat,
}

impl AudioCaps {
    pub fn is_compatible_with(&self, other: &AudioCaps) -> bool {
        Self::scalar_compatible(self.sample_rate_hz, other.sample_rate_hz)
            && Self::scalar_compatible(self.frame_samples, other.frame_samples)
            && self.channel_layout.is_compatible_with(other.channel_layout)
            && self.format == other.format
    }

    fn scalar_compatible<T: PartialEq>(lhs: Option<T>, rhs: Option<T>) -> bool {
        match (lhs, rhs) {
            (Some(a), Some(b)) => a == b,
            _ => true,
        }
    }

    fn narrow(&self, other: &AudioCaps) -> AudioCaps {
        AudioCaps {
            sample_rate_hz: self.sample_rate_hz.or(other.sample_rate_hz),
            frame_samples: self.frame_samples.or(other.frame_samples),
            channel_layout: if matches!(self.channel_layout, ChannelLayout::Any) {
                other.channel_layout
            } else {
                self.channel_layout
            },
            format: self.format,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MediaCaps {
    Audio(AudioCaps),
    Text,
    Event,
    Control,
    Any, // wildcard port; has no single MediaKind
}

impl MediaCaps {
    pub fn kind(&self) -> Option<MediaKind> {
        match self {
            Self::Audio(_) => Some(MediaKind::AudioPcm),
            Self::Text => Some(MediaKind::Text),
            Self::Event => Some(MediaKind::Event),
            Self::Control => Some(MediaKind::Control),
            Self::Any => None,
        }
    }

    pub fn is_compatible_with(&self, other: &MediaCaps) -> bool {
        match (self, other) {
            (Self::Any, _) | (_, Self::Any) => true,
            (Self::Audio(a), Self::Audio(b)) => a.is_compatible_with(b),
            (Self::Text, Self::Text) => true,
            (Self::Event, Self::Event) => true,
            (Self::Control, Self::Control) => true,
            _ => false,
        }
    }

    pub fn negotiate(&self, other: &MediaCaps) -> Option<MediaCaps> {
        match (self, other) {
            (Self::Any, narrower) | (narrower, Self::Any) => Some(*narrower),
            (Self::Audio(a), Self::Audio(b)) if a.is_compatible_with(b) => {
                Some(Self::Audio(a.narrow(b)))
            }
            (Self::Text, Self::Text) => Some(Self::Text),
            (Self::Event, Self::Event) => Some(Self::Event),
            (Self::Control, Self::Control) => Some(Self::Control),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PortDirection {
    Input,
    Output,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Multiplicity {
    One,
    Many,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PortSpec {
    pub name: String,
    pub direction: PortDirection,
    pub media: MediaCaps,
    pub multiplicity: Multiplicity,
    pub required: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClockDomain {
    Capture,
    Playback,
    Network,
    Model,
    Wallclock,
}

impl ClockDomain {
    pub fn is_realtime(self) -> bool {
        matches!(self, Self::Capture | Self::Playback)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BackpressurePolicy {
    DropNewest,     // shed the incoming frame; preserves already-queued order
    DropOldest,     // evict head to admit newest; freshest-wins for realtime audio
    BoundedQueue,   // block producer only via capacity; never drops silently
    BlockForbidden, // producer must never block; overflow is a hard error upstream
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeliverySemantics {
    BestEffortRealtime,
    Ordered,
    ExactlyOnceNotRealtime,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CopyPolicy {
    BorrowReadOnly,
    SharedBufferHandle,
    CopyAllowed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LossPolicy {
    ConcealForAudio,          // PLC-eligible; dropped audio is concealed downstream
    NeverDropFinalTranscript, // final model output must survive backpressure
    DropAllowed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EdgeObservabilityLevel {
    Off,
    Counters,
    Full,
}

impl EdgeObservabilityLevel {
    pub fn rank(self) -> u8 {
        match self {
            Self::Off => 0,
            Self::Counters => 1,
            Self::Full => 2,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EdgeContract {
    pub media: MediaCaps,
    pub clock: ClockDomain,
    pub latency_budget_ms: Option<u32>,
    pub jitter_budget_ms: Option<u32>,
    pub backpressure: BackpressurePolicy,
    pub delivery: DeliverySemantics,
    pub loss: LossPolicy,
    pub copy_policy: CopyPolicy,
    pub encryption: EncryptionMode,
    pub observability: EdgeObservabilityLevel,
}

impl EdgeContract {
    pub fn voice_default() -> Self {
        Self {
            media: MediaCaps::Audio(AudioCaps {
                sample_rate_hz: Some(VOICE_SAMPLE_RATE_HZ),
                frame_samples: Some(VOICE_FRAME_SAMPLES),
                channel_layout: ChannelLayout::Any,
                format: SampleFormat::F32Interleaved,
            }),
            clock: ClockDomain::Capture,
            latency_budget_ms: Some(VOICE_LATENCY_BUDGET_MS),
            jitter_budget_ms: Some(VOICE_JITTER_BUDGET_MS),
            backpressure: BackpressurePolicy::DropNewest,
            delivery: DeliverySemantics::Ordered,
            loss: LossPolicy::ConcealForAudio,
            copy_policy: CopyPolicy::SharedBufferHandle,
            encryption: EncryptionMode::None,
            observability: EdgeObservabilityLevel::Counters,
        }
    }

    pub fn model_default() -> Self {
        Self {
            media: MediaCaps::Event,
            backpressure: BackpressurePolicy::BoundedQueue,
            loss: LossPolicy::NeverDropFinalTranscript,
            delivery: DeliverySemantics::Ordered,
            ..Self::voice_default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    fn stereo_caps() -> AudioCaps {
        AudioCaps {
            sample_rate_hz: Some(VOICE_SAMPLE_RATE_HZ),
            frame_samples: Some(VOICE_FRAME_SAMPLES),
            channel_layout: ChannelLayout::Stereo,
            format: SampleFormat::F32Interleaved,
        }
    }

    #[test]
    fn given_mono_and_stereo_when_channel_count_then_returns_one_and_two() {
        assert_eq!(ChannelLayout::Mono.channel_count(), Some(1));
        assert_eq!(ChannelLayout::Stereo.channel_count(), Some(2));
        assert_eq!(ChannelLayout::Any.channel_count(), None);
    }

    #[test]
    fn given_any_layout_when_compat_checked_both_directions_then_matches() {
        assert!(ChannelLayout::Any.is_compatible_with(ChannelLayout::Mono));
        assert!(ChannelLayout::Stereo.is_compatible_with(ChannelLayout::Any));
        assert!(!ChannelLayout::Mono.is_compatible_with(ChannelLayout::Stereo));
    }

    #[test]
    fn given_wildcard_rate_when_audio_compat_checked_then_matches_concrete() {
        let wildcard = AudioCaps {
            sample_rate_hz: None,
            frame_samples: None,
            channel_layout: ChannelLayout::Any,
            format: SampleFormat::F32Interleaved,
        };
        assert!(wildcard.is_compatible_with(&stereo_caps()));
        assert!(stereo_caps().is_compatible_with(&wildcard));
    }

    #[test]
    fn given_mismatched_rate_when_audio_compat_checked_then_incompatible() {
        let other = AudioCaps {
            sample_rate_hz: Some(44_100),
            ..stereo_caps()
        };
        assert!(!stereo_caps().is_compatible_with(&other));
    }

    #[test]
    fn given_audio_pair_when_media_compat_checked_then_compatible() {
        let a = MediaCaps::Audio(stereo_caps());
        let b = MediaCaps::Audio(stereo_caps());
        assert!(a.is_compatible_with(&b));
    }

    #[test]
    fn given_audio_and_text_when_media_compat_checked_then_incompatible() {
        let audio = MediaCaps::Audio(stereo_caps());
        assert!(!audio.is_compatible_with(&MediaCaps::Text));
    }

    #[test]
    fn given_any_media_when_compat_checked_both_directions_then_matches() {
        let audio = MediaCaps::Audio(stereo_caps());
        assert!(MediaCaps::Any.is_compatible_with(&audio));
        assert!(audio.is_compatible_with(&MediaCaps::Any));
    }

    #[test]
    fn given_any_and_audio_when_negotiated_then_yields_audio() {
        let audio = MediaCaps::Audio(stereo_caps());
        assert_eq!(MediaCaps::Any.negotiate(&audio), Some(audio));
    }

    #[test]
    fn given_incompatible_media_when_negotiated_then_none() {
        let audio = MediaCaps::Audio(stereo_caps());
        assert_eq!(audio.negotiate(&MediaCaps::Text), None);
    }

    #[test]
    fn given_wildcard_audio_when_negotiated_then_narrows_to_concrete() {
        let wildcard = MediaCaps::Audio(AudioCaps {
            sample_rate_hz: None,
            frame_samples: None,
            channel_layout: ChannelLayout::Any,
            format: SampleFormat::F32Interleaved,
        });
        let concrete = MediaCaps::Audio(stereo_caps());
        assert_eq!(wildcard.negotiate(&concrete), Some(concrete));
    }

    #[test]
    fn given_voice_default_when_built_then_carries_voice_budgets() {
        let edge = EdgeContract::voice_default();
        assert_eq!(edge.latency_budget_ms, Some(VOICE_LATENCY_BUDGET_MS));
        assert_eq!(edge.jitter_budget_ms, Some(VOICE_JITTER_BUDGET_MS));
        assert_eq!(edge.backpressure, BackpressurePolicy::DropNewest);
        assert_eq!(edge.loss, LossPolicy::ConcealForAudio);
    }

    #[test]
    fn given_model_default_when_built_then_overrides_loss_and_backpressure() {
        let edge = EdgeContract::model_default();
        assert_eq!(edge.loss, LossPolicy::NeverDropFinalTranscript);
        assert_eq!(edge.backpressure, BackpressurePolicy::BoundedQueue);
        assert_eq!(edge.media, MediaCaps::Event);
        assert_eq!(edge.delivery, DeliverySemantics::Ordered);
    }

    #[test]
    fn given_observability_levels_when_ranked_then_ordered_ascending() {
        assert!(EdgeObservabilityLevel::Off.rank() < EdgeObservabilityLevel::Counters.rank());
        assert!(EdgeObservabilityLevel::Counters.rank() < EdgeObservabilityLevel::Full.rank());
    }

    fn any_layout() -> impl Strategy<Value = ChannelLayout> {
        prop_oneof![
            Just(ChannelLayout::Mono),
            Just(ChannelLayout::Stereo),
            Just(ChannelLayout::Any),
        ]
    }

    fn any_audio_caps() -> impl Strategy<Value = AudioCaps> {
        (
            prop::option::of(any::<u32>()),
            prop::option::of(any::<usize>()),
            any_layout(),
        )
            .prop_map(
                |(sample_rate_hz, frame_samples, channel_layout)| AudioCaps {
                    sample_rate_hz,
                    frame_samples,
                    channel_layout,
                    format: SampleFormat::F32Interleaved,
                },
            )
    }

    proptest! {
        #[test]
        fn given_any_audio_caps_when_compat_checked_then_reflexive_and_symmetric(
            a in any_audio_caps(),
            b in any_audio_caps(),
        ) {
            prop_assert!(a.is_compatible_with(&a));
            prop_assert_eq!(a.is_compatible_with(&b), b.is_compatible_with(&a));
        }
    }
}
