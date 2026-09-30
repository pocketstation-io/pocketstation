use super::config::{ENGINE_FRAME_SAMPLES, OUTPUT_POOL_FRAMES, SAMPLE_RATE_HZ};
use super::{
    AecConfiguration, EchoCancellationObservations, EchoCancellationState, ObservationState,
};
use crate::timing::cadence_error_ns;
use crate::{
    AudioBufferHandle, AudioBufferPool, AudioFrame, AudioProcessing, NodeError, OperatorId,
    OutputGeneration, SampleFormat, SampleSpec, SignalDerivation, SignalEnvelope, SignalLineage,
    SignalPayload, SignalTiming,
};
use std::{collections::VecDeque, sync::Arc, time::Instant};
use webrtc_audio_processing::{config::EchoCanceller, Config, Processor};

// At 48 kHz: 384 samples block/OLA buffering + nominal 48 filter samples.
// Frequency-dependent signal delay, not CPU time or a pure sample shift.
const NOMINAL_DELAY_SAMPLES: u32 = 432;
const DRAIN_DURATION_MS: u32 = 40;

struct LastMicrophone {
    lineage: SignalLineage,
    timing: SignalTiming,
    generation: Option<OutputGeneration>,
}

pub(crate) struct EchoProcessor {
    configuration: AecConfiguration,
    processor: Option<Processor>,
    output_pool: Option<Arc<AudioBufferPool>>,
    microphone: VecDeque<SignalEnvelope>,
    reference: VecDeque<SignalEnvelope>,
    previous: [Option<(SignalLineage, SignalTiming)>; 2],
    diagnostics: EchoCancellationObservations,
    observations: ObservationState,
    operator_id: OperatorId,
    last_render_time_ns: Option<u64>,
    last_microphone: Option<LastMicrophone>,
    next_output_sequence: u64,
    flushed: bool,
}

fn failure(message: impl ToString) -> NodeError {
    NodeError::Process(message.to_string())
}

impl EchoProcessor {
    pub(crate) fn new(
        configuration: AecConfiguration,
        operator_id: OperatorId,
        observations: ObservationState,
    ) -> Self {
        Self {
            configuration,
            processor: None,
            output_pool: None,
            microphone: VecDeque::with_capacity(configuration.capacity_frames()),
            reference: VecDeque::with_capacity(configuration.capacity_frames()),
            previous: [None, None],
            diagnostics: EchoCancellationObservations::new(configuration.capacity_frames()),
            observations,
            operator_id,
            last_render_time_ns: None,
            last_microphone: None,
            next_output_sequence: 0,
            flushed: false,
        }
    }

    fn reset(&mut self) {
        self.diagnostics.discarded_microphone_frames_total += self.microphone.len() as u64;
        self.diagnostics.discarded_reference_frames_total += self.reference.len() as u64;
        self.microphone.clear();
        self.reference.clear();
        if self.last_microphone.take().is_some() {
            self.diagnostics.discarded_tail_generations_total += 1;
        }
        self.previous = [None, None];
        self.last_render_time_ns = None;
        if let Some(processor) = &self.processor {
            processor.reinitialize();
        }
        self.diagnostics.resets_total += 1;
        self.diagnostics.processing_generation =
            self.diagnostics.processing_generation.saturating_add(1);
        self.diagnostics.state = EchoCancellationState::Reset;
    }

    fn validate_input(
        &self,
        input: &SignalEnvelope,
    ) -> Result<(SignalLineage, SignalTiming), NodeError> {
        input.validate().map_err(failure)?;
        let lineage = input
            .lineage()
            .ok_or_else(|| failure("AEC input requires source lineage"))?;
        let timing = input.timing();
        if timing.source_timestamp_ns().is_none()
            || timing.duration_ns()
                != Some(u64::from(self.configuration.frame_duration_ms) * 1_000_000)
        {
            return Err(failure(
                "AEC input requires exact source time and frame duration",
            ));
        }
        let SignalPayload::Audio(frame) = input.payload() else {
            return Err(failure("AEC input must be PCM"));
        };
        let channels = self.configuration.reference_channels.count();
        if frame.sample_rate_hz() != SAMPLE_RATE_HZ
            || frame.channels() != channels
            || frame.samples().len() != self.configuration.frame_samples() * usize::from(channels)
            || frame.samples().iter().any(|s| !s.is_finite())
        {
            return Err(failure(
                "AEC input format, length or finite-sample contract violated",
            ));
        }
        Ok((lineage, timing))
    }

    pub(crate) fn publish(&mut self) {
        self.diagnostics.microphone_queue_depth_frames = self.microphone.len();
        self.diagnostics.reference_queue_depth_frames = self.reference.len();
        self.observations.update(self.diagnostics.clone());
    }

    fn derivation(
        &self,
        lineage: SignalLineage,
        timing: SignalTiming,
    ) -> Result<SignalDerivation, NodeError> {
        SignalDerivation::new(
            lineage,
            timing,
            self.operator_id.clone(),
            1,
            // Operator generation identifies the registered implementation.
            // Adaptation resets have a separate observable processing generation.
            1,
            None,
        )
        .map_err(failure)
    }

    pub(crate) fn accept(
        &mut self,
        port: &str,
        input: SignalEnvelope,
    ) -> Result<Vec<SignalEnvelope>, NodeError> {
        if self.flushed {
            return Err(failure("AEC input after graceful finish"));
        }
        if self.processor.is_none() {
            return Err(failure("AEC worker is not prepared or is stopped"));
        }
        let index = match port {
            "microphone" => 0,
            "reference" => 1,
            _ => return Err(failure("unknown AEC input port")),
        };
        let (lineage, timing) = self.validate_input(&input)?;
        if let Some((last, last_time)) = self.previous[index] {
            let cadence_error = cadence_error_ns(
                last_time.source_timestamp_ns().unwrap_or(0),
                timing.source_timestamp_ns().unwrap_or(0),
                u64::from(self.configuration.frame_duration_ms) * 1_000_000,
            );
            self.diagnostics.maximum_cadence_error_ns =
                self.diagnostics.maximum_cadence_error_ns.max(cadence_error);
            let changed = last.source_id() != lineage.source_id()
                || last.stream_id() != lineage.stream_id()
                || last.clock_id() != lineage.clock_id()
                || last.source_generation() != lineage.source_generation()
                || last.policy_epoch() != lineage.policy_epoch()
                || last.discontinuity_epoch() != lineage.discontinuity_epoch()
                || last.sequence_number().checked_add(1) != Some(lineage.sequence_number())
                || cadence_error > self.configuration.maximum_cadence_error_ns;
            if changed {
                self.reset();
            }
        }
        self.previous[index] = Some((lineage, timing));
        let queue = if index == 0 {
            &mut self.microphone
        } else {
            &mut self.reference
        };
        if queue.len() == self.configuration.capacity_frames() {
            return Err(failure(
                "AEC unmatched input capacity exhausted; reference unavailable or clocks unaligned",
            ));
        }
        queue.push_back(input);
        let mut outputs = Vec::new();
        while let Some(mic) = self.microphone.front() {
            let ml = mic
                .lineage()
                .ok_or_else(|| failure("missing microphone lineage"))?;
            let mt = mic
                .timing()
                .source_timestamp_ns()
                .ok_or_else(|| failure("missing microphone time"))?;
            if let Some(reference) = self.reference.front() {
                let rl = reference
                    .lineage()
                    .ok_or_else(|| failure("missing reference lineage"))?;
                let rt = reference
                    .timing()
                    .source_timestamp_ns()
                    .ok_or_else(|| failure("missing reference time"))?;
                if ml.clock_id() != rl.clock_id() || ml.session_id() != rl.session_id() {
                    return Err(failure(
                        "AEC inputs must share a Session and declared clock domain",
                    ));
                }
                // Render and capture are independent engine inputs. Feed each
                // actual render block once, in source-time order, with no sample
                // splicing or invented reference after EOF.
                if rt <= mt || self.last_render_time_ns.is_none() {
                    if rt.saturating_sub(mt)
                        > u64::from(super::config::MAXIMUM_PENDING_AUDIO_MS) * 1_000_000
                    {
                        return Err(failure(
                            "AEC reference begins beyond the bounded microphone startup interval",
                        ));
                    }
                    let reference = self
                        .reference
                        .pop_front()
                        .ok_or_else(|| failure("missing queued reference"))?;
                    self.analyze_reference(reference)?;
                    continue;
                }
            }
            let Some(reference_ns) = self.last_render_time_ns else {
                break;
            };
            let age_ns = mt.saturating_sub(reference_ns);
            if age_ns > u64::from(super::config::MAXIMUM_PENDING_AUDIO_MS) * 1_000_000 {
                // Wait only within the existing bounded microphone queue. A
                // delayed or missing reference cannot grow retained audio.
                break;
            }
            self.diagnostics.latest_reference_age_ns = age_ns;
            self.diagnostics.latest_reference_lead_ns = reference_ns.saturating_sub(mt);
            self.diagnostics.microphone_source_id = Some(ml.source_id());
            let microphone = self
                .microphone
                .pop_front()
                .ok_or_else(|| failure("missing queued microphone"))?;
            outputs.push(self.process_microphone(microphone)?);
        }
        if !self.microphone.is_empty() || self.diagnostics.processed_microphone_frames_total == 0 {
            self.diagnostics.state = EchoCancellationState::WaitingForReference;
        }
        self.publish();
        Ok(outputs)
    }

    fn analyze_reference(&mut self, reference: SignalEnvelope) -> Result<(), NodeError> {
        let lineage = reference
            .lineage()
            .ok_or_else(|| failure("reference lineage missing"))?;
        let timestamp_ns = reference
            .timing()
            .source_timestamp_ns()
            .ok_or_else(|| failure("reference time missing"))?;
        let SignalPayload::Audio(frame) = reference.payload() else {
            return Err(failure("reference payload changed"));
        };
        let processor = self
            .processor
            .as_ref()
            .ok_or_else(|| failure("AEC processor absent"))?;
        let channels = usize::from(self.configuration.reference_channels.count());
        for block in frame
            .samples()
            .chunks_exact(ENGINE_FRAME_SAMPLES * channels)
        {
            let mut render = [[0.0_f32; ENGINE_FRAME_SAMPLES]; 2];
            for (channel, values) in render.iter_mut().take(channels).enumerate() {
                for (sample, value) in values.iter_mut().enumerate() {
                    *value = block[sample * channels + channel];
                }
            }
            processor
                .analyze_render_frame(render[..channels].iter().map(|c| c.as_slice()))
                .map_err(failure)?;
        }
        self.last_render_time_ns = Some(timestamp_ns);
        self.diagnostics.reference_source_id = Some(lineage.source_id());
        self.diagnostics.analyzed_reference_frames_total += 1;
        Ok(())
    }

    fn process_microphone(
        &mut self,
        microphone: SignalEnvelope,
    ) -> Result<SignalEnvelope, NodeError> {
        let lineage = microphone
            .lineage()
            .ok_or_else(|| failure("microphone lineage missing"))?;
        let timing = microphone.timing();
        let generation = microphone.output_generation().cloned();
        let SignalPayload::Audio(mic) = microphone.into_payload() else {
            return Err(failure("microphone payload changed"));
        };
        let buffer = self.process_capture(Some(mic.samples()))?;
        let metadata = self.processing_metadata(lineage, timing, 0, 0)?;
        let output = self.output(
            buffer,
            lineage,
            timing,
            timing,
            generation.clone(),
            metadata,
        )?;
        self.last_microphone = Some(LastMicrophone {
            lineage,
            timing,
            generation,
        });
        self.diagnostics.processed_microphone_frames_total += 1;
        self.diagnostics.state = EchoCancellationState::Processing;
        Ok(output)
    }

    // None is internal EOF padding, never invented capture or reference.
    fn process_capture(&mut self, samples: Option<&[f32]>) -> Result<AudioBufferHandle, NodeError> {
        let mut buffer = self
            .output_pool
            .as_ref()
            .and_then(|pool| pool.acquire())
            .ok_or_else(|| failure("AEC output pool exhausted"))?;
        let processor = self
            .processor
            .as_ref()
            .ok_or_else(|| failure("AEC processor absent"))?;
        let channels = usize::from(self.configuration.reference_channels.count());
        let start = Instant::now();
        for block_index in 0..(self.configuration.frame_samples() / ENGINE_FRAME_SAMPLES) {
            let mut capture = [[0.0_f32; ENGINE_FRAME_SAMPLES]; 2];
            if let Some(samples) = samples {
                for (channel, values) in capture.iter_mut().take(channels).enumerate() {
                    for (sample, value) in values.iter_mut().enumerate() {
                        *value = samples
                            [(block_index * ENGINE_FRAME_SAMPLES + sample) * channels + channel];
                    }
                }
            }
            processor
                .process_capture_frame(capture[..channels].iter_mut().map(|c| c.as_mut_slice()))
                .map_err(failure)?;
            for (channel, values) in capture.iter().take(channels).enumerate() {
                for (sample, value) in values.iter().enumerate() {
                    buffer.as_mut_slice()
                        [(block_index * ENGINE_FRAME_SAMPLES + sample) * channels + channel] =
                        *value;
                }
            }
        }
        let duration_ns = start.elapsed().as_nanos().min(u128::from(u64::MAX)) as u64;
        self.diagnostics.latest_processing_duration_ns = duration_ns;
        self.diagnostics.maximum_processing_duration_ns = self
            .diagnostics
            .maximum_processing_duration_ns
            .max(duration_ns);
        Ok(buffer)
    }

    fn processing_metadata(
        &self,
        lineage: SignalLineage,
        timing: SignalTiming,
        padding_samples: u32,
        tail_offset_samples: u32,
    ) -> Result<AudioProcessing, NodeError> {
        Ok(AudioProcessing {
            input_source_id: lineage.source_id(),
            input_stream_id: lineage.stream_id(),
            input_sequence_number: lineage.sequence_number(),
            input_timestamp_ns: timing
                .source_timestamp_ns()
                .ok_or_else(|| failure("AEC input time missing"))?,
            input_duration_ns: timing
                .duration_ns()
                .ok_or_else(|| failure("AEC input duration missing"))?,
            input_source_generation: lineage.source_generation(),
            input_discontinuity_epoch: lineage.discontinuity_epoch(),
            generation: self.diagnostics.processing_generation,
            nominal_delay_samples: NOMINAL_DELAY_SAMPLES,
            padding_samples,
            tail_offset_samples,
        })
    }

    #[allow(clippy::too_many_arguments)]
    fn output(
        &mut self,
        buffer: AudioBufferHandle,
        input: SignalLineage,
        upstream_timing: SignalTiming,
        timing: SignalTiming,
        generation: Option<OutputGeneration>,
        metadata: AudioProcessing,
    ) -> Result<SignalEnvelope, NodeError> {
        let sequence = self.next_output_sequence;
        self.next_output_sequence = sequence
            .checked_add(1)
            .ok_or_else(|| failure("AEC output sequence exhausted"))?;
        let lineage = SignalLineage::try_new(
            input.session_id(),
            input.stream_id(),
            input.source_id(),
            input.clock_id(),
            sequence,
            input.source_generation(),
            input.discontinuity_epoch(),
            input.policy_epoch(),
        )
        .map_err(failure)?;
        let output = AudioFrame::try_new(
            lineage.stream_id(),
            lineage.source_id(),
            sequence,
            timing
                .source_timestamp_ns()
                .ok_or_else(|| failure("AEC output time missing"))?,
            SampleSpec::new(
                SAMPLE_RATE_HZ,
                self.configuration.reference_channels.count(),
                SampleFormat::F32Interleaved,
            ),
            buffer,
        )
        .map_err(failure)?
        .with_processing(metadata);
        self.diagnostics.output_frames_total += 1;
        // The generated-audio bridge assigns the declared derived source/stream.
        // Metadata retains the actual microphone input independently.
        Ok(SignalEnvelope::from_audio(output, None)
            .with_lineage(lineage, timing)
            .with_derivation(self.derivation(input, upstream_timing)?)
            .with_output_generation(generation))
    }

    pub(crate) fn stop(&mut self) {
        self.diagnostics.discarded_microphone_frames_total += self.microphone.len() as u64;
        self.diagnostics.discarded_reference_frames_total += self.reference.len() as u64;
        self.microphone.clear();
        self.reference.clear();
        if !self.flushed && self.last_microphone.take().is_some() {
            self.diagnostics.discarded_tail_generations_total += 1;
        }
        self.processor = None;
        self.output_pool = None;
        if self.diagnostics.last_error.is_none() {
            self.diagnostics.state = EchoCancellationState::Stopped;
        }
        self.publish();
    }
}

impl EchoProcessor {
    pub(crate) fn prepare(&mut self) -> Result<(), NodeError> {
        let processor =
            Processor::new(SAMPLE_RATE_HZ).map_err(|e| NodeError::Prepare(e.to_string()))?;
        processor.set_config(Config {
            echo_canceller: Some(EchoCanceller::Full {
                stream_delay_ms: None,
            }),
            pipeline: webrtc_audio_processing::config::Pipeline {
                multi_channel_render: self.configuration.reference_channels.count() > 1,
                multi_channel_capture: self.configuration.reference_channels.count() > 1,
                ..Default::default()
            },
            ..Default::default()
        });
        // APM's first capture configures its format and clears queued render.
        // Establish both formats privately, then reset that initialization audio
        // before accepting any source frame. Otherwise the first real reference
        // is lost and the first microphone block has a different signal delay.
        let channels = usize::from(self.configuration.reference_channels.count());
        let render = [[0.0_f32; ENGINE_FRAME_SAMPLES]; 2];
        let mut capture = render;
        processor
            .analyze_render_frame(render[..channels].iter().map(|c| c.as_slice()))
            .map_err(|error| NodeError::Prepare(error.to_string()))?;
        processor
            .process_capture_frame(capture[..channels].iter_mut().map(|c| c.as_mut_slice()))
            .map_err(|error| NodeError::Prepare(error.to_string()))?;
        processor.reinitialize();
        self.processor = Some(processor);
        self.output_pool = Some(AudioBufferPool::new(
            OUTPUT_POOL_FRAMES,
            self.configuration.frame_samples()
                * usize::from(self.configuration.reference_channels.count()),
        ));
        self.publish();
        Ok(())
    }

    pub(crate) fn flush(&mut self) -> Result<Vec<SignalEnvelope>, NodeError> {
        if self.flushed {
            return match self.diagnostics.last_error.as_ref() {
                Some(error) => Err(failure(error)),
                None => Ok(Vec::new()),
            };
        }
        // A failed drain cannot retry an already advanced native state. Its
        // discarded generation remains explicit, and failure is durable.
        self.flushed = true;
        let had_history = self.last_microphone.is_some();
        let result = self.drain();
        if let Err(error) = &result {
            if had_history {
                self.diagnostics.discarded_tail_generations_total += 1;
            }
            self.last_microphone = None;
            self.fail(error);
        }
        result
    }

    fn drain(&mut self) -> Result<Vec<SignalEnvelope>, NodeError> {
        if !self.microphone.is_empty() {
            return Err(failure("AEC ended with unpaired microphone frames"));
        }
        while let Some(reference) = self.reference.pop_front() {
            self.analyze_reference(reference)?;
        }
        let Some(last) = self.last_microphone.take() else {
            self.publish();
            return Ok(Vec::new());
        };
        let frame_ms = self.configuration.frame_duration_ms;
        let mut outputs = Vec::with_capacity((DRAIN_DURATION_MS / frame_ms) as usize);
        for offset_ms in (0..DRAIN_DURATION_MS).step_by(frame_ms as usize) {
            let delta_ns = u64::from(frame_ms + offset_ms) * 1_000_000;
            let advance = |time: Option<u64>| -> Result<Option<u64>, NodeError> {
                time.map(|t| {
                    t.checked_add(delta_ns)
                        .ok_or_else(|| failure("AEC tail timestamp overflow"))
                })
                .transpose()
            };
            let timing = SignalTiming::try_new(
                advance(last.timing.source_timestamp_ns())?,
                last.timing.observed_timestamp_ns(),
                advance(last.timing.session_timestamp_ns())?,
                last.timing.duration_ns(),
            )
            .map_err(failure)?;
            let metadata = self.processing_metadata(
                last.lineage,
                last.timing,
                self.configuration.frame_samples() as u32,
                offset_ms * 48,
            )?;
            let buffer = self.process_capture(None)?;
            outputs.push(self.output(
                buffer,
                last.lineage,
                last.timing,
                timing,
                last.generation.clone(),
                metadata,
            )?);
            self.diagnostics.tail_frames_total += 1;
            self.diagnostics.tail_padding_samples_total += u64::from(metadata.padding_samples);
        }
        // AEC3 continues comfort noise: bound drain at this termination policy.
        // Do not wait for zero output or claim all-state convergence.
        self.publish();
        Ok(outputs)
    }

    pub(crate) fn fail(&mut self, error: &NodeError) {
        self.diagnostics.state = EchoCancellationState::Failed;
        self.diagnostics.last_error = Some(error.to_string());
        self.publish();
    }
}
