use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

const FACT_BITS: u32 = 6;
const MAX_REVISION: u64 = u64::MAX >> FACT_BITS;

/// Processing facts reported by the backend for one opened capture stream.
///
/// `None` means unobserved, never false. Support does not imply that an effect
/// is active, that its playback reference is suitable, or that it removes echo.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CaptureProcessingObservations {
    pub native_aec_supported: Option<bool>,
    pub echo_processed: Option<bool>,
    /// Whether this backend can also supply input before echo processing.
    /// This does not assert that the delivered input is unprocessed.
    pub raw_audio_available: Option<bool>,
}

/// Read-only processing observations for one opened stream.
/// Reads belong on control threads, never native audio callbacks.
#[derive(Clone, Debug)]
pub struct CaptureProcessingObservationHandle {
    state: Arc<AtomicU64>,
}

impl CaptureProcessingObservationHandle {
    pub fn observations(&self) -> CaptureProcessingObservations {
        self.admission_snapshot().0
    }

    /// Reads stream facts and their invalidation history in one atomic load.
    /// Saturation returns a permanently unsafe revision of `u64::MAX`.
    pub(crate) fn admission_snapshot(&self) -> (CaptureProcessingObservations, u64) {
        let state = self.state.load(Ordering::Acquire);
        let revision = state >> FACT_BITS;
        (
            decode_observations(state),
            if revision == MAX_REVISION {
                u64::MAX
            } else {
                revision
            },
        )
    }
}

/// Backend-owned writer for stream processing facts.
///
/// Publish complete observations on the backend control thread. Before a route
/// or effect change invalidates known facts, replace them with unknown values.
/// A writer belongs to one open; never reuse it for another capture.
#[derive(Clone, Debug)]
pub struct CaptureProcessingReporter {
    state: Arc<AtomicU64>,
}

impl CaptureProcessingReporter {
    pub fn replace(&self, observations: CaptureProcessingObservations) {
        let facts = encode_observations(observations);
        // The closure always supplies a value: contention retries on the
        // control thread. No reported revision can wrap or erase an unsafe
        // transition merely because the effect is later reported off.
        let _ = self
            .state
            .fetch_update(Ordering::AcqRel, Ordering::Acquire, |current| {
                let previous = decode_observations(current);
                let revision = current >> FACT_BITS;
                let invalidated = observations.echo_processed != previous.echo_processed
                    && observations.echo_processed != Some(false);
                let next_revision = if invalidated {
                    revision.saturating_add(1).min(MAX_REVISION)
                } else {
                    revision
                };
                Some((next_revision << FACT_BITS) | facts)
            });
    }
}

fn encode_fact(fact: Option<bool>) -> u64 {
    match fact {
        None => 0,
        Some(false) => 1,
        Some(true) => 2,
    }
}

fn decode_fact(encoded: u64) -> Option<bool> {
    match encoded & 3 {
        1 => Some(false),
        2 => Some(true),
        _ => None,
    }
}

fn encode_observations(observations: CaptureProcessingObservations) -> u64 {
    encode_fact(observations.native_aec_supported)
        | (encode_fact(observations.echo_processed) << 2)
        | (encode_fact(observations.raw_audio_available) << 4)
}

fn decode_observations(encoded: u64) -> CaptureProcessingObservations {
    CaptureProcessingObservations {
        native_aec_supported: decode_fact(encoded),
        echo_processed: decode_fact(encoded >> 2),
        raw_audio_available: decode_fact(encoded >> 4),
    }
}

/// Creates an initially unknown observation pair for one capture open.
pub fn capture_processing_observations() -> (
    CaptureProcessingReporter,
    CaptureProcessingObservationHandle,
) {
    let state = Arc::new(AtomicU64::new(0));
    (
        CaptureProcessingReporter {
            state: Arc::clone(&state),
        },
        CaptureProcessingObservationHandle { state },
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn facts(value: Option<bool>) -> CaptureProcessingObservations {
        CaptureProcessingObservations {
            native_aec_supported: value,
            echo_processed: value,
            raw_audio_available: value,
        }
    }

    #[test]
    fn given_unobserved_stream_when_snapshot_read_then_input_is_not_declared_raw() {
        let (_, handle) = capture_processing_observations();
        assert_eq!(handle.observations(), facts(None));
        assert_ne!(handle.observations().echo_processed, Some(false));
        assert_eq!(handle.admission_snapshot().1, 0);
    }

    #[test]
    fn given_complete_facts_when_replaced_then_all_fields_change_together() {
        let (reporter, handle) = capture_processing_observations();
        for supported in [None, Some(false), Some(true)] {
            for processed in [None, Some(false), Some(true)] {
                for raw_available in [None, Some(false), Some(true)] {
                    let observations = CaptureProcessingObservations {
                        native_aec_supported: supported,
                        echo_processed: processed,
                        raw_audio_available: raw_available,
                    };
                    reporter.replace(observations);
                    assert_eq!(handle.observations(), observations);
                }
            }
        }
    }

    #[test]
    fn given_raw_input_when_effect_turns_on_then_later_raw_report_retains_invalidation() {
        let (reporter, handle) = capture_processing_observations();
        reporter.replace(facts(Some(false)));
        let admitted_revision = handle.admission_snapshot().1;
        reporter.replace(facts(Some(true)));
        let unsafe_revision = handle.admission_snapshot().1;
        reporter.replace(facts(Some(false)));
        let (current, revision) = handle.admission_snapshot();
        assert_eq!(current.echo_processed, Some(false));
        assert_eq!(unsafe_revision, admitted_revision + 1);
        assert_eq!(revision, unsafe_revision);
        reporter.replace(facts(Some(false)));
        assert_eq!(handle.admission_snapshot().1, revision);
    }

    #[test]
    fn given_raw_input_when_processing_becomes_unknown_then_admission_is_invalidated() {
        let (reporter, handle) = capture_processing_observations();
        reporter.replace(facts(Some(false)));
        let admitted_revision = handle.admission_snapshot().1;
        reporter.replace(facts(None));
        assert_eq!(
            handle.admission_snapshot(),
            (facts(None), admitted_revision + 1)
        );
        reporter.replace(facts(Some(false)));
        assert_eq!(handle.admission_snapshot().1, admitted_revision + 1);
    }

    #[test]
    fn given_concurrent_updates_when_snapshots_read_then_fact_sets_are_never_torn() {
        let (reporter, handle) = capture_processing_observations();
        let writer = std::thread::spawn(move || {
            for _ in 0..10_000 {
                reporter.replace(facts(Some(true)));
                reporter.replace(facts(Some(false)));
            }
        });
        for _ in 0..20_000 {
            let snapshot = handle.observations();
            assert_eq!(snapshot.native_aec_supported, snapshot.echo_processed);
            assert_eq!(snapshot.raw_audio_available, snapshot.echo_processed);
        }
        writer.join().expect("control-thread reporter completed");
        assert_eq!(handle.admission_snapshot(), (facts(Some(false)), 10_000));
    }

    #[test]
    fn given_revision_near_limit_when_invalidated_then_saturation_stays_permanently_unsafe() {
        let (reporter, handle) = capture_processing_observations();
        reporter.state.store(
            ((MAX_REVISION - 1) << FACT_BITS) | encode_observations(facts(Some(false))),
            Ordering::Release,
        );
        reporter.replace(facts(Some(true)));
        assert_eq!(handle.admission_snapshot().1, u64::MAX);
        reporter.replace(facts(Some(false)));
        assert_eq!(handle.admission_snapshot(), (facts(Some(false)), u64::MAX));
        reporter.replace(facts(None));
        assert_eq!(handle.admission_snapshot(), (facts(None), u64::MAX));
    }
}
