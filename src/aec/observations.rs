use crate::SourceId;
use std::sync::{Arc, Mutex};

/// Processor lifecycle; Processing does not assert acoustic convergence.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EchoCancellationState {
    WaitingForReference,
    Processing,
    Reset,
    Interrupted,
    Failed,
    Stopped,
}

/// Local processing observations, readable after the Session stops.
#[derive(Debug, Clone)]
pub struct EchoCancellationObservations {
    pub state: EchoCancellationState,
    pub processed_microphone_frames_total: u64,
    /// Includes normal frames and the bounded graceful-finish tail.
    pub output_frames_total: u64,
    /// Produced frames discarded because the awaiting request was interrupted.
    pub discarded_output_frames_total: u64,
    pub tail_frames_total: u64,
    /// Internal EOF padding per channel; never captured-input samples.
    pub tail_padding_samples_total: u64,
    pub discarded_tail_generations_total: u64,
    /// Nominal frequency-dependent signal delay, distinct from CPU duration.
    pub nominal_delay_samples: u32,
    /// Explicit termination policy; adaptive/noise state may continue after it.
    pub drain_duration_ms: u32,
    pub discarded_microphone_frames_total: u64,
    pub discarded_reference_frames_total: u64,
    pub resets_total: u64,
    pub processing_generation: u64,
    pub microphone_queue_depth_frames: usize,
    pub reference_queue_depth_frames: usize,
    pub queue_capacity_frames: usize,
    /// Complete native command wall time, including queue wait, resets, render
    /// analysis and capture processing; not just DSP CPU time or signal delay.
    pub latest_processing_duration_ns: u64,
    /// Maximum complete native command wall time observed before shutdown.
    pub maximum_processing_duration_ns: u64,
    pub latest_reference_age_ns: u64,
    pub latest_reference_lead_ns: u64,
    pub maximum_cadence_error_ns: u64,
    pub analyzed_reference_frames_total: u64,
    pub interrupted_requests_total: u64,
    pub reference_source_id: Option<SourceId>,
    pub microphone_source_id: Option<SourceId>,
    /// Unknown until qualified for the selected processor configuration.
    pub qualified_algorithmic_delay_samples: Option<u32>,
    pub last_error: Option<String>,
}

#[cfg(feature = "aec")]
impl EchoCancellationObservations {
    pub(crate) fn new(queue_capacity_frames: usize) -> Self {
        Self {
            state: EchoCancellationState::WaitingForReference,
            processed_microphone_frames_total: 0,
            output_frames_total: 0,
            discarded_output_frames_total: 0,
            tail_frames_total: 0,
            tail_padding_samples_total: 0,
            discarded_tail_generations_total: 0,
            nominal_delay_samples: 432,
            drain_duration_ms: 40,
            discarded_microphone_frames_total: 0,
            discarded_reference_frames_total: 0,
            resets_total: 0,
            processing_generation: 1,
            microphone_queue_depth_frames: 0,
            reference_queue_depth_frames: 0,
            queue_capacity_frames,
            latest_processing_duration_ns: 0,
            maximum_processing_duration_ns: 0,
            latest_reference_age_ns: 0,
            latest_reference_lead_ns: 0,
            maximum_cadence_error_ns: 0,
            analyzed_reference_frames_total: 0,
            interrupted_requests_total: 0,
            reference_source_id: None,
            microphone_source_id: None,
            qualified_algorithmic_delay_samples: None,
            last_error: None,
        }
    }
}

/// Used only on setup, DSP and observation threads, never capture callbacks.
#[derive(Clone)]
pub(crate) struct ObservationState(Arc<Mutex<EchoCancellationObservations>>);

impl ObservationState {
    #[cfg(feature = "aec")]
    pub(crate) fn new(capacity_frames: usize) -> Self {
        Self(Arc::new(Mutex::new(EchoCancellationObservations::new(
            capacity_frames,
        ))))
    }

    #[cfg(feature = "aec")]
    pub(crate) fn update(&self, mut value: EchoCancellationObservations) {
        let mut current = self
            .0
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        // A native reply can arrive after its awaiting future was cancelled.
        // Preserve the first terminal error even if the worker later completes.
        if let Some(error) = &current.last_error {
            value.state = EchoCancellationState::Failed;
            value.last_error = Some(error.clone());
        }
        value.interrupted_requests_total = current.interrupted_requests_total;
        value.discarded_output_frames_total = current.discarded_output_frames_total;
        if current.state == EchoCancellationState::Interrupted && value.last_error.is_none() {
            value.state = EchoCancellationState::Interrupted;
        }
        *current = value;
    }

    #[cfg(feature = "aec")]
    pub(crate) fn fail(&self, message: String) {
        let mut current = self
            .0
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        current.state = EchoCancellationState::Failed;
        current.last_error.get_or_insert(message);
    }

    #[cfg(feature = "aec")]
    pub(crate) fn interrupt(&self) {
        let mut current = self
            .0
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        current.interrupted_requests_total = current.interrupted_requests_total.saturating_add(1);
        if current.last_error.is_none() {
            current.state = EchoCancellationState::Interrupted;
        }
    }

    #[cfg(feature = "aec")]
    pub(crate) fn discard_outputs(&self, outputs: usize) {
        let mut current = self
            .0
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        current.discarded_output_frames_total = current
            .discarded_output_frames_total
            .saturating_add(outputs as u64);
        if current.last_error.is_none() {
            current.state = EchoCancellationState::Interrupted;
        }
    }

    pub(crate) fn snapshot(&self) -> EchoCancellationObservations {
        self.0
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .clone()
    }
}

#[cfg(all(test, feature = "aec"))]
mod tests {
    use super::*;

    #[test]
    fn given_cancelled_response_when_native_work_later_finishes_then_failure_is_retained() {
        let state = ObservationState::new(4);
        state.fail("response interrupted".into());
        let mut late = EchoCancellationObservations::new(4);
        late.processed_microphone_frames_total = 1;
        late.state = EchoCancellationState::Stopped;
        state.update(late);
        let observed = state.snapshot();
        assert_eq!(observed.state, EchoCancellationState::Failed);
        assert_eq!(observed.last_error.as_deref(), Some("response interrupted"));
        assert_eq!(observed.processed_microphone_frames_total, 1);
    }
}
