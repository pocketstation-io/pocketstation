//! Bounded ownership for application-generated output.

use std::fmt;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

const TERMINAL_GENERATION: u64 = u64::MAX;
const MAX_OUTPUT_GENERATION: u64 = TERMINAL_GENERATION - 1;

/// Core assigns no user, turn, model, or conversation meaning to this ID.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OutputGenerationId(u64);

impl OutputGenerationId {
    pub const fn get(self) -> u64 {
        self.0
    }
}

pub(crate) struct OutputGenerationState {
    active_id: AtomicU64,
}

impl OutputGenerationState {
    pub(crate) fn new() -> Arc<Self> {
        Arc::new(Self {
            active_id: AtomicU64::new(0),
        })
    }

    pub(crate) fn begin(self: &Arc<Self>) -> Result<OutputGeneration, OutputGenerationError> {
        let previous = self
            .active_id
            .fetch_update(Ordering::AcqRel, Ordering::Acquire, |current| {
                (current < MAX_OUTPUT_GENERATION).then_some(current + 1)
            })
            .map_err(|_| OutputGenerationError::LimitReached)?;
        Ok(OutputGeneration {
            id: OutputGenerationId(previous + 1),
            state: Arc::clone(self),
        })
    }

    pub(crate) fn owns(self: &Arc<Self>, generation: &OutputGeneration) -> bool {
        Arc::ptr_eq(self, &generation.state)
    }
}

/// Keeps replaceable output attached to the operation that produced it.
#[derive(Clone)]
pub struct OutputGeneration {
    id: OutputGenerationId,
    state: Arc<OutputGenerationState>,
}

impl OutputGeneration {
    pub const fn id(&self) -> OutputGenerationId {
        self.id
    }

    pub fn is_active(&self) -> bool {
        self.state.active_id.load(Ordering::Acquire) == self.id.0
    }

    pub(crate) fn should_discard(&self) -> bool {
        self.state.active_id.load(Ordering::Acquire) > self.id.0
    }

    /// Cancels pending output without stopping its Source or Session.
    pub fn cancel(&self) -> OutputCancelResult {
        let inactive_id = self.id.0.saturating_add(1);
        match self.state.active_id.compare_exchange(
            self.id.0,
            inactive_id,
            Ordering::AcqRel,
            Ordering::Acquire,
        ) {
            Ok(_) => OutputCancelResult::Cancelled,
            Err(_) => OutputCancelResult::AlreadyInactive,
        }
    }
}

impl fmt::Debug for OutputGeneration {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("OutputGeneration")
            .field("id", &self.id)
            .field("is_active", &self.is_active())
            .finish()
    }
}

impl PartialEq for OutputGeneration {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && Arc::ptr_eq(&self.state, &other.state)
    }
}

impl Eq for OutputGeneration {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum OutputGenerationError {
    #[error("output generation limit reached")]
    LimitReached,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputCancelResult {
    Cancelled,
    AlreadyInactive,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_active_output_when_new_generation_begins_then_previous_output_is_inactive() {
        let state = OutputGenerationState::new();
        let first = state.begin().unwrap();
        let second = state.begin().unwrap();

        assert!(!first.is_active());
        assert!(second.is_active());
        assert_eq!(first.id().get(), 1);
        assert_eq!(second.id().get(), 2);
    }

    #[test]
    fn given_active_output_when_cancel_repeats_then_only_first_call_is_applied() {
        let state = OutputGenerationState::new();
        let generation = state.begin().unwrap();

        assert_eq!(generation.cancel(), OutputCancelResult::Cancelled);
        assert_eq!(generation.cancel(), OutputCancelResult::AlreadyInactive);
        assert!(!generation.is_active());
    }

    #[test]
    fn given_last_output_generation_when_cancelled_then_state_becomes_terminal() {
        let state = Arc::new(OutputGenerationState {
            active_id: AtomicU64::new(MAX_OUTPUT_GENERATION),
        });
        let generation = OutputGeneration {
            id: OutputGenerationId(MAX_OUTPUT_GENERATION),
            state: Arc::clone(&state),
        };

        assert_eq!(generation.cancel(), OutputCancelResult::Cancelled);
        assert_eq!(state.active_id.load(Ordering::Acquire), TERMINAL_GENERATION);
        assert_eq!(state.begin(), Err(OutputGenerationError::LimitReached));
    }
}
