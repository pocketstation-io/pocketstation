//! Async operator lifecycle and throughput observations.

use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;

use tokio::sync::Notify;

#[derive(Default)]
pub(super) struct AsyncOperatorObservationState {
    pub(super) input_attempted_total: AtomicU64,
    pub(super) input_dropped_total: AtomicU64,
    pub(super) processed_total: AtomicU64,
    pub(super) output_emitted_total: AtomicU64,
    pub(super) output_dropped_total: AtomicU64,
    pub(super) output_nonterminal_total: AtomicU64,
    pub(super) output_terminal_total: AtomicU64,
    pub(super) process_failure_total: AtomicU64,
    pub(super) timeout_total: AtomicU64,
    pub(super) cancellation_total: AtomicU64,
    pub(super) graceful_finish_total: AtomicU64,
    pub(super) idle_poll_total: AtomicU64,
    pub(super) ready: AtomicBool,
    pub(super) joined: AtomicBool,
    pub(super) ready_notify: Notify,
    pub(super) terminal_notify: Notify,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AsyncOperatorObservations {
    pub input_attempted_total: u64,
    pub input_dropped_total: u64,
    pub processed_total: u64,
    pub output_emitted_total: u64,
    pub output_dropped_total: u64,
    pub output_nonterminal_total: u64,
    pub output_terminal_total: u64,
    pub process_failure_total: u64,
    pub timeout_total: u64,
    pub cancellation_total: u64,
    pub graceful_finish_total: u64,
    pub idle_poll_total: u64,
    pub ready: bool,
    pub joined: bool,
}

#[derive(Clone)]
pub struct AsyncOperatorObservationHandle {
    pub(super) state: Arc<AsyncOperatorObservationState>,
}

impl AsyncOperatorObservationHandle {
    pub fn snapshot(&self) -> AsyncOperatorObservations {
        AsyncOperatorObservations {
            input_attempted_total: self.state.input_attempted_total.load(Ordering::Relaxed),
            input_dropped_total: self.state.input_dropped_total.load(Ordering::Relaxed),
            processed_total: self.state.processed_total.load(Ordering::Relaxed),
            output_emitted_total: self.state.output_emitted_total.load(Ordering::Relaxed),
            output_dropped_total: self.state.output_dropped_total.load(Ordering::Relaxed),
            output_nonterminal_total: self.state.output_nonterminal_total.load(Ordering::Relaxed),
            output_terminal_total: self.state.output_terminal_total.load(Ordering::Relaxed),
            process_failure_total: self.state.process_failure_total.load(Ordering::Relaxed),
            timeout_total: self.state.timeout_total.load(Ordering::Relaxed),
            cancellation_total: self.state.cancellation_total.load(Ordering::Relaxed),
            graceful_finish_total: self.state.graceful_finish_total.load(Ordering::Relaxed),
            idle_poll_total: self.state.idle_poll_total.load(Ordering::Relaxed),
            ready: self.state.ready.load(Ordering::Acquire),
            joined: self.state.joined.load(Ordering::Acquire),
        }
    }

    #[cfg(any(test, feature = "internal-testing"))]
    pub async fn wait_ready(&self) -> bool {
        loop {
            let notified = self.state.ready_notify.notified();
            if self.state.ready.load(Ordering::Acquire) {
                return true;
            }
            if self.state.joined.load(Ordering::Acquire) {
                return false;
            }
            notified.await;
        }
    }

    #[cfg(any(test, feature = "internal-testing"))]
    pub async fn wait_terminal(&self) {
        loop {
            let notified = self.state.terminal_notify.notified();
            if self.state.joined.load(Ordering::Acquire) {
                return;
            }
            notified.await;
        }
    }
}
