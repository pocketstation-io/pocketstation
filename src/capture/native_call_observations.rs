//! Opt-in observations of synchronous native capture control calls.
//!
//! These records contain no capture data or native object identity. A current
//! call is published before entering the native operation and stays visible
//! while that operation is blocked. Observations do not impose a timeout.

#[cfg(any(test, all(target_os = "macos", feature = "coreaudio-capture")))]
use std::cell::Cell;
#[cfg(any(test, all(target_os = "macos", feature = "coreaudio-capture")))]
use std::marker::PhantomData;
use std::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, Ordering};
use std::sync::Arc;

use crate::timing::monotonic_timestamp_ns;

/// Maximum independently observed capture opens on one cancellation token.
pub const NATIVE_CALL_CURRENT_CAPACITY: usize = 64;
/// First completed calls retained by one observed cancellation token.
pub const NATIVE_CALL_COMPLETED_CAPACITY: usize = 128;
const SNAPSHOT_ATTEMPTS: usize = 3;

/// Static control-call names; values never contain source or device identity.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum NativeCallOperation {
    SourceDeviceDiscovery = 1,
    DeviceListSize,
    DeviceListData,
    DeviceStreamsSize,
    DeviceStreamsData,
    DeviceUid,
    DeviceName,
    DeviceSampleRate,
    ProcessListSize,
    ProcessListData,
    ProcessId,
    ProcessBundleId,
    ProcessRunningOutput,
    ProcessStartTime,
    ProcessName,
    ApplicationLabel,
    ResolveProcessListSize,
    ResolveProcessListData,
    ResolveProcessId,
    CreateProcessTap,
    TapUid,
    CreateAggregateDevice,
    StreamFormat,
    IoBufferSizeBefore,
    IoBufferSizeRange,
    IoBufferSizeSettable,
    IoBufferSizeSet,
    IoBufferSizeApplied,
    RegisterIoProc,
    StartDevice,
    StopDevice,
    UnregisterIoProc,
    DestroyAggregateDevice,
    DestroyProcessTap,
    DeviceLatency,
    SafetyOffset,
    SafetyOffsetSettable,
    StreamListSize,
    StreamListData,
    StreamLatency,
    GlobalTapDescription,
    ProcessTapDescription,
    TapMuteBehavior,
}

impl NativeCallOperation {
    pub(crate) fn from_code(code: u32) -> Option<Self> {
        use NativeCallOperation::*;
        const VALUES: [NativeCallOperation; 43] = [
            SourceDeviceDiscovery,
            DeviceListSize,
            DeviceListData,
            DeviceStreamsSize,
            DeviceStreamsData,
            DeviceUid,
            DeviceName,
            DeviceSampleRate,
            ProcessListSize,
            ProcessListData,
            ProcessId,
            ProcessBundleId,
            ProcessRunningOutput,
            ProcessStartTime,
            ProcessName,
            ApplicationLabel,
            ResolveProcessListSize,
            ResolveProcessListData,
            ResolveProcessId,
            CreateProcessTap,
            TapUid,
            CreateAggregateDevice,
            StreamFormat,
            IoBufferSizeBefore,
            IoBufferSizeRange,
            IoBufferSizeSettable,
            IoBufferSizeSet,
            IoBufferSizeApplied,
            RegisterIoProc,
            StartDevice,
            StopDevice,
            UnregisterIoProc,
            DestroyAggregateDevice,
            DestroyProcessTap,
            DeviceLatency,
            SafetyOffset,
            SafetyOffsetSettable,
            StreamListSize,
            StreamListData,
            StreamLatency,
            GlobalTapDescription,
            ProcessTapDescription,
            TapMuteBehavior,
        ];
        code.checked_sub(1)
            .and_then(|index| VALUES.get(index as usize))
            .copied()
    }
}

/// One native-call fact in the process monotonic nanosecond domain.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NativeCallSpan {
    /// Ordinal within this observed token, never a native handle or source ID.
    pub open_ordinal: u64,
    /// Unique call ordinal within this observed token.
    pub call_ordinal: u64,
    pub operation: NativeCallOperation,
    pub started_at_ns: u64,
    /// Absent until the call returned; absence does not establish a hang.
    pub returned_at_ns: Option<u64>,
    /// Signed return status when present. HAL leaves expose raw OSStatus.
    /// RegisterIoProc exposes the existing control result, which normalizes
    /// zero plus a missing IOProc to an unspecified error. A zero result does
    /// not itself prove ownership or cleanup. Non-status APIs leave it absent.
    pub status_code: Option<i32>,
}

/// Bounded, independently coherent call records, rather than a global instant.
///
/// Concurrent calls can return while this snapshot is being read. A call may
/// appear in both arrays across that transition. No record combines fields
/// from separate publications. Busy entries are omitted from this read only.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NativeCallObservations {
    pub observed_at_ns: u64,
    pub current: [Option<NativeCallSpan>; NATIVE_CALL_CURRENT_CAPACITY],
    pub completed: [Option<NativeCallSpan>; NATIVE_CALL_COMPLETED_CAPACITY],
    pub busy_slots: u32,
    pub omitted_opens_total: u64,
    pub omitted_calls_total: u64,
    pub completed_calls_total: u64,
    /// Completed calls beyond the first128 entries are counted, not retained.
    pub truncated_completed_total: u64,
    /// A bounded counter update was omitted after contention or u64 exhaustion.
    pub counters_inexact: bool,
}

#[derive(Default, Debug)]
struct AtomicSpan {
    sequence: AtomicU64,
    open: AtomicU64,
    call: AtomicU64,
    operation: AtomicU32,
    started: AtomicU64,
    returned: AtomicU64,
    status: AtomicU32,
    has_status: AtomicU32,
}

impl AtomicSpan {
    #[cfg(any(test, all(target_os = "macos", feature = "coreaudio-capture")))]
    fn publish(&self, value: Option<NativeCallSpan>) {
        self.sequence.fetch_add(1, Ordering::SeqCst);
        if let Some(span) = value {
            self.open.store(span.open_ordinal, Ordering::SeqCst);
            self.operation
                .store(span.operation as u32, Ordering::SeqCst);
            self.started.store(span.started_at_ns, Ordering::SeqCst);
            self.returned
                .store(span.returned_at_ns.unwrap_or(0), Ordering::SeqCst);
            self.status
                .store(span.status_code.unwrap_or(0) as u32, Ordering::SeqCst);
            self.has_status
                .store(u32::from(span.status_code.is_some()), Ordering::SeqCst);
            self.call.store(span.call_ordinal, Ordering::SeqCst);
        } else {
            self.call.store(0, Ordering::SeqCst);
        }
        self.sequence.fetch_add(1, Ordering::SeqCst);
    }

    fn snapshot(&self) -> Result<Option<NativeCallSpan>, ()> {
        for _ in 0..SNAPSHOT_ATTEMPTS {
            let before = self.sequence.load(Ordering::SeqCst);
            if before & 1 != 0 {
                continue;
            }
            let call_ordinal = self.call.load(Ordering::SeqCst);
            let open_ordinal = self.open.load(Ordering::SeqCst);
            let operation = self.operation.load(Ordering::SeqCst);
            let started_at_ns = self.started.load(Ordering::SeqCst);
            let returned = self.returned.load(Ordering::SeqCst);
            let status = self.status.load(Ordering::SeqCst) as i32;
            let has_status = self.has_status.load(Ordering::SeqCst) != 0;
            if before != self.sequence.load(Ordering::SeqCst) {
                continue;
            }
            if call_ordinal == 0 {
                return Ok(None);
            }
            let Some(operation) = NativeCallOperation::from_code(operation) else {
                return Err(());
            };
            return Ok(Some(NativeCallSpan {
                open_ordinal,
                call_ordinal,
                operation,
                started_at_ns,
                returned_at_ns: (returned != 0).then_some(returned),
                status_code: has_status.then_some(status),
            }));
        }
        Err(())
    }
}

#[derive(Default, Debug)]
struct CurrentSlot {
    #[cfg(any(test, all(target_os = "macos", feature = "coreaudio-capture")))]
    owner: AtomicU64,
    span: AtomicSpan,
}

/// Shared read state. Mutable reporter ownership is never cloned.
#[derive(Debug)]
pub(crate) struct NativeCallObservationState {
    #[cfg(any(test, all(target_os = "macos", feature = "coreaudio-capture")))]
    next_open: AtomicU64,
    #[cfg(any(test, all(target_os = "macos", feature = "coreaudio-capture")))]
    next_call: AtomicU64,
    current: [CurrentSlot; NATIVE_CALL_CURRENT_CAPACITY],
    completed: [AtomicSpan; NATIVE_CALL_COMPLETED_CAPACITY],
    omitted_opens: AtomicU64,
    omitted_calls: AtomicU64,
    completed_count: AtomicU64,
    counters_inexact: AtomicBool,
}

impl NativeCallObservationState {
    pub(crate) fn new() -> Arc<Self> {
        // Initialize the clock on the explicit opt-in control path.
        monotonic_timestamp_ns();
        Arc::new(Self {
            #[cfg(any(test, all(target_os = "macos", feature = "coreaudio-capture")))]
            next_open: AtomicU64::new(1),
            #[cfg(any(test, all(target_os = "macos", feature = "coreaudio-capture")))]
            next_call: AtomicU64::new(1),
            current: std::array::from_fn(|_| CurrentSlot::default()),
            completed: std::array::from_fn(|_| AtomicSpan::default()),
            omitted_opens: AtomicU64::new(0),
            omitted_calls: AtomicU64::new(0),
            completed_count: AtomicU64::new(0),
            counters_inexact: AtomicBool::new(false),
        })
    }

    #[cfg(any(test, all(target_os = "macos", feature = "coreaudio-capture")))]
    fn ordinal(counter: &AtomicU64) -> Option<u64> {
        // Contention or exhaustion loses diagnostics only, never capture.
        for _ in 0..4 {
            let value = counter.load(Ordering::SeqCst);
            if value == u64::MAX {
                return None;
            }
            if counter
                .compare_exchange(value, value + 1, Ordering::SeqCst, Ordering::SeqCst)
                .is_ok()
            {
                return Some(value);
            }
        }
        None
    }

    #[cfg(any(test, all(target_os = "macos", feature = "coreaudio-capture")))]
    fn increment(&self, counter: &AtomicU64) -> Option<u64> {
        let result = Self::ordinal(counter);
        if result.is_none() {
            self.counters_inexact.store(true, Ordering::Release);
        }
        result
    }

    #[cfg(any(test, all(target_os = "macos", feature = "coreaudio-capture")))]
    pub(crate) fn reporter(self: &Arc<Self>) -> Option<NativeCallReporter> {
        if let Some(open) = self.increment(&self.next_open) {
            for (index, slot) in self.current.iter().enumerate() {
                if slot
                    .owner
                    .compare_exchange(0, open, Ordering::Acquire, Ordering::Relaxed)
                    .is_ok()
                {
                    return Some(NativeCallReporter {
                        state: Arc::clone(self),
                        index,
                        open,
                        single_writer: PhantomData,
                    });
                }
            }
        }
        self.increment(&self.omitted_opens);
        None
    }

    pub(crate) fn snapshot(&self) -> NativeCallObservations {
        let mut result = NativeCallObservations {
            observed_at_ns: 0,
            current: [None; NATIVE_CALL_CURRENT_CAPACITY],
            completed: [None; NATIVE_CALL_COMPLETED_CAPACITY],
            busy_slots: 0,
            omitted_opens_total: self.omitted_opens.load(Ordering::Relaxed),
            omitted_calls_total: self.omitted_calls.load(Ordering::Relaxed),
            completed_calls_total: self.completed_count.load(Ordering::Acquire),
            truncated_completed_total: 0,
            counters_inexact: false,
        };
        for (source, target) in self
            .current
            .iter()
            .map(|slot| &slot.span)
            .zip(result.current.iter_mut())
        {
            match source.snapshot() {
                Ok(span) => *target = span,
                Err(()) => result.busy_slots += 1,
            }
        }
        let mut completed_busy = [false; NATIVE_CALL_COMPLETED_CAPACITY];
        for (index, (source, target)) in self
            .completed
            .iter()
            .zip(result.completed.iter_mut())
            .enumerate()
        {
            match source.snapshot() {
                Ok(span) => *target = span,
                Err(()) => {
                    result.busy_slots += 1;
                    completed_busy[index] = true;
                }
            }
        }
        // Reservations precede publication; reload after records so a newly
        // retained entry cannot be paired with an earlier zero total.
        result.completed_calls_total = self.completed_count.load(Ordering::SeqCst);
        for (index, span) in result.completed.iter().enumerate() {
            if (index as u64) < result.completed_calls_total
                && span.is_none()
                && !completed_busy[index]
            {
                result.busy_slots += 1;
            }
        }
        result.omitted_opens_total = self.omitted_opens.load(Ordering::SeqCst);
        result.omitted_calls_total = self.omitted_calls.load(Ordering::SeqCst);
        result.counters_inexact = self.counters_inexact.load(Ordering::Acquire);
        result.truncated_completed_total = result
            .completed_calls_total
            .saturating_sub(NATIVE_CALL_COMPLETED_CAPACITY as u64);
        result.observed_at_ns = monotonic_timestamp_ns();
        result
    }
}

/// One exclusive control-thread reporter, movable with its capture owner.
#[cfg(any(test, all(target_os = "macos", feature = "coreaudio-capture")))]
pub(crate) struct NativeCallReporter {
    state: Arc<NativeCallObservationState>,
    index: usize,
    open: u64,
    // Movable Send, but not Sync: scoped native callbacks have one writer.
    single_writer: PhantomData<Cell<()>>,
}

#[cfg(any(test, all(target_os = "macos", feature = "coreaudio-capture")))]
impl NativeCallReporter {
    pub(crate) fn begin(&self, operation: NativeCallOperation) -> u64 {
        let slot = &self.state.current[self.index].span;
        // A nested marker cannot overwrite the current blocked outer call.
        if slot.call.load(Ordering::Relaxed) != 0 {
            self.state.increment(&self.state.omitted_calls);
            return 0;
        }
        let Some(call) = self.state.increment(&self.state.next_call) else {
            self.state.increment(&self.state.omitted_calls);
            return 0;
        };
        slot.publish(Some(NativeCallSpan {
            open_ordinal: self.open,
            call_ordinal: call,
            operation,
            started_at_ns: monotonic_timestamp_ns(),
            returned_at_ns: None,
            status_code: None,
        }));
        call
    }

    pub(crate) fn end(&self, call: u64, status: Option<i32>) {
        if call == 0 {
            return;
        }
        let slot = &self.state.current[self.index].span;
        let Ok(Some(mut span)) = slot.snapshot() else {
            return;
        };
        if span.call_ordinal != call {
            return;
        }
        span.returned_at_ns = Some(monotonic_timestamp_ns());
        span.status_code = status;
        if let Some(entry) = self.state.increment(&self.state.completed_count) {
            if let Ok(index) = usize::try_from(entry) {
                if let Some(target) = self.state.completed.get(index) {
                    target.publish(Some(span));
                }
            }
        }
        slot.publish(None);
    }
}

#[cfg(any(test, all(target_os = "macos", feature = "coreaudio-capture")))]
impl Drop for NativeCallReporter {
    fn drop(&mut self) {
        self.state.current[self.index].span.publish(None);
        self.state.current[self.index]
            .owner
            .store(0, Ordering::Release);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_independent_open_reporters_when_calls_overlap_then_both_current_calls_remain_visible()
    {
        let state = NativeCallObservationState::new();
        let first = state.reporter().unwrap();
        let second = state.reporter().unwrap();
        let a = first.begin(NativeCallOperation::StopDevice);
        let b = second.begin(NativeCallOperation::RegisterIoProc);
        let snapshot = state.snapshot();
        assert_eq!(snapshot.current.iter().flatten().count(), 2);
        assert_ne!(
            snapshot.current[0].unwrap().open_ordinal,
            snapshot.current[1].unwrap().open_ordinal
        );
        first.end(a, Some(-1));
        second.end(b, Some(0));
        assert_eq!(state.snapshot().completed.iter().flatten().count(), 2);
    }

    #[test]
    fn given_full_completed_history_when_more_calls_return_then_current_call_and_explicit_truncation_remain(
    ) {
        let state = NativeCallObservationState::new();
        let reporter = state.reporter().unwrap();
        for _ in 0..130 {
            let call = reporter.begin(NativeCallOperation::ProcessId);
            reporter.end(call, Some(0));
        }
        let call = reporter.begin(NativeCallOperation::StartDevice);
        let snapshot = state.snapshot();
        assert_eq!(snapshot.completed.iter().flatten().count(), 128);
        assert_eq!(snapshot.truncated_completed_total, 2);
        assert_eq!(snapshot.current[0].unwrap().call_ordinal, call);
        reporter.end(call, Some(0));
    }

    #[test]
    fn given_full_current_slots_when_an_open_is_omitted_then_capture_admission_is_not_involved() {
        let state = NativeCallObservationState::new();
        let reporters = (0..64)
            .map(|_| state.reporter().unwrap())
            .collect::<Vec<_>>();
        assert!(state.reporter().is_none());
        assert_eq!(state.snapshot().omitted_opens_total, 1);
        drop(reporters);
        assert!(state.reporter().is_some());
    }

    #[test]
    fn given_busy_publication_when_snapshot_is_read_then_read_is_bounded_and_marks_busy() {
        let state = NativeCallObservationState::new();
        state.current[0].span.sequence.store(1, Ordering::Release);
        let snapshot = state.snapshot();
        assert_eq!(snapshot.busy_slots, 1);
        assert_eq!(snapshot.current[0], None);
    }

    #[test]
    fn given_nested_marker_when_outer_call_is_pending_then_only_outer_identity_is_retained() {
        let state = NativeCallObservationState::new();
        let reporter = state.reporter().unwrap();
        let call = reporter.begin(NativeCallOperation::StartDevice);
        let nested = reporter.begin(NativeCallOperation::ProcessName);
        assert_eq!(nested, 0);
        reporter.end(nested, None);
        let snapshot = state.snapshot();
        assert_eq!(snapshot.current[0].unwrap().call_ordinal, call);
        assert_eq!(snapshot.omitted_calls_total, 1);
        reporter.end(call, Some(-19));
        assert_eq!(
            state.snapshot().completed[0].unwrap().status_code,
            Some(-19)
        );
    }

    #[test]
    fn given_exhausted_diagnostic_counter_when_call_returns_then_count_saturates_without_affecting_owner(
    ) {
        let state = NativeCallObservationState::new();
        let reporter = state.reporter().unwrap();
        state.completed_count.store(u64::MAX, Ordering::SeqCst);
        let call = reporter.begin(NativeCallOperation::ProcessName);
        reporter.end(call, None);
        let snapshot = state.snapshot();
        assert_eq!(snapshot.completed_calls_total, u64::MAX);
        assert!(snapshot.counters_inexact);
        assert!(snapshot.current.iter().all(Option::is_none));
        drop(reporter);
        assert!(state.reporter().is_some());
    }

    #[test]
    fn given_concurrent_control_writer_when_snapshots_race_then_each_retained_span_has_one_publication(
    ) {
        let state = NativeCallObservationState::new();
        let reporter = state.reporter().unwrap();
        let barrier = Arc::new(std::sync::Barrier::new(2));
        let writer_barrier = Arc::clone(&barrier);
        let worker = std::thread::spawn(move || {
            writer_barrier.wait();
            for _ in 0..256 {
                let call = reporter.begin(NativeCallOperation::StartDevice);
                reporter.end(call, Some(-700));
            }
        });
        barrier.wait();
        for _ in 0..256 {
            let snapshot = state.snapshot();
            for span in snapshot
                .current
                .iter()
                .chain(snapshot.completed.iter())
                .flatten()
            {
                assert_eq!(span.open_ordinal, 1);
                assert_eq!(span.operation, NativeCallOperation::StartDevice);
                assert!(span.call_ordinal > 0);
                assert!(span.started_at_ns > 0);
                match span.returned_at_ns {
                    None => assert_eq!(span.status_code, None),
                    Some(returned) => {
                        assert!(returned >= span.started_at_ns);
                        assert_eq!(span.status_code, Some(-700));
                    }
                }
            }
        }
        worker.join().unwrap();
        assert_eq!(state.snapshot().completed_calls_total, 256);
    }
}
