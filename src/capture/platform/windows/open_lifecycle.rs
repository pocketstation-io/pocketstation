use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{Receiver, RecvTimeoutError};
use std::sync::Arc;
use std::time::{Duration, Instant};

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
pub(crate) enum OpenWaitOutcome<T, E> {
    Opened(T),
    Failed(E),
    TimedOut,
    WorkerExited,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum OpenReportError {
    Cancelled,
    ReceiverUnavailable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CancellableWaitOutcome {
    Completed,
    Cancelled,
    TimedOut,
    ProducerExited,
}

pub(crate) fn wait_for_completion(
    completion_rx: &Receiver<()>,
    cancellation: &OpenCancellation,
    timeout_duration: Duration,
    cancellation_poll_duration: Duration,
) -> CancellableWaitOutcome {
    let deadline = Instant::now() + timeout_duration;
    loop {
        if cancellation.is_cancelled() {
            return CancellableWaitOutcome::Cancelled;
        }
        let now = Instant::now();
        if now >= deadline {
            return CancellableWaitOutcome::TimedOut;
        }
        let remaining = deadline.saturating_duration_since(now);
        let wait_duration = remaining.min(cancellation_poll_duration);
        match completion_rx.recv_timeout(wait_duration) {
            Ok(()) => return CancellableWaitOutcome::Completed,
            Err(RecvTimeoutError::Disconnected) => {
                return CancellableWaitOutcome::ProducerExited;
            }
            Err(RecvTimeoutError::Timeout) => {}
        }
    }
}

pub(crate) fn wait_for_open<T, E>(
    open_rx: &Receiver<Result<T, E>>,
    timeout_duration: Duration,
) -> OpenWaitOutcome<T, E> {
    match open_rx.recv_timeout(timeout_duration) {
        Ok(Ok(value)) => OpenWaitOutcome::Opened(value),
        Ok(Err(error)) => OpenWaitOutcome::Failed(error),
        Err(RecvTimeoutError::Timeout) => OpenWaitOutcome::TimedOut,
        Err(RecvTimeoutError::Disconnected) => OpenWaitOutcome::WorkerExited,
    }
}

pub(crate) fn report_open<T, E>(
    open_tx: &std::sync::mpsc::SyncSender<Result<T, E>>,
    value: T,
    open_cancellation: &OpenCancellation,
) -> Result<(), OpenReportError> {
    if open_cancellation.is_cancelled() {
        return Err(OpenReportError::Cancelled);
    }
    open_tx
        .try_send(Ok(value))
        .map_err(|_| OpenReportError::ReceiverUnavailable)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_hung_open_worker_when_waiting_then_timeout_is_returned() {
        let (_open_tx, open_rx) = std::sync::mpsc::sync_channel::<Result<u64, String>>(1);

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
        let (open_tx, _open_rx) = std::sync::mpsc::sync_channel::<Result<u64, String>>(1);
        let open_cancellation = OpenCancellation::default();
        open_cancellation.cancel();

        assert_eq!(
            report_open(&open_tx, 7, &open_cancellation),
            Err(OpenReportError::Cancelled)
        );
    }

    #[test]
    fn given_timed_out_receiver_when_late_worker_reports_then_success_is_rejected() {
        let (open_tx, open_rx) = std::sync::mpsc::sync_channel::<Result<u64, String>>(1);
        let open_cancellation = OpenCancellation::default();
        drop(open_rx);

        assert_eq!(
            report_open(&open_tx, 7, &open_cancellation),
            Err(OpenReportError::ReceiverUnavailable)
        );
    }

    #[test]
    fn given_worker_error_when_waiting_then_exact_failure_is_retained() {
        let (open_tx, open_rx) = std::sync::mpsc::sync_channel::<Result<u64, String>>(1);
        open_tx.send(Err("activation failed".to_owned())).unwrap();

        assert_eq!(
            wait_for_open(&open_rx, Duration::from_millis(1)),
            OpenWaitOutcome::Failed("activation failed".to_owned())
        );
    }

    #[test]
    fn given_cancelled_activation_when_waiting_then_cancellation_is_bounded() {
        let (_completion_tx, completion_rx) = std::sync::mpsc::sync_channel(1);
        let cancellation = OpenCancellation::default();
        cancellation.cancel();

        assert_eq!(
            wait_for_completion(
                &completion_rx,
                &cancellation,
                Duration::from_secs(5),
                Duration::from_millis(10),
            ),
            CancellableWaitOutcome::Cancelled
        );
    }

    #[test]
    fn given_completed_activation_when_waiting_then_completion_is_returned() {
        let (completion_tx, completion_rx) = std::sync::mpsc::sync_channel(1);
        let cancellation = OpenCancellation::default();
        completion_tx.try_send(()).unwrap();

        assert_eq!(
            wait_for_completion(
                &completion_rx,
                &cancellation,
                Duration::from_secs(5),
                Duration::from_millis(10),
            ),
            CancellableWaitOutcome::Completed
        );
    }

    #[test]
    fn given_hung_activation_when_deadline_expires_then_timeout_is_returned() {
        let (_completion_tx, completion_rx) = std::sync::mpsc::sync_channel(1);
        let cancellation = OpenCancellation::default();

        assert_eq!(
            wait_for_completion(
                &completion_rx,
                &cancellation,
                Duration::from_millis(1),
                Duration::from_millis(1),
            ),
            CancellableWaitOutcome::TimedOut
        );
    }
}
