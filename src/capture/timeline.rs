//! Source-sample time mapped into PocketStation's monotonic clock domain.

#[cfg(any(
    test,
    feature = "internal-testing",
    all(target_os = "macos", feature = "coreaudio-capture"),
    all(
        target_os = "linux",
        any(feature = "pipewire-capture", feature = "alsa-fallback")
    )
))]
use std::num::NonZeroU32;

/// Initializes the process-wide capture timestamp domain from a setup thread.
///
/// Capture backends call this before starting a realtime callback so the
/// callback only reads the initialized monotonic origin.
#[cfg(any(
    test,
    feature = "internal-testing",
    all(target_os = "macos", feature = "coreaudio-capture"),
    all(target_os = "windows", feature = "wasapi-capture"),
    all(
        target_os = "linux",
        any(feature = "pipewire-capture", feature = "alsa-fallback")
    )
))]
pub fn initialize_monotonic_timestamp_domain() {
    let _ = crate::timing::monotonic_timestamp_ns();
}

/// Process-wide monotonic timestamp domain used by every capture adapter.
/// The value is non-zero and comparable across PocketStation crates in the
/// same process; it is never derived from a wall clock and cannot jump.
pub fn monotonic_timestamp_ns() -> u64 {
    crate::timing::monotonic_timestamp_ns()
}

/// Source-time clock for capture streams whose media cadence is defined by
/// the number of sample frames produced by the device.
///
/// Callback arrival time is scheduler time, not audio presentation time. This
/// clock anchors the first observed sample frame to the process monotonic clock
/// and advances only by represented sample count. Callers must advance it even
/// when a captured buffer is dropped so downstream gaps remain observable.
#[derive(Debug)]
#[cfg(any(
    test,
    feature = "internal-testing",
    all(target_os = "macos", feature = "coreaudio-capture"),
    all(
        target_os = "linux",
        any(feature = "pipewire-capture", feature = "alsa-fallback")
    )
))]
pub struct CaptureSampleTimeline {
    sample_rate_hz: NonZeroU32,
    origin_timestamp_ns: Option<u64>,
    elapsed_sample_frames: u64,
    source_origin_sample_frame: Option<u64>,
    next_source_sample_frame: Option<u64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg(any(
    test,
    feature = "internal-testing",
    all(target_os = "macos", feature = "coreaudio-capture"),
    all(
        target_os = "linux",
        any(feature = "pipewire-capture", feature = "alsa-fallback")
    )
))]
pub enum CaptureSampleTimelineError {
    MixedAdvanceModes,
    SourcePositionOverflow,
    SourcePositionMovedBackward {
        expected_at_least: u64,
        observed: u64,
    },
}

#[cfg(any(
    test,
    feature = "internal-testing",
    all(target_os = "macos", feature = "coreaudio-capture"),
    all(
        target_os = "linux",
        any(feature = "pipewire-capture", feature = "alsa-fallback")
    )
))]
impl CaptureSampleTimeline {
    pub fn new(sample_rate_hz: NonZeroU32) -> Self {
        Self {
            sample_rate_hz,
            origin_timestamp_ns: None,
            elapsed_sample_frames: 0,
            source_origin_sample_frame: None,
            next_source_sample_frame: None,
        }
    }

    pub fn anchored(sample_rate_hz: NonZeroU32, origin_timestamp_ns: u64) -> Self {
        Self {
            sample_rate_hz,
            origin_timestamp_ns: Some(origin_timestamp_ns.max(1)),
            elapsed_sample_frames: 0,
            source_origin_sample_frame: None,
            next_source_sample_frame: None,
        }
    }

    /// Returns this buffer's source-time start and advances the next start.
    #[cfg(any(test, target_os = "linux", target_os = "macos"))]
    pub fn advance(&mut self, sample_frames: u64) -> u64 {
        let origin_timestamp_ns = *self
            .origin_timestamp_ns
            .get_or_insert_with(monotonic_timestamp_ns);
        let elapsed_ns = u128::from(self.elapsed_sample_frames)
            .saturating_mul(1_000_000_000)
            .checked_div(u128::from(self.sample_rate_hz.get()))
            .unwrap_or(0)
            .min(u128::from(u64::MAX)) as u64;
        self.elapsed_sample_frames = self.elapsed_sample_frames.saturating_add(sample_frames);
        origin_timestamp_ns.saturating_add(elapsed_ns)
    }

    /// Returns a buffer's source-time start from its native sample-frame
    /// position. Forward gaps are preserved in the returned timestamp without
    /// separately advancing this clock from an aggregate drop counter.
    pub fn advance_from_source_position(
        &mut self,
        source_sample_frame: u64,
        sample_frames: u64,
    ) -> Result<u64, CaptureSampleTimelineError> {
        if self.elapsed_sample_frames != 0 {
            return Err(CaptureSampleTimelineError::MixedAdvanceModes);
        }
        let next_source_sample_frame = source_sample_frame
            .checked_add(sample_frames)
            .ok_or(CaptureSampleTimelineError::SourcePositionOverflow)?;
        if let Some(expected_at_least) = self.next_source_sample_frame {
            if source_sample_frame < expected_at_least {
                return Err(CaptureSampleTimelineError::SourcePositionMovedBackward {
                    expected_at_least,
                    observed: source_sample_frame,
                });
            }
        }
        let source_origin_sample_frame = *self
            .source_origin_sample_frame
            .get_or_insert(source_sample_frame);
        let elapsed_sample_frames = source_sample_frame
            .checked_sub(source_origin_sample_frame)
            .ok_or(CaptureSampleTimelineError::SourcePositionMovedBackward {
                expected_at_least: source_origin_sample_frame,
                observed: source_sample_frame,
            })?;
        let origin_timestamp_ns = *self
            .origin_timestamp_ns
            .get_or_insert_with(monotonic_timestamp_ns);
        let elapsed_ns = u128::from(elapsed_sample_frames)
            .saturating_mul(1_000_000_000)
            .checked_div(u128::from(self.sample_rate_hz.get()))
            .unwrap_or(0)
            .min(u128::from(u64::MAX)) as u64;
        self.next_source_sample_frame = Some(next_source_sample_frame);
        Ok(origin_timestamp_ns.saturating_add(elapsed_ns))
    }
}
