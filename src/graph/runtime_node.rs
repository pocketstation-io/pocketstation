use crate::frame::AudioFrame;
use crate::graph::node::{NodeError, PrepareContext};

/// Realtime invariant: for nodes whose ExecutionClass::is_realtime is true, process()
/// must stay alloc-free, lock-free, log-free, and blocking-free (LAW 15). All working
/// state is sized once in prepare() and reused for the lifetime of the node.
pub trait RuntimeNode: Send {
    fn prepare(&mut self, cx: &PrepareContext) -> Result<(), NodeError>;
    fn process(&mut self, frame: AudioFrame) -> Result<Option<AudioFrame>, NodeError>;
}
