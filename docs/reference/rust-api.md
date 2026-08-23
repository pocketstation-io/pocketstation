# Rust API reference

<!-- claims: CLM-REF-001-CAP-001,CLM-REF-001-CAP-002,CLM-REF-001-CAP-003,CLM-REF-001-CAP-004,CLM-REF-001-CAP-005,CLM-REF-001-CAP-006,CLM-REF-001-CAP-007,CLM-REF-001-CAP-008,CLM-REF-001-CAP-009,CLM-REF-001-CAP-010,CLM-REF-001-CAP-011,CLM-REF-001-CAP-012,CLM-REF-001-CAP-013,CLM-REF-001-CAP-014,CLM-REF-001-CAP-015,CLM-REF-001-CAP-016,CLM-REF-001-CAP-017,CLM-REF-001-CAP-018,CLM-REF-001-CAP-019,CLM-REF-001-CAP-020,CLM-REF-001-CAP-021,CLM-REF-001-CAP-022,CLM-REF-001-CAP-023,CLM-REF-001-CAP-024,CLM-REF-001-CAP-025,CLM-REF-001-CAP-026,CLM-REF-001-CAP-027,CLM-REF-001-CAP-028,CLM-REF-001-CAP-029,CLM-REF-001-CAP-030,CLM-REF-001-CAP-031,CLM-REF-001-CAP-032,CLM-REF-001-CAP-033,CLM-REF-001-SOURCE-001 -->

## Scope

- **Install and feature-select the crate.** Add PocketStation to a Cargo package and choose native capture, contracts-only, conformance, or internal test features.
- **Declare a Session.** Describe sources, operators, endpoints, streams, and recording routes before runtime preparation.
- **Select and resolve capture sources.** Discover capture candidates and resolve application, process, device, and system queries to stable source identities.
- **Capture application audio.** Prepare application-scoped capture through the platform backend selected for the current target.
- **Capture microphone audio.** Select the default or identified input device and open native microphone capture.
- **Capture system audio.** Represent and open platform system-loopback capture where the selected backend implements it.
- **Observe permission and source lifecycle.** Query non-prompting authorization where observable and receive source-generation, loss, and permission-epoch changes.
- **Preserve frame identity and lineage.** Carry source, stream, stem, route, clock, sequence, generation, and derivation identity with audio frames.
- **Map clocks and correct drift.** Map source timestamps into a Session timeline and estimate or correct clock drift.
- **Compile Session declarations.** Validate declarations, resolve bindings, and lower a Session specification into an executable plan.
- **Prepare runtime resources.** Prepare source and endpoint runtimes while preserving the mapping back to declaration identities.
- **Start, cancel, stop, and finalize a Session.** Coordinate Session startup, rollback, cancellation, steady-state ownership, stop, and terminal outcomes.
- **Route realtime audio.** Deliver pooled audio frames through independent fixed-capacity routes governed by explicit edge policy.
- **Poll bounded audio batches.** Consume routed audio from the built-in polled-audio endpoint through bounded batch leases and receipts.
- **Record aligned multistem output.** Configure stems and finalize per-stem outcomes through the recording endpoint lifecycle.
- **Describe graph contracts.** Declare typed ports, media capabilities, partition safety, copy, loss, delivery, and observability policy.
- **Implement asynchronous operators.** Register operator factories that consume and emit named typed signals on the asynchronous execution lane.
- **Carry typed signals.** Represent audio-adjacent text, event, binary, metric, and custom-schema payloads with timing and lineage.
- **Bridge asynchronous output into audio.** Return generated PCM from asynchronous processing through an explicit bounded audio reentry bridge.
- **Implement endpoint drivers.** Prepare, start, receive, cancel, and finalize destinations behind the endpoint driver contract.
- **Declare connector manifests and configuration.** Describe connector identity, ports, configuration schema, secrets, and delivery policy without embedding a provider protocol in Core.
- **Run connector workers.** Supervise connector delivery, acknowledgement, retry budgets, readiness, cancellation, drain, and abort.
- **Load native extension libraries.** Validate and load a versioned native library, acquire registrations, and retain executable ownership for their lifetime.
- **Use the versioned C ABI.** Declare, start, observe, stop, and release Sessions and extension callbacks through the public C boundary.
- **Host managed-process sidecars.** Exchange bounded protocol messages with a child process under explicit deadlines and lifecycle states.
- **Observe Session metrics and events.** Read route, source, operator, sidecar, endpoint, drop, latency, queue, and terminal observations.
- **Record and validate Session traces.** Persist lifecycle trace records and validate their structural and terminal consistency.
- **Inject external PCM.** Acquire bounded input buffers and write externally produced PCM through the source extension lifecycle.
- **Encode and decode Opus.** Configure stateful Opus encoders and decoders and convert between PocketStation audio frames and packets.
- **Classify public failures.** Expose stable typed errors and cross-boundary error codes without inferring retry or recovery guarantees.
- **Validate protocol and conformance boundaries.** Check ABI layout, cross-language behavior, connector vectors, and protocol compatibility against versioned fixtures.
- **Build and publish repository artifacts.** Run architecture, protocol, package, platform, and release checks used by the repository publication workflow.
- **Integrate transcription processing.** Use the repository-owned Whisper example to send captured stems to an external transcription process with evidence output.

These statements describe repository contracts at the documented snapshot. They do not extend platform qualification, performance, retry, or delivery guarantees beyond the native API contracts and executable evidence.

## Reference authority

The generated [docs.rs API](https://docs.rs/pocketstation/latest/pocketstation/) is the exhaustive symbol-level Rust reference. Human reference pages organize responsibilities and cross-boundary behavior; they do not duplicate every rustdoc signature.

## Public surface

| Declaration | Kind | Purpose | Source |
|---|---|---|---|
| `INITIAL` | assoc_const | Provides the initial value for `PermissionEpoch`. | `src/capture/authorization.rs:270` |
| `INITIAL` | assoc_const | Provides the initial value for `SourceGeneration`. | `src/capture/events.rs:15` |
| `Error` | assoc_type | Specifies the error type returned by `SidecarMessageKind` operations. | `src/runtime/lifecycle/sidecar_protocol.rs:23` |
| `pocketstation::abi::extension::PKS_EXTENSION_ABI_MAJOR` | constant | Defines the major version of extension ABI. | `src/abi/extension.rs:7` |
| `pocketstation::abi::extension::PKS_EXTENSION_ABI_MINOR` | constant | Defines the minor version of extension ABI. | `src/abi/extension.rs:8` |
| `pocketstation::codec::constants::OPUS_FRAME_SAMPLES` | constant | 20 ms frame = 960 samples at 48 kHz (AUDIO-012). | `src/codec/constants.rs:5` |
| `pocketstation::codec::constants::OPUS_MAX_PACKET_BYTES` | constant | Maximum number of bytes the Opus encoder can emit per 20 ms frame. libopus guarantees this upper bound. | `src/codec/constants.rs:13` |
| `pocketstation::codec::constants::OPUS_SAMPLE_RATE_HZ` | constant | 48 000 Hz, mono, VOIP application profile (AUDIO-012 default). | `src/codec/constants.rs:2` |
| `pocketstation::codec::constants::VOICE_AGENT_FRAME_SAMPLES` | constant | 10 ms frame = 480 samples at 48 kHz (voice-agent low-latency mode, RFC 6716 §3.1). Ten milliseconds of mono PCM at 48 kHz. | `src/codec/constants.rs:9` |
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
| `pocketstation::graph::ports::MAX_ASYNC_PAYLOAD_BYTES` | constant | Sets the maximum supported async payload bytes. | `src/graph/ports.rs:13` |
| `pocketstation::native_extension::EXTENSION_LIBRARY_ENTRYPOINT_V1` | constant | Exact exported symbol required from a native Extension ABI v1 dynamic library. The suffix follows the ABI major; compatible minor revisions use the same entrypoint. | `src/native_extension/mod.rs:24` |
| `pocketstation::session::extensions::audio_input::PCM_SOURCE_TYPE_ID` | constant | Stable runtime identity of the underlying PCM source implementation. | `src/session/extensions/audio_input/mod.rs:19` |
| `pocketstation::timing::PROCESS_MONOTONIC_CLOCK_DOMAIN_ID` | constant | Clock-domain identity for timestamps produced by PocketStation's shared process-wide monotonic clock. | `src/timing/mod.rs:20` |
| `pocketstation::SessionCancelDisposition` | enum | Classifies the observable session cancel disposition. | `src/lib.rs:1085` |
| `pocketstation::SessionEndpointError` | enum | Classifies failures reported as session endpoint error. | `src/lib.rs:1035` |
| `pocketstation::SessionOperatorError` | enum | Classifies failures reported as session operator error. | `src/lib.rs:1051` |
| `pocketstation::SessionRuntimeError` | enum | Classifies failures reported as session runtime error. | `src/lib.rs:1073` |
| `pocketstation::SessionSidecarError` | enum | Classifies failures reported as session sidecar error. | `src/lib.rs:1045` |
| `pocketstation::SessionSourceError` | enum | Classifies failures reported as session source error. | `src/lib.rs:1057` |
| `pocketstation::SessionStartErrorKind` | enum | Selects the session start error kind used by PocketStation. | `src/lib.rs:1063` |
| `pocketstation::SessionStopDisposition` | enum | Classifies the observable session stop disposition. | `src/lib.rs:1079` |
| `pocketstation::abi::extension::PksExtensionKind` | enum | Selects the extension kind used by PocketStation. | `src/abi/extension.rs:32` |
| `pocketstation::abi::extension::PksExtensionPortDirection` | enum | Selects the extension port direction used by PocketStation. | `src/abi/extension.rs:40` |
| `pocketstation::abi::session::abi::PksSessionStatusCode` | enum | Enumerates the supported session status code cases. | `src/abi/session/abi.rs:79` |
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
| `pocketstation::codec::decoder::OpusDecodeError` | enum | Classifies failures reported as opus decode error. | `src/codec/decoder.rs:25` |
| `pocketstation::codec::encoder::OpusApplication` | enum | Opus application mode. | `src/codec/encoder.rs:58` |
| `pocketstation::codec::encoder::OpusChannels` | enum | Typed channel count for Opus — prevents silent u8 misuse. | `src/codec/encoder.rs:27` |
| `pocketstation::codec::encoder::OpusEncodeError` | enum | Classifies failures reported as opus encode error. | `src/codec/encoder.rs:131` |
| `pocketstation::codec::encoder::OpusFrameDuration` | enum | Supported Opus frame duration at 48 kHz. | `src/codec/encoder.rs:7` |
| `pocketstation::codec::encoder::OpusSampleRate` | enum | Typed sample rate. Opus internally always uses 48 kHz; this type makes the constraint explicit rather than hiding it behind a `u32` constant. | `src/codec/encoder.rs:44` |
| `pocketstation::codec::profile::StreamProfile` | enum | Enumerates the supported stream profile cases. | `src/codec/profile.rs:11` |
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
| `pocketstation::frame::audio::AudioFrameBuildError` | enum | Classifies failures reported as audio frame build error. | `src/frame/audio.rs:51` |
| `pocketstation::frame::audio::SampleFormat` | enum | Selects the sample format used by PocketStation. | `src/frame/audio.rs:13` |
| `pocketstation::frame::lineage::FrameLineageBuildError` | enum | Classifies failures reported as frame lineage build error. | `src/frame/lineage.rs:93` |
| `pocketstation::frame::platform::Platform` | enum | Enumerates the supported platform cases. | `src/frame/platform.rs:4` |
| `pocketstation::frame::pool::AudioBufferWriteError` | enum | Classifies failures reported as audio buffer write error. | `src/frame/pool.rs:14` |
| `pocketstation::graph::compile::resolve::CompileError` | enum | Classifies failures reported as compile error. | `src/graph/compile/resolve.rs:26` |
| `pocketstation::graph::node::ConfigError` | enum | Classifies failures reported as config error. | `src/graph/node.rs:141` |
| `pocketstation::graph::node::NodeDescriptorError` | enum | Classifies failures reported as node descriptor error. | `src/graph/node.rs:252` |
| `pocketstation::graph::node::NodeError` | enum | Classifies failures reported as node error. | `src/graph/node.rs:149` |
| `pocketstation::graph::partition::ExecutionPartition` | enum | WHERE an operator runs. | `src/graph/partition.rs:18` |
| `pocketstation::graph::partition::SafetyContract` | enum | WHAT an operator guarantees about its runtime behaviour. | `src/graph/partition.rs:82` |
| `pocketstation::graph::plan::PlanError` | enum | Classifies failures reported as plan error. | `src/graph/plan.rs:21` |
| `pocketstation::graph::ports::BackpressurePolicy` | enum | Selects the backpressure policy used by PocketStation. | `src/graph/ports.rs:265` |
| `pocketstation::graph::ports::ChannelLayout` | enum | Enumerates the supported channel layout cases. | `src/graph/ports.rs:27` |
| `pocketstation::graph::ports::ClockDomain` | enum | Enumerates the supported clock domain cases. | `src/graph/ports.rs:249` |
| `pocketstation::graph::ports::CopyPolicy` | enum | Selects the copy policy used by PocketStation. | `src/graph/ports.rs:280` |
| `pocketstation::graph::ports::DeliverySemantics` | enum | Selects the delivery semantics used by PocketStation. | `src/graph/ports.rs:273` |
| `pocketstation::graph::ports::EdgeObservabilityLevel` | enum | Selects the edge observability level used by PocketStation. | `src/graph/ports.rs:294` |
| `pocketstation::graph::ports::LossPolicy` | enum | Selects the loss policy used by PocketStation. | `src/graph/ports.rs:287` |
| `pocketstation::graph::ports::MediaCaps` | enum | Enumerates the supported media caps cases. | `src/graph/ports.rs:85` |
| `pocketstation::graph::ports::MediaKind` | enum | Selects the media kind used by PocketStation. | `src/graph/ports.rs:16` |
| `pocketstation::graph::ports::Multiplicity` | enum | Enumerates the supported multiplicity cases. | `src/graph/ports.rs:169` |
| `pocketstation::graph::ports::PortDirection` | enum | Selects the port direction used by PocketStation. | `src/graph/ports.rs:163` |
| `pocketstation::graph::ports::PortSpecError` | enum | Classifies failures reported as port spec error. | `src/graph/ports.rs:239` |
| `pocketstation::graph::registry::NodeDefinitionRef` | enum | Enumerates the supported node definition ref cases. | `src/graph/registry.rs:32` |
| `pocketstation::graph::registry::NodeRegistrationError` | enum | Classifies failures reported as node registration error. | `src/graph/registry.rs:57` |
| `pocketstation::graph::signal::continuity::SignalContinuityError` | enum | Classifies failures reported as signal continuity error. | `src/graph/signal/continuity.rs:89` |
| `pocketstation::graph::signal::envelope::SignalEnvelopeError` | enum | Classifies failures reported as signal envelope error. | `src/graph/signal/envelope.rs:137` |
| `pocketstation::graph::signal::lineage::SignalDerivationError` | enum | Classifies failures reported as signal derivation error. | `src/graph/signal/lineage.rs:159` |
| `pocketstation::graph::signal::lineage::SignalLineageError` | enum | Classifies failures reported as signal lineage error. | `src/graph/signal/lineage.rs:86` |
| `pocketstation::graph::signal::operator::AsyncOperatorManifestError` | enum | Classifies failures reported as async operator manifest error. | `src/graph/signal/operator.rs:321` |
| `pocketstation::graph::signal::operator::OperatorCancellationPolicy` | enum | Selects the operator cancellation policy used by PocketStation. | `src/graph/signal/operator.rs:57` |
| `pocketstation::graph::signal::operator::OperatorFailurePolicy` | enum | Selects the operator failure policy used by PocketStation. | `src/graph/signal/operator.rs:63` |
| `pocketstation::graph::signal::payload::SignalPayload` | enum | Enumerates the supported signal payload cases. | `src/graph/signal/payload.rs:10` |
| `pocketstation::graph::signal::spec::BinaryFormat` | enum | Binary encoding hint for `SignalClass::Binary`. | `src/graph/signal/spec.rs:141` |
| `pocketstation::graph::signal::spec::Codec` | enum | Audio encoding format for `SignalClass::EncodedAudio`. | `src/graph/signal/spec.rs:113` |
| `pocketstation::graph::signal::spec::EventFormat` | enum | Event structure hint for `SignalClass::Event`. | `src/graph/signal/spec.rs:132` |
| `pocketstation::graph::signal::spec::SignalClass` | enum | The fundamental class of data flowing through a port. | `src/graph/signal/spec.rs:156` |
| `pocketstation::graph::signal::spec::SignalSpecError` | enum | Classifies failures reported as signal spec error. | `src/graph/signal/spec.rs:351` |
| `pocketstation::graph::signal::spec::TextFormat` | enum | Text encoding hint for `SignalClass::Text`. | `src/graph/signal/spec.rs:124` |
| `pocketstation::graph::signal::timing::SignalTimingError` | enum | Classifies failures reported as signal timing error. | `src/graph/signal/timing.rs:89` |
| `pocketstation::native_extension::NativeExtensionKind` | enum | Selects the native extension kind used by PocketStation. | `src/native_extension/mod.rs:27` |
| `pocketstation::native_extension::NativeExtensionLibraryErrorCode` | enum | Enumerates the supported native extension library error code cases. | `src/native_extension/mod.rs:78` |
| `pocketstation::recording::error_code::RecordingErrorCode` | enum | Stable language-neutral code for a recording failure. | `src/recording/error_code.rs:9` |
| `pocketstation::recording::writer::DiscontinuityKind` | enum | Selects the discontinuity kind used by PocketStation. | `src/recording/writer.rs:104` |
| `pocketstation::recording::writer::RecordingState` | enum | Selects the recording state used by PocketStation. | `src/recording/writer.rs:85` |
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
| `AsyncNode::cancel` | function | Requests cancellation of `AsyncNode`. | `src/graph/signal/operator.rs:36` |
| `AsyncNode::close` | function | Closes `AsyncNode` to further work. | `src/graph/signal/operator.rs:40` |
| `AsyncNode::flush` | function | Flushes pending output from `AsyncNode` at the end of a run. | `src/graph/signal/operator.rs:32` |
| `AsyncNode::prepare` | function | Prepares resources required by `AsyncNode`. | `src/graph/signal/operator.rs:14` |
| `AsyncNode::process` | function | Processes an input value through `AsyncNode`. | `src/graph/signal/operator.rs:19` |
| `AsyncNode::process_port` | function | Returns the process port associated with `AsyncNode`. | `src/graph/signal/operator.rs:24` |
| `AsyncOperatorFactory::create` | function | Creates the runtime implementation described by `AsyncOperatorFactory`. | `src/graph/signal/operator.rs:378` |
| `AsyncOperatorFactory::manifest` | function | Returns the manifest associated with `AsyncOperatorFactory`. | `src/graph/signal/operator.rs:369` |
| `AsyncOperatorFactory::resolve_manifest` | function | Resolves manifest for `AsyncOperatorFactory`. | `src/graph/signal/operator.rs:371` |
| `AsyncOperatorFactory::validate_config` | function | Validates config for `AsyncOperatorFactory`. | `src/graph/signal/operator.rs:370` |
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
| `NodeDefinition::descriptor` | function | Returns the descriptor associated with `NodeDefinition`. | `src/graph/registry.rs:22` |
| `NodeDefinition::validate_config` | function | Validates config for `NodeDefinition`. | `src/graph/registry.rs:23` |
| `NodeFactory::descriptor` | function | Returns the descriptor associated with `NodeFactory`. | `src/graph/registry.rs:12` |
| `NodeFactory::instantiate` | function | Instantiates the runtime node described by `NodeFactory`. | `src/graph/registry.rs:14` |
| `NodeFactory::validate_config` | function | Validates config for `NodeFactory`. | `src/graph/registry.rs:13` |
| `PreparedCaptureBackend::open` | function | Opens the resource represented by `PreparedCaptureBackend`. | `src/capture/capture_owner.rs:89` |
| `PreparedEndpointDriver::cancel_preparation` | function | Cancels preparation for `PreparedEndpointDriver`. | `src/endpoint/runtime.rs:328` |
| `PreparedEndpointDriver::start` | function | Makes endpoint resources ready behind the supplied closed gate. | `src/endpoint/runtime.rs:323` |
| `RunningEndpointDriver::join_and_finalize` | function | Joins `RunningEndpointDriver` and returns its finalization outcome. | `src/endpoint/runtime.rs:346` |
| `RunningEndpointDriver::observations` | function | Returns the observations exposed by `RunningEndpointDriver`. | `src/endpoint/runtime.rs:337` |
| `RunningEndpointDriver::request_shutdown` | function | Requests the selected shutdown mode from `RunningEndpointDriver`. | `src/endpoint/runtime.rs:341` |
| `RunningEndpointDriver::request_stop` | function | Requests a graceful stop from `RunningEndpointDriver`. | `src/endpoint/runtime.rs:339` |
| `RuntimeNode::prepare` | function | Prepares resources required by `RuntimeNode`. | `src/graph/runtime_node.rs:8` |
| `RuntimeNode::process` | function | Processes an input value through `RuntimeNode`. | `src/graph/runtime_node.rs:9` |
| `SourceDriver::close` | function | Closes `SourceDriver` to further work. | `src/session/extensions/source.rs:273` |
| `SourceDriver::next` | function | Produces the next source emission from `SourceDriver`. | `src/session/extensions/source.rs:269` |
| `SourceDriver::prepare` | function | Prepares resources required by `SourceDriver`. | `src/session/extensions/source.rs:268` |
| `SourceFactory::create` | function | Creates the runtime implementation described by `SourceFactory`. | `src/session/extensions/source.rs:279` |
| `SourceFactory::manifest` | function | Returns the manifest associated with `SourceFactory`. | `src/session/extensions/source.rs:277` |
| `SourceFactory::validate_config` | function | Validates config for `SourceFactory`. | `src/session/extensions/source.rs:278` |
| `StreamSignal::signal_spec` | function | Returns the signal spec associated with `StreamSignal`. | `src/session/declaration/typed_stream.rs:16` |
| `accepts` | function | Returns the accepts associated with `OperatorOutputRolePolicy`. | `src/graph/signal/operator.rs:75` |
| `accepts_delivery` | function | Returns whether accepts delivery applies to `ConnectorDeliveryReadiness`. | `src/connector/status.rs:10` |
| `accepts_delivery` | function | Returns whether accepts delivery applies to `ConnectorServiceStatus`. | `src/connector/status.rs:74` |
| `accumulated_error_ns` | function | Returns the accumulated error nanoseconds associated with `ClockDriftEstimator`. | `src/timing/clock_drift.rs:62` |
| `acquire` | function | Attempts to acquire an available buffer slot from `AudioBufferPool`. | `src/frame/pool.rs:75` |
| `acquire_failures` | function | Returns the acquire failures associated with `AudioBufferPool`. | `src/frame/pool.rs:68` |
| `add_node` | function | Adds node for `Pipeline`. | `src/graph/dsl.rs:44` |
| `after_failed_open` | function | Records a backend open failure without guessing that permission was denied. | `src/capture/authorization.rs:45` |
| `after_successful_open` | function | Records the evidence available after an explicitly selected source opens. | `src/capture/authorization.rs:31` |
| `any` | function | Convenience constructor for a deliberately open boundary port. | `src/graph/signal/spec.rs:264` |
| `api_revision` | function | Returns the API revision associated with `ConnectorManifest`. | `src/connector/manifest.rs:124` |
| `application` | function | Returns the application associated with `StreamProfile`. | `src/codec/profile.rs:41` |
| `application` | function | Returns the application associated with `Source`. | `src/session/declaration/selector.rs:140` |
| `as_mut_slice` | function | Borrows `AudioBufferHandle` as mut slice. | `src/frame/pool.rs:218` |
| `as_slice` | function | Borrows `AudioBufferHandle` as slice. | `src/frame/pool.rs:214` |
| `as_slice` | function | Borrows `SharedAudioBufferHandle` as slice. | `src/frame/pool.rs:300` |
| `as_str` | function | Returns the stable string representation of `ConnectorConfigurationErrorCode`. | `src/connector/configuration.rs:585` |
| `as_str` | function | Returns the stable string representation of `ConnectorErrorCode`. | `src/connector/error.rs:29` |
| `as_str` | function | Returns the stable string representation of `EndpointGroupId`. | `src/endpoint/identity.rs:16` |
| `as_str` | function | Returns the stable string representation of `NodeTypeId`. | `src/graph/node.rs:16` |
| `as_str` | function | Returns the stable string representation of `OperatorId`. | `src/graph/operator.rs:23` |
| `as_str` | function | Returns the stable string representation of `SignalId`. | `src/graph/signal/spec.rs:29` |
| `as_str` | function | Returns the stable string representation of `SemanticRole`. | `src/graph/signal/spec.rs:64` |
| `as_str` | function | Returns the stable string representation of `SchemaRef`. | `src/graph/signal/spec.rs:94` |
| `as_str` | function | Returns the stable string representation of `NativeExtensionLibraryErrorCode`. | `src/native_extension/mod.rs:97` |
| `as_str` | function | Returns the stable string representation of `RecordingErrorCode`. | `src/recording/error_code.rs:32` |
| `as_str` | function | Returns the stable string representation of `DeviceId`. | `src/session/declaration/selector.rs:26` |
| `as_str` | function | Returns the stable string representation of `SessionDeclarationErrorCode`. | `src/session/error_code.rs:31` |
| `as_str` | function | Returns the stable string representation of `SessionStartErrorCode`. | `src/session/error_code.rs:86` |
| `as_str` | function | Returns the stable string representation of `SessionRuntimeErrorCode`. | `src/session/error_code.rs:121` |
| `as_str` | function | Returns the stable string representation of `PolledAudioPollErrorCode`. | `src/session/error_code.rs:138` |
| `as_str` | function | Returns the stable string representation of `SessionStopCode`. | `src/session/error_code.rs:157` |
| `as_str` | function | Returns the stable string representation of `SessionStopFailureCode`. | `src/session/error_code.rs:182` |
| `as_str` | function | Returns the stable string representation of `SourceTypeId`. | `src/session/extensions/source.rs:52` |
| `async_factory` | function | Returns the async factory associated with `NodeRegistry`. | `src/graph/registry.rs:144` |
| `async_factory_by_operator` | function | Returns the async factory by operator associated with `NodeRegistry`. | `src/graph/registry.rs:151` |
| `audio` | function | Convenience constructor for PCM audio ports. | `src/graph/signal/spec.rs:269` |
| `audio_frames_enqueued_total` | function | Returns the audio frames enqueued total associated with `SessionAudioReentryMetrics`. | `src/session/lifecycle/observations.rs:368` |
| `audio_input` | function | Opens a bounded input for audio already owned by the embedding application. | `src/lib.rs:447` |
| `audio_observations` | function | Returns the audio observations associated with `RunningSession`. | `src/lib.rs:806` |
| `audio_reentry_metrics` | function | Returns exact queue, pool, loss, and lifecycle accounting for every Session-owned typed-PCM reentry into the specialized audio lane. | `src/lib.rs:874` |
| `audio_stem_id` | function | Returns the audio stem identifier associated with `EndpointRouteContext`. | `src/endpoint/runtime.rs:88` |
| `available_slots` | function | Returns the available slots associated with `AudioBufferPool`. | `src/frame/pool.rs:71` |
| `backpressure` | function | Returns the backpressure associated with `EdgeContract`. | `src/graph/ports.rs:341` |
| `before_open` | function | Records a source that was resolved but not opened because another required source failed first. | `src/capture/authorization.rs:60` |
| `binary` | function | Convenience constructor for opaque or schema-backed binary ports. | `src/graph/signal/spec.rs:299` |
| `bitrate_kbps` | function | Returns the bitrate kbps associated with `StreamProfile`. | `src/codec/profile.rs:51` |
| `bounded_async` | function | Generic bounded asynchronous edge. Connected ports supply the payload representation and the envelope preserves its producer clock. | `src/graph/ports.rs:413` |
| `browser` | function | Declares a browser/remote receiver. Register its transport implementation with [`Self::register_browser_driver`]. | `src/lib.rs:534` |
| `build` | function | Builds the Session declaration owner. | `src/lib.rs:336` |
| `builder` | function | Creates a builder for declaring `Session` sources, routes, and endpoints. | `src/lib.rs:374` |
| `bundle_id` | function | Returns the bundle identifier associated with `ApplicationSelector`. | `src/session/declaration/selector.rs:44` |

## Interpretation

An inventory row establishes that a declaration, test, lifecycle operation, configuration surface, or protocol element exists at the frozen snapshot. Fields shown as unknown remain deliberately unspecified. Consult the native API and error contract before relying on panic behavior, blocking, cancellation, ordering, limits, retry, or recovery.

## Related documentation

- [Glossary](/docs/glossary.md)
- [PocketStation](/README.md)
- [Behavior evidence index](/docs/reference/behavior-evidence.md)
- [Configuration reference](/docs/reference/configuration.md)
- [Frames or signals are dropped](/docs/troubleshooting/drops-and-saturation.md)
- [Protocol surface index](/docs/reference/protocol-surface.md)
- [Capture failures](/docs/errors/capture.md)
- [Session stop reports component failures](/docs/troubleshooting/session-stop.md)

## Evidence boundary

This page was verified against Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `src/lib.rs:1-1129` (`DIRECT`)

A file's presence proves implementation or declaration at this snapshot. It does not by itself prove physical-device qualification, operational performance, retry safety, or behavior outside the recorded test conditions.
