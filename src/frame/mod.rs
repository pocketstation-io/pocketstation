//! Realtime audio frame ownership.
//!
//! Platform and graph identities are intentionally separate from pooled audio
//! storage. The audio module remains specialized; arbitrary signals use the
//! asynchronous signal lane.

mod audio;
mod identity;
mod lineage;
mod platform;
mod pool;

pub use audio::*;
pub use identity::*;
pub use lineage::*;
pub use platform::Platform;
pub use pool::*;
