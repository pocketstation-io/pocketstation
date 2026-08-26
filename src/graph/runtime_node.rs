use crate::frame::AudioFrame;
use crate::graph::node::{NodeError, PrepareContext};

/// Realtime invariant: when `ExecutionClass::is_realtime` is true, `process()`
/// stays allocation-free, lock-free, log-free, and blocking-free. `prepare()`
/// sizes the working state once for the lifetime of the node.
pub trait RuntimeNode: Send {
    fn prepare(&mut self, cx: &PrepareContext) -> Result<(), NodeError>;
    fn process(&mut self, frame: AudioFrame) -> Result<Option<AudioFrame>, NodeError>;
}
