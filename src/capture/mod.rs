//! Desktop capture contracts and bounded callback delivery.
//!
//! Platform code owns OS integration, `frame_stream` owns the bounded SPSC
//! crossing, and `capture_owner` owns non-realtime lifecycle. Public capture
//! declarations are data contracts only.
//!
//! Every native callback follows the same permanent contract: acquire from a
//! fixed-capacity pool, write through non-panicking fixed-slot APIs, publish
//! with a nonblocking bounded send, and account for rejection. Callback code
//! must not allocate, lock, block, await, log, or unwind.

mod authorization;
mod capture_owner;
mod events;
mod frame_stream;
mod identity;
#[cfg(feature = "internal-testing")]
mod lifecycle_registry;
mod observations;
pub(crate) mod platform;
mod query;
mod selection;
mod timeline;

pub use authorization::*;
#[cfg(any(test, feature = "internal-testing", feature = "native-capture"))]
pub use capture_owner::{
    join_capture_worker, prepare_capture, CaptureOpenMetadata, PreparedCapture,
    CAPTURE_MONOTONIC_CLOCK_DOMAIN_ID,
};
pub use capture_owner::{
    prepare_capture_with_start_gate, ActiveCaptureBackend, CallbackCaptureBackend, CaptureDelivery,
    CaptureLineageSeed, CaptureObservationReceipt, CaptureOwner, CaptureOwnerObservations,
    CapturePrepareRequest, CaptureStopOutcome, PreparedCaptureBackend,
};
pub use events::*;
pub use frame_stream::{
    capture_delivery_start_gate, CaptureDeliveryStartGate, CapturedFrameDelivery,
    CapturedFrameObservationHandle, CapturedFrameSender, CapturedFrameStream,
    CapturedFrameStreamStats,
};
#[cfg(any(test, feature = "internal-testing"))]
pub use frame_stream::{
    captured_frame_stream, captured_frame_stream_with_start_gate,
    CaptureDeliveryStartGateController,
};
pub use identity::*;
#[cfg(feature = "internal-testing")]
pub use lifecycle_registry::*;
#[cfg(any(test, feature = "internal-testing", feature = "native-capture"))]
pub use observations::CaptureObservationCounters;
pub use observations::{CaptureObservationHandle, CaptureObservations};
pub use query::*;
pub use selection::*;
#[cfg(any(test, feature = "internal-testing", feature = "native-capture"))]
pub use timeline::CaptureSampleTimelineError;
#[cfg(any(test, feature = "internal-testing", feature = "native-capture"))]
pub use timeline::{
    initialize_monotonic_timestamp_domain, monotonic_timestamp_ns, CaptureSampleTimeline,
};

#[cfg(test)]
mod tests;
