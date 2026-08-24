use crate::recording::{RecorderError, RecordingOutcome, RecordingState};

/// Stable language-neutral code for a recording failure.
///
/// The string returned by [`Self::as_str`] is the compatibility contract.
/// Rust variant names and discriminants remain implementation details.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecordingErrorCode {
    #[doc = "Reports that output already exists and would be overwritten."]
    OutputExists,
    #[doc = "Reports that the supplied stem label is invalid."]
    InvalidStemLabel,
    #[doc = "Reports that stem label duplicates an existing declaration or record."]
    DuplicateStemLabel,
    #[doc = "Reports that session does not match the expected contract."]
    SessionMismatch,
    #[doc = "Reports that the required permission was denied."]
    PermissionDenied,
    #[doc = "Reports that the supplied sample spec is invalid."]
    InvalidSampleSpec,
    #[doc = "Reports that source does not match the expected contract."]
    SourceMismatch,
    #[doc = "Reports that lineage does not match the expected contract."]
    LineageMismatch,
    #[doc = "Reports that frame spec does not match the expected contract."]
    FrameSpecMismatch,
    #[doc = "Reports that samples does not align to complete frames or channels."]
    UnalignedSamples,
    #[doc = "Reports that timestamp falls outside the supported range."]
    TimestampOutOfRange,
    #[doc = "Reports that gap exceeds the supported size limit."]
    GapTooLarge,
    #[doc = "Reports that the number of gaps exceeds the supported limit."]
    TooManyGaps,
    #[doc = "Reports that worker panicked while the operation was active."]
    WorkerPanicked,
    #[doc = "Reports that I/O failed."]
    IoFailed,
    #[doc = "Reports that wav failed."]
    WavFailed,
    #[doc = "Reports that json failed."]
    JsonFailed,
    #[doc = "Reports that no t finalized is available."]
    NotFinalized,
    #[doc = "Reports that the operation ended without producing a complete terminal result."]
    Incomplete,
}

impl RecordingErrorCode {
    #[doc = "Returns the stable string representation of `RecordingErrorCode`."]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OutputExists => "recording.output_exists",
            Self::InvalidStemLabel => "recording.invalid_stem_label",
            Self::DuplicateStemLabel => "recording.duplicate_stem_label",
            Self::SessionMismatch => "recording.session_mismatch",
            Self::PermissionDenied => "recording.permission_denied",
            Self::InvalidSampleSpec => "recording.invalid_sample_spec",
            Self::SourceMismatch => "recording.source_mismatch",
            Self::LineageMismatch => "recording.lineage_mismatch",
            Self::FrameSpecMismatch => "recording.frame_spec_mismatch",
            Self::UnalignedSamples => "recording.unaligned_samples",
            Self::TimestampOutOfRange => "recording.timestamp_out_of_range",
            Self::GapTooLarge => "recording.gap_too_large",
            Self::TooManyGaps => "recording.too_many_gaps",
            Self::WorkerPanicked => "recording.worker_panicked",
            Self::IoFailed => "recording.io_failed",
            Self::WavFailed => "recording.wav_failed",
            Self::JsonFailed => "recording.json_failed",
            Self::NotFinalized => "recording.not_finalized",
            Self::Incomplete => "recording.incomplete",
        }
    }
}

impl RecorderError {
    #[cfg(any(test, feature = "internal-testing"))]
    #[doc = "Returns the stable error or status code represented by `RecorderError`."]
    pub const fn code(&self) -> RecordingErrorCode {
        match self {
            Self::OutputExists(_) => RecordingErrorCode::OutputExists,
            Self::InvalidStemLabel(_) => RecordingErrorCode::InvalidStemLabel,
            Self::DuplicateStemLabel(_) => RecordingErrorCode::DuplicateStemLabel,
            Self::SessionMismatch { .. } => RecordingErrorCode::SessionMismatch,
            Self::PermissionDenied(_) => RecordingErrorCode::PermissionDenied,
            Self::InvalidSampleSpec { .. } => RecordingErrorCode::InvalidSampleSpec,
            Self::SourceMismatch { .. } => RecordingErrorCode::SourceMismatch,
            Self::LineageMismatch { .. } => RecordingErrorCode::LineageMismatch,
            Self::FrameSpecMismatch { .. } => RecordingErrorCode::FrameSpecMismatch,
            Self::UnalignedSamples(_) => RecordingErrorCode::UnalignedSamples,
            Self::TimestampOutOfRange(_) => RecordingErrorCode::TimestampOutOfRange,
            Self::GapTooLarge { .. } => RecordingErrorCode::GapTooLarge,
            Self::TooManyGaps(_) => RecordingErrorCode::TooManyGaps,
            Self::WorkerPanicked(_) => RecordingErrorCode::WorkerPanicked,
            Self::Io(_) => RecordingErrorCode::IoFailed,
            Self::Wav(_) => RecordingErrorCode::WavFailed,
            Self::Json(_) => RecordingErrorCode::JsonFailed,
        }
    }
}

#[doc = "Returns the recording outcome error code held by `error_code`."]
pub const fn recording_outcome_error_code(
    outcome: &RecordingOutcome,
) -> Option<RecordingErrorCode> {
    match outcome.state {
        RecordingState::Complete => None,
        RecordingState::Recording => Some(RecordingErrorCode::NotFinalized),
        RecordingState::Incomplete => Some(RecordingErrorCode::Incomplete),
    }
}

#[cfg(test)]
mod tests {
    use super::{recording_outcome_error_code, RecordingErrorCode};
    use crate::recording::{RecordingOutcome, RecordingState};

    #[test]
    fn given_recording_codes_when_serialized_then_values_are_exact_and_unique() {
        let expected = [
            (RecordingErrorCode::OutputExists, "recording.output_exists"),
            (
                RecordingErrorCode::InvalidStemLabel,
                "recording.invalid_stem_label",
            ),
            (
                RecordingErrorCode::DuplicateStemLabel,
                "recording.duplicate_stem_label",
            ),
            (
                RecordingErrorCode::SessionMismatch,
                "recording.session_mismatch",
            ),
            (
                RecordingErrorCode::PermissionDenied,
                "recording.permission_denied",
            ),
            (
                RecordingErrorCode::InvalidSampleSpec,
                "recording.invalid_sample_spec",
            ),
            (
                RecordingErrorCode::SourceMismatch,
                "recording.source_mismatch",
            ),
            (
                RecordingErrorCode::LineageMismatch,
                "recording.lineage_mismatch",
            ),
            (
                RecordingErrorCode::FrameSpecMismatch,
                "recording.frame_spec_mismatch",
            ),
            (
                RecordingErrorCode::UnalignedSamples,
                "recording.unaligned_samples",
            ),
            (
                RecordingErrorCode::TimestampOutOfRange,
                "recording.timestamp_out_of_range",
            ),
            (RecordingErrorCode::GapTooLarge, "recording.gap_too_large"),
            (RecordingErrorCode::TooManyGaps, "recording.too_many_gaps"),
            (
                RecordingErrorCode::WorkerPanicked,
                "recording.worker_panicked",
            ),
            (RecordingErrorCode::IoFailed, "recording.io_failed"),
            (RecordingErrorCode::WavFailed, "recording.wav_failed"),
            (RecordingErrorCode::JsonFailed, "recording.json_failed"),
            (RecordingErrorCode::NotFinalized, "recording.not_finalized"),
            (RecordingErrorCode::Incomplete, "recording.incomplete"),
        ];
        let mut unique = std::collections::HashSet::new();
        for (code, value) in expected {
            assert_eq!(code.as_str(), value);
            assert!(unique.insert(value));
        }
    }

    #[test]
    fn given_terminal_failure_when_projected_then_code_is_typed() {
        let incomplete = RecordingOutcome {
            session_dir: std::path::PathBuf::from("session-1"),
            state: RecordingState::Incomplete,
            completed_stems: 0,
            failed_stems: 1,
            stems: Vec::new(),
        };
        assert_eq!(
            recording_outcome_error_code(&incomplete),
            Some(RecordingErrorCode::Incomplete)
        );
    }
}
