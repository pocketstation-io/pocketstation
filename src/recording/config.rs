//! Validated recording declaration and lineage expectations.

use crate::frame::{ClockDomainId, SessionId, SourceId, StemId};
use crate::timing::TimelineMapping;
use serde::Serialize;

use super::writer::RecorderError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecorderLineageField {
    Session,
    Source,
    Stem,
    Clock,
    SourceGeneration,
    PermissionEpoch,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StemLabel(String);

impl StemLabel {
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

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PermissionDecision {
    Allowed,
    #[allow(dead_code)]
    Denied,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PermissionScope {
    SessionCaptureGrant,
}

#[derive(Debug, Clone)]
pub struct RecorderStemConfig {
    pub session_id: SessionId,
    pub source_id: SourceId,
    pub stem_id: StemId,
    pub clock_id: ClockDomainId,
    pub source_generation: u32,
    pub permission_epoch: u64,
    pub permission_scope: PermissionScope,
    pub permission: PermissionDecision,
    pub label: StemLabel,
    pub sample_rate_hz: u32,
    pub channels: u8,
    pub timeline_mapping: TimelineMapping,
}
