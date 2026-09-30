//! Synthetic PCM source with explicit source-clock metadata, never a device proof.

use pocketstation::{
    AudioBufferPool, AudioCaps, AudioFrame, ChannelLayout, ClockDomainId, ConfigError,
    ExecutionPartition, ExecutionSafety, MediaCaps, Multiplicity, PortDirection, PortSpec,
    SampleFormat, SampleSpec, Session, SignalEnvelope, SignalLineage, SignalSpec, SignalTiming,
    SourceCancellation, SourceConfiguration, SourceDriver, SourceDriverError, SourceEmission,
    SourceFactory, SourceManifest, SourceOutputHandle, SourcePrepareContext, SourceSessionContext,
    SourceTypeId,
};
use std::sync::{mpsc, Arc, Mutex};
use std::time::Duration;

pub const FRAME_SAMPLES: usize = 960;
pub const FRAME_DURATION_NS: u64 = 20_000_000;
pub const CLOCK_EPOCH_NS: u64 = 5_000_000_000;

pub struct InputFrame {
    pub samples: Vec<f32>,
    pub timestamp_ns: u64,
    pub sequence_number: u64,
    pub source_generation: u32,
    pub discontinuity_epoch: u64,
}

impl InputFrame {
    pub fn new(sequence_number: u64, timestamp_ns: u64, samples: Vec<f32>) -> Self {
        assert_eq!(samples.len(), FRAME_SAMPLES);
        Self {
            samples,
            timestamp_ns,
            sequence_number,
            source_generation: 1,
            discontinuity_epoch: 0,
        }
    }
}

pub struct AecSource {
    pub output: SourceOutputHandle,
    sender: Option<mpsc::SyncSender<InputFrame>>,
    samples_per_frame: usize,
}

impl AecSource {
    pub fn declare(session: &Session, label: &str) -> Self {
        Self::declare_format(
            session,
            label,
            SampleSpec::new(48_000, 1, SampleFormat::F32Interleaved),
            FRAME_SAMPLES,
        )
    }

    pub fn declare_format(
        session: &Session,
        label: &str,
        sample_spec: SampleSpec,
        frame_samples: usize,
    ) -> Self {
        let source_type = SourceTypeId::new(format!("io.pocketstation.source.aec{label}.v1"))
            .expect("valid fixture source identifier");
        let (sender, receiver) = mpsc::sync_channel(8);
        let manifest = SourceManifest::new(
            source_type.clone(),
            1,
            1,
            vec![PortSpec::new(
                "audio",
                PortDirection::Output,
                SignalSpec::audio(),
                MediaCaps::Audio(AudioCaps {
                    sample_rate_hz: Some(sample_spec.sample_rate_hz),
                    frame_samples: Some(frame_samples),
                    channel_layout: if sample_spec.channels == 1 {
                        ChannelLayout::Mono
                    } else {
                        ChannelLayout::Stereo
                    },
                    format: SampleFormat::F32Interleaved,
                }),
                Multiplicity::Many,
                true,
            )
            .unwrap()],
            ExecutionPartition::BlockingWorker,
            ExecutionSafety::AllocationAllowed,
        )
        .unwrap();
        session
            .register_source(Arc::new(FixtureFactory {
                manifest,
                sample_spec,
                frame_samples,
                receiver: Mutex::new(Some(receiver)),
            }))
            .unwrap();
        let source = session
            .source(source_type, SourceConfiguration::default())
            .unwrap();
        Self {
            output: source.output("audio").unwrap(),
            sender: Some(sender),
            samples_per_frame: frame_samples * usize::from(sample_spec.channels),
        }
    }

    pub fn send(&self, frame: InputFrame) {
        assert_eq!(frame.samples.len(), self.samples_per_frame);
        assert!(frame.samples.iter().all(|sample| sample.is_finite()));
        self.sender
            .as_ref()
            .expect("fixture is open")
            .try_send(frame)
            .unwrap_or_else(|error| panic!("bounded fixture submission failed: {error}"));
    }

    pub fn close(&mut self) {
        self.sender.take();
    }
}

struct FixtureFactory {
    manifest: SourceManifest,
    sample_spec: SampleSpec,
    frame_samples: usize,
    receiver: Mutex<Option<mpsc::Receiver<InputFrame>>>,
}

impl SourceFactory for FixtureFactory {
    fn manifest(&self) -> &SourceManifest {
        &self.manifest
    }

    fn validate_config(&self, configuration: &SourceConfiguration) -> Result<(), ConfigError> {
        assert!(configuration.iter().next().is_none());
        Ok(())
    }

    fn create(
        &self,
        configuration: &SourceConfiguration,
    ) -> Result<Box<dyn SourceDriver>, SourceDriverError> {
        self.validate_config(configuration).unwrap();
        Ok(Box::new(FixtureDriver {
            receiver: self.receiver.lock().unwrap().take().unwrap(),
            session: None,
            pool: AudioBufferPool::new(
                16,
                self.frame_samples * usize::from(self.sample_spec.channels),
            ),
            sample_spec: self.sample_spec,
            duration_ns: self.frame_samples as u64 * 1_000_000_000
                / u64::from(self.sample_spec.sample_rate_hz),
        }))
    }
}

struct FixtureDriver {
    receiver: mpsc::Receiver<InputFrame>,
    session: Option<SourceSessionContext>,
    pool: Arc<AudioBufferPool>,
    sample_spec: SampleSpec,
    duration_ns: u64,
}

impl SourceDriver for FixtureDriver {
    fn prepare(&mut self, context: &SourcePrepareContext) -> Result<(), SourceDriverError> {
        self.session = context.session.clone();
        Ok(())
    }

    fn next(
        &mut self,
        cancellation: &SourceCancellation,
    ) -> Result<Option<SourceEmission>, SourceDriverError> {
        loop {
            if cancellation.is_cancelled() {
                return Ok(None);
            }
            let input = match self.receiver.recv_timeout(Duration::from_millis(1)) {
                Ok(input) => input,
                Err(mpsc::RecvTimeoutError::Timeout) => continue,
                Err(mpsc::RecvTimeoutError::Disconnected) => return Ok(None),
            };
            let session = self.session.as_ref().unwrap();
            let stream_id = session.output("audio").unwrap().stream_id;
            let mut buffer = self.pool.acquire().expect("fixture output pool is bounded");
            buffer.try_copy_from_slice(&input.samples).unwrap();
            let frame = AudioFrame::try_new(
                stream_id,
                session.source_id,
                input.sequence_number,
                input.timestamp_ns,
                self.sample_spec,
                buffer,
            )
            .unwrap();
            let lineage = SignalLineage::try_new(
                session.session_id,
                stream_id,
                session.source_id,
                ClockDomainId::new(1),
                input.sequence_number,
                input.source_generation,
                input.discontinuity_epoch,
                0,
            )
            .unwrap();
            let timing = SignalTiming::try_new(
                Some(input.timestamp_ns),
                input.timestamp_ns,
                Some(input.timestamp_ns),
                Some(self.duration_ns),
            )
            .unwrap();
            return Ok(Some(SourceEmission {
                output_port: "audio".into(),
                envelope: SignalEnvelope::from_audio(frame, None).with_lineage(lineage, timing),
                terminal: false,
            }));
        }
    }

    fn close(&mut self) -> Result<(), SourceDriverError> {
        Ok(())
    }
}
