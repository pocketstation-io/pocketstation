use std::collections::VecDeque;
use pocketstation_frame::AudioFrame;

#[derive(Debug, Clone, Copy)]
pub enum OpusFrameDuration {
    Ms10,
    Ms20,
    Ms40,
    Ms60,
}

impl OpusFrameDuration {
    pub fn samples_at_48k(self) -> usize {
        match self { Self::Ms10 => 480, Self::Ms20 => 960, Self::Ms40 => 1920, Self::Ms60 => 2880 }
    }
}

pub struct EncodedFrame {
    pub sequence_number: u64,
    pub timestamp_ns: u64,
    pub payload: Vec<u8>,
}

pub struct MockOpusEncoder;
pub struct MockOpusDecoder;

impl MockOpusEncoder {
    pub fn encode(&mut self, frame: &AudioFrame) -> EncodedFrame {
        let mut payload = Vec::with_capacity(frame.buffer.len() * 4);
        for s in frame.buffer.as_slice() { payload.extend_from_slice(&s.to_le_bytes()); }
        EncodedFrame { sequence_number: frame.sequence_number, timestamp_ns: frame.timestamp_ns, payload }
    }
}

impl MockOpusDecoder {
    pub fn decode_to_vec(&mut self, encoded: &EncodedFrame) -> Vec<f32> {
        encoded.payload.chunks_exact(4).map(|b| f32::from_le_bytes([b[0],b[1],b[2],b[3]])).collect()
    }
}

#[cfg(feature = "real-opus")]
pub mod real_opus {
    use opus::{Application, Channels, Decoder, Encoder};

    pub struct RealOpusEncoder { inner: Encoder, channels: usize }
    pub struct RealOpusDecoder { inner: Decoder, channels: usize }

    impl RealOpusEncoder {
        pub fn new(sample_rate: u32, channels: usize) -> Result<Self, opus::Error> {
            let ch = if channels == 1 { Channels::Mono } else { Channels::Stereo };
            Ok(Self { inner: Encoder::new(sample_rate, ch, Application::Audio)?, channels })
        }
        pub fn encode_float(&mut self, pcm: &[f32], out: &mut [u8]) -> Result<usize, opus::Error> {
            self.inner.encode_float(pcm, out)
        }
    }

    impl RealOpusDecoder {
        pub fn new(sample_rate: u32, channels: usize) -> Result<Self, opus::Error> {
            let ch = if channels == 1 { Channels::Mono } else { Channels::Stereo };
            Ok(Self { inner: Decoder::new(sample_rate, ch)?, channels })
        }
        pub fn decode_float(&mut self, payload: &[u8], out: &mut [f32]) -> Result<usize, opus::Error> {
            self.inner.decode_float(payload, out, false)
        }
    }
}

pub struct JitterBuffer {
    target_depth: usize,
    queue: VecDeque<EncodedFrame>,
}

impl JitterBuffer {
    pub fn new(target_depth: usize) -> Self { Self { target_depth, queue: VecDeque::new() } }
    pub fn push(&mut self, frame: EncodedFrame) { self.queue.push_back(frame); }
    pub fn pop_ready(&mut self) -> Option<EncodedFrame> {
        if self.queue.len() >= self.target_depth { self.queue.pop_front() } else { None }
    }
    pub fn depth(&self) -> usize { self.queue.len() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn duration_samples() { assert_eq!(OpusFrameDuration::Ms20.samples_at_48k(), 960); }
}
