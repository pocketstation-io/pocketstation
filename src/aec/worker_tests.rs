use super::*;
use crate::EchoCancellationState;
use crate::{
    AudioBufferPool, AudioFrame, ClockDomainId, SampleFormat, SampleSpec, SessionId, SignalLineage,
    SignalPayload, SignalTiming, SourceId, StreamId,
};
use std::sync::Arc;
use std::time::Duration;

// This fixture delays delivery to the real native worker. It neither replaces
// APM nor changes Session's production manifest, timeout or cancellation policy.
struct DelayedCommandFactory {
    factory: crate::aec::AecOperatorFactory,
    barrier: std::sync::Mutex<Option<CommandBarrier>>,
    joined: Arc<std::sync::atomic::AtomicBool>,
}

struct CommandBarrier {
    entered: SyncSender<()>,
    release: mpsc::Receiver<()>,
}

struct DelayedCommandWorker {
    worker: AecWorker,
    barrier: Option<CommandBarrier>,
    relay: Option<JoinHandle<()>>,
    joined: Arc<std::sync::atomic::AtomicBool>,
}

impl crate::AsyncOperatorFactory for DelayedCommandFactory {
    fn manifest(&self) -> &crate::AsyncOperatorManifest {
        self.factory.manifest()
    }

    fn validate_config(
        &self,
        configuration: &crate::OperatorConfiguration,
    ) -> Result<(), crate::ConfigError> {
        self.factory.validate_config(configuration)
    }

    fn create(
        &self,
        configuration: &crate::OperatorConfiguration,
    ) -> Result<Box<dyn AsyncNode>, NodeError> {
        self.validate_config(configuration)
            .map_err(|error| NodeError::Prepare(error.to_string()))?;
        Ok(Box::new(DelayedCommandWorker {
            worker: AecWorker::new(
                self.factory.configuration,
                self.factory.operator_id.clone(),
                self.factory.observations.clone(),
            ),
            barrier: self.barrier.lock().unwrap().take(),
            relay: None,
            joined: self.joined.clone(),
        }))
    }
}

impl DelayedCommandWorker {
    async fn join(&mut self) -> Result<(), NodeError> {
        self.worker.sender.take();
        if let Some(relay) = self.relay.as_mut() {
            relay
                .await
                .map_err(|error| NodeError::Process(error.to_string()))?;
            self.relay.take();
        }
        self.worker.stop().await?;
        self.joined
            .store(true, std::sync::atomic::Ordering::Release);
        Ok(())
    }
}

impl AsyncNode for DelayedCommandWorker {
    fn prepare<'a>(
        &'a mut self,
        cx: &'a AsyncOperatorPrepareContext,
    ) -> AsyncNodeFuture<'a, Result<(), NodeError>> {
        Box::pin(async move {
            self.worker.prepare(cx).await?;
            let barrier = self.barrier.take().expect("one prepared fixture");
            let native = self.worker.sender.take().expect("prepared native sender");
            let (sender, receiver) = mpsc::sync_channel(1);
            self.worker.sender = Some(sender);
            self.relay = Some(tokio::task::spawn_blocking(move || {
                while let Ok(command) = receiver.recv() {
                    let held =
                        matches!(&command, Command::Process { port, .. } if port == "microphone");
                    if held {
                        let _ = barrier.entered.try_send(());
                        // A disconnected test guard also releases teardown on panic.
                        let _ = barrier.release.recv_timeout(Duration::from_secs(2));
                    }
                    if native.send(command).is_err() || held {
                        break;
                    }
                }
            }));
            Ok(())
        })
    }

    fn process<'a>(
        &'a mut self,
        input: SignalEnvelope,
    ) -> AsyncNodeFuture<'a, Result<Vec<SignalEnvelope>, NodeError>> {
        self.worker.process(input)
    }

    fn process_port<'a>(
        &'a mut self,
        port: &'a str,
        input: SignalEnvelope,
    ) -> AsyncNodeFuture<'a, Result<Vec<SignalEnvelope>, NodeError>> {
        self.worker.process_port(port, input)
    }

    fn flush<'a>(&'a mut self) -> AsyncNodeFuture<'a, Result<Vec<SignalEnvelope>, NodeError>> {
        self.worker.flush()
    }

    fn cancel<'a>(&'a mut self) -> AsyncNodeFuture<'a, Result<(), NodeError>> {
        Box::pin(self.join())
    }

    fn close<'a>(&'a mut self) -> AsyncNodeFuture<'a, Result<(), NodeError>> {
        Box::pin(self.join())
    }
}

#[test]
fn given_delayed_native_command_when_session_deadline_expires_then_application_continues_and_late_output_is_discarded(
) {
    use crate::{AsyncOperatorFactory, AudioInputConfig, PlaybackReference, Session};
    use std::sync::atomic::{AtomicBool, Ordering};

    let session = Session::new();
    let input_configuration = AudioInputConfig::new(
        SampleSpec::new(48_000, 1, SampleFormat::F32Interleaved),
        8,
        960,
    )
    .unwrap();
    let mut microphone = session.audio_input(input_configuration).unwrap();
    let mut reference = session.audio_input(input_configuration).unwrap();
    let mut application = session.audio_input(input_configuration).unwrap();
    let processed = session
        .echo_cancel(
            microphone.output(),
            PlaybackReference::rendered_audio(reference.output()),
        )
        .unwrap();
    let endpoint = session.polled_audio().unwrap();
    processed.audio().send(endpoint).unwrap();
    application.output().send(endpoint).unwrap();
    let observations = ObservationState::new(4);
    let joined = Arc::new(AtomicBool::new(false));
    let (entered, pending) = mpsc::sync_channel(1);
    let (release, barrier_release) = mpsc::sync_channel(1);
    {
        // Replace only the test instance's command transport after normal Session
        // composition. All execution settings come from the production factory.
        let mut factories = session.operator_registrations.lock().unwrap();
        assert_eq!(factories.len(), 1);
        let operator_id = factories[0].manifest().operator_id.clone();
        let factory = crate::aec::AecOperatorFactory::new(
            AecConfiguration::new(20, crate::aec::Channels::Mono),
            operator_id,
            observations.clone(),
        )
        .unwrap();
        assert_eq!(factory.manifest().deadline.process_timeout_ms, 100);
        factories[0] = Arc::new(DelayedCommandFactory {
            factory,
            barrier: std::sync::Mutex::new(Some(CommandBarrier {
                entered,
                release: barrier_release,
            })),
            joined: joined.clone(),
        });
    }
    let mut running = session.start().unwrap();
    // Declared after RunningSession so unwinding disconnects the barrier first.
    let release = release;
    reference.try_write(&[0.1; 960]).unwrap();
    let deadline = Instant::now() + Duration::from_secs(2);
    while observations.snapshot().reference_queue_depth_frames != 1 {
        assert!(
            Instant::now() < deadline,
            "reference did not reach native APM"
        );
        std::thread::yield_now();
    }
    microphone.try_write(&[0.2; 960]).unwrap();
    pending.recv_timeout(Duration::from_secs(2)).unwrap();
    application.try_write(&[0.25; 960]).unwrap();
    let deadline = Instant::now() + Duration::from_secs(2);
    let mut application_delivered = false;
    loop {
        if let Ok(batch) = running.try_poll_audio() {
            for index in 0..batch.len() {
                let frame = batch.frame(index).unwrap();
                assert_eq!(
                    frame.lineage().source_id(),
                    application.source().source_id()
                );
                assert_eq!(frame.samples(), &[0.25; 960]);
                application_delivered = true;
            }
        }
        let metrics = running.metrics_snapshot().unwrap();
        if application_delivered
            && observations.snapshot().interrupted_requests_total == 1
            && metrics.operator(0).unwrap().worker.timeout_total == 1
        {
            break;
        }
        assert!(
            Instant::now() < deadline,
            "Session failed to isolate the native wait"
        );
        std::thread::yield_now();
    }
    assert_eq!(observations.snapshot().processed_microphone_frames_total, 0);
    assert!(!joined.load(Ordering::Acquire));
    release.try_send(()).unwrap();
    let deadline = Instant::now() + Duration::from_secs(2);
    while !joined.load(Ordering::Acquire) {
        assert!(
            Instant::now() < deadline,
            "relay and native worker were not joined"
        );
        std::thread::yield_now();
    }
    let late = observations.snapshot();
    assert_eq!(late.state, EchoCancellationState::Interrupted);
    assert_eq!(late.processed_microphone_frames_total, 1);
    assert_eq!(late.analyzed_reference_frames_total, 1);
    assert_eq!(late.output_frames_total, 1);
    assert_eq!(late.discarded_output_frames_total, 1);
    assert!(late.latest_processing_duration_ns >= 100_000_000);
    application.try_write(&[0.375; 960]).unwrap();
    let deadline = Instant::now() + Duration::from_secs(2);
    loop {
        if let Ok(batch) = running.try_poll_audio() {
            assert_eq!(batch.len(), 1, "late native audio escaped into delivery");
            let frame = batch.frame(0).unwrap();
            assert_eq!(
                frame.lineage().source_id(),
                application.source().source_id()
            );
            assert_eq!(frame.samples(), &[0.375; 960]);
            break;
        }
        assert!(
            Instant::now() < deadline,
            "application stopped after AEC failed"
        );
        std::thread::yield_now();
    }
    assert!(
        running.try_poll_audio().is_err(),
        "late audio reached a consumer"
    );
    microphone.close();
    reference.close();
    application.close();
    assert!(!running.stop().is_success());
    let metrics = running.metrics_snapshot().unwrap();
    assert_eq!(metrics.operator(0).unwrap().worker.timeout_total, 1);
    assert!(metrics.operator(0).unwrap().worker.joined);
    let stopped = observations.snapshot();
    assert_eq!(stopped.state, late.state);
    assert_eq!(stopped.interrupted_requests_total, 1);
    assert_eq!(stopped.discarded_output_frames_total, 1);
    assert_eq!(stopped.output_frames_total, 1);
}

#[tokio::test]
async fn given_waiting_native_reply_when_future_is_cancelled_then_interruption_is_observable() {
    let state = ObservationState::new(4);
    let result = tokio::time::timeout(Duration::from_millis(1), async {
        let _response = PendingResponse::new(state.clone());
        std::future::pending::<()>().await;
    })
    .await;
    assert!(result.is_err());
    let observed = state.snapshot();
    assert_eq!(observed.state, EchoCancellationState::Interrupted);
    assert!(observed.last_error.is_none());
    assert_eq!(observed.interrupted_requests_total, 1);
}

#[test]
fn given_completed_reply_when_await_guard_drops_then_it_does_not_report_cancellation() {
    let state = ObservationState::new(4);
    let mut response = PendingResponse::new(state.clone());
    response.complete(&Ok(()));
    drop(response);
    assert_eq!(
        state.snapshot().state,
        EchoCancellationState::WaitingForReference
    );
    assert!(state.snapshot().last_error.is_none());
}

#[test]
fn given_abandoned_reply_when_native_command_finishes_then_outputs_are_counted_and_worker_stops() {
    for flush in [false, true] {
        let (mut processor, state, configuration, pool) = prepared_processor(20, false);
        let stamp = initial_stamp();
        processor
            .accept(
                "reference",
                pcm_input(configuration, &pool, 20, stamp, false),
            )
            .unwrap();
        if flush {
            let output = processor
                .accept(
                    "microphone",
                    pcm_input(configuration, &pool, 10, stamp, true),
                )
                .unwrap();
            assert_eq!(output.len(), 1);
        }
        let (reply, response) = oneshot::channel();
        let pending = PendingResponse::new(state.clone());
        drop(response);
        drop(pending);
        let submitted_at = Instant::now()
            .checked_sub(Duration::from_millis(20))
            .unwrap();
        let command = if flush {
            Command::Flush {
                reply,
                submitted_at,
            }
        } else {
            Command::Process {
                port: "microphone".into(),
                input: Box::new(pcm_input(configuration, &pool, 10, stamp, true)),
                reply,
                submitted_at,
            }
        };
        assert!(!execute_command(&mut processor, command, &state));
        processor.stop();
        let observed = state.snapshot();
        assert_eq!(observed.state, EchoCancellationState::Interrupted);
        assert_eq!(observed.interrupted_requests_total, 1);
        assert!(observed.last_error.is_none());
        assert_eq!(observed.processed_microphone_frames_total, 1);
        assert_eq!(observed.discarded_microphone_frames_total, 0);
        assert_eq!(
            observed.discarded_output_frames_total,
            if flush { 2 } else { 1 }
        );
        assert_eq!(observed.output_frames_total, if flush { 3 } else { 1 });
        assert!(observed.latest_processing_duration_ns >= 20_000_000);
        assert!(observed.maximum_processing_duration_ns >= observed.latest_processing_duration_ns);
    }
}

#[test]
fn given_delivered_native_reply_when_command_finishes_then_output_is_not_discarded() {
    let (mut processor, state, configuration, pool) = prepared_processor(20, false);
    let stamp = initial_stamp();
    processor
        .accept(
            "reference",
            pcm_input(configuration, &pool, 20, stamp, false),
        )
        .unwrap();
    let (reply, mut response) = oneshot::channel();
    assert!(execute_command(
        &mut processor,
        Command::Process {
            port: "microphone".into(),
            input: Box::new(pcm_input(configuration, &pool, 10, stamp, false)),
            reply,
            submitted_at: Instant::now(),
        },
        &state,
    ));
    assert_eq!(response.try_recv().unwrap().into_result().unwrap().len(), 1);
    processor.stop();
    let observed = state.snapshot();
    assert_eq!(observed.state, EchoCancellationState::Stopped);
    assert_eq!(observed.discarded_output_frames_total, 0);
    assert_eq!(observed.interrupted_requests_total, 0);
    assert!(observed.maximum_processing_duration_ns > 0);
}

#[test]
fn given_enqueued_reply_when_receiver_drops_before_poll_then_late_stop_retains_discarded_output() {
    let (mut processor, state, configuration, pool) = prepared_processor(20, false);
    let stamp = initial_stamp();
    processor
        .accept(
            "reference",
            pcm_input(configuration, &pool, 20, stamp, false),
        )
        .unwrap();
    let (reply, response) = oneshot::channel();
    let pending = PendingResponse::new(state.clone());
    assert!(execute_command(
        &mut processor,
        Command::Process {
            port: "microphone".into(),
            input: Box::new(pcm_input(configuration, &pool, 10, stamp, true)),
            reply,
            submitted_at: Instant::now(),
        },
        &state
    ));
    assert_eq!(state.snapshot().discarded_output_frames_total, 0);
    drop(response);
    drop(pending);
    processor.stop();
    let observed = state.snapshot();
    assert_eq!(observed.state, EchoCancellationState::Interrupted);
    assert_eq!(observed.output_frames_total, 1);
    assert_eq!(observed.discarded_output_frames_total, 1);
    assert_eq!(observed.interrupted_requests_total, 1);
    assert!(observed.last_error.is_none());
}

#[derive(Clone, Copy)]
struct InputStamp {
    sequence: u64,
    timestamp_ns: u64,
    generation: u32,
    epoch: u64,
}

fn prepared_processor(
    frame_ms: u32,
    stereo: bool,
) -> (
    EchoProcessor,
    ObservationState,
    AecConfiguration,
    Arc<AudioBufferPool>,
) {
    let channels = if stereo {
        crate::aec::Channels::Stereo
    } else {
        crate::aec::Channels::Mono
    };
    let configuration = AecConfiguration::new(frame_ms, channels);
    let state = ObservationState::new(configuration.capacity_frames());
    let mut processor = EchoProcessor::new(
        configuration,
        OperatorId::new("io.pocketstation.test.aec-lifecycle.v1"),
        state.clone(),
    );
    processor.prepare().expect("real APM preparation");
    let pool = AudioBufferPool::new(
        4,
        configuration.frame_samples() * usize::from(channels.count()),
    );
    (processor, state, configuration, pool)
}

fn pcm_input(
    configuration: AecConfiguration,
    pool: &Arc<AudioBufferPool>,
    source: u64,
    stamp: InputStamp,
    final_impulse: bool,
) -> SignalEnvelope {
    let mut buffer = pool.acquire().expect("bounded input slot");
    buffer.as_mut_slice().fill(0.0);
    if final_impulse {
        let offset = (configuration.frame_samples() - 1)
            * usize::from(configuration.reference_channels.count());
        buffer.as_mut_slice()[offset] = 0.35;
    }
    let frame = AudioFrame::try_new(
        StreamId::new(source + 100),
        SourceId::new(source),
        stamp.sequence,
        stamp.timestamp_ns,
        SampleSpec::new(
            48_000,
            configuration.reference_channels.count(),
            SampleFormat::F32Interleaved,
        ),
        buffer,
    )
    .unwrap();
    let lineage = SignalLineage::try_new(
        SessionId::new(31),
        frame.stream_id(),
        frame.source_id(),
        ClockDomainId::new(4),
        stamp.sequence,
        stamp.generation,
        stamp.epoch,
        5,
    )
    .unwrap();
    let timing = SignalTiming::try_new(
        Some(stamp.timestamp_ns),
        stamp.timestamp_ns,
        Some(stamp.timestamp_ns),
        Some(u64::from(configuration.frame_duration_ms) * 1_000_000),
    )
    .unwrap();
    SignalEnvelope::from_audio(frame, None).with_lineage(lineage, timing)
}

fn initial_stamp() -> InputStamp {
    InputStamp {
        sequence: 71,
        timestamp_ns: 1_000_000_000,
        generation: 3,
        epoch: 7,
    }
}

fn audio(envelope: &SignalEnvelope) -> &AudioFrame {
    let SignalPayload::Audio(frame) = envelope.payload() else {
        panic!("AEC emitted a non-audio payload");
    };
    frame
}

#[test]
fn given_empty_processor_when_flushed_then_no_input_or_tail_is_invented() {
    let (mut processor, state, configuration, pool) = prepared_processor(20, false);
    assert!(processor.flush().unwrap().is_empty());
    assert!(processor.flush().unwrap().is_empty());
    assert!(processor
        .accept(
            "reference",
            pcm_input(configuration, &pool, 20, initial_stamp(), false)
        )
        .is_err());
    processor.stop();
    let observed = state.snapshot();
    assert_eq!(observed.processed_microphone_frames_total, 0);
    assert_eq!(observed.analyzed_reference_frames_total, 0);
    assert_eq!(observed.output_frames_total, 0);
    assert_eq!(observed.tail_frames_total, 0);
    assert_eq!(observed.tail_padding_samples_total, 0);
    assert_eq!(observed.discarded_tail_generations_total, 0);
}

#[test]
fn given_exhausted_output_pool_when_tail_flush_fails_then_loss_is_counted_and_retry_stays_failed() {
    // Exercise failure before any padding and after one native tail frame.
    for available_slots in [0, 1] {
        let (mut processor, state, configuration, pool) = prepared_processor(20, false);
        let actual_frames = super::super::config::OUTPUT_POOL_FRAMES - available_slots;
        let mut held_outputs = Vec::with_capacity(actual_frames);
        let first = initial_stamp();
        for index in 0..actual_frames {
            let stamp = InputStamp {
                sequence: first.sequence + index as u64,
                timestamp_ns: first.timestamp_ns + index as u64 * 20_000_000,
                ..first
            };
            processor
                .accept(
                    "reference",
                    pcm_input(configuration, &pool, 20, stamp, false),
                )
                .unwrap();
            held_outputs.extend(
                processor
                    .accept(
                        "microphone",
                        pcm_input(configuration, &pool, 10, stamp, true),
                    )
                    .unwrap(),
            );
        }

        let error = processor
            .flush()
            .expect_err("retained outputs exhaust the bounded pool");
        assert!(error.to_string().contains("output pool exhausted"));
        let failed = state.snapshot();
        assert_eq!(failed.state, EchoCancellationState::Failed);
        assert_eq!(failed.discarded_tail_generations_total, 1);
        assert_eq!(
            failed.processed_microphone_frames_total,
            actual_frames as u64
        );
        assert_eq!(failed.analyzed_reference_frames_total, actual_frames as u64);
        assert_eq!(failed.tail_frames_total, available_slots as u64);

        drop(held_outputs);
        assert!(
            processor.flush().is_err(),
            "freeing slots cannot replay advanced DSP history"
        );
        assert_eq!(
            state.snapshot().output_frames_total,
            failed.output_frames_total
        );
        processor.stop();
        processor.stop();
        let stopped = state.snapshot();
        assert_eq!(stopped.state, EchoCancellationState::Failed);
        assert_eq!(stopped.discarded_tail_generations_total, 1);
        assert_eq!(stopped.last_error, failed.last_error);
    }
}

#[test]
fn given_real_input_when_flushed_once_then_only_bounded_tail_uses_last_actual_input() {
    for frame_ms in [10, 20] {
        for stereo in [false, true] {
            let (mut processor, state, configuration, pool) = prepared_processor(frame_ms, stereo);
            let first = initial_stamp();
            for index in 0..2 {
                let stamp = InputStamp {
                    sequence: first.sequence + index,
                    timestamp_ns: first.timestamp_ns + index * u64::from(frame_ms) * 1_000_000,
                    ..first
                };
                assert!(processor
                    .accept(
                        "reference",
                        pcm_input(configuration, &pool, 20, stamp, false)
                    )
                    .unwrap()
                    .is_empty());
                let outputs = processor
                    .accept(
                        "microphone",
                        pcm_input(configuration, &pool, 10, stamp, true),
                    )
                    .unwrap();
                assert_eq!(outputs.len(), 1);
                let frame = audio(&outputs[0]);
                assert_eq!(frame.sequence_number(), index);
                let processing = frame.processing().unwrap();
                assert_eq!(processing.input_sequence_number, stamp.sequence);
                assert_eq!(processing.generation, 1);
                assert!(!processing.is_tail());
            }
            let last = InputStamp {
                sequence: first.sequence + 1,
                timestamp_ns: first.timestamp_ns + u64::from(frame_ms) * 1_000_000,
                ..first
            };
            let tail = processor.flush().unwrap();
            assert_eq!(tail.len(), (40 / frame_ms) as usize);
            let mut tail_energy = 0.0_f64;
            for (index, envelope) in tail.iter().enumerate() {
                let frame = audio(envelope);
                let processing = frame.processing().unwrap();
                assert_eq!(frame.sequence_number(), 2 + index as u64);
                assert_eq!(
                    frame.timestamp_ns(),
                    last.timestamp_ns + (index as u64 + 1) * u64::from(frame_ms) * 1_000_000
                );
                assert_eq!(
                    frame.samples().len(),
                    configuration.frame_samples()
                        * usize::from(configuration.reference_channels.count())
                );
                assert!(frame.samples().iter().all(|sample| sample.is_finite()));
                tail_energy += frame
                    .samples()
                    .iter()
                    .map(|sample| f64::from(*sample).powi(2))
                    .sum::<f64>();
                assert_eq!(processing.input_source_id, SourceId::new(10));
                assert_eq!(processing.input_stream_id, StreamId::new(110));
                assert_eq!(processing.input_sequence_number, last.sequence);
                assert_eq!(processing.input_timestamp_ns, last.timestamp_ns);
                assert_eq!(
                    processing.input_duration_ns,
                    u64::from(frame_ms) * 1_000_000
                );
                assert_eq!(processing.input_source_generation, last.generation);
                assert_eq!(processing.input_discontinuity_epoch, last.epoch);
                assert_eq!(processing.generation, 1);
                assert_eq!(processing.nominal_delay_samples, 432);
                assert_eq!(
                    processing.padding_samples,
                    configuration.frame_samples() as u32
                );
                assert_eq!(
                    processing.tail_offset_samples,
                    index as u32 * configuration.frame_samples() as u32
                );
                assert!(processing.is_tail());
                let derivation = envelope.derivation().unwrap();
                assert_eq!(
                    derivation.upstream_lineage().sequence_number(),
                    last.sequence
                );
                assert_eq!(
                    derivation.upstream_timing().source_timestamp_ns(),
                    Some(last.timestamp_ns)
                );
            }
            assert!(
                tail_energy > 0.001,
                "actual final impulse must emerge in the tail"
            );
            assert!(processor.flush().unwrap().is_empty());
            assert!(processor
                .accept(
                    "microphone",
                    pcm_input(configuration, &pool, 10, last, false)
                )
                .is_err());
            processor.stop();
            let observed = state.snapshot();
            assert_eq!(observed.processed_microphone_frames_total, 2);
            assert_eq!(observed.analyzed_reference_frames_total, 2);
            assert_eq!(observed.output_frames_total, 2 + u64::from(40 / frame_ms));
            assert_eq!(observed.tail_frames_total, u64::from(40 / frame_ms));
            assert_eq!(observed.tail_padding_samples_total, 1_920);
            assert_eq!(observed.discarded_tail_generations_total, 0);
            assert_eq!(observed.discarded_microphone_frames_total, 0);
            assert_eq!(observed.discarded_reference_frames_total, 0);
        }
    }
}

#[test]
fn given_processed_history_when_stopped_or_interrupted_then_no_tail_is_emitted_and_loss_is_counted()
{
    for interrupted in [false, true] {
        let (mut processor, state, configuration, pool) = prepared_processor(20, false);
        let stamp = initial_stamp();
        processor
            .accept(
                "reference",
                pcm_input(configuration, &pool, 20, stamp, false),
            )
            .unwrap();
        assert_eq!(
            processor
                .accept(
                    "microphone",
                    pcm_input(configuration, &pool, 10, stamp, true)
                )
                .unwrap()
                .len(),
            1
        );
        let queued = InputStamp {
            sequence: stamp.sequence + 1,
            timestamp_ns: stamp.timestamp_ns + 20_000_000,
            ..stamp
        };
        assert!(processor
            .accept(
                "reference",
                pcm_input(configuration, &pool, 20, queued, false)
            )
            .unwrap()
            .is_empty());
        if interrupted {
            state.interrupt();
        }
        processor.stop();
        processor.stop();
        assert!(processor.flush().unwrap().is_empty());
        let observed = state.snapshot();
        assert_eq!(
            observed.state,
            if interrupted {
                EchoCancellationState::Interrupted
            } else {
                EchoCancellationState::Stopped
            }
        );
        assert_eq!(observed.processed_microphone_frames_total, 1);
        assert_eq!(observed.analyzed_reference_frames_total, 1);
        assert_eq!(observed.output_frames_total, 1);
        assert_eq!(observed.tail_frames_total, 0);
        assert_eq!(observed.tail_padding_samples_total, 0);
        assert_eq!(observed.discarded_tail_generations_total, 1);
        assert_eq!(observed.discarded_reference_frames_total, 1);
        assert_eq!(observed.interrupted_requests_total, u64::from(interrupted));
    }
}

#[test]
fn given_old_impulse_when_cadence_or_epoch_resets_then_new_generation_contains_no_old_tail() {
    for cadence_jump in [false, true] {
        let (mut processor, state, configuration, pool) = prepared_processor(20, false);
        let first = initial_stamp();
        processor
            .accept(
                "reference",
                pcm_input(configuration, &pool, 20, first, false),
            )
            .unwrap();
        assert_eq!(
            processor
                .accept(
                    "microphone",
                    pcm_input(configuration, &pool, 10, first, true)
                )
                .unwrap()
                .len(),
            1
        );
        let next = InputStamp {
            sequence: first.sequence + 1,
            timestamp_ns: first.timestamp_ns + if cadence_jump { 25_000_000 } else { 20_000_000 },
            epoch: first.epoch + u64::from(!cadence_jump),
            ..first
        };
        // Microphone owns the reset boundary. Its new frame waits for new render.
        assert!(processor
            .accept(
                "microphone",
                pcm_input(configuration, &pool, 10, next, false)
            )
            .unwrap()
            .is_empty());
        let mut outputs = processor
            .accept(
                "reference",
                pcm_input(configuration, &pool, 20, next, false),
            )
            .unwrap();
        assert_eq!(outputs.len(), 1);
        outputs.extend(processor.flush().unwrap());
        assert_eq!(outputs.len(), 3);
        for (index, envelope) in outputs.iter().enumerate() {
            let frame = audio(envelope);
            let processing = frame.processing().unwrap();
            assert_eq!(frame.sequence_number(), index as u64 + 1);
            assert_eq!(processing.generation, 2);
            assert_eq!(processing.input_sequence_number, next.sequence);
            assert_eq!(processing.input_discontinuity_epoch, next.epoch);
            assert_eq!(processing.is_tail(), index != 0);
            assert!(
                frame.samples().iter().all(|value| value.abs() <= 1e-8),
                "reset leaked prior buffered impulse"
            );
        }
        processor.stop();
        let observed = state.snapshot();
        assert_eq!(observed.resets_total, 1);
        assert_eq!(observed.processing_generation, 2);
        assert_eq!(observed.discarded_tail_generations_total, 1);
        assert_eq!(observed.processed_microphone_frames_total, 2);
        assert_eq!(observed.analyzed_reference_frames_total, 2);
        assert_eq!(observed.output_frames_total, 4);
        assert_eq!(observed.tail_frames_total, 2);
        assert_eq!(observed.tail_padding_samples_total, 1_920);
    }
}
