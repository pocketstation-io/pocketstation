//! Built-in adapter from an exact native process tap to registered Source audio.
//! Capture framing, native timing, pools and finalization remain in macos_tap.

use super::source::{
    SourceCancellation, SourceConfiguration, SourceDriver, SourceDriverError, SourceEmission,
    SourceFactory, SourceManifest, SourcePrepareContext, SourceTypeId,
};
use crate::graph::{
    AudioCaps, ChannelLayout, ConfigError, ExecutionPartition, ExecutionSafety, MediaCaps,
    Multiplicity, PortDirection, PortSpec, SignalSpec,
};
#[cfg(all(target_os = "macos", feature = "coreaudio-capture"))]
use crate::graph::{SignalEnvelope, SignalLineage, SignalTiming};
use crate::{
    ApplicationProcessSet, AudioFrameDuration, CaptureOpenCancellation, ProcessSetError,
    SampleSpec, SourceOutputHandle,
};
use std::collections::BTreeMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

#[derive(Default)]
struct CaptureCleanupReceipt {
    unconfirmed: AtomicBool,
    cause: Mutex<Option<Box<dyn std::error::Error + Send + Sync>>>,
}
impl CaptureCleanupReceipt {
    fn record(&self, cause: Option<Box<dyn std::error::Error + Send + Sync>>) {
        self.unconfirmed.store(true, Ordering::Release);
        if let Ok(mut slot) = self.cause.lock() {
            if slot.is_none() {
                *slot = cause;
            }
        }
    }
}

#[cfg(all(target_os = "macos", feature = "coreaudio-capture"))]
use super::source::SourceSessionContext;

const SOURCE_TYPE: &str = "io.pocketstation.source.application-process-set.v1";
const OUTPUT: &str = "audio";

pub(crate) struct ProcessSetFactory {
    manifest: SourceManifest,
    #[cfg(all(target_os = "macos", feature = "coreaudio-capture"))]
    duration: AudioFrameDuration,
    #[cfg(all(target_os = "macos", feature = "coreaudio-capture"))]
    sample_spec: SampleSpec,
    pending: Arc<Mutex<BTreeMap<u64, ApplicationProcessSet>>>,
    cleanup: Arc<CaptureCleanupReceipt>,
}
impl ProcessSetFactory {
    pub(crate) fn new(
        sample_spec: SampleSpec,
        duration: AudioFrameDuration,
    ) -> Result<Arc<Self>, ProcessSetError> {
        let manifest = SourceManifest::new(
            SourceTypeId::new(SOURCE_TYPE)
                .map_err(|error| ProcessSetError::Registration(error.to_string()))?,
            1,
            1,
            vec![PortSpec {
                name: OUTPUT.to_owned(),
                direction: PortDirection::Output,
                signal: SignalSpec::audio(),
                media: MediaCaps::Audio(AudioCaps {
                    sample_rate_hz: Some(sample_spec.sample_rate_hz),
                    frame_samples: Some(duration.samples_per_channel(sample_spec.sample_rate_hz)),
                    channel_layout: ChannelLayout::Stereo,
                    format: sample_spec.format,
                }),
                multiplicity: Multiplicity::Many,
                required: true,
            }],
            ExecutionPartition::BlockingWorker,
            ExecutionSafety::AllocationAllowed,
        )
        .map_err(|error| ProcessSetError::Registration(error.to_string()))?;
        Ok(Arc::new(Self {
            manifest,
            #[cfg(all(target_os = "macos", feature = "coreaudio-capture"))]
            duration,
            #[cfg(all(target_os = "macos", feature = "coreaudio-capture"))]
            sample_spec,
            pending: Arc::new(Mutex::new(BTreeMap::new())),
            cleanup: Arc::new(CaptureCleanupReceipt::default()),
        }))
    }
    pub(crate) fn cleanup_unconfirmed(&self) -> bool {
        self.cleanup.unconfirmed.load(Ordering::Acquire)
    }
    pub(crate) fn cleanup_diagnostic(&self) -> String {
        self.cleanup
            .cause
            .lock()
            .ok()
            .and_then(|cause| cause.as_ref().map(ToString::to_string))
            .unwrap_or_else(|| "native capture cleanup was not confirmed".to_owned())
    }
    pub(crate) fn declare(
        &self,
        declaration: &crate::session::Session,
        set: ApplicationProcessSet,
    ) -> Result<SourceOutputHandle, ProcessSetError> {
        let source = declaration.source(
            self.manifest.source_type_id().clone(),
            SourceConfiguration::default(),
        )?;
        self.pending
            .lock()
            .map_err(|_| ProcessSetError::Registration("reservation state unavailable".into()))?
            .insert(source.source_id().get(), set);
        match source.output(OUTPUT) {
            Ok(output) => Ok(output),
            Err(error) => {
                if let Ok(mut pending) = self.pending.lock() {
                    pending.remove(&source.source_id().get());
                }
                Err(error.into())
            }
        }
    }
}
impl SourceFactory for ProcessSetFactory {
    fn manifest(&self) -> &SourceManifest {
        &self.manifest
    }
    fn validate_config(&self, configuration: &SourceConfiguration) -> Result<(), ConfigError> {
        if configuration.iter().next().is_none() {
            Ok(())
        } else {
            Err(ConfigError::Invalid {
                key: "<configuration>".into(),
                reason: "process sets use immutable typed observations".into(),
            })
        }
    }
    fn create(
        &self,
        configuration: &SourceConfiguration,
    ) -> Result<Box<dyn SourceDriver>, SourceDriverError> {
        self.validate_config(configuration)
            .map_err(|error| SourceDriverError::Failed(error.to_string()))?;
        Ok(Box::new(ProcessSetDriver {
            pending: Arc::clone(&self.pending),
            #[cfg(all(target_os = "macos", feature = "coreaudio-capture"))]
            duration: self.duration,
            #[cfg(all(target_os = "macos", feature = "coreaudio-capture"))]
            sample_spec: self.sample_spec,
            #[cfg(all(target_os = "macos", feature = "coreaudio-capture"))]
            context: None,
            failure: None,
            cleanup: Arc::clone(&self.cleanup),
            #[cfg(all(target_os = "macos", feature = "coreaudio-capture"))]
            last_sequence: None,
            #[cfg(all(target_os = "macos", feature = "coreaudio-capture"))]
            discontinuity_epoch: 0,
            #[cfg(all(target_os = "macos", feature = "coreaudio-capture"))]
            last_loss_counts: [0; 5],
            #[cfg(all(target_os = "macos", feature = "coreaudio-capture"))]
            capture: None,
            #[cfg(all(target_os = "macos", feature = "coreaudio-capture"))]
            capture_observations: None,
            #[cfg(all(target_os = "macos", feature = "coreaudio-capture"))]
            frames: None,
            #[cfg(all(target_os = "macos", feature = "coreaudio-capture"))]
            events: None,
        }))
    }
}

struct ProcessSetDriver {
    pending: Arc<Mutex<BTreeMap<u64, ApplicationProcessSet>>>,
    #[cfg(all(target_os = "macos", feature = "coreaudio-capture"))]
    duration: AudioFrameDuration,
    #[cfg(all(target_os = "macos", feature = "coreaudio-capture"))]
    sample_spec: SampleSpec,
    #[cfg(all(target_os = "macos", feature = "coreaudio-capture"))]
    context: Option<SourceSessionContext>,
    failure: Option<Box<dyn std::error::Error + Send + Sync>>,
    cleanup: Arc<CaptureCleanupReceipt>,
    #[cfg(all(target_os = "macos", feature = "coreaudio-capture"))]
    last_sequence: Option<u64>,
    #[cfg(all(target_os = "macos", feature = "coreaudio-capture"))]
    discontinuity_epoch: u64,
    #[cfg(all(target_os = "macos", feature = "coreaudio-capture"))]
    last_loss_counts: [u64; 5],
    #[cfg(all(target_os = "macos", feature = "coreaudio-capture"))]
    capture: Option<crate::capture::platform::macos::macos_tap::TapLoopbackSource>,
    #[cfg(all(target_os = "macos", feature = "coreaudio-capture"))]
    capture_observations: Option<crate::CaptureObservationHandle>,
    #[cfg(all(target_os = "macos", feature = "coreaudio-capture"))]
    frames: Option<crate::capture::CapturedFrameStream>,
    #[cfg(all(target_os = "macos", feature = "coreaudio-capture"))]
    events: Option<crate::capture::SourceRuntimeEventReceiver>,
}
impl ProcessSetDriver {
    fn fail(&mut self, cause: impl std::error::Error + Send + Sync + 'static) -> SourceDriverError {
        let error = SourceDriverError::Failed(cause.to_string());
        self.failure = Some(Box::new(cause));
        error
    }
    #[cfg(all(target_os = "macos", feature = "coreaudio-capture"))]
    fn emission(
        &mut self,
        mut frame: crate::AudioFrame,
    ) -> Result<Option<SourceEmission>, SourceDriverError> {
        let session = self
            .context
            .as_ref()
            .ok_or_else(|| SourceDriverError::Failed("process capture is not prepared".into()))?;
        let output = session
            .output(OUTPUT)
            .ok_or_else(|| SourceDriverError::Failed("process capture output missing".into()))?;
        if frame.sample_rate_hz != self.sample_spec.sample_rate_hz
            || frame.channels != 2
            || frame.format != self.sample_spec.format
        {
            return Err(self.fail(crate::CaptureError::BackendInit(
                "native process capture format differs from the declared Session format".into(),
            )));
        }
        frame.source_id = session.source_id;
        frame.stream_id = output.stream_id;
        let timestamp_ns = frame.timestamp_ns;
        let sequence_number = frame.sequence_number;
        let native = self
            .capture_observations
            .as_ref()
            .map(crate::CaptureObservationHandle::observations)
            .unwrap_or_default();
        let stream_drops = self
            .frames
            .as_ref()
            .map(|frames| {
                frames
                    .observation_handle()
                    .observations()
                    .dropped_newest_frames
            })
            .unwrap_or(0);
        let losses = [
            native.dispatch_queue_full_total,
            native.pool_exhausted_total,
            native.invalid_buffer_total,
            native.oversized_buffer_total,
            stream_drops,
        ];
        if self.last_sequence.map_or(sequence_number != 0, |last| {
            last.checked_add(1) != Some(sequence_number)
        }) || losses != self.last_loss_counts
        {
            self.discontinuity_epoch = self.discontinuity_epoch.saturating_add(1);
        }
        self.last_sequence = Some(sequence_number);
        self.last_loss_counts = losses;
        let duration_ns = (frame.buffer.len() as u64 / u64::from(frame.channels)) * 1_000_000_000
            / u64::from(frame.sample_rate_hz);
        let lineage = SignalLineage {
            session_id: session.session_id,
            source_id: session.source_id,
            stream_id: output.stream_id,
            clock_id: crate::timing::PROCESS_MONOTONIC_CLOCK_DOMAIN_ID,
            sequence_number,
            source_generation: 1,
            discontinuity_epoch: self.discontinuity_epoch,
            policy_epoch: 0,
        };
        Ok(Some(SourceEmission {
            output_port: OUTPUT.into(),
            terminal: false,
            envelope: SignalEnvelope::from_audio(frame, None).with_lineage(
                lineage,
                SignalTiming {
                    source_timestamp_ns: Some(timestamp_ns),
                    observed_timestamp_ns: crate::timing::monotonic_timestamp_ns(),
                    session_timestamp_ns: Some(timestamp_ns),
                    duration_ns: Some(duration_ns),
                },
            ),
        }))
    }
}
impl SourceDriver for ProcessSetDriver {
    fn prepare(&mut self, context: &SourcePrepareContext) -> Result<(), SourceDriverError> {
        self.prepare_cancellable(context, &CaptureOpenCancellation::default())
    }
    fn prepare_cancellable(
        &mut self,
        context: &SourcePrepareContext,
        cancellation: &CaptureOpenCancellation,
    ) -> Result<(), SourceDriverError> {
        let session = context.session.clone().ok_or_else(|| {
            SourceDriverError::Failed("process capture requires Session identity".into())
        })?;
        let selection = self
            .pending
            .lock()
            .map_err(|_| {
                SourceDriverError::Failed("process capture reservation unavailable".into())
            })?
            .remove(&session.source_id.get())
            .ok_or_else(|| {
                SourceDriverError::Failed("process capture reservation absent or consumed".into())
            })?;
        #[cfg(all(target_os = "macos", feature = "coreaudio-capture"))]
        {
            let (sender, frames) = crate::capture::captured_frame_stream_with_start_gate(
                32,
                crate::capture::CaptureDeliveryStartGate::opened(),
            )
            .map_err(|error| self.fail(error))?;
            let (events, receiver) = crate::capture::source_runtime_event_channel(8)
                .map_err(|error| self.fail(error))?;
            let mut capture = crate::capture::platform::macos::macos_tap::TapLoopbackSource::capture_process_set_cancellable(
                selection, self.duration, sender.into_callback(), Some(events), cancellation)
                .map_err(|error| self.fail(error))?;
            let native_spec = capture.native_spec();
            if native_spec.sample_rate_hz != self.sample_spec.sample_rate_hz
                || native_spec.channels != 2
                || native_spec.format != self.sample_spec.format
            {
                let cleanup_error = capture.stop_and_join().err();
                return Err(self.fail(crate::CaptureOpenFailure::Backend {
                    source: crate::CaptureError::BackendInit("native process capture format differs from the declared Session format; no resampling is applied".into()),
                    cleanup_error,
                }));
            }
            self.frames = Some(frames);
            self.events = Some(receiver);
            self.capture_observations = Some(capture.observation_handle());
            self.capture = Some(capture);
            self.context = Some(session);
            Ok(())
        }
        #[cfg(not(all(target_os = "macos", feature = "coreaudio-capture")))]
        {
            let _ = (selection, session, cancellation);
            Err(self.fail(crate::CaptureError::NotSupported))
        }
    }
    fn take_failure_cause(&mut self) -> Option<Box<dyn std::error::Error + Send + Sync>> {
        self.failure.take()
    }
    fn next(
        &mut self,
        cancellation: &SourceCancellation,
    ) -> Result<Option<SourceEmission>, SourceDriverError> {
        #[cfg(all(target_os = "macos", feature = "coreaudio-capture"))]
        loop {
            if cancellation.is_cancelled() {
                return Ok(None);
            }
            if let Some(frame) = self
                .frames
                .as_mut()
                .and_then(crate::capture::CapturedFrameStream::try_next)
            {
                return self.emission(frame);
            }
            if let Some(receiver) = &self.events {
                match receiver.try_recv() {
                    crate::capture::SourceRuntimeEventReceive::Event(
                        crate::SourceRuntimeEvent::SourceUnavailable { stable_id, .. },
                    ) => {
                        return Err(self.fail(crate::CaptureError::SourceUnavailable {
                            stable_key: stable_id.stable_key,
                        }))
                    }
                    crate::capture::SourceRuntimeEventReceive::Event(
                        crate::SourceRuntimeEvent::BackendFailure { .. },
                    ) => {
                        return Err(self.fail(crate::CaptureError::BackendInit(
                            "native exact process-set source failed".into(),
                        )))
                    }
                    _ => {}
                }
            }
            std::thread::park_timeout(std::time::Duration::from_millis(1));
        }
        #[cfg(not(all(target_os = "macos", feature = "coreaudio-capture")))]
        {
            let _ = cancellation;
            Err(self.fail(crate::CaptureError::NotSupported))
        }
    }
    fn close(&mut self) -> Result<(), SourceDriverError> {
        #[cfg(all(target_os = "macos", feature = "coreaudio-capture"))]
        if let Some(capture) = self.capture.as_mut() {
            if let Err(error) = capture.stop_and_join() {
                self.cleanup.unconfirmed.store(true, Ordering::Release);
                return Err(self.fail(error));
            }
            self.capture = None;
        }
        Ok(())
    }
}

/// Drop invariants: control/BlockingWorker ownership only, never a capture
/// callback or realtime partition. Cleanup may join and lock; it must not panic
/// or log, and an unconfirmed native close must remain in the cleanup receipt.
impl Drop for ProcessSetDriver {
    fn drop(&mut self) {
        if self.close().is_err() {
            self.cleanup.record(self.failure.take());
        }
    }
}

#[cfg(all(test, target_os = "macos", feature = "coreaudio-capture"))]
mod tests;
