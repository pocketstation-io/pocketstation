use std::collections::{HashMap, VecDeque};

use super::{AudioHistoryConfig, AudioHistoryError, AudioHistoryObservations, AudioHistoryState};
use crate::frame::{FrameLineage, SampleFormat, SampleSpec, StemId};
use crate::recording::{RecordedStem, RecordingClipWindow, MAX_RECORDING_CLIP_BYTES};

const SECOND_NS: u128 = 1_000_000_000;

pub(super) struct HistoryState {
    config: AudioHistoryConfig,
    stems: HashMap<StemId, RetainedStem>,
    newest_timestamp_ns: u64,
    pub observations: AudioHistoryObservations,
}

struct RetainedStem {
    metadata: RecordedStem,
    buffers: VecDeque<Buffer>,
    next_sequence_number: Option<u64>,
    discontinuity_epoch: u64,
    segment_origin_ns: u64,
    segment_sample_frames: u64,
    next_sample_frame: u64,
}

struct Buffer {
    start_ns: u64,
    end_ns: u64,
    first_sample_frame: u64,
    samples: Box<[f32]>,
    is_discontinuous: bool,
}

pub(super) struct ClipSnapshot {
    pub stem: RecordedStem,
    pub samples: Vec<f32>,
    pub actual: RecordingClipWindow,
    pub first_sample_frame: u64,
    pub sample_frames: u64,
}

impl HistoryState {
    pub fn new(config: AudioHistoryConfig) -> Self {
        Self {
            config,
            stems: HashMap::new(),
            newest_timestamp_ns: 0,
            observations: AudioHistoryObservations {
                state: AudioHistoryState::Preparing,
                retained_pcm_bytes: 0,
                retained_buffers: 0,
                received_buffers_total: 0,
                evicted_buffers_total: 0,
                rejected_buffers_total: 0,
                discontinuities_total: 0,
                source_resets_total: 0,
            },
        }
    }

    pub fn metadata(&self) -> Vec<RecordedStem> {
        let mut stems: Vec<_> = self
            .stems
            .values()
            .map(|stem| stem.metadata.clone())
            .collect();
        stems.sort_by_key(|stem| stem.stem_id.get());
        stems
    }

    pub fn clear(&mut self) {
        self.observations.evicted_buffers_total = self
            .observations
            .evicted_buffers_total
            .saturating_add(self.observations.retained_buffers as u64);
        for stem in self.stems.values_mut() {
            stem.buffers.clear();
            stem.buffers.shrink_to_fit();
        }
        self.observations.retained_buffers = 0;
        self.observations.retained_pcm_bytes = 0;
    }

    pub fn push(
        &mut self,
        lineage: FrameLineage,
        spec: SampleSpec,
        timestamp_ns: u64,
        samples: &[f32],
    ) -> Result<(), AudioHistoryError> {
        self.observations.received_buffers_total =
            self.observations.received_buffers_total.saturating_add(1);
        let result = self.accept(lineage, spec, timestamp_ns, samples);
        if result.is_err() {
            self.observations.rejected_buffers_total =
                self.observations.rejected_buffers_total.saturating_add(1);
        }
        result
    }

    fn accept(
        &mut self,
        lineage: FrameLineage,
        spec: SampleSpec,
        timestamp_ns: u64,
        samples: &[f32],
    ) -> Result<(), AudioHistoryError> {
        let channel_count = usize::from(spec.channels);
        if !(8000..=192000).contains(&spec.sample_rate_hz)
            || !(1..=32).contains(&spec.channels)
            || spec.format != SampleFormat::F32Interleaved
            || samples.is_empty()
            || !samples.len().is_multiple_of(channel_count)
            || samples.iter().any(|sample| !sample.is_finite())
        {
            return Err(AudioHistoryError::InvalidFrame);
        }
        let pcm_bytes = std::mem::size_of_val(samples);
        if pcm_bytes > self.config.max_pcm_bytes {
            return Err(AudioHistoryError::SizeLimit);
        }
        let stem_id = lineage.stem_id();
        if let Some(stem) = self.stems.get(&stem_id) {
            if stem.metadata.session_id != lineage.session_id()
                || stem.metadata.source_id != lineage.source_id()
                || stem.metadata.sample_rate_hz != spec.sample_rate_hz
                || stem.metadata.channels != u16::from(spec.channels)
            {
                return Err(AudioHistoryError::InvalidFrame);
            }
            if stem.metadata.clock_id != lineage.clock_id()
                || stem.metadata.source_generation != lineage.source_generation()
                || stem.metadata.permission_epoch != lineage.permission_epoch()
            {
                let old = self
                    .stems
                    .remove(&stem_id)
                    .ok_or(AudioHistoryError::Failed)?;
                for buffer in old.buffers {
                    self.account_eviction(&buffer);
                }
                self.observations.source_resets_total =
                    self.observations.source_resets_total.saturating_add(1);
            }
        }
        if !self.stems.contains_key(&stem_id) {
            if self.stems.len() >= 64 {
                return Err(AudioHistoryError::SizeLimit);
            }
            self.stems.insert(
                stem_id,
                RetainedStem {
                    metadata: RecordedStem {
                        label: format!("stem-{}", stem_id.get()),
                        session_id: lineage.session_id(),
                        source_id: lineage.source_id(),
                        stem_id,
                        clock_id: lineage.clock_id(),
                        source_generation: lineage.source_generation(),
                        permission_epoch: lineage.permission_epoch(),
                        sample_rate_hz: spec.sample_rate_hz,
                        channels: u16::from(spec.channels),
                        first_timestamp_ns: timestamp_ns,
                        final_timestamp_ns: timestamp_ns,
                    },
                    buffers: VecDeque::new(),
                    next_sequence_number: None,
                    discontinuity_epoch: lineage.discontinuity_epoch(),
                    segment_origin_ns: timestamp_ns,
                    segment_sample_frames: 0,
                    next_sample_frame: 0,
                },
            );
        }
        let stem = self
            .stems
            .get_mut(&stem_id)
            .ok_or(AudioHistoryError::Failed)?;
        let is_discontinuous = stem
            .next_sequence_number
            .is_some_and(|next| next != lineage.sequence_number())
            || stem.discontinuity_epoch != lineage.discontinuity_epoch();
        // As with the recorder, continuous PCM uses sample cadence, not host jitter.
        let start_ns = if stem.next_sequence_number.is_some() && !is_discontinuous {
            stem.metadata.final_timestamp_ns
        } else {
            timestamp_ns
        };
        if stem.next_sequence_number.is_some() && start_ns < stem.metadata.final_timestamp_ns {
            return Err(AudioHistoryError::InvalidFrame);
        }
        let duration_ns = ((samples.len() / channel_count) as u128 * SECOND_NS
            / u128::from(spec.sample_rate_hz)) as u64;
        if duration_ns == 0 || duration_ns > self.config.retention_ns {
            return Err(AudioHistoryError::InvalidFrame);
        }
        if is_discontinuous {
            stem.segment_origin_ns = start_ns;
            stem.segment_sample_frames = 0;
            stem.next_sample_frame = (u128::from(start_ns - stem.metadata.first_timestamp_ns)
                * u128::from(spec.sample_rate_hz)
                / SECOND_NS) as u64;
        }
        let first_sample_frame = stem.next_sample_frame;
        let next_sample_frame = first_sample_frame
            .checked_add((samples.len() / channel_count) as u64)
            .ok_or(AudioHistoryError::InvalidFrame)?;
        let total_frames = stem
            .segment_sample_frames
            .checked_add((samples.len() / channel_count) as u64)
            .ok_or(AudioHistoryError::InvalidFrame)?;
        let end_ns = stem
            .segment_origin_ns
            .checked_add(
                (u128::from(total_frames) * SECOND_NS / u128::from(spec.sample_rate_hz)) as u64,
            )
            .ok_or(AudioHistoryError::InvalidFrame)?;
        stem.segment_sample_frames = total_frames;
        stem.next_sample_frame = next_sample_frame;
        stem.metadata.final_timestamp_ns = end_ns;
        stem.next_sequence_number = lineage.sequence_number().checked_add(1);
        stem.discontinuity_epoch = lineage.discontinuity_epoch();
        if is_discontinuous {
            self.observations.discontinuities_total =
                self.observations.discontinuities_total.saturating_add(1);
        }
        // Evict before copying so retained PCM never transiently exceeds the cap.
        self.newest_timestamp_ns = self.newest_timestamp_ns.max(end_ns);
        self.evict(pcm_bytes);
        self.stems
            .get_mut(&stem_id)
            .ok_or(AudioHistoryError::Failed)?
            .buffers
            .push_back(Buffer {
                start_ns,
                end_ns,
                first_sample_frame,
                samples: samples.to_vec().into_boxed_slice(),
                is_discontinuous,
            });
        self.observations.retained_pcm_bytes += pcm_bytes;
        self.observations.retained_buffers += 1;
        Ok(())
    }

    fn account_eviction(&mut self, buffer: &Buffer) {
        self.observations.retained_pcm_bytes -= std::mem::size_of_val(buffer.samples.as_ref());
        self.observations.retained_buffers -= 1;
        self.observations.evicted_buffers_total =
            self.observations.evicted_buffers_total.saturating_add(1);
    }

    fn evict(&mut self, incoming_bytes: usize) {
        let cutoff_ns = self
            .newest_timestamp_ns
            .saturating_sub(self.config.retention_ns);
        loop {
            let oldest = self
                .stems
                .iter()
                .filter_map(|(id, stem)| stem.buffers.front().map(|buffer| (*id, buffer.start_ns)))
                .min_by_key(|(id, start_ns)| (*start_ns, id.get()));
            let Some((stem_id, start_ns)) = oldest else {
                break;
            };
            if start_ns >= cutoff_ns
                && self.observations.retained_pcm_bytes + incoming_bytes
                    <= self.config.max_pcm_bytes
                && self.observations.retained_buffers < self.config.max_buffers
            {
                break;
            }
            if let Some(buffer) = self
                .stems
                .get_mut(&stem_id)
                .and_then(|stem| stem.buffers.pop_front())
            {
                self.account_eviction(&buffer);
                if let Some(stem) = self.stems.get_mut(&stem_id) {
                    let target_buffers = stem.buffers.len().max(8);
                    if stem.buffers.capacity() > target_buffers.saturating_mul(4) {
                        stem.buffers.shrink_to(target_buffers);
                    }
                }
            }
        }
    }

    pub fn snapshot(
        &self,
        stem_id: StemId,
        window: RecordingClipWindow,
    ) -> Result<ClipSnapshot, AudioHistoryError> {
        match self.observations.state {
            AudioHistoryState::Cancelled => return Err(AudioHistoryError::Cancelled),
            AudioHistoryState::Failed => return Err(AudioHistoryError::Failed),
            _ => {}
        }
        let stem = self
            .stems
            .get(&stem_id)
            .ok_or(AudioHistoryError::UnknownStem)?;
        let first = stem.buffers.front().ok_or(AudioHistoryError::Expired)?;
        if window.start_ns() < first.start_ns {
            return Err(AudioHistoryError::Expired);
        }
        if window.end_ns() > stem.metadata.final_timestamp_ns {
            return Err(if self.observations.state == AudioHistoryState::Complete {
                AudioHistoryError::Ended
            } else {
                AudioHistoryError::NotReady
            });
        }
        let rate = u128::from(stem.metadata.sample_rate_hz);
        let channel_count = usize::from(stem.metadata.channels);
        let output_frames =
            (u128::from(window.end_ns() - window.start_ns()) * rate).div_ceil(SECOND_NS) + 2;
        if output_frames * channel_count as u128 * 4 + 128 > MAX_RECORDING_CLIP_BYTES as u128 {
            return Err(AudioHistoryError::SizeLimit);
        }
        let mut samples = Vec::with_capacity(output_frames as usize * channel_count);
        let mut actual_start_ns = None;
        let mut first_sample_frame = None;
        let mut actual_end_ns = 0;
        for buffer in &stem.buffers {
            if buffer.end_ns <= window.start_ns() {
                continue;
            }
            if buffer.start_ns >= window.end_ns() {
                break;
            }
            if buffer.is_discontinuous && buffer.start_ns >= window.start_ns()
                || actual_start_ns.is_some() && buffer.start_ns != actual_end_ns
                || actual_start_ns.is_none() && buffer.start_ns > window.start_ns()
            {
                return Err(AudioHistoryError::MissingContext);
            }
            let first_frame = (u128::from(window.start_ns().saturating_sub(buffer.start_ns)) * rate
                / SECOND_NS) as usize;
            let last_frame = (u128::from(window.end_ns().min(buffer.end_ns) - buffer.start_ns)
                * rate)
                .div_ceil(SECOND_NS) as usize;
            let last_frame = last_frame.min(buffer.samples.len() / channel_count);
            if first_frame >= last_frame {
                continue;
            }
            actual_start_ns
                .get_or_insert(buffer.start_ns + (first_frame as u128 * SECOND_NS / rate) as u64);
            first_sample_frame.get_or_insert(buffer.first_sample_frame + first_frame as u64);
            actual_end_ns = if last_frame == buffer.samples.len() / channel_count {
                buffer.end_ns
            } else {
                buffer.start_ns + (last_frame as u128 * SECOND_NS / rate) as u64
            };
            samples.extend_from_slice(
                &buffer.samples[first_frame * channel_count..last_frame * channel_count],
            );
        }
        let start_ns = actual_start_ns.ok_or(AudioHistoryError::MissingContext)?;
        if actual_end_ns < window.end_ns() {
            return Err(AudioHistoryError::MissingContext);
        }
        Ok(ClipSnapshot {
            sample_frames: (samples.len() / channel_count) as u64,
            first_sample_frame: first_sample_frame.ok_or(AudioHistoryError::MissingContext)?,
            stem: stem.metadata.clone(),
            samples,
            actual: RecordingClipWindow::sample_aligned(start_ns, actual_end_ns),
        })
    }
}
