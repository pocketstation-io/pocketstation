# Observation API

<!-- claims: CLM-REF-017-CAP-001,CLM-REF-017-CAP-002,CLM-REF-017-SOURCE-001 -->

## Scope

- **Observe Session metrics and events.** Read route, source, operator, sidecar, endpoint, drop, latency, queue, and terminal observations.
- **Record and validate Session traces.** Persist lifecycle trace records and validate their structural and terminal consistency.

These statements describe repository contracts at the documented snapshot. They do not extend platform qualification, performance, retry, or delivery guarantees beyond the native API contracts and executable evidence.

## Reference authority

The generated [docs.rs API](https://docs.rs/pocketstation/latest/pocketstation/) is the exhaustive symbol-level Rust reference. Human reference pages organize responsibilities and cross-boundary behavior; they do not duplicate every rustdoc signature.

## Public surface

| Declaration | Kind | Purpose | Source |
|---|---|---|---|
| `INITIAL` | assoc_const | Provides the initial value for `PermissionEpoch`. | `src/capture/authorization.rs:270` |
| `INITIAL` | assoc_const | Provides the initial value for `SourceGeneration`. | `src/capture/events.rs:15` |
| `Error` | assoc_type | Specifies the error type returned by `SidecarMessageKind` operations. | `src/runtime/lifecycle/sidecar_protocol.rs:23` |
| `pocketstation::connector::configuration::MAX_CONNECTOR_CONFIGURATION_FIELDS` | constant | Sets the maximum supported connector configuration fields. | `src/connector/configuration.rs:7` |
| `pocketstation::connector::configuration::MAX_CONNECTOR_CONFIGURATION_TEXT_BYTES` | constant | Sets the maximum supported connector configuration text bytes. | `src/connector/configuration.rs:8` |
| `pocketstation::connector::error::MAX_CONNECTOR_ERROR_CODE_BYTES` | constant | Sets the maximum supported connector error code bytes. | `src/connector/error.rs:6` |
| `pocketstation::connector::error::MAX_CONNECTOR_ERROR_MESSAGE_BYTES` | constant | Sets the maximum supported connector error message bytes. | `src/connector/error.rs:7` |
| `pocketstation::connector::manifest::CONNECTOR_API_REVISION` | constant | Defines the public connector API revision value. | `src/connector/manifest.rs:7` |
| `pocketstation::connector::manifest::MAX_CONNECTOR_MANIFEST_ENTRIES` | constant | Sets the maximum supported connector manifest entries. | `src/connector/manifest.rs:8` |
| `pocketstation::connector::manifest::MAX_CONNECTOR_MANIFEST_TEXT_BYTES` | constant | Sets the maximum supported connector manifest text bytes. | `src/connector/manifest.rs:9` |
| `pocketstation::connector::readiness::MAX_CONNECTOR_READINESS_THRESHOLD` | constant | Sets the maximum supported connector readiness threshold. | `src/connector/readiness.rs:3` |
| `pocketstation::connector::readiness::MAX_CONNECTOR_READINESS_TIMEOUT` | constant | Sets the maximum supported connector readiness timeout. | `src/connector/readiness.rs:4` |
| `pocketstation::connector::sidecar::CONNECTOR_AUDIO_RECORD_SCHEMA` | constant | Defines the public connector audio record schema value. | `src/connector/sidecar.rs:16` |
| `pocketstation::connector::sidecar::CONNECTOR_AUDIO_RECORD_SIGNAL_ID` | constant | Defines the public connector audio record signal identifier value. | `src/connector/sidecar.rs:15` |
| `pocketstation::connector::transport::CONNECTOR_AUDIO_RECORD_MAJOR` | constant | Defines the major version of connector audio record. | `src/connector/transport.rs:19` |
| `pocketstation::connector::transport::CONNECTOR_AUDIO_RECORD_MINOR` | constant | Defines the minor version of connector audio record. | `src/connector/transport.rs:20` |
| `pocketstation::connector::transport::CONNECTOR_CONFIGURATION_RECORD_MAJOR` | constant | Defines the major version of connector configuration record. | `src/connector/transport.rs:35` |
| `pocketstation::connector::transport::CONNECTOR_CONFIGURATION_RECORD_MINOR` | constant | Defines the minor version of connector configuration record. | `src/connector/transport.rs:36` |
| `pocketstation::connector::transport::MAX_CONNECTOR_AUDIO_RECORD_PORT_BYTES` | constant | Sets the maximum supported connector audio record port bytes. | `src/connector/transport.rs:21` |
| `pocketstation::connector::transport::MAX_CONNECTOR_AUDIO_RECORD_SAMPLES` | constant | Sets the maximum supported connector audio record samples. | `src/connector/transport.rs:22` |
| `pocketstation::session::extensions::audio_input::PCM_SOURCE_TYPE_ID` | constant | Stable runtime identity of the underlying PCM source implementation. | `src/session/extensions/audio_input/mod.rs:19` |
| `pocketstation::capture::authorization::ApplicationPolicyObservation` | enum | Classifies the observable application policy observation. | `src/capture/authorization.rs:231` |
| `pocketstation::capture::authorization::CaptureCapabilityState` | enum | Selects the capture capability state used by PocketStation. | `src/capture/authorization.rs:145` |
| `pocketstation::capture::authorization::CaptureError` | enum | Classifies failures reported as capture error. | `src/capture/authorization.rs:290` |
| `pocketstation::capture::authorization::CaptureOpenOutcome` | enum | Classifies the observable capture open outcome. | `src/capture/authorization.rs:281` |
| `pocketstation::capture::authorization::CaptureScope` | enum | Selects the capture scope used by PocketStation. | `src/capture/authorization.rs:248` |
| `pocketstation::capture::authorization::CaptureSessionGrant` | enum | Enumerates the supported capture session grant cases. | `src/capture/authorization.rs:240` |
| `pocketstation::capture::authorization::PermissionObservation` | enum | Classifies the observable permission observation. | `src/capture/authorization.rs:153` |
| `pocketstation::capture::authorization::SourceIdentityStrength` | enum | Enumerates the supported source identity strength cases. | `src/capture/authorization.rs:257` |
| `pocketstation::capture::events::CaptureRuntimeFailureClass` | enum | Enumerates the supported capture runtime failure class cases. | `src/capture/events.rs:40` |
| `pocketstation::capture::events::SourceLifecycleEventKind` | enum | Selects the source lifecycle event kind used by PocketStation. | `src/capture/events.rs:25` |
| `pocketstation::capture::events::SourceRecoveryRequirement` | enum | Selects the source recovery requirement used by PocketStation. | `src/capture/events.rs:35` |
| `pocketstation::capture::events::SourceRuntimeEvent` | enum | Classifies the observable source runtime event. | `src/capture/events.rs:53` |
| `pocketstation::capture::events::SourceRuntimeEventDelivery` | enum | Enumerates the supported source runtime event delivery cases. | `src/capture/events.rs:96` |
| `pocketstation::capture::frame_stream::CapturedFrameDelivery` | enum | Enumerates the supported captured frame delivery cases. | `src/capture/frame_stream.rs:10` |
| `pocketstation::capture::identity::SourceKind` | enum | Selects the source kind used by PocketStation. | `src/capture/identity.rs:9` |
| `pocketstation::capture::identity::SourceState` | enum | Selects the source state used by PocketStation. | `src/capture/identity.rs:17` |
| `pocketstation::capture::query::SourceQuery` | enum | Enumerates the supported source query cases. | `src/capture/query.rs:13` |
| `pocketstation::capture::selection::CaptureMode` | enum | Selects the capture mode used by PocketStation. | `src/capture/selection.rs:16` |
| `pocketstation::capture::selection::InputDeviceSelector` | enum | Enumerates the supported input device selector cases. | `src/capture/selection.rs:9` |
| `pocketstation::capture::selection::ProcessTreeScope` | enum | Selects the process tree scope used by PocketStation. | `src/capture/selection.rs:83` |
| `pocketstation::capture::selection::SelectorPersistenceScope` | enum | Selects the selector persistence scope used by PocketStation. | `src/capture/selection.rs:73` |
| `pocketstation::connector::ConnectorDeclarationError` | enum | Classifies failures reported as connector declaration error. | `src/connector/mod.rs:233` |
| `pocketstation::connector::ConnectorObservationLookupError` | enum | Classifies failures reported as connector observation lookup error. | `src/connector/mod.rs:246` |
| `pocketstation::connector::ConnectorRegistrationError` | enum | Classifies failures reported as connector registration error. | `src/connector/mod.rs:225` |
| `pocketstation::connector::configuration::ConnectorConfigurationConstraint` | enum | Enumerates the supported connector configuration constraint cases. | `src/connector/configuration.rs:159` |
| `pocketstation::connector::configuration::ConnectorConfigurationErrorCode` | enum | Enumerates the supported connector configuration error code cases. | `src/connector/configuration.rs:568` |
| `pocketstation::connector::configuration::ConnectorConfigurationRequirement` | enum | Selects the connector configuration requirement used by PocketStation. | `src/connector/configuration.rs:152` |
| `pocketstation::connector::configuration::ConnectorConfigurationValue` | enum | Enumerates the supported connector configuration value cases. | `src/connector/configuration.rs:66` |
| `pocketstation::connector::configuration::ConnectorConfigurationValueKind` | enum | Selects the connector configuration value kind used by PocketStation. | `src/connector/configuration.rs:55` |
| `pocketstation::connector::error::ConnectorErrorBuildError` | enum | Classifies failures reported as connector error build error. | `src/connector/error.rs:184` |
| `pocketstation::connector::error::ConnectorErrorCodeError` | enum | Classifies failures reported as connector error code error. | `src/connector/error.rs:50` |
| `pocketstation::connector::error::ConnectorErrorStage` | enum | Selects the connector error stage used by PocketStation. | `src/connector/error.rs:60` |
| `pocketstation::connector::error::ConnectorRetryability` | enum | Enumerates the supported connector retryability cases. | `src/connector/error.rs:72` |
| `pocketstation::connector::manifest::ConnectorManifestError` | enum | Classifies failures reported as connector manifest error. | `src/connector/manifest.rs:231` |
| `pocketstation::connector::observations::ConnectorObservationError` | enum | Classifies failures reported as connector observation error. | `src/connector/observations.rs:175` |
| `pocketstation::connector::readiness::ConnectorReadinessPolicyError` | enum | Classifies failures reported as connector readiness policy error. | `src/connector/readiness.rs:61` |
| `pocketstation::connector::status::ConnectorDeliveryReadiness` | enum | Enumerates the supported connector delivery readiness cases. | `src/connector/status.rs:4` |
| `pocketstation::connector::status::ConnectorHealth` | enum | Enumerates the supported connector health cases. | `src/connector/status.rs:17` |
| `pocketstation::connector::status::ConnectorRecovery` | enum | Enumerates the supported connector recovery cases. | `src/connector/status.rs:24` |
| `pocketstation::connector::transport::ConnectorAudioRecordError` | enum | Classifies failures reported as connector audio record error. | `src/connector/transport.rs:568` |
| `pocketstation::connector::transport::ConnectorConfigurationRecordError` | enum | Classifies failures reported as connector configuration record error. | `src/connector/transport.rs:251` |
| `pocketstation::connector::worker::driver::ConnectorDeliveryOutcome` | enum | Explicit delivery result used for Core-owned accounting. | `src/connector/worker/driver.rs:83` |
| `pocketstation::connector::worker::driver::ConnectorItem` | enum | One bounded item delivered by Core to a connector driver. | `src/connector/worker/driver.rs:62` |
| `pocketstation::endpoint::contract::EndpointReceiver` | enum | Enumerates the supported endpoint receiver cases. | `src/endpoint/contract.rs:145` |
| `pocketstation::endpoint::identity::EndpointPreparationGroup` | enum | Factory-owned decision for batching endpoint inputs into one lifecycle. | `src/endpoint/identity.rs:23` |
| `pocketstation::endpoint::polled_audio_driver::PolledAudioEndpointConfigError` | enum | Classifies failures reported as polled audio endpoint config error. | `src/endpoint/polled_audio_driver.rs:40` |
| `pocketstation::endpoint::polled_audio_driver::PolledAudioPollError` | enum | Classifies failures reported as polled audio poll error. | `src/endpoint/polled_audio_driver.rs:74` |
| `pocketstation::endpoint::registry::EndpointDriverRegistryError` | enum | Classifies failures reported as endpoint driver registry error. | `src/endpoint/registry.rs:16` |
| `pocketstation::endpoint::registry::EndpointPrepareError` | enum | Classifies failures reported as endpoint prepare error. | `src/endpoint/registry.rs:39` |
| `pocketstation::endpoint::runtime::EndpointFailureRetryability` | enum | Machine-readable recovery classification retained in Session outcomes. | `src/endpoint/runtime.rs:166` |
| `pocketstation::endpoint::runtime::EndpointFailureStage` | enum | Selects the endpoint failure stage used by PocketStation. | `src/endpoint/runtime.rs:156` |
| `pocketstation::endpoint::runtime::EndpointInputOrigin` | enum | Provenance of one endpoint input, independent of its physical receiver. | `src/endpoint/runtime.rs:31` |
| `pocketstation::endpoint::runtime::EndpointShutdownMode` | enum | Session shutdown intent delivered to an active endpoint. | `src/endpoint/runtime.rs:356` |
| `pocketstation::endpoint::runtime::EndpointStartFailureCause` | enum | Enumerates the supported endpoint start failure cause cases. | `src/endpoint/runtime.rs:438` |
| `pocketstation::runtime::audio::executor::ExecError` | enum | Classifies failures reported as exec error. | `src/runtime/audio/executor.rs:20` |
| `pocketstation::runtime::audio::runner::PlanRunnerError` | enum | Classifies failures reported as plan runner error. | `src/runtime/audio/runner.rs:256` |
| `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostError` | enum | Classifies failures reported as sidecar host error. | `src/runtime/lifecycle/sidecar_host.rs:686` |
| `pocketstation::runtime::lifecycle::sidecar_host::SidecarState` | enum | Selects the sidecar state used by PocketStation. | `src/runtime/lifecycle/sidecar_host.rs:21` |
| `pocketstation::runtime::lifecycle::sidecar_protocol::SidecarMessageKind` | enum | Selects the sidecar message kind used by PocketStation. | `src/runtime/lifecycle/sidecar_protocol.rs:9` |
| `pocketstation::runtime::lifecycle::sidecar_protocol::SidecarProtocolError` | enum | Classifies failures reported as sidecar protocol error. | `src/runtime/lifecycle/sidecar_protocol.rs:292` |
| `pocketstation::session::compile::error::SessionCompileError` | enum | Classifies failures reported as session compile error. | `src/session/compile/error.rs:7` |
| `pocketstation::session::declaration::selector::ApplicationSelector` | enum | Enumerates the supported application selector cases. | `src/session/declaration/selector.rs:32` |
| `pocketstation::session::declaration::selector::DeviceSelector` | enum | Enumerates the supported device selector cases. | `src/session/declaration/selector.rs:107` |
| `pocketstation::session::declaration::selector::Source` | enum | Enumerates the supported source cases. | `src/session/declaration/selector.rs:134` |
| `pocketstation::session::declaration::spec::ConnectionTarget` | enum | Stable destination of a declared Session connection. | `src/session/declaration/spec.rs:224` |
| `pocketstation::session::declaration::spec::StreamOrigin` | enum | Stable origin of a declared Session stream. | `src/session/declaration/spec.rs:208` |
| `pocketstation::session::declaration::typed_stream::TypedStreamError` | enum | Classifies failures reported as typed stream error. | `src/session/declaration/typed_stream.rs:185` |
| `pocketstation::session::error::SessionError` | enum | Classifies failures reported as session error. | `src/session/error.rs:6` |
| `pocketstation::session::error_code::PolledAudioPollErrorCode` | enum | Stable language-neutral code for bounded polled-audio status. | `src/session/error_code.rs:131` |
| `pocketstation::session::error_code::SessionDeclarationErrorCode` | enum | Stable language-neutral code for a Session declaration failure. | `src/session/error_code.rs:10` |
| `pocketstation::session::error_code::SessionRuntimeErrorCode` | enum | Stable language-neutral code for a running-Session projection failure. | `src/session/error_code.rs:116` |
| `pocketstation::session::error_code::SessionStartErrorCode` | enum | Stable language-neutral code for Session startup. | `src/session/error_code.rs:61` |
| `pocketstation::session::error_code::SessionStopCode` | enum | Stable language-neutral status for an idempotent Session stop. | `src/session/error_code.rs:150` |
| `pocketstation::session::error_code::SessionStopFailureCode` | enum | Stable language-neutral cause retained by a failed Session stop. | `src/session/error_code.rs:171` |
| `pocketstation::session::extensions::audio_input::AudioInputConfigError` | enum | Classifies failures reported as audio input config error. | `src/session/extensions/audio_input/mod.rs:77` |
| `pocketstation::session::extensions::audio_input::buffer::AudioInputBufferAcquireError` | enum | Classifies failures reported as audio input buffer acquire error. | `src/session/extensions/audio_input/buffer.rs:271` |
| `pocketstation::session::extensions::audio_input::buffer::AudioInputBufferError` | enum | Classifies failures reported as audio input buffer error. | `src/session/extensions/audio_input/buffer.rs:281` |
| `pocketstation::session::extensions::audio_input::buffer::AudioInputWriteErrorKind` | enum | Selects the audio input write error kind used by PocketStation. | `src/session/extensions/audio_input/buffer.rs:298` |
| `pocketstation::session::extensions::audio_input::source::AudioInputError` | enum | Classifies failures reported as audio input error. | `src/session/extensions/audio_input/source.rs:85` |
| `pocketstation::session::extensions::builtins::SessionGraphRegistrationError` | enum | Classifies failures reported as session graph registration error. | `src/session/extensions/builtins.rs:30` |
| `pocketstation::session::extensions::source::SourceDriverError` | enum | Classifies failures reported as source driver error. | `src/session/extensions/source.rs:748` |
| `pocketstation::session::extensions::source::SourceManifestError` | enum | Classifies failures reported as source manifest error. | `src/session/extensions/source.rs:677` |
| `pocketstation::session::extensions::source::SourceRegistrationError` | enum | Classifies failures reported as source registration error. | `src/session/extensions/source.rs:701` |
| `pocketstation::session::extensions::source::SourceTypeIdError` | enum | Classifies failures reported as source type id error. | `src/session/extensions/source.rs:68` |
| `pocketstation::session::lifecycle::engine::EndpointExtensionRegistrationError` | enum | Classifies failures reported as endpoint extension registration error. | `src/session/lifecycle/engine.rs:305` |
| `pocketstation::session::lifecycle::engine::SessionEngineBuildError` | enum | Classifies failures reported as session engine build error. | `src/session/lifecycle/engine.rs:295` |
| `pocketstation::session::lifecycle::engine::SessionEngineStartError` | enum | Classifies failures reported as session engine start error. | `src/session/lifecycle/engine.rs:315` |
| `pocketstation::session::lifecycle::events::SessionComponentId` | enum | Stable identity of the component that produced a session control failure. | `src/session/lifecycle/events.rs:51` |
| `pocketstation::session::lifecycle::events::SessionEventKind` | enum | Payload of one authoritative session event. | `src/session/lifecycle/events.rs:294` |
| `pocketstation::session::lifecycle::events::SessionEventReceive` | enum | Result of non-blocking event polling. | `src/session/lifecycle/events.rs:492` |
| `pocketstation::session::lifecycle::events::SessionFinalizationStage` | enum | The finalization operation that failed while stopping a session. | `src/session/lifecycle/events.rs:39` |
| `pocketstation::session::lifecycle::events::SessionLifecycleState` | enum | Public lifecycle states emitted by a running session. | `src/session/lifecycle/events.rs:19` |
| `pocketstation::session::lifecycle::events::SessionRollbackStage` | enum | The rollback operation that failed while unwinding a partial start. | `src/session/lifecycle/events.rs:29` |
| `pocketstation::session::lifecycle::events::SessionTerminalState` | enum | Final state carried by the terminal session event. | `src/session/lifecycle/events.rs:210` |
| `pocketstation::session::lifecycle::host::SessionEngineHostBuildError` | enum | Classifies failures reported as session engine host build error. | `src/session/lifecycle/host.rs:362` |
| `pocketstation::session::lifecycle::observations::EndpointObservationStage` | enum | Selects the endpoint observation stage used by PocketStation. | `src/session/lifecycle/observations.rs:441` |
| `pocketstation::session::lifecycle::observations::SessionRouteLatencyBoundary` | enum | Enumerates the supported session route latency boundary cases. | `src/session/lifecycle/observations.rs:196` |
| `pocketstation::session::lifecycle::observations::SessionRouteLatencyUnit` | enum | Enumerates the supported session route latency unit cases. | `src/session/lifecycle/observations.rs:201` |
| `pocketstation::session::lifecycle::observations::SessionRouteObservationInterval` | enum | Interval covered by monotonic route counters. | `src/session/lifecycle/observations.rs:150` |
| `pocketstation::session::lifecycle::start_contract::SessionStartError` | enum | Classifies failures reported as session start error. | `src/session/lifecycle/start_contract.rs:113` |
| `pocketstation::session::lifecycle::trace::SessionTraceRecordKind` | enum | Selects the session trace record kind used by PocketStation. | `src/session/lifecycle/trace.rs:27` |
| `pocketstation::session::lifecycle::trace::SessionTraceRecorderFinishError` | enum | Classifies failures reported as session trace recorder finish error. | `src/session/lifecycle/trace.rs:98` |
| `pocketstation::session::lifecycle::trace::SessionTraceRecorderStartError` | enum | Classifies failures reported as session trace recorder start error. | `src/session/lifecycle/trace.rs:88` |
| `pocketstation::session::lifecycle::trace::SessionTraceValidationError` | enum | Classifies failures reported as session trace validation error. | `src/session/lifecycle/trace.rs:356` |
| `pocketstation::session::prepare::error::SessionPrepareError` | enum | Classifies failures reported as session prepare error. | `src/session/prepare/error.rs:9` |
| `ActiveCaptureBackend::observation_handle` | function | Returns a handle for reading observations from `ActiveCaptureBackend`. | `src/capture/capture_owner.rs:107` |
| `ActiveCaptureBackend::observations` | function | Returns the observations exposed by `ActiveCaptureBackend`. | `src/capture/capture_owner.rs:109` |
| `ActiveCaptureBackend::source_id` | function | Resolved native source identity for every frame emitted by this open. | `src/capture/capture_owner.rs:105` |
| `ActiveCaptureBackend::stop_and_join` | function | Stops and join for `ActiveCaptureBackend`. | `src/capture/capture_owner.rs:111` |
| `CallbackCaptureBackend::prepare` | function | Prepares resources required by `CallbackCaptureBackend`. | `src/capture/capture_owner.rs:84` |
| `ConnectorDriver::cancel_preparation` | function | Cancels preparation for `ConnectorDriver`. | `src/connector/worker/driver.rs:116` |
| `ConnectorDriver::deliver` | function | Delivers the next input through `ConnectorDriver`. | `src/connector/worker/driver.rs:98` |
| `ConnectorDriver::idle` | function | Advances `ConnectorDriver` while no input is available. | `src/connector/worker/driver.rs:104` |
| `ConnectorDriver::shutdown` | function | Shuts down `ConnectorDriver` according to its lifecycle contract. | `src/connector/worker/driver.rs:108` |
| `ConnectorDriver::start` | function | Starts the lifecycle represented by `ConnectorDriver`. | `src/connector/worker/driver.rs:93` |
| `ConnectorDriverFactory::preparation_group` | function | Returns the preparation group associated with `ConnectorDriverFactory`. | `src/connector/worker/driver.rs:124` |
| `ConnectorDriverFactory::prepare` | function | Prepares resources required by `ConnectorDriverFactory`. | `src/connector/worker/driver.rs:132` |
| `ConnectorFactory::preparation_group` | function | Returns the preparation group associated with `ConnectorFactory`. | `src/connector/worker/mod.rs:18` |
| `ConnectorFactory::prepare` | function | Prepares resources required by `ConnectorFactory`. | `src/connector/worker/mod.rs:26` |
| `ConnectorWorker::cancel_preparation` | function | Cancels preparation for `ConnectorWorker`. | `src/connector/worker/mod.rs:35` |
| `ConnectorWorker::run` | function | Runs `ConnectorWorker` until completion or cancellation. | `src/connector/worker/mod.rs:33` |
| `EndpointDriverFactory::preparation_group` | function | Returns the preparation group associated with `EndpointDriverFactory`. | `src/endpoint/contract.rs:233` |
| `EndpointDriverFactory::prepare` | function | Prepares resources required by `EndpointDriverFactory`. | `src/endpoint/contract.rs:241` |
| `PreparedCaptureBackend::open` | function | Opens the resource represented by `PreparedCaptureBackend`. | `src/capture/capture_owner.rs:89` |
| `PreparedEndpointDriver::cancel_preparation` | function | Cancels preparation for `PreparedEndpointDriver`. | `src/endpoint/runtime.rs:328` |
| `PreparedEndpointDriver::start` | function | Makes endpoint resources ready behind the supplied closed gate. | `src/endpoint/runtime.rs:323` |
| `RunningEndpointDriver::join_and_finalize` | function | Joins `RunningEndpointDriver` and returns its finalization outcome. | `src/endpoint/runtime.rs:346` |
| `RunningEndpointDriver::observations` | function | Returns the observations exposed by `RunningEndpointDriver`. | `src/endpoint/runtime.rs:337` |
| `RunningEndpointDriver::request_shutdown` | function | Requests the selected shutdown mode from `RunningEndpointDriver`. | `src/endpoint/runtime.rs:341` |
| `RunningEndpointDriver::request_stop` | function | Requests a graceful stop from `RunningEndpointDriver`. | `src/endpoint/runtime.rs:339` |
| `SourceDriver::close` | function | Closes `SourceDriver` to further work. | `src/session/extensions/source.rs:273` |
| `SourceDriver::next` | function | Produces the next source emission from `SourceDriver`. | `src/session/extensions/source.rs:269` |
| `SourceDriver::prepare` | function | Prepares resources required by `SourceDriver`. | `src/session/extensions/source.rs:268` |
| `SourceFactory::create` | function | Creates the runtime implementation described by `SourceFactory`. | `src/session/extensions/source.rs:279` |
| `SourceFactory::manifest` | function | Returns the manifest associated with `SourceFactory`. | `src/session/extensions/source.rs:277` |
| `SourceFactory::validate_config` | function | Validates config for `SourceFactory`. | `src/session/extensions/source.rs:278` |
| `StreamSignal::signal_spec` | function | Returns the signal spec associated with `StreamSignal`. | `src/session/declaration/typed_stream.rs:16` |
| `accepts_delivery` | function | Returns whether accepts delivery applies to `ConnectorDeliveryReadiness`. | `src/connector/status.rs:10` |
| `accepts_delivery` | function | Returns whether accepts delivery applies to `ConnectorServiceStatus`. | `src/connector/status.rs:74` |
| `after_failed_open` | function | Records a backend open failure without guessing that permission was denied. | `src/capture/authorization.rs:45` |
| `after_successful_open` | function | Records the evidence available after an explicitly selected source opens. | `src/capture/authorization.rs:31` |
| `api_revision` | function | Returns the API revision associated with `ConnectorManifest`. | `src/connector/manifest.rs:124` |
| `application` | function | Returns the application associated with `Source`. | `src/session/declaration/selector.rs:140` |
| `as_str` | function | Returns the stable string representation of `ConnectorConfigurationErrorCode`. | `src/connector/configuration.rs:585` |
| `as_str` | function | Returns the stable string representation of `ConnectorErrorCode`. | `src/connector/error.rs:29` |
| `as_str` | function | Returns the stable string representation of `EndpointGroupId`. | `src/endpoint/identity.rs:16` |
| `as_str` | function | Returns the stable string representation of `DeviceId`. | `src/session/declaration/selector.rs:26` |
| `as_str` | function | Returns the stable string representation of `SessionDeclarationErrorCode`. | `src/session/error_code.rs:31` |
| `as_str` | function | Returns the stable string representation of `SessionStartErrorCode`. | `src/session/error_code.rs:86` |
| `as_str` | function | Returns the stable string representation of `SessionRuntimeErrorCode`. | `src/session/error_code.rs:121` |
| `as_str` | function | Returns the stable string representation of `PolledAudioPollErrorCode`. | `src/session/error_code.rs:138` |
| `as_str` | function | Returns the stable string representation of `SessionStopCode`. | `src/session/error_code.rs:157` |
| `as_str` | function | Returns the stable string representation of `SessionStopFailureCode`. | `src/session/error_code.rs:182` |
| `as_str` | function | Returns the stable string representation of `SourceTypeId`. | `src/session/extensions/source.rs:52` |
| `audio_frames_enqueued_total` | function | Returns the audio frames enqueued total associated with `SessionAudioReentryMetrics`. | `src/session/lifecycle/observations.rs:368` |
| `audio_stem_id` | function | Returns the audio stem identifier associated with `EndpointRouteContext`. | `src/endpoint/runtime.rs:88` |
| `before_open` | function | Records a source that was resolved but not opened because another required source failed first. | `src/capture/authorization.rs:60` |
| `bundle_id` | function | Returns the bundle identifier associated with `ApplicationSelector`. | `src/session/declaration/selector.rs:44` |
| `cancellation_total` | function | Returns the cancellation total associated with `SessionAudioReentryMetrics`. | `src/session/lifecycle/observations.rs:372` |
| `capabilities` | function | Returns the capabilities associated with `ConnectorManifest`. | `src/connector/manifest.rs:152` |
| `capacity_frames` | function | Returns the capacity frames associated with `AudioInputConfig`. | `src/session/extensions/audio_input/mod.rs:63` |
| `capture_finalization_failures_total` | function | Returns the capture finalization failures total associated with `SessionStopOutcome`. | `src/session/lifecycle/start_contract.rs:330` |
| `channels` | function | Returns the channel count represented by `EndpointAudioFrame`. | `src/endpoint/contract.rs:47` |
| `channels` | function | Returns the channel count represented by `PolledAudioFrame`. | `src/endpoint/polled_audio_driver.rs:239` |
| `close` | function | Closes `AudioInputWriter` to further work. | `src/session/extensions/audio_input/buffer.rs:242` |
| `close` | function | Closes this application-owned input after its accepted frames drain. | `src/session/extensions/audio_input/mod.rs:127` |
| `code` | function | Returns the stable error or status code represented by `ConnectorConfigurationError`. | `src/connector/configuration.rs:615` |
| `code` | function | Returns the stable error or status code represented by `ConnectorError`. | `src/connector/error.rs:109` |
| `code` | function | Returns the stable error or status code represented by `EndpointFailure`. | `src/endpoint/runtime.rs:212` |
| `component` | function | Returns the component associated with `SessionControlFailure`. | `src/session/lifecycle/events.rs:89` |
| `configuration` | function | Returns the configuration associated with `ConnectorManifest`. | `src/connector/manifest.rs:144` |
| `configuration` | function | Returns the configuration associated with `ConnectorConfigurationRecord`. | `src/connector/transport.rs:53` |
| `configuration` | function | Returns the configuration associated with `ConnectorInputDescriptor`. | `src/connector/worker/driver.rs:56` |
| `configuration` | function | Returns the configuration associated with `Operator`. | `src/session/declaration/draft.rs:311` |
| `configuration` | function | Returns the configuration associated with `EndpointDescriptor`. | `src/session/declaration/endpoint.rs:149` |
| `configuration` | function | Returns the configuration associated with `SourceInstanceSpec`. | `src/session/declaration/spec.rs:88` |
| `configuration` | function | Returns the configuration associated with `OperatorInstanceSpec`. | `src/session/declaration/spec.rs:253` |
| `configuration` | function | Returns the configuration associated with `AudioInputWriter`. | `src/session/extensions/audio_input/buffer.rs:103` |
| `connect` | function | Connects the requested ports through `StemHandle`. | `src/session/declaration/draft.rs:819` |
| `connect` | function | Connects the requested ports through `SourceOutputHandle`. | `src/session/declaration/draft.rs:955` |
| `connect` | function | Connects the requested ports through `DerivedStreamHandle`. | `src/session/declaration/draft.rs:1051` |
| `connector_id` | function | Returns the connector identifier associated with `ConnectorInputDescriptor`. | `src/connector/worker/driver.rs:32` |
| `connector_id` | function | Returns the connector identifier associated with `PolledAudioFrame`. | `src/endpoint/polled_audio_driver.rs:231` |
| `connector_id` | function | Returns the connector identifier associated with `EndpointPrepareContext`. | `src/endpoint/runtime.rs:138` |
| `connector_id` | function | Returns the connector identifier associated with `EndpointHandle`. | `src/session/declaration/draft.rs:607` |
| `constraints` | function | Returns the constraints associated with `ConnectorConfigurationField`. | `src/connector/configuration.rs:222` |
| `context` | function | Returns the context associated with `EndpointPortInput`. | `src/endpoint/contract.rs:219` |
| `current` | function | Returns the current value observed by `CapturePermissionLifecycle`. | `src/capture/authorization.rs:196` |
| `declare` | function | Adds the declaration represented by `RegisteredConnector` to its Session. | `src/connector/mod.rs:159` |
| `decode` | function | Decodes input through `ConnectorConfigurationRecord`. | `src/connector/transport.rs:93` |
| `decode` | function | Decodes input through `ConnectorAudioRecord`. | `src/connector/transport.rs:403` |
| `decode` | function | Decodes input through `SidecarMessage`. | `src/runtime/lifecycle/sidecar_protocol.rs:121` |
| `default` | function | Returns the default `SidecarDeadlines` value. | `src/runtime/lifecycle/sidecar_host.rs:61` |
| `default` | function | Returns the default `SidecarProtocolLimits` value. | `src/runtime/lifecycle/sidecar_protocol.rs:51` |
| `default` | function | Returns the default `DeviceSelector` value. | `src/session/declaration/selector.rs:113` |
| `delivery_readiness` | function | Returns the delivery readiness associated with `ConnectorServiceStatus`. | `src/connector/status.rs:42` |
| `deprecated` | function | Returns the deprecated associated with `ConnectorConfigurationField`. | `src/connector/configuration.rs:201` |
| `deprecation` | function | Returns the deprecation associated with `ConnectorConfigurationField`. | `src/connector/configuration.rs:226` |
| `derived_route` | function | Returns the derived route associated with `SessionMetricsSnapshot`. | `src/session/lifecycle/observations.rs:111` |
| `derived_route_count` | function | Returns the derived route count associated with `SessionMetricsSnapshot`. | `src/session/lifecycle/observations.rs:107` |
| `discover` | function | Discovers the resources visible to `LocalSourceProvider`. | `src/capture/query.rs:55` |
| `documentation` | function | Returns the documentation associated with `ConnectorConfigurationField`. | `src/connector/configuration.rs:218` |
| `documentation` | function | Returns the documentation associated with `ConnectorCapability`. | `src/connector/manifest.rs:34` |
| `documentation` | function | Returns the documentation associated with `ConnectorRequirement`. | `src/connector/manifest.rs:69` |
| `drop` | function | Releases resources owned by `ConnectorSecret`. | `src/connector/configuration.rs:49` |
| `drop` | function | Releases resources owned by `PolledAudioBatchLease`. | `src/endpoint/polled_audio_driver.rs:195` |
| `drop` | function | Releases resources owned by `SessionTraceRecorder`. | `src/session/lifecycle/trace.rs:249` |
| `drop_observations` | function | Returns the drop observations associated with `SessionRouteMetrics`. | `src/session/lifecycle/observations.rs:206` |
| `drop_rate_pct` | function | Returns the drop rate pct associated with `EdgeObservations`. | `src/runtime/audio/router.rs:165` |
| `drop_rate_pct` | function | Returns the drop rate pct associated with `SessionRouteDropObservations`. | `src/session/lifecycle/observations.rs:171` |
| `edge_contract` | function | Returns the edge contract associated with `ConnectorInputDescriptor`. | `src/connector/worker/driver.rs:52` |
| `edge_contract` | function | Returns the edge contract associated with `EndpointPortInput`. | `src/endpoint/contract.rs:215` |
| `encode` | function | Encodes input through `ConnectorConfigurationRecord`. | `src/connector/transport.rs:61` |
| `encode` | function | Encodes input through `ConnectorAudioRecord`. | `src/connector/transport.rs:347` |
| `encode` | function | Encodes input through `SidecarMessage`. | `src/runtime/lifecycle/sidecar_protocol.rs:86` |
| `endpoint_finalization_failures_total` | function | Returns the endpoint finalization failures total associated with `SessionStopOutcome`. | `src/session/lifecycle/start_contract.rs:334` |
| `endpoint_id` | function | Returns the endpoint identifier associated with `ConnectorInputDescriptor`. | `src/connector/worker/driver.rs:28` |
| `endpoint_id` | function | Returns the endpoint identifier associated with `PolledAudioFrame`. | `src/endpoint/polled_audio_driver.rs:219` |
| `endpoint_id` | function | Returns the endpoint identifier associated with `EndpointPrepareContext`. | `src/endpoint/runtime.rs:129` |
| `endpoint_observations` | function | Returns the endpoint observations associated with `ConnectorContext`. | `src/connector/worker/coordination.rs:139` |
| `error_class` | function | Returns the error class associated with `SessionControlFailure`. | `src/session/lifecycle/events.rs:97` |
| `event_queue` | function | Returns the event queue associated with `SessionMetricsSnapshot`. | `src/session/lifecycle/observations.rs:67` |
| `execution` | function | Returns the execution associated with `SourceManifest`. | `src/session/extensions/source.rs:174` |
| `expose_secret` | function | Exposes the secret to the owning connector during setup or worker use. | `src/connector/configuration.rs:37` |
| `external_source` | function | Returns the external source associated with `SessionMetricsSnapshot`. | `src/session/lifecycle/observations.rs:87` |
| `external_source_count` | function | Returns the external source count associated with `SessionMetricsSnapshot`. | `src/session/lifecycle/observations.rs:83` |
| `failure` | function | Returns the failure associated with `ConnectorRunOutcome`. | `src/connector/worker/mod.rs:55` |
| `failure_threshold` | function | Returns the failure threshold associated with `ConnectorReadinessPolicy`. | `src/connector/readiness.rs:55` |
| `field` | function | Returns the field associated with `ConnectorConfigurationSchema`. | `src/connector/configuration.rs:255` |
| `field` | function | Returns the field associated with `ConnectorConfigurationError`. | `src/connector/configuration.rs:619` |
| `fields` | function | Returns the fields associated with `ConnectorConfigurationSchema`. | `src/connector/configuration.rs:251` |
| `finish` | function | Finishes work owned by `SessionTraceRecorder`. | `src/session/lifecycle/trace.rs:201` |
| `fmt` | function | Formats `ConnectorSecret` with the requested formatter. | `src/connector/configuration.rs:43` |
| `fmt` | function | Formats `ConnectorErrorCode` with the requested formatter. | `src/connector/error.rs:35` |
| `fmt` | function | Formats `ConnectorErrorCode` with the requested formatter. | `src/connector/error.rs:44` |
| `fmt` | function | Formats `OperatorInstanceHandle` with the requested formatter. | `src/session/declaration/draft.rs:770` |
| `fmt` | function | Formats `OperatorInputHandle` with the requested formatter. | `src/session/declaration/draft.rs:780` |
| `fmt` | function | Formats `SourceInstanceHandle` with the requested formatter. | `src/session/declaration/draft.rs:911` |
| `fmt` | function | Formats `SourceOutputHandle` with the requested formatter. | `src/session/declaration/draft.rs:991` |
| `fmt` | function | Formats `DerivedStreamHandle` with the requested formatter. | `src/session/declaration/draft.rs:1114` |
| `fmt` | function | Formats `StemHandle` with the requested formatter. | `src/session/declaration/draft.rs:1125` |
| `fmt` | function | Formats `EndpointConfiguration` with the requested formatter. | `src/session/declaration/endpoint.rs:85` |
| `fmt` | function | Formats `AudioInputBuffer` with the requested formatter. | `src/session/extensions/audio_input/buffer.rs:52` |
| `fmt` | function | Formats `AudioInputWriter` with the requested formatter. | `src/session/extensions/audio_input/buffer.rs:261` |
| `fmt` | function | Formats `AudioInputWriteError` with the requested formatter. | `src/session/extensions/audio_input/buffer.rs:336` |
| `fmt` | function | Formats `AudioInputWriteError` with the requested formatter. | `src/session/extensions/audio_input/buffer.rs:346` |
| `fmt` | function | Formats `AudioInput` with the requested formatter. | `src/session/extensions/audio_input/mod.rs:143` |
| `fmt` | function | Formats `PcmSource` with the requested formatter. | `src/session/extensions/audio_input/source.rs:74` |
| `fmt` | function | Formats `SourceTypeId` with the requested formatter. | `src/session/extensions/source.rs:742` |
| `frame` | function | Returns the frame associated with `PolledAudioBatchLease`. | `src/endpoint/polled_audio_driver.rs:186` |
| `frame_capacity_samples` | function | Returns the frame capacity samples associated with `SessionAudioReentryMetrics`. | `src/session/lifecycle/observations.rs:340` |
| `frame_samples_per_channel` | function | Returns the frame samples per channel associated with `AudioInputConfig`. | `src/session/extensions/audio_input/mod.rs:67` |
| `frames_attempted_total` | function | Returns the frames attempted total associated with `EdgeObservations`. | `src/runtime/audio/router.rs:160` |
| `from` | function | Converts the supplied value into `AudioInputWriteError`. | `src/session/extensions/audio_input/buffer.rs:325` |
| `from_item` | function | Creates `ConnectorAudioRecord` from item. | `src/connector/transport.rs:314` |
| `from_monotonic_timestamp_ns` | function | Creates `SessionTimelineOrigin` from monotonic timestamp nanoseconds. | `src/endpoint/runtime.rs:18` |
| `from_open_observations` | function | Records platform authorization observations without inferring them from a generic backend result. Callers must pass `NotObservable` when their platform has no authoritative query for the requested capture class. | `src/capture/authorization.rs:76` |
| `from_resolved` | function | Creates `ConnectorConfigurationRecord` from resolved. | `src/connector/transport.rs:45` |
| `from_source` | function | Creates `EndpointRouteContext` from source. | `src/endpoint/runtime.rs:57` |
| `from_source_output` | function | Wraps a public external-source output in the same typed Rust façade. Runtime identity remains the output's stable `SignalSpec` and schema. | `src/session/declaration/typed_stream.rs:118` |
| `from_stem` | function | Creates `EndpointRouteContext` from stem. | `src/endpoint/runtime.rs:50` |
| `from_stem` | function | Creates `Stream` from stem. | `src/session/declaration/typed_stream.rs:103` |
| `generation` | function | Returns the implementation generation. | `src/session/extensions/source.rs:166` |
| `get` | function | Returns the value held by `ConnectorConfiguration`. | `src/connector/configuration.rs:134` |
| `get` | function | Returns the value held by `ResolvedConnectorConfiguration`. | `src/connector/configuration.rs:394` |
| `get` | function | Returns the value held by `EndpointConfiguration`. | `src/session/declaration/endpoint.rs:60` |
| `get` | function | Returns the value held by `ProcessId`. | `src/session/declaration/selector.rs:13` |
| `get` | function | Returns the value held by `SourceConfiguration`. | `src/session/extensions/source.rs:100` |
| `handle` | function | Returns the handle associated with `SessionTraceRecorder`. | `src/session/lifecycle/trace.rs:197` |
| `health` | function | Returns the health associated with `ConnectorServiceStatus`. | `src/connector/status.rs:46` |
| `health_reason_code` | function | Returns the health reason code associated with `ConnectorServiceStatus`. | `src/connector/status.rs:58` |
| `id` | function | Returns the id associated with `ConnectorCapability`. | `src/connector/manifest.rs:30` |
| `id` | function | Returns the id associated with `ConnectorRequirement`. | `src/connector/manifest.rs:61` |
| `id` | function | Returns the id associated with `EndpointHandle`. | `src/session/declaration/draft.rs:603` |
| `id` | function | Returns the id associated with `StemHandle`. | `src/session/declaration/draft.rs:795` |
| `id` | function | Returns the id associated with `DeviceSelector`. | `src/session/declaration/selector.rs:117` |
| `id` | function | Returns the id associated with `ConnectionSpec`. | `src/session/declaration/spec.rs:259` |
| `identity_strength` | function | Returns the identity strength associated with `CaptureSource`. | `src/capture/identity.rs:94` |
| `implementation_generation` | function | Monotonic implementation generation for this manifest revision. | `src/session/extensions/source.rs:158` |

## Interpretation

An inventory row establishes that a declaration, test, lifecycle operation, configuration surface, or protocol element exists at the frozen snapshot. Fields shown as unknown remain deliberately unspecified. Consult the native API and error contract before relying on panic behavior, blocking, cancellation, ordering, limits, retry, or recovery.

## Related documentation

- [Glossary](/docs/glossary.md)
- [PocketStation](/README.md)
- [Instrument a Session](/docs/how-to/instrument-session.md)
- [Behavior evidence index](/docs/reference/behavior-evidence.md)
- [Rust API reference](/docs/reference/rust-api.md)
- [Session trace validation fails](/docs/troubleshooting/session-trace.md)
- [Observations and metrics](/docs/concepts/observability.md)
- [Running ownership](/docs/lifecycle/running.md)

## Evidence boundary

This page was verified against Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `src/session/lifecycle/observations.rs:1-636` (`DIRECT`)

A file's presence proves implementation or declaration at this snapshot. It does not by itself prove physical-device qualification, operational performance, retry safety, or behavior outside the recorded test conditions.
