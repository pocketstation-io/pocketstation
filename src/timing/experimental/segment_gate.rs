pub type SegmentId = u64;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SegmentGateState {
    Idle,
    Active {
        segment_id: SegmentId,
    },
    Interrupted {
        segment_id: SegmentId,
        played_ms: u64,
    },
    Truncated {
        segment_id: SegmentId,
        audio_end_ms: u64,
    },
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SegmentGateTrace {
    pub interruption_count: u64,
    pub truncation_count: u64,
    pub clear_count: u64,
    pub last_played_ms: u64,
    pub flush_events_count: u64,
}

/// Preserved voice-output interruption state machine.
///
/// This remains experimental until a generated-audio endpoint consumes it.
pub struct SegmentGate {
    state: SegmentGateState,
    trace: SegmentGateTrace,
}

impl SegmentGate {
    pub fn new() -> Self {
        Self {
            state: SegmentGateState::Idle,
            trace: SegmentGateTrace::default(),
        }
    }

    pub fn begin(&mut self, segment_id: SegmentId) {
        self.state = SegmentGateState::Active { segment_id };
    }

    pub fn interrupt(&mut self, segment_id: SegmentId, played_ms: u64) {
        self.state = SegmentGateState::Interrupted {
            segment_id,
            played_ms,
        };
        self.trace.interruption_count = self.trace.interruption_count.saturating_add(1);
        self.trace.last_played_ms = played_ms;
        self.trace.flush_events_count = self.trace.flush_events_count.saturating_add(1);
    }

    pub fn truncate(&mut self, segment_id: SegmentId, audio_end_ms: u64) {
        self.state = SegmentGateState::Truncated {
            segment_id,
            audio_end_ms,
        };
        self.trace.truncation_count = self.trace.truncation_count.saturating_add(1);
        self.trace.flush_events_count = self.trace.flush_events_count.saturating_add(1);
    }

    pub fn clear(&mut self) -> u64 {
        let cleared_count = u64::from(!matches!(self.state, SegmentGateState::Idle));
        self.state = SegmentGateState::Idle;
        self.trace.clear_count = self.trace.clear_count.saturating_add(1);
        if cleared_count > 0 {
            self.trace.flush_events_count = self.trace.flush_events_count.saturating_add(1);
        }
        cleared_count
    }

    pub fn state(&self) -> &SegmentGateState {
        &self.state
    }

    pub fn should_flush(&self) -> bool {
        matches!(
            self.state,
            SegmentGateState::Interrupted { .. } | SegmentGateState::Truncated { .. }
        )
    }

    pub fn flush_segment_id(&self) -> Option<SegmentId> {
        match &self.state {
            SegmentGateState::Interrupted { segment_id, .. }
            | SegmentGateState::Truncated { segment_id, .. } => Some(*segment_id),
            _ => None,
        }
    }

    pub fn trace(&self) -> &SegmentGateTrace {
        &self.trace
    }
}

impl Default for SegmentGate {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_active_segment_when_interrupted_then_segment_is_flushable() {
        let mut gate = SegmentGate::new();
        gate.begin(42);
        gate.interrupt(42, 500);
        assert!(gate.should_flush());
        assert_eq!(gate.flush_segment_id(), Some(42));
    }

    #[test]
    fn given_active_segment_when_truncated_then_position_is_preserved() {
        let mut gate = SegmentGate::new();
        gate.begin(7);
        gate.truncate(7, 1_200);
        assert_eq!(
            gate.state(),
            &SegmentGateState::Truncated {
                segment_id: 7,
                audio_end_ms: 1_200,
            }
        );
    }

    #[test]
    fn given_interrupted_segment_when_cleared_then_gate_returns_to_idle() {
        let mut gate = SegmentGate::new();
        gate.begin(1);
        gate.interrupt(1, 100);
        assert_eq!(gate.clear(), 1);
        assert_eq!(gate.state(), &SegmentGateState::Idle);
        assert!(!gate.should_flush());
    }

    #[test]
    fn given_interruption_and_truncation_when_observed_then_trace_counts_events() {
        let mut gate = SegmentGate::new();
        gate.begin(1);
        gate.interrupt(1, 250);
        gate.begin(2);
        gate.truncate(2, 800);
        gate.clear();
        assert_eq!(gate.trace().interruption_count, 1);
        assert_eq!(gate.trace().truncation_count, 1);
        assert_eq!(gate.trace().last_played_ms, 250);
        assert_eq!(gate.trace().flush_events_count, 3);
    }
}
