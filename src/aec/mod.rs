//! Optional echo processing; Session composition remains available in lean builds.
#[cfg(feature = "aec")]
mod config;
#[cfg(feature = "aec")]
mod factory;
mod observations;
#[cfg(feature = "aec")]
mod processor;
#[cfg(feature = "aec")]
mod worker;

#[cfg(feature = "aec")]
pub(crate) use config::{AecConfiguration, Channels};
#[cfg(feature = "aec")]
pub(crate) use factory::AecOperatorFactory;
pub(crate) use observations::ObservationState;
pub use observations::{EchoCancellationObservations, EchoCancellationState};

/// Whether this build includes the AEC engine. This does not qualify a device route.
pub const fn aec_available() -> bool {
    cfg!(feature = "aec")
}
