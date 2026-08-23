//! Canonical port, media-capability, and edge contracts for the typed graph.
//! Negotiation rules decide which graph ports may connect and what each edge
//! guarantees at runtime.

use crate::frame::SampleFormat;

use crate::graph::signal::{BinaryFormat, Codec, SignalClass, SignalSpec};

/// Default maximum owned payload admitted to one asynchronous signal edge.
/// Large streams must be chunked by their source/connector instead of turning
/// one queue item into an unbounded allocation.
pub const DEFAULT_ASYNC_MAX_PAYLOAD_BYTES: usize = 1_048_576;
#[doc = "Sets the maximum supported async payload bytes."]
pub const MAX_ASYNC_PAYLOAD_BYTES: usize = 16_777_216;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "Selects the media kind used by PocketStation."]
pub enum MediaKind {
    #[doc = "Selects audio PCM behavior for `MediaKind`."]
    AudioPcm,
    #[doc = "Selects audio encoded behavior for `MediaKind`."]
    AudioEncoded,
    #[doc = "Selects text behavior for `MediaKind`."]
    Text,
    #[doc = "Selects event behavior for `MediaKind`."]
    Event,
    #[doc = "Selects metrics behavior for `MediaKind`."]
    Metrics,
    #[doc = "Selects control behavior for `MediaKind`."]
    Control,
    #[doc = "Selects binary behavior for `MediaKind`."]
    Binary,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "Enumerates the supported channel layout cases."]
pub enum ChannelLayout {
    #[doc = "Represents the mono case of `ChannelLayout`."]
    Mono,
    #[doc = "Represents the stereo case of `ChannelLayout`."]
    Stereo,
    #[doc = "Represents the any case of `ChannelLayout`."]
    Any, // wildcard; matches any concrete layout during negotiation
}

impl ChannelLayout {
    #[doc = "Returns the channel count associated with `ChannelLayout`."]
    pub fn channel_count(self) -> Option<u8> {
        match self {
            Self::Mono => Some(1),
            Self::Stereo => Some(2),
            Self::Any => None,
        }
    }

    #[doc = "Returns whether compatible with applies to `ChannelLayout`."]
    pub fn is_compatible_with(self, other: ChannelLayout) -> bool {
        matches!(self, Self::Any) || matches!(other, Self::Any) || self == other
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "Represents audio caps in the PocketStation API."]
pub struct AudioCaps {
    #[doc = "Stores the sample rate value for `AudioCaps`, in hertz."]
    pub sample_rate_hz: Option<u32>, // None = any rate accepted
    #[doc = "Stores the frame samples associated with `AudioCaps`."]
    pub frame_samples: Option<usize>, // None = any frame length accepted
    #[doc = "Stores the channel layout associated with `AudioCaps`."]
    pub channel_layout: ChannelLayout,
    #[doc = "Stores the format associated with `AudioCaps`."]
    pub format: SampleFormat,
}

impl AudioCaps {
    #[doc = "Returns whether compatible with applies to `AudioCaps`."]
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
#[doc = "Enumerates the supported media caps cases."]
pub enum MediaCaps {
    #[doc = "Represents the audio case of `MediaCaps`."]
    Audio(AudioCaps),
    #[doc = "Represents the encoded audio case of `MediaCaps`."]
    EncodedAudio(Codec),
    #[doc = "Represents the text case of `MediaCaps`."]
    Text,
    #[doc = "Represents the event case of `MediaCaps`."]
    Event,
    #[doc = "Represents the metrics case of `MediaCaps`."]
    Metrics,
    #[doc = "Represents the control case of `MediaCaps`."]
    Control,
    #[doc = "Represents the binary case of `MediaCaps`."]
    Binary(BinaryFormat),
    #[doc = "Represents the any case of `MediaCaps`."]
    Any, // wildcard port; has no single MediaKind
}

impl MediaCaps {
    #[doc = "Returns the kind represented by `MediaCaps`."]
    pub fn kind(&self) -> Option<MediaKind> {
        match self {
            Self::Audio(_) => Some(MediaKind::AudioPcm),
            Self::EncodedAudio(_) => Some(MediaKind::AudioEncoded),
            Self::Text => Some(MediaKind::Text),
            Self::Event => Some(MediaKind::Event),
            Self::Metrics => Some(MediaKind::Metrics),
            Self::Control => Some(MediaKind::Control),
            Self::Binary(_) => Some(MediaKind::Binary),
            Self::Any => None,
        }
    }

    #[doc = "Returns whether compatible with applies to `MediaCaps`."]
    pub fn is_compatible_with(&self, other: &MediaCaps) -> bool {
        match (self, other) {
            (Self::Any, _) | (_, Self::Any) => true,
            (Self::Audio(a), Self::Audio(b)) => a.is_compatible_with(b),
            (Self::EncodedAudio(a), Self::EncodedAudio(b)) => a == b,
            (Self::Text, Self::Text) => true,
            (Self::Event, Self::Event) => true,
            (Self::Metrics, Self::Metrics) => true,
            (Self::Control, Self::Control) => true,
            (Self::Binary(a), Self::Binary(b)) => a == b,
            _ => false,
        }
    }

    #[doc = "Negotiates the compatible media capabilities shared by `MediaCaps` and its peer."]
    pub fn negotiate(&self, other: &MediaCaps) -> Option<MediaCaps> {
        match (self, other) {
            (Self::Any, narrower) | (narrower, Self::Any) => Some(*narrower),
            (Self::Audio(a), Self::Audio(b)) if a.is_compatible_with(b) => {
                Some(Self::Audio(a.narrow(b)))
            }
            (Self::EncodedAudio(a), Self::EncodedAudio(b)) if a == b => {
                Some(Self::EncodedAudio(*a))
            }
            (Self::Text, Self::Text) => Some(Self::Text),
            (Self::Event, Self::Event) => Some(Self::Event),
            (Self::Metrics, Self::Metrics) => Some(Self::Metrics),
            (Self::Control, Self::Control) => Some(Self::Control),
            (Self::Binary(a), Self::Binary(b)) if a == b => Some(Self::Binary(*a)),
            _ => None,
        }
    }

    #[doc = "Returns whether supports signal applies to `MediaCaps`."]
    pub fn supports_signal(&self, signal: &SignalSpec) -> bool {
        match (&signal.class, self) {
            (_, Self::Any) | (SignalClass::Any, _) => true,
            (SignalClass::PcmAudio, Self::Audio(_)) => true,
            (SignalClass::EncodedAudio(signal_codec), Self::EncodedAudio(media_codec)) => {
                signal_codec == media_codec
            }
            (SignalClass::Text(_), Self::Text) => true,
            (SignalClass::Event(_), Self::Event) => true,
            (SignalClass::Metrics, Self::Metrics) => true,
            (SignalClass::Control, Self::Control) => true,
            (SignalClass::Binary(signal_format), Self::Binary(media_format)) => {
                signal_format == media_format
            }
            (SignalClass::Custom(_), Self::Binary(_)) => signal.schema.is_some(),
            _ => false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[doc = "Selects the port direction used by PocketStation."]
pub enum PortDirection {
    #[doc = "Selects input behavior for `PortDirection`."]
    Input,
    #[doc = "Selects output behavior for `PortDirection`."]
    Output,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "Enumerates the supported multiplicity cases."]
pub enum Multiplicity {
    #[doc = "Represents the one case of `Multiplicity`."]
    One,
    #[doc = "Represents the many case of `Multiplicity`."]
    Many,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "Configures port."]
pub struct PortSpec {
    pub(crate) name: String,
    pub(crate) direction: PortDirection,
    pub(crate) signal: SignalSpec,
    pub(crate) media: MediaCaps,
    pub(crate) multiplicity: Multiplicity,
    pub(crate) required: bool,
}

impl PortSpec {
    #[doc = "Creates a new `PortSpec`."]
    pub fn new(
        name: impl Into<String>,
        direction: PortDirection,
        signal: SignalSpec,
        media: MediaCaps,
        multiplicity: Multiplicity,
        required: bool,
    ) -> Result<Self, PortSpecError> {
        let name = name.into();
        if name.trim().is_empty() {
            return Err(PortSpecError::EmptyName);
        }
        signal
            .validate()
            .map_err(|_| PortSpecError::InvalidSignal)?;
        if !media.supports_signal(&signal) {
            return Err(PortSpecError::SignalMediaMismatch);
        }
        Ok(Self {
            name,
            direction,
            signal,
            media,
            multiplicity,
            required,
        })
    }

    #[doc = "Returns the name associated with `PortSpec`."]
    pub fn name(&self) -> &str {
        &self.name
    }

    #[doc = "Returns the direction associated with `PortSpec`."]
    pub const fn direction(&self) -> PortDirection {
        self.direction
    }

    #[doc = "Returns the signal associated with `PortSpec`."]
    pub const fn signal(&self) -> &SignalSpec {
        &self.signal
    }

    #[doc = "Returns the media associated with `PortSpec`."]
    pub const fn media(&self) -> MediaCaps {
        self.media
    }

    #[doc = "Returns the multiplicity associated with `PortSpec`."]
    pub const fn multiplicity(&self) -> Multiplicity {
        self.multiplicity
    }

    #[doc = "Returns the required associated with `PortSpec`."]
    pub const fn required(&self) -> bool {
        self.required
    }
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[doc = "Classifies failures reported as port spec error."]
pub enum PortSpecError {
    #[error("port name cannot be empty")]
    #[doc = "Reports empty name."]
    EmptyName,
    #[error("port SignalSpec is invalid")]
    #[doc = "Reports invalid signal."]
    InvalidSignal,
    #[error("port signal and media representation are incompatible")]
    #[doc = "Reports signal media mismatch."]
    SignalMediaMismatch,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "Enumerates the supported clock domain cases."]
pub enum ClockDomain {
    #[doc = "Represents the capture case of `ClockDomain`."]
    Capture,
    #[doc = "Represents the playback case of `ClockDomain`."]
    Playback,
    #[doc = "Represents the network case of `ClockDomain`."]
    Network,
    /// Preserve the clock carried by the producer's signal envelope.
    Inherited,
    #[doc = "Represents the wallclock case of `ClockDomain`."]
    Wallclock,
}

impl ClockDomain {
    #[doc = "Returns whether realtime applies to `ClockDomain`."]
    pub fn is_realtime(self) -> bool {
        matches!(self, Self::Capture | Self::Playback)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "Selects the backpressure policy used by PocketStation."]
pub enum BackpressurePolicy {
    #[doc = "Selects drop newest behavior for `BackpressurePolicy`."]
    DropNewest, // shed the incoming frame; preserves already-queued order
    #[doc = "Selects drop oldest behavior for `BackpressurePolicy`."]
    DropOldest, // evict head to admit newest; freshest-wins for realtime audio
    #[doc = "Selects bounded queue behavior for `BackpressurePolicy`."]
    BoundedQueue, // block producer only via capacity; never drops silently
    #[doc = "Selects block forbidden behavior for `BackpressurePolicy`."]
    BlockForbidden, // producer must never block; overflow is a hard error upstream
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "Selects the delivery semantics used by PocketStation."]
pub enum DeliverySemantics {
    #[doc = "Indicates the best effort realtime state for `DeliverySemantics`."]
    BestEffortRealtime,
    #[doc = "Indicates the ordered state for `DeliverySemantics`."]
    Ordered,
    #[doc = "Indicates the exactly once not realtime state for `DeliverySemantics`."]
    ExactlyOnceNotRealtime,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "Selects the copy policy used by PocketStation."]
pub enum CopyPolicy {
    #[doc = "Selects move exclusive behavior for `CopyPolicy`."]
    MoveExclusive,
    #[doc = "Selects share read only behavior for `CopyPolicy`."]
    ShareReadOnly,
    #[doc = "Selects copy to branch pool behavior for `CopyPolicy`."]
    CopyToBranchPool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "Selects the loss policy used by PocketStation."]
pub enum LossPolicy {
    #[doc = "Selects conceal for audio behavior for `LossPolicy`."]
    ConcealForAudio, // PLC-eligible; dropped audio is concealed downstream
    #[doc = "Selects must deliver or fail behavior for `LossPolicy`."]
    MustDeliverOrFail, // terminal output must be delivered or the branch fails visibly
    #[doc = "Selects drop allowed behavior for `LossPolicy`."]
    DropAllowed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "Selects the edge observability level used by PocketStation."]
pub enum EdgeObservabilityLevel {
    #[doc = "Selects off behavior for `EdgeObservabilityLevel`."]
    Off,
    #[doc = "Selects counters behavior for `EdgeObservabilityLevel`."]
    Counters,
    #[doc = "Reports that bounded capacity is full."]
    Full,
}

impl EdgeObservabilityLevel {
    #[doc = "Returns the rank associated with `EdgeObservabilityLevel`."]
    pub fn rank(self) -> u8 {
        match self {
            Self::Off => 0,
            Self::Counters => 1,
            Self::Full => 2,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "Represents edge contract in the PocketStation API."]
pub struct EdgeContract {
    pub(crate) media: MediaCaps,
    pub(crate) clock: ClockDomain,
    pub(crate) latency_budget_ms: Option<u32>,
    pub(crate) jitter_budget_ms: Option<u32>,
    pub(crate) backpressure: BackpressurePolicy,
    pub(crate) delivery: DeliverySemantics,
    pub(crate) loss: LossPolicy,
    pub(crate) copy_policy: CopyPolicy,
    pub(crate) observability: EdgeObservabilityLevel,
    pub(crate) max_payload_bytes: Option<usize>,
}

impl EdgeContract {
    #[doc = "Returns the media associated with `EdgeContract`."]
    pub const fn media(&self) -> MediaCaps {
        self.media
    }

    #[doc = "Returns the clock associated with `EdgeContract`."]
    pub const fn clock(&self) -> ClockDomain {
        self.clock
    }

    #[doc = "Returns the latency budget milliseconds associated with `EdgeContract`."]
    pub const fn latency_budget_ms(&self) -> Option<u32> {
        self.latency_budget_ms
    }

    #[doc = "Returns the jitter budget milliseconds associated with `EdgeContract`."]
    pub const fn jitter_budget_ms(&self) -> Option<u32> {
        self.jitter_budget_ms
    }

    #[doc = "Returns the backpressure associated with `EdgeContract`."]
    pub const fn backpressure(&self) -> BackpressurePolicy {
        self.backpressure
    }

    #[doc = "Returns the delivery associated with `EdgeContract`."]
    pub const fn delivery(&self) -> DeliverySemantics {
        self.delivery
    }

    #[doc = "Returns the loss associated with `EdgeContract`."]
    pub const fn loss(&self) -> LossPolicy {
        self.loss
    }

    #[doc = "Returns the copy policy associated with `EdgeContract`."]
    pub const fn copy_policy(&self) -> CopyPolicy {
        self.copy_policy
    }

    #[doc = "Returns the observability associated with `EdgeContract`."]
    pub const fn observability(&self) -> EdgeObservabilityLevel {
        self.observability
    }

    #[doc = "Returns the max payload bytes associated with `EdgeContract`."]
    pub const fn max_payload_bytes(&self) -> Option<usize> {
        self.max_payload_bytes
    }

    #[doc = "Sets the media on `EdgeContract` and returns the updated value."]
    pub fn with_media(mut self, media: MediaCaps) -> Self {
        self.media = media;
        self
    }

    #[doc = "Sets the backpressure on `EdgeContract` and returns the updated value."]
    pub fn with_backpressure(mut self, backpressure: BackpressurePolicy) -> Self {
        self.backpressure = backpressure;
        self
    }

    #[doc = "Sets the copy policy on `EdgeContract` and returns the updated value."]
    pub fn with_copy_policy(mut self, copy_policy: CopyPolicy) -> Self {
        self.copy_policy = copy_policy;
        self
    }

    #[doc = "Sets the jitter budget milliseconds on `EdgeContract` and returns the updated value."]
    pub fn with_jitter_budget_ms(mut self, jitter_budget_ms: Option<u32>) -> Self {
        self.jitter_budget_ms = jitter_budget_ms;
        self
    }

    #[doc = "Sets the max payload bytes on `EdgeContract` and returns the updated value."]
    pub fn with_max_payload_bytes(mut self, max_payload_bytes: usize) -> Self {
        self.max_payload_bytes = Some(max_payload_bytes);
        self
    }
    /// Generic realtime PCM edge. Concrete sample rate, frame size, and
    /// channel layout are negotiated from connected ports.
    pub fn realtime_audio() -> Self {
        Self {
            media: MediaCaps::Audio(AudioCaps {
                sample_rate_hz: None,
                frame_samples: None,
                channel_layout: ChannelLayout::Any,
                format: SampleFormat::F32Interleaved,
            }),
            clock: ClockDomain::Capture,
            latency_budget_ms: None,
            jitter_budget_ms: None,
            backpressure: BackpressurePolicy::DropNewest,
            delivery: DeliverySemantics::Ordered,
            loss: LossPolicy::ConcealForAudio,
            copy_policy: CopyPolicy::ShareReadOnly,
            observability: EdgeObservabilityLevel::Counters,
            max_payload_bytes: None,
        }
    }

    /// Generic bounded asynchronous edge. Connected ports supply the payload
    /// representation and the envelope preserves its producer clock.
    pub fn bounded_async() -> Self {
        Self {
            media: MediaCaps::Any,
            clock: ClockDomain::Inherited,
            latency_budget_ms: None,
            jitter_budget_ms: None,
            backpressure: BackpressurePolicy::BoundedQueue,
            delivery: DeliverySemantics::Ordered,
            loss: LossPolicy::MustDeliverOrFail,
            copy_policy: CopyPolicy::ShareReadOnly,
            observability: EdgeObservabilityLevel::Counters,
            max_payload_bytes: Some(DEFAULT_ASYNC_MAX_PAYLOAD_BYTES),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    fn stereo_caps() -> AudioCaps {
        AudioCaps {
            sample_rate_hz: Some(48_000),
            frame_samples: Some(960),
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
    fn given_realtime_audio_when_built_then_physical_caps_remain_negotiable() {
        let edge = EdgeContract::realtime_audio();
        assert_eq!(edge.latency_budget_ms, None);
        assert_eq!(edge.jitter_budget_ms, None);
        assert_eq!(edge.backpressure, BackpressurePolicy::DropNewest);
        assert_eq!(edge.loss, LossPolicy::ConcealForAudio);
        assert!(matches!(
            edge.media,
            MediaCaps::Audio(AudioCaps {
                sample_rate_hz: None,
                frame_samples: None,
                ..
            })
        ));
    }

    #[test]
    fn given_bounded_async_when_built_then_contains_no_payload_or_clock_origin_assumption() {
        let edge = EdgeContract::bounded_async();
        assert_eq!(edge.loss, LossPolicy::MustDeliverOrFail);
        assert_eq!(edge.backpressure, BackpressurePolicy::BoundedQueue);
        assert_eq!(edge.media, MediaCaps::Any);
        assert_eq!(edge.delivery, DeliverySemantics::Ordered);
        assert_eq!(edge.clock, ClockDomain::Inherited);
        assert_eq!(edge.latency_budget_ms, None);
        assert_eq!(edge.jitter_budget_ms, None);
        assert_eq!(
            edge.max_payload_bytes,
            Some(DEFAULT_ASYNC_MAX_PAYLOAD_BYTES)
        );
    }

    #[test]
    fn given_observability_levels_when_ranked_then_ordered_ascending() {
        assert!(EdgeObservabilityLevel::Off.rank() < EdgeObservabilityLevel::Counters.rank());
        assert!(EdgeObservabilityLevel::Counters.rank() < EdgeObservabilityLevel::Full.rank());
    }

    #[test]
    fn given_supported_non_audio_signals_when_checked_then_media_is_symmetric() {
        assert!(MediaCaps::EncodedAudio(Codec::Opus)
            .supports_signal(&SignalSpec::encoded_audio(Codec::Opus)));
        assert!(MediaCaps::Text.supports_signal(&SignalSpec::text(crate::graph::TextFormat::Json)));
        assert!(MediaCaps::Event
            .supports_signal(&SignalSpec::event(crate::graph::EventFormat::Protobuf)));
        assert!(MediaCaps::Metrics.supports_signal(&SignalSpec::metrics()));
        assert!(MediaCaps::Control.supports_signal(&SignalSpec::control()));
        assert!(MediaCaps::Binary(BinaryFormat::Cbor)
            .supports_signal(&SignalSpec::binary(BinaryFormat::Cbor)));
        assert!(MediaCaps::Binary(BinaryFormat::Protobuf).supports_signal(
            &SignalSpec::custom("com.acme.signal.v1").with_schema("proto:acme.Signal")
        ));
    }

    #[test]
    fn given_custom_signal_without_schema_when_checked_then_binary_media_rejects_it() {
        assert!(!MediaCaps::Binary(BinaryFormat::Raw)
            .supports_signal(&SignalSpec::custom("com.acme.signal.v1")));
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
