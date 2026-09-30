//! Fixed-size provenance for processed audio, including bounded processor tails.

use super::{SourceId, StreamId};

/// The actual input consumed to produce an audio frame.
///
/// Output identity, sequence and timing belong to the containing frame. These
/// fields retain the most recent actual input, including when a processor emits
/// buffered audio after input ends. They do not describe every sample retained
/// in an adaptive processor's history. Operator identity belongs to the Session
/// declaration; this record remains allocation-free when copied or dropped.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AudioProcessing {
    pub input_source_id: SourceId,
    pub input_stream_id: StreamId,
    pub input_sequence_number: u64,
    pub input_timestamp_ns: u64,
    pub input_duration_ns: u64,
    pub input_source_generation: u32,
    pub input_discontinuity_epoch: u64,
    /// Adaptation generation; a reset starts a new generation.
    pub generation: u64,
    /// Nominal signal delay in samples per channel, separate from CPU time.
    pub nominal_delay_samples: u32,
    /// Internal zero-input samples per channel used for this terminal frame.
    /// A nonzero value marks processor tail output, not additional capture.
    pub padding_samples: u32,
    /// Tail frame offset from actual input EOF, in samples per channel.
    /// The first tail frame has offset zero.
    pub tail_offset_samples: u32,
}

impl AudioProcessing {
    /// Whether this frame drains processor history after actual input ended.
    pub const fn is_tail(self) -> bool {
        self.padding_samples != 0
    }
}

#[cfg(all(test, target_pointer_width = "64"))]
mod tests {
    use super::AudioProcessing;
    use crate::connector::ConnectorItem;
    use crate::frame::{AudioFrame, LineagedAudioFrame};
    use crate::runtime::PlanSourceSendOutcome;

    #[test]
    fn given_inline_audio_ownership_when_layout_is_measured_then_storage_remains_bounded() {
        // These are reviewed 64-bit storage ceilings, not a stable C ABI.
        // Metadata stays inline instead of introducing a per-frame allocation;
        // growth must revisit the queue-memory and stack budgets explicitly.
        let layouts = [
            ("AudioProcessing", size_of::<AudioProcessing>(), 72),
            (
                "Option<AudioProcessing>",
                size_of::<Option<AudioProcessing>>(),
                80,
            ),
            ("AudioFrame", size_of::<AudioFrame>(), 136),
            ("LineagedAudioFrame", size_of::<LineagedAudioFrame>(), 224),
            (
                "PlanSourceSendOutcome",
                size_of::<PlanSourceSendOutcome>(),
                232,
            ),
            ("ConnectorItem", size_of::<ConnectorItem<'static>>(), 256),
        ];
        for (name, bytes, ceiling_bytes) in layouts {
            assert!(
                bytes <= ceiling_bytes,
                "{name}: {bytes} bytes exceeds {ceiling_bytes}"
            );
        }
        assert!(!std::mem::needs_drop::<AudioProcessing>());
    }
}
