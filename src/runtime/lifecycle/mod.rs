//! Non-realtime runtime ownership and process-protocol lifecycle.

mod async_host;
mod sidecar_host;
mod sidecar_protocol;

pub use async_host::AsyncRuntimeHost;
#[cfg(any(test, feature = "internal-testing"))]
pub use async_host::AsyncRuntimeHostError;
#[cfg(test)]
pub(crate) use sidecar_host::CONTROL_SIGNAL_ID;
pub use sidecar_host::{
    SidecarDeadlines, SidecarHost, SidecarHostError, SidecarHostSnapshot, SidecarProcessSpec,
    SidecarState,
};
pub use sidecar_protocol::{
    SidecarMessage, SidecarMessageKind, SidecarProtocolError, SidecarProtocolLimits,
};
