use super::config::{ENGINE_FRAME_SAMPLES, OUTPUT_POOL_FRAMES, SAMPLE_RATE_HZ};
use super::{
    AecConfiguration, EchoCancellationObservations, EchoCancellationState, ObservationState,
};
use crate::{
    AudioBufferPool, AudioFrame, NodeError, OperatorId, SampleFormat, SampleSpec, SignalDerivation,
    SignalEnvelope, SignalLineage, SignalPayload, SignalTiming,
};
use std::{collections::VecDeque, sync::Arc, time::Instant};
use webrtc_audio_processing::{config::EchoCanceller, Config, Processor};

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
        }
    }

    fn reset(&mut self) {
        self.diagnostics.discarded_microphone_frames_total += self.microphone.len() as u64;
        self.diagnostics.discarded_reference_frames_total += self.reference.len() as u64;
        self.microphone.clear();
        self.reference.clear();
        self.previous = [None, None];
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
            let changed = last.source_id() != lineage.source_id()
                || last.stream_id() != lineage.stream_id()
                || last.clock_id() != lineage.clock_id()
                || last.source_generation() != lineage.source_generation()
                || last.policy_epoch() != lineage.policy_epoch()
                || last.discontinuity_epoch() != lineage.discontinuity_epoch()
                || last.sequence_number().checked_add(1) != Some(lineage.sequence_number())
                || last_time.timestamp_end_ns() != timing.source_timestamp_ns();
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
        while let (Some(mic), Some(reference)) = (self.microphone.front(), self.reference.front()) {
            let ml = mic
                .lineage()
                .ok_or_else(|| failure("missing microphone lineage"))?;
            let rl = reference
                .lineage()
                .ok_or_else(|| failure("missing reference lineage"))?;
            if ml.clock_id() != rl.clock_id() || ml.session_id() != rl.session_id() {
                return Err(failure(
                    "AEC inputs must share a Session and explicitly aligned clock",
                ));
            }
            let mt = mic
                .timing()
                .source_timestamp_ns()
                .ok_or_else(|| failure("missing microphone time"))?;
            let rt = reference
                .timing()
                .source_timestamp_ns()
                .ok_or_else(|| failure("missing reference time"))?;
            if mt.abs_diff(rt) > self.configuration.maximum_pair_skew_ns {
                // A stale reference can be dropped. Never silently discard microphone audio.
                if rt < mt {
                    self.reference.pop_front();
                    self.diagnostics.discarded_reference_frames_total += 1;
                    continue;
                }
                return Err(failure("AEC reference advanced beyond pending microphone; discontinuity requires explicit recovery"));
            }
            let mic = self
                .microphone
                .pop_front()
                .ok_or_else(|| failure("missing queued microphone"))?;
            let reference = self
                .reference
                .pop_front()
                .ok_or_else(|| failure("missing queued reference"))?;
            self.diagnostics.latest_pair_skew_ns = mt.abs_diff(rt);
            self.diagnostics.reference_source_id = Some(rl.source_id());
            self.diagnostics.microphone_source_id = Some(ml.source_id());
            outputs.push(self.process_pair(mic, reference)?);
        }
        if !self.microphone.is_empty() || self.diagnostics.processed_microphone_frames_total == 0 {
            self.diagnostics.state = EchoCancellationState::WaitingForReference;
        }
        self.publish();
        Ok(outputs)
    }

    fn process_pair(
        &mut self,
        microphone: SignalEnvelope,
        reference: SignalEnvelope,
    ) -> Result<SignalEnvelope, NodeError> {
        let lineage = microphone
            .lineage()
            .ok_or_else(|| failure("microphone lineage missing"))?;
        let timing = microphone.timing();
        let generation = microphone.output_generation().cloned();
        let SignalPayload::Audio(mic) = microphone.into_payload() else {
            return Err(failure("microphone payload changed"));
        };
        let SignalPayload::Audio(reference) = reference.into_payload() else {
            return Err(failure("reference payload changed"));
        };
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
            let mut render = [[0.0_f32; ENGINE_FRAME_SAMPLES]; 2];
            let mut capture = [[0.0_f32; ENGINE_FRAME_SAMPLES]; 2];
            for channel in 0..channels {
                for sample in 0..ENGINE_FRAME_SAMPLES {
                    let offset = (block_index * ENGINE_FRAME_SAMPLES + sample) * channels + channel;
                    render[channel][sample] = reference.samples()[offset];
                    capture[channel][sample] = mic.samples()[offset];
                }
            }
            processor
                .analyze_render_frame(render[..channels].iter().map(|c| c.as_slice()))
                .map_err(failure)?;
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
        self.diagnostics.processed_microphone_frames_total += 1;
        self.diagnostics.state = EchoCancellationState::Processing;
        let output = AudioFrame::try_new(
            mic.stream_id(),
            mic.source_id(),
            mic.sequence_number(),
            mic.timestamp_ns(),
            SampleSpec::new(SAMPLE_RATE_HZ, channels as u8, SampleFormat::F32Interleaved),
            buffer,
        )
        .map_err(failure)?;
        Ok(SignalEnvelope::from_audio(output, None)
            .with_lineage(lineage, timing)
            .with_derivation(self.derivation(lineage, timing)?)
            .with_output_generation(generation))
    }

    pub(crate) fn stop(&mut self) {
        self.diagnostics.discarded_microphone_frames_total += self.microphone.len() as u64;
        self.diagnostics.discarded_reference_frames_total += self.reference.len() as u64;
        self.microphone.clear();
        self.reference.clear();
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
        if !self.microphone.is_empty() {
            return Err(failure("AEC ended with unpaired microphone frames"));
        }
        Ok(Vec::new())
    }

    pub(crate) fn fail(&mut self, error: &NodeError) {
        self.diagnostics.state = EchoCancellationState::Failed;
        self.diagnostics.last_error = Some(error.to_string());
        self.publish();
    }
}
