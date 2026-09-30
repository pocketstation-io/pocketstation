use crate::SourceId;
use std::sync::{Arc, Mutex};

/// Processor lifecycle; Processing does not assert acoustic convergence.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EchoCancellationState {
    WaitingForReference,
    Processing,
    Reset,
    Failed,
    Stopped,
}

/// Local processing observations, readable after the Session stops.
#[derive(Debug, Clone)]
pub struct EchoCancellationObservations {
    pub state: EchoCancellationState,
    pub processed_microphone_frames_total: u64,
    pub discarded_microphone_frames_total: u64,
    pub discarded_reference_frames_total: u64,
    pub resets_total: u64,
    pub processing_generation: u64,
    pub microphone_queue_depth_frames: usize,
    pub reference_queue_depth_frames: usize,
    pub queue_capacity_frames: usize,
    pub latest_processing_duration_ns: u64,
    pub maximum_processing_duration_ns: u64,
    pub latest_pair_skew_ns: u64,
    pub reference_source_id: Option<SourceId>,
    pub microphone_source_id: Option<SourceId>,
    /// Unknown until qualified for the selected processor configuration.
    pub qualified_algorithmic_delay_samples: Option<u32>,
    pub last_error: Option<String>,
}

impl EchoCancellationObservations {
    pub(crate) fn new(queue_capacity_frames: usize) -> Self {
        Self {
            state: EchoCancellationState::WaitingForReference,
            processed_microphone_frames_total: 0,
            discarded_microphone_frames_total: 0,
            discarded_reference_frames_total: 0,
            resets_total: 0,
            processing_generation: 1,
            microphone_queue_depth_frames: 0,
            reference_queue_depth_frames: 0,
            queue_capacity_frames,
            latest_processing_duration_ns: 0,
            maximum_processing_duration_ns: 0,
            latest_pair_skew_ns: 0,
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
    pub(crate) fn new(capacity_frames: usize) -> Self {
        Self(Arc::new(Mutex::new(EchoCancellationObservations::new(
            capacity_frames,
        ))))
    }

    pub(crate) fn update(&self, value: EchoCancellationObservations) {
        *self
            .0
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner) = value;
    }

    pub(crate) fn snapshot(&self) -> EchoCancellationObservations {
        self.0
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .clone()
    }
}
