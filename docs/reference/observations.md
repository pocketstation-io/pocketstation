# Observation API

<!-- claims: CLM-REF-017-CAP-001,CLM-REF-017-CAP-002,CLM-REF-017-SOURCE-001 -->

## Scope

- **Observe Session metrics and events.** Read route, source, operator, sidecar, endpoint, drop, latency, queue, and terminal observations.
- **Record and validate Session traces.** Persist lifecycle trace records and validate their structural and terminal consistency.

The scope of **Observation API** ends at the native contracts and executable conditions cited below. Platform qualification, performance, retry, and delivery require their own explicit evidence.

## Reference authority

For **Observation API**, the generated [docs.rs API](https://docs.rs/pocketstation/latest/pocketstation/) provides compiler-rendered signatures and navigation. This repository page adds the frozen evidence identifiers, responsibilities, and cross-boundary interpretation used by the documentation verifier.

## Public surface

| Evidence record | Declaration | Kind | Purpose | Source |
|---|---|---|---|---|
| sym-d45896d5cc6abbfcd3e2 | `INITIAL` | assoc_const | Provides the initial value for `PermissionEpoch`. | `src/capture/authorization.rs:270` |
| sym-c22033b2bb1da21bc723 | `INITIAL` | assoc_const | Provides the initial value for `SourceGeneration`. | `src/capture/events.rs:15` |
| sym-74f10710a2809a6dd708 | `Error` | assoc_type | Specifies the error type returned by `SidecarMessageKind` operations. | `src/runtime/lifecycle/sidecar_protocol.rs:23` |
| sym-27e7db0a9ab3edb1a125 | `pocketstation::capture::capture_owner::CAPTURE_MONOTONIC_CLOCK_DOMAIN_ID` | constant | Monotonic timestamp domain used by native capture backends. | `src/capture/capture_owner.rs:20` |
| sym-3ddc5124a70a7fd4c6e0 | `pocketstation::capture::events::MAX_SOURCE_RUNTIME_EVENT_OWNED_BYTES` | constant | Maximum heap storage retained by one queued capture-runtime event. | `src/capture/events.rs:72` |
| sym-d2fef3176fadd031414b | `pocketstation::connector::configuration::MAX_CONNECTOR_CONFIGURATION_FIELDS` | constant | Sets the maximum supported connector configuration fields. | `src/connector/configuration.rs:7` |
| sym-a43816ac11ef8cc81714 | `pocketstation::connector::configuration::MAX_CONNECTOR_CONFIGURATION_TEXT_BYTES` | constant | Sets the maximum supported connector configuration text bytes. | `src/connector/configuration.rs:8` |
| sym-6add408cbc2bf612a68b | `pocketstation::connector::error::MAX_CONNECTOR_ERROR_CODE_BYTES` | constant | Sets the maximum supported connector error code bytes. | `src/connector/error.rs:6` |
| sym-9fd76cb6933c9c31700f | `pocketstation::connector::error::MAX_CONNECTOR_ERROR_MESSAGE_BYTES` | constant | Sets the maximum supported connector error message bytes. | `src/connector/error.rs:7` |
| sym-367b9a1751358138bcc8 | `pocketstation::connector::manifest::CONNECTOR_API_REVISION` | constant | Defines the public connector API revision value. | `src/connector/manifest.rs:7` |
| sym-49f5548b0f162983a485 | `pocketstation::connector::manifest::MAX_CONNECTOR_MANIFEST_ENTRIES` | constant | Sets the maximum supported connector manifest entries. | `src/connector/manifest.rs:8` |
| sym-c317835a9019aee97bf0 | `pocketstation::connector::manifest::MAX_CONNECTOR_MANIFEST_TEXT_BYTES` | constant | Sets the maximum supported connector manifest text bytes. | `src/connector/manifest.rs:9` |
| sym-6d10e937e6b71442d370 | `pocketstation::connector::readiness::MAX_CONNECTOR_READINESS_THRESHOLD` | constant | Sets the maximum supported connector readiness threshold. | `src/connector/readiness.rs:3` |
| sym-0f8963cd4aff37da994c | `pocketstation::connector::readiness::MAX_CONNECTOR_READINESS_TIMEOUT` | constant | Sets the maximum supported connector readiness timeout. | `src/connector/readiness.rs:4` |
| sym-2b3a399c6d301c1c156d | `pocketstation::connector::sidecar::CONNECTOR_AUDIO_RECORD_SCHEMA` | constant | Defines the public connector audio record schema value. | `src/connector/sidecar.rs:16` |
| sym-167e6ab7307591369c65 | `pocketstation::connector::sidecar::CONNECTOR_AUDIO_RECORD_SIGNAL_ID` | constant | Defines the public connector audio record signal identifier value. | `src/connector/sidecar.rs:15` |
| sym-a7d541537eae46555a07 | `pocketstation::connector::transport::CONNECTOR_AUDIO_RECORD_MAJOR` | constant | Defines the major version of connector audio record. | `src/connector/transport.rs:19` |
| sym-57b6690e2708bbeed295 | `pocketstation::connector::transport::CONNECTOR_AUDIO_RECORD_MINOR` | constant | Defines the minor version of connector audio record. | `src/connector/transport.rs:20` |
| sym-a2fcb3b3779761886d2d | `pocketstation::connector::transport::CONNECTOR_CONFIGURATION_RECORD_MAJOR` | constant | Defines the major version of connector configuration record. | `src/connector/transport.rs:35` |
| sym-c11d895bfffb51d5b95f | `pocketstation::connector::transport::CONNECTOR_CONFIGURATION_RECORD_MINOR` | constant | Defines the minor version of connector configuration record. | `src/connector/transport.rs:36` |
| sym-6e7b10da444a5e266240 | `pocketstation::connector::transport::MAX_CONNECTOR_AUDIO_RECORD_PORT_BYTES` | constant | Sets the maximum supported connector audio record port bytes. | `src/connector/transport.rs:21` |
| sym-85ead1181974f4b572f6 | `pocketstation::connector::transport::MAX_CONNECTOR_AUDIO_RECORD_SAMPLES` | constant | Sets the maximum supported connector audio record samples. | `src/connector/transport.rs:22` |
| sym-804cb4d1e469d61cc829 | `pocketstation::session::declaration::endpoint::BROWSER_NODE_TYPE_ID` | constant | Defines the public browser node type identifier value. | `src/session/declaration/endpoint.rs:9` |
| sym-6d008f47599f153a8603 | `pocketstation::session::declaration::endpoint::BROWSER_OPERATOR_ID` | constant | Defines the public browser operator identifier value. | `src/session/declaration/endpoint.rs:10` |
| sym-16ac09af997af1b6665e | `pocketstation::session::declaration::endpoint::CONNECTOR_NODE_TYPE_ID` | constant | Defines the public connector node type identifier value. | `src/session/declaration/endpoint.rs:8` |
| sym-85c50b0491e2a79b0657 | `pocketstation::session::declaration::spec::SESSION_SPEC_VERSION` | constant | Defines the public session spec version value. | `src/session/declaration/spec.rs:11` |
| sym-6d245e987c9a3e5c5d97 | `pocketstation::session::extensions::audio_input::PCM_SOURCE_TYPE_ID` | constant | Stable runtime identity of the underlying PCM source implementation. | `src/session/extensions/audio_input/mod.rs:19` |
| sym-3ae0d007bba659df6242 | `pocketstation::session::extensions::builtins::APPLICATION_SOURCE_NODE_TYPE_ID` | constant | Defines the public application source node type identifier value. | `src/session/extensions/builtins.rs:23` |
| sym-517965e07210915c7b77 | `pocketstation::session::extensions::builtins::MICROPHONE_SOURCE_NODE_TYPE_ID` | constant | Defines the public microphone source node type identifier value. | `src/session/extensions/builtins.rs:24` |
| sym-06cf9a8ce933978248bf | `pocketstation::session::extensions::recording::DEFAULT_MULTISTEM_RECORDING_GROUP_ID` | constant | Defines the public default multistem recording group identifier value. | `src/session/extensions/recording.rs:24` |
| sym-03489f746c101da321bb | `pocketstation::session::extensions::recording::RECORDER_NODE_TYPE_ID` | constant | Defines the public recorder node type identifier value. | `src/session/extensions/recording.rs:20` |
| sym-c993ca68ad4304032514 | `pocketstation::session::extensions::recording::RECORDER_OPERATOR_ID` | constant | Defines the public recorder operator identifier value. | `src/session/extensions/recording.rs:21` |
| sym-2ce482f65f371b4f7b49 | `pocketstation::session::extensions::recording::RECORDING_GROUP_CONFIGURATION_KEY` | constant | Defines the public recording group configuration key value. | `src/session/extensions/recording.rs:23` |
| sym-45544e6ea59c7420a9f9 | `pocketstation::capture::authorization::ApplicationPolicyObservation` | enum | Classifies the observable application policy observation. | `src/capture/authorization.rs:231` |
| sym-8e6567a1c2e9d6a4db50 | `pocketstation::capture::authorization::CaptureCapabilityState` | enum | Selects the capture capability state used by PocketStation. | `src/capture/authorization.rs:145` |
| sym-d136a6815bee037339a8 | `pocketstation::capture::authorization::CaptureError` | enum | Classifies failures reported as capture error. | `src/capture/authorization.rs:290` |
| sym-5f31b7e8f71b798c2bc3 | `pocketstation::capture::authorization::CaptureOpenOutcome` | enum | Classifies the observable capture open outcome. | `src/capture/authorization.rs:281` |
| sym-00e7060ce8fe6b8d80c9 | `pocketstation::capture::authorization::CaptureScope` | enum | Selects the capture scope used by PocketStation. | `src/capture/authorization.rs:248` |
| sym-62a2caf00b43e1bc9477 | `pocketstation::capture::authorization::CaptureSessionGrant` | enum | Enumerates the supported capture session grant cases. | `src/capture/authorization.rs:240` |
| sym-19385c71e1663f64be9e | `pocketstation::capture::authorization::PermissionObservation` | enum | Classifies the observable permission observation. | `src/capture/authorization.rs:153` |
| sym-cb9329e443cc1161cd37 | `pocketstation::capture::authorization::SourceIdentityStrength` | enum | Enumerates the supported source identity strength cases. | `src/capture/authorization.rs:257` |
| sym-a4c1d5e417a864f7851c | `pocketstation::capture::events::CaptureRuntimeFailureClass` | enum | Enumerates the supported capture runtime failure class cases. | `src/capture/events.rs:40` |
| sym-2de679293542511064e1 | `pocketstation::capture::events::SourceLifecycleEventKind` | enum | Selects the source lifecycle event kind used by PocketStation. | `src/capture/events.rs:25` |
| sym-73291cc2f1ac425b2f2f | `pocketstation::capture::events::SourceRecoveryRequirement` | enum | Selects the source recovery requirement used by PocketStation. | `src/capture/events.rs:35` |
| sym-64eaba6d85604fbcf37f | `pocketstation::capture::events::SourceRuntimeEvent` | enum | Classifies the observable source runtime event. | `src/capture/events.rs:53` |
| sym-ec6a41a2470440748019 | `pocketstation::capture::events::SourceRuntimeEventDelivery` | enum | Enumerates the supported source runtime event delivery cases. | `src/capture/events.rs:96` |
| sym-4fd8cc22a3328607bc3f | `pocketstation::capture::events::SourceRuntimeEventReceive` | enum | Enumerates the supported source runtime event receive cases. | `src/capture/events.rs:104` |
| sym-a9f392ffee51c33f8cd9 | `pocketstation::capture::frame_stream::CapturedFrameDelivery` | enum | Enumerates the supported captured frame delivery cases. | `src/capture/frame_stream.rs:10` |
| sym-6feea6318d057dc0a2fa | `pocketstation::capture::identity::SourceKind` | enum | Selects the source kind used by PocketStation. | `src/capture/identity.rs:9` |
| sym-a477f29ca688939ef8c1 | `pocketstation::capture::identity::SourceState` | enum | Selects the source state used by PocketStation. | `src/capture/identity.rs:17` |
| sym-8e57c3486871455ef5f0 | `pocketstation::capture::lifecycle_registry::SourceGenerationTransition` | enum | Enumerates the supported source generation transition cases. | `src/capture/lifecycle_registry.rs:8` |
| sym-42dfecbc7ac0471e6287 | `pocketstation::capture::query::SourceQuery` | enum | Enumerates the supported source query cases. | `src/capture/query.rs:13` |
| sym-270a444524627875d7e2 | `pocketstation::capture::selection::CaptureMode` | enum | Selects the capture mode used by PocketStation. | `src/capture/selection.rs:16` |
| sym-a982dfd305482431b4c3 | `pocketstation::capture::selection::InputDeviceSelector` | enum | Enumerates the supported input device selector cases. | `src/capture/selection.rs:9` |
| sym-21e17566ba71f7fbee27 | `pocketstation::capture::selection::ProcessTreeScope` | enum | Selects the process tree scope used by PocketStation. | `src/capture/selection.rs:83` |
| sym-fa016117ddeaa461fb2a | `pocketstation::capture::selection::SelectorPersistenceScope` | enum | Selects the selector persistence scope used by PocketStation. | `src/capture/selection.rs:73` |
| sym-9709f9e8853c47ef7563 | `pocketstation::capture::timeline::CaptureSampleTimelineError` | enum | Classifies failures reported as capture sample timeline error. | `src/capture/timeline.rs:41` |
| sym-3d4c1d38a9989d7b0f50 | `pocketstation::connector::ConnectorDeclarationError` | enum | Classifies failures reported as connector declaration error. | `src/connector/mod.rs:233` |
| sym-164d6b584a8bdd009168 | `pocketstation::connector::ConnectorObservationLookupError` | enum | Classifies failures reported as connector observation lookup error. | `src/connector/mod.rs:246` |
| sym-865e8231bc1517b8164c | `pocketstation::connector::ConnectorRegistrationError` | enum | Classifies failures reported as connector registration error. | `src/connector/mod.rs:225` |
| sym-6f2b5405b421f658d115 | `pocketstation::connector::configuration::ConnectorConfigurationConstraint` | enum | Enumerates the supported connector configuration constraint cases. | `src/connector/configuration.rs:159` |
| sym-f031f03c101f9a911940 | `pocketstation::connector::configuration::ConnectorConfigurationErrorCode` | enum | Enumerates the supported connector configuration error code cases. | `src/connector/configuration.rs:568` |
| sym-2db3fc6aeb733e5f80df | `pocketstation::connector::configuration::ConnectorConfigurationRequirement` | enum | Selects the connector configuration requirement used by PocketStation. | `src/connector/configuration.rs:152` |
| sym-799b6998995c65809689 | `pocketstation::connector::configuration::ConnectorConfigurationValue` | enum | Enumerates the supported connector configuration value cases. | `src/connector/configuration.rs:66` |
| sym-bf5b922f78e11c7c2724 | `pocketstation::connector::configuration::ConnectorConfigurationValueKind` | enum | Selects the connector configuration value kind used by PocketStation. | `src/connector/configuration.rs:55` |
| sym-f081d4c9f38635df144f | `pocketstation::connector::error::ConnectorErrorBuildError` | enum | Classifies failures reported as connector error build error. | `src/connector/error.rs:184` |
| sym-a87ed93ccba79867bb45 | `pocketstation::connector::error::ConnectorErrorCodeError` | enum | Classifies failures reported as connector error code error. | `src/connector/error.rs:50` |
| sym-9eccbf9e7fa5f752f186 | `pocketstation::connector::error::ConnectorErrorStage` | enum | Selects the connector error stage used by PocketStation. | `src/connector/error.rs:60` |
| sym-da47eb6e0afc1e331bdc | `pocketstation::connector::error::ConnectorRetryability` | enum | Enumerates the supported connector retryability cases. | `src/connector/error.rs:72` |
| sym-f6ee7a1c722da2532838 | `pocketstation::connector::manifest::ConnectorManifestError` | enum | Classifies failures reported as connector manifest error. | `src/connector/manifest.rs:231` |
| sym-2ea2f585fe271a4b349f | `pocketstation::connector::observations::ConnectorObservationError` | enum | Classifies failures reported as connector observation error. | `src/connector/observations.rs:175` |
| sym-ce552b8b636181eb9e46 | `pocketstation::connector::readiness::ConnectorReadinessPolicyError` | enum | Classifies failures reported as connector readiness policy error. | `src/connector/readiness.rs:61` |
| sym-0e83cdb9273ca8e5cebe | `pocketstation::connector::status::ConnectorDeliveryReadiness` | enum | Enumerates the supported connector delivery readiness cases. | `src/connector/status.rs:4` |
| sym-91837f146f991c3204dd | `pocketstation::connector::status::ConnectorHealth` | enum | Enumerates the supported connector health cases. | `src/connector/status.rs:17` |
| sym-b9a739eb2939aecfec6d | `pocketstation::connector::status::ConnectorRecovery` | enum | Enumerates the supported connector recovery cases. | `src/connector/status.rs:24` |
| sym-c2e8030ba03927259a57 | `pocketstation::connector::transport::ConnectorAudioRecordError` | enum | Classifies failures reported as connector audio record error. | `src/connector/transport.rs:568` |
| sym-452c6036ca5ce68e8efe | `pocketstation::connector::transport::ConnectorConfigurationRecordError` | enum | Classifies failures reported as connector configuration record error. | `src/connector/transport.rs:251` |
| sym-202d924e1265e8101308 | `pocketstation::connector::worker::driver::ConnectorDeliveryOutcome` | enum | Explicit delivery result used for Core-owned accounting. | `src/connector/worker/driver.rs:83` |
| sym-7ffd9236e9f7524a07f3 | `pocketstation::connector::worker::driver::ConnectorItem` | enum | One bounded item delivered by Core to a connector driver. | `src/connector/worker/driver.rs:62` |
| sym-adc64dc434868308ba4d | `pocketstation::endpoint::contract::EndpointReceiver` | enum | Enumerates the supported endpoint receiver cases. | `src/endpoint/contract.rs:145` |
| sym-280ef62d37b739c2237d | `pocketstation::endpoint::identity::EndpointPreparationGroup` | enum | Factory-owned decision for batching endpoint inputs into one lifecycle. | `src/endpoint/identity.rs:23` |
| sym-29ac19d5f350b7ef1b18 | `pocketstation::endpoint::polled_audio_driver::PolledAudioEndpointConfigError` | enum | Classifies failures reported as polled audio endpoint config error. | `src/endpoint/polled_audio_driver.rs:40` |
| sym-8ca869922a8deea3a920 | `pocketstation::endpoint::polled_audio_driver::PolledAudioPollError` | enum | Classifies failures reported as polled audio poll error. | `src/endpoint/polled_audio_driver.rs:74` |
| sym-94ff39a7865d0ba80250 | `pocketstation::endpoint::registry::EndpointDriverRegistryError` | enum | Classifies failures reported as endpoint driver registry error. | `src/endpoint/registry.rs:16` |
| sym-10b2ad948f87c981bacf | `pocketstation::endpoint::registry::EndpointPrepareError` | enum | Classifies failures reported as endpoint prepare error. | `src/endpoint/registry.rs:39` |
| sym-d1ab12a403969a2601f7 | `pocketstation::endpoint::runtime::EndpointFailureRetryability` | enum | Machine-readable recovery classification retained in Session outcomes. | `src/endpoint/runtime.rs:166` |
| sym-9a3122f3d2a19e134d9c | `pocketstation::endpoint::runtime::EndpointFailureStage` | enum | Selects the endpoint failure stage used by PocketStation. | `src/endpoint/runtime.rs:156` |
| sym-b59835aff00d66ed49b4 | `pocketstation::endpoint::runtime::EndpointInputOrigin` | enum | Provenance of one endpoint input, independent of its physical receiver. | `src/endpoint/runtime.rs:31` |
| sym-71b48b21c17d0623f40a | `pocketstation::endpoint::runtime::EndpointShutdownMode` | enum | Session shutdown intent delivered to an active endpoint. | `src/endpoint/runtime.rs:356` |
| sym-4e3aaf03500efef1dfcc | `pocketstation::endpoint::runtime::EndpointStartFailureCause` | enum | Enumerates the supported endpoint start failure cause cases. | `src/endpoint/runtime.rs:438` |
| sym-72778fc141679a99580a | `pocketstation::runtime::audio::executor::ExecError` | enum | Classifies failures reported as exec error. | `src/runtime/audio/executor.rs:20` |
| sym-7ca2c045f724dd2ae6c3 | `pocketstation::runtime::audio::router::PlanEdgeFrame` | enum | Enumerates the supported plan edge frame cases. | `src/runtime/audio/router.rs:29` |
| sym-f29dde8d44376648b56f | `pocketstation::runtime::audio::router::PlanRouterError` | enum | Classifies failures reported as plan router error. | `src/runtime/audio/router.rs:17` |
| sym-366f6936eb4a182d1370 | `pocketstation::runtime::audio::runner::PlanRunnerDrainPolicy` | enum | Selects the plan runner drain policy used by PocketStation. | `src/runtime/audio/runner.rs:16` |
| sym-c21696bfb75ebc3cc1da | `pocketstation::runtime::audio::runner::PlanRunnerError` | enum | Classifies failures reported as plan runner error. | `src/runtime/audio/runner.rs:256` |
| sym-0424de1ded2a867975e7 | `pocketstation::runtime::audio::runner::PlanSourceSendError` | enum | Classifies failures reported as plan source send error. | `src/runtime/audio/runner.rs:116` |
| sym-3a2190961d96af49fcde | `pocketstation::runtime::audio::runner::PlanSourceSendOutcome` | enum | Classifies the observable plan source send outcome. | `src/runtime/audio/runner.rs:123` |
| sym-e24b6f525fdda9a6952b | `pocketstation::runtime::bridge::audio::GeneratedAudioBridgeStartError` | enum | Classifies failures reported as generated audio bridge start error. | `src/runtime/bridge/audio.rs:46` |
| sym-5d012c772456cf235912 | `pocketstation::runtime::lifecycle::async_host::AsyncRuntimeHostError` | enum | Classifies failures reported as async runtime host error. | `src/runtime/lifecycle/async_host.rs:10` |
| sym-3cb273ebe961a3868346 | `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostError` | enum | Classifies failures reported as sidecar host error. | `src/runtime/lifecycle/sidecar_host.rs:686` |
| sym-44474c3c44bb3b6c3cdf | `pocketstation::runtime::lifecycle::sidecar_host::SidecarState` | enum | Selects the sidecar state used by PocketStation. | `src/runtime/lifecycle/sidecar_host.rs:21` |
| sym-6b577e3272162747940e | `pocketstation::runtime::lifecycle::sidecar_protocol::SidecarMessageKind` | enum | Selects the sidecar message kind used by PocketStation. | `src/runtime/lifecycle/sidecar_protocol.rs:9` |
| sym-4b79111cbd8e1f42594c | `pocketstation::runtime::lifecycle::sidecar_protocol::SidecarProtocolError` | enum | Classifies failures reported as sidecar protocol error. | `src/runtime/lifecycle/sidecar_protocol.rs:292` |
| sym-0db32d5b03ea32000c08 | `pocketstation::runtime::signal::edge::TypedEdgeBuildError` | enum | Classifies failures reported as typed edge build error. | `src/runtime/signal/edge.rs:386` |
| sym-5aa7b19d5378ce5eaea8 | `pocketstation::runtime::signal::edge::TypedEdgePublishError` | enum | Classifies failures reported as typed edge publish error. | `src/runtime/signal/edge.rs:408` |
| sym-19ea473fd41c1d05b328 | `pocketstation::runtime::signal::error::AsyncOperatorWorkerError` | enum | Classifies failures reported as async operator worker error. | `src/runtime/signal/error.rs:6` |
| sym-f0066e049c5c7d4afe06 | `pocketstation::session::compile::error::SessionCompileError` | enum | Classifies failures reported as session compile error. | `src/session/compile/error.rs:7` |
| sym-e5036e25894a9bd19ced | `pocketstation::session::declaration::selector::ApplicationSelector` | enum | Enumerates the supported application selector cases. | `src/session/declaration/selector.rs:32` |
| sym-2a37b7aba7766eb2085f | `pocketstation::session::declaration::selector::DeviceSelector` | enum | Enumerates the supported device selector cases. | `src/session/declaration/selector.rs:107` |
| sym-67a7cd80127200dd3e53 | `pocketstation::session::declaration::selector::Source` | enum | Enumerates the supported source cases. | `src/session/declaration/selector.rs:134` |
| sym-9b171b3b14186263c246 | `pocketstation::session::declaration::spec::ConnectionTarget` | enum | Stable destination of a declared Session connection. | `src/session/declaration/spec.rs:224` |
| sym-bb9aaa0a9f7b8c47969e | `pocketstation::session::declaration::spec::StreamOrigin` | enum | Stable origin of a declared Session stream. | `src/session/declaration/spec.rs:208` |
| sym-bcd838d23144fe0a5150 | `pocketstation::session::declaration::typed_stream::TypedStreamError` | enum | Classifies failures reported as typed stream error. | `src/session/declaration/typed_stream.rs:185` |
| sym-2d44a9b09e73f57e1980 | `pocketstation::session::error::SessionError` | enum | Classifies failures reported as session error. | `src/session/error.rs:6` |
| sym-40c269da41ffa4244fb3 | `pocketstation::session::error_code::PolledAudioPollErrorCode` | enum | Stable language-neutral code for bounded polled-audio status. | `src/session/error_code.rs:131` |
| sym-1ea01073e5a531190c8c | `pocketstation::session::error_code::SessionDeclarationErrorCode` | enum | Stable language-neutral code for a Session declaration failure. | `src/session/error_code.rs:10` |
| sym-9bbf4ca7353eb8b80a3b | `pocketstation::session::error_code::SessionRuntimeErrorCode` | enum | Stable language-neutral code for a running-Session projection failure. | `src/session/error_code.rs:116` |
| sym-dc084881210fd1f95a31 | `pocketstation::session::error_code::SessionStartErrorCode` | enum | Stable language-neutral code for Session startup. | `src/session/error_code.rs:61` |
| sym-cd977cb86cc028461907 | `pocketstation::session::error_code::SessionStopCode` | enum | Stable language-neutral status for an idempotent Session stop. | `src/session/error_code.rs:150` |
| sym-25520ca6a2c0f92958c7 | `pocketstation::session::error_code::SessionStopFailureCode` | enum | Stable language-neutral cause retained by a failed Session stop. | `src/session/error_code.rs:171` |
| sym-05274d9384700f30be94 | `pocketstation::session::extensions::audio_input::AudioInputConfigError` | enum | Classifies failures reported as audio input config error. | `src/session/extensions/audio_input/mod.rs:77` |
| sym-9bac772278df8b92b021 | `pocketstation::session::extensions::audio_input::buffer::AudioInputBufferAcquireError` | enum | Classifies failures reported as audio input buffer acquire error. | `src/session/extensions/audio_input/buffer.rs:271` |
| sym-d951941d9c9d6c5a1681 | `pocketstation::session::extensions::audio_input::buffer::AudioInputBufferError` | enum | Classifies failures reported as audio input buffer error. | `src/session/extensions/audio_input/buffer.rs:281` |
| sym-27597869b36ecf28803a | `pocketstation::session::extensions::audio_input::buffer::AudioInputWriteErrorKind` | enum | Selects the audio input write error kind used by PocketStation. | `src/session/extensions/audio_input/buffer.rs:298` |
| sym-f02fcab0fe4642b1b3ef | `pocketstation::session::extensions::audio_input::source::AudioInputError` | enum | Classifies failures reported as audio input error. | `src/session/extensions/audio_input/source.rs:85` |
| sym-31004e7841e29c97436e | `pocketstation::session::extensions::builtins::SessionGraphRegistrationError` | enum | Classifies failures reported as session graph registration error. | `src/session/extensions/builtins.rs:30` |
| sym-f89a3c8d7286d8b8f3ba | `pocketstation::session::extensions::source::SourceDriverError` | enum | Classifies failures reported as source driver error. | `src/session/extensions/source.rs:748` |
| sym-eae000aeb99308b88ddb | `pocketstation::session::extensions::source::SourceManifestError` | enum | Classifies failures reported as source manifest error. | `src/session/extensions/source.rs:677` |
| sym-b131ab7dede03efdad94 | `pocketstation::session::extensions::source::SourceRegistrationError` | enum | Classifies failures reported as source registration error. | `src/session/extensions/source.rs:701` |
| sym-641f0a6ba790503ad776 | `pocketstation::session::extensions::source::SourceRuntimeError` | enum | Classifies failures reported as source runtime error. | `src/session/extensions/source.rs:754` |
| sym-6eba5d6cddf8c6dfd76b | `pocketstation::session::extensions::source::SourceTypeIdError` | enum | Classifies failures reported as source type id error. | `src/session/extensions/source.rs:68` |
| sym-2d63dddb0c5548988336 | `pocketstation::session::lifecycle::engine::EndpointExtensionRegistrationError` | enum | Classifies failures reported as endpoint extension registration error. | `src/session/lifecycle/engine.rs:305` |
| sym-a203beaba8ba055a2bce | `pocketstation::session::lifecycle::engine::SessionEngineBuildError` | enum | Classifies failures reported as session engine build error. | `src/session/lifecycle/engine.rs:295` |
| sym-d2cc0c1cb5a4797e8572 | `pocketstation::session::lifecycle::engine::SessionEngineStartError` | enum | Classifies failures reported as session engine start error. | `src/session/lifecycle/engine.rs:315` |
| sym-53fa735e8159440881c8 | `pocketstation::session::lifecycle::events::SessionComponentId` | enum | Stable identity of the component that produced a session control failure. | `src/session/lifecycle/events.rs:51` |
| sym-c05c6d8f9186a094f9cc | `pocketstation::session::lifecycle::events::SessionEventKind` | enum | Payload of one authoritative session event. | `src/session/lifecycle/events.rs:294` |
| sym-8f259fc2d08ba3ecfbe5 | `pocketstation::session::lifecycle::events::SessionEventReceive` | enum | Result of non-blocking event polling. | `src/session/lifecycle/events.rs:492` |
| sym-937af8d19d7b66689e63 | `pocketstation::session::lifecycle::events::SessionFinalizationStage` | enum | The finalization operation that failed while stopping a session. | `src/session/lifecycle/events.rs:39` |
| sym-4ab5ff2fe6ee8a4e2361 | `pocketstation::session::lifecycle::events::SessionLifecycleState` | enum | Public lifecycle states emitted by a running session. | `src/session/lifecycle/events.rs:19` |
| sym-2bf9be8d950fbd8884dd | `pocketstation::session::lifecycle::events::SessionRollbackStage` | enum | The rollback operation that failed while unwinding a partial start. | `src/session/lifecycle/events.rs:29` |
| sym-69cb56ffa5b64e3b0c71 | `pocketstation::session::lifecycle::events::SessionTerminalState` | enum | Final state carried by the terminal session event. | `src/session/lifecycle/events.rs:210` |
| sym-52428312d2351433c079 | `pocketstation::session::lifecycle::host::SessionEngineHostBuildError` | enum | Classifies failures reported as session engine host build error. | `src/session/lifecycle/host.rs:362` |
| sym-549fa713c6641184c362 | `pocketstation::session::lifecycle::observations::EndpointObservationStage` | enum | Selects the endpoint observation stage used by PocketStation. | `src/session/lifecycle/observations.rs:441` |
| sym-facb5cfd020e190f4388 | `pocketstation::session::lifecycle::observations::SessionRouteLatencyBoundary` | enum | Enumerates the supported session route latency boundary cases. | `src/session/lifecycle/observations.rs:196` |
| sym-c41b6edbf361d6aa66e8 | `pocketstation::session::lifecycle::observations::SessionRouteLatencyUnit` | enum | Enumerates the supported session route latency unit cases. | `src/session/lifecycle/observations.rs:201` |
| sym-44d5399a1f5cac145c91 | `pocketstation::session::lifecycle::observations::SessionRouteObservationInterval` | enum | Interval covered by monotonic route counters. | `src/session/lifecycle/observations.rs:150` |
| sym-5d076a7f9e8470132d97 | `pocketstation::session::lifecycle::start_contract::SessionStartError` | enum | Classifies failures reported as session start error. | `src/session/lifecycle/start_contract.rs:113` |
| sym-b00006ccd18bda0129e1 | `pocketstation::session::lifecycle::trace::SessionTraceRecordKind` | enum | Selects the session trace record kind used by PocketStation. | `src/session/lifecycle/trace.rs:27` |
| sym-a564cb9f2cdd1012b5ad | `pocketstation::session::lifecycle::trace::SessionTraceRecorderFinishError` | enum | Classifies failures reported as session trace recorder finish error. | `src/session/lifecycle/trace.rs:98` |
| sym-95f8c3a9ad8e444ec798 | `pocketstation::session::lifecycle::trace::SessionTraceRecorderStartError` | enum | Classifies failures reported as session trace recorder start error. | `src/session/lifecycle/trace.rs:88` |
| sym-08c0d1f7736c0d5d8d3a | `pocketstation::session::lifecycle::trace::SessionTraceValidationError` | enum | Classifies failures reported as session trace validation error. | `src/session/lifecycle/trace.rs:356` |
| sym-f2369116f5c7d5602df9 | `pocketstation::session::prepare::error::SessionPrepareError` | enum | Classifies failures reported as session prepare error. | `src/session/prepare/error.rs:9` |
| sym-bceadb7ddb30e7466491 | `ActiveCaptureBackend::observation_handle` | function | Returns a handle for reading observations from `ActiveCaptureBackend`. | `src/capture/capture_owner.rs:107` |
| sym-5d9a59fe0fc2f3b7037e | `ActiveCaptureBackend::observations` | function | Returns the observations exposed by `ActiveCaptureBackend`. | `src/capture/capture_owner.rs:109` |
| sym-a0a449182596416f9615 | `ActiveCaptureBackend::source_id` | function | Resolved native source identity for every frame emitted by this open. | `src/capture/capture_owner.rs:105` |
| sym-74a75f4e4c0263afe3b0 | `ActiveCaptureBackend::stop_and_join` | function | Stops `ActiveCaptureBackend`, joins its worker, and returns the terminal result. | `src/capture/capture_owner.rs:111` |
| sym-1b834e57e91f557c870d | `CallbackCaptureBackend::prepare` | function | Prepares resources required by `CallbackCaptureBackend`. | `src/capture/capture_owner.rs:84` |
| sym-ff648a10e022adbf86c1 | `ConnectorDriver::cancel_preparation` | function | Cancels preparation for `ConnectorDriver`. | `src/connector/worker/driver.rs:116` |
| sym-0c57c15fdb1d0bfce39a | `ConnectorDriver::deliver` | function | Delivers the next input through `ConnectorDriver`. | `src/connector/worker/driver.rs:98` |
| sym-e82141b3e8f7b5589f0b | `ConnectorDriver::idle` | function | Advances `ConnectorDriver` while no input is available. | `src/connector/worker/driver.rs:104` |
| sym-071595728f4e48b1264a | `ConnectorDriver::shutdown` | function | Shuts down `ConnectorDriver` according to its lifecycle contract. | `src/connector/worker/driver.rs:108` |
| sym-8c7afd4ca51afc8e2e2f | `ConnectorDriver::start` | function | Starts the lifecycle represented by `ConnectorDriver`. | `src/connector/worker/driver.rs:93` |
| sym-b25b150e306029002c43 | `ConnectorDriverFactory::preparation_group` | function | Returns the preparation group associated with `ConnectorDriverFactory`. | `src/connector/worker/driver.rs:124` |
| sym-c2659e6eab26e8acded1 | `ConnectorDriverFactory::prepare` | function | Prepares resources required by `ConnectorDriverFactory`. | `src/connector/worker/driver.rs:132` |
| sym-464c595c2d49e8b34238 | `ConnectorFactory::preparation_group` | function | Returns the preparation group associated with `ConnectorFactory`. | `src/connector/worker/mod.rs:18` |
| sym-5518367e21abbc9b9c68 | `ConnectorFactory::prepare` | function | Prepares resources required by `ConnectorFactory`. | `src/connector/worker/mod.rs:26` |
| sym-3cedffc9b00820c04f73 | `ConnectorWorker::cancel_preparation` | function | Cancels preparation for `ConnectorWorker`. | `src/connector/worker/mod.rs:35` |
| sym-78787b869bd20ca714a5 | `ConnectorWorker::run` | function | Runs `ConnectorWorker` until completion or cancellation. | `src/connector/worker/mod.rs:33` |
| sym-4785d950f1deba565d3d | `EndpointDriverFactory::preparation_group` | function | Returns the preparation group associated with `EndpointDriverFactory`. | `src/endpoint/contract.rs:233` |
| sym-5858eb75b928c8f5a1d0 | `EndpointDriverFactory::prepare` | function | Prepares resources required by `EndpointDriverFactory`. | `src/endpoint/contract.rs:241` |
| sym-f70e9390f0489e77ec23 | `PreparedCaptureBackend::open` | function | Opens the resource represented by `PreparedCaptureBackend`. | `src/capture/capture_owner.rs:89` |
| sym-775076f9f636c432e69c | `PreparedEndpointDriver::cancel_preparation` | function | Cancels preparation for `PreparedEndpointDriver`. | `src/endpoint/runtime.rs:328` |
| sym-5c9f2e79390cbd3a6554 | `PreparedEndpointDriver::start` | function | Makes endpoint resources ready behind the supplied closed gate. | `src/endpoint/runtime.rs:323` |
| sym-f0e8cdf84d48aca086d6 | `RunningEndpointDriver::join_and_finalize` | function | Joins and finalize for `RunningEndpointDriver`. | `src/endpoint/runtime.rs:346` |
| sym-459a9a074e0856994405 | `RunningEndpointDriver::observations` | function | Returns the observations exposed by `RunningEndpointDriver`. | `src/endpoint/runtime.rs:337` |
| sym-4cb08eb58cfec6f939e5 | `RunningEndpointDriver::request_shutdown` | function | Requests the selected shutdown mode from `RunningEndpointDriver`. | `src/endpoint/runtime.rs:341` |
| sym-d1f376b21fdab92dc979 | `RunningEndpointDriver::request_stop` | function | Requests a graceful stop from `RunningEndpointDriver`. | `src/endpoint/runtime.rs:339` |
| sym-617d7fa081197246ac8a | `SourceDriver::close` | function | Closes `SourceDriver` to further work. | `src/session/extensions/source.rs:273` |
| sym-b93a63c9f08d180a1385 | `SourceDriver::next` | function | Produces the next source emission from `SourceDriver`. | `src/session/extensions/source.rs:269` |
| sym-1791481d286846241ac9 | `SourceDriver::prepare` | function | Prepares resources required by `SourceDriver`. | `src/session/extensions/source.rs:268` |
| sym-2e8c31657a884516d1db | `SourceFactory::create` | function | Creates the runtime implementation described by `SourceFactory`. | `src/session/extensions/source.rs:279` |
| sym-6a128ee236756cc8905e | `SourceFactory::manifest` | function | Returns the manifest held by `SourceFactory`. | `src/session/extensions/source.rs:277` |
| sym-6cd2abb6461a40ec9b16 | `SourceFactory::validate_config` | function | Validates config for `SourceFactory`. | `src/session/extensions/source.rs:278` |
| sym-5d1bf5166addc34ce5d2 | `StreamSignal::signal_spec` | function | Returns the signal spec held by `StreamSignal`. | `src/session/declaration/typed_stream.rs:16` |
| sym-1674b375b35b6e757720 | `accepts_delivery` | function | Returns whether accepts delivery applies to `ConnectorDeliveryReadiness`. | `src/connector/status.rs:10` |
| sym-1a7508978d9c24e664bb | `accepts_delivery` | function | Returns whether accepts delivery applies to `ConnectorServiceStatus`. | `src/connector/status.rs:74` |
| sym-0760bc020ba894b3f5bf | `advance_from_source_position` | function | Returns a buffer's source-time start from its native sample-frame position. Forward gaps are preserved in the returned timestamp without separately advancing this clock from an aggregate drop counter. | `src/capture/timeline.rs:80` |
| sym-fa0ffc427c129433bceb | `after_failed_open` | function | Records a backend open failure without guessing that permission was denied. | `src/capture/authorization.rs:45` |
| sym-d0f5605e55cfb06011a6 | `after_successful_open` | function | Records the evidence available after an explicitly selected source opens. | `src/capture/authorization.rs:31` |
| sym-e92fd812c090f3a0cb3c | `api_revision` | function | Returns the API revision held by `ConnectorManifest`. | `src/connector/manifest.rs:124` |
| sym-ef79fdde27b96aa64bd9 | `application` | function | Returns the application held by `Source`. | `src/session/declaration/selector.rs:140` |
| sym-ff0a5c3647be65f22b11 | `as_str` | function | Returns the stable string representation of `ConnectorConfigurationErrorCode`. | `src/connector/configuration.rs:585` |
| sym-97f88351f6898933637f | `as_str` | function | Returns the stable string representation of `ConnectorErrorCode`. | `src/connector/error.rs:29` |
| sym-095f9600d9b7441f9d03 | `as_str` | function | Returns the stable string representation of `EndpointGroupId`. | `src/endpoint/identity.rs:16` |
| sym-bf6c00beab1af63e1860 | `as_str` | function | Returns the stable string representation of `DeviceId`. | `src/session/declaration/selector.rs:26` |
| sym-24fd498b139cb8cb0ac2 | `as_str` | function | Returns the stable string representation of `SessionDeclarationErrorCode`. | `src/session/error_code.rs:31` |
| sym-d690b5d01072ac18d623 | `as_str` | function | Returns the stable string representation of `SessionStartErrorCode`. | `src/session/error_code.rs:86` |
| sym-bae0d7a3f3cb88aef9ae | `as_str` | function | Returns the stable string representation of `SessionRuntimeErrorCode`. | `src/session/error_code.rs:121` |
| sym-a050f7fae6bde7725ea6 | `as_str` | function | Returns the stable string representation of `PolledAudioPollErrorCode`. | `src/session/error_code.rs:138` |
| sym-e7079625485c166b9318 | `as_str` | function | Returns the stable string representation of `SessionStopCode`. | `src/session/error_code.rs:157` |
| sym-ea9faf951c1432c15b29 | `as_str` | function | Returns the stable string representation of `SessionStopFailureCode`. | `src/session/error_code.rs:182` |
| sym-02909264b072a2cd7279 | `as_str` | function | Returns the stable string representation of `SourceTypeId`. | `src/session/extensions/source.rs:52` |
| sym-2bae5d019dea07a6f21b | `audio_frames_enqueued_total` | function | Returns the audio frames enqueued total held by `SessionAudioReentryMetrics`. | `src/session/lifecycle/observations.rs:368` |
| sym-3619d61a5d57a170ea24 | `audio_reentry_metrics` | function | Returns the audio reentry metrics held by `RunningSession`. | `src/session/lifecycle/running.rs:217` |
| sym-7f4c17f9bbbf1584724e | `audio_stem_id` | function | Returns the audio stem identifier held by `EndpointRouteContext`. | `src/endpoint/runtime.rs:88` |
| sym-2ecc6be8dc695c834430 | `before_open` | function | Records a source that was resolved but not opened because another required source failed first. | `src/capture/authorization.rs:60` |
| sym-aad1ecb21c3c011a267f | `browser` | function | Returns the browser associated with `Session`. | `src/session/declaration/draft.rs:443` |
| sym-9bf522ae209685bd39bf | `build` | function | Consumes all setup state so no partially populated registry can escape. | `src/session/lifecycle/engine.rs:176` |
| sym-2d52a1225970e7513f70 | `build` | function | Builds its owned operation for `SessionEngineHostBuilder`. | `src/session/lifecycle/host.rs:344` |
| sym-8dcf1610f287d6f7139c | `bundle_id` | function | Returns the bundle identifier held by `ApplicationSelector`. | `src/session/declaration/selector.rs:44` |
| sym-07d5fd0f7862cc3e0ddf | `cancel` | function | Requests cancellation of `SourceRuntime`. | `src/session/extensions/source.rs:575` |
| sym-cd665b6efd7a7a26e678 | `cancel` | function | Requests cancellation of `RunningSession`. | `src/session/lifecycle/running.rs:417` |
| sym-3761e5ac37050c354529 | `cancel_and_join` | function | Cancels and join for `GeneratedAudioBridge`. | `src/runtime/bridge/audio.rs:184` |
| sym-c6eb9568705b06fea58e | `cancel_and_join` | function | Cancels and join for `AsyncOperatorWorker`. | `src/runtime/signal/operator.rs:925` |
| sym-363ceb64721f1577a42d | `cancel_and_reap` | function | Cancels and reap for `SidecarHost`. | `src/runtime/lifecycle/sidecar_host.rs:322` |
| sym-cd9034aece59b4e2f5ad | `cancellation_requested` | function | Returns whether cancellation requested is true for `PreparedSession`. | `src/session/prepare/prepared.rs:73` |
| sym-6a03bc81241e552b45a7 | `cancellation_total` | function | Returns the cancellation total held by `SessionAudioReentryMetrics`. | `src/session/lifecycle/observations.rs:372` |
| sym-71c8248552577ce31a95 | `capabilities` | function | Returns the capabilities held by `ConnectorManifest`. | `src/connector/manifest.rs:152` |
| sym-099e7f35723bb117ce7d | `capacity_frames` | function | Returns the capacity frames held by `CapturedFrameStream`. | `src/capture/frame_stream.rs:165` |
| sym-0347dd7ca21a781e74ea | `capacity_frames` | function | Returns the capacity frames held by `AudioInputConfig`. | `src/session/extensions/audio_input/mod.rs:63` |
| sym-53e7fa01c1555e29c482 | `capture` | function | Declares a capture source on `Session` and returns its Session-scoped handle. | `src/session/declaration/draft.rs:347` |
| sym-8f46c10e198bfff05070 | `capture_finalization_failures_total` | function | Returns the capture finalization failures total held by `SessionStopOutcome`. | `src/session/lifecycle/start_contract.rs:330` |
| sym-b75c58a2c4f1f420edb8 | `capture_mode` | function | Returns the capture mode held by `SystemLoopbackSource`. | `src/capture/platform/macos/loopback.rs:57` |
| sym-1e412832c9049ffd81cf | `capture_mode` | function | Returns the capture mode held by `DesktopCaptureSource`. | `src/capture/platform/macos/mod.rs:44` |
| sym-e0ccb37acd23f29eaeeb | `channels` | function | Returns the channel count represented by `EndpointAudioFrame`. | `src/endpoint/contract.rs:47` |
| sym-7114c597af9660b246ff | `channels` | function | Returns the channel count represented by `PolledAudioFrame`. | `src/endpoint/polled_audio_driver.rs:239` |
| sym-7bfc0bf29cb72d1895d8 | `channels` | function | Returns the channel count represented by `PlanEdgeFrame`. | `src/runtime/audio/router.rs:70` |
| sym-f0ea5047357f5a71e926 | `clipped_samples` | function | Returns the clipped samples held by `MixerTelemetry`. | `src/runtime/nodes.rs:260` |
| sym-5974737f23bbd935c441 | `close` | function | Closes `AudioInputWriter` to further work. | `src/session/extensions/audio_input/buffer.rs:242` |
| sym-fe6bbea7f6396ae52e3f | `close` | function | Closes this application-owned input after its accepted frames drain. | `src/session/extensions/audio_input/mod.rs:127` |
| sym-3533308edab8aa54e59a | `close_and_reap` | function | Closes `SidecarHost` and reaps its child process. | `src/runtime/lifecycle/sidecar_host.rs:326` |
| sym-7d045272667522a51932 | `code` | function | Returns the stable error or status code represented by `ConnectorConfigurationError`. | `src/connector/configuration.rs:615` |
| sym-cfeaffe44c2c31baa3a1 | `code` | function | Returns the stable error or status code represented by `ConnectorError`. | `src/connector/error.rs:109` |
| sym-91cb556f2fd4381a2927 | `code` | function | Returns the stable error or status code represented by `EndpointFailure`. | `src/endpoint/runtime.rs:212` |
| sym-9473a946f8e1e9763a7f | `compile` | function | Compiles its owned operation for `SessionCompiler`. | `src/session/compile/mod.rs:103` |
| sym-799be876bf20539f034a | `compile` | function | Compiles its owned operation for `SessionEngine`. | `src/session/lifecycle/engine.rs:221` |
| sym-dc5030b6c84ca3e2a8be | `compile` | function | Compiles its owned operation for `SessionEngineHost`. | `src/session/lifecycle/host.rs:55` |
| sym-57ec0512402f2f8c6afb | `component` | function | Returns the component associated with `SessionControlFailure`. | `src/session/lifecycle/events.rs:89` |
| sym-6f5e7397353830b89518 | `configuration` | function | Returns the configuration held by `ConnectorManifest`. | `src/connector/manifest.rs:144` |
| sym-f07d31ab0709980d6ef6 | `configuration` | function | Returns the configuration held by `ConnectorConfigurationRecord`. | `src/connector/transport.rs:53` |
| sym-002112bdc145b6baed87 | `configuration` | function | Returns the configuration held by `ConnectorInputDescriptor`. | `src/connector/worker/driver.rs:56` |
| sym-7a4db6580a74ea5f240a | `configuration` | function | Returns the configuration held by `Operator`. | `src/session/declaration/draft.rs:311` |
| sym-e7cfed4a3cb883c49121 | `configuration` | function | Returns the configuration held by `EndpointDescriptor`. | `src/session/declaration/endpoint.rs:149` |
| sym-9f230d21bdc84a596287 | `configuration` | function | Returns the configuration held by `SourceInstanceSpec`. | `src/session/declaration/spec.rs:88` |
| sym-2836ab43d9c89c62a2b1 | `configuration` | function | Returns the configuration held by `EndpointSpec`. | `src/session/declaration/spec.rs:187` |
| sym-ebd1827cb46a384379c9 | `configuration` | function | Returns the configuration held by `OperatorInstanceSpec`. | `src/session/declaration/spec.rs:253` |
| sym-fcf389dd71118ef3a41c | `configuration` | function | Returns the configuration held by `AudioInputWriter`. | `src/session/extensions/audio_input/buffer.rs:103` |
| sym-97b189b635cb7c6dbcaf | `connect` | function | Connects the requested ports through `StemHandle`. | `src/session/declaration/draft.rs:819` |
| sym-b5ff73fcfbbc618dae3f | `connect` | function | Connects the requested ports through `SourceOutputHandle`. | `src/session/declaration/draft.rs:955` |
| sym-4720f8c89561ea4b9ea0 | `connect` | function | Connects the requested ports through `DerivedStreamHandle`. | `src/session/declaration/draft.rs:1051` |
| sym-6b73136640e4945a0e3d | `connections` | function | Returns the connections associated with `SessionSpec`. | `src/session/declaration/spec.rs:347` |
| sym-8381579d195d96495dd5 | `connector` | function | Declares a connector endpoint on `Session` with the supplied operator identity and configuration. | `src/session/declaration/draft.rs:411` |
| sym-13ba1f8b4e64872e2f75 | `connector_id` | function | Returns the connector identifier held by `ConnectorInputDescriptor`. | `src/connector/worker/driver.rs:32` |
| sym-1f4adc143d29ff96861c | `connector_id` | function | Returns the connector identifier held by `PolledAudioFrame`. | `src/endpoint/polled_audio_driver.rs:231` |
| sym-411c5f107d46d687613f | `connector_id` | function | Returns the connector identifier held by `EndpointPrepareContext`. | `src/endpoint/runtime.rs:138` |
| sym-f4f0b73539dae0c559db | `connector_id` | function | Returns the connector identifier held by `EndpointHandle`. | `src/session/declaration/draft.rs:607` |
| sym-cf7d333c61a83867f2a0 | `connector_id` | function | Returns the connector identifier held by `EndpointSpec`. | `src/session/declaration/spec.rs:175` |
| sym-1165bb75ef97e65e025a | `constraints` | function | Returns the constraints held by `ConnectorConfigurationField`. | `src/connector/configuration.rs:222` |
| sym-765b19e5b736c4a1abb1 | `context` | function | Returns the context held by `EndpointPortInput`. | `src/endpoint/contract.rs:219` |
| sym-b9dec8c2506c097ed3e7 | `current` | function | Returns the current value observed by `CapturePermissionLifecycle`. | `src/capture/authorization.rs:196` |
| sym-c8fe3feaf7fa7ca52e7d | `declare` | function | Adds the declaration represented by `RegisteredConnector` to its Session. | `src/connector/mod.rs:159` |
| sym-54fecfe30fb4f38fba2b | `declares_multistem_recording` | function | Returns whether `Session` declares multistem recording. | `src/session/extensions/recording.rs:98` |
| sym-dc7ecd45f2947dea2ccb | `decode` | function | Decodes input through `ConnectorConfigurationRecord`. | `src/connector/transport.rs:93` |
| sym-53ba1550fb3c2f8d1dba | `decode` | function | Decodes input through `ConnectorAudioRecord`. | `src/connector/transport.rs:403` |
| sym-8d92b0a47a80648a77d8 | `decode` | function | Decodes input through `SidecarMessage`. | `src/runtime/lifecycle/sidecar_protocol.rs:121` |
| sym-6a7daffc3610023cad91 | `default` | function | Returns the default `PolledAudioEndpointConfig` value. | `src/endpoint/polled_audio_driver.rs:30` |
| sym-5d74bb4ba301e13b6548 | `default` | function | Returns the default `PlanRunnerCancellation` value. | `src/runtime/audio/runner.rs:110` |
| sym-7db8245fb4f216c6a814 | `default` | function | Returns the default `SidecarDeadlines` value. | `src/runtime/lifecycle/sidecar_host.rs:61` |
| sym-af19b8b3405091440643 | `default` | function | Returns the default `SidecarProtocolLimits` value. | `src/runtime/lifecycle/sidecar_protocol.rs:51` |
| sym-9c03c0728aa593e1d73f | `default` | function | Returns the default `Session` value. | `src/session/declaration/draft.rs:577` |
| sym-b4a0bbefbec170268134 | `default` | function | Returns the default `DeviceSelector` value. | `src/session/declaration/selector.rs:113` |
| sym-fbbdd70b9d27a4341930 | `default` | function | Returns the default `NativeSessionEngineHostOptions` value. | `src/session/lifecycle/host.rs:172` |
| sym-7d2662192d98d7cb188f | `default` | function | Returns the default `SessionStartOptions` value. | `src/session/lifecycle/start_contract.rs:33` |
| sym-263af9445d9a180f56bc | `delivery_readiness` | function | Returns the delivery readiness associated with `ConnectorServiceStatus`. | `src/connector/status.rs:42` |
| sym-6bc2402fe269fe7f6b4e | `deprecated` | function | Returns the deprecated held by `ConnectorConfigurationField`. | `src/connector/configuration.rs:201` |
| sym-a1444469e193165c6f22 | `deprecation` | function | Returns the deprecation held by `ConnectorConfigurationField`. | `src/connector/configuration.rs:226` |
| sym-12355a31167104ea1ffd | `derived_route` | function | Returns the derived route associated with `SessionMetricsSnapshot`. | `src/session/lifecycle/observations.rs:111` |
| sym-0a891da3911ec35e5ef0 | `derived_route_count` | function | Returns the derived route count held by `SessionMetricsSnapshot`. | `src/session/lifecycle/observations.rs:107` |
| sym-f48d91e269c7742941c0 | `derived_route_metrics` | function | Returns the derived route metrics held by `RunningSession`. | `src/session/lifecycle/running.rs:213` |
| sym-458cc89a27928f87c11f | `descriptor` | function | Returns the descriptor associated with `SystemOutputSourceFactory`. | `src/runtime/nodes.rs:87` |
| sym-3bc3623fa5be2ed7f264 | `descriptor` | function | Returns the descriptor associated with `BridgeSinkFactory`. | `src/runtime/nodes.rs:191` |
| sym-f7b04b27aaa8ae652791 | `discover` | function | Discovers the resources visible to `LocalSourceProvider`. | `src/capture/query.rs:55` |
| sym-c80e661237ec531c7b87 | `dispatch_from` | function | Routes one lineaged audio frame from the named plan output through `PlanEdgeRouter`. | `src/runtime/audio/router.rs:748` |
| sym-91a5f8f773d352edab66 | `documentation` | function | Returns the documentation held by `ConnectorConfigurationField`. | `src/connector/configuration.rs:218` |
| sym-4b3bc0edf728e8a49312 | `documentation` | function | Returns the documentation held by `ConnectorCapability`. | `src/connector/manifest.rs:34` |
| sym-fdf4039f27c61720c6a2 | `documentation` | function | Returns the documentation held by `ConnectorRequirement`. | `src/connector/manifest.rs:69` |
| sym-7a9a3832c93c1ee7b3f2 | `drop` | function | Releases resources owned by `MacosInputSource`. | `src/capture/platform/macos/input.rs:251` |
| sym-c4021746d79020bddaed | `drop` | function | Releases resources owned by `SystemLoopbackSource`. | `src/capture/platform/macos/loopback.rs:299` |
| sym-7e10b6736a3774e1a045 | `drop` | function | Releases resources owned by `ConnectorSecret`. | `src/connector/configuration.rs:49` |
| sym-4d610cca23cad69c4723 | `drop` | function | Releases resources owned by `PolledAudioBatchLease`. | `src/endpoint/polled_audio_driver.rs:195` |
| sym-da94af93e9fc30978cec | `drop` | function | Releases resources owned by `PlanEdgeReceiver`. | `src/runtime/audio/router.rs:647` |
| sym-7ef8f0f3ed6efc006705 | `drop` | function | Releases resources owned by `GeneratedAudioBridge`. | `src/runtime/bridge/audio.rs:193` |
| sym-346774e7be9e63a47d71 | `drop` | function | Releases resources owned by `AsyncRuntimeHost`. | `src/runtime/lifecycle/async_host.rs:105` |
| sym-32a844e5382e088dcfad | `drop` | function | Releases resources owned by `SidecarHost`. | `src/runtime/lifecycle/sidecar_host.rs:521` |
| sym-f3973e0a054b591131ff | `drop` | function | Releases resources owned by `PreparedSourceRuntime`. | `src/session/extensions/source.rs:556` |
| sym-83f894af5771b0d83fef | `drop` | function | Releases resources owned by `SourceRuntime`. | `src/session/extensions/source.rs:594` |
| sym-b9e30ae1a157e2696385 | `drop` | function | Releases resources owned by `RunningSession`. | `src/session/lifecycle/running.rs:607` |
| sym-ff8e4ef4847b0ab0bcfd | `drop` | function | Releases resources owned by `SessionTraceRecorder`. | `src/session/lifecycle/trace.rs:249` |
| sym-227ae9201f38ed2ea882 | `drop_observations` | function | Returns the drop observations held by `SessionRouteMetrics`. | `src/session/lifecycle/observations.rs:206` |
| sym-75267ee39a6db0b853ae | `drop_rate_pct` | function | Returns the drop rate pct held by `EdgeObservations`. | `src/runtime/audio/router.rs:165` |

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

The claims on **Observation API** are anchored to Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `src/session/lifecycle/observations.rs:1-636` (`DIRECT`)

For **Observation API**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.
