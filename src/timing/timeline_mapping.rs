#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "Represents timeline mapping in the PocketStation API."]
pub struct TimelineMapping {
    #[doc = "Stores the source origin value for `TimelineMapping`, in nanoseconds."]
    pub source_origin_ns: u64,
    #[doc = "Stores the session origin value for `TimelineMapping`, in nanoseconds."]
    pub session_origin_ns: u64,
}

impl TimelineMapping {
    #[doc = "Creates a new `TimelineMapping`."]
    pub const fn new(source_origin_ns: u64, session_origin_ns: u64) -> Self {
        Self {
            source_origin_ns,
            session_origin_ns,
        }
    }

    #[doc = "Returns the normalize timestamp nanoseconds associated with `TimelineMapping`."]
    pub fn normalize_timestamp_ns(self, source_timestamp_ns: u64) -> Option<u64> {
        if source_timestamp_ns >= self.source_origin_ns {
            self.session_origin_ns
                .checked_add(source_timestamp_ns - self.source_origin_ns)
        } else {
            self.session_origin_ns
                .checked_sub(self.source_origin_ns - source_timestamp_ns)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_later_source_timestamp_when_normalized_then_session_delta_is_preserved() {
        let mapping = TimelineMapping::new(1_000, 4_000);

        assert_eq!(mapping.normalize_timestamp_ns(1_250), Some(4_250));
    }

    #[test]
    fn given_earlier_source_timestamp_when_normalized_then_session_delta_is_preserved() {
        let mapping = TimelineMapping::new(1_000, 4_000);

        assert_eq!(mapping.normalize_timestamp_ns(750), Some(3_750));
    }

    #[test]
    fn given_unrepresentable_timestamp_when_normalized_then_none_is_returned() {
        let mapping = TimelineMapping::new(10, 5);

        assert_eq!(mapping.normalize_timestamp_ns(0), None);
    }
}
