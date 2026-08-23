# Sidecar protocol

<!-- claims: CLM-REF-011-CAP-001,CLM-REF-011-SOURCE-001 -->

## Scope

- **Host managed-process sidecars.** Exchange bounded protocol messages with a child process under explicit deadlines and lifecycle states.

The scope of **Sidecar protocol** ends at the native contracts and executable conditions cited below. Platform qualification, performance, retry, and delivery require their own explicit evidence.

## Reference authority

For **Sidecar protocol**, the generated [docs.rs API](https://docs.rs/pocketstation/latest/pocketstation/) provides compiler-rendered signatures and navigation. This repository page adds the frozen evidence identifiers, responsibilities, and cross-boundary interpretation used by the documentation verifier.

## Public surface

| Evidence record | Declaration | Kind | Purpose | Source |
|---|---|---|---|---|
| sym-74f10710a2809a6dd708 | `Error` | assoc_type | Specifies the error type returned by `SidecarMessageKind` operations. | `src/runtime/lifecycle/sidecar_protocol.rs:23` |
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
| sym-1674b375b35b6e757720 | `accepts_delivery` | function | Returns whether accepts delivery applies to `ConnectorDeliveryReadiness`. | `src/connector/status.rs:10` |
| sym-1a7508978d9c24e664bb | `accepts_delivery` | function | Returns whether accepts delivery applies to `ConnectorServiceStatus`. | `src/connector/status.rs:74` |
| sym-e92fd812c090f3a0cb3c | `api_revision` | function | Returns the API revision held by `ConnectorManifest`. | `src/connector/manifest.rs:124` |
| sym-ff0a5c3647be65f22b11 | `as_str` | function | Returns the stable string representation of `ConnectorConfigurationErrorCode`. | `src/connector/configuration.rs:585` |
| sym-97f88351f6898933637f | `as_str` | function | Returns the stable string representation of `ConnectorErrorCode`. | `src/connector/error.rs:29` |
| sym-3761e5ac37050c354529 | `cancel_and_join` | function | Cancels and join for `GeneratedAudioBridge`. | `src/runtime/bridge/audio.rs:184` |
| sym-c6eb9568705b06fea58e | `cancel_and_join` | function | Cancels and join for `AsyncOperatorWorker`. | `src/runtime/signal/operator.rs:925` |
| sym-363ceb64721f1577a42d | `cancel_and_reap` | function | Cancels and reap for `SidecarHost`. | `src/runtime/lifecycle/sidecar_host.rs:322` |
| sym-71c8248552577ce31a95 | `capabilities` | function | Returns the capabilities held by `ConnectorManifest`. | `src/connector/manifest.rs:152` |
| sym-7bfc0bf29cb72d1895d8 | `channels` | function | Returns the channel count represented by `PlanEdgeFrame`. | `src/runtime/audio/router.rs:70` |
| sym-f0ea5047357f5a71e926 | `clipped_samples` | function | Returns the clipped samples held by `MixerTelemetry`. | `src/runtime/nodes.rs:260` |
| sym-3533308edab8aa54e59a | `close_and_reap` | function | Closes `SidecarHost` and reaps its child process. | `src/runtime/lifecycle/sidecar_host.rs:326` |
| sym-7d045272667522a51932 | `code` | function | Returns the stable error or status code represented by `ConnectorConfigurationError`. | `src/connector/configuration.rs:615` |
| sym-cfeaffe44c2c31baa3a1 | `code` | function | Returns the stable error or status code represented by `ConnectorError`. | `src/connector/error.rs:109` |
| sym-6f5e7397353830b89518 | `configuration` | function | Returns the configuration held by `ConnectorManifest`. | `src/connector/manifest.rs:144` |
| sym-f07d31ab0709980d6ef6 | `configuration` | function | Returns the configuration held by `ConnectorConfigurationRecord`. | `src/connector/transport.rs:53` |
| sym-002112bdc145b6baed87 | `configuration` | function | Returns the configuration held by `ConnectorInputDescriptor`. | `src/connector/worker/driver.rs:56` |
| sym-13ba1f8b4e64872e2f75 | `connector_id` | function | Returns the connector identifier held by `ConnectorInputDescriptor`. | `src/connector/worker/driver.rs:32` |
| sym-1165bb75ef97e65e025a | `constraints` | function | Returns the constraints held by `ConnectorConfigurationField`. | `src/connector/configuration.rs:222` |
| sym-c8fe3feaf7fa7ca52e7d | `declare` | function | Adds the declaration represented by `RegisteredConnector` to its Session. | `src/connector/mod.rs:159` |
| sym-dc7ecd45f2947dea2ccb | `decode` | function | Decodes input through `ConnectorConfigurationRecord`. | `src/connector/transport.rs:93` |
| sym-53ba1550fb3c2f8d1dba | `decode` | function | Decodes input through `ConnectorAudioRecord`. | `src/connector/transport.rs:403` |
| sym-8d92b0a47a80648a77d8 | `decode` | function | Decodes input through `SidecarMessage`. | `src/runtime/lifecycle/sidecar_protocol.rs:121` |
| sym-5d74bb4ba301e13b6548 | `default` | function | Returns the default `PlanRunnerCancellation` value. | `src/runtime/audio/runner.rs:110` |
| sym-7db8245fb4f216c6a814 | `default` | function | Returns the default `SidecarDeadlines` value. | `src/runtime/lifecycle/sidecar_host.rs:61` |
| sym-af19b8b3405091440643 | `default` | function | Returns the default `SidecarProtocolLimits` value. | `src/runtime/lifecycle/sidecar_protocol.rs:51` |
| sym-263af9445d9a180f56bc | `delivery_readiness` | function | Returns the delivery readiness associated with `ConnectorServiceStatus`. | `src/connector/status.rs:42` |
| sym-6bc2402fe269fe7f6b4e | `deprecated` | function | Returns the deprecated held by `ConnectorConfigurationField`. | `src/connector/configuration.rs:201` |
| sym-a1444469e193165c6f22 | `deprecation` | function | Returns the deprecation held by `ConnectorConfigurationField`. | `src/connector/configuration.rs:226` |
| sym-458cc89a27928f87c11f | `descriptor` | function | Returns the descriptor associated with `SystemOutputSourceFactory`. | `src/runtime/nodes.rs:87` |
| sym-3bc3623fa5be2ed7f264 | `descriptor` | function | Returns the descriptor associated with `BridgeSinkFactory`. | `src/runtime/nodes.rs:191` |
| sym-c80e661237ec531c7b87 | `dispatch_from` | function | Routes one lineaged audio frame from the named plan output through `PlanEdgeRouter`. | `src/runtime/audio/router.rs:748` |
| sym-91a5f8f773d352edab66 | `documentation` | function | Returns the documentation held by `ConnectorConfigurationField`. | `src/connector/configuration.rs:218` |
| sym-4b3bc0edf728e8a49312 | `documentation` | function | Returns the documentation held by `ConnectorCapability`. | `src/connector/manifest.rs:34` |
| sym-fdf4039f27c61720c6a2 | `documentation` | function | Returns the documentation held by `ConnectorRequirement`. | `src/connector/manifest.rs:69` |
| sym-7e10b6736a3774e1a045 | `drop` | function | Releases resources owned by `ConnectorSecret`. | `src/connector/configuration.rs:49` |
| sym-da94af93e9fc30978cec | `drop` | function | Releases resources owned by `PlanEdgeReceiver`. | `src/runtime/audio/router.rs:647` |
| sym-7ef8f0f3ed6efc006705 | `drop` | function | Releases resources owned by `GeneratedAudioBridge`. | `src/runtime/bridge/audio.rs:193` |
| sym-346774e7be9e63a47d71 | `drop` | function | Releases resources owned by `AsyncRuntimeHost`. | `src/runtime/lifecycle/async_host.rs:105` |
| sym-32a844e5382e088dcfad | `drop` | function | Releases resources owned by `SidecarHost`. | `src/runtime/lifecycle/sidecar_host.rs:521` |
| sym-75267ee39a6db0b853ae | `drop_rate_pct` | function | Returns the drop rate pct held by `EdgeObservations`. | `src/runtime/audio/router.rs:165` |
| sym-10fb748905467401f2f8 | `edge_contract` | function | Returns the edge contract held by `ConnectorInputDescriptor`. | `src/connector/worker/driver.rs:52` |
| sym-0a3df42f064ce049f1df | `edge_id` | function | Returns the edge identifier held by `PlanEdgeReceiver`. | `src/runtime/audio/router.rs:508` |
| sym-84dc2a5a1d2002f1b94c | `encode` | function | Encodes input through `ConnectorConfigurationRecord`. | `src/connector/transport.rs:61` |
| sym-2519efc461a4e280fd1b | `encode` | function | Encodes input through `ConnectorAudioRecord`. | `src/connector/transport.rs:347` |
| sym-2e3dadd8ca135c9e03ab | `encode` | function | Encodes input through `SidecarMessage`. | `src/runtime/lifecycle/sidecar_protocol.rs:86` |
| sym-6ca9531125094143aa37 | `endpoint_id` | function | Returns the endpoint identifier held by `ConnectorInputDescriptor`. | `src/connector/worker/driver.rs:28` |
| sym-267630205817f07547fe | `endpoint_observations` | function | Returns the endpoint observations held by `ConnectorContext`. | `src/connector/worker/coordination.rs:139` |
| sym-55a49f2fc0acfe356a28 | `execute` | function | Executes its owned operation for `AsyncRuntimeHost`. | `src/runtime/lifecycle/async_host.rs:65` |
| sym-978803573e99b67ca948 | `execute_from` | function | Executes from for `RealtimePlanExecutor`. | `src/runtime/audio/executor.rs:149` |
| sym-fc869d5bdd95e2de6ca5 | `expose_secret` | function | Exposes the secret to the owning connector during setup or worker use. | `src/connector/configuration.rs:37` |
| sym-e6c0ce7d51dfcbaf96e4 | `failure` | function | Returns the failure held by `ConnectorRunOutcome`. | `src/connector/worker/mod.rs:55` |
| sym-b603c77500103cbf4489 | `failure_threshold` | function | Returns the failure threshold held by `ConnectorReadinessPolicy`. | `src/connector/readiness.rs:55` |
| sym-2145392fb082e13eb627 | `field` | function | Returns the field held by `ConnectorConfigurationSchema`. | `src/connector/configuration.rs:255` |
| sym-fc4c5f29bbc17d42d8db | `field` | function | Returns the field held by `ConnectorConfigurationError`. | `src/connector/configuration.rs:619` |
| sym-5b8b978acff4848278a9 | `fields` | function | Returns the fields held by `ConnectorConfigurationSchema`. | `src/connector/configuration.rs:251` |
| sym-1c3150cdcd10e04cc1d2 | `finish` | function | Finishes work owned by `RealtimePlanRunner`. | `src/runtime/audio/runner.rs:359` |
| sym-9f81457d2803b2181343 | `finish_and_join` | function | Finishes input to `GeneratedAudioBridge`, joins its worker, and returns the terminal result. | `src/runtime/bridge/audio.rs:178` |
| sym-73129516fd04e3f04d43 | `finish_and_join` | function | Finishes input to `AsyncOperatorWorker`, joins its worker, and returns the terminal result. | `src/runtime/signal/operator.rs:933` |
| sym-fb998d5e97976d9cf219 | `fmt` | function | Formats `ConnectorSecret` with the requested formatter. | `src/connector/configuration.rs:43` |
| sym-64b09159091bd3b96f5b | `fmt` | function | Formats `ConnectorErrorCode` with the requested formatter. | `src/connector/error.rs:35` |
| sym-e029f0d778ba91592b0f | `fmt` | function | Formats `ConnectorErrorCode` with the requested formatter. | `src/connector/error.rs:44` |
| sym-f1a93945c2a5d0f69de9 | `fmt` | function | Formats `PlanEdgeFrame` with the requested formatter. | `src/runtime/audio/router.rs:100` |
| sym-e0308dc7a8bcf8b82c01 | `frames_attempted_total` | function | Returns the frames attempted total held by `EdgeObservations`. | `src/runtime/audio/router.rs:160` |
| sym-ccae372acdb4065bcaf9 | `frames_captured` | function | Returns the frames captured associated with `SystemOutputTelemetry`. | `src/runtime/nodes.rs:55` |
| sym-ed04cf67bf3449078c30 | `frames_emitted` | function | Returns the frames emitted associated with `SystemOutputTelemetry`. | `src/runtime/nodes.rs:58` |
| sym-19c5d4424aefe07bf723 | `frames_mixed` | function | Returns the frames mixed associated with `MixerTelemetry`. | `src/runtime/nodes.rs:254` |
| sym-ac35a0c1639c349ca85b | `frames_pushed` | function | Returns the frames pushed associated with `BridgeSinkTelemetry`. | `src/runtime/nodes.rs:162` |
| sym-08222e995b79ce013f0a | `from` | function | Converts the supplied value into `PlanEdgeReceiver`. | `src/runtime/audio/router.rs:513` |
| sym-6fa3e8d79c890c7717ff | `from_item` | function | Creates `ConnectorAudioRecord` from item. | `src/connector/transport.rs:314` |
| sym-5d13de0df7ad71b668dd | `from_node` | function | Creates `ExecError` from node. | `src/runtime/audio/executor.rs:26` |
| sym-33b9ee5cdb6059b6e5f9 | `from_resolved` | function | Creates `ConnectorConfigurationRecord` from resolved. | `src/connector/transport.rs:45` |
| sym-a8f5198e79b286269788 | `get` | function | Returns the value held by `ConnectorConfiguration`. | `src/connector/configuration.rs:134` |
| sym-4d9bc8528c0202d5425f | `get` | function | Returns the value held by `ResolvedConnectorConfiguration`. | `src/connector/configuration.rs:394` |
| sym-8540ee158dc2da2131fc | `health` | function | Returns the health held by `ConnectorServiceStatus`. | `src/connector/status.rs:46` |
| sym-8e222ad1c1c59e1ccb04 | `health_reason_code` | function | Returns the health reason code held by `ConnectorServiceStatus`. | `src/connector/status.rs:58` |
| sym-59a056aeb32ea6af5817 | `id` | function | Returns the id held by `ConnectorCapability`. | `src/connector/manifest.rs:30` |
| sym-d1a311cfb7fb6b0beb5a | `id` | function | Returns the id held by `ConnectorRequirement`. | `src/connector/manifest.rs:61` |
| sym-176f01e820896ed91ecd | `id` | function | Returns the id held by `SidecarHost`. | `src/runtime/lifecycle/sidecar_host.rs:253` |
| sym-126179a82c1dac44596e | `input` | function | Returns the input held by `ConnectorItem`. | `src/connector/worker/driver.rs:74` |
| sym-ed19944ab65c48c7ca45 | `input_mut` | function | Returns the input mut associated with `AsyncOperatorWorker`. | `src/runtime/signal/operator.rs:917` |
| sym-3625c2cf36ae58dfe77c | `insert` | function | Inserts a typed configuration value into `ConnectorConfiguration`. | `src/connector/configuration.rs:126` |
| sym-55f54ca7b33bd5a63b62 | `instantiate` | function | Instantiates the runtime node described by `SystemOutputSourceFactory`. | `src/runtime/nodes.rs:103` |
| sym-90fcec8e9310c96f2ab4 | `instantiate` | function | Instantiates the runtime node described by `BridgeSinkFactory`. | `src/runtime/nodes.rs:207` |
| sym-478867138b32cee52cc9 | `into_configuration` | function | Converts `ConnectorConfigurationRecord` into configuration. | `src/connector/transport.rs:57` |
| sym-1adbe724a6a8d4fff4de | `into_endpoint_failure` | function | Converts `ConnectorError` into endpoint failure. | `src/connector/error.rs:125` |
| sym-7f0fcfd19bd6a964571b | `into_rejected` | function | Converts `SignalEdgeSendError` into rejected. | `src/runtime/signal/edge.rs:123` |
| sym-110de2cce94828ffaefe | `is_abandoned` | function | Returns whether abandoned applies to `PlanEdgeReceiver`. | `src/runtime/audio/router.rs:549` |
| sym-b419412d315c2cb0fede | `is_abort_requested` | function | Returns whether abort requested applies to `ConnectorContext`. | `src/connector/worker/coordination.rs:36` |
| sym-3a89509bef9042425729 | `is_empty` | function | Returns whether `ConnectorConfiguration` contains no values. | `src/connector/configuration.rs:146` |
| sym-6b84dae9ee40e790945c | `is_requested` | function | Returns whether requested applies to `PlanRunnerCancellation`. | `src/runtime/audio/runner.rs:104` |
| sym-0ced882e7a0707ac4cf2 | `is_stop_requested` | function | Returns whether stop requested applies to `ConnectorContext`. | `src/connector/worker/coordination.rs:28` |
| sym-b513b191cf132d40afa6 | `iter` | function | Iterates over the values held by `ConnectorConfiguration`. | `src/connector/configuration.rs:138` |
| sym-f8a82a36670eedb24825 | `iter` | function | Iterates over the values held by `ResolvedConnectorConfiguration`. | `src/connector/configuration.rs:398` |
| sym-de42a0012afbd21fb026 | `kind` | function | Returns the kind represented by `ConnectorConfigurationValue`. | `src/connector/configuration.rs:77` |
| sym-b54e214adc9db1ca7010 | `lane_underruns` | function | Returns the lane underruns associated with `MixerTelemetry`. | `src/runtime/nodes.rs:257` |
| sym-c170db02f7c98809c93c | `last_transition_elapsed_ns` | function | Returns the last transition elapsed nanoseconds held by `ConnectorServiceStatus`. | `src/connector/status.rs:70` |
| sym-6a4f6ad61d1cb38db315 | `len` | function | Returns the number of values held by `ConnectorConfiguration`. | `src/connector/configuration.rs:142` |
| sym-d1214ab49fdde708bfa8 | `lineage` | function | Returns the frame lineage carried by `PlanEdgeFrame`. | `src/runtime/audio/router.rs:91` |
| sym-d0ce46423a79b4284e2b | `manifest` | function | Returns the manifest held by `Connector`. | `src/connector/mod.rs:119` |
| sym-17ad3cce17ae0a5d4250 | `manifest` | function | Returns the manifest held by `RegisteredConnector`. | `src/connector/mod.rs:136` |
| sym-d467250e43d30e9cf376 | `manifest_revision` | function | Returns the manifest revision held by `ConnectorManifest`. | `src/connector/manifest.rs:128` |
| sym-fe0871cf3d73d9d0e989 | `mark_discontinuity` | function | Marks the next value from `PlanEdgeReceiver` as discontinuous. | `src/runtime/audio/router.rs:630` |
| sym-de48a84bcae85e82ba79 | `mark_worker_failure` | function | Returns the mark worker failure held by `PlanEdgeReceiver`. | `src/runtime/audio/router.rs:639` |
| sym-978e99c20c1d0e860bd3 | `max_frame_bytes` | function | Returns the max frame bytes held by `SidecarProtocolLimits`. | `src/runtime/lifecycle/sidecar_protocol.rs:62` |
| sym-df8acba32de33eb8a03f | `media` | function | Returns the media held by `ConnectorInputDescriptor`. | `src/connector/worker/driver.rs:48` |
| sym-e7176215446668955472 | `message` | function | Returns the diagnostic message reported by `ConnectorConfigurationError`. | `src/connector/configuration.rs:623` |
| sym-778a0382ff61f0b25a61 | `message` | function | Returns the diagnostic message reported by `ConnectorError`. | `src/connector/error.rs:121` |
| sym-28ab778ccae2b9600465 | `metadata` | function | Returns the metadata held by `ConnectorAudioRecord`. | `src/connector/transport.rs:339` |
| sym-26594b415b62661d0b08 | `name` | function | Returns the name associated with `ConnectorConfigurationField`. | `src/connector/configuration.rs:206` |
| sym-4fef42d34554948a377e | `new` | function | Creates a new `ConnectorSecret`. | `src/connector/configuration.rs:14` |
| sym-5dbcd2b0492b57d2caec | `new` | function | Creates a new `ConnectorConfiguration`. | `src/connector/configuration.rs:116` |
| sym-786ec69aa13676716979 | `new` | function | Creates a new `ConnectorConfigurationField`. | `src/connector/configuration.rs:178` |
| sym-1e568ce463c9de9f0e2d | `new` | function | Creates a new `ConnectorConfigurationSchema`. | `src/connector/configuration.rs:238` |
| sym-c0bae520b6f06db19419 | `new` | function | Creates a new `ConnectorErrorCode`. | `src/connector/error.rs:13` |
| sym-62c253f40c872cbb25ee | `new` | function | Creates a new `ConnectorError`. | `src/connector/error.rs:88` |
| sym-c84159d464d7be992972 | `new` | function | Creates a new `ConnectorCapability`. | `src/connector/manifest.rs:18` |
| sym-8042d6c3c5c65953b059 | `new` | function | Creates a new `ConnectorRequirement`. | `src/connector/manifest.rs:47` |
| sym-d7356bbfbee47c85b556 | `new` | function | Creates a new `ConnectorManifest`. | `src/connector/manifest.rs:89` |
| sym-d9a8bf8a72a0470f7292 | `new` | function | Creates a new `Connector`. | `src/connector/mod.rs:68` |
| sym-a5b0fc5d8c9ad242d447 | `new` | function | Creates a new `ConnectorReadinessPolicy`. | `src/connector/readiness.rs:15` |
| sym-63acba5d8a6034750b2b | `new` | function | Creates a new `SidecarConnectorDriverFactory`. | `src/connector/sidecar.rs:29` |
| sym-4a2e863d40d6095c241f | `new` | function | Creates a new `ConnectorRunOutcome`. | `src/connector/worker/mod.rs:47` |
| sym-21e2ca99159d1baa9353 | `new` | function | Creates a new `RealtimePlanExecutor`. | `src/runtime/audio/executor.rs:62` |
| sym-eb0fac256b17a095cd18 | `new` | function | Creates a new `PlanEdgeRouter`. | `src/runtime/audio/router.rs:680` |
| sym-15c9d2f4a353218916de | `new` | function | Creates a new `PlanRunnerCancellation`. | `src/runtime/audio/runner.rs:94` |
| sym-58471ad992ab2a02b94d | `new` | function | Creates a new `RealtimePlanRunner`. | `src/runtime/audio/runner.rs:314` |
| sym-74526cc19c9485813592 | `new` | function | Creates a new `AsyncRuntimeHost`. | `src/runtime/lifecycle/async_host.rs:33` |
| sym-c4f65bbc78887a2b7f02 | `new` | function | Creates a new `SidecarProcessSpec`. | `src/runtime/lifecycle/sidecar_host.rs:82` |
| sym-04bd1b40282161c0a6dc | `new` | function | Creates a new `SystemOutputSourceFactory`. | `src/runtime/nodes.rs:72` |
| sym-ca663a1fc727a4b84429 | `new` | function | Creates a new `BridgeSinkFactory`. | `src/runtime/nodes.rs:176` |
| sym-9f5ec8061e1c62168121 | `new` | function | Creates a new `TypedEdgeFanout`. | `src/runtime/signal/edge.rs:264` |
| sym-d3bd67f80d05f51d19dc | `new_with_output_channels` | function | Creates `MixerSourceNode` with the supplied output channels. | `src/runtime/nodes.rs:280` |
| sym-41c19799e4db0d0dd702 | `node` | function | Returns the node held by `ConnectorManifest`. | `src/connector/manifest.rs:140` |
| sym-5f51988bb89a156b5ab1 | `observation` | function | Returns the current observation exposed by `RegisteredConnector`. | `src/connector/mod.rs:140` |
| sym-61e912b7cc5a34ad7d15 | `observation_handle` | function | Returns a read-only handle to this edge's authoritative live telemetry. | `src/runtime/audio/router.rs:526` |
| sym-c2e24bdeaa8112fdb1e9 | `observation_handle` | function | Returns a handle for reading observations from `PlanSourceSender`. | `src/runtime/audio/runner.rs:181` |
| sym-0952d710b28badc9600c | `observations` | function | Returns the observations exposed by `RegisteredConnector`. | `src/connector/mod.rs:153` |
| sym-907061b242fbf241320d | `observations` | function | Returns the observations exposed by `RealtimePlanExecutor`. | `src/runtime/audio/executor.rs:188` |
| sym-252bc04e689b888ace9b | `observations` | function | Returns a point-in-time snapshot of the edge's live observations. | `src/runtime/audio/router.rs:217` |
| sym-5b26179db3eb4f409e28 | `observations` | function | Returns the observations exposed by `PlanEdgeReceiver`. | `src/runtime/audio/router.rs:626` |
| sym-85045357c2c8bda7180f | `observations` | function | Returns the observations exposed by `PlanEdgeRouter`. | `src/runtime/audio/router.rs:852` |
| sym-2f6c167a8568aa579cd9 | `observations` | function | Returns the observations exposed by `PlanSourceObservationHandle`. | `src/runtime/audio/runner.rs:143` |
| sym-5ee03bf0844b79d1404b | `observations` | function | Returns the observations exposed by `PlanSourceSender`. | `src/runtime/audio/runner.rs:177` |
| sym-032d40321fea55178703 | `observations` | function | Returns the observations exposed by `PlanSourceInput`. | `src/runtime/audio/runner.rs:200` |
| sym-dc7f94d7dc77d5e39322 | `observations` | function | Returns the observations exposed by `SidecarHost`. | `src/runtime/lifecycle/sidecar_host.rs:261` |
| sym-94219cd0c936dc73bfab | `observations` | function | Returns the observations exposed by `AsyncOperatorWorker`. | `src/runtime/signal/operator.rs:921` |
| sym-2cc9104058399195bb19 | `operator_id` | function | Returns the operator identifier held by `ConnectorManifest`. | `src/connector/manifest.rs:132` |
| sym-45a40b5ea1885461ee97 | `output_pool_exhaustions` | function | Returns the output pool exhaustions associated with `MixerTelemetry`. | `src/runtime/nodes.rs:263` |
| sym-fce088a006f3e399abdc | `overrun_count` | function | Returns the overrun count held by `BridgeSinkTelemetry`. | `src/runtime/nodes.rs:165` |
| sym-0f6c33d9765cf812be60 | `package_version` | function | Returns the package version held by `ConnectorManifest`. | `src/connector/manifest.rs:136` |
| sym-5e4327b53963a537d944 | `pocketstation::connector::sidecar::sidecar_connector_factory` | function | Creates a connector driver factory backed by the supplied sidecar process. | `src/connector/sidecar.rs:264` |
| sym-09c4a31af53a881d75c2 | `pocketstation::runtime::audio::runner::plan_source_channel` | function | Plans source channel for `runner`. | `src/runtime/audio/runner.rs:229` |
| sym-84b89e94dd3cc15a54f5 | `pocketstation::runtime::nodes::register_runtime_nodes` | function | Registers runtime nodes for `nodes`. | `src/runtime/nodes.rs:43` |
| sym-6fe2c67fee8fd8d4039e | `port_name` | function | Returns the port name held by `ConnectorAudioRecord`. | `src/connector/transport.rs:335` |
| sym-b5e5c0c3e0b0d5c896a4 | `port_name` | function | Returns the port name held by `ConnectorInputDescriptor`. | `src/connector/worker/driver.rs:40` |
| sym-f8d30689c2c6840b3836 | `preparation_group` | function | Returns the preparation group associated with `SidecarConnectorDriverFactory`. | `src/connector/sidecar.rs:39` |
| sym-40b458a6269e8e0c2262 | `prepare` | function | Prepares resources required by `SidecarConnectorDriverFactory`. | `src/connector/sidecar.rs:49` |
| sym-e10a87f8168ef6956cba | `prepare` | function | Prepares resources required by `MixerSourceNode`. | `src/runtime/nodes.rs:432` |
| sym-79539f26eb4055bb36aa | `prepare_and_spawn_from_plan_edge` | function | Prepares and spawn from plan edge for `AsyncOperatorWorker`. | `src/runtime/signal/operator.rs:764` |
| sym-f2c2a0f54a85d4eebaf3 | `probe_interval` | function | Returns the probe interval held by `ConnectorReadinessPolicy`. | `src/connector/readiness.rs:47` |
| sym-0f013d2539513ea3dc69 | `process` | function | Processes an input value through `SidecarConnectorDriverFactory`. | `src/connector/sidecar.rs:33` |
| sym-c4cc2cba811b43f3cb33 | `process` | function | Processes an input value through `MixerSourceNode`. | `src/runtime/nodes.rs:441` |
| sym-f30ce2949796d2affa10 | `process_ready` | function | Processes the ready inputs for `RealtimePlanRunner`. | `src/runtime/audio/runner.rs:338` |
| sym-5ba1b3010d14039efdc2 | `publish` | function | Publishes its owned operation for `TypedEdgeFanout`. | `src/runtime/signal/edge.rs:308` |
| sym-fb62d7a4401cec2a8906 | `readiness` | function | Returns the readiness held by `ConnectorManifest`. | `src/connector/manifest.rs:148` |
| sym-c04c490781b939e286b3 | `readiness_reason_code` | function | Returns the readiness reason code held by `ConnectorServiceStatus`. | `src/connector/status.rs:54` |
| sym-576cfa1deee2ac13a9b9 | `receive_signal` | function | Receives signal for `SidecarHost`. | `src/runtime/lifecycle/sidecar_host.rs:307` |
| sym-be1a636293abde806859 | `record_discontinuity` | function | Records discontinuity for `ConnectorContext`. | `src/connector/worker/coordination.rs:126` |
| sym-1b530434b2a4e6bdbe95 | `record_failure` | function | Records failure for `ConnectorContext`. | `src/connector/worker/coordination.rs:134` |
| sym-deb447a53e3a1cf34a98 | `record_frame_delivered` | function | Records frame delivered for `ConnectorContext`. | `src/connector/worker/coordination.rs:118` |
| sym-b5fa937a483422151652 | `record_frame_dropped` | function | Records frame dropped for `ConnectorContext`. | `src/connector/worker/coordination.rs:122` |
| sym-19d177974704b689d231 | `record_frame_received` | function | Records frame received for `ConnectorContext`. | `src/connector/worker/coordination.rs:114` |
| sym-5543bb8d0f9cccb93092 | `record_retry` | function | Records retry for `ConnectorContext`. | `src/connector/worker/coordination.rs:130` |
| sym-5f1c1862b0746339d0c9 | `recovery` | function | Returns the recovery held by `ConnectorServiceStatus`. | `src/connector/status.rs:50` |
| sym-63ed6756a1423a186c9b | `recovery_reason_code` | function | Returns the recovery reason code held by `ConnectorServiceStatus`. | `src/connector/status.rs:62` |
| sym-3499d87b1f471c1e5679 | `register_connector` | function | Registers connector for `Session`. | `src/connector/mod.rs:204` |
| sym-870f7d2bc291a855e4a7 | `report_readiness_failure` | function | Returns the report readiness failure held by `ConnectorContext`. | `src/connector/worker/coordination.rs:97` |
| sym-c3518cf53f18f24be63b | `report_readiness_success` | function | Records a successful readiness probe for `ConnectorContext`. | `src/connector/worker/coordination.rs:80` |
| sym-549f138bef7f356d45e7 | `request` | function | Requests the state transition represented by `PlanRunnerCancellation`. | `src/runtime/audio/runner.rs:100` |
| sym-a69045a3fb6aaed54980 | `required` | function | Returns the required held by `ConnectorRequirement`. | `src/connector/manifest.rs:65` |
| sym-3991bd370414c1a6bf57 | `requirement` | function | Returns the requirement held by `ConnectorConfigurationField`. | `src/connector/configuration.rs:214` |
| sym-1578e0c96d33ae028176 | `requirements` | function | Returns the requirements held by `ConnectorManifest`. | `src/connector/manifest.rs:156` |
| sym-9ff5d39a59fcef0a980f | `resolve` | function | Resolves `ConnectorConfigurationSchema` into its validated representation. | `src/connector/configuration.rs:259` |
| sym-c227984556b300940fc1 | `result` | function | Returns the result represented by `ConnectorRunOutcome`. | `src/connector/worker/mod.rs:59` |
| sym-b033b0817db4909e396c | `retryability` | function | Returns the retryability associated with `ConnectorError`. | `src/connector/error.rs:117` |
| sym-045acf7e73303738579f | `revision` | function | Returns the revision held by `ConnectorConfigurationSchema`. | `src/connector/configuration.rs:247` |
| sym-d2a03fe25ee3733863b5 | `revision` | function | Returns the revision held by `ConnectorServiceStatus`. | `src/connector/status.rs:66` |
| sym-aebbad9605773e0d6979 | `route_id` | function | Returns the route identifier held by `ConnectorInputDescriptor`. | `src/connector/worker/driver.rs:36` |
| sym-ecdca2eb323722da79ba | `sample_format` | function | Returns the sample format associated with `PlanEdgeFrame`. | `src/runtime/audio/router.rs:77` |
| sym-54871e067d253d625643 | `sample_rate_hz` | function | Returns the sample rate hertz held by `PlanEdgeFrame`. | `src/runtime/audio/router.rs:63` |
| sym-548ea228160fc75d2bad | `samples` | function | Returns the audio samples held by `ConnectorAudioRecord`. | `src/connector/transport.rs:343` |
| sym-d11b2cc16cb95f091f6e | `samples` | function | Returns the audio samples held by `PlanEdgeFrame`. | `src/runtime/audio/router.rs:84` |
| sym-b257850baa8119ad494f | `send` | function | Sends a value through `AsyncOperatorInput`. | `src/runtime/signal/io.rs:39` |
| sym-5a7647d65531f376b26a | `send_audio` | function | Sends audio for `AsyncOperatorInput`. | `src/runtime/signal/io.rs:55` |
| sym-84cef5b6533963f55094 | `sequence_number` | function | Returns the sequence number held by `PlanEdgeFrame`. | `src/runtime/audio/router.rs:49` |
| sym-85cdd76ca50eb5baf768 | `session_id` | function | Returns the session identifier held by `RegisteredConnector`. | `src/connector/mod.rs:132` |
| sym-63c54342848999d62803 | `set_connected` | function | Sets the connected used by `ConnectorContext`. | `src/connector/worker/coordination.rs:74` |
| sym-cf48b27584a245401247 | `set_degraded` | function | Sets the degraded used by `ConnectorContext`. | `src/connector/worker/coordination.rs:56` |
| sym-740bbd73e15643480eb9 | `set_healthy` | function | Sets the healthy used by `ConnectorContext`. | `src/connector/worker/coordination.rs:62` |
| sym-2b8d78ff36bb186d1d53 | `set_not_ready` | function | Sets the not ready used by `ConnectorContext`. | `src/connector/worker/coordination.rs:50` |
| sym-5319ff2f9f98c58f7c49 | `set_ready` | function | Sets the ready used by `ConnectorContext`. | `src/connector/worker/coordination.rs:44` |
| sym-2f689a4c0a801493252f | `set_reconnecting` | function | Sets the reconnecting used by `ConnectorContext`. | `src/connector/worker/coordination.rs:68` |
| sym-6446238cf23d54b235da | `shutdown` | function | Shuts down `AsyncRuntimeHost` according to its lifecycle contract. | `src/runtime/lifecycle/async_host.rs:91` |
| sym-4a0b2aae18dbc0be8d91 | `shutdown_mode` | function | Returns the shutdown mode held by `ConnectorContext`. | `src/connector/worker/coordination.rs:32` |
| sym-901266c8f4f31ab58f4f | `sidecar` | function | Builds an outbound Connector backed by one bounded sidecar process. | `src/connector/mod.rs:112` |
| sym-63a584618047103b17fb | `signal_spec` | function | Returns the signal spec held by `ConnectorInputDescriptor`. | `src/connector/worker/driver.rs:44` |
| sym-73ace5aff979ab1a8403 | `snapshot` | function | Returns a point-in-time snapshot of `ConnectorObservationHandle`. | `src/connector/observations.rs:53` |
| sym-5978b98570fa78720e42 | `snapshot` | function | Returns a point-in-time snapshot of `AsyncOperatorObservationHandle`. | `src/runtime/signal/observations.rs:52` |
| sym-b7cc2b54a01a8c5180c8 | `source_id` | function | Returns the source identifier held by `PlanEdgeFrame`. | `src/runtime/audio/router.rs:42` |
| sym-3959f42c65362a5408e5 | `source_node_id` | function | Returns the source node identifier held by `PlanSourceInput`. | `src/runtime/audio/runner.rs:196` |
| sym-1e39f60c2fc9cef89efe | `source_observations` | function | Returns the source observations held by `RealtimePlanRunner`. | `src/runtime/audio/runner.rs:349` |
| sym-5a06df3383e632a7cbf8 | `spawn` | function | Spawns its owned operation for `GeneratedAudioBridge`. | `src/runtime/bridge/audio.rs:131` |
| sym-4eafd5e007133d46792e | `spawn` | function | Spawns its owned operation for `SidecarHost`. | `src/runtime/lifecycle/sidecar_host.rs:174` |
| sym-e15df2672ade067b190c | `spawn` | function | Spawns its owned operation for `AsyncOperatorWorker`. | `src/runtime/signal/operator.rs:536` |
| sym-25dc31c898dc120339e8 | `spawn_composed` | function | Spawns composed for `AsyncOperatorWorker`. | `src/runtime/signal/operator.rs:604` |
| sym-110df16dbd7916614c33 | `spawn_with_context` | function | Starts a directly-fed worker with an already negotiated signal-shaped prepare context. Session-owned graph execution uses the compiled-edge path; this entry point exists for external harnesses that negotiate the boundary before constructing a full Session. | `src/runtime/signal/operator.rs:563` |
| sym-a2f7e8afbcf6f5834845 | `stage` | function | Returns the stage held by `ConnectorError`. | `src/connector/error.rs:113` |
| sym-3157359ad4c3cebcd1b3 | `startup_timeout` | function | Returns the startup timeout held by `ConnectorReadinessPolicy`. | `src/connector/readiness.rs:43` |
| sym-445061b1cdeabb0d84fe | `state` | function | Returns the state associated with `SidecarHost`. | `src/runtime/lifecycle/sidecar_host.rs:257` |
| sym-81b88379be3692cd9338 | `stream_id` | function | Returns the stream identifier held by `PlanEdgeFrame`. | `src/runtime/audio/router.rs:35` |
| sym-5a43501bd451af89b2a4 | `success` | function | Returns whether `ConnectorRunOutcome` completed successfully. | `src/connector/worker/mod.rs:51` |
| sym-59b3a78e38624726ca27 | `success_threshold` | function | Returns the success threshold held by `ConnectorReadinessPolicy`. | `src/connector/readiness.rs:51` |
| sym-ef971d108c38935f8e73 | `timestamp_ns` | function | Returns the timestamp nanoseconds held by `PlanEdgeFrame`. | `src/runtime/audio/router.rs:56` |
| sym-23cd1cb19f75e8bd9259 | `to` | function | Returns the destination owned by `PlanEdgeReceiver`. | `src/runtime/audio/router.rs:517` |
| sym-a8af356094c5c830f747 | `try_from` | function | Attempts to from through `SidecarMessageKind`. | `src/runtime/lifecycle/sidecar_protocol.rs:25` |
| sym-3061c1a5028d4292b75d | `try_new` | function | Creates a new `ConnectorAudioRecord` after validating its inputs. | `src/connector/transport.rs:300` |
| sym-50471b407d97976ae0c3 | `try_receive_signal` | function | Attempts to receive signal through `SidecarHost`. | `src/runtime/lifecycle/sidecar_host.rs:299` |
| sym-bac8dedd23fdc447cb38 | `try_recv` | function | Pops one queued frame before sampling the canonical process clock. | `src/runtime/audio/router.rs:545` |
| sym-47daeaf065cae2bd061e | `try_recv_for_testing` | function | Attempts to recv for testing through `PlanSourceInput`. | `src/runtime/audio/runner.rs:213` |
| sym-90aa02ae9f2c137483c3 | `try_send` | function | Attempts to send a value through `PlanSourceSender` without waiting for capacity. | `src/runtime/audio/runner.rs:149` |
| sym-358bf685d21afad964ea | `try_send_signal` | function | Attempts to send signal through `SidecarHost`. | `src/runtime/lifecycle/sidecar_host.rs:265` |
| sym-ef0d835f0a5d199e2f26 | `underrun_count` | function | Returns the underrun count held by `SystemOutputTelemetry`. | `src/runtime/nodes.rs:61` |

## Interpretation

The **Sidecar protocol** inventory records compiler-visible or extracted evidence at the frozen snapshot. A field marked unknown or not declared remains outside the published guarantee; use the native signature, owning error type, and cited test before relying on panic, blocking, cancellation, ordering, limits, retry, or recovery behavior.

## Related documentation

- [Glossary](/docs/glossary.md)
- [PocketStation](/README.md)
- [Sidecar lifecycle](/docs/concepts/sidecars.md)
- [Host a managed-process sidecar](/docs/how-to/host-sidecar.md)
- [Behavior evidence index](/docs/reference/behavior-evidence.md)
- [Configuration reference](/docs/reference/configuration.md)
- [Lifecycle evidence index](/docs/reference/lifecycle-evidence.md)
- [Protocol surface index](/docs/reference/protocol-surface.md)

## Evidence boundary

The claims on **Sidecar protocol** are anchored to Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `src/runtime/lifecycle/sidecar_protocol.rs:1-374` (`DIRECT`)

For **Sidecar protocol**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.
