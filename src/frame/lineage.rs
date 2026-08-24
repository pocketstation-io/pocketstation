//! Compact source-aware lineage carried on realtime audio frames.

use crate::frame::{ClockDomainId, SessionId, SourceId, StemId};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "Preserves source, stream, sequence, clock, generation, and discontinuity identity for an audio frame."]
pub struct FrameLineage {
    pub(crate) session_id: SessionId,
    pub(crate) source_id: SourceId,
    pub(crate) stem_id: StemId,
    pub(crate) clock_id: ClockDomainId,
    pub(crate) sequence_num: u64,
    pub(crate) timestamp_start_ns: u64,
    pub(crate) duration_ns: u64,
    pub(crate) source_generation: u32,
    pub(crate) discontinuity_epoch: u64,
    pub(crate) permission_epoch: u64,
}

impl FrameLineage {
    #[allow(clippy::too_many_arguments)]
    #[doc = "Creates a new `FrameLineage` after validating its inputs."]
    pub fn try_new(
        session_id: SessionId,
        source_id: SourceId,
        stem_id: StemId,
        clock_id: ClockDomainId,
        sequence_num: u64,
        timestamp_start_ns: u64,
        duration_ns: u64,
        source_generation: u32,
        discontinuity_epoch: u64,
        permission_epoch: u64,
    ) -> Result<Self, FrameLineageBuildError> {
        if duration_ns == 0 {
            return Err(FrameLineageBuildError::ZeroDuration);
        }
        if source_generation == 0 {
            return Err(FrameLineageBuildError::ZeroSourceGeneration);
        }
        timestamp_start_ns
            .checked_add(duration_ns)
            .ok_or(FrameLineageBuildError::TimestampOverflow)?;
        Ok(Self {
            session_id,
            source_id,
            stem_id,
            clock_id,
            sequence_num,
            timestamp_start_ns,
            duration_ns,
            source_generation,
            discontinuity_epoch,
            permission_epoch,
        })
    }

    #[doc = "Returns the session identifier held by `FrameLineage`."]
    pub const fn session_id(self) -> SessionId {
        self.session_id
    }
    #[doc = "Returns the source identifier held by `FrameLineage`."]
    pub const fn source_id(self) -> SourceId {
        self.source_id
    }
    #[doc = "Returns the stem identifier held by `FrameLineage`."]
    pub const fn stem_id(self) -> StemId {
        self.stem_id
    }
    #[doc = "Returns the clock identifier held by `FrameLineage`."]
    pub const fn clock_id(self) -> ClockDomainId {
        self.clock_id
    }
    #[doc = "Returns the sequence number held by `FrameLineage`."]
    pub const fn sequence_number(self) -> u64 {
        self.sequence_num
    }
    #[doc = "Returns the timestamp start nanoseconds held by `FrameLineage`."]
    pub const fn timestamp_start_ns(self) -> u64 {
        self.timestamp_start_ns
    }
    #[doc = "Returns the duration nanoseconds held by `FrameLineage`."]
    pub const fn duration_ns(self) -> u64 {
        self.duration_ns
    }
    #[doc = "Returns the source generation held by `FrameLineage`."]
    pub const fn source_generation(self) -> u32 {
        self.source_generation
    }
    #[doc = "Returns the discontinuity epoch held by `FrameLineage`."]
    pub const fn discontinuity_epoch(self) -> u64 {
        self.discontinuity_epoch
    }
    #[doc = "Returns the permission epoch held by `FrameLineage`."]
    pub const fn permission_epoch(self) -> u64 {
        self.permission_epoch
    }

    #[doc = "Returns the timestamp end nanoseconds held by `FrameLineage`."]
    pub fn timestamp_end_ns(self) -> u64 {
        self.timestamp_start_ns.saturating_add(self.duration_ns)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
#[doc = "Classifies failures produced during frame lineage construction and input validation."]
pub enum FrameLineageBuildError {
    #[error("frame lineage duration must be non-zero")]
    #[doc = "Reports that duration must be greater than zero."]
    ZeroDuration,
    #[error("frame lineage source generation must be non-zero")]
    #[doc = "Reports that source generation must be greater than zero."]
    ZeroSourceGeneration,
    #[error("frame lineage timestamp range overflows u64 nanoseconds")]
    #[doc = "Reports that timestamp exceeds its numeric range."]
    TimestampOverflow,
}
