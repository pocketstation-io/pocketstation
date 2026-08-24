# Observation API

<!-- claims: CLM-REF-017-SCOPE-001,CLM-REF-017-TEXT-001,CLM-REF-017-TEXT-002,CLM-REF-017-SOURCE-001 -->

## Scope

- **Observe Session metrics and events.** Read route, source, operator, sidecar, endpoint, drop, latency, queue, and terminal observations.
- **Record and validate Session traces.** Persist lifecycle trace records and validate their structural and terminal consistency.

The scope of **Observation API** ends at the native contracts and executable conditions cited below. Platform qualification, performance, retry, and delivery require their own explicit evidence.

## Reference authority

For **Observation API**, the generated [docs.rs API](https://docs.rs/pocketstation/latest/pocketstation/) provides compiler-rendered signatures and navigation. This repository page adds the frozen evidence identifiers, responsibilities, and cross-boundary interpretation used by the documentation verifier.

## Public surface

| Evidence record | Declaration | Kind | Purpose | Source |
|---|---|---|---|---|
| sym-f490a9028837996edb07 | `INITIAL` | assoc_const | Provides the initial value for `PermissionEpoch`. | `src/capture/authorization.rs:270` |
| sym-b8894596b234e7b11358 | `INITIAL` | assoc_const | Provides the initial value for `SourceGeneration`. | `src/capture/events.rs:15` |
| sym-f5882a0a4b9983601b78 | `Error` | assoc_type | Specifies the error type returned by `SidecarMessageKind` operations. | `src/runtime/lifecycle/sidecar_protocol.rs:23` |
| sym-e70b2bf8528a2a7977f6 | `pocketstation::capture::capture_owner::CAPTURE_MONOTONIC_CLOCK_DOMAIN_ID` | constant | Monotonic timestamp domain used by native capture backends. | `src/capture/capture_owner.rs:20` |
| sym-dcc09b0402dcc4a2b2e8 | `pocketstation::capture::events::MAX_SOURCE_RUNTIME_EVENT_OWNED_BYTES` | constant | Maximum heap storage retained by one queued capture-runtime event. | `src/capture/events.rs:72` |
| sym-70666075f09fa7e8e1a4 | `pocketstation::connector::configuration::MAX_CONNECTOR_CONFIGURATION_FIELDS` | constant | Sets the maximum supported connector configuration fields. | `src/connector/configuration.rs:7` |
| sym-4d9191cba5863424f902 | `pocketstation::connector::configuration::MAX_CONNECTOR_CONFIGURATION_TEXT_BYTES` | constant | Sets the maximum supported connector configuration text bytes. | `src/connector/configuration.rs:8` |
| sym-924e23cbd531280ca555 | `pocketstation::connector::error::MAX_CONNECTOR_ERROR_CODE_BYTES` | constant | Sets the maximum supported connector error code bytes. | `src/connector/error.rs:6` |
| sym-9e444653483fa4f02fa0 | `pocketstation::connector::error::MAX_CONNECTOR_ERROR_MESSAGE_BYTES` | constant | Sets the maximum supported connector error message bytes. | `src/connector/error.rs:7` |
| sym-757c38dcb30074da8581 | `pocketstation::connector::manifest::CONNECTOR_API_REVISION` | constant | Defines connector API revision as `1` for the owning public contract. | `src/connector/manifest.rs:7` |
| sym-f0d13b3a5e8fa03517d4 | `pocketstation::connector::manifest::MAX_CONNECTOR_MANIFEST_ENTRIES` | constant | Sets the maximum supported connector manifest entries. | `src/connector/manifest.rs:8` |
| sym-8c599603d4781320e76d | `pocketstation::connector::manifest::MAX_CONNECTOR_MANIFEST_TEXT_BYTES` | constant | Sets the maximum supported connector manifest text bytes. | `src/connector/manifest.rs:9` |
| sym-5c81b5fa9bc04f842d85 | `pocketstation::connector::readiness::MAX_CONNECTOR_READINESS_THRESHOLD` | constant | Sets the maximum supported connector readiness threshold. | `src/connector/readiness.rs:3` |
| sym-aec7604fe15b924867aa | `pocketstation::connector::readiness::MAX_CONNECTOR_READINESS_TIMEOUT` | constant | Sets the maximum supported connector readiness timeout. | `src/connector/readiness.rs:4` |
| sym-422d9d058d7efb3d7df3 | `pocketstation::connector::sidecar::CONNECTOR_AUDIO_RECORD_SCHEMA` | constant | Defines connector audio record schema as `"urn:pocketstation:connector:audio-record:v1"` for the owning public contract. | `src/connector/sidecar.rs:16` |
| sym-ebefb51273c068a49190 | `pocketstation::connector::sidecar::CONNECTOR_AUDIO_RECORD_SIGNAL_ID` | constant | Defines connector audio record signal identifier as `"io.pocketstation.connector.audio-record.v1"` for the owning public contract. | `src/connector/sidecar.rs:15` |
| sym-49c977d37090f05fdc4f | `pocketstation::connector::transport::CONNECTOR_AUDIO_RECORD_MAJOR` | constant | Defines the major version of connector audio record. | `src/connector/transport.rs:19` |
| sym-8df8d2d405c7a4341c3b | `pocketstation::connector::transport::CONNECTOR_AUDIO_RECORD_MINOR` | constant | Defines the minor version of connector audio record. | `src/connector/transport.rs:20` |
| sym-d5023e4750724b6eb269 | `pocketstation::connector::transport::CONNECTOR_CONFIGURATION_RECORD_MAJOR` | constant | Defines the major version of connector configuration record. | `src/connector/transport.rs:35` |
| sym-3623310c7f84b4a816da | `pocketstation::connector::transport::CONNECTOR_CONFIGURATION_RECORD_MINOR` | constant | Defines the minor version of connector configuration record. | `src/connector/transport.rs:36` |
| sym-8fa3ab71f953b0df2d48 | `pocketstation::connector::transport::MAX_CONNECTOR_AUDIO_RECORD_PORT_BYTES` | constant | Sets the maximum supported connector audio record port bytes. | `src/connector/transport.rs:21` |
| sym-267083f78abe7e48cdc2 | `pocketstation::connector::transport::MAX_CONNECTOR_AUDIO_RECORD_SAMPLES` | constant | Sets the maximum supported connector audio record samples. | `src/connector/transport.rs:22` |
| sym-c6a638806c91124f0676 | `pocketstation::session::declaration::endpoint::BROWSER_NODE_TYPE_ID` | constant | Defines browser node type identifier as `"endpoint.browser.remote"` for the owning public contract. | `src/session/declaration/endpoint.rs:9` |
| sym-fc1ec929667760d593a2 | `pocketstation::session::declaration::endpoint::BROWSER_OPERATOR_ID` | constant | Defines browser operator identifier as `"io.pocketstation.browser.webrtc.v1"` for the owning public contract. | `src/session/declaration/endpoint.rs:10` |
| sym-648808e081cc4f7b4163 | `pocketstation::session::declaration::endpoint::CONNECTOR_NODE_TYPE_ID` | constant | Defines connector node type identifier as `"endpoint.connector.external"` for the owning public contract. | `src/session/declaration/endpoint.rs:8` |
| sym-6015373ecb248baeeb21 | `pocketstation::session::declaration::spec::SESSION_SPEC_VERSION` | constant | Defines the stable session spec version used by the owning public contract. | `src/session/declaration/spec.rs:11` |
| sym-990126c3b36a72a9663d | `pocketstation::session::extensions::audio_input::PCM_SOURCE_TYPE_ID` | constant | Stable runtime identity of the underlying PCM source implementation. | `src/session/extensions/audio_input/mod.rs:19` |
| sym-f6613170e7551288121d | `pocketstation::session::extensions::builtins::APPLICATION_SOURCE_NODE_TYPE_ID` | constant | Defines application source node type identifier as `"source.application"` for the owning public contract. | `src/session/extensions/builtins.rs:23` |
| sym-212f03d762965fddfe4d | `pocketstation::session::extensions::builtins::MICROPHONE_SOURCE_NODE_TYPE_ID` | constant | Defines microphone source node type identifier as `"source.microphone"` for the owning public contract. | `src/session/extensions/builtins.rs:24` |
| sym-e8adce9ed0b589e380eb | `pocketstation::session::extensions::recording::DEFAULT_MULTISTEM_RECORDING_GROUP_ID` | constant | Defines default multistem recording group identifier as `"session.multistem.default.v1"` for the owning public contract. | `src/session/extensions/recording.rs:24` |
| sym-82534ca48ce3c1db5a8f | `pocketstation::session::extensions::recording::RECORDER_NODE_TYPE_ID` | constant | Defines recorder node type identifier as `"endpoint.recording.multistem"` for the owning public contract. | `src/session/extensions/recording.rs:20` |
| sym-b22481c631aa223b9b04 | `pocketstation::session::extensions::recording::RECORDER_OPERATOR_ID` | constant | Defines recorder operator identifier as `"io.pocketstation.recording.wav-stems.v1"` for the owning public contract. | `src/session/extensions/recording.rs:21` |
| sym-9fd7c3b1a6a29ca05799 | `pocketstation::session::extensions::recording::RECORDING_GROUP_CONFIGURATION_KEY` | constant | Defines recording group configuration key as `MULTISTEM_GROUP_CONFIGURATION_KEY` for the owning public contract. | `src/session/extensions/recording.rs:23` |
| sym-da81e7605f4543a374ca | `pocketstation::session::extensions::recording::SESSION_RECORDING_MANIFEST_FILE_NAME` | constant | Defines session recording manifest file name as `crate::recording::RECORDING_MANIFEST_FILE_NAME` for the owning public contract. | `src/session/extensions/recording.rs:25` |
| sym-448f0aef86c8f3825184 | `pocketstation::session::extensions::recording::SESSION_RECORDING_MANIFEST_SCHEMA_VERSION` | constant | Defines session recording manifest schema version as `crate::recording::RECORDING_MANIFEST_SCHEMA_VERSION` for the owning public contract. | `src/session/extensions/recording.rs:27` |
| sym-fcd611770bad7c70afe3 | `pocketstation::capture::authorization::ApplicationPolicyObservation` | enum | Classifies the observable application policy observation. | `src/capture/authorization.rs:231` |
| sym-e47b2044d4d2d7687620 | `pocketstation::capture::authorization::CaptureCapabilityState` | enum | Selects the capture capability state used by PocketStation. | `src/capture/authorization.rs:145` |
| sym-fc12bf3aac349548522a | `pocketstation::capture::authorization::CaptureError` | enum | Classifies failures surfaced by capture operations. | `src/capture/authorization.rs:290` |
| sym-1890ca72d0990a96bb9b | `pocketstation::capture::authorization::CaptureOpenOutcome` | enum | Classifies the observable capture open outcome. | `src/capture/authorization.rs:281` |
| sym-97ee84b50bcbf0f9a179 | `pocketstation::capture::authorization::CaptureScope` | enum | Selects the capture scope used by PocketStation. | `src/capture/authorization.rs:248` |
| sym-5437ce70388c13d381cb | `pocketstation::capture::authorization::CaptureSessionGrant` | enum | Reports the Session-specific authorization available for capture. | `src/capture/authorization.rs:240` |
| sym-aa65d77d9c53f0f321ac | `pocketstation::capture::authorization::PermissionObservation` | enum | Classifies the observable permission observation. | `src/capture/authorization.rs:153` |
| sym-664a9f3276c944fb9071 | `pocketstation::capture::authorization::SourceIdentityStrength` | enum | Classifies how reliably a capture source identity binds to the same resource. | `src/capture/authorization.rs:257` |
| sym-a11764513121614cd19c | `pocketstation::capture::events::CaptureRuntimeFailureClass` | enum | Classifies the platform, permission, source, or worker cause of a capture failure. | `src/capture/events.rs:40` |
| sym-37ae7422bb95d06bd649 | `pocketstation::capture::events::SourceLifecycleEventKind` | enum | Selects the source lifecycle event kind used by PocketStation. | `src/capture/events.rs:25` |
| sym-12f3cc54979ed8d2e6b7 | `pocketstation::capture::events::SourceRecoveryRequirement` | enum | Selects the source recovery requirement used by PocketStation. | `src/capture/events.rs:35` |
| sym-1c3a6ae15cada35f369d | `pocketstation::capture::events::SourceRuntimeEvent` | enum | Classifies the observable source runtime event. | `src/capture/events.rs:53` |
| sym-d5525d995bef9e7d0117 | `pocketstation::capture::events::SourceRuntimeEventDelivery` | enum | Reports whether a source-runtime event was delivered, dropped, or rejected. | `src/capture/events.rs:96` |
| sym-58b4705ef8fa89213d0b | `pocketstation::capture::events::SourceRuntimeEventReceive` | enum | Reports the outcome of receiving a source-runtime event. | `src/capture/events.rs:104` |
| sym-4e7b74129f3699c44fdf | `pocketstation::capture::frame_stream::CapturedFrameDelivery` | enum | Reports whether a captured frame was accepted, dropped, or rejected by delivery. | `src/capture/frame_stream.rs:10` |
| sym-0ac7ee0914d008935e77 | `pocketstation::capture::identity::SourceKind` | enum | Selects the source kind used by PocketStation. | `src/capture/identity.rs:9` |
| sym-113f3cbbd2a52a3a398a | `pocketstation::capture::identity::SourceState` | enum | Selects the source state used by PocketStation. | `src/capture/identity.rs:17` |
| sym-e92f5dbaf28b24d71288 | `pocketstation::capture::lifecycle_registry::SourceGenerationTransition` | enum | Records whether a capture source disappeared, reappeared, or changed generation. | `src/capture/lifecycle_registry.rs:8` |
| sym-e96fae7f5820c2e17de1 | `pocketstation::capture::query::SourceQuery` | enum | Describes the source kind and optional application or device selector used for discovery. | `src/capture/query.rs:13` |
| sym-548c61f75ace7e30e746 | `pocketstation::capture::selection::CaptureMode` | enum | Selects the capture mode used by PocketStation. | `src/capture/selection.rs:16` |
| sym-c18a0760716b2deb267f | `pocketstation::capture::selection::InputDeviceSelector` | enum | Selects either the default input device or one exact device identity. | `src/capture/selection.rs:9` |
| sym-72d1ae94cff22bf30b0f | `pocketstation::capture::selection::ProcessTreeScope` | enum | Selects the process tree scope used by PocketStation. | `src/capture/selection.rs:83` |
| sym-c690fdfd5adff7b4edd2 | `pocketstation::capture::selection::SelectorPersistenceScope` | enum | Selects the selector persistence scope used by PocketStation. | `src/capture/selection.rs:73` |
| sym-f92ee6c928a919df45a7 | `pocketstation::capture::timeline::CaptureSampleTimelineError` | enum | Classifies failures surfaced by capture sample timeline operations. | `src/capture/timeline.rs:41` |
| sym-8dbc9bd88b1575dd438d | `pocketstation::connector::ConnectorDeclarationError` | enum | Classifies failures surfaced by connector declaration operations. | `src/connector/mod.rs:233` |
| sym-a78af2f1f714d50e5fb2 | `pocketstation::connector::ConnectorObservationLookupError` | enum | Classifies failures surfaced by connector observation lookup operations. | `src/connector/mod.rs:246` |
| sym-c609c374fa8e6866eeee | `pocketstation::connector::ConnectorRegistrationError` | enum | Classifies failures produced during connector registration. | `src/connector/mod.rs:225` |
| sym-13750a98063d811df25f | `pocketstation::connector::configuration::ConnectorConfigurationConstraint` | enum | Classifies validation constraints applied to connector configuration fields. | `src/connector/configuration.rs:159` |
| sym-4c8ab28f0ba8c890be55 | `pocketstation::connector::configuration::ConnectorConfigurationErrorCode` | enum | Provides stable categories for connector configuration validation failures. | `src/connector/configuration.rs:568` |
| sym-a40cb5d8365f9ea24105 | `pocketstation::connector::configuration::ConnectorConfigurationRequirement` | enum | Selects the connector configuration requirement used by PocketStation. | `src/connector/configuration.rs:152` |
| sym-6966c55cb774b54182be | `pocketstation::connector::configuration::ConnectorConfigurationValue` | enum | Carries one validated connector configuration value in its declared scalar or secret form. | `src/connector/configuration.rs:66` |
| sym-80830e3822c869f170f3 | `pocketstation::connector::configuration::ConnectorConfigurationValueKind` | enum | Selects the connector configuration value kind used by PocketStation. | `src/connector/configuration.rs:55` |
| sym-936b382b77b874531096 | `pocketstation::connector::error::ConnectorErrorBuildError` | enum | Classifies failures produced during connector error construction and input validation. | `src/connector/error.rs:184` |
| sym-736ab1f377099246ab6c | `pocketstation::connector::error::ConnectorErrorCodeError` | enum | Classifies failures surfaced by connector error code operations. | `src/connector/error.rs:50` |
| sym-a833cb405d6b510ca450 | `pocketstation::connector::error::ConnectorErrorStage` | enum | Selects the connector error stage used by PocketStation. | `src/connector/error.rs:60` |
| sym-47fa722e871287abe9c1 | `pocketstation::connector::error::ConnectorRetryability` | enum | Declares whether a connector failure may be retried under the connector contract. | `src/connector/error.rs:72` |
| sym-eb9a6bafd07d87b6d706 | `pocketstation::connector::manifest::ConnectorManifestError` | enum | Classifies failures surfaced by connector manifest operations. | `src/connector/manifest.rs:231` |
| sym-a62dc6612ed60bf01537 | `pocketstation::connector::observations::ConnectorObservationError` | enum | Classifies failures surfaced by connector observation operations. | `src/connector/observations.rs:175` |
| sym-0c5d86ed5e47145073e6 | `pocketstation::connector::readiness::ConnectorReadinessPolicyError` | enum | Classifies failures surfaced by connector readiness policy operations. | `src/connector/readiness.rs:61` |
| sym-57a90efe5d877dea505e | `pocketstation::connector::status::ConnectorDeliveryReadiness` | enum | Reports whether connector delivery is ready, degraded, or unavailable. | `src/connector/status.rs:4` |
| sym-95843fa35c7fe6258767 | `pocketstation::connector::status::ConnectorHealth` | enum | Reports the current operational health of a connector worker. | `src/connector/status.rs:17` |
| sym-2e6f8e216a5a39ba1c46 | `pocketstation::connector::status::ConnectorRecovery` | enum | Declares the recovery state exposed after a connector failure. | `src/connector/status.rs:24` |
| sym-a8e9c6600f9fddcb4ca9 | `pocketstation::connector::transport::ConnectorAudioRecordError` | enum | Classifies failures surfaced by connector audio record operations. | `src/connector/transport.rs:568` |
| sym-26e401bc8ce6686aafa5 | `pocketstation::connector::transport::ConnectorConfigurationRecordError` | enum | Classifies failures surfaced by connector configuration record operations. | `src/connector/transport.rs:251` |
| sym-c61e48a0b277b7e058f2 | `pocketstation::connector::worker::driver::ConnectorDeliveryOutcome` | enum | Explicit delivery result used for Core-owned accounting. | `src/connector/worker/driver.rs:83` |
| sym-5d275693e8d66effdd4e | `pocketstation::connector::worker::driver::ConnectorItem` | enum | One bounded item delivered by Core to a connector driver. | `src/connector/worker/driver.rs:62` |
| sym-f41b220549324b1bc03c | `pocketstation::endpoint::contract::EndpointReceiver` | enum | Owns the bounded receiver for the media class accepted by an endpoint. | `src/endpoint/contract.rs:175` |
| sym-c2a87abcabc14ba01e3a | `pocketstation::endpoint::identity::EndpointPreparationGroup` | enum | Factory-owned decision for batching endpoint inputs into one lifecycle. | `src/endpoint/identity.rs:23` |
| sym-a89bc612a3b00ae6ae81 | `pocketstation::endpoint::polled_audio_driver::PolledAudioEndpointConfigError` | enum | Classifies failures surfaced by polled audio endpoint config operations. | `src/endpoint/polled_audio_driver.rs:40` |
| sym-573e7386b423f6e2c79a | `pocketstation::endpoint::polled_audio_driver::PolledAudioPollError` | enum | Classifies failures surfaced by polled audio poll operations. | `src/endpoint/polled_audio_driver.rs:74` |
| sym-23d150abd8808ea27637 | `pocketstation::endpoint::registry::EndpointDriverRegistryError` | enum | Classifies failures surfaced by endpoint driver registry operations. | `src/endpoint/registry.rs:16` |
| sym-dcf022bdce68f457ab12 | `pocketstation::endpoint::registry::EndpointPrepareError` | enum | Classifies failures produced during endpoint resource preparation. | `src/endpoint/registry.rs:39` |
| sym-04c73341db961b3a258d | `pocketstation::endpoint::runtime::EndpointFailureRetryability` | enum | Machine-readable recovery classification retained in Session outcomes. | `src/endpoint/runtime.rs:166` |
| sym-2ac4e71ae5a5c7b9e4d6 | `pocketstation::endpoint::runtime::EndpointFailureStage` | enum | Selects the endpoint failure stage used by PocketStation. | `src/endpoint/runtime.rs:156` |
| sym-f431fd7a79ebac5b019c | `pocketstation::endpoint::runtime::EndpointInputOrigin` | enum | Provenance of one endpoint input, independent of its physical receiver. | `src/endpoint/runtime.rs:31` |
| sym-a056d0a949c31542f472 | `pocketstation::endpoint::runtime::EndpointShutdownMode` | enum | Session shutdown intent delivered to an active endpoint. | `src/endpoint/runtime.rs:356` |
| sym-37b46431c1ff4468753a | `pocketstation::endpoint::runtime::EndpointStartFailureCause` | enum | Classifies the lifecycle stage responsible for an endpoint start failure. | `src/endpoint/runtime.rs:438` |
| sym-bd4cddc024bdfa04c794 | `pocketstation::runtime::audio::executor::ExecError` | enum | Classifies failures surfaced by exec operations. | `src/runtime/audio/executor.rs:20` |
| sym-957ab9d17e2a69e24822 | `pocketstation::runtime::audio::router::PlanEdgeFrame` | enum | Carries either one routed frame or a terminal marker through a plan edge. | `src/runtime/audio/router.rs:29` |
| sym-0f3e073889a85f550e76 | `pocketstation::runtime::audio::router::PlanRouterError` | enum | Classifies failures surfaced by plan router operations. | `src/runtime/audio/router.rs:17` |
| sym-8077ac938f560668fc7d | `pocketstation::runtime::audio::runner::PlanRunnerDrainPolicy` | enum | Selects the plan runner drain policy used by PocketStation. | `src/runtime/audio/runner.rs:16` |
| sym-56de2f851fe07b9e1f22 | `pocketstation::runtime::audio::runner::PlanRunnerError` | enum | Classifies failures surfaced by plan runner operations. | `src/runtime/audio/runner.rs:256` |
| sym-635213208855f982fdd1 | `pocketstation::runtime::audio::runner::PlanSourceSendError` | enum | Classifies failures surfaced by plan source send operations. | `src/runtime/audio/runner.rs:116` |
| sym-48efb656489fafc8bcc9 | `pocketstation::runtime::audio::runner::PlanSourceSendOutcome` | enum | Classifies the observable plan source send outcome. | `src/runtime/audio/runner.rs:123` |
| sym-993f5a91b8567d32c584 | `pocketstation::runtime::bridge::audio::GeneratedAudioBridgeStartError` | enum | Classifies failures produced during generated audio bridge lifecycle start. | `src/runtime/bridge/audio.rs:46` |
| sym-71cc7e43e02a571de594 | `pocketstation::runtime::lifecycle::async_host::AsyncRuntimeHostError` | enum | Classifies failures surfaced by async runtime host operations. | `src/runtime/lifecycle/async_host.rs:10` |
| sym-9de7ba3ad4ec47b816ed | `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostError` | enum | Classifies failures surfaced by sidecar host operations. | `src/runtime/lifecycle/sidecar_host.rs:686` |
| sym-92b3ea8c30f09567053c | `pocketstation::runtime::lifecycle::sidecar_host::SidecarState` | enum | Selects the sidecar state used by PocketStation. | `src/runtime/lifecycle/sidecar_host.rs:21` |
| sym-0e7efaeba01d4c9b9389 | `pocketstation::runtime::lifecycle::sidecar_protocol::SidecarMessageKind` | enum | Selects the sidecar message kind used by PocketStation. | `src/runtime/lifecycle/sidecar_protocol.rs:9` |
| sym-87edea404c6ffea054be | `pocketstation::runtime::lifecycle::sidecar_protocol::SidecarProtocolError` | enum | Classifies failures produced during sidecar protocol parsing and state transitions. | `src/runtime/lifecycle/sidecar_protocol.rs:292` |
| sym-aaff16dc9808b1b8fbf9 | `pocketstation::runtime::signal::edge::TypedEdgeBuildError` | enum | Classifies failures produced during typed edge construction and input validation. | `src/runtime/signal/edge.rs:386` |
| sym-dbb372137f3276f38eb7 | `pocketstation::runtime::signal::edge::TypedEdgePublishError` | enum | Classifies failures surfaced by typed edge publish operations. | `src/runtime/signal/edge.rs:408` |
| sym-d8829db8dadb99f939cb | `pocketstation::runtime::signal::error::AsyncOperatorWorkerError` | enum | Classifies failures surfaced by async operator worker operations. | `src/runtime/signal/error.rs:6` |
| sym-b88918b145af6981a661 | `pocketstation::session::compile::error::SessionCompileError` | enum | Classifies failures surfaced by session compile operations. | `src/session/compile/error.rs:7` |
| sym-d83bf3e81e623b11c715 | `pocketstation::session::declaration::selector::ApplicationSelector` | enum | Selects an application by bundle identity, process identity, stable identity, or name. | `src/session/declaration/selector.rs:32` |
| sym-be6e808aed7e0da57649 | `pocketstation::session::declaration::selector::DeviceSelector` | enum | Selects either the host default device or one stable device identity. | `src/session/declaration/selector.rs:107` |
| sym-b64a7cadc7e6f7c5fa13 | `pocketstation::session::declaration::selector::Source` | enum | Declares the application, microphone, or system source selected by a Session. | `src/session/declaration/selector.rs:134` |
| sym-e0648ee37d245fc7fd22 | `pocketstation::session::declaration::spec::ConnectionTarget` | enum | Stable destination of a declared Session connection. | `src/session/declaration/spec.rs:224` |
| sym-b0dbd87bc9581e0b7066 | `pocketstation::session::declaration::spec::StreamOrigin` | enum | Stable origin of a declared Session stream. | `src/session/declaration/spec.rs:208` |
| sym-35a864f7f5b2fbde0056 | `pocketstation::session::declaration::typed_stream::TypedStreamError` | enum | Classifies failures surfaced by typed stream operations. | `src/session/declaration/typed_stream.rs:185` |
| sym-1955bd9759678ed98b23 | `pocketstation::session::error::SessionError` | enum | Classifies failures surfaced by session operations. | `src/session/error.rs:6` |
| sym-4c59be13624496754b3d | `pocketstation::session::error_code::PolledAudioPollErrorCode` | enum | Stable language-neutral code for bounded polled-audio status. | `src/session/error_code.rs:131` |
| sym-43f2c5d509dad2abb71a | `pocketstation::session::error_code::SessionDeclarationErrorCode` | enum | Stable language-neutral code for a Session declaration failure. | `src/session/error_code.rs:10` |
| sym-cde24d045d00d42757d1 | `pocketstation::session::error_code::SessionRuntimeErrorCode` | enum | Stable language-neutral code for a running-Session projection failure. | `src/session/error_code.rs:116` |
| sym-24efd257193601aef470 | `pocketstation::session::error_code::SessionStartErrorCode` | enum | Stable language-neutral code for Session startup. | `src/session/error_code.rs:61` |
| sym-0b02cb5628054570780f | `pocketstation::session::error_code::SessionStopCode` | enum | Stable language-neutral status for an idempotent Session stop. | `src/session/error_code.rs:150` |
| sym-09d3e56e9e242ad53395 | `pocketstation::session::error_code::SessionStopFailureCode` | enum | Stable language-neutral cause retained by a failed Session stop. | `src/session/error_code.rs:171` |
| sym-59571b2eea7cbe8a23d8 | `pocketstation::session::extensions::audio_input::AudioInputConfigError` | enum | Classifies failures surfaced by audio input config operations. | `src/session/extensions/audio_input/mod.rs:77` |
| sym-e62ed15b46d470f5c856 | `pocketstation::session::extensions::audio_input::buffer::AudioInputBufferAcquireError` | enum | Classifies failures surfaced by audio input buffer acquire operations. | `src/session/extensions/audio_input/buffer.rs:271` |
| sym-cfce1a7f00670e05d9d4 | `pocketstation::session::extensions::audio_input::buffer::AudioInputBufferError` | enum | Classifies failures surfaced by audio input buffer operations. | `src/session/extensions/audio_input/buffer.rs:281` |
| sym-5eadd18110842a6de92b | `pocketstation::session::extensions::audio_input::buffer::AudioInputWriteErrorKind` | enum | Selects the audio input write error kind used by PocketStation. | `src/session/extensions/audio_input/buffer.rs:298` |
| sym-fd0b5b1e3d01611befb0 | `pocketstation::session::extensions::audio_input::source::AudioInputError` | enum | Classifies failures surfaced by audio input operations. | `src/session/extensions/audio_input/source.rs:85` |
| sym-6f966dee9d1515e62a7e | `pocketstation::session::extensions::builtins::SessionGraphRegistrationError` | enum | Classifies failures produced during session graph registration. | `src/session/extensions/builtins.rs:30` |
| sym-83234335e8b4cb70f57e | `pocketstation::session::extensions::source::SourceDriverError` | enum | Classifies failures surfaced by source driver operations. | `src/session/extensions/source.rs:748` |
| sym-1843c1ec8aae096e1d65 | `pocketstation::session::extensions::source::SourceManifestError` | enum | Classifies failures surfaced by source manifest operations. | `src/session/extensions/source.rs:677` |
| sym-61d8e211defe79bbb1a6 | `pocketstation::session::extensions::source::SourceRegistrationError` | enum | Classifies failures produced during source registration. | `src/session/extensions/source.rs:701` |
| sym-99bca5d12c942096f8b0 | `pocketstation::session::extensions::source::SourceRuntimeError` | enum | Classifies failures produced during source runtime execution. | `src/session/extensions/source.rs:754` |
| sym-7871af4f560ec9fe3ac4 | `pocketstation::session::extensions::source::SourceTypeIdError` | enum | Classifies failures surfaced by source type identifier operations. | `src/session/extensions/source.rs:68` |
| sym-5ede7fd5e41898b0061b | `pocketstation::session::lifecycle::control::SessionStartError` | enum | Classifies failures produced during session lifecycle start. | `src/session/lifecycle/control.rs:121` |
| sym-3c6c09975ce94069676c | `pocketstation::session::lifecycle::engine::EndpointExtensionRegistrationError` | enum | Classifies failures produced during endpoint extension registration. | `src/session/lifecycle/engine.rs:305` |
| sym-c641398d8da3cb94c250 | `pocketstation::session::lifecycle::engine::SessionEngineBuildError` | enum | Classifies failures produced during session engine construction and input validation. | `src/session/lifecycle/engine.rs:295` |
| sym-c5305e4d076e18066988 | `pocketstation::session::lifecycle::engine::SessionEngineStartError` | enum | Classifies failures produced during session engine lifecycle start. | `src/session/lifecycle/engine.rs:315` |
| sym-8be5244e9ac8689c706c | `pocketstation::session::lifecycle::events::SessionComponentId` | enum | Stable identity of the component that produced a session control failure. | `src/session/lifecycle/events.rs:51` |
| sym-62d3d25fa4776760b06b | `pocketstation::session::lifecycle::events::SessionEventKind` | enum | Payload of one authoritative session event. | `src/session/lifecycle/events.rs:294` |
| sym-c6d521bc369615540a31 | `pocketstation::session::lifecycle::events::SessionEventReceive` | enum | Result of non-blocking event polling. | `src/session/lifecycle/events.rs:492` |
| sym-758d13d75953fa66b79a | `pocketstation::session::lifecycle::events::SessionFinalizationStage` | enum | The finalization operation that failed while stopping a session. | `src/session/lifecycle/events.rs:39` |
| sym-962361a1358c981f6a9f | `pocketstation::session::lifecycle::events::SessionLifecycleState` | enum | Public lifecycle states emitted by a running session. | `src/session/lifecycle/events.rs:19` |
| sym-ac79480f787ee81818a0 | `pocketstation::session::lifecycle::events::SessionRollbackStage` | enum | The rollback operation that failed while unwinding a partial start. | `src/session/lifecycle/events.rs:29` |
| sym-a2d10031270815731e7b | `pocketstation::session::lifecycle::events::SessionTerminalState` | enum | Final state carried by the terminal session event. | `src/session/lifecycle/events.rs:210` |
| sym-da46a154618824a18994 | `pocketstation::session::lifecycle::host::SessionEngineHostBuildError` | enum | Classifies failures produced during session engine host construction and input validation. | `src/session/lifecycle/host.rs:362` |
| sym-933733f669d7719ffe7a | `pocketstation::session::lifecycle::observations::EndpointObservationStage` | enum | Selects the endpoint observation stage used by PocketStation. | `src/session/lifecycle/observations.rs:441` |
| sym-5cc026a676e0591b94af | `pocketstation::session::lifecycle::observations::SessionRouteLatencyBoundary` | enum | Identifies the route boundary at which Session latency was observed. | `src/session/lifecycle/observations.rs:196` |
| sym-a758550b72412787e99d | `pocketstation::session::lifecycle::observations::SessionRouteLatencyUnit` | enum | Declares the unit used by a Session route-latency observation. | `src/session/lifecycle/observations.rs:201` |
| sym-08c924c4a96c2ff4640f | `pocketstation::session::lifecycle::observations::SessionRouteObservationInterval` | enum | Interval covered by monotonic route counters. | `src/session/lifecycle/observations.rs:150` |
| sym-f9e0e26f14bb954cf294 | `pocketstation::session::lifecycle::trace::SessionTraceRecordKind` | enum | Selects the session trace record kind used by PocketStation. | `src/session/lifecycle/trace.rs:27` |
| sym-88a327bb91bf885ce10b | `pocketstation::session::lifecycle::trace::SessionTraceRecorderFinishError` | enum | Classifies failures surfaced by session trace recorder finish operations. | `src/session/lifecycle/trace.rs:98` |
| sym-edc7211e5a9e61bcb2ea | `pocketstation::session::lifecycle::trace::SessionTraceRecorderStartError` | enum | Classifies failures produced during session trace recorder lifecycle start. | `src/session/lifecycle/trace.rs:88` |
| sym-68ce5d3bae413e0f5470 | `pocketstation::session::lifecycle::trace::SessionTraceValidationError` | enum | Classifies failures produced during session trace validation. | `src/session/lifecycle/trace.rs:356` |
| sym-796bbabbf84e0f47b0d8 | `pocketstation::session::prepare::error::SessionPrepareError` | enum | Classifies failures produced during session resource preparation. | `src/session/prepare/error.rs:9` |
| sym-8c1e01f6032ee9f58e34 | `ActiveCaptureBackend::observation_handle` | function | Returns a handle for reading observations from `ActiveCaptureBackend`. | `src/capture/capture_owner.rs:107` |
| sym-181be60c05b799d9aa88 | `ActiveCaptureBackend::observations` | function | Returns the observations exposed by `ActiveCaptureBackend`. | `src/capture/capture_owner.rs:109` |
| sym-bb192a71775dad273804 | `ActiveCaptureBackend::source_id` | function | Resolved native source identity for every frame emitted by this open. | `src/capture/capture_owner.rs:105` |
| sym-d5183a64d1a21d804af5 | `ActiveCaptureBackend::stop_and_join` | function | Stops `ActiveCaptureBackend`, joins its worker, and returns the terminal result. | `src/capture/capture_owner.rs:111` |
| sym-ce7aba33b4143704632a | `CallbackCaptureBackend::prepare` | function | Prepares resources required by `CallbackCaptureBackend`. | `src/capture/capture_owner.rs:84` |
| sym-116294235e95d6969503 | `ConnectorDriver::cancel_preparation` | function | Cancels resources created while preparing `ConnectorDriver`. | `src/connector/worker/driver.rs:116` |
| sym-c3743aa2faf26b1ae2ba | `ConnectorDriver::deliver` | function | Delivers the next input through `ConnectorDriver`. | `src/connector/worker/driver.rs:98` |
| sym-a015cd11b00099f1959b | `ConnectorDriver::idle` | function | Advances `ConnectorDriver` while no input is available. | `src/connector/worker/driver.rs:104` |
| sym-be0022e2a56f799fb3e7 | `ConnectorDriver::shutdown` | function | Shuts down `ConnectorDriver` according to its lifecycle contract. | `src/connector/worker/driver.rs:108` |
| sym-72f758a85e8e18ea54e5 | `ConnectorDriver::start` | function | Starts the lifecycle represented by `ConnectorDriver`. | `src/connector/worker/driver.rs:93` |
| sym-e79e0f39714d48b7dde0 | `ConnectorDriverFactory::preparation_group` | function | Returns the preparation group associated with `ConnectorDriverFactory`. | `src/connector/worker/driver.rs:124` |
| sym-f83e889dd44a61772bbe | `ConnectorDriverFactory::prepare` | function | Prepares resources required by `ConnectorDriverFactory`. | `src/connector/worker/driver.rs:132` |
| sym-4825cbede5c6ebfaec8f | `ConnectorFactory::preparation_group` | function | Returns the preparation group associated with `ConnectorFactory`. | `src/connector/worker/mod.rs:18` |
| sym-6015b5e7fee5a0b0c418 | `ConnectorFactory::prepare` | function | Prepares resources required by `ConnectorFactory`. | `src/connector/worker/mod.rs:26` |
| sym-832954574d8962ffafee | `ConnectorWorker::cancel_preparation` | function | Cancels resources created while preparing `ConnectorWorker`. | `src/connector/worker/mod.rs:35` |
| sym-b9c16a875bf523033d20 | `ConnectorWorker::run` | function | Runs `ConnectorWorker` until completion or cancellation. | `src/connector/worker/mod.rs:33` |
| sym-4b0ef43987b96b6f3540 | `EndpointDriverFactory::preparation_group` | function | Returns the preparation group associated with `EndpointDriverFactory`. | `src/endpoint/contract.rs:263` |
| sym-f529d30f10097964651a | `EndpointDriverFactory::prepare` | function | Prepares resources required by `EndpointDriverFactory`. | `src/endpoint/contract.rs:271` |
| sym-1076d488d679ca8ce004 | `PreparedCaptureBackend::open` | function | Opens the resource represented by `PreparedCaptureBackend`. | `src/capture/capture_owner.rs:89` |
| sym-78c0c0868e0e6890db43 | `PreparedEndpointDriver::cancel_preparation` | function | Cancels resources created while preparing `PreparedEndpointDriver`. | `src/endpoint/runtime.rs:328` |
| sym-4616280bf6b55d26508d | `PreparedEndpointDriver::start` | function | Makes endpoint resources ready behind the supplied closed gate. | `src/endpoint/runtime.rs:323` |
| sym-10bdf73b1868933ea548 | `RunningEndpointDriver::join_and_finalize` | function | Joins and finalize for `RunningEndpointDriver`. | `src/endpoint/runtime.rs:346` |
| sym-cd04e67c105bddcbcec6 | `RunningEndpointDriver::observations` | function | Returns the observations exposed by `RunningEndpointDriver`. | `src/endpoint/runtime.rs:337` |
| sym-ad8b16eca0934003b206 | `RunningEndpointDriver::request_shutdown` | function | Requests the selected shutdown mode from `RunningEndpointDriver`. | `src/endpoint/runtime.rs:341` |
| sym-6815ea1b0e23a3e58e2f | `RunningEndpointDriver::request_stop` | function | Requests a graceful stop from `RunningEndpointDriver`. | `src/endpoint/runtime.rs:339` |
| sym-995aa9622b5ba2b4a748 | `SourceDriver::close` | function | Closes `SourceDriver` to further work. | `src/session/extensions/source.rs:273` |
| sym-df9cd8b6772b89c63bda | `SourceDriver::next` | function | Produces the next source emission from `SourceDriver`. | `src/session/extensions/source.rs:269` |
| sym-56f0802ba994d7ac9843 | `SourceDriver::prepare` | function | Prepares resources required by `SourceDriver`. | `src/session/extensions/source.rs:268` |
| sym-93000b7d23bf0f46007d | `SourceFactory::create` | function | Creates the runtime implementation described by `SourceFactory`. | `src/session/extensions/source.rs:279` |
| sym-c1d7fb39869e2d34f307 | `SourceFactory::manifest` | function | Returns the manifest held by `SourceFactory`. | `src/session/extensions/source.rs:277` |
| sym-6a881085d8de6dbbb9e6 | `SourceFactory::validate_config` | function | Validates supplied node configuration against the schema declared by `SourceFactory`. | `src/session/extensions/source.rs:278` |
| sym-7370e5df776e21bbd7b7 | `StreamSignal::signal_spec` | function | Returns the signal spec held by `StreamSignal`. | `src/session/declaration/typed_stream.rs:16` |
| sym-42a8e5c4d0cbc083c502 | `accepts_delivery` | function | Reports whether accepts delivery is true for `ConnectorDeliveryReadiness`. | `src/connector/status.rs:10` |
| sym-8d570ccfcc6f8f22adb8 | `accepts_delivery` | function | Reports whether accepts delivery is true for `ConnectorServiceStatus`. | `src/connector/status.rs:74` |
| sym-dc0c82c0beb1a6eb87e9 | `actual` | function | Returns the observed value when a compilation diagnostic compares two values. | `src/session/compile/error.rs:153` |
| sym-3e242114552b00229671 | `advance` | function | Returns this buffer's source-time start and advances the next start. | `src/capture/timeline.rs:74` |
| sym-27a1b14acbb0dfc6e31c | `advance_from_source_position` | function | Returns a buffer's source-time start from its native sample-frame position. Forward gaps are preserved in the returned timestamp without separately advancing this clock from an aggregate drop counter. | `src/capture/timeline.rs:90` |
| sym-71d554d30a07cbb7cc77 | `after_failed_open` | function | Records a backend open failure without guessing that permission was denied. | `src/capture/authorization.rs:45` |
| sym-9db2a2c278a3325269ca | `after_successful_open` | function | Records the evidence available after an explicitly selected source opens. | `src/capture/authorization.rs:31` |
| sym-0d9c647ddb36695c0df0 | `anchored` | function | Creates a sample timeline whose first buffer starts at the supplied nonzero monotonic timestamp. | `src/capture/timeline.rs:62` |
| sym-e9973a9598777cbaefb0 | `api_revision` | function | Returns the API revision held by `ConnectorManifest`. | `src/connector/manifest.rs:124` |
| sym-1682a40407799112e8f3 | `application` | function | Returns the application held by `Source`. | `src/session/declaration/selector.rs:140` |
| sym-ecb602e99c0dba24af4f | `as_str` | function | Returns the stable string representation of `ConnectorConfigurationErrorCode`. | `src/connector/configuration.rs:585` |
| sym-f5caa543ea9c940cafb1 | `as_str` | function | Returns the stable string representation of `ConnectorErrorCode`. | `src/connector/error.rs:29` |
| sym-6acbd29b6724de4b584f | `as_str` | function | Returns the stable string representation of `EndpointGroupId`. | `src/endpoint/identity.rs:16` |
| sym-11d0c6769d2d0c38fd3d | `as_str` | function | Returns the stable string representation of `DeviceId`. | `src/session/declaration/selector.rs:26` |
| sym-642103f95789f4c34de4 | `as_str` | function | Returns the stable string representation of `SessionDeclarationErrorCode`. | `src/session/error_code.rs:31` |
| sym-098447029f757e3ee3fc | `as_str` | function | Returns the stable string representation of `SessionStartErrorCode`. | `src/session/error_code.rs:86` |
| sym-7496f586df598ab4c92a | `as_str` | function | Returns the stable string representation of `SessionRuntimeErrorCode`. | `src/session/error_code.rs:121` |
| sym-ef6574dbc99fcb40c5a2 | `as_str` | function | Returns the stable string representation of `PolledAudioPollErrorCode`. | `src/session/error_code.rs:138` |
| sym-5643015174c990de4aee | `as_str` | function | Returns the stable string representation of `SessionStopCode`. | `src/session/error_code.rs:157` |
| sym-e3f1223778a01130e663 | `as_str` | function | Returns the stable string representation of `SessionStopFailureCode`. | `src/session/error_code.rs:182` |
| sym-d23b80c999773411aaa9 | `as_str` | function | Returns the stable string representation of `SourceTypeId`. | `src/session/extensions/source.rs:52` |
| sym-eac6fe4c4ec72aa2b314 | `audio_frames_enqueued_total` | function | Returns the audio frames enqueued total held by `SessionAudioReentryMetrics`. | `src/session/lifecycle/observations.rs:368` |
| sym-b06ab36e3211268290ef | `audio_reentry_metrics` | function | Returns the audio reentry metrics held by `RunningSession`. | `src/session/lifecycle/running.rs:223` |
| sym-3545be6a5b39ea259c4d | `audio_stem_id` | function | Returns the audio stem identifier held by `EndpointRouteContext`. | `src/endpoint/runtime.rs:88` |
| sym-4d439fa19abd2ab66f61 | `before_open` | function | Records a source that was resolved but not opened because another required source failed first. | `src/capture/authorization.rs:60` |
| sym-9527dd3130d771c3266b | `browser` | function | Returns the browser associated with `Session`. | `src/session/declaration/draft.rs:431` |
| sym-596d144d9deba0aa781a | `build` | function | Consumes all setup state so no partially populated registry can escape. | `src/session/lifecycle/engine.rs:176` |
| sym-57541748485f96465396 | `build` | function | Builds its owned operation for `SessionEngineHostBuilder`. | `src/session/lifecycle/host.rs:344` |
| sym-f3af0a674083c042387f | `bundle_id` | function | Returns the bundle identifier held by `ApplicationSelector`. | `src/session/declaration/selector.rs:44` |
| sym-218585d5338b32dc0e4d | `cancel` | function | Requests cancellation of `SourceRuntime`. | `src/session/extensions/source.rs:575` |
| sym-cfafc3f08dfc105ee2ca | `cancel` | function | Requests cancellation of `RunningSession`. | `src/session/lifecycle/running.rs:423` |
| sym-f638ac422d0b9145e66e | `cancel_and_join` | function | Cancels and join for `GeneratedAudioBridge`. | `src/runtime/bridge/audio.rs:184` |
| sym-009871c0c84118fce16e | `cancel_and_join` | function | Cancels and join for `AsyncOperatorWorker`. | `src/runtime/signal/operator.rs:925` |
| sym-dd1373cfb5ce025352ae | `cancel_and_reap` | function | Cancels and reap for `SidecarHost`. | `src/runtime/lifecycle/sidecar_host.rs:322` |
| sym-bf0f618c9ae56c316380 | `cancellation_requested` | function | Returns whether cancellation requested is true for `PreparedSession`. | `src/session/prepare/prepared.rs:73` |
| sym-c00eb2e5f0f365125ff1 | `cancellation_total` | function | Returns the cancellation total held by `SessionAudioReentryMetrics`. | `src/session/lifecycle/observations.rs:372` |
| sym-4a32dce08606102cb841 | `capabilities` | function | Returns the capabilities held by `ConnectorManifest`. | `src/connector/manifest.rs:152` |
| sym-2ae2a0a1e4991fd03efd | `capacity_frames` | function | Returns the capacity frames held by `CapturedFrameStream`. | `src/capture/frame_stream.rs:165` |
| sym-651d787afb9425aca9fc | `capacity_frames` | function | Returns the capacity frames held by `AudioInputConfig`. | `src/session/extensions/audio_input/mod.rs:63` |
| sym-468dcc7528c63336128c | `capture` | function | Declares a capture source on `Session` and returns its Session-scoped handle. | `src/session/declaration/draft.rs:335` |
| sym-c4062e05a64ec2cd4952 | `capture_finalization_failures_total` | function | Returns the capture finalization failures total held by `SessionStopOutcome`. | `src/session/lifecycle/control.rs:359` |
| sym-9828266fb021eb2aa01c | `capture_mode` | function | Returns the capture mode held by `SystemLoopbackSource`. | `src/capture/platform/macos/loopback.rs:57` |
| sym-d6d0625df36b60fc05ad | `capture_mode` | function | Returns the capture mode held by `DesktopCaptureSource`. | `src/capture/platform/macos/mod.rs:44` |
| sym-16a3ef48594b21bc3b2b | `channels` | function | Returns the channel count represented by `EndpointAudioFrame`. | `src/endpoint/contract.rs:61` |
| sym-4cbe7948fb0b2aa1dbb8 | `channels` | function | Returns the channel count represented by `PolledAudioFrame`. | `src/endpoint/polled_audio_driver.rs:301` |
| sym-ebd8e9d86b76a24a1263 | `channels` | function | Returns the channel count represented by `PlanEdgeFrame`. | `src/runtime/audio/router.rs:70` |
| sym-ba5d7f524eb5ffcfd856 | `clipped_samples` | function | Returns the clipped samples held by `MixerTelemetry`. | `src/runtime/nodes.rs:260` |
| sym-3aeff457f579705093f6 | `close` | function | Closes `AudioInputWriter` to further work. | `src/session/extensions/audio_input/buffer.rs:242` |
| sym-217be49e77da4aabf632 | `close` | function | Closes this application-owned input after its accepted frames drain. | `src/session/extensions/audio_input/mod.rs:127` |
| sym-6b2818ee70b8000ad99d | `close_and_reap` | function | Closes `SidecarHost` and reaps its child process. | `src/runtime/lifecycle/sidecar_host.rs:326` |
| sym-ac60f8c24954ba5af0f0 | `code` | function | Returns the stable error or status code represented by `ConnectorConfigurationError`. | `src/connector/configuration.rs:615` |
| sym-1b9e9ce53fdd2cf3d6a2 | `code` | function | Returns the stable error or status code represented by `ConnectorError`. | `src/connector/error.rs:109` |
| sym-f5cae830f98ad619334b | `code` | function | Returns the stable error or status code represented by `EndpointFailure`. | `src/endpoint/runtime.rs:212` |
| sym-7f946d73a01b4db9ecc6 | `code` | function | Returns the stable error or status code represented by `SessionCompileDiagnostic`. | `src/session/compile/error.rs:113` |
| sym-68093a3f20079dee1c25 | `compile` | function | Compiles its owned operation for `SessionCompiler`. | `src/session/compile/mod.rs:103` |
| sym-46d9ba85d0b5a90bb8da | `compile` | function | Compiles its owned operation for `SessionEngine`. | `src/session/lifecycle/engine.rs:221` |
| sym-ee91490be1c23a752cf2 | `compile` | function | Compiles its owned operation for `SessionEngineHost`. | `src/session/lifecycle/host.rs:55` |
| sym-e03a7b814ae052667ca9 | `component` | function | Returns the component associated with `SessionControlFailure`. | `src/session/lifecycle/events.rs:89` |
| sym-1e5612a33c09f36fa151 | `configuration` | function | Returns the configuration held by `ConnectorManifest`. | `src/connector/manifest.rs:144` |
| sym-53268b83a683ae6a2898 | `configuration` | function | Returns the configuration held by `ConnectorConfigurationRecord`. | `src/connector/transport.rs:53` |
| sym-ea2aa97e4f6c327d4cc6 | `configuration` | function | Returns the configuration held by `ConnectorInputDescriptor`. | `src/connector/worker/driver.rs:56` |
| sym-3c7d5cd5a6971e320608 | `configuration` | function | Returns the configuration held by `Operator`. | `src/session/declaration/draft.rs:299` |
| sym-83fc2990431dc05c1b6e | `configuration` | function | Returns the configuration held by `EndpointDescriptor`. | `src/session/declaration/endpoint.rs:149` |
| sym-a7de949037767e556526 | `configuration` | function | Returns the configuration held by `SourceInstanceSpec`. | `src/session/declaration/spec.rs:88` |
| sym-df71b11f0bf7bd47f91e | `configuration` | function | Returns the configuration held by `EndpointSpec`. | `src/session/declaration/spec.rs:187` |
| sym-0dba4ffdc9d828c28bec | `configuration` | function | Returns the configuration held by `OperatorInstanceSpec`. | `src/session/declaration/spec.rs:253` |
| sym-d7ab9dfb9f46b66659ea | `configuration` | function | Returns the configuration held by `AudioInputWriter`. | `src/session/extensions/audio_input/buffer.rs:103` |
| sym-1c4c7cb9bedad265f145 | `connect` | function | Connects the requested ports through `StemHandle`. | `src/session/declaration/draft.rs:807` |
| sym-8f3431c4252ede73ca6c | `connect` | function | Connects the requested ports through `SourceOutputHandle`. | `src/session/declaration/draft.rs:943` |
| sym-2f51546b7bed5c1ee4c6 | `connect` | function | Connects the requested ports through `DerivedStreamHandle`. | `src/session/declaration/draft.rs:1039` |
| sym-286cb5670f377ab8ef14 | `connections` | function | Returns the connections associated with `SessionSpec`. | `src/session/declaration/spec.rs:347` |
| sym-e8963ce61c3155e55b92 | `connector` | function | Declares a connector endpoint on `Session` with the supplied operator identity and configuration. | `src/session/declaration/draft.rs:399` |
| sym-5da32ec8a46f62e624fc | `connector_id` | function | Returns the connector identifier held by `ConnectorInputDescriptor`. | `src/connector/worker/driver.rs:32` |
| sym-d97706ec8bcbc92af245 | `connector_id` | function | Returns the connector identifier held by `PolledAudioFrame`. | `src/endpoint/polled_audio_driver.rs:293` |
| sym-a7836b3612121275e6ce | `connector_id` | function | Returns the connector identifier held by `EndpointPrepareContext`. | `src/endpoint/runtime.rs:138` |
| sym-7032830c11b003b8c721 | `connector_id` | function | Returns the connector identifier held by `EndpointHandle`. | `src/session/declaration/draft.rs:595` |
| sym-73f349fcfa5dcd416141 | `connector_id` | function | Returns the connector identifier held by `EndpointSpec`. | `src/session/declaration/spec.rs:175` |
| sym-98c5d4b70984a08dd397 | `constraints` | function | Returns the constraints held by `ConnectorConfigurationField`. | `src/connector/configuration.rs:222` |
| sym-3a9c0d6cde74bac8980a | `context` | function | Returns the context held by `EndpointPortInput`. | `src/endpoint/contract.rs:249` |
| sym-84dfe9847cb3775c63d5 | `current` | function | Returns the current value observed by `CapturePermissionLifecycle`. | `src/capture/authorization.rs:196` |
| sym-635f185607de8e4f9da1 | `declare` | function | Adds the declaration represented by `RegisteredConnector` to its Session. | `src/connector/mod.rs:159` |
| sym-06cbcdb626b9a2e220a9 | `declares_multistem_recording` | function | Returns whether `Session` declares multistem recording. | `src/session/extensions/recording.rs:102` |
| sym-e3ef3f2426b2ab2e710c | `decode` | function | Decodes input through `ConnectorConfigurationRecord`. | `src/connector/transport.rs:93` |
| sym-1a5304483b6adaa06f83 | `decode` | function | Decodes input through `ConnectorAudioRecord`. | `src/connector/transport.rs:403` |
| sym-7fa68b299cc6aee2a4f2 | `decode` | function | Decodes input through `SidecarMessage`. | `src/runtime/lifecycle/sidecar_protocol.rs:121` |
| sym-567daa26a7dd7f60fe2f | `default` | function | Returns the default `PolledAudioEndpointConfig` value. | `src/endpoint/polled_audio_driver.rs:30` |
| sym-8b2c4ad5fa0a4ee19c40 | `default` | function | Returns the default `PlanRunnerCancellation` value. | `src/runtime/audio/runner.rs:110` |
| sym-73f57e4942d4df802c11 | `default` | function | Returns the default `SidecarDeadlines` value. | `src/runtime/lifecycle/sidecar_host.rs:61` |
| sym-f2a677038880c785311e | `default` | function | Returns the default `SidecarProtocolLimits` value. | `src/runtime/lifecycle/sidecar_protocol.rs:51` |
| sym-1c5179c9d400732c9b0c | `default` | function | Returns the default `Session` value. | `src/session/declaration/draft.rs:565` |
| sym-0fdf7c5a3b73738000b1 | `default` | function | Returns the default `DeviceSelector` value. | `src/session/declaration/selector.rs:113` |
| sym-e1902db1f798ce94cdea | `default` | function | Returns the default `SessionStartOptions` value. | `src/session/lifecycle/control.rs:33` |
| sym-8684f51f8ee7430fcd49 | `default` | function | Returns the default `NativeSessionEngineHostOptions` value. | `src/session/lifecycle/host.rs:172` |
| sym-6c2f846a56c09b494d97 | `delivery_readiness` | function | Returns the delivery readiness associated with `ConnectorServiceStatus`. | `src/connector/status.rs:42` |
| sym-ad4e94586f88eebd3375 | `deprecated` | function | Returns the deprecated held by `ConnectorConfigurationField`. | `src/connector/configuration.rs:201` |
| sym-6b003ff3808eecc732f0 | `deprecation` | function | Returns the deprecation held by `ConnectorConfigurationField`. | `src/connector/configuration.rs:226` |
| sym-7d47561aba01141608ff | `derived_route` | function | Returns the derived route associated with `SessionMetricsSnapshot`. | `src/session/lifecycle/observations.rs:111` |
| sym-fb88e790fd387a7ae359 | `derived_route_count` | function | Returns the derived route count held by `SessionMetricsSnapshot`. | `src/session/lifecycle/observations.rs:107` |
| sym-143bf8e7f69e1c868835 | `derived_route_metrics` | function | Returns the derived route metrics held by `RunningSession`. | `src/session/lifecycle/running.rs:219` |
| sym-848742f534a9b91337d0 | `descriptor` | function | Returns the descriptor associated with `SystemOutputSourceFactory`. | `src/runtime/nodes.rs:87` |
| sym-b933cf7614529f0028d2 | `descriptor` | function | Returns the descriptor associated with `BridgeSinkFactory`. | `src/runtime/nodes.rs:191` |
| sym-05eefad45f0dfa93d416 | `diagnostic` | function | Converts a Session compiler failure into stable language-neutral location and comparison fields. | `src/session/compile/error.rs:166` |
| sym-c56c74ef48d3d65a7896 | `direction` | function | Returns the direction associated with `SessionCompileDiagnostic`. | `src/session/compile/error.rs:145` |
| sym-6dbe1ce0ed2ff967ed67 | `discover` | function | Discovers the resources visible to `LocalSourceProvider`. | `src/capture/query.rs:55` |
| sym-eb1aeb591e12bc6b9e4e | `dispatch_from` | function | Routes one lineaged audio frame from the named plan output through `PlanEdgeRouter`. | `src/runtime/audio/router.rs:777` |
| sym-3248ef146d7f14d09bc6 | `documentation` | function | Returns the documentation held by `ConnectorConfigurationField`. | `src/connector/configuration.rs:218` |
| sym-91cdc75380f1b1c45072 | `documentation` | function | Returns the documentation held by `ConnectorCapability`. | `src/connector/manifest.rs:34` |
| sym-34ddc898cf906f7e707f | `documentation` | function | Returns the documentation held by `ConnectorRequirement`. | `src/connector/manifest.rs:69` |
| sym-b839c4c5e4b32bc03a85 | `drop` | function | Releases resources owned by `MacosInputSource`. | `src/capture/platform/macos/input.rs:258` |
| sym-3f4c295f1659985a9a22 | `drop` | function | Releases resources owned by `SystemLoopbackSource`. | `src/capture/platform/macos/loopback.rs:299` |
| sym-24451f44bd94778b8d32 | `drop` | function | Releases resources owned by `ConnectorSecret`. | `src/connector/configuration.rs:49` |
| sym-801da87612861d0e8d9a | `drop` | function | Releases resources owned by `PolledAudioBatchLease`. | `src/endpoint/polled_audio_driver.rs:241` |
| sym-1d8fe4e68f55934db10b | `drop` | function | Releases resources owned by `PlanEdgeReceiver`. | `src/runtime/audio/router.rs:676` |
| sym-2958e60b6265b5fef03f | `drop` | function | Releases resources owned by `GeneratedAudioBridge`. | `src/runtime/bridge/audio.rs:193` |

## Interpretation

The **Observation API** inventory records compiler-visible or extracted evidence at the frozen snapshot. A field marked unknown or not declared remains outside the published guarantee; use the native signature, owning error type, and cited test before relying on panic, blocking, cancellation, ordering, limits, retry, or recovery behavior.

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

The claims on **Observation API** are anchored to Git snapshot `136e74888962558aa846d3143a19136a70936f45` and these primary files:

- `src/session/lifecycle/observations.rs:16-16` (`DIRECT`)
- `src/session/lifecycle/observations.rs:16-16` (`DIRECT`)
- `src/session/lifecycle/observations.rs:16-16` (`DIRECT`)
- `src/session/lifecycle/observations.rs:17-29` (`DIRECT`)
- `src/session/lifecycle/observations.rs:18-18` (`DIRECT`)
- `src/session/lifecycle/observations.rs:19-19` (`DIRECT`)
- `src/session/lifecycle/observations.rs:20-20` (`DIRECT`)
- `src/session/lifecycle/observations.rs:21-21` (`DIRECT`)
- `src/session/lifecycle/observations.rs:22-22` (`DIRECT`)
- `src/session/lifecycle/observations.rs:23-23` (`DIRECT`)
- `src/session/lifecycle/observations.rs:24-24` (`DIRECT`)
- `src/session/lifecycle/observations.rs:25-25` (`DIRECT`)
- `src/session/lifecycle/observations.rs:26-26` (`DIRECT`)
- `src/session/lifecycle/observations.rs:27-27` (`DIRECT`)
- `src/session/lifecycle/observations.rs:28-28` (`DIRECT`)
- `src/session/lifecycle/observations.rs:35-35` (`DIRECT`)
- `src/session/lifecycle/observations.rs:35-35` (`DIRECT`)
- `src/session/lifecycle/observations.rs:35-35` (`DIRECT`)
- `src/session/lifecycle/observations.rs:36-44` (`DIRECT`)
- `src/session/lifecycle/observations.rs:37-37` (`DIRECT`)
- `src/session/lifecycle/observations.rs:38-38` (`DIRECT`)
- `src/session/lifecycle/observations.rs:39-39` (`DIRECT`)
- `src/session/lifecycle/observations.rs:40-40` (`DIRECT`)
- `src/session/lifecycle/observations.rs:41-41` (`DIRECT`)

For **Observation API**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.
