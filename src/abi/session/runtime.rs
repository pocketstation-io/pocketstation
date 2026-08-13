use std::sync::atomic::{AtomicU32, AtomicU64, Ordering};
use std::sync::{Arc, Mutex, OnceLock};

use crate::session::{
    ApplicationSelector, CompiledSession, EndpointConfiguration, EndpointDescriptor,
    NativeSessionEngineHostOptions, Operator, OperatorConfiguration, OperatorId,
    PolledAudioBatchLease, PolledAudioPollError, RunningSession, Session, SessionEngineHost,
    SessionEngineHostBuilder, SessionEvent, SessionEventKind, SessionEventReceive,
    SessionEventReceiver, SessionLifecycleState, SessionMetricsSnapshot, SessionStartCancellation,
    SessionStartError, Source, SourceConfiguration, SourceTypeId,
};

use crate::abi::executable_extension::{
    ExecutableExtensionPipeline, ExecutableExtensionRegistration,
};

use crate::abi::session::abi::{
    PksSessionAudioFrame, PksSessionEvent, PksSessionEventKind, PksSessionHandle,
    PksSessionHandleKind, PksSessionLifecycleState, PksSessionSampleFormat, PKS_SESSION_ABI_MAJOR,
    PKS_SESSION_ABI_MINOR,
};
use crate::abi::session::error::AbiError;
use crate::abi::session::handle::HandleTable;

const DEFAULT_ENGINE_CAPACITY_COUNT: usize = 16;
const DEFAULT_SESSION_CAPACITY_COUNT: usize = 1;
const ENGINE_HANDLE_SCOPE_ID: u64 = 1;

static NEXT_ENGINE_SCOPE_ID: AtomicU64 = AtomicU64::new(2);

enum SessionObject {
    Draft(Session),
    Compiled(CompiledSession),
    Running {
        session: RunningSession,
        events: Option<SessionEventReceiver>,
        stopped_successfully: Option<bool>,
    },
    Failed {
        events: Option<SessionEventReceiver>,
    },
    Transitioning,
}

impl SessionObject {
    fn lifecycle_state(&self) -> PksSessionLifecycleState {
        match self {
            Self::Draft(_) => PksSessionLifecycleState::Draft,
            Self::Compiled(_) => PksSessionLifecycleState::Compiled,
            Self::Running {
                stopped_successfully: None,
                ..
            } => PksSessionLifecycleState::Running,
            Self::Running {
                stopped_successfully: Some(true),
                ..
            } => PksSessionLifecycleState::Stopped,
            Self::Running {
                stopped_successfully: Some(false),
                ..
            }
            | Self::Failed { .. }
            | Self::Transitioning => PksSessionLifecycleState::Failed,
        }
    }

    fn request_stop(&mut self) -> Result<(), AbiError> {
        match self {
            Self::Running {
                session,
                stopped_successfully,
                ..
            } => {
                let outcome = session.stop();
                *stopped_successfully = Some(outcome.is_success());
                if outcome.is_success() {
                    Ok(())
                } else {
                    Err(AbiError::BackendFailure)
                }
            }
            Self::Failed { .. } => Err(AbiError::BackendFailure),
            Self::Draft(_) | Self::Compiled(_) | Self::Transitioning => {
                Err(AbiError::InvalidLifecycleState)
            }
        }
    }

    fn events(&self) -> Option<&SessionEventReceiver> {
        match self {
            Self::Running { events, .. } | Self::Failed { events } => events.as_ref(),
            Self::Draft(_) | Self::Compiled(_) | Self::Transitioning => None,
        }
    }
}

pub struct SessionRuntime {
    host: Option<SessionEngineHost>,
    options: NativeSessionEngineHostOptions,
    executable_extensions: Vec<ExecutableExtensionRegistration>,
    sessions: HandleTable<SessionObject>,
    leases: HandleTable<PolledAudioBatchLease>,
    session_created: bool,
    session_scope_id: u64,
}

impl SessionRuntime {
    fn new(options: NativeSessionEngineHostOptions) -> Result<Self, AbiError> {
        let lease_capacity_count = options.polled_audio_endpoint.max_outstanding_leases;
        Ok(Self::with_options(options, lease_capacity_count))
    }

    fn with_options(options: NativeSessionEngineHostOptions, lease_capacity_count: usize) -> Self {
        let scope_id = NEXT_ENGINE_SCOPE_ID.fetch_add(1, Ordering::Relaxed).max(2);
        Self {
            host: None,
            options,
            executable_extensions: Vec::new(),
            sessions: HandleTable::new(
                DEFAULT_SESSION_CAPACITY_COUNT,
                PksSessionHandleKind::Session,
                scope_id,
            ),
            leases: HandleTable::new(
                lease_capacity_count,
                PksSessionHandleKind::AudioBatch,
                scope_id,
            ),
            session_created: false,
            session_scope_id: scope_id,
        }
    }

    #[cfg(any(test, feature = "conformance-fixtures"))]
    fn with_host(host: SessionEngineHost, lease_capacity_count: usize) -> Self {
        let scope_id = NEXT_ENGINE_SCOPE_ID.fetch_add(1, Ordering::Relaxed).max(2);
        Self {
            host: Some(host),
            options: NativeSessionEngineHostOptions::default(),
            executable_extensions: Vec::new(),
            sessions: HandleTable::new(
                DEFAULT_SESSION_CAPACITY_COUNT,
                PksSessionHandleKind::Session,
                scope_id,
            ),
            leases: HandleTable::new(
                lease_capacity_count,
                PksSessionHandleKind::AudioBatch,
                scope_id,
            ),
            session_created: false,
            session_scope_id: scope_id,
        }
    }

    pub fn create_app_mic_session(
        &mut self,
        application_name: String,
    ) -> Result<PksSessionHandle, AbiError> {
        if self.session_created || self.sessions.active_count() != 0 {
            return Err(AbiError::NoCapacity);
        }
        if self.host.is_none() {
            self.host = Some(
                SessionEngineHost::native(self.options).map_err(|_| AbiError::BackendFailure)?,
            );
        }
        let session = Session::new();
        let application = session
            .capture(Source::application(ApplicationSelector::name(
                application_name,
            )))
            .map_err(|_| AbiError::InvalidArgument)?;
        let microphone = session
            .capture(Source::microphone_default())
            .map_err(|_| AbiError::InvalidArgument)?;
        let audio = session
            .polled_audio()
            .map_err(|_| AbiError::InvalidArgument)?;
        let _ = application
            .send(audio)
            .map_err(|_| AbiError::InvalidArgument)?;
        let _ = microphone
            .send(audio)
            .map_err(|_| AbiError::InvalidArgument)?;
        let handle = self.sessions.insert(SessionObject::Draft(session))?;
        self.session_created = true;
        Ok(handle)
    }

    pub fn register_executable_extension(
        &mut self,
        registration: ExecutableExtensionRegistration,
    ) -> Result<(), AbiError> {
        if self.session_created || self.host.is_some() {
            return Err(AbiError::InvalidLifecycleState);
        }
        if self
            .executable_extensions
            .iter()
            .any(|existing| existing.id() == registration.id())
        {
            return Err(AbiError::InvalidArgument);
        }
        self.executable_extensions.push(registration);
        Ok(())
    }

    pub fn create_executable_extension_session(
        &mut self,
        pipeline: ExecutableExtensionPipeline,
    ) -> Result<PksSessionHandle, AbiError> {
        if self.session_created || self.sessions.active_count() != 0 || self.host.is_some() {
            return Err(AbiError::NoCapacity);
        }
        let source_factory = self
            .executable_extensions
            .iter()
            .find_map(|registration| match registration {
                ExecutableExtensionRegistration::Source { id, factory }
                    if id == &pipeline.source_id =>
                {
                    Some(Arc::clone(factory))
                }
                _ => None,
            })
            .ok_or(AbiError::InvalidArgument)?;
        let operator_factory = self
            .executable_extensions
            .iter()
            .find_map(|registration| match registration {
                ExecutableExtensionRegistration::Operator { id, factory }
                    if id == &pipeline.operator_id =>
                {
                    Some(Arc::clone(factory))
                }
                _ => None,
            })
            .ok_or(AbiError::InvalidArgument)?;
        let (endpoint_definition, endpoint_factory) = self
            .executable_extensions
            .iter()
            .find_map(|registration| match registration {
                ExecutableExtensionRegistration::Endpoint {
                    id,
                    definition,
                    factory,
                } if id == &pipeline.endpoint_id => {
                    Some((Arc::clone(definition), Arc::clone(factory)))
                }
                _ => None,
            })
            .ok_or(AbiError::InvalidArgument)?;

        let mut host_builder =
            SessionEngineHostBuilder::native(self.options).map_err(|_| AbiError::BackendFailure)?;
        host_builder
            .engine_builder()
            .register_source_factory(source_factory)
            .map_err(|_| AbiError::InvalidArgument)?;
        host_builder
            .register_async_operator(operator_factory)
            .map_err(|_| AbiError::InvalidArgument)?;
        host_builder
            .register_endpoint(
                OperatorId::new(pipeline.endpoint_id.clone()),
                endpoint_definition,
                endpoint_factory,
            )
            .map_err(|_| AbiError::InvalidArgument)?;
        self.host = Some(host_builder.build().map_err(|_| AbiError::BackendFailure)?);

        let session = Session::new();
        let source = session
            .source(
                SourceTypeId::new(pipeline.source_id.clone())
                    .map_err(|_| AbiError::InvalidArgument)?,
                SourceConfiguration::default(),
            )
            .map_err(|_| AbiError::InvalidArgument)?;
        let source_output = source
            .output(pipeline.source_output_port)
            .map_err(|_| AbiError::InvalidArgument)?;
        let operator = session
            .operator(Operator::new(
                OperatorId::new(pipeline.operator_id),
                OperatorConfiguration::new(),
            ))
            .map_err(|_| AbiError::InvalidArgument)?;
        source_output
            .connect(
                operator
                    .input(pipeline.operator_input_port)
                    .map_err(|_| AbiError::InvalidArgument)?,
            )
            .map_err(|_| AbiError::InvalidArgument)?;
        let endpoint = session
            .endpoint(
                EndpointDescriptor::new(
                    crate::graph::NodeTypeId::from(pipeline.endpoint_id.as_str()),
                    OperatorId::new(pipeline.endpoint_id),
                )
                .with_configuration(EndpointConfiguration::new()),
            )
            .map_err(|_| AbiError::InvalidArgument)?;
        operator
            .output(pipeline.operator_output_port)
            .map_err(|_| AbiError::InvalidArgument)?
            .send_to(endpoint, Some(pipeline.endpoint_input_port))
            .map_err(|_| AbiError::InvalidArgument)?;

        let handle = self.sessions.insert(SessionObject::Draft(session))?;
        self.session_created = true;
        Ok(handle)
    }

    pub fn compile_session(&mut self, handle: PksSessionHandle) -> Result<(), AbiError> {
        let object = self.sessions.get_mut(handle)?;
        let current = std::mem::replace(object, SessionObject::Transitioning);
        let SessionObject::Draft(session) = current else {
            *object = current;
            return Err(AbiError::InvalidLifecycleState);
        };
        let host = self.host.as_ref().ok_or(AbiError::InvalidLifecycleState)?;
        match host.compile(session) {
            Ok(compiled) => {
                *object = SessionObject::Compiled(compiled);
                Ok(())
            }
            Err(_) => {
                *object = SessionObject::Failed { events: None };
                Err(AbiError::BackendFailure)
            }
        }
    }

    pub fn start_session(
        &mut self,
        handle: PksSessionHandle,
        start_cancellation: SessionStartCancellation,
    ) -> Result<(), AbiError> {
        let object = self.sessions.get_mut(handle)?;
        let current = std::mem::replace(object, SessionObject::Transitioning);
        let SessionObject::Compiled(compiled) = current else {
            *object = current;
            return Err(AbiError::InvalidLifecycleState);
        };
        let host = self.host.as_ref().ok_or(AbiError::InvalidLifecycleState)?;
        match host.start_compiled_cancellable(compiled, start_cancellation) {
            Ok(mut session) => {
                let events = session.take_event_receiver();
                *object = SessionObject::Running {
                    session,
                    events,
                    stopped_successfully: None,
                };
                Ok(())
            }
            Err(error) => {
                let cancelled = matches!(
                    error.start_failure().map(|failure| failure.error()),
                    Some(SessionStartError::Cancelled { .. })
                );
                let mut events = None;
                if let Some(mut failure) = error.into_start_failure() {
                    events = failure.take_event_receiver();
                }
                *object = SessionObject::Failed { events };
                Err(if cancelled {
                    AbiError::Cancelled
                } else {
                    AbiError::BackendFailure
                })
            }
        }
    }

    pub fn stop_session(&mut self, handle: PksSessionHandle) -> Result<(), AbiError> {
        self.sessions.get_mut(handle)?.request_stop()
    }

    pub fn session_state(
        &self,
        handle: PksSessionHandle,
    ) -> Result<PksSessionLifecycleState, AbiError> {
        self.sessions
            .get(handle)
            .map(SessionObject::lifecycle_state)
    }

    pub fn session_events(
        &self,
        handle: PksSessionHandle,
    ) -> Result<Option<&SessionEventReceiver>, AbiError> {
        self.sessions.get(handle).map(SessionObject::events)
    }

    pub fn poll_event(&self, handle: PksSessionHandle) -> Result<PksSessionEvent, AbiError> {
        let Some(events) = self.session_events(handle)? else {
            return Err(AbiError::InvalidLifecycleState);
        };
        match events.try_recv() {
            SessionEventReceive::Event(event) => Ok(event_record(event)),
            SessionEventReceive::Empty => Err(AbiError::WouldBlock),
            SessionEventReceive::Closed => Err(AbiError::Cancelled),
        }
    }

    pub fn metrics(&self, handle: PksSessionHandle) -> Result<SessionMetricsSnapshot, AbiError> {
        match self.sessions.get(handle)? {
            SessionObject::Running {
                session, events, ..
            } => self
                .host
                .as_ref()
                .ok_or(AbiError::InvalidLifecycleState)?
                .metrics_snapshot(
                    events.as_ref().ok_or(AbiError::InvalidLifecycleState)?,
                    0,
                    Some(session),
                )
                .ok_or(AbiError::BackendFailure),
            SessionObject::Failed { events } => self
                .host
                .as_ref()
                .ok_or(AbiError::InvalidLifecycleState)?
                .metrics_snapshot(
                    events.as_ref().ok_or(AbiError::InvalidLifecycleState)?,
                    0,
                    None,
                )
                .ok_or(AbiError::BackendFailure),
            SessionObject::Draft(_) | SessionObject::Compiled(_) | SessionObject::Transitioning => {
                Err(AbiError::InvalidLifecycleState)
            }
        }
    }

    pub fn poll_audio(
        &mut self,
        session_handle: PksSessionHandle,
    ) -> Result<(PksSessionHandle, u32), AbiError> {
        if self.session_state(session_handle)? != PksSessionLifecycleState::Running {
            return Err(AbiError::InvalidLifecycleState);
        }
        let lease = self
            .host
            .as_ref()
            .ok_or(AbiError::InvalidLifecycleState)?
            .polled_audio_receipt(0)
            .ok_or(AbiError::BackendFailure)?
            .try_poll()
            .map_err(map_poll_error)?;
        let frame_count = u32::try_from(lease.len()).map_err(|_| AbiError::InvalidArgument)?;
        let handle = self.leases.insert(lease)?;
        Ok((handle, frame_count))
    }

    pub fn audio_frame(
        &self,
        batch_handle: PksSessionHandle,
        frame_index: u32,
    ) -> Result<PksSessionAudioFrame, AbiError> {
        let lease = self.leases.get(batch_handle)?;
        let frame = lease
            .frame(frame_index as usize)
            .ok_or(AbiError::IndexOutOfRange)?;
        let lineage = frame.lineage();
        let samples = frame.samples();
        Ok(PksSessionAudioFrame {
            struct_size_bytes: std::mem::size_of::<PksSessionAudioFrame>() as u32,
            abi_major: PKS_SESSION_ABI_MAJOR,
            abi_minor: PKS_SESSION_ABI_MINOR,
            sample_format: PksSessionSampleFormat::F32Interleaved,
            sample_rate_hz: frame.sample_rate_hz(),
            channel_count: u32::from(frame.channels()),
            reserved: 0,
            session_id: lineage.session_id.0,
            source_id: lineage.source_id.0,
            stem_id: lineage.stem_id.0,
            clock_id: u64::from(lineage.clock_id.0),
            sequence_num: lineage.sequence_num,
            timestamp_start_ns: lineage.timestamp_start_ns,
            duration_ns: lineage.duration_ns,
            source_generation: lineage.source_generation,
            discontinuity_epoch: lineage.discontinuity_epoch,
            permission_epoch: lineage.permission_epoch,
            endpoint_id: frame.endpoint_id().0,
            connector_id: frame.connector_id().0,
            route_id: frame.route_id().0,
            samples: samples.as_ptr(),
            sample_count: u32::try_from(samples.len()).map_err(|_| AbiError::InvalidArgument)?,
            reserved_tail: 0,
        })
    }

    pub fn release_audio(&mut self, batch_handle: PksSessionHandle) -> Result<(), AbiError> {
        let _ = self.leases.remove(batch_handle)?;
        Ok(())
    }

    pub fn destroy_session(&mut self, handle: PksSessionHandle) -> Result<(), AbiError> {
        let mut object = self.sessions.remove(handle)?;
        if matches!(object.lifecycle_state(), PksSessionLifecycleState::Running) {
            object.request_stop()?;
        }
        Ok(())
    }

    fn shutdown(&mut self) -> Result<(), AbiError> {
        let mut failed = false;
        self.sessions.for_each_mut(|object| {
            if matches!(object.lifecycle_state(), PksSessionLifecycleState::Running)
                && object.request_stop().is_err()
            {
                failed = true;
            }
        });
        if failed {
            Err(AbiError::BackendFailure)
        } else {
            Ok(())
        }
    }
}

fn map_poll_error(error: PolledAudioPollError) -> AbiError {
    match error {
        PolledAudioPollError::Empty => AbiError::WouldBlock,
        PolledAudioPollError::LeaseCapacityExhausted => AbiError::NoCapacity,
        PolledAudioPollError::StatePoisoned => AbiError::BackendFailure,
    }
}

fn event_record(event: SessionEvent) -> PksSessionEvent {
    let mut record = PksSessionEvent {
        struct_size_bytes: std::mem::size_of::<PksSessionEvent>() as u32,
        abi_major: PKS_SESSION_ABI_MAJOR,
        abi_minor: PKS_SESSION_ABI_MINOR,
        kind: PksSessionEventKind::Lifecycle as u32,
        lifecycle_state: PksSessionLifecycleState::Failed as u32,
        reserved: 0,
        session_id: event.session_id().0,
        stem_id: 0,
        endpoint_id: 0,
        route_id: 0,
        failures_total: 0,
    };
    match event.kind() {
        SessionEventKind::Lifecycle(state) => {
            record.kind = PksSessionEventKind::Lifecycle as u32;
            record.lifecycle_state = lifecycle_state_from_event(*state) as u32;
        }
        SessionEventKind::Source(failure) => {
            record.kind = PksSessionEventKind::SourceFailure as u32;
            record.stem_id = failure.stem_id().0;
            record.failures_total = 1;
        }
        SessionEventKind::Endpoint(failure) => {
            record.kind = PksSessionEventKind::EndpointFailure as u32;
            record.endpoint_id = failure.endpoint_id().0;
            record.route_id = failure.route_id().0;
            record.failures_total = 1;
        }
        SessionEventKind::Rollback(_) => {
            record.kind = PksSessionEventKind::RollbackFailure as u32;
            record.failures_total = 1;
        }
        SessionEventKind::Finalization(_) => {
            record.kind = PksSessionEventKind::FinalizationFailure as u32;
            record.failures_total = 1;
        }
        SessionEventKind::Terminal(outcome) => {
            record.kind = PksSessionEventKind::Terminal as u32;
            record.lifecycle_state = match outcome.state() {
                crate::session::SessionTerminalState::Stopped => {
                    PksSessionLifecycleState::Stopped as u32
                }
                crate::session::SessionTerminalState::Failed => {
                    PksSessionLifecycleState::Failed as u32
                }
            };
            record.failures_total = outcome.source_failures().len() as u64
                + outcome.endpoint_failures().len() as u64
                + outcome.rollback_failures().len() as u64
                + outcome.finalization_failures().len() as u64;
        }
    }
    record
}

struct EngineRuntime {
    runtime: Mutex<SessionRuntime>,
    start_cancellation: SessionStartCancellation,
    lifecycle_state: AtomicU32,
    session_handle: OnceLock<PksSessionHandle>,
    session_scope_id: u64,
    session_live: std::sync::atomic::AtomicBool,
}

impl EngineRuntime {
    fn new(runtime: SessionRuntime) -> Self {
        let session_scope_id = runtime.session_scope_id;
        Self {
            runtime: Mutex::new(runtime),
            start_cancellation: SessionStartCancellation::default(),
            lifecycle_state: AtomicU32::new(PksSessionLifecycleState::Draft as u32),
            session_handle: OnceLock::new(),
            session_scope_id,
            session_live: std::sync::atomic::AtomicBool::new(false),
        }
    }

    fn state(&self) -> PksSessionLifecycleState {
        match self.lifecycle_state.load(Ordering::Acquire) {
            value if value == PksSessionLifecycleState::Draft as u32 => {
                PksSessionLifecycleState::Draft
            }
            value if value == PksSessionLifecycleState::Compiled as u32 => {
                PksSessionLifecycleState::Compiled
            }
            value if value == PksSessionLifecycleState::Starting as u32 => {
                PksSessionLifecycleState::Starting
            }
            value if value == PksSessionLifecycleState::Running as u32 => {
                PksSessionLifecycleState::Running
            }
            value if value == PksSessionLifecycleState::Stopping as u32 => {
                PksSessionLifecycleState::Stopping
            }
            value if value == PksSessionLifecycleState::Stopped as u32 => {
                PksSessionLifecycleState::Stopped
            }
            _ => PksSessionLifecycleState::Failed,
        }
    }

    fn set_state(&self, state: PksSessionLifecycleState) {
        self.lifecycle_state.store(state as u32, Ordering::Release);
    }

    fn validate_session(&self, handle: PksSessionHandle) -> Result<(), AbiError> {
        if handle.kind != PksSessionHandleKind::Session {
            return Err(AbiError::InvalidHandle);
        }
        if handle.scope_id != self.session_scope_id {
            return Err(AbiError::ForeignHandle);
        }
        let expected = self.session_handle.get().ok_or(AbiError::InvalidHandle)?;
        if !self.session_live.load(Ordering::Acquire) {
            return Err(AbiError::StaleHandle);
        }
        if handle != *expected {
            return Err(AbiError::StaleHandle);
        }
        Ok(())
    }
}

pub struct RuntimeState {
    engines: Mutex<HandleTable<Arc<EngineRuntime>>>,
}

impl RuntimeState {
    pub fn new() -> Self {
        Self {
            engines: Mutex::new(HandleTable::new(
                DEFAULT_ENGINE_CAPACITY_COUNT,
                PksSessionHandleKind::Engine,
                ENGINE_HANDLE_SCOPE_ID,
            )),
        }
    }

    pub fn allocate_engine(
        &self,
        options: NativeSessionEngineHostOptions,
    ) -> Result<PksSessionHandle, AbiError> {
        let runtime = SessionRuntime::new(options)?;
        self.allocate_runtime(runtime)
    }

    #[cfg(any(test, feature = "conformance-fixtures"))]
    pub fn allocate_engine_with_host(
        &self,
        host: SessionEngineHost,
        lease_capacity_count: usize,
    ) -> Result<PksSessionHandle, AbiError> {
        self.allocate_runtime(SessionRuntime::with_host(host, lease_capacity_count))
    }

    fn allocate_runtime(&self, runtime: SessionRuntime) -> Result<PksSessionHandle, AbiError> {
        self.engines
            .lock()
            .map_err(|_| AbiError::BackendFailure)?
            .insert(Arc::new(EngineRuntime::new(runtime)))
    }

    pub fn release_engine(&self, handle: PksSessionHandle) -> Result<(), AbiError> {
        let engine = self
            .engines
            .lock()
            .map_err(|_| AbiError::BackendFailure)?
            .remove(handle)?;
        engine.start_cancellation.request();
        let result = engine
            .runtime
            .lock()
            .map_err(|_| AbiError::BackendFailure)?
            .shutdown();
        result
    }

    pub fn engine_is_live(&self, handle: PksSessionHandle) -> Result<bool, AbiError> {
        self.engine(handle).map(|_| true)
    }

    fn engine(&self, handle: PksSessionHandle) -> Result<Arc<EngineRuntime>, AbiError> {
        self.engines
            .lock()
            .map_err(|_| AbiError::BackendFailure)?
            .get(handle)
            .cloned()
    }

    pub fn with_engine_mut<T>(
        &self,
        handle: PksSessionHandle,
        operation: impl FnOnce(&mut SessionRuntime) -> Result<T, AbiError>,
    ) -> Result<T, AbiError> {
        let engine = self.engine(handle)?;
        let mut runtime = engine
            .runtime
            .lock()
            .map_err(|_| AbiError::BackendFailure)?;
        operation(&mut runtime)
    }

    pub fn with_engine<T>(
        &self,
        handle: PksSessionHandle,
        operation: impl FnOnce(&SessionRuntime) -> Result<T, AbiError>,
    ) -> Result<T, AbiError> {
        let engine = self.engine(handle)?;
        let runtime = engine
            .runtime
            .lock()
            .map_err(|_| AbiError::BackendFailure)?;
        operation(&runtime)
    }

    pub fn create_session(
        &self,
        engine_handle: PksSessionHandle,
        application_name: String,
    ) -> Result<PksSessionHandle, AbiError> {
        let engine = self.engine(engine_handle)?;
        let handle = engine
            .runtime
            .lock()
            .map_err(|_| AbiError::BackendFailure)?
            .create_app_mic_session(application_name)?;
        engine
            .session_handle
            .set(handle)
            .map_err(|_| AbiError::NoCapacity)?;
        engine.session_live.store(true, Ordering::Release);
        engine.set_state(PksSessionLifecycleState::Draft);
        Ok(handle)
    }

    pub fn register_executable_extension(
        &self,
        engine_handle: PksSessionHandle,
        registration: ExecutableExtensionRegistration,
    ) -> Result<(), AbiError> {
        self.with_engine_mut(engine_handle, |runtime| {
            runtime.register_executable_extension(registration)
        })
    }

    pub fn create_executable_extension_session(
        &self,
        engine_handle: PksSessionHandle,
        pipeline: ExecutableExtensionPipeline,
    ) -> Result<PksSessionHandle, AbiError> {
        let engine = self.engine(engine_handle)?;
        let handle = engine
            .runtime
            .lock()
            .map_err(|_| AbiError::BackendFailure)?
            .create_executable_extension_session(pipeline)?;
        engine
            .session_handle
            .set(handle)
            .map_err(|_| AbiError::NoCapacity)?;
        engine.session_live.store(true, Ordering::Release);
        engine.set_state(PksSessionLifecycleState::Draft);
        Ok(handle)
    }

    pub fn compile_session(
        &self,
        engine_handle: PksSessionHandle,
        session_handle: PksSessionHandle,
    ) -> Result<(), AbiError> {
        let engine = self.engine(engine_handle)?;
        engine.validate_session(session_handle)?;
        let result = engine
            .runtime
            .lock()
            .map_err(|_| AbiError::BackendFailure)?
            .compile_session(session_handle);
        engine.set_state(if result.is_ok() {
            PksSessionLifecycleState::Compiled
        } else {
            PksSessionLifecycleState::Failed
        });
        result
    }

    pub fn start_session(
        &self,
        engine_handle: PksSessionHandle,
        session_handle: PksSessionHandle,
    ) -> Result<(), AbiError> {
        let engine = self.engine(engine_handle)?;
        engine.validate_session(session_handle)?;
        if engine
            .lifecycle_state
            .compare_exchange(
                PksSessionLifecycleState::Compiled as u32,
                PksSessionLifecycleState::Starting as u32,
                Ordering::AcqRel,
                Ordering::Acquire,
            )
            .is_err()
        {
            return Err(AbiError::InvalidLifecycleState);
        }
        let result = engine
            .runtime
            .lock()
            .map_err(|_| AbiError::BackendFailure)?
            .start_session(session_handle, engine.start_cancellation.clone());
        match result {
            Ok(()) => {
                if engine.start_cancellation.is_requested()
                    || engine
                        .lifecycle_state
                        .compare_exchange(
                            PksSessionLifecycleState::Starting as u32,
                            PksSessionLifecycleState::Running as u32,
                            Ordering::AcqRel,
                            Ordering::Acquire,
                        )
                        .is_err()
                {
                    engine.set_state(PksSessionLifecycleState::Stopping);
                    let stop_result = engine
                        .runtime
                        .lock()
                        .map_err(|_| AbiError::BackendFailure)?
                        .stop_session(session_handle);
                    engine.set_state(if stop_result.is_ok() {
                        PksSessionLifecycleState::Stopped
                    } else {
                        PksSessionLifecycleState::Failed
                    });
                    Err(AbiError::Cancelled)
                } else {
                    Ok(())
                }
            }
            Err(AbiError::Cancelled) => {
                engine.set_state(PksSessionLifecycleState::Stopped);
                Err(AbiError::Cancelled)
            }
            Err(error) => {
                engine.set_state(PksSessionLifecycleState::Failed);
                Err(error)
            }
        }
    }

    pub fn stop_session(
        &self,
        engine_handle: PksSessionHandle,
        session_handle: PksSessionHandle,
    ) -> Result<(), AbiError> {
        let engine = self.engine(engine_handle)?;
        engine.validate_session(session_handle)?;
        engine.start_cancellation.request();
        match engine.state() {
            PksSessionLifecycleState::Starting => {
                engine.set_state(PksSessionLifecycleState::Stopping);
            }
            PksSessionLifecycleState::Running => {
                engine.set_state(PksSessionLifecycleState::Stopping);
            }
            PksSessionLifecycleState::Stopping => {}
            PksSessionLifecycleState::Stopped => return Ok(()),
            PksSessionLifecycleState::Draft
            | PksSessionLifecycleState::Compiled
            | PksSessionLifecycleState::Failed => {
                return Err(AbiError::InvalidLifecycleState);
            }
        }
        let mut runtime = engine
            .runtime
            .lock()
            .map_err(|_| AbiError::BackendFailure)?;
        let result = match runtime.session_state(session_handle) {
            Ok(PksSessionLifecycleState::Running) => runtime.stop_session(session_handle),
            Ok(PksSessionLifecycleState::Failed) => Ok(()),
            Ok(PksSessionLifecycleState::Stopped) => Ok(()),
            Ok(_) => Err(AbiError::InvalidLifecycleState),
            Err(error) => Err(error),
        };
        engine.set_state(if result.is_ok() {
            PksSessionLifecycleState::Stopped
        } else {
            PksSessionLifecycleState::Failed
        });
        result
    }

    pub fn session_state(
        &self,
        engine_handle: PksSessionHandle,
        session_handle: PksSessionHandle,
    ) -> Result<PksSessionLifecycleState, AbiError> {
        let engine = self.engine(engine_handle)?;
        engine.validate_session(session_handle)?;
        Ok(engine.state())
    }

    pub fn destroy_session(
        &self,
        engine_handle: PksSessionHandle,
        session_handle: PksSessionHandle,
    ) -> Result<(), AbiError> {
        let engine = self.engine(engine_handle)?;
        engine.validate_session(session_handle)?;
        let result = engine
            .runtime
            .lock()
            .map_err(|_| AbiError::BackendFailure)?
            .destroy_session(session_handle);
        if result.is_ok() {
            engine.session_live.store(false, Ordering::Release);
        }
        result
    }
}

pub fn lifecycle_state_from_event(state: SessionLifecycleState) -> PksSessionLifecycleState {
    match state {
        SessionLifecycleState::Starting => PksSessionLifecycleState::Starting,
        SessionLifecycleState::Running => PksSessionLifecycleState::Running,
        SessionLifecycleState::Stopping => PksSessionLifecycleState::Stopping,
        SessionLifecycleState::Stopped => PksSessionLifecycleState::Stopped,
        SessionLifecycleState::Failed => PksSessionLifecycleState::Failed,
    }
}
