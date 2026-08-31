//! Bounded native-callback framing for capture backends.

/// Converts interleaved native callback buffers into fixed-duration frames.
///
/// Construction allocates the one retained frame buffer. `push` only copies
/// into that buffer and invokes the supplied nonblocking emission function.
#[derive(Debug)]
pub(crate) struct CaptureFrameNormalizer {
    samples: Box<[f32]>,
    sample_count: usize,
    samples_per_channel: usize,
    channel_count: usize,
    sample_rate_hz: u32,
    next_timestamp_ns: Option<u64>,
}

impl CaptureFrameNormalizer {
    pub(crate) fn new(samples_per_channel: usize, channel_count: u8, sample_rate_hz: u32) -> Self {
        let channel_count = usize::from(channel_count);
        let frame_sample_count = samples_per_channel.saturating_mul(channel_count);
        Self {
            samples: vec![0.0; frame_sample_count].into_boxed_slice(),
            sample_count: 0,
            samples_per_channel,
            channel_count,
            sample_rate_hz,
            next_timestamp_ns: None,
        }
    }

    pub(crate) fn frame_sample_count(&self) -> usize {
        self.samples.len()
    }

    /// Returns `false` when the native buffer is not aligned to the declared
    /// channel count or the normalizer was constructed with an invalid format.
    pub(crate) fn push(
        &mut self,
        callback_samples: &[f32],
        callback_timestamp_ns: u64,
        mut emit: impl FnMut(u64, &[f32]),
    ) -> bool {
        if self.samples.is_empty()
            || self.channel_count == 0
            || self.sample_rate_hz == 0
            || !callback_samples.len().is_multiple_of(self.channel_count)
        {
            return false;
        }
        if self.sample_count == 0 {
            self.next_timestamp_ns = Some(callback_timestamp_ns.max(1));
        }

        let frame_duration_ns = u64::try_from(self.samples_per_channel)
            .unwrap_or(u64::MAX)
            .saturating_mul(1_000_000_000)
            .checked_div(u64::from(self.sample_rate_hz))
            .unwrap_or(0);
        let mut source_offset = 0usize;
        while source_offset < callback_samples.len() {
            let available = self.samples.len().saturating_sub(self.sample_count);
            let copy_count = available.min(callback_samples.len() - source_offset);
            let destination_end = self.sample_count.saturating_add(copy_count);
            let source_end = source_offset.saturating_add(copy_count);
            self.samples[self.sample_count..destination_end]
                .copy_from_slice(&callback_samples[source_offset..source_end]);
            self.sample_count = destination_end;
            source_offset = source_end;

            if self.sample_count == self.samples.len() {
                let timestamp_ns = self
                    .next_timestamp_ns
                    .take()
                    .unwrap_or(callback_timestamp_ns.max(1));
                emit(timestamp_ns, &self.samples);
                self.sample_count = 0;
                if source_offset < callback_samples.len() {
                    self.next_timestamp_ns = Some(timestamp_ns.saturating_add(frame_duration_ns));
                }
            }
        }
        true
    }

    #[cfg(test)]
    fn pending_sample_count(&self) -> usize {
        self.sample_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_two_1024_frame_stereo_callbacks_when_normalized_then_twenty_ms_frames_are_contiguous()
    {
        let mut normalizer = CaptureFrameNormalizer::new(960, 2, 48_000);
        let callback = vec![0.25; 1_024 * 2];
        let mut emitted = Vec::new();

        assert!(
            normalizer.push(&callback, 1_000_000_000, |timestamp_ns, samples| {
                emitted.push((timestamp_ns, samples.len()));
            })
        );
        assert!(
            normalizer.push(&callback, 1_021_333_333, |timestamp_ns, samples| {
                emitted.push((timestamp_ns, samples.len()));
            })
        );

        assert_eq!(
            emitted,
            vec![(1_000_000_000, 1_920), (1_020_000_000, 1_920)]
        );
        assert_eq!(normalizer.pending_sample_count(), 256);
    }

    #[test]
    fn given_two_half_frames_when_normalized_then_one_complete_frame_is_emitted() {
        let mut normalizer = CaptureFrameNormalizer::new(960, 1, 48_000);
        let mut emitted = Vec::new();

        assert!(
            normalizer.push(&[0.0; 480], 2_000_000_000, |timestamp_ns, samples| {
                emitted.push((timestamp_ns, samples.len()));
            })
        );
        assert!(
            normalizer.push(&[0.0; 480], 2_010_000_000, |timestamp_ns, samples| {
                emitted.push((timestamp_ns, samples.len()));
            })
        );

        assert_eq!(emitted, vec![(2_000_000_000, 960)]);
        assert_eq!(normalizer.pending_sample_count(), 0);
    }

    #[test]
    fn given_voice_frame_duration_when_callback_is_larger_then_ten_ms_frames_are_contiguous() {
        let mut normalizer = CaptureFrameNormalizer::new(480, 1, 48_000);
        let callback = [0.0; 960];
        let mut emitted = Vec::new();

        assert!(
            normalizer.push(&callback, 3_000_000_000, |timestamp_ns, samples| {
                emitted.push((timestamp_ns, samples.len()));
            })
        );

        assert_eq!(emitted, vec![(3_000_000_000, 480), (3_010_000_000, 480)]);
        assert_eq!(normalizer.pending_sample_count(), 0);
    }

    #[test]
    fn given_misaligned_stereo_callback_when_normalized_then_input_is_rejected() {
        let mut normalizer = CaptureFrameNormalizer::new(960, 2, 48_000);

        assert!(!normalizer.push(&[0.0; 3], 1, |_, _| {}));
        assert_eq!(normalizer.pending_sample_count(), 0);
    }
}
