//! Deterministic continuity validation for source-aware signals.

use crate::graph::signal::{SignalEnvelope, SignalEnvelopeError, SignalLineage, SignalTiming};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SignalContinuityObservation {
    pub discontinuity_observed: bool,
    pub source_recovered: bool,
    pub policy_changed: bool,
}

#[derive(Debug, Default)]
pub struct SignalContinuityTracker {
    previous: Option<(SignalLineage, SignalTiming)>,
}

impl SignalContinuityTracker {
    pub fn observe(
        &mut self,
        envelope: &SignalEnvelope,
    ) -> Result<SignalContinuityObservation, SignalContinuityError> {
        envelope
            .validate()
            .map_err(SignalContinuityError::InvalidEnvelope)?;
        let current = envelope
            .lineage()
            .ok_or(SignalContinuityError::MissingLineage)?;
        let current_timing = envelope.timing();
        if let Some((previous, previous_timing)) = self.previous {
            if current.session_id != previous.session_id
                || current.stream_id != previous.stream_id
                || current.source_id != previous.source_id
                || current.clock_id != previous.clock_id
            {
                return Err(SignalContinuityError::IdentityChanged);
            }
            if current.discontinuity_epoch < previous.discontinuity_epoch {
                return Err(SignalContinuityError::DiscontinuityRegressed);
            }
            if current.source_generation < previous.source_generation {
                return Err(SignalContinuityError::GenerationRegressed);
            }
            if current.policy_epoch < previous.policy_epoch {
                return Err(SignalContinuityError::PolicyRegressed);
            }
            let discontinuity_observed = current.discontinuity_epoch > previous.discontinuity_epoch;
            if !discontinuity_observed
                && current.sequence_number != previous.sequence_number.saturating_add(1)
            {
                return Err(SignalContinuityError::SequenceGapWithoutDiscontinuity);
            }
            let source_recovered = current.source_generation > previous.source_generation;
            if source_recovered && !discontinuity_observed {
                return Err(SignalContinuityError::RecoveryWithoutDiscontinuity);
            }
            if timestamp_regressed(previous_timing, current_timing) {
                return Err(SignalContinuityError::TimestampRegression);
            }
            let observation = SignalContinuityObservation {
                discontinuity_observed,
                source_recovered,
                policy_changed: current.policy_epoch > previous.policy_epoch,
            };
            self.previous = Some((current, current_timing));
            return Ok(observation);
        }
        self.previous = Some((current, current_timing));
        Ok(SignalContinuityObservation {
            discontinuity_observed: false,
            source_recovered: false,
            policy_changed: false,
        })
    }
}

fn timestamp_regressed(previous: SignalTiming, current: SignalTiming) -> bool {
    current.observed_timestamp_ns < previous.observed_timestamp_ns
        || previous
            .source_timestamp_ns
            .zip(current.source_timestamp_ns)
            .is_some_and(|(previous, current)| current < previous)
        || previous
            .session_timestamp_ns
            .zip(current.session_timestamp_ns)
            .is_some_and(|(previous, current)| current < previous)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum SignalContinuityError {
    #[error("signal envelope is invalid: {0}")]
    InvalidEnvelope(SignalEnvelopeError),
    #[error("signal continuity requires source-independent lineage")]
    MissingLineage,
    #[error("signal identity changed within one continuity tracker")]
    IdentityChanged,
    #[error("signal sequence gap occurred without a discontinuity epoch change")]
    SequenceGapWithoutDiscontinuity,
    #[error("signal timestamp regressed")]
    TimestampRegression,
    #[error("signal discontinuity epoch regressed")]
    DiscontinuityRegressed,
    #[error("signal source generation regressed")]
    GenerationRegressed,
    #[error("signal source recovery occurred without a discontinuity")]
    RecoveryWithoutDiscontinuity,
    #[error("signal policy epoch regressed")]
    PolicyRegressed,
}
