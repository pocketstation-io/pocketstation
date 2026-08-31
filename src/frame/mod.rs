//! Realtime audio frame ownership.
//!
//! Platform and graph identities are intentionally separate from pooled audio
//! storage. The audio module remains specialized; arbitrary signals use the
//! asynchronous signal lane.

mod audio;
mod duration;
mod identity;
mod lineage;
mod output_generation;
mod platform;
mod pool;

pub use audio::*;
pub use duration::*;
pub use identity::*;
pub use lineage::*;
pub use output_generation::*;
pub use platform::Platform;
pub use pool::*;
