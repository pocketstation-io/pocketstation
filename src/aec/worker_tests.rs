use super::*;
use crate::EchoCancellationState;
use std::time::Duration;

#[tokio::test]
async fn given_waiting_native_reply_when_future_is_cancelled_then_interruption_is_observable() {
    let state = ObservationState::new(4);
    let result = tokio::time::timeout(Duration::from_millis(1), async {
        let _response = PendingResponse::new(state.clone());
        std::future::pending::<()>().await;
    })
    .await;
    assert!(result.is_err());
    let observed = state.snapshot();
    assert_eq!(observed.state, EchoCancellationState::Interrupted);
    assert!(observed.last_error.is_none());
    assert_eq!(observed.interrupted_requests_total, 1);
}

#[test]
fn given_completed_reply_when_await_guard_drops_then_it_does_not_report_cancellation() {
    let state = ObservationState::new(4);
    let mut response = PendingResponse::new(state.clone());
    response.complete(&Ok(()));
    drop(response);
    assert_eq!(
        state.snapshot().state,
        EchoCancellationState::WaitingForReference
    );
    assert!(state.snapshot().last_error.is_none());
}
