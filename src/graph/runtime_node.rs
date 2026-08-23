use crate::frame::AudioFrame;
use crate::graph::node::{NodeError, PrepareContext};

/// Realtime invariant: for nodes whose ExecutionClass::is_realtime is true, process()
/// must stay alloc-free, lock-free, log-free, and blocking-free (LAW 15). All working
/// state is sized once in prepare() and reused for the lifetime of the node.
pub trait RuntimeNode: Send {
    #[doc = "Prepares resources required by `RuntimeNode`."]
    fn prepare(&mut self, cx: &PrepareContext) -> Result<(), NodeError>;
    #[doc = "Processes an input value through `RuntimeNode`."]
    fn process(&mut self, frame: AudioFrame) -> Result<Option<AudioFrame>, NodeError>;
}
