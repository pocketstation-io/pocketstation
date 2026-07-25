use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{Receiver, RecvTimeoutError};
use std::sync::Arc;
use std::time::Duration;

#[derive(Clone, Debug, Default)]
pub(crate) struct OpenCancellation {
    cancelled: Arc<AtomicBool>,
}

impl OpenCancellation {
    pub(crate) fn cancel(&self) {
        self.cancelled.store(true, Ordering::Release);
    }

    pub(crate) fn is_cancelled(&self) -> bool {
        self.cancelled.load(Ordering::Acquire)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum OpenWaitOutcome<E> {
    Opened,
    Failed(E),
    TimedOut,
    WorkerExited,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum OpenReportError {
    Cancelled,
    ReceiverUnavailable,
}

pub(crate) fn wait_for_open<E>(
    open_rx: &Receiver<Result<(), E>>,
    timeout_duration: Duration,
) -> OpenWaitOutcome<E> {
    match open_rx.recv_timeout(timeout_duration) {
        Ok(Ok(())) => OpenWaitOutcome::Opened,
        Ok(Err(error)) => OpenWaitOutcome::Failed(error),
        Err(RecvTimeoutError::Timeout) => OpenWaitOutcome::TimedOut,
        Err(RecvTimeoutError::Disconnected) => OpenWaitOutcome::WorkerExited,
    }
}

pub(crate) fn report_open<E>(
    open_tx: &std::sync::mpsc::SyncSender<Result<(), E>>,
    open_cancellation: &OpenCancellation,
) -> Result<(), OpenReportError> {
    if open_cancellation.is_cancelled() {
        return Err(OpenReportError::Cancelled);
    }
    open_tx
        .try_send(Ok(()))
        .map_err(|_| OpenReportError::ReceiverUnavailable)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_hung_open_worker_when_waiting_then_timeout_is_returned() {
        let (_open_tx, open_rx) = std::sync::mpsc::sync_channel::<Result<(), String>>(1);

        assert_eq!(
            wait_for_open(&open_rx, Duration::from_millis(1)),
            OpenWaitOutcome::TimedOut
        );
    }

    #[test]
    fn given_timeout_when_cancellation_cloned_then_late_worker_observes_it() {
        let caller_cancellation = OpenCancellation::default();
        let worker_cancellation = caller_cancellation.clone();

        caller_cancellation.cancel();

        assert!(worker_cancellation.is_cancelled());
    }

    #[test]
    fn given_cancelled_open_when_late_worker_reports_then_success_is_rejected() {
        let (open_tx, _open_rx) = std::sync::mpsc::sync_channel::<Result<(), String>>(1);
        let open_cancellation = OpenCancellation::default();
        open_cancellation.cancel();

        assert_eq!(
            report_open(&open_tx, &open_cancellation),
            Err(OpenReportError::Cancelled)
        );
    }

    #[test]
    fn given_timed_out_receiver_when_late_worker_reports_then_success_is_rejected() {
        let (open_tx, open_rx) = std::sync::mpsc::sync_channel::<Result<(), String>>(1);
        let open_cancellation = OpenCancellation::default();
        drop(open_rx);

        assert_eq!(
            report_open(&open_tx, &open_cancellation),
            Err(OpenReportError::ReceiverUnavailable)
        );
    }

    #[test]
    fn given_worker_error_when_waiting_then_exact_failure_is_retained() {
        let (open_tx, open_rx) = std::sync::mpsc::sync_channel(1);
        open_tx.send(Err("activation failed".to_owned())).unwrap();

        assert_eq!(
            wait_for_open(&open_rx, Duration::from_millis(1)),
            OpenWaitOutcome::Failed("activation failed".to_owned())
        );
    }
}
