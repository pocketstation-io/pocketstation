# Connector API

<!-- claims: CLM-REF-009-CAP-001,CLM-REF-009-CAP-002,CLM-REF-009-SOURCE-001 -->

## Scope

- **Declare connector manifests and configuration.** Describe connector identity, ports, configuration schema, secrets, and delivery policy without embedding a provider protocol in Core.
- **Run connector workers.** Supervise connector delivery, acknowledgement, retry budgets, readiness, cancellation, drain, and abort.

These statements describe repository contracts at the documented snapshot. They do not extend platform qualification, performance, retry, or delivery guarantees beyond the native API contracts and executable evidence.

## Reference authority

The generated [docs.rs API](https://docs.rs/pocketstation/latest/pocketstation/) is the exhaustive symbol-level Rust reference. Human reference pages organize responsibilities and cross-boundary behavior; they do not duplicate every rustdoc signature.

## Public surface

| Declaration | Kind | Purpose | Source |
|---|---|---|---|
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
| `accepts_delivery` | function | Returns whether accepts delivery applies to `ConnectorDeliveryReadiness`. | `src/connector/status.rs:10` |
| `accepts_delivery` | function | Returns whether accepts delivery applies to `ConnectorServiceStatus`. | `src/connector/status.rs:74` |
| `api_revision` | function | Returns the API revision associated with `ConnectorManifest`. | `src/connector/manifest.rs:124` |
| `as_str` | function | Returns the stable string representation of `ConnectorConfigurationErrorCode`. | `src/connector/configuration.rs:585` |
| `as_str` | function | Returns the stable string representation of `ConnectorErrorCode`. | `src/connector/error.rs:29` |
| `capabilities` | function | Returns the capabilities associated with `ConnectorManifest`. | `src/connector/manifest.rs:152` |
| `code` | function | Returns the stable error or status code represented by `ConnectorConfigurationError`. | `src/connector/configuration.rs:615` |
| `code` | function | Returns the stable error or status code represented by `ConnectorError`. | `src/connector/error.rs:109` |
| `configuration` | function | Returns the configuration associated with `ConnectorManifest`. | `src/connector/manifest.rs:144` |
| `configuration` | function | Returns the configuration associated with `ConnectorConfigurationRecord`. | `src/connector/transport.rs:53` |
| `configuration` | function | Returns the configuration associated with `ConnectorInputDescriptor`. | `src/connector/worker/driver.rs:56` |
| `connector_id` | function | Returns the connector identifier associated with `ConnectorInputDescriptor`. | `src/connector/worker/driver.rs:32` |
| `constraints` | function | Returns the constraints associated with `ConnectorConfigurationField`. | `src/connector/configuration.rs:222` |
| `declare` | function | Adds the declaration represented by `RegisteredConnector` to its Session. | `src/connector/mod.rs:159` |
| `decode` | function | Decodes input through `ConnectorConfigurationRecord`. | `src/connector/transport.rs:93` |
| `decode` | function | Decodes input through `ConnectorAudioRecord`. | `src/connector/transport.rs:403` |
| `delivery_readiness` | function | Returns the delivery readiness associated with `ConnectorServiceStatus`. | `src/connector/status.rs:42` |
| `deprecated` | function | Returns the deprecated associated with `ConnectorConfigurationField`. | `src/connector/configuration.rs:201` |
| `deprecation` | function | Returns the deprecation associated with `ConnectorConfigurationField`. | `src/connector/configuration.rs:226` |
| `documentation` | function | Returns the documentation associated with `ConnectorConfigurationField`. | `src/connector/configuration.rs:218` |
| `documentation` | function | Returns the documentation associated with `ConnectorCapability`. | `src/connector/manifest.rs:34` |
| `documentation` | function | Returns the documentation associated with `ConnectorRequirement`. | `src/connector/manifest.rs:69` |
| `drop` | function | Releases resources owned by `ConnectorSecret`. | `src/connector/configuration.rs:49` |
| `edge_contract` | function | Returns the edge contract associated with `ConnectorInputDescriptor`. | `src/connector/worker/driver.rs:52` |
| `encode` | function | Encodes input through `ConnectorConfigurationRecord`. | `src/connector/transport.rs:61` |
| `encode` | function | Encodes input through `ConnectorAudioRecord`. | `src/connector/transport.rs:347` |
| `endpoint_id` | function | Returns the endpoint identifier associated with `ConnectorInputDescriptor`. | `src/connector/worker/driver.rs:28` |
| `endpoint_observations` | function | Returns the endpoint observations associated with `ConnectorContext`. | `src/connector/worker/coordination.rs:139` |
| `expose_secret` | function | Exposes the secret to the owning connector during setup or worker use. | `src/connector/configuration.rs:37` |
| `failure` | function | Returns the failure associated with `ConnectorRunOutcome`. | `src/connector/worker/mod.rs:55` |
| `failure_threshold` | function | Returns the failure threshold associated with `ConnectorReadinessPolicy`. | `src/connector/readiness.rs:55` |
| `field` | function | Returns the field associated with `ConnectorConfigurationSchema`. | `src/connector/configuration.rs:255` |
| `field` | function | Returns the field associated with `ConnectorConfigurationError`. | `src/connector/configuration.rs:619` |
| `fields` | function | Returns the fields associated with `ConnectorConfigurationSchema`. | `src/connector/configuration.rs:251` |
| `fmt` | function | Formats `ConnectorSecret` with the requested formatter. | `src/connector/configuration.rs:43` |
| `fmt` | function | Formats `ConnectorErrorCode` with the requested formatter. | `src/connector/error.rs:35` |
| `fmt` | function | Formats `ConnectorErrorCode` with the requested formatter. | `src/connector/error.rs:44` |
| `from_item` | function | Creates `ConnectorAudioRecord` from item. | `src/connector/transport.rs:314` |
| `from_resolved` | function | Creates `ConnectorConfigurationRecord` from resolved. | `src/connector/transport.rs:45` |
| `get` | function | Returns the value held by `ConnectorConfiguration`. | `src/connector/configuration.rs:134` |
| `get` | function | Returns the value held by `ResolvedConnectorConfiguration`. | `src/connector/configuration.rs:394` |
| `health` | function | Returns the health associated with `ConnectorServiceStatus`. | `src/connector/status.rs:46` |
| `health_reason_code` | function | Returns the health reason code associated with `ConnectorServiceStatus`. | `src/connector/status.rs:58` |
| `id` | function | Returns the id associated with `ConnectorCapability`. | `src/connector/manifest.rs:30` |
| `id` | function | Returns the id associated with `ConnectorRequirement`. | `src/connector/manifest.rs:61` |
| `input` | function | Returns the input associated with `ConnectorItem`. | `src/connector/worker/driver.rs:74` |
| `insert` | function | Inserts a typed configuration value into `ConnectorConfiguration`. | `src/connector/configuration.rs:126` |
| `into_configuration` | function | Converts `ConnectorConfigurationRecord` into configuration. | `src/connector/transport.rs:57` |
| `into_endpoint_failure` | function | Converts `ConnectorError` into endpoint failure. | `src/connector/error.rs:125` |
| `is_abort_requested` | function | Returns whether abort requested applies to `ConnectorContext`. | `src/connector/worker/coordination.rs:36` |
| `is_empty` | function | Returns whether `ConnectorConfiguration` contains no values. | `src/connector/configuration.rs:146` |
| `is_stop_requested` | function | Returns whether stop requested applies to `ConnectorContext`. | `src/connector/worker/coordination.rs:28` |
| `iter` | function | Iterates over the values held by `ConnectorConfiguration`. | `src/connector/configuration.rs:138` |
| `iter` | function | Iterates over the values held by `ResolvedConnectorConfiguration`. | `src/connector/configuration.rs:398` |
| `kind` | function | Returns the kind represented by `ConnectorConfigurationValue`. | `src/connector/configuration.rs:77` |
| `last_transition_elapsed_ns` | function | Returns the last transition elapsed nanoseconds associated with `ConnectorServiceStatus`. | `src/connector/status.rs:70` |
| `len` | function | Returns the number of values held by `ConnectorConfiguration`. | `src/connector/configuration.rs:142` |
| `manifest` | function | Returns the manifest associated with `Connector`. | `src/connector/mod.rs:119` |
| `manifest` | function | Returns the manifest associated with `RegisteredConnector`. | `src/connector/mod.rs:136` |
| `manifest_revision` | function | Returns the manifest revision associated with `ConnectorManifest`. | `src/connector/manifest.rs:128` |
| `media` | function | Returns the media associated with `ConnectorInputDescriptor`. | `src/connector/worker/driver.rs:48` |
| `message` | function | Returns the diagnostic message associated with `ConnectorConfigurationError`. | `src/connector/configuration.rs:623` |
| `message` | function | Returns the diagnostic message associated with `ConnectorError`. | `src/connector/error.rs:121` |
| `metadata` | function | Returns the metadata associated with `ConnectorAudioRecord`. | `src/connector/transport.rs:339` |
| `name` | function | Returns the name associated with `ConnectorConfigurationField`. | `src/connector/configuration.rs:206` |
| `new` | function | Creates a new `ConnectorSecret`. | `src/connector/configuration.rs:14` |
| `new` | function | Creates a new `ConnectorConfiguration`. | `src/connector/configuration.rs:116` |
| `new` | function | Creates a new `ConnectorConfigurationField`. | `src/connector/configuration.rs:178` |
| `new` | function | Creates a new `ConnectorConfigurationSchema`. | `src/connector/configuration.rs:238` |
| `new` | function | Creates a new `ConnectorErrorCode`. | `src/connector/error.rs:13` |
| `new` | function | Creates a new `ConnectorError`. | `src/connector/error.rs:88` |
| `new` | function | Creates a new `ConnectorCapability`. | `src/connector/manifest.rs:18` |
| `new` | function | Creates a new `ConnectorRequirement`. | `src/connector/manifest.rs:47` |
| `new` | function | Creates a new `ConnectorManifest`. | `src/connector/manifest.rs:89` |
| `new` | function | Creates a new `Connector`. | `src/connector/mod.rs:68` |
| `new` | function | Creates a new `ConnectorReadinessPolicy`. | `src/connector/readiness.rs:15` |
| `new` | function | Creates a new `SidecarConnectorDriverFactory`. | `src/connector/sidecar.rs:29` |
| `new` | function | Creates a new `ConnectorRunOutcome`. | `src/connector/worker/mod.rs:47` |
| `node` | function | Returns the node associated with `ConnectorManifest`. | `src/connector/manifest.rs:140` |
| `observation` | function | Returns the current observation exposed by `RegisteredConnector`. | `src/connector/mod.rs:140` |
| `observations` | function | Returns the observations exposed by `RegisteredConnector`. | `src/connector/mod.rs:153` |
| `operator_id` | function | Returns the operator identifier associated with `ConnectorManifest`. | `src/connector/manifest.rs:132` |
| `package_version` | function | Returns the package version associated with `ConnectorManifest`. | `src/connector/manifest.rs:136` |
| `pocketstation::connector::sidecar::sidecar_connector_factory` | function | Creates a connector driver factory backed by the supplied sidecar process. | `src/connector/sidecar.rs:264` |
| `port_name` | function | Returns the port name associated with `ConnectorAudioRecord`. | `src/connector/transport.rs:335` |
| `port_name` | function | Returns the port name associated with `ConnectorInputDescriptor`. | `src/connector/worker/driver.rs:40` |
| `preparation_group` | function | Returns the preparation group associated with `SidecarConnectorDriverFactory`. | `src/connector/sidecar.rs:39` |
| `prepare` | function | Prepares resources required by `SidecarConnectorDriverFactory`. | `src/connector/sidecar.rs:49` |
| `probe_interval` | function | Returns the probe interval associated with `ConnectorReadinessPolicy`. | `src/connector/readiness.rs:47` |
| `process` | function | Processes an input value through `SidecarConnectorDriverFactory`. | `src/connector/sidecar.rs:33` |
| `readiness` | function | Returns the readiness associated with `ConnectorManifest`. | `src/connector/manifest.rs:148` |
| `readiness_reason_code` | function | Returns the readiness reason code associated with `ConnectorServiceStatus`. | `src/connector/status.rs:54` |
| `record_discontinuity` | function | Records discontinuity for `ConnectorContext`. | `src/connector/worker/coordination.rs:126` |
| `record_failure` | function | Records failure for `ConnectorContext`. | `src/connector/worker/coordination.rs:134` |
| `record_frame_delivered` | function | Records frame delivered for `ConnectorContext`. | `src/connector/worker/coordination.rs:118` |
| `record_frame_dropped` | function | Records frame dropped for `ConnectorContext`. | `src/connector/worker/coordination.rs:122` |
| `record_frame_received` | function | Records frame received for `ConnectorContext`. | `src/connector/worker/coordination.rs:114` |
| `record_retry` | function | Records retry for `ConnectorContext`. | `src/connector/worker/coordination.rs:130` |
| `recovery` | function | Returns the recovery associated with `ConnectorServiceStatus`. | `src/connector/status.rs:50` |
| `recovery_reason_code` | function | Returns the recovery reason code associated with `ConnectorServiceStatus`. | `src/connector/status.rs:62` |
| `register_connector` | function | Registers connector for `Session`. | `src/connector/mod.rs:204` |
| `report_readiness_failure` | function | Returns the report readiness failure associated with `ConnectorContext`. | `src/connector/worker/coordination.rs:97` |
| `report_readiness_success` | function | Records a successful readiness probe for `ConnectorContext`. | `src/connector/worker/coordination.rs:80` |
| `required` | function | Returns the required associated with `ConnectorRequirement`. | `src/connector/manifest.rs:65` |
| `requirement` | function | Returns the requirement associated with `ConnectorConfigurationField`. | `src/connector/configuration.rs:214` |
| `requirements` | function | Returns the requirements associated with `ConnectorManifest`. | `src/connector/manifest.rs:156` |
| `resolve` | function | Resolves `ConnectorConfigurationSchema` into its validated representation. | `src/connector/configuration.rs:259` |
| `result` | function | Returns the result represented by `ConnectorRunOutcome`. | `src/connector/worker/mod.rs:59` |
| `retryability` | function | Returns the retryability associated with `ConnectorError`. | `src/connector/error.rs:117` |
| `revision` | function | Returns the revision associated with `ConnectorConfigurationSchema`. | `src/connector/configuration.rs:247` |
| `revision` | function | Returns the revision associated with `ConnectorServiceStatus`. | `src/connector/status.rs:66` |
| `route_id` | function | Returns the route identifier associated with `ConnectorInputDescriptor`. | `src/connector/worker/driver.rs:36` |
| `samples` | function | Returns the audio samples held by `ConnectorAudioRecord`. | `src/connector/transport.rs:343` |
| `session_id` | function | Returns the session identifier associated with `RegisteredConnector`. | `src/connector/mod.rs:132` |
| `set_connected` | function | Sets the connected used by `ConnectorContext`. | `src/connector/worker/coordination.rs:74` |
| `set_degraded` | function | Sets the degraded used by `ConnectorContext`. | `src/connector/worker/coordination.rs:56` |
| `set_healthy` | function | Sets the healthy used by `ConnectorContext`. | `src/connector/worker/coordination.rs:62` |
| `set_not_ready` | function | Sets the not ready used by `ConnectorContext`. | `src/connector/worker/coordination.rs:50` |
| `set_ready` | function | Sets the ready used by `ConnectorContext`. | `src/connector/worker/coordination.rs:44` |
| `set_reconnecting` | function | Sets the reconnecting used by `ConnectorContext`. | `src/connector/worker/coordination.rs:68` |
| `shutdown_mode` | function | Returns the shutdown mode associated with `ConnectorContext`. | `src/connector/worker/coordination.rs:32` |
| `sidecar` | function | Builds an outbound Connector backed by one bounded sidecar process. | `src/connector/mod.rs:112` |
| `signal_spec` | function | Returns the signal spec associated with `ConnectorInputDescriptor`. | `src/connector/worker/driver.rs:44` |
| `snapshot` | function | Returns a point-in-time snapshot of `ConnectorObservationHandle`. | `src/connector/observations.rs:53` |
| `stage` | function | Returns the stage associated with `ConnectorError`. | `src/connector/error.rs:113` |
| `startup_timeout` | function | Returns the startup timeout associated with `ConnectorReadinessPolicy`. | `src/connector/readiness.rs:43` |
| `success` | function | Returns whether `ConnectorRunOutcome` completed successfully. | `src/connector/worker/mod.rs:51` |
| `success_threshold` | function | Returns the success threshold associated with `ConnectorReadinessPolicy`. | `src/connector/readiness.rs:51` |
| `try_new` | function | Creates a new `ConnectorAudioRecord` after validating its inputs. | `src/connector/transport.rs:300` |
| `validate` | function | Validates `ConnectorManifest` against its declared contract. | `src/connector/manifest.rs:160` |
| `value_kind` | function | Returns the value kind associated with `ConnectorConfigurationField`. | `src/connector/configuration.rs:210` |
| `wait_for_stop` | function | Waits until a stop request is visible to `ConnectorContext`. | `src/connector/worker/coordination.rs:40` |
| `with` | function | Returns `ConnectorConfiguration` with the supplied entry applied. | `src/connector/configuration.rs:121` |
| `with_capability` | function | Sets the capability on `ConnectorManifest` and returns the updated value. | `src/connector/manifest.rs:113` |
| `with_constraint` | function | Sets the constraint on `ConnectorConfigurationField` and returns the updated value. | `src/connector/configuration.rs:195` |
| `with_driver` | function | Builds a connector whose bounded receiver loop is owned by Core. | `src/connector/mod.rs:88` |
| `with_requirement` | function | Sets the requirement on `ConnectorManifest` and returns the updated value. | `src/connector/manifest.rs:119` |
| `pocketstation::connector` | module | Types and operations for connector. | `src/connector/mod.rs:1` |
| `pocketstation::connector::Connector` | struct | Represents connector in the PocketStation API. | `src/connector/mod.rs:61` |
| `pocketstation::connector::RegisteredConnector` | struct | Represents registered connector in the PocketStation API. | `src/connector/mod.rs:125` |
| `pocketstation::connector::configuration::ConnectorConfiguration` | struct | Configures connector. | `src/connector/configuration.rs:111` |
| `pocketstation::connector::configuration::ConnectorConfigurationError` | struct | Reports a connector configuration error. | `src/connector/configuration.rs:608` |
| `pocketstation::connector::configuration::ConnectorConfigurationField` | struct | Represents connector configuration field in the PocketStation API. | `src/connector/configuration.rs:168` |
| `pocketstation::connector::configuration::ConnectorConfigurationSchema` | struct | Represents connector configuration schema in the PocketStation API. | `src/connector/configuration.rs:232` |
| `pocketstation::connector::configuration::ConnectorSecret` | struct | Represents connector secret in the PocketStation API. | `src/connector/configuration.rs:11` |
| `pocketstation::connector::configuration::ResolvedConnectorConfiguration` | struct | Configures resolved connector. | `src/connector/configuration.rs:391` |
| `pocketstation::connector::error::ConnectorError` | struct | Reports a connector error. | `src/connector/error.rs:80` |
| `pocketstation::connector::error::ConnectorErrorCode` | struct | Represents connector error code in the PocketStation API. | `src/connector/error.rs:10` |
| `pocketstation::connector::manifest::ConnectorCapability` | struct | Represents connector capability in the PocketStation API. | `src/connector/manifest.rs:12` |
| `pocketstation::connector::manifest::ConnectorManifest` | struct | Describes the connector manifest contract. | `src/connector/manifest.rs:75` |
| `pocketstation::connector::manifest::ConnectorRequirement` | struct | Represents connector requirement in the PocketStation API. | `src/connector/manifest.rs:40` |
| `pocketstation::connector::observations::ConnectorObservationHandle` | struct | Owns bounded access to connector observation. | `src/connector/observations.rs:15` |
| `pocketstation::connector::observations::ConnectorObservations` | struct | Reports the connector observations collected at an observation boundary. | `src/connector/observations.rs:158` |
| `pocketstation::connector::observations::ConnectorRuntimeObservations` | struct | Reports the connector runtime observations collected at an observation boundary. | `src/connector/observations.rs:168` |
| `pocketstation::connector::readiness::ConnectorReadinessPolicy` | struct | Configures connector readiness. | `src/connector/readiness.rs:7` |
| `pocketstation::connector::sidecar::SidecarConnectorDriverFactory` | struct | Adapts a bounded PocketStation sidecar process to the Connector driver SPI. | `src/connector/sidecar.rs:24` |
| `pocketstation::connector::status::ConnectorServiceStatus` | struct | Reports the structured connector service status. | `src/connector/status.rs:30` |
| `pocketstation::connector::transport::ConnectorAudioMetadata` | struct | Represents connector audio metadata in the PocketStation API. | `src/connector/transport.rs:281` |
| `pocketstation::connector::transport::ConnectorAudioRecord` | struct | Represents connector audio record in the PocketStation API. | `src/connector/transport.rs:293` |
| `pocketstation::connector::transport::ConnectorConfigurationRecord` | struct | Canonical typed configuration handed to a connector sidecar during its bounded Configure handshake. Secret classification survives the boundary; Debug output continues to redact secret values. | `src/connector/transport.rs:42` |
| `pocketstation::connector::worker::ConnectorRunOutcome` | struct | Reports the structured connector run outcome. | `src/connector/worker/mod.rs:42` |
| `pocketstation::connector::worker::coordination::ConnectorContext` | struct | Represents connector context in the PocketStation API. | `src/connector/worker/coordination.rs:14` |
| `pocketstation::connector::worker::driver::ConnectorInputDescriptor` | struct | Immutable Session and graph metadata for one connector input. | `src/connector/worker/driver.rs:16` |
| `ConnectorAudioMetadata::channels` | struct_field | Stores the channels associated with `ConnectorAudioMetadata`. | `src/connector/transport.rs:288` |
| `ConnectorAudioMetadata::connector_id` | struct_field | Identifies the connector associated with `ConnectorAudioMetadata`. | `src/connector/transport.rs:283` |
| `ConnectorAudioMetadata::endpoint_id` | struct_field | Identifies the endpoint associated with `ConnectorAudioMetadata`. | `src/connector/transport.rs:282` |
| `ConnectorAudioMetadata::lineage` | struct_field | Stores the lineage associated with `ConnectorAudioMetadata`. | `src/connector/transport.rs:286` |
| `ConnectorAudioMetadata::route_id` | struct_field | Identifies the route associated with `ConnectorAudioMetadata`. | `src/connector/transport.rs:284` |
| `ConnectorAudioMetadata::sample_format` | struct_field | Stores the sample format associated with `ConnectorAudioMetadata`. | `src/connector/transport.rs:289` |
| `ConnectorAudioMetadata::sample_rate_hz` | struct_field | Stores the sample rate value for `ConnectorAudioMetadata`, in hertz. | `src/connector/transport.rs:287` |
| `ConnectorAudioMetadata::stream_id` | struct_field | Identifies the stream associated with `ConnectorAudioMetadata`. | `src/connector/transport.rs:285` |
| `ConnectorConfigurationConstraint::SignedRange::maximum` | struct_field | Sets the inclusive maximum accepted by `SignedRange`. | `src/connector/configuration.rs:162` |
| `ConnectorConfigurationConstraint::SignedRange::minimum` | struct_field | Sets the inclusive minimum accepted by `SignedRange`. | `src/connector/configuration.rs:162` |
| `ConnectorConfigurationConstraint::TextLengthBytes::maximum` | struct_field | Sets the inclusive maximum accepted by `TextLengthBytes`. | `src/connector/configuration.rs:161` |
| `ConnectorConfigurationConstraint::TextLengthBytes::minimum` | struct_field | Sets the inclusive minimum accepted by `TextLengthBytes`. | `src/connector/configuration.rs:161` |
| `ConnectorConfigurationConstraint::UnsignedRange::maximum` | struct_field | Sets the inclusive maximum accepted by `UnsignedRange`. | `src/connector/configuration.rs:163` |
| `ConnectorConfigurationConstraint::UnsignedRange::minimum` | struct_field | Sets the inclusive minimum accepted by `UnsignedRange`. | `src/connector/configuration.rs:163` |
| `ConnectorItem::Audio::frame` | struct_field | Stores the frame associated with `Audio`. | `src/connector/worker/driver.rs:65` |
| `ConnectorItem::Audio::input` | struct_field | Stores the input associated with `Audio`. | `src/connector/worker/driver.rs:64` |
| `ConnectorItem::Signal::input` | struct_field | Stores the input associated with `Signal`. | `src/connector/worker/driver.rs:68` |
| `ConnectorItem::Signal::signal` | struct_field | Stores the signal associated with `Signal`. | `src/connector/worker/driver.rs:69` |
| `ConnectorManifestError::DuplicateManifestEntry::id` | struct_field | Identifies the id associated with `DuplicateManifestEntry`. | `src/connector/manifest.rs:253` |
| `ConnectorManifestError::UnsupportedApiRevision::requested` | struct_field | Stores the requested associated with `UnsupportedApiRevision`. | `src/connector/manifest.rs:233` |
| `ConnectorManifestError::UnsupportedApiRevision::supported` | struct_field | Stores the supported associated with `UnsupportedApiRevision`. | `src/connector/manifest.rs:233` |
| `ConnectorObservations::failures_total` | struct_field | Counts the total number of failures observed by `ConnectorObservations`. | `src/connector/observations.rs:163` |
| `ConnectorObservations::last_error` | struct_field | Carries the last error reported by `ConnectorObservations`. | `src/connector/observations.rs:164` |
| `ConnectorObservations::reconnects_total` | struct_field | Counts the total number of reconnects observed by `ConnectorObservations`. | `src/connector/observations.rs:162` |
| `ConnectorObservations::retry_attempts_total` | struct_field | Counts the total number of retry attempts observed by `ConnectorObservations`. | `src/connector/observations.rs:161` |
| `ConnectorObservations::service_status` | struct_field | Stores the service status associated with `ConnectorObservations`. | `src/connector/observations.rs:159` |
| `ConnectorObservations::status_transitions_total` | struct_field | Counts the total number of status transitions observed by `ConnectorObservations`. | `src/connector/observations.rs:160` |
| `ConnectorRuntimeObservations::connector` | struct_field | Stores the connector associated with `ConnectorRuntimeObservations`. | `src/connector/observations.rs:170` |
| `ConnectorRuntimeObservations::endpoint` | struct_field | Stores the endpoint associated with `ConnectorRuntimeObservations`. | `src/connector/observations.rs:171` |
| `ConnectorRuntimeObservations::endpoint_ids` | struct_field | Identifies the endpoint associated with `ConnectorRuntimeObservations`. | `src/connector/observations.rs:169` |
| `pocketstation::connector::ConnectorDeclarationError::WrongSession::registered` | struct_field | Stores the registered associated with `WrongSession`. | `src/connector/mod.rs:236` |
| `pocketstation::connector::ConnectorDeclarationError::WrongSession::requested` | struct_field | Stores the requested associated with `WrongSession`. | `src/connector/mod.rs:237` |
| `pocketstation::connector::ConnectorObservationLookupError::WrongSession::registered` | struct_field | Stores the registered associated with `WrongSession`. | `src/connector/mod.rs:249` |
| `pocketstation::connector::ConnectorObservationLookupError::WrongSession::requested` | struct_field | Stores the requested associated with `WrongSession`. | `src/connector/mod.rs:250` |
| `pocketstation::connector::worker::ConnectorFactory` | trait | Defines the implementation contract for connector. | `src/connector/worker/mod.rs:17` |
| `pocketstation::connector::worker::ConnectorWorker` | trait | Defines the implementation contract for connector worker. | `src/connector/worker/mod.rs:32` |
| `pocketstation::connector::worker::driver::ConnectorDriver` | trait | Provider-specific behavior executed on Core's bounded connector worker. | `src/connector/worker/driver.rs:92` |
| `pocketstation::connector::worker::driver::ConnectorDriverFactory` | trait | Prepares provider state while Core retains receiver and lifecycle authority. | `src/connector/worker/driver.rs:123` |
| `pocketstation::connector::ConnectorDeclarationError::Configuration` | variant | Reports configuration. | `src/connector/mod.rs:240` |
| `pocketstation::connector::ConnectorDeclarationError::Session` | variant | Reports session. | `src/connector/mod.rs:242` |
| `pocketstation::connector::ConnectorDeclarationError::WrongSession` | variant | Reports wrong session. | `src/connector/mod.rs:235` |
| `pocketstation::connector::ConnectorObservationLookupError::WrongSession` | variant | Reports wrong session. | `src/connector/mod.rs:248` |
| `pocketstation::connector::ConnectorRegistrationError::InvalidManifest` | variant | Reports invalid manifest. | `src/connector/mod.rs:227` |
| `pocketstation::connector::ConnectorRegistrationError::Session` | variant | Reports session. | `src/connector/mod.rs:229` |
| `pocketstation::connector::configuration::ConnectorConfigurationConstraint::NonEmpty` | variant | Represents the non empty case of `ConnectorConfigurationConstraint`. | `src/connector/configuration.rs:160` |
| `pocketstation::connector::configuration::ConnectorConfigurationConstraint::OneOf` | variant | Represents the one of case of `ConnectorConfigurationConstraint`. | `src/connector/configuration.rs:164` |
| `pocketstation::connector::configuration::ConnectorConfigurationConstraint::SignedRange` | variant | Represents the signed range case of `ConnectorConfigurationConstraint`. | `src/connector/configuration.rs:162` |
| `pocketstation::connector::configuration::ConnectorConfigurationConstraint::TextLengthBytes` | variant | Represents the text length bytes case of `ConnectorConfigurationConstraint`. | `src/connector/configuration.rs:161` |
| `pocketstation::connector::configuration::ConnectorConfigurationConstraint::UnsignedRange` | variant | Represents the unsigned range case of `ConnectorConfigurationConstraint`. | `src/connector/configuration.rs:163` |
| `pocketstation::connector::configuration::ConnectorConfigurationErrorCode::ConstraintViolation` | variant | Reports constraint violation. | `src/connector/configuration.rs:576` |
| `pocketstation::connector::configuration::ConnectorConfigurationErrorCode::DuplicateField` | variant | Reports duplicate field. | `src/connector/configuration.rs:570` |
| `pocketstation::connector::configuration::ConnectorConfigurationErrorCode::EmptySecret` | variant | Reports empty secret. | `src/connector/configuration.rs:578` |
| `pocketstation::connector::configuration::ConnectorConfigurationErrorCode::InvalidSchema` | variant | Reports invalid schema. | `src/connector/configuration.rs:569` |
| `pocketstation::connector::configuration::ConnectorConfigurationErrorCode::InvalidValue` | variant | Reports invalid value. | `src/connector/configuration.rs:575` |
| `pocketstation::connector::configuration::ConnectorConfigurationErrorCode::MissingRequiredField` | variant | Reports missing required field. | `src/connector/configuration.rs:573` |
| `pocketstation::connector::configuration::ConnectorConfigurationErrorCode::SecretClassificationMismatch` | variant | Reports secret classification mismatch. | `src/connector/configuration.rs:580` |
| `pocketstation::connector::configuration::ConnectorConfigurationErrorCode::SecretDefaultForbidden` | variant | Reports secret default forbidden. | `src/connector/configuration.rs:579` |
| `pocketstation::connector::configuration::ConnectorConfigurationErrorCode::TooManyFields` | variant | Reports too many fields. | `src/connector/configuration.rs:571` |
| `pocketstation::connector::configuration::ConnectorConfigurationErrorCode::UnexpectedSensitiveValue` | variant | Reports unexpected sensitive value. | `src/connector/configuration.rs:581` |
| `pocketstation::connector::configuration::ConnectorConfigurationErrorCode::UnknownField` | variant | Reports unknown field. | `src/connector/configuration.rs:572` |
| `pocketstation::connector::configuration::ConnectorConfigurationErrorCode::ValueTooLarge` | variant | Reports value too large. | `src/connector/configuration.rs:577` |
| `pocketstation::connector::configuration::ConnectorConfigurationErrorCode::WrongType` | variant | Reports wrong type. | `src/connector/configuration.rs:574` |
| `pocketstation::connector::configuration::ConnectorConfigurationRequirement::Default` | variant | Selects default behavior for `ConnectorConfigurationRequirement`. | `src/connector/configuration.rs:155` |
| `pocketstation::connector::configuration::ConnectorConfigurationRequirement::Optional` | variant | Selects optional behavior for `ConnectorConfigurationRequirement`. | `src/connector/configuration.rs:154` |
| `pocketstation::connector::configuration::ConnectorConfigurationRequirement::Required` | variant | Selects required behavior for `ConnectorConfigurationRequirement`. | `src/connector/configuration.rs:153` |
| `pocketstation::connector::configuration::ConnectorConfigurationValue::Boolean` | variant | Represents the boolean case of `ConnectorConfigurationValue`. | `src/connector/configuration.rs:68` |
| `pocketstation::connector::configuration::ConnectorConfigurationValue::ByteCount` | variant | Represents the byte count case of `ConnectorConfigurationValue`. | `src/connector/configuration.rs:72` |
| `pocketstation::connector::configuration::ConnectorConfigurationValue::DurationMilliseconds` | variant | Represents the duration milliseconds case of `ConnectorConfigurationValue`. | `src/connector/configuration.rs:71` |
| `pocketstation::connector::configuration::ConnectorConfigurationValue::Secret` | variant | Represents the secret case of `ConnectorConfigurationValue`. | `src/connector/configuration.rs:73` |
| `pocketstation::connector::configuration::ConnectorConfigurationValue::SignedInteger` | variant | Represents the signed integer case of `ConnectorConfigurationValue`. | `src/connector/configuration.rs:69` |
| `pocketstation::connector::configuration::ConnectorConfigurationValue::Text` | variant | Represents the text case of `ConnectorConfigurationValue`. | `src/connector/configuration.rs:67` |
| `pocketstation::connector::configuration::ConnectorConfigurationValue::UnsignedInteger` | variant | Represents the unsigned integer case of `ConnectorConfigurationValue`. | `src/connector/configuration.rs:70` |
| `pocketstation::connector::configuration::ConnectorConfigurationValueKind::Boolean` | variant | Selects boolean behavior for `ConnectorConfigurationValueKind`. | `src/connector/configuration.rs:57` |
| `pocketstation::connector::configuration::ConnectorConfigurationValueKind::ByteCount` | variant | Selects byte count behavior for `ConnectorConfigurationValueKind`. | `src/connector/configuration.rs:61` |
| `pocketstation::connector::configuration::ConnectorConfigurationValueKind::DurationMilliseconds` | variant | Selects duration milliseconds behavior for `ConnectorConfigurationValueKind`. | `src/connector/configuration.rs:60` |
| `pocketstation::connector::configuration::ConnectorConfigurationValueKind::Secret` | variant | Selects secret behavior for `ConnectorConfigurationValueKind`. | `src/connector/configuration.rs:62` |
| `pocketstation::connector::configuration::ConnectorConfigurationValueKind::SignedInteger` | variant | Selects signed integer behavior for `ConnectorConfigurationValueKind`. | `src/connector/configuration.rs:58` |
| `pocketstation::connector::configuration::ConnectorConfigurationValueKind::Text` | variant | Selects text behavior for `ConnectorConfigurationValueKind`. | `src/connector/configuration.rs:56` |
| `pocketstation::connector::configuration::ConnectorConfigurationValueKind::UnsignedInteger` | variant | Selects unsigned integer behavior for `ConnectorConfigurationValueKind`. | `src/connector/configuration.rs:59` |
| `pocketstation::connector::error::ConnectorErrorBuildError::EmptyMessage` | variant | Reports empty message. | `src/connector/error.rs:186` |
| `pocketstation::connector::error::ConnectorErrorBuildError::MessageTooLarge` | variant | Reports message too large. | `src/connector/error.rs:188` |
| `pocketstation::connector::error::ConnectorErrorCodeError::Empty` | variant | Represents an empty value or collection. | `src/connector/error.rs:52` |
| `pocketstation::connector::error::ConnectorErrorCodeError::InvalidCharacter` | variant | Reports invalid character. | `src/connector/error.rs:56` |
| `pocketstation::connector::error::ConnectorErrorCodeError::TooLong` | variant | Reports too long. | `src/connector/error.rs:54` |
| `pocketstation::connector::error::ConnectorErrorStage::Configuration` | variant | Reports configuration. | `src/connector/error.rs:61` |
| `pocketstation::connector::error::ConnectorErrorStage::Delivery` | variant | Reports delivery. | `src/connector/error.rs:65` |
| `pocketstation::connector::error::ConnectorErrorStage::Join` | variant | Reports join. | `src/connector/error.rs:68` |

## Interpretation

An inventory row establishes that a declaration, test, lifecycle operation, configuration surface, or protocol element exists at the frozen snapshot. Fields shown as unknown remain deliberately unspecified. Consult the native API and error contract before relying on panic behavior, blocking, cancellation, ordering, limits, retry, or recovery.

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

This page was verified against Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `src/connector/mod.rs:1-253` (`DIRECT`)

A file's presence proves implementation or declaration at this snapshot. It does not by itself prove physical-device qualification, operational performance, retry safety, or behavior outside the recorded test conditions.
