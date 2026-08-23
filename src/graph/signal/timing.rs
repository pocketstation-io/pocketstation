//! Source, observation, and Session clock coordinates for one signal.

use crate::frame::FrameLineage;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "Carries a signal timestamp, clock domain, and timing semantics without rewriting source lineage."]
pub struct SignalTiming {
    pub(crate) source_timestamp_ns: Option<u64>,
    pub(crate) observed_timestamp_ns: u64,
    pub(crate) session_timestamp_ns: Option<u64>,
    pub(crate) duration_ns: Option<u64>,
}

impl SignalTiming {
    #[doc = "Creates a new `SignalTiming` after validating its inputs."]
    pub fn try_new(
        source_timestamp_ns: Option<u64>,
        observed_timestamp_ns: u64,
        session_timestamp_ns: Option<u64>,
        duration_ns: Option<u64>,
    ) -> Result<Self, SignalTimingError> {
        if duration_ns == Some(0) {
            return Err(SignalTimingError::ZeroDuration);
        }
        if duration_ns.is_some_and(|duration| {
            source_timestamp_ns.is_some_and(|timestamp| timestamp.checked_add(duration).is_none())
                || session_timestamp_ns
                    .is_some_and(|timestamp| timestamp.checked_add(duration).is_none())
        }) {
            return Err(SignalTimingError::TimestampOverflow);
        }
        Ok(Self {
            source_timestamp_ns,
            observed_timestamp_ns,
            session_timestamp_ns,
            duration_ns,
        })
    }

    #[doc = "Creates observed signal timing for `SignalTiming`."]
    pub const fn observed(observed_timestamp_ns: u64) -> Self {
        Self {
            source_timestamp_ns: None,
            observed_timestamp_ns,
            session_timestamp_ns: None,
            duration_ns: None,
        }
    }

    #[doc = "Sets the duration nanoseconds on `SignalTiming` and returns the updated value."]
    pub fn with_duration_ns(self, duration_ns: Option<u64>) -> Result<Self, SignalTimingError> {
        Self::try_new(
            self.source_timestamp_ns,
            self.observed_timestamp_ns,
            self.session_timestamp_ns,
            duration_ns,
        )
    }

    #[doc = "Creates `SignalTiming` from frame."]
    pub const fn from_frame(lineage: FrameLineage, observed_timestamp_ns: u64) -> Self {
        Self {
            source_timestamp_ns: Some(lineage.timestamp_start_ns),
            observed_timestamp_ns,
            session_timestamp_ns: Some(lineage.timestamp_start_ns),
            duration_ns: Some(lineage.duration_ns),
        }
    }

    #[doc = "Returns the timestamp end nanoseconds held by `SignalTiming`."]
    pub fn timestamp_end_ns(self) -> Option<u64> {
        self.source_timestamp_ns
            .zip(self.duration_ns)
            .map(|(start, duration)| start.saturating_add(duration))
    }

    #[doc = "Returns the source timestamp nanoseconds held by `SignalTiming`."]
    pub const fn source_timestamp_ns(self) -> Option<u64> {
        self.source_timestamp_ns
    }

    #[doc = "Returns the observed timestamp nanoseconds held by `SignalTiming`."]
    pub const fn observed_timestamp_ns(self) -> u64 {
        self.observed_timestamp_ns
    }

    #[doc = "Returns the session timestamp nanoseconds held by `SignalTiming`."]
    pub const fn session_timestamp_ns(self) -> Option<u64> {
        self.session_timestamp_ns
    }

    #[doc = "Returns the duration nanoseconds held by `SignalTiming`."]
    pub const fn duration_ns(self) -> Option<u64> {
        self.duration_ns
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
#[doc = "Classifies failures reported as signal timing error."]
pub enum SignalTimingError {
    #[error("signal timing duration must be non-zero when present")]
    #[doc = "Reports zero duration."]
    ZeroDuration,
    #[error("signal timing range overflows u64 nanoseconds")]
    #[doc = "Reports timestamp overflow."]
    TimestampOverflow,
}
