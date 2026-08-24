# Connector API

<!-- claims: CLM-REF-009-SCOPE-001,CLM-REF-009-TEXT-001,CLM-REF-009-TEXT-002,CLM-REF-009-SOURCE-001 -->

## Scope

- **Declare connector manifests and configuration.** Describe connector identity, ports, configuration schema, secrets, and delivery policy without embedding a provider protocol in Core.
- **Run connector workers.** Supervise connector delivery, acknowledgement, readiness, cancellation, drain, and abort while reporting retry attempts and typed retryability.

The scope of **Connector API** ends at the native contracts and executable conditions cited below. Platform qualification, performance, retry, and delivery require their own explicit evidence.

## Reference authority

For **Connector API**, the generated [docs.rs API](https://docs.rs/pocketstation/latest/pocketstation/) provides compiler-rendered signatures and navigation. This repository page adds the frozen evidence identifiers, responsibilities, and cross-boundary interpretation used by the documentation verifier.

## Public surface

| Evidence record | Declaration | Kind | Purpose | Source |
|---|---|---|---|---|
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
| sym-42a8e5c4d0cbc083c502 | `accepts_delivery` | function | Reports whether accepts delivery is true for `ConnectorDeliveryReadiness`. | `src/connector/status.rs:10` |
| sym-8d570ccfcc6f8f22adb8 | `accepts_delivery` | function | Reports whether accepts delivery is true for `ConnectorServiceStatus`. | `src/connector/status.rs:74` |
| sym-e9973a9598777cbaefb0 | `api_revision` | function | Returns the API revision held by `ConnectorManifest`. | `src/connector/manifest.rs:124` |
| sym-ecb602e99c0dba24af4f | `as_str` | function | Returns the stable string representation of `ConnectorConfigurationErrorCode`. | `src/connector/configuration.rs:585` |
| sym-f5caa543ea9c940cafb1 | `as_str` | function | Returns the stable string representation of `ConnectorErrorCode`. | `src/connector/error.rs:29` |
| sym-4a32dce08606102cb841 | `capabilities` | function | Returns the capabilities held by `ConnectorManifest`. | `src/connector/manifest.rs:152` |
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
| sym-6c2f846a56c09b494d97 | `delivery_readiness` | function | Returns the delivery readiness associated with `ConnectorServiceStatus`. | `src/connector/status.rs:42` |
| sym-ad4e94586f88eebd3375 | `deprecated` | function | Returns the deprecated held by `ConnectorConfigurationField`. | `src/connector/configuration.rs:201` |
| sym-6b003ff3808eecc732f0 | `deprecation` | function | Returns the deprecation held by `ConnectorConfigurationField`. | `src/connector/configuration.rs:226` |
| sym-3248ef146d7f14d09bc6 | `documentation` | function | Returns the documentation held by `ConnectorConfigurationField`. | `src/connector/configuration.rs:218` |
| sym-91cdc75380f1b1c45072 | `documentation` | function | Returns the documentation held by `ConnectorCapability`. | `src/connector/manifest.rs:34` |
| sym-34ddc898cf906f7e707f | `documentation` | function | Returns the documentation held by `ConnectorRequirement`. | `src/connector/manifest.rs:69` |
| sym-24451f44bd94778b8d32 | `drop` | function | Releases resources owned by `ConnectorSecret`. | `src/connector/configuration.rs:49` |
| sym-e443085133525c866043 | `edge_contract` | function | Returns the edge contract held by `ConnectorInputDescriptor`. | `src/connector/worker/driver.rs:52` |
| sym-c46a34cb2388608cd3b1 | `encode` | function | Encodes input through `ConnectorConfigurationRecord`. | `src/connector/transport.rs:61` |
| sym-08a8a31e4c30a3df7c8e | `encode` | function | Encodes input through `ConnectorAudioRecord`. | `src/connector/transport.rs:347` |
| sym-0c5da44a14691ca38d74 | `endpoint_id` | function | Returns the endpoint identifier held by `ConnectorInputDescriptor`. | `src/connector/worker/driver.rs:28` |
| sym-575cacd62461aa3cdc9c | `endpoint_observations` | function | Returns the endpoint observations held by `ConnectorContext`. | `src/connector/worker/coordination.rs:139` |
| sym-b9d0e20afe7dce1858a4 | `expose_secret` | function | Exposes the secret to the owning connector during setup or worker use. | `src/connector/configuration.rs:37` |
| sym-d55e9dc582b914e1a93b | `failure` | function | Returns the failure held by `ConnectorRunOutcome`. | `src/connector/worker/mod.rs:55` |
| sym-56df668f46edc7d9e40d | `failure_threshold` | function | Returns the failure threshold held by `ConnectorReadinessPolicy`. | `src/connector/readiness.rs:55` |
| sym-267259f742b6a1cc1924 | `field` | function | Returns the field held by `ConnectorConfigurationSchema`. | `src/connector/configuration.rs:255` |
| sym-8d0d1e80c210dfa25f85 | `field` | function | Returns the field held by `ConnectorConfigurationError`. | `src/connector/configuration.rs:619` |
| sym-61bd3f48c87a4957932a | `fields` | function | Returns the fields held by `ConnectorConfigurationSchema`. | `src/connector/configuration.rs:251` |
| sym-cf4630838130fe080faf | `fmt` | function | Formats `ConnectorSecret` with the requested formatter. | `src/connector/configuration.rs:43` |
| sym-269f701717d0abdc7ced | `fmt` | function | Formats `ConnectorErrorCode` with the requested formatter. | `src/connector/error.rs:35` |
| sym-0652c06beea3ce6befb9 | `fmt` | function | Formats `ConnectorErrorCode` with the requested formatter. | `src/connector/error.rs:44` |
| sym-59196294cd066af3a974 | `from_item` | function | Creates `ConnectorAudioRecord` from item. | `src/connector/transport.rs:314` |
| sym-216ca51c178e3d2bdb1e | `from_resolved` | function | Creates `ConnectorConfigurationRecord` from resolved. | `src/connector/transport.rs:45` |
| sym-c7c63cd95c4233b5a1ad | `get` | function | Returns the value held by `ConnectorConfiguration`. | `src/connector/configuration.rs:134` |
| sym-1baa6743961e6be9c1ac | `get` | function | Returns the value held by `ResolvedConnectorConfiguration`. | `src/connector/configuration.rs:394` |
| sym-8ee18b8cc0f3b596159d | `health` | function | Returns the health held by `ConnectorServiceStatus`. | `src/connector/status.rs:46` |
| sym-dd37553bb3a13016e289 | `health_reason_code` | function | Returns the health reason code held by `ConnectorServiceStatus`. | `src/connector/status.rs:58` |
| sym-b19bfeb5e328f1d64e7f | `id` | function | Returns the id held by `ConnectorCapability`. | `src/connector/manifest.rs:30` |
| sym-43225e980a364baa9ced | `id` | function | Returns the id held by `ConnectorRequirement`. | `src/connector/manifest.rs:61` |
| sym-803ad52760f815fa63af | `input` | function | Returns the input held by `ConnectorItem`. | `src/connector/worker/driver.rs:74` |
| sym-fd92f420eac5795ad4f9 | `insert` | function | Inserts a typed configuration value into `ConnectorConfiguration`. | `src/connector/configuration.rs:126` |
| sym-7108c905ca4780df696b | `into_configuration` | function | Converts `ConnectorConfigurationRecord` into configuration. | `src/connector/transport.rs:57` |
| sym-b74082477b551771a0a0 | `into_endpoint_failure` | function | Converts `ConnectorError` into endpoint failure. | `src/connector/error.rs:125` |
| sym-0eefd4234befb8a34c61 | `is_abort_requested` | function | Reports whether abort requested is true for `ConnectorContext`. | `src/connector/worker/coordination.rs:36` |
| sym-104b3bac350417f37f40 | `is_empty` | function | Returns whether `ConnectorConfiguration` contains no values. | `src/connector/configuration.rs:146` |
| sym-1440b0574deddcb13357 | `is_stop_requested` | function | Reports whether stop requested is true for `ConnectorContext`. | `src/connector/worker/coordination.rs:28` |
| sym-77a3c9fd80fed3c3b18f | `iter` | function | Iterates over the values held by `ConnectorConfiguration`. | `src/connector/configuration.rs:138` |
| sym-d8cf73924fb51e8b2b8e | `iter` | function | Iterates over the values held by `ResolvedConnectorConfiguration`. | `src/connector/configuration.rs:398` |
| sym-e136b77b30dedf88de08 | `kind` | function | Returns the kind represented by `ConnectorConfigurationValue`. | `src/connector/configuration.rs:77` |
| sym-3e3073a9a190630017e1 | `last_transition_elapsed_ns` | function | Returns the last transition elapsed nanoseconds held by `ConnectorServiceStatus`. | `src/connector/status.rs:70` |
| sym-b83c3de0aeafa5c07084 | `len` | function | Returns the number of values held by `ConnectorConfiguration`. | `src/connector/configuration.rs:142` |
| sym-965aa3b29c0459c9cb66 | `manifest` | function | Returns the manifest held by `Connector`. | `src/connector/mod.rs:119` |
| sym-c5ae77d57d3faef142e8 | `manifest` | function | Returns the manifest held by `RegisteredConnector`. | `src/connector/mod.rs:136` |
| sym-e5b207f73575f5d653dc | `manifest_revision` | function | Returns the manifest revision held by `ConnectorManifest`. | `src/connector/manifest.rs:128` |
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
| sym-f033a669e834bb2886e8 | `node` | function | Returns the node held by `ConnectorManifest`. | `src/connector/manifest.rs:140` |
| sym-b9068cec996a567f13a5 | `observation` | function | Returns the current observation exposed by `RegisteredConnector`. | `src/connector/mod.rs:140` |
| sym-4a2c8867bce1f05a18e1 | `observations` | function | Returns the observations exposed by `RegisteredConnector`. | `src/connector/mod.rs:153` |
| sym-1786112a976fa4da87b7 | `operator_id` | function | Returns the operator identifier held by `ConnectorManifest`. | `src/connector/manifest.rs:132` |
| sym-4ccedf87e32e9774b79a | `package_version` | function | Returns the package version held by `ConnectorManifest`. | `src/connector/manifest.rs:136` |
| sym-99d61fd60fa66abaf3db | `pocketstation::connector::sidecar::sidecar_connector_factory` | function | Creates a connector driver factory backed by the supplied sidecar process. | `src/connector/sidecar.rs:264` |
| sym-79e7f64b188402325901 | `port_name` | function | Returns the port name held by `ConnectorAudioRecord`. | `src/connector/transport.rs:335` |
| sym-f510b473b8ca98c83601 | `port_name` | function | Returns the port name held by `ConnectorInputDescriptor`. | `src/connector/worker/driver.rs:40` |
| sym-5479d5ffadb2d4d88a5d | `preparation_group` | function | Returns the preparation group associated with `SidecarConnectorDriverFactory`. | `src/connector/sidecar.rs:39` |
| sym-90bc1b303a52614edb58 | `prepare` | function | Prepares resources required by `SidecarConnectorDriverFactory`. | `src/connector/sidecar.rs:49` |
| sym-56c5be58e62453b0b40a | `probe_interval` | function | Returns the probe interval held by `ConnectorReadinessPolicy`. | `src/connector/readiness.rs:47` |
| sym-dff6c45f96347045eb76 | `process` | function | Processes an input value through `SidecarConnectorDriverFactory`. | `src/connector/sidecar.rs:33` |
| sym-e60f163a42c86e627c15 | `readiness` | function | Returns the readiness held by `ConnectorManifest`. | `src/connector/manifest.rs:148` |
| sym-14159ada749817248906 | `readiness_reason_code` | function | Returns the readiness reason code held by `ConnectorServiceStatus`. | `src/connector/status.rs:54` |
| sym-cf9795799631aa59db4d | `record_discontinuity` | function | Increments the discontinuity observation recorded by `ConnectorContext`. | `src/connector/worker/coordination.rs:126` |
| sym-4db69bd3b958653d5a39 | `record_failure` | function | Records a connector failure and its retry classification in `ConnectorContext`. | `src/connector/worker/coordination.rs:134` |
| sym-56b7d6a6b13ac312c206 | `record_frame_delivered` | function | Records frame delivered for `ConnectorContext`. | `src/connector/worker/coordination.rs:118` |
| sym-5fdc554f555394e3829a | `record_frame_dropped` | function | Records frame dropped for `ConnectorContext`. | `src/connector/worker/coordination.rs:122` |
| sym-fef19c653c5b665bfdb1 | `record_frame_received` | function | Records frame received for `ConnectorContext`. | `src/connector/worker/coordination.rs:114` |
| sym-a45c024f475a58d1b180 | `record_retry` | function | Increments the retry-attempt observation recorded by `ConnectorContext`. | `src/connector/worker/coordination.rs:130` |
| sym-e929ec8f8bd8d46681a3 | `recovery` | function | Returns the recovery held by `ConnectorServiceStatus`. | `src/connector/status.rs:50` |
| sym-4e9f9c99a9cb2e52f040 | `recovery_reason_code` | function | Returns the recovery reason code held by `ConnectorServiceStatus`. | `src/connector/status.rs:62` |
| sym-4a56bcf46740dc03c2d8 | `register_connector` | function | Registers one connector implementation for use by `Session`. | `src/connector/mod.rs:204` |
| sym-076cb297137035bb5445 | `report_readiness_failure` | function | Returns the report readiness failure held by `ConnectorContext`. | `src/connector/worker/coordination.rs:97` |
| sym-d772a418c19fd77add1e | `report_readiness_success` | function | Records a successful readiness probe for `ConnectorContext`. | `src/connector/worker/coordination.rs:80` |
| sym-15106c234ce96f901dad | `required` | function | Returns the required held by `ConnectorRequirement`. | `src/connector/manifest.rs:65` |
| sym-f8be21ec7279a2be3397 | `requirement` | function | Returns the requirement held by `ConnectorConfigurationField`. | `src/connector/configuration.rs:214` |
| sym-6ca6792465c0da4ea967 | `requirements` | function | Returns the requirements held by `ConnectorManifest`. | `src/connector/manifest.rs:156` |
| sym-0bc159f5755990d8d3e7 | `resolve` | function | Resolves `ConnectorConfigurationSchema` into its validated representation. | `src/connector/configuration.rs:259` |
| sym-824f7af5a486a435b990 | `result` | function | Returns the result represented by `ConnectorRunOutcome`. | `src/connector/worker/mod.rs:59` |
| sym-d50e98ae88230f6991ac | `retryability` | function | Returns the retryability associated with `ConnectorError`. | `src/connector/error.rs:117` |
| sym-b5d7c5410981b4ea2c72 | `revision` | function | Returns the revision held by `ConnectorConfigurationSchema`. | `src/connector/configuration.rs:247` |
| sym-c00acee16d79601568b3 | `revision` | function | Returns the revision held by `ConnectorServiceStatus`. | `src/connector/status.rs:66` |
| sym-7a5f4c3b1c19458f2622 | `route_id` | function | Returns the route identifier held by `ConnectorInputDescriptor`. | `src/connector/worker/driver.rs:36` |
| sym-e2f0787e6e779bba80c2 | `samples` | function | Returns the audio samples held by `ConnectorAudioRecord`. | `src/connector/transport.rs:343` |
| sym-98cd1bcf8ce5150f2a87 | `session_id` | function | Returns the session identifier held by `RegisteredConnector`. | `src/connector/mod.rs:132` |
| sym-c16c1279a7c88497027f | `set_connected` | function | Sets the connected used by `ConnectorContext`. | `src/connector/worker/coordination.rs:74` |
| sym-e1c705b3360b5cdb4e3c | `set_degraded` | function | Sets the degraded used by `ConnectorContext`. | `src/connector/worker/coordination.rs:56` |
| sym-0d813e85baaea8c42076 | `set_healthy` | function | Sets the healthy used by `ConnectorContext`. | `src/connector/worker/coordination.rs:62` |
| sym-ee840fcb3044537b46b4 | `set_not_ready` | function | Sets the not ready used by `ConnectorContext`. | `src/connector/worker/coordination.rs:50` |
| sym-75e1e42ad5f9e613a19a | `set_ready` | function | Sets the ready used by `ConnectorContext`. | `src/connector/worker/coordination.rs:44` |
| sym-35f5038470168d3cc14f | `set_reconnecting` | function | Sets the reconnecting used by `ConnectorContext`. | `src/connector/worker/coordination.rs:68` |
| sym-d9d2e0d89468cdfc07e2 | `shutdown_mode` | function | Returns the shutdown mode held by `ConnectorContext`. | `src/connector/worker/coordination.rs:32` |
| sym-c5fc5da6a4d876db8d89 | `sidecar` | function | Builds an outbound Connector backed by one bounded sidecar process. | `src/connector/mod.rs:112` |
| sym-b4e31b8412b1f9584836 | `signal_spec` | function | Returns the signal spec held by `ConnectorInputDescriptor`. | `src/connector/worker/driver.rs:44` |
| sym-5b3105465b6e10369c49 | `snapshot` | function | Returns a point-in-time snapshot of `ConnectorObservationHandle`. | `src/connector/observations.rs:53` |
| sym-cfddbe05a671a2b21449 | `stage` | function | Returns the stage held by `ConnectorError`. | `src/connector/error.rs:113` |
| sym-89969574c9b15ca8049f | `startup_timeout` | function | Returns the startup timeout held by `ConnectorReadinessPolicy`. | `src/connector/readiness.rs:43` |
| sym-85358c2d790b8b9d2d7a | `success` | function | Returns whether `ConnectorRunOutcome` completed successfully. | `src/connector/worker/mod.rs:51` |
| sym-f54e46b4ef85fcf04135 | `success_threshold` | function | Returns the success threshold held by `ConnectorReadinessPolicy`. | `src/connector/readiness.rs:51` |
| sym-10f2001491a5d174cafb | `try_new` | function | Creates a new `ConnectorAudioRecord` after validating its inputs. | `src/connector/transport.rs:300` |
| sym-04aa2dfac6eac63b8ad0 | `validate` | function | Validates `ConnectorManifest` against its declared contract. | `src/connector/manifest.rs:160` |
| sym-b7a32595b7e5e78d9c01 | `value_kind` | function | Returns the value kind held by `ConnectorConfigurationField`. | `src/connector/configuration.rs:210` |
| sym-7877b477089b3893c67b | `wait_for_stop` | function | Waits until a stop request is visible to `ConnectorContext`. | `src/connector/worker/coordination.rs:40` |
| sym-16abcbb0e7ee8b036215 | `with` | function | Returns `ConnectorConfiguration` with the supplied entry applied. | `src/connector/configuration.rs:121` |
| sym-279915258c10c0a210dc | `with_capability` | function | Sets the capability on `ConnectorManifest` and returns the updated value. | `src/connector/manifest.rs:113` |
| sym-4e4a1d1eb2e93242edf6 | `with_constraint` | function | Sets the constraint on `ConnectorConfigurationField` and returns the updated value. | `src/connector/configuration.rs:195` |
| sym-5db6054df927f0d88625 | `with_driver` | function | Builds a connector whose bounded receiver loop is owned by Core. | `src/connector/mod.rs:88` |
| sym-840d33f0f72a5f846357 | `with_requirement` | function | Sets the requirement on `ConnectorManifest` and returns the updated value. | `src/connector/manifest.rs:119` |
| sym-2759496bc4da50b77077 | `pocketstation::connector` | module | Connector manifests, configuration, workers, transport records, readiness, and observations. | `src/connector/mod.rs:1` |
| sym-2643a4aac0860723b15d | `pocketstation::connector::Connector` | struct | Declares a connector endpoint and the manifest-backed configuration used to instantiate it. | `src/connector/mod.rs:61` |
| sym-26833010c1d279ca5fad | `pocketstation::connector::RegisteredConnector` | struct | Retains a connector declaration after its factory has been registered with the node registry. | `src/connector/mod.rs:125` |
| sym-54e5f0babb7503a71dd4 | `pocketstation::connector::configuration::ConnectorConfiguration` | struct | Configures connector behavior at its owning API boundary. | `src/connector/configuration.rs:111` |
| sym-fee181978d0df0e2a327 | `pocketstation::connector::configuration::ConnectorConfigurationError` | struct | Reports a connector configuration error. | `src/connector/configuration.rs:608` |
| sym-ae4b94a01cd9d7f636bf | `pocketstation::connector::configuration::ConnectorConfigurationField` | struct | Declares one typed connector configuration field and its validation constraints. | `src/connector/configuration.rs:168` |
| sym-9d7dcaf80e077048757c | `pocketstation::connector::configuration::ConnectorConfigurationSchema` | struct | Validates connector configuration values against the manifest-declared field set. | `src/connector/configuration.rs:232` |
| sym-d390af4b6e516007bcaa | `pocketstation::connector::configuration::ConnectorSecret` | struct | Owns a connector secret with redacted diagnostics and byte clearing on explicit reset or drop. | `src/connector/configuration.rs:11` |
| sym-34b655651fe9a6612717 | `pocketstation::connector::configuration::ResolvedConnectorConfiguration` | struct | Configures resolved connector behavior at its owning API boundary. | `src/connector/configuration.rs:391` |
| sym-fc733bba3416b6904a41 | `pocketstation::connector::error::ConnectorError` | struct | Reports a connector error. | `src/connector/error.rs:80` |
| sym-852fcec3d11a7f031380 | `pocketstation::connector::error::ConnectorErrorCode` | struct | Carries the stable external error code exported for a connector failure. | `src/connector/error.rs:10` |
| sym-8e8fc2d19d8e3f81f2e6 | `pocketstation::connector::manifest::ConnectorCapability` | struct | Declares a capability advertised by a connector manifest. | `src/connector/manifest.rs:12` |
| sym-80cc7810d7774b6e700f | `pocketstation::connector::manifest::ConnectorManifest` | struct | Declares connector identity, API revision, ports, capabilities, requirements, and configuration schema. | `src/connector/manifest.rs:75` |
| sym-a6270afb57aa40f4748e | `pocketstation::connector::manifest::ConnectorRequirement` | struct | Declares a host or configuration requirement that must be satisfied before connector use. | `src/connector/manifest.rs:40` |
| sym-92d5e9e03f1b807baad9 | `pocketstation::connector::observations::ConnectorObservationHandle` | struct | Holds the ownership or bounded access represented by connector observation handle. | `src/connector/observations.rs:15` |
| sym-4dc6e800ff3daf898c34 | `pocketstation::connector::observations::ConnectorObservations` | struct | Reports the connector observations collected at an observation boundary. | `src/connector/observations.rs:158` |
| sym-fb9390342c3c28a176ed | `pocketstation::connector::observations::ConnectorRuntimeObservations` | struct | Reports the connector runtime observations collected at an observation boundary. | `src/connector/observations.rs:168` |
| sym-2b8230059d41f75e876f | `pocketstation::connector::readiness::ConnectorReadinessPolicy` | struct | Configures connector readiness behavior at its owning API boundary. | `src/connector/readiness.rs:7` |
| sym-3dd3e57393036ec5bb77 | `pocketstation::connector::sidecar::SidecarConnectorDriverFactory` | struct | Adapts a bounded PocketStation sidecar process to the Connector driver SPI. | `src/connector/sidecar.rs:24` |
| sym-86ea9be5668484ecc9ca | `pocketstation::connector::status::ConnectorServiceStatus` | struct | Reports the structured connector service status. | `src/connector/status.rs:30` |
| sym-6cb3d4b00ef6d9b16333 | `pocketstation::connector::transport::ConnectorAudioMetadata` | struct | Carries source, stream, timing, and format metadata beside a connector audio record. | `src/connector/transport.rs:281` |
| sym-1f7ed33259c180b067f5 | `pocketstation::connector::transport::ConnectorAudioRecord` | struct | Records one immutable connector audio observation. | `src/connector/transport.rs:293` |
| sym-b80ff404a70ae89b732e | `pocketstation::connector::transport::ConnectorConfigurationRecord` | struct | Canonical typed configuration handed to a connector sidecar during its bounded Configure handshake. Secret classification survives the boundary; Debug output continues to redact secret values. | `src/connector/transport.rs:42` |
| sym-9b020b7c50129f5ed934 | `pocketstation::connector::worker::ConnectorRunOutcome` | struct | Reports the structured connector run outcome. | `src/connector/worker/mod.rs:42` |
| sym-8019a7ff49d910e90dd0 | `pocketstation::connector::worker::coordination::ConnectorContext` | struct | Carries the inputs and runtime context required to connector. | `src/connector/worker/coordination.rs:14` |
| sym-545609dfc875348c0cad | `pocketstation::connector::worker::driver::ConnectorInputDescriptor` | struct | Immutable Session and graph metadata for one connector input. | `src/connector/worker/driver.rs:16` |
| sym-38ee30a2a78799ac7538 | `ConnectorAudioMetadata::channels` | struct_field | Contains the channels owned or reported by `ConnectorAudioMetadata`. | `src/connector/transport.rs:288` |
| sym-0bcef6e8ce4a79c56e13 | `ConnectorAudioMetadata::connector_id` | struct_field | Identifies the connector identifier recorded by `ConnectorAudioMetadata`. | `src/connector/transport.rs:283` |
| sym-75554d12f59bc50d8daa | `ConnectorAudioMetadata::endpoint_id` | struct_field | Identifies the endpoint identifier recorded by `ConnectorAudioMetadata`. | `src/connector/transport.rs:282` |
| sym-49c28d13d9815161fa91 | `ConnectorAudioMetadata::lineage` | struct_field | Preserves the source and stream lineage attached to `ConnectorAudioMetadata`. | `src/connector/transport.rs:286` |
| sym-6f20ab7210fb2a83371e | `ConnectorAudioMetadata::route_id` | struct_field | Identifies the route identifier recorded by `ConnectorAudioMetadata`. | `src/connector/transport.rs:284` |
| sym-7d045b14d056d39deb9a | `ConnectorAudioMetadata::sample_format` | struct_field | Stores the sample format as a `SampleFormat` value in `ConnectorAudioMetadata`. | `src/connector/transport.rs:289` |
| sym-f695c94706756d4c54ca | `ConnectorAudioMetadata::sample_rate_hz` | struct_field | Stores the sample rate value for `ConnectorAudioMetadata`, in hertz. | `src/connector/transport.rs:287` |
| sym-6078a5aa450d00807875 | `ConnectorAudioMetadata::stream_id` | struct_field | Identifies the stream identifier recorded by `ConnectorAudioMetadata`. | `src/connector/transport.rs:285` |
| sym-fab2083461544e4336cf | `ConnectorConfigurationConstraint::SignedRange::maximum` | struct_field | Sets the inclusive maximum accepted by `SignedRange`. | `src/connector/configuration.rs:162` |
| sym-c763f211b9894ba7e672 | `ConnectorConfigurationConstraint::SignedRange::minimum` | struct_field | Sets the inclusive minimum accepted by `SignedRange`. | `src/connector/configuration.rs:162` |
| sym-32658b6f27f4d3ede9c1 | `ConnectorConfigurationConstraint::TextLengthBytes::maximum` | struct_field | Sets the inclusive maximum accepted by `TextLengthBytes`. | `src/connector/configuration.rs:161` |
| sym-2cb5e15c4fc3df99b57d | `ConnectorConfigurationConstraint::TextLengthBytes::minimum` | struct_field | Sets the inclusive minimum accepted by `TextLengthBytes`. | `src/connector/configuration.rs:161` |
| sym-a81f7a15d20dd2029844 | `ConnectorConfigurationConstraint::UnsignedRange::maximum` | struct_field | Sets the inclusive maximum accepted by `UnsignedRange`. | `src/connector/configuration.rs:163` |
| sym-0d021f4166c067723d29 | `ConnectorConfigurationConstraint::UnsignedRange::minimum` | struct_field | Sets the inclusive minimum accepted by `UnsignedRange`. | `src/connector/configuration.rs:163` |
| sym-540cb13ecaa8e4b2e1e6 | `ConnectorItem::Audio::frame` | struct_field | Stores the frame as a `EndpointAudioFrame` value in `Audio`. | `src/connector/worker/driver.rs:65` |
| sym-62300d0c5453fecd25fc | `ConnectorItem::Audio::input` | struct_field | References the input participating in `Audio`. | `src/connector/worker/driver.rs:64` |
| sym-3b57e5805404d97d9a6a | `ConnectorItem::Signal::input` | struct_field | References the input participating in `Signal`. | `src/connector/worker/driver.rs:68` |
| sym-2a34f67641fcb62ae6f2 | `ConnectorItem::Signal::signal` | struct_field | Stores the signal component of `Signal`. | `src/connector/worker/driver.rs:69` |
| sym-4d9cd4cd7ce144626451 | `ConnectorManifestError::DuplicateManifestEntry::id` | struct_field | Identifies the id recorded by `DuplicateManifestEntry`. | `src/connector/manifest.rs:253` |
| sym-1c697783f13dd686b2c9 | `ConnectorManifestError::UnsupportedApiRevision::requested` | struct_field | Stores the requested component of `UnsupportedApiRevision`. | `src/connector/manifest.rs:233` |
| sym-4582619673410fe66675 | `ConnectorManifestError::UnsupportedApiRevision::supported` | struct_field | References the supported participating in `UnsupportedApiRevision`. | `src/connector/manifest.rs:233` |
| sym-efdfba7041f7cf72d3a1 | `ConnectorObservations::failures_total` | struct_field | Counts the total number of failures observed by `ConnectorObservations`. | `src/connector/observations.rs:163` |
| sym-22d3f76efb08286196d1 | `ConnectorObservations::last_error` | struct_field | Carries the last error reported by `ConnectorObservations`. | `src/connector/observations.rs:164` |
| sym-2588b5cb0817be4d8ac9 | `ConnectorObservations::reconnects_total` | struct_field | Counts the total number of reconnects observed by `ConnectorObservations`. | `src/connector/observations.rs:162` |
| sym-7940ce0e86b9ba2b7e0d | `ConnectorObservations::retry_attempts_total` | struct_field | Counts the total number of retry attempts observed by `ConnectorObservations`. | `src/connector/observations.rs:161` |
| sym-5d26bd73d61604db9f4b | `ConnectorObservations::service_status` | struct_field | Contains the service status owned or reported by `ConnectorObservations`. | `src/connector/observations.rs:159` |
| sym-f6e6b5f611b1a10d986f | `ConnectorObservations::status_transitions_total` | struct_field | Counts the total number of status transitions observed by `ConnectorObservations`. | `src/connector/observations.rs:160` |
| sym-49fe0d4e4422ca286a7c | `ConnectorRuntimeObservations::connector` | struct_field | Stores the connector as a `ConnectorObservations` value in `ConnectorRuntimeObservations`. | `src/connector/observations.rs:170` |
| sym-4e9e7b9291340f1ffd2e | `ConnectorRuntimeObservations::endpoint` | struct_field | References the endpoint participating in `ConnectorRuntimeObservations`. | `src/connector/observations.rs:171` |
| sym-4a969a58f4ac2b5709e8 | `ConnectorRuntimeObservations::endpoint_ids` | struct_field | Identifies the endpoint identifiers recorded by `ConnectorRuntimeObservations`. | `src/connector/observations.rs:169` |
| sym-b6ef30d262e385fa8459 | `pocketstation::connector::ConnectorDeclarationError::WrongSession::registered` | struct_field | Stores the registered as a `SessionId` value in `WrongSession`. | `src/connector/mod.rs:236` |
| sym-0d9c40408e097e6c9eb3 | `pocketstation::connector::ConnectorDeclarationError::WrongSession::requested` | struct_field | Stores the requested as a `SessionId` value in `WrongSession`. | `src/connector/mod.rs:237` |
| sym-b9c47bf9d684d27871ea | `pocketstation::connector::ConnectorObservationLookupError::WrongSession::registered` | struct_field | Stores the registered as a `SessionId` value in `WrongSession`. | `src/connector/mod.rs:249` |
| sym-d3562b133871922e82cf | `pocketstation::connector::ConnectorObservationLookupError::WrongSession::requested` | struct_field | Stores the requested as a `SessionId` value in `WrongSession`. | `src/connector/mod.rs:250` |
| sym-ef774811a957059cea80 | `pocketstation::connector::worker::ConnectorFactory` | trait | Implement this trait to provide connector behavior to PocketStation; its methods define the preparation and runtime contract. | `src/connector/worker/mod.rs:17` |
| sym-d5b0bf5fc25d3e2230c9 | `pocketstation::connector::worker::ConnectorWorker` | trait | Implement this trait to provide connector worker behavior to PocketStation; its methods define the preparation and runtime contract. | `src/connector/worker/mod.rs:32` |
| sym-2c2a9d5455c4d0a0a073 | `pocketstation::connector::worker::driver::ConnectorDriver` | trait | Provider-specific behavior executed on Core's bounded connector worker. | `src/connector/worker/driver.rs:92` |
| sym-7047fbea68de3230c646 | `pocketstation::connector::worker::driver::ConnectorDriverFactory` | trait | Prepares provider state while Core retains receiver and lifecycle authority. | `src/connector/worker/driver.rs:123` |
| sym-20504ee51243d676efd2 | `pocketstation::connector::ConnectorDeclarationError::Configuration` | variant | Classifies a failure at the configuration stage or component of `ConnectorDeclarationError`. | `src/connector/mod.rs:240` |
| sym-ea7f89ebcc7ce3d40861 | `pocketstation::connector::ConnectorDeclarationError::Session` | variant | Classifies a failure at the session stage or component of `ConnectorDeclarationError`. | `src/connector/mod.rs:242` |
| sym-5ca718d6ae8b88bfcc8b | `pocketstation::connector::ConnectorDeclarationError::WrongSession` | variant | Reports that session does not match the required identity or contract. | `src/connector/mod.rs:235` |
| sym-858b5620a5b6192d5543 | `pocketstation::connector::ConnectorObservationLookupError::WrongSession` | variant | Reports that session does not match the required identity or contract. | `src/connector/mod.rs:248` |
| sym-5ea4372428f39cc8c7cb | `pocketstation::connector::ConnectorRegistrationError::InvalidManifest` | variant | Reports that the supplied manifest is invalid. | `src/connector/mod.rs:227` |
| sym-d24bef3f6a3ec69c07c9 | `pocketstation::connector::ConnectorRegistrationError::Session` | variant | Classifies a failure at the session stage or component of `ConnectorRegistrationError`. | `src/connector/mod.rs:229` |
| sym-0ae943d9a157383f9e56 | `pocketstation::connector::configuration::ConnectorConfigurationConstraint::NonEmpty` | variant | Represents the non empty alternative defined by `ConnectorConfigurationConstraint`. | `src/connector/configuration.rs:160` |
| sym-14e3d4519ff8e1e24eb4 | `pocketstation::connector::configuration::ConnectorConfigurationConstraint::OneOf` | variant | Represents the one of alternative defined by `ConnectorConfigurationConstraint`. | `src/connector/configuration.rs:164` |
| sym-cd73fd2f20a364ef3f88 | `pocketstation::connector::configuration::ConnectorConfigurationConstraint::SignedRange` | variant | Represents the signed range alternative defined by `ConnectorConfigurationConstraint`. | `src/connector/configuration.rs:162` |
| sym-4c37185c18ca3ecfee2c | `pocketstation::connector::configuration::ConnectorConfigurationConstraint::TextLengthBytes` | variant | Represents the text length bytes alternative defined by `ConnectorConfigurationConstraint`. | `src/connector/configuration.rs:161` |
| sym-9b15ab1a2019e876ffb7 | `pocketstation::connector::configuration::ConnectorConfigurationConstraint::UnsignedRange` | variant | Represents the unsigned range alternative defined by `ConnectorConfigurationConstraint`. | `src/connector/configuration.rs:163` |
| sym-d2c747adcfa98684f2af | `pocketstation::connector::configuration::ConnectorConfigurationErrorCode::ConstraintViolation` | variant | Classifies a failure at the constraint violation stage or component of `ConnectorConfigurationErrorCode`. | `src/connector/configuration.rs:576` |
| sym-8a104dba8f99928965f2 | `pocketstation::connector::configuration::ConnectorConfigurationErrorCode::DuplicateField` | variant | Reports that field duplicates an existing declaration or record. | `src/connector/configuration.rs:570` |
| sym-9e2f2b50b31db44e2e64 | `pocketstation::connector::configuration::ConnectorConfigurationErrorCode::EmptySecret` | variant | Reports that secret is empty. | `src/connector/configuration.rs:578` |
| sym-bab6c1f53caaf4883592 | `pocketstation::connector::configuration::ConnectorConfigurationErrorCode::InvalidSchema` | variant | Reports that the supplied schema is invalid. | `src/connector/configuration.rs:569` |
| sym-b124cb29d6a7033b8828 | `pocketstation::connector::configuration::ConnectorConfigurationErrorCode::InvalidValue` | variant | Reports that the supplied value is invalid. | `src/connector/configuration.rs:575` |
| sym-d974b1c2473aab3f1804 | `pocketstation::connector::configuration::ConnectorConfigurationErrorCode::MissingRequiredField` | variant | Reports that the required required field is missing. | `src/connector/configuration.rs:573` |
| sym-1f47f8b885b2294727a6 | `pocketstation::connector::configuration::ConnectorConfigurationErrorCode::SecretClassificationMismatch` | variant | Reports that secret classification does not match the expected contract. | `src/connector/configuration.rs:580` |
| sym-dd8a399b30db92f5508d | `pocketstation::connector::configuration::ConnectorConfigurationErrorCode::SecretDefaultForbidden` | variant | Reports that secret default is forbidden by the declared safety contract. | `src/connector/configuration.rs:579` |
| sym-e69e9e65a60e3b2f0800 | `pocketstation::connector::configuration::ConnectorConfigurationErrorCode::TooManyFields` | variant | Reports that the number of fields exceeds the supported limit. | `src/connector/configuration.rs:571` |
| sym-a51c4ed347407ec0d7a3 | `pocketstation::connector::configuration::ConnectorConfigurationErrorCode::UnexpectedSensitiveValue` | variant | Reports that sensitive value is not valid in the current protocol or lifecycle state. | `src/connector/configuration.rs:581` |
| sym-b96b2aede72dccce9c79 | `pocketstation::connector::configuration::ConnectorConfigurationErrorCode::UnknownField` | variant | Reports that the referenced field is not declared or registered. | `src/connector/configuration.rs:572` |
| sym-015cea668b34afb11177 | `pocketstation::connector::configuration::ConnectorConfigurationErrorCode::ValueTooLarge` | variant | Reports that value exceeds the supported size limit. | `src/connector/configuration.rs:577` |
| sym-c77d1e603732e42edaed | `pocketstation::connector::configuration::ConnectorConfigurationErrorCode::WrongType` | variant | Reports that type does not match the required identity or contract. | `src/connector/configuration.rs:574` |
| sym-e8f46354a1f342d259f3 | `pocketstation::connector::configuration::ConnectorConfigurationRequirement::Default` | variant | Declares the connector configuration field to be default. | `src/connector/configuration.rs:155` |
| sym-7a7905799d55908ac371 | `pocketstation::connector::configuration::ConnectorConfigurationRequirement::Optional` | variant | Declares the connector configuration field to be optional. | `src/connector/configuration.rs:154` |
| sym-d790f641e6117466fe75 | `pocketstation::connector::configuration::ConnectorConfigurationRequirement::Required` | variant | Declares the connector configuration field to be required. | `src/connector/configuration.rs:153` |
| sym-c440607f532e230e8c95 | `pocketstation::connector::configuration::ConnectorConfigurationValue::Boolean` | variant | Represents the boolean alternative defined by `ConnectorConfigurationValue`. | `src/connector/configuration.rs:68` |
| sym-c0a0d348fa6e4f8b6e3f | `pocketstation::connector::configuration::ConnectorConfigurationValue::ByteCount` | variant | Represents the byte count alternative defined by `ConnectorConfigurationValue`. | `src/connector/configuration.rs:72` |
| sym-f8bc83ad8a328a716dd9 | `pocketstation::connector::configuration::ConnectorConfigurationValue::DurationMilliseconds` | variant | Represents the duration milliseconds alternative defined by `ConnectorConfigurationValue`. | `src/connector/configuration.rs:71` |
| sym-536af29c80dc53a8eb8c | `pocketstation::connector::configuration::ConnectorConfigurationValue::Secret` | variant | Represents the secret alternative defined by `ConnectorConfigurationValue`. | `src/connector/configuration.rs:73` |
| sym-81c41d1e7047ecdf7412 | `pocketstation::connector::configuration::ConnectorConfigurationValue::SignedInteger` | variant | Represents the signed integer alternative defined by `ConnectorConfigurationValue`. | `src/connector/configuration.rs:69` |
| sym-2f49577ba2188ac42094 | `pocketstation::connector::configuration::ConnectorConfigurationValue::Text` | variant | Represents the text alternative defined by `ConnectorConfigurationValue`. | `src/connector/configuration.rs:67` |
| sym-2dfed62a999d185cde99 | `pocketstation::connector::configuration::ConnectorConfigurationValue::UnsignedInteger` | variant | Represents the unsigned integer alternative defined by `ConnectorConfigurationValue`. | `src/connector/configuration.rs:70` |
| sym-504a0756b59d2900d994 | `pocketstation::connector::configuration::ConnectorConfigurationValueKind::Boolean` | variant | Declares that a connector configuration value is encoded as boolean. | `src/connector/configuration.rs:57` |
| sym-9dfa7cf35f8bffb2a344 | `pocketstation::connector::configuration::ConnectorConfigurationValueKind::ByteCount` | variant | Declares that a connector configuration value is encoded as byte count. | `src/connector/configuration.rs:61` |
| sym-a9b80443061bd30d4c97 | `pocketstation::connector::configuration::ConnectorConfigurationValueKind::DurationMilliseconds` | variant | Declares that a connector configuration value is encoded as duration milliseconds. | `src/connector/configuration.rs:60` |
| sym-5e78dd1a417086528f74 | `pocketstation::connector::configuration::ConnectorConfigurationValueKind::Secret` | variant | Declares that a connector configuration value is encoded as secret. | `src/connector/configuration.rs:62` |
| sym-190faf4d738b28a1d9b8 | `pocketstation::connector::configuration::ConnectorConfigurationValueKind::SignedInteger` | variant | Declares that a connector configuration value is encoded as signed integer. | `src/connector/configuration.rs:58` |
| sym-b3fc65ee658e3effe93f | `pocketstation::connector::configuration::ConnectorConfigurationValueKind::Text` | variant | Declares that a connector configuration value is encoded as text. | `src/connector/configuration.rs:56` |
| sym-be10276b6362605e6101 | `pocketstation::connector::configuration::ConnectorConfigurationValueKind::UnsignedInteger` | variant | Declares that a connector configuration value is encoded as unsigned integer. | `src/connector/configuration.rs:59` |
| sym-52d3e5e7e67b686743f8 | `pocketstation::connector::error::ConnectorErrorBuildError::EmptyMessage` | variant | Reports that message is empty. | `src/connector/error.rs:186` |
| sym-9e361554f4ebbe14a874 | `pocketstation::connector::error::ConnectorErrorBuildError::MessageTooLarge` | variant | Reports that message exceeds the supported size limit. | `src/connector/error.rs:188` |
| sym-c010bfd87b0061c360b9 | `pocketstation::connector::error::ConnectorErrorCodeError::Empty` | variant | Represents an empty value or collection. | `src/connector/error.rs:52` |
| sym-c52cbc6886f8ef27ff7a | `pocketstation::connector::error::ConnectorErrorCodeError::InvalidCharacter` | variant | Reports that the supplied character is invalid. | `src/connector/error.rs:56` |
| sym-cf47009e5570ae11db79 | `pocketstation::connector::error::ConnectorErrorCodeError::TooLong` | variant | Classifies a failure at the too long stage or component of `ConnectorErrorCodeError`. | `src/connector/error.rs:54` |
| sym-f576b14caa64fd181bd9 | `pocketstation::connector::error::ConnectorErrorStage::Configuration` | variant | Identifies the configuration state or stage represented by `ConnectorErrorStage`. | `src/connector/error.rs:61` |
| sym-fd87d9371e6f48e38da2 | `pocketstation::connector::error::ConnectorErrorStage::Delivery` | variant | Identifies the delivery state or stage represented by `ConnectorErrorStage`. | `src/connector/error.rs:65` |
| sym-739b90f7a29cc3393e58 | `pocketstation::connector::error::ConnectorErrorStage::Join` | variant | Identifies the join state or stage represented by `ConnectorErrorStage`. | `src/connector/error.rs:68` |

## Interpretation

The **Connector API** inventory records compiler-visible or extracted evidence at the frozen snapshot. A field marked unknown or not declared remains outside the published guarantee; use the native signature, owning error type, and cited test before relying on panic, blocking, cancellation, ordering, limits, retry, or recovery behavior.

## Related documentation

- [Glossary](/docs/glossary.md)
- [PocketStation](/README.md)
- [Author a connector](/docs/guides/connectors.md)
- [Behavior evidence index](/docs/reference/behavior-evidence.md)
- [Configuration reference](/docs/reference/configuration.md)
- [Protocol surface index](/docs/reference/protocol-surface.md)
- [Rust API reference](/docs/reference/rust-api.md)
- [A connector is not ready](/docs/troubleshooting/connector-readiness.md)

## Evidence boundary

The claims on **Connector API** are anchored to Git snapshot `136e74888962558aa846d3143a19136a70936f45` and these primary files:

- `src/connector/mod.rs:61-65` (`DIRECT`)
- `src/connector/mod.rs:62-62` (`DIRECT`)
- `src/connector/mod.rs:63-63` (`DIRECT`)
- `src/connector/mod.rs:64-64` (`DIRECT`)
- `src/connector/mod.rs:68-81` (`DIRECT`)
- `src/connector/mod.rs:88-105` (`DIRECT`)
- `src/connector/mod.rs:112-117` (`DIRECT`)
- `src/connector/mod.rs:119-121` (`DIRECT`)
- `src/connector/mod.rs:124-124` (`DIRECT`)
- `src/connector/mod.rs:125-129` (`DIRECT`)
- `src/connector/mod.rs:126-126` (`DIRECT`)
- `src/connector/mod.rs:127-127` (`DIRECT`)
- `src/connector/mod.rs:128-128` (`DIRECT`)
- `src/connector/mod.rs:132-134` (`DIRECT`)
- `src/connector/mod.rs:136-138` (`DIRECT`)
- `src/connector/mod.rs:140-151` (`DIRECT`)
- `src/connector/mod.rs:153-157` (`DIRECT`)
- `src/connector/mod.rs:159-179` (`DIRECT`)
- `src/connector/mod.rs:182-184` (`DIRECT`)
- `src/connector/mod.rs:183-183` (`DIRECT`)
- `src/connector/mod.rs:187-189` (`DIRECT`)
- `src/connector/mod.rs:191-200` (`DIRECT`)
- `src/connector/mod.rs:204-221` (`DIRECT`)
- `src/connector/mod.rs:224-224` (`DIRECT`)

For **Connector API**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.
