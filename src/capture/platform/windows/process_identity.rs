const STABLE_KEY_PREFIX: &str = "wasapi:pid:";
const CREATION_TIME_SEPARATOR: &str = ":creation-100ns:";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ProcessInstanceFingerprint {
    pub(crate) process_id: u32,
    pub(crate) creation_time_100ns: u64,
}

impl ProcessInstanceFingerprint {
    pub(crate) fn new(process_id: u32, creation_time_100ns: u64) -> Self {
        Self {
            process_id,
            creation_time_100ns,
        }
    }

    pub(crate) fn stable_key(self) -> String {
        format!(
            "{STABLE_KEY_PREFIX}{}{CREATION_TIME_SEPARATOR}{}",
            self.process_id, self.creation_time_100ns
        )
    }

    pub(crate) fn parse(stable_key: &str) -> Option<Self> {
        let body = stable_key.strip_prefix(STABLE_KEY_PREFIX)?;
        let (process_id, creation_time_100ns) = body.split_once(CREATION_TIME_SEPARATOR)?;
        Some(Self {
            process_id: process_id.parse().ok()?,
            creation_time_100ns: creation_time_100ns.parse().ok()?,
        })
    }

    pub(crate) fn matches(self, process_id: u32, creation_time_100ns: u64) -> bool {
        self.process_id == process_id && self.creation_time_100ns == creation_time_100ns
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_process_instance_when_stable_key_round_trips_then_pid_and_creation_time_are_retained()
    {
        let fingerprint = ProcessInstanceFingerprint::new(4_242, 133_980_144_000_000_000);

        assert_eq!(
            ProcessInstanceFingerprint::parse(&fingerprint.stable_key()),
            Some(fingerprint)
        );
    }

    #[test]
    fn given_legacy_pid_key_when_parsed_then_process_instance_is_rejected() {
        assert_eq!(ProcessInstanceFingerprint::parse("wasapi:pid:4242"), None);
    }

    #[test]
    fn given_reused_pid_when_creation_time_changes_then_process_instance_does_not_match() {
        let fingerprint = ProcessInstanceFingerprint::new(4_242, 100);

        assert!(!fingerprint.matches(4_242, 101));
    }

    #[test]
    fn given_different_pid_when_creation_time_matches_then_process_instance_does_not_match() {
        let fingerprint = ProcessInstanceFingerprint::new(4_242, 100);

        assert!(!fingerprint.matches(4_243, 100));
    }
}
