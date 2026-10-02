use std::sync::Arc;

use crate::graph::{
    AsyncNode, AsyncNodeFuture, AsyncOperatorFactory, AsyncOperatorManifest,
    AsyncOperatorPrepareContext, ConfigError, NodeConfig, NodeError, SignalEnvelope,
};

use super::telemetry::SourceAecAdmission;

/// Session-owned checks around the existing optional processor. The wrapped
/// factory keeps its ports, configuration and cancellation implementation.
pub(super) struct AecAdmissionFactory {
    inner: Arc<dyn AsyncOperatorFactory>,
    inputs: Vec<SourceAecAdmission>,
}

impl AecAdmissionFactory {
    pub(super) fn new(
        inner: Arc<dyn AsyncOperatorFactory>,
        inputs: Vec<SourceAecAdmission>,
    ) -> Self {
        Self { inner, inputs }
    }
}

impl AsyncOperatorFactory for AecAdmissionFactory {
    fn manifest(&self) -> &AsyncOperatorManifest {
        self.inner.manifest()
    }

    fn validate_config(&self, configuration: &NodeConfig) -> Result<(), ConfigError> {
        self.inner.validate_config(configuration)
    }

    fn resolve_manifest(
        &self,
        configuration: &NodeConfig,
    ) -> Result<AsyncOperatorManifest, ConfigError> {
        self.inner.resolve_manifest(configuration)
    }

    fn create(&self, configuration: &NodeConfig) -> Result<Box<dyn AsyncNode>, NodeError> {
        Ok(Box::new(AecAdmissionNode {
            inner: self.inner.create(configuration)?,
            inputs: self.inputs.clone(),
            failure: None,
        }))
    }
}

struct AecAdmissionNode {
    inner: Box<dyn AsyncNode>,
    inputs: Vec<SourceAecAdmission>,
    failure: Option<String>,
}

impl AecAdmissionNode {
    fn check(&mut self) -> Result<(), String> {
        if let Some(failure) = &self.failure {
            return Err(failure.clone());
        }
        for input in &self.inputs {
            if let Err(error) = input.check() {
                let failure = format!("AEC input admission revoked: {error}");
                self.failure = Some(failure.clone());
                return Err(failure);
            }
        }
        Ok(())
    }
}

impl AsyncNode for AecAdmissionNode {
    fn prepare<'a>(
        &'a mut self,
        context: &'a AsyncOperatorPrepareContext,
    ) -> AsyncNodeFuture<'a, Result<(), NodeError>> {
        Box::pin(async move {
            self.check().map_err(NodeError::Prepare)?;
            let result = self.inner.prepare(context).await;
            self.check().map_err(NodeError::Prepare)?;
            result
        })
    }

    fn process<'a>(
        &'a mut self,
        input: SignalEnvelope,
    ) -> AsyncNodeFuture<'a, Result<Vec<SignalEnvelope>, NodeError>> {
        Box::pin(async move {
            self.check().map_err(NodeError::Process)?;
            let result = self.inner.process(input).await;
            self.check().map_err(NodeError::Process)?;
            result
        })
    }

    fn process_port<'a>(
        &'a mut self,
        port: &'a str,
        input: SignalEnvelope,
    ) -> AsyncNodeFuture<'a, Result<Vec<SignalEnvelope>, NodeError>> {
        Box::pin(async move {
            self.check().map_err(NodeError::Process)?;
            let result = self.inner.process_port(port, input).await;
            self.check().map_err(NodeError::Process)?;
            result
        })
    }

    fn flush<'a>(&'a mut self) -> AsyncNodeFuture<'a, Result<Vec<SignalEnvelope>, NodeError>> {
        Box::pin(async move {
            self.check().map_err(NodeError::Process)?;
            let result = self.inner.flush().await;
            self.check().map_err(NodeError::Process)?;
            result
        })
    }

    fn cancel<'a>(&'a mut self) -> AsyncNodeFuture<'a, Result<(), NodeError>> {
        self.inner.cancel()
    }

    fn close<'a>(&'a mut self) -> AsyncNodeFuture<'a, Result<(), NodeError>> {
        self.inner.close()
    }
}

#[cfg(test)]
mod tests {
    use super::super::telemetry::SourceCaptureObservationHandle;
    use super::*;
    use crate::capture::{
        capture_processing_observations, prepare_capture, ActiveCaptureBackend,
        CallbackCaptureBackend, CaptureDelivery, CaptureError, CaptureLineageSeed, CaptureMode,
        CaptureObservationHandle, CaptureObservations, CapturePrepareRequest,
        CaptureProcessingObservationHandle, CaptureProcessingObservations,
        CaptureProcessingReporter, PreparedCaptureBackend,
    };
    use crate::frame::{SessionId, SourceId, StemId};
    use crate::graph::{SignalPayload, SignalSpec, TextFormat};
    use std::sync::atomic::{AtomicUsize, Ordering};

    // These opened backends report simulated facts; no physical processing runs.
    struct ReportedCapture(CaptureProcessingObservationHandle);

    impl CallbackCaptureBackend for ReportedCapture {
        fn prepare(&self, _: CaptureMode) -> Result<Box<dyn PreparedCaptureBackend>, CaptureError> {
            Ok(Box::new(Self(self.0.clone())))
        }
    }

    impl PreparedCaptureBackend for ReportedCapture {
        fn open(
            self: Box<Self>,
            _: CaptureDelivery,
        ) -> Result<Box<dyn ActiveCaptureBackend>, CaptureError> {
            Ok(self)
        }
    }

    impl ActiveCaptureBackend for ReportedCapture {
        fn source_id(&self) -> SourceId {
            SourceId(20)
        }
        fn processing_observation_handle(&self) -> Option<CaptureProcessingObservationHandle> {
            Some(self.0.clone())
        }
        fn observation_handle(&self) -> CaptureObservationHandle {
            CaptureObservationHandle::default()
        }
        fn observations(&self) -> CaptureObservations {
            CaptureObservations::default()
        }
        fn stop_and_join(self: Box<Self>) -> Result<CaptureObservations, CaptureError> {
            Ok(CaptureObservations::default())
        }
    }

    fn admitted_input() -> (CaptureProcessingReporter, SourceAecAdmission) {
        let (reporter, observation) = capture_processing_observations();
        reporter.replace(CaptureProcessingObservations {
            echo_processed: Some(false),
            ..Default::default()
        });
        let capture = prepare_capture(
            &ReportedCapture(observation),
            CapturePrepareRequest {
                mode: CaptureMode::SystemMix,
                lineage_seed: CaptureLineageSeed::new(SessionId(1), StemId(2)),
                frame_capacity_frames: 2,
                runtime_event_capacity_events: 2,
            },
        )
        .unwrap()
        .open()
        .unwrap();
        let source = SourceCaptureObservationHandle::new(
            capture.observation_receipt(),
            capture.open_metadata(),
        );
        let admission = source.aec_admission().unwrap();
        capture.stop_and_join().unwrap();
        (reporter, admission)
    }

    fn processed() -> CaptureProcessingObservations {
        CaptureProcessingObservations {
            echo_processed: Some(true),
            ..Default::default()
        }
    }

    fn input() -> SignalEnvelope {
        SignalEnvelope::untracked(
            SignalPayload::Text("fixture".into()),
            SignalSpec::text(TextFormat::Utf8),
            1,
        )
    }

    #[derive(Default)]
    struct Calls {
        prepared_total: AtomicUsize,
        processed_total: AtomicUsize,
        flushed_total: AtomicUsize,
        cancelled_total: AtomicUsize,
        closed_total: AtomicUsize,
    }

    struct RecordingNode {
        calls: Arc<Calls>,
        during_processing: Option<CaptureProcessingReporter>,
    }

    impl AsyncNode for RecordingNode {
        fn prepare<'a>(
            &'a mut self,
            _: &'a AsyncOperatorPrepareContext,
        ) -> AsyncNodeFuture<'a, Result<(), NodeError>> {
            Box::pin(async move {
                self.calls.prepared_total.fetch_add(1, Ordering::SeqCst);
                Ok(())
            })
        }
        fn process<'a>(
            &'a mut self,
            input: SignalEnvelope,
        ) -> AsyncNodeFuture<'a, Result<Vec<SignalEnvelope>, NodeError>> {
            Box::pin(async move {
                self.calls.processed_total.fetch_add(1, Ordering::SeqCst);
                if let Some(reporter) = &self.during_processing {
                    reporter.replace(processed());
                }
                Ok(vec![input])
            })
        }
        fn flush<'a>(&'a mut self) -> AsyncNodeFuture<'a, Result<Vec<SignalEnvelope>, NodeError>> {
            Box::pin(async move {
                self.calls.flushed_total.fetch_add(1, Ordering::SeqCst);
                Ok(vec![input()])
            })
        }
        fn cancel<'a>(&'a mut self) -> AsyncNodeFuture<'a, Result<(), NodeError>> {
            Box::pin(async move {
                self.calls.cancelled_total.fetch_add(1, Ordering::SeqCst);
                Ok(())
            })
        }
        fn close<'a>(&'a mut self) -> AsyncNodeFuture<'a, Result<(), NodeError>> {
            Box::pin(async move {
                self.calls.closed_total.fetch_add(1, Ordering::SeqCst);
                Ok(())
            })
        }
    }

    fn node(
        admission: SourceAecAdmission,
        reporter: Option<CaptureProcessingReporter>,
    ) -> (AecAdmissionNode, Arc<Calls>) {
        let calls = Arc::new(Calls::default());
        (
            AecAdmissionNode {
                inner: Box::new(RecordingNode {
                    calls: Arc::clone(&calls),
                    during_processing: reporter,
                }),
                inputs: vec![admission],
                failure: None,
            },
            calls,
        )
    }

    #[tokio::test]
    async fn given_revoked_input_when_processing_then_inner_is_not_called_and_cleanup_runs() {
        let (reporter, admission) = admitted_input();
        let (mut node, calls) = node(admission, None);
        reporter.replace(processed());
        assert!(node.process(input()).await.is_err());
        reporter.replace(CaptureProcessingObservations {
            echo_processed: Some(false),
            ..Default::default()
        });
        assert!(node.process_port("microphone", input()).await.is_err());
        assert!(node.flush().await.is_err());
        assert_eq!(calls.processed_total.load(Ordering::SeqCst), 0);
        assert_eq!(calls.flushed_total.load(Ordering::SeqCst), 0);
        node.cancel().await.unwrap();
        node.close().await.unwrap();
        assert_eq!(calls.cancelled_total.load(Ordering::SeqCst), 1);
        assert_eq!(calls.closed_total.load(Ordering::SeqCst), 1);
    }

    #[tokio::test]
    async fn given_revocation_during_processing_when_result_returns_then_outputs_are_discarded() {
        let (reporter, admission) = admitted_input();
        let (mut node, calls) = node(admission, Some(reporter));
        assert!(node.process_port("microphone", input()).await.is_err());
        assert_eq!(calls.processed_total.load(Ordering::SeqCst), 1);
        assert!(node.flush().await.is_err());
        assert_eq!(calls.flushed_total.load(Ordering::SeqCst), 0);
        node.close().await.unwrap();
    }

    #[tokio::test]
    async fn given_admitted_input_when_processing_then_inner_outputs_and_flush_are_preserved() {
        let (_, admission) = admitted_input();
        let (mut node, calls) = node(admission, None);
        assert_eq!(node.process(input()).await.unwrap().len(), 1);
        assert_eq!(
            node.process_port("microphone", input())
                .await
                .unwrap()
                .len(),
            1
        );
        assert_eq!(node.flush().await.unwrap().len(), 1);
        assert_eq!(calls.processed_total.load(Ordering::SeqCst), 2);
        assert_eq!(calls.flushed_total.load(Ordering::SeqCst), 1);
        node.close().await.unwrap();
    }

    #[tokio::test]
    async fn given_revoked_input_when_preparing_then_inner_preparation_is_not_called() {
        use crate::graph::{
            ExecutionPartition, MediaCaps, PortDirection, PortPrepareContext, RouteSettings,
        };
        let (reporter, admission) = admitted_input();
        let (mut node, calls) = node(admission, None);
        reporter.replace(processed());
        let edges = [PortDirection::Input, PortDirection::Output]
            .into_iter()
            .map(|direction| {
                PortPrepareContext::new(
                    None,
                    "fixture",
                    direction,
                    SignalSpec::text(TextFormat::Utf8),
                    MediaCaps::Text,
                    RouteSettings::bounded_async().with_media(MediaCaps::Text),
                    1,
                )
                .unwrap()
            })
            .collect();
        let context =
            AsyncOperatorPrepareContext::new(ExecutionPartition::AsyncWorker, edges).unwrap();
        assert!(matches!(
            node.prepare(&context).await,
            Err(NodeError::Prepare(_))
        ));
        assert_eq!(calls.prepared_total.load(Ordering::SeqCst), 0);
        node.close().await.unwrap();
    }
}
