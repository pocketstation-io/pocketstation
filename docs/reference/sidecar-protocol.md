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
| sym-f5882a0a4b9983601b78 | `Error` | assoc_type | Specifies the error type returned by `SidecarMessageKind` operations. | `src/runtime/lifecycle/sidecar_protocol.rs:23` |
| sym-70666075f09fa7e8e1a4 | `pocketstation::connector::configuration::MAX_CONNECTOR_CONFIGURATION_FIELDS` | constant | Sets the maximum supported connector configuration fields. | `src/connector/configuration.rs:7` |
| sym-4d9191cba5863424f902 | `pocketstation::connector::configuration::MAX_CONNECTOR_CONFIGURATION_TEXT_BYTES` | constant | Sets the maximum supported connector configuration text bytes. | `src/connector/configuration.rs:8` |
| sym-924e23cbd531280ca555 | `pocketstation::connector::error::MAX_CONNECTOR_ERROR_CODE_BYTES` | constant | Sets the maximum supported connector error code bytes. | `src/connector/error.rs:6` |
| sym-9e444653483fa4f02fa0 | `pocketstation::connector::error::MAX_CONNECTOR_ERROR_MESSAGE_BYTES` | constant | Sets the maximum supported connector error message bytes. | `src/connector/error.rs:7` |
| sym-757c38dcb30074da8581 | `pocketstation::connector::manifest::CONNECTOR_API_REVISION` | constant | Defines the public connector API revision value. | `src/connector/manifest.rs:7` |
| sym-f0d13b3a5e8fa03517d4 | `pocketstation::connector::manifest::MAX_CONNECTOR_MANIFEST_ENTRIES` | constant | Sets the maximum supported connector manifest entries. | `src/connector/manifest.rs:8` |
| sym-8c599603d4781320e76d | `pocketstation::connector::manifest::MAX_CONNECTOR_MANIFEST_TEXT_BYTES` | constant | Sets the maximum supported connector manifest text bytes. | `src/connector/manifest.rs:9` |
| sym-5c81b5fa9bc04f842d85 | `pocketstation::connector::readiness::MAX_CONNECTOR_READINESS_THRESHOLD` | constant | Sets the maximum supported connector readiness threshold. | `src/connector/readiness.rs:3` |
| sym-aec7604fe15b924867aa | `pocketstation::connector::readiness::MAX_CONNECTOR_READINESS_TIMEOUT` | constant | Sets the maximum supported connector readiness timeout. | `src/connector/readiness.rs:4` |
| sym-422d9d058d7efb3d7df3 | `pocketstation::connector::sidecar::CONNECTOR_AUDIO_RECORD_SCHEMA` | constant | Defines the public connector audio record schema value. | `src/connector/sidecar.rs:16` |
| sym-ebefb51273c068a49190 | `pocketstation::connector::sidecar::CONNECTOR_AUDIO_RECORD_SIGNAL_ID` | constant | Defines the public connector audio record signal identifier value. | `src/connector/sidecar.rs:15` |
| sym-49c977d37090f05fdc4f | `pocketstation::connector::transport::CONNECTOR_AUDIO_RECORD_MAJOR` | constant | Defines the major version of connector audio record. | `src/connector/transport.rs:19` |
| sym-8df8d2d405c7a4341c3b | `pocketstation::connector::transport::CONNECTOR_AUDIO_RECORD_MINOR` | constant | Defines the minor version of connector audio record. | `src/connector/transport.rs:20` |
| sym-d5023e4750724b6eb269 | `pocketstation::connector::transport::CONNECTOR_CONFIGURATION_RECORD_MAJOR` | constant | Defines the major version of connector configuration record. | `src/connector/transport.rs:35` |
| sym-3623310c7f84b4a816da | `pocketstation::connector::transport::CONNECTOR_CONFIGURATION_RECORD_MINOR` | constant | Defines the minor version of connector configuration record. | `src/connector/transport.rs:36` |
| sym-8fa3ab71f953b0df2d48 | `pocketstation::connector::transport::MAX_CONNECTOR_AUDIO_RECORD_PORT_BYTES` | constant | Sets the maximum supported connector audio record port bytes. | `src/connector/transport.rs:21` |
| sym-267083f78abe7e48cdc2 | `pocketstation::connector::transport::MAX_CONNECTOR_AUDIO_RECORD_SAMPLES` | constant | Sets the maximum supported connector audio record samples. | `src/connector/transport.rs:22` |
| sym-8dbc9bd88b1575dd438d | `pocketstation::connector::ConnectorDeclarationError` | enum | Classifies failures reported as connector declaration error. | `src/connector/mod.rs:233` |
| sym-a78af2f1f714d50e5fb2 | `pocketstation::connector::ConnectorObservationLookupError` | enum | Classifies failures reported as connector observation lookup error. | `src/connector/mod.rs:246` |
| sym-c609c374fa8e6866eeee | `pocketstation::connector::ConnectorRegistrationError` | enum | Classifies failures reported as connector registration error. | `src/connector/mod.rs:225` |
| sym-13750a98063d811df25f | `pocketstation::connector::configuration::ConnectorConfigurationConstraint` | enum | Enumerates the supported connector configuration constraint cases. | `src/connector/configuration.rs:159` |
| sym-4c8ab28f0ba8c890be55 | `pocketstation::connector::configuration::ConnectorConfigurationErrorCode` | enum | Enumerates the supported connector configuration error code cases. | `src/connector/configuration.rs:568` |
| sym-a40cb5d8365f9ea24105 | `pocketstation::connector::configuration::ConnectorConfigurationRequirement` | enum | Selects the connector configuration requirement used by PocketStation. | `src/connector/configuration.rs:152` |
| sym-6966c55cb774b54182be | `pocketstation::connector::configuration::ConnectorConfigurationValue` | enum | Enumerates the supported connector configuration value cases. | `src/connector/configuration.rs:66` |
| sym-80830e3822c869f170f3 | `pocketstation::connector::configuration::ConnectorConfigurationValueKind` | enum | Selects the connector configuration value kind used by PocketStation. | `src/connector/configuration.rs:55` |
| sym-936b382b77b874531096 | `pocketstation::connector::error::ConnectorErrorBuildError` | enum | Classifies failures reported as connector error build error. | `src/connector/error.rs:184` |
| sym-736ab1f377099246ab6c | `pocketstation::connector::error::ConnectorErrorCodeError` | enum | Classifies failures reported as connector error code error. | `src/connector/error.rs:50` |
| sym-a833cb405d6b510ca450 | `pocketstation::connector::error::ConnectorErrorStage` | enum | Selects the connector error stage used by PocketStation. | `src/connector/error.rs:60` |
| sym-47fa722e871287abe9c1 | `pocketstation::connector::error::ConnectorRetryability` | enum | Enumerates the supported connector retryability cases. | `src/connector/error.rs:72` |
| sym-eb9a6bafd07d87b6d706 | `pocketstation::connector::manifest::ConnectorManifestError` | enum | Classifies failures reported as connector manifest error. | `src/connector/manifest.rs:231` |
| sym-a62dc6612ed60bf01537 | `pocketstation::connector::observations::ConnectorObservationError` | enum | Classifies failures reported as connector observation error. | `src/connector/observations.rs:175` |
| sym-0c5d86ed5e47145073e6 | `pocketstation::connector::readiness::ConnectorReadinessPolicyError` | enum | Classifies failures reported as connector readiness policy error. | `src/connector/readiness.rs:61` |
| sym-57a90efe5d877dea505e | `pocketstation::connector::status::ConnectorDeliveryReadiness` | enum | Enumerates the supported connector delivery readiness cases. | `src/connector/status.rs:4` |
| sym-95843fa35c7fe6258767 | `pocketstation::connector::status::ConnectorHealth` | enum | Enumerates the supported connector health cases. | `src/connector/status.rs:17` |
| sym-2e6f8e216a5a39ba1c46 | `pocketstation::connector::status::ConnectorRecovery` | enum | Enumerates the supported connector recovery cases. | `src/connector/status.rs:24` |
| sym-a8e9c6600f9fddcb4ca9 | `pocketstation::connector::transport::ConnectorAudioRecordError` | enum | Classifies failures reported as connector audio record error. | `src/connector/transport.rs:568` |
| sym-26e401bc8ce6686aafa5 | `pocketstation::connector::transport::ConnectorConfigurationRecordError` | enum | Classifies failures reported as connector configuration record error. | `src/connector/transport.rs:251` |
| sym-c61e48a0b277b7e058f2 | `pocketstation::connector::worker::driver::ConnectorDeliveryOutcome` | enum | Explicit delivery result used for Core-owned accounting. | `src/connector/worker/driver.rs:83` |
| sym-5d275693e8d66effdd4e | `pocketstation::connector::worker::driver::ConnectorItem` | enum | One bounded item delivered by Core to a connector driver. | `src/connector/worker/driver.rs:62` |
| sym-bd4cddc024bdfa04c794 | `pocketstation::runtime::audio::executor::ExecError` | enum | Classifies failures reported as exec error. | `src/runtime/audio/executor.rs:20` |
| sym-957ab9d17e2a69e24822 | `pocketstation::runtime::audio::router::PlanEdgeFrame` | enum | Enumerates the supported plan edge frame cases. | `src/runtime/audio/router.rs:29` |
| sym-0f3e073889a85f550e76 | `pocketstation::runtime::audio::router::PlanRouterError` | enum | Classifies failures reported as plan router error. | `src/runtime/audio/router.rs:17` |
| sym-8077ac938f560668fc7d | `pocketstation::runtime::audio::runner::PlanRunnerDrainPolicy` | enum | Selects the plan runner drain policy used by PocketStation. | `src/runtime/audio/runner.rs:16` |
| sym-56de2f851fe07b9e1f22 | `pocketstation::runtime::audio::runner::PlanRunnerError` | enum | Classifies failures reported as plan runner error. | `src/runtime/audio/runner.rs:256` |
| sym-635213208855f982fdd1 | `pocketstation::runtime::audio::runner::PlanSourceSendError` | enum | Classifies failures reported as plan source send error. | `src/runtime/audio/runner.rs:116` |
| sym-48efb656489fafc8bcc9 | `pocketstation::runtime::audio::runner::PlanSourceSendOutcome` | enum | Classifies the observable plan source send outcome. | `src/runtime/audio/runner.rs:123` |
| sym-993f5a91b8567d32c584 | `pocketstation::runtime::bridge::audio::GeneratedAudioBridgeStartError` | enum | Classifies failures reported as generated audio bridge start error. | `src/runtime/bridge/audio.rs:46` |
| sym-71cc7e43e02a571de594 | `pocketstation::runtime::lifecycle::async_host::AsyncRuntimeHostError` | enum | Classifies failures reported as async runtime host error. | `src/runtime/lifecycle/async_host.rs:10` |
| sym-9de7ba3ad4ec47b816ed | `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostError` | enum | Classifies failures reported as sidecar host error. | `src/runtime/lifecycle/sidecar_host.rs:686` |
| sym-92b3ea8c30f09567053c | `pocketstation::runtime::lifecycle::sidecar_host::SidecarState` | enum | Selects the sidecar state used by PocketStation. | `src/runtime/lifecycle/sidecar_host.rs:21` |
| sym-0e7efaeba01d4c9b9389 | `pocketstation::runtime::lifecycle::sidecar_protocol::SidecarMessageKind` | enum | Selects the sidecar message kind used by PocketStation. | `src/runtime/lifecycle/sidecar_protocol.rs:9` |
| sym-87edea404c6ffea054be | `pocketstation::runtime::lifecycle::sidecar_protocol::SidecarProtocolError` | enum | Classifies failures reported as sidecar protocol error. | `src/runtime/lifecycle/sidecar_protocol.rs:292` |
| sym-aaff16dc9808b1b8fbf9 | `pocketstation::runtime::signal::edge::TypedEdgeBuildError` | enum | Classifies failures reported as typed edge build error. | `src/runtime/signal/edge.rs:386` |
| sym-dbb372137f3276f38eb7 | `pocketstation::runtime::signal::edge::TypedEdgePublishError` | enum | Classifies failures reported as typed edge publish error. | `src/runtime/signal/edge.rs:408` |
| sym-d8829db8dadb99f939cb | `pocketstation::runtime::signal::error::AsyncOperatorWorkerError` | enum | Classifies failures reported as async operator worker error. | `src/runtime/signal/error.rs:6` |
| sym-116294235e95d6969503 | `ConnectorDriver::cancel_preparation` | function | Cancels preparation for `ConnectorDriver`. | `src/connector/worker/driver.rs:116` |
| sym-c3743aa2faf26b1ae2ba | `ConnectorDriver::deliver` | function | Delivers the next input through `ConnectorDriver`. | `src/connector/worker/driver.rs:98` |
| sym-a015cd11b00099f1959b | `ConnectorDriver::idle` | function | Advances `ConnectorDriver` while no input is available. | `src/connector/worker/driver.rs:104` |
| sym-be0022e2a56f799fb3e7 | `ConnectorDriver::shutdown` | function | Shuts down `ConnectorDriver` according to its lifecycle contract. | `src/connector/worker/driver.rs:108` |
| sym-72f758a85e8e18ea54e5 | `ConnectorDriver::start` | function | Starts the lifecycle represented by `ConnectorDriver`. | `src/connector/worker/driver.rs:93` |
| sym-e79e0f39714d48b7dde0 | `ConnectorDriverFactory::preparation_group` | function | Returns the preparation group associated with `ConnectorDriverFactory`. | `src/connector/worker/driver.rs:124` |
| sym-f83e889dd44a61772bbe | `ConnectorDriverFactory::prepare` | function | Prepares resources required by `ConnectorDriverFactory`. | `src/connector/worker/driver.rs:132` |
| sym-4825cbede5c6ebfaec8f | `ConnectorFactory::preparation_group` | function | Returns the preparation group associated with `ConnectorFactory`. | `src/connector/worker/mod.rs:18` |
| sym-6015b5e7fee5a0b0c418 | `ConnectorFactory::prepare` | function | Prepares resources required by `ConnectorFactory`. | `src/connector/worker/mod.rs:26` |
| sym-832954574d8962ffafee | `ConnectorWorker::cancel_preparation` | function | Cancels preparation for `ConnectorWorker`. | `src/connector/worker/mod.rs:35` |
| sym-b9c16a875bf523033d20 | `ConnectorWorker::run` | function | Runs `ConnectorWorker` until completion or cancellation. | `src/connector/worker/mod.rs:33` |
| sym-42a8e5c4d0cbc083c502 | `accepts_delivery` | function | Returns whether accepts delivery applies to `ConnectorDeliveryReadiness`. | `src/connector/status.rs:10` |
| sym-8d570ccfcc6f8f22adb8 | `accepts_delivery` | function | Returns whether accepts delivery applies to `ConnectorServiceStatus`. | `src/connector/status.rs:74` |
| sym-e9973a9598777cbaefb0 | `api_revision` | function | Returns the API revision held by `ConnectorManifest`. | `src/connector/manifest.rs:124` |
| sym-ecb602e99c0dba24af4f | `as_str` | function | Returns the stable string representation of `ConnectorConfigurationErrorCode`. | `src/connector/configuration.rs:585` |
| sym-f5caa543ea9c940cafb1 | `as_str` | function | Returns the stable string representation of `ConnectorErrorCode`. | `src/connector/error.rs:29` |
| sym-f638ac422d0b9145e66e | `cancel_and_join` | function | Cancels and join for `GeneratedAudioBridge`. | `src/runtime/bridge/audio.rs:184` |
| sym-009871c0c84118fce16e | `cancel_and_join` | function | Cancels and join for `AsyncOperatorWorker`. | `src/runtime/signal/operator.rs:925` |
| sym-dd1373cfb5ce025352ae | `cancel_and_reap` | function | Cancels and reap for `SidecarHost`. | `src/runtime/lifecycle/sidecar_host.rs:322` |
| sym-4a32dce08606102cb841 | `capabilities` | function | Returns the capabilities held by `ConnectorManifest`. | `src/connector/manifest.rs:152` |
| sym-ebd8e9d86b76a24a1263 | `channels` | function | Returns the channel count represented by `PlanEdgeFrame`. | `src/runtime/audio/router.rs:70` |
| sym-ba5d7f524eb5ffcfd856 | `clipped_samples` | function | Returns the clipped samples held by `MixerTelemetry`. | `src/runtime/nodes.rs:260` |
| sym-6b2818ee70b8000ad99d | `close_and_reap` | function | Closes `SidecarHost` and reaps its child process. | `src/runtime/lifecycle/sidecar_host.rs:326` |
| sym-ac60f8c24954ba5af0f0 | `code` | function | Returns the stable error or status code represented by `ConnectorConfigurationError`. | `src/connector/configuration.rs:615` |
| sym-1b9e9ce53fdd2cf3d6a2 | `code` | function | Returns the stable error or status code represented by `ConnectorError`. | `src/connector/error.rs:109` |
| sym-1e5612a33c09f36fa151 | `configuration` | function | Returns the configuration held by `ConnectorManifest`. | `src/connector/manifest.rs:144` |
| sym-53268b83a683ae6a2898 | `configuration` | function | Returns the configuration held by `ConnectorConfigurationRecord`. | `src/connector/transport.rs:53` |
| sym-ea2aa97e4f6c327d4cc6 | `configuration` | function | Returns the configuration held by `ConnectorInputDescriptor`. | `src/connector/worker/driver.rs:56` |
| sym-5da32ec8a46f62e624fc | `connector_id` | function | Returns the connector identifier held by `ConnectorInputDescriptor`. | `src/connector/worker/driver.rs:32` |
| sym-98c5d4b70984a08dd397 | `constraints` | function | Returns the constraints held by `ConnectorConfigurationField`. | `src/connector/configuration.rs:222` |
| sym-635f185607de8e4f9da1 | `declare` | function | Adds the declaration represented by `RegisteredConnector` to its Session. | `src/connector/mod.rs:159` |
| sym-e3ef3f2426b2ab2e710c | `decode` | function | Decodes input through `ConnectorConfigurationRecord`. | `src/connector/transport.rs:93` |
| sym-1a5304483b6adaa06f83 | `decode` | function | Decodes input through `ConnectorAudioRecord`. | `src/connector/transport.rs:403` |
| sym-7fa68b299cc6aee2a4f2 | `decode` | function | Decodes input through `SidecarMessage`. | `src/runtime/lifecycle/sidecar_protocol.rs:121` |
| sym-8b2c4ad5fa0a4ee19c40 | `default` | function | Returns the default `PlanRunnerCancellation` value. | `src/runtime/audio/runner.rs:110` |
| sym-73f57e4942d4df802c11 | `default` | function | Returns the default `SidecarDeadlines` value. | `src/runtime/lifecycle/sidecar_host.rs:61` |
| sym-f2a677038880c785311e | `default` | function | Returns the default `SidecarProtocolLimits` value. | `src/runtime/lifecycle/sidecar_protocol.rs:51` |
| sym-6c2f846a56c09b494d97 | `delivery_readiness` | function | Returns the delivery readiness associated with `ConnectorServiceStatus`. | `src/connector/status.rs:42` |
| sym-ad4e94586f88eebd3375 | `deprecated` | function | Returns the deprecated held by `ConnectorConfigurationField`. | `src/connector/configuration.rs:201` |
| sym-6b003ff3808eecc732f0 | `deprecation` | function | Returns the deprecation held by `ConnectorConfigurationField`. | `src/connector/configuration.rs:226` |
| sym-848742f534a9b91337d0 | `descriptor` | function | Returns the descriptor associated with `SystemOutputSourceFactory`. | `src/runtime/nodes.rs:87` |
| sym-b933cf7614529f0028d2 | `descriptor` | function | Returns the descriptor associated with `BridgeSinkFactory`. | `src/runtime/nodes.rs:191` |
| sym-eb1aeb591e12bc6b9e4e | `dispatch_from` | function | Routes one lineaged audio frame from the named plan output through `PlanEdgeRouter`. | `src/runtime/audio/router.rs:777` |
| sym-3248ef146d7f14d09bc6 | `documentation` | function | Returns the documentation held by `ConnectorConfigurationField`. | `src/connector/configuration.rs:218` |
| sym-91cdc75380f1b1c45072 | `documentation` | function | Returns the documentation held by `ConnectorCapability`. | `src/connector/manifest.rs:34` |
| sym-34ddc898cf906f7e707f | `documentation` | function | Returns the documentation held by `ConnectorRequirement`. | `src/connector/manifest.rs:69` |
| sym-24451f44bd94778b8d32 | `drop` | function | Releases resources owned by `ConnectorSecret`. | `src/connector/configuration.rs:49` |
| sym-1d8fe4e68f55934db10b | `drop` | function | Releases resources owned by `PlanEdgeReceiver`. | `src/runtime/audio/router.rs:676` |
| sym-2958e60b6265b5fef03f | `drop` | function | Releases resources owned by `GeneratedAudioBridge`. | `src/runtime/bridge/audio.rs:193` |
| sym-bb231d51d6e86fd54874 | `drop` | function | Releases resources owned by `AsyncRuntimeHost`. | `src/runtime/lifecycle/async_host.rs:105` |
| sym-f3a9d500f29837962ac8 | `drop` | function | Releases resources owned by `SidecarHost`. | `src/runtime/lifecycle/sidecar_host.rs:521` |
| sym-17ac8a99af0ede70717f | `drop_rate_pct` | function | Returns the drop rate pct held by `EdgeObservations`. | `src/runtime/audio/router.rs:185` |
| sym-e443085133525c866043 | `edge_contract` | function | Returns the edge contract held by `ConnectorInputDescriptor`. | `src/connector/worker/driver.rs:52` |
| sym-c6c0dece77c2acf50c8f | `edge_id` | function | Returns the edge identifier held by `PlanEdgeReceiver`. | `src/runtime/audio/router.rs:528` |
| sym-c46a34cb2388608cd3b1 | `encode` | function | Encodes input through `ConnectorConfigurationRecord`. | `src/connector/transport.rs:61` |
| sym-08a8a31e4c30a3df7c8e | `encode` | function | Encodes input through `ConnectorAudioRecord`. | `src/connector/transport.rs:347` |
| sym-39e328da4ebd9d04a6a0 | `encode` | function | Encodes input through `SidecarMessage`. | `src/runtime/lifecycle/sidecar_protocol.rs:86` |
| sym-0c5da44a14691ca38d74 | `endpoint_id` | function | Returns the endpoint identifier held by `ConnectorInputDescriptor`. | `src/connector/worker/driver.rs:28` |
| sym-575cacd62461aa3cdc9c | `endpoint_observations` | function | Returns the endpoint observations held by `ConnectorContext`. | `src/connector/worker/coordination.rs:139` |
| sym-dbccbe698db48a061ebe | `execute` | function | Executes its owned operation for `AsyncRuntimeHost`. | `src/runtime/lifecycle/async_host.rs:65` |
| sym-b1658414197792b94c5b | `execute_from` | function | Executes from for `RealtimePlanExecutor`. | `src/runtime/audio/executor.rs:149` |
| sym-b9d0e20afe7dce1858a4 | `expose_secret` | function | Exposes the secret to the owning connector during setup or worker use. | `src/connector/configuration.rs:37` |
| sym-d55e9dc582b914e1a93b | `failure` | function | Returns the failure held by `ConnectorRunOutcome`. | `src/connector/worker/mod.rs:55` |
| sym-56df668f46edc7d9e40d | `failure_threshold` | function | Returns the failure threshold held by `ConnectorReadinessPolicy`. | `src/connector/readiness.rs:55` |
| sym-267259f742b6a1cc1924 | `field` | function | Returns the field held by `ConnectorConfigurationSchema`. | `src/connector/configuration.rs:255` |
| sym-8d0d1e80c210dfa25f85 | `field` | function | Returns the field held by `ConnectorConfigurationError`. | `src/connector/configuration.rs:619` |
| sym-61bd3f48c87a4957932a | `fields` | function | Returns the fields held by `ConnectorConfigurationSchema`. | `src/connector/configuration.rs:251` |
| sym-8e35142f99b058ba0f84 | `finish` | function | Finishes work owned by `RealtimePlanRunner`. | `src/runtime/audio/runner.rs:359` |
| sym-9b1bcdfe87564ca73d82 | `finish_and_join` | function | Finishes input to `GeneratedAudioBridge`, joins its worker, and returns the terminal result. | `src/runtime/bridge/audio.rs:178` |
| sym-6f467d9e1933391df548 | `finish_and_join` | function | Finishes input to `AsyncOperatorWorker`, joins its worker, and returns the terminal result. | `src/runtime/signal/operator.rs:933` |
| sym-cf4630838130fe080faf | `fmt` | function | Formats `ConnectorSecret` with the requested formatter. | `src/connector/configuration.rs:43` |
| sym-269f701717d0abdc7ced | `fmt` | function | Formats `ConnectorErrorCode` with the requested formatter. | `src/connector/error.rs:35` |
| sym-0652c06beea3ce6befb9 | `fmt` | function | Formats `ConnectorErrorCode` with the requested formatter. | `src/connector/error.rs:44` |
| sym-ae4353462736665ac29b | `fmt` | function | Formats `PlanEdgeFrame` with the requested formatter. | `src/runtime/audio/router.rs:100` |
| sym-be67cc8f6802f777a3d7 | `frames_attempted_total` | function | Returns the frames attempted total held by `EdgeObservations`. | `src/runtime/audio/router.rs:180` |
| sym-d1c06126c9b16086f9b9 | `frames_captured` | function | Returns the frames captured associated with `SystemOutputTelemetry`. | `src/runtime/nodes.rs:55` |
| sym-5628415a8d0f45a0bc10 | `frames_emitted` | function | Returns the frames emitted associated with `SystemOutputTelemetry`. | `src/runtime/nodes.rs:58` |
| sym-f6cf84750da0d1dd18fe | `frames_mixed` | function | Returns the frames mixed associated with `MixerTelemetry`. | `src/runtime/nodes.rs:254` |
| sym-50ce6387f249a78d3e81 | `frames_pushed` | function | Returns the frames pushed associated with `BridgeSinkTelemetry`. | `src/runtime/nodes.rs:162` |
| sym-96c56f6eb7a26ad320d2 | `from` | function | Converts the supplied value into `PlanEdgeReceiver`. | `src/runtime/audio/router.rs:533` |
| sym-59196294cd066af3a974 | `from_item` | function | Creates `ConnectorAudioRecord` from item. | `src/connector/transport.rs:314` |
| sym-c0060486788a83e60a1f | `from_node` | function | Creates `ExecError` from node. | `src/runtime/audio/executor.rs:26` |
| sym-216ca51c178e3d2bdb1e | `from_resolved` | function | Creates `ConnectorConfigurationRecord` from resolved. | `src/connector/transport.rs:45` |
| sym-c7c63cd95c4233b5a1ad | `get` | function | Returns the value held by `ConnectorConfiguration`. | `src/connector/configuration.rs:134` |
| sym-1baa6743961e6be9c1ac | `get` | function | Returns the value held by `ResolvedConnectorConfiguration`. | `src/connector/configuration.rs:394` |
| sym-8ee18b8cc0f3b596159d | `health` | function | Returns the health held by `ConnectorServiceStatus`. | `src/connector/status.rs:46` |
| sym-dd37553bb3a13016e289 | `health_reason_code` | function | Returns the health reason code held by `ConnectorServiceStatus`. | `src/connector/status.rs:58` |
| sym-b19bfeb5e328f1d64e7f | `id` | function | Returns the id held by `ConnectorCapability`. | `src/connector/manifest.rs:30` |
| sym-43225e980a364baa9ced | `id` | function | Returns the id held by `ConnectorRequirement`. | `src/connector/manifest.rs:61` |
| sym-ad406a727cd620096863 | `id` | function | Returns the id held by `SidecarHost`. | `src/runtime/lifecycle/sidecar_host.rs:253` |
| sym-803ad52760f815fa63af | `input` | function | Returns the input held by `ConnectorItem`. | `src/connector/worker/driver.rs:74` |
| sym-a6cfc39c2048b414c483 | `input_mut` | function | Returns the input mut associated with `AsyncOperatorWorker`. | `src/runtime/signal/operator.rs:917` |
| sym-fd92f420eac5795ad4f9 | `insert` | function | Inserts a typed configuration value into `ConnectorConfiguration`. | `src/connector/configuration.rs:126` |
| sym-b4b9f4d0abada70447e0 | `instantiate` | function | Instantiates the runtime node described by `SystemOutputSourceFactory`. | `src/runtime/nodes.rs:103` |
| sym-30e204a84afd19dbc60d | `instantiate` | function | Instantiates the runtime node described by `BridgeSinkFactory`. | `src/runtime/nodes.rs:207` |
| sym-7108c905ca4780df696b | `into_configuration` | function | Converts `ConnectorConfigurationRecord` into configuration. | `src/connector/transport.rs:57` |
| sym-b74082477b551771a0a0 | `into_endpoint_failure` | function | Converts `ConnectorError` into endpoint failure. | `src/connector/error.rs:125` |
| sym-64dbb4a8d0ef710f8861 | `into_rejected` | function | Converts `SignalEdgeSendError` into rejected. | `src/runtime/signal/edge.rs:123` |
| sym-d3b35a7e7b9712ddaea7 | `is_abandoned` | function | Returns whether abandoned applies to `PlanEdgeReceiver`. | `src/runtime/audio/router.rs:574` |
| sym-0eefd4234befb8a34c61 | `is_abort_requested` | function | Returns whether abort requested applies to `ConnectorContext`. | `src/connector/worker/coordination.rs:36` |
| sym-104b3bac350417f37f40 | `is_empty` | function | Returns whether `ConnectorConfiguration` contains no values. | `src/connector/configuration.rs:146` |
| sym-4a925d7654caf42d1a2a | `is_requested` | function | Returns whether requested applies to `PlanRunnerCancellation`. | `src/runtime/audio/runner.rs:104` |
| sym-1440b0574deddcb13357 | `is_stop_requested` | function | Returns whether stop requested applies to `ConnectorContext`. | `src/connector/worker/coordination.rs:28` |
| sym-77a3c9fd80fed3c3b18f | `iter` | function | Iterates over the values held by `ConnectorConfiguration`. | `src/connector/configuration.rs:138` |
| sym-d8cf73924fb51e8b2b8e | `iter` | function | Iterates over the values held by `ResolvedConnectorConfiguration`. | `src/connector/configuration.rs:398` |
| sym-e136b77b30dedf88de08 | `kind` | function | Returns the kind represented by `ConnectorConfigurationValue`. | `src/connector/configuration.rs:77` |
| sym-9de1f3176f7cf121db5a | `lane_underruns` | function | Returns the lane underruns associated with `MixerTelemetry`. | `src/runtime/nodes.rs:257` |
| sym-3e3073a9a190630017e1 | `last_transition_elapsed_ns` | function | Returns the last transition elapsed nanoseconds held by `ConnectorServiceStatus`. | `src/connector/status.rs:70` |
| sym-b83c3de0aeafa5c07084 | `len` | function | Returns the number of values held by `ConnectorConfiguration`. | `src/connector/configuration.rs:142` |
| sym-94ff6a80aa2957e7af57 | `lineage` | function | Returns the frame lineage carried by `PlanEdgeFrame`. | `src/runtime/audio/router.rs:91` |
| sym-965aa3b29c0459c9cb66 | `manifest` | function | Returns the manifest held by `Connector`. | `src/connector/mod.rs:119` |
| sym-c5ae77d57d3faef142e8 | `manifest` | function | Returns the manifest held by `RegisteredConnector`. | `src/connector/mod.rs:136` |
| sym-e5b207f73575f5d653dc | `manifest_revision` | function | Returns the manifest revision held by `ConnectorManifest`. | `src/connector/manifest.rs:128` |
| sym-679b7143612407d3c2eb | `mark_discontinuity` | function | Marks the next value from `PlanEdgeReceiver` as discontinuous. | `src/runtime/audio/router.rs:659` |
| sym-fd347baeba2a2565f020 | `mark_worker_failure` | function | Returns the mark worker failure held by `PlanEdgeReceiver`. | `src/runtime/audio/router.rs:668` |
| sym-d3dc8c94b015f5f22897 | `max_frame_bytes` | function | Returns the max frame bytes held by `SidecarProtocolLimits`. | `src/runtime/lifecycle/sidecar_protocol.rs:62` |
| sym-aae44a4fc0ef2c5793f4 | `media` | function | Returns the media held by `ConnectorInputDescriptor`. | `src/connector/worker/driver.rs:48` |
| sym-ab7515a9ee9c63ae46ca | `message` | function | Returns the diagnostic message reported by `ConnectorConfigurationError`. | `src/connector/configuration.rs:623` |
| sym-fa40c22659bda8c12332 | `message` | function | Returns the diagnostic message reported by `ConnectorError`. | `src/connector/error.rs:121` |
| sym-651a18d4220e9dbb7f9a | `metadata` | function | Returns the metadata held by `ConnectorAudioRecord`. | `src/connector/transport.rs:339` |
| sym-c4d51046d48fdc71e04f | `name` | function | Returns the name associated with `ConnectorConfigurationField`. | `src/connector/configuration.rs:206` |
| sym-7b78efb0e24eaa853bb4 | `new` | function | Creates a new `ConnectorSecret`. | `src/connector/configuration.rs:14` |
| sym-2d887b87f4bf2dc6cee5 | `new` | function | Creates a new `ConnectorConfiguration`. | `src/connector/configuration.rs:116` |
| sym-65cfdedb5ad3673dbf76 | `new` | function | Creates a new `ConnectorConfigurationField`. | `src/connector/configuration.rs:178` |
| sym-5b139600e185135acb89 | `new` | function | Creates a new `ConnectorConfigurationSchema`. | `src/connector/configuration.rs:238` |
| sym-894ae7af6a50f8c14a51 | `new` | function | Creates a new `ConnectorErrorCode`. | `src/connector/error.rs:13` |
| sym-7716a10f5d79d6beb91a | `new` | function | Creates a new `ConnectorError`. | `src/connector/error.rs:88` |
| sym-fed04abe587dc97e5d55 | `new` | function | Creates a new `ConnectorCapability`. | `src/connector/manifest.rs:18` |
| sym-9c642af4c593ed6b3501 | `new` | function | Creates a new `ConnectorRequirement`. | `src/connector/manifest.rs:47` |
| sym-0b34ecdede237d02680a | `new` | function | Creates a new `ConnectorManifest`. | `src/connector/manifest.rs:89` |
| sym-68a8d9454f2988870efc | `new` | function | Creates a new `Connector`. | `src/connector/mod.rs:68` |
| sym-9258ebd0258cf3a31e3b | `new` | function | Creates a new `ConnectorReadinessPolicy`. | `src/connector/readiness.rs:15` |
| sym-0d1e72323eefa3cec722 | `new` | function | Creates a new `SidecarConnectorDriverFactory`. | `src/connector/sidecar.rs:29` |
| sym-81c7bb5ffd9a212846b2 | `new` | function | Creates a new `ConnectorRunOutcome`. | `src/connector/worker/mod.rs:47` |
| sym-cb36fc591194cfe17a76 | `new` | function | Creates a new `RealtimePlanExecutor`. | `src/runtime/audio/executor.rs:62` |
| sym-89935ae4f6eb9c47d423 | `new` | function | Creates a new `PlanEdgeRouter`. | `src/runtime/audio/router.rs:709` |
| sym-efe3840761061b373ea8 | `new` | function | Creates a new `PlanRunnerCancellation`. | `src/runtime/audio/runner.rs:94` |
| sym-62cf46ad5c0e2d973940 | `new` | function | Creates a new `RealtimePlanRunner`. | `src/runtime/audio/runner.rs:314` |
| sym-bcc9fc05fc4b41dea325 | `new` | function | Creates a new `AsyncRuntimeHost`. | `src/runtime/lifecycle/async_host.rs:33` |
| sym-417142a74fd6534cd6fb | `new` | function | Creates a new `SidecarProcessSpec`. | `src/runtime/lifecycle/sidecar_host.rs:82` |
| sym-0629581701c5639e034d | `new` | function | Creates a new `SystemOutputSourceFactory`. | `src/runtime/nodes.rs:72` |
| sym-b3414446d83b06b7ed47 | `new` | function | Creates a new `BridgeSinkFactory`. | `src/runtime/nodes.rs:176` |
| sym-621acda29d3c06da0307 | `new` | function | Creates a new `TypedEdgeFanout`. | `src/runtime/signal/edge.rs:264` |
| sym-333a6a244c19b535857c | `new_with_output_channels` | function | Creates `MixerSourceNode` with the supplied output channels. | `src/runtime/nodes.rs:280` |
| sym-f033a669e834bb2886e8 | `node` | function | Returns the node held by `ConnectorManifest`. | `src/connector/manifest.rs:140` |
| sym-b9068cec996a567f13a5 | `observation` | function | Returns the current observation exposed by `RegisteredConnector`. | `src/connector/mod.rs:140` |
| sym-afe19ea26113b4c497b9 | `observation_handle` | function | Returns a read-only handle to this edge's authoritative live telemetry. | `src/runtime/audio/router.rs:546` |
| sym-310408147965a48c816c | `observation_handle` | function | Returns a handle for reading observations from `PlanSourceSender`. | `src/runtime/audio/runner.rs:181` |
| sym-4a2c8867bce1f05a18e1 | `observations` | function | Returns the observations exposed by `RegisteredConnector`. | `src/connector/mod.rs:153` |
| sym-f6c711b3f4fc31ea76e8 | `observations` | function | Returns the observations exposed by `RealtimePlanExecutor`. | `src/runtime/audio/executor.rs:188` |
| sym-fa7f6685167498e25c63 | `observations` | function | Returns a point-in-time snapshot of the edge's live observations. | `src/runtime/audio/router.rs:237` |
| sym-8425b7ab8ed18899aa59 | `observations` | function | Returns the observations exposed by `PlanEdgeReceiver`. | `src/runtime/audio/router.rs:655` |
| sym-bb909f68599f6929350a | `observations` | function | Returns the observations exposed by `PlanEdgeRouter`. | `src/runtime/audio/router.rs:881` |
| sym-95e0883cf9cfcb07662e | `observations` | function | Returns the observations exposed by `PlanSourceObservationHandle`. | `src/runtime/audio/runner.rs:143` |
| sym-ac1f679ae7e9f2fabe14 | `observations` | function | Returns the observations exposed by `PlanSourceSender`. | `src/runtime/audio/runner.rs:177` |
| sym-34f80ad71a31d7e9eef6 | `observations` | function | Returns the observations exposed by `PlanSourceInput`. | `src/runtime/audio/runner.rs:200` |
| sym-5505b9acd1f4733a3e68 | `observations` | function | Returns the observations exposed by `SidecarHost`. | `src/runtime/lifecycle/sidecar_host.rs:261` |
| sym-93216e72854b50f42707 | `observations` | function | Returns the observations exposed by `AsyncOperatorWorker`. | `src/runtime/signal/operator.rs:921` |
| sym-1786112a976fa4da87b7 | `operator_id` | function | Returns the operator identifier held by `ConnectorManifest`. | `src/connector/manifest.rs:132` |
| sym-5c2537e0f0e1194396e1 | `output_pool_exhaustions` | function | Returns the output pool exhaustions associated with `MixerTelemetry`. | `src/runtime/nodes.rs:263` |
| sym-72c06415474a4ff99674 | `overrun_count` | function | Returns the overrun count held by `BridgeSinkTelemetry`. | `src/runtime/nodes.rs:165` |
| sym-4ccedf87e32e9774b79a | `package_version` | function | Returns the package version held by `ConnectorManifest`. | `src/connector/manifest.rs:136` |
| sym-99d61fd60fa66abaf3db | `pocketstation::connector::sidecar::sidecar_connector_factory` | function | Creates a connector driver factory backed by the supplied sidecar process. | `src/connector/sidecar.rs:264` |
| sym-a13fb874065b9f7145f0 | `pocketstation::runtime::audio::runner::plan_source_channel` | function | Plans source channel for `runner`. | `src/runtime/audio/runner.rs:229` |
| sym-6a86a673b89a34b78d71 | `pocketstation::runtime::nodes::register_runtime_nodes` | function | Registers runtime nodes for `nodes`. | `src/runtime/nodes.rs:43` |
| sym-79e7f64b188402325901 | `port_name` | function | Returns the port name held by `ConnectorAudioRecord`. | `src/connector/transport.rs:335` |
| sym-f510b473b8ca98c83601 | `port_name` | function | Returns the port name held by `ConnectorInputDescriptor`. | `src/connector/worker/driver.rs:40` |
| sym-5479d5ffadb2d4d88a5d | `preparation_group` | function | Returns the preparation group associated with `SidecarConnectorDriverFactory`. | `src/connector/sidecar.rs:39` |
| sym-90bc1b303a52614edb58 | `prepare` | function | Prepares resources required by `SidecarConnectorDriverFactory`. | `src/connector/sidecar.rs:49` |
| sym-4ddbee20aa1a4b758d90 | `prepare` | function | Prepares resources required by `MixerSourceNode`. | `src/runtime/nodes.rs:432` |
| sym-b7217c287151cbf6ec09 | `prepare_and_spawn_from_plan_edge` | function | Prepares and spawn from plan edge for `AsyncOperatorWorker`. | `src/runtime/signal/operator.rs:764` |
| sym-56c5be58e62453b0b40a | `probe_interval` | function | Returns the probe interval held by `ConnectorReadinessPolicy`. | `src/connector/readiness.rs:47` |
| sym-dff6c45f96347045eb76 | `process` | function | Processes an input value through `SidecarConnectorDriverFactory`. | `src/connector/sidecar.rs:33` |
| sym-038e30bec596bf39134c | `process` | function | Processes an input value through `MixerSourceNode`. | `src/runtime/nodes.rs:441` |
| sym-fcd72944dcac0c4e4af4 | `process_ready` | function | Processes the ready inputs for `RealtimePlanRunner`. | `src/runtime/audio/runner.rs:338` |
| sym-948e37a429ffcad3e1f3 | `publish` | function | Publishes its owned operation for `TypedEdgeFanout`. | `src/runtime/signal/edge.rs:308` |
| sym-e60f163a42c86e627c15 | `readiness` | function | Returns the readiness held by `ConnectorManifest`. | `src/connector/manifest.rs:148` |
| sym-14159ada749817248906 | `readiness_reason_code` | function | Returns the readiness reason code held by `ConnectorServiceStatus`. | `src/connector/status.rs:54` |
| sym-1c56cd4861b4e8542db3 | `receive_signal` | function | Receives signal for `SidecarHost`. | `src/runtime/lifecycle/sidecar_host.rs:307` |
| sym-cf9795799631aa59db4d | `record_discontinuity` | function | Records discontinuity for `ConnectorContext`. | `src/connector/worker/coordination.rs:126` |
| sym-4db69bd3b958653d5a39 | `record_failure` | function | Records failure for `ConnectorContext`. | `src/connector/worker/coordination.rs:134` |
| sym-56b7d6a6b13ac312c206 | `record_frame_delivered` | function | Records frame delivered for `ConnectorContext`. | `src/connector/worker/coordination.rs:118` |
| sym-5fdc554f555394e3829a | `record_frame_dropped` | function | Records frame dropped for `ConnectorContext`. | `src/connector/worker/coordination.rs:122` |
| sym-fef19c653c5b665bfdb1 | `record_frame_received` | function | Records frame received for `ConnectorContext`. | `src/connector/worker/coordination.rs:114` |
| sym-a45c024f475a58d1b180 | `record_retry` | function | Records retry for `ConnectorContext`. | `src/connector/worker/coordination.rs:130` |
| sym-e929ec8f8bd8d46681a3 | `recovery` | function | Returns the recovery held by `ConnectorServiceStatus`. | `src/connector/status.rs:50` |
| sym-4e9f9c99a9cb2e52f040 | `recovery_reason_code` | function | Returns the recovery reason code held by `ConnectorServiceStatus`. | `src/connector/status.rs:62` |
| sym-4a56bcf46740dc03c2d8 | `register_connector` | function | Registers connector for `Session`. | `src/connector/mod.rs:204` |
| sym-076cb297137035bb5445 | `report_readiness_failure` | function | Returns the report readiness failure held by `ConnectorContext`. | `src/connector/worker/coordination.rs:97` |
| sym-d772a418c19fd77add1e | `report_readiness_success` | function | Records a successful readiness probe for `ConnectorContext`. | `src/connector/worker/coordination.rs:80` |
| sym-4de7461e08a0662dfe4a | `request` | function | Requests the state transition represented by `PlanRunnerCancellation`. | `src/runtime/audio/runner.rs:100` |
| sym-15106c234ce96f901dad | `required` | function | Returns the required held by `ConnectorRequirement`. | `src/connector/manifest.rs:65` |
| sym-f8be21ec7279a2be3397 | `requirement` | function | Returns the requirement held by `ConnectorConfigurationField`. | `src/connector/configuration.rs:214` |
| sym-6ca6792465c0da4ea967 | `requirements` | function | Returns the requirements held by `ConnectorManifest`. | `src/connector/manifest.rs:156` |
| sym-0bc159f5755990d8d3e7 | `resolve` | function | Resolves `ConnectorConfigurationSchema` into its validated representation. | `src/connector/configuration.rs:259` |
| sym-824f7af5a486a435b990 | `result` | function | Returns the result represented by `ConnectorRunOutcome`. | `src/connector/worker/mod.rs:59` |
| sym-d50e98ae88230f6991ac | `retryability` | function | Returns the retryability associated with `ConnectorError`. | `src/connector/error.rs:117` |
| sym-b5d7c5410981b4ea2c72 | `revision` | function | Returns the revision held by `ConnectorConfigurationSchema`. | `src/connector/configuration.rs:247` |
| sym-c00acee16d79601568b3 | `revision` | function | Returns the revision held by `ConnectorServiceStatus`. | `src/connector/status.rs:66` |
| sym-7a5f4c3b1c19458f2622 | `route_id` | function | Returns the route identifier held by `ConnectorInputDescriptor`. | `src/connector/worker/driver.rs:36` |
| sym-fbcfebd530788e070a8c | `sample_format` | function | Returns the sample format associated with `PlanEdgeFrame`. | `src/runtime/audio/router.rs:77` |
| sym-7d6a0b2352d4e8a2dce6 | `sample_rate_hz` | function | Returns the sample rate hertz held by `PlanEdgeFrame`. | `src/runtime/audio/router.rs:63` |
| sym-e2f0787e6e779bba80c2 | `samples` | function | Returns the audio samples held by `ConnectorAudioRecord`. | `src/connector/transport.rs:343` |
| sym-35573b446bcfa5698752 | `samples` | function | Returns the audio samples held by `PlanEdgeFrame`. | `src/runtime/audio/router.rs:84` |
| sym-aa0e0375e545c4bdc101 | `send` | function | Sends a value through `AsyncOperatorInput`. | `src/runtime/signal/io.rs:39` |
| sym-0b2d138a7fc7e6ce5d71 | `send_audio` | function | Sends audio for `AsyncOperatorInput`. | `src/runtime/signal/io.rs:55` |
| sym-2352790cae6eb2f7b359 | `sequence_number` | function | Returns the sequence number held by `PlanEdgeFrame`. | `src/runtime/audio/router.rs:49` |
| sym-98cd1bcf8ce5150f2a87 | `session_id` | function | Returns the session identifier held by `RegisteredConnector`. | `src/connector/mod.rs:132` |
| sym-c16c1279a7c88497027f | `set_connected` | function | Sets the connected used by `ConnectorContext`. | `src/connector/worker/coordination.rs:74` |
| sym-e1c705b3360b5cdb4e3c | `set_degraded` | function | Sets the degraded used by `ConnectorContext`. | `src/connector/worker/coordination.rs:56` |
| sym-0d813e85baaea8c42076 | `set_healthy` | function | Sets the healthy used by `ConnectorContext`. | `src/connector/worker/coordination.rs:62` |
| sym-ee840fcb3044537b46b4 | `set_not_ready` | function | Sets the not ready used by `ConnectorContext`. | `src/connector/worker/coordination.rs:50` |
| sym-75e1e42ad5f9e613a19a | `set_ready` | function | Sets the ready used by `ConnectorContext`. | `src/connector/worker/coordination.rs:44` |
| sym-35f5038470168d3cc14f | `set_reconnecting` | function | Sets the reconnecting used by `ConnectorContext`. | `src/connector/worker/coordination.rs:68` |
| sym-8a2399f88ff09ce306e9 | `shutdown` | function | Shuts down `AsyncRuntimeHost` according to its lifecycle contract. | `src/runtime/lifecycle/async_host.rs:91` |
| sym-d9d2e0d89468cdfc07e2 | `shutdown_mode` | function | Returns the shutdown mode held by `ConnectorContext`. | `src/connector/worker/coordination.rs:32` |
| sym-c5fc5da6a4d876db8d89 | `sidecar` | function | Builds an outbound Connector backed by one bounded sidecar process. | `src/connector/mod.rs:112` |
| sym-b4e31b8412b1f9584836 | `signal_spec` | function | Returns the signal spec held by `ConnectorInputDescriptor`. | `src/connector/worker/driver.rs:44` |
| sym-5b3105465b6e10369c49 | `snapshot` | function | Returns a point-in-time snapshot of `ConnectorObservationHandle`. | `src/connector/observations.rs:53` |
| sym-b34f7de09857843496dd | `snapshot` | function | Returns a point-in-time snapshot of `AsyncOperatorObservationHandle`. | `src/runtime/signal/observations.rs:52` |
| sym-67946fe0311861a74164 | `source_id` | function | Returns the source identifier held by `PlanEdgeFrame`. | `src/runtime/audio/router.rs:42` |
| sym-0928e754c866d93b4664 | `source_node_id` | function | Returns the source node identifier held by `PlanSourceInput`. | `src/runtime/audio/runner.rs:196` |
| sym-368806897864f04748d1 | `source_observations` | function | Returns the source observations held by `RealtimePlanRunner`. | `src/runtime/audio/runner.rs:349` |
| sym-e3ff3a0d7c299d5f93d1 | `spawn` | function | Spawns its owned operation for `GeneratedAudioBridge`. | `src/runtime/bridge/audio.rs:131` |
| sym-36b9156f7e45336488f3 | `spawn` | function | Spawns its owned operation for `SidecarHost`. | `src/runtime/lifecycle/sidecar_host.rs:174` |
| sym-1f34a59bc9b8a05605df | `spawn` | function | Spawns its owned operation for `AsyncOperatorWorker`. | `src/runtime/signal/operator.rs:536` |
| sym-654334b47f76036a36df | `spawn_composed` | function | Spawns composed for `AsyncOperatorWorker`. | `src/runtime/signal/operator.rs:604` |
| sym-cd0608562372467f8a3b | `spawn_with_context` | function | Starts a directly-fed worker with an already negotiated signal-shaped prepare context. Session-owned graph execution uses the compiled-edge path; this entry point exists for external harnesses that negotiate the boundary before constructing a full Session. | `src/runtime/signal/operator.rs:563` |
| sym-cfddbe05a671a2b21449 | `stage` | function | Returns the stage held by `ConnectorError`. | `src/connector/error.rs:113` |
| sym-89969574c9b15ca8049f | `startup_timeout` | function | Returns the startup timeout held by `ConnectorReadinessPolicy`. | `src/connector/readiness.rs:43` |
| sym-8e67bc135bc37c8ef3be | `state` | function | Returns the state associated with `SidecarHost`. | `src/runtime/lifecycle/sidecar_host.rs:257` |
| sym-999cb9e4270bf5715275 | `stream_id` | function | Returns the stream identifier held by `PlanEdgeFrame`. | `src/runtime/audio/router.rs:35` |
| sym-85358c2d790b8b9d2d7a | `success` | function | Returns whether `ConnectorRunOutcome` completed successfully. | `src/connector/worker/mod.rs:51` |
| sym-f54e46b4ef85fcf04135 | `success_threshold` | function | Returns the success threshold held by `ConnectorReadinessPolicy`. | `src/connector/readiness.rs:51` |
| sym-af33bd63d369ef77b28a | `timestamp_ns` | function | Returns the timestamp nanoseconds held by `PlanEdgeFrame`. | `src/runtime/audio/router.rs:56` |
| sym-8e17da30536a48662ee4 | `to` | function | Returns the destination owned by `PlanEdgeReceiver`. | `src/runtime/audio/router.rs:537` |
| sym-d3f1ad03e5025f396cb1 | `try_from` | function | Attempts to from through `SidecarMessageKind`. | `src/runtime/lifecycle/sidecar_protocol.rs:25` |
| sym-10f2001491a5d174cafb | `try_new` | function | Creates a new `ConnectorAudioRecord` after validating its inputs. | `src/connector/transport.rs:300` |
| sym-dc1b416b106fdeed8fa1 | `try_receive_signal` | function | Attempts to receive signal through `SidecarHost`. | `src/runtime/lifecycle/sidecar_host.rs:299` |
| sym-64e490e9924de0b9cb3b | `try_recv` | function | Pops one queued frame before sampling the monotonic process clock. | `src/runtime/audio/router.rs:566` |
| sym-cc9020046d219348d0ec | `try_recv_for_testing` | function | Attempts to recv for testing through `PlanSourceInput`. | `src/runtime/audio/runner.rs:213` |
| sym-4223bc9a8f345400a53c | `try_send` | function | Attempts to send a value through `PlanSourceSender` without waiting for capacity. | `src/runtime/audio/runner.rs:149` |
| sym-0890667724f142094eb6 | `try_send_signal` | function | Attempts to send signal through `SidecarHost`. | `src/runtime/lifecycle/sidecar_host.rs:265` |
| sym-907d6ccce98c264c9a10 | `underrun_count` | function | Returns the underrun count held by `SystemOutputTelemetry`. | `src/runtime/nodes.rs:61` |

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

The claims on **Sidecar protocol** are anchored to Git snapshot `136e74888962558aa846d3143a19136a70936f45` and these primary files:

- `src/runtime/lifecycle/sidecar_protocol.rs:1-374` (`DIRECT`)

For **Sidecar protocol**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.
