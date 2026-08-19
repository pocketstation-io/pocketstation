use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

use pocketstation::{
    AsyncNode, AsyncNodeFuture, AsyncOperatorFactory, AsyncOperatorManifest, AudioCaps,
    AudioInputBufferAcquireError, AudioInputBufferError, AudioInputConfig,
    AudioInputWriteErrorKind, CallbackCaptureBackend, CaptureError, CaptureMode, ChannelLayout,
    ConfigError, CopyPolicy, EdgeContract, ExecutionPartition, MediaCaps, Multiplicity,
    NodeDescriptor, NodeError, Operator, OperatorCancellationPolicy, OperatorConfiguration,
    OperatorDeadlinePolicy, OperatorFailurePolicy, OperatorId, OperatorOutputRolePolicy,
    OperatorPermissionPolicy, PortDirection, PortSpec, PreparedCaptureBackend, SafetyContract,
    SampleFormat, SampleSpec, Session, SessionRecordingState, SignalDerivation, SignalEnvelope,
    SignalPayload, SignalSpec,
};

const FRAME_SAMPLES: usize = 960;
const OPERATOR_ID: &str = "dev.pocketstation.test.pcm-source-pass-through.v1";
const OPERATOR_NODE_ID: &str = "dev.pocketstation.test.pcm-source-pass-through-node.v1";

struct UnexpectedOsCapture;

impl CallbackCaptureBackend for UnexpectedOsCapture {
    fn prepare(&self, _mode: CaptureMode) -> Result<Box<dyn PreparedCaptureBackend>, CaptureError> {
        panic!("caller-owned PCM must not invoke an OS capture backend")
    }
}

fn sample_spec() -> SampleSpec {
    SampleSpec::new(48_000, 1, SampleFormat::F32Interleaved)
}

fn audio_input_config(capacity_frames: usize) -> AudioInputConfig {
    AudioInputConfig::new(sample_spec(), capacity_frames, FRAME_SAMPLES)
        .expect("valid audio input configuration")
}

#[test]
fn given_application_owned_audio_when_written_through_facade_then_session_delivers_its_lineage() {
    let session = Session::builder().sample_spec(sample_spec()).build();
    let mut input = session
        .audio_input(audio_input_config(2))
        .expect("application audio input");
    let source_id = input.source().source_id();
    let stream_id = input.output().stream_id();
    let polled_audio = session.polled_audio().expect("polled audio endpoint");
    input
        .output()
        .send(polled_audio)
        .expect("audio input polling route");

    let mut running = session.start().expect("running audio input Session");
    input
        .try_write(&vec![0.25_f32; FRAME_SAMPLES])
        .expect("nonblocking façade write");

    let deadline = Instant::now() + Duration::from_secs(3);
    let (delivered_source_id, delivered_stream_id) = loop {
        if let Ok(batch) = running.try_poll_audio() {
            if let Some(frame) = batch.frame(0) {
                break (frame.lineage().source_id(), frame.stream_id());
            }
        }
        assert!(Instant::now() < deadline, "façade frame was not delivered");
        std::thread::yield_now();
    };
    assert_eq!(delivered_source_id, source_id);
    assert_eq!(delivered_stream_id, stream_id);

    input.close();
    assert!(running.stop().is_success());
}

#[test]
fn given_bounded_audio_input_when_writes_are_invalid_or_saturated_then_ownership_is_explicit() {
    let session = Session::new();
    let first = session
        .pcm_source(audio_input_config(1))
        .expect("first PCM source");
    let second = session
        .pcm_source(audio_input_config(1))
        .expect("second PCM source");
    assert_ne!(first.source().source_id(), second.source().source_id());
    assert_ne!(first.output().stream_id(), second.output().stream_id());
    let different_frame_size = session
        .pcm_source(
            AudioInputConfig::new(sample_spec(), 2, FRAME_SAMPLES / 2)
                .expect("valid second per-instance frame contract"),
        )
        .expect_err("one compiled Session requires one concrete PCM frame contract");
    assert!(matches!(
        different_frame_size,
        pocketstation::AudioInputError::IncompatibleContract
    ));
    let (_, _, mut writer) = first.into_parts();
    let (_, _, second_writer) = second.into_parts();
    let samples = vec![0.25_f32; FRAME_SAMPLES];

    writer.try_write(&samples).expect("first bounded write");
    let full = writer.try_write(&samples).expect_err("queue must saturate");
    assert_eq!(full.kind(), AudioInputWriteErrorKind::Full);
    drop(full.into_rejected());

    let empty = writer.try_acquire().expect("rejection reserve buffer");
    let empty = writer.try_send(empty).expect_err("empty buffer must fail");
    assert_eq!(
        empty.kind(),
        AudioInputWriteErrorKind::InvalidBuffer(AudioInputBufferError::Empty)
    );
    drop(empty.into_rejected());

    let mut short = writer.try_acquire().expect("short buffer");
    short
        .try_copy_from_slice(&samples[..FRAME_SAMPLES - 1])
        .expect("short copy fits");
    let short = writer
        .try_send(short)
        .expect_err("fixed PCM frame size must be enforced");
    assert_eq!(
        short.kind(),
        AudioInputWriteErrorKind::InvalidBuffer(AudioInputBufferError::WrongFrameLength {
            expected_samples: FRAME_SAMPLES,
            actual_samples: FRAME_SAMPLES - 1,
        })
    );
    drop(short.into_rejected());

    let mut foreign = second_writer.try_acquire().expect("foreign buffer");
    foreign
        .try_copy_from_slice(&samples)
        .expect("foreign frame copy");
    let foreign = writer
        .try_send(foreign)
        .expect_err("buffers cannot cross audio inputs");
    assert_eq!(
        foreign.kind(),
        AudioInputWriteErrorKind::InvalidBuffer(AudioInputBufferError::WrongSource)
    );
    drop(foreign.into_rejected());

    let observations = writer.observations();
    assert_eq!(observations.capacity_frames, 1);
    assert_eq!(observations.buffer_slots, 2);
    assert_eq!(observations.available_buffers, 1);
    assert_eq!(observations.accepted_total, 1);
    assert_eq!(observations.full_total, 1);
    assert_eq!(observations.invalid_total, 3);

    writer.close();
    assert!(matches!(
        writer.try_acquire(),
        Err(AudioInputBufferAcquireError::Closed)
    ));
}

#[test]
fn given_running_audio_input_when_writer_closes_then_accepted_frames_are_drained() {
    let session = Session::builder().sample_spec(sample_spec()).build();
    let pcm = session
        .pcm_source(audio_input_config(4))
        .expect("PCM source");
    let source_id = pcm.source().source_id();
    let stream_id = pcm.output().stream_id();
    let polled_audio = session.polled_audio().expect("polled audio endpoint");
    pcm.output()
        .send(polled_audio)
        .expect("audio input polling route");
    let (_, _, mut writer) = pcm.into_parts();
    let mut running = session.start().expect("running audio input Session");
    let samples = vec![0.25_f32; FRAME_SAMPLES];

    for _ in 0..4 {
        let mut buffer = acquire_before(&writer, Duration::from_secs(2));
        buffer
            .try_copy_from_slice(&samples)
            .expect("PCM frame copy");
        send_before(&mut writer, buffer, Duration::from_secs(2));
    }
    writer.close();

    let deadline = Instant::now() + Duration::from_secs(5);
    let mut sequences = Vec::new();
    while sequences.len() < 4 && Instant::now() < deadline {
        if let Ok(batch) = running.try_poll_audio() {
            for index in 0..batch.len() {
                let frame = batch.frame(index).expect("bounded batch frame");
                assert_eq!(frame.stream_id(), stream_id);
                assert_eq!(frame.lineage().source_id(), source_id);
                sequences.push(frame.lineage().sequence_number());
            }
        }
        thread::sleep(Duration::from_millis(1));
    }

    assert_eq!(sequences, vec![0, 1, 2, 3]);
    assert!(running.stop().is_success());
    let observations = writer.observations();
    assert!(observations.closed);
    assert!(!observations.cancelled);
    assert_eq!(observations.accepted_total, 4);
    assert_eq!(observations.available_buffers, observations.buffer_slots);
    assert_eq!(
        writer
            .try_write(&samples)
            .expect_err("closed audio input")
            .kind(),
        AudioInputWriteErrorKind::Closed
    );
}

#[derive(Default)]
struct OperatorControl {
    processed_total: AtomicU64,
}

struct PassThroughFactory {
    manifest: AsyncOperatorManifest,
    control: Arc<OperatorControl>,
}

impl PassThroughFactory {
    fn new(control: Arc<OperatorControl>) -> Self {
        let media = MediaCaps::Audio(AudioCaps {
            sample_rate_hz: Some(48_000),
            frame_samples: Some(FRAME_SAMPLES),
            channel_layout: ChannelLayout::Mono,
            format: SampleFormat::F32Interleaved,
        });
        let node = NodeDescriptor::new(
            pocketstation::NodeTypeId::from(OPERATOR_NODE_ID),
            "audio input pass-through",
            vec![PortSpec::new(
                "input",
                PortDirection::Input,
                SignalSpec::audio(),
                media,
                Multiplicity::One,
                true,
            )
            .expect("input port")],
            vec![PortSpec::new(
                "output",
                PortDirection::Output,
                SignalSpec::audio(),
                media,
                Multiplicity::One,
                true,
            )
            .expect("output port")],
            ExecutionPartition::AsyncWorker,
            SafetyContract::AllocationAllowed,
            false,
        )
        .expect("operator node");
        let input_edge =
            EdgeContract::realtime_audio().with_copy_policy(CopyPolicy::CopyToBranchPool);
        let output_edge = EdgeContract::bounded_async().with_media(media);
        let manifest = AsyncOperatorManifest::new(
            OperatorId::new(OPERATOR_ID),
            1,
            1,
            node,
            input_edge,
            output_edge,
            8,
            OperatorPermissionPolicy {
                network_allowed: false,
                filesystem_allowed: false,
            },
            OperatorDeadlinePolicy {
                process_timeout_ms: 500,
            },
            OperatorCancellationPolicy::DrainQueued,
            OperatorFailurePolicy::StopWorker,
            OperatorOutputRolePolicy::default(),
        )
        .expect("operator manifest");
        Self { manifest, control }
    }
}

impl AsyncOperatorFactory for PassThroughFactory {
    fn manifest(&self) -> &AsyncOperatorManifest {
        &self.manifest
    }

    fn validate_config(&self, _configuration: &OperatorConfiguration) -> Result<(), ConfigError> {
        Ok(())
    }

    fn create(
        &self,
        _configuration: &OperatorConfiguration,
    ) -> Result<Box<dyn AsyncNode>, NodeError> {
        Ok(Box::new(PassThroughNode {
            control: Arc::clone(&self.control),
        }))
    }
}

struct PassThroughNode {
    control: Arc<OperatorControl>,
}

impl AsyncNode for PassThroughNode {
    fn prepare<'a>(
        &'a mut self,
        _context: &'a pocketstation::AsyncOperatorPrepareContext,
    ) -> AsyncNodeFuture<'a, Result<(), NodeError>> {
        Box::pin(async { Ok(()) })
    }

    fn process<'a>(
        &'a mut self,
        input: SignalEnvelope,
    ) -> AsyncNodeFuture<'a, Result<Vec<SignalEnvelope>, NodeError>> {
        Box::pin(async move {
            let lineage = input.lineage().ok_or_else(|| {
                NodeError::Process("audio input input omitted lineage".to_owned())
            })?;
            let timing = input.timing();
            let SignalPayload::Audio(frame) = input.into_payload() else {
                return Err(NodeError::Process(
                    "audio input input was not audio".to_owned(),
                ));
            };
            self.control.processed_total.fetch_add(1, Ordering::Relaxed);
            let output = SignalEnvelope::from_audio(frame, None)
                .with_lineage(lineage, timing)
                .with_derivation(
                    SignalDerivation::new(
                        lineage,
                        timing,
                        OperatorId::new(OPERATOR_ID),
                        1,
                        1,
                        None,
                    )
                    .map_err(|error| NodeError::Process(error.to_string()))?,
                );
            Ok(vec![output])
        })
    }
}

#[test]
fn given_audio_input_when_session_runs_then_lineage_fanout_reentry_and_recording_are_real() {
    let recording_root = tempfile::tempdir().expect("temporary recording root");
    let unexpected_capture: Arc<dyn CallbackCaptureBackend> = Arc::new(UnexpectedOsCapture);
    let session = Session::builder()
        .recording_root(recording_root.path())
        .sample_spec(sample_spec())
        .capture_backends(Arc::clone(&unexpected_capture), unexpected_capture)
        .build();
    let control = Arc::new(OperatorControl::default());
    session
        .register_operator(Arc::new(PassThroughFactory::new(Arc::clone(&control))))
        .expect("operator registration");

    let pcm = session
        .pcm_source(audio_input_config(8))
        .expect("PCM source");
    let source_id = pcm.source().source_id();
    let stream_id = pcm.output().stream_id();
    let polled_audio = session.polled_audio().expect("polled audio endpoint");
    pcm.output()
        .send(polled_audio)
        .expect("audio input polling route");
    pcm.output()
        .record("application-owned")
        .expect("source recording");

    let generated = pcm
        .output()
        .through(Operator::new(
            OperatorId::new(OPERATOR_ID),
            OperatorConfiguration::new(),
        ))
        .expect("operator route")
        .reenter_audio()
        .expect("generated audio stem");
    let generated_stem_id = generated.id();
    generated
        .send(polled_audio)
        .expect("generated audio polling route");
    generated
        .record("operator-output")
        .expect("generated audio recording");

    #[cfg(feature = "conformance-fixtures")]
    let connector_route = {
        let connector = pocketstation::conformance::observed_connector(&session, Duration::ZERO)
            .expect("observed connector");
        pcm.output()
            .send(connector)
            .expect("audio input connector route")
    };

    let (_, _, mut writer) = pcm.into_parts();
    let mut running = session.start().expect("running audio input Session");
    let samples = vec![0.5_f32; FRAME_SAMPLES];
    for sequence in 0..4 {
        let mut buffer = acquire_before(&writer, Duration::from_secs(2));
        buffer
            .try_copy_from_slice(&samples)
            .expect("PCM frame copy");
        if sequence == 2 {
            buffer.mark_discontinuity();
        }
        send_before(&mut writer, buffer, Duration::from_secs(2));
    }
    let deadline = Instant::now() + Duration::from_secs(5);
    let mut source_lineages = Vec::new();
    let mut generated_lineages = Vec::new();
    while (source_lineages.len() < 4 || generated_lineages.len() < 4) && Instant::now() < deadline {
        if let Ok(batch) = running.try_poll_audio() {
            for index in 0..batch.len() {
                let frame = batch.frame(index).expect("bounded batch frame");
                let lineage = frame.lineage();
                if lineage.source_id() == source_id {
                    assert_eq!(frame.stream_id(), stream_id);
                    source_lineages.push(lineage);
                } else {
                    assert_eq!(lineage.stem_id(), generated_stem_id);
                    generated_lineages.push(lineage);
                }
            }
        }
        thread::sleep(Duration::from_millis(1));
    }
    assert_eq!(
        source_lineages.len(),
        4,
        "all source PCM frames must be polled"
    );
    assert_eq!(
        generated_lineages.len(),
        4,
        "all operator PCM frames must be polled"
    );
    assert_eq!(control.processed_total.load(Ordering::Acquire), 4);
    assert_eq!(
        source_lineages
            .iter()
            .map(|lineage| lineage.sequence_number())
            .collect::<Vec<_>>(),
        vec![0, 1, 2, 3]
    );
    assert_eq!(
        source_lineages
            .iter()
            .map(|lineage| lineage.discontinuity_epoch())
            .collect::<Vec<_>>(),
        vec![0, 0, 1, 1]
    );
    assert_eq!(
        generated_lineages
            .iter()
            .map(|lineage| lineage.sequence_number())
            .collect::<Vec<_>>(),
        vec![0, 1, 2, 3]
    );
    assert_eq!(
        generated_lineages
            .iter()
            .map(|lineage| lineage.discontinuity_epoch())
            .collect::<Vec<_>>(),
        vec![0, 0, 1, 1]
    );

    #[cfg(feature = "conformance-fixtures")]
    wait_for_route_delivery(
        &running,
        connector_route.get(),
        4,
        Instant::now() + Duration::from_secs(5),
    );

    let source_metrics = running.external_source_metrics();
    assert_eq!(source_metrics.len(), 1);
    assert_eq!(source_metrics[0].source_id, source_id);
    assert_eq!(source_metrics[0].runtime.emitted_total, 4);

    let stop = running.stop();
    assert!(stop.is_success(), "audio input Session must stop cleanly");
    let recording = running
        .recording_outcome()
        .expect("multistem recording outcome");
    assert_eq!(recording.state, SessionRecordingState::Complete);
    assert_eq!(recording.completed_stems, 2);
    assert_eq!(recording.failed_stems, 0);

    let observations = writer.observations();
    assert!(observations.cancelled);
    assert_eq!(observations.accepted_total, 4);
    assert_eq!(observations.available_buffers, observations.buffer_slots);
    assert_eq!(
        writer
            .try_write(&samples)
            .expect_err("stopped Session")
            .kind(),
        AudioInputWriteErrorKind::Cancelled
    );

    assert_ne!(source_id.get(), 0);
    assert_ne!(stream_id.get(), 0);
    assert_ne!(generated_stem_id.get(), 0);
}

fn acquire_before(
    writer: &pocketstation::AudioInputWriter,
    timeout: Duration,
) -> pocketstation::AudioInputBuffer {
    let deadline = Instant::now() + timeout;
    loop {
        match writer.try_acquire() {
            Ok(buffer) => return buffer,
            Err(AudioInputBufferAcquireError::Full) if Instant::now() < deadline => {
                thread::sleep(Duration::from_millis(1));
            }
            result => panic!("PCM buffer was not acquired before deadline: {result:?}"),
        }
    }
}

fn send_before(
    writer: &mut pocketstation::AudioInputWriter,
    mut buffer: pocketstation::AudioInputBuffer,
    timeout: Duration,
) {
    let deadline = Instant::now() + timeout;
    loop {
        match writer.try_send(buffer) {
            Ok(()) => return,
            Err(error)
                if error.kind() == AudioInputWriteErrorKind::Full && Instant::now() < deadline =>
            {
                buffer = error
                    .into_rejected()
                    .expect("full write returns its buffer");
                thread::sleep(Duration::from_millis(1));
            }
            Err(error) => panic!("PCM frame was not accepted before deadline: {error}"),
        }
    }
}

#[cfg(feature = "conformance-fixtures")]
fn wait_for_route_delivery(
    running: &pocketstation::RunningSession,
    route_id: u64,
    expected_frames: u64,
    deadline: Instant,
) {
    loop {
        let snapshot = running.metrics_snapshot().expect("Session metrics");
        if (0..snapshot.route_count())
            .filter_map(|index| snapshot.route(index))
            .find(|route| route.route_id.get() == route_id)
            .is_some_and(|route| {
                route.edge.frames_delivered_total >= expected_frames
                    && route
                        .endpoint
                        .is_some_and(|endpoint| endpoint.frames_received_total >= expected_frames)
            })
        {
            return;
        }
        assert!(
            Instant::now() < deadline,
            "connector route delivery timed out"
        );
        thread::sleep(Duration::from_millis(1));
    }
}
