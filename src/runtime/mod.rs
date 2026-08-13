//! Runtime implementation for compiled Session plans.
//!
//! The realtime audio lane and asynchronous signal lane remain separate. Every
//! partition crossing is explicit and bounded; lifecycle code never executes
//! inside an audio callback.

mod audio;
mod bridge;
mod lifecycle;
#[cfg(feature = "internal-testing")]
pub mod nodes;
mod signal;

pub use audio::*;
pub use bridge::*;
pub use lifecycle::*;
pub use signal::*;
