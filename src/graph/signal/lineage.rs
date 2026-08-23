//! Source-independent signal lineage and derivation.

use crate::frame::{ClockDomainId, ConnectorId, FrameLineage, SessionId, SourceId, StreamId};
use crate::graph::operator::OperatorId;
use crate::graph::signal::SignalTiming;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "Preserves source, stream, generation, discontinuity, and policy identity across signal processing."]
pub struct SignalLineage {
    pub(crate) session_id: SessionId,
    pub(crate) stream_id: StreamId,
    pub(crate) source_id: SourceId,
    pub(crate) clock_id: ClockDomainId,
    pub(crate) sequence_number: u64,
    pub(crate) source_generation: u32,
    pub(crate) discontinuity_epoch: u64,
    pub(crate) policy_epoch: u64,
}

impl SignalLineage {
    #[allow(clippy::too_many_arguments)]
    #[doc = "Creates a new `SignalLineage` after validating its inputs."]
    pub fn try_new(
        session_id: SessionId,
        stream_id: StreamId,
        source_id: SourceId,
        clock_id: ClockDomainId,
        sequence_number: u64,
        source_generation: u32,
        discontinuity_epoch: u64,
        policy_epoch: u64,
    ) -> Result<Self, SignalLineageError> {
        if source_generation == 0 {
            return Err(SignalLineageError::ZeroSourceGeneration);
        }
        Ok(Self {
            session_id,
            stream_id,
            source_id,
            clock_id,
            sequence_number,
            source_generation,
            discontinuity_epoch,
            policy_epoch,
        })
    }

    #[doc = "Creates `SignalLineage` from frame."]
    pub const fn from_frame(stream_id: StreamId, lineage: FrameLineage) -> Self {
        Self {
            session_id: lineage.session_id,
            stream_id,
            source_id: lineage.source_id,
            clock_id: lineage.clock_id,
            sequence_number: lineage.sequence_num,
            source_generation: lineage.source_generation,
            discontinuity_epoch: lineage.discontinuity_epoch,
            policy_epoch: lineage.permission_epoch,
        }
    }

    #[doc = "Returns the session identifier held by `SignalLineage`."]
    pub const fn session_id(self) -> SessionId {
        self.session_id
    }
    #[doc = "Returns the stream identifier held by `SignalLineage`."]
    pub const fn stream_id(self) -> StreamId {
        self.stream_id
    }
    #[doc = "Returns the source identifier held by `SignalLineage`."]
    pub const fn source_id(self) -> SourceId {
        self.source_id
    }
    #[doc = "Returns the clock identifier held by `SignalLineage`."]
    pub const fn clock_id(self) -> ClockDomainId {
        self.clock_id
    }
    #[doc = "Returns the sequence number held by `SignalLineage`."]
    pub const fn sequence_number(self) -> u64 {
        self.sequence_number
    }
    #[doc = "Returns the source generation held by `SignalLineage`."]
    pub const fn source_generation(self) -> u32 {
        self.source_generation
    }
    #[doc = "Returns the discontinuity epoch held by `SignalLineage`."]
    pub const fn discontinuity_epoch(self) -> u64 {
        self.discontinuity_epoch
    }
    #[doc = "Returns the policy epoch held by `SignalLineage`."]
    pub const fn policy_epoch(self) -> u64 {
        self.policy_epoch
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
#[doc = "Classifies failures reported as signal lineage error."]
pub enum SignalLineageError {
    #[error("signal lineage source generation must be non-zero")]
    #[doc = "Reports zero source generation."]
    ZeroSourceGeneration,
}

/// Source-independent record of the signal consumed by an operator.
///
/// Derivation deliberately references the upstream typed-signal identity and
/// timing rather than `FrameLineage`. Audio is projected into these generic
/// contracts exactly once at the realtime-to-async boundary.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SignalDerivation {
    pub(crate) upstream_lineage: SignalLineage,
    pub(crate) upstream_timing: SignalTiming,
    pub(crate) operator_id: OperatorId,
    pub(crate) operator_revision: u32,
    pub(crate) operator_generation: u32,
    pub(crate) connector_id: Option<ConnectorId>,
}

impl SignalDerivation {
    #[doc = "Creates a new `SignalDerivation`."]
    pub fn new(
        upstream_lineage: SignalLineage,
        upstream_timing: SignalTiming,
        operator_id: OperatorId,
        operator_revision: u32,
        operator_generation: u32,
        connector_id: Option<ConnectorId>,
    ) -> Result<Self, SignalDerivationError> {
        if operator_id.as_str().trim().is_empty() {
            return Err(SignalDerivationError::EmptyOperatorId);
        }
        if operator_revision == 0 || operator_generation == 0 {
            return Err(SignalDerivationError::ZeroOperatorVersion);
        }
        if upstream_timing
            .source_timestamp_ns
            .zip(upstream_timing.duration_ns)
            .is_some_and(|(start, duration)| start.checked_add(duration).is_none())
        {
            return Err(SignalDerivationError::InvalidTimestampRange);
        }
        Ok(Self {
            upstream_lineage,
            upstream_timing,
            operator_id,
            operator_revision,
            operator_generation,
            connector_id,
        })
    }

    #[doc = "Returns the upstream lineage held by `SignalDerivation`."]
    pub const fn upstream_lineage(&self) -> SignalLineage {
        self.upstream_lineage
    }
    #[doc = "Returns the upstream timing held by `SignalDerivation`."]
    pub const fn upstream_timing(&self) -> SignalTiming {
        self.upstream_timing
    }
    #[doc = "Returns the operator identifier held by `SignalDerivation`."]
    pub const fn operator_id(&self) -> &OperatorId {
        &self.operator_id
    }
    #[doc = "Returns the operator revision held by `SignalDerivation`."]
    pub const fn operator_revision(&self) -> u32 {
        self.operator_revision
    }
    #[doc = "Returns the operator generation held by `SignalDerivation`."]
    pub const fn operator_generation(&self) -> u32 {
        self.operator_generation
    }
    #[doc = "Returns the connector identifier held by `SignalDerivation`."]
    pub const fn connector_id(&self) -> Option<ConnectorId> {
        self.connector_id
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
#[doc = "Classifies failures reported as signal derivation error."]
pub enum SignalDerivationError {
    #[error("derived signal upstream timing is invalid")]
    #[doc = "Reports invalid timestamp range."]
    InvalidTimestampRange,
    #[error("derived signal operator id is empty")]
    #[doc = "Reports empty operator identifier."]
    EmptyOperatorId,
    #[error("derived signal operator revision and generation must be non-zero")]
    #[doc = "Reports zero operator version."]
    ZeroOperatorVersion,
}
