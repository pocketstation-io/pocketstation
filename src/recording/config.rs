//! Validated recording declaration and lineage expectations.

use crate::frame::{ClockDomainId, SessionId, SourceId, StemId};
use crate::timing::TimelineMapping;
use serde::Serialize;

use super::writer::RecorderError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "Enumerates the supported recorder lineage field cases."]
pub enum RecorderLineageField {
    #[doc = "Selects the session case of `RecorderLineageField`."]
    Session,
    #[doc = "Selects the source case of `RecorderLineageField`."]
    Source,
    #[doc = "Selects the stem case of `RecorderLineageField`."]
    Stem,
    #[doc = "Selects the clock case of `RecorderLineageField`."]
    Clock,
    #[doc = "Selects the source generation case of `RecorderLineageField`."]
    SourceGeneration,
    #[doc = "Selects the permission epoch case of `RecorderLineageField`."]
    PermissionEpoch,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[doc = "Stores the validated human-readable label used for one recording stem."]
pub struct StemLabel(String);

impl StemLabel {
    #[doc = "Creates a new `StemLabel`."]
    pub fn new(value: impl Into<String>) -> Result<Self, RecorderError> {
        let value = value.into();
        let valid = !value.is_empty()
            && value.len() <= 64
            && value.bytes().all(|byte| {
                byte.is_ascii_lowercase() || byte.is_ascii_digit() || b"-_".contains(&byte)
            });
        if !valid {
            return Err(RecorderError::InvalidStemLabel(value));
        }
        Ok(Self(value))
    }

    #[doc = "Returns the stable string representation of `StemLabel`."]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
#[doc = "Enumerates the supported permission decision cases."]
pub enum PermissionDecision {
    #[doc = "Selects the allowed case of `PermissionDecision`."]
    Allowed,
    #[doc = "Selects the denied case of `PermissionDecision`."]
    Denied,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
#[doc = "Selects the permission scope used by PocketStation."]
pub enum PermissionScope {
    #[doc = "Selects session capture grant behavior for `PermissionScope`."]
    SessionCaptureGrant,
}

#[derive(Debug, Clone)]
#[doc = "Configures recorder stem behavior at its owning API boundary."]
pub struct RecorderStemConfig {
    #[doc = "Identifies the session identifier recorded by `RecorderStemConfig`."]
    pub session_id: SessionId,
    #[doc = "Identifies the source identifier recorded by `RecorderStemConfig`."]
    pub source_id: SourceId,
    #[doc = "Identifies the stem identifier recorded by `RecorderStemConfig`."]
    pub stem_id: StemId,
    #[doc = "Identifies the clock identifier recorded by `RecorderStemConfig`."]
    pub clock_id: ClockDomainId,
    #[doc = "Stores the source generation used by `RecorderStemConfig`."]
    pub source_generation: u32,
    #[doc = "Stores the permission epoch used by `RecorderStemConfig`."]
    pub permission_epoch: u64,
    #[doc = "Stores the permission scope used by `RecorderStemConfig`."]
    pub permission_scope: PermissionScope,
    #[doc = "Stores the permission used by `RecorderStemConfig`."]
    pub permission: PermissionDecision,
    #[doc = "Stores the label used by `RecorderStemConfig`."]
    pub label: StemLabel,
    #[doc = "Stores the sample rate value for `RecorderStemConfig`, in hertz."]
    pub sample_rate_hz: u32,
    #[doc = "Stores the channels used by `RecorderStemConfig`."]
    pub channels: u8,
    #[doc = "Stores the timeline mapping used by `RecorderStemConfig`."]
    pub timeline_mapping: TimelineMapping,
}
