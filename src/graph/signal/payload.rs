//! Fundamental runtime payload representations.
//!
//! Semantic meaning belongs to `SignalSpec`; this enum must not grow
//! provider-, customer-, or industry-specific variants.

use crate::frame::AudioFrame;
use crate::graph::signal::{SignalClass, SignalSpec};

#[derive(Debug)]
#[doc = "Enumerates the supported signal payload cases."]
pub enum SignalPayload {
    #[doc = "Represents the audio case of `SignalPayload`."]
    Audio(AudioFrame),
    #[doc = "Represents the text case of `SignalPayload`."]
    Text(String),
    #[doc = "Represents the bytes case of `SignalPayload`."]
    Bytes(Vec<u8>),
}

impl SignalPayload {
    #[doc = "Returns the supports associated with `SignalPayload`."]
    pub fn supports(&self, spec: &SignalSpec) -> bool {
        matches!(
            (self, &spec.class),
            (_, SignalClass::Any)
                | (Self::Audio(_), SignalClass::PcmAudio)
                | (Self::Text(_), SignalClass::Text(_))
                | (
                    Self::Bytes(_),
                    SignalClass::EncodedAudio(_)
                        | SignalClass::Event(_)
                        | SignalClass::Metrics
                        | SignalClass::Control
                        | SignalClass::Binary(_)
                        | SignalClass::Custom(_),
                )
        )
    }

    /// Owned media bytes represented by this payload. Envelope metadata and
    /// queue slot storage are fixed-size and accounted separately by the edge.
    pub fn size_bytes(&self) -> usize {
        match self {
            Self::Audio(frame) => frame
                .samples()
                .len()
                .saturating_mul(std::mem::size_of::<f32>()),
            Self::Text(text) => text.len(),
            Self::Bytes(bytes) => bytes.len(),
        }
    }
}
