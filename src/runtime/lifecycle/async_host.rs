use std::future::Future;
use std::sync::mpsc;
use std::thread::{self, JoinHandle};
use std::time::Duration;

use tokio::runtime::{Builder, Handle};
use tokio::sync::oneshot;

#[derive(Debug, thiserror::Error)]
#[doc = "Classifies failures reported as async runtime host error."]
pub enum AsyncRuntimeHostError {
    #[error("failed to start owned async runtime: {0}")]
    #[doc = "Reported when the owning operation encounters start."]
    Start(String),
    #[error("owned async runtime stopped before returning a lifecycle result")]
    #[doc = "Reported when the owning operation encounters runtime stopped."]
    RuntimeStopped,
    #[error("owned async runtime thread panicked during shutdown")]
    #[doc = "Reported when the owning operation encounters shutdown panicked."]
    ShutdownPanicked,
    #[error("owned async runtime did not return within {timeout_ms} ms")]
    #[doc = "Reported when the owning operation encounters host timeout."]
    HostTimeout {
        #[doc = "Stores the timeout value for `HostTimeout`, in milliseconds."]
        timeout_ms: u64,
    },
}

/// Session-owned async executor for connector and derived-endpoint lifecycle.
///
/// Synchronous Session callers submit bounded lifecycle futures to this
/// dedicated runtime thread. This never calls `Handle::block_on`, so it remains
/// valid when the caller already runs inside a Tokio runtime.
pub struct AsyncRuntimeHost {
    handle: Handle,
    shutdown: Option<oneshot::Sender<()>>,
    thread: Option<JoinHandle<()>>,
}

impl AsyncRuntimeHost {
    #[doc = "Creates a new `AsyncRuntimeHost`."]
    pub fn new(thread_name: impl Into<String>) -> Result<Self, AsyncRuntimeHostError> {
        let (handle_sender, handle_receiver) = mpsc::sync_channel(1);
        let (shutdown_sender, shutdown_receiver) = oneshot::channel();
        let thread = thread::Builder::new()
            .name(thread_name.into())
            .spawn(move || {
                let runtime = match Builder::new_current_thread().enable_all().build() {
                    Ok(runtime) => runtime,
                    Err(error) => {
                        let _ = handle_sender.send(Err(error.to_string()));
                        return;
                    }
                };
                if handle_sender.send(Ok(runtime.handle().clone())).is_err() {
                    return;
                }
                runtime.block_on(async {
                    let _ = shutdown_receiver.await;
                });
            })
            .map_err(|error| AsyncRuntimeHostError::Start(error.to_string()))?;
        let handle = handle_receiver
            .recv()
            .map_err(|_| AsyncRuntimeHostError::RuntimeStopped)?
            .map_err(AsyncRuntimeHostError::Start)?;
        Ok(Self {
            handle,
            shutdown: Some(shutdown_sender),
            thread: Some(thread),
        })
    }

    #[doc = "Executes its owned operation for `AsyncRuntimeHost`."]
    pub fn execute<F>(
        &self,
        timeout: Duration,
        future: F,
    ) -> Result<F::Output, AsyncRuntimeHostError>
    where
        F: Future + Send + 'static,
        F::Output: Send + 'static,
    {
        let (result_sender, result_receiver) = mpsc::sync_channel(1);
        self.handle.spawn(async move {
            let output = tokio::time::timeout(timeout, future).await;
            let _ = result_sender.send(output);
        });
        let host_guard_timeout = timeout.saturating_add(Duration::from_millis(100));
        match result_receiver.recv_timeout(host_guard_timeout) {
            Ok(Ok(output)) => Ok(output),
            Ok(Err(_)) | Err(mpsc::RecvTimeoutError::Timeout) => {
                Err(AsyncRuntimeHostError::HostTimeout {
                    timeout_ms: timeout.as_millis().try_into().unwrap_or(u64::MAX),
                })
            }
            Err(mpsc::RecvTimeoutError::Disconnected) => Err(AsyncRuntimeHostError::RuntimeStopped),
        }
    }

    #[doc = "Shuts down `AsyncRuntimeHost` according to its lifecycle contract."]
    pub fn shutdown(mut self) -> Result<(), AsyncRuntimeHostError> {
        if let Some(shutdown) = self.shutdown.take() {
            let _ = shutdown.send(());
        }
        match self.thread.take() {
            Some(thread) => thread
                .join()
                .map_err(|_| AsyncRuntimeHostError::ShutdownPanicked),
            None => Ok(()),
        }
    }
}

impl Drop for AsyncRuntimeHost {
    #[doc = "Releases resources owned by `AsyncRuntimeHost`."]
    fn drop(&mut self) {
        if let Some(shutdown) = self.shutdown.take() {
            let _ = shutdown.send(());
        }
        if let Some(thread) = self.thread.take() {
            let _ = thread.join();
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::Arc;

    use super::*;

    #[test]
    fn given_sync_caller_when_future_executes_then_result_returns_from_owned_runtime() {
        let host = AsyncRuntimeHost::new("pocketstation-runtime-host-sync").unwrap();
        let result = host
            .execute(Duration::from_secs(1), async { 42_u32 })
            .unwrap();
        assert_eq!(result, 42);
    }

    #[tokio::test(flavor = "current_thread")]
    async fn given_tokio_caller_when_sync_lifecycle_executes_then_no_nested_runtime_panics() {
        let host = AsyncRuntimeHost::new("pocketstation-runtime-host-async").unwrap();
        let result = host
            .execute(Duration::from_secs(1), async {
                tokio::time::sleep(std::time::Duration::from_millis(1)).await;
                7_u32
            })
            .unwrap();
        assert_eq!(result, 7);
    }

    #[test]
    fn given_slow_future_when_deadline_expires_then_host_returns_typed_timeout() {
        let host = AsyncRuntimeHost::new("pocketstation-runtime-host-timeout").unwrap();
        let mutated = Arc::new(AtomicBool::new(false));
        let task_mutated = Arc::clone(&mutated);
        let error = host
            .execute(Duration::from_millis(1), async move {
                tokio::time::sleep(Duration::from_secs(1)).await;
                task_mutated.store(true, Ordering::Release);
            })
            .unwrap_err();
        assert!(matches!(
            error,
            AsyncRuntimeHostError::HostTimeout { timeout_ms: 1 }
        ));
        std::thread::sleep(Duration::from_millis(10));
        assert!(!mutated.load(Ordering::Acquire));
    }
}
