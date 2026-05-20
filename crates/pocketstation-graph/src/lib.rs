use pocketstation_frame::AudioFrame;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChannelLayout {
    MonoOnly,
    StereoOnly,
    Either,
}

pub trait AudioProcessorNode: Send {
    fn name(&self) -> &'static str;
    fn process(&mut self, frame: AudioFrame) -> Option<AudioFrame>;
    fn accepted_channels(&self) -> ChannelLayout { ChannelLayout::Either }
}

pub struct ProcessorGraph {
    nodes: Vec<Box<dyn AudioProcessorNode>>,
}

impl ProcessorGraph {
    pub fn new() -> Self { Self { nodes: Vec::new() } }
    pub fn add_node<N: AudioProcessorNode + 'static>(&mut self, node: N) { self.nodes.push(Box::new(node)); }
    pub fn process(&mut self, mut frame: AudioFrame) -> Option<AudioFrame> {
        for node in self.nodes.iter_mut() {
            frame = node.process(frame)?;
        }
        Some(frame)
    }
}

impl Default for ProcessorGraph { fn default() -> Self { Self::new() } }

pub struct PassthroughNode;
impl AudioProcessorNode for PassthroughNode {
    fn name(&self) -> &'static str { "passthrough" }
    fn process(&mut self, frame: AudioFrame) -> Option<AudioFrame> { Some(frame) }
}

pub struct GainNode { gain: f32 }
impl GainNode { pub fn new(gain: f32) -> Self { Self { gain } } }
impl AudioProcessorNode for GainNode {
    fn name(&self) -> &'static str { "gain" }
    fn process(&mut self, mut frame: AudioFrame) -> Option<AudioFrame> {
        for s in frame.buffer.as_mut_slice().iter_mut() { *s *= self.gain; }
        Some(frame)
    }
}

pub struct MonoMixNode;
impl AudioProcessorNode for MonoMixNode {
    fn name(&self) -> &'static str { "mono_mix" }
    fn accepted_channels(&self) -> ChannelLayout { ChannelLayout::StereoOnly }
    fn process(&mut self, mut frame: AudioFrame) -> Option<AudioFrame> {
        if frame.channels == 2 {
            let len = frame.buffer.len();
            let samples = frame.buffer.as_mut_slice();
            let mut out = 0usize;
            let mut i = 0usize;
            while i + 1 < len {
                samples[out] = (samples[i] + samples[i + 1]) * 0.5;
                out += 1;
                i += 2;
            }
            frame.buffer.set_len(out);
            frame.channels = 1;
        }
        Some(frame)
    }
}

pub struct ResampleNode;
impl AudioProcessorNode for ResampleNode {
    fn name(&self) -> &'static str { "resample_placeholder" }
    fn process(&mut self, frame: AudioFrame) -> Option<AudioFrame> {
        // ADR-006 owns real PI-controlled SRC implementation.
        Some(frame)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pocketstation_frame::{AudioBufferPool, AudioFrame, SourceId, StreamId};

    #[test]
    fn gain_node_mutates_samples() {
        let pool = AudioBufferPool::new(1, 4);
        let mut h = pool.acquire().unwrap();
        h.copy_from_slice(&[1.0, -1.0]);
        let frame = AudioFrame::new(StreamId(1), SourceId(1), 0, 0, 1, h);
        let mut graph = ProcessorGraph::new();
        graph.add_node(GainNode::new(0.5));
        let out = graph.process(frame).unwrap();
        assert_eq!(out.buffer.as_slice(), &[0.5, -0.5]);
    }
}
