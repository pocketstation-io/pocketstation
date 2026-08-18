//! Explicit bounded crossings between execution partitions.

mod audio;

pub(crate) use audio::GeneratedAudioBridgeObservations;
pub use audio::{GeneratedAudioBridge, GeneratedAudioBridgeSpec};
