use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;

use crate::capture::{CaptureNativeFormat, CaptureOwnerObservations};
use crate::endpoint::EndpointDriverObservations;
use crate::frame::{EndpointId, RouteId, SourceId, StemId};
use crate::runtime::{
    AsyncOperatorObservations, AsyncOperatorOutputObservations, EdgeObservations,
    GeneratedAudioBridgeObservations, PlanSourceInputObservations, SidecarHostSnapshot,
};

use crate::session::{
    OperatorInstanceId, PolledAudioObservations, SourceInstanceId, SourceRuntimeObservations,
};

/// Point-in-time observations for a session's bounded control-event queue.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SessionEventQueueObservations {
    pub capacity_event_count: u64,
    pub maximum_event_owned_bytes: u64,
    pub maximum_buffered_owned_bytes: u64,
    pub depth_events: u64,
    pub depth_owned_bytes: u64,
    pub peak_depth_event_count: u64,
    pub peak_depth_owned_bytes: u64,
    pub events_enqueued_total: u64,
    pub events_dropped_total: u64,
    pub events_dropped_oversized_total: u64,
    pub receiver_closed_total: u64,
}

/// Point-in-time observations for the current Session.
///
/// The snapshot keeps control-event and foreign-audio queue truth together
/// without exposing either counter owner to a language adapter.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SessionMetricsSnapshot {
    event_queue: SessionEventQueueObservations,
    polled_audio: PolledAudioObservations,
    sources: Box<[SessionSourceMetrics]>,
    source_native_formats: Box<[SessionSourceNativeFormatObservation]>,
    source_replacements: Box<[SessionSourceReplacementObservations]>,
    source_activity: Box<[SessionSourceActivityObservations]>,
    source_signal: Box<[SessionSourceSignalObservations]>,
    external_sources: Box<[SessionExternalSourceMetrics]>,
    routes: Box<[SessionRouteMetrics]>,
    operators: Box<[SessionOperatorMetrics]>,
    derived_routes: Box<[SessionDerivedRouteMetrics]>,
}

pub(crate) struct SessionSourceMetricSnapshots {
    pub(crate) metrics: Box<[SessionSourceMetrics]>,
    pub(crate) native_formats: Box<[SessionSourceNativeFormatObservation]>,
    pub(crate) replacements: Box<[SessionSourceReplacementObservations]>,
    pub(crate) activity: Box<[SessionSourceActivityObservations]>,
    pub(crate) signal: Box<[SessionSourceSignalObservations]>,
}

impl SessionMetricsSnapshot {
    pub(crate) fn new(
        event_queue: SessionEventQueueObservations,
        polled_audio: PolledAudioObservations,
        sources: SessionSourceMetricSnapshots,
        external_sources: Box<[SessionExternalSourceMetrics]>,
        routes: Box<[SessionRouteMetrics]>,
        operators: Box<[SessionOperatorMetrics]>,
        derived_routes: Box<[SessionDerivedRouteMetrics]>,
    ) -> Self {
        let SessionSourceMetricSnapshots {
            metrics,
            native_formats,
            replacements,
            activity,
            signal,
        } = sources;
        Self {
            event_queue,
            polled_audio,
            sources: metrics,
            source_native_formats: native_formats,
            source_replacements: replacements,
            source_activity: activity,
            source_signal: signal,
            external_sources,
            routes,
            operators,
            derived_routes,
        }
    }

    pub const fn event_queue(&self) -> SessionEventQueueObservations {
        self.event_queue
    }

    pub const fn polled_audio(&self) -> PolledAudioObservations {
        self.polled_audio
    }

    pub fn source_count(&self) -> usize {
        self.sources.len()
    }

    pub fn source(&self, index: usize) -> Option<&SessionSourceMetrics> {
        self.sources.get(index)
    }

    /// Returns the native device format opened for the built-in Source at
    /// `index`, in the same stable declaration order as [`Self::source`].
    pub fn source_native_format(
        &self,
        index: usize,
    ) -> Option<&SessionSourceNativeFormatObservation> {
        self.source_native_formats.get(index)
    }

    pub fn source_native_format_count(&self) -> usize {
        self.source_native_formats.len()
    }

    /// Returns explicit host-requested physical-source replacement facts for
    /// the built-in Source at `index`, in declaration order.
    pub fn source_replacement(
        &self,
        index: usize,
    ) -> Option<&SessionSourceReplacementObservations> {
        self.source_replacements.get(index)
    }

    pub fn source_replacement_count(&self) -> usize {
        self.source_replacements.len()
    }

    /// Returns the raw frame-delivery activity for the built-in Source at
    /// `index`.
    ///
    /// Activity entries use the same stable declaration order as [`Self::source`].
    /// They are separate from [`SessionSourceMetrics`] so the existing public
    /// metrics record remains source-compatible.
    pub fn source_activity(&self, index: usize) -> Option<&SessionSourceActivityObservations> {
        self.source_activity.get(index)
    }

    pub fn source_activity_count(&self) -> usize {
        self.source_activity.len()
    }

    /// Returns the latest delivered PCM-window measurement for the built-in
    /// Source at `index`, in the same stable declaration order as
    /// [`Self::source`].
    pub fn source_signal(&self, index: usize) -> Option<&SessionSourceSignalObservations> {
        self.source_signal.get(index)
    }

    pub fn source_signal_count(&self) -> usize {
        self.source_signal.len()
    }

    pub fn external_source_count(&self) -> usize {
        self.external_sources.len()
    }

    pub fn external_source(&self, index: usize) -> Option<&SessionExternalSourceMetrics> {
        self.external_sources.get(index)
    }

    pub fn route_count(&self) -> usize {
        self.routes.len()
    }

    pub fn route(&self, index: usize) -> Option<&SessionRouteMetrics> {
        self.routes.get(index)
    }

    pub fn operator_count(&self) -> usize {
        self.operators.len()
    }

    pub fn operator(&self, index: usize) -> Option<&SessionOperatorMetrics> {
        self.operators.get(index)
    }

    pub fn derived_route_count(&self) -> usize {
        self.derived_routes.len()
    }

    pub fn derived_route(&self, index: usize) -> Option<&SessionDerivedRouteMetrics> {
        self.derived_routes.get(index)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SessionSourceMetrics {
    pub stem_id: StemId,
    pub capture: CaptureOwnerObservations,
    pub ingress: PlanSourceInputObservations,
}

/// Native acquisition format observed when one built-in Source opened.
///
/// `opened_native_format` is `None` for capture backends that do not negotiate
/// a PCM device format. It never changes the canonical format delivered to the
/// Session graph.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SessionSourceNativeFormatObservation {
    pub stem_id: StemId,
    pub opened_native_format: Option<CaptureNativeFormat>,
}

/// Control-path accounting for explicit physical-source replacement.
///
/// A replacement attempt never implies automatic fallback. The host chooses
/// the exact device and calls the replacement operation deliberately.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SessionSourceReplacementObservations {
    pub stem_id: StemId,
    pub attempts_total: u64,
    pub completed_total: u64,
    pub failed_before_attach_total: u64,
    pub response_timeouts_total: u64,
    /// Physical source currently attached to the logical stem. `None` means
    /// an explicit detach completed but reacquisition has not attached.
    pub attached_source_id: Option<SourceId>,
    /// Continuity generation assigned to the attached or next capture.
    pub source_generation: u32,
    pub discontinuity_epoch: u64,
    pub latest_completed_at_ns: Option<u64>,
}

/// Raw process-clock activity observed after a built-in Source frame leaves
/// the capture queue and reaches the canonical Session runtime.
///
/// These values report delivery activity, not audible sound or speech. A frame
/// containing digital silence still counts as a received frame. Timestamps use
/// PocketStation's process-monotonic nanosecond domain.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SessionSourceActivityObservations {
    pub session_started_at_ns: u64,
    pub observed_at_ns: u64,
    pub first_frame_received_at_ns: Option<u64>,
    pub latest_frame_received_at_ns: Option<u64>,
    pub frames_received_total: u64,
}

/// Latest delivered PCM-window measurements for one built-in Source.
///
/// Measurement runs after capture dequeue on the Session runtime worker. The
/// values describe PCM samples only: they do not infer speech, audibility,
/// permission, intended routing, or whether an application should recover.
/// `window_timestamp_start_ns` uses the Source clock carried by frame lineage;
/// `window_observed_at_ns` and `observed_at_ns` use PocketStation's process
/// monotonic clock.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SessionSourceSignalObservations {
    pub observed_at_ns: u64,
    pub samples_observed_total: u64,
    pub exact_zero_samples_observed_total: u64,
    pub nonzero_samples_observed_total: u64,
    pub nonfinite_samples_observed_total: u64,
    pub window_timestamp_start_ns: Option<u64>,
    pub window_duration_ns: u64,
    pub window_observed_at_ns: Option<u64>,
    pub window_sequence_number: Option<u64>,
    pub window_source_generation: u32,
    pub window_discontinuity_epoch: u64,
    pub window_samples_total: u64,
    pub window_exact_zero_samples_total: u64,
    pub window_nonzero_samples_total: u64,
    pub window_nonfinite_samples_total: u64,
    window_peak_linear_bits: u32,
    window_mean_square_linear_bits: u64,
    pub consecutive_exact_zero_duration_ns: u64,
}

impl SessionSourceSignalObservations {
    pub(crate) fn from_atomic_snapshot(snapshot: SessionSourceSignalAtomicSnapshot) -> Self {
        let has_window = snapshot.samples_observed_total > 0;
        Self {
            observed_at_ns: snapshot.observed_at_ns,
            samples_observed_total: snapshot.samples_observed_total,
            exact_zero_samples_observed_total: snapshot.exact_zero_samples_observed_total,
            nonzero_samples_observed_total: snapshot.nonzero_samples_observed_total,
            nonfinite_samples_observed_total: snapshot.nonfinite_samples_observed_total,
            window_timestamp_start_ns: has_window.then_some(snapshot.window_timestamp_start_ns),
            window_duration_ns: snapshot.window_duration_ns,
            window_observed_at_ns: has_window.then_some(snapshot.window_observed_at_ns),
            window_sequence_number: has_window.then_some(snapshot.window_sequence_number),
            window_source_generation: snapshot.window_source_generation,
            window_discontinuity_epoch: snapshot.window_discontinuity_epoch,
            window_samples_total: snapshot.window_samples_total,
            window_exact_zero_samples_total: snapshot.window_exact_zero_samples_total,
            window_nonzero_samples_total: snapshot.window_nonzero_samples_total,
            window_nonfinite_samples_total: snapshot.window_nonfinite_samples_total,
            window_peak_linear_bits: snapshot.window_peak_linear_bits,
            window_mean_square_linear_bits: snapshot.window_mean_square_linear_bits,
            consecutive_exact_zero_duration_ns: snapshot.consecutive_exact_zero_duration_ns,
        }
    }

    pub fn window_peak_linear(self) -> Option<f32> {
        (self.window_samples_total > self.window_nonfinite_samples_total)
            .then(|| f32::from_bits(self.window_peak_linear_bits))
    }

    pub fn window_rms_linear(self) -> Option<f64> {
        (self.window_samples_total > self.window_nonfinite_samples_total)
            .then(|| f64::from_bits(self.window_mean_square_linear_bits).sqrt())
    }

    pub fn window_peak_dbfs(self) -> Option<f64> {
        linear_to_dbfs(self.window_peak_linear().map(f64::from)?)
    }

    pub fn window_rms_dbfs(self) -> Option<f64> {
        linear_to_dbfs(self.window_rms_linear()?)
    }

    pub fn window_exact_zero_ratio(self) -> Option<f64> {
        (self.window_samples_total > 0)
            .then(|| self.window_exact_zero_samples_total as f64 / self.window_samples_total as f64)
    }

    pub fn evaluate(self, policy: SessionSourceSignalPolicy) -> SessionSourceSignalEvaluation {
        let state = if self.window_samples_total == 0 {
            SessionSourceSignalState::NoSamplesObserved
        } else if self.window_nonfinite_samples_total > 0 {
            SessionSourceSignalState::NonFiniteSamplesObserved
        } else if self.window_exact_zero_samples_total == self.window_samples_total {
            if self.consecutive_exact_zero_duration_ns >= policy.exact_zero_timeout_ns {
                SessionSourceSignalState::SustainedExactDigitalZero
            } else {
                SessionSourceSignalState::ExactDigitalZeroPending
            }
        } else {
            let peak_dbfs = self.window_peak_dbfs().unwrap_or(f64::NEG_INFINITY);
            let rms_dbfs = self.window_rms_dbfs().unwrap_or(f64::NEG_INFINITY);
            if peak_dbfs >= policy.minimum_peak_dbfs() && rms_dbfs >= policy.minimum_rms_dbfs() {
                SessionSourceSignalState::MeetsCallerThresholds
            } else {
                SessionSourceSignalState::BelowCallerThresholds
            }
        };
        SessionSourceSignalEvaluation {
            state,
            peak_dbfs: self.window_peak_dbfs(),
            rms_dbfs: self.window_rms_dbfs(),
            consecutive_exact_zero_duration_ns: self.consecutive_exact_zero_duration_ns,
        }
    }
}

pub(crate) struct SessionSourceSignalAtomicSnapshot {
    pub(crate) observed_at_ns: u64,
    pub(crate) samples_observed_total: u64,
    pub(crate) exact_zero_samples_observed_total: u64,
    pub(crate) nonzero_samples_observed_total: u64,
    pub(crate) nonfinite_samples_observed_total: u64,
    pub(crate) window_timestamp_start_ns: u64,
    pub(crate) window_duration_ns: u64,
    pub(crate) window_observed_at_ns: u64,
    pub(crate) window_sequence_number: u64,
    pub(crate) window_source_generation: u32,
    pub(crate) window_discontinuity_epoch: u64,
    pub(crate) window_samples_total: u64,
    pub(crate) window_exact_zero_samples_total: u64,
    pub(crate) window_nonzero_samples_total: u64,
    pub(crate) window_nonfinite_samples_total: u64,
    pub(crate) window_peak_linear_bits: u32,
    pub(crate) window_mean_square_linear_bits: u64,
    pub(crate) consecutive_exact_zero_duration_ns: u64,
}

fn linear_to_dbfs(linear: f64) -> Option<f64> {
    (linear.is_finite() && linear > 0.0).then(|| 20.0 * linear.log10())
}

/// Caller-owned thresholds for interpreting one Source signal observation.
///
/// Core intentionally provides no default. Thresholds say only whether the
/// measured PCM meets the caller's numeric policy; they do not establish
/// speech, audibility, route correctness, or a recovery decision.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SessionSourceSignalPolicy {
    minimum_peak_dbfs_bits: u64,
    minimum_rms_dbfs_bits: u64,
    exact_zero_timeout_ns: u64,
}

impl SessionSourceSignalPolicy {
    pub fn new(
        minimum_peak_dbfs: f64,
        minimum_rms_dbfs: f64,
        exact_zero_timeout: Duration,
    ) -> Result<Self, SessionSourceSignalPolicyError> {
        if !minimum_peak_dbfs.is_finite() || minimum_peak_dbfs > 0.0 {
            return Err(SessionSourceSignalPolicyError::InvalidMinimumPeakDbfs);
        }
        if !minimum_rms_dbfs.is_finite() || minimum_rms_dbfs > 0.0 {
            return Err(SessionSourceSignalPolicyError::InvalidMinimumRmsDbfs);
        }
        let exact_zero_timeout_ns = duration_ns(exact_zero_timeout)
            .ok_or(SessionSourceSignalPolicyError::InvalidExactZeroTimeout)?;
        Ok(Self {
            minimum_peak_dbfs_bits: minimum_peak_dbfs.to_bits(),
            minimum_rms_dbfs_bits: minimum_rms_dbfs.to_bits(),
            exact_zero_timeout_ns,
        })
    }

    pub fn minimum_peak_dbfs(self) -> f64 {
        f64::from_bits(self.minimum_peak_dbfs_bits)
    }

    pub fn minimum_rms_dbfs(self) -> f64 {
        f64::from_bits(self.minimum_rms_dbfs_bits)
    }

    pub const fn exact_zero_timeout_ns(self) -> u64 {
        self.exact_zero_timeout_ns
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
pub enum SessionSourceSignalPolicyError {
    #[error("minimum peak threshold must be finite and no greater than 0 dBFS")]
    InvalidMinimumPeakDbfs,
    #[error("minimum RMS threshold must be finite and no greater than 0 dBFS")]
    InvalidMinimumRmsDbfs,
    #[error(
        "exact-digital-zero timeout must be finite, non-zero, and representable in nanoseconds"
    )]
    InvalidExactZeroTimeout,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SessionSourceSignalState {
    NoSamplesObserved,
    ExactDigitalZeroPending,
    SustainedExactDigitalZero,
    BelowCallerThresholds,
    MeetsCallerThresholds,
    NonFiniteSamplesObserved,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SessionSourceSignalEvaluation {
    pub state: SessionSourceSignalState,
    pub peak_dbfs: Option<f64>,
    pub rms_dbfs: Option<f64>,
    pub consecutive_exact_zero_duration_ns: u64,
}

impl SessionSourceActivityObservations {
    /// Evaluates delivery activity using the caller's workflow-specific
    /// startup and stall deadlines.
    ///
    /// This method does not inspect sample energy, infer permission state,
    /// replace a Source, or restart capture.
    pub fn evaluate(self, policy: SessionSourceActivityPolicy) -> SessionSourceActivityEvaluation {
        let session_age_ns = self
            .observed_at_ns
            .saturating_sub(self.session_started_at_ns);
        match self.latest_frame_received_at_ns {
            None if session_age_ns >= policy.first_frame_timeout_ns => {
                SessionSourceActivityEvaluation {
                    state: SessionSourceActivityState::FirstFrameTimedOut,
                    session_age_ns,
                    latest_frame_age_ns: None,
                }
            }
            None => SessionSourceActivityEvaluation {
                state: SessionSourceActivityState::AwaitingFirstFrame,
                session_age_ns,
                latest_frame_age_ns: None,
            },
            Some(latest_frame_received_at_ns) => {
                let latest_frame_age_ns = self
                    .observed_at_ns
                    .saturating_sub(latest_frame_received_at_ns);
                let state = if latest_frame_age_ns >= policy.stall_timeout_ns {
                    SessionSourceActivityState::Stalled
                } else {
                    SessionSourceActivityState::Active
                };
                SessionSourceActivityEvaluation {
                    state,
                    session_age_ns,
                    latest_frame_age_ns: Some(latest_frame_age_ns),
                }
            }
        }
    }
}

/// Caller-owned deadlines for evaluating Source delivery activity.
///
/// Core intentionally has no universal first-frame or stall threshold: an
/// interactive dictation control and a long-running meeting recorder have
/// different budgets.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SessionSourceActivityPolicy {
    first_frame_timeout_ns: u64,
    stall_timeout_ns: u64,
}

impl SessionSourceActivityPolicy {
    pub fn new(
        first_frame_timeout: Duration,
        stall_timeout: Duration,
    ) -> Result<Self, SessionSourceActivityPolicyError> {
        let first_frame_timeout_ns = duration_ns(first_frame_timeout)
            .ok_or(SessionSourceActivityPolicyError::InvalidFirstFrameTimeout)?;
        let stall_timeout_ns = duration_ns(stall_timeout)
            .ok_or(SessionSourceActivityPolicyError::InvalidStallTimeout)?;
        Ok(Self {
            first_frame_timeout_ns,
            stall_timeout_ns,
        })
    }

    pub const fn first_frame_timeout_ns(self) -> u64 {
        self.first_frame_timeout_ns
    }

    pub const fn stall_timeout_ns(self) -> u64 {
        self.stall_timeout_ns
    }
}

fn duration_ns(duration: Duration) -> Option<u64> {
    if duration.is_zero() {
        return None;
    }
    u64::try_from(duration.as_nanos()).ok()
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
pub enum SessionSourceActivityPolicyError {
    #[error("first-frame timeout must be finite, non-zero, and representable in nanoseconds")]
    InvalidFirstFrameTimeout,
    #[error("stall timeout must be finite, non-zero, and representable in nanoseconds")]
    InvalidStallTimeout,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SessionSourceActivityState {
    AwaitingFirstFrame,
    Active,
    FirstFrameTimedOut,
    Stalled,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SessionSourceActivityEvaluation {
    pub state: SessionSourceActivityState,
    pub session_age_ns: u64,
    pub latest_frame_age_ns: Option<u64>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SessionExternalSourceMetrics {
    pub source_instance_id: SourceInstanceId,
    pub source_id: SourceId,
    pub runtime: SourceRuntimeObservations,
}

/// Exact bounded-queue and process-lifecycle accounting for one Session-owned
/// language-neutral sidecar.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SessionSidecarMetrics {
    pub sidecar_id: u64,
    pub host: SidecarHostSnapshot,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SessionRouteMetrics {
    pub route_id: RouteId,
    pub endpoint_id: EndpointId,
    pub edge: EdgeObservations,
    pub endpoint: Option<EndpointDriverObservations>,
    pub endpoint_observation_stage: EndpointObservationStage,
    pub endpoint_finalization_failures_total: u64,
}

/// Interval covered by monotonic route counters.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SessionRouteObservationInterval {
    /// From route start through the instant of the Session snapshot.
    RouteLifetimeToSnapshot,
}

/// Explicit numerator, denominator, interval, and typed reasons for one route.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SessionRouteDropObservations {
    pub route_id: RouteId,
    pub interval: SessionRouteObservationInterval,
    pub frames_dropped_total: u64,
    pub frames_attempted_total: u64,
    pub receiver_unavailable_drops_total: u64,
    pub queue_full_drops_total: u64,
    pub shared_reference_exhausted_drops_total: u64,
    pub branch_pool_exhausted_drops_total: u64,
    pub invalid_copy_policy_drops_total: u64,
    pub freeze_failed_drops_total: u64,
}

impl SessionRouteDropObservations {
    pub fn drop_rate_pct(self) -> f64 {
        if self.frames_attempted_total == 0 {
            0.0
        } else {
            self.frames_dropped_total as f64 / self.frames_attempted_total as f64 * 100.0
        }
    }
}

/// Common-clock source timestamp to route-receive latency in nanoseconds.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SessionRouteLatencyObservations {
    pub route_id: RouteId,
    pub measurement: RouteLatencyMeasurement,
    pub unit: SessionRouteLatencyUnit,
    pub samples_total: u64,
    pub missing_or_incompatible_clock_total: u64,
    pub future_timestamp_total: u64,
    pub p50_ns: u64,
    pub p95_ns: u64,
    pub p99_ns: u64,
    pub max_ns: u64,
}

impl SessionRouteLatencyObservations {
    /// Returns the timestamps used to calculate this latency.
    pub const fn measurement(self) -> RouteLatencyMeasurement {
        self.measurement
    }
}

/// Identifies the timestamps used to calculate route latency.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RouteLatencyMeasurement {
    SourceMonotonicTimestampToRouteReceive,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SessionRouteLatencyUnit {
    Nanoseconds,
}

impl SessionRouteMetrics {
    pub fn drop_observations(self) -> SessionRouteDropObservations {
        SessionRouteDropObservations {
            route_id: self.route_id,
            interval: SessionRouteObservationInterval::RouteLifetimeToSnapshot,
            frames_dropped_total: self.edge.frames_dropped_total,
            frames_attempted_total: self.edge.frames_attempted_total(),
            receiver_unavailable_drops_total: self.edge.receiver_unavailable_drops_total,
            queue_full_drops_total: self.edge.queue_full_drops_total,
            shared_reference_exhausted_drops_total: self
                .edge
                .shared_reference_exhausted_drops_total,
            branch_pool_exhausted_drops_total: self.edge.branch_pool_exhausted_drops_total,
            invalid_copy_policy_drops_total: self.edge.invalid_copy_policy_drops_total,
            freeze_failed_drops_total: self.edge.freeze_failed_drops_total,
        }
    }

    pub const fn source_to_receive_latency(self) -> SessionRouteLatencyObservations {
        SessionRouteLatencyObservations {
            route_id: self.route_id,
            measurement: RouteLatencyMeasurement::SourceMonotonicTimestampToRouteReceive,
            unit: SessionRouteLatencyUnit::Nanoseconds,
            samples_total: self.edge.source_timestamp_to_receive_samples_total,
            missing_or_incompatible_clock_total: self
                .edge
                .source_timestamp_to_receive_missing_total,
            future_timestamp_total: self.edge.source_timestamp_to_receive_future_total,
            p50_ns: self.edge.source_timestamp_to_receive_p50_ns,
            p95_ns: self.edge.source_timestamp_to_receive_p95_ns,
            p99_ns: self.edge.source_timestamp_to_receive_p99_ns,
            max_ns: self.edge.source_timestamp_to_receive_max_ns,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SessionOperatorInputMetrics {
    pub port_name: String,
    pub edge: EdgeObservations,
}

/// Exact boundedness and lifecycle accounting for one operator PCM output
/// re-entering the Session audio lane.
///
/// This measurement belongs to the Session. The bridge worker and its queue are
/// deliberately not public extension APIs.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SessionAudioReentryMetrics {
    operator_instance_id: OperatorInstanceId,
    stem_id: StemId,
    queue_capacity_signals: u64,
    queue_depth_signals: u64,
    queue_peak_signals: u64,
    signals_enqueued_total: u64,
    signals_received_total: u64,
    signals_dropped_total: u64,
    pool_slots: u64,
    frame_capacity_samples: u64,
    maximum_buffered_audio_bytes: u64,
    normalized_total: u64,
    invalid_total: u64,
    shared_audio_rejected_total: u64,
    pool_exhausted_total: u64,
    ingress_rejected_total: u64,
    audio_frames_enqueued_total: u64,
    cancellation_total: u64,
    joined: bool,
}

impl SessionAudioReentryMetrics {
    pub(crate) fn from_bridge(
        operator_instance_id: OperatorInstanceId,
        stem_id: StemId,
        bridge: GeneratedAudioBridgeObservations,
    ) -> Self {
        Self {
            operator_instance_id,
            stem_id,
            queue_capacity_signals: bridge.input_edge.capacity_signals,
            queue_depth_signals: bridge.input_edge.depth_signals,
            queue_peak_signals: bridge.input_edge.peak_depth_signals,
            signals_enqueued_total: bridge.input_edge.enqueued_total,
            signals_received_total: bridge.input_edge.received_total,
            signals_dropped_total: bridge.input_edge.dropped_total,
            pool_slots: bridge.pool_slots,
            frame_capacity_samples: bridge.frame_capacity_samples,
            maximum_buffered_audio_bytes: bridge.maximum_buffered_audio_bytes,
            normalized_total: bridge.normalized_total,
            invalid_total: bridge.invalid_total,
            shared_audio_rejected_total: bridge.shared_audio_rejected_total,
            pool_exhausted_total: bridge.pool_exhausted_total,
            ingress_rejected_total: bridge.ingress_rejected_total,
            audio_frames_enqueued_total: bridge.enqueued_total,
            cancellation_total: bridge.cancellation_total,
            joined: bridge.joined,
        }
    }

    pub const fn operator_instance_id(self) -> OperatorInstanceId {
        self.operator_instance_id
    }

    pub const fn stem_id(self) -> StemId {
        self.stem_id
    }

    pub const fn queue_capacity_signals(self) -> u64 {
        self.queue_capacity_signals
    }

    pub const fn queue_depth_signals(self) -> u64 {
        self.queue_depth_signals
    }

    pub const fn queue_peak_signals(self) -> u64 {
        self.queue_peak_signals
    }

    pub const fn signals_enqueued_total(self) -> u64 {
        self.signals_enqueued_total
    }

    pub const fn signals_received_total(self) -> u64 {
        self.signals_received_total
    }

    pub const fn signals_dropped_total(self) -> u64 {
        self.signals_dropped_total
    }

    pub const fn pool_slots(self) -> u64 {
        self.pool_slots
    }

    pub const fn frame_capacity_samples(self) -> u64 {
        self.frame_capacity_samples
    }

    pub const fn maximum_buffered_audio_bytes(self) -> u64 {
        self.maximum_buffered_audio_bytes
    }

    pub const fn normalized_total(self) -> u64 {
        self.normalized_total
    }

    pub const fn invalid_total(self) -> u64 {
        self.invalid_total
    }

    pub const fn shared_audio_rejected_total(self) -> u64 {
        self.shared_audio_rejected_total
    }

    pub const fn pool_exhausted_total(self) -> u64 {
        self.pool_exhausted_total
    }

    pub const fn ingress_rejected_total(self) -> u64 {
        self.ingress_rejected_total
    }

    pub const fn audio_frames_enqueued_total(self) -> u64 {
        self.audio_frames_enqueued_total
    }

    pub const fn cancellation_total(self) -> u64 {
        self.cancellation_total
    }

    pub const fn joined(self) -> bool {
        self.joined
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SessionOperatorMetrics {
    pub operator_instance_id: OperatorInstanceId,
    /// Sole counter authority for input delivered by the compiled Session plan.
    ///
    /// `worker.input_*` remains meaningful only for workers fed through the
    /// direct `AsyncOperatorInput` API. Compiled Session operators consume this
    /// plan edge, so callers must use these observations for input accounting.
    pub input_delivery: EdgeObservations,
    /// Exact per-port input accounting. `input_delivery` is the aggregate
    /// across this slice.
    pub input_ports: Box<[SessionOperatorInputMetrics]>,
    pub worker: AsyncOperatorObservations,
    pub finalization_failures_total: u64,
}

impl SessionOperatorMetrics {
    pub fn input_port(&self, name: &str) -> Option<&SessionOperatorInputMetrics> {
        self.input_ports.iter().find(|port| port.port_name == name)
    }
    pub const fn input_queue_capacity_frames(&self) -> u64 {
        self.input_delivery.queue_capacity_frames
    }

    pub const fn input_queue_depth_frames(&self) -> u64 {
        self.input_delivery.queue_depth_frames
    }

    pub const fn input_queue_peak_frames(&self) -> u64 {
        self.input_delivery.queue_peak_frames
    }

    pub fn input_attempted_total(&self) -> u64 {
        self.input_delivery.frames_attempted_total()
    }

    pub const fn input_enqueued_total(&self) -> u64 {
        self.input_delivery.frames_enqueued_total
    }

    pub const fn input_delivered_total(&self) -> u64 {
        self.input_delivery.frames_delivered_total
    }

    pub const fn input_dropped_total(&self) -> u64 {
        self.input_delivery.frames_dropped_total
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SessionDerivedRouteMetrics {
    pub route_id: RouteId,
    pub endpoint_id: EndpointId,
    pub output: AsyncOperatorOutputObservations,
    pub endpoint: Option<EndpointDriverObservations>,
    pub endpoint_observation_stage: EndpointObservationStage,
    pub endpoint_finalization_failures_total: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EndpointObservationStage {
    Unavailable,
    Live,
    Finalized,
}

#[derive(Debug)]
pub(crate) struct SessionEventQueueCounters {
    capacity_event_count: u64,
    maximum_event_owned_bytes: u64,
    maximum_buffered_owned_bytes: u64,
    depth_events: AtomicU64,
    depth_owned_bytes: AtomicU64,
    peak_depth_event_count: AtomicU64,
    peak_depth_owned_bytes: AtomicU64,
    events_enqueued_total: AtomicU64,
    events_dropped_total: AtomicU64,
    events_dropped_oversized_total: AtomicU64,
    receiver_closed_total: AtomicU64,
}

pub(crate) enum SessionEventReservation {
    Reserved,
    Full,
    Oversized,
}

impl SessionEventQueueCounters {
    pub(crate) fn new(capacity_events: usize, maximum_event_owned_bytes: usize) -> Self {
        Self {
            capacity_event_count: capacity_events as u64,
            maximum_event_owned_bytes: maximum_event_owned_bytes as u64,
            maximum_buffered_owned_bytes: capacity_events.saturating_mul(maximum_event_owned_bytes)
                as u64,
            depth_events: AtomicU64::new(0),
            depth_owned_bytes: AtomicU64::new(0),
            peak_depth_event_count: AtomicU64::new(0),
            peak_depth_owned_bytes: AtomicU64::new(0),
            events_enqueued_total: AtomicU64::new(0),
            events_dropped_total: AtomicU64::new(0),
            events_dropped_oversized_total: AtomicU64::new(0),
            receiver_closed_total: AtomicU64::new(0),
        }
    }

    pub(crate) fn reserve_event(&self, owned_bytes: usize) -> SessionEventReservation {
        if owned_bytes as u64 > self.maximum_event_owned_bytes {
            self.events_dropped_total.fetch_add(1, Ordering::Relaxed);
            self.events_dropped_oversized_total
                .fetch_add(1, Ordering::Relaxed);
            return SessionEventReservation::Oversized;
        }
        let mut depth_events = self.depth_events.load(Ordering::Relaxed);
        loop {
            if depth_events >= self.capacity_event_count {
                self.events_dropped_total.fetch_add(1, Ordering::Relaxed);
                return SessionEventReservation::Full;
            }

            match self.depth_events.compare_exchange_weak(
                depth_events,
                depth_events + 1,
                Ordering::AcqRel,
                Ordering::Relaxed,
            ) {
                Ok(_) => {
                    self.peak_depth_event_count
                        .fetch_max(depth_events + 1, Ordering::Relaxed);
                    let depth_owned_bytes = self
                        .depth_owned_bytes
                        .fetch_add(owned_bytes as u64, Ordering::AcqRel)
                        .saturating_add(owned_bytes as u64);
                    self.peak_depth_owned_bytes
                        .fetch_max(depth_owned_bytes, Ordering::Relaxed);
                    return SessionEventReservation::Reserved;
                }
                Err(observed_depth_events) => depth_events = observed_depth_events,
            }
        }
    }

    pub(crate) fn observe_enqueued(&self) {
        self.events_enqueued_total.fetch_add(1, Ordering::Relaxed);
    }

    pub(crate) fn observe_send_full(&self, owned_bytes: usize) {
        self.cancel_reservation(owned_bytes);
        self.events_dropped_total.fetch_add(1, Ordering::Relaxed);
    }

    pub(crate) fn observe_receiver_closed(&self, owned_bytes: usize) {
        self.cancel_reservation(owned_bytes);
        self.events_dropped_total.fetch_add(1, Ordering::Relaxed);
        self.receiver_closed_total.fetch_add(1, Ordering::Relaxed);
    }

    pub(crate) fn observe_dequeued(&self, owned_bytes: usize) {
        let previous_depth_events = self.depth_events.fetch_sub(1, Ordering::AcqRel);
        debug_assert!(previous_depth_events > 0);
        let previous_depth_owned_bytes = self
            .depth_owned_bytes
            .fetch_sub(owned_bytes as u64, Ordering::AcqRel);
        debug_assert!(previous_depth_owned_bytes >= owned_bytes as u64);
    }

    pub(crate) fn snapshot(&self) -> SessionEventQueueObservations {
        SessionEventQueueObservations {
            capacity_event_count: self.capacity_event_count,
            maximum_event_owned_bytes: self.maximum_event_owned_bytes,
            maximum_buffered_owned_bytes: self.maximum_buffered_owned_bytes,
            depth_events: self.depth_events.load(Ordering::Acquire),
            depth_owned_bytes: self.depth_owned_bytes.load(Ordering::Acquire),
            peak_depth_event_count: self.peak_depth_event_count.load(Ordering::Relaxed),
            peak_depth_owned_bytes: self.peak_depth_owned_bytes.load(Ordering::Relaxed),
            events_enqueued_total: self.events_enqueued_total.load(Ordering::Relaxed),
            events_dropped_total: self.events_dropped_total.load(Ordering::Relaxed),
            events_dropped_oversized_total: self
                .events_dropped_oversized_total
                .load(Ordering::Relaxed),
            receiver_closed_total: self.receiver_closed_total.load(Ordering::Relaxed),
        }
    }

    fn cancel_reservation(&self, owned_bytes: usize) {
        let previous_depth_events = self.depth_events.fetch_sub(1, Ordering::AcqRel);
        debug_assert!(previous_depth_events > 0);
        let previous_depth_owned_bytes = self
            .depth_owned_bytes
            .fetch_sub(owned_bytes as u64, Ordering::AcqRel);
        debug_assert!(previous_depth_owned_bytes >= owned_bytes as u64);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn signal_observations(
        samples_total: u64,
        exact_zero_total: u64,
        nonzero_total: u64,
        nonfinite_total: u64,
        peak_linear: f32,
        rms_linear: f64,
        consecutive_exact_zero_duration_ns: u64,
    ) -> SessionSourceSignalObservations {
        SessionSourceSignalObservations::from_atomic_snapshot(SessionSourceSignalAtomicSnapshot {
            observed_at_ns: 500,
            samples_observed_total: samples_total,
            exact_zero_samples_observed_total: exact_zero_total,
            nonzero_samples_observed_total: nonzero_total,
            nonfinite_samples_observed_total: nonfinite_total,
            window_timestamp_start_ns: 100,
            window_duration_ns: 20_000_000,
            window_observed_at_ns: 200,
            window_sequence_number: 7,
            window_source_generation: 1,
            window_discontinuity_epoch: 0,
            window_samples_total: samples_total,
            window_exact_zero_samples_total: exact_zero_total,
            window_nonzero_samples_total: nonzero_total,
            window_nonfinite_samples_total: nonfinite_total,
            window_peak_linear_bits: peak_linear.to_bits(),
            window_mean_square_linear_bits: (rms_linear * rms_linear).to_bits(),
            consecutive_exact_zero_duration_ns,
        })
    }

    #[test]
    fn given_source_signal_measurements_when_evaluated_then_numeric_states_remain_distinct() {
        let policy = SessionSourceSignalPolicy::new(-40.0, -50.0, Duration::from_millis(40))
            .expect("finite caller thresholds");

        let no_samples = signal_observations(0, 0, 0, 0, 0.0, 0.0, 0).evaluate(policy);
        assert_eq!(
            no_samples.state,
            SessionSourceSignalState::NoSamplesObserved
        );

        let pending_zero =
            signal_observations(960, 960, 0, 0, 0.0, 0.0, 20_000_000).evaluate(policy);
        assert_eq!(
            pending_zero.state,
            SessionSourceSignalState::ExactDigitalZeroPending
        );
        assert_eq!(pending_zero.peak_dbfs, None);
        assert_eq!(pending_zero.rms_dbfs, None);

        let sustained_zero =
            signal_observations(960, 960, 0, 0, 0.0, 0.0, 40_000_000).evaluate(policy);
        assert_eq!(
            sustained_zero.state,
            SessionSourceSignalState::SustainedExactDigitalZero
        );

        let below = signal_observations(960, 0, 960, 0, 0.001, 0.001, 0).evaluate(policy);
        assert_eq!(below.state, SessionSourceSignalState::BelowCallerThresholds);

        let meets = signal_observations(960, 0, 960, 0, 0.1, 0.01, 0).evaluate(policy);
        assert_eq!(meets.state, SessionSourceSignalState::MeetsCallerThresholds);

        let nonfinite = signal_observations(960, 0, 959, 1, 0.1, 0.01, 0).evaluate(policy);
        assert_eq!(
            nonfinite.state,
            SessionSourceSignalState::NonFiniteSamplesObserved
        );
    }

    #[test]
    fn given_source_signal_window_when_read_then_units_and_ratios_are_explicit() {
        let observations = signal_observations(960, 240, 720, 0, 0.5, 0.25, 0);
        assert_eq!(observations.window_timestamp_start_ns, Some(100));
        assert_eq!(observations.window_duration_ns, 20_000_000);
        assert_eq!(observations.window_observed_at_ns, Some(200));
        assert_eq!(observations.window_sequence_number, Some(7));
        assert_eq!(observations.window_source_generation, 1);
        assert_eq!(observations.window_discontinuity_epoch, 0);
        assert_eq!(observations.window_exact_zero_ratio(), Some(0.25));
        assert_eq!(observations.window_peak_linear(), Some(0.5));
        assert_eq!(observations.window_rms_linear(), Some(0.25));
        assert!((observations.window_peak_dbfs().expect("non-zero peak") + 6.0206).abs() < 0.001);
        assert!((observations.window_rms_dbfs().expect("non-zero RMS") + 12.0412).abs() < 0.001);
    }

    #[test]
    fn given_invalid_source_signal_policy_when_constructed_then_each_input_fails_closed() {
        assert_eq!(
            SessionSourceSignalPolicy::new(f64::NAN, -60.0, Duration::from_secs(1),),
            Err(SessionSourceSignalPolicyError::InvalidMinimumPeakDbfs)
        );
        assert_eq!(
            SessionSourceSignalPolicy::new(1.0, -60.0, Duration::from_secs(1)),
            Err(SessionSourceSignalPolicyError::InvalidMinimumPeakDbfs)
        );
        assert_eq!(
            SessionSourceSignalPolicy::new(-40.0, f64::INFINITY, Duration::from_secs(1)),
            Err(SessionSourceSignalPolicyError::InvalidMinimumRmsDbfs)
        );
        assert_eq!(
            SessionSourceSignalPolicy::new(-40.0, -60.0, Duration::ZERO),
            Err(SessionSourceSignalPolicyError::InvalidExactZeroTimeout)
        );
    }

    #[test]
    fn given_source_activity_when_evaluated_then_first_frame_and_stall_states_are_distinct() {
        let policy =
            SessionSourceActivityPolicy::new(Duration::from_nanos(100), Duration::from_nanos(20))
                .expect("finite non-zero activity policy");
        let awaiting = SessionSourceActivityObservations {
            session_started_at_ns: 100,
            observed_at_ns: 199,
            first_frame_received_at_ns: None,
            latest_frame_received_at_ns: None,
            frames_received_total: 0,
        }
        .evaluate(policy);
        assert_eq!(
            awaiting.state,
            SessionSourceActivityState::AwaitingFirstFrame
        );
        assert_eq!(awaiting.session_age_ns, 99);
        assert_eq!(awaiting.latest_frame_age_ns, None);

        let timed_out = SessionSourceActivityObservations {
            observed_at_ns: 200,
            ..SessionSourceActivityObservations {
                session_started_at_ns: 100,
                observed_at_ns: 0,
                first_frame_received_at_ns: None,
                latest_frame_received_at_ns: None,
                frames_received_total: 0,
            }
        }
        .evaluate(policy);
        assert_eq!(
            timed_out.state,
            SessionSourceActivityState::FirstFrameTimedOut
        );

        let active_observations = SessionSourceActivityObservations {
            session_started_at_ns: 100,
            observed_at_ns: 209,
            first_frame_received_at_ns: Some(150),
            latest_frame_received_at_ns: Some(190),
            frames_received_total: 3,
        };
        let active = active_observations.evaluate(policy);
        assert_eq!(active.state, SessionSourceActivityState::Active);
        assert_eq!(active.latest_frame_age_ns, Some(19));

        let stalled = SessionSourceActivityObservations {
            observed_at_ns: 210,
            ..active_observations
        }
        .evaluate(policy);
        assert_eq!(stalled.state, SessionSourceActivityState::Stalled);
        assert_eq!(stalled.latest_frame_age_ns, Some(20));
    }

    #[test]
    fn given_zero_or_unrepresentable_activity_deadline_when_constructed_then_policy_is_rejected() {
        assert_eq!(
            SessionSourceActivityPolicy::new(Duration::ZERO, Duration::from_secs(1)),
            Err(SessionSourceActivityPolicyError::InvalidFirstFrameTimeout)
        );
        assert_eq!(
            SessionSourceActivityPolicy::new(Duration::from_secs(1), Duration::ZERO),
            Err(SessionSourceActivityPolicyError::InvalidStallTimeout)
        );
        assert_eq!(
            SessionSourceActivityPolicy::new(Duration::MAX, Duration::from_secs(1)),
            Err(SessionSourceActivityPolicyError::InvalidFirstFrameTimeout)
        );
    }

    #[test]
    fn given_route_snapshot_when_drop_observed_then_rate_has_explicit_denominator_and_reasons() {
        let route = SessionRouteMetrics {
            route_id: RouteId(7),
            endpoint_id: EndpointId(8),
            edge: EdgeObservations {
                frames_enqueued_total: 3,
                frames_dropped_total: 1,
                queue_full_drops_total: 1,
                ..EdgeObservations::default()
            },
            endpoint: None,
            endpoint_observation_stage: EndpointObservationStage::Unavailable,
            endpoint_finalization_failures_total: 0,
        };

        let drops = route.drop_observations();
        assert_eq!(drops.route_id, RouteId(7));
        assert_eq!(drops.frames_dropped_total, 1);
        assert_eq!(drops.frames_attempted_total, 4);
        assert_eq!(drops.queue_full_drops_total, 1);
        assert_eq!(
            drops.interval,
            SessionRouteObservationInterval::RouteLifetimeToSnapshot
        );
        assert!((drops.drop_rate_pct() - 25.0).abs() < f64::EPSILON);
    }

    #[test]
    fn given_route_snapshot_when_latency_observed_then_boundary_units_and_coverage_are_explicit() {
        let route = SessionRouteMetrics {
            route_id: RouteId(7),
            endpoint_id: EndpointId(8),
            edge: EdgeObservations {
                source_timestamp_to_receive_samples_total: 9,
                source_timestamp_to_receive_missing_total: 2,
                source_timestamp_to_receive_future_total: 1,
                source_timestamp_to_receive_p95_ns: 42,
                ..EdgeObservations::default()
            },
            endpoint: None,
            endpoint_observation_stage: EndpointObservationStage::Unavailable,
            endpoint_finalization_failures_total: 0,
        };

        let latency = route.source_to_receive_latency();
        assert_eq!(latency.route_id, RouteId(7));
        assert_eq!(
            latency.measurement,
            RouteLatencyMeasurement::SourceMonotonicTimestampToRouteReceive
        );
        assert_eq!(
            latency.measurement(),
            RouteLatencyMeasurement::SourceMonotonicTimestampToRouteReceive
        );
        assert_eq!(latency.unit, SessionRouteLatencyUnit::Nanoseconds);
        assert_eq!(latency.samples_total, 9);
        assert_eq!(latency.missing_or_incompatible_clock_total, 2);
        assert_eq!(latency.future_timestamp_total, 1);
        assert_eq!(latency.p95_ns, 42);
    }
}
