# Connector API

<!-- claims: CLM-REF-009-CAP-001,CLM-REF-009-CAP-002,CLM-REF-009-SOURCE-001 -->

## Scope

- **Declare connector manifests and configuration.** Describe connector identity, ports, configuration schema, secrets, and delivery policy without embedding a provider protocol in Core.
- **Run connector workers.** Supervise connector delivery, acknowledgement, readiness, cancellation, drain, and abort while reporting retry attempts and typed retryability.

The scope of **Connector API** ends at the native contracts and executable conditions cited below. Platform qualification, performance, retry, and delivery require their own explicit evidence.

## Reference authority

For **Connector API**, the generated [docs.rs API](https://docs.rs/pocketstation/latest/pocketstation/) provides compiler-rendered signatures and navigation. This repository page adds the frozen evidence identifiers, responsibilities, and cross-boundary interpretation used by the documentation verifier.

## Public surface

| Evidence record | Declaration | Kind | Purpose | Source |
|---|---|---|---|---|
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
| sym-71c8248552577ce31a95 | `capabilities` | function | Returns the capabilities held by `ConnectorManifest`. | `src/connector/manifest.rs:152` |
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
| sym-263af9445d9a180f56bc | `delivery_readiness` | function | Returns the delivery readiness associated with `ConnectorServiceStatus`. | `src/connector/status.rs:42` |
| sym-6bc2402fe269fe7f6b4e | `deprecated` | function | Returns the deprecated held by `ConnectorConfigurationField`. | `src/connector/configuration.rs:201` |
| sym-a1444469e193165c6f22 | `deprecation` | function | Returns the deprecation held by `ConnectorConfigurationField`. | `src/connector/configuration.rs:226` |
| sym-91a5f8f773d352edab66 | `documentation` | function | Returns the documentation held by `ConnectorConfigurationField`. | `src/connector/configuration.rs:218` |
| sym-4b3bc0edf728e8a49312 | `documentation` | function | Returns the documentation held by `ConnectorCapability`. | `src/connector/manifest.rs:34` |
| sym-fdf4039f27c61720c6a2 | `documentation` | function | Returns the documentation held by `ConnectorRequirement`. | `src/connector/manifest.rs:69` |
| sym-7e10b6736a3774e1a045 | `drop` | function | Releases resources owned by `ConnectorSecret`. | `src/connector/configuration.rs:49` |
| sym-10fb748905467401f2f8 | `edge_contract` | function | Returns the edge contract held by `ConnectorInputDescriptor`. | `src/connector/worker/driver.rs:52` |
| sym-84dc2a5a1d2002f1b94c | `encode` | function | Encodes input through `ConnectorConfigurationRecord`. | `src/connector/transport.rs:61` |
| sym-2519efc461a4e280fd1b | `encode` | function | Encodes input through `ConnectorAudioRecord`. | `src/connector/transport.rs:347` |
| sym-6ca9531125094143aa37 | `endpoint_id` | function | Returns the endpoint identifier held by `ConnectorInputDescriptor`. | `src/connector/worker/driver.rs:28` |
| sym-267630205817f07547fe | `endpoint_observations` | function | Returns the endpoint observations held by `ConnectorContext`. | `src/connector/worker/coordination.rs:139` |
| sym-fc869d5bdd95e2de6ca5 | `expose_secret` | function | Exposes the secret to the owning connector during setup or worker use. | `src/connector/configuration.rs:37` |
| sym-e6c0ce7d51dfcbaf96e4 | `failure` | function | Returns the failure held by `ConnectorRunOutcome`. | `src/connector/worker/mod.rs:55` |
| sym-b603c77500103cbf4489 | `failure_threshold` | function | Returns the failure threshold held by `ConnectorReadinessPolicy`. | `src/connector/readiness.rs:55` |
| sym-2145392fb082e13eb627 | `field` | function | Returns the field held by `ConnectorConfigurationSchema`. | `src/connector/configuration.rs:255` |
| sym-fc4c5f29bbc17d42d8db | `field` | function | Returns the field held by `ConnectorConfigurationError`. | `src/connector/configuration.rs:619` |
| sym-5b8b978acff4848278a9 | `fields` | function | Returns the fields held by `ConnectorConfigurationSchema`. | `src/connector/configuration.rs:251` |
| sym-fb998d5e97976d9cf219 | `fmt` | function | Formats `ConnectorSecret` with the requested formatter. | `src/connector/configuration.rs:43` |
| sym-64b09159091bd3b96f5b | `fmt` | function | Formats `ConnectorErrorCode` with the requested formatter. | `src/connector/error.rs:35` |
| sym-e029f0d778ba91592b0f | `fmt` | function | Formats `ConnectorErrorCode` with the requested formatter. | `src/connector/error.rs:44` |
| sym-6fa3e8d79c890c7717ff | `from_item` | function | Creates `ConnectorAudioRecord` from item. | `src/connector/transport.rs:314` |
| sym-33b9ee5cdb6059b6e5f9 | `from_resolved` | function | Creates `ConnectorConfigurationRecord` from resolved. | `src/connector/transport.rs:45` |
| sym-a8f5198e79b286269788 | `get` | function | Returns the value held by `ConnectorConfiguration`. | `src/connector/configuration.rs:134` |
| sym-4d9bc8528c0202d5425f | `get` | function | Returns the value held by `ResolvedConnectorConfiguration`. | `src/connector/configuration.rs:394` |
| sym-8540ee158dc2da2131fc | `health` | function | Returns the health held by `ConnectorServiceStatus`. | `src/connector/status.rs:46` |
| sym-8e222ad1c1c59e1ccb04 | `health_reason_code` | function | Returns the health reason code held by `ConnectorServiceStatus`. | `src/connector/status.rs:58` |
| sym-59a056aeb32ea6af5817 | `id` | function | Returns the id held by `ConnectorCapability`. | `src/connector/manifest.rs:30` |
| sym-d1a311cfb7fb6b0beb5a | `id` | function | Returns the id held by `ConnectorRequirement`. | `src/connector/manifest.rs:61` |
| sym-126179a82c1dac44596e | `input` | function | Returns the input held by `ConnectorItem`. | `src/connector/worker/driver.rs:74` |
| sym-3625c2cf36ae58dfe77c | `insert` | function | Inserts a typed configuration value into `ConnectorConfiguration`. | `src/connector/configuration.rs:126` |
| sym-478867138b32cee52cc9 | `into_configuration` | function | Converts `ConnectorConfigurationRecord` into configuration. | `src/connector/transport.rs:57` |
| sym-1adbe724a6a8d4fff4de | `into_endpoint_failure` | function | Converts `ConnectorError` into endpoint failure. | `src/connector/error.rs:125` |
| sym-b419412d315c2cb0fede | `is_abort_requested` | function | Returns whether abort requested applies to `ConnectorContext`. | `src/connector/worker/coordination.rs:36` |
| sym-3a89509bef9042425729 | `is_empty` | function | Returns whether `ConnectorConfiguration` contains no values. | `src/connector/configuration.rs:146` |
| sym-0ced882e7a0707ac4cf2 | `is_stop_requested` | function | Returns whether stop requested applies to `ConnectorContext`. | `src/connector/worker/coordination.rs:28` |
| sym-b513b191cf132d40afa6 | `iter` | function | Iterates over the values held by `ConnectorConfiguration`. | `src/connector/configuration.rs:138` |
| sym-f8a82a36670eedb24825 | `iter` | function | Iterates over the values held by `ResolvedConnectorConfiguration`. | `src/connector/configuration.rs:398` |
| sym-de42a0012afbd21fb026 | `kind` | function | Returns the kind represented by `ConnectorConfigurationValue`. | `src/connector/configuration.rs:77` |
| sym-c170db02f7c98809c93c | `last_transition_elapsed_ns` | function | Returns the last transition elapsed nanoseconds held by `ConnectorServiceStatus`. | `src/connector/status.rs:70` |
| sym-6a4f6ad61d1cb38db315 | `len` | function | Returns the number of values held by `ConnectorConfiguration`. | `src/connector/configuration.rs:142` |
| sym-d0ce46423a79b4284e2b | `manifest` | function | Returns the manifest held by `Connector`. | `src/connector/mod.rs:119` |
| sym-17ad3cce17ae0a5d4250 | `manifest` | function | Returns the manifest held by `RegisteredConnector`. | `src/connector/mod.rs:136` |
| sym-d467250e43d30e9cf376 | `manifest_revision` | function | Returns the manifest revision held by `ConnectorManifest`. | `src/connector/manifest.rs:128` |
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
| sym-41c19799e4db0d0dd702 | `node` | function | Returns the node held by `ConnectorManifest`. | `src/connector/manifest.rs:140` |
| sym-5f51988bb89a156b5ab1 | `observation` | function | Returns the current observation exposed by `RegisteredConnector`. | `src/connector/mod.rs:140` |
| sym-0952d710b28badc9600c | `observations` | function | Returns the observations exposed by `RegisteredConnector`. | `src/connector/mod.rs:153` |
| sym-2cc9104058399195bb19 | `operator_id` | function | Returns the operator identifier held by `ConnectorManifest`. | `src/connector/manifest.rs:132` |
| sym-0f6c33d9765cf812be60 | `package_version` | function | Returns the package version held by `ConnectorManifest`. | `src/connector/manifest.rs:136` |
| sym-5e4327b53963a537d944 | `pocketstation::connector::sidecar::sidecar_connector_factory` | function | Creates a connector driver factory backed by the supplied sidecar process. | `src/connector/sidecar.rs:264` |
| sym-6fe2c67fee8fd8d4039e | `port_name` | function | Returns the port name held by `ConnectorAudioRecord`. | `src/connector/transport.rs:335` |
| sym-b5e5c0c3e0b0d5c896a4 | `port_name` | function | Returns the port name held by `ConnectorInputDescriptor`. | `src/connector/worker/driver.rs:40` |
| sym-f8d30689c2c6840b3836 | `preparation_group` | function | Returns the preparation group associated with `SidecarConnectorDriverFactory`. | `src/connector/sidecar.rs:39` |
| sym-40b458a6269e8e0c2262 | `prepare` | function | Prepares resources required by `SidecarConnectorDriverFactory`. | `src/connector/sidecar.rs:49` |
| sym-f2c2a0f54a85d4eebaf3 | `probe_interval` | function | Returns the probe interval held by `ConnectorReadinessPolicy`. | `src/connector/readiness.rs:47` |
| sym-0f013d2539513ea3dc69 | `process` | function | Processes an input value through `SidecarConnectorDriverFactory`. | `src/connector/sidecar.rs:33` |
| sym-fb62d7a4401cec2a8906 | `readiness` | function | Returns the readiness held by `ConnectorManifest`. | `src/connector/manifest.rs:148` |
| sym-c04c490781b939e286b3 | `readiness_reason_code` | function | Returns the readiness reason code held by `ConnectorServiceStatus`. | `src/connector/status.rs:54` |
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
| sym-a69045a3fb6aaed54980 | `required` | function | Returns the required held by `ConnectorRequirement`. | `src/connector/manifest.rs:65` |
| sym-3991bd370414c1a6bf57 | `requirement` | function | Returns the requirement held by `ConnectorConfigurationField`. | `src/connector/configuration.rs:214` |
| sym-1578e0c96d33ae028176 | `requirements` | function | Returns the requirements held by `ConnectorManifest`. | `src/connector/manifest.rs:156` |
| sym-9ff5d39a59fcef0a980f | `resolve` | function | Resolves `ConnectorConfigurationSchema` into its validated representation. | `src/connector/configuration.rs:259` |
| sym-c227984556b300940fc1 | `result` | function | Returns the result represented by `ConnectorRunOutcome`. | `src/connector/worker/mod.rs:59` |
| sym-b033b0817db4909e396c | `retryability` | function | Returns the retryability associated with `ConnectorError`. | `src/connector/error.rs:117` |
| sym-045acf7e73303738579f | `revision` | function | Returns the revision held by `ConnectorConfigurationSchema`. | `src/connector/configuration.rs:247` |
| sym-d2a03fe25ee3733863b5 | `revision` | function | Returns the revision held by `ConnectorServiceStatus`. | `src/connector/status.rs:66` |
| sym-aebbad9605773e0d6979 | `route_id` | function | Returns the route identifier held by `ConnectorInputDescriptor`. | `src/connector/worker/driver.rs:36` |
| sym-548ea228160fc75d2bad | `samples` | function | Returns the audio samples held by `ConnectorAudioRecord`. | `src/connector/transport.rs:343` |
| sym-85cdd76ca50eb5baf768 | `session_id` | function | Returns the session identifier held by `RegisteredConnector`. | `src/connector/mod.rs:132` |
| sym-63c54342848999d62803 | `set_connected` | function | Sets the connected used by `ConnectorContext`. | `src/connector/worker/coordination.rs:74` |
| sym-cf48b27584a245401247 | `set_degraded` | function | Sets the degraded used by `ConnectorContext`. | `src/connector/worker/coordination.rs:56` |
| sym-740bbd73e15643480eb9 | `set_healthy` | function | Sets the healthy used by `ConnectorContext`. | `src/connector/worker/coordination.rs:62` |
| sym-2b8d78ff36bb186d1d53 | `set_not_ready` | function | Sets the not ready used by `ConnectorContext`. | `src/connector/worker/coordination.rs:50` |
| sym-5319ff2f9f98c58f7c49 | `set_ready` | function | Sets the ready used by `ConnectorContext`. | `src/connector/worker/coordination.rs:44` |
| sym-2f689a4c0a801493252f | `set_reconnecting` | function | Sets the reconnecting used by `ConnectorContext`. | `src/connector/worker/coordination.rs:68` |
| sym-4a0b2aae18dbc0be8d91 | `shutdown_mode` | function | Returns the shutdown mode held by `ConnectorContext`. | `src/connector/worker/coordination.rs:32` |
| sym-901266c8f4f31ab58f4f | `sidecar` | function | Builds an outbound Connector backed by one bounded sidecar process. | `src/connector/mod.rs:112` |
| sym-63a584618047103b17fb | `signal_spec` | function | Returns the signal spec held by `ConnectorInputDescriptor`. | `src/connector/worker/driver.rs:44` |
| sym-73ace5aff979ab1a8403 | `snapshot` | function | Returns a point-in-time snapshot of `ConnectorObservationHandle`. | `src/connector/observations.rs:53` |
| sym-a2f7e8afbcf6f5834845 | `stage` | function | Returns the stage held by `ConnectorError`. | `src/connector/error.rs:113` |
| sym-3157359ad4c3cebcd1b3 | `startup_timeout` | function | Returns the startup timeout held by `ConnectorReadinessPolicy`. | `src/connector/readiness.rs:43` |
| sym-5a43501bd451af89b2a4 | `success` | function | Returns whether `ConnectorRunOutcome` completed successfully. | `src/connector/worker/mod.rs:51` |
| sym-59b3a78e38624726ca27 | `success_threshold` | function | Returns the success threshold held by `ConnectorReadinessPolicy`. | `src/connector/readiness.rs:51` |
| sym-3061c1a5028d4292b75d | `try_new` | function | Creates a new `ConnectorAudioRecord` after validating its inputs. | `src/connector/transport.rs:300` |
| sym-fec48c2902cf14237c10 | `validate` | function | Validates `ConnectorManifest` against its declared contract. | `src/connector/manifest.rs:160` |
| sym-94b5238b1063adc98ddc | `value_kind` | function | Returns the value kind held by `ConnectorConfigurationField`. | `src/connector/configuration.rs:210` |
| sym-8fc0f5e0f6045de84cce | `wait_for_stop` | function | Waits until a stop request is visible to `ConnectorContext`. | `src/connector/worker/coordination.rs:40` |
| sym-a745759ed1a6838f5b8b | `with` | function | Returns `ConnectorConfiguration` with the supplied entry applied. | `src/connector/configuration.rs:121` |
| sym-afd5dee85ea66002a474 | `with_capability` | function | Sets the capability on `ConnectorManifest` and returns the updated value. | `src/connector/manifest.rs:113` |
| sym-8942f83b86e8489b8716 | `with_constraint` | function | Sets the constraint on `ConnectorConfigurationField` and returns the updated value. | `src/connector/configuration.rs:195` |
| sym-8a30324e0b3660d9594b | `with_driver` | function | Builds a connector whose bounded receiver loop is owned by Core. | `src/connector/mod.rs:88` |
| sym-037be4868ac4e8b742fe | `with_requirement` | function | Sets the requirement on `ConnectorManifest` and returns the updated value. | `src/connector/manifest.rs:119` |
| sym-d23ab2d2747aa3aa43ab | `pocketstation::connector` | module | Types and operations for connector. | `src/connector/mod.rs:1` |
| sym-bf2224c32fb3c2eebfe1 | `pocketstation::connector::Connector` | struct | Declares a connector endpoint and the manifest-backed configuration used to instantiate it. | `src/connector/mod.rs:61` |
| sym-5460a717928003664850 | `pocketstation::connector::RegisteredConnector` | struct | Retains a connector declaration after its factory has been registered with the node registry. | `src/connector/mod.rs:125` |
| sym-c394549aa6a4d387a0ea | `pocketstation::connector::configuration::ConnectorConfiguration` | struct | Configures connector behavior at its owning API boundary. | `src/connector/configuration.rs:111` |
| sym-32e40121c0704e8a80b9 | `pocketstation::connector::configuration::ConnectorConfigurationError` | struct | Reports a connector configuration error. | `src/connector/configuration.rs:608` |
| sym-ba8785d08b910782a43c | `pocketstation::connector::configuration::ConnectorConfigurationField` | struct | Declares one typed connector configuration field and its validation constraints. | `src/connector/configuration.rs:168` |
| sym-808f50a1a1dc0eb0a558 | `pocketstation::connector::configuration::ConnectorConfigurationSchema` | struct | Validates connector configuration values against the manifest-declared field set. | `src/connector/configuration.rs:232` |
| sym-6858dc2f8d963ec47672 | `pocketstation::connector::configuration::ConnectorSecret` | struct | Owns a connector secret with redacted diagnostics and byte clearing on explicit reset or drop. | `src/connector/configuration.rs:11` |
| sym-d681d41880e9d237c553 | `pocketstation::connector::configuration::ResolvedConnectorConfiguration` | struct | Configures resolved connector behavior at its owning API boundary. | `src/connector/configuration.rs:391` |
| sym-097d24a379f92bcf58d5 | `pocketstation::connector::error::ConnectorError` | struct | Reports a connector error. | `src/connector/error.rs:80` |
| sym-e9aa865eabc0fc3e8b01 | `pocketstation::connector::error::ConnectorErrorCode` | struct | Carries the stable external error code exported for a connector failure. | `src/connector/error.rs:10` |
| sym-2bc92b241327b4dea7a6 | `pocketstation::connector::manifest::ConnectorCapability` | struct | Declares a capability advertised by a connector manifest. | `src/connector/manifest.rs:12` |
| sym-aa1b8eb30f0c6c52304d | `pocketstation::connector::manifest::ConnectorManifest` | struct | Describes the connector manifest contract. | `src/connector/manifest.rs:75` |
| sym-c93fcf7f6eb3edac0970 | `pocketstation::connector::manifest::ConnectorRequirement` | struct | Declares a host or configuration requirement that must be satisfied before connector use. | `src/connector/manifest.rs:40` |
| sym-15527a12bc2212bfbc4e | `pocketstation::connector::observations::ConnectorObservationHandle` | struct | Owns bounded access to connector observation. | `src/connector/observations.rs:15` |
| sym-a4b0eed71143f95a96ce | `pocketstation::connector::observations::ConnectorObservations` | struct | Reports the connector observations collected at an observation boundary. | `src/connector/observations.rs:158` |
| sym-a9dc2b1ee0887bb339a1 | `pocketstation::connector::observations::ConnectorRuntimeObservations` | struct | Reports the connector runtime observations collected at an observation boundary. | `src/connector/observations.rs:168` |
| sym-a1991da6be24e32179b5 | `pocketstation::connector::readiness::ConnectorReadinessPolicy` | struct | Configures connector readiness behavior at its owning API boundary. | `src/connector/readiness.rs:7` |
| sym-f29b4e150e91ea80e409 | `pocketstation::connector::sidecar::SidecarConnectorDriverFactory` | struct | Adapts a bounded PocketStation sidecar process to the Connector driver SPI. | `src/connector/sidecar.rs:24` |
| sym-bafacbbb0b5cd04c244c | `pocketstation::connector::status::ConnectorServiceStatus` | struct | Reports the structured connector service status. | `src/connector/status.rs:30` |
| sym-4148e6877c42b9e51fc0 | `pocketstation::connector::transport::ConnectorAudioMetadata` | struct | Carries source, stream, timing, and format metadata beside a connector audio record. | `src/connector/transport.rs:281` |
| sym-92516bc76b6180aba4c9 | `pocketstation::connector::transport::ConnectorAudioRecord` | struct | Records one immutable connector audio observation. | `src/connector/transport.rs:293` |
| sym-fe7c904d3dc0792c3eb2 | `pocketstation::connector::transport::ConnectorConfigurationRecord` | struct | Canonical typed configuration handed to a connector sidecar during its bounded Configure handshake. Secret classification survives the boundary; Debug output continues to redact secret values. | `src/connector/transport.rs:42` |
| sym-6cb7ef6c669d4137bb26 | `pocketstation::connector::worker::ConnectorRunOutcome` | struct | Reports the structured connector run outcome. | `src/connector/worker/mod.rs:42` |
| sym-65e57047a4d359b7a97e | `pocketstation::connector::worker::coordination::ConnectorContext` | struct | Carries the inputs and runtime context required to connector. | `src/connector/worker/coordination.rs:14` |
| sym-9effe975facba5bc3f3e | `pocketstation::connector::worker::driver::ConnectorInputDescriptor` | struct | Immutable Session and graph metadata for one connector input. | `src/connector/worker/driver.rs:16` |
| sym-2f0da758afdc3b6b69c5 | `ConnectorAudioMetadata::channels` | struct_field | Stores the channels used by `ConnectorAudioMetadata`. | `src/connector/transport.rs:288` |
| sym-3ace85340518651afe09 | `ConnectorAudioMetadata::connector_id` | struct_field | Identifies the connector identifier recorded by `ConnectorAudioMetadata`. | `src/connector/transport.rs:283` |
| sym-71e25f3826ba9a0741d1 | `ConnectorAudioMetadata::endpoint_id` | struct_field | Identifies the endpoint identifier recorded by `ConnectorAudioMetadata`. | `src/connector/transport.rs:282` |
| sym-e8b9e5da0c0049063c5a | `ConnectorAudioMetadata::lineage` | struct_field | Stores the lineage used by `ConnectorAudioMetadata`. | `src/connector/transport.rs:286` |
| sym-ac7467ded79526c2d44d | `ConnectorAudioMetadata::route_id` | struct_field | Identifies the route identifier recorded by `ConnectorAudioMetadata`. | `src/connector/transport.rs:284` |
| sym-47fae17b4f525f983b7b | `ConnectorAudioMetadata::sample_format` | struct_field | Stores the sample format used by `ConnectorAudioMetadata`. | `src/connector/transport.rs:289` |
| sym-4c4a5acb8c7eb6ac25dc | `ConnectorAudioMetadata::sample_rate_hz` | struct_field | Stores the sample rate value for `ConnectorAudioMetadata`, in hertz. | `src/connector/transport.rs:287` |
| sym-305e404d1e4cffe11d5d | `ConnectorAudioMetadata::stream_id` | struct_field | Identifies the stream identifier recorded by `ConnectorAudioMetadata`. | `src/connector/transport.rs:285` |
| sym-c1aa72ba989b2ea81d60 | `ConnectorConfigurationConstraint::SignedRange::maximum` | struct_field | Sets the inclusive maximum accepted by `SignedRange`. | `src/connector/configuration.rs:162` |
| sym-e97a1b6700d44890ee50 | `ConnectorConfigurationConstraint::SignedRange::minimum` | struct_field | Sets the inclusive minimum accepted by `SignedRange`. | `src/connector/configuration.rs:162` |
| sym-60adaf6075ef42855c17 | `ConnectorConfigurationConstraint::TextLengthBytes::maximum` | struct_field | Sets the inclusive maximum accepted by `TextLengthBytes`. | `src/connector/configuration.rs:161` |
| sym-4a258ea3c8220ff206c7 | `ConnectorConfigurationConstraint::TextLengthBytes::minimum` | struct_field | Sets the inclusive minimum accepted by `TextLengthBytes`. | `src/connector/configuration.rs:161` |
| sym-4b665a96a0a8f00b1f26 | `ConnectorConfigurationConstraint::UnsignedRange::maximum` | struct_field | Sets the inclusive maximum accepted by `UnsignedRange`. | `src/connector/configuration.rs:163` |
| sym-e964af1dd8e689bdfec9 | `ConnectorConfigurationConstraint::UnsignedRange::minimum` | struct_field | Sets the inclusive minimum accepted by `UnsignedRange`. | `src/connector/configuration.rs:163` |
| sym-8b85f3303edcd53ef767 | `ConnectorItem::Audio::frame` | struct_field | Stores the frame used by `Audio`. | `src/connector/worker/driver.rs:65` |
| sym-4c573d5e6c1089fc53f6 | `ConnectorItem::Audio::input` | struct_field | Stores the input used by `Audio`. | `src/connector/worker/driver.rs:64` |
| sym-d5d77e1694a64ab4a3c0 | `ConnectorItem::Signal::input` | struct_field | Stores the input used by `Signal`. | `src/connector/worker/driver.rs:68` |
| sym-80d3e1d741300efef18e | `ConnectorItem::Signal::signal` | struct_field | Stores the signal used by `Signal`. | `src/connector/worker/driver.rs:69` |
| sym-e93e9fabef5634350544 | `ConnectorManifestError::DuplicateManifestEntry::id` | struct_field | Identifies the id recorded by `DuplicateManifestEntry`. | `src/connector/manifest.rs:253` |
| sym-1a52074e315938edd04c | `ConnectorManifestError::UnsupportedApiRevision::requested` | struct_field | Stores the requested used by `UnsupportedApiRevision`. | `src/connector/manifest.rs:233` |
| sym-ba6c50b78ab59d74d0e7 | `ConnectorManifestError::UnsupportedApiRevision::supported` | struct_field | Stores the supported used by `UnsupportedApiRevision`. | `src/connector/manifest.rs:233` |
| sym-3a013d76ca19c8e58869 | `ConnectorObservations::failures_total` | struct_field | Counts the total number of failures observed by `ConnectorObservations`. | `src/connector/observations.rs:163` |
| sym-30ef5233fd334a57ff2a | `ConnectorObservations::last_error` | struct_field | Carries the last error reported by `ConnectorObservations`. | `src/connector/observations.rs:164` |
| sym-3548c3a6a9f166f49ffe | `ConnectorObservations::reconnects_total` | struct_field | Counts the total number of reconnects observed by `ConnectorObservations`. | `src/connector/observations.rs:162` |
| sym-abfaad9e23621d93cf3e | `ConnectorObservations::retry_attempts_total` | struct_field | Counts the total number of retry attempts observed by `ConnectorObservations`. | `src/connector/observations.rs:161` |
| sym-1cbd5d0daa005bb63d0d | `ConnectorObservations::service_status` | struct_field | Stores the service status used by `ConnectorObservations`. | `src/connector/observations.rs:159` |
| sym-1dd368e2615fc9948e4b | `ConnectorObservations::status_transitions_total` | struct_field | Counts the total number of status transitions observed by `ConnectorObservations`. | `src/connector/observations.rs:160` |
| sym-b54a9c09aa030986ea9d | `ConnectorRuntimeObservations::connector` | struct_field | Stores the connector used by `ConnectorRuntimeObservations`. | `src/connector/observations.rs:170` |
| sym-bd4272164ddd1dd1f602 | `ConnectorRuntimeObservations::endpoint` | struct_field | Stores the endpoint used by `ConnectorRuntimeObservations`. | `src/connector/observations.rs:171` |
| sym-2f4fe069489f47349703 | `ConnectorRuntimeObservations::endpoint_ids` | struct_field | Identifies the endpoint identifiers recorded by `ConnectorRuntimeObservations`. | `src/connector/observations.rs:169` |
| sym-d1153d617a98481c966e | `pocketstation::connector::ConnectorDeclarationError::WrongSession::registered` | struct_field | Stores the registered used by `WrongSession`. | `src/connector/mod.rs:236` |
| sym-3f52f0a98a39a82cb3cc | `pocketstation::connector::ConnectorDeclarationError::WrongSession::requested` | struct_field | Stores the requested used by `WrongSession`. | `src/connector/mod.rs:237` |
| sym-cfef2464d1289822c2ef | `pocketstation::connector::ConnectorObservationLookupError::WrongSession::registered` | struct_field | Stores the registered used by `WrongSession`. | `src/connector/mod.rs:249` |
| sym-c61cdedce41edfe48070 | `pocketstation::connector::ConnectorObservationLookupError::WrongSession::requested` | struct_field | Stores the requested used by `WrongSession`. | `src/connector/mod.rs:250` |
| sym-e38e2d3daefdbafe1737 | `pocketstation::connector::worker::ConnectorFactory` | trait | Implement this trait to provide connector behavior to PocketStation; its methods define the preparation and runtime contract. | `src/connector/worker/mod.rs:17` |
| sym-b8944599561449d3bff7 | `pocketstation::connector::worker::ConnectorWorker` | trait | Implement this trait to provide connector worker behavior to PocketStation; its methods define the preparation and runtime contract. | `src/connector/worker/mod.rs:32` |
| sym-0f52b43dab2320808210 | `pocketstation::connector::worker::driver::ConnectorDriver` | trait | Provider-specific behavior executed on Core's bounded connector worker. | `src/connector/worker/driver.rs:92` |
| sym-22009b8aa13b514c452d | `pocketstation::connector::worker::driver::ConnectorDriverFactory` | trait | Prepares provider state while Core retains receiver and lifecycle authority. | `src/connector/worker/driver.rs:123` |
| sym-414ec219914f7fff50e1 | `pocketstation::connector::ConnectorDeclarationError::Configuration` | variant | Reported when the owning operation encounters configuration. | `src/connector/mod.rs:240` |
| sym-acac6f2e7654fb5cf534 | `pocketstation::connector::ConnectorDeclarationError::Session` | variant | Reported when the owning operation encounters session. | `src/connector/mod.rs:242` |
| sym-5c585e04bab17d872bc7 | `pocketstation::connector::ConnectorDeclarationError::WrongSession` | variant | Reported when the owning operation encounters wrong session. | `src/connector/mod.rs:235` |
| sym-1b8436cfa27bb142abe9 | `pocketstation::connector::ConnectorObservationLookupError::WrongSession` | variant | Reported when the owning operation encounters wrong session. | `src/connector/mod.rs:248` |
| sym-569d7a72b3ea09b45095 | `pocketstation::connector::ConnectorRegistrationError::InvalidManifest` | variant | Reported when the owning operation encounters invalid manifest. | `src/connector/mod.rs:227` |
| sym-39b8c096d8fdc70c1b1b | `pocketstation::connector::ConnectorRegistrationError::Session` | variant | Reported when the owning operation encounters session. | `src/connector/mod.rs:229` |
| sym-e2a90b05dceed2364bbe | `pocketstation::connector::configuration::ConnectorConfigurationConstraint::NonEmpty` | variant | Represents the non empty alternative defined by `ConnectorConfigurationConstraint`. | `src/connector/configuration.rs:160` |
| sym-bfb6114ac108e8245be7 | `pocketstation::connector::configuration::ConnectorConfigurationConstraint::OneOf` | variant | Represents the one of alternative defined by `ConnectorConfigurationConstraint`. | `src/connector/configuration.rs:164` |
| sym-206d798b914bd816218a | `pocketstation::connector::configuration::ConnectorConfigurationConstraint::SignedRange` | variant | Represents the signed range alternative defined by `ConnectorConfigurationConstraint`. | `src/connector/configuration.rs:162` |
| sym-29cefd091e117ca08f9d | `pocketstation::connector::configuration::ConnectorConfigurationConstraint::TextLengthBytes` | variant | Represents the text length bytes alternative defined by `ConnectorConfigurationConstraint`. | `src/connector/configuration.rs:161` |
| sym-d56d2a4243ec22a6b185 | `pocketstation::connector::configuration::ConnectorConfigurationConstraint::UnsignedRange` | variant | Represents the unsigned range alternative defined by `ConnectorConfigurationConstraint`. | `src/connector/configuration.rs:163` |
| sym-024f6367fc4db4c187d5 | `pocketstation::connector::configuration::ConnectorConfigurationErrorCode::ConstraintViolation` | variant | Reported when the owning operation encounters constraint violation. | `src/connector/configuration.rs:576` |
| sym-073dfaa61e34aa943efb | `pocketstation::connector::configuration::ConnectorConfigurationErrorCode::DuplicateField` | variant | Reported when the owning operation encounters duplicate field. | `src/connector/configuration.rs:570` |
| sym-22a0ae098dfe3102c3cd | `pocketstation::connector::configuration::ConnectorConfigurationErrorCode::EmptySecret` | variant | Reported when the owning operation encounters empty secret. | `src/connector/configuration.rs:578` |
| sym-0982f4d2d3c1c6607a82 | `pocketstation::connector::configuration::ConnectorConfigurationErrorCode::InvalidSchema` | variant | Reported when the owning operation encounters invalid schema. | `src/connector/configuration.rs:569` |
| sym-d87dd87600a1e1310c80 | `pocketstation::connector::configuration::ConnectorConfigurationErrorCode::InvalidValue` | variant | Reported when the owning operation encounters invalid value. | `src/connector/configuration.rs:575` |
| sym-cf257a38a983c258c3e5 | `pocketstation::connector::configuration::ConnectorConfigurationErrorCode::MissingRequiredField` | variant | Reported when the owning operation encounters missing required field. | `src/connector/configuration.rs:573` |
| sym-75695c0e279415da1473 | `pocketstation::connector::configuration::ConnectorConfigurationErrorCode::SecretClassificationMismatch` | variant | Reported when the owning operation encounters secret classification mismatch. | `src/connector/configuration.rs:580` |
| sym-7c4c6a2817bde1668801 | `pocketstation::connector::configuration::ConnectorConfigurationErrorCode::SecretDefaultForbidden` | variant | Reported when the owning operation encounters secret default forbidden. | `src/connector/configuration.rs:579` |
| sym-6bcbb141cae8c6980bd3 | `pocketstation::connector::configuration::ConnectorConfigurationErrorCode::TooManyFields` | variant | Reported when the owning operation encounters too many fields. | `src/connector/configuration.rs:571` |
| sym-cf609fe340fb499e286b | `pocketstation::connector::configuration::ConnectorConfigurationErrorCode::UnexpectedSensitiveValue` | variant | Reported when the owning operation encounters unexpected sensitive value. | `src/connector/configuration.rs:581` |
| sym-dc8e1599236437fae1da | `pocketstation::connector::configuration::ConnectorConfigurationErrorCode::UnknownField` | variant | Reported when the owning operation encounters unknown field. | `src/connector/configuration.rs:572` |
| sym-7d5b7c50028c8197e54d | `pocketstation::connector::configuration::ConnectorConfigurationErrorCode::ValueTooLarge` | variant | Reported when the owning operation encounters value too large. | `src/connector/configuration.rs:577` |
| sym-71e40af7cb6a49ad09a7 | `pocketstation::connector::configuration::ConnectorConfigurationErrorCode::WrongType` | variant | Reported when the owning operation encounters wrong type. | `src/connector/configuration.rs:574` |
| sym-290aac9598f5cd15ff58 | `pocketstation::connector::configuration::ConnectorConfigurationRequirement::Default` | variant | Selects default behavior for `ConnectorConfigurationRequirement`. | `src/connector/configuration.rs:155` |
| sym-f56ddc63eff0274be451 | `pocketstation::connector::configuration::ConnectorConfigurationRequirement::Optional` | variant | Selects optional behavior for `ConnectorConfigurationRequirement`. | `src/connector/configuration.rs:154` |
| sym-303d8c9bcafb5e80ed2b | `pocketstation::connector::configuration::ConnectorConfigurationRequirement::Required` | variant | Selects required behavior for `ConnectorConfigurationRequirement`. | `src/connector/configuration.rs:153` |
| sym-d85ed1401f25f07d9c34 | `pocketstation::connector::configuration::ConnectorConfigurationValue::Boolean` | variant | Represents the boolean alternative defined by `ConnectorConfigurationValue`. | `src/connector/configuration.rs:68` |
| sym-8978c162f66dc04f10fa | `pocketstation::connector::configuration::ConnectorConfigurationValue::ByteCount` | variant | Represents the byte count alternative defined by `ConnectorConfigurationValue`. | `src/connector/configuration.rs:72` |
| sym-cba9e0960f91393975f2 | `pocketstation::connector::configuration::ConnectorConfigurationValue::DurationMilliseconds` | variant | Represents the duration milliseconds alternative defined by `ConnectorConfigurationValue`. | `src/connector/configuration.rs:71` |
| sym-262d6bd986bcc1c4f9e0 | `pocketstation::connector::configuration::ConnectorConfigurationValue::Secret` | variant | Represents the secret alternative defined by `ConnectorConfigurationValue`. | `src/connector/configuration.rs:73` |
| sym-fec451e1b7cd44dceae9 | `pocketstation::connector::configuration::ConnectorConfigurationValue::SignedInteger` | variant | Represents the signed integer alternative defined by `ConnectorConfigurationValue`. | `src/connector/configuration.rs:69` |
| sym-e722e96b996dedff423a | `pocketstation::connector::configuration::ConnectorConfigurationValue::Text` | variant | Represents the text alternative defined by `ConnectorConfigurationValue`. | `src/connector/configuration.rs:67` |
| sym-b7cb6b780f851a91c5fe | `pocketstation::connector::configuration::ConnectorConfigurationValue::UnsignedInteger` | variant | Represents the unsigned integer alternative defined by `ConnectorConfigurationValue`. | `src/connector/configuration.rs:70` |
| sym-40e42ad4df34751d0a32 | `pocketstation::connector::configuration::ConnectorConfigurationValueKind::Boolean` | variant | Selects boolean behavior for `ConnectorConfigurationValueKind`. | `src/connector/configuration.rs:57` |
| sym-ad75d5879007c17ec267 | `pocketstation::connector::configuration::ConnectorConfigurationValueKind::ByteCount` | variant | Selects byte count behavior for `ConnectorConfigurationValueKind`. | `src/connector/configuration.rs:61` |
| sym-1077ae22fa5720421f56 | `pocketstation::connector::configuration::ConnectorConfigurationValueKind::DurationMilliseconds` | variant | Selects duration milliseconds behavior for `ConnectorConfigurationValueKind`. | `src/connector/configuration.rs:60` |
| sym-d88618f6128c09d9c02f | `pocketstation::connector::configuration::ConnectorConfigurationValueKind::Secret` | variant | Selects secret behavior for `ConnectorConfigurationValueKind`. | `src/connector/configuration.rs:62` |
| sym-53e3ed96e46b4f50a42e | `pocketstation::connector::configuration::ConnectorConfigurationValueKind::SignedInteger` | variant | Selects signed integer behavior for `ConnectorConfigurationValueKind`. | `src/connector/configuration.rs:58` |
| sym-4f05ecc38abecab147c4 | `pocketstation::connector::configuration::ConnectorConfigurationValueKind::Text` | variant | Selects text behavior for `ConnectorConfigurationValueKind`. | `src/connector/configuration.rs:56` |
| sym-62ee1bc806bdef4fd95a | `pocketstation::connector::configuration::ConnectorConfigurationValueKind::UnsignedInteger` | variant | Selects unsigned integer behavior for `ConnectorConfigurationValueKind`. | `src/connector/configuration.rs:59` |
| sym-7daaa4b52df4150bfcb1 | `pocketstation::connector::error::ConnectorErrorBuildError::EmptyMessage` | variant | Reported when the owning operation encounters empty message. | `src/connector/error.rs:186` |
| sym-3f7aad7e44723bcabcea | `pocketstation::connector::error::ConnectorErrorBuildError::MessageTooLarge` | variant | Reported when the owning operation encounters message too large. | `src/connector/error.rs:188` |
| sym-5ea36a0347be90f004a9 | `pocketstation::connector::error::ConnectorErrorCodeError::Empty` | variant | Represents an empty value or collection. | `src/connector/error.rs:52` |
| sym-c939c0d9edbfc0c25711 | `pocketstation::connector::error::ConnectorErrorCodeError::InvalidCharacter` | variant | Reported when the owning operation encounters invalid character. | `src/connector/error.rs:56` |
| sym-b05538458a2e0ee3196d | `pocketstation::connector::error::ConnectorErrorCodeError::TooLong` | variant | Reported when the owning operation encounters too long. | `src/connector/error.rs:54` |
| sym-d13e4f83481022be2dfc | `pocketstation::connector::error::ConnectorErrorStage::Configuration` | variant | Identifies the configuration state or stage represented by `ConnectorErrorStage`. | `src/connector/error.rs:61` |
| sym-a7235d1700b2884a28c4 | `pocketstation::connector::error::ConnectorErrorStage::Delivery` | variant | Identifies the delivery state or stage represented by `ConnectorErrorStage`. | `src/connector/error.rs:65` |
| sym-d5f3649551282db001d3 | `pocketstation::connector::error::ConnectorErrorStage::Join` | variant | Identifies the join state or stage represented by `ConnectorErrorStage`. | `src/connector/error.rs:68` |

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

The claims on **Connector API** are anchored to Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `src/connector/mod.rs:1-253` (`DIRECT`)

For **Connector API**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.
