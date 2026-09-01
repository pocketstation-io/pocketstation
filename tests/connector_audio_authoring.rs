use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use pocketstation::connector::{
    AudioConnector, Connector, ConnectorError, ConnectorErrorCode, ConnectorErrorStage,
    ConnectorRetryability,
};
use pocketstation::{AudioInputConfig, SampleFormat, SampleSpec, Session, SourceId};

const FRAME_SAMPLES: usize = 4;

#[derive(Default)]
struct ProviderState {
    starts: AtomicU64,
    stops: AtomicU64,
    sources: Mutex<Vec<SourceId>>,
}

struct CollectingConnector {
    state: Arc<ProviderState>,
}

impl AudioConnector for CollectingConnector {
    fn start(&mut self) -> Result<(), ConnectorError> {
        self.state.starts.fetch_add(1, Ordering::Relaxed);
        Ok(())
    }

    fn send(&mut self, frame: &pocketstation::EndpointAudioFrame) -> Result<(), ConnectorError> {
        self.state
            .sources
            .lock()
            .expect("test source collection lock")
            .push(frame.source_id());
        Ok(())
    }

    fn stop(&mut self) -> Result<(), ConnectorError> {
        self.state.stops.fetch_add(1, Ordering::Relaxed);
        Ok(())
    }
}

fn input_config() -> AudioInputConfig {
    AudioInputConfig::new(
        SampleSpec::new(48_000, 1, SampleFormat::F32Interleaved),
        4,
        FRAME_SAMPLES,
    )
    .expect("valid test input")
}

fn provider_error(stage: ConnectorErrorStage) -> ConnectorError {
    ConnectorError::new(
        ConnectorErrorCode::new("provider.test_failure").expect("test error code"),
        stage,
        ConnectorRetryability::Never,
        "provider test failure",
    )
    .expect("test Connector error")
}

fn wait_for_deliveries(state: &ProviderState, expected: usize) {
    let deadline = Instant::now() + Duration::from_secs(2);
    while state.sources.lock().expect("test source lock").len() < expected {
        assert!(Instant::now() < deadline, "Connector delivery timed out");
        std::thread::yield_now();
    }
}

#[test]
fn given_one_audio_connector_when_two_sources_send_then_one_lifecycle_receives_both() {
    let state = Arc::new(ProviderState::default());
    let connector = Connector::from_audio(CollectingConnector {
        state: Arc::clone(&state),
    })
    .expect("audio Connector");
    let session = Session::builder()
        .sample_spec(SampleSpec::new(48_000, 1, SampleFormat::F32Interleaved))
        .build();
    let mut application = session
        .audio_input(input_config())
        .expect("application input");
    let mut microphone = session
        .audio_input(input_config())
        .expect("microphone input");
    let application_source = application.source().source_id();
    let microphone_source = microphone.source().source_id();
    let destination = session
        .destination(connector)
        .expect("Connector destination");
    application
        .output()
        .send(destination)
        .expect("application route");
    microphone
        .output()
        .send(destination)
        .expect("microphone route");

    let mut running = session.start().expect("running Session");
    application
        .try_write(&[0.1, 0.2, 0.3, 0.4])
        .expect("application frame");
    microphone
        .try_write(&[0.5, 0.6, 0.7, 0.8])
        .expect("microphone frame");
    wait_for_deliveries(&state, 2);
    application.close();
    microphone.close();
    assert!(running.stop().is_success());

    assert_eq!(state.starts.load(Ordering::Relaxed), 1);
    assert_eq!(state.stops.load(Ordering::Relaxed), 1);
    let sources = state.sources.lock().expect("test source lock");
    assert!(sources.contains(&application_source));
    assert!(sources.contains(&microphone_source));
}

#[test]
fn given_two_audio_connectors_when_one_source_sends_then_lifecycles_are_independent() {
    let first = Arc::new(ProviderState::default());
    let second = Arc::new(ProviderState::default());
    let session = Session::builder()
        .sample_spec(SampleSpec::new(48_000, 1, SampleFormat::F32Interleaved))
        .build();
    let mut input = session.audio_input(input_config()).expect("audio input");
    let first_destination = session
        .destination(
            Connector::from_audio(CollectingConnector {
                state: Arc::clone(&first),
            })
            .expect("first Connector"),
        )
        .expect("first destination");
    let second_destination = session
        .destination(
            Connector::from_audio(CollectingConnector {
                state: Arc::clone(&second),
            })
            .expect("second Connector"),
        )
        .expect("second destination");
    input.output().send(first_destination).expect("first route");
    input
        .output()
        .send(second_destination)
        .expect("second route");

    let mut running = session.start().expect("running Session");
    input.try_write(&[0.1, 0.2, 0.3, 0.4]).expect("audio frame");
    wait_for_deliveries(&first, 1);
    wait_for_deliveries(&second, 1);
    input.close();
    assert!(running.stop().is_success());

    for state in [&first, &second] {
        assert_eq!(state.starts.load(Ordering::Relaxed), 1);
        assert_eq!(state.stops.load(Ordering::Relaxed), 1);
        assert_eq!(state.sources.lock().expect("test source lock").len(), 1);
    }
}

struct StartFailureConnector {
    stops: Arc<AtomicU64>,
}

impl AudioConnector for StartFailureConnector {
    fn start(&mut self) -> Result<(), ConnectorError> {
        Err(provider_error(ConnectorErrorStage::Startup))
    }

    fn send(&mut self, _frame: &pocketstation::EndpointAudioFrame) -> Result<(), ConnectorError> {
        Ok(())
    }

    fn stop(&mut self) -> Result<(), ConnectorError> {
        self.stops.fetch_add(1, Ordering::Relaxed);
        Ok(())
    }
}

#[test]
fn given_provider_start_failure_when_session_starts_then_provider_is_stopped_once() {
    let stops = Arc::new(AtomicU64::new(0));
    let session = Session::builder()
        .sample_spec(SampleSpec::new(48_000, 1, SampleFormat::F32Interleaved))
        .build();
    let input = session.audio_input(input_config()).expect("audio input");
    let destination = session
        .destination(
            Connector::from_audio(StartFailureConnector {
                stops: Arc::clone(&stops),
            })
            .expect("failing Connector"),
        )
        .expect("Connector destination");
    input.output().send(destination).expect("audio route");

    let mut running = session.start().expect("running Session");
    let outcome = running.stop();

    assert!(!outcome.is_success());
    assert_eq!(stops.load(Ordering::Relaxed), 1);
}

#[test]
fn given_audio_function_when_a_frame_arrives_then_core_delivers_it() {
    let delivered = Arc::new(AtomicU64::new(0));
    let observed = Arc::clone(&delivered);
    let connector = Connector::from_audio_fn(move |frame| {
        assert_eq!(frame.samples(), &[0.1, 0.2, 0.3, 0.4]);
        observed.fetch_add(1, Ordering::Release);
        Ok(())
    })
    .expect("function Connector");
    let session = Session::builder()
        .sample_spec(SampleSpec::new(48_000, 1, SampleFormat::F32Interleaved))
        .build();
    let mut input = session.audio_input(input_config()).expect("audio input");
    let destination = session
        .destination(connector)
        .expect("Connector destination");
    input.output().send(destination).expect("audio route");

    let mut running = session.start().expect("running Session");
    input.try_write(&[0.1, 0.2, 0.3, 0.4]).expect("audio frame");
    let deadline = Instant::now() + Duration::from_secs(2);
    while delivered.load(Ordering::Acquire) == 0 {
        assert!(
            Instant::now() < deadline,
            "function did not receive a frame"
        );
        std::thread::yield_now();
    }
    input.close();
    assert!(running.stop().is_success());
    assert_eq!(delivered.load(Ordering::Acquire), 1);
}
