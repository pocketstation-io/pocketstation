# Rust API reference

<!-- claims: CLM-REF-001-SCOPE-001,CLM-REF-001-TEXT-001,CLM-REF-001-TEXT-002,CLM-REF-001-SOURCE-001 -->

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
- **Run connector workers.** Supervise connector delivery, acknowledgement, readiness, cancellation, drain, and abort while reporting retry attempts and typed retryability.
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

The scope of **Rust API reference** ends at the native contracts and executable conditions cited below. Platform qualification, performance, retry, and delivery require their own explicit evidence.

## Reference authority

For **Rust API reference**, the generated [docs.rs API](https://docs.rs/pocketstation/latest/pocketstation/) provides compiler-rendered signatures and navigation. This repository page adds the frozen evidence identifiers, responsibilities, and cross-boundary interpretation used by the documentation verifier.

## Public surface

| Evidence record | Declaration | Kind | Purpose | Source |
|---|---|---|---|---|
| sym-f490a9028837996edb07 | `INITIAL` | assoc_const | Provides the initial value for `PermissionEpoch`. | `src/capture/authorization.rs:270` |
| sym-b8894596b234e7b11358 | `INITIAL` | assoc_const | Provides the initial value for `SourceGeneration`. | `src/capture/events.rs:15` |
| sym-f5882a0a4b9983601b78 | `Error` | assoc_type | Specifies the error type returned by `SidecarMessageKind` operations. | `src/runtime/lifecycle/sidecar_protocol.rs:23` |
| sym-3eb7316123cefcdc0aa2 | `pocketstation::abi::extension::PKS_EXTENSION_ABI_MAJOR` | constant | Defines the major version of extension ABI. | `src/abi/extension.rs:7` |
| sym-a60dfa04bb15c0df7de2 | `pocketstation::abi::extension::PKS_EXTENSION_ABI_MINOR` | constant | Defines the minor version of extension ABI. | `src/abi/extension.rs:8` |
| sym-e70b2bf8528a2a7977f6 | `pocketstation::capture::capture_owner::CAPTURE_MONOTONIC_CLOCK_DOMAIN_ID` | constant | Monotonic timestamp domain used by native capture backends. | `src/capture/capture_owner.rs:20` |
| sym-dcc09b0402dcc4a2b2e8 | `pocketstation::capture::events::MAX_SOURCE_RUNTIME_EVENT_OWNED_BYTES` | constant | Maximum heap storage retained by one queued capture-runtime event. | `src/capture/events.rs:72` |
| sym-fef590d7370c703d21f0 | `pocketstation::codec::constants::OPUS_FRAME_SAMPLES` | constant | 20 ms frame = 960 samples at 48 kHz (AUDIO-012). | `src/codec/constants.rs:5` |
| sym-2a520fd0f59b0d4e8007 | `pocketstation::codec::constants::OPUS_MAX_PACKET_BYTES` | constant | Maximum number of bytes the Opus encoder can emit per 20 ms frame. libopus guarantees this upper bound. | `src/codec/constants.rs:13` |
| sym-a09db02191ca16a128fa | `pocketstation::codec::constants::OPUS_SAMPLE_RATE_HZ` | constant | 48 000 Hz, mono, VOIP application profile (AUDIO-012 default). | `src/codec/constants.rs:2` |
| sym-0873bf4291fe4840e3c7 | `pocketstation::codec::constants::VOICE_AGENT_FRAME_SAMPLES` | constant | 10 ms frame = 480 samples at 48 kHz (voice-agent low-latency mode, RFC 6716 §3.1). Ten milliseconds of mono PCM at 48 kHz. | `src/codec/constants.rs:9` |
| sym-1b91ac08f16299bb9ee6 | `pocketstation::conformance::EXTENSION_ENDPOINT_ID` | constant | Defines extension endpoint identifier as `"org.pocketstation.conformance.endpoint.v1"` for the owning public contract. | `src/conformance.rs:558` |
| sym-b113b77f933e5eafa4d1 | `pocketstation::conformance::EXTENSION_ENDPOINT_INPUT_PORT` | constant | Defines extension endpoint input port as `"in"` for the owning public contract. | `src/conformance.rs:563` |
| sym-7a425eb8f81556b0e37d | `pocketstation::conformance::EXTENSION_ENDPOINT_NODE_ID` | constant | Defines extension endpoint node identifier as `"org.pocketstation.conformance.endpoint-node.v1"` for the owning public contract. | `src/conformance.rs:559` |
| sym-8805b326aa49232d5900 | `pocketstation::conformance::EXTENSION_INPUT_PAYLOAD` | constant | Defines extension input payload as `b"seed"` for the owning public contract. | `src/conformance.rs:564` |
| sym-68245ccb5fa39e2a1dca | `pocketstation::conformance::EXTENSION_OPERATOR_ID` | constant | Defines extension operator identifier as `"org.pocketstation.conformance.operator.v1"` for the owning public contract. | `src/conformance.rs:556` |
| sym-6533c98a2e4ca1673b93 | `pocketstation::conformance::EXTENSION_OPERATOR_INPUT_PORT` | constant | Defines extension operator input port as `"in"` for the owning public contract. | `src/conformance.rs:561` |
| sym-33e6dc22e3396e947b17 | `pocketstation::conformance::EXTENSION_OPERATOR_NODE_ID` | constant | Defines extension operator node identifier as `"org.pocketstation.conformance.operator-node.v1"` for the owning public contract. | `src/conformance.rs:557` |
| sym-7875f549dec972c290de | `pocketstation::conformance::EXTENSION_OPERATOR_OUTPUT_PORT` | constant | Defines extension operator output port as `"out"` for the owning public contract. | `src/conformance.rs:562` |
| sym-87697e19133317067c8b | `pocketstation::conformance::EXTENSION_OUTPUT_PAYLOAD` | constant | Defines extension output payload as `b"seed!"` for the owning public contract. | `src/conformance.rs:565` |
| sym-b7b698a0e3c1bff9d57f | `pocketstation::conformance::EXTENSION_ROLE_ID` | constant | Defines extension role identifier as `"org.pocketstation.conformance.terminal.v1"` for the owning public contract. | `src/conformance.rs:554` |
| sym-bcc519efe742ec26b8c5 | `pocketstation::conformance::EXTENSION_SCHEMA_ID` | constant | Defines extension schema identifier as `"urn:pocketstation:conformance:signal:v1"` for the owning public contract. | `src/conformance.rs:553` |
| sym-764b674cf9a07c7fb6a2 | `pocketstation::conformance::EXTENSION_SIGNAL_ID` | constant | Defines extension signal identifier as `"org.pocketstation.conformance.signal.v1"` for the owning public contract. | `src/conformance.rs:552` |
| sym-60f3c6446c71904985cd | `pocketstation::conformance::EXTENSION_SOURCE_PORT` | constant | Defines extension source port as `"out"` for the owning public contract. | `src/conformance.rs:560` |
| sym-b5041e636bc2a19f0785 | `pocketstation::conformance::EXTENSION_SOURCE_TYPE_ID` | constant | Defines extension source type identifier as `"org.pocketstation.conformance.source.fixture.v1"` for the owning public contract. | `src/conformance.rs:555` |
| sym-813da3fa28b721c75f12 | `pocketstation::conformance::FRAMES_PER_SOURCE` | constant | Frames emitted per source by the finite deterministic fixture. | `src/conformance.rs:39` |
| sym-97fbc7a8960de5a95f06 | `pocketstation::conformance::OBSERVED_CONNECTOR_OPERATOR_ID` | constant | Defines observed connector operator identifier as `"io.pocketstation.conformance.connector.v1"` for the owning public contract. | `src/conformance.rs:40` |
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
| sym-7c9bc39a246042394c48 | `pocketstation::frame::audio::POOL_SLOT_SAMPLES` | constant | Defines pool slot samples as `960` for the owning public contract. | `src/frame/audio.rs:10` |
| sym-c747591501f5767e9363 | `pocketstation::frame::audio::SAMPLE_RATE_HZ` | constant | Defines sample rate hertz as `48_000` for the owning public contract. | `src/frame/audio.rs:6` |
| sym-af2221631095d6323bae | `pocketstation::frame::pool::POOL_MAX_SLOTS` | constant | Defines pool max slots as `64` for the owning public contract. | `src/frame/pool.rs:11` |
| sym-7e71261c61f78cefd417 | `pocketstation::graph::operator::OPERATOR_ID_SYNTAX_VERSION` | constant | Version of the serialized operator-identifier syntax. | `src/graph/operator.rs:6` |
| sym-3cc63fdcaf2e3841e729 | `pocketstation::graph::plan::EDGE_RECEIVER_MAX_IN_FLIGHT_FRAMES` | constant | A sequential edge receiver may retain the frame it just popped while it processes that frame. Copy-pool sizing must cover that owned frame in addition to every frame that can still be queued. | `src/graph/plan.rs:16` |
| sym-a7b4280132651539cc89 | `pocketstation::graph::plan::EDGE_RING_CAPACITY_FRAMES` | constant | Defines edge ring capacity frames as `8` for the owning public contract. | `src/graph/plan.rs:12` |
| sym-202840de95069aac7815 | `pocketstation::graph::plan::FRAME_BYTES_MONO_48K` | constant | Defines the stable frame bytes mono 48 k used by the owning public contract. | `src/graph/plan.rs:11` |
| sym-eec261911eeb3838d12f | `pocketstation::graph::plan::MAX_EDGE_RING_CAPACITY_FRAMES` | constant | Sets the maximum supported edge ring capacity frames. | `src/graph/plan.rs:17` |
| sym-6104a324170be362b4e2 | `pocketstation::graph::ports::MAX_ASYNC_PAYLOAD_BYTES` | constant | Sets the maximum supported async payload bytes. | `src/graph/ports.rs:13` |
| sym-61cc214c9f17f5efc20a | `pocketstation::native_extension::EXTENSION_LIBRARY_ENTRYPOINT_V1` | constant | Exact exported symbol required from a native Extension ABI v1 dynamic library. The suffix follows the ABI major; compatible minor revisions use the same entrypoint. | `src/native_extension/mod.rs:24` |
| sym-954bd5d43215025cbd67 | `pocketstation::recording::endpoint::MULTISTEM_GROUP_CONFIGURATION_KEY` | constant | Defines multistem group configuration key as `"recording_group_id"` for the owning public contract. | `src/recording/endpoint.rs:24` |
| sym-2a6d00c225bdfa9776c5 | `pocketstation::recording::endpoint::MULTISTEM_NAME_CONFIGURATION_KEY` | constant | Defines multistem name configuration key as `"stem_name"` for the owning public contract. | `src/recording/endpoint.rs:25` |
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
| sym-7e140bdb0b67db980cf5 | `pocketstation::timing::PROCESS_MONOTONIC_CLOCK_DOMAIN_ID` | constant | Clock-domain identity for timestamps produced by PocketStation's shared process-wide monotonic clock. | `src/timing/mod.rs:24` |
| sym-04b18c890463686203e5 | `pocketstation::SessionCancelDisposition` | enum | Classifies the observable session cancel disposition. | `src/lib.rs:1117` |
| sym-d2105b3875e7c40fd9b0 | `pocketstation::SessionEndpointError` | enum | Classifies failures surfaced by session endpoint operations. | `src/lib.rs:1067` |
| sym-776e1e8e18780e7b1a73 | `pocketstation::SessionOperatorError` | enum | Classifies failures surfaced by session operator operations. | `src/lib.rs:1083` |
| sym-30de3aa8567402102ec0 | `pocketstation::SessionRuntimeError` | enum | Classifies failures produced during session runtime execution. | `src/lib.rs:1105` |
| sym-99f7c3ce73181797e98a | `pocketstation::SessionSidecarError` | enum | Classifies failures surfaced by session sidecar operations. | `src/lib.rs:1077` |
| sym-ac25c018744df2954349 | `pocketstation::SessionSourceError` | enum | Classifies failures surfaced by session source operations. | `src/lib.rs:1089` |
| sym-4072783c0ed4a6edf51c | `pocketstation::SessionStartErrorKind` | enum | Selects the session start error kind used by PocketStation. | `src/lib.rs:1095` |
| sym-ee99ebae0b895c91d5d0 | `pocketstation::SessionStopDisposition` | enum | Classifies the observable session stop disposition. | `src/lib.rs:1111` |
| sym-ce4c2a4d569001a1911b | `pocketstation::abi::extension::PksExtensionKind` | enum | Selects the extension kind used by PocketStation. | `src/abi/extension.rs:32` |
| sym-9296aa9eb2ebd632f22e | `pocketstation::abi::extension::PksExtensionPortDirection` | enum | Selects the extension port direction used by PocketStation. | `src/abi/extension.rs:40` |
| sym-81e037fbd73e62639416 | `pocketstation::abi::session::abi::PksSessionStatusCode` | enum | Provides stable C ABI status categories returned by Session operations. | `src/abi/session/abi.rs:79` |
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
| sym-222549994b61881e820d | `pocketstation::codec::decoder::OpusDecodeError` | enum | Classifies failures produced during opus decoding. | `src/codec/decoder.rs:25` |
| sym-8a3a4031d3f6082ba4c1 | `pocketstation::codec::encoder::OpusApplication` | enum | Selects the Opus encoder mode used to tune speech or general audio. | `src/codec/encoder.rs:58` |
| sym-9226f5cf30a6da6fae37 | `pocketstation::codec::encoder::OpusChannels` | enum | Typed channel count for Opus — prevents silent u8 misuse. | `src/codec/encoder.rs:27` |
| sym-184d3f010be6d087cba0 | `pocketstation::codec::encoder::OpusEncodeError` | enum | Classifies failures produced during opus encoding. | `src/codec/encoder.rs:131` |
| sym-4a458387887842682d94 | `pocketstation::codec::encoder::OpusFrameDuration` | enum | Supported Opus frame duration at 48 kHz. | `src/codec/encoder.rs:7` |
| sym-8cbe8e70763fec07c936 | `pocketstation::codec::encoder::OpusSampleRate` | enum | Typed sample rate. Opus internally always uses 48 kHz; this type makes the constraint explicit rather than hiding it behind a `u32` constant. | `src/codec/encoder.rs:44` |
| sym-292549adb1d699be0ee1 | `pocketstation::codec::profile::StreamProfile` | enum | Selects the supported Opus stream profile used for codec validation. | `src/codec/profile.rs:11` |
| sym-d9688fdb13af3de4be20 | `pocketstation::conformance::ObservedEndpointError` | enum | Classifies failures surfaced by observed endpoint operations. | `src/conformance.rs:344` |
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
| sym-75c0d9ea95fe08480487 | `pocketstation::frame::audio::AudioFrameBuildError` | enum | Classifies failures produced during audio frame construction and input validation. | `src/frame/audio.rs:51` |
| sym-e372b47263dc149e2565 | `pocketstation::frame::audio::FrameLineageError` | enum | Classifies failures surfaced by frame lineage operations. | `src/frame/audio.rs:250` |
| sym-8dcddebc5c2a3910991b | `pocketstation::frame::audio::SampleFormat` | enum | Selects the sample format used by PocketStation. | `src/frame/audio.rs:13` |
| sym-4343e1e50b9eca8f5850 | `pocketstation::frame::lineage::FrameLineageBuildError` | enum | Classifies failures produced during frame lineage construction and input validation. | `src/frame/lineage.rs:93` |
| sym-7e350671a794cf32f491 | `pocketstation::frame::platform::Platform` | enum | Identifies the operating-system platform attached to captured lineage. | `src/frame/platform.rs:4` |
| sym-9e482a2d7e66b0d9c0e6 | `pocketstation::frame::pool::AudioBufferWriteError` | enum | Classifies failures produced during audio buffer writing. | `src/frame/pool.rs:14` |
| sym-ef308cb6434678c35687 | `pocketstation::graph::compile::resolve::CompileError` | enum | Classifies failures surfaced by compile operations. | `src/graph/compile/resolve.rs:26` |
| sym-f23fbf485a1abdca3ff2 | `pocketstation::graph::node::ConfigError` | enum | Classifies failures surfaced by config operations. | `src/graph/node.rs:141` |
| sym-63eacff5c07a2f53dc8d | `pocketstation::graph::node::NodeDescriptorError` | enum | Classifies failures surfaced by node descriptor operations. | `src/graph/node.rs:252` |
| sym-8839c0168f2c08f2c049 | `pocketstation::graph::node::NodeError` | enum | Classifies failures surfaced by node operations. | `src/graph/node.rs:149` |
| sym-62ce3a27efa7fea45df0 | `pocketstation::graph::partition::ExecutionPartition` | enum | WHERE an operator runs. | `src/graph/partition.rs:18` |
| sym-142ebf4cccf857875822 | `pocketstation::graph::partition::SafetyContract` | enum | WHAT an operator guarantees about its runtime behaviour. | `src/graph/partition.rs:82` |
| sym-04dd5abde97700d6bea8 | `pocketstation::graph::plan::PlanError` | enum | Classifies failures surfaced by plan operations. | `src/graph/plan.rs:21` |
| sym-bb29d2e9bc5ef5790a5b | `pocketstation::graph::ports::BackpressurePolicy` | enum | Selects the backpressure policy used by PocketStation. | `src/graph/ports.rs:265` |
| sym-8c71eb7bf77b533a3950 | `pocketstation::graph::ports::ChannelLayout` | enum | Declares the number and arrangement of channels in an audio signal. | `src/graph/ports.rs:27` |
| sym-c994bd3c6236b085ea39 | `pocketstation::graph::ports::ClockDomain` | enum | Identifies the clock used to interpret signal timestamps. | `src/graph/ports.rs:249` |
| sym-b708b27c76144c20fa7b | `pocketstation::graph::ports::CopyPolicy` | enum | Selects the copy policy used by PocketStation. | `src/graph/ports.rs:280` |
| sym-780123bab167f93fc9d2 | `pocketstation::graph::ports::DeliverySemantics` | enum | Selects the delivery semantics used by PocketStation. | `src/graph/ports.rs:273` |
| sym-7cb8e2639c16f6f287d7 | `pocketstation::graph::ports::EdgeObservabilityLevel` | enum | Selects the edge observability level used by PocketStation. | `src/graph/ports.rs:294` |
| sym-0940509ee3453cf34ddf | `pocketstation::graph::ports::LossPolicy` | enum | Selects the loss policy used by PocketStation. | `src/graph/ports.rs:287` |
| sym-822c3c845e91e2ffb25a | `pocketstation::graph::ports::MediaCaps` | enum | Declares the media capabilities accepted by a graph port. | `src/graph/ports.rs:85` |
| sym-74305f5acfae6a732e2b | `pocketstation::graph::ports::MediaKind` | enum | Selects the media kind used by PocketStation. | `src/graph/ports.rs:16` |
| sym-1b42b2cf2aa6f3bb4d1a | `pocketstation::graph::ports::Multiplicity` | enum | Declares whether a graph port accepts one edge or multiple edges. | `src/graph/ports.rs:169` |
| sym-c3e0d763e39512ba85d5 | `pocketstation::graph::ports::PortDirection` | enum | Selects the port direction used by PocketStation. | `src/graph/ports.rs:163` |
| sym-b9f6cf6d84646f2f54cd | `pocketstation::graph::ports::PortSpecError` | enum | Classifies failures surfaced by port spec operations. | `src/graph/ports.rs:239` |
| sym-9f1d225abf797e856b20 | `pocketstation::graph::registry::NodeDefinitionRef` | enum | Borrows either a synchronous or asynchronous registered node definition. | `src/graph/registry.rs:32` |
| sym-08a016336466041e5717 | `pocketstation::graph::registry::NodeRegistrationError` | enum | Classifies failures produced during node registration. | `src/graph/registry.rs:57` |
| sym-578043ee573a0f9b99e6 | `pocketstation::graph::signal::continuity::SignalContinuityError` | enum | Classifies failures surfaced by signal continuity operations. | `src/graph/signal/continuity.rs:89` |
| sym-a4afff76933f8071989c | `pocketstation::graph::signal::envelope::SignalEnvelopeError` | enum | Classifies failures surfaced by signal envelope operations. | `src/graph/signal/envelope.rs:137` |
| sym-5b1af0913743b3dc15fe | `pocketstation::graph::signal::lineage::SignalDerivationError` | enum | Classifies failures surfaced by signal derivation operations. | `src/graph/signal/lineage.rs:159` |
| sym-321d364165d7c21bacec | `pocketstation::graph::signal::lineage::SignalLineageError` | enum | Classifies failures surfaced by signal lineage operations. | `src/graph/signal/lineage.rs:86` |
| sym-7c4acf8b5348e2b02362 | `pocketstation::graph::signal::operator::AsyncOperatorManifestError` | enum | Classifies failures surfaced by async operator manifest operations. | `src/graph/signal/operator.rs:321` |
| sym-de8d9bf729d99cd1b90c | `pocketstation::graph::signal::operator::OperatorCancellationPolicy` | enum | Selects the operator cancellation policy used by PocketStation. | `src/graph/signal/operator.rs:57` |
| sym-c2d8be1a7356019be8c9 | `pocketstation::graph::signal::operator::OperatorFailurePolicy` | enum | Selects the operator failure policy used by PocketStation. | `src/graph/signal/operator.rs:63` |
| sym-d1b44af5c9c63b1683e8 | `pocketstation::graph::signal::payload::SignalPayload` | enum | Carries the typed audio, text, event, or binary body of a signal envelope. | `src/graph/signal/payload.rs:10` |
| sym-5ee081d3cfce2f617af2 | `pocketstation::graph::signal::spec::BinaryFormat` | enum | Binary encoding hint for `SignalClass::Binary`. | `src/graph/signal/spec.rs:141` |
| sym-1b3317725d97bbefcd4d | `pocketstation::graph::signal::spec::Codec` | enum | Audio encoding format for `SignalClass::EncodedAudio`. | `src/graph/signal/spec.rs:113` |
| sym-1effef32701817ca9bef | `pocketstation::graph::signal::spec::EventFormat` | enum | Event structure hint for `SignalClass::Event`. | `src/graph/signal/spec.rs:132` |
| sym-24b831fc21a3c9d7c3d3 | `pocketstation::graph::signal::spec::SignalClass` | enum | The fundamental class of data flowing through a port. | `src/graph/signal/spec.rs:156` |
| sym-b386818365cee36fb88f | `pocketstation::graph::signal::spec::SignalSpecError` | enum | Classifies failures surfaced by signal spec operations. | `src/graph/signal/spec.rs:351` |
| sym-30151b96c6d0821dc2db | `pocketstation::graph::signal::spec::TextFormat` | enum | Text encoding hint for `SignalClass::Text`. | `src/graph/signal/spec.rs:124` |
| sym-d6d356025cf7255edba3 | `pocketstation::graph::signal::timing::SignalTimingError` | enum | Classifies failures surfaced by signal timing operations. | `src/graph/signal/timing.rs:89` |
| sym-3b00d82a04d35afbba20 | `pocketstation::native_extension::NativeExtensionKind` | enum | Selects the native extension kind used by PocketStation. | `src/native_extension/mod.rs:27` |
| sym-c144f166f28ea7fc2604 | `pocketstation::native_extension::NativeExtensionLibraryErrorCode` | enum | Provides stable categories for native-extension load and validation failures. | `src/native_extension/mod.rs:78` |
| sym-145f00201ff09d31790f | `pocketstation::recording::config::PermissionDecision` | enum | Records whether recording permission was granted, denied, or not observable. | `src/recording/config.rs:43` |
| sym-b8eeef703f71e3e42e6a | `pocketstation::recording::config::PermissionScope` | enum | Selects the permission scope used by PocketStation. | `src/recording/config.rs:50` |
| sym-701a0fa1d075d83a1e69 | `pocketstation::recording::config::RecorderLineageField` | enum | Identifies the lineage field that differs while validating a recording stem. | `src/recording/config.rs:10` |
| sym-57e0714519b0b3c6db3a | `pocketstation::recording::error_code::RecordingErrorCode` | enum | Stable language-neutral code for a recording failure. | `src/recording/error_code.rs:9` |
| sym-03b3fb37a492e410c60b | `pocketstation::recording::writer::DiscontinuityKind` | enum | Selects the discontinuity kind used by PocketStation. | `src/recording/writer.rs:105` |
| sym-a42d8491068ec2279223 | `pocketstation::recording::writer::RecorderError` | enum | Classifies failures surfaced by recorder operations. | `src/recording/writer.rs:24` |
| sym-8d0381d85b60078d9ed3 | `pocketstation::recording::writer::RecordingState` | enum | Selects the recording state used by PocketStation. | `src/recording/writer.rs:86` |
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
| sym-b3e6577a839ba5232766 | `pocketstation::timing::domain::ClockDomainKind` | enum | The authority that defines timestamps carried by one clock-domain ID. | `src/timing/domain.rs:7` |
| sym-f73c46c82e8ef2c1787d | `pocketstation::timing::domain::ClockDomainOrigin` | enum | The origin against which timestamps in one clock domain are measured. | `src/timing/domain.rs:15` |
| sym-8c1e01f6032ee9f58e34 | `ActiveCaptureBackend::observation_handle` | function | Returns a handle for reading observations from `ActiveCaptureBackend`. | `src/capture/capture_owner.rs:107` |
| sym-181be60c05b799d9aa88 | `ActiveCaptureBackend::observations` | function | Returns the observations exposed by `ActiveCaptureBackend`. | `src/capture/capture_owner.rs:109` |
| sym-bb192a71775dad273804 | `ActiveCaptureBackend::source_id` | function | Resolved native source identity for every frame emitted by this open. | `src/capture/capture_owner.rs:105` |
| sym-d5183a64d1a21d804af5 | `ActiveCaptureBackend::stop_and_join` | function | Stops `ActiveCaptureBackend`, joins its worker, and returns the terminal result. | `src/capture/capture_owner.rs:111` |
| sym-9642176d7ce0fae2cfdb | `AsyncNode::cancel` | function | Requests cancellation of `AsyncNode`. | `src/graph/signal/operator.rs:36` |
| sym-5c9e1eb4c40189e4507c | `AsyncNode::close` | function | Closes `AsyncNode` to further work. | `src/graph/signal/operator.rs:40` |
| sym-d19b1106016b42c888a3 | `AsyncNode::flush` | function | Flushes pending output from `AsyncNode` at the end of a run. | `src/graph/signal/operator.rs:32` |
| sym-810a3c6d136ea253d9c1 | `AsyncNode::prepare` | function | Prepares resources required by `AsyncNode`. | `src/graph/signal/operator.rs:14` |
| sym-60579117d36ce3ae29e0 | `AsyncNode::process` | function | Processes an input value through `AsyncNode`. | `src/graph/signal/operator.rs:19` |
| sym-f7b2dd8f4291dd8d5924 | `AsyncNode::process_port` | function | Returns the process port held by `AsyncNode`. | `src/graph/signal/operator.rs:24` |
| sym-ab50a788828394fe1a21 | `AsyncOperatorFactory::create` | function | Creates the runtime implementation described by `AsyncOperatorFactory`. | `src/graph/signal/operator.rs:378` |
| sym-0a1384e3fd04b4d39049 | `AsyncOperatorFactory::manifest` | function | Returns the manifest held by `AsyncOperatorFactory`. | `src/graph/signal/operator.rs:369` |
| sym-807e19db4b5ec22d82c8 | `AsyncOperatorFactory::resolve_manifest` | function | Resolves and validates the operator manifest exposed by `AsyncOperatorFactory`. | `src/graph/signal/operator.rs:371` |
| sym-058eb6025ed335fcad46 | `AsyncOperatorFactory::validate_config` | function | Validates supplied node configuration against the schema declared by `AsyncOperatorFactory`. | `src/graph/signal/operator.rs:370` |
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
| sym-95d186f05bc1b9a0e1fc | `NodeDefinition::descriptor` | function | Returns the descriptor associated with `NodeDefinition`. | `src/graph/registry.rs:22` |
| sym-756730521855d6410b74 | `NodeDefinition::validate_config` | function | Validates supplied node configuration against the schema declared by `NodeDefinition`. | `src/graph/registry.rs:23` |
| sym-5f3b142ed649e97a8a16 | `NodeFactory::descriptor` | function | Returns the descriptor associated with `NodeFactory`. | `src/graph/registry.rs:12` |
| sym-308f05bdc22e7d8614b8 | `NodeFactory::instantiate` | function | Instantiates the runtime node described by `NodeFactory`. | `src/graph/registry.rs:14` |
| sym-0c7ecbd3f919b7ef72eb | `NodeFactory::validate_config` | function | Validates supplied node configuration against the schema declared by `NodeFactory`. | `src/graph/registry.rs:13` |
| sym-1076d488d679ca8ce004 | `PreparedCaptureBackend::open` | function | Opens the resource represented by `PreparedCaptureBackend`. | `src/capture/capture_owner.rs:89` |
| sym-78c0c0868e0e6890db43 | `PreparedEndpointDriver::cancel_preparation` | function | Cancels resources created while preparing `PreparedEndpointDriver`. | `src/endpoint/runtime.rs:328` |
| sym-4616280bf6b55d26508d | `PreparedEndpointDriver::start` | function | Makes endpoint resources ready behind the supplied closed gate. | `src/endpoint/runtime.rs:323` |
| sym-10bdf73b1868933ea548 | `RunningEndpointDriver::join_and_finalize` | function | Joins and finalize for `RunningEndpointDriver`. | `src/endpoint/runtime.rs:346` |
| sym-cd04e67c105bddcbcec6 | `RunningEndpointDriver::observations` | function | Returns the observations exposed by `RunningEndpointDriver`. | `src/endpoint/runtime.rs:337` |
| sym-ad8b16eca0934003b206 | `RunningEndpointDriver::request_shutdown` | function | Requests the selected shutdown mode from `RunningEndpointDriver`. | `src/endpoint/runtime.rs:341` |
| sym-6815ea1b0e23a3e58e2f | `RunningEndpointDriver::request_stop` | function | Requests a graceful stop from `RunningEndpointDriver`. | `src/endpoint/runtime.rs:339` |
| sym-910d45960a7a0e969a3b | `RuntimeNode::prepare` | function | Prepares resources required by `RuntimeNode`. | `src/graph/runtime_node.rs:8` |
| sym-f6799f84a742f4bbb94e | `RuntimeNode::process` | function | Processes an input value through `RuntimeNode`. | `src/graph/runtime_node.rs:9` |
| sym-995aa9622b5ba2b4a748 | `SourceDriver::close` | function | Closes `SourceDriver` to further work. | `src/session/extensions/source.rs:273` |
| sym-df9cd8b6772b89c63bda | `SourceDriver::next` | function | Produces the next source emission from `SourceDriver`. | `src/session/extensions/source.rs:269` |
| sym-56f0802ba994d7ac9843 | `SourceDriver::prepare` | function | Prepares resources required by `SourceDriver`. | `src/session/extensions/source.rs:268` |
| sym-93000b7d23bf0f46007d | `SourceFactory::create` | function | Creates the runtime implementation described by `SourceFactory`. | `src/session/extensions/source.rs:279` |
| sym-c1d7fb39869e2d34f307 | `SourceFactory::manifest` | function | Returns the manifest held by `SourceFactory`. | `src/session/extensions/source.rs:277` |
| sym-6a881085d8de6dbbb9e6 | `SourceFactory::validate_config` | function | Validates supplied node configuration against the schema declared by `SourceFactory`. | `src/session/extensions/source.rs:278` |
| sym-7370e5df776e21bbd7b7 | `StreamSignal::signal_spec` | function | Returns the signal spec held by `StreamSignal`. | `src/session/declaration/typed_stream.rs:16` |
| sym-1dc345935cb016e9ada7 | `accepts` | function | Returns whether accepts is true for `OperatorOutputRolePolicy`. | `src/graph/signal/operator.rs:75` |
| sym-42a8e5c4d0cbc083c502 | `accepts_delivery` | function | Reports whether accepts delivery is true for `ConnectorDeliveryReadiness`. | `src/connector/status.rs:10` |
| sym-8d570ccfcc6f8f22adb8 | `accepts_delivery` | function | Reports whether accepts delivery is true for `ConnectorServiceStatus`. | `src/connector/status.rs:74` |
| sym-e9bc18581c3e02587015 | `accumulated_error_ns` | function | Returns the accumulated error nanoseconds held by `ClockDriftEstimator`. | `src/timing/clock_drift.rs:62` |
| sym-c5be09000b38bb0c866a | `acquire` | function | Attempts to acquire an available buffer slot from `AudioBufferPool`. | `src/frame/pool.rs:75` |
| sym-ba21839ffaec3909eb0b | `acquire_failures` | function | Returns the acquire failures associated with `AudioBufferPool`. | `src/frame/pool.rs:68` |
| sym-dc0c82c0beb1a6eb87e9 | `actual` | function | Returns the observed value when a compilation diagnostic compares two values. | `src/session/compile/error.rs:153` |
| sym-33c855e1e7e0d665cbfb | `add_node` | function | Adds one node declaration to the graph owned by `Pipeline`. | `src/graph/dsl.rs:44` |
| sym-3e242114552b00229671 | `advance` | function | Returns this buffer's source-time start and advances the next start. | `src/capture/timeline.rs:74` |
| sym-27a1b14acbb0dfc6e31c | `advance_from_source_position` | function | Returns a buffer's source-time start from its native sample-frame position. Forward gaps are preserved in the returned timestamp without separately advancing this clock from an aggregate drop counter. | `src/capture/timeline.rs:90` |
| sym-71d554d30a07cbb7cc77 | `after_failed_open` | function | Records a backend open failure without guessing that permission was denied. | `src/capture/authorization.rs:45` |
| sym-9db2a2c278a3325269ca | `after_successful_open` | function | Records the evidence available after an explicitly selected source opens. | `src/capture/authorization.rs:31` |
| sym-0d9c647ddb36695c0df0 | `anchored` | function | Creates a sample timeline whose first buffer starts at the supplied nonzero monotonic timestamp. | `src/capture/timeline.rs:62` |
| sym-2e9bb7ccca392ce72b7f | `any` | function | Convenience constructor for a deliberately open boundary port. | `src/graph/signal/spec.rs:264` |
| sym-e9973a9598777cbaefb0 | `api_revision` | function | Returns the API revision held by `ConnectorManifest`. | `src/connector/manifest.rs:124` |
| sym-6f89da131606829b0842 | `application` | function | Returns the application held by `StreamProfile`. | `src/codec/profile.rs:41` |
| sym-1682a40407799112e8f3 | `application` | function | Returns the application held by `Source`. | `src/session/declaration/selector.rs:140` |
| sym-4393ee74246a52c49218 | `as_mut_slice` | function | Borrows `AudioBufferHandle` as mut slice. | `src/frame/pool.rs:218` |
| sym-cb4dc9d97c99890dec4a | `as_slice` | function | Borrows `AudioBufferHandle` as slice. | `src/frame/pool.rs:214` |
| sym-5895db9d0220cfc81825 | `as_slice` | function | Borrows `SharedAudioBufferHandle` as slice. | `src/frame/pool.rs:300` |
| sym-ecb602e99c0dba24af4f | `as_str` | function | Returns the stable string representation of `ConnectorConfigurationErrorCode`. | `src/connector/configuration.rs:585` |
| sym-f5caa543ea9c940cafb1 | `as_str` | function | Returns the stable string representation of `ConnectorErrorCode`. | `src/connector/error.rs:29` |
| sym-6acbd29b6724de4b584f | `as_str` | function | Returns the stable string representation of `EndpointGroupId`. | `src/endpoint/identity.rs:16` |
| sym-c4d691ed75e3107f9e7d | `as_str` | function | Returns the stable string representation of `NodeTypeId`. | `src/graph/node.rs:16` |
| sym-c1514acd18e430e01b9c | `as_str` | function | Returns the stable string representation of `OperatorId`. | `src/graph/operator.rs:23` |
| sym-e7833d1e66813f75c4fa | `as_str` | function | Returns the stable string representation of `SignalId`. | `src/graph/signal/spec.rs:29` |
| sym-afb07b88012cf5d9f435 | `as_str` | function | Returns the stable string representation of `SemanticRole`. | `src/graph/signal/spec.rs:64` |
| sym-705795c35cd0de970c84 | `as_str` | function | Returns the stable string representation of `SchemaRef`. | `src/graph/signal/spec.rs:94` |
| sym-504709804e2abc76d105 | `as_str` | function | Returns the stable string representation of `NativeExtensionLibraryErrorCode`. | `src/native_extension/mod.rs:97` |
| sym-30397adab4cc0ba2e559 | `as_str` | function | Returns the stable string representation of `StemLabel`. | `src/recording/config.rs:36` |
| sym-02f17b61160530b6c94b | `as_str` | function | Returns the stable string representation of `RecordingErrorCode`. | `src/recording/error_code.rs:32` |
| sym-11d0c6769d2d0c38fd3d | `as_str` | function | Returns the stable string representation of `DeviceId`. | `src/session/declaration/selector.rs:26` |
| sym-642103f95789f4c34de4 | `as_str` | function | Returns the stable string representation of `SessionDeclarationErrorCode`. | `src/session/error_code.rs:31` |
| sym-098447029f757e3ee3fc | `as_str` | function | Returns the stable string representation of `SessionStartErrorCode`. | `src/session/error_code.rs:86` |
| sym-7496f586df598ab4c92a | `as_str` | function | Returns the stable string representation of `SessionRuntimeErrorCode`. | `src/session/error_code.rs:121` |
| sym-ef6574dbc99fcb40c5a2 | `as_str` | function | Returns the stable string representation of `PolledAudioPollErrorCode`. | `src/session/error_code.rs:138` |
| sym-5643015174c990de4aee | `as_str` | function | Returns the stable string representation of `SessionStopCode`. | `src/session/error_code.rs:157` |
| sym-e3f1223778a01130e663 | `as_str` | function | Returns the stable string representation of `SessionStopFailureCode`. | `src/session/error_code.rs:182` |
| sym-d23b80c999773411aaa9 | `as_str` | function | Returns the stable string representation of `SourceTypeId`. | `src/session/extensions/source.rs:52` |
| sym-e18051e89c18feace5e6 | `async_factory` | function | Returns the async factory associated with `NodeRegistry`. | `src/graph/registry.rs:144` |
| sym-fe6e57f9a3d6642b2913 | `async_factory_by_operator` | function | Returns the async factory by operator associated with `NodeRegistry`. | `src/graph/registry.rs:151` |
| sym-575f25f9e00122ebe881 | `async_node_type_id` | function | Returns the async node type identifier held by `NodeRegistry`. | `src/graph/registry.rs:160` |
| sym-eda77a12b588bd3de5b1 | `audio` | function | Convenience constructor for PCM audio ports. | `src/graph/signal/spec.rs:269` |
| sym-eac6fe4c4ec72aa2b314 | `audio_frames_enqueued_total` | function | Returns the audio frames enqueued total held by `SessionAudioReentryMetrics`. | `src/session/lifecycle/observations.rs:368` |
| sym-dfcf0770716d94d558a9 | `audio_input` | function | Opens a bounded input for audio already owned by the embedding application. | `src/lib.rs:451` |
| sym-1b3c87ce3f8727641e26 | `audio_observations` | function | Returns the audio observations held by `RunningSession`. | `src/lib.rs:821` |
| sym-0dd31873b1f2e2b4daf0 | `audio_reentry_metrics` | function | Returns exact queue, pool, loss, and lifecycle accounting for every Session-owned typed-PCM reentry into the specialized audio lane. | `src/lib.rs:889` |
| sym-b06ab36e3211268290ef | `audio_reentry_metrics` | function | Returns the audio reentry metrics held by `RunningSession`. | `src/session/lifecycle/running.rs:223` |
| sym-3545be6a5b39ea259c4d | `audio_stem_id` | function | Returns the audio stem identifier held by `EndpointRouteContext`. | `src/endpoint/runtime.rs:88` |
| sym-5c520eb1b2678b3a8e30 | `available_slots` | function | Returns the available slots associated with `AudioBufferPool`. | `src/frame/pool.rs:71` |
| sym-556d991164d8862d47a0 | `backpressure` | function | Returns the backpressure associated with `EdgeContract`. | `src/graph/ports.rs:341` |
| sym-4d439fa19abd2ab66f61 | `before_open` | function | Records a source that was resolved but not opened because another required source failed first. | `src/capture/authorization.rs:60` |
| sym-6451c64a209cd4c0b3e4 | `binary` | function | Convenience constructor for opaque or schema-backed binary ports. | `src/graph/signal/spec.rs:299` |
| sym-8f2182e0218cb6561476 | `bitrate_kbps` | function | Returns the bitrate kbps associated with `StreamProfile`. | `src/codec/profile.rs:51` |
| sym-22e3046e14d9c5d31b22 | `bounded_async` | function | Generic bounded asynchronous edge. Connected ports supply the payload representation and the envelope preserves its producer clock. | `src/graph/ports.rs:413` |
| sym-f4f70bce71bd42b44339 | `branch_copy_pool_bytes` | function | Returns the branch copy pool bytes held by `EdgeBufferPlan`. | `src/graph/plan.rs:57` |
| sym-765df13202cdcd582c2a | `branch_copy_pool_capacity_frames` | function | Returns the branch copy pool capacity frames held by `EdgeBufferPlan`. | `src/graph/plan.rs:48` |
| sym-ab18d860480ef532896a | `browser` | function | Declares a browser/remote receiver. Register its transport implementation with [`Self::register_browser_driver`]. | `src/lib.rs:538` |
| sym-9527dd3130d771c3266b | `browser` | function | Returns the browser associated with `Session`. | `src/session/declaration/draft.rs:431` |
| sym-71b324aed7337f39d77d | `build` | function | Builds the Session declaration owner. | `src/lib.rs:340` |
| sym-596d144d9deba0aa781a | `build` | function | Consumes all setup state so no partially populated registry can escape. | `src/session/lifecycle/engine.rs:176` |
| sym-57541748485f96465396 | `build` | function | Builds its owned operation for `SessionEngineHostBuilder`. | `src/session/lifecycle/host.rs:344` |
| sym-0fe897f78065c16fd857 | `builder` | function | Creates a builder for declaring `Session` sources, routes, and endpoints. | `src/lib.rs:378` |
| sym-f3af0a674083c042387f | `bundle_id` | function | Returns the bundle identifier held by `ApplicationSelector`. | `src/session/declaration/selector.rs:44` |
| sym-81b4ef31adec7f3076da | `cancel` | function | Cancels active asynchronous Operators, then finalizes capture, runtime, endpoints, and recording through the same bounded Session authority. | `src/lib.rs:918` |
| sym-218585d5338b32dc0e4d | `cancel` | function | Requests cancellation of `SourceRuntime`. | `src/session/extensions/source.rs:575` |
| sym-cfafc3f08dfc105ee2ca | `cancel` | function | Requests cancellation of `RunningSession`. | `src/session/lifecycle/running.rs:423` |
| sym-f638ac422d0b9145e66e | `cancel_and_join` | function | Cancels and join for `GeneratedAudioBridge`. | `src/runtime/bridge/audio.rs:184` |
| sym-009871c0c84118fce16e | `cancel_and_join` | function | Cancels and join for `AsyncOperatorWorker`. | `src/runtime/signal/operator.rs:925` |
| sym-dd1373cfb5ce025352ae | `cancel_and_reap` | function | Cancels and reap for `SidecarHost`. | `src/runtime/lifecycle/sidecar_host.rs:322` |
| sym-2cd53fcb9d9466bb699e | `cancellation` | function | Returns the cancellation associated with `AsyncOperatorManifest`. | `src/graph/signal/operator.rs:212` |
| sym-bf0f618c9ae56c316380 | `cancellation_requested` | function | Returns whether cancellation requested is true for `PreparedSession`. | `src/session/prepare/prepared.rs:73` |
| sym-c00eb2e5f0f365125ff1 | `cancellation_total` | function | Returns the cancellation total held by `SessionAudioReentryMetrics`. | `src/session/lifecycle/observations.rs:372` |
| sym-337292e97a842fdf2668 | `canonical_path` | function | Returns the canonical path associated with `NativeExtensionLibrary`. | `src/native_extension/mod.rs:68` |
| sym-4a32dce08606102cb841 | `capabilities` | function | Returns the capabilities held by `ConnectorManifest`. | `src/connector/manifest.rs:152` |
| sym-2ae2a0a1e4991fd03efd | `capacity_frames` | function | Returns the capacity frames held by `CapturedFrameStream`. | `src/capture/frame_stream.rs:165` |
| sym-651d787afb9425aca9fc | `capacity_frames` | function | Returns the capacity frames held by `AudioInputConfig`. | `src/session/extensions/audio_input/mod.rs:63` |
| sym-6989e7d604357fb2446c | `capacity_signals` | function | Returns the capacity signals associated with `PortPrepareContext`. | `src/graph/node.rs:361` |
| sym-45bcf7fa11f5094b71bc | `capture` | function | Declares a capture source on `Session` and returns its Session-scoped handle. | `src/lib.rs:386` |
| sym-468dcc7528c63336128c | `capture` | function | Declares a capture source on `Session` and returns its Session-scoped handle. | `src/session/declaration/draft.rs:335` |
| sym-151154363dcfde713ace | `capture_backends` | function | Uses caller-owned capture backends while retaining the Session Session compiler, runtime, endpoint lifecycle, and recording ownership. | `src/lib.rs:312` |
| sym-c4062e05a64ec2cd4952 | `capture_finalization_failures_total` | function | Returns the capture finalization failures total held by `SessionStopOutcome`. | `src/session/lifecycle/control.rs:359` |
| sym-9828266fb021eb2aa01c | `capture_mode` | function | Returns the capture mode held by `SystemLoopbackSource`. | `src/capture/platform/macos/loopback.rs:57` |
| sym-d6d0625df36b60fc05ad | `capture_mode` | function | Returns the capture mode held by `DesktopCaptureSource`. | `src/capture/platform/macos/mod.rs:44` |
| sym-9e49b4a8ad90ee1c7d4e | `channel_count` | function | Returns the channel count held by `ChannelLayout`. | `src/graph/ports.rs:34` |
| sym-c488d4533e942b4a7698 | `channels` | function | Returns the channel count represented by `StreamProfile`. | `src/codec/profile.rs:21` |
| sym-16a3ef48594b21bc3b2b | `channels` | function | Returns the channel count represented by `EndpointAudioFrame`. | `src/endpoint/contract.rs:61` |
| sym-4cbe7948fb0b2aa1dbb8 | `channels` | function | Returns the channel count represented by `PolledAudioFrame`. | `src/endpoint/polled_audio_driver.rs:301` |
| sym-00c85c79fcb93baa197f | `channels` | function | Returns the channel count represented by `AudioFrame`. | `src/frame/audio.rs:130` |
| sym-9b9383895d480d72636f | `channels` | function | Returns the channel count represented by `SharedAudioFrame`. | `src/frame/audio.rs:200` |
| sym-ebd8e9d86b76a24a1263 | `channels` | function | Returns the channel count represented by `PlanEdgeFrame`. | `src/runtime/audio/router.rs:70` |
| sym-1a857f3aa63ddf26c1b2 | `class` | function | Returns the class associated with `SignalSpec`. | `src/graph/signal/spec.rs:215` |
| sym-ba5d7f524eb5ffcfd856 | `clipped_samples` | function | Returns the clipped samples held by `MixerTelemetry`. | `src/runtime/nodes.rs:260` |
| sym-053c78f7e0d2836eaed8 | `clock` | function | Returns the clock associated with `EdgeContract`. | `src/graph/ports.rs:329` |
| sym-f756cb02786f8b9e4a22 | `clock_id` | function | Returns the clock identifier held by `FrameLineage`. | `src/frame/lineage.rs:65` |
| sym-44a7d16dea5bbbda4d6d | `clock_id` | function | Returns the clock identifier held by `SignalLineage`. | `src/graph/signal/lineage.rs:68` |
| sym-3aeff457f579705093f6 | `close` | function | Closes `AudioInputWriter` to further work. | `src/session/extensions/audio_input/buffer.rs:242` |
| sym-217be49e77da4aabf632 | `close` | function | Closes this application-owned input after its accepted frames drain. | `src/session/extensions/audio_input/mod.rs:127` |
| sym-6b2818ee70b8000ad99d | `close_and_reap` | function | Closes `SidecarHost` and reaps its child process. | `src/runtime/lifecycle/sidecar_host.rs:326` |
| sym-ac60f8c24954ba5af0f0 | `code` | function | Returns the stable error or status code represented by `ConnectorConfigurationError`. | `src/connector/configuration.rs:615` |
| sym-1b9e9ce53fdd2cf3d6a2 | `code` | function | Returns the stable error or status code represented by `ConnectorError`. | `src/connector/error.rs:109` |
| sym-f5cae830f98ad619334b | `code` | function | Returns the stable error or status code represented by `EndpointFailure`. | `src/endpoint/runtime.rs:212` |
| sym-0da94b4ea7aec7d555fd | `code` | function | Returns the stable error or status code represented by `SessionRuntimeError`. | `src/error_code.rs:5` |
| sym-13037040612a8aedb3cb | `code` | function | Returns the stable error or status code represented by `SessionStopResult`. | `src/error_code.rs:15` |
| sym-0b07fd05d3d72c65fa2d | `code` | function | Returns the stable error or status code represented by `SessionStartError`. | `src/lib.rs:974` |
| sym-e74cb3e63e1024a31011 | `code` | function | Returns the stable error or status code represented by `NativeExtensionLibraryError`. | `src/native_extension/mod.rs:131` |
| sym-9ab28f94b738bac6ae31 | `code` | function | Returns the stable error or status code represented by `RecorderError`. | `src/recording/error_code.rs:59` |
| sym-7f946d73a01b4db9ecc6 | `code` | function | Returns the stable error or status code represented by `SessionCompileDiagnostic`. | `src/session/compile/error.rs:113` |
| sym-f772dacab759a8e9e66b | `compile` | function | Compiles its owned operation for `Compiler`. | `src/graph/compile/resolve.rs:464` |
| sym-68093a3f20079dee1c25 | `compile` | function | Compiles its owned operation for `SessionCompiler`. | `src/session/compile/mod.rs:103` |
| sym-46d9ba85d0b5a90bb8da | `compile` | function | Compiles its owned operation for `SessionEngine`. | `src/session/lifecycle/engine.rs:221` |
| sym-ee91490be1c23a752cf2 | `compile` | function | Compiles its owned operation for `SessionEngineHost`. | `src/session/lifecycle/host.rs:55` |
| sym-c1db03665d9c98f846e7 | `compile_diagnostic` | function | Returns structured compiler facts when startup failed while compiling the declared Session. | `src/lib.rs:987` |
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
| sym-ec1e34e47737b444a4ae | `connect` | function | Connects the requested ports through `Pipeline`. | `src/graph/dsl.rs:55` |
| sym-1c4c7cb9bedad265f145 | `connect` | function | Connects the requested ports through `StemHandle`. | `src/session/declaration/draft.rs:807` |
| sym-8f3431c4252ede73ca6c | `connect` | function | Connects the requested ports through `SourceOutputHandle`. | `src/session/declaration/draft.rs:943` |
| sym-2f51546b7bed5c1ee4c6 | `connect` | function | Connects the requested ports through `DerivedStreamHandle`. | `src/session/declaration/draft.rs:1039` |
| sym-806b172733c210276185 | `connect_with` | function | Connects pipeline ports using the supplied edge contract on `Pipeline`. | `src/graph/dsl.rs:59` |
| sym-286cb5670f377ab8ef14 | `connections` | function | Returns the connections associated with `SessionSpec`. | `src/session/declaration/spec.rs:347` |
| sym-5f4453784884813e12c2 | `connector` | function | Declares an external connector. Register its implementation after route identities are available with [`Self::register_connector_driver`]. | `src/lib.rs:511` |
| sym-e8963ce61c3155e55b92 | `connector` | function | Declares a connector endpoint on `Session` with the supplied operator identity and configuration. | `src/session/declaration/draft.rs:399` |
| sym-5da32ec8a46f62e624fc | `connector_id` | function | Returns the connector identifier held by `ConnectorInputDescriptor`. | `src/connector/worker/driver.rs:32` |
| sym-d97706ec8bcbc92af245 | `connector_id` | function | Returns the connector identifier held by `PolledAudioFrame`. | `src/endpoint/polled_audio_driver.rs:293` |
| sym-a7836b3612121275e6ce | `connector_id` | function | Returns the connector identifier held by `EndpointPrepareContext`. | `src/endpoint/runtime.rs:138` |
| sym-6929300e9be787b99234 | `connector_id` | function | Returns the connector identifier held by `SignalDerivation`. | `src/graph/signal/lineage.rs:153` |
| sym-7032830c11b003b8c721 | `connector_id` | function | Returns the connector identifier held by `EndpointHandle`. | `src/session/declaration/draft.rs:595` |
| sym-73f349fcfa5dcd416141 | `connector_id` | function | Returns the connector identifier held by `EndpointSpec`. | `src/session/declaration/spec.rs:175` |
| sym-98c5d4b70984a08dd397 | `constraints` | function | Returns the constraints held by `ConnectorConfigurationField`. | `src/connector/configuration.rs:222` |
| sym-d2ff6deafe0c2e7e26c8 | `contains` | function | Returns whether contains is true for `NodeRegistry`. | `src/graph/registry.rs:164` |
| sym-3a9c0d6cde74bac8980a | `context` | function | Returns the context held by `EndpointPortInput`. | `src/endpoint/contract.rs:249` |
| sym-580a1dc419b01bf7c5af | `control` | function | Convenience constructor for control ports. | `src/graph/signal/spec.rs:294` |
| sym-494f99c0b5d652874f70 | `copy_policy` | function | Returns the copy policy held by `EdgeContract`. | `src/graph/ports.rs:353` |
| sym-448b7fda12b3625ab426 | `copy_to_pool` | function | Copies the shared frame into storage acquired from the supplied pool for `SharedAudioFrame`. | `src/frame/audio.rs:233` |
| sym-dce49826a02f0e21e6a4 | `copy_to_pool` | function | Copies the shared frame into storage acquired from the supplied pool for `SharedLineagedAudioFrame`. | `src/frame/audio.rs:319` |
| sym-25b9f1dfb7e287566018 | `count` | function | Returns the count associated with `OpusChannels`. | `src/codec/encoder.rs:33` |
| sym-84dfe9847cb3775c63d5 | `current` | function | Returns the current value observed by `CapturePermissionLifecycle`. | `src/capture/authorization.rs:196` |
| sym-93251a13ab5d4a5e5072 | `custom` | function | Convenience constructor for custom / vendor extension ports. | `src/graph/signal/spec.rs:304` |
| sym-d702ca116c48c799c774 | `deadline` | function | Returns the deadline associated with `AsyncOperatorManifest`. | `src/graph/signal/operator.rs:208` |
| sym-635f185607de8e4f9da1 | `declare` | function | Adds the declaration represented by `RegisteredConnector` to its Session. | `src/connector/mod.rs:159` |
| sym-06cbcdb626b9a2e220a9 | `declares_multistem_recording` | function | Returns whether `Session` declares multistem recording. | `src/session/extensions/recording.rs:102` |
| sym-e3ef3f2426b2ab2e710c | `decode` | function | Decodes input through `ConnectorConfigurationRecord`. | `src/connector/transport.rs:93` |
| sym-1a5304483b6adaa06f83 | `decode` | function | Decodes input through `ConnectorAudioRecord`. | `src/connector/transport.rs:403` |
| sym-7fa68b299cc6aee2a4f2 | `decode` | function | Decodes input through `SidecarMessage`. | `src/runtime/lifecycle/sidecar_protocol.rs:121` |
| sym-10aacc26c0e2fa7a7394 | `decode_into` | function | Decode a compressed Opus packet into i16 samples, then convert to f32. | `src/codec/decoder.rs:81` |
| sym-5841419d47fad0e02f75 | `decode_plc_into` | function | Conceal one missing packet while preserving libopus decoder state. | `src/codec/decoder.rs:116` |
| sym-b5f91227cb949970ac0c | `default` | function | Returns the default `OpusDecoder` value. | `src/codec/decoder.rs:175` |
| sym-9e807c1498052f3fa447 | `default` | function | Returns the default `OpusConfig` value. | `src/codec/encoder.rs:92` |
| sym-66537e7c28d4505fc8eb | `default` | function | Returns the default `OpusEncoder` value. | `src/codec/encoder.rs:303` |
| sym-567daa26a7dd7f60fe2f | `default` | function | Returns the default `PolledAudioEndpointConfig` value. | `src/endpoint/polled_audio_driver.rs:30` |
| sym-de350093988e7d14c0ac | `default` | function | Returns the default `RuntimePlanner` value. | `src/graph/compile/plan.rs:349` |
| sym-b1902764a29198ef1867 | `default` | function | Returns the default `Compiler` value. | `src/graph/compile/resolve.rs:513` |
| sym-99dc997286cb4856fac8 | `default` | function | Returns the default `SessionBuilder` value. | `src/lib.rs:283` |
| sym-24fb78dea2b62553e984 | `default` | function | Returns the default `Session` value. | `src/lib.rs:784` |
| sym-8b2c4ad5fa0a4ee19c40 | `default` | function | Returns the default `PlanRunnerCancellation` value. | `src/runtime/audio/runner.rs:110` |
| sym-73f57e4942d4df802c11 | `default` | function | Returns the default `SidecarDeadlines` value. | `src/runtime/lifecycle/sidecar_host.rs:61` |
| sym-f2a677038880c785311e | `default` | function | Returns the default `SidecarProtocolLimits` value. | `src/runtime/lifecycle/sidecar_protocol.rs:51` |
| sym-1c5179c9d400732c9b0c | `default` | function | Returns the default `Session` value. | `src/session/declaration/draft.rs:565` |
| sym-0fdf7c5a3b73738000b1 | `default` | function | Returns the default `DeviceSelector` value. | `src/session/declaration/selector.rs:113` |
| sym-e1902db1f798ce94cdea | `default` | function | Returns the default `SessionStartOptions` value. | `src/session/lifecycle/control.rs:33` |
| sym-8684f51f8ee7430fcd49 | `default` | function | Returns the default `NativeSessionEngineHostOptions` value. | `src/session/lifecycle/host.rs:172` |
| sym-db7c11406c22fe9249af | `default` | function | Returns the default `ClockCorrectionController` value. | `src/timing/clock_correction.rs:52` |
| sym-afa922b78c896e4ffb57 | `default` | function | Returns the default `ClockDriftEstimator` value. | `src/timing/clock_drift.rs:115` |
| sym-dcb5e0588fab365a19c7 | `definition` | function | Returns the definition associated with `NodeRegistry`. | `src/graph/registry.rs:134` |
| sym-e916d03fa72a650b78b3 | `delivery` | function | Returns the delivery associated with `EdgeContract`. | `src/graph/ports.rs:345` |
| sym-6c2f846a56c09b494d97 | `delivery_readiness` | function | Returns the delivery readiness associated with `ConnectorServiceStatus`. | `src/connector/status.rs:42` |
| sym-ad4e94586f88eebd3375 | `deprecated` | function | Returns the deprecated held by `ConnectorConfigurationField`. | `src/connector/configuration.rs:201` |
| sym-6b003ff3808eecc732f0 | `deprecation` | function | Returns the deprecation held by `ConnectorConfigurationField`. | `src/connector/configuration.rs:226` |
| sym-28451209252058476fb8 | `derivation` | function | Returns the derivation associated with `SignalEnvelope`. | `src/graph/signal/envelope.rs:82` |
| sym-7d47561aba01141608ff | `derived_route` | function | Returns the derived route associated with `SessionMetricsSnapshot`. | `src/session/lifecycle/observations.rs:111` |
| sym-fb88e790fd387a7ae359 | `derived_route_count` | function | Returns the derived route count held by `SessionMetricsSnapshot`. | `src/session/lifecycle/observations.rs:107` |
| sym-4f304b08fba83ba0abe8 | `derived_route_metrics` | function | Returns one observation handle per derived operator-output route. | `src/lib.rs:883` |
| sym-143bf8e7f69e1c868835 | `derived_route_metrics` | function | Returns the derived route metrics held by `RunningSession`. | `src/session/lifecycle/running.rs:219` |
| sym-4d22610034cf756ca6ea | `descriptor` | function | Returns the descriptor associated with `PassthroughFactory`. | `src/graph/builtins.rs:70` |
| sym-45c32ecad1ac42cbd26d | `descriptor` | function | Returns the descriptor associated with `GainFactory`. | `src/graph/builtins.rs:110` |
| sym-06610d9f4ca3e6efac10 | `descriptor` | function | Returns the descriptor associated with `MonoMixFactory`. | `src/graph/builtins.rs:169` |
| sym-d8a211489124a08425d2 | `descriptor` | function | Returns the descriptor associated with `NodeDefinitionRef`. | `src/graph/registry.rs:39` |
| sym-848742f534a9b91337d0 | `descriptor` | function | Returns the descriptor associated with `SystemOutputSourceFactory`. | `src/runtime/nodes.rs:87` |
| sym-b933cf7614529f0028d2 | `descriptor` | function | Returns the descriptor associated with `BridgeSinkFactory`. | `src/runtime/nodes.rs:191` |
| sym-05eefad45f0dfa93d416 | `diagnostic` | function | Converts a Session compiler failure into stable language-neutral location and comparison fields. | `src/session/compile/error.rs:166` |
| sym-13032dd31eb4cc492fdb | `direction` | function | Returns the direction associated with `PortPrepareContext`. | `src/graph/node.rs:345` |
| sym-893b8f9f841f5d379b68 | `direction` | function | Returns the direction associated with `PortSpec`. | `src/graph/ports.rs:217` |
| sym-c56c74ef48d3d65a7896 | `direction` | function | Returns the direction associated with `SessionCompileDiagnostic`. | `src/session/compile/error.rs:145` |
| sym-0e6ca5ef323b36dc1ffb | `discontinuity_epoch` | function | Returns the discontinuity epoch associated with `FrameLineage`. | `src/frame/lineage.rs:80` |
| sym-9a0ea2064c79ae612aca | `discontinuity_epoch` | function | Returns the discontinuity epoch associated with `SignalLineage`. | `src/graph/signal/lineage.rs:77` |
| sym-6dbe1ce0ed2ff967ed67 | `discover` | function | Discovers the resources visible to `LocalSourceProvider`. | `src/capture/query.rs:55` |
| sym-eb1aeb591e12bc6b9e4e | `dispatch_from` | function | Routes one lineaged audio frame from the named plan output through `PlanEdgeRouter`. | `src/runtime/audio/router.rs:777` |
| sym-6231de0f2ff63f025a56 | `display_name` | function | Returns the display name held by `NodeDescriptor`. | `src/graph/node.rs:226` |
| sym-c64bde627ce081bddc75 | `disposition` | function | Returns the disposition associated with `SessionCancelResult`. | `src/lib.rs:1129` |
| sym-a89d8d95a6a9491a2ca8 | `disposition` | function | Returns the disposition associated with `SessionStopResult`. | `src/lib.rs:1149` |
| sym-3248ef146d7f14d09bc6 | `documentation` | function | Returns the documentation held by `ConnectorConfigurationField`. | `src/connector/configuration.rs:218` |
| sym-91cdc75380f1b1c45072 | `documentation` | function | Returns the documentation held by `ConnectorCapability`. | `src/connector/manifest.rs:34` |
| sym-34ddc898cf906f7e707f | `documentation` | function | Returns the documentation held by `ConnectorRequirement`. | `src/connector/manifest.rs:69` |
| sym-e2f1dc5f386c4947fe93 | `drift_ppm` | function | Returns the drift ppm associated with `ClockDriftEstimator`. | `src/timing/clock_drift.rs:59` |
| sym-b839c4c5e4b32bc03a85 | `drop` | function | Releases resources owned by `MacosInputSource`. | `src/capture/platform/macos/input.rs:258` |
| sym-3f4c295f1659985a9a22 | `drop` | function | Releases resources owned by `SystemLoopbackSource`. | `src/capture/platform/macos/loopback.rs:299` |
| sym-24451f44bd94778b8d32 | `drop` | function | Releases resources owned by `ConnectorSecret`. | `src/connector/configuration.rs:49` |
| sym-801da87612861d0e8d9a | `drop` | function | Releases resources owned by `PolledAudioBatchLease`. | `src/endpoint/polled_audio_driver.rs:241` |
| sym-7c022772d35f9183ccfb | `drop` | function | Releases resources owned by `AudioBufferHandle`. | `src/frame/pool.rs:265` |
| sym-4d50a59cc7313401f655 | `drop` | function | Releases resources owned by `SharedAudioBufferHandle`. | `src/frame/pool.rs:322` |
| sym-bf00675d3ce79f830ecb | `drop` | function | Releases resources owned by `MultistemRecording`. | `src/recording/writer.rs:361` |
| sym-1d8fe4e68f55934db10b | `drop` | function | Releases resources owned by `PlanEdgeReceiver`. | `src/runtime/audio/router.rs:676` |
| sym-2958e60b6265b5fef03f | `drop` | function | Releases resources owned by `GeneratedAudioBridge`. | `src/runtime/bridge/audio.rs:193` |
| sym-bb231d51d6e86fd54874 | `drop` | function | Releases resources owned by `AsyncRuntimeHost`. | `src/runtime/lifecycle/async_host.rs:105` |
| sym-f3a9d500f29837962ac8 | `drop` | function | Releases resources owned by `SidecarHost`. | `src/runtime/lifecycle/sidecar_host.rs:521` |
| sym-fdff2f61cb56c8996295 | `drop` | function | Releases resources owned by `PreparedSourceRuntime`. | `src/session/extensions/source.rs:556` |
| sym-e0b320e2052a0c631823 | `drop` | function | Releases resources owned by `SourceRuntime`. | `src/session/extensions/source.rs:594` |
| sym-50cad3198e714c4154f7 | `drop` | function | Releases resources owned by `RunningSession`. | `src/session/lifecycle/running.rs:619` |
| sym-3a61bb04d6a986cbcb51 | `drop` | function | Releases resources owned by `SessionTraceRecorder`. | `src/session/lifecycle/trace.rs:249` |
| sym-3a193e845f256dd2f356 | `drop_observations` | function | Returns the drop observations held by `SessionRouteMetrics`. | `src/session/lifecycle/observations.rs:206` |
| sym-17ac8a99af0ede70717f | `drop_rate_pct` | function | Returns the drop rate pct held by `EdgeObservations`. | `src/runtime/audio/router.rs:185` |
| sym-88c3e8dfe1cf74414c54 | `drop_rate_pct` | function | Returns the drop rate pct held by `SessionRouteDropObservations`. | `src/session/lifecycle/observations.rs:171` |
| sym-073479ab2b9fc73b1ab0 | `duration_ns` | function | Returns the duration nanoseconds held by `FrameLineage`. | `src/frame/lineage.rs:74` |
| sym-db7739acf4a049ac817e | `duration_ns` | function | Returns the duration nanoseconds held by `SignalTiming`. | `src/graph/signal/timing.rs:83` |
| sym-b25bfeb7fc80ca6ae6ee | `edge_buffer` | function | Returns the edge buffer associated with `MemoryPlan`. | `src/graph/plan.rs:71` |
| sym-e443085133525c866043 | `edge_contract` | function | Returns the edge contract held by `ConnectorInputDescriptor`. | `src/connector/worker/driver.rs:52` |
| sym-4d7a7ced948b546687bf | `edge_contract` | function | Returns the edge contract held by `EndpointPortInput`. | `src/endpoint/contract.rs:245` |
| sym-c8da66f370729379cea7 | `edge_contract` | function | Returns the edge contract held by `PortPrepareContext`. | `src/graph/node.rs:357` |
| sym-74e190b45549845b8de4 | `edge_count` | function | Returns the edge count held by `GraphIr`. | `src/graph/ir.rs:43` |
| sym-a9bd0817b7a3e0d0a511 | `edge_count` | function | Returns the edge count held by `GraphSpec`. | `src/graph/spec.rs:69` |
| sym-38977d2ca402b0e4145b | `edge_count` | function | Returns the edge count held by `CompiledSession`. | `src/session/compile/compiled.rs:52` |
| sym-ce4d5aa9e601fb810c11 | `edge_id` | function | Returns the edge identifier held by `PortPrepareContext`. | `src/graph/node.rs:337` |
| sym-c6c0dece77c2acf50c8f | `edge_id` | function | Returns the edge identifier held by `PlanEdgeReceiver`. | `src/runtime/audio/router.rs:528` |
| sym-8ac4397e918d84da100f | `edge_index` | function | Returns the edge index held by `SessionCompileDiagnostic`. | `src/session/compile/error.rs:121` |
| sym-c46a34cb2388608cd3b1 | `encode` | function | Encodes input through `ConnectorConfigurationRecord`. | `src/connector/transport.rs:61` |
| sym-08a8a31e4c30a3df7c8e | `encode` | function | Encodes input through `ConnectorAudioRecord`. | `src/connector/transport.rs:347` |
| sym-39e328da4ebd9d04a6a0 | `encode` | function | Encodes input through `SidecarMessage`. | `src/runtime/lifecycle/sidecar_protocol.rs:86` |
| sym-dc78315d25ca0081c076 | `encode_into` | function | Encode an interleaved PCM slice into `out`. | `src/codec/encoder.rs:235` |
| sym-7a8bbbedb1dada1a1e63 | `encoded_audio` | function | Convenience constructor for encoded audio ports. | `src/graph/signal/spec.rs:274` |
| sym-93dea0a16a6d08dcd677 | `endpoint` | function | Declares an endpoint on `Session` and returns its Session-scoped handle. | `src/lib.rs:486` |
| sym-6abe660491a0c79f396d | `endpoint` | function | Declares an endpoint on `Session` and returns its Session-scoped handle. | `src/session/declaration/draft.rs:394` |
| sym-c6dbb7792fc664b765f7 | `endpoint_declarations` | function | Returns the endpoint declarations associated with `CompiledSession`. | `src/session/compile/compiled.rs:42` |
| sym-13d0c33d9b533c4ca958 | `endpoint_enqueued_at_ns` | function | Returns the endpoint enqueued at nanoseconds held by `PolledAudioFrame`. | `src/endpoint/polled_audio_driver.rs:281` |
| sym-516dc966b917a4f240ef | `endpoint_failures` | function | Returns the endpoint failures associated with `SessionTerminalOutcome`. | `src/session/lifecycle/events.rs:279` |
| sym-23c14c23a493121cf53b | `endpoint_finalization_failures_total` | function | Returns the endpoint finalization failures total held by `SessionStopOutcome`. | `src/session/lifecycle/control.rs:363` |
| sym-0c5da44a14691ca38d74 | `endpoint_id` | function | Returns the endpoint identifier held by `ConnectorInputDescriptor`. | `src/connector/worker/driver.rs:28` |
| sym-c6b6e5f1e34176dcab49 | `endpoint_id` | function | Returns the endpoint identifier held by `PolledAudioFrame`. | `src/endpoint/polled_audio_driver.rs:265` |
| sym-3429083d9425fded8a3f | `endpoint_id` | function | Returns the endpoint identifier held by `EndpointPrepareContext`. | `src/endpoint/runtime.rs:129` |
| sym-039bdeff82f22e216144 | `endpoint_id` | function | Returns the endpoint identifier held by `SessionEndpointFailure`. | `src/session/lifecycle/events.rs:150` |
| sym-4bec51422253ac2e9473 | `endpoint_id` | function | Returns the endpoint identifier held by `PreparedWorkerMapping`. | `src/session/prepare/mappings.rs:253` |
| sym-575cacd62461aa3cdc9c | `endpoint_observations` | function | Returns the endpoint observations held by `ConnectorContext`. | `src/connector/worker/coordination.rs:139` |
| sym-68ff5ed2fe9768966073 | `endpoints` | function | Returns the endpoints associated with `SessionSpec`. | `src/session/declaration/spec.rs:339` |
| sym-b97f3ea15b1278c1a5cd | `engine` | function | Returns the engine associated with `SessionEngineHost`. | `src/session/lifecycle/host.rs:158` |
| sym-795bbe983a2449575b66 | `engine_builder` | function | Borrows the mutable engine builder owned by `SessionEngineHostBuilder`. | `src/session/lifecycle/host.rs:277` |
| sym-d0e7ecee43b4fb211527 | `error` | function | Returns the error associated with `SessionStartFailure`. | `src/session/lifecycle/control.rs:307` |
| sym-7864f46099de5598eb03 | `error_class` | function | Returns the error class associated with `SessionControlFailure`. | `src/session/lifecycle/events.rs:97` |
| sym-1ed93660c3142fc23078 | `event` | function | Convenience constructor for event ports. | `src/graph/signal/spec.rs:284` |
| sym-3871b4999157943369c3 | `event` | function | Returns the event associated with `SessionSourceFailure`. | `src/session/lifecycle/events.rs:118` |
| sym-ad22a96a7584673fba4d | `event_observations` | function | Returns the event observations held by `RunningSession`. | `src/lib.rs:835` |
| sym-e005811e6b121d4e888b | `event_queue` | function | Returns the event queue associated with `SessionMetricsSnapshot`. | `src/session/lifecycle/observations.rs:67` |
| sym-dbccbe698db48a061ebe | `execute` | function | Executes its owned operation for `AsyncRuntimeHost`. | `src/runtime/lifecycle/async_host.rs:65` |
| sym-b1658414197792b94c5b | `execute_from` | function | Executes one lineaged frame from the named source node through `RealtimePlanExecutor`. | `src/runtime/audio/executor.rs:149` |
| sym-109839bcc0f547d406a9 | `execution` | function | Returns the execution held by `NodeDescriptor`. | `src/graph/node.rs:238` |
| sym-992aa61dc221d91aba49 | `execution` | function | Returns the execution held by `SourceManifest`. | `src/session/extensions/source.rs:174` |
| sym-8d74fcdee897cc6b2612 | `execution_partition` | function | Returns the execution partition associated with `AsyncOperatorPrepareContext`. | `src/graph/signal/preparation.rs:58` |
| sym-835c3fe6f42cb5df0be0 | `expected` | function | Returns the expected value when a compilation diagnostic compares two values. | `src/session/compile/error.rs:149` |
| sym-b9d0e20afe7dce1858a4 | `expose_secret` | function | Exposes the secret to the owning connector during setup or worker use. | `src/connector/configuration.rs:37` |
| sym-b813a339705fd17d01fe | `external_source` | function | Returns the external source held by `SessionMetricsSnapshot`. | `src/session/lifecycle/observations.rs:87` |
| sym-a14092e356d511085706 | `external_source_count` | function | Returns the external source count held by `SessionMetricsSnapshot`. | `src/session/lifecycle/observations.rs:83` |
| sym-1a79107a12b86eca14ee | `external_source_declarations` | function | Returns the external source declarations associated with `CompiledSession`. | `src/session/compile/compiled.rs:37` |
| sym-fb55b96b6a4e16b28a7a | `external_source_metrics` | function | Returns one observation handle per Session-owned external source. | `src/lib.rs:852` |
| sym-d79af986ffbd30650d2d | `external_source_metrics` | function | Returns the external source metrics held by `RunningSession`. | `src/session/lifecycle/running.rs:215` |
| sym-d55e9dc582b914e1a93b | `failure` | function | Returns the failure held by `ConnectorRunOutcome`. | `src/connector/worker/mod.rs:55` |
| sym-ce1d5048575f2b7df65a | `failure` | function | Returns the failure held by `AsyncOperatorManifest`. | `src/graph/signal/operator.rs:216` |
| sym-30c45a25347393930ba2 | `failure` | function | Returns the failure held by `SessionEndpointFailure`. | `src/session/lifecycle/events.rs:158` |
| sym-5d9e7c56c63f18de4dda | `failure` | function | Returns the failure held by `SessionRollbackFailure`. | `src/session/lifecycle/events.rs:179` |
| sym-73a681fc858156508e9e | `failure` | function | Returns the failure held by `SessionFinalizationFailure`. | `src/session/lifecycle/events.rs:203` |
| sym-eecb915937d8d262c12c | `failure_codes` | function | Returns the failure codes associated with `SessionStopResult`. | `src/error_code.rs:26` |
| sym-56df668f46edc7d9e40d | `failure_threshold` | function | Returns the failure threshold held by `ConnectorReadinessPolicy`. | `src/connector/readiness.rs:55` |
| sym-267259f742b6a1cc1924 | `field` | function | Returns the field held by `ConnectorConfigurationSchema`. | `src/connector/configuration.rs:255` |
| sym-8d0d1e80c210dfa25f85 | `field` | function | Returns the field held by `ConnectorConfigurationError`. | `src/connector/configuration.rs:619` |
| sym-61bd3f48c87a4957932a | `fields` | function | Returns the fields held by `ConnectorConfigurationSchema`. | `src/connector/configuration.rs:251` |
| sym-8cf6b272ddf136ea84bd | `finalization_failures` | function | Returns the finalization failures associated with `SessionTerminalOutcome`. | `src/session/lifecycle/events.rs:287` |
| sym-dd38ad71820965668aba | `finish` | function | Finishes work owned by `MultistemRecording`. | `src/recording/writer.rs:283` |
| sym-8e35142f99b058ba0f84 | `finish` | function | Finishes work owned by `RealtimePlanRunner`. | `src/runtime/audio/runner.rs:359` |
| sym-3c724fc69bd1f09f64f9 | `finish` | function | Finishes work owned by `SessionTraceRecorder`. | `src/session/lifecycle/trace.rs:201` |
| sym-9b1bcdfe87564ca73d82 | `finish_and_join` | function | Finishes input to `GeneratedAudioBridge`, joins its worker, and returns the terminal result. | `src/runtime/bridge/audio.rs:178` |
| sym-6f467d9e1933391df548 | `finish_and_join` | function | Finishes input to `AsyncOperatorWorker`, joins its worker, and returns the terminal result. | `src/runtime/signal/operator.rs:933` |
| sym-cf4630838130fe080faf | `fmt` | function | Formats `ConnectorSecret` with the requested formatter. | `src/connector/configuration.rs:43` |
| sym-269f701717d0abdc7ced | `fmt` | function | Formats `ConnectorErrorCode` with the requested formatter. | `src/connector/error.rs:35` |
| sym-0652c06beea3ce6befb9 | `fmt` | function | Formats `ConnectorErrorCode` with the requested formatter. | `src/connector/error.rs:44` |
| sym-b29386002c10cabfeedf | `fmt` | function | Formats `AudioBufferHandle` with the requested formatter. | `src/frame/pool.rs:273` |
| sym-5100d1c981bdddd8fe29 | `fmt` | function | Formats `SharedAudioBufferHandle` with the requested formatter. | `src/frame/pool.rs:328` |
| sym-394bf065ef645171da20 | `fmt` | function | Formats `NodeTypeId` with the requested formatter. | `src/graph/node.rs:31` |
| sym-7a52b52dec52c8241c9b | `fmt` | function | Formats `NodeConfig` with the requested formatter. | `src/graph/node.rs:116` |
| sym-ae4353462736665ac29b | `fmt` | function | Formats `PlanEdgeFrame` with the requested formatter. | `src/runtime/audio/router.rs:100` |
| sym-01727a673bf9a902bf95 | `fmt` | function | Formats `Session` with the requested formatter. | `src/session/declaration/draft.rs:571` |
| sym-eab626d44d36d59f91cd | `fmt` | function | Formats `OperatorInstanceHandle` with the requested formatter. | `src/session/declaration/draft.rs:758` |
| sym-9d1e9847c6dbbbb46448 | `fmt` | function | Formats `OperatorInputHandle` with the requested formatter. | `src/session/declaration/draft.rs:768` |
| sym-739491c95aeb9ac42b2e | `fmt` | function | Formats `SourceInstanceHandle` with the requested formatter. | `src/session/declaration/draft.rs:899` |
| sym-8f3e725ce80caafcd704 | `fmt` | function | Formats `SourceOutputHandle` with the requested formatter. | `src/session/declaration/draft.rs:979` |
| sym-3a209436fba7f0e9a025 | `fmt` | function | Formats `DerivedStreamHandle` with the requested formatter. | `src/session/declaration/draft.rs:1102` |
| sym-531181e9f721dfa0fed4 | `fmt` | function | Formats `StemHandle` with the requested formatter. | `src/session/declaration/draft.rs:1113` |
| sym-c7930932213c66549448 | `fmt` | function | Formats `EndpointConfiguration` with the requested formatter. | `src/session/declaration/endpoint.rs:85` |
| sym-819f5be085fcc43f908f | `fmt` | function | Formats `AudioInputBuffer` with the requested formatter. | `src/session/extensions/audio_input/buffer.rs:52` |
| sym-65b34a6a7dc21e66a32e | `fmt` | function | Formats `AudioInputWriter` with the requested formatter. | `src/session/extensions/audio_input/buffer.rs:261` |
| sym-a4e771b5443c526bb318 | `fmt` | function | Formats `AudioInputWriteError` with the requested formatter. | `src/session/extensions/audio_input/buffer.rs:336` |
| sym-11db8228446be4636238 | `fmt` | function | Formats `AudioInputWriteError` with the requested formatter. | `src/session/extensions/audio_input/buffer.rs:346` |
| sym-12f31e225ca22137a4b4 | `fmt` | function | Formats `AudioInput` with the requested formatter. | `src/session/extensions/audio_input/mod.rs:143` |
| sym-77e95a330ed1efa59ca7 | `fmt` | function | Formats `PcmSource` with the requested formatter. | `src/session/extensions/audio_input/source.rs:74` |
| sym-3d3673f48bd6d7176455 | `fmt` | function | Formats `SourceTypeId` with the requested formatter. | `src/session/extensions/source.rs:742` |
| sym-4c3026fd4b0bb717700e | `fmt` | function | Formats `SessionStartFailure` with the requested formatter. | `src/session/lifecycle/control.rs:325` |
| sym-36cac98d0b30ecc6fe30 | `format` | function | Returns the format associated with `AudioFrame`. | `src/frame/audio.rs:134` |
| sym-7c2c9f033567cc4cb1c7 | `format` | function | Returns the format associated with `SharedAudioFrame`. | `src/frame/audio.rs:204` |
| sym-2ef59b897969c3489269 | `frame` | function | Returns the frame held by `PolledAudioBatchLease`. | `src/endpoint/polled_audio_driver.rs:232` |
| sym-6449e35cd6d5ee588e33 | `frame` | function | Returns the frame held by `LineagedAudioFrame`. | `src/frame/audio.rs:277` |
| sym-e54682a6f63431a1a44b | `frame` | function | Returns the frame held by `SharedLineagedAudioFrame`. | `src/frame/audio.rs:304` |
| sym-c9374230e421a25e4065 | `frame_capacity_samples` | function | Returns the frame capacity samples held by `SessionAudioReentryMetrics`. | `src/session/lifecycle/observations.rs:340` |
| sym-b2a72b13670ff17c8d51 | `frame_duration` | function | Returns the frame duration associated with `StreamProfile`. | `src/codec/profile.rs:31` |
| sym-b49dc5d31d95b5b4cddc | `frame_ms` | function | Returns the frame milliseconds held by `StreamProfile`. | `src/codec/profile.rs:60` |
| sym-e1d1201c50927893a266 | `frame_samples_for_duration_ms` | function | Returns the frame samples for duration milliseconds held by `SampleSpec`. | `src/frame/audio.rs:33` |
| sym-52af0d0b16a2213b8647 | `frame_samples_per_channel` | function | Returns the frame samples per channel associated with `AudioInputConfig`. | `src/session/extensions/audio_input/mod.rs:67` |
| sym-36cfcc7b327a2189a1ec | `frame_stream_closed` | function | Returns the frame stream closed associated with `CaptureOwner`. | `src/capture/capture_owner.rs:251` |
| sym-be67cc8f6802f777a3d7 | `frames_attempted_total` | function | Returns the frames attempted total held by `EdgeObservations`. | `src/runtime/audio/router.rs:180` |
| sym-d1c06126c9b16086f9b9 | `frames_captured` | function | Returns the frames captured associated with `SystemOutputTelemetry`. | `src/runtime/nodes.rs:55` |
| sym-5628415a8d0f45a0bc10 | `frames_emitted` | function | Returns the frames emitted associated with `SystemOutputTelemetry`. | `src/runtime/nodes.rs:58` |
| sym-f6cf84750da0d1dd18fe | `frames_mixed` | function | Returns the frames mixed associated with `MixerTelemetry`. | `src/runtime/nodes.rs:254` |
| sym-50ce6387f249a78d3e81 | `frames_pushed` | function | Returns the frames pushed associated with `BridgeSinkTelemetry`. | `src/runtime/nodes.rs:162` |
| sym-d6638e90985ae20d2266 | `freeze` | function | Freezes mutable storage owned by `AudioFrame` into its shared immutable form. | `src/frame/audio.rs:150` |
| sym-18121b64fb4c656cfff9 | `freeze` | function | Freezes mutable storage owned by `LineagedAudioFrame` into its shared immutable form. | `src/frame/audio.rs:289` |
| sym-87c30352006bb76291ee | `freeze` | function | Freezes mutable storage owned by `AudioBufferHandle` into its shared immutable form. | `src/frame/pool.rs:246` |
| sym-cb7e1786405961aea020 | `freeze` | function | Freezes mutable storage owned by `Session` into its shared immutable form. | `src/session/declaration/draft.rs:454` |
| sym-860c89400cb03842aff4 | `from` | function | Converts the supplied value into `NodeTypeId`. | `src/graph/node.rs:37` |
| sym-3d7f6422ad7c37c6fba4 | `from` | function | Converts the supplied value into `SignalId`. | `src/graph/signal/spec.rs:41` |
| sym-c845451343259baf21d7 | `from` | function | Converts the supplied value into `SignalId`. | `src/graph/signal/spec.rs:47` |
| sym-10f672fb4fd9ea0353b5 | `from` | function | Converts the supplied value into `SemanticRole`. | `src/graph/signal/spec.rs:70` |
| sym-16ae1fc4ab19f08b73f9 | `from` | function | Converts the supplied value into `SemanticRole`. | `src/graph/signal/spec.rs:76` |
| sym-ebc447f732c6b6f2bc2c | `from` | function | Converts the supplied value into `SchemaRef`. | `src/graph/signal/spec.rs:100` |
| sym-2bbd9d7852288cc41867 | `from` | function | Converts the supplied value into `SchemaRef`. | `src/graph/signal/spec.rs:106` |
| sym-d089c87e85fdd05a84a7 | `from` | function | Converts the supplied value into `SessionStartError`. | `src/lib.rs:1013` |
| sym-0a09e0720c1f3f33a7a6 | `from` | function | Converts the supplied value into `SessionStartError`. | `src/lib.rs:1022` |
| sym-1c4934e4aa50b3ec4ba1 | `from` | function | Converts the supplied value into `SessionStartError`. | `src/lib.rs:1036` |
| sym-e09bcbd4e53c8ea3bf2a | `from` | function | Converts the supplied value into `SessionStartError`. | `src/lib.rs:1058` |
| sym-96c56f6eb7a26ad320d2 | `from` | function | Converts the supplied value into `PlanEdgeReceiver`. | `src/runtime/audio/router.rs:533` |
| sym-dab7653eb1cd828c2cd6 | `from` | function | Converts the supplied value into `AudioInputWriteError`. | `src/session/extensions/audio_input/buffer.rs:325` |
| sym-45f4c0cdf51bcdc4938c | `from_audio` | function | Creates `SignalEnvelope` from audio. | `src/graph/signal/envelope.rs:27` |
| sym-ed540708783254c6310f | `from_config` | function | Create an encoder from an explicit OpusConfig. | `src/codec/encoder.rs:173` |
| sym-ab62f3af0b2b53f135a3 | `from_frame` | function | Creates `SignalLineage` from frame. | `src/graph/signal/lineage.rs:46` |
| sym-afed6d749b1c8f4c3fe2 | `from_frame` | function | Creates `SignalTiming` from frame. | `src/graph/signal/timing.rs:56` |
| sym-925a2e0b934e56ec63a6 | `from_index` | function | Creates a stable runtime node identifier for externally assembled plans. | `src/graph/spec.rs:12` |
| sym-59196294cd066af3a974 | `from_item` | function | Creates `ConnectorAudioRecord` from item. | `src/connector/transport.rs:314` |
| sym-2d85553d88646e996814 | `from_monotonic_timestamp_ns` | function | Creates `SessionTimelineOrigin` from monotonic timestamp nanoseconds. | `src/endpoint/runtime.rs:18` |
| sym-c0060486788a83e60a1f | `from_node` | function | Creates `ExecError` from node. | `src/runtime/audio/executor.rs:26` |
| sym-94ab6945ccd49e5981c0 | `from_open_observations` | function | Records platform authorization observations without inferring them from a generic backend result. Callers must pass `NotObservable` when their platform has no authoritative query for the requested capture class. | `src/capture/authorization.rs:76` |
| sym-216ca51c178e3d2bdb1e | `from_resolved` | function | Creates `ConnectorConfigurationRecord` from resolved. | `src/connector/transport.rs:45` |
| sym-54144482285221055f1c | `from_source` | function | Creates `EndpointRouteContext` from source. | `src/endpoint/runtime.rs:57` |
| sym-bda4c6758070e1b945d8 | `from_source_output` | function | Wraps a public external-source output in the same typed Rust façade. Runtime identity remains the output's stable `SignalSpec` and schema. | `src/session/declaration/typed_stream.rs:118` |
| sym-efba1f597832298035b1 | `from_stem` | function | Creates `EndpointRouteContext` from stem. | `src/endpoint/runtime.rs:50` |
| sym-8c6c8318a76c699d5180 | `from_stem` | function | Creates `Stream` from stem. | `src/session/declaration/typed_stream.rs:103` |
| sym-d2f25b72435c052d1f41 | `generated_audio_ingresses` | function | Returns the generated audio ingresses associated with `SessionSpec`. | `src/session/declaration/spec.rs:335` |
| sym-902f4b348d0a0b41ec9a | `generation` | function | Returns the generation associated with `SourceLifecycleRegistry`. | `src/capture/lifecycle_registry.rs:84` |
| sym-1b8d79e615e1fe954f3c | `generation` | function | Returns the generation associated with `AsyncOperatorManifest`. | `src/graph/signal/operator.rs:184` |
| sym-0fb2f47c4afb90cac532 | `generation` | function | Returns the generation associated with `NativeExtensionRegistration`. | `src/native_extension/mod.rs:54` |
| sym-164550678c275cd1ca9e | `generation` | function | Returns the implementation generation. | `src/session/extensions/source.rs:166` |
| sym-c7c63cd95c4233b5a1ad | `get` | function | Returns the value held by `ConnectorConfiguration`. | `src/connector/configuration.rs:134` |
| sym-1baa6743961e6be9c1ac | `get` | function | Returns the value held by `ResolvedConnectorConfiguration`. | `src/connector/configuration.rs:394` |
| sym-a9c7fe9b88f1bfd173a5 | `get` | function | Returns the value held by `ClockDomainId`. | `src/frame/identity.rs:36` |
| sym-3a76f39dc504291bffd6 | `get` | function | Returns the value held by `NodeConfig`. | `src/graph/node.rs:92` |
| sym-bdcdf4508ad2f85d3973 | `get` | function | Returns the value held by `NodeRegistry`. | `src/graph/registry.rs:127` |
| sym-a5be6413e3e1a68543cb | `get` | function | Returns the value held by `EndpointConfiguration`. | `src/session/declaration/endpoint.rs:60` |
| sym-d8cbedb909a6848e51df | `get` | function | Returns the value held by `ProcessId`. | `src/session/declaration/selector.rs:13` |
| sym-e3343c30584c85eae6ef | `get` | function | Returns the value held by `SourceConfiguration`. | `src/session/extensions/source.rs:100` |
| sym-43ea790b9b2e11b1f367 | `get_f32` | function | Returns the get f32 associated with `NodeConfig`. | `src/graph/node.rs:100` |
| sym-a60956ec58426d610a4a | `get_u32` | function | Returns the get u32 associated with `NodeConfig`. | `src/graph/node.rs:104` |
| sym-f98868400346c425f7be | `group_id` | function | Returns the group identifier held by `SessionMultistemEndpointCoordinator`. | `src/recording/endpoint.rs:70` |
| sym-f6cc06d0b9a22649ff12 | `handle` | function | Returns the handle associated with `SessionTraceRecorder`. | `src/session/lifecycle/trace.rs:197` |
| sym-8ee18b8cc0f3b596159d | `health` | function | Returns the health held by `ConnectorServiceStatus`. | `src/connector/status.rs:46` |
| sym-dd37553bb3a13016e289 | `health_reason_code` | function | Returns the health reason code held by `ConnectorServiceStatus`. | `src/connector/status.rs:58` |
| sym-951ac3ec1d62f3926c6c | `hz` | function | Returns the hz associated with `OpusSampleRate`. | `src/codec/encoder.rs:49` |
| sym-b19bfeb5e328f1d64e7f | `id` | function | Returns the id held by `ConnectorCapability`. | `src/connector/manifest.rs:30` |
| sym-43225e980a364baa9ced | `id` | function | Returns the id held by `ConnectorRequirement`. | `src/connector/manifest.rs:61` |
| sym-e0cb77466f989d46496c | `id` | function | Returns the id held by `NodeHandle`. | `src/graph/dsl.rs:15` |
| sym-a96cb8990200bc9574fc | `id` | function | Returns the id held by `ResolvedNode`. | `src/graph/ir.rs:16` |
| sym-09e7e32d4dfc4ecbf4e6 | `id` | function | Returns the id held by `Session`. | `src/lib.rs:382` |
| sym-5910add440db99cd12a3 | `id` | function | Returns the id held by `NativeExtensionRegistration`. | `src/native_extension/mod.rs:42` |
| sym-ad406a727cd620096863 | `id` | function | Returns the id held by `SidecarHost`. | `src/runtime/lifecycle/sidecar_host.rs:253` |
| sym-a7ad1513b93c5ad30864 | `id` | function | Returns the id held by `Session`. | `src/session/declaration/draft.rs:331` |
| sym-15050b2d888ca56d48b8 | `id` | function | Returns the id held by `EndpointHandle`. | `src/session/declaration/draft.rs:591` |
| sym-b54048695892b0319f8e | `id` | function | Returns the id held by `StemHandle`. | `src/session/declaration/draft.rs:783` |
| sym-2447943f965c85f4ddfb | `id` | function | Returns the id held by `DeviceSelector`. | `src/session/declaration/selector.rs:117` |
| sym-207a83540bcfaf042651 | `id` | function | Returns the id held by `StemSpec`. | `src/session/declaration/spec.rs:151` |
| sym-04ea444c538a333dad7d | `id` | function | Returns the id held by `EndpointSpec`. | `src/session/declaration/spec.rs:171` |
| sym-dc328e0cd0b6bacf262d | `id` | function | Returns the id held by `ConnectionSpec`. | `src/session/declaration/spec.rs:259` |
| sym-311cba5892f020a3e297 | `id` | function | Returns the id held by `ClockDomainDescriptor`. | `src/timing/domain.rs:31` |
| sym-ac5996bdaf9b5597f56e | `identity_strength` | function | Returns the identity strength held by `CaptureSource`. | `src/capture/identity.rs:94` |
| sym-3500a56c54d98069d733 | `implementation_generation` | function | Monotonic implementation generation for this manifest revision. | `src/session/extensions/source.rs:158` |
| sym-5c1cf0d28403174ca276 | `in_` | function | Selects a named input port from `NodeHandle`. | `src/graph/dsl.rs:24` |
| sym-55c28d8253dcbcfabb81 | `index` | function | Returns the index held by `AudioBufferHandle`. | `src/frame/pool.rs:211` |
| sym-29252c2c91d60625b91c | `index` | function | Returns the index held by `SharedAudioBufferHandle`. | `src/frame/pool.rs:296` |
| sym-924609a46165ae2cfe6e | `index` | function | Returns the index held by `NodeId`. | `src/graph/spec.rs:16` |
| sym-c69d054b4be929761cf1 | `index` | function | Returns the index held by `EdgeId`. | `src/graph/spec.rs:25` |
| sym-f325d519bb3a4440848c | `ingress_rejected_total` | function | Returns the ingress rejected total held by `SessionAudioReentryMetrics`. | `src/session/lifecycle/observations.rs:364` |
| sym-803ad52760f815fa63af | `input` | function | Returns the input held by `ConnectorItem`. | `src/connector/worker/driver.rs:74` |
| sym-82a3c6ac01987407ccb5 | `input` | function | Returns the input held by `OperatorInstanceHandle`. | `src/session/declaration/draft.rs:717` |
| sym-eba82c263e0eb85f9aaa | `input_attempted_total` | function | Returns the input attempted total held by `SessionOperatorMetrics`. | `src/session/lifecycle/observations.rs:413` |
| sym-e4e0debc087e09405c5a | `input_delivered_total` | function | Returns the input delivered total held by `SessionOperatorMetrics`. | `src/session/lifecycle/observations.rs:421` |
| sym-e10f29d8fd098a134e69 | `input_dropped_total` | function | Returns the input dropped total held by `SessionOperatorMetrics`. | `src/session/lifecycle/observations.rs:425` |
| sym-dd96ef2aeb6ef1eb26f8 | `input_edge` | function | Returns the input edge associated with `AsyncOperatorManifest`. | `src/graph/signal/operator.rs:192` |
| sym-f1ae93c990799f3815cd | `input_edge` | function | Returns the input edge associated with `EndpointDescriptor`. | `src/session/declaration/endpoint.rs:153` |
| sym-7cb3b2c18857bf487602 | `input_edge` | function | Returns the input edge associated with `EndpointSpec`. | `src/session/declaration/spec.rs:191` |
| sym-fcbe4e1bf437f2f0a843 | `input_enqueued_total` | function | Returns the input enqueued total held by `SessionOperatorMetrics`. | `src/session/lifecycle/observations.rs:417` |
| sym-a6cfc39c2048b414c483 | `input_mut` | function | Returns the input mut associated with `AsyncOperatorWorker`. | `src/runtime/signal/operator.rs:917` |
| sym-53ea5f8920e32a5e87d1 | `input_port` | function | Returns the input port held by `TypedOperator`. | `src/session/declaration/typed_stream.rs:75` |
| sym-a9741e2e08e6ad5feee9 | `input_port` | function | Returns the input port held by `SessionOperatorMetrics`. | `src/session/lifecycle/observations.rs:398` |
| sym-d9b0585f544c51598a14 | `input_ports` | function | Returns the input ports held by `AsyncOperatorManifest`. | `src/graph/signal/operator.rs:224` |
| sym-f781aa13fb4c138d7535 | `input_queue_capacity_frames` | function | Returns the input queue capacity frames held by `SessionOperatorMetrics`. | `src/session/lifecycle/observations.rs:401` |
| sym-e0bad5d8373d18d407d4 | `input_queue_depth_frames` | function | Returns the input queue depth frames held by `SessionOperatorMetrics`. | `src/session/lifecycle/observations.rs:405` |
| sym-3e190ddbfb9ec39c506e | `input_queue_peak_frames` | function | Returns the input queue peak frames held by `SessionOperatorMetrics`. | `src/session/lifecycle/observations.rs:409` |
| sym-8ad9865b29a5785fee96 | `input_spec` | function | Returns the input spec held by `TypedOperator`. | `src/session/declaration/typed_stream.rs:83` |
| sym-1b14d521019c46c6df5f | `inputs` | function | Returns the inputs associated with `NodeDescriptor`. | `src/graph/node.rs:230` |
| sym-8f5ae7c7298283723995 | `inputs` | function | Returns the inputs associated with `AsyncOperatorPrepareContext`. | `src/graph/signal/preparation.rs:62` |
| sym-fd92f420eac5795ad4f9 | `insert` | function | Inserts a typed configuration value into `ConnectorConfiguration`. | `src/connector/configuration.rs:126` |
| sym-8176a230bc859ac5f791 | `insert` | function | Adds declared source configuration. | `src/session/extensions/source.rs:96` |
| sym-fdc8d3af1d058f8c25cd | `instance_id` | function | Returns the instance identifier held by `OperatorInstanceHandle`. | `src/session/declaration/draft.rs:713` |
| sym-f3fd41c23b6d9ab2d5b8 | `instance_id` | function | Returns the instance identifier held by `SourceInstanceHandle`. | `src/session/declaration/draft.rs:846` |
| sym-c1301f10816aa2a31d70 | `instance_id` | function | Returns the instance identifier held by `SourceInstanceSpec`. | `src/session/declaration/spec.rs:76` |
| sym-53543b9e48524d7298bc | `instance_id` | function | Returns the instance identifier held by `OperatorInstanceSpec`. | `src/session/declaration/spec.rs:245` |
| sym-4f321ed7a203a3f5cf9c | `instantiate` | function | Instantiates the runtime node described by `PassthroughFactory`. | `src/graph/builtins.rs:86` |
| sym-66e194664cbbd9404707 | `instantiate` | function | Instantiates the runtime node described by `GainFactory`. | `src/graph/builtins.rs:135` |
| sym-c7f732c51c86e2736226 | `instantiate` | function | Instantiates the runtime node described by `MonoMixFactory`. | `src/graph/builtins.rs:185` |
| sym-b4b9f4d0abada70447e0 | `instantiate` | function | Instantiates the runtime node described by `SystemOutputSourceFactory`. | `src/runtime/nodes.rs:103` |
| sym-30e204a84afd19dbc60d | `instantiate` | function | Instantiates the runtime node described by `BridgeSinkFactory`. | `src/runtime/nodes.rs:207` |
| sym-b5be27424500bdeaf8ac | `integral_error_ns` | function | Returns the integral error nanoseconds held by `ClockCorrectionController`. | `src/timing/clock_correction.rs:42` |
| sym-4d54a78d3c364c587267 | `integral_ns` | function | Returns the integral nanoseconds held by `ClockCorrectionController`. | `src/timing/clock_correction.rs:46` |
| sym-45a4263550e920c7598d | `into_callback` | function | Converts `CapturedFrameSender` into callback. | `src/capture/frame_stream.rs:132` |
| sym-7108c905ca4780df696b | `into_configuration` | function | Converts `ConnectorConfigurationRecord` into configuration. | `src/connector/transport.rs:57` |
| sym-b74082477b551771a0a0 | `into_endpoint_failure` | function | Converts `ConnectorError` into endpoint failure. | `src/connector/error.rs:125` |
| sym-473286cbfb3dd1982f85 | `into_error` | function | Converts `SessionStartFailure` into error. | `src/session/lifecycle/control.rs:319` |
| sym-d75c6c2ea414e5af58f9 | `into_parts` | function | Consumes `EndpointPortInput` and returns its component values. | `src/endpoint/contract.rs:257` |
| sym-cf86a3d6bcb80a95cb9d | `into_parts` | function | Consumes `LineagedAudioFrame` and returns its component values. | `src/frame/audio.rs:285` |
| sym-a10bbd6f35f0f08a386f | `into_parts` | function | Consumes `PcmSource` and returns its component values. | `src/session/extensions/audio_input/source.rs:68` |
| sym-c64fa9fb69d22a43a170 | `into_payload` | function | Converts `SignalEnvelope` into payload. | `src/graph/signal/envelope.rs:86` |
| sym-aa61f33aa27c9b536714 | `into_pcm_source` | function | Converts the convenience façade into explicit source, output, and producer ownership. | `src/session/extensions/audio_input/mod.rs:137` |
| sym-fe9469ed3661a3b13b96 | `into_plan_edge_receiver` | function | Converts `EndpointAudioReceiver` into plan edge receiver. | `src/endpoint/contract.rs:106` |
| sym-64dbb4a8d0ef710f8861 | `into_rejected` | function | Converts `SignalEdgeSendError` into rejected. | `src/runtime/signal/edge.rs:123` |
| sym-648793e6c554ee61d325 | `into_rejected` | function | Converts `AudioInputWriteError` into rejected. | `src/session/extensions/audio_input/buffer.rs:319` |
| sym-577f70ac8cddb985786a | `into_spec` | function | Converts `Pipeline` into spec. | `src/graph/dsl.rs:90` |
| sym-b0c9411e8bbf3f1bab03 | `into_start_failure` | function | Converts `SessionEngineStartError` into start failure. | `src/session/lifecycle/engine.rs:336` |
| sym-6239186ad2720fa23c3e | `invalid_total` | function | Returns the invalid total held by `SessionAudioReentryMetrics`. | `src/session/lifecycle/observations.rs:352` |
| sym-1bed125889cc7f8a2391 | `is_abandoned` | function | Reports whether abandoned is true for `EndpointAudioReceiver`. | `src/endpoint/contract.rs:122` |
| sym-adf717467b01bb7fab5e | `is_abandoned` | function | Reports whether abandoned is true for `EndpointSignalReceiver`. | `src/endpoint/contract.rs:170` |
| sym-d3b35a7e7b9712ddaea7 | `is_abandoned` | function | Reports whether abandoned is true for `PlanEdgeReceiver`. | `src/runtime/audio/router.rs:574` |
| sym-0eefd4234befb8a34c61 | `is_abort_requested` | function | Reports whether abort requested is true for `ConnectorContext`. | `src/connector/worker/coordination.rs:36` |
| sym-b906c1bced2bdb6a5317 | `is_audio` | function | Returns `true` for classes that carry real-time audio on the hot path. | `src/graph/signal/spec.rs:180` |
| sym-4eaeb64e7d8506a0bbd6 | `is_cancelled` | function | Reports whether cancelled is true for `SessionStartError`. | `src/lib.rs:1007` |
| sym-e0f463ea1df54f114b27 | `is_cancelled` | function | Reports whether cancelled is true for `SourceCancellation`. | `src/session/extensions/source.rs:255` |
| sym-9bf1b58161c9c643628e | `is_closed` | function | Reports whether closed is true for `CapturedFrameStream`. | `src/capture/frame_stream.rs:170` |
| sym-13ba64b8902a67ddb451 | `is_compatible_with` | function | Reports whether compatible with is true for `ChannelLayout`. | `src/graph/ports.rs:42` |
| sym-ba6f1a5ae34c37391c97 | `is_compatible_with` | function | Reports whether compatible with is true for `AudioCaps`. | `src/graph/ports.rs:56` |
| sym-1660e53891c50aea7f58 | `is_compatible_with` | function | Reports whether compatible with is true for `MediaCaps`. | `src/graph/ports.rs:110` |
| sym-53a017982fc527349282 | `is_compatible_with` | function | Returns `true` if two signal classes are compatible for edge wiring. | `src/graph/signal/spec.rs:188` |
| sym-5762003566d065479b7e | `is_compatible_with` | function | Returns `true` if this spec can connect to `other` on an edge. | `src/graph/signal/spec.rs:324` |
| sym-3a06e547847d70e25e6b | `is_complete` | function | Reports whether complete is true for `SessionTraceRecorderOutcome`. | `src/session/lifecycle/trace.rs:80` |
| sym-104b3bac350417f37f40 | `is_empty` | function | Returns whether `ConnectorConfiguration` contains no values. | `src/connector/configuration.rs:146` |
| sym-88bb7e90d368d40d5266 | `is_empty` | function | Returns whether `PolledAudioBatchLease` contains no values. | `src/endpoint/polled_audio_driver.rs:228` |
| sym-694b0417ceb4093d53c5 | `is_empty` | function | Returns whether `AudioBufferHandle` contains no values. | `src/frame/pool.rs:208` |
| sym-377854407927a5521f81 | `is_empty` | function | Returns whether `SharedAudioBufferHandle` contains no values. | `src/frame/pool.rs:292` |
| sym-b2fd5f431d9ee97ce804 | `is_empty` | function | Returns whether `NodeRegistry` contains no values. | `src/graph/registry.rs:174` |
| sym-7f13b15d2c80392c67f2 | `is_in_use` | function | Reports whether in use is true for `AudioBufferPool`. | `src/frame/pool.rs:98` |
| sym-3ae6167df33d7c121fb4 | `is_open` | function | Reports whether open is true for `EndpointStartGate`. | `src/endpoint/runtime.rs:376` |
| sym-6ece39cf3776addca714 | `is_portable` | function | Reports whether this value is a portable implementation contract ID. | `src/graph/operator.rs:31` |
| sym-93b640de9e9b9e6dae8c | `is_portable` | function | Reports whether this custom signal ID is portable across packages, languages, and processes. | `src/graph/signal/spec.rs:35` |
| sym-0d8267066ef89ab05c61 | `is_realtime` | function | Reports whether realtime is true for `ClockDomain`. | `src/graph/ports.rs:259` |
| sym-4a925d7654caf42d1a2a | `is_requested` | function | Reports whether requested is true for `PlanRunnerCancellation`. | `src/runtime/audio/runner.rs:104` |
| sym-5ded1e414552d586b30c | `is_requested` | function | Reports whether requested is true for `SessionStartCancellation`. | `src/session/lifecycle/control.rs:115` |
| sym-963f249bd1aea6af2b47 | `is_sensitive` | function | Reports whether sensitive is true for `NodeConfig`. | `src/graph/node.rs:96` |
| sym-1ad646036d38dee279d4 | `is_sensitive` | function | Reports whether sensitive is true for `EndpointConfiguration`. | `src/session/declaration/endpoint.rs:64` |
| sym-2afbd1ea7c3f124dfc31 | `is_stateful` | function | Reports whether stateful is true for `NodeDescriptor`. | `src/graph/node.rs:246` |
| sym-fa0244c5440eb6e18364 | `is_stereo` | function | Reports whether stereo is true for `StreamProfile`. | `src/codec/profile.rs:69` |
| sym-1440b0574deddcb13357 | `is_stop_requested` | function | Reports whether stop requested is true for `ConnectorContext`. | `src/connector/worker/coordination.rs:28` |
| sym-470e0001ab5a1616f788 | `is_success` | function | Reports whether success is true for `SessionCancelResult`. | `src/lib.rs:1137` |
| sym-28e2bbffc0337bc13f4c | `is_success` | function | Reports whether success is true for `SessionStopResult`. | `src/lib.rs:1157` |
| sym-7fcb6d2fa6694ec5e39a | `is_success` | function | Reports whether success is true for `SessionStopOutcome`. | `src/session/lifecycle/control.rs:349` |
| sym-aade164ba212f0ffdcd9 | `is_terminal` | function | Reports whether terminal is true for `OperatorOutputRolePolicy`. | `src/graph/signal/operator.rs:86` |
| sym-62d0ddd2396f4158a7c8 | `is_valid_for` | function | Returns `true` if this contract is compatible with the given partition. | `src/graph/partition.rs:107` |
| sym-5c57b5f3925d88b78690 | `is_well_formed` | function | Reports whether this value follows the portable node-type syntax. | `src/graph/node.rs:25` |
| sym-77a3c9fd80fed3c3b18f | `iter` | function | Iterates over the values held by `ConnectorConfiguration`. | `src/connector/configuration.rs:138` |
| sym-d8cf73924fb51e8b2b8e | `iter` | function | Iterates over the values held by `ResolvedConnectorConfiguration`. | `src/connector/configuration.rs:398` |
| sym-e800a22871e2d39d4a92 | `iter` | function | Iterates over the values held by `NodeConfig`. | `src/graph/node.rs:108` |
| sym-3b2e1392dc84776988a4 | `iter` | function | Iterates over the values held by `EndpointConfiguration`. | `src/session/declaration/endpoint.rs:68` |
| sym-d742331b94ed5efc53cc | `iter` | function | Iterates over the values held by `SourceConfiguration`. | `src/session/extensions/source.rs:104` |
| sym-737471186be91f70d361 | `jitter_budget_ms` | function | Returns the jitter budget milliseconds held by `EdgeContract`. | `src/graph/ports.rs:337` |
| sym-3062cd188c729dd16e4c | `join` | function | Joins its owned operation for `SourceRuntime`. | `src/session/extensions/source.rs:583` |
| sym-f41915605cfe34eae0c1 | `joined` | function | Returns whether joined is true for `SessionAudioReentryMetrics`. | `src/session/lifecycle/observations.rs:376` |
| sym-e136b77b30dedf88de08 | `kind` | function | Returns the kind represented by `ConnectorConfigurationValue`. | `src/connector/configuration.rs:77` |
| sym-57621d660613d124bc7d | `kind` | function | Returns the kind represented by `MediaCaps`. | `src/graph/ports.rs:97` |
| sym-e98d263d46b95716776c | `kind` | function | Returns the kind represented by `SessionStartError`. | `src/lib.rs:991` |
| sym-e73975dc1292035caa2d | `kind` | function | Returns the kind represented by `NativeExtensionRegistration`. | `src/native_extension/mod.rs:46` |
| sym-b7faa716d321d69280cc | `kind` | function | Returns the kind represented by `AudioInputWriteError`. | `src/session/extensions/audio_input/buffer.rs:315` |
| sym-bcdd279afc1d281f56db | `kind` | function | Returns the kind represented by `SessionEvent`. | `src/session/lifecycle/events.rs:322` |
| sym-c7bb8ee02a040b352a18 | `kind` | function | Returns the kind represented by `ClockDomainDescriptor`. | `src/timing/domain.rs:35` |
| sym-9de1f3176f7cf121db5a | `lane_underruns` | function | Returns the lane underruns associated with `MixerTelemetry`. | `src/runtime/nodes.rs:257` |
| sym-4b74678cd492f3180b81 | `last_correction_ns` | function | Returns the last correction nanoseconds held by `ClockCorrectionController`. | `src/timing/clock_correction.rs:39` |
| sym-7047cde0c35ee9a4eebd | `last_offset_ns` | function | Returns the last offset nanoseconds held by `ClockCorrectionController`. | `src/timing/clock_correction.rs:36` |
| sym-3e3073a9a190630017e1 | `last_transition_elapsed_ns` | function | Returns the last transition elapsed nanoseconds held by `ConnectorServiceStatus`. | `src/connector/status.rs:70` |
| sym-88ac630e6853f144d88b | `latency_budget_ms` | function | Returns the latency budget milliseconds held by `EdgeContract`. | `src/graph/ports.rs:333` |
| sym-b83c3de0aeafa5c07084 | `len` | function | Returns the number of values held by `ConnectorConfiguration`. | `src/connector/configuration.rs:142` |
| sym-3538665e71576de44f59 | `len` | function | Returns the number of values held by `PolledAudioBatchLease`. | `src/endpoint/polled_audio_driver.rs:224` |
| sym-f4dfdeb380903e4528bd | `len` | function | Returns the number of values held by `AudioBufferHandle`. | `src/frame/pool.rs:205` |
| sym-708eb84562629092eee6 | `len` | function | Returns the number of values held by `SharedAudioBufferHandle`. | `src/frame/pool.rs:288` |
| sym-5584661d81e492674fbc | `len` | function | Returns the number of values held by `NodeRegistry`. | `src/graph/registry.rs:169` |
| sym-b64b3175821197baa243 | `lineage` | function | Returns the frame lineage carried by `EndpointAudioFrame`. | `src/endpoint/contract.rs:73` |
| sym-2d7ab65f4fc048dcf957 | `lineage` | function | Returns the frame lineage carried by `PolledAudioFrame`. | `src/endpoint/polled_audio_driver.rs:261` |
| sym-6aca508b0efb6ddab602 | `lineage` | function | Returns the frame lineage carried by `LineagedAudioFrame`. | `src/frame/audio.rs:281` |
| sym-c825aea26873c6f6ee1a | `lineage` | function | Returns the frame lineage carried by `SharedLineagedAudioFrame`. | `src/frame/audio.rs:308` |
| sym-d928dfcd4acd982941e6 | `lineage` | function | Returns the frame lineage carried by `SignalEnvelope`. | `src/graph/signal/envelope.rs:78` |
| sym-94ff6a80aa2957e7af57 | `lineage` | function | Returns the frame lineage carried by `PlanEdgeFrame`. | `src/runtime/audio/router.rs:91` |
| sym-25b1030b088f2e4bbf32 | `lineage_failures_total` | function | Returns the lineage failures total held by `SessionStopOutcome`. | `src/session/lifecycle/control.rs:379` |
| sym-20d353b749f431e7b1d2 | `load_native_extension_library` | function | Loads one trusted native dynamic library from an exact absolute path and imports its supported non-realtime source, operator, and endpoint registrations into this Session as one validated set. | `src/session/extensions/native_library.rs:29` |
| sym-0678413cd713daedd00b | `loss` | function | Returns the loss associated with `EdgeContract`. | `src/graph/ports.rs:349` |
| sym-777d37dbdf0f69839bf3 | `major` | function | Returns the major associated with `SessionSpecVersion`. | `src/session/declaration/spec.rs:51` |
| sym-965aa3b29c0459c9cb66 | `manifest` | function | Returns the manifest held by `Connector`. | `src/connector/mod.rs:119` |
| sym-c5ae77d57d3faef142e8 | `manifest` | function | Returns the manifest held by `RegisteredConnector`. | `src/connector/mod.rs:136` |
| sym-068a7cf335367ac83e4c | `manifest` | function | Returns the manifest held by `SourceRegistry`. | `src/session/extensions/source.rs:291` |
| sym-e5b207f73575f5d653dc | `manifest_revision` | function | Returns the manifest revision held by `ConnectorManifest`. | `src/connector/manifest.rs:128` |
| sym-3e7d253933f0017b44ba | `map_payload` | function | Transforms the payload held by `SignalEnvelope` while preserving envelope metadata. | `src/graph/signal/envelope.rs:45` |
| sym-f3819a7c5ca28bd21136 | `mark_discontinuity` | function | Marks the next value from `EndpointAudioReceiver` as discontinuous. | `src/endpoint/contract.rs:126` |
| sym-679b7143612407d3c2eb | `mark_discontinuity` | function | Marks the next value from `PlanEdgeReceiver` as discontinuous. | `src/runtime/audio/router.rs:659` |
| sym-7ee49939dfaf9f2513af | `mark_discontinuity` | function | Marks the next value from `AudioInputBuffer` as discontinuous. | `src/session/extensions/audio_input/buffer.rs:46` |
| sym-cb3538ab3a970277e126 | `mark_worker_failure` | function | Returns the mark worker failure held by `EndpointAudioReceiver`. | `src/endpoint/contract.rs:130` |
| sym-fd347baeba2a2565f020 | `mark_worker_failure` | function | Returns the mark worker failure held by `PlanEdgeReceiver`. | `src/runtime/audio/router.rs:668` |
| sym-2e8e580510f3a911800b | `matches` | function | Returns whether an input satisfies `SourceQuery`. | `src/capture/query.rs:22` |
| sym-d3dc8c94b015f5f22897 | `max_frame_bytes` | function | Returns the max frame bytes held by `SidecarProtocolLimits`. | `src/runtime/lifecycle/sidecar_protocol.rs:62` |
| sym-ef33765f60bd132eadb2 | `max_payload_bytes` | function | Returns the max payload bytes held by `EdgeContract`. | `src/graph/ports.rs:361` |
| sym-ff2185c4e90cb1f5c4ef | `maximum_buffered_audio_bytes` | function | Returns the maximum buffered audio bytes held by `SessionAudioReentryMetrics`. | `src/session/lifecycle/observations.rs:344` |
| sym-aae44a4fc0ef2c5793f4 | `media` | function | Returns the media held by `ConnectorInputDescriptor`. | `src/connector/worker/driver.rs:48` |
| sym-42f74cceaa2efbea8eb6 | `media` | function | Returns the media held by `EndpointPortInput`. | `src/endpoint/contract.rs:241` |
| sym-772e938f00772a8503ef | `media` | function | Returns the media held by `PortPrepareContext`. | `src/graph/node.rs:353` |
| sym-7e8ce0544afa645581b4 | `media` | function | Returns the media held by `PortSpec`. | `src/graph/ports.rs:225` |
| sym-22676867b0d15bab67f4 | `media` | function | Returns the media held by `EdgeContract`. | `src/graph/ports.rs:325` |
| sym-ab7515a9ee9c63ae46ca | `message` | function | Returns the diagnostic message reported by `ConnectorConfigurationError`. | `src/connector/configuration.rs:623` |
| sym-fa40c22659bda8c12332 | `message` | function | Returns the diagnostic message reported by `ConnectorError`. | `src/connector/error.rs:121` |
| sym-b919fbaaf3ee95bd524f | `message` | function | Returns the diagnostic message reported by `EndpointFailure`. | `src/endpoint/runtime.rs:208` |
| sym-103f03ff1cd5df7d34be | `message` | function | Returns the diagnostic message reported by `SessionStartError`. | `src/lib.rs:978` |
| sym-7c29f2093b359646c32c | `message` | function | Returns the diagnostic message reported by `NativeExtensionLibraryError`. | `src/native_extension/mod.rs:135` |
| sym-651a18d4220e9dbb7f9a | `metadata` | function | Returns the metadata held by `ConnectorAudioRecord`. | `src/connector/transport.rs:339` |
| sym-113e05ae5fbaa2c0f028 | `metric_id` | function | Returns the metric identifier held by `RuntimePlan`. | `src/graph/plan.rs:145` |
| sym-c2c639401b42645bb6ba | `metrics` | function | Convenience constructor for metrics ports. | `src/graph/signal/spec.rs:289` |
| sym-cec54ccfc90f9b356ac1 | `metrics_snapshot` | function | Returns the metrics snapshot associated with `RunningSession`. | `src/lib.rs:839` |
| sym-7523c93d162838f46c5a | `metrics_snapshot` | function | Returns the metrics snapshot associated with `SessionEngineHost`. | `src/session/lifecycle/host.rs:117` |
| sym-2d28a8df3bf63e332c0b | `microphone` | function | Creates `Source` for the selected microphone device. | `src/session/declaration/selector.rs:144` |
| sym-d4bcb79d234d924695a6 | `microphone_default` | function | Creates `Source` for the host default microphone. | `src/session/declaration/selector.rs:148` |
| sym-9e158e07f5d9263a8c21 | `minor` | function | Returns the minor associated with `SessionSpecVersion`. | `src/session/declaration/spec.rs:56` |
| sym-0e94a9647c8615d0dc7a | `monotonic_timestamp_ns` | function | Returns the monotonic timestamp nanoseconds held by `SessionTimelineOrigin`. | `src/endpoint/runtime.rs:24` |
| sym-fc477cd677af53c92c6b | `multiplicity` | function | Returns the multiplicity associated with `PortSpec`. | `src/graph/ports.rs:229` |
| sym-c4d51046d48fdc71e04f | `name` | function | Returns the name associated with `ConnectorConfigurationField`. | `src/connector/configuration.rs:206` |
| sym-788106bf95ab2bdd7f7c | `name` | function | Returns the name associated with `PortSpec`. | `src/graph/ports.rs:213` |
| sym-58de22dd7a29f7960d69 | `name` | function | Returns the name associated with `ApplicationSelector`. | `src/session/declaration/selector.rs:63` |
| sym-2beace5df40535edecfd | `native` | function | Returns the native associated with `SessionEngineHost`. | `src/session/lifecycle/host.rs:40` |
| sym-5893a4d114a25f68ad01 | `native` | function | Creates the production host builder with the platform's native capture backend, leaving endpoint registration open to the owning application. | `src/session/lifecycle/host.rs:223` |
| sym-09373ca757e99da6a9e6 | `native_with_multistem_recording` | function | Builds a native Session host with one multistem recorder. | `src/session/lifecycle/host.rs:48` |
| sym-fb87a273b6f988c4aa46 | `needs_bridge_to` | function | Returns `true` if crossing from `self` to `other` requires a compiler-inserted Bridge. | `src/graph/partition.rs:71` |
| sym-4b477dab963879323344 | `negotiate` | function | Negotiates the compatible media capabilities shared by `MediaCaps` and its peer. | `src/graph/ports.rs:124` |
| sym-a4c74d7d271cbf17cdd3 | `new` | function | Creates a new `PksSessionStatus`. | `src/abi/session/abi.rs:69` |
| sym-4f9f71d68deb2e375cec | `new` | function | Creates a new `CapturePermissionLifecycle`. | `src/capture/authorization.rs:189` |
| sym-2bce5173c11b0fedfccf | `new` | function | Creates a new `CaptureLineageSeed`. | `src/capture/capture_owner.rs:31` |
| sym-ac4709197071b041b435 | `new` | function | Creates a new `StableSourceId`. | `src/capture/identity.rs:33` |
| sym-c7a07ecd7218e8ef4290 | `new` | function | Creates a new `CaptureSampleTimeline`. | `src/capture/timeline.rs:52` |
| sym-83741bae354adc93a18c | `new` | function | Mono decoder (48 kHz). Back-compatible default for the existing pipeline. | `src/codec/decoder.rs:39` |
| sym-f69c5522aea4db917a27 | `new` | function | Create a new encoder with default config (48 kHz, mono, Voip, 20 ms). | `src/codec/encoder.rs:168` |
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
| sym-cfc6333600b6679e4a06 | `new` | function | Creates a new `EndpointGroupId`. | `src/endpoint/identity.rs:12` |
| sym-4de54826fa27a0e9d1c1 | `new` | function | Creates a new `PolledAudioEndpoint`. | `src/endpoint/polled_audio.rs:22` |
| sym-87d5185fa15ce8aee6f4 | `new` | function | Creates a new `EndpointPrepareContext`. | `src/endpoint/runtime.rs:108` |
| sym-70d2ba55ff9f6b881949 | `new` | function | Creates a new `EndpointFailure`. | `src/endpoint/runtime.rs:182` |
| sym-5125e04d40034efc2ebe | `new` | function | Creates a new `SampleSpec`. | `src/frame/audio.rs:25` |
| sym-f947c3edff5ed55495b3 | `new` | function | Creates a new `LineagedAudioFrame`. | `src/frame/audio.rs:272` |
| sym-7cb01f758280769116b4 | `new` | function | Creates a new `ClockDomainId`. | `src/frame/identity.rs:32` |
| sym-7e14191fea2ca3eed1a3 | `new` | function | Creates a new `AudioBufferPool`. | `src/frame/pool.rs:39` |
| sym-b68c3e616aaa58b68339 | `new` | function | Creates a new `RuntimePlanner`. | `src/graph/compile/plan.rs:14` |
| sym-7e5f2e28b6331376d63a | `new` | function | Creates a new `Compiler`. | `src/graph/compile/resolve.rs:449` |
| sym-bd3a861e9622497aecca | `new` | function | Creates a new `Pipeline`. | `src/graph/dsl.rs:40` |
| sym-b5064e4af7ce624453f8 | `new` | function | Creates a new `NodeConfig`. | `src/graph/node.rs:62` |
| sym-687e670038b6609a360f | `new` | function | Creates a new `NodeDescriptor`. | `src/graph/node.rs:176` |
| sym-39e31561ba3bfa2ed241 | `new` | function | Creates a new `PrepareContext`. | `src/graph/node.rs:271` |
| sym-b053bd428778a9cd31eb | `new` | function | Creates a new `PortPrepareContext`. | `src/graph/node.rs:293` |
| sym-f923baa2343ca7973bb0 | `new` | function | Creates a new `OperatorId`. | `src/graph/operator.rs:19` |
| sym-94e3d1cec3698575733a | `new` | function | Creates a new `PortSpec`. | `src/graph/ports.rs:185` |
| sym-d7732458296b48f768a6 | `new` | function | Creates a new `NodeRegistry`. | `src/graph/registry.rs:73` |
| sym-fe437b9fb0aaf12d51c8 | `new` | function | Creates a new `SignalDerivation`. | `src/graph/signal/lineage.rs:107` |
| sym-c8580f82ceed08e7c92b | `new` | function | Creates a new `AsyncOperatorManifest`. | `src/graph/signal/operator.rs:144` |
| sym-371f2a20d5a64c7f8939 | `new` | function | Creates a new `AsyncOperatorPrepareContext`. | `src/graph/signal/preparation.rs:29` |
| sym-7bcc18095a8907bf9b15 | `new` | function | Creates a new `SignalId`. | `src/graph/signal/spec.rs:25` |
| sym-46acac4d46fcc2cdad86 | `new` | function | Creates a new `SemanticRole`. | `src/graph/signal/spec.rs:60` |
| sym-54c64d2b00dfed01239b | `new` | function | Creates a new `SchemaRef`. | `src/graph/signal/spec.rs:90` |
| sym-f469a7085bd1b4e5dfbe | `new` | function | Creates a new `SignalSpec`. | `src/graph/signal/spec.rs:226` |
| sym-3451f8706ede10fe944e | `new` | function | Creates a new `Session`. | `src/lib.rs:360` |
| sym-f7519d2b4cb7be1e15b3 | `new` | function | Creates a new `StemLabel`. | `src/recording/config.rs:23` |
| sym-1aca5dad2dcbd16384aa | `new` | function | Creates a new `SessionMultistemEndpointCoordinator`. | `src/recording/endpoint.rs:56` |
| sym-cb36fc591194cfe17a76 | `new` | function | Creates a new `RealtimePlanExecutor`. | `src/runtime/audio/executor.rs:62` |
| sym-89935ae4f6eb9c47d423 | `new` | function | Creates a new `PlanEdgeRouter`. | `src/runtime/audio/router.rs:709` |
| sym-efe3840761061b373ea8 | `new` | function | Creates a new `PlanRunnerCancellation`. | `src/runtime/audio/runner.rs:94` |
| sym-62cf46ad5c0e2d973940 | `new` | function | Creates a new `RealtimePlanRunner`. | `src/runtime/audio/runner.rs:314` |
| sym-bcc9fc05fc4b41dea325 | `new` | function | Creates a new `AsyncRuntimeHost`. | `src/runtime/lifecycle/async_host.rs:33` |
| sym-417142a74fd6534cd6fb | `new` | function | Creates a new `SidecarProcessSpec`. | `src/runtime/lifecycle/sidecar_host.rs:82` |
| sym-0629581701c5639e034d | `new` | function | Creates a new `SystemOutputSourceFactory`. | `src/runtime/nodes.rs:72` |
| sym-b3414446d83b06b7ed47 | `new` | function | Creates a new `BridgeSinkFactory`. | `src/runtime/nodes.rs:176` |
| sym-621acda29d3c06da0307 | `new` | function | Creates a new `TypedEdgeFanout`. | `src/runtime/signal/edge.rs:264` |
| sym-c99d805db5b08d88eea5 | `new` | function | Creates a new `SessionCompiler`. | `src/session/compile/mod.rs:77` |
| sym-c310c4fdcba85ce1bb81 | `new` | function | Creates a new `Operator`. | `src/session/declaration/draft.rs:288` |
| sym-cd88413a3d4c1769deb7 | `new` | function | Creates a new `Session`. | `src/session/declaration/draft.rs:321` |
| sym-d339328f6aa60fc53bed | `new` | function | Creates a new `EndpointConfiguration`. | `src/session/declaration/endpoint.rs:33` |
| sym-996ea1179a30ef46df07 | `new` | function | Creates a new `EndpointDescriptor`. | `src/session/declaration/endpoint.rs:118` |
| sym-db57039f490cc15bdeb6 | `new` | function | Creates a new `ProcessId`. | `src/session/declaration/selector.rs:9` |
| sym-8ea5fa3d4f654ae80326 | `new` | function | Creates a new `DeviceId`. | `src/session/declaration/selector.rs:22` |
| sym-e461d6e42e02f1b28a7c | `new` | function | Creates a new `SourceInstanceId`. | `src/session/declaration/spec.rs:17` |
| sym-4321bd952a0ea917f39b | `new` | function | Creates a new `OperatorInstanceId`. | `src/session/declaration/spec.rs:30` |
| sym-9d8afa1acc9ba715a12a | `new` | function | Creates a new `SessionSpecVersion`. | `src/session/declaration/spec.rs:46` |
| sym-8e581b5e343b30df275c | `new` | function | Creates a new `TypedOperator`. | `src/session/declaration/typed_stream.rs:30` |
| sym-f8dfb59be6f7b2625ad2 | `new` | function | Creates a new `AudioInputConfig`. | `src/session/extensions/audio_input/mod.rs:29` |
| sym-3221f69addb4cf7e9359 | `new` | function | Creates a stable source implementation identity. | `src/session/extensions/source.rs:26` |
| sym-49e20a0c57100acc1c60 | `new` | function | Creates a new `SourceManifest`. | `src/session/extensions/source.rs:122` |
| sym-761c838a14e333e0c953 | `new` | function | Creates a new `SessionEngineBuilder`. | `src/session/lifecycle/engine.rs:43` |
| sym-5a84a5cee97b869331fd | `new` | function | Creates a new `SessionEngineHostBuilder`. | `src/session/lifecycle/host.rs:243` |
| sym-2059252abf3a743c704d | `new` | function | Creates a new `ClockCorrectionController`. | `src/timing/clock_correction.rs:13` |
| sym-433c1f9402adccdef4f0 | `new` | function | Creates a new `ClockDriftEstimator`. | `src/timing/clock_drift.rs:22` |
| sym-0d08ae9c5f3506a6addd | `new` | function | Creates a new `TimelineMapping`. | `src/timing/timeline_mapping.rs:8` |
| sym-333a6a244c19b535857c | `new_with_output_channels` | function | Creates `MixerSourceNode` with the supplied output channels. | `src/runtime/nodes.rs:280` |
| sym-6bb46773032fd2641eff | `next` | function | Advances the local evidence epoch after an observed authorization change or an explicit source reopen. | `src/capture/authorization.rs:274` |
| sym-6a28ad2e5841da992baa | `next` | function | Returns the generation assigned after explicit rediscovery. | `src/capture/events.rs:18` |
| sym-f033a669e834bb2886e8 | `node` | function | Returns the node held by `ConnectorManifest`. | `src/connector/manifest.rs:140` |
| sym-326b76705931c8f8480d | `node` | function | Returns the node held by `GraphIr`. | `src/graph/ir.rs:47` |
| sym-b0afa83546e4319231fa | `node` | function | Returns the node held by `AsyncOperatorManifest`. | `src/graph/signal/operator.rs:188` |
| sym-192c1a9561eb55f5cadd | `node` | function | Returns the node held by `GraphSpec`. | `src/graph/spec.rs:73` |
| sym-535c2950ab66dcde7852 | `node_configuration` | function | Returns the node configuration held by `EndpointPrepareContext`. | `src/endpoint/runtime.rs:150` |
| sym-14186bbb667595923b39 | `node_configuration` | function | Returns the node configuration held by `PreparedWorkerMapping`. | `src/session/prepare/mappings.rs:258` |
| sym-520efa0c85a457ce848a | `node_count` | function | Returns the node count held by `GraphIr`. | `src/graph/ir.rs:40` |
| sym-4fda4173b78ebd43a378 | `node_count` | function | Returns the node count held by `RuntimePlan`. | `src/graph/plan.rs:135` |
| sym-8f6bfd5768e658b5f70a | `node_count` | function | Returns the node count held by `GraphSpec`. | `src/graph/spec.rs:65` |
| sym-8050fe5c5aca6992ebef | `node_count` | function | Returns the node count held by `CompiledSession`. | `src/session/compile/compiled.rs:47` |
| sym-329a764a024a64e239a7 | `node_index` | function | Returns the node index held by `SessionCompileDiagnostic`. | `src/session/compile/error.rs:117` |
| sym-48d0f42440c7fc7842ea | `node_type_id` | function | Returns the node type identifier held by `SessionCompileDiagnostic`. | `src/session/compile/error.rs:133` |
| sym-5992cbcecadf5a313006 | `node_type_id` | function | Returns the node type identifier held by `EndpointDescriptor`. | `src/session/declaration/endpoint.rs:141` |
| sym-e0729c229c69c441b3d5 | `node_type_id` | function | Returns the node type identifier held by `EndpointSpec`. | `src/session/declaration/spec.rs:179` |
| sym-97f49aec5a4257d6a8a5 | `normalize_timestamp_ns` | function | Returns the normalize timestamp nanoseconds held by `TimelineMapping`. | `src/timing/timeline_mapping.rs:15` |
| sym-67fa3af81d3df0dfbdb2 | `normalized_total` | function | Returns the normalized total held by `SessionAudioReentryMetrics`. | `src/session/lifecycle/observations.rs:348` |
| sym-9a415f567ef1471d0a8e | `observability` | function | Returns the observability associated with `EdgeContract`. | `src/graph/ports.rs:357` |
| sym-b9068cec996a567f13a5 | `observation` | function | Returns the current observation exposed by `RegisteredConnector`. | `src/connector/mod.rs:140` |
| sym-ba67c9ad9110afe18990 | `observation_handle` | function | Returns a handle for reading observations from `SourceRuntimeEventSender`. | `src/capture/events.rs:270` |
| sym-9a6f131ff1bed5bba171 | `observation_handle` | function | Returns a handle for reading observations from `SourceRuntimeEventReceiver`. | `src/capture/events.rs:321` |
| sym-6cafcb762fcee8844383 | `observation_handle` | function | Returns a handle for reading observations from `CapturedFrameSender`. | `src/capture/frame_stream.rs:142` |
| sym-f16b50c61176e14cce08 | `observation_handle` | function | Returns a handle for reading observations from `CapturedFrameStream`. | `src/capture/frame_stream.rs:183` |
| sym-cfee5a73a4a1a0d26432 | `observation_handle` | function | Returns a handle for reading observations from `CaptureObservationCounters`. | `src/capture/observations.rs:107` |
| sym-9e3205058b70597fd267 | `observation_handle` | function | Returns a handle for reading observations from `MacosInputSource`. | `src/capture/platform/macos/input.rs:238` |
| sym-dbd38fdac902eda65ee0 | `observation_handle` | function | Returns a handle for reading observations from `SystemLoopbackSource`. | `src/capture/platform/macos/loopback.rs:267` |
| sym-4afc2da96bf440324791 | `observation_handle` | function | Returns a handle for reading observations from `DesktopCaptureSource`. | `src/capture/platform/macos/mod.rs:96` |
| sym-afe19ea26113b4c497b9 | `observation_handle` | function | Returns a read-only handle to this edge's authoritative live telemetry. | `src/runtime/audio/router.rs:546` |
| sym-310408147965a48c816c | `observation_handle` | function | Returns a handle for reading observations from `PlanSourceSender`. | `src/runtime/audio/runner.rs:181` |
| sym-6cd08650f2afcc43b517 | `observation_receipt` | function | Returns the observation receipt associated with `CaptureOwner`. | `src/capture/capture_owner.rs:260` |
| sym-9047a37d17f8c9a3a47e | `observations` | function | Returns the observations exposed by `CaptureObservationReceipt`. | `src/capture/capture_owner.rs:174` |
| sym-d4d05dfdf43eba23f452 | `observations` | function | Returns the observations exposed by `CaptureOwner`. | `src/capture/capture_owner.rs:256` |
| sym-4b3a171c8c6ecf236523 | `observations` | function | Returns the observations exposed by `SourceRuntimeEventObservationHandle`. | `src/capture/events.rs:205` |
| sym-5aed9a1ce34773b88ead | `observations` | function | Returns the observations exposed by `SourceRuntimeEventSender`. | `src/capture/events.rs:266` |
| sym-984a539d92e5ea942c2d | `observations` | function | Returns the observations exposed by `SourceRuntimeEventReceiver`. | `src/capture/events.rs:317` |
| sym-b74fbfcc413c6394e9a2 | `observations` | function | Returns the observations exposed by `CapturedFrameObservationHandle`. | `src/capture/frame_stream.rs:36` |
| sym-dae44d2785f01b630ae4 | `observations` | function | Returns the observations exposed by `CaptureObservationHandle`. | `src/capture/observations.rs:37` |
| sym-f90860471b2bdeebfdf2 | `observations` | function | Returns the observations exposed by `MacosInputSource`. | `src/capture/platform/macos/input.rs:234` |
| sym-838a00051f8d286c3fe1 | `observations` | function | Returns the observations exposed by `SystemLoopbackSource`. | `src/capture/platform/macos/loopback.rs:248` |
| sym-73e229943d6a6eee5b3d | `observations` | function | Returns the observations exposed by `DesktopCaptureSource`. | `src/capture/platform/macos/mod.rs:82` |
| sym-4a2c8867bce1f05a18e1 | `observations` | function | Returns the observations exposed by `RegisteredConnector`. | `src/connector/mod.rs:153` |
| sym-9d0cf1b3fe4d1c6b600a | `observations` | function | Snapshots the bounded edge counters for this endpoint input. | `src/endpoint/contract.rs:138` |
| sym-adfa1bc7f0448364704c | `observations` | function | Returns the observations exposed by `PolledAudioReceipt`. | `src/endpoint/polled_audio_driver.rs:168` |
| sym-697f8ecef5dd23861a71 | `observations` | function | Returns the observations exposed by `MultistemRecording`. | `src/recording/writer.rs:255` |
| sym-f6c711b3f4fc31ea76e8 | `observations` | function | Returns the observations exposed by `RealtimePlanExecutor`. | `src/runtime/audio/executor.rs:188` |
| sym-fa7f6685167498e25c63 | `observations` | function | Returns a point-in-time snapshot of the edge's live observations. | `src/runtime/audio/router.rs:237` |
| sym-8425b7ab8ed18899aa59 | `observations` | function | Returns the observations exposed by `PlanEdgeReceiver`. | `src/runtime/audio/router.rs:655` |
| sym-bb909f68599f6929350a | `observations` | function | Returns the observations exposed by `PlanEdgeRouter`. | `src/runtime/audio/router.rs:881` |
| sym-95e0883cf9cfcb07662e | `observations` | function | Returns the observations exposed by `PlanSourceObservationHandle`. | `src/runtime/audio/runner.rs:143` |
| sym-ac1f679ae7e9f2fabe14 | `observations` | function | Returns the observations exposed by `PlanSourceSender`. | `src/runtime/audio/runner.rs:177` |
| sym-34f80ad71a31d7e9eef6 | `observations` | function | Returns the observations exposed by `PlanSourceInput`. | `src/runtime/audio/runner.rs:200` |
| sym-5505b9acd1f4733a3e68 | `observations` | function | Returns the observations exposed by `SidecarHost`. | `src/runtime/lifecycle/sidecar_host.rs:261` |
| sym-93216e72854b50f42707 | `observations` | function | Returns the observations exposed by `AsyncOperatorWorker`. | `src/runtime/signal/operator.rs:921` |
| sym-4c663c8c64bc960e342b | `observations` | function | Returns the observations exposed by `AudioInputWriter`. | `src/session/extensions/audio_input/buffer.rs:246` |
| sym-958cba61d7f8fc6b936e | `observations` | function | Returns the observations exposed by `AudioInput`. | `src/session/extensions/audio_input/mod.rs:131` |
| sym-a44b57485e977f0e7bbb | `observations` | function | Returns the observations exposed by `SourceRuntime`. | `src/session/extensions/source.rs:579` |
| sym-4475a719b62bcedfe19e | `observations` | function | Returns the observations exposed by `SessionEventReceiver`. | `src/session/lifecycle/events.rs:517` |
| sym-5f3a9e2a447883ad6321 | `observe` | function | Returns the current observation exposed by `CapturePermissionLifecycle`. | `src/capture/authorization.rs:204` |
| sym-992b257f18f1c4cc5298 | `observe` | function | Returns the current observation exposed by `SignalContinuityTracker`. | `src/graph/signal/continuity.rs:18` |
| sym-5e431e856805d2224f11 | `observe` | function | Returns the current observation exposed by `ClockDriftEstimator`. | `src/timing/clock_drift.rs:35` |
| sym-89e25f10fc9dcc43465d | `observe_callback_buffer` | function | Records an observation for callback buffer for `CaptureObservationCounters`. | `src/capture/observations.rs:52` |
| sym-ca5cadedf3e7bf840bbb | `observe_complete_snapshot` | function | Records an observation for complete snapshot for `SourceLifecycleRegistry`. | `src/capture/lifecycle_registry.rs:36` |
| sym-1db6dab530aa274c6481 | `observe_dispatch_queue_full` | function | Records an observation for dispatch queue full for `CaptureObservationCounters`. | `src/capture/observations.rs:70` |
| sym-898836c5e54b77ef4603 | `observe_dispatch_queue_full_frames` | function | Records a known number of frames lost at a bounded native or Rust delivery edge. | `src/capture/observations.rs:76` |
| sym-40816db4cd81050d5521 | `observe_enqueued_frame` | function | Records an observation for enqueued frame for `CaptureObservationCounters`. | `src/capture/observations.rs:58` |
| sym-a17aa53905e405582ed0 | `observe_oversized_buffer` | function | Records an observation for oversized buffer for `CaptureObservationCounters`. | `src/capture/observations.rs:89` |
| sym-f4d752494107139e9afa | `observe_pool_exhaustion` | function | Records an observation for pool exhaustion for `CaptureObservationCounters`. | `src/capture/observations.rs:64` |
| sym-1d38577ec6acc760dbf9 | `observe_stream_error` | function | Records an observation for stream error for `CaptureObservationCounters`. | `src/capture/observations.rs:95` |
| sym-91a3331001dec71cad22 | `observe_timestamp_epoch_clamp` | function | Records an observation for timestamp epoch clamp for `CaptureObservationCounters`. | `src/capture/observations.rs:101` |
| sym-12dc62d0bd0885d8fc00 | `observed` | function | Creates observed signal timing for `SignalTiming`. | `src/graph/signal/timing.rs:38` |
| sym-074b05e12d2a3ea9963e | `observed_timestamp_ns` | function | Returns the observed timestamp nanoseconds held by `SignalTiming`. | `src/graph/signal/timing.rs:75` |
| sym-1f0d61c4d674829cdd06 | `ok` | function | Creates a successful status value for `PksSessionStatus`. | `src/abi/session/abi.rs:62` |
| sym-527e677b3cfffbeaf56a | `open` | function | Opens the resource represented by `PreparedCapture`. | `src/capture/capture_owner.rs:128` |
| sym-3887d294de2eb7775f03 | `open` | function | Opens the resource represented by `CaptureDeliveryStartGateController`. | `src/capture/frame_stream.rs:77` |
| sym-5c090a7a6ea0f2676aff | `open_metadata` | function | Returns the open metadata associated with `CaptureOwner`. | `src/capture/capture_owner.rs:242` |
| sym-19f26353af4e287d1753 | `operation` | function | Returns the operation associated with `SessionControlFailure`. | `src/session/lifecycle/events.rs:93` |
| sym-7ee45fd268c719853a1f | `operator` | function | Declares exactly one operator instance. Connect streams to named inputs and select named outputs through the returned Session-scoped handle. | `src/lib.rs:460` |
| sym-261781ef50f8b1fd6907 | `operator` | function | Declares exactly one Session-owned operator instance. | `src/session/declaration/draft.rs:383` |
| sym-29517160d501c6b59612 | `operator` | function | Returns the operator associated with `SessionMetricsSnapshot`. | `src/session/lifecycle/observations.rs:103` |
| sym-fafe5a7548a83f07ec0a | `operator_count` | function | Returns the operator count held by `SessionMetricsSnapshot`. | `src/session/lifecycle/observations.rs:99` |
| sym-04507f8b0309ee95a3c7 | `operator_finalization_failures_total` | function | Returns the operator finalization failures total held by `SessionStopOutcome`. | `src/session/lifecycle/control.rs:367` |
| sym-24b00937acc05b71f6db | `operator_generation` | function | Returns the operator generation associated with `SignalDerivation`. | `src/graph/signal/lineage.rs:150` |
| sym-1786112a976fa4da87b7 | `operator_id` | function | Returns the operator identifier held by `ConnectorManifest`. | `src/connector/manifest.rs:132` |
| sym-8ff73184ea1f85f0b753 | `operator_id` | function | Returns the operator identifier held by `SignalDerivation`. | `src/graph/signal/lineage.rs:144` |
| sym-9fdae1b4561a92b38e0c | `operator_id` | function | Returns the operator identifier held by `AsyncOperatorManifest`. | `src/graph/signal/operator.rs:176` |
| sym-03648afae92dab918e48 | `operator_id` | function | Returns the operator identifier held by `SessionCompileDiagnostic`. | `src/session/compile/error.rs:125` |
| sym-ff8a7407275bbdf7c2ea | `operator_id` | function | Returns the operator identifier held by `Operator`. | `src/session/declaration/draft.rs:295` |
| sym-cadee6ac3abd3d6102a5 | `operator_id` | function | Returns the operator identifier held by `EndpointDescriptor`. | `src/session/declaration/endpoint.rs:145` |
| sym-6b854d00163b92b0c277 | `operator_id` | function | Returns the operator identifier held by `EndpointSpec`. | `src/session/declaration/spec.rs:183` |
| sym-8da8fd9807769380a1d7 | `operator_id` | function | Returns the operator identifier held by `OperatorInstanceSpec`. | `src/session/declaration/spec.rs:249` |
| sym-20523cd313ea1458a503 | `operator_id` | function | Returns the operator identifier held by `TypedOperator`. | `src/session/declaration/typed_stream.rs:71` |
| sym-c27986ca2f030b2e909f | `operator_instance_id` | function | Returns the operator instance identifier held by `SessionCompileDiagnostic`. | `src/session/compile/error.rs:129` |
| sym-15a41ed6e47d9bcbd5dd | `operator_instance_id` | function | Returns the operator instance identifier held by `DerivedStreamHandle`. | `src/session/declaration/draft.rs:1016` |
| sym-0a5251932704664d3e60 | `operator_instance_id` | function | Returns the operator instance identifier held by `SessionAudioReentryMetrics`. | `src/session/lifecycle/observations.rs:304` |
| sym-482722bf897c1451962b | `operator_mappings` | function | Returns the operator mappings associated with `PreparedSession`. | `src/session/prepare/prepared.rs:56` |
| sym-c1b45b0e3b91ee07993d | `operator_metrics` | function | Returns one finalizable observation handle per Session-owned operator instance, including exact per-input-port edge counters. | `src/lib.rs:847` |
| sym-ba51b3dc2b3ebb67e7e3 | `operator_metrics` | function | Returns the operator metrics held by `RunningSession`. | `src/session/lifecycle/running.rs:211` |
| sym-a4a72750d0738fe4fc9b | `operator_revision` | function | Returns the operator revision held by `SignalDerivation`. | `src/graph/signal/lineage.rs:147` |
| sym-3d32e9f8f65c96eefc79 | `operators` | function | Returns the operators associated with `SessionSpec`. | `src/session/declaration/spec.rs:343` |
| sym-af5e48f013d42753aaaf | `opus_config` | function | Returns the opus config held by `StreamProfile`. | `src/codec/profile.rs:73` |
| sym-dfcacf7bacb5a454f532 | `origin` | function | Returns the origin held by `EndpointRouteContext`. | `src/endpoint/runtime.rs:84` |
| sym-aa50e22c859bc41e18b5 | `origin` | function | Returns the origin held by `ConnectionSpec`. | `src/session/declaration/spec.rs:263` |
| sym-e8cf98b0bcffc28ac74f | `origin` | function | Returns the origin held by `ClockDomainDescriptor`. | `src/timing/domain.rs:39` |
| sym-e3c6eb1de0ca96234dfc | `out` | function | Selects a named output port from `NodeHandle`. | `src/graph/dsl.rs:18` |
| sym-8a479e9bb8a8ea99b512 | `outcome` | function | Returns the outcome held by `SessionCancelResult`. | `src/lib.rs:1133` |
| sym-7e020e70edafdd0460bf | `outcome` | function | Returns the outcome held by `SessionStopResult`. | `src/lib.rs:1153` |
| sym-8fc7f17a4a5ce92f7008 | `outcome` | function | Returns the outcome held by `SessionRecordingReceipt`. | `src/session/extensions/recording.rs:36` |
| sym-e72f307e0a3e8bfb016c | `outcome` | function | Returns the outcome held by `SessionTraceRecorder`. | `src/session/lifecycle/trace.rs:243` |
| sym-46184da0e5f6844c22d9 | `outcome` | function | Returns the outcome held by `SessionTrace`. | `src/session/lifecycle/trace.rs:276` |
| sym-1e848a89312484f725a3 | `output` | function | Returns the output held by `OperatorInstanceHandle`. | `src/session/declaration/draft.rs:732` |
| sym-bbf6442f1b447ba134e3 | `output` | function | Returns the output held by `SourceInstanceHandle`. | `src/session/declaration/draft.rs:854` |
| sym-3ab351fd3a63820a3179 | `output` | function | Returns the output held by `DerivedStreamHandle`. | `src/session/declaration/draft.rs:1024` |
| sym-b3b8b7cbc2207c5d5995 | `output` | function | Returns the output held by `AudioInput`. | `src/session/extensions/audio_input/mod.rs:107` |
| sym-1e3bd4d53be1938eacb2 | `output` | function | Returns the output held by `PcmSource`. | `src/session/extensions/audio_input/source.rs:56` |
| sym-3454febcc9704bc56791 | `output` | function | Returns the output held by `SourceSessionContext`. | `src/session/extensions/source.rs:242` |
| sym-e68babbafd6f6b97672c | `output_edge` | function | Returns the output edge associated with `AsyncOperatorManifest`. | `src/graph/signal/operator.rs:196` |
| sym-5c2537e0f0e1194396e1 | `output_pool_exhaustions` | function | Returns the output pool exhaustions associated with `MixerTelemetry`. | `src/runtime/nodes.rs:263` |
| sym-90d3ffa48560606eaf27 | `output_port` | function | Returns the output port held by `SourceOutputHandle`. | `src/session/declaration/draft.rs:935` |
| sym-4b938d8c386fac8b05ab | `output_port` | function | Returns the output port held by `DerivedStreamHandle`. | `src/session/declaration/draft.rs:1020` |
| sym-a944e0ff9385d1203678 | `output_port` | function | Returns the output port held by `SourceOutputSpec`. | `src/session/declaration/spec.rs:141` |
| sym-147483776f9a02d73781 | `output_port` | function | Returns the output port held by `TypedOperator`. | `src/session/declaration/typed_stream.rs:79` |
| sym-8dfdc8dd3a09c0f1b609 | `output_port` | function | Returns the output port held by `SourceManifest`. | `src/session/extensions/source.rs:217` |
| sym-5f7590d23635def1696e | `output_ports` | function | Returns the output ports held by `AsyncOperatorManifest`. | `src/graph/signal/operator.rs:231` |
| sym-ee093c39da623635f4ef | `output_roles` | function | Returns the output roles associated with `AsyncOperatorManifest`. | `src/graph/signal/operator.rs:220` |
| sym-38498759647f8a8f4471 | `output_root` | function | Returns the output root associated with `SessionMultistemEndpointCoordinator`. | `src/recording/endpoint.rs:65` |
| sym-af99ed72a6b2cc205f2e | `output_spec` | function | Returns the output spec held by `TypedOperator`. | `src/session/declaration/typed_stream.rs:87` |
| sym-8417dc284f756364cbd0 | `outputs` | function | Returns the outputs associated with `NodeDescriptor`. | `src/graph/node.rs:234` |
| sym-48cad7900de712e74f64 | `outputs` | function | Returns the outputs associated with `AsyncOperatorPrepareContext`. | `src/graph/signal/preparation.rs:66` |
| sym-0265bfc696710c57e223 | `outputs` | function | Returns the outputs associated with `SourceManifest`. | `src/session/extensions/source.rs:170` |
| sym-72c06415474a4ff99674 | `overrun_count` | function | Returns the overrun count held by `BridgeSinkTelemetry`. | `src/runtime/nodes.rs:165` |
| sym-4ccedf87e32e9774b79a | `package_version` | function | Returns the package version held by `ConnectorManifest`. | `src/connector/manifest.rs:136` |
| sym-bcde5bc10f8c38ef38b0 | `partition` | function | Returns the partition associated with `RuntimePlan`. | `src/graph/plan.rs:140` |
| sym-33ac3ff52504ab1aa3eb | `path` | function | Returns the path associated with `NativeExtensionLibraryError`. | `src/native_extension/mod.rs:139` |
| sym-a2d755aefd3247e4069b | `payload` | function | Returns the payload associated with `SignalEnvelope`. | `src/graph/signal/envelope.rs:62` |
| sym-5fa61a47369bd239fced | `payload_size_bytes` | function | Returns the payload size bytes held by `SignalEnvelope`. | `src/graph/signal/envelope.rs:66` |
| sym-074b3b44d431ba5ffca9 | `pcm_source` | function | Declares the low-level bounded PCM source and returns its explicit Session handles and producer writer ownership. | `src/lib.rs:404` |
| sym-c3632754280e719cb8a0 | `permission` | function | Returns the permission associated with `AsyncOperatorManifest`. | `src/graph/signal/operator.rs:204` |
| sym-1d87a0ad18fa6d4f87c4 | `permission_epoch` | function | Returns the permission epoch held by `CapturePermissionLifecycle`. | `src/capture/authorization.rs:200` |
| sym-140f0d05874e36bc8ab1 | `permission_epoch` | function | Returns the permission epoch held by `FrameLineage`. | `src/frame/lineage.rs:83` |
| sym-221910aa2a407a20b2a6 | `plan` | function | Lower a verified IR into an execution-ready plan. | `src/graph/compile/plan.rs:24` |
| sym-0021c29402fd0b2670af | `plan_edge_observation_handle` | function | Plans edge observation handle for `EndpointAudioReceiver`. | `src/endpoint/contract.rs:147` |
| sym-6d77239070a4c75eb2a1 | `planned_edge_count` | function | Returns the planned edge count held by `CompiledSession`. | `src/session/compile/compiled.rs:57` |
| sym-221e5a8050de5301cd79 | `pocketstation::capture::capture_owner::join_capture_worker` | function | Joins one owned capture worker and preserves panic as a typed failure. | `src/capture/capture_owner.rs:334` |
| sym-1b3e57d59e646040c389 | `pocketstation::capture::capture_owner::prepare_capture` | function | Prepares a bounded capture owner without starting native delivery. | `src/capture/capture_owner.rs:298` |
| sym-94f34460ef980ae89e34 | `pocketstation::capture::capture_owner::prepare_capture_with_start_gate` | function | Prepares a bounded capture owner behind a caller-owned one-way start gate. | `src/capture/capture_owner.rs:306` |
| sym-179581e8a2aa8587517b | `pocketstation::capture::events::publish_backend_failure` | function | Publishes one exact post-open backend failure without introducing another event queue or worker. | `src/capture/events.rs:280` |
| sym-56233572de3bb3ef20fe | `pocketstation::capture::events::source_runtime_event_channel` | function | Creates the bounded sender and receiver used for source runtime events. | `src/capture/events.rs:328` |
| sym-ecab0c07b346f1031392 | `pocketstation::capture::frame_stream::capture_delivery_start_gate` | function | Creates a closed Session-owned controller and callback-visible start gate. | `src/capture/frame_stream.rs:83` |
| sym-bbc1cfea18e83b9a7588 | `pocketstation::capture::frame_stream::captured_frame_stream` | function | Wraps the supplied capture receiver as a stream of captured frames. | `src/capture/frame_stream.rs:191` |
| sym-d53722dc6aaa613337d8 | `pocketstation::capture::platform::macos::input::discover_input_sources_native` | function | Discovers microphone input sources through the native macOS backend. | `src/capture/platform/macos/input.rs:263` |
| sym-140a3f747734f8e81fc1 | `pocketstation::capture::platform::macos::macos_tap::discover_sources_native` | function | Enumerate all running processes that have audio output. Returns an empty `Vec` on macOS < 14.4 (public support floor) or on non-macOS platforms. | `src/capture/platform/macos/macos_tap.rs:87` |
| sym-ade1e9ee6092e84a7633 | `pocketstation::capture::platform::macos::macos_tap::tap_available` | function | Returns `true` when the CoreAudio process tap API is available. | `src/capture/platform/macos/macos_tap.rs:76` |
| sym-99ff6b9e702e2b18cb4f | `pocketstation::capture::query::application_capture_available` | function | Reports whether this host exposes the native application-capture facility. | `src/capture/query.rs:64` |
| sym-6c41c9513e4c1cd3f646 | `pocketstation::capture::query::discover_sources` | function | Discovers capture sources available from the local provider. | `src/capture/query.rs:85` |
| sym-f5b166160c26c3ff3463 | `pocketstation::capture::query::resolve_query` | function | Filters discovered capture sources using the supplied source query. | `src/capture/query.rs:40` |
| sym-f02c42cdfe82b6305eaf | `pocketstation::capture::timeline::initialize_monotonic_timestamp_domain` | function | Initializes the process-wide capture timestamp domain from a setup thread. | `src/capture/timeline.rs:11` |
| sym-23bb2f2bb9eacdded34f | `pocketstation::capture::timeline::monotonic_timestamp_ns` | function | Process-wide monotonic timestamp domain used by every capture adapter. The value is non-zero and comparable across PocketStation crates in the same process; it is never derived from a wall clock and cannot jump. | `src/capture/timeline.rs:18` |
| sym-1ccfa4db4cee61149d1c | `pocketstation::conformance::observed_browser` | function | Declares and registers a deterministic native browser boundary used only by cross-language conformance harnesses. | `src/conformance.rs:334` |
| sym-0c9d60e96581458a54c8 | `pocketstation::conformance::observed_connector` | function | Declares and registers a deterministic native connector used only by cross-language conformance harnesses. | `src/conformance.rs:273` |
| sym-c11e32fc095e55792358 | `pocketstation::conformance::run_extension_vector` | function | Executes the neutral typed Source -> `Stream<T>` -> Operator -> Endpoint vector through the public Session. | `src/conformance.rs:1005` |
| sym-c3cea804c52f4d529e33 | `pocketstation::conformance::session` | function | Runs the conformance assertions for the Session contract. | `src/conformance.rs:198` |
| sym-b998990d7f590b8e30fc | `pocketstation::conformance::session_for_saturation` | function | Creates a finite fixture that produces enough frames to overflow a deliberately unconsumed route. | `src/conformance.rs:204` |
| sym-53b02d37ebfdede8cf98 | `pocketstation::conformance::session_with_recording` | function | Creates the deterministic Session fixture with multistem recording. | `src/conformance.rs:209` |
| sym-2738a58edbfacaab1570 | `pocketstation::conformance::session_with_recording_and_trace` | function | Creates the deterministic Session fixture with both aligned multistem recording and a bounded Session diagnostic trace. | `src/conformance.rs:230` |
| sym-66e12b5fcbeb0e2033a5 | `pocketstation::conformance::session_with_trace` | function | Creates the deterministic Session fixture with a bounded diagnostic trace. | `src/conformance.rs:216` |
| sym-99d61fd60fa66abaf3db | `pocketstation::connector::sidecar::sidecar_connector_factory` | function | Creates a connector driver factory backed by the supplied sidecar process. | `src/connector/sidecar.rs:264` |
| sym-874507cffde971fe579a | `pocketstation::graph::builtins::register_builtins` | function | Registers the passthrough, gain, and mono-mix node factories in the supplied registry. | `src/graph/builtins.rs:220` |
| sym-995d8497f83ddee379cb | `pocketstation::microphone_permission_observation` | function | Reads the current microphone authorization state without prompting. | `src/lib.rs:55` |
| sym-31cc3e49969412418e61 | `pocketstation::recording::error_code::recording_outcome_error_code` | function | Returns the recording outcome error code held by `error_code`. | `src/recording/error_code.rs:82` |
| sym-a13fb874065b9f7145f0 | `pocketstation::runtime::audio::runner::plan_source_channel` | function | Plans source channel for `runner`. | `src/runtime/audio/runner.rs:229` |
| sym-6a86a673b89a34b78d71 | `pocketstation::runtime::nodes::register_runtime_nodes` | function | Registers runtime nodes for `nodes`. | `src/runtime/nodes.rs:43` |
| sym-ae934e93f5b4bf2d9646 | `pocketstation::session::error_code::polled_audio_poll_error_code` | function | Returns the polled audio poll error code held by `error_code`. | `src/session/error_code.rs:255` |
| sym-a1cac35222f254f15699 | `pocketstation::session::error_code::session_declaration_error_code` | function | Returns the session declaration error code held by `error_code`. | `src/session/error_code.rs:195` |
| sym-4118e94726a1cbad9439 | `pocketstation::session::error_code::session_start_failure_code` | function | Returns the session start failure code held by `error_code`. | `src/session/error_code.rs:225` |
| sym-66f5ae8654f7c9e7be54 | `pocketstation::session::error_code::session_stop_failure_codes` | function | Returns every stable failure code carried by a Session stop result. | `src/session/error_code.rs:265` |
| sym-0166a3fa054747f57db5 | `pocketstation::session::extensions::builtins::register_session_graph_nodes` | function | Registers session graph nodes for `builtins`. | `src/session/extensions/builtins.rs:36` |
| sym-7370721b4b1c5034e352 | `pocketstation::session::lifecycle::running::start_prepared_session` | function | Starts prepared session for `running`. | `src/session/lifecycle/running.rs:627` |
| sym-aa72725f34d34f369c69 | `pocketstation::session::lifecycle::running::start_prepared_session_cancellable` | function | Starts prepared session cancellable for `running`. | `src/session/lifecycle/running.rs:643` |
| sym-aa95b346f153eb1819a4 | `pocketstation::session::prepare::prepare_session_runtime` | function | Prepares session runtime for `prepare`. | `src/session/prepare/mod.rs:33` |
| sym-f0bb36b243ff1edb3d1e | `pocketstation::timing::domain::describe_clock_domain` | function | Describes the stable semantics Core can assert for a clock-domain ID. | `src/timing/domain.rs:54` |
| sym-50908e7003d52b4da731 | `pocketstation::timing::monotonic_timestamp_ns` | function | Process-wide monotonic timestamp domain shared by capture, routing, and destination workers. | `src/timing/mod.rs:32` |
| sym-7db5666d2dff33bc81b5 | `policy_epoch` | function | Returns the policy epoch associated with `SignalLineage`. | `src/graph/signal/lineage.rs:80` |
| sym-2a370db0288245b8728d | `polled_at_ns` | function | Returns the polled at nanoseconds held by `PolledAudioFrame`. | `src/endpoint/polled_audio_driver.rs:285` |
| sym-a83f30c5e265141f4c02 | `polled_audio` | function | Declares a bounded polled-audio endpoint on `Session`. | `src/lib.rs:501` |
| sym-5c799578e333bdbbb717 | `polled_audio` | function | Declares a bounded polled-audio endpoint on `Session`. | `src/session/extensions/polled_audio.rs:14` |
| sym-8838bcb7e66cd00374af | `polled_audio` | function | Declares a bounded polled-audio endpoint on `SessionMetricsSnapshot`. | `src/session/lifecycle/observations.rs:71` |
| sym-8a12e2f5e580f899d681 | `polled_audio_receipt` | function | Returns the polled audio receipt associated with `SessionEngineHost`. | `src/session/lifecycle/host.rs:99` |
| sym-c3150cbea2522c278540 | `polled_audio_receipts_total` | function | Returns the polled audio receipts total held by `SessionEngineHost`. | `src/session/lifecycle/host.rs:104` |
| sym-cd281aeca19791b5ed7b | `pool_exhausted_total` | function | Returns the pool exhausted total held by `SessionAudioReentryMetrics`. | `src/session/lifecycle/observations.rs:360` |
| sym-4bef940acb4d63a5dfd1 | `pool_slots` | function | Returns the pool slots associated with `SessionAudioReentryMetrics`. | `src/session/lifecycle/observations.rs:336` |
| sym-79e7f64b188402325901 | `port_name` | function | Returns the port name held by `ConnectorAudioRecord`. | `src/connector/transport.rs:335` |
| sym-f510b473b8ca98c83601 | `port_name` | function | Returns the port name held by `ConnectorInputDescriptor`. | `src/connector/worker/driver.rs:40` |
| sym-061986f6042d31f2d0cb | `port_name` | function | Returns the port name held by `EndpointPortInput`. | `src/endpoint/contract.rs:233` |
| sym-0321107d9ee385b6a2d9 | `port_name` | function | Returns the port name held by `PortPrepareContext`. | `src/graph/node.rs:341` |
| sym-296bb14d04a7c41b60b7 | `port_name` | function | Returns the port name held by `SessionCompileDiagnostic`. | `src/session/compile/error.rs:141` |
| sym-5479d5ffadb2d4d88a5d | `preparation_group` | function | Returns the preparation group associated with `SidecarConnectorDriverFactory`. | `src/connector/sidecar.rs:39` |
| sym-ccf25713d1363e17c791 | `preparation_group` | function | Returns the preparation group associated with `SessionMultistemEndpointCoordinator`. | `src/recording/endpoint.rs:82` |
| sym-036fa6b33dbbdd9ec6b7 | `prepare` | function | Prepares resources required by `DesktopCaptureBackend`. | `src/capture/platform/macos/session_backend.rs:22` |
| sym-90bc1b303a52614edb58 | `prepare` | function | Prepares resources required by `SidecarConnectorDriverFactory`. | `src/connector/sidecar.rs:49` |
| sym-1b072df2e8c199b88812 | `prepare` | function | Prepares resources required by `PassthroughNode`. | `src/graph/builtins.rs:98` |
| sym-45fe39b372136f053d22 | `prepare` | function | Prepares resources required by `GainNode`. | `src/graph/builtins.rs:154` |
| sym-f4df6d2fb38479a041f3 | `prepare` | function | Prepares resources required by `MonoMixNode`. | `src/graph/builtins.rs:197` |
| sym-bb5d5fe39646ae5bb11c | `prepare` | function | Prepares resources required by `SessionMultistemEndpointCoordinator`. | `src/recording/endpoint.rs:99` |
| sym-4ddbee20aa1a4b758d90 | `prepare` | function | Prepares resources required by `MixerSourceNode`. | `src/runtime/nodes.rs:432` |
| sym-c9583eab2b616f3a7638 | `prepare` | function | Prepares resources required by `SourceRegistry`. | `src/session/extensions/source.rs:339` |
| sym-b7217c287151cbf6ec09 | `prepare_and_spawn_from_plan_edge` | function | Prepares and spawn from plan edge for `AsyncOperatorWorker`. | `src/runtime/signal/operator.rs:764` |
| sym-e5da18cae03b5535d94e | `prepare_context` | function | Returns the immutable preparation context retained by `PreparedWorkerMapping`. | `src/session/prepare/mappings.rs:268` |
| sym-2636dfda1a2aebb8a995 | `prepare_session` | function | Builds the source preparation context for the current Session through `SourceRegistry`. | `src/session/extensions/source.rs:353` |
| sym-56c5be58e62453b0b40a | `probe_interval` | function | Returns the probe interval held by `ConnectorReadinessPolicy`. | `src/connector/readiness.rs:47` |
| sym-dff6c45f96347045eb76 | `process` | function | Processes an input value through `SidecarConnectorDriverFactory`. | `src/connector/sidecar.rs:33` |
| sym-cb61f77e7c88e858fa6f | `process` | function | Processes an input value through `PassthroughNode`. | `src/graph/builtins.rs:102` |
| sym-23912e56afc7e14ab0f3 | `process` | function | Processes an input value through `GainNode`. | `src/graph/builtins.rs:158` |
| sym-af499cf346452c91c428 | `process` | function | Processes an input value through `MonoMixNode`. | `src/graph/builtins.rs:201` |
| sym-038e30bec596bf39134c | `process` | function | Processes an input value through `MixerSourceNode`. | `src/runtime/nodes.rs:441` |
| sym-017c2deddf84860ec046 | `process_id` | function | Returns the process identifier held by `ApplicationSelector`. | `src/session/declaration/selector.rs:48` |
| sym-5e6165f795a91997033b | `process_instance` | function | Creates `ApplicationSelector` for one exact process instance. | `src/session/declaration/selector.rs:52` |
| sym-fcd72944dcac0c4e4af4 | `process_ready` | function | Processes the ready inputs for `RealtimePlanRunner`. | `src/runtime/audio/runner.rs:338` |
| sym-f3e3f6bd39d9945c426b | `process_tree_scope` | function | Reports the native process boundary represented by this discovery result without making the CLI reconstruct a private capture mode. | `src/capture/identity.rs:140` |
| sym-1175eaebd4ae1636db04 | `process_tree_scope` | function | Reports the process boundary requested from the native backend. | `src/capture/selection.rs:55` |
| sym-948e37a429ffcad3e1f3 | `publish` | function | Publishes its owned operation for `TypedEdgeFanout`. | `src/runtime/signal/edge.rs:308` |
| sym-a6c64d37c7096431b619 | `query::SourceProvider::discover` | function | Discovers the resources visible to `SourceProvider`. | `src/capture/query.rs:49` |
| sym-4e088039148f4ee6cbcb | `queue_capacity_frames` | function | Returns the queue capacity frames held by `AsyncOperatorManifest`. | `src/graph/signal/operator.rs:200` |
| sym-0950908c474b942f3c45 | `queue_capacity_signals` | function | Returns the queue capacity signals associated with `SessionAudioReentryMetrics`. | `src/session/lifecycle/observations.rs:312` |
| sym-bac6ece3e92be8707ce4 | `queue_depth_signals` | function | Returns the queue depth signals associated with `SessionAudioReentryMetrics`. | `src/session/lifecycle/observations.rs:316` |
| sym-f5823cbd74883be3ae32 | `queue_peak_signals` | function | Returns the queue peak signals associated with `SessionAudioReentryMetrics`. | `src/session/lifecycle/observations.rs:320` |
| sym-d39bc3b5e4947ef45350 | `rank` | function | Priority rank for scheduling: lower = higher priority. | `src/graph/partition.rs:60` |
| sym-b1f93c6ba89bb907ab5e | `rank` | function | Returns the rank associated with `EdgeObservabilityLevel`. | `src/graph/ports.rs:301` |
| sym-4592a971dec5eea41198 | `read` | function | Reads the persisted representation of `SessionTrace`. | `src/session/lifecycle/trace.rs:262` |
| sym-e60f163a42c86e627c15 | `readiness` | function | Returns the readiness held by `ConnectorManifest`. | `src/connector/manifest.rs:148` |
| sym-14159ada749817248906 | `readiness_reason_code` | function | Returns the readiness reason code held by `ConnectorServiceStatus`. | `src/connector/status.rs:54` |
| sym-f2823e099d189909ba5f | `realtime_audio` | function | Generic realtime PCM edge. Concrete sample rate, frame size, and channel layout are negotiated from connected ports. | `src/graph/ports.rs:391` |
| sym-5063d668bb96d3521fd8 | `receipt` | function | Returns the receipt associated with `PolledAudioEndpoint`. | `src/endpoint/polled_audio.rs:30` |
| sym-9225c439dda86e4640fc | `receipt` | function | Returns the receipt associated with `SessionMultistemEndpointCoordinator`. | `src/recording/endpoint.rs:74` |
| sym-8bbb0672c00800f6977a | `receive_sidecar_signal` | function | Receives sidecar signal for `RunningSession`. | `src/lib.rs:875` |
| sym-b4c046053df265a1292b | `receive_sidecar_signal` | function | Receives sidecar signal for `RunningSession`. | `src/session/lifecycle/running.rs:283` |
| sym-1c56cd4861b4e8542db3 | `receive_signal` | function | Receives and decodes the next signal message from `SidecarHost`. | `src/runtime/lifecycle/sidecar_host.rs:307` |
| sym-15772bae169ea1388101 | `receiver` | function | Returns the receiver held by `EndpointPortInput`. | `src/endpoint/contract.rs:253` |
| sym-25bbff12e01c4cc43f06 | `receiver_observations` | function | Returns the receiver observations held by `PreparedWorkerMapping`. | `src/session/prepare/mappings.rs:263` |
| sym-8cab603b282bab9ebadb | `record` | function | Attaches recording output to `StemHandle`. | `src/session/extensions/recording.rs:64` |
| sym-1e7dcb7c3751c9ee9647 | `record` | function | Attaches recording output to `SourceOutputHandle`. | `src/session/extensions/recording.rs:83` |
| sym-cf9795799631aa59db4d | `record_discontinuity` | function | Increments the discontinuity observation recorded by `ConnectorContext`. | `src/connector/worker/coordination.rs:126` |
| sym-4db69bd3b958653d5a39 | `record_failure` | function | Records a connector failure and its retry classification in `ConnectorContext`. | `src/connector/worker/coordination.rs:134` |
| sym-56b7d6a6b13ac312c206 | `record_frame_delivered` | function | Records frame delivered for `ConnectorContext`. | `src/connector/worker/coordination.rs:118` |
| sym-5fdc554f555394e3829a | `record_frame_dropped` | function | Records frame dropped for `ConnectorContext`. | `src/connector/worker/coordination.rs:122` |
| sym-fef19c653c5b665bfdb1 | `record_frame_received` | function | Records frame received for `ConnectorContext`. | `src/connector/worker/coordination.rs:114` |
| sym-a45c024f475a58d1b180 | `record_retry` | function | Increments the retry-attempt observation recorded by `ConnectorContext`. | `src/connector/worker/coordination.rs:130` |
| sym-fc64a1cf170802033e63 | `recording_outcome` | function | Returns the recording outcome held by `RunningSession`. | `src/lib.rs:825` |
| sym-c132583ec70d77a8e127 | `recording_receipt` | function | Returns the recording receipt associated with `SessionEngineHost`. | `src/session/lifecycle/host.rs:108` |
| sym-cca63f57bd7b078ef547 | `recording_receipts_total` | function | Returns the recording receipts total held by `SessionEngineHost`. | `src/session/lifecycle/host.rs:113` |
| sym-50fb93fb5e848d46ba51 | `recording_root` | function | Configures the artifact root used by declared multistem recording routes. | `src/lib.rs:296` |
| sym-cae763390c1c1afbf26b | `records` | function | Returns the records associated with `SessionTrace`. | `src/session/lifecycle/trace.rs:272` |
| sym-e929ec8f8bd8d46681a3 | `recovery` | function | Returns the recovery held by `ConnectorServiceStatus`. | `src/connector/status.rs:50` |
| sym-4e9f9c99a9cb2e52f040 | `recovery_reason_code` | function | Returns the recovery reason code held by `ConnectorServiceStatus`. | `src/connector/status.rs:62` |
| sym-0ad1eada52eee6784c16 | `recv` | function | Receives the next value from `EndpointSignalReceiver`. | `src/endpoint/contract.rs:166` |
| sym-dbb00de070d8eb6b0810 | `reenter_audio` | function | Re-enters this operator output into the Session's specialized audio lane. | `src/session/declaration/draft.rs:1075` |
| sym-127d4e8eac621aa2cd73 | `register` | function | Registers a node definition with `NodeRegistry` while preserving unique identities. | `src/graph/registry.rs:77` |
| sym-20458571d90e28917c89 | `register` | function | Registers a node definition with `SourceRegistry` while preserving unique identities. | `src/session/extensions/source.rs:297` |
| sym-d26f92546b597ee8eef7 | `register_async` | function | Validates and registers one asynchronous operator factory with `NodeRegistry`. | `src/graph/registry.rs:89` |
| sym-7b091b39fc9bcf798cf8 | `register_async_operator` | function | Registers async operator for `SessionEngineBuilder`. | `src/session/lifecycle/engine.rs:114` |
| sym-f673ad9edc8fa0ce4167 | `register_async_operator` | function | Registers async operator for `SessionEngineHostBuilder`. | `src/session/lifecycle/host.rs:305` |
| sym-84ccb34b37ccfcad3ac1 | `register_audio_endpoint_driver` | function | Registers audio endpoint driver for `SessionEngineBuilder`. | `src/session/lifecycle/engine.rs:74` |
| sym-4372671327154d071db5 | `register_audio_endpoint_driver` | function | Registers one externally owned endpoint implementation with the Session engine. | `src/session/lifecycle/host.rs:283` |
| sym-75a578b8ae23707a4df0 | `register_browser_driver` | function | Registers the externally owned browser/remote transport implementation. | `src/lib.rs:543` |
| sym-4a56bcf46740dc03c2d8 | `register_connector` | function | Registers one connector implementation for use by `Session`. | `src/connector/mod.rs:204` |
| sym-88ded8c4c9819c0577b8 | `register_connector_driver` | function | Registers the externally owned implementation for a declared connector. | `src/lib.rs:524` |
| sym-38ceb9e02ee487d3eb18 | `register_definition` | function | Registers one validated node definition with `NodeRegistry`. | `src/graph/registry.rs:112` |
| sym-edc0e3b3177b8f55e33d | `register_endpoint` | function | Registers one externally owned endpoint as a single compiler/runtime extension. The endpoint definition and driver cannot be installed independently through this authority. | `src/lib.rs:575` |
| sym-937033ec6a10c443a169 | `register_endpoint` | function | Atomically registers an endpoint's compiler contract and runtime driver. | `src/session/lifecycle/engine.rs:91` |
| sym-8e5eb001bee2111e2de7 | `register_endpoint` | function | Registers one endpoint implementation with `SessionEngineHostBuilder`. | `src/session/lifecycle/host.rs:294` |
| sym-06c9c55409cce995d33b | `register_multistem_recording` | function | Registers multistem recording for `SessionEngineBuilder`. | `src/session/extensions/recording.rs:42` |
| sym-2137efdc32dac8bf187b | `register_multistem_recording` | function | Registers multistem recording for `SessionEngineHostBuilder`. | `src/session/lifecycle/host.rs:333` |
| sym-47d100962a3bf2cae9e2 | `register_operator` | function | Registers one asynchronous operator implementation for use by `Session`. | `src/lib.rs:490` |
| sym-1fe31bd09a69f07d140d | `register_polled_audio_endpoint` | function | Registers polled audio endpoint for `SessionEngineBuilder`. | `src/session/extensions/polled_audio.rs:23` |
| sym-acd553bb8e9394c15a7f | `register_polled_audio_endpoint` | function | Registers polled audio endpoint for `SessionEngineHostBuilder`. | `src/session/lifecycle/host.rs:321` |
| sym-dcc4f9bee8d8883799b6 | `register_sidecar` | function | Registers one language-neutral sidecar under this Session's bounded process lifecycle. The child is spawned only during transactional start. | `src/lib.rs:478` |
| sym-2e6114d9d2dc4a84ef66 | `register_sidecar_process` | function | Retains one externally implemented sidecar under the Session lifecycle. IDs are unique within the engine so observations and shutdown failures remain attributable without process-global state. | `src/session/lifecycle/engine.rs:125` |
| sym-759850594b5adaa4d601 | `register_sidecar_process` | function | Registers sidecar process for `SessionEngineHostBuilder`. | `src/session/lifecycle/host.rs:313` |
| sym-4ce7c7c13c12e518bb82 | `register_source` | function | Retains an external Source factory for this Session. | `src/lib.rs:465` |
| sym-3113468e35377719ec59 | `register_source_factory` | function | Registers one externally implemented source contract by stable type ID. | `src/session/lifecycle/engine.rs:146` |
| sym-14292393f6d9f0b57801 | `registrations` | function | Returns the registrations associated with `NativeExtensionLibrary`. | `src/native_extension/mod.rs:72` |
| sym-076cb297137035bb5445 | `report_readiness_failure` | function | Returns the report readiness failure held by `ConnectorContext`. | `src/connector/worker/coordination.rs:97` |
| sym-d772a418c19fd77add1e | `report_readiness_success` | function | Records a successful readiness probe for `ConnectorContext`. | `src/connector/worker/coordination.rs:80` |
| sym-4de7461e08a0662dfe4a | `request` | function | Requests the state transition represented by `PlanRunnerCancellation`. | `src/runtime/audio/runner.rs:100` |
| sym-3476c11a19c3953a6cdb | `request` | function | Requests the state transition represented by `SessionStartCancellation`. | `src/session/lifecycle/control.rs:111` |
| sym-7ff05de91f06e67e578e | `request_stop` | function | Requests a graceful stop from `MultistemRecording`. | `src/recording/writer.rs:277` |
| sym-15106c234ce96f901dad | `required` | function | Returns the required held by `ConnectorRequirement`. | `src/connector/manifest.rs:65` |
| sym-7a0c4d40c5310e826b12 | `required` | function | Returns the required held by `PortSpec`. | `src/graph/ports.rs:233` |
| sym-f8be21ec7279a2be3397 | `requirement` | function | Returns the requirement held by `ConnectorConfigurationField`. | `src/connector/configuration.rs:214` |
| sym-6ca6792465c0da4ea967 | `requirements` | function | Returns the requirements held by `ConnectorManifest`. | `src/connector/manifest.rs:156` |
| sym-6127dbc8a3fa24244d1a | `requires_realtime_safety` | function | Returns `true` if the partition requires strict real-time safety. | `src/graph/partition.rs:55` |
| sym-0bc159f5755990d8d3e7 | `resolve` | function | Resolves `ConnectorConfigurationSchema` into its validated representation. | `src/connector/configuration.rs:259` |
| sym-824f7af5a486a435b990 | `result` | function | Returns the result represented by `ConnectorRunOutcome`. | `src/connector/worker/mod.rs:59` |
| sym-4b718d65571b93843407 | `result` | function | Returns the result represented by `MultistemRecordingReceipt`. | `src/recording/endpoint.rs:33` |
| sym-d50e98ae88230f6991ac | `retryability` | function | Returns the retryability associated with `ConnectorError`. | `src/connector/error.rs:117` |
| sym-7a6657b985a5058c5383 | `retryability` | function | Returns the retryability associated with `EndpointFailure`. | `src/endpoint/runtime.rs:216` |
| sym-b5d7c5410981b4ea2c72 | `revision` | function | Returns the revision held by `ConnectorConfigurationSchema`. | `src/connector/configuration.rs:247` |
| sym-c00acee16d79601568b3 | `revision` | function | Returns the revision held by `ConnectorServiceStatus`. | `src/connector/status.rs:66` |
| sym-8a92331d70fc48323ebd | `revision` | function | Returns the revision held by `AsyncOperatorManifest`. | `src/graph/signal/operator.rs:180` |
| sym-ca9371c7d79ce617e7c9 | `revision` | function | Returns the revision held by `NativeExtensionRegistration`. | `src/native_extension/mod.rs:50` |
| sym-3c8804dcb9d67a70e8f0 | `revision` | function | Additive descriptor revision within the compatibility major encoded by the [`SourceTypeId`] suffix. A breaking source contract uses a new identifier ending in the next `vN`; it does not reuse this field. | `src/session/extensions/source.rs:149` |
| sym-6cbb5a9702d66bb9acab | `role` | function | Returns the role associated with `SignalSpec`. | `src/graph/signal/spec.rs:219` |
| sym-7b587545232042df4a15 | `rollback_failures` | function | Returns the rollback failures associated with `SessionStartFailure`. | `src/session/lifecycle/control.rs:311` |
| sym-dae8e3c5a387e80a83c8 | `rollback_failures` | function | Returns the rollback failures associated with `SessionTerminalOutcome`. | `src/session/lifecycle/events.rs:283` |
| sym-d0c29369831049a746dc | `rollback_failures_total` | function | Returns the rollback failures total held by `SessionStartError`. | `src/session/lifecycle/control.rs:227` |
| sym-8597b7af198e2d4a7fa3 | `route` | function | Returns the route associated with `SessionMetricsSnapshot`. | `src/session/lifecycle/observations.rs:95` |
| sym-03b173a350c6a57a7269 | `route_context` | function | Returns the route context associated with `EndpointPrepareContext`. | `src/endpoint/runtime.rs:142` |
| sym-97d7a5b622196283178f | `route_count` | function | Returns the route count held by `SessionMetricsSnapshot`. | `src/session/lifecycle/observations.rs:91` |
| sym-1b20947938fd533901ed | `route_enqueued_at_ns` | function | Monotonic instant when the runtime accepted this frame into the route. | `src/endpoint/contract.rs:78` |
| sym-a077f8087bd1b0ea36cc | `route_enqueued_at_ns` | function | Returns the route enqueued at nanoseconds held by `PolledAudioFrame`. | `src/endpoint/polled_audio_driver.rs:273` |
| sym-7a5f4c3b1c19458f2622 | `route_id` | function | Returns the route identifier held by `ConnectorInputDescriptor`. | `src/connector/worker/driver.rs:36` |
| sym-869484fc3f9250429c55 | `route_id` | function | Returns the route identifier held by `PolledAudioFrame`. | `src/endpoint/polled_audio_driver.rs:269` |
| sym-c5ba950a8b743723f401 | `route_id` | function | Returns the route identifier held by `EndpointRouteContext`. | `src/endpoint/runtime.rs:80` |
| sym-aeb4bc1f70f948233605 | `route_id` | function | Returns the route identifier held by `SessionEndpointFailure`. | `src/session/lifecycle/events.rs:146` |
| sym-c2546891d13d4e436ecc | `route_id` | function | Returns the route identifier held by `PreparedWorkerMapping`. | `src/session/prepare/mappings.rs:243` |
| sym-3c98cb4f337c39d8f86e | `route_observations` | function | Returns the route observations held by `PreparedSession`. | `src/session/prepare/prepared.rs:61` |
| sym-5a21c9e9cb6492af0861 | `route_received_at_ns` | function | Monotonic instant when this endpoint received the frame from the route. | `src/endpoint/contract.rs:83` |
| sym-25666093672968916805 | `route_received_at_ns` | function | Returns the route received at nanoseconds held by `PolledAudioFrame`. | `src/endpoint/polled_audio_driver.rs:277` |
| sym-18bf3e18e724e053ab01 | `runtime_events_total` | function | Returns the runtime events total held by `SessionStopOutcome`. | `src/session/lifecycle/control.rs:387` |
| sym-53f97664fe5a7e62dbc9 | `runtime_failures_total` | function | Returns the runtime failures total held by `SessionStopOutcome`. | `src/session/lifecycle/control.rs:375` |
| sym-fb4d00daf3ca2f9fe29d | `runtime_worker_panicked` | function | Returns whether runtime worker panicked is true for `SessionStopOutcome`. | `src/session/lifecycle/control.rs:371` |
| sym-2a11bb5d1823a37a93d1 | `safety` | function | Returns the safety held by `NodeDescriptor`. | `src/graph/node.rs:242` |
| sym-65050ce70c28d8fb9840 | `safety` | function | Returns the safety held by `SourceManifest`. | `src/session/extensions/source.rs:178` |
| sym-42f95bbf07f21badb40d | `sample_capacity` | function | Returns the sample capacity held by `AudioInputBuffer`. | `src/session/extensions/audio_input/buffer.rs:19` |
| sym-1b15d056b7846cee586f | `sample_count` | function | Returns the sample count held by `AudioInputBuffer`. | `src/session/extensions/audio_input/buffer.rs:23` |
| sym-fd3eb6acc1e5c5cd9dc4 | `sample_format` | function | Returns the sample format associated with `EndpointAudioFrame`. | `src/endpoint/contract.rs:65` |
| sym-fbcfebd530788e070a8c | `sample_format` | function | Returns the sample format associated with `PlanEdgeFrame`. | `src/runtime/audio/router.rs:77` |
| sym-17889b1aaf08c5787fe6 | `sample_rate_hz` | function | Returns the sample rate hertz held by `EndpointAudioFrame`. | `src/endpoint/contract.rs:57` |
| sym-81dd0a065a56af223425 | `sample_rate_hz` | function | Returns the sample rate hertz held by `PolledAudioFrame`. | `src/endpoint/polled_audio_driver.rs:297` |
| sym-7aff210034f3493f65f7 | `sample_rate_hz` | function | Returns the sample rate hertz held by `AudioFrame`. | `src/frame/audio.rs:126` |
| sym-4a8991acef0dc4873962 | `sample_rate_hz` | function | Returns the sample rate hertz held by `SharedAudioFrame`. | `src/frame/audio.rs:196` |
| sym-7d6a0b2352d4e8a2dce6 | `sample_rate_hz` | function | Returns the sample rate hertz held by `PlanEdgeFrame`. | `src/runtime/audio/router.rs:63` |
| sym-331ed8b9923e72f5d75e | `sample_spec` | function | Declares the PCM format produced by the configured capture backends and consumed by compiled Session routes. | `src/lib.rs:304` |
| sym-bc9a1e68aaf2c74275bc | `sample_spec` | function | Returns the sample spec held by `AudioInputConfig`. | `src/session/extensions/audio_input/mod.rs:59` |
| sym-e2f0787e6e779bba80c2 | `samples` | function | Returns the audio samples held by `ConnectorAudioRecord`. | `src/connector/transport.rs:343` |
| sym-e9e52ba605e1fcf03748 | `samples` | function | Returns the audio samples held by `EndpointAudioFrame`. | `src/endpoint/contract.rs:69` |
| sym-cdbeb7d9b5fd4ea1bebf | `samples` | function | Returns the audio samples held by `PolledAudioFrame`. | `src/endpoint/polled_audio_driver.rs:305` |
| sym-a28d0bab384af6c24835 | `samples` | function | Returns the audio samples held by `AudioFrame`. | `src/frame/audio.rs:146` |
| sym-0000b34e0f76d20dfe6f | `samples` | function | Returns the audio samples held by `SharedAudioFrame`. | `src/frame/audio.rs:216` |
| sym-35573b446bcfa5698752 | `samples` | function | Returns the audio samples held by `PlanEdgeFrame`. | `src/runtime/audio/router.rs:84` |
| sym-cbc177b60d1fcce0df49 | `samples` | function | Returns the audio samples held by `AudioInputBuffer`. | `src/session/extensions/audio_input/buffer.rs:38` |
| sym-c561ccb29a1915c9b687 | `samples_at_48k` | function | Returns the samples at 48k associated with `OpusFrameDuration`. | `src/codec/encoder.rs:15` |
| sym-93ab577d63a0af01da53 | `samples_mut` | function | Returns the samples mut held by `AudioInputBuffer`. | `src/session/extensions/audio_input/buffer.rs:42` |
| sym-87959113d8c029a0e5b5 | `schema` | function | Returns the schema held by `SignalSpec`. | `src/graph/signal/spec.rs:223` |
| sym-a17526c1972e9e7804b4 | `selector_persistence_scope` | function | Reports how long this discovered selector can be reused without rediscovery. The capture owner remains authoritative for opening it. | `src/capture/identity.rs:114` |
| sym-9ee94498311f09f4fec1 | `selector_persistence_scope` | function | Describes how long the selector may be reused without rediscovery. | `src/capture/selection.rs:36` |
| sym-aa0e0375e545c4bdc101 | `send` | function | Sends a value through `AsyncOperatorInput`. | `src/runtime/signal/io.rs:39` |
| sym-c0aad181c613ed17e977 | `send` | function | Sends a value through `StemHandle`. | `src/session/declaration/draft.rs:787` |
| sym-7c69992d4858a56273ba | `send` | function | Sends a value through `SourceOutputHandle`. | `src/session/declaration/draft.rs:939` |
| sym-952690bdf8b6f7248780 | `send` | function | Sends a value through `DerivedStreamHandle`. | `src/session/declaration/draft.rs:1057` |
| sym-fd09904f68e38e6fc723 | `send` | function | Sends a value through `Stream`. | `src/session/declaration/typed_stream.rs:151` |
| sym-0b2d138a7fc7e6ce5d71 | `send_audio` | function | Sends one audio signal through the bounded input owned by `AsyncOperatorInput`. | `src/runtime/signal/io.rs:55` |
| sym-4a591dfe216a4db3372b | `send_to` | function | Connects this stream to one explicit endpoint input port. | `src/session/declaration/draft.rs:792` |
| sym-87fbcd5a2298ceed1ae2 | `send_to` | function | Connects the current stream to one explicit endpoint input through `SourceOutputHandle`. | `src/session/declaration/draft.rs:947` |
| sym-09b6a8e55613b01f7814 | `send_to` | function | Connects this derived output to one explicit endpoint input port. | `src/session/declaration/draft.rs:1062` |
| sym-d8bec0296b24489ae915 | `sender_observations` | function | Returns the sender observations held by `PreparedSourceMapping`. | `src/session/prepare/mappings.rs:30` |
| sym-0822a99c4c9d5ae4d890 | `sequence_number` | function | Returns the sequence number held by `EndpointAudioFrame`. | `src/endpoint/contract.rs:49` |
| sym-772339cccbc94efd4611 | `sequence_number` | function | Returns the sequence number held by `AudioFrame`. | `src/frame/audio.rs:142` |
| sym-91c14c9883179bf54302 | `sequence_number` | function | Returns the sequence number held by `SharedAudioFrame`. | `src/frame/audio.rs:212` |
| sym-34372e0f01d72dc147ea | `sequence_number` | function | Returns the sequence number held by `FrameLineage`. | `src/frame/lineage.rs:68` |
| sym-3d646b762682ed19138f | `sequence_number` | function | Returns the sequence number held by `SignalEnvelope`. | `src/graph/signal/envelope.rs:90` |
| sym-bb0dc9b29e51083e200b | `sequence_number` | function | Returns the sequence number held by `SignalLineage`. | `src/graph/signal/lineage.rs:71` |
| sym-2352790cae6eb2f7b359 | `sequence_number` | function | Returns the sequence number held by `PlanEdgeFrame`. | `src/runtime/audio/router.rs:49` |
| sym-f3081dd71c51b0b3c86d | `session_id` | function | Returns the session identifier held by `CaptureLineageSeed`. | `src/capture/capture_owner.rs:38` |
| sym-98cd1bcf8ce5150f2a87 | `session_id` | function | Returns the session identifier held by `RegisteredConnector`. | `src/connector/mod.rs:132` |
| sym-3d2f454d3fb300f11250 | `session_id` | function | Returns the session identifier held by `EndpointPrepareContext`. | `src/endpoint/runtime.rs:125` |
| sym-4725630db086a1a9ed42 | `session_id` | function | Returns the session identifier held by `FrameLineage`. | `src/frame/lineage.rs:56` |
| sym-27466d271df564e717ec | `session_id` | function | Returns the session identifier held by `SignalLineage`. | `src/graph/signal/lineage.rs:59` |
| sym-49bf3d2e129c97a20184 | `session_id` | function | Returns the session identifier held by `RunningSession`. | `src/lib.rs:802` |
| sym-74a5ff2076b0a0bc23b9 | `session_id` | function | Returns the session identifier held by `CompiledSession`. | `src/session/compile/compiled.rs:22` |
| sym-9fd77a5e8f38555f95ae | `session_id` | function | Returns the session identifier held by `EndpointHandle`. | `src/session/declaration/draft.rs:587` |
| sym-1137383ee2d55aab114d | `session_id` | function | Returns the session identifier held by `OperatorInstanceHandle`. | `src/session/declaration/draft.rs:709` |
| sym-05ea9eef63f2cbe19eb5 | `session_id` | function | Returns the session identifier held by `StemHandle`. | `src/session/declaration/draft.rs:779` |
| sym-e4bf3607b3a874c3f877 | `session_id` | function | Returns the session identifier held by `SourceInstanceHandle`. | `src/session/declaration/draft.rs:842` |
| sym-80313a46173b6b3ac151 | `session_id` | function | Returns the session identifier held by `SourceOutputHandle`. | `src/session/declaration/draft.rs:919` |
| sym-6b67dd88e766a0d4c665 | `session_id` | function | Returns the session identifier held by `DerivedStreamHandle`. | `src/session/declaration/draft.rs:1012` |
| sym-e5f553d99347e8e73746 | `session_id` | function | Returns the session identifier held by `SessionSpec`. | `src/session/declaration/spec.rs:319` |
| sym-96e9acf60e070d2a9a6b | `session_id` | function | Returns the session identifier held by `SessionTerminalOutcome`. | `src/session/lifecycle/events.rs:267` |
| sym-3def62043afe64b2e50d | `session_id` | function | Returns the session identifier held by `SessionEvent`. | `src/session/lifecycle/events.rs:318` |
| sym-d81bf87b50a7196d4777 | `session_id` | function | Returns the session identifier held by `RunningSession`. | `src/session/lifecycle/running.rs:198` |
| sym-13928ccb2e55e72f280c | `session_id` | function | Returns the session identifier held by `SessionTrace`. | `src/session/lifecycle/trace.rs:268` |
| sym-4960992aec8e74498dea | `session_id` | function | Returns the session identifier held by `PreparedSession`. | `src/session/prepare/prepared.rs:31` |
| sym-501622870af6238258a3 | `session_timeline_origin` | function | Returns the session timeline origin associated with `EndpointPrepareContext`. | `src/endpoint/runtime.rs:146` |
| sym-0267117466ccdec4ecb8 | `session_timestamp_ns` | function | Returns the session timestamp nanoseconds held by `SignalTiming`. | `src/graph/signal/timing.rs:79` |
| sym-af302dce91a9a38e4fe7 | `session_trace` | function | Enables the bounded Session Session trace recorder. | `src/lib.rs:330` |
| sym-774286b418d83925e6c7 | `session_trace_outcome` | function | Returns the session trace outcome held by `RunningSession`. | `src/lib.rs:893` |
| sym-60e70c8ae1b54b54118d | `set_application_backend` | function | Sets the application backend used by `SessionEngineHostBuilder`. | `src/session/lifecycle/host.rs:261` |
| sym-174cfb8ae938db7b41e2 | `set_bitrate_kbps` | function | Update the live encoder bitrate. Called by CODEC_HINT handler (AUDIO-021). `kbps` = 0 switches to Opus auto (VBR). Safe to call mid-stream. | `src/codec/encoder.rs:280` |
| sym-82a0413af3d835e8998b | `set_complexity` | function | Set encoder complexity (0 = fastest, 10 = highest quality). | `src/codec/encoder.rs:274` |
| sym-c16c1279a7c88497027f | `set_connected` | function | Sets the connected used by `ConnectorContext`. | `src/connector/worker/coordination.rs:74` |
| sym-e1c705b3360b5cdb4e3c | `set_degraded` | function | Sets the degraded used by `ConnectorContext`. | `src/connector/worker/coordination.rs:56` |
| sym-0d813e85baaea8c42076 | `set_healthy` | function | Sets the healthy used by `ConnectorContext`. | `src/connector/worker/coordination.rs:62` |
| sym-06f6a148899524e1860d | `set_microphone_backend` | function | Sets the microphone backend used by `SessionEngineHostBuilder`. | `src/session/lifecycle/host.rs:269` |
| sym-ee840fcb3044537b46b4 | `set_not_ready` | function | Sets the not ready used by `ConnectorContext`. | `src/connector/worker/coordination.rs:50` |
| sym-75e1e42ad5f9e613a19a | `set_ready` | function | Sets the ready used by `ConnectorContext`. | `src/connector/worker/coordination.rs:44` |
| sym-35f5038470168d3cc14f | `set_reconnecting` | function | Sets the reconnecting used by `ConnectorContext`. | `src/connector/worker/coordination.rs:68` |
| sym-b0bc25babc178bcc34b2 | `set_session_trace` | function | Sets the session trace used by `SessionEngineBuilder`. | `src/session/lifecycle/engine.rs:66` |
| sym-9c598d0cfdbf7c7703eb | `shared_audio_rejected_total` | function | Returns the shared audio rejected total held by `SessionAudioReentryMetrics`. | `src/session/lifecycle/observations.rs:356` |
| sym-a699fd81177045ee4b6e | `shared_ref_count` | function | Returns the shared ref count held by `AudioBufferPool`. | `src/frame/pool.rs:102` |
| sym-cfc84a3029a025cc281d | `shared_ref_count` | function | Returns the shared ref count held by `SharedAudioBufferHandle`. | `src/frame/pool.rs:315` |
| sym-8a2399f88ff09ce306e9 | `shutdown` | function | Shuts down `AsyncRuntimeHost` according to its lifecycle contract. | `src/runtime/lifecycle/async_host.rs:91` |
| sym-d9d2e0d89468cdfc07e2 | `shutdown_mode` | function | Returns the shutdown mode held by `ConnectorContext`. | `src/connector/worker/coordination.rs:32` |
| sym-c5fc5da6a4d876db8d89 | `sidecar` | function | Builds an outbound Connector backed by one bounded sidecar process. | `src/connector/mod.rs:112` |
| sym-1fe1d196d9e3d59aa09c | `sidecar_metrics` | function | Returns the sidecar metrics held by `RunningSession`. | `src/lib.rs:856` |
| sym-5721cd08a4a2428187fb | `sidecar_metrics` | function | Returns the sidecar metrics held by `RunningSession`. | `src/session/lifecycle/running.rs:245` |
| sym-1233436237bf54848643 | `signal` | function | Returns the signal held by `EndpointRouteContext`. | `src/endpoint/runtime.rs:73` |
| sym-7dd55d12dd6380c1deeb | `signal` | function | Returns the signal held by `PortPrepareContext`. | `src/graph/node.rs:349` |
| sym-dd99cd344d21bf7dbb2a | `signal` | function | Returns the signal held by `PortSpec`. | `src/graph/ports.rs:221` |
| sym-8fbd7a290ae01c91f601 | `signal_spec` | function | Returns the signal spec held by `ExtensionSignal`. | `src/conformance.rs:1183` |
| sym-b4e31b8412b1f9584836 | `signal_spec` | function | Returns the signal spec held by `ConnectorInputDescriptor`. | `src/connector/worker/driver.rs:44` |
| sym-1aaa28c6c59496229026 | `signal_spec` | function | Returns the signal spec held by `EndpointPortInput`. | `src/endpoint/contract.rs:237` |
| sym-08f37b5d5044dd0831e3 | `signal_spec` | function | Returns the signal spec held by `SignalEnvelope`. | `src/graph/signal/envelope.rs:70` |
| sym-1d0c1447a7892c91bfa6 | `signal_spec` | function | Returns the signal spec held by `Stream`. | `src/session/declaration/typed_stream.rs:128` |
| sym-b37469b23fc5fa217a9f | `signals_dropped_total` | function | Returns the signals dropped total held by `SessionAudioReentryMetrics`. | `src/session/lifecycle/observations.rs:332` |
| sym-b7933539a0a56faf01ad | `signals_enqueued_total` | function | Returns the signals enqueued total held by `SessionAudioReentryMetrics`. | `src/session/lifecycle/observations.rs:324` |
| sym-29325d8808ddfda846db | `signals_received_total` | function | Returns the signals received total held by `SessionAudioReentryMetrics`. | `src/session/lifecycle/observations.rs:328` |
| sym-822f8306016599aabc4b | `size_bytes` | function | Owned media bytes represented by this payload. Envelope metadata and queue slot storage are fixed-size and accounted separately by the edge. | `src/graph/signal/payload.rs:37` |
| sym-14f0a5f35858fb569bf4 | `slot_count` | function | Returns the slot count held by `AudioBufferPool`. | `src/frame/pool.rs:65` |
| sym-a44fd84794d6cff47da9 | `slot_size` | function | Returns the slot size associated with `AudioBufferPool`. | `src/frame/pool.rs:62` |
| sym-cc9f9c2b8135d828a747 | `snapshot` | function | Returns a point-in-time snapshot of `CaptureObservationCounters`. | `src/capture/observations.rs:113` |
| sym-5b3105465b6e10369c49 | `snapshot` | function | Returns a point-in-time snapshot of `ConnectorObservationHandle`. | `src/connector/observations.rs:53` |
| sym-b34f7de09857843496dd | `snapshot` | function | Returns a point-in-time snapshot of `AsyncOperatorObservationHandle`. | `src/runtime/signal/observations.rs:52` |
| sym-c348d016a0ddc601bf25 | `snapshot` | function | Returns a point-in-time snapshot of `SourceRuntimeObservationHandle`. | `src/session/extensions/source.rs:412` |
| sym-650fc92c17cdabab7bd9 | `snapshot` | function | Returns a point-in-time snapshot of `ClockDriftEstimator`. | `src/timing/clock_drift.rs:66` |
| sym-f1f4285cdc12116ec66b | `source` | function | Declares one instance of an open external source type. | `src/lib.rs:394` |
| sym-e2e276be861b39dc6c14 | `source` | function | Declares one externally implemented source instance. | `src/session/declaration/draft.rs:355` |
| sym-cc57c91ea66f5a37dff9 | `source` | function | Returns the source held by `StemSpec`. | `src/session/declaration/spec.rs:155` |
| sym-0ed4f5ae950ecd275dda | `source` | function | Returns the source held by `AudioInput`. | `src/session/extensions/audio_input/mod.rs:103` |
| sym-a61ae1035ab61fb8ad5a | `source` | function | Returns the source held by `PcmSource`. | `src/session/extensions/audio_input/source.rs:52` |
| sym-4ec7db627f83d7646e42 | `source` | function | Returns the source held by `SessionStartFailure`. | `src/session/lifecycle/control.rs:331` |
| sym-d882bd92e4e11bbf8b68 | `source` | function | Returns the source held by `SessionMetricsSnapshot`. | `src/session/lifecycle/observations.rs:79` |
| sym-24e2956ea410d9098765 | `source_count` | function | Returns the source count held by `SessionMetricsSnapshot`. | `src/session/lifecycle/observations.rs:75` |
| sym-fc1b81719c18435d41cf | `source_declarations` | function | Returns the source declarations associated with `CompiledSession`. | `src/session/compile/compiled.rs:32` |
| sym-de09c834b6eb8e52de01 | `source_failures` | function | Returns the source failures associated with `SessionTerminalOutcome`. | `src/session/lifecycle/events.rs:275` |
| sym-5183f904654f35cdfad7 | `source_generation` | function | Returns the source generation associated with `FrameLineage`. | `src/frame/lineage.rs:77` |
| sym-e86a63738361451935e5 | `source_generation` | function | Returns the source generation associated with `SignalLineage`. | `src/graph/signal/lineage.rs:74` |
| sym-e242c2581097f70c164a | `source_id` | function | Derives the immutable captured-frame identity for this resolved source. | `src/capture/identity.rs:46` |
| sym-fba5cce843823f0dee0a | `source_id` | function | Returns the source identifier held by `MacosInputSource`. | `src/capture/platform/macos/input.rs:230` |
| sym-023c662556cbd7906a4d | `source_id` | function | Returns the source identifier held by `SystemLoopbackSource`. | `src/capture/platform/macos/loopback.rs:255` |
| sym-d51c8a4ef1101beb5b61 | `source_id` | function | Returns the source identifier held by `DesktopCaptureSource`. | `src/capture/platform/macos/mod.rs:89` |
| sym-0c795be22d819fc224e6 | `source_id` | function | Returns the source identifier held by `EndpointAudioFrame`. | `src/endpoint/contract.rs:41` |
| sym-20981e860d32b9b1cf29 | `source_id` | function | Returns the source identifier held by `AudioFrame`. | `src/frame/audio.rs:122` |
| sym-27e6d5f80ecefd5f91d5 | `source_id` | function | Returns the source identifier held by `SharedAudioFrame`. | `src/frame/audio.rs:192` |
| sym-9beb92b3b06757ff589b | `source_id` | function | Returns the source identifier held by `FrameLineage`. | `src/frame/lineage.rs:59` |
| sym-a6ff5b3bbd5b6dda6463 | `source_id` | function | Returns the source identifier held by `SignalEnvelope`. | `src/graph/signal/envelope.rs:100` |
| sym-b0efd6d304ca0b3dd9d1 | `source_id` | function | Returns the source identifier held by `SignalLineage`. | `src/graph/signal/lineage.rs:65` |
| sym-67946fe0311861a74164 | `source_id` | function | Returns the source identifier held by `PlanEdgeFrame`. | `src/runtime/audio/router.rs:42` |
| sym-718724cbde2e947e5b83 | `source_id` | function | Returns the source identifier held by `SourceInstanceHandle`. | `src/session/declaration/draft.rs:850` |
| sym-7058d57c27f058d076dc | `source_id` | function | Returns the source identifier held by `SourceOutputHandle`. | `src/session/declaration/draft.rs:927` |
| sym-4b4e520640d0107f7759 | `source_id` | function | Returns the source identifier held by `SourceInstanceSpec`. | `src/session/declaration/spec.rs:80` |
| sym-01684e5eafd0d7d43bf6 | `source_input_count` | function | Returns the source input count held by `PreparedSession`. | `src/session/prepare/prepared.rs:46` |
| sym-f6bff20d73eda4129b72 | `source_instance_id` | function | Returns the source instance identifier held by `SourceOutputHandle`. | `src/session/declaration/draft.rs:923` |
| sym-5cde87c38dc671370004 | `source_instance_id` | function | Returns the source instance identifier held by `SourceOutputSpec`. | `src/session/declaration/spec.rs:137` |
| sym-e0baa9b30631741c6ff6 | `source_instances` | function | Returns the source instances associated with `SessionSpec`. | `src/session/declaration/spec.rs:327` |
| sym-3d595a1c9d605d912a82 | `source_manifest` | function | Returns the validated manifest currently registered for `source_type_id`. | `src/session/lifecycle/engine.rs:171` |
| sym-5e8bf667a16ecfbb06f4 | `source_manifest` | function | Returns the validated source manifest retained by this engine. | `src/session/lifecycle/engine.rs:217` |
| sym-05a32493aeeaf15986af | `source_mappings` | function | Returns the source mappings associated with `PreparedSession`. | `src/session/prepare/prepared.rs:41` |
| sym-0928e754c866d93b4664 | `source_node_id` | function | Returns the source node identifier held by `PlanSourceInput`. | `src/runtime/audio/runner.rs:196` |
| sym-368806897864f04748d1 | `source_observations` | function | Returns the source observations held by `RealtimePlanRunner`. | `src/runtime/audio/runner.rs:349` |
| sym-a2c3106df4490f97c64e | `source_outputs` | function | Returns the source outputs held by `SessionSpec`. | `src/session/declaration/spec.rs:331` |
| sym-a2f1064197d8f6249e13 | `source_send_rejections_total` | function | Returns the source send rejections total held by `SessionStopOutcome`. | `src/session/lifecycle/control.rs:383` |
| sym-89d57a64f65f1ba17140 | `source_timestamp_ns` | function | Returns the source timestamp nanoseconds held by `SignalTiming`. | `src/graph/signal/timing.rs:71` |
| sym-b38bce5e32c4a115e55f | `source_to_receive_latency` | function | Returns the source to receive latency associated with `SessionRouteMetrics`. | `src/session/lifecycle/observations.rs:223` |
| sym-53166cae1f1e30cc4b67 | `source_type_id` | function | Returns the source type identifier held by `SessionCompileDiagnostic`. | `src/session/compile/error.rs:137` |
| sym-2f4093b1ce4adef8eb07 | `source_type_id` | function | Returns the source type identifier held by `SourceInstanceSpec`. | `src/session/declaration/spec.rs:84` |
| sym-3f21b18b5704c78d6bbe | `source_type_id` | function | Returns the source type identifier held by `SourceManifest`. | `src/session/extensions/source.rs:142` |
| sym-e3ff3a0d7c299d5f93d1 | `spawn` | function | Spawns its owned operation for `GeneratedAudioBridge`. | `src/runtime/bridge/audio.rs:131` |
| sym-36b9156f7e45336488f3 | `spawn` | function | Spawns its owned operation for `SidecarHost`. | `src/runtime/lifecycle/sidecar_host.rs:174` |
| sym-1f34a59bc9b8a05605df | `spawn` | function | Spawns its owned operation for `AsyncOperatorWorker`. | `src/runtime/signal/operator.rs:536` |
| sym-e86e9f7d7c7df80fac20 | `spawn` | function | Spawns its owned operation for `SourceRegistry`. | `src/session/extensions/source.rs:328` |
| sym-c1cb2b6a786fe477aa4a | `spawn` | function | Spawns its owned operation for `SourceRuntime`. | `src/session/extensions/source.rs:565` |
| sym-654334b47f76036a36df | `spawn_composed` | function | Spawns `AsyncOperatorWorker` with the supplied typed input and fan-out outputs. | `src/runtime/signal/operator.rs:604` |
| sym-cd0608562372467f8a3b | `spawn_with_context` | function | Starts a directly-fed worker with an already negotiated signal-shaped prepare context. Session-owned graph execution uses the compiled-edge path; this entry point exists for external harnesses that negotiate the boundary before constructing a full Session. | `src/runtime/signal/operator.rs:563` |
| sym-67a67719d3a5121b84ba | `spec` | function | Returns the spec associated with `Pipeline`. | `src/graph/dsl.rs:86` |
| sym-0b8785b5ac7f39fdbe22 | `spec` | function | Returns the spec associated with `CompiledSession`. | `src/session/compile/compiled.rs:27` |
| sym-3eba8c9c77667942414f | `spec` | function | Returns the spec associated with `PreparedSession`. | `src/session/prepare/prepared.rs:36` |
| sym-53be45839ab2fab9d7df | `stable_id` | function | Returns the stable identifier held by `ApplicationSelector`. | `src/session/declaration/selector.rs:59` |
| sym-cfddbe05a671a2b21449 | `stage` | function | Returns the stage held by `ConnectorError`. | `src/connector/error.rs:113` |
| sym-c50865f7d057cce6f672 | `stage` | function | Returns the stage held by `EndpointFailure`. | `src/endpoint/runtime.rs:204` |
| sym-df0fea53fffbd8a7d4ab | `stage` | function | Returns the stage held by `SessionEndpointFailure`. | `src/session/lifecycle/events.rs:154` |
| sym-c9921f5cf274420603bc | `stage` | function | Returns the stage held by `SessionRollbackFailure`. | `src/session/lifecycle/events.rs:175` |
| sym-9879dd827deb19a8239c | `stage` | function | Returns the stage held by `SessionFinalizationFailure`. | `src/session/lifecycle/events.rs:199` |
| sym-0eb8a5f2cbc46f794c51 | `start` | function | Starts the lifecycle represented by `Session`. | `src/lib.rs:621` |
| sym-2ccfa3b2ba06c37cb09e | `start` | function | Starts the lifecycle represented by `PreparedSourceRuntime`. | `src/session/extensions/source.rs:505` |
| sym-25aaa54c91a04d79c42f | `start` | function | Starts the lifecycle represented by `SessionEngine`. | `src/session/lifecycle/engine.rs:284` |
| sym-430d545b0be9d10a95f3 | `start` | function | Starts the lifecycle represented by `SessionEngineHost`. | `src/session/lifecycle/host.rs:60` |
| sym-2b596a8a2da270bbc10a | `start` | function | Starts the lifecycle represented by `SessionTraceRecorder`. | `src/session/lifecycle/trace.rs:160` |
| sym-e894c7e9d934cb4c4f20 | `start_cancellable` | function | Starts `Session` transactionally while observing the supplied cancellation handle. | `src/lib.rs:625` |
| sym-8d65f4aabf4b2af73473 | `start_compiled` | function | Starts a previously compiled Session through `SessionEngine`. | `src/session/lifecycle/engine.rs:234` |
| sym-e89d2d59c3e2e2bd4282 | `start_compiled` | function | Starts a previously compiled Session through `SessionEngineHost`. | `src/session/lifecycle/host.rs:71` |
| sym-5bee57f6c3630c2fcf28 | `start_compiled_cancellable` | function | Starts compiled cancellable for `SessionEngine`. | `src/session/lifecycle/engine.rs:258` |
| sym-f407f1e40d09a2b695be | `start_compiled_cancellable` | function | Starts compiled cancellable for `SessionEngineHost`. | `src/session/lifecycle/host.rs:84` |
| sym-1c7e2faf6d66e4ad2524 | `start_failure` | function | Returns the transactional start failure carried by `SessionEngineStartError`, if this error represents one. | `src/session/lifecycle/engine.rs:329` |
| sym-89969574c9b15ca8049f | `startup_timeout` | function | Returns the startup timeout held by `ConnectorReadinessPolicy`. | `src/connector/readiness.rs:43` |
| sym-a8e3fa5ac25beeb6e1c5 | `state` | function | Returns the state associated with `RunningSession`. | `src/lib.rs:817` |
| sym-8e67bc135bc37c8ef3be | `state` | function | Returns the state associated with `SidecarHost`. | `src/runtime/lifecycle/sidecar_host.rs:257` |
| sym-c4af05bdb49bc89b4337 | `state` | function | Returns the state associated with `SessionTerminalOutcome`. | `src/session/lifecycle/events.rs:271` |
| sym-90436e0f28a02e284daa | `state` | function | Returns the authoritative current lifecycle state owned by this Session. | `src/session/lifecycle/running.rs:203` |
| sym-0d95ad2d241532bab63f | `stats` | function | Returns the current statistics for `CapturedFrameSender`. | `src/capture/frame_stream.rs:138` |
| sym-9c6f7313a6ef848fa3f7 | `stats` | function | Returns the current statistics for `CapturedFrameStream`. | `src/capture/frame_stream.rs:179` |
| sym-04f3132fd50e72551536 | `stem_id` | function | Returns the stem identifier held by `CaptureLineageSeed`. | `src/capture/capture_owner.rs:42` |
| sym-09598cb65ff60c6a276e | `stem_id` | function | Returns the stem identifier held by `FrameLineage`. | `src/frame/lineage.rs:62` |
| sym-82535d24a8934525ba0d | `stem_id` | function | Returns the stem identifier held by `SessionSourceFailure`. | `src/session/lifecycle/events.rs:114` |
| sym-0fb5f0bfed54ec91d501 | `stem_id` | function | Returns the stem identifier held by `SessionAudioReentryMetrics`. | `src/session/lifecycle/observations.rs:308` |
| sym-7e69cd4447f3438ac5db | `stem_id` | function | Returns the stem identifier held by `PreparedSourceMapping`. | `src/session/prepare/mappings.rs:25` |
| sym-b554e1d2676208e0d8dd | `stem_id` | function | Returns the stem identifier held by `PreparedWorkerMapping`. | `src/session/prepare/mappings.rs:248` |
| sym-a889cb0f54d5d971ecac | `stems` | function | Returns the stems associated with `SessionSpec`. | `src/session/declaration/spec.rs:323` |
| sym-d34077d6fda42b2b23e2 | `stereo_broadcast` | function | 20 ms stereo audio transport profile with an explicit bitrate. | `src/codec/encoder.rs:116` |
| sym-5a89a49c5047203f6db3 | `stop` | function | Stops `RunningSession` and returns its terminal result. | `src/lib.rs:901` |
| sym-65ae686cb34d2113858c | `stop` | function | Stops `RunningSession` and returns its terminal result. | `src/session/lifecycle/running.rs:412` |
| sym-25b059d674b5b37c1998 | `stop_and_join` | function | Stops `CaptureOwner`, joins its worker, and returns the terminal result. | `src/capture/capture_owner.rs:264` |
| sym-b9d59b97c1ce355391a0 | `stop_and_join` | function | Stops `MacosInputSource`, joins its worker, and returns the terminal result. | `src/capture/platform/macos/input.rs:242` |
| sym-c9e0f85bce4565b39679 | `stop_and_join` | function | Stops `SystemLoopbackSource`, joins its worker, and returns the terminal result. | `src/capture/platform/macos/loopback.rs:274` |
| sym-76f2b8525f01a2fa145a | `stop_and_join` | function | Stops `DesktopCaptureSource`, joins its worker, and returns the terminal result. | `src/capture/platform/macos/mod.rs:103` |
| sym-569afd1361d7a8fabc78 | `stream_id` | function | Returns the stream identifier held by `EndpointAudioFrame`. | `src/endpoint/contract.rs:45` |
| sym-3302bf8592ac13f92cde | `stream_id` | function | Returns the stream identifier held by `PolledAudioFrame`. | `src/endpoint/polled_audio_driver.rs:289` |
| sym-513d916fdd74b9382d04 | `stream_id` | function | Returns the stream identifier held by `AudioFrame`. | `src/frame/audio.rs:118` |
| sym-20c8ee64a5e5bb493ef4 | `stream_id` | function | Returns the stream identifier held by `SharedAudioFrame`. | `src/frame/audio.rs:188` |
| sym-428715befe67be9cea63 | `stream_id` | function | Returns the stream identifier held by `SignalLineage`. | `src/graph/signal/lineage.rs:62` |
| sym-999cb9e4270bf5715275 | `stream_id` | function | Returns the stream identifier held by `PlanEdgeFrame`. | `src/runtime/audio/router.rs:35` |
| sym-c48f6b73a83c33385ee7 | `stream_id` | function | Returns the stream identifier held by `SourceOutputHandle`. | `src/session/declaration/draft.rs:931` |
| sym-4c4288b1c114fef23dd2 | `stream_id` | function | Returns the stream identifier held by `SourceOutputSpec`. | `src/session/declaration/spec.rs:145` |
| sym-85358c2d790b8b9d2d7a | `success` | function | Returns whether `ConnectorRunOutcome` completed successfully. | `src/connector/worker/mod.rs:51` |
| sym-f54e46b4ef85fcf04135 | `success_threshold` | function | Returns the success threshold held by `ConnectorReadinessPolicy`. | `src/connector/readiness.rs:51` |
| sym-ba4f5e56d43535277f59 | `supports` | function | Returns whether supports is true for `SignalPayload`. | `src/graph/signal/payload.rs:17` |
| sym-e855bceefb46d8582818 | `supports_signal` | function | Reports whether supports signal is true for `MediaCaps`. | `src/graph/ports.rs:142` |
| sym-e8e668a7e4cace617ce0 | `syntax_version` | function | Returns the syntax version held by `OperatorId`. | `src/graph/operator.rs:35` |
| sym-6cb771547bf4b11b01ca | `take_event_receiver` | function | Takes event receiver for `SessionStartFailure`. | `src/session/lifecycle/control.rs:315` |
| sym-e75ca475e38b34bbe9d4 | `take_event_receiver` | function | Takes event receiver for `RunningSession`. | `src/session/lifecycle/running.rs:207` |
| sym-0d35b097b01f29c4fb45 | `target` | function | Returns the target associated with `ConnectionSpec`. | `src/session/declaration/spec.rs:267` |
| sym-a455ae640500e774203c | `text` | function | Convenience constructor for text ports. | `src/graph/signal/spec.rs:279` |
| sym-2875aa0dac215db542ff | `through` | function | Routes the current stream through a declared operator using `StemHandle`. | `src/session/declaration/draft.rs:811` |
| sym-4f367acb930c9b955ddc | `through` | function | Routes the current stream through a declared operator using `SourceOutputHandle`. | `src/session/declaration/draft.rs:963` |
| sym-014e4d894a0a61fa9186 | `through` | function | Routes the current stream through a declared operator using `DerivedStreamHandle`. | `src/session/declaration/draft.rs:1043` |
| sym-54aaebb488819bad6df9 | `through` | function | Routes the current stream through a declared operator using `Stream`. | `src/session/declaration/typed_stream.rs:132` |
| sym-2c34b154cc9adbb1bd9d | `through_ports` | function | Returns the through ports held by `StemHandle`. | `src/session/declaration/draft.rs:815` |
| sym-bb361fbf187b313916e7 | `through_ports` | function | Returns the through ports held by `SourceOutputHandle`. | `src/session/declaration/draft.rs:967` |
| sym-dd91ff1fabc676df06db | `through_ports` | function | Returns the through ports held by `DerivedStreamHandle`. | `src/session/declaration/draft.rs:1047` |
| sym-9def9300f09440ab219b | `tick` | function | Applies one measured clock offset to `ClockCorrectionController` and returns the bounded correction. | `src/timing/clock_correction.rs:23` |
| sym-7d275195264aacbf43e2 | `tick_rate_hz` | function | Returns the tick rate hertz held by `ClockDomainDescriptor`. | `src/timing/domain.rs:43` |
| sym-b201578c15154d7494bb | `timestamp_end_ns` | function | Returns the timestamp end nanoseconds held by `FrameLineage`. | `src/frame/lineage.rs:87` |
| sym-50d2b8c719e7733cec6b | `timestamp_end_ns` | function | Returns the timestamp end nanoseconds held by `SignalTiming`. | `src/graph/signal/timing.rs:65` |
| sym-01c9958af9a5b4195caa | `timestamp_ns` | function | Returns the timestamp nanoseconds held by `EndpointAudioFrame`. | `src/endpoint/contract.rs:53` |
| sym-311c8660b26ac916a68c | `timestamp_ns` | function | Returns the timestamp nanoseconds held by `AudioFrame`. | `src/frame/audio.rs:138` |
| sym-6f2251357630ef9d5aed | `timestamp_ns` | function | Returns the timestamp nanoseconds held by `SharedAudioFrame`. | `src/frame/audio.rs:208` |
| sym-c1b6f572ff5263b5a11b | `timestamp_ns` | function | Returns the timestamp nanoseconds held by `SignalEnvelope`. | `src/graph/signal/envelope.rs:110` |
| sym-af33bd63d369ef77b28a | `timestamp_ns` | function | Returns the timestamp nanoseconds held by `PlanEdgeFrame`. | `src/runtime/audio/router.rs:56` |
| sym-134ced1b0fa7572310f8 | `timestamp_start_ns` | function | Returns the timestamp start nanoseconds held by `FrameLineage`. | `src/frame/lineage.rs:71` |
| sym-0ab1b64bcc0cbfe5043a | `timing` | function | Returns the timing associated with `SignalEnvelope`. | `src/graph/signal/envelope.rs:74` |
| sym-8e17da30536a48662ee4 | `to` | function | Returns the destination owned by `PlanEdgeReceiver`. | `src/runtime/audio/router.rs:537` |
| sym-166cb316032972045648 | `topo_order` | function | Returns the topo order associated with `GraphIr`. | `src/graph/ir.rs:51` |
| sym-7ca2c113e686021a5382 | `total_bytes` | function | Returns the total bytes held by `EdgeBufferPlan`. | `src/graph/plan.rs:44` |
| sym-89af2be181b60307cb0b | `try_acquire` | function | Attempts to acquire through `AudioInputWriter`. | `src/session/extensions/audio_input/buffer.rs:107` |
| sym-3202fe1ad7d046f5a586 | `try_acquire` | function | Acquires one preallocated buffer owned by this input. | `src/session/extensions/audio_input/mod.rs:112` |
| sym-0ed55f1e90f999634106 | `try_clone` | function | Attempts to clone through `SharedAudioFrame`. | `src/frame/audio.rs:220` |
| sym-690e4b013f6b5dfe27df | `try_clone` | function | Attempts to clone through `SharedLineagedAudioFrame`. | `src/frame/audio.rs:312` |
| sym-a37013eb54b74cccac7f | `try_clone` | function | Attempts to clone through `SharedAudioBufferHandle`. | `src/frame/pool.rs:304` |
| sym-d4c335f294f449d23a76 | `try_copy_from_slice` | function | Copies samples into this fixed-capacity slot without panicking. | `src/frame/pool.rs:240` |
| sym-236abc96c450709aa6de | `try_copy_from_slice` | function | Attempts to copy from slice through `AudioInputBuffer`. | `src/session/extensions/audio_input/buffer.rs:34` |
| sym-d3f1ad03e5025f396cb1 | `try_from` | function | Attempts to from through `SidecarMessageKind`. | `src/runtime/lifecycle/sidecar_protocol.rs:25` |
| sym-10f2001491a5d174cafb | `try_new` | function | Creates a new `ConnectorAudioRecord` after validating its inputs. | `src/connector/transport.rs:300` |
| sym-83f62304819c589f4f4b | `try_new` | function | Creates a new `AudioFrame` after validating its inputs. | `src/frame/audio.rs:61` |
| sym-0828b915e37452bf1cc1 | `try_new` | function | Creates a new `FrameLineage` after validating its inputs. | `src/frame/lineage.rs:21` |
| sym-35bfe2e5904a7f79c1a3 | `try_new` | function | Creates a new `SignalLineage` after validating its inputs. | `src/graph/signal/lineage.rs:21` |
| sym-9296b749e3314413d49b | `try_new` | function | Creates a new `SignalTiming` after validating its inputs. | `src/graph/signal/timing.rs:14` |
| sym-2d38c15d8edcbd16f43b | `try_next` | function | Attempts to next through `CapturedFrameStream`. | `src/capture/frame_stream.rs:160` |
| sym-9ff7b9339b077183855b | `try_next_lineaged_frame` | function | Attempts to next lineaged frame through `CaptureOwner`. | `src/capture/capture_owner.rs:213` |
| sym-85887ef875bea65803e6 | `try_poll` | function | Attempts to poll through `PolledAudioReceipt`. | `src/endpoint/polled_audio_driver.rs:110` |
| sym-c7dadd1112d86237c689 | `try_poll_audio` | function | Attempts to poll audio through `RunningSession`. | `src/lib.rs:806` |
| sym-998f59082a33a1ea1879 | `try_receive_sidecar_signal` | function | Attempts to receive sidecar signal through `RunningSession`. | `src/lib.rs:868` |
| sym-58956f95e61f3dfc2b79 | `try_receive_sidecar_signal` | function | Attempts to receive sidecar signal through `RunningSession`. | `src/session/lifecycle/running.rs:272` |
| sym-dc1b416b106fdeed8fa1 | `try_receive_signal` | function | Attempts to receive signal through `SidecarHost`. | `src/runtime/lifecycle/sidecar_host.rs:299` |
| sym-3ec7dd8424f8012b44e4 | `try_recv` | function | Attempts to receive the next value from `SourceRuntimeEventReceiver` without waiting. | `src/capture/events.rs:304` |
| sym-00b7264beee8a8158b9d | `try_recv` | function | Attempts to receive the next value from `EndpointAudioReceiver` without waiting. | `src/endpoint/contract.rs:110` |
| sym-35d3830bf4f780585bae | `try_recv` | function | Attempts to receive the next value from `EndpointSignalReceiver` without waiting. | `src/endpoint/contract.rs:162` |
| sym-64e490e9924de0b9cb3b | `try_recv` | function | Pops one queued frame before sampling the monotonic process clock. | `src/runtime/audio/router.rs:566` |
| sym-e8d67bcabbeda3162bc8 | `try_recv` | function | Attempts to receive the next value from `SessionEventReceiver` without waiting. | `src/session/lifecycle/events.rs:506` |
| sym-7470a0b85562dbcd0da5 | `try_recv_event` | function | Attempts to recv event through `RunningSession`. | `src/lib.rs:831` |
| sym-cc9020046d219348d0ec | `try_recv_for_testing` | function | Attempts to recv for testing through `PlanSourceInput`. | `src/runtime/audio/runner.rs:213` |
| sym-5858fde37bc6fdfd2345 | `try_recv_runtime_event` | function | Attempts to recv runtime event through `CaptureOwner`. | `src/capture/capture_owner.rs:205` |
| sym-258e94793e00760fa96c | `try_send` | function | Publishes from a capture worker without blocking. When the bounded control channel is full, the newest event is dropped and counted. | `src/capture/events.rs:232` |
| sym-56d3152240fe4fc51432 | `try_send` | function | Attempts to send a value through `CapturedFrameSender` without waiting for capacity. | `src/capture/frame_stream.rs:109` |
| sym-4223bc9a8f345400a53c | `try_send` | function | Attempts to send a value through `PlanSourceSender` without waiting for capacity. | `src/runtime/audio/runner.rs:149` |
| sym-cdd08916c7e3716bd0d3 | `try_send` | function | Attempts to send a value through `AudioInputWriter` without waiting for capacity. | `src/session/extensions/audio_input/buffer.rs:147` |
| sym-12ed8088ffb4685abea5 | `try_send` | function | Submits one previously acquired buffer without blocking. | `src/session/extensions/audio_input/mod.rs:122` |
| sym-3c43dd81280d318ff9ca | `try_send_sidecar_signal` | function | Attempts to send sidecar signal through `RunningSession`. | `src/lib.rs:860` |
| sym-c78e880308afbe3a7505 | `try_send_sidecar_signal` | function | Attempts to send sidecar signal through `RunningSession`. | `src/session/lifecycle/running.rs:260` |
| sym-0890667724f142094eb6 | `try_send_signal` | function | Attempts to send signal through `SidecarHost`. | `src/runtime/lifecycle/sidecar_host.rs:265` |
| sym-e9167f9b18af1e32aad6 | `try_set_len` | function | Changes the visible sample length without panicking. | `src/frame/pool.rs:228` |
| sym-81f45c8ba6aefdac63bf | `try_set_sample_count` | function | Attempts to set sample count through `AudioInputBuffer`. | `src/session/extensions/audio_input/buffer.rs:27` |
| sym-493881808d15ae7e825f | `try_write` | function | Attempts to write through `AudioInputWriter`. | `src/session/extensions/audio_input/buffer.rs:135` |
| sym-eeeb392ffc8c1974c478 | `try_write` | function | Writes one complete interleaved frame without blocking. | `src/session/extensions/audio_input/mod.rs:117` |
| sym-0bd81d1b03195e37a8c3 | `type_id` | function | Returns the type identifier held by `NodeDescriptor`. | `src/graph/node.rs:222` |
| sym-4acc73f21d0ef46c885f | `type_ids` | function | Returns the type identifiers held by `NodeRegistry`. | `src/graph/registry.rs:179` |
| sym-1baffffec7b4283a4fb3 | `type_str` | function | Returns the type str associated with `ResolvedNode`. | `src/graph/ir.rs:19` |
| sym-28db26cc562bb969a6a2 | `typed_edge` | function | Returns the typed edge associated with `RuntimePlan`. | `src/graph/plan.rs:152` |
| sym-907d6ccce98c264c9a10 | `underrun_count` | function | Returns the underrun count held by `SystemOutputTelemetry`. | `src/runtime/nodes.rs:61` |
| sym-f993040e646ca6b9454e | `untracked` | function | Creates an envelope for data that has not yet entered a source-aware Session. Session sources must attach lineage before routing it. | `src/graph/signal/envelope.rs:17` |
| sym-4b70b02be3b032f7dc81 | `upstream_lineage` | function | Returns the upstream lineage held by `SignalDerivation`. | `src/graph/signal/lineage.rs:138` |
| sym-0c7b7dd6fd5c06082796 | `upstream_timing` | function | Returns the upstream timing associated with `SignalDerivation`. | `src/graph/signal/lineage.rs:141` |
| sym-04aa2dfac6eac63b8ad0 | `validate` | function | Validates `ConnectorManifest` against its declared contract. | `src/connector/manifest.rs:160` |
| sym-1a9b138a5be7f705864e | `validate` | function | Validates `SignalEnvelope` against its declared contract. | `src/graph/signal/envelope.rs:117` |
| sym-de5e159b7b99c2a19739 | `validate` | function | Validates `AsyncOperatorManifest` against its declared contract. | `src/graph/signal/operator.rs:238` |
| sym-9f7dee1de2898f998aab | `validate` | function | Validates `SignalSpec` against its declared contract. | `src/graph/signal/spec.rs:328` |
| sym-1a59cb76f267b697b2ad | `validate` | function | Validates `GeneratedAudioBridgeSpec` against its declared contract. | `src/runtime/bridge/audio.rs:31` |
| sym-b8f83b87710e89ce0b1a | `validate` | function | Validates `SidecarMessage` against its declared contract. | `src/runtime/lifecycle/sidecar_protocol.rs:195` |
| sym-8c4723bb4bc213174ca8 | `validate` | function | Validates `SessionSpec` against its declared contract. | `src/session/declaration/spec.rs:351` |
| sym-98724a05e3d8ea99d290 | `validate` | function | Validates `SourceManifest` against its declared contract. | `src/session/extensions/source.rs:182` |
| sym-947cafada00473194dbc | `validate` | function | Validates `SessionTrace` against its declared contract. | `src/session/lifecycle/trace.rs:280` |
| sym-f633d73f182b59133294 | `validate_config` | function | Validates supplied node configuration against the schema declared by `PassthroughFactory`. | `src/graph/builtins.rs:82` |
| sym-c4442f34c129c1d217e7 | `validate_config` | function | Validates supplied node configuration against the schema declared by `GainFactory`. | `src/graph/builtins.rs:122` |
| sym-fd6807d3d194959e8584 | `validate_config` | function | Validates supplied node configuration against the schema declared by `MonoMixFactory`. | `src/graph/builtins.rs:181` |
| sym-cf0afdda5671d8271d09 | `validate_config` | function | Validates supplied node configuration against the schema declared by `NodeDefinitionRef`. | `src/graph/registry.rs:47` |
| sym-fe880ea33c288fc4358d | `validate_config` | function | Validates supplied node configuration against the schema declared by `SystemOutputSourceFactory`. | `src/runtime/nodes.rs:99` |
| sym-b270e31a3294f7d3fb01 | `validate_config` | function | Validates supplied node configuration against the schema declared by `BridgeSinkFactory`. | `src/runtime/nodes.rs:203` |
| sym-615e253c0b08099ba7e0 | `validate_config` | function | Validates supplied node configuration against the schema declared by `SourceRegistry`. | `src/session/extensions/source.rs:313` |
| sym-f0ed0a688dd79c5febf5 | `validate_frame_sample_count` | function | Validate an interleaved frame length without reading its samples. | `src/codec/encoder.rs:210` |
| sym-7b1364d0103e3e9bcd64 | `value` | function | Returns the value held by `SourceInstanceId`. | `src/session/declaration/spec.rs:21` |
| sym-02865b2172278b00fb32 | `value` | function | Returns the value held by `OperatorInstanceId`. | `src/session/declaration/spec.rs:34` |
| sym-b7a32595b7e5e78d9c01 | `value_kind` | function | Returns the value kind held by `ConnectorConfigurationField`. | `src/connector/configuration.rs:210` |
| sym-290a3bc618061cf45f33 | `version` | function | Returns the version associated with `SessionSpec`. | `src/session/declaration/spec.rs:315` |
| sym-ff3d056b1d34bd3f2351 | `visited` | function | Returns the visited held by `SidecarHostSnapshot`. | `src/runtime/lifecycle/sidecar_host.rs:146` |
| sym-81cb0865578354002668 | `voice_broadcast` | function | Standard 20 ms mono voice transport profile with in-band FEC. | `src/codec/encoder.rs:108` |
| sym-115a5b7c97e09def78ff | `wait_audio` | function | Waits for audio for `RunningSession`. | `src/lib.rs:810` |
| sym-7877b477089b3893c67b | `wait_for_stop` | function | Waits until a stop request is visible to `ConnectorContext`. | `src/connector/worker/coordination.rs:40` |
| sym-e453bdef6eaa09f39117 | `wait_poll` | function | Waits for a batch until the finite deadline expires. | `src/endpoint/polled_audio_driver.rs:176` |
| sym-a574f6c2895abcc48958 | `wait_ready` | function | Waits for ready for `AsyncOperatorObservationHandle`. | `src/runtime/signal/observations.rs:72` |
| sym-19f6e3b98d514625aa9a | `wait_terminal` | function | Waits for terminal for `AsyncOperatorObservationHandle`. | `src/runtime/signal/observations.rs:86` |
| sym-b8f00f5913d4dc0b8ea1 | `wire_id` | function | Stable language-neutral identifier for the fundamental wire class. Semantic role and schema remain separate fields. | `src/graph/signal/spec.rs:236` |
| sym-16abcbb0e7ee8b036215 | `with` | function | Returns `ConnectorConfiguration` with the supplied entry applied. | `src/connector/configuration.rs:121` |
| sym-5c3a13ef33fc1607a0a2 | `with` | function | Returns `NodeConfig` with the supplied entry applied. | `src/graph/node.rs:66` |
| sym-d4ada7e3cd7c03fbec9b | `with` | function | Returns `EndpointConfiguration` with the supplied entry applied. | `src/session/declaration/endpoint.rs:37` |
| sym-4e45baf748617df48a70 | `with_backpressure` | function | Sets the backpressure on `EdgeContract` and returns the updated value. | `src/graph/ports.rs:370` |
| sym-279915258c10c0a210dc | `with_capability` | function | Sets the capability on `ConnectorManifest` and returns the updated value. | `src/connector/manifest.rs:113` |
| sym-2ec3fc0531f952bbe505 | `with_capture_backends` | function | Creates the standard Session host builder with caller-owned capture backends. This is the reuse seam for CLIs, tests, and platform adapters that decorate native capture without rebuilding Session semantics. | `src/session/lifecycle/host.rs:200` |
| sym-12bc89c2553a5ab2528e | `with_channels` | function | Decoder for an explicit channel layout and a maximum 20 ms packet. | `src/codec/decoder.rs:44` |
| sym-af5bbb2762545cd8b0be | `with_configuration` | function | Sets the configuration on `EndpointDescriptor` and returns the updated value. | `src/session/declaration/endpoint.rs:127` |
| sym-4e4a1d1eb2e93242edf6 | `with_constraint` | function | Sets the constraint on `ConnectorConfigurationField` and returns the updated value. | `src/connector/configuration.rs:195` |
| sym-a9ae595a7b53ac1d3a85 | `with_copy_policy` | function | Sets the copy policy on `EdgeContract` and returns the updated value. | `src/graph/ports.rs:375` |
| sym-e30c7ea084f435683404 | `with_derivation` | function | Sets the derivation on `SignalEnvelope` and returns the updated value. | `src/graph/signal/envelope.rs:57` |
| sym-5db6054df927f0d88625 | `with_driver` | function | Builds a connector whose bounded receiver loop is owned by Core. | `src/connector/mod.rs:88` |
| sym-1b243564c06db150e470 | `with_duration_ns` | function | Sets the duration nanoseconds on `SignalTiming` and returns the updated value. | `src/graph/signal/timing.rs:47` |
| sym-7b240a56f2c38a14cb4d | `with_external_details` | function | Attaches stable external failure details without changing Endpoint's provider-neutral lifecycle authority. | `src/endpoint/runtime.rs:194` |
| sym-11c9ffbba6a387fd8a32 | `with_input_edge` | function | Declares the bounded delivery policy for routes entering this endpoint. | `src/session/declaration/endpoint.rs:136` |
| sym-2af95d23d18cceb763aa | `with_jitter_budget_ms` | function | Sets the jitter budget milliseconds on `EdgeContract` and returns the updated value. | `src/graph/ports.rs:380` |
| sym-1e65e79dd7b779a051a4 | `with_lineage` | function | Sets the lineage on `SignalEnvelope` and returns the updated value. | `src/graph/signal/envelope.rs:51` |
| sym-69d0d4189eca9444a498 | `with_max_frame_duration` | function | Decoder with an explicit maximum packet duration. | `src/codec/decoder.rs:53` |
| sym-555b47b19472a4dc3f3d | `with_max_payload_bytes` | function | Sets the max payload bytes on `EdgeContract` and returns the updated value. | `src/graph/ports.rs:385` |
| sym-63bcba5c6bc3f69d83f4 | `with_media` | function | Sets the media on `EdgeContract` and returns the updated value. | `src/graph/ports.rs:365` |
| sym-840d33f0f72a5f846357 | `with_requirement` | function | Sets the requirement on `ConnectorManifest` and returns the updated value. | `src/connector/manifest.rs:119` |
| sym-791f270dedc9b5e1803d | `with_role` | function | Attach a semantic role annotation. | `src/graph/signal/spec.rs:309` |
| sym-5de4cecef3f1cd823fd9 | `with_schema` | function | Attach a schema reference. | `src/graph/signal/spec.rs:315` |
| sym-887474f5adecb94d44db | `with_sensitive` | function | Adds a setup-time value whose normal debug representation is redacted. | `src/graph/node.rs:81` |
| sym-fd441cd1e21faaae2cf8 | `with_sensitive` | function | Adds a setup-time value whose normal debug representation is redacted. | `src/session/declaration/endpoint.rs:49` |
| sym-e0d2ab074e333ae191b6 | `worker_mappings` | function | Returns the worker mappings associated with `PreparedSession`. | `src/session/prepare/prepared.rs:51` |
| sym-4a0341e60a1d5320c50d | `writer` | function | Returns the writer held by `PcmSource`. | `src/session/extensions/audio_input/source.rs:60` |
| sym-618f67eb8db37243cf7f | `writer_mut` | function | Returns the writer mut held by `PcmSource`. | `src/session/extensions/audio_input/source.rs:64` |
| sym-fd1a5e4bf81be429c380 | `audio` | module | Realtime audio routing, execution, plan-runner, and runtime observation types. | `src/frame/audio.rs:1` |
| sym-3122516e1e820e426e0a | `audio` | module | Allocation-free realtime audio execution lane. | `src/runtime/audio/mod.rs:1` |
| sym-cba59f674b927f6a3e94 | `authorization` | module | Explicit capture authorization evidence and open outcomes. | `src/capture/authorization.rs:1` |
| sym-b4ba56c43ee4555e78dd | `identity` | module | Stable source identity and source discovery state. | `src/capture/identity.rs:1` |
| sym-954fd1cc9dabb18fb2d2 | `lifecycle` | module | Non-realtime runtime ownership and process-protocol lifecycle. | `src/runtime/lifecycle/mod.rs:1` |
| sym-d9e7c0a66c2f232e1613 | `lineage` | module | Compact source-aware lineage carried on realtime audio frames. | `src/frame/lineage.rs:1` |
| sym-6765d130ad10c007cc70 | `pocketstation` | module | # PocketStation | `src/lib.rs:1` |
| sym-e739e53f46a6fb3cfe72 | `pocketstation::codec` | module | Real Opus encode, decode, and packet-loss concealment primitives. | `src/codec/mod.rs:1` |
| sym-23e283d6070b62ee135c | `pocketstation::conformance` | module | Deterministic Session fixture for external conformance harnesses. | `src/conformance.rs:1` |
| sym-2759496bc4da50b77077 | `pocketstation::connector` | module | Connector manifests, configuration, workers, transport records, readiness, and observations. | `src/connector/mod.rs:1` |
| sym-85c70119164eb3ec742a | `pocketstation::graph` | module | Stable signal, port, capability, partition, and extension contracts. | `src/graph/mod.rs:1` |
| sym-87bc206feb231edfe436 | `pocketstation::runtime::nodes` | module | First-party CLI realtime nodes retained behind `internal-testing`. | `src/runtime/nodes.rs:1` |
| sym-46f35620ed864442b2a0 | `pocketstation::timing` | module | Timing primitives owned by the PocketStation runtime. | `src/timing/mod.rs:1` |
| sym-da89361ec9773f03cfbe | `pool` | module | Fixed-capacity realtime audio storage and ownership handles. | `src/frame/pool.rs:1` |
| sym-549a2b77c08e128f796b | `query` | module | Control-plane source discovery queries used by the first-party CLI. | `src/capture/query.rs:1` |
| sym-419801ec17e4626ad6d5 | `selection` | module | Capture selection semantics; control-plane only. | `src/capture/selection.rs:1` |
| sym-4b2712e20fb3f784bcf7 | `signal` | module | Bounded asynchronous signal execution lane. | `src/runtime/signal/mod.rs:1` |
| sym-b71bf70731725bc71aa4 | `pocketstation::RunningSession` | struct | Owns a started Session together with event, polling, recording, trace, and stop resources. | `src/lib.rs:789` |
| sym-15edc7fb76d06b9387d9 | `pocketstation::Session` | struct | Owns a mutable Session declaration and the host configuration used to compile, prepare, and start it. | `src/lib.rs:236` |
| sym-aa6ea6a26c5720597f40 | `pocketstation::SessionBuilder` | struct | Setup-time configuration for the public Rust Session. | `src/lib.rs:275` |
| sym-fc408df50a6503df7617 | `pocketstation::SessionCancelResult` | struct | Reports the structured session cancel result. | `src/lib.rs:1123` |
| sym-e1c322720a72c2656952 | `pocketstation::SessionStartError` | struct | Stable façade error for Session startup. | `src/lib.rs:955` |
| sym-ac6ffc677b9f7ea03239 | `pocketstation::SessionStopResult` | struct | Reports the structured session stop result. | `src/lib.rs:1143` |
| sym-b9ca441db554330302d9 | `pocketstation::abi::executable_extension::PksExtensionCallbacks` | struct | Defines the optional function table through which a native extension prepares, runs, stops, and releases instances. | `src/abi/executable_extension.rs:91` |
| sym-ab00ae321687f73286ca | `pocketstation::abi::executable_extension::PksExtensionLibrary` | struct | Owns a loaded native-extension library and the registrations imported from its validated descriptor. | `src/abi/executable_extension.rs:123` |
| sym-f632b21433ce6ab18231 | `pocketstation::abi::executable_extension::PksExtensionPipelineDeclaration` | struct | Declares one extension pipeline instance and the native registrations it uses. | `src/abi/executable_extension.rs:168` |
| sym-a6d4a635612b1aaf4d15 | `pocketstation::abi::executable_extension::PksExtensionSignalBuffer` | struct | Provides bounded extension-owned storage for a signal returned through the native ABI. | `src/abi/executable_extension.rs:153` |
| sym-f18199c6541f4add2eeb | `pocketstation::abi::executable_extension::PksExtensionSignalView` | struct | Borrows one signal payload and metadata for delivery into a native-extension callback. | `src/abi/executable_extension.rs:138` |
| sym-7c08b84faec371e236f1 | `pocketstation::abi::extension::PksExtensionAbiVersion` | struct | Carries the major and minor native-extension ABI versions checked during loading. | `src/abi/extension.rs:14` |
| sym-271409255d3a0750ce7b | `pocketstation::abi::extension::PksExtensionDescriptor` | struct | Declares a native extension's ABI version, library callbacks, and registration entrypoint. | `src/abi/extension.rs:47` |
| sym-7b822cc6a15684bd7f84 | `pocketstation::abi::extension::PksExtensionPort` | struct | Describes one native-extension port across the C ABI, including direction and signal metadata. | `src/abi/extension.rs:60` |
| sym-0ed9edb0f5260fbc6220 | `pocketstation::abi::session::abi::PksSessionStatus` | struct | Reports the structured session status. | `src/abi/session/abi.rs:56` |
| sym-0d106bb641ad4be881c4 | `pocketstation::abi::session::abi::PksSessionUtf8` | struct | Borrows a UTF-8 byte range across the C Session ABI as a pointer and length. | `src/abi/session/abi.rs:101` |
| sym-895ca8f26ee62f0755a3 | `pocketstation::capture::authorization::CaptureAuthorizationSnapshot` | struct | Point-in-time authorization evidence for opening one exact capture source. | `src/capture/authorization.rs:17` |
| sym-be7aba4ae15d283efb2a | `pocketstation::capture::authorization::CapturePermissionLifecycle` | struct | Control-plane owner for one source's observed authorization epoch. | `src/capture/authorization.rs:183` |
| sym-6aa8b39acb3133ed2c09 | `pocketstation::capture::authorization::CapturePermissionTransition` | struct | One authoritative authorization-state transition observed by the host. | `src/capture/authorization.rs:168` |
| sym-74330e7cd8c298c34020 | `pocketstation::capture::authorization::PermissionEpoch` | struct | Identifies the permission-observation generation attached to captured lineage. | `src/capture/authorization.rs:267` |
| sym-801e41161f622e4c8375 | `pocketstation::capture::capture_owner::CaptureDelivery` | struct | Callback delivery endpoints transferred to a prepared native backend. | `src/capture/capture_owner.rs:73` |
| sym-c61304d3358e379705a0 | `pocketstation::capture::capture_owner::CaptureLineageSeed` | struct | Stable session and stem identity assigned before an exact source is opened. | `src/capture/capture_owner.rs:25` |
| sym-a009e79af7cb329a10a4 | `pocketstation::capture::capture_owner::CaptureObservationReceipt` | struct | Retains the identity and observation access returned for capture observation. | `src/capture/capture_owner.rs:167` |
| sym-8b016df4d1351142eb09 | `pocketstation::capture::capture_owner::CaptureOpenMetadata` | struct | Authoritative lineage state established only after native capture opens. | `src/capture/capture_owner.rs:49` |
| sym-0062f094cf04ce808da1 | `pocketstation::capture::capture_owner::CaptureOwner` | struct | RAII owner for native capture, its bounded frame stream, and runtime events. | `src/capture/capture_owner.rs:194` |
| sym-754798c8762195990799 | `pocketstation::capture::capture_owner::CaptureOwnerObservations` | struct | Aggregate observations from one active capture ownership boundary. | `src/capture/capture_owner.rs:160` |
| sym-0ff19b79d15eb60b7944 | `pocketstation::capture::capture_owner::CapturePrepareRequest` | struct | Setup-time request for one bounded callback-oriented capture owner. | `src/capture/capture_owner.rs:61` |
| sym-ae71a73019da4eedbfdd | `pocketstation::capture::capture_owner::CaptureStopOutcome` | struct | Final observations returned only after backend stop and join complete. | `src/capture/capture_owner.rs:185` |
| sym-f68a0563e3fbb06758fd | `pocketstation::capture::capture_owner::PreparedCapture` | struct | Prepared capture plus its preallocated delivery endpoints. | `src/capture/capture_owner.rs:119` |
| sym-b6f34d0d3b18e6cab4b9 | `pocketstation::capture::events::CaptureRuntimeFailure` | struct | Reports a capture runtime failure. | `src/capture/events.rs:47` |
| sym-583e7e4b72bbe6e2124f | `pocketstation::capture::events::SourceGeneration` | struct | Identifies one appearance generation of a capture source across loss and reappearance. | `src/capture/events.rs:12` |
| sym-926445cfdcc4a65cb18b | `pocketstation::capture::events::SourceRuntimeEventObservationHandle` | struct | Holds the ownership or bounded access represented by source runtime event observation handle. | `src/capture/events.rs:200` |
| sym-a3891e64195ad9eefd10 | `pocketstation::capture::events::SourceRuntimeEventObservations` | struct | Reports the source runtime event observations collected at an observation boundary. | `src/capture/events.rs:111` |
| sym-0a97a8c5cc04ee68954b | `pocketstation::capture::events::SourceRuntimeEventReceiver` | struct | Receives source runtime event values across its declared ownership boundary. | `src/capture/events.rs:298` |
| sym-4100bfd509bc312470bf | `pocketstation::capture::events::SourceRuntimeEventSender` | struct | Sends source runtime event values across its declared ownership boundary. | `src/capture/events.rs:224` |
| sym-df376380871adcda955a | `pocketstation::capture::frame_stream::CaptureDeliveryStartGate` | struct | Read-only one-way start barrier checked by capture delivery callbacks. | `src/capture/frame_stream.rs:54` |
| sym-509e830b388e193452d0 | `pocketstation::capture::frame_stream::CaptureDeliveryStartGateController` | struct | Session-owned authority that opens one capture delivery start gate. | `src/capture/frame_stream.rs:72` |
| sym-4e918c3304fa3607c83d | `pocketstation::capture::frame_stream::CapturedFrameObservationHandle` | struct | Holds the ownership or bounded access represented by captured frame observation handle. | `src/capture/frame_stream.rs:31` |
| sym-011bce3cc267f0f461f1 | `pocketstation::capture::frame_stream::CapturedFrameSender` | struct | Single-producer endpoint passed into a platform capture callback. | `src/capture/frame_stream.rs:102` |
| sym-e50aeb0b46ace0193171 | `pocketstation::capture::frame_stream::CapturedFrameStream` | struct | Non-blocking consumer for captured `AudioFrame`s. | `src/capture/frame_stream.rs:154` |
| sym-8a20db7016746746f7a2 | `pocketstation::capture::frame_stream::CapturedFrameStreamStats` | struct | Reports the captured frame stream stats collected at an observation boundary. | `src/capture/frame_stream.rs:17` |
| sym-2900ed324c0cddac02d7 | `pocketstation::capture::identity::CaptureSource` | struct | Owns production of capture values and its lifecycle state. | `src/capture/identity.rs:82` |
| sym-2f4135245cffde0ce33f | `pocketstation::capture::identity::StableSourceId` | struct | Uniquely identifies stable source within its PocketStation ownership scope. | `src/capture/identity.rs:26` |
| sym-3a5b02712461d3956ee1 | `pocketstation::capture::lifecycle_registry::SourceLifecycleRegistry` | struct | Assigns source generations across complete discovery snapshots. | `src/capture/lifecycle_registry.rs:31` |
| sym-f002a9e3f754a8ed3282 | `pocketstation::capture::observations::CaptureObservationCounters` | struct | Setup-time cloneable handle; every observation is one relaxed atomic operation and remains allocation-free, lock-free, and log-free. | `src/capture/observations.rs:46` |
| sym-603db656d3f59b7c5d95 | `pocketstation::capture::observations::CaptureObservationHandle` | struct | Holds the ownership or bounded access represented by capture observation handle. | `src/capture/observations.rs:32` |
| sym-ad8a7d734a7b06b83340 | `pocketstation::capture::observations::CaptureObservations` | struct | Reports the capture observations collected at an observation boundary. | `src/capture/observations.rs:8` |
| sym-fdde89910590812c39a3 | `pocketstation::capture::platform::macos::DesktopCaptureSource` | struct | Owns production of desktop capture values and its lifecycle state. | `src/capture/platform/macos/mod.rs:33` |
| sym-f278f1a4ffd61d8c0158 | `pocketstation::capture::platform::macos::input::MacosInputSource` | struct | Owns production of macos input values and its lifecycle state. | `src/capture/platform/macos/input.rs:65` |
| sym-f32c9d0ff1652848c4b6 | `pocketstation::capture::platform::macos::loopback::SystemLoopbackSource` | struct | Manages a macOS loopback capture session. | `src/capture/platform/macos/loopback.rs:53` |
| sym-ef99ea56dfe5149e45a8 | `pocketstation::capture::platform::macos::session_backend::DesktopCaptureBackend` | struct | macOS adapter from the platform-neutral Session capture contract to the existing CoreAudio/input RAII owner. | `src/capture/platform/macos/session_backend.rs:11` |
| sym-1e2e36ccd063adac6c20 | `pocketstation::capture::query::LocalSourceProvider` | struct | Discovers and resolves capture sources through the target platform backend. | `src/capture/query.rs:52` |
| sym-d588746d1ca3571cff04 | `pocketstation::capture::timeline::CaptureSampleTimeline` | struct | Source-time clock for capture streams whose media cadence is defined by the number of sample frames produced by the device. | `src/capture/timeline.rs:31` |
| sym-900e448d428e76f25c9a | `pocketstation::codec::decoder::OpusDecoder` | struct | Real Opus decoder wrapping libopus via the `opus` crate. | `src/codec/decoder.rs:15` |
| sym-5b5d7a123f884a58a5bd | `pocketstation::codec::encoder::OpusConfig` | struct | Explicit configuration for an Opus encoder instance. | `src/codec/encoder.rs:72` |
| sym-b9d251ce5c10d6980b9b | `pocketstation::codec::encoder::OpusEncoder` | struct | Real Opus encoder wrapping libopus via the `opus` crate. | `src/codec/encoder.rs:157` |
| sym-80d7db80c1e6f2bf10f0 | `pocketstation::conformance::ExtensionConformanceReport` | struct | Language-neutral outcome returned by the W20 fixture. | `src/conformance.rs:572` |
| sym-63969fd30fbf20be6f60 | `pocketstation::conformance::ExtensionSignal` | struct | Owns one signal payload used by the native-extension conformance fixtures. | `src/conformance.rs:1180` |
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
| sym-5d92cd6abbf10b538cae | `pocketstation::endpoint::contract::EndpointAudioFrame` | struct | Read-only audio frame delivered to an external endpoint. | `src/endpoint/contract.rs:18` |
| sym-cb8259906b02d741922e | `pocketstation::endpoint::contract::EndpointAudioReceiver` | struct | Exclusive consumer for one bounded realtime-audio endpoint edge. | `src/endpoint/contract.rs:92` |
| sym-ec411b1248474f059cb2 | `pocketstation::endpoint::contract::EndpointPortInput` | struct | Carries typed input for endpoint port. | `src/endpoint/contract.rs:183` |
| sym-f73348a54557eb092c7e | `pocketstation::endpoint::contract::EndpointSignalReceiver` | struct | Exclusive consumer for one bounded asynchronous signal endpoint edge. | `src/endpoint/contract.rs:153` |
| sym-4bb86447850f820d560d | `pocketstation::endpoint::identity::EndpointGroupId` | struct | Explicit Session-scoped grouping key for endpoints that share one lifecycle. | `src/endpoint/identity.rs:9` |
| sym-086112341efa8cc4d7a6 | `pocketstation::endpoint::polled_audio::PolledAudioEndpoint` | struct | Declares application-polled audio and retains its bounded receipt. | `src/endpoint/polled_audio.rs:16` |
| sym-c8c14c5956491c4ccbc8 | `pocketstation::endpoint::polled_audio_driver::PolledAudioBatchLease` | struct | Holds the ownership or bounded access represented by polled audio batch lease. | `src/endpoint/polled_audio_driver.rs:218` |
| sym-ce53392dc25d08a0be07 | `pocketstation::endpoint::polled_audio_driver::PolledAudioEndpointConfig` | struct | Configures polled audio endpoint behavior at its owning API boundary. | `src/endpoint/polled_audio_driver.rs:23` |
| sym-b48b465032534fae4941 | `pocketstation::endpoint::polled_audio_driver::PolledAudioFrame` | struct | Carries one polled audio payload together with its declared metadata. | `src/endpoint/polled_audio_driver.rs:256` |
| sym-e19b0ef994fca1143d6a | `pocketstation::endpoint::polled_audio_driver::PolledAudioObservations` | struct | Reports the polled audio observations collected at an observation boundary. | `src/endpoint/polled_audio_driver.rs:56` |
| sym-41e3b3d9082325e3e017 | `pocketstation::endpoint::polled_audio_driver::PolledAudioReceipt` | struct | Retains the identity and observation access returned for polled audio. | `src/endpoint/polled_audio_driver.rs:105` |
| sym-6c4c9e9d93ce90fd85ee | `pocketstation::endpoint::registry::EndpointDriverRegistry` | struct | Indexes registered endpoint driver implementations by their stable identities. | `src/endpoint/registry.rs:54` |
| sym-e967ffd7c9fe16ee8764 | `pocketstation::endpoint::runtime::EndpointCancellationOutcome` | struct | Reports the structured endpoint cancellation outcome. | `src/endpoint/runtime.rs:289` |
| sym-33d3d9274786c3d40f79 | `pocketstation::endpoint::runtime::EndpointDriverFinalization` | struct | Reports an endpoint driver's terminal observations and any finalization failure. | `src/endpoint/runtime.rs:295` |
| sym-5f26c4f369af94e22c4b | `pocketstation::endpoint::runtime::EndpointDriverObservations` | struct | Reports the endpoint driver observations collected at an observation boundary. | `src/endpoint/runtime.rs:228` |
| sym-c5098d60dc65d940b524 | `pocketstation::endpoint::runtime::EndpointFailure` | struct | Classifies failures surfaced by endpoint failure operations. | `src/endpoint/runtime.rs:174` |
| sym-947b30b6d4b4443dc7af | `pocketstation::endpoint::runtime::EndpointFinalizationOutcome` | struct | Reports the structured endpoint finalization outcome. | `src/endpoint/runtime.rs:301` |
| sym-1398b98a6ac10d07b508 | `pocketstation::endpoint::runtime::EndpointPrepareContext` | struct | Carries the inputs and runtime context required to endpoint prepare. | `src/endpoint/runtime.rs:98` |
| sym-f0bba6959b4f6c86b37d | `pocketstation::endpoint::runtime::EndpointRouteContext` | struct | Typed Session route identity supplied to every endpoint input. | `src/endpoint/runtime.rs:44` |
| sym-37b086b66e8a7ff7a807 | `pocketstation::endpoint::runtime::EndpointStartFailure` | struct | Classifies failures surfaced by endpoint start failure operations. | `src/endpoint/runtime.rs:443` |
| sym-4aa43f188989d8622704 | `pocketstation::endpoint::runtime::EndpointStartGate` | struct | Read-only one-way start barrier shared by endpoint drivers in one startup. | `src/endpoint/runtime.rs:371` |
| sym-fc2716255038bebf5351 | `pocketstation::endpoint::runtime::PreparedEndpoint` | struct | Owns endpoint resources after preparation and before its runtime driver starts. | `src/endpoint/runtime.rs:405` |
| sym-8ce8ed6399db88e1031c | `pocketstation::endpoint::runtime::RunningEndpoint` | struct | Owns a started endpoint driver until shutdown and finalization complete. | `src/endpoint/runtime.rs:481` |
| sym-cbde7491554cf1458df0 | `pocketstation::endpoint::runtime::SessionTimelineOrigin` | struct | One Session-owned anchor in PocketStation's monotonic nanosecond clock. | `src/endpoint/runtime.rs:13` |
| sym-8577d9382df72191707a | `pocketstation::frame::audio::AudioFrame` | struct | Carries one audio payload together with its declared metadata. | `src/frame/audio.rs:39` |
| sym-e57bd2ef111f9b1c749b | `pocketstation::frame::audio::LineagedAudioFrame` | struct | An exclusive audio frame and the immutable lineage snapshot captured before the frame crosses a bounded edge. | `src/frame/audio.rs:266` |
| sym-2e4f5994a3848fc24f17 | `pocketstation::frame::audio::SampleSpec` | struct | Configures sample behavior at its owning API boundary. | `src/frame/audio.rs:18` |
| sym-29f6af9dddfae2ea461b | `pocketstation::frame::audio::SharedAudioFrame` | struct | Carries one shared audio payload together with its declared metadata. | `src/frame/audio.rs:176` |
| sym-31af59b4777ad01c6d45 | `pocketstation::frame::audio::SharedLineagedAudioFrame` | struct | Carries one shared lineaged audio payload together with its declared metadata. | `src/frame/audio.rs:298` |
| sym-644caca167920c58fe6a | `pocketstation::frame::identity::ClockDomainId` | struct | Uniquely identifies clock domain within its PocketStation ownership scope. | `src/frame/identity.rs:29` |
| sym-68f6f5cb9b238976cdc6 | `pocketstation::frame::identity::ConnectorId` | struct | Uniquely identifies connector within its PocketStation ownership scope. | `src/frame/identity.rs:25` |
| sym-651bdd72775fb5995723 | `pocketstation::frame::identity::EndpointId` | struct | Uniquely identifies endpoint within its PocketStation ownership scope. | `src/frame/identity.rs:24` |
| sym-e70f709a3e484b9d5d5c | `pocketstation::frame::identity::RouteId` | struct | Uniquely identifies route within its PocketStation ownership scope. | `src/frame/identity.rs:26` |
| sym-906bbaa23baea185d5aa | `pocketstation::frame::identity::SessionId` | struct | Uniquely identifies session within its PocketStation ownership scope. | `src/frame/identity.rs:22` |
| sym-009b1169b888de1dce8b | `pocketstation::frame::identity::SourceId` | struct | Uniquely identifies source within its PocketStation ownership scope. | `src/frame/identity.rs:21` |
| sym-82a2b9584ea1d7a2dae6 | `pocketstation::frame::identity::StemId` | struct | Uniquely identifies stem within its PocketStation ownership scope. | `src/frame/identity.rs:23` |
| sym-0010fb7e6f1802e2c0c1 | `pocketstation::frame::identity::StreamId` | struct | Uniquely identifies stream within its PocketStation ownership scope. | `src/frame/identity.rs:20` |
| sym-4f5d7dda4dde3e3ceaf9 | `pocketstation::frame::lineage::FrameLineage` | struct | Preserves source, stream, sequence, clock, generation, and discontinuity identity for an audio frame. | `src/frame/lineage.rs:6` |
| sym-de68a53d211a83bfc33c | `pocketstation::frame::pool::AudioBufferHandle` | struct | Holds the ownership or bounded access represented by audio buffer handle. | `src/frame/pool.rs:198` |
| sym-73a3a95b920434157c7b | `pocketstation::frame::pool::AudioBufferPool` | struct | Owns fixed-capacity reusable audio slots and reports acquisition pressure without allocating per frame. | `src/frame/pool.rs:24` |
| sym-d1d3a10ca5f3d96627df | `pocketstation::frame::pool::SharedAudioBufferHandle` | struct | Holds the ownership or bounded access represented by shared audio buffer handle. | `src/frame/pool.rs:281` |
| sym-c29c3e128aaa67f89e86 | `pocketstation::graph::builtins::GainFactory` | struct | Constructs gain implementations from validated declarations. | `src/graph/builtins.rs:107` |
| sym-8434e3c12de20a019e55 | `pocketstation::graph::builtins::GainNode` | struct | Represents the executable graph node for gain. | `src/graph/builtins.rs:149` |
| sym-b9cafd7e4da90ab989df | `pocketstation::graph::builtins::MonoMixFactory` | struct | Constructs mono mix implementations from validated declarations. | `src/graph/builtins.rs:166` |
| sym-0d65716bde974c01167b | `pocketstation::graph::builtins::MonoMixNode` | struct | Represents the executable graph node for mono mix. | `src/graph/builtins.rs:194` |
| sym-fc3b416de48719777376 | `pocketstation::graph::builtins::PassthroughFactory` | struct | Constructs passthrough implementations from validated declarations. | `src/graph/builtins.rs:67` |
| sym-9778746018188fd10a5d | `pocketstation::graph::builtins::PassthroughNode` | struct | Represents the executable graph node for passthrough. | `src/graph/builtins.rs:95` |
| sym-d9c8bf6fbddb97f3035e | `pocketstation::graph::compile::plan::RuntimePlanner` | struct | Validates the graph and produces the bounded runtime execution and memory plan. | `src/graph/compile/plan.rs:11` |
| sym-ce8d91865ad2ae1fc597 | `pocketstation::graph::compile::resolve::Compiler` | struct | Runs the ordered graph-validation passes that resolve a graph specification into executable IR. | `src/graph/compile/resolve.rs:444` |
| sym-1b8920191028f9300db3 | `pocketstation::graph::dsl::NodeHandle` | struct | Holds the ownership or bounded access represented by node handle. | `src/graph/dsl.rs:10` |
| sym-a1f5a14ee8fd7a1e22ee | `pocketstation::graph::dsl::Pipeline` | struct | Builds typed operator connections on a Session while preserving port and signal contracts. | `src/graph/dsl.rs:33` |
| sym-1c4657bf1812df484e9b | `pocketstation::graph::ir::GraphIr` | struct | Contains the resolved nodes, edges, and topological order consumed by runtime planning. | `src/graph/ir.rs:32` |
| sym-602bbc92c11bf28aee54 | `pocketstation::graph::ir::ResolvedEdge` | struct | Binds one compiled graph edge to its resolved source, destination, and contract. | `src/graph/ir.rs:25` |
| sym-5b1f5407ff29e8086598 | `pocketstation::graph::ir::ResolvedNode` | struct | Represents the executable graph node for resolved. | `src/graph/ir.rs:10` |
| sym-f0fd112e60a2a4bc4b2d | `pocketstation::graph::node::NodeConfig` | struct | Configures node behavior at its owning API boundary. | `src/graph/node.rs:43` |
| sym-d0614ee6a0352e30c911 | `pocketstation::graph::node::NodeDescriptor` | struct | Declares a graph node's stable type identity, ports, execution partition, and safety contract. | `src/graph/node.rs:165` |
| sym-bd0de0ad1dfaf278c380 | `pocketstation::graph::node::NodeTypeId` | struct | Uniquely identifies node type within its PocketStation ownership scope. | `src/graph/node.rs:13` |
| sym-7931df15c6f8f477452e | `pocketstation::graph::node::PortPrepareContext` | struct | Exact graph-owned contract for one prepared node port. | `src/graph/node.rs:282` |
| sym-9ecb81c8ab3c92c6ad05 | `pocketstation::graph::node::PrepareContext` | struct | Carries the inputs and runtime context required to prepare. | `src/graph/node.rs:266` |
| sym-01b531f2ae032a881c18 | `pocketstation::graph::operator::OperatorId` | struct | Open identifier for a registered graph operator implementation. | `src/graph/operator.rs:16` |
| sym-dc97b00dee0164659dbf | `pocketstation::graph::plan::EdgeBufferPlan` | struct | Records the compiled execution and resource plan for edge buffer. | `src/graph/plan.rs:36` |
| sym-9821786b0d47b336f36b | `pocketstation::graph::plan::EdgeMetricId` | struct | Uniquely identifies edge metric within its PocketStation ownership scope. | `src/graph/plan.rs:33` |
| sym-27aba2f641b8fc8cf218 | `pocketstation::graph::plan::FanInGroup` | struct | Groups the compiled edges mixed into one input port. | `src/graph/plan.rs:90` |
| sym-334a263efea344eeab23 | `pocketstation::graph::plan::FanOutGroup` | struct | Groups the compiled edges that share one output port as their origin. | `src/graph/plan.rs:84` |
| sym-9ee357f3c207815e97cf | `pocketstation::graph::plan::MemoryPlan` | struct | Records the compiled execution and resource plan for memory. | `src/graph/plan.rs:64` |
| sym-80b8974a0c48cb9dbd17 | `pocketstation::graph::plan::PartitionGroup` | struct | A group of nodes assigned to the same execution partition in a compiled plan. | `src/graph/plan.rs:78` |
| sym-701edcf1b6c502eccf79 | `pocketstation::graph::plan::RuntimePlan` | struct | Records the compiled execution and resource plan for runtime. | `src/graph/plan.rs:121` |
| sym-dcf16d02bf875329220f | `pocketstation::graph::plan::SourceOutputPlan` | struct | One connected output of a graph root that runtime preparation must feed. | `src/graph/plan.rs:113` |
| sym-0fcad53a1c9529c4c446 | `pocketstation::graph::plan::TypedEdgePlan` | struct | Records the compiled execution and resource plan for typed edge. | `src/graph/plan.rs:96` |
| sym-d29c15e4ec9e8fff4cf5 | `pocketstation::graph::ports::AudioCaps` | struct | Declares the sample formats, channel layouts, and rates accepted by an audio port. | `src/graph/ports.rs:48` |
| sym-15ff34ae95d4bcf91114 | `pocketstation::graph::ports::EdgeContract` | struct | Declares the validated constraints applied to edge. | `src/graph/ports.rs:311` |
| sym-494e48377a4a9f2d51ba | `pocketstation::graph::ports::PortSpec` | struct | Configures port behavior at its owning API boundary. | `src/graph/ports.rs:175` |
| sym-109ea37f2ba9e428d7f6 | `pocketstation::graph::registry::NodeRegistry` | struct | Indexes registered node implementations by their stable identities. | `src/graph/registry.rs:67` |
| sym-1f5c96e9a640a4c82f0f | `pocketstation::graph::signal::continuity::SignalContinuityObservation` | struct | Reports sequence or timestamp continuity observed for one signal stream. | `src/graph/signal/continuity.rs:6` |
| sym-7db9f05f6653b543fee2 | `pocketstation::graph::signal::continuity::SignalContinuityTracker` | struct | Tracks sequence and timing progress so discontinuities remain observable. | `src/graph/signal/continuity.rs:13` |
| sym-5c2e10e43766bb76550b | `pocketstation::graph::signal::envelope::SignalEnvelope` | struct | Carries a typed signal payload together with timing, lineage, continuity, and terminal metadata. | `src/graph/signal/envelope.rs:6` |
| sym-1b0ed5c22e621fb8c312 | `pocketstation::graph::signal::lineage::SignalDerivation` | struct | Source-independent record of the signal consumed by an operator. | `src/graph/signal/lineage.rs:97` |
| sym-36ff27ee8e452de051cf | `pocketstation::graph::signal::lineage::SignalLineage` | struct | Preserves source, stream, generation, discontinuity, and policy identity across signal processing. | `src/graph/signal/lineage.rs:8` |
| sym-bce9a1d2f2560ff42385 | `pocketstation::graph::signal::operator::AsyncOperatorManifest` | struct | Declares an asynchronous operator's ports, execution partition, failure policy, and cancellation policy. | `src/graph/signal/operator.rs:127` |
| sym-4be099089b3dc313b37b | `pocketstation::graph::signal::operator::OperatorDeadlinePolicy` | struct | Configures operator deadline behavior at its owning API boundary. | `src/graph/signal/operator.rs:52` |
| sym-b1a45126f7f1b1008593 | `pocketstation::graph::signal::operator::OperatorOutputRolePolicy` | struct | Configures operator output role behavior at its owning API boundary. | `src/graph/signal/operator.rs:69` |
| sym-519d2e4e1814f86eb84a | `pocketstation::graph::signal::operator::OperatorPermissionPolicy` | struct | Configures operator permission behavior at its owning API boundary. | `src/graph/signal/operator.rs:46` |
| sym-bfa43d7592e8acb7f8f9 | `pocketstation::graph::signal::preparation::AsyncOperatorPrepareContext` | struct | Complete graph-owned preparation contract for one asynchronous Operator. | `src/graph/signal/preparation.rs:22` |
| sym-32d89a4f5b4173ac9b77 | `pocketstation::graph::signal::spec::SchemaRef` | struct | Reference to an external schema document. | `src/graph/signal/spec.rs:87` |
| sym-ab0f69d9024dff162ecb | `pocketstation::graph::signal::spec::SemanticRole` | struct | Semantic role annotation on a port. | `src/graph/signal/spec.rs:57` |
| sym-9216553fb42e8bcd9874 | `pocketstation::graph::signal::spec::SignalId` | struct | Opaque identifier for a custom signal type. | `src/graph/signal/spec.rs:22` |
| sym-d81535bc5c0ff81d7c0e | `pocketstation::graph::signal::spec::SignalSpec` | struct | Full signal contract for a single port. | `src/graph/signal/spec.rs:205` |
| sym-f30a924a860208382b51 | `pocketstation::graph::signal::timing::SignalTiming` | struct | Carries a signal timestamp, clock domain, and timing semantics without rewriting source lineage. | `src/graph/signal/timing.rs:6` |
| sym-b03bf7fabce8759ca684 | `pocketstation::graph::spec::EdgeId` | struct | Uniquely identifies edge within its PocketStation ownership scope. | `src/graph/spec.rs:22` |
| sym-8cef43d51d8ba85781c5 | `pocketstation::graph::spec::EdgeSpec` | struct | Configures edge behavior at its owning API boundary. | `src/graph/spec.rs:50` |
| sym-77b21de9d764b1c6f4c2 | `pocketstation::graph::spec::GraphSpec` | struct | Configures graph behavior at its owning API boundary. | `src/graph/spec.rs:58` |
| sym-92d882839a2104dde44f | `pocketstation::graph::spec::InputPortRef` | struct | Names an operator or endpoint input port used as the target of a graph connection. | `src/graph/spec.rs:37` |
| sym-a1995f8b8ea4c8146b77 | `pocketstation::graph::spec::NodeId` | struct | Uniquely identifies node within its PocketStation ownership scope. | `src/graph/spec.rs:8` |
| sym-d5aa8e2768de33a52bdb | `pocketstation::graph::spec::NodeSpec` | struct | Configures node behavior at its owning API boundary. | `src/graph/spec.rs:43` |
| sym-28d6671c2db732d0a2e8 | `pocketstation::graph::spec::OutputPortRef` | struct | Names an operator output port used as the origin of a graph connection. | `src/graph/spec.rs:31` |
| sym-53933707dc1f0cf8a775 | `pocketstation::native_extension::NativeExtensionLibrary` | struct | Immutable receipt for registrations imported into one Session. Executable code ownership remains internal to the registered factories and drivers. | `src/native_extension/mod.rs:62` |
| sym-cd19944854ed52da6f88 | `pocketstation::native_extension::NativeExtensionLibraryError` | struct | Reports a native extension library error. | `src/native_extension/mod.rs:124` |
| sym-f2148d5a24dce2026a1f | `pocketstation::native_extension::NativeExtensionRegistration` | struct | Identifies one node registration imported transactionally from a native extension. | `src/native_extension/mod.rs:34` |
| sym-de855f294ed777beba6f | `pocketstation::recording::config::RecorderStemConfig` | struct | Configures recorder stem behavior at its owning API boundary. | `src/recording/config.rs:55` |
| sym-24870ae1cdb40ba62366 | `pocketstation::recording::config::StemLabel` | struct | Stores the validated human-readable label used for one recording stem. | `src/recording/config.rs:20` |
| sym-38051cba9e220e6e06d4 | `pocketstation::recording::endpoint::MultistemRecordingReceipt` | struct | Retains the identity and observation access returned for multistem recording. | `src/recording/endpoint.rs:28` |
| sym-3542d4de8fc719bc2c06 | `pocketstation::recording::endpoint::SessionMultistemEndpointCoordinator` | struct | Canonical Session-owned multistem recorder declaration. | `src/recording/endpoint.rs:49` |
| sym-688465041cd82d29a469 | `pocketstation::recording::writer::DiscontinuityRecord` | struct | Records one immutable discontinuity observation. | `src/recording/writer.rs:93` |
| sym-93addc782c5e8e6b3271 | `pocketstation::recording::writer::MultistemRecording` | struct | Owns the per-stem recording workers and coordinates their terminal finalization outcome. | `src/recording/writer.rs:139` |
| sym-a780bdc9245464b4deb9 | `pocketstation::recording::writer::RecordingObservations` | struct | Reports the recording observations collected at an observation boundary. | `src/recording/writer.rs:131` |
| sym-d36af3820ac5cb9b5294 | `pocketstation::recording::writer::RecordingOutcome` | struct | Reports the structured recording outcome. | `src/recording/writer.rs:112` |
| sym-0b38412f23568b36ab45 | `pocketstation::recording::writer::RecordingStemOutcome` | struct | Reports the structured recording stem outcome. | `src/recording/writer.rs:121` |
| sym-fe48110e01901959ec92 | `pocketstation::runtime::audio::executor::PlanExecutionSummary` | struct | Reports the counters and terminal facts collected for plan execution. | `src/runtime/audio/executor.rs:37` |
| sym-de8e881869143ab8717c | `pocketstation::runtime::audio::executor::RealtimePlanExecutor` | struct | Executes realtime plan according to its compiled plan and cancellation contract. | `src/runtime/audio/executor.rs:54` |
| sym-ca029607eb110269e010 | `pocketstation::runtime::audio::router::DispatchSummary` | struct | Reports the counters and terminal facts collected for dispatch. | `src/runtime/audio/router.rs:696` |
| sym-ff927b4e72f86f283b36 | `pocketstation::runtime::audio::router::EdgeObservations` | struct | Reports the edge observations collected at an observation boundary. | `src/runtime/audio/router.rs:142` |
| sym-6f9e0b24a61bf92c0e60 | `pocketstation::runtime::audio::router::PlanEdgeObservationHandle` | struct | Cloneable read-only access to one plan edge's authoritative live telemetry. | `src/runtime/audio/router.rs:231` |
| sym-b5de730cc0243997611d | `pocketstation::runtime::audio::router::PlanEdgeReceiver` | struct | Receives plan edge values across its declared ownership boundary. | `src/runtime/audio/router.rs:508` |
| sym-a518d53a93b584fc9e28 | `pocketstation::runtime::audio::router::PlanEdgeRouter` | struct | Routes plan edge according to the compiled edge contracts. | `src/runtime/audio/router.rs:704` |
| sym-76a0380402945c7e1e0e | `pocketstation::runtime::audio::runner::PlanRunnerCancellation` | struct | Shares a lock-free cancellation flag between the Session owner and the realtime plan runner. | `src/runtime/audio/runner.rs:89` |
| sym-6cbaffc72daeb4f0bf3f | `pocketstation::runtime::audio::runner::PlanRunnerFinishSummary` | struct | Reports the counters and terminal facts collected for plan runner finish. | `src/runtime/audio/runner.rs:298` |
| sym-4ad4f37c09daf9ca4868 | `pocketstation::runtime::audio::runner::PlanRunnerStepSummary` | struct | Reports the counters and terminal facts collected for plan runner step. | `src/runtime/audio/runner.rs:270` |
| sym-6ab0afc5fb44459d3ce0 | `pocketstation::runtime::audio::runner::PlanSourceInput` | struct | Carries typed input for plan source. | `src/runtime/audio/runner.rs:188` |
| sym-9ff992142384d442f9e8 | `pocketstation::runtime::audio::runner::PlanSourceInputObservations` | struct | Reports the plan source input observations collected at an observation boundary. | `src/runtime/audio/runner.rs:22` |
| sym-8321b041d44ef9bcf775 | `pocketstation::runtime::audio::runner::PlanSourceObservationHandle` | struct | Holds the ownership or bounded access represented by plan source observation handle. | `src/runtime/audio/runner.rs:138` |
| sym-48ad3b2812fda94faf23 | `pocketstation::runtime::audio::runner::PlanSourceSender` | struct | Sends plan source values across its declared ownership boundary. | `src/runtime/audio/runner.rs:131` |
| sym-99317db3dea27d4a5c53 | `pocketstation::runtime::audio::runner::RealtimePlanRunner` | struct | Executes realtime plan according to its compiled plan and cancellation contract. | `src/runtime/audio/runner.rs:305` |
| sym-36c2a3676a9c2e60e8e1 | `pocketstation::runtime::bridge::audio::GeneratedAudioBridge` | struct | Transfers generated audio across the bounded runtime boundary it owns. | `src/runtime/bridge/audio.rs:123` |
| sym-84b844455d8be96c03e6 | `pocketstation::runtime::bridge::audio::GeneratedAudioBridgeSpec` | struct | Configures generated audio bridge behavior at its owning API boundary. | `src/runtime/bridge/audio.rs:19` |
| sym-1466ff3cd1faa6c48106 | `pocketstation::runtime::lifecycle::async_host::AsyncRuntimeHost` | struct | Session-owned async executor for connector and derived-endpoint lifecycle. | `src/runtime/lifecycle/async_host.rs:26` |
| sym-73c083348ce6a7e6b8d1 | `pocketstation::runtime::lifecycle::sidecar_host::SidecarDeadlines` | struct | Sets finite startup, I/O, shutdown, and reap deadlines for a sidecar process. | `src/runtime/lifecycle/sidecar_host.rs:54` |
| sym-03844c883f9f4b0be64a | `pocketstation::runtime::lifecycle::sidecar_host::SidecarHost` | struct | Owns the resources and lifecycle for sidecar. | `src/runtime/lifecycle/sidecar_host.rs:157` |
| sym-b73ede32dbf2f6117744 | `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostObservations` | struct | Reports the sidecar host observations collected at an observation boundary. | `src/runtime/lifecycle/sidecar_host.rs:109` |
| sym-2c37bf0eea3cd707b878 | `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostSnapshot` | struct | Reports the sidecar host snapshot collected at an observation boundary. | `src/runtime/lifecycle/sidecar_host.rs:133` |
| sym-c40de69ecfa5672542ee | `pocketstation::runtime::lifecycle::sidecar_host::SidecarProcessSpec` | struct | Configures sidecar process behavior at its owning API boundary. | `src/runtime/lifecycle/sidecar_host.rs:71` |
| sym-946b65790e6016ecef8a | `pocketstation::runtime::lifecycle::sidecar_protocol::SidecarMessage` | struct | Carries one typed control or signal message across the sidecar protocol. | `src/runtime/lifecycle/sidecar_protocol.rs:73` |
| sym-371fd8b00cc461c3eef5 | `pocketstation::runtime::lifecycle::sidecar_protocol::SidecarProtocolLimits` | struct | Sets the maximum sidecar message and buffered-byte sizes enforced by protocol I/O. | `src/runtime/lifecycle/sidecar_protocol.rs:43` |
| sym-1018381633c6074cd2a9 | `pocketstation::runtime::nodes::BridgeSinkFactory` | struct | Constructs bridge sink implementations from validated declarations. | `src/runtime/nodes.rs:170` |
| sym-7aad40fda10e3ba8d830 | `pocketstation::runtime::nodes::BridgeSinkTelemetry` | struct | Reports the counters and terminal facts collected for bridge sink. | `src/runtime/nodes.rs:156` |
| sym-f229251ec6480f8b5efc | `pocketstation::runtime::nodes::MixerSourceNode` | struct | Represents the executable graph node for mixer source. | `src/runtime/nodes.rs:268` |
| sym-c370a20412082781b0c0 | `pocketstation::runtime::nodes::MixerTelemetry` | struct | Reports the counters and terminal facts collected for mixer. | `src/runtime/nodes.rs:246` |
| sym-0c9e67cb8a03ca4402bd | `pocketstation::runtime::nodes::SystemOutputSourceFactory` | struct | Constructs system output source implementations from validated declarations. | `src/runtime/nodes.rs:66` |
| sym-a967c65b7a68d2a1a1ed | `pocketstation::runtime::nodes::SystemOutputTelemetry` | struct | Reports the counters and terminal facts collected for system output. | `src/runtime/nodes.rs:48` |
| sym-c2052167580477ebabed | `pocketstation::runtime::signal::edge::SignalEdgeObservationHandle` | struct | Holds the ownership or bounded access represented by signal edge observation handle. | `src/runtime/signal/edge.rs:49` |
| sym-9ae8f4b901bcd6982adb | `pocketstation::runtime::signal::edge::SignalEdgeObservations` | struct | Reports the signal edge observations collected at an observation boundary. | `src/runtime/signal/edge.rs:31` |
| sym-f3d13de50402023ca5b7 | `pocketstation::runtime::signal::edge::SignalEdgeReceiver` | struct | Receives signal edge values across its declared ownership boundary. | `src/runtime/signal/edge.rs:204` |
| sym-4612120754fe112cb207 | `pocketstation::runtime::signal::edge::SignalEdgeSendError` | struct | Reports a signal edge send error. | `src/runtime/signal/edge.rs:118` |
| sym-126b1dd5ddcefeaaa2bd | `pocketstation::runtime::signal::edge::TypedEdgeBranchSpec` | struct | Configures typed edge branch behavior at its owning API boundary. | `src/runtime/signal/edge.rs:248` |
| sym-e7ba56f46a6eecc8813b | `pocketstation::runtime::signal::edge::TypedEdgeFanout` | struct | Publishes one immutable signal envelope to the bounded branches of a compiled fan-out edge. | `src/runtime/signal/edge.rs:259` |
| sym-10c81e5eba35a5a9af84 | `pocketstation::runtime::signal::edge::TypedEdgePublishReport` | struct | Reports how many fan-out branches accepted or dropped one published signal. | `src/runtime/signal/edge.rs:380` |
| sym-1f4a66979ba60389ef4d | `pocketstation::runtime::signal::io::AsyncOperatorInput` | struct | Carries typed input for async operator. | `src/runtime/signal/io.rs:23` |
| sym-12ca9b63f9b9b69659b0 | `pocketstation::runtime::signal::io::AsyncOperatorInputAccessError` | struct | Classifies failures surfaced by async operator input access operations. | `src/runtime/signal/io.rs:31` |
| sym-063dc9ab9c8a852b5de9 | `pocketstation::runtime::signal::io::AsyncOperatorNamedOutput` | struct | Carries typed output from async operator named. | `src/runtime/signal/io.rs:95` |
| sym-d9fc5a2474976673acdf | `pocketstation::runtime::signal::io::AsyncOperatorNamedOutputBranchSpec` | struct | Configures async operator named output branch behavior at its owning API boundary. | `src/runtime/signal/io.rs:90` |
| sym-e38dcb8a573d61d455cb | `pocketstation::runtime::signal::io::AsyncOperatorTypedInput` | struct | Carries typed input for async operator typed. | `src/runtime/signal/io.rs:79` |
| sym-3965adf0f95c140ed9ff | `pocketstation::runtime::signal::observations::AsyncOperatorObservationHandle` | struct | Holds the ownership or bounded access represented by async operator observation handle. | `src/runtime/signal/observations.rs:47` |
| sym-b4f069dcad3496b02f71 | `pocketstation::runtime::signal::observations::AsyncOperatorObservations` | struct | Reports the async operator observations collected at an observation boundary. | `src/runtime/signal/observations.rs:29` |
| sym-b6317bb4530a3cc0d1ad | `pocketstation::runtime::signal::operator::AsyncOperatorWorker` | struct | Owns the asynchronous operator task, typed I/O, cancellation, and terminal join result. | `src/runtime/signal/operator.rs:250` |
| sym-77b61e6016b1412842de | `pocketstation::runtime::signal::operator::CompiledOperatorInputContract` | struct | Declares the validated constraints applied to compiled operator input. | `src/runtime/signal/operator.rs:103` |
| sym-ab8fbade845ec4f557fa | `pocketstation::session::compile::SessionCompiler` | struct | Compiles an immutable Session declaration into a validated graph and runtime plan. | `src/session/compile/mod.rs:41` |
| sym-13bafc535d0d0461f379 | `pocketstation::session::compile::compiled::CompiledSession` | struct | Owns the validated Session specification and declarations produced by compilation. | `src/session/compile/compiled.rs:13` |
| sym-ca9f16cf42aec2ba15f3 | `pocketstation::session::compile::error::SessionCompileDiagnostic` | struct | Stable, language-neutral location facts for one Session compilation error. | `src/session/compile/error.rs:98` |
| sym-856150fca4f4c923955e | `pocketstation::session::declaration::draft::DerivedStreamHandle` | struct | Holds the ownership or bounded access represented by derived stream handle. | `src/session/declaration/draft.rs:827` |
| sym-26c368ef10cf4369d92f | `pocketstation::session::declaration::draft::EndpointHandle` | struct | Holds the ownership or bounded access represented by endpoint handle. | `src/session/declaration/draft.rs:580` |
| sym-4c41a20169460056e90a | `pocketstation::session::declaration::draft::Operator` | struct | Declares one operator instance, including its stable operator identity and validated node configuration. | `src/session/declaration/draft.rs:282` |
| sym-9208421d2a4023ada2e5 | `pocketstation::session::declaration::draft::OperatorInputHandle` | struct | Holds the ownership or bounded access represented by operator input handle. | `src/session/declaration/draft.rs:701` |
| sym-8657845acc63cb1ddc9a | `pocketstation::session::declaration::draft::OperatorInstanceHandle` | struct | Holds the ownership or bounded access represented by operator instance handle. | `src/session/declaration/draft.rs:694` |
| sym-22e5c9cafaec4b5b54f7 | `pocketstation::session::declaration::draft::Session` | struct | Owns a mutable Session declaration and the host configuration used to compile, prepare, and start it. | `src/session/declaration/draft.rs:316` |
| sym-0fe59a8c2718fecb38f0 | `pocketstation::session::declaration::draft::SourceInstanceHandle` | struct | Holds the ownership or bounded access represented by source instance handle. | `src/session/declaration/draft.rs:834` |
| sym-92a7ddba0bc275477a55 | `pocketstation::session::declaration::draft::SourceOutputHandle` | struct | Holds the ownership or bounded access represented by source output handle. | `src/session/declaration/draft.rs:910` |
| sym-2f23b06b9fb7211a456a | `pocketstation::session::declaration::draft::StemHandle` | struct | Holds the ownership or bounded access represented by stem handle. | `src/session/declaration/draft.rs:688` |
| sym-355302a480d969237d6c | `pocketstation::session::declaration::endpoint::EndpointConfiguration` | struct | Configures endpoint behavior at its owning API boundary. | `src/session/declaration/endpoint.rs:14` |
| sym-670230260458c0c7bac5 | `pocketstation::session::declaration::endpoint::EndpointDescriptor` | struct | Declares an endpoint's node identity, media contract, configuration, and execution requirements. | `src/session/declaration/endpoint.rs:110` |
| sym-326ae28c371cf2b1f9f2 | `pocketstation::session::declaration::selector::DeviceId` | struct | Uniquely identifies device within its PocketStation ownership scope. | `src/session/declaration/selector.rs:19` |
| sym-31891ac0c0cadb435693 | `pocketstation::session::declaration::selector::ProcessId` | struct | Uniquely identifies process within its PocketStation ownership scope. | `src/session/declaration/selector.rs:6` |
| sym-951d614ce5fe18b6cc19 | `pocketstation::session::declaration::spec::ConnectionSpec` | struct | The single Session connection declaration used for every stream origin and every operator/endpoint destination. | `src/session/declaration/spec.rs:238` |
| sym-c7217647970c5259ce89 | `pocketstation::session::declaration::spec::EndpointSpec` | struct | Configures endpoint behavior at its owning API boundary. | `src/session/declaration/spec.rs:161` |
| sym-9002065c6c832a09e5c3 | `pocketstation::session::declaration::spec::GeneratedAudioIngressSpec` | struct | One operator PCM output that re-enters the specialized Session audio lane. | `src/session/declaration/spec.rs:106` |
| sym-4f45a81c2fd756286a2e | `pocketstation::session::declaration::spec::OperatorInstanceId` | struct | Uniquely identifies operator instance within its PocketStation ownership scope. | `src/session/declaration/spec.rs:27` |
| sym-77edcf634933f646c17b | `pocketstation::session::declaration::spec::OperatorInstanceSpec` | struct | Configures operator instance behavior at its owning API boundary. | `src/session/declaration/spec.rs:197` |
| sym-22e41e3837b6aefcacef | `pocketstation::session::declaration::spec::SessionSpec` | struct | Configures session behavior at its owning API boundary. | `src/session/declaration/spec.rs:277` |
| sym-3bb50f6554ba10cd793e | `pocketstation::session::declaration::spec::SessionSpecVersion` | struct | Identifies the major and minor version of the immutable Session declaration schema. | `src/session/declaration/spec.rs:40` |
| sym-de8ddcdeb72a5fec6353 | `pocketstation::session::declaration::spec::SourceInstanceId` | struct | Uniquely identifies source instance within its PocketStation ownership scope. | `src/session/declaration/spec.rs:14` |
| sym-ca64d1b26eb7ad96dc8d | `pocketstation::session::declaration::spec::SourceInstanceSpec` | struct | Configures source instance behavior at its owning API boundary. | `src/session/declaration/spec.rs:68` |
| sym-b5f82684c1b8500420cd | `pocketstation::session::declaration::spec::SourceOutputSpec` | struct | Configures source output behavior at its owning API boundary. | `src/session/declaration/spec.rs:94` |
| sym-a2b6f5b625f65bd3d1d8 | `pocketstation::session::declaration::spec::StemSpec` | struct | Configures stem behavior at its owning API boundary. | `src/session/declaration/spec.rs:62` |
| sym-d2674745b36921365a4c | `pocketstation::session::declaration::typed_stream::Stream` | struct | Typed Rust declaration façade compiled into stable dynamic signal, schema, port, and edge contracts. This wrapper carries no frames and is not a generic runtime queue. | `src/session/declaration/typed_stream.rs:96` |
| sym-d5e1ab2d8690806393f3 | `pocketstation::session::declaration::typed_stream::TypedOperator` | struct | Binds an operator declaration to its typed input and output ports so graph connections preserve signal specifications. | `src/session/declaration/typed_stream.rs:20` |
| sym-8465535da4bc90be30a6 | `pocketstation::session::extensions::audio_input::AudioInput` | struct | Intent-first façade for feeding audio already owned by the embedding application into a Session. | `src/session/extensions/audio_input/mod.rs:94` |
| sym-b8fd3f656e58669b873f | `pocketstation::session::extensions::audio_input::AudioInputConfig` | struct | Configures audio input behavior at its owning API boundary. | `src/session/extensions/audio_input/mod.rs:22` |
| sym-95c8cfa928df89d4594d | `pocketstation::session::extensions::audio_input::buffer::AudioInputBuffer` | struct | Leases bounded PCM storage from an external-audio input until the caller submits or releases it. | `src/session/extensions/audio_input/buffer.rs:11` |
| sym-ba04da329d2baa75968e | `pocketstation::session::extensions::audio_input::buffer::AudioInputObservations` | struct | Reports the audio input observations collected at an observation boundary. | `src/session/extensions/audio_input/buffer.rs:72` |
| sym-98b22817eebc9cda96f6 | `pocketstation::session::extensions::audio_input::buffer::AudioInputWriteError` | struct | Classifies failures produced during audio input writing. | `src/session/extensions/audio_input/buffer.rs:305` |
| sym-78b8d13f6175077eb2ae | `pocketstation::session::extensions::audio_input::buffer::AudioInputWriter` | struct | Sends audio input values across its declared ownership boundary. | `src/session/extensions/audio_input/buffer.rs:91` |
| sym-54480454b87697ddec57 | `pocketstation::session::extensions::audio_input::source::PcmSource` | struct | Low-level PCM source ownership for integrations that separately retain the Session handles and producer writer. | `src/session/extensions/audio_input/source.rs:33` |
| sym-b0f7adbcacff1861cede | `pocketstation::session::extensions::recording::SessionRecordingReceipt` | struct | Retains the identity and observation access returned for session recording. | `src/session/extensions/recording.rs:31` |
| sym-4e3c8fa340f216b23656 | `pocketstation::session::extensions::source::PreparedSourceRuntime` | struct | Fully validated source resources which have not started producing signals. | `src/session/extensions/source.rs:437` |
| sym-7bf3b92386b5beaaa600 | `pocketstation::session::extensions::source::SourceCancellation` | struct | Exposes the cancellation state observed by a running external source driver. | `src/session/extensions/source.rs:250` |
| sym-940b54ec7b47a949f6b4 | `pocketstation::session::extensions::source::SourceConfiguration` | struct | Configures source behavior at its owning API boundary. | `src/session/extensions/source.rs:87` |
| sym-c7176eab4efbff0fb414 | `pocketstation::session::extensions::source::SourceEmission` | struct | Carries one external-source emission with its output-port identity and signal envelope. | `src/session/extensions/source.rs:261` |
| sym-e3d39b49d7b92878f123 | `pocketstation::session::extensions::source::SourceManifest` | struct | Declares an external source's identity, outputs, preparation group, and execution requirements. | `src/session/extensions/source.rs:112` |
| sym-49a7565157d6daca7987 | `pocketstation::session::extensions::source::SourceOutputBranchSpec` | struct | Configures source output branch behavior at its owning API boundary. | `src/session/extensions/source.rs:370` |
| sym-62891a3960a9e7029705 | `pocketstation::session::extensions::source::SourceOutputIdentity` | struct | Identifies one declared source output by source type, output port, and stream identity. | `src/session/extensions/source.rs:229` |
| sym-3b3f63770eb90ea6d83d | `pocketstation::session::extensions::source::SourceOutputReceiver` | struct | Receives source output values across its declared ownership boundary. | `src/session/extensions/source.rs:375` |
| sym-b4c843f30e0bca6d1537 | `pocketstation::session::extensions::source::SourcePrepareContext` | struct | Carries the inputs and runtime context required to source prepare. | `src/session/extensions/source.rs:223` |
| sym-28bf0eddf9bb4dd0c568 | `pocketstation::session::extensions::source::SourceRegistry` | struct | Indexes registered source implementations by their stable identities. | `src/session/extensions/source.rs:286` |
| sym-82096de9260f62b2b8f8 | `pocketstation::session::extensions::source::SourceRuntime` | struct | Owns an external source driver's cancellation handle, observations, and terminal worker join. | `src/session/extensions/source.rs:427` |
| sym-92b2aae5b16f9641d36b | `pocketstation::session::extensions::source::SourceRuntimeObservationHandle` | struct | Holds the ownership or bounded access represented by source runtime observation handle. | `src/session/extensions/source.rs:407` |
| sym-23520d1fea3f72bd48b9 | `pocketstation::session::extensions::source::SourceRuntimeObservations` | struct | Reports the source runtime observations collected at an observation boundary. | `src/session/extensions/source.rs:394` |
| sym-81cdd13cae6a0dfe2f6b | `pocketstation::session::extensions::source::SourceSessionContext` | struct | Carries the inputs and runtime context required to source session. | `src/session/extensions/source.rs:235` |
| sym-909ac8c9cfde21d8810b | `pocketstation::session::extensions::source::SourceTypeId` | struct | Uniquely identifies source type within its PocketStation ownership scope. | `src/session/extensions/source.rs:17` |
| sym-07d6115c59bfee9368e0 | `pocketstation::session::lifecycle::control::CaptureBackendSet` | struct | Supplies the application and microphone capture backends used while preparing a Session. | `src/session/lifecycle/control.rs:17` |
| sym-827a6365a893eb683796 | `pocketstation::session::lifecycle::control::SessionStartCancellation` | struct | Thread-safe cancellation request for a Session that has not reached `Running` yet. | `src/session/lifecycle/control.rs:106` |
| sym-ae264ef215a74195896e | `pocketstation::session::lifecycle::control::SessionStartFailure` | struct | Reports a session start failure. | `src/session/lifecycle/control.rs:292` |
| sym-dd793f1c809a665f89af | `pocketstation::session::lifecycle::control::SessionStartOptions` | struct | Configures session start behavior at its owning API boundary. | `src/session/lifecycle/control.rs:23` |
| sym-ae49046b32ebad6a6333 | `pocketstation::session::lifecycle::control::SessionStopOutcome` | struct | Reports the structured session stop outcome. | `src/session/lifecycle/control.rs:337` |
| sym-e36924784f3e8092a7d9 | `pocketstation::session::lifecycle::engine::SessionEngine` | struct | Canonical production composition path for one safe Rust Session engine. | `src/session/lifecycle/engine.rs:202` |
| sym-1c65a7ab55ecd9be2b8f | `pocketstation::session::lifecycle::engine::SessionEngineBuilder` | struct | Registers the components and runtime configuration for one Session. | `src/session/lifecycle/engine.rs:30` |
| sym-9ac25988f4252600a50f | `pocketstation::session::lifecycle::events::SessionControlFailure` | struct | Typed control-plane failure without exposing an implementation error type. | `src/session/lifecycle/events.rs:70` |
| sym-92154a2e8955f61afb41 | `pocketstation::session::lifecycle::events::SessionEndpointFailure` | struct | Endpoint failure associated with one stable route and endpoint. | `src/session/lifecycle/events.rs:125` |
| sym-f5944a79bde242e0c729 | `pocketstation::session::lifecycle::events::SessionEvent` | struct | Event emitted by the session lifecycle authority. | `src/session/lifecycle/events.rs:308` |
| sym-04315a0906e23274171c | `pocketstation::session::lifecycle::events::SessionEventReceiver` | struct | Sole consumer for a session's bounded control-event queue. | `src/session/lifecycle/events.rs:500` |
| sym-b88e4bff4a0bac13eff9 | `pocketstation::session::lifecycle::events::SessionFinalizationFailure` | struct | Failure observed while finalizing a stopping session. | `src/session/lifecycle/events.rs:186` |
| sym-06b87e7f0db6e4875a31 | `pocketstation::session::lifecycle::events::SessionRollbackFailure` | struct | Failure observed while rolling back a partial session start. | `src/session/lifecycle/events.rs:165` |
| sym-5afed7f41125268245b9 | `pocketstation::session::lifecycle::events::SessionSourceFailure` | struct | Source failure associated with one stable session stem. | `src/session/lifecycle/events.rs:104` |
| sym-d0c5fa494c12c763d159 | `pocketstation::session::lifecycle::events::SessionTerminalOutcome` | struct | Complete terminal result. Failure categories remain separate for diagnosis. | `src/session/lifecycle/events.rs:217` |
| sym-54771765004df84c0dd7 | `pocketstation::session::lifecycle::host::NativeSessionEngineHostOptions` | struct | Configures native session engine host behavior at its owning API boundary. | `src/session/lifecycle/host.rs:164` |
| sym-5da049dfb470c3b8a7df | `pocketstation::session::lifecycle::host::SessionEngineHost` | struct | Owns the native Session resources projected to language adapters. | `src/session/lifecycle/host.rs:31` |
| sym-f1683c63122b14eb81b1 | `pocketstation::session::lifecycle::host::SessionEngineHostBuilder` | struct | Builds a Session host from caller-owned or native capture backends. | `src/session/lifecycle/host.rs:188` |
| sym-ed887803c3e301f2839e | `pocketstation::session::lifecycle::observations::SessionAudioReentryMetrics` | struct | Exact boundedness and lifecycle accounting for one operator PCM output re-entering the Session audio lane. | `src/session/lifecycle/observations.rs:253` |
| sym-86b1a65358de5b15bb95 | `pocketstation::session::lifecycle::observations::SessionDerivedRouteMetrics` | struct | Reports the session derived route metrics collected at an observation boundary. | `src/session/lifecycle/observations.rs:431` |
| sym-2848741efd2a9a30ef0d | `pocketstation::session::lifecycle::observations::SessionEventQueueObservations` | struct | Point-in-time observations for a session's bounded control-event queue. | `src/session/lifecycle/observations.rs:17` |
| sym-b0bbe81112d09dd39eb1 | `pocketstation::session::lifecycle::observations::SessionExternalSourceMetrics` | struct | Reports the session external source metrics collected at an observation boundary. | `src/session/lifecycle/observations.rs:124` |
| sym-5c921b64b075186748b6 | `pocketstation::session::lifecycle::observations::SessionMetricsSnapshot` | struct | Authoritative point-in-time observations for the current Session boundary. | `src/session/lifecycle/observations.rs:36` |
| sym-444b445ceb6b45f18970 | `pocketstation::session::lifecycle::observations::SessionOperatorInputMetrics` | struct | Reports the session operator input metrics collected at an observation boundary. | `src/session/lifecycle/observations.rs:242` |
| sym-f05ca0140fdd8e6d6ce8 | `pocketstation::session::lifecycle::observations::SessionOperatorMetrics` | struct | Reports the session operator metrics collected at an observation boundary. | `src/session/lifecycle/observations.rs:382` |
| sym-6f0f43af8955f3d99d76 | `pocketstation::session::lifecycle::observations::SessionRouteDropObservations` | struct | Explicit numerator, denominator, interval, and typed reasons for one route. | `src/session/lifecycle/observations.rs:157` |
| sym-61abcba91a60aacb0de6 | `pocketstation::session::lifecycle::observations::SessionRouteLatencyObservations` | struct | Common-clock source timestamp to route-receive latency in nanoseconds. | `src/session/lifecycle/observations.rs:182` |
| sym-4b225f4a748c61c3a2b8 | `pocketstation::session::lifecycle::observations::SessionRouteMetrics` | struct | Reports the session route metrics collected at an observation boundary. | `src/session/lifecycle/observations.rs:139` |
| sym-44cac6f6bfcf189a48ee | `pocketstation::session::lifecycle::observations::SessionSidecarMetrics` | struct | Exact bounded-queue and process-lifecycle accounting for one Session-owned language-neutral sidecar. | `src/session/lifecycle/observations.rs:133` |
| sym-4159552cb46d81741c35 | `pocketstation::session::lifecycle::observations::SessionSourceMetrics` | struct | Reports the session source metrics collected at an observation boundary. | `src/session/lifecycle/observations.rs:117` |
| sym-32b1ad3fd2381f01eaaa | `pocketstation::session::lifecycle::running::RunningSession` | struct | Owns a started Session together with event, polling, recording, trace, and stop resources. | `src/session/lifecycle/running.rs:173` |
| sym-84ece645a6c49a4ca8af | `pocketstation::session::lifecycle::trace::SessionTrace` | struct | Contains the ordered lifecycle records read from a Session trace artifact. | `src/session/lifecycle/trace.rs:255` |
| sym-b78f79216b1e5d291894 | `pocketstation::session::lifecycle::trace::SessionTraceRecord` | struct | Records one immutable session trace observation. | `src/session/lifecycle/trace.rs:55` |
| sym-11e920666c648f72b4a5 | `pocketstation::session::lifecycle::trace::SessionTraceRecorder` | struct | Collects ordered lifecycle records and writes the trace artifact during Session finalization. | `src/session/lifecycle/trace.rs:152` |
| sym-0b69e0b79e369014bf45 | `pocketstation::session::lifecycle::trace::SessionTraceRecorderHandle` | struct | Holds the ownership or bounded access represented by session trace recorder handle. | `src/session/lifecycle/trace.rs:108` |
| sym-0b97eef7194205aacb95 | `pocketstation::session::lifecycle::trace::SessionTraceRecorderOutcome` | struct | Reports the structured session trace recorder outcome. | `src/session/lifecycle/trace.rs:70` |
| sym-3be78f85526506223be9 | `pocketstation::session::lifecycle::trace::SessionTraceTerminal` | struct | Records the terminal Session disposition and component failures stored in a trace. | `src/session/lifecycle/trace.rs:339` |
| sym-adf09edfc0bd09c49159 | `pocketstation::session::lifecycle::trace::SessionTraceValidation` | struct | Reports the validated identity and record count of a parsed Session trace. | `src/session/lifecycle/trace.rs:348` |
| sym-f389015a71c379e77eef | `pocketstation::session::prepare::mappings::PreparedOperatorMapping` | struct | Correlates the prepared identities and runtime resources for prepared operator. | `src/session/prepare/mappings.rs:160` |
| sym-9fde8967b03e22b75dc6 | `pocketstation::session::prepare::mappings::PreparedSignalRouteMapping` | struct | Correlates the prepared identities and runtime resources for prepared signal route. | `src/session/prepare/mappings.rs:131` |
| sym-c2e2873ff4ba1efbd707 | `pocketstation::session::prepare::mappings::PreparedSourceMapping` | struct | Correlates the prepared identities and runtime resources for prepared source. | `src/session/prepare/mappings.rs:18` |
| sym-d945a3bd62cf3bbdc414 | `pocketstation::session::prepare::mappings::PreparedWorkerMapping` | struct | Correlates the prepared identities and runtime resources for prepared worker. | `src/session/prepare/mappings.rs:35` |
| sym-f8122b0813353de23883 | `pocketstation::session::prepare::prepared::PreparedSession` | struct | Setup-time ownership for one compiled Session. | `src/session/prepare/prepared.rs:18` |
| sym-ae405074cfb9d4d5b291 | `pocketstation::timing::clock_correction::ClockCorrectionController` | struct | Applies bounded proportional corrections from measured clock offsets without changing lineage. | `src/timing/clock_correction.rs:4` |
| sym-64a18bac4bce29e0a26c | `pocketstation::timing::clock_drift::ClockDriftEstimator` | struct | Estimates source-clock drift from accumulated source and Session timing observations. | `src/timing/clock_drift.rs:10` |
| sym-9982b0b272411ab60fb6 | `pocketstation::timing::clock_drift::ClockDriftSnapshot` | struct | Reports the clock drift snapshot collected at an observation boundary. | `src/timing/clock_drift.rs:4` |
| sym-d78fbc7a6425539ac041 | `pocketstation::timing::domain::ClockDomainDescriptor` | struct | Finite description of a clock identity carried by frame and signal lineage. | `src/timing/domain.rs:23` |
| sym-8cb95ccfe2d318c37c6b | `pocketstation::timing::timeline_mapping::TimelineMapping` | struct | Correlates the prepared identities and runtime resources for timeline. | `src/timing/timeline_mapping.rs:2` |
| sym-b98cf61ee08d27a5c9ef | `ApplicationSelector::ProcessInstance::process_id` | struct_field | Identifies the process identifier recorded by `ProcessInstance`. | `src/session/declaration/selector.rs:36` |
| sym-98816553989c382ad28c | `ApplicationSelector::ProcessInstance::stable_id` | struct_field | Identifies the stable identifier recorded by `ProcessInstance`. | `src/session/declaration/selector.rs:37` |
| sym-0c2d07d98eb853ed0be8 | `AsyncOperatorNamedOutput::output_port` | struct_field | References the output port participating in `AsyncOperatorNamedOutput`. | `src/runtime/signal/io.rs:96` |
| sym-bb33706b4b4c785b20da | `AsyncOperatorNamedOutput::receiver` | struct_field | Owns the receiver endpoint through which `AsyncOperatorNamedOutput` exchanges values. | `src/runtime/signal/io.rs:97` |
| sym-80eccee263e9b855e5af | `AsyncOperatorNamedOutputBranchSpec::branch` | struct_field | References the branch participating in `AsyncOperatorNamedOutputBranchSpec`. | `src/runtime/signal/io.rs:92` |
| sym-84ccc10e0621da535f9f | `AsyncOperatorNamedOutputBranchSpec::output_port` | struct_field | References the output port participating in `AsyncOperatorNamedOutputBranchSpec`. | `src/runtime/signal/io.rs:91` |
| sym-b845b050d958a730f7b4 | `AsyncOperatorObservations::cancellation_total` | struct_field | Counts the total number of cancellation observed by `AsyncOperatorObservations`. | `src/runtime/signal/observations.rs:39` |
| sym-5599803b5d2f3167c7a2 | `AsyncOperatorObservations::graceful_finish_total` | struct_field | Counts the total number of graceful finish observed by `AsyncOperatorObservations`. | `src/runtime/signal/observations.rs:40` |
| sym-c78c37f4af2a9aa5faa0 | `AsyncOperatorObservations::idle_poll_total` | struct_field | Counts the total number of idle poll observed by `AsyncOperatorObservations`. | `src/runtime/signal/observations.rs:41` |
| sym-1a82f8555a73922c0e7b | `AsyncOperatorObservations::input_attempted_total` | struct_field | Counts the total number of input attempted observed by `AsyncOperatorObservations`. | `src/runtime/signal/observations.rs:30` |
| sym-952b8dd6007864dd3145 | `AsyncOperatorObservations::input_dropped_total` | struct_field | Counts the total number of input dropped observed by `AsyncOperatorObservations`. | `src/runtime/signal/observations.rs:31` |
| sym-492fd40d36c84b08613a | `AsyncOperatorObservations::joined` | struct_field | Reports whether joined is true for `AsyncOperatorObservations`. | `src/runtime/signal/observations.rs:43` |
| sym-fa89a1649b5ef3a9df2b | `AsyncOperatorObservations::output_dropped_total` | struct_field | Counts the total number of output dropped observed by `AsyncOperatorObservations`. | `src/runtime/signal/observations.rs:34` |
| sym-7d417791b87fcd60a688 | `AsyncOperatorObservations::output_emitted_total` | struct_field | Counts the total number of output emitted observed by `AsyncOperatorObservations`. | `src/runtime/signal/observations.rs:33` |
| sym-8753c02e8f308fa5e6d0 | `AsyncOperatorObservations::output_nonterminal_total` | struct_field | Counts the total number of output nonterminal observed by `AsyncOperatorObservations`. | `src/runtime/signal/observations.rs:35` |
| sym-a85d0b04d07634aac03b | `AsyncOperatorObservations::output_terminal_total` | struct_field | Counts the total number of output terminal observed by `AsyncOperatorObservations`. | `src/runtime/signal/observations.rs:36` |
| sym-ddc70d9a50319d972eb0 | `AsyncOperatorObservations::process_failure_total` | struct_field | Counts the total number of process failure observed by `AsyncOperatorObservations`. | `src/runtime/signal/observations.rs:37` |
| sym-5c6c30ea0fe72055215e | `AsyncOperatorObservations::processed_total` | struct_field | Counts the total number of processed observed by `AsyncOperatorObservations`. | `src/runtime/signal/observations.rs:32` |
| sym-e7c144836e422fa1a994 | `AsyncOperatorObservations::ready` | struct_field | Indicates whether ready applies to `AsyncOperatorObservations`. | `src/runtime/signal/observations.rs:42` |
| sym-edca6a58968447d855cc | `AsyncOperatorObservations::timeout_total` | struct_field | Counts the total number of timeout observed by `AsyncOperatorObservations`. | `src/runtime/signal/observations.rs:38` |
| sym-88449497b4191d07e3b4 | `AsyncOperatorTypedInput::capacity_signals` | struct_field | Sets the capacity signals available to `AsyncOperatorTypedInput`. | `src/runtime/signal/io.rs:86` |
| sym-5c02199b1b985d665b52 | `AsyncOperatorTypedInput::edge_contract` | struct_field | References the edge contract participating in `AsyncOperatorTypedInput`. | `src/runtime/signal/io.rs:85` |
| sym-3bc11585209c206da563 | `AsyncOperatorTypedInput::edge_id` | struct_field | Identifies the edge identifier recorded by `AsyncOperatorTypedInput`. | `src/runtime/signal/io.rs:82` |
| sym-c2df102b4075cf26ab16 | `AsyncOperatorTypedInput::media` | struct_field | Records the media selected for `AsyncOperatorTypedInput`. | `src/runtime/signal/io.rs:84` |
| sym-56ee321d8bb7bf146a09 | `AsyncOperatorTypedInput::port_name` | struct_field | Stores the human-readable port used to identify `AsyncOperatorTypedInput`. | `src/runtime/signal/io.rs:80` |
| sym-7e8ebdd3f1e6d1c1b5b4 | `AsyncOperatorTypedInput::receiver` | struct_field | Owns the receiver endpoint through which `AsyncOperatorTypedInput` exchanges values. | `src/runtime/signal/io.rs:81` |
| sym-ecc296912547c9718f7f | `AsyncOperatorTypedInput::signal_spec` | struct_field | Declares the signal class and format accepted by `AsyncOperatorTypedInput`. | `src/runtime/signal/io.rs:83` |
| sym-5a844df15cd09a15d5ae | `AsyncOperatorWorkerError::CancelTimeout::timeout_ms` | struct_field | Stores the timeout value for `CancelTimeout`, in milliseconds. | `src/runtime/signal/error.rs:22` |
| sym-631b37c03ee49775d11c | `AsyncOperatorWorkerError::CloseTimeout::timeout_ms` | struct_field | Stores the timeout value for `CloseTimeout`, in milliseconds. | `src/runtime/signal/error.rs:18` |
| sym-7d7030da17841e6132d7 | `AsyncOperatorWorkerError::InvalidPlanInput::kind` | struct_field | Records the kind selected for `InvalidPlanInput`. | `src/runtime/signal/error.rs:50` |
| sym-ed59380446aa045aa512 | `AsyncOperatorWorkerError::OutputPayloadTooLarge::branch_index` | struct_field | Identifies the branch index position within `OutputPayloadTooLarge`. | `src/runtime/signal/error.rs:43` |
| sym-5d6fc456eca0df2881b6 | `AsyncOperatorWorkerError::OutputPayloadTooLarge::max_payload_bytes` | struct_field | Limits payload storage for `OutputPayloadTooLarge`, in bytes. | `src/runtime/signal/error.rs:45` |
| sym-6c95e60a53670936af4d | `AsyncOperatorWorkerError::OutputPayloadTooLarge::payload_bytes` | struct_field | Stores the payload size for `OutputPayloadTooLarge`, in bytes. | `src/runtime/signal/error.rs:44` |
| sym-7202582f2453e2bb9408 | `AsyncOperatorWorkerError::PrepareTimeout::timeout_ms` | struct_field | Stores the timeout value for `PrepareTimeout`, in milliseconds. | `src/runtime/signal/error.rs:10` |
| sym-5078adb77e0063cf68cf | `AsyncOperatorWorkerError::TerminalOutputDropped::branch_index` | struct_field | Identifies the branch index position within `TerminalOutputDropped`. | `src/runtime/signal/error.rs:38` |
| sym-c402b73dee2752eacdee | `AsyncOperatorWorkerError::Timeout::timeout_ms` | struct_field | Stores the timeout value for `Timeout`, in milliseconds. | `src/runtime/signal/error.rs:14` |
| sym-b0ae8ee2c7e82779124a | `AsyncOperatorWorkerError::UnknownInputPort::port_name` | struct_field | Stores the human-readable port used to identify `UnknownInputPort`. | `src/runtime/signal/error.rs:34` |
| sym-f83601e715fae3a99934 | `AsyncRuntimeHostError::HostTimeout::timeout_ms` | struct_field | Stores the timeout value for `HostTimeout`, in milliseconds. | `src/runtime/lifecycle/async_host.rs:18` |
| sym-ccf236449a294e208714 | `AudioCaps::channel_layout` | struct_field | Declares the channel arrangement accepted by `AudioCaps`. | `src/graph/ports.rs:51` |
| sym-bc6d1a301360bb6efdc2 | `AudioCaps::format` | struct_field | Records the format selected for `AudioCaps`. | `src/graph/ports.rs:52` |
| sym-6a32bc756b095ab72a0b | `AudioCaps::frame_samples` | struct_field | Contains the frame samples owned or reported by `AudioCaps`. | `src/graph/ports.rs:50` |
| sym-06b93d1c01439832ce86 | `AudioCaps::sample_rate_hz` | struct_field | Stores the sample rate value for `AudioCaps`, in hertz. | `src/graph/ports.rs:49` |
| sym-9f6df5494182b64591aa | `AudioInputBufferError::WrongFrameLength::actual_samples` | struct_field | Contains the actual samples owned or reported by `WrongFrameLength`. | `src/session/extensions/audio_input/buffer.rs:291` |
| sym-4bfb97ad9db8691b332d | `AudioInputBufferError::WrongFrameLength::expected_samples` | struct_field | Contains the expected samples owned or reported by `WrongFrameLength`. | `src/session/extensions/audio_input/buffer.rs:290` |
| sym-8634752ad0746743faf4 | `AudioInputObservations::accepted_total` | struct_field | Counts the total number of accepted observed by `AudioInputObservations`. | `src/session/extensions/audio_input/buffer.rs:76` |
| sym-f5913563884208bacf7f | `AudioInputObservations::available_buffers` | struct_field | Contains the available buffers owned or reported by `AudioInputObservations`. | `src/session/extensions/audio_input/buffer.rs:75` |
| sym-eee885bf1a51fba8cedd | `AudioInputObservations::buffer_slots` | struct_field | Contains the buffer slots owned or reported by `AudioInputObservations`. | `src/session/extensions/audio_input/buffer.rs:74` |
| sym-1be7aed32cc859009e2f | `AudioInputObservations::cancelled` | struct_field | Reports whether cancelled is true for `AudioInputObservations`. | `src/session/extensions/audio_input/buffer.rs:79` |
| sym-97f77b9ae69029cdb922 | `AudioInputObservations::capacity_frames` | struct_field | Sets the capacity frames available to `AudioInputObservations`. | `src/session/extensions/audio_input/buffer.rs:73` |
| sym-d6e704a2a538986f8889 | `AudioInputObservations::closed` | struct_field | Reports whether closed is true for `AudioInputObservations`. | `src/session/extensions/audio_input/buffer.rs:80` |
| sym-3b63f8247f523bf9ad8c | `AudioInputObservations::full_total` | struct_field | Counts the total number of full observed by `AudioInputObservations`. | `src/session/extensions/audio_input/buffer.rs:77` |
| sym-13ac28cfc2a99c5916ca | `AudioInputObservations::invalid_total` | struct_field | Counts the total number of invalid observed by `AudioInputObservations`. | `src/session/extensions/audio_input/buffer.rs:78` |
| sym-176263a96d5d381b5fc3 | `CaptureBackendSet::application` | struct_field | Stores the application component of `CaptureBackendSet`. | `src/session/lifecycle/control.rs:18` |
| sym-78eacf628b7d76d579a2 | `CaptureBackendSet::microphone` | struct_field | Stores the microphone component of `CaptureBackendSet`. | `src/session/lifecycle/control.rs:19` |
| sym-f8d352a600d3ff315814 | `CaptureDelivery::frame_sender` | struct_field | Sends captured frames from `CaptureDelivery` into the Session runtime. | `src/capture/capture_owner.rs:74` |
| sym-bdc39fcfa064a60d2306 | `CaptureDelivery::runtime_event_sender` | struct_field | Sends capture lifecycle and failure events from `CaptureDelivery` to the Session runtime. | `src/capture/capture_owner.rs:75` |
| sym-946cee9b73047bf5f3ad | `CaptureObservations::callback_buffers_total` | struct_field | Counts the total number of callback buffers observed by `CaptureObservations`. | `src/capture/observations.rs:9` |
| sym-f7dfab792d4767d20aaa | `CaptureObservations::dispatch_queue_full_total` | struct_field | Counts the total number of dispatch queue full observed by `CaptureObservations`. | `src/capture/observations.rs:12` |
| sym-d6b7afdc8574439c7e36 | `CaptureObservations::frames_enqueued_total` | struct_field | Counts the total number of frames enqueued observed by `CaptureObservations`. | `src/capture/observations.rs:10` |
| sym-40f1b50af6cd8dd44901 | `CaptureObservations::invalid_buffer_total` | struct_field | Counts the total number of invalid buffer observed by `CaptureObservations`. | `src/capture/observations.rs:13` |
| sym-27f712c546a6dd895221 | `CaptureObservations::oversized_buffer_total` | struct_field | Counts the total number of oversized buffer observed by `CaptureObservations`. | `src/capture/observations.rs:14` |
| sym-5ac3ff4ff8ee3a71c952 | `CaptureObservations::pool_exhausted_total` | struct_field | Counts the total number of pool exhausted observed by `CaptureObservations`. | `src/capture/observations.rs:11` |
| sym-1b04c98ba87668704d93 | `CaptureObservations::stream_errors_total` | struct_field | Counts the total number of stream errors observed by `CaptureObservations`. | `src/capture/observations.rs:15` |
| sym-39bbe39f601d1d91db4a | `CaptureObservations::timestamp_epoch_clamps_total` | struct_field | Counts the total number of timestamp epoch clamps observed by `CaptureObservations`. | `src/capture/observations.rs:16` |
| sym-9581a6832ae177d4d20d | `CaptureOpenMetadata::clock_id` | struct_field | Identifies the clock identifier recorded by `CaptureOpenMetadata`. | `src/capture/capture_owner.rs:53` |
| sym-a5fdf8f0d7203d8d8290 | `CaptureOpenMetadata::discontinuity_epoch` | struct_field | Identifies the discontinuity generation attached to `CaptureOpenMetadata`. | `src/capture/capture_owner.rs:55` |
| sym-5839ebf0b360299b61b5 | `CaptureOpenMetadata::permission_epoch` | struct_field | Identifies the permission-observation generation attached to `CaptureOpenMetadata`. | `src/capture/capture_owner.rs:56` |
| sym-1d56f52c749bd33c7318 | `CaptureOpenMetadata::session_id` | struct_field | Identifies the session identifier recorded by `CaptureOpenMetadata`. | `src/capture/capture_owner.rs:50` |
| sym-742271d74f0cbcccb9f3 | `CaptureOpenMetadata::source_generation` | struct_field | References the source generation participating in `CaptureOpenMetadata`. | `src/capture/capture_owner.rs:54` |
| sym-6bd3fc8d78157a0489ca | `CaptureOpenMetadata::source_id` | struct_field | Identifies the source identifier recorded by `CaptureOpenMetadata`. | `src/capture/capture_owner.rs:51` |
| sym-b48a734f0148bcf322a9 | `CaptureOpenMetadata::stem_id` | struct_field | Identifies the stem identifier recorded by `CaptureOpenMetadata`. | `src/capture/capture_owner.rs:52` |
| sym-3570f8e51df4d215ffbf | `CaptureOwnerObservations::backend` | struct_field | Stores the backend as a `CaptureObservations` value in `CaptureOwnerObservations`. | `src/capture/capture_owner.rs:161` |
| sym-c3bfaab3824d224f64d5 | `CaptureOwnerObservations::frame_stream` | struct_field | Stores the frame stream as a `CapturedFrameStreamStats` value in `CaptureOwnerObservations`. | `src/capture/capture_owner.rs:162` |
| sym-965e7fb36a9e169d75e2 | `CaptureOwnerObservations::runtime_events` | struct_field | Contains the runtime events owned or reported by `CaptureOwnerObservations`. | `src/capture/capture_owner.rs:163` |
| sym-f539df5c6e22c14da5bf | `CapturePrepareRequest::frame_capacity_frames` | struct_field | Sets the frame capacity frames available to `CapturePrepareRequest`. | `src/capture/capture_owner.rs:64` |
| sym-372f3e2fc658f3fcbf9a | `CapturePrepareRequest::lineage_seed` | struct_field | Supplies the initial lineage identity used when `CapturePrepareRequest` opens capture. | `src/capture/capture_owner.rs:63` |
| sym-33b9b2dc6e5767b5841c | `CapturePrepareRequest::mode` | struct_field | Stores the mode as a `CaptureMode` value in `CapturePrepareRequest`. | `src/capture/capture_owner.rs:62` |
| sym-6ed46377d3675e08fd87 | `CapturePrepareRequest::runtime_event_capacity_events` | struct_field | Sets the runtime event capacity events available to `CapturePrepareRequest`. | `src/capture/capture_owner.rs:65` |
| sym-f997ca49b1ce8cef7258 | `CaptureSampleTimelineError::SourcePositionMovedBackward::expected_at_least` | struct_field | Stores the expected at least component of `SourcePositionMovedBackward`. | `src/capture/timeline.rs:45` |
| sym-1b6f1aaa7ad89f48c375 | `CaptureSampleTimelineError::SourcePositionMovedBackward::observed` | struct_field | Stores the observed component of `SourcePositionMovedBackward`. | `src/capture/timeline.rs:46` |
| sym-7b5cf48f4ad1b2f180f2 | `CaptureStopOutcome::observations` | struct_field | Carries the observations collected for `CaptureStopOutcome`. | `src/capture/capture_owner.rs:186` |
| sym-f0b279d0e24131872d5d | `CapturedFrameStreamStats::delivered_frames` | struct_field | Contains the delivered frames owned or reported by `CapturedFrameStreamStats`. | `src/capture/frame_stream.rs:18` |
| sym-9c25a7556595b2dd079b | `CapturedFrameStreamStats::dropped_newest_frames` | struct_field | Contains the dropped newest frames owned or reported by `CapturedFrameStreamStats`. | `src/capture/frame_stream.rs:19` |
| sym-e64bfcfb14cca95eee75 | `CapturedFrameStreamStats::frames_discarded_before_start_total` | struct_field | Counts the total number of frames discarded before start observed by `CapturedFrameStreamStats`. | `src/capture/frame_stream.rs:20` |
| sym-a04ca7a73c6d365d0091 | `ClockDriftSnapshot::accumulated_error_ns` | struct_field | Stores the accumulated error value for `ClockDriftSnapshot`, in nanoseconds. | `src/timing/clock_drift.rs:6` |
| sym-3302deaa63c3bb51ac21 | `ClockDriftSnapshot::drift_ppm` | struct_field | Reports the estimated clock drift for `ClockDriftSnapshot`, in parts per million. | `src/timing/clock_drift.rs:5` |
| sym-50189778197fe2717210 | `ClockDriftSnapshot::observed_samples_count` | struct_field | Stores the number of observed samples represented by `ClockDriftSnapshot`. | `src/timing/clock_drift.rs:7` |
| sym-dc0cf62cb5d21cebb918 | `CompileError::AdapterUnavailable::edge` | struct_field | References the edge participating in `AdapterUnavailable`. | `src/graph/compile/resolve.rs:62` |
| sym-83de2c93f15e8b08bc67 | `CompileError::AdapterUnavailable::type_id` | struct_field | Identifies the type identifier recorded by `AdapterUnavailable`. | `src/graph/compile/resolve.rs:62` |
| sym-deb935f617545d0b2b82 | `CompileError::ClockDomainMismatch::expected` | struct_field | Records the value expected by `ClockDomainMismatch`. | `src/graph/compile/resolve.rs:41` |
| sym-af39086f305521d9c4ae | `CompileError::ClockDomainMismatch::found` | struct_field | Stores the found as a `ClockDomain` value in `ClockDomainMismatch`. | `src/graph/compile/resolve.rs:42` |
| sym-79376b793d7a72469ba3 | `CompileError::ClockDomainMismatch::node` | struct_field | References the node participating in `ClockDomainMismatch`. | `src/graph/compile/resolve.rs:39` |
| sym-ad4a2121ca0ce0f4c323 | `CompileError::ClockDomainMismatch::port` | struct_field | References the port participating in `ClockDomainMismatch`. | `src/graph/compile/resolve.rs:40` |
| sym-e8479bdcca9d05dbbcb7 | `CompileError::InvalidConfig::reason` | struct_field | Carries the reason reported by `InvalidConfig`. | `src/graph/compile/resolve.rs:30` |
| sym-311d14052bcce57e6610 | `CompileError::InvalidConfig::type_id` | struct_field | Identifies the type identifier recorded by `InvalidConfig`. | `src/graph/compile/resolve.rs:30` |
| sym-d933bad9bd819b977689 | `CompileError::InvalidRealtimeEdge::edge` | struct_field | References the edge participating in `InvalidRealtimeEdge`. | `src/graph/compile/resolve.rs:58` |
| sym-a0dbc8cf791fde629f19 | `CompileError::InvalidRealtimeEdge::reason` | struct_field | Carries the reason reported by `InvalidRealtimeEdge`. | `src/graph/compile/resolve.rs:58` |
| sym-922519ced3d994154142 | `CompileError::InvalidSafetyContract::execution` | struct_field | Records the execution selected for `InvalidSafetyContract`. | `src/graph/compile/resolve.rs:54` |
| sym-93f24018726f7eef8c39 | `CompileError::InvalidSafetyContract::node` | struct_field | References the node participating in `InvalidSafetyContract`. | `src/graph/compile/resolve.rs:52` |
| sym-4ef2d35aa0e01fa6f653 | `CompileError::InvalidSafetyContract::safety` | struct_field | Stores the safety as a `SafetyContract` value in `InvalidSafetyContract`. | `src/graph/compile/resolve.rs:55` |
| sym-d184135c816a5c440eba | `CompileError::InvalidSafetyContract::type_id` | struct_field | Identifies the type identifier recorded by `InvalidSafetyContract`. | `src/graph/compile/resolve.rs:53` |
| sym-7df8a193b49e9844868a | `CompileError::MediaMismatch::edge` | struct_field | References the edge participating in `MediaMismatch`. | `src/graph/compile/resolve.rs:45` |
| sym-9a932c7e36691843c7bb | `CompileError::MediaMismatch::from` | struct_field | Identifies the origin represented by `MediaMismatch`. | `src/graph/compile/resolve.rs:45` |
| sym-573bd08fe082475e2238 | `CompileError::MediaMismatch::to` | struct_field | Identifies the destination represented by `MediaMismatch`. | `src/graph/compile/resolve.rs:45` |
| sym-5bd936c8fed52c3b554e | `CompileError::SignalMismatch::edge` | struct_field | References the edge participating in `SignalMismatch`. | `src/graph/compile/resolve.rs:47` |
| sym-7b37fc1fb6f0c81d1b12 | `CompileError::SignalMismatch::from` | struct_field | Identifies the origin represented by `SignalMismatch`. | `src/graph/compile/resolve.rs:47` |
| sym-835c109871aee9eef7c0 | `CompileError::SignalMismatch::to` | struct_field | Identifies the destination represented by `SignalMismatch`. | `src/graph/compile/resolve.rs:47` |
| sym-b643ce81406d4f0fb392 | `CompileError::UnknownPort::node` | struct_field | References the node participating in `UnknownPort`. | `src/graph/compile/resolve.rs:34` |
| sym-ae32bfd11d646d773668 | `CompileError::UnknownPort::port` | struct_field | References the port participating in `UnknownPort`. | `src/graph/compile/resolve.rs:34` |
| sym-3d58f0ecd50ef98e7e14 | `CompileError::WrongPortDirection::node` | struct_field | References the node participating in `WrongPortDirection`. | `src/graph/compile/resolve.rs:36` |
| sym-8403bc897837e6fd048c | `CompileError::WrongPortDirection::port` | struct_field | References the port participating in `WrongPortDirection`. | `src/graph/compile/resolve.rs:36` |
| sym-305dc1831d7b346cef28 | `CompiledOperatorInputContract::capacity_signals` | struct_field | Sets the capacity signals available to `CompiledOperatorInputContract`. | `src/runtime/signal/operator.rs:113` |
| sym-c9858a88b36081128289 | `CompiledOperatorInputContract::edge_contract` | struct_field | References the edge contract participating in `CompiledOperatorInputContract`. | `src/runtime/signal/operator.rs:112` |
| sym-9434282db04627d1212e | `CompiledOperatorInputContract::edge_id` | struct_field | Identifies the edge identifier recorded by `CompiledOperatorInputContract`. | `src/runtime/signal/operator.rs:104` |
| sym-13a16cc7af202ca53638 | `CompiledOperatorInputContract::input_port` | struct_field | References the input port participating in `CompiledOperatorInputContract`. | `src/runtime/signal/operator.rs:109` |
| sym-bef140fd6aef68c5c033 | `CompiledOperatorInputContract::media` | struct_field | Records the media selected for `CompiledOperatorInputContract`. | `src/runtime/signal/operator.rs:111` |
| sym-02c0291aa4af2474b047 | `CompiledOperatorInputContract::operator_node` | struct_field | References the operator node participating in `CompiledOperatorInputContract`. | `src/runtime/signal/operator.rs:105` |
| sym-b042ed4e3104b730179c | `CompiledOperatorInputContract::session_id` | struct_field | Identifies the session identifier recorded by `CompiledOperatorInputContract`. | `src/runtime/signal/operator.rs:106` |
| sym-72b81e8e352798f5ec82 | `CompiledOperatorInputContract::signal_spec` | struct_field | Declares the signal class and format accepted by `CompiledOperatorInputContract`. | `src/runtime/signal/operator.rs:110` |
| sym-d21c025cd3c6bf65797f | `CompiledOperatorInputContract::source_id` | struct_field | Identifies the source identifier recorded by `CompiledOperatorInputContract`. | `src/runtime/signal/operator.rs:108` |
| sym-7b5adbdbeec677ebd9d7 | `CompiledOperatorInputContract::stem_id` | struct_field | Identifies the stem identifier recorded by `CompiledOperatorInputContract`. | `src/runtime/signal/operator.rs:107` |
| sym-927e2db274c3651257a6 | `ConfigError::Invalid::key` | struct_field | Stores the key text reported by `Invalid`. | `src/graph/node.rs:145` |
| sym-a27a8d2d663b5a8efd75 | `ConfigError::Invalid::reason` | struct_field | Carries the reason reported by `Invalid`. | `src/graph/node.rs:145` |
| sym-f501c0ee0f1920d04b88 | `ConnectionTarget::EndpointInput::endpoint_id` | struct_field | Identifies the endpoint identifier recorded by `EndpointInput`. | `src/session/declaration/spec.rs:230` |
| sym-54ddbee44b9350b8ab94 | `ConnectionTarget::EndpointInput::input_port` | struct_field | References the input port participating in `EndpointInput`. | `src/session/declaration/spec.rs:231` |
| sym-2cf50203bb5782cc56fa | `ConnectionTarget::OperatorInput::input_port` | struct_field | References the input port participating in `OperatorInput`. | `src/session/declaration/spec.rs:227` |
| sym-fd6c7b9dbef1f05dc0ff | `ConnectionTarget::OperatorInput::operator_instance_id` | struct_field | Identifies the operator instance identifier recorded by `OperatorInput`. | `src/session/declaration/spec.rs:226` |
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
| sym-9e941a881632af08bf23 | `DiscontinuityRecord::kind` | struct_field | Records the kind selected for `DiscontinuityRecord`. | `src/recording/writer.rs:96` |
| sym-03ec7a71680c75a71368 | `DiscontinuityRecord::label` | struct_field | Stores the human-readable label used to identify `DiscontinuityRecord`. | `src/recording/writer.rs:95` |
| sym-ac6bd0adc25798673aab | `DiscontinuityRecord::sequence_end` | struct_field | Records the last sequence number covered by `DiscontinuityRecord`. | `src/recording/writer.rs:100` |
| sym-2f28406e52717c3a205d | `DiscontinuityRecord::sequence_start` | struct_field | Records the first sequence number covered by `DiscontinuityRecord`. | `src/recording/writer.rs:99` |
| sym-adc81069ce45696c3093 | `DiscontinuityRecord::stem_id` | struct_field | Identifies the stem identifier recorded by `DiscontinuityRecord`. | `src/recording/writer.rs:94` |
| sym-e5fd2de238aea90ac3fd | `DiscontinuityRecord::timestamp_end_ns` | struct_field | Stores the timestamp end value for `DiscontinuityRecord`, in nanoseconds. | `src/recording/writer.rs:98` |
| sym-eb813f0373241f2bfa70 | `DiscontinuityRecord::timestamp_start_ns` | struct_field | Stores the timestamp start value for `DiscontinuityRecord`, in nanoseconds. | `src/recording/writer.rs:97` |
| sym-2b415befd9417293bc15 | `DispatchSummary::attempted_edges` | struct_field | References the attempted edges participating in `DispatchSummary`. | `src/runtime/audio/router.rs:697` |
| sym-145e2bd2ca8aee068b82 | `DispatchSummary::copy_pool_exhausted_edges` | struct_field | References the copy pool exhausted edges participating in `DispatchSummary`. | `src/runtime/audio/router.rs:700` |
| sym-26c5c081a1c3a92c9749 | `DispatchSummary::dropped_edges` | struct_field | References the dropped edges participating in `DispatchSummary`. | `src/runtime/audio/router.rs:699` |
| sym-563e0c54a1287f64a220 | `DispatchSummary::enqueued_edges` | struct_field | References the enqueued edges participating in `DispatchSummary`. | `src/runtime/audio/router.rs:698` |
| sym-f246f8bfcc7ac4ef3f8d | `DispatchSummary::freeze_failed_edges` | struct_field | References the freeze failed edges participating in `DispatchSummary`. | `src/runtime/audio/router.rs:701` |
| sym-b4df6fbf82c690ad050f | `EdgeBufferPlan::bytes_per_frame` | struct_field | Stores the encoded or in-memory size of one frame for `EdgeBufferPlan`, in bytes. | `src/graph/plan.rs:39` |
| sym-56ce8c734285bb8421e3 | `EdgeBufferPlan::capacity_frames` | struct_field | Sets the capacity frames available to `EdgeBufferPlan`. | `src/graph/plan.rs:38` |
| sym-f5ae0cfda4b6a1002c99 | `EdgeBufferPlan::copy_policy` | struct_field | Declares whether routing through `EdgeBufferPlan` may share or must copy frame storage. | `src/graph/plan.rs:40` |
| sym-b9efe3d4827e9056db91 | `EdgeBufferPlan::edge` | struct_field | References the edge participating in `EdgeBufferPlan`. | `src/graph/plan.rs:37` |
| sym-3cdf6838f5975baa8d31 | `EdgeObservations::branch_pool_exhausted_drops_total` | struct_field | Counts the total number of branch pool exhausted drops observed by `EdgeObservations`. | `src/runtime/audio/router.rs:153` |
| sym-fca83aa2b3c5622e6fdf | `EdgeObservations::discontinuities_total` | struct_field | Counts the total number of discontinuities observed by `EdgeObservations`. | `src/runtime/audio/router.rs:156` |
| sym-e4d0b59a99a199894312 | `EdgeObservations::enqueue_to_receive_invalid_order_total` | struct_field | Counts the total number of enqueue to receive invalid order observed by `EdgeObservations`. | `src/runtime/audio/router.rs:163` |
| sym-05f6be4d46e4116b4722 | `EdgeObservations::enqueue_to_receive_max_ns` | struct_field | Stores the enqueue to receive max value for `EdgeObservations`, in nanoseconds. | `src/runtime/audio/router.rs:167` |
| sym-25134bef6ef2db5daed3 | `EdgeObservations::enqueue_to_receive_p50_ns` | struct_field | Stores the enqueue to receive p50 value for `EdgeObservations`, in nanoseconds. | `src/runtime/audio/router.rs:164` |
| sym-1b6df992b52b731bc24c | `EdgeObservations::enqueue_to_receive_p95_ns` | struct_field | Stores the enqueue to receive p95 value for `EdgeObservations`, in nanoseconds. | `src/runtime/audio/router.rs:165` |
| sym-155282e84cad530d7fe8 | `EdgeObservations::enqueue_to_receive_p99_ns` | struct_field | Stores the enqueue to receive p99 value for `EdgeObservations`, in nanoseconds. | `src/runtime/audio/router.rs:166` |
| sym-3851bc0aae896934cfb8 | `EdgeObservations::enqueue_to_receive_samples_total` | struct_field | Counts the total number of enqueue to receive samples observed by `EdgeObservations`. | `src/runtime/audio/router.rs:162` |
| sym-ff77bb0b849cdbb21816 | `EdgeObservations::frames_delivered_total` | struct_field | Counts the total number of frames delivered observed by `EdgeObservations`. | `src/runtime/audio/router.rs:147` |
| sym-7b746d2d5b6f712c23d3 | `EdgeObservations::frames_dropped_total` | struct_field | Counts the total number of frames dropped observed by `EdgeObservations`. | `src/runtime/audio/router.rs:148` |
| sym-ce4dce28a5c9719e469a | `EdgeObservations::frames_enqueued_total` | struct_field | Counts the total number of frames enqueued observed by `EdgeObservations`. | `src/runtime/audio/router.rs:146` |
| sym-8dc0dc1a881157183dc2 | `EdgeObservations::freeze_failed_drops_total` | struct_field | Counts the total number of freeze failed drops observed by `EdgeObservations`. | `src/runtime/audio/router.rs:155` |
| sym-6d685747b5f28d042b52 | `EdgeObservations::invalid_copy_policy_drops_total` | struct_field | Counts the total number of invalid copy policy drops observed by `EdgeObservations`. | `src/runtime/audio/router.rs:154` |
| sym-317f16fac2f727395fc4 | `EdgeObservations::lineage_epoch_discontinuities_total` | struct_field | Counts the total number of lineage epoch discontinuities observed by `EdgeObservations`. | `src/runtime/audio/router.rs:160` |
| sym-c7ba5b001d62d7500abe | `EdgeObservations::manually_reported_discontinuities_total` | struct_field | Counts the total number of manually reported discontinuities observed by `EdgeObservations`. | `src/runtime/audio/router.rs:161` |
| sym-4ca9397d7c66663b35a7 | `EdgeObservations::overruns_total` | struct_field | Counts the total number of overruns observed by `EdgeObservations`. | `src/runtime/audio/router.rs:149` |
| sym-5295c704bb1b7f04a0d6 | `EdgeObservations::queue_capacity_frames` | struct_field | Sets the queue capacity frames available to `EdgeObservations`. | `src/runtime/audio/router.rs:143` |
| sym-3bfc72cc78bbdf7803a4 | `EdgeObservations::queue_depth_frames` | struct_field | Reports the queue depth frames observed by `EdgeObservations`. | `src/runtime/audio/router.rs:144` |
| sym-b376caf96d7bcc75e000 | `EdgeObservations::queue_full_drops_total` | struct_field | Counts the total number of queue full drops observed by `EdgeObservations`. | `src/runtime/audio/router.rs:151` |
| sym-19402b0256d71d88d084 | `EdgeObservations::queue_peak_frames` | struct_field | Reports the queue peak frames observed by `EdgeObservations`. | `src/runtime/audio/router.rs:145` |
| sym-0126eebefdefc8504f8a | `EdgeObservations::receiver_unavailable_drops_total` | struct_field | Counts the total number of receiver unavailable drops observed by `EdgeObservations`. | `src/runtime/audio/router.rs:150` |
| sym-da5820b29621f8c81cb1 | `EdgeObservations::sequence_discontinuities_total` | struct_field | Counts the total number of sequence discontinuities observed by `EdgeObservations`. | `src/runtime/audio/router.rs:158` |
| sym-8a3b467084e567bfbc2e | `EdgeObservations::shared_reference_exhausted_drops_total` | struct_field | Counts the total number of shared reference exhausted drops observed by `EdgeObservations`. | `src/runtime/audio/router.rs:152` |
| sym-6610bea8d4a3d67b31cc | `EdgeObservations::shutdown_discarded_total` | struct_field | Counts the total number of shutdown discarded observed by `EdgeObservations`. | `src/runtime/audio/router.rs:176` |
| sym-e37594323a14118e7488 | `EdgeObservations::source_identity_discontinuities_total` | struct_field | Counts the total number of source identity discontinuities observed by `EdgeObservations`. | `src/runtime/audio/router.rs:157` |
| sym-2c968bc32f7c79a21e45 | `EdgeObservations::source_timestamp_to_receive_future_total` | struct_field | Counts the total number of source timestamp to receive future observed by `EdgeObservations`. | `src/runtime/audio/router.rs:170` |
| sym-6dc6ba917ffc87b9cf00 | `EdgeObservations::source_timestamp_to_receive_max_ns` | struct_field | Stores the source timestamp to receive max value for `EdgeObservations`, in nanoseconds. | `src/runtime/audio/router.rs:174` |
| sym-59495a9a2cbe6cdf6d90 | `EdgeObservations::source_timestamp_to_receive_missing_total` | struct_field | Counts the total number of source timestamp to receive missing observed by `EdgeObservations`. | `src/runtime/audio/router.rs:169` |
| sym-11fce4f2aacc2b712d5e | `EdgeObservations::source_timestamp_to_receive_p50_ns` | struct_field | Stores the source timestamp to receive p50 value for `EdgeObservations`, in nanoseconds. | `src/runtime/audio/router.rs:171` |
| sym-fda669788111c42abf5b | `EdgeObservations::source_timestamp_to_receive_p95_ns` | struct_field | Stores the source timestamp to receive p95 value for `EdgeObservations`, in nanoseconds. | `src/runtime/audio/router.rs:172` |
| sym-9392aac8083ef87dadaf | `EdgeObservations::source_timestamp_to_receive_p99_ns` | struct_field | Stores the source timestamp to receive p99 value for `EdgeObservations`, in nanoseconds. | `src/runtime/audio/router.rs:173` |
| sym-b0368f38d2f1d9da6f89 | `EdgeObservations::source_timestamp_to_receive_samples_total` | struct_field | Counts the total number of source timestamp to receive samples observed by `EdgeObservations`. | `src/runtime/audio/router.rs:168` |
| sym-b44cb70f3f690bd11bfb | `EdgeObservations::timestamp_discontinuities_total` | struct_field | Counts the total number of timestamp discontinuities observed by `EdgeObservations`. | `src/runtime/audio/router.rs:159` |
| sym-2a761216085a54b5c2ba | `EdgeObservations::worker_failures_total` | struct_field | Counts the total number of worker failures observed by `EdgeObservations`. | `src/runtime/audio/router.rs:175` |
| sym-2762d38a732b56b3ec17 | `EdgeSpec::from` | struct_field | Identifies the origin represented by `EdgeSpec`. | `src/graph/spec.rs:52` |
| sym-ab58adee1431c8d25a67 | `EdgeSpec::id` | struct_field | Identifies the id recorded by `EdgeSpec`. | `src/graph/spec.rs:51` |
| sym-831c7533344dc5770adc | `EdgeSpec::requested` | struct_field | Stores the requested component of `EdgeSpec`. | `src/graph/spec.rs:54` |
| sym-78a1b123e9a638d9279f | `EdgeSpec::to` | struct_field | Identifies the destination represented by `EdgeSpec`. | `src/graph/spec.rs:53` |
| sym-1dbd93f00685779469ae | `EndpointCancellationOutcome::observations` | struct_field | Carries the observations collected for `EndpointCancellationOutcome`. | `src/endpoint/runtime.rs:290` |
| sym-0bb31c638640c40d65ea | `EndpointCancellationOutcome::result` | struct_field | Records whether the result operation succeeded and preserves its typed failure for `EndpointCancellationOutcome`. | `src/endpoint/runtime.rs:291` |
| sym-e30b971fde8bafec2d7d | `EndpointDriverFinalization::observations` | struct_field | Carries the observations collected for `EndpointDriverFinalization`. | `src/endpoint/runtime.rs:296` |
| sym-3f312e11c9c30f537c17 | `EndpointDriverFinalization::result` | struct_field | Records whether the result operation succeeded and preserves its typed failure for `EndpointDriverFinalization`. | `src/endpoint/runtime.rs:297` |
| sym-26591ca38fc91f403f98 | `EndpointDriverObservations::discontinuities_total` | struct_field | Counts the total number of discontinuities observed by `EndpointDriverObservations`. | `src/endpoint/runtime.rs:232` |
| sym-3042fb4688f1c3bc09f5 | `EndpointDriverObservations::failures_total` | struct_field | Counts the total number of failures observed by `EndpointDriverObservations`. | `src/endpoint/runtime.rs:233` |
| sym-2914129c9eb77a6ac869 | `EndpointDriverObservations::frames_delivered_total` | struct_field | Counts the total number of frames delivered observed by `EndpointDriverObservations`. | `src/endpoint/runtime.rs:230` |
| sym-ba04d920f19dc9ef617b | `EndpointDriverObservations::frames_dropped_total` | struct_field | Counts the total number of frames dropped observed by `EndpointDriverObservations`. | `src/endpoint/runtime.rs:231` |
| sym-fd2745ee2fc824c172eb | `EndpointDriverObservations::frames_received_total` | struct_field | Counts the total number of frames received observed by `EndpointDriverObservations`. | `src/endpoint/runtime.rs:229` |
| sym-5dbe8de137698daf52f5 | `EndpointDriverRegistryError::Duplicate::node_type_id` | struct_field | Identifies the node type identifier recorded by `Duplicate`. | `src/endpoint/registry.rs:26` |
| sym-853c1daeeb38e3466ec6 | `EndpointDriverRegistryError::Duplicate::operator_id` | struct_field | Identifies the operator identifier recorded by `Duplicate`. | `src/endpoint/registry.rs:25` |
| sym-c70b71d5a1909757f792 | `EndpointDriverRegistryError::OperatorNodeTypeConflict::operator_id` | struct_field | Identifies the operator identifier recorded by `OperatorNodeTypeConflict`. | `src/endpoint/registry.rs:32` |
| sym-a45719e51311db56ae19 | `EndpointDriverRegistryError::OperatorNodeTypeConflict::registered_node_type_id` | struct_field | Identifies the registered node type identifier recorded by `OperatorNodeTypeConflict`. | `src/endpoint/registry.rs:33` |
| sym-098ec2fcef156091c011 | `EndpointDriverRegistryError::OperatorNodeTypeConflict::requested_node_type_id` | struct_field | Identifies the requested node type identifier recorded by `OperatorNodeTypeConflict`. | `src/endpoint/registry.rs:34` |
| sym-65538c5ca30cb6f29977 | `EndpointExtensionRegistrationError::ConflictingDefinition::node_type_id` | struct_field | Identifies the node type identifier recorded by `ConflictingDefinition`. | `src/session/lifecycle/engine.rs:311` |
| sym-d7450b886665f693c255 | `EndpointFinalizationOutcome::join_finalize_result` | struct_field | Records whether the join finalize result operation succeeded and preserves its typed failure for `EndpointFinalizationOutcome`. | `src/endpoint/runtime.rs:304` |
| sym-f71a74d2c891690533eb | `EndpointFinalizationOutcome::observations` | struct_field | Carries the observations collected for `EndpointFinalizationOutcome`. | `src/endpoint/runtime.rs:302` |
| sym-bc281d3d8b09873a9efa | `EndpointFinalizationOutcome::request_stop_result` | struct_field | Records whether the request stop result operation succeeded and preserves its typed failure for `EndpointFinalizationOutcome`. | `src/endpoint/runtime.rs:303` |
| sym-71e2bdab0ab1062febe4 | `EndpointInputOrigin::Source::audio_stem_id` | struct_field | Identifies the audio stem identifier recorded by `Source`. | `src/endpoint/runtime.rs:38` |
| sym-36dd04c79f54df69253b | `EndpointInputOrigin::Source::source_id` | struct_field | Identifies the source identifier recorded by `Source`. | `src/endpoint/runtime.rs:36` |
| sym-c3aa2156b2e8ae48a9d5 | `EndpointInputOrigin::Source::stream_id` | struct_field | Identifies the stream identifier recorded by `Source`. | `src/endpoint/runtime.rs:37` |
| sym-a1bc17c54e0fd69fb009 | `EndpointPrepareError::NotRegistered::node_type_id` | struct_field | Identifies the node type identifier recorded by `NotRegistered`. | `src/endpoint/registry.rs:47` |
| sym-08ea7ae98668e4f410e8 | `EndpointPrepareError::NotRegistered::operator_id` | struct_field | Identifies the operator identifier recorded by `NotRegistered`. | `src/endpoint/registry.rs:46` |
| sym-0280bcbb7b36e1f9d20b | `EndpointReceiver::Audio::receiver` | struct_field | Owns the receiver endpoint through which `Audio` exchanges values. | `src/endpoint/contract.rs:177` |
| sym-7c15983f5471308539d8 | `EndpointReceiver::Audio::sample_spec` | struct_field | Declares the sample rate, channel layout, and format used by `Audio`. | `src/endpoint/contract.rs:178` |
| sym-94005662812f10019762 | `FanInGroup::into` | struct_field | Stores the into as a `InputPortRef` value in `FanInGroup`. | `src/graph/plan.rs:91` |
| sym-f6e4dd94c03f86a7006d | `FanInGroup::sources` | struct_field | References the sources participating in `FanInGroup`. | `src/graph/plan.rs:92` |
| sym-eb00aad9305f74d4e2ac | `FanOutGroup::from` | struct_field | Identifies the origin represented by `FanOutGroup`. | `src/graph/plan.rs:85` |
| sym-4d57db48e9d138e6d7fd | `FanOutGroup::targets` | struct_field | Contains the targets owned or reported by `FanOutGroup`. | `src/graph/plan.rs:86` |
| sym-6234497c4595825d4984 | `GeneratedAudioBridgeSpec::clock_id` | struct_field | Identifies the clock identifier recorded by `GeneratedAudioBridgeSpec`. | `src/runtime/bridge/audio.rs:24` |
| sym-09d9b067f79e38a0ebd7 | `GeneratedAudioBridgeSpec::pool_slots` | struct_field | Contains the pool slots owned or reported by `GeneratedAudioBridgeSpec`. | `src/runtime/bridge/audio.rs:27` |
| sym-4e68a2bdda3c214e407b | `GeneratedAudioBridgeSpec::sample_spec` | struct_field | Declares the sample rate, channel layout, and format used by `GeneratedAudioBridgeSpec`. | `src/runtime/bridge/audio.rs:25` |
| sym-7689324ba940fd3aedec | `GeneratedAudioBridgeSpec::samples_per_frame` | struct_field | Stores the number of samples in each channel of a frame handled by `GeneratedAudioBridgeSpec`. | `src/runtime/bridge/audio.rs:26` |
| sym-1f5418b8f1e20db61218 | `GeneratedAudioBridgeSpec::session_id` | struct_field | Identifies the session identifier recorded by `GeneratedAudioBridgeSpec`. | `src/runtime/bridge/audio.rs:20` |
| sym-17f465234ae8c5206489 | `GeneratedAudioBridgeSpec::source_id` | struct_field | Identifies the source identifier recorded by `GeneratedAudioBridgeSpec`. | `src/runtime/bridge/audio.rs:23` |
| sym-f840803c8f922386d020 | `GeneratedAudioBridgeSpec::stem_id` | struct_field | Identifies the stem identifier recorded by `GeneratedAudioBridgeSpec`. | `src/runtime/bridge/audio.rs:21` |
| sym-d47f56e1bfa77cfe287b | `GeneratedAudioBridgeSpec::stream_id` | struct_field | Identifies the stream identifier recorded by `GeneratedAudioBridgeSpec`. | `src/runtime/bridge/audio.rs:22` |
| sym-04452887704079dbc0d6 | `GraphIr::edges` | struct_field | References the edges participating in `GraphIr`. | `src/graph/ir.rs:34` |
| sym-0f01fd4d8112465c16de | `GraphIr::nodes` | struct_field | References the nodes participating in `GraphIr`. | `src/graph/ir.rs:33` |
| sym-2306f2c975880d9c0be8 | `GraphIr::topo_order` | struct_field | Lists graph nodes in the validated topological execution order for `GraphIr`. | `src/graph/ir.rs:35` |
| sym-cdbc64024b10cab6cec9 | `GraphSpec::edges` | struct_field | References the edges participating in `GraphSpec`. | `src/graph/spec.rs:60` |
| sym-5801b8654a028f803f16 | `GraphSpec::nodes` | struct_field | References the nodes participating in `GraphSpec`. | `src/graph/spec.rs:59` |
| sym-52cf6c22d7a45ca2034b | `InputPortRef::node` | struct_field | References the node participating in `InputPortRef`. | `src/graph/spec.rs:38` |
| sym-a00d7d2fb1cd7212b247 | `InputPortRef::port` | struct_field | References the port participating in `InputPortRef`. | `src/graph/spec.rs:39` |
| sym-5f8c1be10a99be27c9de | `MemoryPlan::branch_copy_pool_bytes` | struct_field | Stores the branch copy pool size for `MemoryPlan`, in bytes. | `src/graph/plan.rs:66` |
| sym-4b93c800c77a200a5fda | `MemoryPlan::edge_buffers` | struct_field | References the edge buffers participating in `MemoryPlan`. | `src/graph/plan.rs:67` |
| sym-5a2d5bdee92831a59dd7 | `MemoryPlan::realtime_pool_bytes` | struct_field | Stores the realtime pool size for `MemoryPlan`, in bytes. | `src/graph/plan.rs:65` |
| sym-6dc1707a8f53e4333e3a | `NativeSessionEngineHostOptions::polled_audio_endpoint` | struct_field | References the polled audio endpoint participating in `NativeSessionEngineHostOptions`. | `src/session/lifecycle/host.rs:168` |
| sym-71e6c8f0600a6c9e95d9 | `NativeSessionEngineHostOptions::sample_spec` | struct_field | Declares the sample rate, channel layout, and format used by `NativeSessionEngineHostOptions`. | `src/session/lifecycle/host.rs:165` |
| sym-a4c2a30899c34c64ab87 | `NativeSessionEngineHostOptions::source_queue_capacity_frames` | struct_field | Sets the source queue capacity frames available to `NativeSessionEngineHostOptions`. | `src/session/lifecycle/host.rs:166` |
| sym-1f910aa161a624f23c42 | `NativeSessionEngineHostOptions::start_options` | struct_field | Contains the start options owned or reported by `NativeSessionEngineHostOptions`. | `src/session/lifecycle/host.rs:167` |
| sym-2b17d8cdd5847dd15d96 | `NodeError::ExternalBoundaryExecution::node_type_id` | struct_field | Identifies the node type identifier recorded by `ExternalBoundaryExecution`. | `src/graph/node.rs:159` |
| sym-c6a5ef988f89164491fd | `NodeError::ProcessTimeout::timeout_ms` | struct_field | Stores the timeout value for `ProcessTimeout`, in milliseconds. | `src/graph/node.rs:155` |
| sym-07f681a8acd285813ce7 | `NodeRegistrationError::DuplicateNodeType::node_type_id` | struct_field | Identifies the node type identifier recorded by `DuplicateNodeType`. | `src/graph/registry.rs:61` |
| sym-7d77c0554b3041f49b34 | `NodeRegistrationError::DuplicateOperatorId::operator_id` | struct_field | Identifies the operator identifier recorded by `DuplicateOperatorId`. | `src/graph/registry.rs:63` |
| sym-69a1f70e2c9008b2c876 | `NodeSpec::config` | struct_field | Stores the config as a `NodeConfig` value in `NodeSpec`. | `src/graph/spec.rs:46` |
| sym-7b9ce9455576643285e7 | `NodeSpec::id` | struct_field | Identifies the id recorded by `NodeSpec`. | `src/graph/spec.rs:44` |
| sym-6832ce998ba842f2a854 | `NodeSpec::type_id` | struct_field | Identifies the type identifier recorded by `NodeSpec`. | `src/graph/spec.rs:45` |
| sym-c5cb4f86d171747ff531 | `OperatorDeadlinePolicy::process_timeout_ms` | struct_field | Stores the process timeout value for `OperatorDeadlinePolicy`, in milliseconds. | `src/graph/signal/operator.rs:53` |
| sym-f552cb05757b70bde069 | `OperatorOutputRolePolicy::allowed` | struct_field | Stores the allowed component of `OperatorOutputRolePolicy`. | `src/graph/signal/operator.rs:70` |
| sym-412f3b00ac99acbd3f27 | `OperatorOutputRolePolicy::terminal` | struct_field | Indicates whether terminal applies to `OperatorOutputRolePolicy`. | `src/graph/signal/operator.rs:71` |
| sym-6926a63fd6a6228d9035 | `OperatorPermissionPolicy::filesystem_allowed` | struct_field | Reports whether filesystem is allowed for `OperatorPermissionPolicy`. | `src/graph/signal/operator.rs:48` |
| sym-db110792e41d4e4a1cb6 | `OperatorPermissionPolicy::network_allowed` | struct_field | Reports whether network is allowed for `OperatorPermissionPolicy`. | `src/graph/signal/operator.rs:47` |
| sym-b62b4c170af777f0ba26 | `OpusConfig::application` | struct_field | Selects the Opus application mode used when the encoder is created. | `src/codec/encoder.rs:80` |
| sym-517b83dbabadfb9c7e13 | `OpusConfig::bitrate_kbps` | struct_field | Target bitrate in kbps. None = Opus auto (variable bitrate). | `src/codec/encoder.rs:82` |
| sym-16b114736cc7634805ac | `OpusConfig::channels` | struct_field | Selects the mono or stereo channel layout accepted by the encoder. | `src/codec/encoder.rs:76` |
| sym-c80e591dd7da39fe1ec6 | `OpusConfig::complexity` | struct_field | Encoder complexity 0–10. Higher = better quality, more CPU. | `src/codec/encoder.rs:84` |
| sym-063407a74191d8ee5726 | `OpusConfig::dtx` | struct_field | Discontinuous transmission (silence suppression). | `src/codec/encoder.rs:86` |
| sym-a834c990f442972720d1 | `OpusConfig::fec` | struct_field | In-band forward error correction. | `src/codec/encoder.rs:88` |
| sym-261d6440732347b21057 | `OpusConfig::frame_duration` | struct_field | Frame duration. Default: 20 ms (AUDIO-012). | `src/codec/encoder.rs:78` |
| sym-658cc1d8211b18ff7a02 | `OpusConfig::sample_rate` | struct_field | Sample rate. Opus only supports 48 kHz internally. | `src/codec/encoder.rs:74` |
| sym-8689ac88deced0826cdd | `OpusDecodeError::FrameDurationExceedsConfiguredMaximum::maximum_samples_per_channel` | struct_field | Records the configured maximum frame length, in samples per channel, enforced by `FrameDurationExceedsConfiguredMaximum`. | `src/codec/decoder.rs:31` |
| sym-f7da454cbdb2d87e5835 | `OpusDecodeError::FrameDurationExceedsConfiguredMaximum::requested_samples_per_channel` | struct_field | Records the requested frame length, in samples per channel, that caused `FrameDurationExceedsConfiguredMaximum`. | `src/codec/decoder.rs:30` |
| sym-280c69ee026bde4fe5d3 | `OpusEncodeError::InvalidFrameSampleCount::channels` | struct_field | Contains the channels owned or reported by `InvalidFrameSampleCount`. | `src/codec/encoder.rs:137` |
| sym-ed3451897f9409138985 | `OpusEncodeError::InvalidFrameSampleCount::expected_sample_count` | struct_field | Stores the number of expected sample represented by `InvalidFrameSampleCount`. | `src/codec/encoder.rs:138` |
| sym-58d3176756f862642135 | `OpusEncodeError::InvalidFrameSampleCount::sample_count` | struct_field | Stores the number of sample represented by `InvalidFrameSampleCount`. | `src/codec/encoder.rs:136` |
| sym-f34b3117cf03b8d0da21 | `OutputPortRef::node` | struct_field | References the node participating in `OutputPortRef`. | `src/graph/spec.rs:32` |
| sym-40a35bf97da8c418bfa4 | `OutputPortRef::port` | struct_field | References the port participating in `OutputPortRef`. | `src/graph/spec.rs:33` |
| sym-a3034b79b656a8c2ab82 | `PartitionGroup::execution` | struct_field | Records the execution selected for `PartitionGroup`. | `src/graph/plan.rs:79` |
| sym-f9edad39eb194e9014c1 | `PartitionGroup::nodes` | struct_field | References the nodes participating in `PartitionGroup`. | `src/graph/plan.rs:80` |
| sym-9d17e8591a9ffed70991 | `PksExtensionAbiVersion::abi_major` | struct_field | Stores the major ABI version expected by `PksExtensionAbiVersion`. | `src/abi/extension.rs:16` |
| sym-fee228f569854aa8601d | `PksExtensionAbiVersion::abi_minor` | struct_field | Stores the minor ABI version expected by `PksExtensionAbiVersion`. | `src/abi/extension.rs:17` |
| sym-f8cef8ca2c3556aae4ab | `PksExtensionAbiVersion::struct_size_bytes` | struct_field | Stores the byte size of the `PksExtensionAbiVersion` ABI structure. | `src/abi/extension.rs:15` |
| sym-83b2969d41412b4d7a37 | `PksExtensionCallbacks::abi_major` | struct_field | Stores the major ABI version expected by `PksExtensionCallbacks`. | `src/abi/executable_extension.rs:93` |
| sym-b9ec1cd153ae49f53149 | `PksExtensionCallbacks::abi_minor` | struct_field | Stores the minor ABI version expected by `PksExtensionCallbacks`. | `src/abi/executable_extension.rs:94` |
| sym-d1651e41136e427ac15c | `PksExtensionCallbacks::create` | struct_field | Provides the create callback used by `PksExtensionCallbacks`. | `src/abi/executable_extension.rs:99` |
| sym-fa371318b9fa5538ab95 | `PksExtensionCallbacks::destroy_instance` | struct_field | Provides the destroy instance callback used by `PksExtensionCallbacks`. | `src/abi/executable_extension.rs:106` |
| sym-fd8e8561d432e6ca0902 | `PksExtensionCallbacks::destroy_registration` | struct_field | Provides the destroy registration callback used by `PksExtensionCallbacks`. | `src/abi/executable_extension.rs:107` |
| sym-cdf572e57f6d9cf70938 | `PksExtensionCallbacks::endpoint_consume` | struct_field | Provides the endpoint consume callback used by `PksExtensionCallbacks`. | `src/abi/executable_extension.rs:103` |
| sym-4eb162bc05a3e9ec5044 | `PksExtensionCallbacks::finish` | struct_field | Provides the finish callback used by `PksExtensionCallbacks`. | `src/abi/executable_extension.rs:105` |
| sym-d20d58f4a00e154cceaf | `PksExtensionCallbacks::max_payload_bytes` | struct_field | Limits payload storage for `PksExtensionCallbacks`, in bytes. | `src/abi/executable_extension.rs:96` |
| sym-8423ba41181baeea0642 | `PksExtensionCallbacks::operator_process` | struct_field | Provides the operator process callback used by `PksExtensionCallbacks`. | `src/abi/executable_extension.rs:102` |
| sym-3300284912e249800c1f | `PksExtensionCallbacks::prepare` | struct_field | Provides the prepare callback used by `PksExtensionCallbacks`. | `src/abi/executable_extension.rs:100` |
| sym-523f4f07fe0b25036492 | `PksExtensionCallbacks::registration_context` | struct_field | Carries the opaque registration context used by `PksExtensionCallbacks` callbacks. | `src/abi/executable_extension.rs:95` |
| sym-34377707f885299004be | `PksExtensionCallbacks::request_stop` | struct_field | Provides the request stop callback used by `PksExtensionCallbacks`. | `src/abi/executable_extension.rs:104` |
| sym-8ab345c39de303abf923 | `PksExtensionCallbacks::reserved` | struct_field | Reserves storage for forward-compatible evolution of `PksExtensionCallbacks`. | `src/abi/executable_extension.rs:97` |
| sym-7e8d6042a0cf3e833b88 | `PksExtensionCallbacks::source_next` | struct_field | Provides the source next callback used by `PksExtensionCallbacks`. | `src/abi/executable_extension.rs:101` |
| sym-b7abfaa36ba35b10fa98 | `PksExtensionCallbacks::struct_size_bytes` | struct_field | Stores the byte size of the `PksExtensionCallbacks` ABI structure. | `src/abi/executable_extension.rs:92` |
| sym-b27888731b6a70f14b95 | `PksExtensionCallbacks::validate_configuration` | struct_field | Provides the validate configuration callback used by `PksExtensionCallbacks`. | `src/abi/executable_extension.rs:98` |
| sym-de7af8723bb618302962 | `PksExtensionDescriptor::abi_major` | struct_field | Stores the major ABI version expected by `PksExtensionDescriptor`. | `src/abi/extension.rs:49` |
| sym-fb4fceba7f10043d503e | `PksExtensionDescriptor::abi_minor` | struct_field | Stores the minor ABI version expected by `PksExtensionDescriptor`. | `src/abi/extension.rs:50` |
| sym-b09b163379b1e632ca91 | `PksExtensionDescriptor::extension_id` | struct_field | Identifies the extension identifier recorded by `PksExtensionDescriptor`. | `src/abi/extension.rs:55` |
| sym-cd76df9447a5702c0696 | `PksExtensionDescriptor::generation` | struct_field | Identifies the generation of the resource represented by `PksExtensionDescriptor`. | `src/abi/extension.rs:53` |
| sym-848be399c38fd5e3f56e | `PksExtensionDescriptor::kind` | struct_field | Records the kind selected for `PksExtensionDescriptor`. | `src/abi/extension.rs:51` |
| sym-6b86e7d040a2d2300864 | `PksExtensionDescriptor::port_count` | struct_field | Stores the number of port represented by `PksExtensionDescriptor`. | `src/abi/extension.rs:54` |
| sym-6c10774fd0745685cb27 | `PksExtensionDescriptor::revision` | struct_field | Stores the revision component of `PksExtensionDescriptor`. | `src/abi/extension.rs:52` |
| sym-cd0a07994945fa3a75b1 | `PksExtensionDescriptor::struct_size_bytes` | struct_field | Stores the byte size of the `PksExtensionDescriptor` ABI structure. | `src/abi/extension.rs:48` |
| sym-bed4ba74d429e253186a | `PksExtensionLibrary::abi_major` | struct_field | Stores the major ABI version expected by `PksExtensionLibrary`. | `src/abi/executable_extension.rs:125` |
| sym-c684b876503061de1dd2 | `PksExtensionLibrary::abi_minor` | struct_field | Stores the minor ABI version expected by `PksExtensionLibrary`. | `src/abi/executable_extension.rs:126` |
| sym-f6923e733b4102570f6a | `PksExtensionLibrary::acquire_registration` | struct_field | Provides the acquire registration callback used by `PksExtensionLibrary`. | `src/abi/executable_extension.rs:130` |
| sym-d2a462ea5971aeadf65c | `PksExtensionLibrary::library_context` | struct_field | Carries the opaque library context used by `PksExtensionLibrary` callbacks. | `src/abi/executable_extension.rs:129` |
| sym-5325c06e543fe3fb7e86 | `PksExtensionLibrary::registration_count` | struct_field | Stores the number of registration represented by `PksExtensionLibrary`. | `src/abi/executable_extension.rs:127` |
| sym-1e04b4fa1e558d0d8ec0 | `PksExtensionLibrary::reserved` | struct_field | Reserves storage for forward-compatible evolution of `PksExtensionLibrary`. | `src/abi/executable_extension.rs:128` |
| sym-633e3ddad5eae6455415 | `PksExtensionLibrary::struct_size_bytes` | struct_field | Stores the byte size of the `PksExtensionLibrary` ABI structure. | `src/abi/executable_extension.rs:124` |
| sym-492ec59461a98588d709 | `PksExtensionPipelineDeclaration::abi_major` | struct_field | Stores the major ABI version expected by `PksExtensionPipelineDeclaration`. | `src/abi/executable_extension.rs:170` |
| sym-7b594b2883fcfe1fd9b2 | `PksExtensionPipelineDeclaration::abi_minor` | struct_field | Stores the minor ABI version expected by `PksExtensionPipelineDeclaration`. | `src/abi/executable_extension.rs:171` |
| sym-290fa7d306be450d3937 | `PksExtensionPipelineDeclaration::endpoint_id` | struct_field | Identifies the endpoint identifier recorded by `PksExtensionPipelineDeclaration`. | `src/abi/executable_extension.rs:177` |
| sym-68738b1952cdefe542fd | `PksExtensionPipelineDeclaration::endpoint_input_port` | struct_field | References the endpoint input port participating in `PksExtensionPipelineDeclaration`. | `src/abi/executable_extension.rs:178` |
| sym-ecddf8d73c8e9d6284de | `PksExtensionPipelineDeclaration::operator_id` | struct_field | Identifies the operator identifier recorded by `PksExtensionPipelineDeclaration`. | `src/abi/executable_extension.rs:174` |
| sym-4c8bec614b543c654393 | `PksExtensionPipelineDeclaration::operator_input_port` | struct_field | References the operator input port participating in `PksExtensionPipelineDeclaration`. | `src/abi/executable_extension.rs:175` |
| sym-46c0b0de16ff6914228d | `PksExtensionPipelineDeclaration::operator_output_port` | struct_field | References the operator output port participating in `PksExtensionPipelineDeclaration`. | `src/abi/executable_extension.rs:176` |
| sym-dfc95dd8fbd85a2da7c2 | `PksExtensionPipelineDeclaration::source_id` | struct_field | Identifies the source identifier recorded by `PksExtensionPipelineDeclaration`. | `src/abi/executable_extension.rs:172` |
| sym-3def34d3b3e284749986 | `PksExtensionPipelineDeclaration::source_output_port` | struct_field | References the source output port participating in `PksExtensionPipelineDeclaration`. | `src/abi/executable_extension.rs:173` |
| sym-abf6494dea7fa11f40a2 | `PksExtensionPipelineDeclaration::struct_size_bytes` | struct_field | Stores the byte size of the `PksExtensionPipelineDeclaration` ABI structure. | `src/abi/executable_extension.rs:169` |
| sym-92b6de43d2bd3d825994 | `PksExtensionPort::abi_major` | struct_field | Stores the major ABI version expected by `PksExtensionPort`. | `src/abi/extension.rs:62` |
| sym-2f878cc54cb97dd7c8c0 | `PksExtensionPort::abi_minor` | struct_field | Stores the minor ABI version expected by `PksExtensionPort`. | `src/abi/extension.rs:63` |
| sym-c3e7ffa3de4c0ec33dd5 | `PksExtensionPort::direction` | struct_field | Records the direction selected for `PksExtensionPort`. | `src/abi/extension.rs:64` |
| sym-b138dd2d12d72a0caf00 | `PksExtensionPort::name` | struct_field | Stores the human-readable name used to identify `PksExtensionPort`. | `src/abi/extension.rs:66` |
| sym-818c1e80bf72e7a8b6e6 | `PksExtensionPort::required` | struct_field | Indicates whether required applies to `PksExtensionPort`. | `src/abi/extension.rs:65` |
| sym-85feaf1eeb9223172262 | `PksExtensionPort::schema` | struct_field | Records the schema selected for `PksExtensionPort`. | `src/abi/extension.rs:69` |
| sym-d45470b7ba721a6b7ae6 | `PksExtensionPort::semantic_role` | struct_field | Names the semantic role assigned to the extension port in `PksExtensionPort`. | `src/abi/extension.rs:68` |
| sym-5bcff79544e6a681652d | `PksExtensionPort::signal_id` | struct_field | Identifies the signal identifier recorded by `PksExtensionPort`. | `src/abi/extension.rs:67` |
| sym-5ee9368f2d7b3929c0fd | `PksExtensionPort::struct_size_bytes` | struct_field | Stores the byte size of the `PksExtensionPort` ABI structure. | `src/abi/extension.rs:61` |
| sym-536f459749f3b16614fe | `PksExtensionSignalBuffer::abi_major` | struct_field | Stores the major ABI version expected by `PksExtensionSignalBuffer`. | `src/abi/executable_extension.rs:155` |
| sym-d19ceb062a9973011b63 | `PksExtensionSignalBuffer::abi_minor` | struct_field | Stores the minor ABI version expected by `PksExtensionSignalBuffer`. | `src/abi/executable_extension.rs:156` |
| sym-ffa5f045303ccd040870 | `PksExtensionSignalBuffer::capacity_bytes` | struct_field | Stores the capacity size for `PksExtensionSignalBuffer`, in bytes. | `src/abi/executable_extension.rs:158` |
| sym-ccaa1c0a9bd3280ec321 | `PksExtensionSignalBuffer::data` | struct_field | Carries the data owned or referenced by `PksExtensionSignalBuffer`. | `src/abi/executable_extension.rs:157` |
| sym-f6b8f42abb75af395c08 | `PksExtensionSignalBuffer::duration_ns` | struct_field | Stores the duration value for `PksExtensionSignalBuffer`, in nanoseconds. | `src/abi/executable_extension.rs:163` |
| sym-d06178d02b904847b699 | `PksExtensionSignalBuffer::flags` | struct_field | Carries the bit flags defined by `PksExtensionSignalBuffer`. | `src/abi/executable_extension.rs:160` |
| sym-bb1d0f45d5053827a1a4 | `PksExtensionSignalBuffer::len_bytes` | struct_field | Stores the len size for `PksExtensionSignalBuffer`, in bytes. | `src/abi/executable_extension.rs:159` |
| sym-689e33ea28a222ec6a20 | `PksExtensionSignalBuffer::observed_timestamp_ns` | struct_field | Stores the observed timestamp value for `PksExtensionSignalBuffer`, in nanoseconds. | `src/abi/executable_extension.rs:161` |
| sym-7c88133aeb8bd6977839 | `PksExtensionSignalBuffer::source_timestamp_ns` | struct_field | Stores the source timestamp value for `PksExtensionSignalBuffer`, in nanoseconds. | `src/abi/executable_extension.rs:162` |
| sym-7a56c132b5667065efc9 | `PksExtensionSignalBuffer::struct_size_bytes` | struct_field | Stores the byte size of the `PksExtensionSignalBuffer` ABI structure. | `src/abi/executable_extension.rs:154` |
| sym-61507865b14f20ecaad3 | `PksExtensionSignalView::abi_major` | struct_field | Stores the major ABI version expected by `PksExtensionSignalView`. | `src/abi/executable_extension.rs:140` |
| sym-270a32533e02e0419217 | `PksExtensionSignalView::abi_minor` | struct_field | Stores the minor ABI version expected by `PksExtensionSignalView`. | `src/abi/executable_extension.rs:141` |
| sym-e9ddc46f15f2bbd137ac | `PksExtensionSignalView::data` | struct_field | Carries the data owned or referenced by `PksExtensionSignalView`. | `src/abi/executable_extension.rs:142` |
| sym-e46da0951e8cca035a04 | `PksExtensionSignalView::duration_ns` | struct_field | Stores the duration value for `PksExtensionSignalView`, in nanoseconds. | `src/abi/executable_extension.rs:147` |
| sym-3213ff660528d242f617 | `PksExtensionSignalView::flags` | struct_field | Carries the bit flags defined by `PksExtensionSignalView`. | `src/abi/executable_extension.rs:144` |
| sym-13edc82c1dcd24f80eb5 | `PksExtensionSignalView::len_bytes` | struct_field | Stores the len size for `PksExtensionSignalView`, in bytes. | `src/abi/executable_extension.rs:143` |
| sym-d71866163388ce8559ba | `PksExtensionSignalView::observed_timestamp_ns` | struct_field | Stores the observed timestamp value for `PksExtensionSignalView`, in nanoseconds. | `src/abi/executable_extension.rs:145` |
| sym-5ff4b121979a55de8c16 | `PksExtensionSignalView::sequence_number` | struct_field | Orders `PksExtensionSignalView` within its protocol or stream sequence. | `src/abi/executable_extension.rs:148` |
| sym-4b76013d0b87cdfde384 | `PksExtensionSignalView::source_timestamp_ns` | struct_field | Stores the source timestamp value for `PksExtensionSignalView`, in nanoseconds. | `src/abi/executable_extension.rs:146` |
| sym-70ccfc11e50c3418fa7a | `PksExtensionSignalView::struct_size_bytes` | struct_field | Stores the byte size of the `PksExtensionSignalView` ABI structure. | `src/abi/executable_extension.rs:139` |
| sym-81dcdf507990a523fe0e | `PksSessionStatus::code` | struct_field | Stores the code component of `PksSessionStatus`. | `src/abi/session/abi.rs:57` |
| sym-3314c210b3949d1bcd1d | `PksSessionStatus::detail` | struct_field | Stores the detail component of `PksSessionStatus`. | `src/abi/session/abi.rs:58` |
| sym-4598b1658958f9bd6a71 | `PksSessionUtf8::data` | struct_field | Carries the data owned or referenced by `PksSessionUtf8`. | `src/abi/session/abi.rs:102` |
| sym-062d1ef0071e13f96677 | `PksSessionUtf8::len_bytes` | struct_field | Stores the len size for `PksSessionUtf8`, in bytes. | `src/abi/session/abi.rs:103` |
| sym-8340b8cbc7f475ab1f6f | `PlanError::FanInOnSinglePort::node` | struct_field | References the node participating in `FanInOnSinglePort`. | `src/graph/plan.rs:23` |
| sym-58faed3ab0d0726cbc9b | `PlanError::FanInOnSinglePort::port` | struct_field | References the port participating in `FanInOnSinglePort`. | `src/graph/plan.rs:23` |
| sym-a282e08d0d17815531e8 | `PlanError::MissingEdgeContract::edge` | struct_field | References the edge participating in `MissingEdgeContract`. | `src/graph/plan.rs:27` |
| sym-aa9b27efc6764dc6a02b | `PlanError::MissingOutputSignal::edge` | struct_field | References the edge participating in `MissingOutputSignal`. | `src/graph/plan.rs:29` |
| sym-6dd3856057576b4160f0 | `PlanError::MoveExclusiveFanOut::node` | struct_field | References the node participating in `MoveExclusiveFanOut`. | `src/graph/plan.rs:25` |
| sym-38b2b892574e568c79be | `PlanError::MoveExclusiveFanOut::port` | struct_field | References the port participating in `MoveExclusiveFanOut`. | `src/graph/plan.rs:25` |
| sym-e876db7bb8f86eb3c876 | `PlanExecutionSummary::edges_attempted` | struct_field | References the edges attempted participating in `PlanExecutionSummary`. | `src/runtime/audio/executor.rs:39` |
| sym-a7c41404db85d6f31854 | `PlanExecutionSummary::edges_dropped` | struct_field | References the edges dropped participating in `PlanExecutionSummary`. | `src/runtime/audio/executor.rs:41` |
| sym-8da381f94c4546c352f8 | `PlanExecutionSummary::edges_enqueued` | struct_field | References the edges enqueued participating in `PlanExecutionSummary`. | `src/runtime/audio/executor.rs:40` |
| sym-7ed9b6cb1363dc401434 | `PlanExecutionSummary::nodes_executed` | struct_field | References the nodes executed participating in `PlanExecutionSummary`. | `src/runtime/audio/executor.rs:38` |
| sym-2969959c98a58ccab9de | `PlanRouterError::InvalidFrameBytes::bytes_per_frame` | struct_field | Stores the encoded or in-memory size of one frame for `InvalidFrameBytes`, in bytes. | `src/runtime/audio/router.rs:25` |
| sym-f10f7fe82ed0c32ca2bd | `PlanRouterError::InvalidFrameBytes::edge_id` | struct_field | Identifies the edge identifier recorded by `InvalidFrameBytes`. | `src/runtime/audio/router.rs:24` |
| sym-96479969fb6733c0608e | `PlanRouterError::MissingMemoryPlan::edge_id` | struct_field | Identifies the edge identifier recorded by `MissingMemoryPlan`. | `src/runtime/audio/router.rs:19` |
| sym-0b01a53a37e34706abd6 | `PlanRouterError::ZeroCapacity::edge_id` | struct_field | Identifies the edge identifier recorded by `ZeroCapacity`. | `src/runtime/audio/router.rs:21` |
| sym-592bd103f2ff0f088871 | `PlanRunnerError::DuplicateSource::source_node_id` | struct_field | Identifies the source node identifier recorded by `DuplicateSource`. | `src/runtime/audio/runner.rs:260` |
| sym-cbce35b9cdff18103db7 | `PlanRunnerError::ZeroSourceCapacity::source_node_id` | struct_field | Identifies the source node identifier recorded by `ZeroSourceCapacity`. | `src/runtime/audio/runner.rs:258` |
| sym-902745b5d98177332fa5 | `PlanRunnerFinishSummary::drain_budget_exhausted` | struct_field | Reports whether drain budget exhausted is true for `PlanRunnerFinishSummary`. | `src/runtime/audio/runner.rs:301` |
| sym-62c9b96bf2d1cac9e06c | `PlanRunnerFinishSummary::execution` | struct_field | Records the execution selected for `PlanRunnerFinishSummary`. | `src/runtime/audio/runner.rs:302` |
| sym-fa5bd93849428b134782 | `PlanRunnerFinishSummary::source_frames_discarded_total` | struct_field | Counts the total number of source frames discarded observed by `PlanRunnerFinishSummary`. | `src/runtime/audio/runner.rs:300` |
| sym-b69dfc6b2f92ce36c94b | `PlanRunnerFinishSummary::source_frames_processed_total` | struct_field | Counts the total number of source frames processed observed by `PlanRunnerFinishSummary`. | `src/runtime/audio/runner.rs:299` |
| sym-9232b264677c1c4c847a | `PlanRunnerStepSummary::execution` | struct_field | Records the execution selected for `PlanRunnerStepSummary`. | `src/runtime/audio/runner.rs:272` |
| sym-acbecf488b1ed74aeae2 | `PlanRunnerStepSummary::source_frames_processed_total` | struct_field | Counts the total number of source frames processed observed by `PlanRunnerStepSummary`. | `src/runtime/audio/runner.rs:271` |
| sym-c88f3485be88351e707c | `PlanSourceInputObservations::frames_delivered_total` | struct_field | Counts the total number of frames delivered observed by `PlanSourceInputObservations`. | `src/runtime/audio/runner.rs:27` |
| sym-46c22f69d807bb2b7ace | `PlanSourceInputObservations::frames_discarded_total` | struct_field | Counts the total number of frames discarded observed by `PlanSourceInputObservations`. | `src/runtime/audio/runner.rs:30` |
| sym-e81062b7d02cc222e61e | `PlanSourceInputObservations::frames_enqueued_total` | struct_field | Counts the total number of frames enqueued observed by `PlanSourceInputObservations`. | `src/runtime/audio/runner.rs:26` |
| sym-dee4dbbcda8a68e4376f | `PlanSourceInputObservations::frames_rejected_cancelled_total` | struct_field | Counts the total number of frames rejected cancelled observed by `PlanSourceInputObservations`. | `src/runtime/audio/runner.rs:29` |
| sym-811f79c63204908fd99c | `PlanSourceInputObservations::frames_rejected_full_total` | struct_field | Counts the total number of frames rejected full observed by `PlanSourceInputObservations`. | `src/runtime/audio/runner.rs:28` |
| sym-2d933dafca507c315208 | `PlanSourceInputObservations::queue_capacity_frames` | struct_field | Sets the queue capacity frames available to `PlanSourceInputObservations`. | `src/runtime/audio/runner.rs:23` |
| sym-d7def031ad56c8f924b8 | `PlanSourceInputObservations::queue_depth_frames` | struct_field | Reports the queue depth frames observed by `PlanSourceInputObservations`. | `src/runtime/audio/runner.rs:24` |
| sym-fe9d698adbb3e209fae8 | `PlanSourceInputObservations::queue_peak_frames` | struct_field | Reports the queue peak frames observed by `PlanSourceInputObservations`. | `src/runtime/audio/runner.rs:25` |
| sym-f0540ac36d6684c26146 | `PlanSourceSendOutcome::Rejected::error` | struct_field | Stores the error as a `PlanSourceSendError` value in `Rejected`. | `src/runtime/audio/runner.rs:126` |
| sym-e13e9b16fb1da76ce6c7 | `PlanSourceSendOutcome::Rejected::frame` | struct_field | Stores the frame as a `LineagedAudioFrame` value in `Rejected`. | `src/runtime/audio/runner.rs:127` |
| sym-8c85ed728f6ca1126122 | `PolledAudioEndpointConfig::max_batch_frames` | struct_field | Contains the max batch frames owned or reported by `PolledAudioEndpointConfig`. | `src/endpoint/polled_audio_driver.rs:25` |
| sym-1aec62a1a8a6db977ea2 | `PolledAudioEndpointConfig::max_outstanding_leases` | struct_field | Contains the max outstanding leases owned or reported by `PolledAudioEndpointConfig`. | `src/endpoint/polled_audio_driver.rs:26` |
| sym-ddb97ef8c35f40eb3ce0 | `PolledAudioEndpointConfig::queue_capacity_frames` | struct_field | Sets the queue capacity frames available to `PolledAudioEndpointConfig`. | `src/endpoint/polled_audio_driver.rs:24` |
| sym-f6373c492e3f40c10afe | `PolledAudioObservations::batches_polled_total` | struct_field | Counts the total number of batches polled observed by `PolledAudioObservations`. | `src/endpoint/polled_audio_driver.rs:69` |
| sym-3d3c6f2c81509b6e7f8e | `PolledAudioObservations::frames_delivered_total` | struct_field | Counts the total number of frames delivered observed by `PolledAudioObservations`. | `src/endpoint/polled_audio_driver.rs:63` |
| sym-c32ee63eeee4beefe418 | `PolledAudioObservations::frames_polled_total` | struct_field | Counts the total number of frames polled observed by `PolledAudioObservations`. | `src/endpoint/polled_audio_driver.rs:70` |
| sym-f6787b0d50c3053ccc88 | `PolledAudioObservations::frames_received_total` | struct_field | Counts the total number of frames received observed by `PolledAudioObservations`. | `src/endpoint/polled_audio_driver.rs:62` |
| sym-251f5c4e9b5c32858d01 | `PolledAudioObservations::invalid_ownership_drops_total` | struct_field | Counts the total number of invalid ownership drops observed by `PolledAudioObservations`. | `src/endpoint/polled_audio_driver.rs:65` |
| sym-5b27eb7e661114bf1de2 | `PolledAudioObservations::lease_capacity_count` | struct_field | Sets the lease capacity count available to `PolledAudioObservations`. | `src/endpoint/polled_audio_driver.rs:66` |
| sym-db46e2482b9f0de59d9b | `PolledAudioObservations::lease_exhausted_total` | struct_field | Counts the total number of lease exhausted observed by `PolledAudioObservations`. | `src/endpoint/polled_audio_driver.rs:68` |
| sym-60e6d205dc4532a6886d | `PolledAudioObservations::outstanding_leases` | struct_field | Contains the outstanding leases owned or reported by `PolledAudioObservations`. | `src/endpoint/polled_audio_driver.rs:67` |
| sym-78b34b072e453fa9edf1 | `PolledAudioObservations::queue_capacity_frames` | struct_field | Sets the queue capacity frames available to `PolledAudioObservations`. | `src/endpoint/polled_audio_driver.rs:58` |
| sym-c2724347551ef46ad6d3 | `PolledAudioObservations::queue_depth_frames` | struct_field | Reports the queue depth frames observed by `PolledAudioObservations`. | `src/endpoint/polled_audio_driver.rs:59` |
| sym-f6fb9f6920263449e1e2 | `PolledAudioObservations::queue_depth_invariant_failures_total` | struct_field | Counts the total number of queue depth invariant failures observed by `PolledAudioObservations`. | `src/endpoint/polled_audio_driver.rs:61` |
| sym-55dad236cdb33cb2ee6a | `PolledAudioObservations::queue_full_drops_total` | struct_field | Counts the total number of queue full drops observed by `PolledAudioObservations`. | `src/endpoint/polled_audio_driver.rs:64` |
| sym-265935d3610a950b210e | `PolledAudioObservations::queue_peak_frames` | struct_field | Reports the queue peak frames observed by `PolledAudioObservations`. | `src/endpoint/polled_audio_driver.rs:60` |
| sym-cc5b4708a4dbd8bac663 | `PolledAudioObservations::registered_endpoints` | struct_field | References the registered endpoints participating in `PolledAudioObservations`. | `src/endpoint/polled_audio_driver.rs:57` |
| sym-5d84ff39c8347d078ae1 | `PrepareContext::sample_spec` | struct_field | Declares the sample rate, channel layout, and format used by `PrepareContext`. | `src/graph/node.rs:267` |
| sym-fbdd84800ad4ca934c0c | `RecorderError::FrameSpecMismatch::actual_channels` | struct_field | Contains the actual channels owned or reported by `FrameSpecMismatch`. | `src/recording/writer.rs:62` |
| sym-c16279fac7c130087a6e | `RecorderError::FrameSpecMismatch::actual_rate_hz` | struct_field | Stores the actual rate value for `FrameSpecMismatch`, in hertz. | `src/recording/writer.rs:61` |
| sym-24a41ce2d0d004086cda | `RecorderError::FrameSpecMismatch::expected_channels` | struct_field | Contains the expected channels owned or reported by `FrameSpecMismatch`. | `src/recording/writer.rs:64` |
| sym-fb560a6c63bda0736640 | `RecorderError::FrameSpecMismatch::expected_rate_hz` | struct_field | Stores the expected rate value for `FrameSpecMismatch`, in hertz. | `src/recording/writer.rs:63` |
| sym-d37cbb3ea82b89c7a330 | `RecorderError::FrameSpecMismatch::label` | struct_field | Stores the human-readable label used to identify `FrameSpecMismatch`. | `src/recording/writer.rs:60` |
| sym-d26b3ce2fc6f17d13c54 | `RecorderError::GapTooLarge::duration_ns` | struct_field | Stores the duration value for `GapTooLarge`, in nanoseconds. | `src/recording/writer.rs:71` |
| sym-905e952e31087239c1b8 | `RecorderError::GapTooLarge::label` | struct_field | Stores the human-readable label used to identify `GapTooLarge`. | `src/recording/writer.rs:71` |
| sym-242f9b584650610f7625 | `RecorderError::InvalidSampleSpec::channels` | struct_field | Contains the channels owned or reported by `InvalidSampleSpec`. | `src/recording/writer.rs:43` |
| sym-966feac0aca6a44f4a90 | `RecorderError::InvalidSampleSpec::label` | struct_field | Stores the human-readable label used to identify `InvalidSampleSpec`. | `src/recording/writer.rs:41` |
| sym-9c7f05d988547182c5ea | `RecorderError::InvalidSampleSpec::sample_rate_hz` | struct_field | Stores the sample rate value for `InvalidSampleSpec`, in hertz. | `src/recording/writer.rs:42` |
| sym-7aa58db63062f1afbac9 | `RecorderError::LineageMismatch::actual` | struct_field | Records the value observed by `LineageMismatch`. | `src/recording/writer.rs:55` |
| sym-ef2eb2cc10fd85fcb60b | `RecorderError::LineageMismatch::expected` | struct_field | Records the value expected by `LineageMismatch`. | `src/recording/writer.rs:56` |
| sym-b566e4bd36366e72f99d | `RecorderError::LineageMismatch::field` | struct_field | Stores the field as a `RecorderLineageField` value in `LineageMismatch`. | `src/recording/writer.rs:54` |
| sym-9a002a0640cd66f9ecf9 | `RecorderError::LineageMismatch::label` | struct_field | Stores the human-readable label used to identify `LineageMismatch`. | `src/recording/writer.rs:53` |
| sym-2138f7f644c4d524bec5 | `RecorderError::SessionMismatch::actual` | struct_field | Records the value observed by `SessionMismatch`. | `src/recording/writer.rs:34` |
| sym-1d3fb51a9707e3f036b0 | `RecorderError::SessionMismatch::expected` | struct_field | Records the value expected by `SessionMismatch`. | `src/recording/writer.rs:35` |
| sym-9dd80dcbf6d6283756aa | `RecorderError::SessionMismatch::label` | struct_field | Stores the human-readable label used to identify `SessionMismatch`. | `src/recording/writer.rs:33` |
| sym-318026910b3b8a9c1215 | `RecorderError::SourceMismatch::actual` | struct_field | Records the value observed by `SourceMismatch`. | `src/recording/writer.rs:48` |
| sym-dc91ced665ac8ef2e2a7 | `RecorderError::SourceMismatch::expected` | struct_field | Records the value expected by `SourceMismatch`. | `src/recording/writer.rs:49` |
| sym-9c7ebccea34099688080 | `RecorderError::SourceMismatch::label` | struct_field | Stores the human-readable label used to identify `SourceMismatch`. | `src/recording/writer.rs:47` |
| sym-22d19ddffe19d25a86ed | `RecorderStemConfig::channels` | struct_field | Contains the channels owned or reported by `RecorderStemConfig`. | `src/recording/config.rs:66` |
| sym-f38130c2fad8fd98a9ff | `RecorderStemConfig::clock_id` | struct_field | Identifies the clock identifier recorded by `RecorderStemConfig`. | `src/recording/config.rs:59` |
| sym-8760034a6327e4361a3e | `RecorderStemConfig::label` | struct_field | Stores the human-readable label used to identify `RecorderStemConfig`. | `src/recording/config.rs:64` |
| sym-b4b869b508c00922bfa1 | `RecorderStemConfig::permission` | struct_field | Stores the permission as a `PermissionDecision` value in `RecorderStemConfig`. | `src/recording/config.rs:63` |
| sym-a407e04d6cef589b4df2 | `RecorderStemConfig::permission_epoch` | struct_field | Identifies the permission-observation generation attached to `RecorderStemConfig`. | `src/recording/config.rs:61` |
| sym-9c5d49f737f979fed8c0 | `RecorderStemConfig::permission_scope` | struct_field | Stores the permission scope as a `PermissionScope` value in `RecorderStemConfig`. | `src/recording/config.rs:62` |
| sym-9e4c78bd164b555cfe3b | `RecorderStemConfig::sample_rate_hz` | struct_field | Stores the sample rate value for `RecorderStemConfig`, in hertz. | `src/recording/config.rs:65` |
| sym-dc1ff0dc0cf84d71bf8a | `RecorderStemConfig::session_id` | struct_field | Identifies the session identifier recorded by `RecorderStemConfig`. | `src/recording/config.rs:56` |
| sym-89360da39a6ddf56eb2c | `RecorderStemConfig::source_generation` | struct_field | References the source generation participating in `RecorderStemConfig`. | `src/recording/config.rs:60` |
| sym-37c313abbccb5717ac05 | `RecorderStemConfig::source_id` | struct_field | Identifies the source identifier recorded by `RecorderStemConfig`. | `src/recording/config.rs:57` |
| sym-f1110ae1a4b69575d2b1 | `RecorderStemConfig::stem_id` | struct_field | Identifies the stem identifier recorded by `RecorderStemConfig`. | `src/recording/config.rs:58` |
| sym-b5fecfbb0e3e03b133f4 | `RecorderStemConfig::timeline_mapping` | struct_field | Maps source timestamps into the Session timeline for `RecorderStemConfig`. | `src/recording/config.rs:67` |
| sym-94733d80b8d4c4ee69cb | `RecordingObservations::discontinuities_total` | struct_field | Counts the total number of discontinuities observed by `RecordingObservations`. | `src/recording/writer.rs:135` |
| sym-13d19ab6eab7887174b5 | `RecordingObservations::failures_total` | struct_field | Counts the total number of failures observed by `RecordingObservations`. | `src/recording/writer.rs:136` |
| sym-4d09259616c8e924d325 | `RecordingObservations::frames_received_total` | struct_field | Counts the total number of frames received observed by `RecordingObservations`. | `src/recording/writer.rs:132` |
| sym-97cbb27eb143381a12cb | `RecordingObservations::frames_rejected_total` | struct_field | Counts the total number of frames rejected observed by `RecordingObservations`. | `src/recording/writer.rs:134` |
| sym-323c023545b92c242e9b | `RecordingObservations::frames_written_total` | struct_field | Counts the total number of frames written observed by `RecordingObservations`. | `src/recording/writer.rs:133` |
| sym-3e25b88341cc7c8a704c | `RecordingOutcome::completed_stems` | struct_field | Contains the completed stems owned or reported by `RecordingOutcome`. | `src/recording/writer.rs:115` |
| sym-42d3fade12a4b11815aa | `RecordingOutcome::failed_stems` | struct_field | Contains the failed stems owned or reported by `RecordingOutcome`. | `src/recording/writer.rs:116` |
| sym-e0e6d9a62d466f6760b1 | `RecordingOutcome::session_dir` | struct_field | Points to the directory containing the Session recording represented by `RecordingOutcome`. | `src/recording/writer.rs:113` |
| sym-506ed4409ed08fb26990 | `RecordingOutcome::state` | struct_field | Records the state selected for `RecordingOutcome`. | `src/recording/writer.rs:114` |
| sym-67c3734e45702552c46d | `RecordingOutcome::stems` | struct_field | Contains the stems owned or reported by `RecordingOutcome`. | `src/recording/writer.rs:117` |
| sym-e13b68e70fe9c0649e88 | `RecordingStemOutcome::edge_observations` | struct_field | References the edge observations participating in `RecordingStemOutcome`. | `src/recording/writer.rs:127` |
| sym-72a89b0a596b23cb5c48 | `RecordingStemOutcome::error` | struct_field | Stores the error component of `RecordingStemOutcome`. | `src/recording/writer.rs:126` |
| sym-8fa3dd94d66a46911497 | `RecordingStemOutcome::gap_ranges` | struct_field | Contains the gap ranges owned or reported by `RecordingStemOutcome`. | `src/recording/writer.rs:125` |
| sym-4f2de8c2b3babdf85240 | `RecordingStemOutcome::label` | struct_field | Stores the human-readable label used to identify `RecordingStemOutcome`. | `src/recording/writer.rs:122` |
| sym-ca4b86eadd092aeb2c5b | `RecordingStemOutcome::stale_frames` | struct_field | Contains the stale frames owned or reported by `RecordingStemOutcome`. | `src/recording/writer.rs:124` |
| sym-60f07bcfb21e28c95634 | `RecordingStemOutcome::written_frames` | struct_field | Contains the written frames owned or reported by `RecordingStemOutcome`. | `src/recording/writer.rs:123` |
| sym-1819dcb0cee424f54655 | `ResolvedEdge::contract` | struct_field | Stores the contract component of `ResolvedEdge`. | `src/graph/ir.rs:28` |
| sym-9ee8d911997630c9620c | `ResolvedEdge::media` | struct_field | Records the media selected for `ResolvedEdge`. | `src/graph/ir.rs:27` |
| sym-5fbba85205a30c7a0eb4 | `ResolvedEdge::spec` | struct_field | Stores the spec as a `EdgeSpec` value in `ResolvedEdge`. | `src/graph/ir.rs:26` |
| sym-ded5db90eb26226804a4 | `ResolvedNode::descriptor` | struct_field | Stores the descriptor as a `NodeDescriptor` value in `ResolvedNode`. | `src/graph/ir.rs:12` |
| sym-dd97758e969373a49191 | `ResolvedNode::spec` | struct_field | Stores the spec as a `NodeSpec` value in `ResolvedNode`. | `src/graph/ir.rs:11` |
| sym-f75a1f92eee85e63c8fb | `RuntimePlan::edge_count` | struct_field | Stores the number of edge represented by `RuntimePlan`. | `src/graph/plan.rs:130` |
| sym-13ef1315b430a3111800 | `RuntimePlan::edge_metrics` | struct_field | References the edge metrics participating in `RuntimePlan`. | `src/graph/plan.rs:125` |
| sym-baf7841f739d2bd90a2e | `RuntimePlan::fan_in` | struct_field | Lists compiled edge groups that converge on one input in `RuntimePlan`. | `src/graph/plan.rs:127` |
| sym-26408ec8e3f24444eda7 | `RuntimePlan::fan_out` | struct_field | Lists compiled edge groups that branch from one output in `RuntimePlan`. | `src/graph/plan.rs:126` |
| sym-68cdfd8dce6567eaad3b | `RuntimePlan::memory_plan` | struct_field | Carries the bounded buffer and allocation plan compiled into `RuntimePlan`. | `src/graph/plan.rs:124` |
| sym-f9279ee6d77adee39430 | `RuntimePlan::node_order` | struct_field | References the node order participating in `RuntimePlan`. | `src/graph/plan.rs:122` |
| sym-39a389fa883430d8a63e | `RuntimePlan::partitions` | struct_field | Contains the partitions owned or reported by `RuntimePlan`. | `src/graph/plan.rs:123` |
| sym-aa5eba322dd84bf55912 | `RuntimePlan::source_outputs` | struct_field | References the source outputs participating in `RuntimePlan`. | `src/graph/plan.rs:129` |
| sym-1d11b612c173d52450d0 | `RuntimePlan::typed_edges` | struct_field | References the typed edges participating in `RuntimePlan`. | `src/graph/plan.rs:128` |
| sym-a4970e2f3925c0b44225 | `SessionCompileError::AmbiguousEndpointInput::input_ports_total` | struct_field | Counts the total number of input ports observed by `AmbiguousEndpointInput`. | `src/session/compile/error.rs:29` |
| sym-db63ed3cb1ee7469ce56 | `SessionCompileError::AmbiguousEndpointInput::node_type_id` | struct_field | Identifies the node type identifier recorded by `AmbiguousEndpointInput`. | `src/session/compile/error.rs:28` |
| sym-4837c186d5ae2e0c3cfd | `SessionCompileError::AmbiguousOperatorPort::direction` | struct_field | Records the direction selected for `AmbiguousOperatorPort`. | `src/session/compile/error.rs:34` |
| sym-a0fb3db6f044a9e2106a | `SessionCompileError::AmbiguousOperatorPort::operator_id` | struct_field | Identifies the operator identifier recorded by `AmbiguousOperatorPort`. | `src/session/compile/error.rs:33` |
| sym-0a6e86f4eb25ffe43b55 | `SessionCompileError::AudioBridgeOutputNotExclusive::operator_instance_id` | struct_field | Identifies the operator instance identifier recorded by `AudioBridgeOutputNotExclusive`. | `src/session/compile/error.rs:58` |
| sym-df9dbdb6cdb7f5809390 | `SessionCompileError::AudioBridgeOutputNotExclusive::output_port` | struct_field | References the output port participating in `AudioBridgeOutputNotExclusive`. | `src/session/compile/error.rs:59` |
| sym-4097ae04218ce6d6944f | `SessionCompileError::DuplicateOperatorInputConnection::operator_instance_id` | struct_field | Identifies the operator instance identifier recorded by `DuplicateOperatorInputConnection`. | `src/session/compile/error.rs:63` |
| sym-f39d08542a7d5bee8a60 | `SessionCompileError::DuplicateOperatorInputConnection::port_name` | struct_field | Stores the human-readable port used to identify `DuplicateOperatorInputConnection`. | `src/session/compile/error.rs:64` |
| sym-8c79aea03ab39082ab35 | `SessionCompileError::InvalidAudioBridgeOutput::operator_instance_id` | struct_field | Identifies the operator instance identifier recorded by `InvalidAudioBridgeOutput`. | `src/session/compile/error.rs:51` |
| sym-3c9c767a48369d34e469 | `SessionCompileError::InvalidAudioBridgeOutput::output_port` | struct_field | References the output port participating in `InvalidAudioBridgeOutput`. | `src/session/compile/error.rs:52` |
| sym-91e49c4f5da7dbbfd5ee | `SessionCompileError::InvalidExternalSourceConfiguration::reason` | struct_field | Carries the reason reported by `InvalidExternalSourceConfiguration`. | `src/session/compile/error.rs:80` |
| sym-3a2a3930b9f25405739c | `SessionCompileError::InvalidExternalSourceConfiguration::source_type_id` | struct_field | Identifies the source type identifier recorded by `InvalidExternalSourceConfiguration`. | `src/session/compile/error.rs:79` |
| sym-724f0616f195271c17bd | `SessionCompileError::MissingRequiredOperatorInput::operator_instance_id` | struct_field | Identifies the operator instance identifier recorded by `MissingRequiredOperatorInput`. | `src/session/compile/error.rs:44` |
| sym-d7399e2258c51f035b38 | `SessionCompileError::MissingRequiredOperatorInput::port_name` | struct_field | Stores the human-readable port used to identify `MissingRequiredOperatorInput`. | `src/session/compile/error.rs:45` |
| sym-6f7ec8489f22beeeda1b | `SessionCompileError::OperatorNodeTypeMismatch::declared_node_type_id` | struct_field | Identifies the declared node type identifier recorded by `OperatorNodeTypeMismatch`. | `src/session/compile/error.rs:18` |
| sym-5188e6e15f8910b9a7eb | `SessionCompileError::OperatorNodeTypeMismatch::operator_id` | struct_field | Identifies the operator identifier recorded by `OperatorNodeTypeMismatch`. | `src/session/compile/error.rs:16` |
| sym-32109fa26431f26c845a | `SessionCompileError::OperatorNodeTypeMismatch::registered_node_type_id` | struct_field | Identifies the registered node type identifier recorded by `OperatorNodeTypeMismatch`. | `src/session/compile/error.rs:17` |
| sym-88734a7b82f9dd563216 | `SessionCompileError::UnknownAsyncOperator::operator_id` | struct_field | Identifies the operator identifier recorded by `UnknownAsyncOperator`. | `src/session/compile/error.rs:21` |
| sym-bc876b76ea71e3827164 | `SessionCompileError::UnknownEndpointInputPort::node_type_id` | struct_field | Identifies the node type identifier recorded by `UnknownEndpointInputPort`. | `src/session/compile/error.rs:84` |
| sym-6123462823e025fb0c83 | `SessionCompileError::UnknownEndpointInputPort::port_name` | struct_field | Stores the human-readable port used to identify `UnknownEndpointInputPort`. | `src/session/compile/error.rs:85` |
| sym-c8cc512937867ec77970 | `SessionCompileError::UnknownEndpointNodeType::node_type_id` | struct_field | Identifies the node type identifier recorded by `UnknownEndpointNodeType`. | `src/session/compile/error.rs:23` |
| sym-7b67102491eb6267b22d | `SessionCompileError::UnknownExternalSource::source_type_id` | struct_field | Identifies the source type identifier recorded by `UnknownExternalSource`. | `src/session/compile/error.rs:69` |
| sym-3bf527b942b742692dbd | `SessionCompileError::UnknownExternalSourceOutput::output_port` | struct_field | References the output port participating in `UnknownExternalSourceOutput`. | `src/session/compile/error.rs:75` |
| sym-9e4a936ebfbfd41715c1 | `SessionCompileError::UnknownExternalSourceOutput::source_type_id` | struct_field | Identifies the source type identifier recorded by `UnknownExternalSourceOutput`. | `src/session/compile/error.rs:74` |
| sym-fe19db63157be9d896e2 | `SessionCompileError::UnknownOperator::operator_id` | struct_field | Identifies the operator identifier recorded by `UnknownOperator`. | `src/session/compile/error.rs:11` |
| sym-3865e127dbd3a2c19d52 | `SessionCompileError::UnknownOperatorPort::direction` | struct_field | Records the direction selected for `UnknownOperatorPort`. | `src/session/compile/error.rs:39` |
| sym-b62371e58130a50741d1 | `SessionCompileError::UnknownOperatorPort::operator_id` | struct_field | Identifies the operator identifier recorded by `UnknownOperatorPort`. | `src/session/compile/error.rs:38` |
| sym-07a6c1bcdf8cd28877ae | `SessionCompileError::UnknownOperatorPort::port_name` | struct_field | Stores the human-readable port used to identify `UnknownOperatorPort`. | `src/session/compile/error.rs:40` |
| sym-a4b5e9d94cd79a29ec2b | `SessionCompileError::UnknownSourceNodeType::node_type_id` | struct_field | Identifies the node type identifier recorded by `UnknownSourceNodeType`. | `src/session/compile/error.rs:67` |
| sym-d5649dda3aee5c4c9c78 | `SessionComponentId::Endpoint::endpoint_id` | struct_field | Identifies the endpoint identifier recorded by `Endpoint`. | `src/session/lifecycle/events.rs:57` |
| sym-2578ef735cc60fc3070c | `SessionComponentId::Endpoint::route_id` | struct_field | Identifies the route identifier recorded by `Endpoint`. | `src/session/lifecycle/events.rs:56` |
| sym-69667c72c2f7f8cf97c5 | `SessionComponentId::Operator::operator_instance_id` | struct_field | Identifies the operator instance identifier recorded by `Operator`. | `src/session/lifecycle/events.rs:60` |
| sym-c548e3a595359a49d0dd | `SessionComponentId::Sidecar::sidecar_id` | struct_field | Identifies the sidecar identifier recorded by `Sidecar`. | `src/session/lifecycle/events.rs:63` |
| sym-d2c36182beacc662fdbe | `SessionComponentId::Source::stem_id` | struct_field | Identifies the stem identifier recorded by `Source`. | `src/session/lifecycle/events.rs:53` |
| sym-5410c4bc8603e1edb7d3 | `SessionDerivedRouteMetrics::endpoint` | struct_field | References the endpoint participating in `SessionDerivedRouteMetrics`. | `src/session/lifecycle/observations.rs:435` |
| sym-db4e841c60fe5081a974 | `SessionDerivedRouteMetrics::endpoint_finalization_failures_total` | struct_field | Counts the total number of endpoint finalization failures observed by `SessionDerivedRouteMetrics`. | `src/session/lifecycle/observations.rs:437` |
| sym-8386624ae176cbb2da33 | `SessionDerivedRouteMetrics::endpoint_id` | struct_field | Identifies the endpoint identifier recorded by `SessionDerivedRouteMetrics`. | `src/session/lifecycle/observations.rs:433` |
| sym-e75246090d12d6a92fa1 | `SessionDerivedRouteMetrics::endpoint_observation_stage` | struct_field | References the endpoint observation stage participating in `SessionDerivedRouteMetrics`. | `src/session/lifecycle/observations.rs:436` |
| sym-0ada49279afad4e102ef | `SessionDerivedRouteMetrics::output` | struct_field | Carries the output produced by `SessionDerivedRouteMetrics`. | `src/session/lifecycle/observations.rs:434` |
| sym-5e15e87b038d1ae8069a | `SessionDerivedRouteMetrics::route_id` | struct_field | Identifies the route identifier recorded by `SessionDerivedRouteMetrics`. | `src/session/lifecycle/observations.rs:432` |
| sym-a2c538e548a296402b3b | `SessionEngineBuildError::DuplicateSidecarId::sidecar_id` | struct_field | Identifies the sidecar identifier recorded by `DuplicateSidecarId`. | `src/session/lifecycle/engine.rs:301` |
| sym-65236b3bc3af825baf2a | `SessionEngineBuildError::InvalidConfiguration::reason` | struct_field | Carries the reason reported by `InvalidConfiguration`. | `src/session/lifecycle/engine.rs:299` |
| sym-40ace7ffe6c9b22e0368 | `SessionError::DraftFrozen::session_id` | struct_field | Identifies the session identifier recorded by `DraftFrozen`. | `src/session/error.rs:36` |
| sym-f2d78d18fc26d6842f1b | `SessionError::ForeignEndpoint::actual` | struct_field | Records the value observed by `ForeignEndpoint`. | `src/session/error.rs:33` |
| sym-c6af79ff62aaec2ea67d | `SessionError::ForeignEndpoint::expected` | struct_field | Records the value expected by `ForeignEndpoint`. | `src/session/error.rs:32` |
| sym-e647809ed474f1ebc82e | `SessionError::InvalidEndpoint::reason` | struct_field | Carries the reason reported by `InvalidEndpoint`. | `src/session/error.rs:25` |
| sym-89573f8b53eb4c95121d | `SessionError::InvalidOperator::reason` | struct_field | Carries the reason reported by `InvalidOperator`. | `src/session/error.rs:27` |
| sym-c177d4850e6b5fd9551d | `SessionError::InvalidRoute::reason` | struct_field | Carries the reason reported by `InvalidRoute`. | `src/session/error.rs:29` |
| sym-bf0765cead7c70e96f21 | `SessionError::InvalidSelector::reason` | struct_field | Carries the reason reported by `InvalidSelector`. | `src/session/error.rs:23` |
| sym-150a640d6b399c133e14 | `SessionError::NoRoutes::stem_id` | struct_field | Identifies the stem identifier recorded by `NoRoutes`. | `src/session/error.rs:10` |
| sym-95257e0e51cc42b5ebd0 | `SessionError::NoSourceOutputRoutes::output_port` | struct_field | References the output port participating in `NoSourceOutputRoutes`. | `src/session/error.rs:20` |
| sym-784e0d73c28ca48214fc | `SessionError::NoSourceOutputRoutes::source_instance_id` | struct_field | Identifies the source instance identifier recorded by `NoSourceOutputRoutes`. | `src/session/error.rs:19` |
| sym-86927a593e6e35c7337f | `SessionError::NoSourceOutputs::source_instance_id` | struct_field | Identifies the source instance identifier recorded by `NoSourceOutputs`. | `src/session/error.rs:13` |
| sym-6e1296e526c4d797f8c9 | `SessionError::OperatorHasNoDestination::operator_instance_id` | struct_field | Identifies the operator instance identifier recorded by `OperatorHasNoDestination`. | `src/session/error.rs:64` |
| sym-5c0c964f9b4947e29111 | `SessionError::UnknownEndpoint::endpoint_id` | struct_field | Identifies the endpoint identifier recorded by `UnknownEndpoint`. | `src/session/error.rs:44` |
| sym-6be2236ace0da67fc819 | `SessionError::UnknownOperatorInstance::operator_instance_id` | struct_field | Identifies the operator instance identifier recorded by `UnknownOperatorInstance`. | `src/session/error.rs:60` |
| sym-293f305cc174738dda19 | `SessionError::UnknownSourceInstance::source_instance_id` | struct_field | Identifies the source instance identifier recorded by `UnknownSourceInstance`. | `src/session/error.rs:49` |
| sym-620c89b033b154a65e52 | `SessionError::UnknownSourceOutput::output_port` | struct_field | References the output port participating in `UnknownSourceOutput`. | `src/session/error.rs:56` |
| sym-9dcac08fb2dd6072c95b | `SessionError::UnknownSourceOutput::source_instance_id` | struct_field | Identifies the source instance identifier recorded by `UnknownSourceOutput`. | `src/session/error.rs:55` |
| sym-e40b74230c3154310948 | `SessionError::UnknownStem::stem_id` | struct_field | Identifies the stem identifier recorded by `UnknownStem`. | `src/session/error.rs:46` |
| sym-d94ee887a9f4f1b0e947 | `SessionError::UnsupportedVersion::major` | struct_field | Stores the major component of `UnsupportedVersion`. | `src/session/error.rs:42` |
| sym-25d49dbd88a311b1e6f1 | `SessionError::UnsupportedVersion::minor` | struct_field | Stores the minor component of `UnsupportedVersion`. | `src/session/error.rs:42` |
| sym-cd4510aa48f70f7da3a8 | `SessionEventQueueObservations::capacity_event_count` | struct_field | Sets the capacity event count available to `SessionEventQueueObservations`. | `src/session/lifecycle/observations.rs:18` |
| sym-a8b2a7d1d98e0f02dfac | `SessionEventQueueObservations::depth_events` | struct_field | Reports the depth events observed by `SessionEventQueueObservations`. | `src/session/lifecycle/observations.rs:21` |
| sym-b85611c85ba7dcd38a6a | `SessionEventQueueObservations::depth_owned_bytes` | struct_field | Stores the depth owned size for `SessionEventQueueObservations`, in bytes. | `src/session/lifecycle/observations.rs:22` |
| sym-5cb79ba6a9931d4385f9 | `SessionEventQueueObservations::events_dropped_oversized_total` | struct_field | Counts the total number of events dropped oversized observed by `SessionEventQueueObservations`. | `src/session/lifecycle/observations.rs:27` |
| sym-cd0860992c1d309e1801 | `SessionEventQueueObservations::events_dropped_total` | struct_field | Counts the total number of events dropped observed by `SessionEventQueueObservations`. | `src/session/lifecycle/observations.rs:26` |
| sym-a01c2b0bc3e1762439b3 | `SessionEventQueueObservations::events_enqueued_total` | struct_field | Counts the total number of events enqueued observed by `SessionEventQueueObservations`. | `src/session/lifecycle/observations.rs:25` |
| sym-917f58fce4212e6615f9 | `SessionEventQueueObservations::maximum_buffered_owned_bytes` | struct_field | Stores the maximum buffered owned size for `SessionEventQueueObservations`, in bytes. | `src/session/lifecycle/observations.rs:20` |
| sym-d7af7cc493ab999ccdb9 | `SessionEventQueueObservations::maximum_event_owned_bytes` | struct_field | Stores the maximum event owned size for `SessionEventQueueObservations`, in bytes. | `src/session/lifecycle/observations.rs:19` |
| sym-ccbb76292451b274efef | `SessionEventQueueObservations::peak_depth_event_count` | struct_field | Reports the peak depth event count observed by `SessionEventQueueObservations`. | `src/session/lifecycle/observations.rs:23` |
| sym-4b575c00a7f8de60d20b | `SessionEventQueueObservations::peak_depth_owned_bytes` | struct_field | Stores the peak depth owned size for `SessionEventQueueObservations`, in bytes. | `src/session/lifecycle/observations.rs:24` |
| sym-3ce0768a577530accd76 | `SessionEventQueueObservations::receiver_closed_total` | struct_field | Counts the total number of receiver closed observed by `SessionEventQueueObservations`. | `src/session/lifecycle/observations.rs:28` |
| sym-df66018105f463ec4a75 | `SessionExternalSourceMetrics::runtime` | struct_field | Stores the runtime as a `SourceRuntimeObservations` value in `SessionExternalSourceMetrics`. | `src/session/lifecycle/observations.rs:127` |
| sym-a9495720cb5114ce8a1c | `SessionExternalSourceMetrics::source_id` | struct_field | Identifies the source identifier recorded by `SessionExternalSourceMetrics`. | `src/session/lifecycle/observations.rs:126` |
| sym-b7c8465d9bba76093fbc | `SessionExternalSourceMetrics::source_instance_id` | struct_field | Identifies the source instance identifier recorded by `SessionExternalSourceMetrics`. | `src/session/lifecycle/observations.rs:125` |
| sym-6d332cae4699709a69a6 | `SessionGraphRegistrationError::DuplicateNodeType::node_type_id` | struct_field | Identifies the node type identifier recorded by `DuplicateNodeType`. | `src/session/extensions/builtins.rs:32` |
| sym-9f230272c9dd93039706 | `SessionOperatorInputMetrics::edge` | struct_field | References the edge participating in `SessionOperatorInputMetrics`. | `src/session/lifecycle/observations.rs:244` |
| sym-a5cf03518a7382563c53 | `SessionOperatorInputMetrics::port_name` | struct_field | Stores the human-readable port used to identify `SessionOperatorInputMetrics`. | `src/session/lifecycle/observations.rs:243` |
| sym-81393ffc824cad3c2e54 | `SessionOperatorMetrics::finalization_failures_total` | struct_field | Counts the total number of finalization failures observed by `SessionOperatorMetrics`. | `src/session/lifecycle/observations.rs:394` |
| sym-cda48345a8f0740ff376 | `SessionOperatorMetrics::input_edge` | struct_field | Sole counter authority for input delivered by the compiled Session plan. | `src/session/lifecycle/observations.rs:389` |
| sym-0087315a1229ec515662 | `SessionOperatorMetrics::input_ports` | struct_field | Exact per-port input accounting. `input_edge` is the compatibility aggregate across this slice. | `src/session/lifecycle/observations.rs:392` |
| sym-ceb055de255a325df343 | `SessionOperatorMetrics::operator_instance_id` | struct_field | Identifies the operator instance identifier recorded by `SessionOperatorMetrics`. | `src/session/lifecycle/observations.rs:383` |
| sym-5f70265fddb35428e8b9 | `SessionOperatorMetrics::worker` | struct_field | Stores the worker as a `AsyncOperatorObservations` value in `SessionOperatorMetrics`. | `src/session/lifecycle/observations.rs:393` |
| sym-f580dcfdfd8200b78ce8 | `SessionPrepareError::DuplicateOperatorInput::node_id` | struct_field | Identifies the node identifier recorded by `DuplicateOperatorInput`. | `src/session/prepare/error.rs:76` |
| sym-361d6c0695672b348fb3 | `SessionPrepareError::DuplicateSignalRoute::route_id` | struct_field | Identifies the route identifier recorded by `DuplicateSignalRoute`. | `src/session/prepare/error.rs:80` |
| sym-4b85071689a0c1fc9420 | `SessionPrepareError::DuplicateSourceNode::stem_id` | struct_field | Identifies the stem identifier recorded by `DuplicateSourceNode`. | `src/session/prepare/error.rs:17` |
| sym-b2019210076f1c17b6b1 | `SessionPrepareError::DuplicateWorkerRoute::route_id` | struct_field | Identifies the route identifier recorded by `DuplicateWorkerRoute`. | `src/session/prepare/error.rs:53` |
| sym-fc20441e2f45ca1b6123 | `SessionPrepareError::IncompatibleNodeBinding::node_id` | struct_field | Identifies the node identifier recorded by `IncompatibleNodeBinding`. | `src/session/prepare/error.rs:74` |
| sym-080e4e28cad1ad9fc634 | `SessionPrepareError::InvalidExternalAudioMedia::output_port` | struct_field | References the output port participating in `InvalidExternalAudioMedia`. | `src/session/prepare/error.rs:28` |
| sym-ca52d6c73378d9bfd37c | `SessionPrepareError::InvalidExternalAudioMedia::source_instance_id` | struct_field | Identifies the source instance identifier recorded by `InvalidExternalAudioMedia`. | `src/session/prepare/error.rs:27` |
| sym-b5f451dfd56b6714c6d5 | `SessionPrepareError::InvalidGeneratedAudioMedia::stem_id` | struct_field | Identifies the stem identifier recorded by `InvalidGeneratedAudioMedia`. | `src/session/prepare/error.rs:33` |
| sym-a45d2a975a0754181882 | `SessionPrepareError::InvalidOperatorInputPort::edge_id` | struct_field | Identifies the edge identifier recorded by `InvalidOperatorInputPort`. | `src/session/prepare/error.rs:49` |
| sym-da72b92db431a8796ec1 | `SessionPrepareError::InvalidOperatorInputPort::port_name` | struct_field | Stores the human-readable port used to identify `InvalidOperatorInputPort`. | `src/session/prepare/error.rs:49` |
| sym-cdc617553050c8cc2860 | `SessionPrepareError::MissingAsyncOperatorFactory::node_id` | struct_field | Identifies the node identifier recorded by `MissingAsyncOperatorFactory`. | `src/session/prepare/error.rs:68` |
| sym-0613da32527d22e7d4ba | `SessionPrepareError::MissingExternalAudioIngress::output_port` | struct_field | References the output port participating in `MissingExternalAudioIngress`. | `src/session/prepare/error.rs:21` |
| sym-05790b4cf0499acd74d5 | `SessionPrepareError::MissingExternalAudioIngress::source_instance_id` | struct_field | Identifies the source instance identifier recorded by `MissingExternalAudioIngress`. | `src/session/prepare/error.rs:20` |
| sym-9233c68529a1a6e1b507 | `SessionPrepareError::MissingExternalSourceDefinition::source_type_id` | struct_field | Identifies the source type identifier recorded by `MissingExternalSourceDefinition`. | `src/session/prepare/error.rs:24` |
| sym-2eb121b90e3743ff44a1 | `SessionPrepareError::MissingExternalSourceRouteEdge::route_id` | struct_field | Identifies the route identifier recorded by `MissingExternalSourceRouteEdge`. | `src/session/prepare/error.rs:37` |
| sym-0abbd4f73f73f467a13f | `SessionPrepareError::MissingGeneratedAudioBridge::stem_id` | struct_field | Identifies the stem identifier recorded by `MissingGeneratedAudioBridge`. | `src/session/prepare/error.rs:35` |
| sym-b8a0fa7d8cf8eddc4e4c | `SessionPrepareError::MissingGeneratedAudioIngress::stem_id` | struct_field | Identifies the stem identifier recorded by `MissingGeneratedAudioIngress`. | `src/session/prepare/error.rs:31` |
| sym-ab1c320911eb886375a9 | `SessionPrepareError::MissingNodeBinding::node_id` | struct_field | Identifies the node identifier recorded by `MissingNodeBinding`. | `src/session/prepare/error.rs:72` |
| sym-9d3c1bceaab164615d35 | `SessionPrepareError::MissingOperatorSignalInput::edge_id` | struct_field | Identifies the edge identifier recorded by `MissingOperatorSignalInput`. | `src/session/prepare/error.rs:78` |
| sym-1f6da65bc0a008a1e973 | `SessionPrepareError::MissingSourceNode::stem_id` | struct_field | Identifies the stem identifier recorded by `MissingSourceNode`. | `src/session/prepare/error.rs:15` |
| sym-484380e573de809f51d7 | `SessionPrepareError::MissingTypedEdgePlan::edge_id` | struct_field | Identifies the edge identifier recorded by `MissingTypedEdgePlan`. | `src/session/prepare/error.rs:66` |
| sym-e730e73d90441cf62abc | `SessionPrepareError::MissingWorkerCapacity::edge_id` | struct_field | Identifies the edge identifier recorded by `MissingWorkerCapacity`. | `src/session/prepare/error.rs:45` |
| sym-2cc54d05397d50e7138b | `SessionPrepareError::MissingWorkerEdge::edge_id` | struct_field | Identifies the edge identifier recorded by `MissingWorkerEdge`. | `src/session/prepare/error.rs:41` |
| sym-f1e3c31658bb25491bcf | `SessionPrepareError::MissingWorkerEdgeContract::edge_id` | struct_field | Identifies the edge identifier recorded by `MissingWorkerEdgeContract`. | `src/session/prepare/error.rs:43` |
| sym-fde5202ccf38c03a1e12 | `SessionPrepareError::MissingWorkerSampleSpec::edge_id` | struct_field | Identifies the edge identifier recorded by `MissingWorkerSampleSpec`. | `src/session/prepare/error.rs:47` |
| sym-3a2da3cd67fe72b228e4 | `SessionPrepareError::MissingWorkerTarget::edge_id` | struct_field | Identifies the edge identifier recorded by `MissingWorkerTarget`. | `src/session/prepare/error.rs:39` |
| sym-d181bf51d143640a3482 | `SessionPrepareError::OperatorDeclarationMismatch::node_id` | struct_field | Identifies the node identifier recorded by `OperatorDeclarationMismatch`. | `src/session/prepare/error.rs:70` |
| sym-c2d49cc5722c869c8751 | `SessionPrepareError::SignalRouteMismatch::edge_id` | struct_field | Identifies the edge identifier recorded by `SignalRouteMismatch`. | `src/session/prepare/error.rs:82` |
| sym-ece6b424f2e565decf44 | `SessionPrepareError::SignalRouteMismatch::route_id` | struct_field | Identifies the route identifier recorded by `SignalRouteMismatch`. | `src/session/prepare/error.rs:82` |
| sym-9674c471a4bc70ef537a | `SessionPrepareError::UnknownWorkerRoute::edge_id` | struct_field | Identifies the edge identifier recorded by `UnknownWorkerRoute`. | `src/session/prepare/error.rs:51` |
| sym-0c555c2bb4cd1410a1d0 | `SessionPrepareError::UnknownWorkerRoute::route_id` | struct_field | Identifies the route identifier recorded by `UnknownWorkerRoute`. | `src/session/prepare/error.rs:51` |
| sym-cf447cc664106c1be849 | `SessionPrepareError::WorkerRouteMismatch::actual_endpoint_id` | struct_field | Identifies the actual endpoint identifier recorded by `WorkerRouteMismatch`. | `src/session/prepare/error.rs:63` |
| sym-0d752ecc028d78664630 | `SessionPrepareError::WorkerRouteMismatch::actual_stem_id` | struct_field | Identifies the actual stem identifier recorded by `WorkerRouteMismatch`. | `src/session/prepare/error.rs:61` |
| sym-7b5bd1b84006e8a0316b | `SessionPrepareError::WorkerRouteMismatch::edge_id` | struct_field | Identifies the edge identifier recorded by `WorkerRouteMismatch`. | `src/session/prepare/error.rs:58` |
| sym-4858257d91269d1c172b | `SessionPrepareError::WorkerRouteMismatch::expected_endpoint_id` | struct_field | Identifies the expected endpoint identifier recorded by `WorkerRouteMismatch`. | `src/session/prepare/error.rs:62` |
| sym-b1725f5874cf29b19517 | `SessionPrepareError::WorkerRouteMismatch::expected_stem_id` | struct_field | Identifies the expected stem identifier recorded by `WorkerRouteMismatch`. | `src/session/prepare/error.rs:60` |
| sym-10e8d50244f1b1a358d0 | `SessionPrepareError::WorkerRouteMismatch::route_id` | struct_field | Identifies the route identifier recorded by `WorkerRouteMismatch`. | `src/session/prepare/error.rs:59` |
| sym-4b6836d311a0dc39d12d | `SessionPrepareError::WorkerTopologyMismatch::actual` | struct_field | Records the value observed by `WorkerTopologyMismatch`. | `src/session/prepare/error.rs:88` |
| sym-747436fc378d4b39dd86 | `SessionPrepareError::WorkerTopologyMismatch::actual_operator_inputs` | struct_field | References the actual operator inputs participating in `WorkerTopologyMismatch`. | `src/session/prepare/error.rs:90` |
| sym-c931dcb34e8e86394681 | `SessionPrepareError::WorkerTopologyMismatch::actual_signal_endpoints` | struct_field | References the actual signal endpoints participating in `WorkerTopologyMismatch`. | `src/session/prepare/error.rs:92` |
| sym-3d4286933facf2f8f895 | `SessionPrepareError::WorkerTopologyMismatch::expected` | struct_field | Records the value expected by `WorkerTopologyMismatch`. | `src/session/prepare/error.rs:87` |
| sym-d906b501ce4a5ac7d83c | `SessionPrepareError::WorkerTopologyMismatch::expected_operator_inputs` | struct_field | References the expected operator inputs participating in `WorkerTopologyMismatch`. | `src/session/prepare/error.rs:89` |
| sym-da143f37e116d14bad29 | `SessionPrepareError::WorkerTopologyMismatch::expected_signal_endpoints` | struct_field | References the expected signal endpoints participating in `WorkerTopologyMismatch`. | `src/session/prepare/error.rs:91` |
| sym-bd76e405628d0dce1caf | `SessionRouteDropObservations::branch_pool_exhausted_drops_total` | struct_field | Counts the total number of branch pool exhausted drops observed by `SessionRouteDropObservations`. | `src/session/lifecycle/observations.rs:165` |
| sym-906dd745bc2cb43e947a | `SessionRouteDropObservations::frames_attempted_total` | struct_field | Counts the total number of frames attempted observed by `SessionRouteDropObservations`. | `src/session/lifecycle/observations.rs:161` |
| sym-50913e2baa39e673bca5 | `SessionRouteDropObservations::frames_dropped_total` | struct_field | Counts the total number of frames dropped observed by `SessionRouteDropObservations`. | `src/session/lifecycle/observations.rs:160` |
| sym-07fbf25742a59c73fa6c | `SessionRouteDropObservations::freeze_failed_drops_total` | struct_field | Counts the total number of freeze failed drops observed by `SessionRouteDropObservations`. | `src/session/lifecycle/observations.rs:167` |
| sym-773eab47c8e39af0fb9e | `SessionRouteDropObservations::interval` | struct_field | Stores the interval as a `SessionRouteObservationInterval` value in `SessionRouteDropObservations`. | `src/session/lifecycle/observations.rs:159` |
| sym-e04f36a028141caaa2e1 | `SessionRouteDropObservations::invalid_copy_policy_drops_total` | struct_field | Counts the total number of invalid copy policy drops observed by `SessionRouteDropObservations`. | `src/session/lifecycle/observations.rs:166` |
| sym-8d4078dcdce8ff689193 | `SessionRouteDropObservations::queue_full_drops_total` | struct_field | Counts the total number of queue full drops observed by `SessionRouteDropObservations`. | `src/session/lifecycle/observations.rs:163` |
| sym-1dfee180c52d8d545101 | `SessionRouteDropObservations::receiver_unavailable_drops_total` | struct_field | Counts the total number of receiver unavailable drops observed by `SessionRouteDropObservations`. | `src/session/lifecycle/observations.rs:162` |
| sym-ed1e4148ed79042e39e2 | `SessionRouteDropObservations::route_id` | struct_field | Identifies the route identifier recorded by `SessionRouteDropObservations`. | `src/session/lifecycle/observations.rs:158` |
| sym-c32c46ea31954937403e | `SessionRouteDropObservations::shared_reference_exhausted_drops_total` | struct_field | Counts the total number of shared reference exhausted drops observed by `SessionRouteDropObservations`. | `src/session/lifecycle/observations.rs:164` |
| sym-ccc5041614b365a90ed7 | `SessionRouteLatencyObservations::boundary` | struct_field | Stores the boundary as a `SessionRouteLatencyBoundary` value in `SessionRouteLatencyObservations`. | `src/session/lifecycle/observations.rs:184` |
| sym-0335926e6b4e22ef18fa | `SessionRouteLatencyObservations::future_timestamp_total` | struct_field | Counts the total number of future timestamp observed by `SessionRouteLatencyObservations`. | `src/session/lifecycle/observations.rs:188` |
| sym-a40d84d8c2639a36874e | `SessionRouteLatencyObservations::max_ns` | struct_field | Stores the max value for `SessionRouteLatencyObservations`, in nanoseconds. | `src/session/lifecycle/observations.rs:192` |
| sym-d154b68937a7a8eac066 | `SessionRouteLatencyObservations::missing_or_incompatible_clock_total` | struct_field | Counts the total number of missing or incompatible clock observed by `SessionRouteLatencyObservations`. | `src/session/lifecycle/observations.rs:187` |
| sym-1db8771f1a681eecf8d7 | `SessionRouteLatencyObservations::p50_ns` | struct_field | Stores the p50 value for `SessionRouteLatencyObservations`, in nanoseconds. | `src/session/lifecycle/observations.rs:189` |
| sym-883e2c5fa4fe1d1f5aeb | `SessionRouteLatencyObservations::p95_ns` | struct_field | Stores the p95 value for `SessionRouteLatencyObservations`, in nanoseconds. | `src/session/lifecycle/observations.rs:190` |
| sym-12a2416e5685fe74cf27 | `SessionRouteLatencyObservations::p99_ns` | struct_field | Stores the p99 value for `SessionRouteLatencyObservations`, in nanoseconds. | `src/session/lifecycle/observations.rs:191` |
| sym-bb75974291381210081e | `SessionRouteLatencyObservations::route_id` | struct_field | Identifies the route identifier recorded by `SessionRouteLatencyObservations`. | `src/session/lifecycle/observations.rs:183` |
| sym-f68ddf7c61d0e66d2daf | `SessionRouteLatencyObservations::samples_total` | struct_field | Counts the total number of samples observed by `SessionRouteLatencyObservations`. | `src/session/lifecycle/observations.rs:186` |
| sym-6662ed1eaf0fc3c3c827 | `SessionRouteLatencyObservations::unit` | struct_field | Stores the unit as a `SessionRouteLatencyUnit` value in `SessionRouteLatencyObservations`. | `src/session/lifecycle/observations.rs:185` |
| sym-8d95e1bc8ed3a5b91ac7 | `SessionRouteMetrics::edge` | struct_field | References the edge participating in `SessionRouteMetrics`. | `src/session/lifecycle/observations.rs:142` |
| sym-f274e638f1c96bf820af | `SessionRouteMetrics::endpoint` | struct_field | References the endpoint participating in `SessionRouteMetrics`. | `src/session/lifecycle/observations.rs:143` |
| sym-ea33d4aa404340be893b | `SessionRouteMetrics::endpoint_finalization_failures_total` | struct_field | Counts the total number of endpoint finalization failures observed by `SessionRouteMetrics`. | `src/session/lifecycle/observations.rs:145` |
| sym-df5ae7aba3e9c07ad3bd | `SessionRouteMetrics::endpoint_id` | struct_field | Identifies the endpoint identifier recorded by `SessionRouteMetrics`. | `src/session/lifecycle/observations.rs:141` |
| sym-13852ae31747058f0d2a | `SessionRouteMetrics::endpoint_observation_stage` | struct_field | References the endpoint observation stage participating in `SessionRouteMetrics`. | `src/session/lifecycle/observations.rs:144` |
| sym-4a28c2878005960b0352 | `SessionRouteMetrics::route_id` | struct_field | Identifies the route identifier recorded by `SessionRouteMetrics`. | `src/session/lifecycle/observations.rs:140` |
| sym-1e8cedc8ef5217aa573f | `SessionSidecarMetrics::host` | struct_field | Stores the host as a `SidecarHostSnapshot` value in `SessionSidecarMetrics`. | `src/session/lifecycle/observations.rs:135` |
| sym-feaf9b640db10c4d7050 | `SessionSidecarMetrics::sidecar_id` | struct_field | Identifies the sidecar identifier recorded by `SessionSidecarMetrics`. | `src/session/lifecycle/observations.rs:134` |
| sym-a870cc6e67933d1594e1 | `SessionSourceMetrics::capture` | struct_field | Stores the capture as a `CaptureOwnerObservations` value in `SessionSourceMetrics`. | `src/session/lifecycle/observations.rs:119` |
| sym-9395f2690c37ecda483d | `SessionSourceMetrics::ingress` | struct_field | Contains the ingress owned or reported by `SessionSourceMetrics`. | `src/session/lifecycle/observations.rs:120` |
| sym-d7609e3f45e9321d93e9 | `SessionSourceMetrics::stem_id` | struct_field | Identifies the stem identifier recorded by `SessionSourceMetrics`. | `src/session/lifecycle/observations.rs:118` |
| sym-46935ba116a5ea6e28d4 | `SessionStartError::Cancelled::rollback_failures_total` | struct_field | Counts the total number of rollback failures observed by `Cancelled`. | `src/session/lifecycle/control.rs:202` |
| sym-f3226ae27d4c9102872b | `SessionStartError::CaptureOpen::rollback_failures_total` | struct_field | Counts the total number of rollback failures observed by `CaptureOpen`. | `src/session/lifecycle/control.rs:177` |
| sym-91fb53b8c171aaf84842 | `SessionStartError::CaptureOpen::source` | struct_field | Carries the source selected for `CaptureOpen`. | `src/session/lifecycle/control.rs:176` |
| sym-e3c723cc3bb4308416de | `SessionStartError::CaptureOpen::stem_id` | struct_field | Identifies the stem identifier recorded by `CaptureOpen`. | `src/session/lifecycle/control.rs:174` |
| sym-ee4f4c30486b21fce3d4 | `SessionStartError::CapturePrepare::rollback_failures_total` | struct_field | Counts the total number of rollback failures observed by `CapturePrepare`. | `src/session/lifecycle/control.rs:170` |
| sym-2d427fab1de9adf9b8cf | `SessionStartError::CapturePrepare::source` | struct_field | Carries the source selected for `CapturePrepare`. | `src/session/lifecycle/control.rs:169` |
| sym-d192d038f4dbe864764c | `SessionStartError::CapturePrepare::stem_id` | struct_field | Identifies the stem identifier recorded by `CapturePrepare`. | `src/session/lifecycle/control.rs:167` |
| sym-cec4364d5b9af97c6a0e | `SessionStartError::EndpointPrepare::rollback_failures_total` | struct_field | Counts the total number of rollback failures observed by `EndpointPrepare`. | `src/session/lifecycle/control.rs:163` |
| sym-9a41e71e009d4ebb78cf | `SessionStartError::EndpointPrepare::source` | struct_field | Carries the source selected for `EndpointPrepare`. | `src/session/lifecycle/control.rs:162` |
| sym-2ca1f6ddcdb726409013 | `SessionStartError::EndpointStart::rollback_failures_total` | struct_field | Counts the total number of rollback failures observed by `EndpointStart`. | `src/session/lifecycle/control.rs:183` |
| sym-1ffc185cfba360ce4d02 | `SessionStartError::EndpointStart::source` | struct_field | Carries the source selected for `EndpointStart`. | `src/session/lifecycle/control.rs:182` |
| sym-aa2a0b8054bdb2904e69 | `SessionStartError::ExternalAudioBridge::message` | struct_field | Carries the diagnostic message reported by `ExternalAudioBridge`. | `src/session/lifecycle/control.rs:133` |
| sym-2de47c050e0d854f7859 | `SessionStartError::ExternalAudioBridge::rollback_failures_total` | struct_field | Counts the total number of rollback failures observed by `ExternalAudioBridge`. | `src/session/lifecycle/control.rs:134` |
| sym-cdd00f1e3c9bcc99d85e | `SessionStartError::ExternalSourcePrepare::message` | struct_field | Carries the diagnostic message reported by `ExternalSourcePrepare`. | `src/session/lifecycle/control.rs:128` |
| sym-dbb3debf6d33d4011a56 | `SessionStartError::ExternalSourcePrepare::rollback_failures_total` | struct_field | Counts the total number of rollback failures observed by `ExternalSourcePrepare`. | `src/session/lifecycle/control.rs:129` |
| sym-dc9f09a78f3b9b7e96a7 | `SessionStartError::ExternalSourceStart::message` | struct_field | Carries the diagnostic message reported by `ExternalSourceStart`. | `src/session/lifecycle/control.rs:143` |
| sym-13441eca2554ec2ac66b | `SessionStartError::ExternalSourceStart::rollback_failures_total` | struct_field | Counts the total number of rollback failures observed by `ExternalSourceStart`. | `src/session/lifecycle/control.rs:144` |
| sym-09569b4f7bbc4fb42297 | `SessionStartError::GeneratedAudioBridge::message` | struct_field | Carries the diagnostic message reported by `GeneratedAudioBridge`. | `src/session/lifecycle/control.rs:138` |
| sym-57880d5443cb373a51e4 | `SessionStartError::GeneratedAudioBridge::rollback_failures_total` | struct_field | Counts the total number of rollback failures observed by `GeneratedAudioBridge`. | `src/session/lifecycle/control.rs:139` |
| sym-7b1745466bac5bc7323a | `SessionStartError::InvalidOptions::reason` | struct_field | Carries the reason reported by `InvalidOptions`. | `src/session/lifecycle/control.rs:123` |
| sym-f5572db3ec48930c4545 | `SessionStartError::MissingEndpointDeclaration::endpoint_id` | struct_field | Identifies the endpoint identifier recorded by `MissingEndpointDeclaration`. | `src/session/lifecycle/control.rs:158` |
| sym-ed07777ef3986b1e776a | `SessionStartError::OperatorPrepare::message` | struct_field | Carries the diagnostic message reported by `OperatorPrepare`. | `src/session/lifecycle/control.rs:154` |
| sym-46a6fc395d1e28859e91 | `SessionStartError::OperatorPrepare::operator_instance_id` | struct_field | Identifies the operator instance identifier recorded by `OperatorPrepare`. | `src/session/lifecycle/control.rs:153` |
| sym-3e831a7e53db9420aa18 | `SessionStartError::OperatorPrepare::rollback_failures_total` | struct_field | Counts the total number of rollback failures observed by `OperatorPrepare`. | `src/session/lifecycle/control.rs:155` |
| sym-bdec3468f9a8d2e123f3 | `SessionStartError::OperatorRuntimeHost::message` | struct_field | Carries the diagnostic message reported by `OperatorRuntimeHost`. | `src/session/lifecycle/control.rs:148` |
| sym-d81ccc068e8c8af613fe | `SessionStartError::OperatorRuntimeHost::rollback_failures_total` | struct_field | Counts the total number of rollback failures observed by `OperatorRuntimeHost`. | `src/session/lifecycle/control.rs:149` |
| sym-349cdd99f48af5b24b1b | `SessionStartError::RuntimeRunner::rollback_failures_total` | struct_field | Counts the total number of rollback failures observed by `RuntimeRunner`. | `src/session/lifecycle/control.rs:189` |
| sym-18fed7ee5623dd08890b | `SessionStartError::RuntimeRunner::source` | struct_field | Carries the source selected for `RuntimeRunner`. | `src/session/lifecycle/control.rs:188` |
| sym-ce1b11e4550dafe71e43 | `SessionStartError::RuntimeWorkerReady::message` | struct_field | Carries the diagnostic message reported by `RuntimeWorkerReady`. | `src/session/lifecycle/control.rs:198` |
| sym-457e9e5026b287232ebd | `SessionStartError::RuntimeWorkerReady::rollback_failures_total` | struct_field | Counts the total number of rollback failures observed by `RuntimeWorkerReady`. | `src/session/lifecycle/control.rs:199` |
| sym-c9a5d8600a8336a8e3b4 | `SessionStartError::RuntimeWorkerSpawn::message` | struct_field | Carries the diagnostic message reported by `RuntimeWorkerSpawn`. | `src/session/lifecycle/control.rs:193` |
| sym-cd551019878edb5349f2 | `SessionStartError::RuntimeWorkerSpawn::rollback_failures_total` | struct_field | Counts the total number of rollback failures observed by `RuntimeWorkerSpawn`. | `src/session/lifecycle/control.rs:194` |
| sym-5ee88616427099d4f9d1 | `SessionStartOptions::capture_frame_capacity_frames` | struct_field | Sets the capture frame capacity frames available to `SessionStartOptions`. | `src/session/lifecycle/control.rs:24` |
| sym-bfd09d99749983328ad5 | `SessionStartOptions::capture_runtime_event_capacity_events` | struct_field | Sets the capture runtime event capacity events available to `SessionStartOptions`. | `src/session/lifecycle/control.rs:25` |
| sym-598e03faef3d4f001b97 | `SessionStartOptions::runtime_idle_poll_ms` | struct_field | Stores the runtime idle poll value for `SessionStartOptions`, in milliseconds. | `src/session/lifecycle/control.rs:27` |
| sym-3ed98a84f306b8097d9c | `SessionStartOptions::runtime_ready_timeout_ms` | struct_field | Stores the runtime ready timeout value for `SessionStartOptions`, in milliseconds. | `src/session/lifecycle/control.rs:28` |
| sym-4d219883c8fdebe386d9 | `SessionStartOptions::runtime_work_budget_frames` | struct_field | Contains the runtime work budget frames owned or reported by `SessionStartOptions`. | `src/session/lifecycle/control.rs:26` |
| sym-2d42d2a63106a72c6ca9 | `SessionStartOptions::session_event_capacity_events` | struct_field | Sets the session event capacity events available to `SessionStartOptions`. | `src/session/lifecycle/control.rs:29` |
| sym-b4a7f875aa780b816506 | `SessionTraceRecord::kind` | struct_field | Records the kind selected for `SessionTraceRecord`. | `src/session/lifecycle/trace.rs:59` |
| sym-67911249802cc2181c05 | `SessionTraceRecord::observed_at_ns` | struct_field | Stores the observed at value for `SessionTraceRecord`, in nanoseconds. | `src/session/lifecycle/trace.rs:57` |
| sym-25204beb956a5394ca40 | `SessionTraceRecord::sequence_index` | struct_field | Identifies the sequence index position within `SessionTraceRecord`. | `src/session/lifecycle/trace.rs:56` |
| sym-f2d8726546af55389191 | `SessionTraceRecord::session_id` | struct_field | Identifies the session identifier recorded by `SessionTraceRecord`. | `src/session/lifecycle/trace.rs:58` |
| sym-48fe215ec293f8721836 | `SessionTraceRecordKind::EndpointFailure::endpoint_id` | struct_field | Identifies the endpoint identifier recorded by `EndpointFailure`. | `src/session/lifecycle/trace.rs:36` |
| sym-c74ae852c7585ab09fd7 | `SessionTraceRecordKind::EndpointFailure::route_id` | struct_field | Identifies the route identifier recorded by `EndpointFailure`. | `src/session/lifecycle/trace.rs:35` |
| sym-ea2ec9de50e9ba82a825 | `SessionTraceRecordKind::EndpointFailure::stage_code` | struct_field | Stores the stage code component of `EndpointFailure`. | `src/session/lifecycle/trace.rs:37` |
| sym-4ad7f2fe29d8f31e8a2c | `SessionTraceRecordKind::FinalizationFailure::stage` | struct_field | Records the stage selected for `FinalizationFailure`. | `src/session/lifecycle/trace.rs:43` |
| sym-b964f65a672d5680e4b2 | `SessionTraceRecordKind::Lifecycle::state` | struct_field | Records the state selected for `Lifecycle`. | `src/session/lifecycle/trace.rs:29` |
| sym-c53e75bfa7dfa199d567 | `SessionTraceRecordKind::RollbackFailure::stage` | struct_field | Records the stage selected for `RollbackFailure`. | `src/session/lifecycle/trace.rs:40` |
| sym-130408c1fe51bc6ff9d4 | `SessionTraceRecordKind::SourceFailure::stem_id` | struct_field | Identifies the stem identifier recorded by `SourceFailure`. | `src/session/lifecycle/trace.rs:32` |
| sym-907be6ef2c4e0bd601f9 | `SessionTraceRecordKind::Terminal::endpoint_failures_total` | struct_field | Counts the total number of endpoint failures observed by `Terminal`. | `src/session/lifecycle/trace.rs:48` |
| sym-ef3f102c5690fba26190 | `SessionTraceRecordKind::Terminal::finalization_failures_total` | struct_field | Counts the total number of finalization failures observed by `Terminal`. | `src/session/lifecycle/trace.rs:50` |
| sym-4cd4f0784f0bd65b0c5a | `SessionTraceRecordKind::Terminal::rollback_failures_total` | struct_field | Counts the total number of rollback failures observed by `Terminal`. | `src/session/lifecycle/trace.rs:49` |
| sym-703636f87f736e015c85 | `SessionTraceRecordKind::Terminal::source_failures_total` | struct_field | Counts the total number of source failures observed by `Terminal`. | `src/session/lifecycle/trace.rs:47` |
| sym-5900063a814823405058 | `SessionTraceRecordKind::Terminal::state` | struct_field | Records the state selected for `Terminal`. | `src/session/lifecycle/trace.rs:46` |
| sym-4b40cfa5e72eb4544869 | `SessionTraceRecorderOutcome::path` | struct_field | Points to the path used by `SessionTraceRecorderOutcome`. | `src/session/lifecycle/trace.rs:71` |
| sym-33fab61fe0cc919ca0bc | `SessionTraceRecorderOutcome::records_attempted_total` | struct_field | Counts the total number of records attempted observed by `SessionTraceRecorderOutcome`. | `src/session/lifecycle/trace.rs:72` |
| sym-b0794ebd0ec65012b0b4 | `SessionTraceRecorderOutcome::records_dropped_total` | struct_field | Counts the total number of records dropped observed by `SessionTraceRecorderOutcome`. | `src/session/lifecycle/trace.rs:74` |
| sym-0f3103133cd75bfeaccb | `SessionTraceRecorderOutcome::records_enqueued_total` | struct_field | Counts the total number of records enqueued observed by `SessionTraceRecorderOutcome`. | `src/session/lifecycle/trace.rs:73` |
| sym-4d03184b2c31c0b8dc28 | `SessionTraceRecorderOutcome::records_written_total` | struct_field | Counts the total number of records written observed by `SessionTraceRecorderOutcome`. | `src/session/lifecycle/trace.rs:75` |
| sym-f1ab13a97fea567b67f1 | `SessionTraceRecorderOutcome::rolling_hash` | struct_field | Stores the rolling integrity hash computed for `SessionTraceRecorderOutcome`. | `src/session/lifecycle/trace.rs:76` |
| sym-e83bbf9cd07d6618398e | `SessionTraceRecorderStartError::OutputExists::path` | struct_field | Points to the path used by `OutputExists`. | `src/session/lifecycle/trace.rs:92` |
| sym-a006f60f52fe1a0c9e49 | `SessionTraceTerminal::endpoint_failures_total` | struct_field | Counts the total number of endpoint failures observed by `SessionTraceTerminal`. | `src/session/lifecycle/trace.rs:342` |
| sym-8d815c80a2885a3325c2 | `SessionTraceTerminal::finalization_failures_total` | struct_field | Counts the total number of finalization failures observed by `SessionTraceTerminal`. | `src/session/lifecycle/trace.rs:344` |
| sym-85c291dc87673ec335c1 | `SessionTraceTerminal::rollback_failures_total` | struct_field | Counts the total number of rollback failures observed by `SessionTraceTerminal`. | `src/session/lifecycle/trace.rs:343` |
| sym-7a9fec37f0d1e77ba871 | `SessionTraceTerminal::source_failures_total` | struct_field | Counts the total number of source failures observed by `SessionTraceTerminal`. | `src/session/lifecycle/trace.rs:341` |
| sym-8ed5729a86ed199cc346 | `SessionTraceTerminal::state` | struct_field | Records the state selected for `SessionTraceTerminal`. | `src/session/lifecycle/trace.rs:340` |
| sym-9fc13308e35cf75a6727 | `SessionTraceValidation::lifecycle` | struct_field | Stores the lifecycle component of `SessionTraceValidation`. | `src/session/lifecycle/trace.rs:350` |
| sym-1fcbf490016943ea4274 | `SessionTraceValidation::records_validated_total` | struct_field | Counts the total number of records validated observed by `SessionTraceValidation`. | `src/session/lifecycle/trace.rs:352` |
| sym-7e0300a19e3d4508e1e4 | `SessionTraceValidation::session_id` | struct_field | Identifies the session identifier recorded by `SessionTraceValidation`. | `src/session/lifecycle/trace.rs:349` |
| sym-b665a9a5d4d08f4b869c | `SessionTraceValidation::terminal` | struct_field | Indicates whether terminal applies to `SessionTraceValidation`. | `src/session/lifecycle/trace.rs:351` |
| sym-b5d3ca595117fb2183a3 | `SidecarDeadlines::processing` | struct_field | Sets the processing duration enforced by `SidecarDeadlines`. | `src/runtime/lifecycle/sidecar_host.rs:56` |
| sym-ac388510622d8e98140a | `SidecarDeadlines::ready` | struct_field | Indicates whether ready applies to `SidecarDeadlines`. | `src/runtime/lifecycle/sidecar_host.rs:55` |
| sym-9108684741cf77ac8933 | `SidecarDeadlines::shutdown` | struct_field | Sets the shutdown duration enforced by `SidecarDeadlines`. | `src/runtime/lifecycle/sidecar_host.rs:57` |
| sym-c23a4546ba6ebb63f36f | `SidecarHostError::InvalidState::actual` | struct_field | Records the value observed by `InvalidState`. | `src/runtime/lifecycle/sidecar_host.rs:721` |
| sym-237d859483885dc087c7 | `SidecarHostError::InvalidState::expected` | struct_field | Records the value expected by `InvalidState`. | `src/runtime/lifecycle/sidecar_host.rs:720` |
| sym-7754a06b880f4cca7f5e | `SidecarHostError::UnexpectedMessage::actual` | struct_field | Records the value observed by `UnexpectedMessage`. | `src/runtime/lifecycle/sidecar_host.rs:712` |
| sym-00b19faace0697568e3e | `SidecarHostError::UnexpectedMessage::expected` | struct_field | Records the value expected by `UnexpectedMessage`. | `src/runtime/lifecycle/sidecar_host.rs:711` |
| sym-70cc270d0e15c088f9f9 | `SidecarHostSnapshot::data_dropped_total` | struct_field | Counts the total number of data dropped observed by `SidecarHostSnapshot`. | `src/runtime/lifecycle/sidecar_host.rs:138` |
| sym-135ee6bf071c1935f81b | `SidecarHostSnapshot::data_enqueued_total` | struct_field | Counts the total number of data enqueued observed by `SidecarHostSnapshot`. | `src/runtime/lifecycle/sidecar_host.rs:136` |
| sym-27cfcd0b2aab8e31b261 | `SidecarHostSnapshot::data_received_total` | struct_field | Counts the total number of data received observed by `SidecarHostSnapshot`. | `src/runtime/lifecycle/sidecar_host.rs:137` |
| sym-93286f0c42b9d4cce3ec | `SidecarHostSnapshot::forced_kills_total` | struct_field | Counts the total number of forced kills observed by `SidecarHostSnapshot`. | `src/runtime/lifecycle/sidecar_host.rs:141` |
| sym-ffe7561fe2f2c94fdf0b | `SidecarHostSnapshot::protocol_failures_total` | struct_field | Counts the total number of protocol failures observed by `SidecarHostSnapshot`. | `src/runtime/lifecycle/sidecar_host.rs:139` |
| sym-6ddf0604c34fe298d847 | `SidecarHostSnapshot::reaps_total` | struct_field | Counts the total number of reaps observed by `SidecarHostSnapshot`. | `src/runtime/lifecycle/sidecar_host.rs:142` |
| sym-efa161c332a79ebf1b3b | `SidecarHostSnapshot::state` | struct_field | Records the state selected for `SidecarHostSnapshot`. | `src/runtime/lifecycle/sidecar_host.rs:134` |
| sym-48b776bf4c0565b0e85e | `SidecarHostSnapshot::state_transitions` | struct_field | Contains the state transitions owned or reported by `SidecarHostSnapshot`. | `src/runtime/lifecycle/sidecar_host.rs:135` |
| sym-5269f042eff30c62099e | `SidecarHostSnapshot::timeouts_total` | struct_field | Counts the total number of timeouts observed by `SidecarHostSnapshot`. | `src/runtime/lifecycle/sidecar_host.rs:140` |
| sym-f03467925cf34fdd60a6 | `SidecarMessage::kind` | struct_field | Records the kind selected for `SidecarMessage`. | `src/runtime/lifecycle/sidecar_protocol.rs:74` |
| sym-da03adb20515396502e9 | `SidecarMessage::payload` | struct_field | Contains the encoded message body carried by `SidecarMessage`. | `src/runtime/lifecycle/sidecar_protocol.rs:82` |
| sym-0b94331e1dbc26b75b6b | `SidecarMessage::role` | struct_field | Records the role selected for `SidecarMessage`. | `src/runtime/lifecycle/sidecar_protocol.rs:80` |
| sym-1565b4e42797aaf6f224 | `SidecarMessage::schema` | struct_field | Records the schema selected for `SidecarMessage`. | `src/runtime/lifecycle/sidecar_protocol.rs:81` |
| sym-9c5f831f1ee40c325911 | `SidecarMessage::sequence_number` | struct_field | Orders `SidecarMessage` within its protocol or stream sequence. | `src/runtime/lifecycle/sidecar_protocol.rs:77` |
| sym-20462da9c0dc554db6a7 | `SidecarMessage::signal_id` | struct_field | Identifies the signal identifier recorded by `SidecarMessage`. | `src/runtime/lifecycle/sidecar_protocol.rs:79` |
| sym-1d7195184a5788560e32 | `SidecarMessage::stream_id` | struct_field | Identifies the stream identifier recorded by `SidecarMessage`. | `src/runtime/lifecycle/sidecar_protocol.rs:76` |
| sym-949cb8eff9dc8885e305 | `SidecarMessage::terminal` | struct_field | Indicates whether terminal applies to `SidecarMessage`. | `src/runtime/lifecycle/sidecar_protocol.rs:75` |
| sym-018ff998d33451c5f8c7 | `SidecarMessage::timestamp_ns` | struct_field | Stores the timestamp value for `SidecarMessage`, in nanoseconds. | `src/runtime/lifecycle/sidecar_protocol.rs:78` |
| sym-b4c89d0365fb84397402 | `SidecarProcessSpec::arguments` | struct_field | Contains the arguments owned or reported by `SidecarProcessSpec`. | `src/runtime/lifecycle/sidecar_host.rs:74` |
| sym-b7504b4023efbe5b5015 | `SidecarProcessSpec::configuration` | struct_field | Contains the serialized configuration passed to `SidecarProcessSpec`. | `src/runtime/lifecycle/sidecar_host.rs:75` |
| sym-7b1273773a56a86ac87b | `SidecarProcessSpec::data_capacity_messages` | struct_field | Sets the data capacity messages available to `SidecarProcessSpec`. | `src/runtime/lifecycle/sidecar_host.rs:76` |
| sym-d80c6f3c3b0ddd18914c | `SidecarProcessSpec::deadlines` | struct_field | Contains the deadlines owned or reported by `SidecarProcessSpec`. | `src/runtime/lifecycle/sidecar_host.rs:78` |
| sym-ea04c89eaa70b4263660 | `SidecarProcessSpec::id` | struct_field | Identifies the id recorded by `SidecarProcessSpec`. | `src/runtime/lifecycle/sidecar_host.rs:72` |
| sym-f53b1fc4b0cedc2d9718 | `SidecarProcessSpec::program` | struct_field | Points to the executable launched for `SidecarProcessSpec`. | `src/runtime/lifecycle/sidecar_host.rs:73` |
| sym-6b4e0b66ed963c5cf369 | `SidecarProcessSpec::protocol_limits` | struct_field | Contains the protocol limits owned or reported by `SidecarProcessSpec`. | `src/runtime/lifecycle/sidecar_host.rs:77` |
| sym-4af223c5de5ff4028fd2 | `SidecarProtocolError::FieldTooLarge::actual` | struct_field | Records the value observed by `FieldTooLarge`. | `src/runtime/lifecycle/sidecar_protocol.rs:314` |
| sym-cecb8a218d04d549adad | `SidecarProtocolError::FieldTooLarge::field` | struct_field | Stores the field component of `FieldTooLarge`. | `src/runtime/lifecycle/sidecar_protocol.rs:313` |
| sym-7d52000257cc231be019 | `SidecarProtocolError::FieldTooLarge::maximum` | struct_field | Sets the inclusive maximum accepted by `FieldTooLarge`. | `src/runtime/lifecycle/sidecar_protocol.rs:315` |
| sym-fa9f3e4458080b7d15e7 | `SidecarProtocolLimits::max_payload_bytes` | struct_field | Limits payload storage for `SidecarProtocolLimits`, in bytes. | `src/runtime/lifecycle/sidecar_protocol.rs:47` |
| sym-a711b489ebc4e6141c40 | `SidecarProtocolLimits::max_role_bytes` | struct_field | Stores the max role size for `SidecarProtocolLimits`, in bytes. | `src/runtime/lifecycle/sidecar_protocol.rs:45` |
| sym-9b3e98e3bbd00ea8566d | `SidecarProtocolLimits::max_schema_bytes` | struct_field | Stores the max schema size for `SidecarProtocolLimits`, in bytes. | `src/runtime/lifecycle/sidecar_protocol.rs:46` |
| sym-66fa0ad2732d8e30beec | `SidecarProtocolLimits::max_signal_id_bytes` | struct_field | Stores the max signal id size for `SidecarProtocolLimits`, in bytes. | `src/runtime/lifecycle/sidecar_protocol.rs:44` |
| sym-ebff1fabcff91fe593ac | `SignalContinuityObservation::discontinuity_observed` | struct_field | Reports whether discontinuity is observed for `SignalContinuityObservation`. | `src/graph/signal/continuity.rs:7` |
| sym-92308a34adb95cf7f1a7 | `SignalContinuityObservation::policy_changed` | struct_field | Reports whether policy changed is true for `SignalContinuityObservation`. | `src/graph/signal/continuity.rs:9` |
| sym-deeb64f1744fd089685b | `SignalContinuityObservation::source_recovered` | struct_field | References the source recovered participating in `SignalContinuityObservation`. | `src/graph/signal/continuity.rs:8` |
| sym-d4f3f4b5edc3c961aec4 | `SignalEdgeObservations::capacity_signals` | struct_field | Sets the capacity signals available to `SignalEdgeObservations`. | `src/runtime/signal/edge.rs:32` |
| sym-595e8c2f9806dcc80e5b | `SignalEdgeObservations::delivered_total` | struct_field | Compatibility alias for `enqueued_total`. | `src/runtime/signal/edge.rs:44` |
| sym-5b070bbf11558c7b9df7 | `SignalEdgeObservations::depth_signals` | struct_field | Reports the depth signals observed by `SignalEdgeObservations`. | `src/runtime/signal/edge.rs:35` |
| sym-89857fdce352fd589c3c | `SignalEdgeObservations::dropped_total` | struct_field | Counts the total number of dropped observed by `SignalEdgeObservations`. | `src/runtime/signal/edge.rs:45` |
| sym-c4253176d287b86134c3 | `SignalEdgeObservations::enqueued_total` | struct_field | Counts the total number of enqueued observed by `SignalEdgeObservations`. | `src/runtime/signal/edge.rs:37` |
| sym-0ec8ca545960d3469f59 | `SignalEdgeObservations::max_payload_bytes` | struct_field | Limits payload storage for `SignalEdgeObservations`, in bytes. | `src/runtime/signal/edge.rs:33` |
| sym-a46d8e69978aae10bc43 | `SignalEdgeObservations::maximum_buffered_payload_bytes` | struct_field | Stores the maximum buffered payload size for `SignalEdgeObservations`, in bytes. | `src/runtime/signal/edge.rs:34` |
| sym-e5f029ce27d9f07d4d08 | `SignalEdgeObservations::peak_depth_signals` | struct_field | Reports the peak depth signals observed by `SignalEdgeObservations`. | `src/runtime/signal/edge.rs:36` |
| sym-a709c4a32af1ce6fb844 | `SignalEdgeObservations::received_total` | struct_field | Counts the total number of received observed by `SignalEdgeObservations`. | `src/runtime/signal/edge.rs:38` |
| sym-ada1f3d418ca0b90146b | `SourceEmission::envelope` | struct_field | Stores the envelope as a `SignalEnvelope` value in `SourceEmission`. | `src/session/extensions/source.rs:263` |
| sym-e4538bd6b4736baa478d | `SourceEmission::output_port` | struct_field | References the output port participating in `SourceEmission`. | `src/session/extensions/source.rs:262` |
| sym-43c058f74044abe2541c | `SourceEmission::terminal` | struct_field | Indicates whether terminal applies to `SourceEmission`. | `src/session/extensions/source.rs:264` |
| sym-5afb3b117ac734a06bbe | `SourceOutputBranchSpec::branch` | struct_field | References the branch participating in `SourceOutputBranchSpec`. | `src/session/extensions/source.rs:372` |
| sym-fb43bfcac06837fe298f | `SourceOutputBranchSpec::output_port` | struct_field | References the output port participating in `SourceOutputBranchSpec`. | `src/session/extensions/source.rs:371` |
| sym-4a84c8922138013ee550 | `SourceOutputIdentity::output_port` | struct_field | References the output port participating in `SourceOutputIdentity`. | `src/session/extensions/source.rs:230` |
| sym-6d49e8cacc1d84449cbd | `SourceOutputIdentity::stream_id` | struct_field | Identifies the stream identifier recorded by `SourceOutputIdentity`. | `src/session/extensions/source.rs:231` |
| sym-bee1d6c8d5435f92f69f | `SourceOutputPlan::branch_edges` | struct_field | References the branch edges participating in `SourceOutputPlan`. | `src/graph/plan.rs:117` |
| sym-b87a3b79bdf5966e7182 | `SourceOutputPlan::from` | struct_field | Identifies the origin represented by `SourceOutputPlan`. | `src/graph/plan.rs:114` |
| sym-40183e7c8df71f2c1e67 | `SourceOutputPlan::media` | struct_field | Records the media selected for `SourceOutputPlan`. | `src/graph/plan.rs:116` |
| sym-e4dce88c834bc12ec6e8 | `SourceOutputPlan::signal` | struct_field | Stores the signal as a `SignalSpec` value in `SourceOutputPlan`. | `src/graph/plan.rs:115` |
| sym-a743df0671fef79b8e84 | `SourceOutputReceiver::output_port` | struct_field | References the output port participating in `SourceOutputReceiver`. | `src/session/extensions/source.rs:376` |
| sym-25d2b74299944d52b711 | `SourceOutputReceiver::receiver` | struct_field | Owns the receiver endpoint through which `SourceOutputReceiver` exchanges values. | `src/session/extensions/source.rs:377` |
| sym-e28086b5f7702574d8ae | `SourcePrepareContext::manifest` | struct_field | Stores the manifest as a `SourceManifest` value in `SourcePrepareContext`. | `src/session/extensions/source.rs:224` |
| sym-2d968c5bf5bcdfff0ddf | `SourcePrepareContext::session` | struct_field | Stores the session component of `SourcePrepareContext`. | `src/session/extensions/source.rs:225` |
| sym-12cfa469aee004b61588 | `SourceRuntimeObservations::cancellation_total` | struct_field | Counts the total number of cancellation observed by `SourceRuntimeObservations`. | `src/session/extensions/source.rs:398` |
| sym-c779a6408883e05e7002 | `SourceRuntimeObservations::discontinuity_total` | struct_field | Counts the total number of discontinuity observed by `SourceRuntimeObservations`. | `src/session/extensions/source.rs:399` |
| sym-dc3c0a2e7b8118e43701 | `SourceRuntimeObservations::dropped_total` | struct_field | Counts the total number of dropped observed by `SourceRuntimeObservations`. | `src/session/extensions/source.rs:396` |
| sym-d3ca0373fe9c0c3f4bac | `SourceRuntimeObservations::emitted_total` | struct_field | Counts the total number of emitted observed by `SourceRuntimeObservations`. | `src/session/extensions/source.rs:395` |
| sym-23f194e8321697933635 | `SourceRuntimeObservations::failure_total` | struct_field | Counts the total number of failure observed by `SourceRuntimeObservations`. | `src/session/extensions/source.rs:397` |
| sym-a71d9ff30f4dd08d05d0 | `SourceRuntimeObservations::joined` | struct_field | Reports whether joined is true for `SourceRuntimeObservations`. | `src/session/extensions/source.rs:403` |
| sym-6697f018fdd01f6f33f2 | `SourceRuntimeObservations::policy_change_total` | struct_field | Counts the total number of policy change observed by `SourceRuntimeObservations`. | `src/session/extensions/source.rs:401` |
| sym-b0d9e95b6aabb75126a7 | `SourceRuntimeObservations::ready` | struct_field | Indicates whether ready applies to `SourceRuntimeObservations`. | `src/session/extensions/source.rs:402` |
| sym-3834131afbe070171983 | `SourceRuntimeObservations::recovery_total` | struct_field | Counts the total number of recovery observed by `SourceRuntimeObservations`. | `src/session/extensions/source.rs:400` |
| sym-bfd307f653d9b863e98d | `SourceSessionContext::outputs` | struct_field | References the outputs participating in `SourceSessionContext`. | `src/session/extensions/source.rs:238` |
| sym-dc65e4a6b8c41995b7ba | `SourceSessionContext::session_id` | struct_field | Identifies the session identifier recorded by `SourceSessionContext`. | `src/session/extensions/source.rs:236` |
| sym-cc34204351e635fefef0 | `SourceSessionContext::source_id` | struct_field | Identifies the source identifier recorded by `SourceSessionContext`. | `src/session/extensions/source.rs:237` |
| sym-7aca69169fd2214d169a | `SourceTypeIdError::TooLong::actual_bytes` | struct_field | Stores the actual size for `TooLong`, in bytes. | `src/session/extensions/source.rs:75` |
| sym-0eaac69973f6fdea02d6 | `SourceTypeIdError::TooLong::maximum_bytes` | struct_field | Stores the maximum size for `TooLong`, in bytes. | `src/session/extensions/source.rs:76` |
| sym-fc48cb2e95c743fa5af8 | `StreamOrigin::OperatorOutput::operator_instance_id` | struct_field | Identifies the operator instance identifier recorded by `OperatorOutput`. | `src/session/declaration/spec.rs:217` |
| sym-5b4b312f7a177276fe83 | `StreamOrigin::OperatorOutput::output_port` | struct_field | References the output port participating in `OperatorOutput`. | `src/session/declaration/spec.rs:218` |
| sym-96c78f0d0f3e33f12f10 | `StreamOrigin::SourceOutput::output_port` | struct_field | References the output port participating in `SourceOutput`. | `src/session/declaration/spec.rs:212` |
| sym-e78912142e2e65bf4d1b | `StreamOrigin::SourceOutput::source_id` | struct_field | Identifies the source identifier recorded by `SourceOutput`. | `src/session/declaration/spec.rs:214` |
| sym-26d18c9bbf3c8ee74942 | `StreamOrigin::SourceOutput::source_instance_id` | struct_field | Identifies the source instance identifier recorded by `SourceOutput`. | `src/session/declaration/spec.rs:211` |
| sym-2920c2ea4971791a24d2 | `StreamOrigin::SourceOutput::stream_id` | struct_field | Identifies the stream identifier recorded by `SourceOutput`. | `src/session/declaration/spec.rs:213` |
| sym-03e0e3607e9d368df095 | `TimelineMapping::session_origin_ns` | struct_field | Stores the session origin value for `TimelineMapping`, in nanoseconds. | `src/timing/timeline_mapping.rs:4` |
| sym-addb257b22be852d57aa | `TimelineMapping::source_origin_ns` | struct_field | Stores the source origin value for `TimelineMapping`, in nanoseconds. | `src/timing/timeline_mapping.rs:3` |
| sym-e5953d3ff15848521d25 | `TypedEdgeBranchSpec::capacity_signals` | struct_field | Sets the capacity signals available to `TypedEdgeBranchSpec`. | `src/runtime/signal/edge.rs:249` |
| sym-5aa9cd4acbcce6f05411 | `TypedEdgeBranchSpec::edge_contract` | struct_field | References the edge contract participating in `TypedEdgeBranchSpec`. | `src/runtime/signal/edge.rs:250` |
| sym-6ccd3bb7bbc26fbce117 | `TypedEdgeBuildError::CapacityTooLarge::capacity_signals` | struct_field | Sets the capacity signals available to `CapacityTooLarge`. | `src/runtime/signal/edge.rs:393` |
| sym-d8a95aa699cc0af02871 | `TypedEdgeBuildError::CapacityTooLarge::maximum` | struct_field | Sets the inclusive maximum accepted by `CapacityTooLarge`. | `src/runtime/signal/edge.rs:394` |
| sym-e78d65d0cb0e5293e36d | `TypedEdgeBuildError::PayloadLimitTooLarge::max_payload_bytes` | struct_field | Limits payload storage for `PayloadLimitTooLarge`, in bytes. | `src/runtime/signal/edge.rs:402` |
| sym-9ced44ea520ffda8e80b | `TypedEdgeBuildError::PayloadLimitTooLarge::maximum` | struct_field | Sets the inclusive maximum accepted by `PayloadLimitTooLarge`. | `src/runtime/signal/edge.rs:403` |
| sym-4abe587c0e45171e2523 | `TypedEdgePlan::capacity_signals` | struct_field | Sets the capacity signals available to `TypedEdgePlan`. | `src/graph/plan.rs:103` |
| sym-1509887c8a320e949809 | `TypedEdgePlan::contract` | struct_field | Stores the contract as a `EdgeContract` value in `TypedEdgePlan`. | `src/graph/plan.rs:102` |
| sym-0a752dbb91d95f794c7f | `TypedEdgePlan::edge` | struct_field | References the edge participating in `TypedEdgePlan`. | `src/graph/plan.rs:97` |
| sym-26b02b917943fd358384 | `TypedEdgePlan::from` | struct_field | Identifies the origin represented by `TypedEdgePlan`. | `src/graph/plan.rs:98` |
| sym-992fea22207263524b97 | `TypedEdgePlan::media` | struct_field | Records the media selected for `TypedEdgePlan`. | `src/graph/plan.rs:101` |
| sym-544527e48d854909faf1 | `TypedEdgePlan::metric_id` | struct_field | Identifies the metric identifier recorded by `TypedEdgePlan`. | `src/graph/plan.rs:104` |
| sym-9155f31c72ab49424241 | `TypedEdgePlan::signal` | struct_field | Stores the signal as a `SignalSpec` value in `TypedEdgePlan`. | `src/graph/plan.rs:100` |
| sym-1f5aa8044c16497cc567 | `TypedEdgePlan::to` | struct_field | Identifies the destination represented by `TypedEdgePlan`. | `src/graph/plan.rs:99` |
| sym-75366b3dc6cf80d25301 | `TypedEdgePublishError::PayloadTooLarge::branch_index` | struct_field | Identifies the branch index position within `PayloadTooLarge`. | `src/runtime/signal/edge.rs:417` |
| sym-340f450a83ae1075f50e | `TypedEdgePublishError::PayloadTooLarge::max_payload_bytes` | struct_field | Limits payload storage for `PayloadTooLarge`, in bytes. | `src/runtime/signal/edge.rs:419` |
| sym-79ec29900c9574ca3a19 | `TypedEdgePublishError::PayloadTooLarge::payload_bytes` | struct_field | Stores the payload size for `PayloadTooLarge`, in bytes. | `src/runtime/signal/edge.rs:418` |
| sym-a02f3d34dba068ab258a | `TypedEdgePublishError::RequiredBranchFull::branch_index` | struct_field | Identifies the branch index position within `RequiredBranchFull`. | `src/runtime/signal/edge.rs:422` |
| sym-00f26bfacd50c4178067 | `TypedEdgePublishReport::delivered_total` | struct_field | Counts the total number of delivered observed by `TypedEdgePublishReport`. | `src/runtime/signal/edge.rs:381` |
| sym-5e4eeeb8900d8c98e47b | `TypedEdgePublishReport::dropped_total` | struct_field | Counts the total number of dropped observed by `TypedEdgePublishReport`. | `src/runtime/signal/edge.rs:382` |
| sym-c565f2280bbf46abf495 | `TypedStreamError::AmbiguousPort::direction` | struct_field | Records the direction selected for `AmbiguousPort`. | `src/session/declaration/typed_stream.rs:203` |
| sym-5aabd294beb68e46c916 | `TypedStreamError::InputSignalMismatch::port` | struct_field | References the port participating in `InputSignalMismatch`. | `src/session/declaration/typed_stream.rs:205` |
| sym-7282bafc5270e49fc383 | `TypedStreamError::MissingPort::direction` | struct_field | Records the direction selected for `MissingPort`. | `src/session/declaration/typed_stream.rs:201` |
| sym-7281926eaed5f200adbb | `TypedStreamError::OperatorIdentityMismatch::declaration` | struct_field | Stores the declaration text reported by `OperatorIdentityMismatch`. | `src/session/declaration/typed_stream.rs:192` |
| sym-84f363c0fa205ff767a9 | `TypedStreamError::OperatorIdentityMismatch::manifest` | struct_field | Stores the manifest text reported by `OperatorIdentityMismatch`. | `src/session/declaration/typed_stream.rs:193` |
| sym-722f480edde036fa8831 | `TypedStreamError::OutputSignalMismatch::port` | struct_field | References the port participating in `OutputSignalMismatch`. | `src/session/declaration/typed_stream.rs:207` |
| sym-c49327cbe3dc16eabc36 | `TypedStreamError::UnknownPort::direction` | struct_field | Records the direction selected for `UnknownPort`. | `src/session/declaration/typed_stream.rs:197` |
| sym-1486107fdeaa7707cfdf | `TypedStreamError::UnknownPort::port` | struct_field | References the port participating in `UnknownPort`. | `src/session/declaration/typed_stream.rs:198` |
| sym-46bd88499244eadd7644 | `audio::AudioFrameBuildError::MisalignedSamples::channels` | struct_field | Contains the channels owned or reported by `MisalignedSamples`. | `src/frame/audio.rs:57` |
| sym-c152aa4eaf517b7e0f38 | `audio::AudioFrameBuildError::MisalignedSamples::samples` | struct_field | Contains the samples owned or reported by `MisalignedSamples`. | `src/frame/audio.rs:57` |
| sym-d03c8216ec1500493017 | `audio::SampleSpec::channels` | struct_field | Contains the channels owned or reported by `SampleSpec`. | `src/frame/audio.rs:20` |
| sym-63a8cb9ef418fdd86d75 | `audio::SampleSpec::format` | struct_field | Records the format selected for `SampleSpec`. | `src/frame/audio.rs:21` |
| sym-aa5c5c984c98f972616a | `audio::SampleSpec::sample_rate_hz` | struct_field | Stores the sample rate value for `SampleSpec`, in hertz. | `src/frame/audio.rs:19` |
| sym-d3ef57efc035e370fdaa | `authorization::CaptureAuthorizationSnapshot::application_policy` | struct_field | Reports the application-level capture policy observed by `CaptureAuthorizationSnapshot`. | `src/capture/authorization.rs:20` |
| sym-2fd0c2d149e69098b259 | `authorization::CaptureAuthorizationSnapshot::capability` | struct_field | Stores the capability as a `CaptureCapabilityState` value in `CaptureAuthorizationSnapshot`. | `src/capture/authorization.rs:18` |
| sym-0110608b4d90ebfbdcfa | `authorization::CaptureAuthorizationSnapshot::capture_scope` | struct_field | Declares the exact resource scope authorized by `CaptureAuthorizationSnapshot`. | `src/capture/authorization.rs:22` |
| sym-bad5218134c7bab1a456 | `authorization::CaptureAuthorizationSnapshot::identity_strength` | struct_field | Reports how strongly the selected source identity is bound in `CaptureAuthorizationSnapshot`. | `src/capture/authorization.rs:23` |
| sym-6786cd68c1708d931a04 | `authorization::CaptureAuthorizationSnapshot::observed_at_ns` | struct_field | Stores the observed at value for `CaptureAuthorizationSnapshot`, in nanoseconds. | `src/capture/authorization.rs:25` |
| sym-8881e3efc02b5a8e9f82 | `authorization::CaptureAuthorizationSnapshot::open_outcome` | struct_field | Reports whether opening capture is allowed, denied, or requires setup in `CaptureAuthorizationSnapshot`. | `src/capture/authorization.rs:26` |
| sym-6546ea14b8dd5c616bc6 | `authorization::CaptureAuthorizationSnapshot::os_permission` | struct_field | Reports the operating-system permission state observed by `CaptureAuthorizationSnapshot`. | `src/capture/authorization.rs:19` |
| sym-324bf896e839322c55a1 | `authorization::CaptureAuthorizationSnapshot::permission_epoch` | struct_field | Identifies the permission-observation generation attached to `CaptureAuthorizationSnapshot`. | `src/capture/authorization.rs:24` |
| sym-3af6c6b819f8a1ecb411 | `authorization::CaptureAuthorizationSnapshot::session_grant` | struct_field | Reports whether the Session-specific capture grant is present for `CaptureAuthorizationSnapshot`. | `src/capture/authorization.rs:21` |
| sym-f14eb2b2d9294d4668da | `authorization::CaptureError::BackendSetupRequired::action` | struct_field | Describes the corrective action reported with `BackendSetupRequired`. | `src/capture/authorization.rs:298` |
| sym-3bac54bd2f14fd0f2865 | `authorization::CaptureError::BackendSetupRequired::backend` | struct_field | Stores the backend component of `BackendSetupRequired`. | `src/capture/authorization.rs:297` |
| sym-b0dc71598094f0792079 | `authorization::CaptureError::BackendStatus::operation` | struct_field | Names the operation that produced `BackendStatus`. | `src/capture/authorization.rs:304` |
| sym-5a64301c9c536c12387d | `authorization::CaptureError::BackendStatus::status_code` | struct_field | Preserves the platform or protocol status code reported by `BackendStatus`. | `src/capture/authorization.rs:305` |
| sym-1428c658d60dfc6b7829 | `authorization::CaptureError::CaptureWorkerPanicked::worker` | struct_field | Stores the worker component of `CaptureWorkerPanicked`. | `src/capture/authorization.rs:316` |
| sym-1c348e186990e94407cf | `authorization::CaptureError::PermissionDenied::operation` | struct_field | Names the operation that produced `PermissionDenied`. | `src/capture/authorization.rs:301` |
| sym-239abfa4cef9a73b47de | `authorization::CaptureError::SourceUnavailable::stable_key` | struct_field | Stores the stable source key associated with `SourceUnavailable`. | `src/capture/authorization.rs:308` |
| sym-beea7f3e3748e7ba0ab6 | `authorization::CapturePermissionTransition::current` | struct_field | Stores the current as a `PermissionObservation` value in `CapturePermissionTransition`. | `src/capture/authorization.rs:171` |
| sym-e0c5160d0b04c368e706 | `authorization::CapturePermissionTransition::kind` | struct_field | Records the kind selected for `CapturePermissionTransition`. | `src/capture/authorization.rs:169` |
| sym-05ca6b08d4055835136e | `authorization::CapturePermissionTransition::permission_epoch` | struct_field | Identifies the permission-observation generation attached to `CapturePermissionTransition`. | `src/capture/authorization.rs:172` |
| sym-99ccabb5727887f5bad9 | `authorization::CapturePermissionTransition::previous` | struct_field | Contains the previous owned or reported by `CapturePermissionTransition`. | `src/capture/authorization.rs:170` |
| sym-2efcfc236510e43ce157 | `authorization::CaptureScope::ExactApplication::stable_id` | struct_field | Identifies the stable identifier recorded by `ExactApplication`. | `src/capture/authorization.rs:249` |
| sym-d67f47bb4cc6ab2218fe | `authorization::CaptureScope::ExactInputDevice::stable_id` | struct_field | Identifies the stable identifier recorded by `ExactInputDevice`. | `src/capture/authorization.rs:250` |
| sym-e5dd0f97470b834ad39f | `authorization::CaptureScope::ExactOutputDevice::stable_id` | struct_field | Identifies the stable identifier recorded by `ExactOutputDevice`. | `src/capture/authorization.rs:251` |
| sym-bc2f6d9785044dca4c28 | `events::CaptureRuntimeFailure::error_class` | struct_field | Contains the error class owned or reported by `CaptureRuntimeFailure`. | `src/capture/events.rs:49` |
| sym-d8b03278d57ae8b670e5 | `events::CaptureRuntimeFailure::operation` | struct_field | Names the operation that produced `CaptureRuntimeFailure`. | `src/capture/events.rs:48` |
| sym-51ed6637889912ebd5f7 | `events::CaptureRuntimeFailureClass::BackendClass::class` | struct_field | Contains the class owned or reported by `BackendClass`. | `src/capture/events.rs:43` |
| sym-32d556ad550751c4f609 | `events::CaptureRuntimeFailureClass::PlatformStatus::status_code` | struct_field | Preserves the platform or protocol status code reported by `PlatformStatus`. | `src/capture/events.rs:42` |
| sym-0f9091de7a694dcf4118 | `events::SourceRuntimeEvent::BackendFailure::failure` | struct_field | Carries the failure reported by `BackendFailure`. | `src/capture/events.rs:63` |
| sym-718021fa09a4e5f78448 | `events::SourceRuntimeEvent::BackendFailure::generation` | struct_field | Identifies the generation of the resource represented by `BackendFailure`. | `src/capture/events.rs:62` |
| sym-b6cac24daa0bbd0f164b | `events::SourceRuntimeEvent::BackendFailure::stable_id` | struct_field | Identifies the stable identifier recorded by `BackendFailure`. | `src/capture/events.rs:61` |
| sym-958fdd7dfee0e0ba6bc2 | `events::SourceRuntimeEvent::SourceUnavailable::failure` | struct_field | Carries the failure reported by `SourceUnavailable`. | `src/capture/events.rs:58` |
| sym-9093ae9b0e9990551709 | `events::SourceRuntimeEvent::SourceUnavailable::generation` | struct_field | Identifies the generation of the resource represented by `SourceUnavailable`. | `src/capture/events.rs:56` |
| sym-eb5d2e8d0f6f67c1b6ae | `events::SourceRuntimeEvent::SourceUnavailable::recovery_requirement` | struct_field | Declares the recovery action required after the source event in `SourceUnavailable`. | `src/capture/events.rs:57` |
| sym-914c7166fd31047a7512 | `events::SourceRuntimeEvent::SourceUnavailable::stable_id` | struct_field | Identifies the stable identifier recorded by `SourceUnavailable`. | `src/capture/events.rs:55` |
| sym-31eb70081b1d8cbfeddc | `events::SourceRuntimeEventObservations::capacity_event_count` | struct_field | Sets the capacity event count available to `SourceRuntimeEventObservations`. | `src/capture/events.rs:112` |
| sym-213aab1d92988b9d1e7a | `events::SourceRuntimeEventObservations::depth_events` | struct_field | Reports the depth events observed by `SourceRuntimeEventObservations`. | `src/capture/events.rs:115` |
| sym-9f13913903153e3cc97b | `events::SourceRuntimeEventObservations::depth_owned_bytes` | struct_field | Stores the depth owned size for `SourceRuntimeEventObservations`, in bytes. | `src/capture/events.rs:116` |
| sym-e7a002fd63b713511326 | `events::SourceRuntimeEventObservations::events_dropped_oversized_total` | struct_field | Counts the total number of events dropped oversized observed by `SourceRuntimeEventObservations`. | `src/capture/events.rs:120` |
| sym-2c930f0bab38446e6cf4 | `events::SourceRuntimeEventObservations::events_dropped_total` | struct_field | Counts the total number of events dropped observed by `SourceRuntimeEventObservations`. | `src/capture/events.rs:119` |
| sym-26bd5b31a552d7ad0324 | `events::SourceRuntimeEventObservations::events_enqueued_total` | struct_field | Counts the total number of events enqueued observed by `SourceRuntimeEventObservations`. | `src/capture/events.rs:118` |
| sym-1cf87d35b7171c68647c | `events::SourceRuntimeEventObservations::maximum_buffered_owned_bytes` | struct_field | Stores the maximum buffered owned size for `SourceRuntimeEventObservations`, in bytes. | `src/capture/events.rs:114` |
| sym-20ce8fdb10817b100b20 | `events::SourceRuntimeEventObservations::maximum_event_owned_bytes` | struct_field | Stores the maximum event owned size for `SourceRuntimeEventObservations`, in bytes. | `src/capture/events.rs:113` |
| sym-8784bcf4ca188444a359 | `events::SourceRuntimeEventObservations::peak_depth_owned_bytes` | struct_field | Stores the peak depth owned size for `SourceRuntimeEventObservations`, in bytes. | `src/capture/events.rs:117` |
| sym-16305c01737e7713d5f9 | `identity::CaptureSource::app_id` | struct_field | Identifies the app identifier recorded by `CaptureSource`. | `src/capture/identity.rs:86` |
| sym-d225360a6ae1b977abb3 | `identity::CaptureSource::channels` | struct_field | Contains the channels owned or reported by `CaptureSource`. | `src/capture/identity.rs:90` |
| sym-3df6f05fe270375921df | `identity::CaptureSource::device_uid` | struct_field | Stores the device uid component of `CaptureSource`. | `src/capture/identity.rs:87` |
| sym-e4408de392f9a76613fe | `identity::CaptureSource::name` | struct_field | Stores the human-readable name used to identify `CaptureSource`. | `src/capture/identity.rs:84` |
| sym-82f007255ac4c63383e6 | `identity::CaptureSource::process_id` | struct_field | Identifies the process identifier recorded by `CaptureSource`. | `src/capture/identity.rs:85` |
| sym-4ea89de58375f6de493e | `identity::CaptureSource::sample_rate_hz` | struct_field | Stores the sample rate value for `CaptureSource`, in hertz. | `src/capture/identity.rs:89` |
| sym-6a93020e6ac8e5b5a388 | `identity::CaptureSource::stable_id` | struct_field | Identifies the stable identifier recorded by `CaptureSource`. | `src/capture/identity.rs:83` |
| sym-8fd041f3afc8e178a998 | `identity::CaptureSource::state` | struct_field | Records the state selected for `CaptureSource`. | `src/capture/identity.rs:88` |
| sym-fbb9a4a1e0b9a37de640 | `identity::StableSourceId::kind` | struct_field | Records the kind selected for `StableSourceId`. | `src/capture/identity.rs:28` |
| sym-2f193a52c138d27634e8 | `identity::StableSourceId::platform` | struct_field | Stores the platform as a `Platform` value in `StableSourceId`. | `src/capture/identity.rs:27` |
| sym-56a5f906e53f087b807f | `identity::StableSourceId::stable_key` | struct_field | Stores the stable source key associated with `StableSourceId`. | `src/capture/identity.rs:29` |
| sym-77c4144009ab0b7314ea | `lifecycle_registry::SourceGenerationTransition::Disappeared::generation` | struct_field | Identifies the generation of the resource represented by `Disappeared`. | `src/capture/lifecycle_registry.rs:11` |
| sym-09a3cdd7140fe84d7fb0 | `lifecycle_registry::SourceGenerationTransition::Disappeared::stable_id` | struct_field | Identifies the stable identifier recorded by `Disappeared`. | `src/capture/lifecycle_registry.rs:10` |
| sym-828ed5429fb6a2e1f8ed | `lifecycle_registry::SourceGenerationTransition::Reappeared::generation` | struct_field | Identifies the generation of the resource represented by `Reappeared`. | `src/capture/lifecycle_registry.rs:16` |
| sym-089a0fcbac44a118ef59 | `lifecycle_registry::SourceGenerationTransition::Reappeared::previous_generation` | struct_field | Identifies the generation that preceded the transition recorded by `Reappeared`. | `src/capture/lifecycle_registry.rs:15` |
| sym-5177764d9a2229260068 | `lifecycle_registry::SourceGenerationTransition::Reappeared::stable_id` | struct_field | Identifies the stable identifier recorded by `Reappeared`. | `src/capture/lifecycle_registry.rs:14` |
| sym-24113584973f5dca5b04 | `pocketstation::SessionEndpointError::DuplicateNodeTypeId::node_type_id` | struct_field | Identifies the node type identifier recorded by `DuplicateNodeTypeId`. | `src/lib.rs:1073` |
| sym-79fb6c5308003cba1893 | `pocketstation::SessionEndpointError::DuplicateOperatorId::operator_id` | struct_field | Identifies the operator identifier recorded by `DuplicateOperatorId`. | `src/lib.rs:1071` |
| sym-d346e6ee8ea56058d269 | `pocketstation::conformance::ExtensionConformanceReport::endpoint_finalized_total` | struct_field | Counts the total number of endpoint finalized observed by `ExtensionConformanceReport`. | `src/conformance.rs:593` |
| sym-8d13a1ced4ace229a7ed | `pocketstation::conformance::ExtensionConformanceReport::endpoint_id` | struct_field | Identifies the endpoint identifier recorded by `ExtensionConformanceReport`. | `src/conformance.rs:578` |
| sym-7c0ff1aa93c07b69f624 | `pocketstation::conformance::ExtensionConformanceReport::endpoint_prepared_total` | struct_field | Counts the total number of endpoint prepared observed by `ExtensionConformanceReport`. | `src/conformance.rs:590` |
| sym-2e6130c4c19f6819780c | `pocketstation::conformance::ExtensionConformanceReport::endpoint_received_total` | struct_field | Counts the total number of endpoint received observed by `ExtensionConformanceReport`. | `src/conformance.rs:591` |
| sym-4be84b6192eae5a5d2e0 | `pocketstation::conformance::ExtensionConformanceReport::endpoint_stopped_total` | struct_field | Counts the total number of endpoint stopped observed by `ExtensionConformanceReport`. | `src/conformance.rs:592` |
| sym-e2ae487172c89ebd4af4 | `pocketstation::conformance::ExtensionConformanceReport::failure_requested` | struct_field | Reports whether failure is requested for `ExtensionConformanceReport`. | `src/conformance.rs:581` |
| sym-e48035b79f0fa7cc46ec | `pocketstation::conformance::ExtensionConformanceReport::input_payload` | struct_field | References the input payload participating in `ExtensionConformanceReport`. | `src/conformance.rs:579` |
| sym-d6994b0d5681562c6b58 | `pocketstation::conformance::ExtensionConformanceReport::lifecycle_event_total` | struct_field | Counts the total number of lifecycle event observed by `ExtensionConformanceReport`. | `src/conformance.rs:594` |
| sym-5d163508589863468ba0 | `pocketstation::conformance::ExtensionConformanceReport::maximum_buffered_payload_bytes` | struct_field | Stores the maximum buffered payload size for `ExtensionConformanceReport`, in bytes. | `src/conformance.rs:601` |
| sym-ab1beb2518960fb0169d | `pocketstation::conformance::ExtensionConformanceReport::operator_closed_total` | struct_field | Counts the total number of operator closed observed by `ExtensionConformanceReport`. | `src/conformance.rs:589` |
| sym-4e20a2d4307f0fd9f01c | `pocketstation::conformance::ExtensionConformanceReport::operator_failure_total` | struct_field | Counts the total number of operator failure observed by `ExtensionConformanceReport`. | `src/conformance.rs:588` |
| sym-5db4329a547a0e01f4d2 | `pocketstation::conformance::ExtensionConformanceReport::operator_id` | struct_field | Identifies the operator identifier recorded by `ExtensionConformanceReport`. | `src/conformance.rs:577` |
| sym-f2dd073467613a10c19f | `pocketstation::conformance::ExtensionConformanceReport::operator_output_total` | struct_field | Counts the total number of operator output observed by `ExtensionConformanceReport`. | `src/conformance.rs:587` |
| sym-8db395541f450490e6dc | `pocketstation::conformance::ExtensionConformanceReport::operator_prepared_total` | struct_field | Counts the total number of operator prepared observed by `ExtensionConformanceReport`. | `src/conformance.rs:585` |
| sym-fe9e694bc57c96f2702c | `pocketstation::conformance::ExtensionConformanceReport::operator_processed_total` | struct_field | Counts the total number of operator processed observed by `ExtensionConformanceReport`. | `src/conformance.rs:586` |
| sym-f6e9c2994b004d45c50b | `pocketstation::conformance::ExtensionConformanceReport::output_payload` | struct_field | References the output payload participating in `ExtensionConformanceReport`. | `src/conformance.rs:580` |
| sym-905e2189b4173af0fd44 | `pocketstation::conformance::ExtensionConformanceReport::queue_capacity_signals` | struct_field | Sets the queue capacity signals available to `ExtensionConformanceReport`. | `src/conformance.rs:596` |
| sym-3d288f50cd06ff52ca37 | `pocketstation::conformance::ExtensionConformanceReport::queue_peak_signals` | struct_field | Reports the queue peak signals observed by `ExtensionConformanceReport`. | `src/conformance.rs:597` |
| sym-317cb6e624a68c6b2fa1 | `pocketstation::conformance::ExtensionConformanceReport::role_id` | struct_field | Identifies the role identifier recorded by `ExtensionConformanceReport`. | `src/conformance.rs:575` |
| sym-47387418d18cac0c1219 | `pocketstation::conformance::ExtensionConformanceReport::route_capacity_signals` | struct_field | Sets the route capacity signals available to `ExtensionConformanceReport`. | `src/conformance.rs:598` |
| sym-2951d06d6fc58fccd08d | `pocketstation::conformance::ExtensionConformanceReport::route_delivered_total` | struct_field | Counts the total number of route delivered observed by `ExtensionConformanceReport`. | `src/conformance.rs:600` |
| sym-ee5dd7329383fe855688 | `pocketstation::conformance::ExtensionConformanceReport::route_peak_signals` | struct_field | Reports the route peak signals observed by `ExtensionConformanceReport`. | `src/conformance.rs:599` |
| sym-56b96c6ba2929767c9c6 | `pocketstation::conformance::ExtensionConformanceReport::schema_id` | struct_field | Identifies the schema identifier recorded by `ExtensionConformanceReport`. | `src/conformance.rs:574` |
| sym-2e638a67449a41279130 | `pocketstation::conformance::ExtensionConformanceReport::signal_id` | struct_field | Identifies the signal identifier recorded by `ExtensionConformanceReport`. | `src/conformance.rs:573` |
| sym-82b040fa7ed8e0bca388 | `pocketstation::conformance::ExtensionConformanceReport::source_closed_total` | struct_field | Counts the total number of source closed observed by `ExtensionConformanceReport`. | `src/conformance.rs:584` |
| sym-e5a433995b3b7734e228 | `pocketstation::conformance::ExtensionConformanceReport::source_emitted_total` | struct_field | Counts the total number of source emitted observed by `ExtensionConformanceReport`. | `src/conformance.rs:583` |
| sym-944d88263f96eada7dec | `pocketstation::conformance::ExtensionConformanceReport::source_prepared_total` | struct_field | Counts the total number of source prepared observed by `ExtensionConformanceReport`. | `src/conformance.rs:582` |
| sym-9aab05b59c00aed3419d | `pocketstation::conformance::ExtensionConformanceReport::source_type_id` | struct_field | Identifies the source type identifier recorded by `ExtensionConformanceReport`. | `src/conformance.rs:576` |
| sym-f25df70430d170ed5d26 | `pocketstation::conformance::ExtensionConformanceReport::stop_success` | struct_field | Contains the stop success owned or reported by `ExtensionConformanceReport`. | `src/conformance.rs:602` |
| sym-281d4d8f3acec829b42f | `pocketstation::conformance::ExtensionConformanceReport::terminal_event_total` | struct_field | Counts the total number of terminal event observed by `ExtensionConformanceReport`. | `src/conformance.rs:595` |
| sym-b6ef30d262e385fa8459 | `pocketstation::connector::ConnectorDeclarationError::WrongSession::registered` | struct_field | Stores the registered as a `SessionId` value in `WrongSession`. | `src/connector/mod.rs:236` |
| sym-0d9c40408e097e6c9eb3 | `pocketstation::connector::ConnectorDeclarationError::WrongSession::requested` | struct_field | Stores the requested as a `SessionId` value in `WrongSession`. | `src/connector/mod.rs:237` |
| sym-b9c47bf9d684d27871ea | `pocketstation::connector::ConnectorObservationLookupError::WrongSession::registered` | struct_field | Stores the registered as a `SessionId` value in `WrongSession`. | `src/connector/mod.rs:249` |
| sym-d3562b133871922e82cf | `pocketstation::connector::ConnectorObservationLookupError::WrongSession::requested` | struct_field | Stores the requested as a `SessionId` value in `WrongSession`. | `src/connector/mod.rs:250` |
| sym-d4623f39a117cbed7815 | `pool::AudioBufferWriteError::CapacityExceeded::capacity_samples` | struct_field | Sets the capacity samples available to `CapacityExceeded`. | `src/frame/pool.rs:20` |
| sym-7fb9e3e159af61c62ea6 | `pool::AudioBufferWriteError::CapacityExceeded::requested_samples` | struct_field | Contains the requested samples owned or reported by `CapacityExceeded`. | `src/frame/pool.rs:19` |
| sym-c71fbab34189dda90085 | `selection::CaptureMode::ExactApplication::process_id` | struct_field | Identifies the process identifier recorded by `ExactApplication`. | `src/capture/selection.rs:22` |
| sym-408321762eba07162e31 | `selection::CaptureMode::ExactApplication::stable_id` | struct_field | Identifies the stable identifier recorded by `ExactApplication`. | `src/capture/selection.rs:23` |
| sym-6447901cd24495ba2dfb | `selection::CaptureMode::ExactApplicationStable::stable_id` | struct_field | Identifies the stable identifier recorded by `ExactApplicationStable`. | `src/capture/selection.rs:26` |
| sym-fb9e4310f58f26299c89 | `pocketstation::capture::capture_owner::ActiveCaptureBackend` | trait | Native capture resources owned for exactly one active capture. | `src/capture/capture_owner.rs:100` |
| sym-4da42fff88ec45e5e56b | `pocketstation::capture::capture_owner::CallbackCaptureBackend` | trait | Platform-neutral prepare/open boundary for callback-oriented capture. | `src/capture/capture_owner.rs:83` |
| sym-a354a1be3ec1974fc5fe | `pocketstation::capture::capture_owner::PreparedCaptureBackend` | trait | Backend state that has passed validation but has not started delivery. | `src/capture/capture_owner.rs:88` |
| sym-b0869d923b3fd79406d8 | `pocketstation::capture::query::SourceProvider` | trait | Implement this trait to provide source behavior to PocketStation; its methods define the preparation and runtime contract. | `src/capture/query.rs:48` |
| sym-ef774811a957059cea80 | `pocketstation::connector::worker::ConnectorFactory` | trait | Implement this trait to provide connector behavior to PocketStation; its methods define the preparation and runtime contract. | `src/connector/worker/mod.rs:17` |
| sym-d5b0bf5fc25d3e2230c9 | `pocketstation::connector::worker::ConnectorWorker` | trait | Implement this trait to provide connector worker behavior to PocketStation; its methods define the preparation and runtime contract. | `src/connector/worker/mod.rs:32` |
| sym-2c2a9d5455c4d0a0a073 | `pocketstation::connector::worker::driver::ConnectorDriver` | trait | Provider-specific behavior executed on Core's bounded connector worker. | `src/connector/worker/driver.rs:92` |
| sym-7047fbea68de3230c646 | `pocketstation::connector::worker::driver::ConnectorDriverFactory` | trait | Prepares provider state while Core retains receiver and lifecycle authority. | `src/connector/worker/driver.rs:123` |
| sym-c47143b8b73e85dd114d | `pocketstation::endpoint::contract::EndpointDriverFactory` | trait | Implement this trait to provide endpoint behavior to PocketStation; its methods define the preparation and runtime contract. | `src/endpoint/contract.rs:262` |
| sym-5bb941fb228e70e5ee32 | `pocketstation::endpoint::runtime::PreparedEndpointDriver` | trait | Prepared endpoint resources that have not started consuming their edge. | `src/endpoint/runtime.rs:318` |
| sym-d5be5ddfe876105c0106 | `pocketstation::endpoint::runtime::RunningEndpointDriver` | trait | Active endpoint resources owned until finalization. | `src/endpoint/runtime.rs:336` |
| sym-306915dbf407b9290a9e | `pocketstation::graph::registry::NodeDefinition` | trait | Implement this trait to provide node definition behavior to PocketStation; its methods define the preparation and runtime contract. | `src/graph/registry.rs:21` |
| sym-9f49a447df3eea019bd7 | `pocketstation::graph::registry::NodeFactory` | trait | Implement this trait to provide node behavior to PocketStation; its methods define the preparation and runtime contract. | `src/graph/registry.rs:11` |
| sym-8d99d9a87246f8b8c15f | `pocketstation::graph::runtime_node::RuntimeNode` | trait | Realtime invariant: for nodes whose ExecutionClass::is_realtime is true, process() must stay alloc-free, lock-free, log-free, and blocking-free (LAW 15). All working state is sized once in prepare() and reused for the lifetime of the node. | `src/graph/runtime_node.rs:7` |
| sym-07be678de424b7075252 | `pocketstation::graph::signal::operator::AsyncNode` | trait | Async operator contract for model, connector, transport, and control-plane work. | `src/graph/signal/operator.rs:13` |
| sym-ce77d1d902e3ee004e28 | `pocketstation::graph::signal::operator::AsyncOperatorFactory` | trait | Implement this trait to provide async operator behavior to PocketStation; its methods define the preparation and runtime contract. | `src/graph/signal/operator.rs:368` |
| sym-c3b12a9b3547711a3a04 | `pocketstation::session::declaration::typed_stream::StreamSignal` | trait | Compile-time marker supplied by an SDK or external package. | `src/session/declaration/typed_stream.rs:15` |
| sym-e3af4749be7cfa836972 | `pocketstation::session::extensions::source::SourceDriver` | trait | Implement this trait to provide source behavior to PocketStation; its methods define the preparation and runtime contract. | `src/session/extensions/source.rs:267` |
| sym-e1acd32a4d4b9349ed5a | `pocketstation::session::extensions::source::SourceFactory` | trait | Implement this trait to provide source behavior to PocketStation; its methods define the preparation and runtime contract. | `src/session/extensions/source.rs:276` |
| sym-aeeaa5604243332b5394 | `pocketstation::abi::executable_extension::PksExtensionAcquireRegistrationCallback` | type_alias | Defines the optional C callback used to acquire an extension registration; pointer validity and ownership follow the extension ABI contract. | `src/abi/executable_extension.rs:110` |
| sym-086627215efa123e182d | `pocketstation::abi::executable_extension::PksExtensionCreateCallback` | type_alias | Defines the optional C callback used to create an extension instance; pointer validity and ownership follow the extension ABI contract. | `src/abi/executable_extension.rs:56` |
| sym-32efbef473a9a5697a1c | `pocketstation::abi::executable_extension::PksExtensionDestroyCallback` | type_alias | Defines the optional C callback used to destroy extension-owned context; pointer validity and ownership follow the extension ABI contract. | `src/abi/executable_extension.rs:87` |
| sym-03cc3c3d4a79421ab83c | `pocketstation::abi::executable_extension::PksExtensionEndpointConsumeCallback` | type_alias | Defines the optional C callback used to consume an endpoint input; pointer validity and ownership follow the extension ABI contract. | `src/abi/executable_extension.rs:77` |
| sym-09f27cd611795c65bbc8 | `pocketstation::abi::executable_extension::PksExtensionFinishCallback` | type_alias | Defines the optional C callback used to finish extension work; pointer validity and ownership follow the extension ABI contract. | `src/abi/executable_extension.rs:85` |
| sym-497f48c6c93d03e14de7 | `pocketstation::abi::executable_extension::PksExtensionLibraryEntrypoint` | type_alias | Exposes `PksSessionStatus` as the public `PksExtensionLibraryEntrypoint` alias at this API boundary. | `src/abi/executable_extension.rs:133` |
| sym-e3262cbc0403e5714c35 | `pocketstation::abi::executable_extension::PksExtensionOperatorProcessCallback` | type_alias | Defines the optional C callback used to process an operator input; pointer validity and ownership follow the extension ABI contract. | `src/abi/executable_extension.rs:70` |
| sym-98dd08100b2055a1b49b | `pocketstation::abi::executable_extension::PksExtensionPrepareCallback` | type_alias | Defines the optional C callback used to prepare an extension instance; pointer validity and ownership follow the extension ABI contract. | `src/abi/executable_extension.rs:48` |
| sym-cf706ae4f39c2bc1b613 | `pocketstation::abi::executable_extension::PksExtensionSourceNextCallback` | type_alias | Defines the optional C callback used to produce the next source signal; pointer validity and ownership follow the extension ABI contract. | `src/abi/executable_extension.rs:63` |
| sym-8ed828f9fa01d64e22e6 | `pocketstation::abi::executable_extension::PksExtensionStopCallback` | type_alias | Defines the optional C callback used to request an extension instance to stop; pointer validity and ownership follow the extension ABI contract. | `src/abi/executable_extension.rs:83` |
| sym-802847170fd9ec2831b4 | `pocketstation::abi::executable_extension::PksExtensionValidateConfigurationCallback` | type_alias | Defines the optional C callback used to validate extension configuration; pointer validity and ownership follow the extension ABI contract. | `src/abi/executable_extension.rs:50` |
| sym-0af53956a40cf8c1bdd8 | `pocketstation::graph::signal::preparation::AsyncNodeFuture` | type_alias | Names the future returned by async node operations. | `src/graph/signal/preparation.rs:9` |
| sym-bb4f97d3a463910cfd26 | `pocketstation::graph::signal::preparation::AsyncOperatorEdgePrepareContext` | type_alias | Exact bounded graph edge supplied to an asynchronous Operator at prepare time. | `src/graph/signal/preparation.rs:18` |
| sym-efa27deb4c4274ec4086 | `pocketstation::runtime::signal::edge::TypedEdgeObservationHandle` | type_alias | Exposes `SignalEdgeObservationHandle` as the public `TypedEdgeObservationHandle` alias at this API boundary. | `src/runtime/signal/edge.rs:245` |
| sym-4138655e07c2164fb614 | `pocketstation::runtime::signal::edge::TypedEdgeObservations` | type_alias | Exposes `SignalEdgeObservations` as the public `TypedEdgeObservations` alias at this API boundary. | `src/runtime/signal/edge.rs:244` |
| sym-468a9123ecb70b012293 | `pocketstation::runtime::signal::edge::TypedEdgeReceiver` | type_alias | Exposes `SignalEdgeReceiver` as the public `TypedEdgeReceiver` alias at this API boundary. | `src/runtime/signal/edge.rs:243` |
| sym-d3307f47d46fe23350af | `pocketstation::runtime::signal::io::AsyncOperatorOutput` | type_alias | Exposes `TypedEdgeReceiver` as the public `AsyncOperatorOutput` alias at this API boundary. | `src/runtime/signal/io.rs:74` |
| sym-2bb036185e815e39dcce | `pocketstation::runtime::signal::io::AsyncOperatorOutputBranchSpec` | type_alias | Exposes `TypedEdgeBranchSpec` as the public `AsyncOperatorOutputBranchSpec` alias at this API boundary. | `src/runtime/signal/io.rs:76` |
| sym-0e603da19374fa7f0250 | `pocketstation::runtime::signal::io::AsyncOperatorOutputObservationHandle` | type_alias | Exposes `TypedEdgeObservationHandle` as the public `AsyncOperatorOutputObservationHandle` alias at this API boundary. | `src/runtime/signal/io.rs:75` |
| sym-1c808b224809cebdfbcf | `pocketstation::runtime::signal::io::AsyncOperatorOutputObservations` | type_alias | Exposes `TypedEdgeObservations` as the public `AsyncOperatorOutputObservations` alias at this API boundary. | `src/runtime/signal/io.rs:77` |
| sym-b5fa983882995e5f6f6a | `pocketstation::session::declaration::spec::OperatorSpec` | type_alias | Exposes `OperatorInstanceSpec` as the public `OperatorSpec` alias at this API boundary. | `src/session/declaration/spec.rs:274` |
| sym-5ced9a66919382178a7f | `pocketstation::SessionCancelDisposition::AlreadyStopped` | variant | Indicates that the operation had already stopped. | `src/lib.rs:1119` |
| sym-886228c12cb2e85ed6c9 | `pocketstation::SessionCancelDisposition::Cancelled` | variant | Indicates that the operation was cancelled. | `src/lib.rs:1118` |
| sym-6c01af2bd5c1bbf7f4d9 | `pocketstation::SessionEndpointError::DuplicateNodeTypeId` | variant | Reports that node type identifier duplicates an existing declaration or record. | `src/lib.rs:1073` |
| sym-75041a121ce9f90e99f0 | `pocketstation::SessionEndpointError::DuplicateOperatorId` | variant | Reports that operator identifier duplicates an existing declaration or record. | `src/lib.rs:1071` |
| sym-b7b13d9f4576ed4ab491 | `pocketstation::SessionEndpointError::RegistrationStateUnavailable` | variant | Reports that registration state is unavailable. | `src/lib.rs:1069` |
| sym-aa21ef7c86595f95bd43 | `pocketstation::SessionOperatorError::RegistrationStateUnavailable` | variant | Reports that registration state is unavailable. | `src/lib.rs:1085` |
| sym-13b3842f41f66723178b | `pocketstation::SessionRuntimeError::MissingMetricsSnapshot` | variant | Reports that the required metrics snapshot is missing. | `src/lib.rs:1107` |
| sym-cbd193553e98cf36c60b | `pocketstation::SessionSidecarError::RegistrationStateUnavailable` | variant | Reports that registration state is unavailable. | `src/lib.rs:1079` |
| sym-c03fa48bd294fe211e39 | `pocketstation::SessionSourceError::RegistrationStateUnavailable` | variant | Reports that registration state is unavailable. | `src/lib.rs:1091` |
| sym-670152a45f71ed6a648c | `pocketstation::SessionStartErrorKind::Cancelled` | variant | Indicates that the operation was cancelled. | `src/lib.rs:1098` |
| sym-cee2d0540ec19a42fb06 | `pocketstation::SessionStartErrorKind::Engine` | variant | Classifies a Session start failure attributed to engine. | `src/lib.rs:1097` |
| sym-a1426677d52946d8a970 | `pocketstation::SessionStartErrorKind::Host` | variant | Classifies a Session start failure attributed to host. | `src/lib.rs:1096` |
| sym-a4e0907fc533f964b908 | `pocketstation::SessionStartErrorKind::InvalidSelector` | variant | Classifies a Session start failure attributed to invalid selector. | `src/lib.rs:1099` |
| sym-d13f202a2b50039e9f4e | `pocketstation::SessionStartErrorKind::Invariant` | variant | Classifies a Session start failure attributed to invariant. | `src/lib.rs:1101` |
| sym-bbd5e5215e1d32be9920 | `pocketstation::SessionStartErrorKind::MissingRecordingConfiguration` | variant | Classifies a Session start failure attributed to missing recording configuration. | `src/lib.rs:1100` |
| sym-18f3f67406a892d0353e | `pocketstation::SessionStopDisposition::AlreadyStopped` | variant | Indicates that the operation had already stopped. | `src/lib.rs:1113` |
| sym-dbd0bc72f9929fcd50a8 | `pocketstation::SessionStopDisposition::Stopped` | variant | Indicates that the operation stopped normally. | `src/lib.rs:1112` |
| sym-56c936241baf53dc318b | `pocketstation::abi::extension::PksExtensionKind::Endpoint` | variant | Registers the native extension as a endpoint implementation. | `src/abi/extension.rs:35` |
| sym-5397638d011436a1d64f | `pocketstation::abi::extension::PksExtensionKind::Operator` | variant | Registers the native extension as a operator implementation. | `src/abi/extension.rs:34` |
| sym-6cdad11f0f57d9b7536c | `pocketstation::abi::extension::PksExtensionKind::Source` | variant | Registers the native extension as a source implementation. | `src/abi/extension.rs:33` |
| sym-de4a9df3781e78ea305d | `pocketstation::abi::extension::PksExtensionPortDirection::Input` | variant | Declares a native-extension port as input. | `src/abi/extension.rs:41` |
| sym-682b021885ebe1d80fb1 | `pocketstation::abi::extension::PksExtensionPortDirection::Output` | variant | Declares a native-extension port as output. | `src/abi/extension.rs:42` |
| sym-88a5c91a8b7995b883c6 | `pocketstation::abi::session::abi::PksSessionStatusCode::BackendFailure` | variant | Identifies the backend failure state or stage represented by `PksSessionStatusCode`. | `src/abi/session/abi.rs:93` |
| sym-4c8096adcbaca6dc4a3b | `pocketstation::abi::session::abi::PksSessionStatusCode::Cancelled` | variant | Indicates that the operation was cancelled. | `src/abi/session/abi.rs:94` |
| sym-90f9d5d0c6c0d83ae11e | `pocketstation::abi::session::abi::PksSessionStatusCode::ForeignHandle` | variant | Identifies the foreign handle state or stage represented by `PksSessionStatusCode`. | `src/abi/session/abi.rs:90` |
| sym-e0d4f694924adb5b5024 | `pocketstation::abi::session::abi::PksSessionStatusCode::IndexOutOfRange` | variant | Identifies the index out of range state or stage represented by `PksSessionStatusCode`. | `src/abi/session/abi.rs:95` |
| sym-fec2d38860664c7ec550 | `pocketstation::abi::session::abi::PksSessionStatusCode::InternalPanic` | variant | Identifies the internal panic state or stage represented by `PksSessionStatusCode`. | `src/abi/session/abi.rs:87` |
| sym-07f3c3d53a9850d3339a | `pocketstation::abi::session::abi::PksSessionStatusCode::InvalidArgument` | variant | Identifies the invalid argument state or stage represented by `PksSessionStatusCode`. | `src/abi/session/abi.rs:89` |
| sym-40f070ac74e52d3c1b18 | `pocketstation::abi::session::abi::PksSessionStatusCode::InvalidHandle` | variant | Identifies the invalid handle state or stage represented by `PksSessionStatusCode`. | `src/abi/session/abi.rs:84` |
| sym-d43fee26dadd663d4521 | `pocketstation::abi::session::abi::PksSessionStatusCode::InvalidLifecycleState` | variant | Identifies the invalid lifecycle state state or stage represented by `PksSessionStatusCode`. | `src/abi/session/abi.rs:91` |
| sym-529b3161af5cd85f4d11 | `pocketstation::abi::session::abi::PksSessionStatusCode::InvalidStructSize` | variant | Identifies the invalid struct size state or stage represented by `PksSessionStatusCode`. | `src/abi/session/abi.rs:83` |
| sym-3bd08434e914c776ed21 | `pocketstation::abi::session::abi::PksSessionStatusCode::MisalignedPointer` | variant | Identifies the misaligned pointer state or stage represented by `PksSessionStatusCode`. | `src/abi/session/abi.rs:88` |
| sym-c12ad57ffb8e0b63f712 | `pocketstation::abi::session::abi::PksSessionStatusCode::NoCapacity` | variant | Identifies the no capacity state or stage represented by `PksSessionStatusCode`. | `src/abi/session/abi.rs:86` |
| sym-bc160c9b6402292fae07 | `pocketstation::abi::session::abi::PksSessionStatusCode::NullArgument` | variant | Identifies the null argument state or stage represented by `PksSessionStatusCode`. | `src/abi/session/abi.rs:81` |
| sym-d36caf9deaa7e27f346e | `pocketstation::abi::session::abi::PksSessionStatusCode::Ok` | variant | Indicates that the operation completed successfully. | `src/abi/session/abi.rs:80` |
| sym-df81b8d4f63657ebd2c0 | `pocketstation::abi::session::abi::PksSessionStatusCode::StaleHandle` | variant | Identifies the stale handle state or stage represented by `PksSessionStatusCode`. | `src/abi/session/abi.rs:85` |
| sym-c02a61702684195fdbc7 | `pocketstation::abi::session::abi::PksSessionStatusCode::UnsupportedAbiMajor` | variant | Identifies the unsupported ABI major state or stage represented by `PksSessionStatusCode`. | `src/abi/session/abi.rs:82` |
| sym-71be19497787c8ce4b62 | `pocketstation::abi::session::abi::PksSessionStatusCode::UnsupportedAbiMinor` | variant | Identifies the unsupported ABI minor state or stage represented by `PksSessionStatusCode`. | `src/abi/session/abi.rs:96` |
| sym-3427ce389f485bbdf541 | `pocketstation::abi::session::abi::PksSessionStatusCode::WouldBlock` | variant | Identifies the would block state or stage represented by `PksSessionStatusCode`. | `src/abi/session/abi.rs:92` |
| sym-33c0221c0be7ca184faf | `pocketstation::capture::authorization::ApplicationPolicyObservation::Allowed` | variant | Reports the observed application policy as allowed. | `src/capture/authorization.rs:232` |
| sym-0fc0e1b2e536e8e3f13b | `pocketstation::capture::authorization::ApplicationPolicyObservation::Denied` | variant | Reports the observed application policy as denied. | `src/capture/authorization.rs:233` |
| sym-566c52c2a9ff86bd463b | `pocketstation::capture::authorization::ApplicationPolicyObservation::NotApplicable` | variant | Reports the observed application policy as not applicable. | `src/capture/authorization.rs:235` |
| sym-319b6fd00c14b98da3e9 | `pocketstation::capture::authorization::ApplicationPolicyObservation::NotObservable` | variant | Reports the observed application policy as not observable. | `src/capture/authorization.rs:234` |
| sym-506cb6d27f10d2778cc6 | `pocketstation::capture::authorization::CaptureCapabilityState::Available` | variant | Identifies the available state or stage represented by `CaptureCapabilityState`. | `src/capture/authorization.rs:146` |
| sym-fbb29cc04db014ac7fed | `pocketstation::capture::authorization::CaptureCapabilityState::Unavailable` | variant | Reports that the requested resource is unavailable. | `src/capture/authorization.rs:147` |
| sym-8e1d9479ec55dc17f67f | `pocketstation::capture::authorization::CaptureCapabilityState::Unsupported` | variant | Reports that the requested operation is unsupported. | `src/capture/authorization.rs:148` |
| sym-fb15b1417c558dc3e721 | `pocketstation::capture::authorization::CaptureError::BackendInit` | variant | Classifies a failure at the backend init stage or component of `CaptureError`. | `src/capture/authorization.rs:294` |
| sym-14963fd27a47e148745e | `pocketstation::capture::authorization::CaptureError::BackendSetupRequired` | variant | Classifies a failure at the backend setup required stage or component of `CaptureError`. | `src/capture/authorization.rs:296` |
| sym-0c96a931316066180385 | `pocketstation::capture::authorization::CaptureError::BackendStatus` | variant | Classifies a failure at the backend status stage or component of `CaptureError`. | `src/capture/authorization.rs:303` |
| sym-1f3f4efaa42cdef56974 | `pocketstation::capture::authorization::CaptureError::CaptureWorkerPanicked` | variant | Reports that capture worker panicked while the operation was active. | `src/capture/authorization.rs:316` |
| sym-abc62254bcb0bacffd05 | `pocketstation::capture::authorization::CaptureError::InvalidRuntimeEventCapacity` | variant | Reports that the supplied runtime event capacity is invalid. | `src/capture/authorization.rs:314` |
| sym-7a6318761cc619d71e07 | `pocketstation::capture::authorization::CaptureError::InvalidStreamCapacity` | variant | Reports that the supplied stream capacity is invalid. | `src/capture/authorization.rs:312` |
| sym-a86d1e11b3ece7cbb4a8 | `pocketstation::capture::authorization::CaptureError::ModeUnsupported` | variant | Reports that mode is unsupported by the active backend or contract. | `src/capture/authorization.rs:310` |
| sym-95c001a9a5be7dc48b6a | `pocketstation::capture::authorization::CaptureError::NotSupported` | variant | Reports that no t supported is available. | `src/capture/authorization.rs:292` |
| sym-cc5abdd62a8be53377b9 | `pocketstation::capture::authorization::CaptureError::PermissionDenied` | variant | Reports that the required permission was denied. | `src/capture/authorization.rs:301` |
| sym-0d50195408fa113a1d05 | `pocketstation::capture::authorization::CaptureError::SourceUnavailable` | variant | Reports that source is unavailable. | `src/capture/authorization.rs:308` |
| sym-e95e3b2bfbf3b0e7d031 | `pocketstation::capture::authorization::CaptureOpenOutcome::BackendFailed` | variant | Identifies the backend failed state or stage represented by `CaptureOpenOutcome`. | `src/capture/authorization.rs:286` |
| sym-4a4bb006ffadef4e4fc8 | `pocketstation::capture::authorization::CaptureOpenOutcome::NotAttempted` | variant | Identifies the not attempted state or stage represented by `CaptureOpenOutcome`. | `src/capture/authorization.rs:282` |
| sym-a13b9b4b05d56a0dbd22 | `pocketstation::capture::authorization::CaptureOpenOutcome::PermissionDenied` | variant | Reports that the required permission was denied. | `src/capture/authorization.rs:284` |
| sym-62a359b0809770b77466 | `pocketstation::capture::authorization::CaptureOpenOutcome::SourceUnavailable` | variant | Identifies the source unavailable state or stage represented by `CaptureOpenOutcome`. | `src/capture/authorization.rs:285` |
| sym-00ec5f8234fc9fcedb02 | `pocketstation::capture::authorization::CaptureOpenOutcome::Succeeded` | variant | Identifies the succeeded state or stage represented by `CaptureOpenOutcome`. | `src/capture/authorization.rs:283` |
| sym-3704169384e672bb3dad | `pocketstation::capture::authorization::CaptureScope::ExactApplication` | variant | Limits capture authorization to exact application. | `src/capture/authorization.rs:249` |
| sym-767238270726977cc51f | `pocketstation::capture::authorization::CaptureScope::ExactInputDevice` | variant | Limits capture authorization to exact input device. | `src/capture/authorization.rs:250` |
| sym-b56d7a97a07a40f0654d | `pocketstation::capture::authorization::CaptureScope::ExactOutputDevice` | variant | Limits capture authorization to exact output device. | `src/capture/authorization.rs:251` |
| sym-ee28c6fa465ee4b189ae | `pocketstation::capture::authorization::CaptureScope::SystemMix` | variant | Limits capture authorization to system mix. | `src/capture/authorization.rs:252` |
| sym-d57ec3d01d9ca01792c0 | `pocketstation::capture::authorization::CaptureSessionGrant::Denied` | variant | Represents the denied alternative defined by `CaptureSessionGrant`. | `src/capture/authorization.rs:242` |
| sym-4c4624f453b75741a427 | `pocketstation::capture::authorization::CaptureSessionGrant::GrantedByExplicitSelection` | variant | Represents the granted by explicit selection alternative defined by `CaptureSessionGrant`. | `src/capture/authorization.rs:241` |
| sym-3fe3f128e5f7492a3135 | `pocketstation::capture::authorization::CaptureSessionGrant::NotEvaluated` | variant | Represents the not evaluated alternative defined by `CaptureSessionGrant`. | `src/capture/authorization.rs:243` |
| sym-a59b1847f02845a0bba0 | `pocketstation::capture::authorization::PermissionObservation::Allowed` | variant | Represents the allowed alternative defined by `PermissionObservation`. | `src/capture/authorization.rs:154` |
| sym-6399768a1d5278efa9fa | `pocketstation::capture::authorization::PermissionObservation::Denied` | variant | Represents the denied alternative defined by `PermissionObservation`. | `src/capture/authorization.rs:155` |
| sym-f707a59a7f3fc2afd013 | `pocketstation::capture::authorization::PermissionObservation::NotApplicable` | variant | Represents the not applicable alternative defined by `PermissionObservation`. | `src/capture/authorization.rs:160` |
| sym-e137869f9015edf9bfe2 | `pocketstation::capture::authorization::PermissionObservation::NotDetermined` | variant | Represents the not determined alternative defined by `PermissionObservation`. | `src/capture/authorization.rs:157` |
| sym-fbf87c0a8fee396de84a | `pocketstation::capture::authorization::PermissionObservation::NotObservable` | variant | Represents the not observable alternative defined by `PermissionObservation`. | `src/capture/authorization.rs:159` |
| sym-2e084c36a298ccc0b020 | `pocketstation::capture::authorization::PermissionObservation::Restricted` | variant | Represents the restricted alternative defined by `PermissionObservation`. | `src/capture/authorization.rs:156` |
| sym-89395667b4be1913005f | `pocketstation::capture::authorization::PermissionObservation::Revoked` | variant | Represents the revoked alternative defined by `PermissionObservation`. | `src/capture/authorization.rs:158` |
| sym-2f340dc8bf45834d096c | `pocketstation::capture::authorization::SourceIdentityStrength::ApplicationIdAndProcessId` | variant | Represents the application id and process identifier alternative defined by `SourceIdentityStrength`. | `src/capture/authorization.rs:258` |
| sym-382a657b144bd47accac | `pocketstation::capture::authorization::SourceIdentityStrength::PlatformStableId` | variant | Represents the platform stable identifier alternative defined by `SourceIdentityStrength`. | `src/capture/authorization.rs:262` |
| sym-4d458f9ea2f9417f0345 | `pocketstation::capture::authorization::SourceIdentityStrength::ProcessId` | variant | Represents the process identifier alternative defined by `SourceIdentityStrength`. | `src/capture/authorization.rs:260` |
| sym-b59f50eec9f6ccfc00dd | `pocketstation::capture::authorization::SourceIdentityStrength::StableApplicationId` | variant | Represents the stable application identifier alternative defined by `SourceIdentityStrength`. | `src/capture/authorization.rs:259` |
| sym-bf5faed7fe149dc783ab | `pocketstation::capture::authorization::SourceIdentityStrength::StableDeviceUid` | variant | Represents the stable device uid alternative defined by `SourceIdentityStrength`. | `src/capture/authorization.rs:261` |
| sym-05c2be95e33a2c73c338 | `pocketstation::capture::events::CaptureRuntimeFailureClass::BackendClass` | variant | Classifies a failure at the backend class stage or component of `CaptureRuntimeFailureClass`. | `src/capture/events.rs:43` |
| sym-8b08b69da80dd602abea | `pocketstation::capture::events::CaptureRuntimeFailureClass::PlatformStatus` | variant | Classifies a failure at the platform status stage or component of `CaptureRuntimeFailureClass`. | `src/capture/events.rs:42` |
| sym-754bd5aca4122426854c | `pocketstation::capture::events::CaptureRuntimeFailureClass::SourceInstanceExited` | variant | Classifies a failure at the source instance exited stage or component of `CaptureRuntimeFailureClass`. | `src/capture/events.rs:41` |
| sym-af6659ae12524240fc68 | `pocketstation::capture::events::SourceLifecycleEventKind::PermissionChanged` | variant | Identifies the permission changed state or stage represented by `SourceLifecycleEventKind`. | `src/capture/events.rs:28` |
| sym-2d5e679e15cbf3dabfaa | `pocketstation::capture::events::SourceLifecycleEventKind::PermissionRevoked` | variant | Identifies the permission revoked state or stage represented by `SourceLifecycleEventKind`. | `src/capture/events.rs:29` |
| sym-e9cd842c025717366c17 | `pocketstation::capture::events::SourceLifecycleEventKind::ReplacementObserved` | variant | Identifies the replacement observed state or stage represented by `SourceLifecycleEventKind`. | `src/capture/events.rs:27` |
| sym-a721f38ca37f96d42989 | `pocketstation::capture::events::SourceLifecycleEventKind::SourceReappeared` | variant | Identifies the source reappeared state or stage represented by `SourceLifecycleEventKind`. | `src/capture/events.rs:30` |
| sym-503cab5d644faac8451f | `pocketstation::capture::events::SourceLifecycleEventKind::SourceUnavailable` | variant | Identifies the source unavailable state or stage represented by `SourceLifecycleEventKind`. | `src/capture/events.rs:26` |
| sym-1785831de9d9339143f5 | `pocketstation::capture::events::SourceRecoveryRequirement::ExplicitRediscoveryAndNewSession` | variant | Requires explicit rediscovery and new session recovery after source loss. | `src/capture/events.rs:36` |
| sym-0de595653c8bb66bd25d | `pocketstation::capture::events::SourceRuntimeEvent::BackendFailure` | variant | Identifies the backend failure state or stage represented by `SourceRuntimeEvent`. | `src/capture/events.rs:60` |
| sym-16a81002f90df9f8f781 | `pocketstation::capture::events::SourceRuntimeEvent::SourceUnavailable` | variant | Identifies the source unavailable state or stage represented by `SourceRuntimeEvent`. | `src/capture/events.rs:54` |
| sym-e36850b4ba542480196c | `pocketstation::capture::events::SourceRuntimeEventDelivery::DroppedFull` | variant | Identifies the dropped full state or stage represented by `SourceRuntimeEventDelivery`. | `src/capture/events.rs:98` |
| sym-d4b4f45d30b74a72368c | `pocketstation::capture::events::SourceRuntimeEventDelivery::DroppedOversized` | variant | Identifies the dropped oversized state or stage represented by `SourceRuntimeEventDelivery`. | `src/capture/events.rs:99` |
| sym-21efa1534dab07bf5fdf | `pocketstation::capture::events::SourceRuntimeEventDelivery::Enqueued` | variant | Identifies the enqueued state or stage represented by `SourceRuntimeEventDelivery`. | `src/capture/events.rs:97` |
| sym-5db748cd4af961830ff8 | `pocketstation::capture::events::SourceRuntimeEventDelivery::ReceiverClosed` | variant | Identifies the receiver closed state or stage represented by `SourceRuntimeEventDelivery`. | `src/capture/events.rs:100` |
| sym-da0e2b42fb3d6f4db944 | `pocketstation::capture::events::SourceRuntimeEventReceive::Closed` | variant | Reports that the underlying channel or resource is closed. | `src/capture/events.rs:107` |
| sym-441605244f3ad0e3f726 | `pocketstation::capture::events::SourceRuntimeEventReceive::Empty` | variant | Represents an empty value or collection. | `src/capture/events.rs:106` |
| sym-f9dc143caed8fb73ea1d | `pocketstation::capture::events::SourceRuntimeEventReceive::Event` | variant | Identifies the event state or stage represented by `SourceRuntimeEventReceive`. | `src/capture/events.rs:105` |
| sym-a8a7d4c009b54c646efc | `pocketstation::capture::frame_stream::CapturedFrameDelivery::Delivered` | variant | Identifies the delivered state or stage represented by `CapturedFrameDelivery`. | `src/capture/frame_stream.rs:11` |
| sym-a9bdd53c7a9fa0333fe3 | `pocketstation::capture::frame_stream::CapturedFrameDelivery::DiscardedBeforeStart` | variant | Identifies the discarded before start state or stage represented by `CapturedFrameDelivery`. | `src/capture/frame_stream.rs:13` |
| sym-54cded3873cedf93ff16 | `pocketstation::capture::frame_stream::CapturedFrameDelivery::DroppedNewest` | variant | Identifies the dropped newest state or stage represented by `CapturedFrameDelivery`. | `src/capture/frame_stream.rs:12` |
| sym-761e8abee5532d3f3509 | `pocketstation::capture::identity::SourceKind::Application` | variant | Classifies a capture source as application. | `src/capture/identity.rs:10` |
| sym-edfc2dea1388a14599fb | `pocketstation::capture::identity::SourceKind::InputDevice` | variant | Classifies a capture source as input device. | `src/capture/identity.rs:12` |
| sym-8d57dcc7348ede36058e | `pocketstation::capture::identity::SourceKind::OutputDevice` | variant | Classifies a capture source as output device. | `src/capture/identity.rs:11` |
| sym-06187730062066ecfb5f | `pocketstation::capture::identity::SourceKind::SystemMix` | variant | Classifies a capture source as system mix. | `src/capture/identity.rs:13` |
| sym-ec06c5d3b5297f955f66 | `pocketstation::capture::identity::SourceState::Available` | variant | Identifies the available state or stage represented by `SourceState`. | `src/capture/identity.rs:18` |
| sym-d2e9c52321b3c3c6f961 | `pocketstation::capture::identity::SourceState::PermissionBlocked` | variant | Identifies the permission blocked state or stage represented by `SourceState`. | `src/capture/identity.rs:22` |
| sym-5558fc49899e6f298dd9 | `pocketstation::capture::identity::SourceState::Playing` | variant | Identifies the playing state or stage represented by `SourceState`. | `src/capture/identity.rs:19` |
| sym-d5f889ad0bd7fca53ad9 | `pocketstation::capture::identity::SourceState::Silent` | variant | Identifies the silent state or stage represented by `SourceState`. | `src/capture/identity.rs:20` |
| sym-1c8bccbeaa47630c6041 | `pocketstation::capture::identity::SourceState::Unavailable` | variant | Reports that the requested resource is unavailable. | `src/capture/identity.rs:21` |
| sym-751aed77caa31bd8cd40 | `pocketstation::capture::lifecycle_registry::SourceGenerationTransition::Disappeared` | variant | Represents the disappeared alternative defined by `SourceGenerationTransition`. | `src/capture/lifecycle_registry.rs:9` |
| sym-dbb6f9d1bee0f139c925 | `pocketstation::capture::lifecycle_registry::SourceGenerationTransition::Reappeared` | variant | Represents the reappeared alternative defined by `SourceGenerationTransition`. | `src/capture/lifecycle_registry.rs:13` |
| sym-e3d4b26ef0147e3da6fe | `pocketstation::capture::query::SourceQuery::Any` | variant | Represents the any alternative defined by `SourceQuery`. | `src/capture/query.rs:14` |
| sym-b755bea3123fb5816ff4 | `pocketstation::capture::query::SourceQuery::App` | variant | Represents the app alternative defined by `SourceQuery`. | `src/capture/query.rs:15` |
| sym-fc93436707a35b0a1aa8 | `pocketstation::capture::query::SourceQuery::ByKind` | variant | Represents the by kind alternative defined by `SourceQuery`. | `src/capture/query.rs:16` |
| sym-b4255255519e65223ed2 | `pocketstation::capture::query::SourceQuery::ByStableKey` | variant | Represents the by stable key alternative defined by `SourceQuery`. | `src/capture/query.rs:17` |
| sym-824328446d81b30fe2d9 | `pocketstation::capture::query::SourceQuery::Playing` | variant | Represents the playing alternative defined by `SourceQuery`. | `src/capture/query.rs:18` |
| sym-56492f0763f1b43928e1 | `pocketstation::capture::selection::CaptureMode::Application` | variant | Requests capture in application mode. | `src/capture/selection.rs:19` |
| sym-832e6742f9c93cc9f32d | `pocketstation::capture::selection::CaptureMode::ExactApplication` | variant | Requests capture in exact application mode. | `src/capture/selection.rs:21` |
| sym-b675bcce1900cfb04d7c | `pocketstation::capture::selection::CaptureMode::ExactApplicationStable` | variant | Requests capture in exact application stable mode. | `src/capture/selection.rs:25` |
| sym-5650435a58bb47fec38c | `pocketstation::capture::selection::CaptureMode::InputDevice` | variant | Requests capture in input device mode. | `src/capture/selection.rs:28` |
| sym-ca98280ac30456ff1d4b | `pocketstation::capture::selection::CaptureMode::Process` | variant | Requests capture in process mode. | `src/capture/selection.rs:20` |
| sym-271e2cc66b50330a1e3a | `pocketstation::capture::selection::CaptureMode::SystemMix` | variant | Requests capture in system mix mode. | `src/capture/selection.rs:18` |
| sym-0e310c8fe83e336f4613 | `pocketstation::capture::selection::InputDeviceSelector::Default` | variant | Selects an input device by default. | `src/capture/selection.rs:11` |
| sym-efb759bc0a6a5c5844f9 | `pocketstation::capture::selection::InputDeviceSelector::StableId` | variant | Selects an input device by stable identifier. | `src/capture/selection.rs:12` |
| sym-c8375a8d5fcdcfd36edb | `pocketstation::capture::selection::ProcessTreeScope::ApplicationIdentity` | variant | Limits process capture to application identity. | `src/capture/selection.rs:86` |
| sym-ea20b6fa4c8cc760d4da | `pocketstation::capture::selection::ProcessTreeScope::NotApplicable` | variant | Limits process capture to not applicable. | `src/capture/selection.rs:87` |
| sym-1743216b1ebfa1ec6f5b | `pocketstation::capture::selection::ProcessTreeScope::SelectedProcessAndDescendants` | variant | Limits process capture to selected process and descendants. | `src/capture/selection.rs:85` |
| sym-2ee2aaeab3d9346050a3 | `pocketstation::capture::selection::ProcessTreeScope::SelectedProcessOnly` | variant | Limits process capture to selected process only. | `src/capture/selection.rs:84` |
| sym-f3ef64950e2782677cf5 | `pocketstation::capture::selection::SelectorPersistenceScope::ApplicationIdentity` | variant | Limits selector persistence to the application identity scope. | `src/capture/selection.rs:75` |
| sym-9a7c98db38d4df0a2103 | `pocketstation::capture::selection::SelectorPersistenceScope::DeviceIdentity` | variant | Limits selector persistence to the device identity scope. | `src/capture/selection.rs:76` |
| sym-8f775987a5e3bfe11589 | `pocketstation::capture::selection::SelectorPersistenceScope::PlatformIdentity` | variant | Limits selector persistence to the platform identity scope. | `src/capture/selection.rs:78` |
| sym-6dea22b2cf2ee587ec1b | `pocketstation::capture::selection::SelectorPersistenceScope::ProcessLifetime` | variant | Limits selector persistence to the process lifetime scope. | `src/capture/selection.rs:74` |
| sym-8484abea3f970a94fdaf | `pocketstation::capture::selection::SelectorPersistenceScope::SessionDefaultDevice` | variant | Limits selector persistence to the session default device scope. | `src/capture/selection.rs:77` |
| sym-8f2cfa23ca3964d3395e | `pocketstation::capture::timeline::CaptureSampleTimelineError::MixedAdvanceModes` | variant | Classifies a failure at the mixed advance modes stage or component of `CaptureSampleTimelineError`. | `src/capture/timeline.rs:42` |
| sym-100bc2d014f48a734ada | `pocketstation::capture::timeline::CaptureSampleTimelineError::SourcePositionMovedBackward` | variant | Reports that source position moved backward instead of remaining monotonic. | `src/capture/timeline.rs:44` |
| sym-3b1af21ba72e5799c99d | `pocketstation::capture::timeline::CaptureSampleTimelineError::SourcePositionOverflow` | variant | Reports that source position exceeds its numeric range. | `src/capture/timeline.rs:43` |
| sym-6360a1745e73e35a1ee9 | `pocketstation::codec::decoder::OpusDecodeError::FrameDurationExceedsConfiguredMaximum` | variant | Classifies a failure at the frame duration exceeds configured maximum stage or component of `OpusDecodeError`. | `src/codec/decoder.rs:29` |
| sym-d1e880d40246e0d56838 | `pocketstation::codec::decoder::OpusDecodeError::Opus` | variant | Classifies a failure at the opus stage or component of `OpusDecodeError`. | `src/codec/decoder.rs:34` |
| sym-f4f8b8b6ade7b3bcb55e | `pocketstation::codec::encoder::OpusApplication::Audio` | variant | Optimised for audio quality (music/broadcast). | `src/codec/encoder.rs:64` |
| sym-623597840615b67edfeb | `pocketstation::codec::encoder::OpusApplication::LowDelay` | variant | Optimised for low algorithmic delay. Use for real-time voice agents. | `src/codec/encoder.rs:62` |
| sym-8d3176b17cebeed6a833 | `pocketstation::codec::encoder::OpusApplication::Voip` | variant | Optimised for voice (VOIP). Default for PocketStation broadcast. | `src/codec/encoder.rs:60` |
| sym-b70ca21af71a48b9f225 | `pocketstation::codec::encoder::OpusChannels::Mono` | variant | Represents the mono alternative defined by `OpusChannels`. | `src/codec/encoder.rs:28` |
| sym-83fd05f9f594d0f8ffa2 | `pocketstation::codec::encoder::OpusChannels::Stereo` | variant | Represents the stereo alternative defined by `OpusChannels`. | `src/codec/encoder.rs:29` |
| sym-acc7fcb0b9445e901454 | `pocketstation::codec::encoder::OpusEncodeError::InvalidFrameSampleCount` | variant | Reports that the supplied frame sample count is invalid. | `src/codec/encoder.rs:135` |
| sym-4dfe2a1585087596cbf4 | `pocketstation::codec::encoder::OpusEncodeError::Opus` | variant | Classifies a failure at the opus stage or component of `OpusEncodeError`. | `src/codec/encoder.rs:141` |
| sym-d470ca3d4a4d83df93be | `pocketstation::codec::encoder::OpusFrameDuration::Ms10` | variant | Represents the ms10 alternative defined by `OpusFrameDuration`. | `src/codec/encoder.rs:8` |
| sym-0a66371187b386d36cec | `pocketstation::codec::encoder::OpusFrameDuration::Ms20` | variant | Represents the ms20 alternative defined by `OpusFrameDuration`. | `src/codec/encoder.rs:9` |
| sym-c6c7a8575e6bf02d6e24 | `pocketstation::codec::encoder::OpusFrameDuration::Ms40` | variant | Represents the ms40 alternative defined by `OpusFrameDuration`. | `src/codec/encoder.rs:10` |
| sym-e06392f2828535772ac2 | `pocketstation::codec::encoder::OpusFrameDuration::Ms60` | variant | Represents the ms60 alternative defined by `OpusFrameDuration`. | `src/codec/encoder.rs:11` |
| sym-7a6b4409e8e011fcc1d9 | `pocketstation::codec::encoder::OpusSampleRate::Hz48000` | variant | Represents the hz48000 alternative defined by `OpusSampleRate`. | `src/codec/encoder.rs:45` |
| sym-fcabf47da79c199416e3 | `pocketstation::codec::profile::StreamProfile::BroadcastStereo20ms` | variant | Represents the broadcast stereo20ms alternative defined by `StreamProfile`. | `src/codec/profile.rs:16` |
| sym-cfb5a8c73f8d95b492fa | `pocketstation::codec::profile::StreamProfile::HifiStereo20ms` | variant | Represents the hifi stereo20ms alternative defined by `StreamProfile`. | `src/codec/profile.rs:17` |
| sym-d394608f197561065186 | `pocketstation::codec::profile::StreamProfile::MusicStereo10ms` | variant | Represents the music stereo10ms alternative defined by `StreamProfile`. | `src/codec/profile.rs:15` |
| sym-98403b70ef3f8b4d9277 | `pocketstation::codec::profile::StreamProfile::MusicStereo20ms` | variant | Represents the music stereo20ms alternative defined by `StreamProfile`. | `src/codec/profile.rs:14` |
| sym-840a2a98af4eaa31725f | `pocketstation::codec::profile::StreamProfile::VoiceAgentMono10ms` | variant | Represents the voice agent mono10ms alternative defined by `StreamProfile`. | `src/codec/profile.rs:13` |
| sym-dadff3a786bf8dbc65bc | `pocketstation::codec::profile::StreamProfile::VoiceMono20ms` | variant | Represents the voice mono20ms alternative defined by `StreamProfile`. | `src/codec/profile.rs:12` |
| sym-4038ddd62a2c8584b363 | `pocketstation::conformance::ObservedEndpointError::ConnectorDeclaration` | variant | Classifies a failure at the connector declaration stage or component of `ObservedEndpointError`. | `src/conformance.rs:354` |
| sym-abbb5010c131422f2ff3 | `pocketstation::conformance::ObservedEndpointError::ConnectorRegistration` | variant | Classifies a failure at the connector registration stage or component of `ObservedEndpointError`. | `src/conformance.rs:352` |
| sym-14da1fa030aded0469a2 | `pocketstation::conformance::ObservedEndpointError::Contract` | variant | Classifies a failure at the contract stage or component of `ObservedEndpointError`. | `src/conformance.rs:346` |
| sym-47ccb82b6b934686a853 | `pocketstation::conformance::ObservedEndpointError::Declaration` | variant | Classifies a failure at the declaration stage or component of `ObservedEndpointError`. | `src/conformance.rs:348` |
| sym-2a9fb8d2ca0b69a57bff | `pocketstation::conformance::ObservedEndpointError::Registration` | variant | Classifies a failure at the registration stage or component of `ObservedEndpointError`. | `src/conformance.rs:350` |
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
| sym-8fd974f87d10883bc244 | `pocketstation::connector::error::ConnectorErrorStage::Prepare` | variant | Identifies the prepare state or stage represented by `ConnectorErrorStage`. | `src/connector/error.rs:62` |
| sym-5266ff259c1fca735731 | `pocketstation::connector::error::ConnectorErrorStage::Readiness` | variant | Identifies the readiness state or stage represented by `ConnectorErrorStage`. | `src/connector/error.rs:64` |
| sym-9f15ec887af0c5290261 | `pocketstation::connector::error::ConnectorErrorStage::Retry` | variant | Identifies the retry state or stage represented by `ConnectorErrorStage`. | `src/connector/error.rs:66` |
| sym-56be3a8997c3c9b8bbe1 | `pocketstation::connector::error::ConnectorErrorStage::Shutdown` | variant | Identifies the shutdown state or stage represented by `ConnectorErrorStage`. | `src/connector/error.rs:67` |
| sym-3581830b4e144d507580 | `pocketstation::connector::error::ConnectorErrorStage::Startup` | variant | Identifies the startup state or stage represented by `ConnectorErrorStage`. | `src/connector/error.rs:63` |
| sym-595f008e6eb6108bedd6 | `pocketstation::connector::error::ConnectorRetryability::Never` | variant | Declares a connector failure to be never. | `src/connector/error.rs:73` |
| sym-8eb5a814256274f85d14 | `pocketstation::connector::error::ConnectorRetryability::RetryAfterReconfiguration` | variant | Declares a connector failure to be retry after reconfiguration. | `src/connector/error.rs:75` |
| sym-1c2aa3299aced8362a04 | `pocketstation::connector::error::ConnectorRetryability::Retryable` | variant | Declares a connector failure to be retryable. | `src/connector/error.rs:74` |
| sym-d4b8ee5ec893ed431a01 | `pocketstation::connector::manifest::ConnectorManifestError::DuplicateManifestEntry` | variant | Reports that manifest entry duplicates an existing declaration or record. | `src/connector/manifest.rs:253` |
| sym-ec0470be1f278e449a3a | `pocketstation::connector::manifest::ConnectorManifestError::EmptyOperatorId` | variant | Reports that operator identifier is empty. | `src/connector/manifest.rs:237` |
| sym-51bbe58ed5e0dcc684c9 | `pocketstation::connector::manifest::ConnectorManifestError::InvalidManifestEntry` | variant | Reports that the supplied manifest entry is invalid. | `src/connector/manifest.rs:247` |
| sym-76d452f0131a63c1a41f | `pocketstation::connector::manifest::ConnectorManifestError::InvalidManifestRevision` | variant | Reports that the supplied manifest revision is invalid. | `src/connector/manifest.rs:235` |
| sym-6119d1f7fb4d73d9237f | `pocketstation::connector::manifest::ConnectorManifestError::InvalidPackageVersion` | variant | Reports that the supplied package version is invalid. | `src/connector/manifest.rs:239` |
| sym-56e90472f69f90836625 | `pocketstation::connector::manifest::ConnectorManifestError::ManifestEntryTooLarge` | variant | Reports that manifest entry exceeds the supported size limit. | `src/connector/manifest.rs:249` |
| sym-61c1344f18c3a47c48e1 | `pocketstation::connector::manifest::ConnectorManifestError::MissingInputPort` | variant | Reports that the required input port is missing. | `src/connector/manifest.rs:241` |
| sym-96aa7239b56b554f882a | `pocketstation::connector::manifest::ConnectorManifestError::OutputPortNotSupported` | variant | Reports that output port is not supported by this boundary. | `src/connector/manifest.rs:243` |
| sym-bf79dfb9a2751e0fa670 | `pocketstation::connector::manifest::ConnectorManifestError::RealtimeExecutionForbidden` | variant | Reports that realtime execution is forbidden by the declared safety contract. | `src/connector/manifest.rs:245` |
| sym-15745a45f5919fc782c3 | `pocketstation::connector::manifest::ConnectorManifestError::TooManyManifestEntries` | variant | Reports that the number of manifest entries exceeds the supported limit. | `src/connector/manifest.rs:251` |
| sym-21130408b8a4f375b431 | `pocketstation::connector::manifest::ConnectorManifestError::UnsupportedApiRevision` | variant | Reports that the requested API revision is unsupported. | `src/connector/manifest.rs:233` |
| sym-10b4c39f4529a7832662 | `pocketstation::connector::observations::ConnectorObservationError::StateUnavailable` | variant | Reports that state is unavailable. | `src/connector/observations.rs:177` |
| sym-f9fed9235eaae9411ca2 | `pocketstation::connector::readiness::ConnectorReadinessPolicyError::InvalidDeadline` | variant | Reports that the connector readiness invalid deadline value is invalid. | `src/connector/readiness.rs:63` |
| sym-a9098ebe827ce29af520 | `pocketstation::connector::readiness::ConnectorReadinessPolicyError::InvalidThreshold` | variant | Reports that the connector readiness invalid threshold value is invalid. | `src/connector/readiness.rs:65` |
| sym-d1638685121819b4fdb0 | `pocketstation::connector::status::ConnectorDeliveryReadiness::NotReady` | variant | Identifies the not ready state or stage represented by `ConnectorDeliveryReadiness`. | `src/connector/status.rs:5` |
| sym-d041776d9a0e9d37fd64 | `pocketstation::connector::status::ConnectorDeliveryReadiness::Ready` | variant | Identifies the ready state or stage represented by `ConnectorDeliveryReadiness`. | `src/connector/status.rs:6` |
| sym-9de09f628ac31ae3d813 | `pocketstation::connector::status::ConnectorHealth::Degraded` | variant | Represents the degraded alternative defined by `ConnectorHealth`. | `src/connector/status.rs:19` |
| sym-848db6512c8b91cfb556 | `pocketstation::connector::status::ConnectorHealth::Healthy` | variant | Represents the healthy alternative defined by `ConnectorHealth`. | `src/connector/status.rs:18` |
| sym-9119b9b0a1b019cc4011 | `pocketstation::connector::status::ConnectorRecovery::Idle` | variant | Represents the idle alternative defined by `ConnectorRecovery`. | `src/connector/status.rs:25` |
| sym-05a78eaf4468a8ed37f7 | `pocketstation::connector::status::ConnectorRecovery::Reconnecting` | variant | Represents the reconnecting alternative defined by `ConnectorRecovery`. | `src/connector/status.rs:26` |
| sym-24e6dcbb1e78e5cd5507 | `pocketstation::connector::transport::ConnectorAudioRecordError::InvalidConnectorId` | variant | Reports that the supplied connector identifier is invalid. | `src/connector/transport.rs:596` |
| sym-6afbab6e208232ee46b6 | `pocketstation::connector::transport::ConnectorAudioRecordError::InvalidHeaderSize` | variant | Reports that the supplied header size is invalid. | `src/connector/transport.rs:582` |
| sym-de52cdac15ca832a27f9 | `pocketstation::connector::transport::ConnectorAudioRecordError::InvalidLineage` | variant | Reports that the supplied lineage is invalid. | `src/connector/transport.rs:594` |
| sym-f36f0f7ff679d1b9f95a | `pocketstation::connector::transport::ConnectorAudioRecordError::InvalidMagic` | variant | Reports that the supplied magic is invalid. | `src/connector/transport.rs:576` |
| sym-f5b2cccc3b596928e4cb | `pocketstation::connector::transport::ConnectorAudioRecordError::InvalidPortName` | variant | Reports that the supplied port name is invalid. | `src/connector/transport.rs:586` |
| sym-44daa1d3bc5fbfaa8cfb | `pocketstation::connector::transport::ConnectorAudioRecordError::InvalidSampleCount` | variant | Reports that the supplied sample count is invalid. | `src/connector/transport.rs:592` |
| sym-29afd08cd25ce4e1df1e | `pocketstation::connector::transport::ConnectorAudioRecordError::InvalidSampleSpec` | variant | Reports that the supplied sample spec is invalid. | `src/connector/transport.rs:588` |
| sym-4a82624a6522ac0e5c4d | `pocketstation::connector::transport::ConnectorAudioRecordError::LengthOverflow` | variant | Reports that length exceeds its numeric range. | `src/connector/transport.rs:598` |
| sym-b62b5d4b7860254a0603 | `pocketstation::connector::transport::ConnectorAudioRecordError::NotAudio` | variant | Reports that no t audio is available. | `src/connector/transport.rs:570` |
| sym-27f2b2329a88a76bcc61 | `pocketstation::connector::transport::ConnectorAudioRecordError::ReservedFieldSet` | variant | Reports that a reserved compatibility field contains a nonzero value. | `src/connector/transport.rs:584` |
| sym-c1a720e2e80b19131d79 | `pocketstation::connector::transport::ConnectorAudioRecordError::TrailingBytes` | variant | Reports that bytes remain after decoding the complete record. | `src/connector/transport.rs:574` |
| sym-a17cf9251f85c2a2b3c6 | `pocketstation::connector::transport::ConnectorAudioRecordError::Truncated` | variant | Reports that the encoded input ended before the complete record was available. | `src/connector/transport.rs:572` |
| sym-edff21b9108f46a36467 | `pocketstation::connector::transport::ConnectorAudioRecordError::UnsupportedMajor` | variant | Reports that the requested major is unsupported. | `src/connector/transport.rs:578` |
| sym-a4cdfc62d3ca11edc4df | `pocketstation::connector::transport::ConnectorAudioRecordError::UnsupportedMinor` | variant | Reports that the requested minor is unsupported. | `src/connector/transport.rs:580` |
| sym-38989948cbff5a371e99 | `pocketstation::connector::transport::ConnectorAudioRecordError::UnsupportedSampleFormat` | variant | Reports that the requested sample format is unsupported. | `src/connector/transport.rs:590` |
| sym-42cc0c6bddc29a89cca1 | `pocketstation::connector::transport::ConnectorConfigurationRecordError::DuplicateField` | variant | Reports that field duplicates an existing declaration or record. | `src/connector/transport.rs:269` |
| sym-7e511aa4d01f489195d8 | `pocketstation::connector::transport::ConnectorConfigurationRecordError::InvalidFieldName` | variant | Reports that the supplied field name is invalid. | `src/connector/transport.rs:267` |
| sym-73b0278c62a5e90f740a | `pocketstation::connector::transport::ConnectorConfigurationRecordError::InvalidMagic` | variant | Reports that the supplied magic is invalid. | `src/connector/transport.rs:257` |
| sym-8f143b89cf289121a663 | `pocketstation::connector::transport::ConnectorConfigurationRecordError::InvalidValue` | variant | Reports that the supplied value is invalid. | `src/connector/transport.rs:273` |
| sym-6eaa605b7494e4b3f832 | `pocketstation::connector::transport::ConnectorConfigurationRecordError::LengthOverflow` | variant | Reports that length exceeds its numeric range. | `src/connector/transport.rs:277` |
| sym-205af479c34c5e9b10ed | `pocketstation::connector::transport::ConnectorConfigurationRecordError::ReservedFieldSet` | variant | Reports that a reserved compatibility field contains a nonzero value. | `src/connector/transport.rs:263` |
| sym-7870e44974767635eeea | `pocketstation::connector::transport::ConnectorConfigurationRecordError::TooManyFields` | variant | Reports that the number of fields exceeds the supported limit. | `src/connector/transport.rs:265` |
| sym-89881ac280a6a6b61c71 | `pocketstation::connector::transport::ConnectorConfigurationRecordError::TrailingBytes` | variant | Reports that bytes remain after decoding the complete record. | `src/connector/transport.rs:255` |
| sym-b6a820a5cacc56b50c04 | `pocketstation::connector::transport::ConnectorConfigurationRecordError::Truncated` | variant | Reports that the encoded input ended before the complete record was available. | `src/connector/transport.rs:253` |
| sym-af5af26ea090b274458e | `pocketstation::connector::transport::ConnectorConfigurationRecordError::UnknownValueKind` | variant | Reports that the referenced value kind is not declared or registered. | `src/connector/transport.rs:275` |
| sym-006418dd057b18b941fb | `pocketstation::connector::transport::ConnectorConfigurationRecordError::UnsupportedMajor` | variant | Reports that the requested major is unsupported. | `src/connector/transport.rs:259` |
| sym-2a653776fa2f234be30d | `pocketstation::connector::transport::ConnectorConfigurationRecordError::UnsupportedMinor` | variant | Reports that the requested minor is unsupported. | `src/connector/transport.rs:261` |
| sym-a9d935ac58cc7a100b90 | `pocketstation::connector::transport::ConnectorConfigurationRecordError::ValueTooLarge` | variant | Reports that value exceeds the supported size limit. | `src/connector/transport.rs:271` |
| sym-fe375067060ae019a081 | `pocketstation::connector::worker::driver::ConnectorDeliveryOutcome::Delivered` | variant | Identifies the delivered state or stage represented by `ConnectorDeliveryOutcome`. | `src/connector/worker/driver.rs:84` |
| sym-6b161540c2905cddd3da | `pocketstation::connector::worker::driver::ConnectorDeliveryOutcome::Dropped` | variant | Identifies the dropped state or stage represented by `ConnectorDeliveryOutcome`. | `src/connector/worker/driver.rs:85` |
| sym-ae6b315a5c5eeba2a2eb | `pocketstation::connector::worker::driver::ConnectorItem::Audio` | variant | Represents the audio alternative defined by `ConnectorItem`. | `src/connector/worker/driver.rs:63` |
| sym-a398bccb576f40f0e322 | `pocketstation::connector::worker::driver::ConnectorItem::Signal` | variant | Represents the signal alternative defined by `ConnectorItem`. | `src/connector/worker/driver.rs:67` |
| sym-dba8e3e51898fdfc29a8 | `pocketstation::endpoint::contract::EndpointReceiver::Audio` | variant | Represents the audio alternative defined by `EndpointReceiver`. | `src/endpoint/contract.rs:176` |
| sym-ef61ae80d6d7068f5f8e | `pocketstation::endpoint::contract::EndpointReceiver::Signal` | variant | Represents the signal alternative defined by `EndpointReceiver`. | `src/endpoint/contract.rs:180` |
| sym-558f64d9bd04a5036345 | `pocketstation::endpoint::identity::EndpointPreparationGroup::Route` | variant | Represents the route alternative defined by `EndpointPreparationGroup`. | `src/endpoint/identity.rs:24` |
| sym-daa9085c5659242b0b72 | `pocketstation::endpoint::identity::EndpointPreparationGroup::Shared` | variant | Represents the shared alternative defined by `EndpointPreparationGroup`. | `src/endpoint/identity.rs:25` |
| sym-978375a0be59249f7f2c | `pocketstation::endpoint::polled_audio_driver::PolledAudioEndpointConfigError::BatchCapacityTooLarge` | variant | Reports that batch capacity exceeds the supported size limit. | `src/endpoint/polled_audio_driver.rs:50` |
| sym-63380bb9a43d9b88f8d6 | `pocketstation::endpoint::polled_audio_driver::PolledAudioEndpointConfigError::LeaseCapacityTooLarge` | variant | Reports that lease capacity exceeds the supported size limit. | `src/endpoint/polled_audio_driver.rs:52` |
| sym-dddafdff62c6c8fc3ed7 | `pocketstation::endpoint::polled_audio_driver::PolledAudioEndpointConfigError::QueueCapacityTooLarge` | variant | Reports that queue capacity exceeds the supported size limit. | `src/endpoint/polled_audio_driver.rs:48` |
| sym-a43421d697e769284905 | `pocketstation::endpoint::polled_audio_driver::PolledAudioEndpointConfigError::ZeroBatchCapacity` | variant | Reports that batch capacity must be greater than zero. | `src/endpoint/polled_audio_driver.rs:44` |
| sym-af48981a7240618749f6 | `pocketstation::endpoint::polled_audio_driver::PolledAudioEndpointConfigError::ZeroLeaseCapacity` | variant | Reports that lease capacity must be greater than zero. | `src/endpoint/polled_audio_driver.rs:46` |
| sym-4af8748ea496f1f89604 | `pocketstation::endpoint::polled_audio_driver::PolledAudioEndpointConfigError::ZeroQueueCapacity` | variant | Reports that queue capacity must be greater than zero. | `src/endpoint/polled_audio_driver.rs:42` |
| sym-e5094729635ad491978b | `pocketstation::endpoint::polled_audio_driver::PolledAudioPollError::Empty` | variant | Represents an empty value or collection. | `src/endpoint/polled_audio_driver.rs:76` |
| sym-504c5a64f147c1337468 | `pocketstation::endpoint::polled_audio_driver::PolledAudioPollError::LeaseCapacityExhausted` | variant | Reports that the available lease capacity range or capacity is exhausted. | `src/endpoint/polled_audio_driver.rs:78` |
| sym-abc5f1e44969a01e61ca | `pocketstation::endpoint::polled_audio_driver::PolledAudioPollError::StatePoisoned` | variant | Reports that shared state became unavailable after a panic while locked. | `src/endpoint/polled_audio_driver.rs:80` |
| sym-44596104fc2b88bc781b | `pocketstation::endpoint::registry::EndpointDriverRegistryError::Duplicate` | variant | Reports that the supplied value duplicates an existing record. | `src/endpoint/registry.rs:24` |
| sym-3fa64548a35a9c949169 | `pocketstation::endpoint::registry::EndpointDriverRegistryError::EmptyNodeTypeId` | variant | Reports that node type identifier is empty. | `src/endpoint/registry.rs:20` |
| sym-98aa3609aa51ffbe1420 | `pocketstation::endpoint::registry::EndpointDriverRegistryError::EmptyOperatorId` | variant | Reports that operator identifier is empty. | `src/endpoint/registry.rs:18` |
| sym-8276bbd388b55f176a66 | `pocketstation::endpoint::registry::EndpointDriverRegistryError::OperatorNodeTypeConflict` | variant | Reports that operator node type conflicts with an existing registration or declaration. | `src/endpoint/registry.rs:31` |
| sym-ed5d2de77317c48cb92e | `pocketstation::endpoint::registry::EndpointPrepareError::Driver` | variant | Classifies a failure at the driver stage or component of `EndpointPrepareError`. | `src/endpoint/registry.rs:50` |
| sym-e59935f9269e7cbc5a04 | `pocketstation::endpoint::registry::EndpointPrepareError::EmptyBatch` | variant | Reports that batch is empty. | `src/endpoint/registry.rs:41` |
| sym-ce86d37ee7b44ccbb6ce | `pocketstation::endpoint::registry::EndpointPrepareError::NotRegistered` | variant | Reports that no t registered is available. | `src/endpoint/registry.rs:45` |
| sym-dddb1010865603fbcb39 | `pocketstation::endpoint::runtime::EndpointFailureRetryability::Never` | variant | Declares an endpoint failure to be never. | `src/endpoint/runtime.rs:167` |
| sym-80a40b94cc9fcf9aeaf2 | `pocketstation::endpoint::runtime::EndpointFailureRetryability::ReconfigurationRequired` | variant | Declares an endpoint failure to be reconfiguration required. | `src/endpoint/runtime.rs:169` |
| sym-c1c3f9244f8f503af1c5 | `pocketstation::endpoint::runtime::EndpointFailureRetryability::Retryable` | variant | Declares an endpoint failure to be retryable. | `src/endpoint/runtime.rs:168` |
| sym-3cdfb3dc720339cd3977 | `pocketstation::endpoint::runtime::EndpointFailureStage::CancelPreparation` | variant | Identifies the cancel preparation state or stage represented by `EndpointFailureStage`. | `src/endpoint/runtime.rs:158` |
| sym-6914934ddf03a07ca48c | `pocketstation::endpoint::runtime::EndpointFailureStage::JoinFinalize` | variant | Identifies the join finalize state or stage represented by `EndpointFailureStage`. | `src/endpoint/runtime.rs:161` |
| sym-f05233c8ff710b0a20b1 | `pocketstation::endpoint::runtime::EndpointFailureStage::Prepare` | variant | Identifies the prepare state or stage represented by `EndpointFailureStage`. | `src/endpoint/runtime.rs:157` |
| sym-fb1fa7b7bce6ae748d4b | `pocketstation::endpoint::runtime::EndpointFailureStage::RequestStop` | variant | Identifies the request stop state or stage represented by `EndpointFailureStage`. | `src/endpoint/runtime.rs:160` |
| sym-d93e81069b2d5082a258 | `pocketstation::endpoint::runtime::EndpointFailureStage::Start` | variant | Identifies the start state or stage represented by `EndpointFailureStage`. | `src/endpoint/runtime.rs:159` |
| sym-128c9814777b16f53af6 | `pocketstation::endpoint::runtime::EndpointInputOrigin::Signal` | variant | A typed signal whose detailed provenance is carried by `SignalLineage`. | `src/endpoint/runtime.rs:34` |
| sym-01b74597c85f85568c5d | `pocketstation::endpoint::runtime::EndpointInputOrigin::Source` | variant | Represents the source alternative defined by `EndpointInputOrigin`. | `src/endpoint/runtime.rs:35` |
| sym-c098bd91856062a35d55 | `pocketstation::endpoint::runtime::EndpointInputOrigin::Stem` | variant | Represents the stem alternative defined by `EndpointInputOrigin`. | `src/endpoint/runtime.rs:32` |
| sym-f6aa89e96e4b00ae7ff3 | `pocketstation::endpoint::runtime::EndpointShutdownMode::Abort` | variant | Shuts an endpoint down using the abort mode. | `src/endpoint/runtime.rs:358` |
| sym-8cd975e76d6427c0529b | `pocketstation::endpoint::runtime::EndpointShutdownMode::Drain` | variant | Shuts an endpoint down using the drain mode. | `src/endpoint/runtime.rs:357` |
| sym-8a0c6873e9c295993f2d | `pocketstation::endpoint::runtime::EndpointStartFailureCause::Driver` | variant | Classifies a failure at the driver stage or component of `EndpointStartFailureCause`. | `src/endpoint/runtime.rs:440` |
| sym-ceddb4a392fd1ba1cf84 | `pocketstation::endpoint::runtime::EndpointStartFailureCause::GateAlreadyOpen` | variant | Classifies a failure at the gate already open stage or component of `EndpointStartFailureCause`. | `src/endpoint/runtime.rs:439` |
| sym-19eaba59e9cfed66e693 | `pocketstation::frame::audio::AudioFrameBuildError::MisalignedSamples` | variant | Reports that samples does not satisfy the required alignment. | `src/frame/audio.rs:57` |
| sym-eaf6e41fb330ca815075 | `pocketstation::frame::audio::AudioFrameBuildError::ZeroChannels` | variant | Reports that channels must be greater than zero. | `src/frame/audio.rs:55` |
| sym-c684902a1f1c52d3b842 | `pocketstation::frame::audio::AudioFrameBuildError::ZeroSampleRate` | variant | Reports that sample rate must be greater than zero. | `src/frame/audio.rs:53` |
| sym-2c22a35c423b0c4ec202 | `pocketstation::frame::audio::FrameLineageError::SequenceNumber` | variant | Classifies a failure at the sequence number stage or component of `FrameLineageError`. | `src/frame/audio.rs:254` |
| sym-d5d0ca3146b94c396fdb | `pocketstation::frame::audio::FrameLineageError::Source` | variant | Classifies a failure at the source stage or component of `FrameLineageError`. | `src/frame/audio.rs:252` |
| sym-85195a3401c0fe61e495 | `pocketstation::frame::audio::FrameLineageError::Timestamp` | variant | Classifies a failure at the timestamp stage or component of `FrameLineageError`. | `src/frame/audio.rs:256` |
| sym-3a769b03af01431e0e85 | `pocketstation::frame::audio::SampleFormat::F32Interleaved` | variant | Declares PCM samples in f32 interleaved format. | `src/frame/audio.rs:14` |
| sym-1189c02b7250275259f9 | `pocketstation::frame::lineage::FrameLineageBuildError::TimestampOverflow` | variant | Reports that timestamp exceeds its numeric range. | `src/frame/lineage.rs:99` |
| sym-8565c6b0e0b425dc0c01 | `pocketstation::frame::lineage::FrameLineageBuildError::ZeroDuration` | variant | Reports that duration must be greater than zero. | `src/frame/lineage.rs:95` |
| sym-a1e996dbffa813d8ec0f | `pocketstation::frame::lineage::FrameLineageBuildError::ZeroSourceGeneration` | variant | Reports that source generation must be greater than zero. | `src/frame/lineage.rs:97` |
| sym-a5faed540ad6de134b1c | `pocketstation::frame::platform::Platform::Android` | variant | Represents the android alternative defined by `Platform`. | `src/frame/platform.rs:9` |
| sym-d7895c7b24722dbb63c2 | `pocketstation::frame::platform::Platform::Ios` | variant | Represents the ios alternative defined by `Platform`. | `src/frame/platform.rs:8` |
| sym-9652799b7d15ebc70b3a | `pocketstation::frame::platform::Platform::Linux` | variant | Represents the linux alternative defined by `Platform`. | `src/frame/platform.rs:7` |
| sym-925c9311fede4c71c8fc | `pocketstation::frame::platform::Platform::Macos` | variant | Represents the macos alternative defined by `Platform`. | `src/frame/platform.rs:5` |
| sym-05ee566e953b53c50178 | `pocketstation::frame::platform::Platform::Unknown` | variant | Represents the unknown alternative defined by `Platform`. | `src/frame/platform.rs:11` |
| sym-2e39b3ebc8ab3c6e06bd | `pocketstation::frame::platform::Platform::Web` | variant | Represents the web alternative defined by `Platform`. | `src/frame/platform.rs:10` |
| sym-31d821d576822bb09505 | `pocketstation::frame::platform::Platform::Windows` | variant | Represents the windows alternative defined by `Platform`. | `src/frame/platform.rs:6` |
| sym-29e6dc01c36a90fb7576 | `pocketstation::frame::pool::AudioBufferWriteError::CapacityExceeded` | variant | Classifies a failure at the capacity exceeded stage or component of `AudioBufferWriteError`. | `src/frame/pool.rs:18` |
| sym-81404314ed0f3c7a36cd | `pocketstation::graph::compile::resolve::CompileError::AdapterUnavailable` | variant | Reports that adapter is unavailable. | `src/graph/compile/resolve.rs:62` |
| sym-c20211926d7ff84bf16b | `pocketstation::graph::compile::resolve::CompileError::ClockDomainMismatch` | variant | Reports that clock domain does not match the expected contract. | `src/graph/compile/resolve.rs:38` |
| sym-0db72812466f4aa00d80 | `pocketstation::graph::compile::resolve::CompileError::CycleDetected` | variant | Reports that the declared graph contains a dependency cycle. | `src/graph/compile/resolve.rs:60` |
| sym-ce916ba1279b345f15db | `pocketstation::graph::compile::resolve::CompileError::InvalidConfig` | variant | Reports that the supplied config is invalid. | `src/graph/compile/resolve.rs:30` |
| sym-7310556867da57a3b62f | `pocketstation::graph::compile::resolve::CompileError::InvalidRealtimeEdge` | variant | Reports that the supplied realtime edge is invalid. | `src/graph/compile/resolve.rs:58` |
| sym-cc2ba97b1b5c55568f84 | `pocketstation::graph::compile::resolve::CompileError::InvalidSafetyContract` | variant | Reports that the supplied safety contract is invalid. | `src/graph/compile/resolve.rs:51` |
| sym-6d489ed37ae07149138d | `pocketstation::graph::compile::resolve::CompileError::MediaMismatch` | variant | Reports that media does not match the expected contract. | `src/graph/compile/resolve.rs:45` |
| sym-6a39f0005971fc375ee8 | `pocketstation::graph::compile::resolve::CompileError::SignalMismatch` | variant | Reports that signal does not match the expected contract. | `src/graph/compile/resolve.rs:47` |
| sym-0eb3b62184881a833b6f | `pocketstation::graph::compile::resolve::CompileError::UnknownNode` | variant | Reports that the referenced node is not declared or registered. | `src/graph/compile/resolve.rs:32` |
| sym-00e428164d01ecea7eda | `pocketstation::graph::compile::resolve::CompileError::UnknownNodeType` | variant | Reports that the referenced node type is not declared or registered. | `src/graph/compile/resolve.rs:28` |
| sym-4df28a87228f4ad2ab85 | `pocketstation::graph::compile::resolve::CompileError::UnknownPort` | variant | Reports that the referenced port is not declared or registered. | `src/graph/compile/resolve.rs:34` |
| sym-fb37149595e30ca77797 | `pocketstation::graph::compile::resolve::CompileError::WrongPortDirection` | variant | Reports that port direction does not match the required identity or contract. | `src/graph/compile/resolve.rs:36` |
| sym-28ba3d6bfe65c5b0b82d | `pocketstation::graph::node::ConfigError::Invalid` | variant | Reports that validation rejected the supplied value. | `src/graph/node.rs:145` |
| sym-e4a76a9a6ae26d649b2f | `pocketstation::graph::node::ConfigError::Missing` | variant | Reports that a required value is missing. | `src/graph/node.rs:143` |
| sym-58e706ac5c81a692ca28 | `pocketstation::graph::node::NodeDescriptorError::DuplicatePort` | variant | Reports that port duplicates an existing declaration or record. | `src/graph/node.rs:262` |
| sym-c887061af3c2535f369a | `pocketstation::graph::node::NodeDescriptorError::EmptyDisplayName` | variant | Reports that display name is empty. | `src/graph/node.rs:256` |
| sym-3bd9b336fe6ef7d907f6 | `pocketstation::graph::node::NodeDescriptorError::EmptyTypeId` | variant | Reports that type identifier is empty. | `src/graph/node.rs:254` |
| sym-9436b84a8e4c6d47c36c | `pocketstation::graph::node::NodeDescriptorError::InvalidSafetyContract` | variant | Reports that the supplied safety contract is invalid. | `src/graph/node.rs:258` |
| sym-e428f267a113ad7e2d2a | `pocketstation::graph::node::NodeDescriptorError::PortDirectionMismatch` | variant | Reports that port direction does not match the expected contract. | `src/graph/node.rs:260` |
| sym-bc2ad57ca24ead5b96f1 | `pocketstation::graph::node::NodeError::Config` | variant | Classifies a failure at the config stage or component of `NodeError`. | `src/graph/node.rs:161` |
| sym-e2316e4217cb60149611 | `pocketstation::graph::node::NodeError::ExternalBoundaryExecution` | variant | Classifies a failure at the external boundary execution stage or component of `NodeError`. | `src/graph/node.rs:159` |
| sym-8fecbd5409dc022b61a2 | `pocketstation::graph::node::NodeError::Prepare` | variant | Classifies a failure at the prepare stage or component of `NodeError`. | `src/graph/node.rs:151` |
| sym-de3de88949ae5e45ea3a | `pocketstation::graph::node::NodeError::Process` | variant | Classifies a failure at the process stage or component of `NodeError`. | `src/graph/node.rs:153` |
| sym-79798dbdbe03ac5e3bdf | `pocketstation::graph::node::NodeError::ProcessTimeout` | variant | Reports that process exceeded its deadline. | `src/graph/node.rs:155` |
| sym-49477dc74ada3dcf2196 | `pocketstation::graph::partition::ExecutionPartition::AsyncWorker` | variant | Tokio async task. | `src/graph/partition.rs:36` |
| sym-16b9d5ad8aa643d557ea | `pocketstation::graph::partition::ExecutionPartition::AudioCallback` | variant | Platform OS audio callback — the strictest domain. | `src/graph/partition.rs:24` |
| sym-8a04bd5c7c5fba600ff0 | `pocketstation::graph::partition::ExecutionPartition::BlockingWorker` | variant | `spawn_blocking` thread. | `src/graph/partition.rs:42` |
| sym-99283fa324a76885ccfc | `pocketstation::graph::partition::ExecutionPartition::External` | variant | Remote service — always async, always network-required. | `src/graph/partition.rs:48` |
| sym-999023591a3450afa729 | `pocketstation::graph::partition::ExecutionPartition::RealtimeCpu` | variant | Dedicated real-time processing thread. | `src/graph/partition.rs:30` |
| sym-94d1eedeac764c1b24a9 | `pocketstation::graph::partition::SafetyContract::AllocationAllowed` | variant | May heap-allocate but must not block or make network calls. | `src/graph/partition.rs:90` |
| sym-c748311d8316d9f7cddb | `pocketstation::graph::partition::SafetyContract::BlockingAllowed` | variant | May block the current OS thread. | `src/graph/partition.rs:93` |
| sym-b93c8c22c8e946741e61 | `pocketstation::graph::partition::SafetyContract::ExternalService` | variant | Backed by a remote service; all calls are async network operations. | `src/graph/partition.rs:99` |
| sym-40db3a7096ad7d8010e9 | `pocketstation::graph::partition::SafetyContract::NetworkAllowed` | variant | May make network calls (implies async + allocation allowed). | `src/graph/partition.rs:96` |
| sym-76e626915f863e907ecd | `pocketstation::graph::partition::SafetyContract::RealtimeSafe` | variant | No heap allocation, no locking, no blocking, no logging. | `src/graph/partition.rs:87` |
| sym-fd961524c69f6f8b733b | `pocketstation::graph::plan::PlanError::FanInOnSinglePort` | variant | Classifies a failure at the fan in on single port stage or component of `PlanError`. | `src/graph/plan.rs:23` |
| sym-299754ef8d917bcce8c7 | `pocketstation::graph::plan::PlanError::MissingEdgeContract` | variant | Reports that the required edge contract is missing. | `src/graph/plan.rs:27` |
| sym-922e61ce56b460eb50a6 | `pocketstation::graph::plan::PlanError::MissingOutputSignal` | variant | Reports that the required output signal is missing. | `src/graph/plan.rs:29` |
| sym-b0e00475bf5e0fe3dd38 | `pocketstation::graph::plan::PlanError::MoveExclusiveFanOut` | variant | Classifies a failure at the move exclusive fan out stage or component of `PlanError`. | `src/graph/plan.rs:25` |
| sym-c6cb82ef66240ecee059 | `pocketstation::graph::ports::BackpressurePolicy::BlockForbidden` | variant | Handles bounded queue pressure using the block forbidden policy. | `src/graph/ports.rs:269` |
| sym-5a61b1eb45e7a5ad7ae0 | `pocketstation::graph::ports::BackpressurePolicy::BoundedQueue` | variant | Handles bounded queue pressure using the bounded queue policy. | `src/graph/ports.rs:268` |
| sym-aafbc7eb0c00eb3477ba | `pocketstation::graph::ports::BackpressurePolicy::DropNewest` | variant | Handles bounded queue pressure using the drop newest policy. | `src/graph/ports.rs:266` |
| sym-1c6b13e9c020efbb8fad | `pocketstation::graph::ports::BackpressurePolicy::DropOldest` | variant | Handles bounded queue pressure using the drop oldest policy. | `src/graph/ports.rs:267` |
| sym-b599718deaaa18814819 | `pocketstation::graph::ports::ChannelLayout::Any` | variant | Represents the any alternative defined by `ChannelLayout`. | `src/graph/ports.rs:30` |
| sym-56ef86ccad518ea59756 | `pocketstation::graph::ports::ChannelLayout::Mono` | variant | Represents the mono alternative defined by `ChannelLayout`. | `src/graph/ports.rs:28` |
| sym-c0005fa6207c396f72fe | `pocketstation::graph::ports::ChannelLayout::Stereo` | variant | Represents the stereo alternative defined by `ChannelLayout`. | `src/graph/ports.rs:29` |
| sym-8aff82eb6902ece00a36 | `pocketstation::graph::ports::ClockDomain::Capture` | variant | Represents the capture alternative defined by `ClockDomain`. | `src/graph/ports.rs:250` |
| sym-713ec9829d6d37d26660 | `pocketstation::graph::ports::ClockDomain::Inherited` | variant | Preserve the clock carried by the producer's signal envelope. | `src/graph/ports.rs:254` |
| sym-bd35c405fad0442fc409 | `pocketstation::graph::ports::ClockDomain::Network` | variant | Represents the network alternative defined by `ClockDomain`. | `src/graph/ports.rs:252` |
| sym-c0372fae9a06b5b339e8 | `pocketstation::graph::ports::ClockDomain::Playback` | variant | Represents the playback alternative defined by `ClockDomain`. | `src/graph/ports.rs:251` |
| sym-13772f02efdf974a5eda | `pocketstation::graph::ports::ClockDomain::Wallclock` | variant | Represents the wallclock alternative defined by `ClockDomain`. | `src/graph/ports.rs:255` |
| sym-4ea28cfc07ba3147c9a2 | `pocketstation::graph::ports::CopyPolicy::CopyToBranchPool` | variant | Applies the copy to branch pool storage-sharing policy to routed values. | `src/graph/ports.rs:283` |
| sym-3627ce255343f58eb3a0 | `pocketstation::graph::ports::CopyPolicy::MoveExclusive` | variant | Applies the move exclusive storage-sharing policy to routed values. | `src/graph/ports.rs:281` |
| sym-0437ff7b7278a0616a2f | `pocketstation::graph::ports::CopyPolicy::ShareReadOnly` | variant | Applies the share read only storage-sharing policy to routed values. | `src/graph/ports.rs:282` |
| sym-26567f3f67f31aca8562 | `pocketstation::graph::ports::DeliverySemantics::BestEffortRealtime` | variant | Identifies the best effort realtime state or stage represented by `DeliverySemantics`. | `src/graph/ports.rs:274` |
| sym-6ac2fb99f76b4e6943d9 | `pocketstation::graph::ports::DeliverySemantics::ExactlyOnceNotRealtime` | variant | Identifies the exactly once not realtime state or stage represented by `DeliverySemantics`. | `src/graph/ports.rs:276` |
| sym-d28ac60205bbfe5b9fe1 | `pocketstation::graph::ports::DeliverySemantics::Ordered` | variant | Identifies the ordered state or stage represented by `DeliverySemantics`. | `src/graph/ports.rs:275` |
| sym-c58cf958eb453a5b8fc2 | `pocketstation::graph::ports::EdgeObservabilityLevel::Counters` | variant | Exposes counters observations for a graph edge. | `src/graph/ports.rs:296` |
| sym-b829f2de272438a8e4cb | `pocketstation::graph::ports::EdgeObservabilityLevel::Full` | variant | Reports that bounded capacity is full. | `src/graph/ports.rs:297` |
| sym-d2d43ea84e7076e501e8 | `pocketstation::graph::ports::EdgeObservabilityLevel::Off` | variant | Exposes off observations for a graph edge. | `src/graph/ports.rs:295` |
| sym-bc5dd7f93ee3e26063e1 | `pocketstation::graph::ports::LossPolicy::ConcealForAudio` | variant | Handles delivery loss using the conceal for audio policy. | `src/graph/ports.rs:288` |
| sym-70b24ff8de94a0da1ffb | `pocketstation::graph::ports::LossPolicy::DropAllowed` | variant | Handles delivery loss using the drop allowed policy. | `src/graph/ports.rs:290` |
| sym-8600e17237171ed8dc58 | `pocketstation::graph::ports::LossPolicy::MustDeliverOrFail` | variant | Handles delivery loss using the must deliver or fail policy. | `src/graph/ports.rs:289` |
| sym-7e4de59fccaf17d81295 | `pocketstation::graph::ports::MediaCaps::Any` | variant | Represents the any alternative defined by `MediaCaps`. | `src/graph/ports.rs:93` |
| sym-07b48ddf95df481af8b6 | `pocketstation::graph::ports::MediaCaps::Audio` | variant | Represents the audio alternative defined by `MediaCaps`. | `src/graph/ports.rs:86` |
| sym-061acac84de921604473 | `pocketstation::graph::ports::MediaCaps::Binary` | variant | Represents the binary alternative defined by `MediaCaps`. | `src/graph/ports.rs:92` |
| sym-d7b5e0b16870db02f813 | `pocketstation::graph::ports::MediaCaps::Control` | variant | Represents the control alternative defined by `MediaCaps`. | `src/graph/ports.rs:91` |
| sym-da68d101d059b5a5bc51 | `pocketstation::graph::ports::MediaCaps::EncodedAudio` | variant | Represents the encoded audio alternative defined by `MediaCaps`. | `src/graph/ports.rs:87` |
| sym-e2123ea4bc9e388326af | `pocketstation::graph::ports::MediaCaps::Event` | variant | Represents the event alternative defined by `MediaCaps`. | `src/graph/ports.rs:89` |
| sym-b329ebcb74e0dac9d89f | `pocketstation::graph::ports::MediaCaps::Metrics` | variant | Represents the metrics alternative defined by `MediaCaps`. | `src/graph/ports.rs:90` |
| sym-3774a178201a97853423 | `pocketstation::graph::ports::MediaCaps::Text` | variant | Represents the text alternative defined by `MediaCaps`. | `src/graph/ports.rs:88` |
| sym-144970f23dbc54e8e28a | `pocketstation::graph::ports::MediaKind::AudioEncoded` | variant | Declares that the signal carries audio encoded media. | `src/graph/ports.rs:18` |
| sym-d9edff11348431abe853 | `pocketstation::graph::ports::MediaKind::AudioPcm` | variant | Declares that the signal carries audio PCM media. | `src/graph/ports.rs:17` |
| sym-99d04058aca60da4c0b2 | `pocketstation::graph::ports::MediaKind::Binary` | variant | Declares that the signal carries binary media. | `src/graph/ports.rs:23` |
| sym-e85d833fc66356e4ca5b | `pocketstation::graph::ports::MediaKind::Control` | variant | Declares that the signal carries control media. | `src/graph/ports.rs:22` |
| sym-ea71e4bbc5533fb65bfc | `pocketstation::graph::ports::MediaKind::Event` | variant | Declares that the signal carries event media. | `src/graph/ports.rs:20` |
| sym-b288855942f27d0fe647 | `pocketstation::graph::ports::MediaKind::Metrics` | variant | Declares that the signal carries metrics media. | `src/graph/ports.rs:21` |
| sym-14248c361d2b59c38895 | `pocketstation::graph::ports::MediaKind::Text` | variant | Declares that the signal carries text media. | `src/graph/ports.rs:19` |
| sym-b7e174cc4ef234976605 | `pocketstation::graph::ports::Multiplicity::Many` | variant | Represents the many alternative defined by `Multiplicity`. | `src/graph/ports.rs:171` |
| sym-a099754282605ec55bfa | `pocketstation::graph::ports::Multiplicity::One` | variant | Represents the one alternative defined by `Multiplicity`. | `src/graph/ports.rs:170` |
| sym-c5f2ed7b148489a800d1 | `pocketstation::graph::ports::PortDirection::Input` | variant | Declares a graph port as input. | `src/graph/ports.rs:164` |
| sym-989bc096e5a34e38004d | `pocketstation::graph::ports::PortDirection::Output` | variant | Declares a graph port as output. | `src/graph/ports.rs:165` |
| sym-a562439c1573c3c54b4a | `pocketstation::graph::ports::PortSpecError::EmptyName` | variant | Reports that name is empty. | `src/graph/ports.rs:241` |
| sym-adac67ffff9d19518044 | `pocketstation::graph::ports::PortSpecError::InvalidSignal` | variant | Reports that the supplied signal is invalid. | `src/graph/ports.rs:243` |
| sym-bc3d8d1d83279246ce4c | `pocketstation::graph::ports::PortSpecError::SignalMediaMismatch` | variant | Reports that signal media does not match the expected contract. | `src/graph/ports.rs:245` |
| sym-6f96bb9e1eeed32bed50 | `pocketstation::graph::registry::NodeDefinitionRef::Async` | variant | Represents the async alternative defined by `NodeDefinitionRef`. | `src/graph/registry.rs:34` |
| sym-1b99b6d0fb7ade453bf5 | `pocketstation::graph::registry::NodeDefinitionRef::Definition` | variant | Represents the definition alternative defined by `NodeDefinitionRef`. | `src/graph/registry.rs:35` |
| sym-fe9d52c892d31c1ef5f0 | `pocketstation::graph::registry::NodeDefinitionRef::Runtime` | variant | Represents the runtime alternative defined by `NodeDefinitionRef`. | `src/graph/registry.rs:33` |
| sym-e87b62668947de20815f | `pocketstation::graph::registry::NodeRegistrationError::DuplicateNodeType` | variant | Reports that node type duplicates an existing declaration or record. | `src/graph/registry.rs:61` |
| sym-fdee2066cc14e166583e | `pocketstation::graph::registry::NodeRegistrationError::DuplicateOperatorId` | variant | Reports that operator identifier duplicates an existing declaration or record. | `src/graph/registry.rs:63` |
| sym-9b3465f43c081890fe62 | `pocketstation::graph::registry::NodeRegistrationError::InvalidAsyncManifest` | variant | Reports that the supplied async manifest is invalid. | `src/graph/registry.rs:59` |
| sym-c89fc86ec565b62da30d | `pocketstation::graph::signal::continuity::SignalContinuityError::DiscontinuityRegressed` | variant | Reports that discontinuity moved backward instead of remaining monotonic. | `src/graph/signal/continuity.rs:101` |
| sym-dbdeb749b5fa07696513 | `pocketstation::graph::signal::continuity::SignalContinuityError::GenerationRegressed` | variant | Reports that generation moved backward instead of remaining monotonic. | `src/graph/signal/continuity.rs:103` |
| sym-b798eeb7163f9d5a5752 | `pocketstation::graph::signal::continuity::SignalContinuityError::IdentityChanged` | variant | Reports that identity changed across a boundary that requires stability. | `src/graph/signal/continuity.rs:95` |
| sym-ed20717ebe3b9b3def85 | `pocketstation::graph::signal::continuity::SignalContinuityError::InvalidEnvelope` | variant | Reports that the supplied envelope is invalid. | `src/graph/signal/continuity.rs:91` |
| sym-687792ad67a519b61f63 | `pocketstation::graph::signal::continuity::SignalContinuityError::MissingLineage` | variant | Reports that the required lineage is missing. | `src/graph/signal/continuity.rs:93` |
| sym-867c4f30b55e6e703842 | `pocketstation::graph::signal::continuity::SignalContinuityError::PolicyRegressed` | variant | Reports that policy moved backward instead of remaining monotonic. | `src/graph/signal/continuity.rs:107` |
| sym-6b6e35d98922707b53c3 | `pocketstation::graph::signal::continuity::SignalContinuityError::RecoveryWithoutDiscontinuity` | variant | Classifies a failure at the recovery without discontinuity stage or component of `SignalContinuityError`. | `src/graph/signal/continuity.rs:105` |
| sym-6ab1246a5d4158053b66 | `pocketstation::graph::signal::continuity::SignalContinuityError::SequenceGapWithoutDiscontinuity` | variant | Classifies a failure at the sequence gap without discontinuity stage or component of `SignalContinuityError`. | `src/graph/signal/continuity.rs:97` |
| sym-49f470362f2bdfe65399 | `pocketstation::graph::signal::continuity::SignalContinuityError::TimestampRegression` | variant | Reports that timestamp moved backward instead of remaining monotonic. | `src/graph/signal/continuity.rs:99` |
| sym-780625b7bc16456d747a | `pocketstation::graph::signal::envelope::SignalEnvelopeError::InvalidSignalSpec` | variant | Reports that the supplied signal spec is invalid. | `src/graph/signal/envelope.rs:139` |
| sym-7852face2eba75933b31 | `pocketstation::graph::signal::envelope::SignalEnvelopeError::PayloadSpecMismatch` | variant | Reports that payload spec does not match the expected contract. | `src/graph/signal/envelope.rs:141` |
| sym-25814dc7647868c5f76f | `pocketstation::graph::signal::envelope::SignalEnvelopeError::SequenceMismatch` | variant | Reports that sequence does not match the expected contract. | `src/graph/signal/envelope.rs:143` |
| sym-fba65b5a2830e7f34c3b | `pocketstation::graph::signal::envelope::SignalEnvelopeError::SourceMismatch` | variant | Reports that source does not match the expected contract. | `src/graph/signal/envelope.rs:145` |
| sym-144a4deb8e04e57af145 | `pocketstation::graph::signal::lineage::SignalDerivationError::EmptyOperatorId` | variant | Reports that operator identifier is empty. | `src/graph/signal/lineage.rs:163` |
| sym-ef6563dcacd18166771a | `pocketstation::graph::signal::lineage::SignalDerivationError::InvalidTimestampRange` | variant | Reports that the supplied timestamp range is invalid. | `src/graph/signal/lineage.rs:161` |
| sym-a4685751a6f25cbbcd2f | `pocketstation::graph::signal::lineage::SignalDerivationError::ZeroOperatorVersion` | variant | Reports that operator version must be greater than zero. | `src/graph/signal/lineage.rs:165` |
| sym-2f497f1f81a17f695cd1 | `pocketstation::graph::signal::lineage::SignalLineageError::ZeroSourceGeneration` | variant | Reports that source generation must be greater than zero. | `src/graph/signal/lineage.rs:88` |
| sym-3e4cf17fffad3a75604c | `pocketstation::graph::signal::operator::AsyncOperatorManifestError::DuplicateOutputRole` | variant | Reports that output role duplicates an existing declaration or record. | `src/graph/signal/operator.rs:363` |
| sym-af506a05fa79f2a48005 | `pocketstation::graph::signal::operator::AsyncOperatorManifestError::EmptyOperatorId` | variant | Reports that operator identifier is empty. | `src/graph/signal/operator.rs:323` |
| sym-ec23741dac5b34493e45 | `pocketstation::graph::signal::operator::AsyncOperatorManifestError::EmptyOutputRole` | variant | Reports that output role is empty. | `src/graph/signal/operator.rs:361` |
| sym-e9ee06982576ed0f8b34 | `pocketstation::graph::signal::operator::AsyncOperatorManifestError::InputEdgeMediaMismatch` | variant | Reports that input edge media does not match the expected contract. | `src/graph/signal/operator.rs:349` |
| sym-5d8b87826f9ed390d58a | `pocketstation::graph::signal::operator::AsyncOperatorManifestError::InputSignalMediaMismatch` | variant | Reports that input signal media does not match the expected contract. | `src/graph/signal/operator.rs:357` |
| sym-3528137a60929f65aefe | `pocketstation::graph::signal::operator::AsyncOperatorManifestError::InvalidInputSignal` | variant | Reports that the supplied input signal is invalid. | `src/graph/signal/operator.rs:353` |
| sym-c41dab63beeee3ac47de | `pocketstation::graph::signal::operator::AsyncOperatorManifestError::InvalidOutputSignal` | variant | Reports that the supplied output signal is invalid. | `src/graph/signal/operator.rs:355` |
| sym-aad52fe3b40e3348e2b7 | `pocketstation::graph::signal::operator::AsyncOperatorManifestError::InvalidSafetyContract` | variant | Reports that the supplied safety contract is invalid. | `src/graph/signal/operator.rs:335` |
| sym-92d61fc528f5796643db | `pocketstation::graph::signal::operator::AsyncOperatorManifestError::MissingInputPort` | variant | Reports that the required input port is missing. | `src/graph/signal/operator.rs:339` |
| sym-5ac417c893d199c3cd34 | `pocketstation::graph::signal::operator::AsyncOperatorManifestError::MissingOutputPort` | variant | Reports that the required output port is missing. | `src/graph/signal/operator.rs:341` |
| sym-13e844a696bb6d0760cc | `pocketstation::graph::signal::operator::AsyncOperatorManifestError::NetworkPermissionMismatch` | variant | Reports that network permission does not match the expected contract. | `src/graph/signal/operator.rs:337` |
| sym-01680fd1d72b1ac81c39 | `pocketstation::graph::signal::operator::AsyncOperatorManifestError::OutputEdgeMediaMismatch` | variant | Reports that output edge media does not match the expected contract. | `src/graph/signal/operator.rs:351` |
| sym-dd08c202282490ae7e49 | `pocketstation::graph::signal::operator::AsyncOperatorManifestError::OutputSignalMediaMismatch` | variant | Reports that output signal media does not match the expected contract. | `src/graph/signal/operator.rs:359` |
| sym-8cad0aca787acd99528a | `pocketstation::graph::signal::operator::AsyncOperatorManifestError::RealtimePartition` | variant | Classifies a failure at the realtime partition stage or component of `AsyncOperatorManifestError`. | `src/graph/signal/operator.rs:333` |
| sym-f976b690bf3bf7f78b4d | `pocketstation::graph::signal::operator::AsyncOperatorManifestError::TerminalOutputRoleNotAllowed` | variant | Classifies a failure at the terminal output role not allowed stage or component of `AsyncOperatorManifestError`. | `src/graph/signal/operator.rs:365` |
| sym-29dbb0d36dc936968c93 | `pocketstation::graph::signal::operator::AsyncOperatorManifestError::UnsupportedBackpressure` | variant | Reports that the requested backpressure is unsupported. | `src/graph/signal/operator.rs:343` |
| sym-f53383294d556586550d | `pocketstation::graph::signal::operator::AsyncOperatorManifestError::UnsupportedInputCopyPolicy` | variant | Reports that the requested input copy policy is unsupported. | `src/graph/signal/operator.rs:345` |
| sym-656d6ce03d568139e6b7 | `pocketstation::graph::signal::operator::AsyncOperatorManifestError::UnsupportedOutputBackpressure` | variant | Reports that the requested output backpressure is unsupported. | `src/graph/signal/operator.rs:347` |
| sym-b651a371f1d438bef8e3 | `pocketstation::graph::signal::operator::AsyncOperatorManifestError::ZeroGeneration` | variant | Reports that generation must be greater than zero. | `src/graph/signal/operator.rs:327` |
| sym-d37623147b3ecee1abff | `pocketstation::graph::signal::operator::AsyncOperatorManifestError::ZeroProcessTimeout` | variant | Reports that process timeout must be greater than zero. | `src/graph/signal/operator.rs:331` |
| sym-1d4c9d3b168aa12d58f0 | `pocketstation::graph::signal::operator::AsyncOperatorManifestError::ZeroQueueCapacity` | variant | Reports that queue capacity must be greater than zero. | `src/graph/signal/operator.rs:329` |
| sym-b34074f9ddb4365fe013 | `pocketstation::graph::signal::operator::AsyncOperatorManifestError::ZeroRevision` | variant | Reports that revision must be greater than zero. | `src/graph/signal/operator.rs:325` |
| sym-49155e40ce38cfe34f8f | `pocketstation::graph::signal::operator::OperatorCancellationPolicy::DiscardQueued` | variant | Cancels an operator using the discard queued policy. | `src/graph/signal/operator.rs:58` |
| sym-1d746b2d1e47b5037b12 | `pocketstation::graph::signal::operator::OperatorCancellationPolicy::DrainQueued` | variant | Cancels an operator using the drain queued policy. | `src/graph/signal/operator.rs:59` |
| sym-4ecf8ef62ea98a6ff444 | `pocketstation::graph::signal::operator::OperatorFailurePolicy::Continue` | variant | Handles an operator failure using the continue policy. | `src/graph/signal/operator.rs:64` |
| sym-2eae566628440871a3dd | `pocketstation::graph::signal::operator::OperatorFailurePolicy::StopWorker` | variant | Handles an operator failure using the stop worker policy. | `src/graph/signal/operator.rs:65` |
| sym-d490066419e2b338ba70 | `pocketstation::graph::signal::payload::SignalPayload::Audio` | variant | Represents the audio alternative defined by `SignalPayload`. | `src/graph/signal/payload.rs:11` |
| sym-f763de7220d8948a7b14 | `pocketstation::graph::signal::payload::SignalPayload::Bytes` | variant | Represents the bytes alternative defined by `SignalPayload`. | `src/graph/signal/payload.rs:13` |
| sym-b1184cca607eaba84924 | `pocketstation::graph::signal::payload::SignalPayload::Text` | variant | Represents the text alternative defined by `SignalPayload`. | `src/graph/signal/payload.rs:12` |
| sym-bae2f721ed715aa78b57 | `pocketstation::graph::signal::spec::BinaryFormat::Cbor` | variant | Declares the binary payload representation as cbor. | `src/graph/signal/spec.rs:145` |
| sym-21524d6cead6b2ada8a0 | `pocketstation::graph::signal::spec::BinaryFormat::Flatbuffers` | variant | Declares the binary payload representation as flatbuffers. | `src/graph/signal/spec.rs:144` |
| sym-22e6920058840214afef | `pocketstation::graph::signal::spec::BinaryFormat::Protobuf` | variant | Declares the binary payload representation as protobuf. | `src/graph/signal/spec.rs:143` |
| sym-d7496a05bba569ac462f | `pocketstation::graph::signal::spec::BinaryFormat::Raw` | variant | Declares the binary payload representation as raw. | `src/graph/signal/spec.rs:142` |
| sym-4b843e9167c325bbb8a6 | `pocketstation::graph::signal::spec::Codec::Aac` | variant | Represents the aac alternative defined by `Codec`. | `src/graph/signal/spec.rs:115` |
| sym-f270b0e40e4fbe1e102f | `pocketstation::graph::signal::spec::Codec::G711Alaw` | variant | Represents the g711 alaw alternative defined by `Codec`. | `src/graph/signal/spec.rs:118` |
| sym-1cf5a51a5ac16bb57ecd | `pocketstation::graph::signal::spec::Codec::G711Ulaw` | variant | Represents the g711 ulaw alternative defined by `Codec`. | `src/graph/signal/spec.rs:117` |
| sym-21864c2b6bb5f8c0a2de | `pocketstation::graph::signal::spec::Codec::Mp3` | variant | Represents the mp3 alternative defined by `Codec`. | `src/graph/signal/spec.rs:116` |
| sym-73bf6eded6a1cb57c885 | `pocketstation::graph::signal::spec::Codec::Opus` | variant | Represents the opus alternative defined by `Codec`. | `src/graph/signal/spec.rs:114` |
| sym-7c9a11a0cf1eb56fd765 | `pocketstation::graph::signal::spec::Codec::WebmOpus` | variant | Represents the webm opus alternative defined by `Codec`. | `src/graph/signal/spec.rs:119` |
| sym-4f84f23152922630cfe2 | `pocketstation::graph::signal::spec::EventFormat::Cbor` | variant | Identifies the cbor state or stage represented by `EventFormat`. | `src/graph/signal/spec.rs:136` |
| sym-72a7c88637b9599e2f66 | `pocketstation::graph::signal::spec::EventFormat::Flatbuffers` | variant | Identifies the flatbuffers state or stage represented by `EventFormat`. | `src/graph/signal/spec.rs:135` |
| sym-e674b8fbed55e83b91e0 | `pocketstation::graph::signal::spec::EventFormat::Json` | variant | Identifies the json state or stage represented by `EventFormat`. | `src/graph/signal/spec.rs:133` |
| sym-a694d6638015f466c731 | `pocketstation::graph::signal::spec::EventFormat::Protobuf` | variant | Identifies the protobuf state or stage represented by `EventFormat`. | `src/graph/signal/spec.rs:134` |
| sym-54dc3e94c6c40e2d1ff9 | `pocketstation::graph::signal::spec::SignalClass::Any` | variant | Wildcard accepted only at deliberately open graph boundaries. | `src/graph/signal/spec.rs:158` |
| sym-7e4dc7c8d423342764aa | `pocketstation::graph::signal::spec::SignalClass::Binary` | variant | Carries an opaque binary payload described by a `BinaryFormat`. | `src/graph/signal/spec.rs:172` |
| sym-fb9706e851009647aaa4 | `pocketstation::graph::signal::spec::SignalClass::Control` | variant | Graph control messages (route patches, session lifecycle, mute/unmute). | `src/graph/signal/spec.rs:170` |
| sym-8f543205e11a81da234e | `pocketstation::graph::signal::spec::SignalClass::Custom` | variant | Extension point for community / vendor signals. Use a stable reverse-domain identifier. | `src/graph/signal/spec.rs:175` |
| sym-d347e4209dd68cac73aa | `pocketstation::graph::signal::spec::SignalClass::EncodedAudio` | variant | Compressed audio bitstream (Opus packet, AAC frame, …). | `src/graph/signal/spec.rs:162` |
| sym-13c83ab529baf850deae | `pocketstation::graph::signal::spec::SignalClass::Event` | variant | Carries discrete event payloads described by an `EventFormat`. | `src/graph/signal/spec.rs:166` |
| sym-a27820ee6c4bbf2b3f23 | `pocketstation::graph::signal::spec::SignalClass::Metrics` | variant | Telemetry and observability counters / gauges. | `src/graph/signal/spec.rs:168` |
| sym-14c9ec8627dfdadcc51a | `pocketstation::graph::signal::spec::SignalClass::PcmAudio` | variant | Interleaved PCM audio samples (format described by the edge AudioCaps). | `src/graph/signal/spec.rs:160` |
| sym-aec8a764adf9ef08722f | `pocketstation::graph::signal::spec::SignalClass::Text` | variant | UTF-8 or structured text. | `src/graph/signal/spec.rs:164` |
| sym-fa66065de75cd997efc4 | `pocketstation::graph::signal::spec::SignalSpecError::EmptyCustomId` | variant | Reports that custom identifier is empty. | `src/graph/signal/spec.rs:353` |
| sym-1eb3bac3ee466bd13e82 | `pocketstation::graph::signal::spec::SignalSpecError::EmptyRole` | variant | Reports that role is empty. | `src/graph/signal/spec.rs:355` |
| sym-4959138498f204510d9e | `pocketstation::graph::signal::spec::SignalSpecError::EmptySchema` | variant | Reports that schema is empty. | `src/graph/signal/spec.rs:357` |
| sym-8d902b692d8ccaba534e | `pocketstation::graph::signal::spec::TextFormat::Json` | variant | Declares the text payload representation as json. | `src/graph/signal/spec.rs:126` |
| sym-9a8594743582ca74fbe2 | `pocketstation::graph::signal::spec::TextFormat::Markdown` | variant | Declares the text payload representation as markdown. | `src/graph/signal/spec.rs:127` |
| sym-6c6fb22ea05ff76f4984 | `pocketstation::graph::signal::spec::TextFormat::Utf8` | variant | Declares the text payload representation as utf8. | `src/graph/signal/spec.rs:125` |
| sym-e9437129c6ca0e87f480 | `pocketstation::graph::signal::timing::SignalTimingError::TimestampOverflow` | variant | Reports that timestamp exceeds its numeric range. | `src/graph/signal/timing.rs:93` |
| sym-a0f256e69c4e72881c2c | `pocketstation::graph::signal::timing::SignalTimingError::ZeroDuration` | variant | Reports that duration must be greater than zero. | `src/graph/signal/timing.rs:91` |
| sym-9ac15cf1a9fcb5fd0049 | `pocketstation::native_extension::NativeExtensionKind::Endpoint` | variant | Classifies the loaded native extension as endpoint. | `src/native_extension/mod.rs:30` |
| sym-cda90339325d8912240c | `pocketstation::native_extension::NativeExtensionKind::Operator` | variant | Classifies the loaded native extension as operator. | `src/native_extension/mod.rs:29` |
| sym-97a72cc297fac195637f | `pocketstation::native_extension::NativeExtensionKind::Source` | variant | Classifies the loaded native extension as source. | `src/native_extension/mod.rs:28` |
| sym-d04a12f5276e75e7fa39 | `pocketstation::native_extension::NativeExtensionLibraryErrorCode::DuplicateRegistration` | variant | Reports that registration duplicates an existing declaration or record. | `src/native_extension/mod.rs:92` |
| sym-0cb39a2ef4b2ec75b861 | `pocketstation::native_extension::NativeExtensionLibraryErrorCode::EntrypointFailed` | variant | Reports that entrypoint failed. | `src/native_extension/mod.rs:85` |
| sym-f0fbbf4e893a1816faed | `pocketstation::native_extension::NativeExtensionLibraryErrorCode::EntrypointMissing` | variant | Classifies a failure at the entrypoint missing stage or component of `NativeExtensionLibraryErrorCode`. | `src/native_extension/mod.rs:83` |
| sym-00f358f7b87a5ae34033 | `pocketstation::native_extension::NativeExtensionLibraryErrorCode::EntrypointPanicked` | variant | Reports that entrypoint panicked while the operation was active. | `src/native_extension/mod.rs:84` |
| sym-76d5dc8173f5517c54fd | `pocketstation::native_extension::NativeExtensionLibraryErrorCode::InvalidLibraryDescriptor` | variant | Reports that the supplied library descriptor is invalid. | `src/native_extension/mod.rs:88` |
| sym-c6031a1c816a569c5631 | `pocketstation::native_extension::NativeExtensionLibraryErrorCode::InvalidRegistration` | variant | Reports that the supplied registration is invalid. | `src/native_extension/mod.rs:91` |
| sym-a2c4921680a7f521aae7 | `pocketstation::native_extension::NativeExtensionLibraryErrorCode::LibraryLoadFailed` | variant | Reports that library load failed. | `src/native_extension/mod.rs:82` |
| sym-6fff5ddf0723aca51c02 | `pocketstation::native_extension::NativeExtensionLibraryErrorCode::PathCanonicalizationFailed` | variant | Reports that path canonicalization failed. | `src/native_extension/mod.rs:80` |
| sym-d5e9531372dc38b02581 | `pocketstation::native_extension::NativeExtensionLibraryErrorCode::PathNotAbsolute` | variant | Classifies a failure at the path not absolute stage or component of `NativeExtensionLibraryErrorCode`. | `src/native_extension/mod.rs:79` |
| sym-167af7ba4cf1f211d713 | `pocketstation::native_extension::NativeExtensionLibraryErrorCode::PathNotFile` | variant | Classifies a failure at the path not file stage or component of `NativeExtensionLibraryErrorCode`. | `src/native_extension/mod.rs:81` |
| sym-14602df76b6f8f5edabf | `pocketstation::native_extension::NativeExtensionLibraryErrorCode::RegistrationAcquisitionFailed` | variant | Reports that registration acquisition failed. | `src/native_extension/mod.rs:90` |
| sym-9ac83079cf775bb2358b | `pocketstation::native_extension::NativeExtensionLibraryErrorCode::RegistrationAcquisitionPanicked` | variant | Reports that registration acquisition panicked while the operation was active. | `src/native_extension/mod.rs:89` |
| sym-1c1a4d173444716528f0 | `pocketstation::native_extension::NativeExtensionLibraryErrorCode::RegistrationStateUnavailable` | variant | Reports that registration state is unavailable. | `src/native_extension/mod.rs:93` |
| sym-1c4a9e98244853258d35 | `pocketstation::native_extension::NativeExtensionLibraryErrorCode::UnsupportedAbiMajor` | variant | Reports that the requested ABI major is unsupported. | `src/native_extension/mod.rs:86` |
| sym-b3acb550909cebd73afb | `pocketstation::native_extension::NativeExtensionLibraryErrorCode::UnsupportedAbiMinor` | variant | Reports that the requested ABI minor is unsupported. | `src/native_extension/mod.rs:87` |
| sym-4139c58288925a6fd466 | `pocketstation::recording::config::PermissionDecision::Allowed` | variant | Represents the allowed alternative defined by `PermissionDecision`. | `src/recording/config.rs:44` |
| sym-2c42f67f70b486c879b7 | `pocketstation::recording::config::PermissionDecision::Denied` | variant | Represents the denied alternative defined by `PermissionDecision`. | `src/recording/config.rs:45` |
| sym-72dfabd62faca652bb06 | `pocketstation::recording::config::PermissionScope::SessionCaptureGrant` | variant | Limits the permission decision to session capture grant. | `src/recording/config.rs:51` |
| sym-d96127c11864649ebe7d | `pocketstation::recording::config::RecorderLineageField::Clock` | variant | Represents the clock alternative defined by `RecorderLineageField`. | `src/recording/config.rs:14` |
| sym-0367231a7b610784733d | `pocketstation::recording::config::RecorderLineageField::PermissionEpoch` | variant | Represents the permission epoch alternative defined by `RecorderLineageField`. | `src/recording/config.rs:16` |
| sym-650f042a3c53ea89a277 | `pocketstation::recording::config::RecorderLineageField::Session` | variant | Represents the session alternative defined by `RecorderLineageField`. | `src/recording/config.rs:11` |
| sym-a54152169703a18001ca | `pocketstation::recording::config::RecorderLineageField::Source` | variant | Represents the source alternative defined by `RecorderLineageField`. | `src/recording/config.rs:12` |
| sym-ee3be57514e2866b3493 | `pocketstation::recording::config::RecorderLineageField::SourceGeneration` | variant | Represents the source generation alternative defined by `RecorderLineageField`. | `src/recording/config.rs:15` |
| sym-2e5a5cbf66a695eace88 | `pocketstation::recording::config::RecorderLineageField::Stem` | variant | Represents the stem alternative defined by `RecorderLineageField`. | `src/recording/config.rs:13` |
| sym-de698bba8189939e1086 | `pocketstation::recording::error_code::RecordingErrorCode::DuplicateStemLabel` | variant | Reports that stem label duplicates an existing declaration or record. | `src/recording/error_code.rs:12` |
| sym-12b138a7064e71a0cda7 | `pocketstation::recording::error_code::RecordingErrorCode::FrameSpecMismatch` | variant | Reports that frame spec does not match the expected contract. | `src/recording/error_code.rs:18` |
| sym-e528bba8aa257bdd951c | `pocketstation::recording::error_code::RecordingErrorCode::GapTooLarge` | variant | Reports that gap exceeds the supported size limit. | `src/recording/error_code.rs:21` |
| sym-2b31271acad6cbf07679 | `pocketstation::recording::error_code::RecordingErrorCode::Incomplete` | variant | Reports that the operation ended without producing a complete terminal result. | `src/recording/error_code.rs:28` |
| sym-995c89d1d07551e2fd7a | `pocketstation::recording::error_code::RecordingErrorCode::InvalidSampleSpec` | variant | Reports that the supplied sample spec is invalid. | `src/recording/error_code.rs:15` |
| sym-94d294658d46c4f1d601 | `pocketstation::recording::error_code::RecordingErrorCode::InvalidStemLabel` | variant | Reports that the supplied stem label is invalid. | `src/recording/error_code.rs:11` |
| sym-023581c92d22cc90e089 | `pocketstation::recording::error_code::RecordingErrorCode::IoFailed` | variant | Reports that I/O failed. | `src/recording/error_code.rs:24` |
| sym-232b41897f6144e2296b | `pocketstation::recording::error_code::RecordingErrorCode::JsonFailed` | variant | Reports that json failed. | `src/recording/error_code.rs:26` |
| sym-bb95f5419b2243cec930 | `pocketstation::recording::error_code::RecordingErrorCode::LineageMismatch` | variant | Reports that lineage does not match the expected contract. | `src/recording/error_code.rs:17` |
| sym-f33ab449eb0d58eeeb14 | `pocketstation::recording::error_code::RecordingErrorCode::NotFinalized` | variant | Reports that no t finalized is available. | `src/recording/error_code.rs:27` |
| sym-e5f184f5829c9a168425 | `pocketstation::recording::error_code::RecordingErrorCode::OutputExists` | variant | Reports that output already exists and would be overwritten. | `src/recording/error_code.rs:10` |
| sym-9886bba784496517f781 | `pocketstation::recording::error_code::RecordingErrorCode::PermissionDenied` | variant | Reports that the required permission was denied. | `src/recording/error_code.rs:14` |
| sym-5f0acc1d064226c4e815 | `pocketstation::recording::error_code::RecordingErrorCode::SessionMismatch` | variant | Reports that session does not match the expected contract. | `src/recording/error_code.rs:13` |
| sym-c1cd6de5268ac4b8d885 | `pocketstation::recording::error_code::RecordingErrorCode::SourceMismatch` | variant | Reports that source does not match the expected contract. | `src/recording/error_code.rs:16` |
| sym-c12cfbbb8a6917e88723 | `pocketstation::recording::error_code::RecordingErrorCode::TimestampOutOfRange` | variant | Reports that timestamp falls outside the supported range. | `src/recording/error_code.rs:20` |
| sym-6388befc975243af8db7 | `pocketstation::recording::error_code::RecordingErrorCode::TooManyGaps` | variant | Reports that the number of gaps exceeds the supported limit. | `src/recording/error_code.rs:22` |
| sym-dff5f8d8a97ab6d32afa | `pocketstation::recording::error_code::RecordingErrorCode::UnalignedSamples` | variant | Reports that samples does not align to complete frames or channels. | `src/recording/error_code.rs:19` |
| sym-ef257966e67e1472fdfb | `pocketstation::recording::error_code::RecordingErrorCode::WavFailed` | variant | Reports that wav failed. | `src/recording/error_code.rs:25` |
| sym-0030708c2ea3eccf3813 | `pocketstation::recording::error_code::RecordingErrorCode::WorkerPanicked` | variant | Reports that worker panicked while the operation was active. | `src/recording/error_code.rs:23` |
| sym-c3ba172d10fca913b9d5 | `pocketstation::recording::writer::DiscontinuityKind::OverlapRejected` | variant | Classifies the observed stream discontinuity as overlap rejected. | `src/recording/writer.rs:108` |
| sym-21f0b61d2c70ae437768 | `pocketstation::recording::writer::DiscontinuityKind::SequenceGap` | variant | Classifies the observed stream discontinuity as sequence gap. | `src/recording/writer.rs:107` |
| sym-7d360341f21b92581d5c | `pocketstation::recording::writer::DiscontinuityKind::TimestampGap` | variant | Classifies the observed stream discontinuity as timestamp gap. | `src/recording/writer.rs:106` |
| sym-09a86bc1b2a3681702bc | `pocketstation::recording::writer::RecorderError::DuplicateStemLabel` | variant | Reports that stem label duplicates an existing declaration or record. | `src/recording/writer.rs:30` |
| sym-8ac9048e6258db768d31 | `pocketstation::recording::writer::RecorderError::FrameSpecMismatch` | variant | Reports that frame spec does not match the expected contract. | `src/recording/writer.rs:59` |
| sym-3bf811546a88f51368d8 | `pocketstation::recording::writer::RecorderError::GapTooLarge` | variant | Reports that gap exceeds the supported size limit. | `src/recording/writer.rs:71` |
| sym-ff6f9b3ee4fee92478db | `pocketstation::recording::writer::RecorderError::InvalidSampleSpec` | variant | Reports that the supplied sample spec is invalid. | `src/recording/writer.rs:40` |
| sym-7ea829746a98e1385e78 | `pocketstation::recording::writer::RecorderError::InvalidStemLabel` | variant | Reports that the supplied stem label is invalid. | `src/recording/writer.rs:28` |
| sym-8764eef568a0a7905493 | `pocketstation::recording::writer::RecorderError::Io` | variant | Reports an operating-system or filesystem I/O failure. | `src/recording/writer.rs:77` |
| sym-874aaad200ad7e3b0a87 | `pocketstation::recording::writer::RecorderError::Json` | variant | Reports that JSON serialization or parsing failed. | `src/recording/writer.rs:81` |
| sym-d67027b1a399227ee00e | `pocketstation::recording::writer::RecorderError::LineageMismatch` | variant | Reports that lineage does not match the expected contract. | `src/recording/writer.rs:52` |
| sym-0fa716dacfbf164f7d8d | `pocketstation::recording::writer::RecorderError::OutputExists` | variant | Reports that output already exists and would be overwritten. | `src/recording/writer.rs:26` |
| sym-1fedc505f7b4b3a39caf | `pocketstation::recording::writer::RecorderError::PermissionDenied` | variant | Reports that the required permission was denied. | `src/recording/writer.rs:38` |
| sym-59a79c0477a191febf84 | `pocketstation::recording::writer::RecorderError::SessionMismatch` | variant | Reports that session does not match the expected contract. | `src/recording/writer.rs:32` |
| sym-f0ab33824f7b789d3541 | `pocketstation::recording::writer::RecorderError::SourceMismatch` | variant | Reports that source does not match the expected contract. | `src/recording/writer.rs:46` |
| sym-0df6acf2d649c40b87cf | `pocketstation::recording::writer::RecorderError::TimestampOutOfRange` | variant | Reports that timestamp falls outside the supported range. | `src/recording/writer.rs:69` |
| sym-c1e195e527879d463869 | `pocketstation::recording::writer::RecorderError::TooManyGaps` | variant | Reports that the number of gaps exceeds the supported limit. | `src/recording/writer.rs:73` |
| sym-9b49168aab7a0155f7bf | `pocketstation::recording::writer::RecorderError::UnalignedSamples` | variant | Reports that samples does not align to complete frames or channels. | `src/recording/writer.rs:67` |
| sym-d45cc49d5cb4f2b6859b | `pocketstation::recording::writer::RecorderError::Wav` | variant | Classifies a failure at the wav stage or component of `RecorderError`. | `src/recording/writer.rs:79` |
| sym-21cbed79c30e8122d7b2 | `pocketstation::recording::writer::RecorderError::WorkerPanicked` | variant | Reports that worker panicked while the operation was active. | `src/recording/writer.rs:75` |
| sym-5b5bbe3d94b04ab61c8c | `pocketstation::recording::writer::RecordingState::Complete` | variant | Identifies the complete state or stage represented by `RecordingState`. | `src/recording/writer.rs:88` |
| sym-759e4b539d6888eb49cf | `pocketstation::recording::writer::RecordingState::Incomplete` | variant | Identifies the incomplete state or stage represented by `RecordingState`. | `src/recording/writer.rs:89` |
| sym-d8c7ab99450027b2acd0 | `pocketstation::recording::writer::RecordingState::Recording` | variant | Identifies the recording state or stage represented by `RecordingState`. | `src/recording/writer.rs:87` |
| sym-5a819dc3e8f740a03965 | `pocketstation::runtime::audio::executor::ExecError::Node` | variant | Reports that no de is available. | `src/runtime/audio/executor.rs:22` |
| sym-cb89114ad274e44db4fd | `pocketstation::runtime::audio::router::PlanEdgeFrame::Exclusive` | variant | Represents the exclusive alternative defined by `PlanEdgeFrame`. | `src/runtime/audio/router.rs:30` |
| sym-92c44ae0f4d92d48eaa5 | `pocketstation::runtime::audio::router::PlanEdgeFrame::Shared` | variant | Represents the shared alternative defined by `PlanEdgeFrame`. | `src/runtime/audio/router.rs:31` |
| sym-c528a2502defd76873e0 | `pocketstation::runtime::audio::router::PlanRouterError::InvalidFrameBytes` | variant | Reports that the supplied frame bytes is invalid. | `src/runtime/audio/router.rs:23` |
| sym-6c60784b27f69661890f | `pocketstation::runtime::audio::router::PlanRouterError::MissingMemoryPlan` | variant | Reports that the required memory plan is missing. | `src/runtime/audio/router.rs:19` |
| sym-07a190d8f79f14431828 | `pocketstation::runtime::audio::router::PlanRouterError::ZeroCapacity` | variant | Reports that capacity must be greater than zero. | `src/runtime/audio/router.rs:21` |
| sym-870e6a69061a05c29862 | `pocketstation::runtime::audio::runner::PlanRunnerDrainPolicy::DiscardQueued` | variant | Drains the runtime plan using the discard queued policy. | `src/runtime/audio/runner.rs:18` |
| sym-b923de7b83193c0fd2c1 | `pocketstation::runtime::audio::runner::PlanRunnerDrainPolicy::DrainQueued` | variant | Drains the runtime plan using the drain queued policy. | `src/runtime/audio/runner.rs:17` |
| sym-52671a6b6ac35844fe4a | `pocketstation::runtime::audio::runner::PlanRunnerError::AlreadyFinished` | variant | Reports that finished already occurred before this operation. | `src/runtime/audio/runner.rs:264` |
| sym-d01904ba27d1750fc69e | `pocketstation::runtime::audio::runner::PlanRunnerError::DuplicateSource` | variant | Reports that source duplicates an existing declaration or record. | `src/runtime/audio/runner.rs:260` |
| sym-6b7af025a8832ea937cb | `pocketstation::runtime::audio::runner::PlanRunnerError::Execution` | variant | Classifies a failure at the execution stage or component of `PlanRunnerError`. | `src/runtime/audio/runner.rs:266` |
| sym-fb2486bb4302356a5a2e | `pocketstation::runtime::audio::runner::PlanRunnerError::ZeroSourceCapacity` | variant | Reports that source capacity must be greater than zero. | `src/runtime/audio/runner.rs:258` |
| sym-b8a09d4893af867327d9 | `pocketstation::runtime::audio::runner::PlanRunnerError::ZeroWorkBudget` | variant | Reports that work budget must be greater than zero. | `src/runtime/audio/runner.rs:262` |
| sym-ea96c57d593cec1984f0 | `pocketstation::runtime::audio::runner::PlanSourceSendError::Cancelled` | variant | Indicates that the operation was cancelled. | `src/runtime/audio/runner.rs:117` |
| sym-11f8f542b7c1469731c9 | `pocketstation::runtime::audio::runner::PlanSourceSendError::Full` | variant | Reports that bounded capacity is full. | `src/runtime/audio/runner.rs:118` |
| sym-7c51ebb1a02c585c5a9e | `pocketstation::runtime::audio::runner::PlanSourceSendOutcome::Enqueued` | variant | Identifies the enqueued state or stage represented by `PlanSourceSendOutcome`. | `src/runtime/audio/runner.rs:124` |
| sym-8eead55a9d0ba9fef7b4 | `pocketstation::runtime::audio::runner::PlanSourceSendOutcome::Rejected` | variant | Identifies the rejected state or stage represented by `PlanSourceSendOutcome`. | `src/runtime/audio/runner.rs:125` |
| sym-06c7e2e634f072d7253e | `pocketstation::runtime::bridge::audio::GeneratedAudioBridgeStartError::InvalidPoolSlots` | variant | Reports that the supplied pool slots is invalid. | `src/runtime/bridge/audio.rs:52` |
| sym-f5fb2550ba9e78f18e84 | `pocketstation::runtime::bridge::audio::GeneratedAudioBridgeStartError::InvalidSampleSpec` | variant | Reports that the supplied sample spec is invalid. | `src/runtime/bridge/audio.rs:48` |
| sym-603999d381bec5fa9635 | `pocketstation::runtime::bridge::audio::GeneratedAudioBridgeStartError::ThreadStart` | variant | Classifies a failure at the thread start stage or component of `GeneratedAudioBridgeStartError`. | `src/runtime/bridge/audio.rs:54` |
| sym-35b4d25b6cdcf49875ab | `pocketstation::runtime::bridge::audio::GeneratedAudioBridgeStartError::ZeroFrameSamples` | variant | Reports that frame samples must be greater than zero. | `src/runtime/bridge/audio.rs:50` |
| sym-d3621da91a2ae18ccc92 | `pocketstation::runtime::lifecycle::async_host::AsyncRuntimeHostError::HostTimeout` | variant | Reports that host exceeded its deadline. | `src/runtime/lifecycle/async_host.rs:18` |
| sym-e4c39c4a5d710d9ffee3 | `pocketstation::runtime::lifecycle::async_host::AsyncRuntimeHostError::RuntimeStopped` | variant | Classifies a failure at the runtime stopped stage or component of `AsyncRuntimeHostError`. | `src/runtime/lifecycle/async_host.rs:14` |
| sym-6e84c5e34bbfc6c3c0ae | `pocketstation::runtime::lifecycle::async_host::AsyncRuntimeHostError::ShutdownPanicked` | variant | Reports that shutdown panicked while the operation was active. | `src/runtime/lifecycle/async_host.rs:16` |
| sym-d42c57f1ee58c7e7a2f9 | `pocketstation::runtime::lifecycle::async_host::AsyncRuntimeHostError::Start` | variant | Classifies a failure at the start stage or component of `AsyncRuntimeHostError`. | `src/runtime/lifecycle/async_host.rs:12` |
| sym-48e94d9b6f7bf50aaf03 | `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostError::AlreadyReaped` | variant | Reports that reaped already occurred before this operation. | `src/runtime/lifecycle/sidecar_host.rs:730` |
| sym-3943e9030e18921d2bd6 | `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostError::Closed` | variant | Reports that the underlying channel or resource is closed. | `src/runtime/lifecycle/sidecar_host.rs:706` |
| sym-f93a0c40f2dfde57a558 | `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostError::ControlQueueFull` | variant | Reports that the bounded control queue has no remaining capacity. | `src/runtime/lifecycle/sidecar_host.rs:704` |
| sym-ba7f3b928a2ff8e26f28 | `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostError::DataQueueFull` | variant | Reports that the bounded data queue has no remaining capacity. | `src/runtime/lifecycle/sidecar_host.rs:702` |
| sym-ee6a1235d0402c677787 | `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostError::FrameTooLarge` | variant | Reports that frame exceeds the supported size limit. | `src/runtime/lifecycle/sidecar_host.rs:700` |
| sym-210e2cd7b3ede546f8c8 | `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostError::InvalidConfiguration` | variant | Reports that the supplied configuration is invalid. | `src/runtime/lifecycle/sidecar_host.rs:688` |
| sym-8656c104d1107d78b2f9 | `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostError::InvalidDataKind` | variant | Reports that the supplied data kind is invalid. | `src/runtime/lifecycle/sidecar_host.rs:724` |
| sym-83ea871350867937ac64 | `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostError::InvalidState` | variant | Reports that the supplied state is invalid. | `src/runtime/lifecycle/sidecar_host.rs:719` |
| sym-3624fc2ce3d10959daf0 | `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostError::Io` | variant | Reports an operating-system or filesystem I/O failure. | `src/runtime/lifecycle/sidecar_host.rs:696` |
| sym-78f7fdc086c82d650799 | `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostError::Kill` | variant | Classifies a failure at the kill stage or component of `SidecarHostError`. | `src/runtime/lifecycle/sidecar_host.rs:728` |
| sym-02921c193de0110ad5d7 | `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostError::MissingPipe` | variant | Reports that the required pipe is missing. | `src/runtime/lifecycle/sidecar_host.rs:694` |
| sym-3593919359009664913f | `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostError::ProcessingTimeout` | variant | Reports that processing exceeded its deadline. | `src/runtime/lifecycle/sidecar_host.rs:717` |
| sym-70ee5aa505c6d068acff | `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostError::Protocol` | variant | Classifies a failure at the protocol stage or component of `SidecarHostError`. | `src/runtime/lifecycle/sidecar_host.rs:698` |
| sym-5860d0101e18350d019a | `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostError::Spawn` | variant | Classifies a failure at the spawn stage or component of `SidecarHostError`. | `src/runtime/lifecycle/sidecar_host.rs:690` |
| sym-3950b51e2db1a60ae07b | `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostError::ThreadSpawn` | variant | Classifies a failure at the thread spawn stage or component of `SidecarHostError`. | `src/runtime/lifecycle/sidecar_host.rs:692` |
| sym-9bc083cc834a304af97f | `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostError::Timeout` | variant | Reports that the operation exceeded its deadline. | `src/runtime/lifecycle/sidecar_host.rs:715` |
| sym-b3f0ef67620688beffb7 | `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostError::UnexpectedEof` | variant | Reports that eof is not valid in the current protocol or lifecycle state. | `src/runtime/lifecycle/sidecar_host.rs:708` |
| sym-2d51107d019e8ce6f3ec | `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostError::UnexpectedMessage` | variant | Reports that message is not valid in the current protocol or lifecycle state. | `src/runtime/lifecycle/sidecar_host.rs:710` |
| sym-fb1af15a29588f31d777 | `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostError::UnknownSidecar` | variant | Reports that the referenced sidecar is not declared or registered. | `src/runtime/lifecycle/sidecar_host.rs:732` |
| sym-c968d1237744236ff33a | `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostError::Wait` | variant | Classifies a failure at the wait stage or component of `SidecarHostError`. | `src/runtime/lifecycle/sidecar_host.rs:726` |
| sym-98b71b087807fbe6d0f9 | `pocketstation::runtime::lifecycle::sidecar_host::SidecarState::Cancelling` | variant | Identifies the cancelling state or stage represented by `SidecarState`. | `src/runtime/lifecycle/sidecar_host.rs:28` |
| sym-b35955b5a6d903c7ff3f | `pocketstation::runtime::lifecycle::sidecar_host::SidecarState::Closed` | variant | Reports that the underlying channel or resource is closed. | `src/runtime/lifecycle/sidecar_host.rs:30` |
| sym-bb371c42e2ca010adec0 | `pocketstation::runtime::lifecycle::sidecar_host::SidecarState::Closing` | variant | Identifies the closing state or stage represented by `SidecarState`. | `src/runtime/lifecycle/sidecar_host.rs:29` |
| sym-dd2fe6d9b8c16e56c06c | `pocketstation::runtime::lifecycle::sidecar_host::SidecarState::Configure` | variant | Identifies the configure state or stage represented by `SidecarState`. | `src/runtime/lifecycle/sidecar_host.rs:25` |
| sym-77c1e69dcf9d321d156b | `pocketstation::runtime::lifecycle::sidecar_host::SidecarState::Failed` | variant | Identifies the failed state or stage represented by `SidecarState`. | `src/runtime/lifecycle/sidecar_host.rs:32` |
| sym-b2fc6ec6f01ef2cc75a9 | `pocketstation::runtime::lifecycle::sidecar_host::SidecarState::Hello` | variant | Identifies the hello state or stage represented by `SidecarState`. | `src/runtime/lifecycle/sidecar_host.rs:23` |
| sym-ebe5b755fba06e4bbf58 | `pocketstation::runtime::lifecycle::sidecar_host::SidecarState::Manifest` | variant | Identifies the manifest state or stage represented by `SidecarState`. | `src/runtime/lifecycle/sidecar_host.rs:24` |
| sym-2ad353f39591099a2c54 | `pocketstation::runtime::lifecycle::sidecar_host::SidecarState::Ready` | variant | Identifies the ready state or stage represented by `SidecarState`. | `src/runtime/lifecycle/sidecar_host.rs:26` |
| sym-ff9a8d66f2357a00cb7d | `pocketstation::runtime::lifecycle::sidecar_host::SidecarState::Reaped` | variant | Identifies the reaped state or stage represented by `SidecarState`. | `src/runtime/lifecycle/sidecar_host.rs:31` |
| sym-a7debc2882d39bc40f06 | `pocketstation::runtime::lifecycle::sidecar_host::SidecarState::Running` | variant | Identifies the running state or stage represented by `SidecarState`. | `src/runtime/lifecycle/sidecar_host.rs:27` |
| sym-7b9ab57cfca2eb380c48 | `pocketstation::runtime::lifecycle::sidecar_host::SidecarState::Spawned` | variant | Identifies the spawned state or stage represented by `SidecarState`. | `src/runtime/lifecycle/sidecar_host.rs:22` |
| sym-cea265cf91efde5f0fd5 | `pocketstation::runtime::lifecycle::sidecar_protocol::SidecarMessageKind::Cancel` | variant | Identifies a sidecar protocol message carrying or representing cancel. | `src/runtime/lifecycle/sidecar_protocol.rs:13` |
| sym-3c2397d374ee8daa2de9 | `pocketstation::runtime::lifecycle::sidecar_protocol::SidecarMessageKind::Close` | variant | Identifies a sidecar protocol message carrying or representing close. | `src/runtime/lifecycle/sidecar_protocol.rs:14` |
| sym-bcc3f3f4d208c529f21f | `pocketstation::runtime::lifecycle::sidecar_protocol::SidecarMessageKind::Closed` | variant | Reports that the underlying channel or resource is closed. | `src/runtime/lifecycle/sidecar_protocol.rs:19` |
| sym-eb061b82f6bd6993ab67 | `pocketstation::runtime::lifecycle::sidecar_protocol::SidecarMessageKind::Configure` | variant | Identifies a sidecar protocol message carrying or representing configure. | `src/runtime/lifecycle/sidecar_protocol.rs:17` |
| sym-8b102af3e02188156830 | `pocketstation::runtime::lifecycle::sidecar_protocol::SidecarMessageKind::Error` | variant | Identifies a sidecar protocol message carrying or representing error. | `src/runtime/lifecycle/sidecar_protocol.rs:12` |
| sym-b8543b6d1144c225f146 | `pocketstation::runtime::lifecycle::sidecar_protocol::SidecarMessageKind::Hello` | variant | Identifies a sidecar protocol message carrying or representing hello. | `src/runtime/lifecycle/sidecar_protocol.rs:15` |
| sym-bc261cf66401018775f5 | `pocketstation::runtime::lifecycle::sidecar_protocol::SidecarMessageKind::Manifest` | variant | Identifies a sidecar protocol message carrying or representing manifest. | `src/runtime/lifecycle/sidecar_protocol.rs:16` |
| sym-ac93cfe319fbf303683d | `pocketstation::runtime::lifecycle::sidecar_protocol::SidecarMessageKind::Observation` | variant | Identifies a sidecar protocol message carrying or representing observation. | `src/runtime/lifecycle/sidecar_protocol.rs:18` |
| sym-f3e7f0d64487f7c71ac6 | `pocketstation::runtime::lifecycle::sidecar_protocol::SidecarMessageKind::Ready` | variant | Identifies a sidecar protocol message carrying or representing ready. | `src/runtime/lifecycle/sidecar_protocol.rs:11` |
| sym-afa9864dabf9b4f4661a | `pocketstation::runtime::lifecycle::sidecar_protocol::SidecarMessageKind::Signal` | variant | Identifies a sidecar protocol message carrying or representing signal. | `src/runtime/lifecycle/sidecar_protocol.rs:10` |
| sym-e47be507d6c17ce9c951 | `pocketstation::runtime::lifecycle::sidecar_protocol::SidecarProtocolError::EmptySignalId` | variant | Reports that signal identifier is empty. | `src/runtime/lifecycle/sidecar_protocol.rs:310` |
| sym-eedd9c5bd4244db36b1c | `pocketstation::runtime::lifecycle::sidecar_protocol::SidecarProtocolError::FieldTooLarge` | variant | Reports that field exceeds the supported size limit. | `src/runtime/lifecycle/sidecar_protocol.rs:312` |
| sym-b2553dd50f6bb5c15102 | `pocketstation::runtime::lifecycle::sidecar_protocol::SidecarProtocolError::FrameLengthOverflow` | variant | Reports that frame length exceeds its numeric range. | `src/runtime/lifecycle/sidecar_protocol.rs:320` |
| sym-c8ab5048b836f4cbb004 | `pocketstation::runtime::lifecycle::sidecar_protocol::SidecarProtocolError::FrameTooLarge` | variant | Reports that frame exceeds the supported size limit. | `src/runtime/lifecycle/sidecar_protocol.rs:322` |
| sym-c776ce9db61e889adf4f | `pocketstation::runtime::lifecycle::sidecar_protocol::SidecarProtocolError::InvalidMagic` | variant | Reports that the supplied magic is invalid. | `src/runtime/lifecycle/sidecar_protocol.rs:298` |
| sym-152b6911249796607d69 | `pocketstation::runtime::lifecycle::sidecar_protocol::SidecarProtocolError::InvalidTerminal` | variant | Reports that the supplied terminal is invalid. | `src/runtime/lifecycle/sidecar_protocol.rs:306` |
| sym-97f2c4ee4c4d865ad2a0 | `pocketstation::runtime::lifecycle::sidecar_protocol::SidecarProtocolError::InvalidUtf8` | variant | Reports that the supplied utf8 is invalid. | `src/runtime/lifecycle/sidecar_protocol.rs:318` |
| sym-790730d520feec658b24 | `pocketstation::runtime::lifecycle::sidecar_protocol::SidecarProtocolError::ReservedFieldSet` | variant | Reports that a reserved compatibility field contains a nonzero value. | `src/runtime/lifecycle/sidecar_protocol.rs:308` |
| sym-2e68fd6e83f3630090b0 | `pocketstation::runtime::lifecycle::sidecar_protocol::SidecarProtocolError::TrailingBytes` | variant | Reports that bytes remain after decoding the complete record. | `src/runtime/lifecycle/sidecar_protocol.rs:296` |
| sym-35ac3b3f0c3d3b1912b9 | `pocketstation::runtime::lifecycle::sidecar_protocol::SidecarProtocolError::Truncated` | variant | Reports that the encoded input ended before the complete record was available. | `src/runtime/lifecycle/sidecar_protocol.rs:294` |
| sym-9e7ff05fcd844cc5b7ef | `pocketstation::runtime::lifecycle::sidecar_protocol::SidecarProtocolError::UnknownMessageKind` | variant | Reports that the referenced message kind is not declared or registered. | `src/runtime/lifecycle/sidecar_protocol.rs:304` |
| sym-419bde89c5abf1a3932b | `pocketstation::runtime::lifecycle::sidecar_protocol::SidecarProtocolError::UnsupportedMajor` | variant | Reports that the requested major is unsupported. | `src/runtime/lifecycle/sidecar_protocol.rs:300` |
| sym-146e5e3075ce49830536 | `pocketstation::runtime::lifecycle::sidecar_protocol::SidecarProtocolError::UnsupportedMinor` | variant | Reports that the requested minor is unsupported. | `src/runtime/lifecycle/sidecar_protocol.rs:302` |
| sym-ac46736da1b66c100826 | `pocketstation::runtime::signal::edge::TypedEdgeBuildError::CapacityTooLarge` | variant | Reports that capacity exceeds the supported size limit. | `src/runtime/signal/edge.rs:392` |
| sym-4dd9fd2cf1db65e865cf | `pocketstation::runtime::signal::edge::TypedEdgeBuildError::MissingPayloadLimit` | variant | Reports that the required payload limit is missing. | `src/runtime/signal/edge.rs:397` |
| sym-044424384df71e6614e8 | `pocketstation::runtime::signal::edge::TypedEdgeBuildError::NoBranches` | variant | Reports that no branches is available. | `src/runtime/signal/edge.rs:388` |
| sym-2543e853dd65aa0b705a | `pocketstation::runtime::signal::edge::TypedEdgeBuildError::PayloadLimitTooLarge` | variant | Reports that payload limit exceeds the supported size limit. | `src/runtime/signal/edge.rs:401` |
| sym-438090c37005ed68185f | `pocketstation::runtime::signal::edge::TypedEdgeBuildError::ZeroCapacity` | variant | Reports that capacity must be greater than zero. | `src/runtime/signal/edge.rs:390` |
| sym-a85a03453ad3db8eac12 | `pocketstation::runtime::signal::edge::TypedEdgeBuildError::ZeroPayloadLimit` | variant | Reports that payload limit must be greater than zero. | `src/runtime/signal/edge.rs:399` |
| sym-a1106f8035060c84516c | `pocketstation::runtime::signal::edge::TypedEdgePublishError::InvalidEnvelope` | variant | Reports that the supplied envelope is invalid. | `src/runtime/signal/edge.rs:412` |
| sym-6fd729840cdb0d512c9c | `pocketstation::runtime::signal::edge::TypedEdgePublishError::NoBranches` | variant | Reports that no branches is available. | `src/runtime/signal/edge.rs:410` |
| sym-8ce3cc8d50f71d344f12 | `pocketstation::runtime::signal::edge::TypedEdgePublishError::PayloadTooLarge` | variant | Reports that payload exceeds the supported size limit. | `src/runtime/signal/edge.rs:416` |
| sym-d8f7417da4714b42124e | `pocketstation::runtime::signal::edge::TypedEdgePublishError::RequiredBranchFull` | variant | Reports that the bounded required branch has no remaining capacity. | `src/runtime/signal/edge.rs:422` |
| sym-f0d5f8994f69bb9f5a24 | `pocketstation::runtime::signal::error::AsyncOperatorWorkerError::AmbiguousOutputPort` | variant | Reports that output port resolves to more than one candidate. | `src/runtime/signal/error.rs:36` |
| sym-241ae48aff5ad83ec495 | `pocketstation::runtime::signal::error::AsyncOperatorWorkerError::Cancel` | variant | Classifies a failure at the cancel stage or component of `AsyncOperatorWorkerError`. | `src/runtime/signal/error.rs:20` |
| sym-ba32254853328634e478 | `pocketstation::runtime::signal::error::AsyncOperatorWorkerError::CancelTimeout` | variant | Reports that cancel exceeded its deadline. | `src/runtime/signal/error.rs:22` |
| sym-1522810f4538e57ce853 | `pocketstation::runtime::signal::error::AsyncOperatorWorkerError::Close` | variant | Classifies a failure at the close stage or component of `AsyncOperatorWorkerError`. | `src/runtime/signal/error.rs:16` |
| sym-39a2ed9fe4d8ab5114ac | `pocketstation::runtime::signal::error::AsyncOperatorWorkerError::CloseTimeout` | variant | Reports that close exceeded its deadline. | `src/runtime/signal/error.rs:18` |
| sym-6c041adb0e23a17f9e9f | `pocketstation::runtime::signal::error::AsyncOperatorWorkerError::DerivedLineageMismatch` | variant | Reports that derived lineage does not match the expected contract. | `src/runtime/signal/error.rs:26` |
| sym-969371c37ead3d7ff707 | `pocketstation::runtime::signal::error::AsyncOperatorWorkerError::InvalidPlanInput` | variant | Reports that the supplied plan input is invalid. | `src/runtime/signal/error.rs:50` |
| sym-aa8f0a179b3f0af54bae | `pocketstation::runtime::signal::error::AsyncOperatorWorkerError::Join` | variant | Classifies a failure at the join stage or component of `AsyncOperatorWorkerError`. | `src/runtime/signal/error.rs:48` |
| sym-d7ae594f4af2e4be6590 | `pocketstation::runtime::signal::error::AsyncOperatorWorkerError::MissingDerivedLineage` | variant | Reports that the required derived lineage is missing. | `src/runtime/signal/error.rs:24` |
| sym-9d3ec9ad1f04060a0cfa | `pocketstation::runtime::signal::error::AsyncOperatorWorkerError::MissingOutputContract` | variant | Reports that the required output contract is missing. | `src/runtime/signal/error.rs:32` |
| sym-0fec72c43db425c4dec6 | `pocketstation::runtime::signal::error::AsyncOperatorWorkerError::OutputPayloadTooLarge` | variant | Reports that output payload exceeds the supported size limit. | `src/runtime/signal/error.rs:42` |
| sym-391f22c13af125383bf3 | `pocketstation::runtime::signal::error::AsyncOperatorWorkerError::OutputSignalMismatch` | variant | Reports that output signal does not match the expected contract. | `src/runtime/signal/error.rs:28` |
| sym-01e4adb2edddb7f4fd6c | `pocketstation::runtime::signal::error::AsyncOperatorWorkerError::PlanInputLineageMismatch` | variant | Reports that plan input lineage does not match the expected contract. | `src/runtime/signal/error.rs:52` |
| sym-c1b0151afe950a2873c1 | `pocketstation::runtime::signal::error::AsyncOperatorWorkerError::Prepare` | variant | Classifies a failure at the prepare stage or component of `AsyncOperatorWorkerError`. | `src/runtime/signal/error.rs:8` |
| sym-00045953b1dff3b05962 | `pocketstation::runtime::signal::error::AsyncOperatorWorkerError::PrepareTimeout` | variant | Reports that prepare exceeded its deadline. | `src/runtime/signal/error.rs:10` |
| sym-1eac3fa80a43dbaba980 | `pocketstation::runtime::signal::error::AsyncOperatorWorkerError::Process` | variant | Classifies a failure at the process stage or component of `AsyncOperatorWorkerError`. | `src/runtime/signal/error.rs:12` |
| sym-5132e5c456e23a858ebe | `pocketstation::runtime::signal::error::AsyncOperatorWorkerError::SharedAudioTypedInput` | variant | Classifies a failure at the shared audio typed input stage or component of `AsyncOperatorWorkerError`. | `src/runtime/signal/error.rs:54` |
| sym-56941d2df5660c003984 | `pocketstation::runtime::signal::error::AsyncOperatorWorkerError::TerminalOutputDropped` | variant | Reports that terminal output was dropped before delivery completed. | `src/runtime/signal/error.rs:38` |
| sym-d2a19ae613cbd6bc3d6b | `pocketstation::runtime::signal::error::AsyncOperatorWorkerError::Timeout` | variant | Reports that the operation exceeded its deadline. | `src/runtime/signal/error.rs:14` |
| sym-b31f3651046484fd8947 | `pocketstation::runtime::signal::error::AsyncOperatorWorkerError::UndeclaredOutputRole` | variant | Reports that output role was emitted or requested without a declaration. | `src/runtime/signal/error.rs:30` |
| sym-975b64b06170a65dfb77 | `pocketstation::runtime::signal::error::AsyncOperatorWorkerError::UnknownInputPort` | variant | Reports that the referenced input port is not declared or registered. | `src/runtime/signal/error.rs:34` |
| sym-b40b3a6c1c2db888dada | `pocketstation::session::compile::error::SessionCompileError::AmbiguousEndpointInput` | variant | Reports that endpoint input resolves to more than one candidate. | `src/session/compile/error.rs:27` |
| sym-2fb81d0081b525e86d71 | `pocketstation::session::compile::error::SessionCompileError::AmbiguousOperatorPort` | variant | Reports that operator port resolves to more than one candidate. | `src/session/compile/error.rs:32` |
| sym-cf2d2ab96b9476ccc51c | `pocketstation::session::compile::error::SessionCompileError::AudioBridgeOutputNotExclusive` | variant | Reports that audio bridge output must have exclusive ownership but is shared. | `src/session/compile/error.rs:57` |
| sym-69a37d13f4f219ca4a6a | `pocketstation::session::compile::error::SessionCompileError::DuplicateOperatorInputConnection` | variant | Reports that operator input connection duplicates an existing declaration or record. | `src/session/compile/error.rs:62` |
| sym-c1d494e2162c38faa05f | `pocketstation::session::compile::error::SessionCompileError::GraphCompile` | variant | Classifies a failure at the graph compile stage or component of `SessionCompileError`. | `src/session/compile/error.rs:88` |
| sym-2661bdf9d207ca6b836c | `pocketstation::session::compile::error::SessionCompileError::InvalidAudioBridgeOutput` | variant | Reports that the supplied audio bridge output is invalid. | `src/session/compile/error.rs:50` |
| sym-a95bb06bbf7347778c11 | `pocketstation::session::compile::error::SessionCompileError::InvalidExternalSourceConfiguration` | variant | Reports that the supplied external source configuration is invalid. | `src/session/compile/error.rs:78` |
| sym-34c58d625854dc5306ae | `pocketstation::session::compile::error::SessionCompileError::InvalidSpec` | variant | Reports that the supplied spec is invalid. | `src/session/compile/error.rs:9` |
| sym-9bc5888abe950e04718c | `pocketstation::session::compile::error::SessionCompileError::MissingRequiredOperatorInput` | variant | Reports that the required required operator input is missing. | `src/session/compile/error.rs:43` |
| sym-51eaaa829cfd33da6abe | `pocketstation::session::compile::error::SessionCompileError::OperatorNodeTypeMismatch` | variant | Reports that operator node type does not match the expected contract. | `src/session/compile/error.rs:15` |
| sym-973be2d42b4eebb3e7f0 | `pocketstation::session::compile::error::SessionCompileError::RuntimePlan` | variant | Classifies a failure at the runtime plan stage or component of `SessionCompileError`. | `src/session/compile/error.rs:90` |
| sym-86c351e9d6b92e53240d | `pocketstation::session::compile::error::SessionCompileError::UnknownAsyncOperator` | variant | Reports that the referenced async operator is not declared or registered. | `src/session/compile/error.rs:21` |
| sym-f2257e9008d83c5266b6 | `pocketstation::session::compile::error::SessionCompileError::UnknownEndpointInputPort` | variant | Reports that the referenced endpoint input port is not declared or registered. | `src/session/compile/error.rs:83` |
| sym-c713ca8eab49857013d1 | `pocketstation::session::compile::error::SessionCompileError::UnknownEndpointNodeType` | variant | Reports that the referenced endpoint node type is not declared or registered. | `src/session/compile/error.rs:23` |
| sym-d772c446a527fbb40ee8 | `pocketstation::session::compile::error::SessionCompileError::UnknownExternalSource` | variant | Reports that the referenced external source is not declared or registered. | `src/session/compile/error.rs:69` |
| sym-2a2b40a1c3c0d5bf1182 | `pocketstation::session::compile::error::SessionCompileError::UnknownExternalSourceOutput` | variant | Reports that the referenced external source output is not declared or registered. | `src/session/compile/error.rs:73` |
| sym-c359f4f9f64d751fbf29 | `pocketstation::session::compile::error::SessionCompileError::UnknownOperator` | variant | Reports that the referenced operator is not declared or registered. | `src/session/compile/error.rs:11` |
| sym-331c30d7001199547804 | `pocketstation::session::compile::error::SessionCompileError::UnknownOperatorPort` | variant | Reports that the referenced operator port is not declared or registered. | `src/session/compile/error.rs:37` |
| sym-847fcd49364d22b54c38 | `pocketstation::session::compile::error::SessionCompileError::UnknownSourceNodeType` | variant | Reports that the referenced source node type is not declared or registered. | `src/session/compile/error.rs:67` |
| sym-60231a09dcef3bdc8026 | `pocketstation::session::declaration::selector::ApplicationSelector::BundleId` | variant | Selects applications by bundle identifier. | `src/session/declaration/selector.rs:33` |
| sym-42e15e484d08d514cced | `pocketstation::session::declaration::selector::ApplicationSelector::Name` | variant | Selects applications by name. | `src/session/declaration/selector.rs:40` |
| sym-96dc4c2dd1aacafefefa | `pocketstation::session::declaration::selector::ApplicationSelector::ProcessId` | variant | Selects applications by process identifier. | `src/session/declaration/selector.rs:34` |
| sym-327b8d84447899380674 | `pocketstation::session::declaration::selector::ApplicationSelector::ProcessInstance` | variant | Selects applications by process instance. | `src/session/declaration/selector.rs:35` |
| sym-51374dba6f4b71666f99 | `pocketstation::session::declaration::selector::ApplicationSelector::StableId` | variant | Selects applications by stable identifier. | `src/session/declaration/selector.rs:39` |
| sym-87148ff78c3efd67eda1 | `pocketstation::session::declaration::selector::DeviceSelector::Default` | variant | Selects an audio device by default. | `src/session/declaration/selector.rs:108` |
| sym-75870b74add45cd650af | `pocketstation::session::declaration::selector::DeviceSelector::Id` | variant | Selects an audio device by id. | `src/session/declaration/selector.rs:109` |
| sym-2553e6ca33b75c0564b3 | `pocketstation::session::declaration::selector::Source::Application` | variant | Represents the application alternative defined by `Source`. | `src/session/declaration/selector.rs:135` |
| sym-5779688823a30b24c8d9 | `pocketstation::session::declaration::selector::Source::Microphone` | variant | Represents the microphone alternative defined by `Source`. | `src/session/declaration/selector.rs:136` |
| sym-a9983cce70b766b5629f | `pocketstation::session::declaration::spec::ConnectionTarget::EndpointInput` | variant | Represents the endpoint input alternative defined by `ConnectionTarget`. | `src/session/declaration/spec.rs:229` |
| sym-3c45ca933d9b71e00bba | `pocketstation::session::declaration::spec::ConnectionTarget::OperatorInput` | variant | Represents the operator input alternative defined by `ConnectionTarget`. | `src/session/declaration/spec.rs:225` |
| sym-11c0074872f2eb776fe2 | `pocketstation::session::declaration::spec::StreamOrigin::OperatorOutput` | variant | Represents the operator output alternative defined by `StreamOrigin`. | `src/session/declaration/spec.rs:216` |
| sym-3d22fd4add6c32db0e21 | `pocketstation::session::declaration::spec::StreamOrigin::SourceOutput` | variant | Represents the source output alternative defined by `StreamOrigin`. | `src/session/declaration/spec.rs:210` |
| sym-570927c4848d78aeb6a4 | `pocketstation::session::declaration::spec::StreamOrigin::Stem` | variant | Represents the stem alternative defined by `StreamOrigin`. | `src/session/declaration/spec.rs:209` |
| sym-9156cb5bba7fe4079013 | `pocketstation::session::declaration::typed_stream::TypedStreamError::AmbiguousPort` | variant | Reports that port resolves to more than one candidate. | `src/session/declaration/typed_stream.rs:203` |
| sym-61370ad91046fb4f6e19 | `pocketstation::session::declaration::typed_stream::TypedStreamError::InputSignalMismatch` | variant | Reports that input signal does not match the expected contract. | `src/session/declaration/typed_stream.rs:205` |
| sym-a00323bb8c9996fc86b9 | `pocketstation::session::declaration::typed_stream::TypedStreamError::InvalidManifest` | variant | Reports that the supplied manifest is invalid. | `src/session/declaration/typed_stream.rs:189` |
| sym-45729fff396769ee009b | `pocketstation::session::declaration::typed_stream::TypedStreamError::InvalidSignal` | variant | Reports that the supplied signal is invalid. | `src/session/declaration/typed_stream.rs:187` |
| sym-6b287a7e702d1b874366 | `pocketstation::session::declaration::typed_stream::TypedStreamError::MissingPort` | variant | Reports that the required port is missing. | `src/session/declaration/typed_stream.rs:201` |
| sym-318b652a62de47814002 | `pocketstation::session::declaration::typed_stream::TypedStreamError::OperatorIdentityMismatch` | variant | Reports that operator identity does not match the expected contract. | `src/session/declaration/typed_stream.rs:191` |
| sym-4f2bb5252e9556d87c34 | `pocketstation::session::declaration::typed_stream::TypedStreamError::OutputSignalMismatch` | variant | Reports that output signal does not match the expected contract. | `src/session/declaration/typed_stream.rs:207` |
| sym-5302c39919e36a8d811e | `pocketstation::session::declaration::typed_stream::TypedStreamError::Session` | variant | Classifies a failure at the session stage or component of `TypedStreamError`. | `src/session/declaration/typed_stream.rs:213` |
| sym-62559d9f1576d1746242 | `pocketstation::session::declaration::typed_stream::TypedStreamError::StemRequiresPcmAudio` | variant | Classifies a failure at the stem requires PCM audio stage or component of `TypedStreamError`. | `src/session/declaration/typed_stream.rs:209` |
| sym-480a7706b04b48709f06 | `pocketstation::session::declaration::typed_stream::TypedStreamError::StreamInputMismatch` | variant | Reports that stream input does not match the expected contract. | `src/session/declaration/typed_stream.rs:211` |
| sym-041368ec2b26667b952a | `pocketstation::session::declaration::typed_stream::TypedStreamError::UnknownPort` | variant | Reports that the referenced port is not declared or registered. | `src/session/declaration/typed_stream.rs:196` |
| sym-b08b1b1933cbd4a4e33e | `pocketstation::session::error::SessionError::DraftFrozen` | variant | Classifies a failure at the draft frozen stage or component of `SessionError`. | `src/session/error.rs:36` |
| sym-d297958c199af65f521a | `pocketstation::session::error::SessionError::DraftPoisoned` | variant | Reports that shared draft became unavailable after a panic while locked. | `src/session/error.rs:38` |
| sym-5980c2a46080f6f7ac38 | `pocketstation::session::error::SessionError::ForeignEndpoint` | variant | Reports that endpoint belongs to a different owning Session or declaration. | `src/session/error.rs:31` |
| sym-2aef3e65812454e21eec | `pocketstation::session::error::SessionError::IdExhausted` | variant | Reports that the available id range or capacity is exhausted. | `src/session/error.rs:40` |
| sym-03817c4e66f3ed09d800 | `pocketstation::session::error::SessionError::InvalidEndpoint` | variant | Reports that the supplied endpoint is invalid. | `src/session/error.rs:25` |
| sym-41bd336c7ce591c09595 | `pocketstation::session::error::SessionError::InvalidOperator` | variant | Reports that the supplied operator is invalid. | `src/session/error.rs:27` |
| sym-f307e4dcfe70df92a33f | `pocketstation::session::error::SessionError::InvalidRoute` | variant | Reports that the supplied route is invalid. | `src/session/error.rs:29` |
| sym-50cc2d1dea46d1a09dc2 | `pocketstation::session::error::SessionError::InvalidSelector` | variant | Reports that the supplied selector is invalid. | `src/session/error.rs:23` |
| sym-780b39a8acd682f843d3 | `pocketstation::session::error::SessionError::NoRoutes` | variant | Reports that no routes is available. | `src/session/error.rs:10` |
| sym-6966b00db62c70e34daa | `pocketstation::session::error::SessionError::NoSourceOutputRoutes` | variant | Reports that no source output routes is available. | `src/session/error.rs:18` |
| sym-93e4b372124125427596 | `pocketstation::session::error::SessionError::NoSourceOutputs` | variant | Reports that no source outputs is available. | `src/session/error.rs:12` |
| sym-7bc0a7c271e6ed409c5b | `pocketstation::session::error::SessionError::NoSources` | variant | Reports that no sources is available. | `src/session/error.rs:8` |
| sym-846502c99b7e6f8a7d38 | `pocketstation::session::error::SessionError::OperatorHasNoDestination` | variant | Classifies a failure at the operator has no destination stage or component of `SessionError`. | `src/session/error.rs:63` |
| sym-ca4f45bf8358ac078f31 | `pocketstation::session::error::SessionError::UnknownEndpoint` | variant | Reports that the referenced endpoint is not declared or registered. | `src/session/error.rs:44` |
| sym-96ea82235867ebcdc165 | `pocketstation::session::error::SessionError::UnknownOperatorInstance` | variant | Reports that the referenced operator instance is not declared or registered. | `src/session/error.rs:59` |
| sym-9e0f1292b5cf6c4b3ba0 | `pocketstation::session::error::SessionError::UnknownSourceInstance` | variant | Reports that the referenced source instance is not declared or registered. | `src/session/error.rs:48` |
| sym-bed71ed97a407423ffc7 | `pocketstation::session::error::SessionError::UnknownSourceOutput` | variant | Reports that the referenced source output is not declared or registered. | `src/session/error.rs:54` |
| sym-9dc3384961cb81b7f406 | `pocketstation::session::error::SessionError::UnknownStem` | variant | Reports that the referenced stem is not declared or registered. | `src/session/error.rs:46` |
| sym-cc366e16a6f7b47d1708 | `pocketstation::session::error::SessionError::UnsupportedVersion` | variant | Reports that the requested version is unsupported. | `src/session/error.rs:42` |
| sym-5c0543801bad78e98116 | `pocketstation::session::error_code::PolledAudioPollErrorCode::Empty` | variant | Represents an empty value or collection. | `src/session/error_code.rs:132` |
| sym-e988153aa6683b34936d | `pocketstation::session::error_code::PolledAudioPollErrorCode::InternalStateUnavailable` | variant | Reports that internal state is unavailable. | `src/session/error_code.rs:134` |
| sym-ada9e9c1441fecf8efa3 | `pocketstation::session::error_code::PolledAudioPollErrorCode::LeaseCapacityExhausted` | variant | Reports that the available lease capacity range or capacity is exhausted. | `src/session/error_code.rs:133` |
| sym-631fd4f8a4e05bba9658 | `pocketstation::session::error_code::SessionDeclarationErrorCode::DraftFrozen` | variant | Classifies a failure at the draft frozen stage or component of `SessionDeclarationErrorCode`. | `src/session/error_code.rs:19` |
| sym-270640cd5f8df59db426 | `pocketstation::session::error_code::SessionDeclarationErrorCode::ForeignEndpoint` | variant | Reports that endpoint belongs to a different owning Session or declaration. | `src/session/error_code.rs:18` |
| sym-75ed1c8cd1404c3da001 | `pocketstation::session::error_code::SessionDeclarationErrorCode::IdExhausted` | variant | Reports that the available id range or capacity is exhausted. | `src/session/error_code.rs:21` |
| sym-b1b1874aca7a557a7f41 | `pocketstation::session::error_code::SessionDeclarationErrorCode::InternalStateUnavailable` | variant | Reports that internal state is unavailable. | `src/session/error_code.rs:20` |
| sym-c96e50871de738168e7a | `pocketstation::session::error_code::SessionDeclarationErrorCode::InvalidEndpoint` | variant | Reports that the supplied endpoint is invalid. | `src/session/error_code.rs:15` |
| sym-8edd69071432107821f1 | `pocketstation::session::error_code::SessionDeclarationErrorCode::InvalidOperator` | variant | Reports that the supplied operator is invalid. | `src/session/error_code.rs:16` |
| sym-796a84d98343745113b7 | `pocketstation::session::error_code::SessionDeclarationErrorCode::InvalidRoute` | variant | Reports that the supplied route is invalid. | `src/session/error_code.rs:17` |
| sym-c63321d7827bf03fbe28 | `pocketstation::session::error_code::SessionDeclarationErrorCode::InvalidSelector` | variant | Reports that the supplied selector is invalid. | `src/session/error_code.rs:14` |
| sym-46f6b0a4a246b49e7d5f | `pocketstation::session::error_code::SessionDeclarationErrorCode::NoRoutes` | variant | Reports that no routes is available. | `src/session/error_code.rs:12` |
| sym-872614134bb6ad345c14 | `pocketstation::session::error_code::SessionDeclarationErrorCode::NoSourceOutputs` | variant | Reports that no source outputs is available. | `src/session/error_code.rs:13` |
| sym-4fadf9454590373542b9 | `pocketstation::session::error_code::SessionDeclarationErrorCode::NoSources` | variant | Reports that no sources is available. | `src/session/error_code.rs:11` |
| sym-51a3c059c48f057bb646 | `pocketstation::session::error_code::SessionDeclarationErrorCode::OperatorHasNoDestination` | variant | Classifies a failure at the operator has no destination stage or component of `SessionDeclarationErrorCode`. | `src/session/error_code.rs:27` |
| sym-9e307e267cd4621ed5b3 | `pocketstation::session::error_code::SessionDeclarationErrorCode::UnknownEndpoint` | variant | Reports that the referenced endpoint is not declared or registered. | `src/session/error_code.rs:23` |
| sym-514a04d19c7800ae3a3a | `pocketstation::session::error_code::SessionDeclarationErrorCode::UnknownOperatorInstance` | variant | Reports that the referenced operator instance is not declared or registered. | `src/session/error_code.rs:26` |
| sym-dc85798562b0ac8c91a9 | `pocketstation::session::error_code::SessionDeclarationErrorCode::UnknownSource` | variant | Reports that the referenced source is not declared or registered. | `src/session/error_code.rs:25` |
| sym-f805f9f63a3f590c70d0 | `pocketstation::session::error_code::SessionDeclarationErrorCode::UnknownStem` | variant | Reports that the referenced stem is not declared or registered. | `src/session/error_code.rs:24` |
| sym-2b52d170a7c5154bcab4 | `pocketstation::session::error_code::SessionDeclarationErrorCode::UnsupportedVersion` | variant | Reports that the requested version is unsupported. | `src/session/error_code.rs:22` |
| sym-597903c4631814258f2f | `pocketstation::session::error_code::SessionRuntimeErrorCode::MissingMetricsSnapshot` | variant | Reports that the required metrics snapshot is missing. | `src/session/error_code.rs:117` |
| sym-a10a0e8fbfd4b41d746f | `pocketstation::session::error_code::SessionStartErrorCode::CaptureBackendFailed` | variant | Reports that capture backend failed. | `src/session/error_code.rs:76` |
| sym-2d58ceef04fa7c5dc002 | `pocketstation::session::error_code::SessionStartErrorCode::CapturePermissionDenied` | variant | Reports that capture permission was denied by the active permission or policy boundary. | `src/session/error_code.rs:73` |
| sym-dfdc446e623cbf57187c | `pocketstation::session::error_code::SessionStartErrorCode::CaptureSourceUnavailable` | variant | Reports that capture source is unavailable. | `src/session/error_code.rs:74` |
| sym-37ed062b12bcf5dcffa6 | `pocketstation::session::error_code::SessionStartErrorCode::CaptureUnsupported` | variant | Reports that capture is unsupported by the active backend or contract. | `src/session/error_code.rs:75` |
| sym-755a8daa9159864e84b8 | `pocketstation::session::error_code::SessionStartErrorCode::CompileFailed` | variant | Reports that compile failed. | `src/session/error_code.rs:67` |
| sym-87211654eae969cd6d15 | `pocketstation::session::error_code::SessionStartErrorCode::DeclarationInvalid` | variant | Classifies a failure at the declaration invalid stage or component of `SessionStartErrorCode`. | `src/session/error_code.rs:66` |
| sym-929151acf11059010796 | `pocketstation::session::error_code::SessionStartErrorCode::EndpointPrepareFailed` | variant | Reports that endpoint prepare failed. | `src/session/error_code.rs:72` |
| sym-072e1aa00c2e92c205e3 | `pocketstation::session::error_code::SessionStartErrorCode::EndpointStartFailed` | variant | Reports that endpoint start failed. | `src/session/error_code.rs:77` |
| sym-503715b1bc8dcae459fd | `pocketstation::session::error_code::SessionStartErrorCode::HostSetupFailed` | variant | Reports that host setup failed. | `src/session/error_code.rs:62` |
| sym-719949038f21c73b1f1e | `pocketstation::session::error_code::SessionStartErrorCode::InvalidSelector` | variant | Reports that the supplied selector is invalid. | `src/session/error_code.rs:65` |
| sym-5b07d43072f9db2f49c9 | `pocketstation::session::error_code::SessionStartErrorCode::InvalidStartOptions` | variant | Reports that the supplied start options is invalid. | `src/session/error_code.rs:69` |
| sym-fdd39f6b258858922ffc | `pocketstation::session::error_code::SessionStartErrorCode::MissingAudioReceipt` | variant | Reports that the required audio receipt is missing. | `src/session/error_code.rs:79` |
| sym-4961e8e01d23ccbd9557 | `pocketstation::session::error_code::SessionStartErrorCode::MissingEndpointDeclaration` | variant | Reports that the required endpoint declaration is missing. | `src/session/error_code.rs:71` |
| sym-70cd7cf39f9a9bfa71cd | `pocketstation::session::error_code::SessionStartErrorCode::MissingEventReceiver` | variant | Reports that the required event receiver is missing. | `src/session/error_code.rs:81` |
| sym-3756899957d3aa8c2a42 | `pocketstation::session::error_code::SessionStartErrorCode::MissingRecordingConfiguration` | variant | Reports that the required recording configuration is missing. | `src/session/error_code.rs:80` |
| sym-674bd0639bfb841eb418 | `pocketstation::session::error_code::SessionStartErrorCode::RuntimePrepareFailed` | variant | Reports that runtime prepare failed. | `src/session/error_code.rs:68` |
| sym-51119758f37e964ac191 | `pocketstation::session::error_code::SessionStartErrorCode::RuntimeStartFailed` | variant | Reports that runtime start failed. | `src/session/error_code.rs:78` |
| sym-114615f09ce97d9ced49 | `pocketstation::session::error_code::SessionStartErrorCode::StartCancelled` | variant | Reports that start was cancelled before completion. | `src/session/error_code.rs:64` |
| sym-bc26dad157c159db1d1a | `pocketstation::session::error_code::SessionStartErrorCode::TraceRecorderSetupFailed` | variant | Reports that trace recorder setup failed. | `src/session/error_code.rs:82` |
| sym-f8af2a20c5696bb55576 | `pocketstation::session::error_code::SessionStartErrorCode::UnsupportedPlatform` | variant | Reports that the requested platform is unsupported. | `src/session/error_code.rs:63` |
| sym-12d3b673f27be4c50737 | `pocketstation::session::error_code::SessionStartErrorCode::UnsupportedSourceTopology` | variant | Reports that the requested source topology is unsupported. | `src/session/error_code.rs:70` |
| sym-23144c73e539cbbeee43 | `pocketstation::session::error_code::SessionStopCode::AlreadyStopped` | variant | Indicates that the operation had already stopped. | `src/session/error_code.rs:152` |
| sym-6242216e9bab4ca878a2 | `pocketstation::session::error_code::SessionStopCode::StopFailed` | variant | Represents the stop failed alternative defined by `SessionStopCode`. | `src/session/error_code.rs:153` |
| sym-dba7ae1d469e00e1ba03 | `pocketstation::session::error_code::SessionStopCode::Stopped` | variant | Indicates that the operation stopped normally. | `src/session/error_code.rs:151` |
| sym-a2e1f4fae03ce7bfe353 | `pocketstation::session::error_code::SessionStopFailureCode::CaptureFinalizationFailed` | variant | Reports that capture finalization failed. | `src/session/error_code.rs:173` |
| sym-4a58e13b1395cddd92c0 | `pocketstation::session::error_code::SessionStopFailureCode::EndpointFinalizationFailed` | variant | Reports that endpoint finalization failed. | `src/session/error_code.rs:175` |
| sym-386d14a7a012dd0d3773 | `pocketstation::session::error_code::SessionStopFailureCode::LineageFailed` | variant | Reports that lineage failed. | `src/session/error_code.rs:177` |
| sym-4641d83181e32d7f9832 | `pocketstation::session::error_code::SessionStopFailureCode::OperatorFinalizationFailed` | variant | Reports that operator finalization failed. | `src/session/error_code.rs:174` |
| sym-ef88f18b35bd1b360b2d | `pocketstation::session::error_code::SessionStopFailureCode::RuntimeFailed` | variant | Reports that runtime failed. | `src/session/error_code.rs:176` |
| sym-e0768ed75cb90782fa38 | `pocketstation::session::error_code::SessionStopFailureCode::RuntimeWorkerPanicked` | variant | Reports that runtime worker panicked while the operation was active. | `src/session/error_code.rs:172` |
| sym-ff6d1a76053a12f49d07 | `pocketstation::session::error_code::SessionStopFailureCode::SourceSendRejected` | variant | Reports that source send was rejected by the destination contract. | `src/session/error_code.rs:178` |
| sym-4da94015876cade61990 | `pocketstation::session::extensions::audio_input::AudioInputConfigError::FrameSampleCountOverflow` | variant | Reports that frame sample count exceeds its numeric range. | `src/session/extensions/audio_input/mod.rs:89` |
| sym-591fdc99f30018b0c21d | `pocketstation::session::extensions::audio_input::AudioInputConfigError::InvalidCapacity` | variant | Reports that the supplied capacity is invalid. | `src/session/extensions/audio_input/mod.rs:85` |
| sym-9e09b3cbf9ffe0abd197 | `pocketstation::session::extensions::audio_input::AudioInputConfigError::UnsupportedChannelCount` | variant | Reports that the requested channel count is unsupported. | `src/session/extensions/audio_input/mod.rs:81` |
| sym-5e5f170b6a4f5ff0892b | `pocketstation::session::extensions::audio_input::AudioInputConfigError::UnsupportedSampleFormat` | variant | Reports that the requested sample format is unsupported. | `src/session/extensions/audio_input/mod.rs:83` |
| sym-d9350dc8658e4be8ddb0 | `pocketstation::session::extensions::audio_input::AudioInputConfigError::ZeroFrameSamples` | variant | Reports that frame samples must be greater than zero. | `src/session/extensions/audio_input/mod.rs:87` |
| sym-c83458fecd0e649b6f8f | `pocketstation::session::extensions::audio_input::AudioInputConfigError::ZeroSampleRate` | variant | Reports that sample rate must be greater than zero. | `src/session/extensions/audio_input/mod.rs:79` |
| sym-b0913a07732cba8bf854 | `pocketstation::session::extensions::audio_input::buffer::AudioInputBufferAcquireError::Cancelled` | variant | Indicates that the operation was cancelled. | `src/session/extensions/audio_input/buffer.rs:277` |
| sym-c34ace60b6a80eaa5a03 | `pocketstation::session::extensions::audio_input::buffer::AudioInputBufferAcquireError::Closed` | variant | Reports that the underlying channel or resource is closed. | `src/session/extensions/audio_input/buffer.rs:275` |
| sym-d5113f31b21b5edec63f | `pocketstation::session::extensions::audio_input::buffer::AudioInputBufferAcquireError::Full` | variant | Reports that bounded capacity is full. | `src/session/extensions/audio_input/buffer.rs:273` |
| sym-f6cade642798b8d9fc50 | `pocketstation::session::extensions::audio_input::buffer::AudioInputBufferError::Capacity` | variant | Classifies a failure at the capacity stage or component of `AudioInputBufferError`. | `src/session/extensions/audio_input/buffer.rs:294` |
| sym-09ed3d3c184591ebddaf | `pocketstation::session::extensions::audio_input::buffer::AudioInputBufferError::Empty` | variant | Represents an empty value or collection. | `src/session/extensions/audio_input/buffer.rs:285` |
| sym-c8837fcbf9233c99d38d | `pocketstation::session::extensions::audio_input::buffer::AudioInputBufferError::MisalignedChannels` | variant | Reports that channels does not satisfy the required alignment. | `src/session/extensions/audio_input/buffer.rs:287` |
| sym-1c7f771004b95c8e39a8 | `pocketstation::session::extensions::audio_input::buffer::AudioInputBufferError::WrongFrameLength` | variant | Reports that frame length does not match the required identity or contract. | `src/session/extensions/audio_input/buffer.rs:289` |
| sym-f06aab3468af90e8884d | `pocketstation::session::extensions::audio_input::buffer::AudioInputBufferError::WrongSource` | variant | Reports that source does not match the required identity or contract. | `src/session/extensions/audio_input/buffer.rs:283` |
| sym-abe41b58f107b7d89664 | `pocketstation::session::extensions::audio_input::buffer::AudioInputWriteErrorKind::Cancelled` | variant | Indicates that the operation was cancelled. | `src/session/extensions/audio_input/buffer.rs:301` |
| sym-c0d1e9d8d746634cef01 | `pocketstation::session::extensions::audio_input::buffer::AudioInputWriteErrorKind::Closed` | variant | Reports that the underlying channel or resource is closed. | `src/session/extensions/audio_input/buffer.rs:300` |
| sym-6c29f4e13ee1191271b9 | `pocketstation::session::extensions::audio_input::buffer::AudioInputWriteErrorKind::Full` | variant | Reports that bounded capacity is full. | `src/session/extensions/audio_input/buffer.rs:299` |
| sym-df69e26d995c5b45d9ae | `pocketstation::session::extensions::audio_input::buffer::AudioInputWriteErrorKind::InvalidBuffer` | variant | Classifies an external-audio write failure as invalid buffer. | `src/session/extensions/audio_input/buffer.rs:302` |
| sym-8de391c649b8deb2a2b6 | `pocketstation::session::extensions::audio_input::source::AudioInputError::Configuration` | variant | Classifies a failure at the configuration stage or component of `AudioInputError`. | `src/session/extensions/audio_input/source.rs:87` |
| sym-95dccf16ed37e27ba4af | `pocketstation::session::extensions::audio_input::source::AudioInputError::IncompatibleContract` | variant | Reports that contract is incompatible with the required contract. | `src/session/extensions/audio_input/source.rs:99` |
| sym-6e29bc80625b23b4c02b | `pocketstation::session::extensions::audio_input::source::AudioInputError::InstanceIdentityExhausted` | variant | Reports that the available instance identity range or capacity is exhausted. | `src/session/extensions/audio_input/source.rs:101` |
| sym-3f84085c2a71af093817 | `pocketstation::session::extensions::audio_input::source::AudioInputError::Manifest` | variant | Classifies a failure at the manifest stage or component of `AudioInputError`. | `src/session/extensions/audio_input/source.rs:91` |
| sym-3cf6c24669e863886d26 | `pocketstation::session::extensions::audio_input::source::AudioInputError::RegistrationStateUnavailable` | variant | Reports that registration state is unavailable. | `src/session/extensions/audio_input/source.rs:95` |
| sym-6cb92e808d350333f8e3 | `pocketstation::session::extensions::audio_input::source::AudioInputError::Session` | variant | Classifies a failure at the session stage or component of `AudioInputError`. | `src/session/extensions/audio_input/source.rs:93` |
| sym-3aadd690dcaba3805450 | `pocketstation::session::extensions::audio_input::source::AudioInputError::SourceTypeId` | variant | Classifies a failure at the source type identifier stage or component of `AudioInputError`. | `src/session/extensions/audio_input/source.rs:89` |
| sym-af6ac511596ee4fde4b0 | `pocketstation::session::extensions::builtins::SessionGraphRegistrationError::DuplicateNodeType` | variant | Reports that node type duplicates an existing declaration or record. | `src/session/extensions/builtins.rs:32` |
| sym-5a4109b0ddc8b6582e11 | `pocketstation::session::extensions::source::SourceDriverError::Failed` | variant | Reports that the requested operation failed. | `src/session/extensions/source.rs:750` |
| sym-22139d595e77e7cd1c23 | `pocketstation::session::extensions::source::SourceManifestError::DuplicateOutputName` | variant | Reports that output name duplicates an existing declaration or record. | `src/session/extensions/source.rs:689` |
| sym-5430c727f24ee697769b | `pocketstation::session::extensions::source::SourceManifestError::EmptyOutputName` | variant | Reports that output name is empty. | `src/session/extensions/source.rs:687` |
| sym-bcf6b3a47d6a52862e2e | `pocketstation::session::extensions::source::SourceManifestError::EmptySourceTypeId` | variant | Reports that source type identifier is empty. | `src/session/extensions/source.rs:679` |
| sym-5a24784ba6dd4e14ab9d | `pocketstation::session::extensions::source::SourceManifestError::InvalidSafetyContract` | variant | Reports that the supplied safety contract is invalid. | `src/session/extensions/source.rs:695` |
| sym-227ec305d84c139889a9 | `pocketstation::session::extensions::source::SourceManifestError::InvalidSignal` | variant | Reports that the supplied signal is invalid. | `src/session/extensions/source.rs:691` |
| sym-7ad561f1bcad628f48cb | `pocketstation::session::extensions::source::SourceManifestError::NoOutputs` | variant | Reports that no outputs is available. | `src/session/extensions/source.rs:683` |
| sym-adc2b7259c5bd104522b | `pocketstation::session::extensions::source::SourceManifestError::NonOutputPort` | variant | Reports that no n output port is available. | `src/session/extensions/source.rs:685` |
| sym-ae7c786c8351272dfe9a | `pocketstation::session::extensions::source::SourceManifestError::SignalMediaMismatch` | variant | Reports that signal media does not match the expected contract. | `src/session/extensions/source.rs:693` |
| sym-010ce7c3b8cfc1be5650 | `pocketstation::session::extensions::source::SourceManifestError::UnsupportedExecutionPartition` | variant | Reports that the requested execution partition is unsupported. | `src/session/extensions/source.rs:697` |
| sym-91a351e59966b9fc75f3 | `pocketstation::session::extensions::source::SourceManifestError::ZeroVersion` | variant | Reports that version must be greater than zero. | `src/session/extensions/source.rs:681` |
| sym-807f79275055aa688c31 | `pocketstation::session::extensions::source::SourceRegistrationError::DuplicateSourceType` | variant | Reports that source type duplicates an existing declaration or record. | `src/session/extensions/source.rs:705` |
| sym-20b9fa1926ad42b59be5 | `pocketstation::session::extensions::source::SourceRegistrationError::InvalidManifest` | variant | Reports that the supplied manifest is invalid. | `src/session/extensions/source.rs:703` |
| sym-46e631c833c2583b09d1 | `pocketstation::session::extensions::source::SourceRegistrationError::NodeTypeConflict` | variant | Reports that no de type conflict is available. | `src/session/extensions/source.rs:707` |
| sym-c2df6923ad2d5e5f6d03 | `pocketstation::session::extensions::source::SourceRuntimeError::AlreadyJoined` | variant | Reports that joined already occurred before this operation. | `src/session/extensions/source.rs:784` |
| sym-545f86eb541b09952a31 | `pocketstation::session::extensions::source::SourceRuntimeError::Continuity` | variant | Classifies a failure at the continuity stage or component of `SourceRuntimeError`. | `src/session/extensions/source.rs:776` |
| sym-e6d8214b8dd6026e38b7 | `pocketstation::session::extensions::source::SourceRuntimeError::Driver` | variant | Classifies a failure at the driver stage or component of `SourceRuntimeError`. | `src/session/extensions/source.rs:760` |
| sym-3cfcc2924dbd6b8edef7 | `pocketstation::session::extensions::source::SourceRuntimeError::EdgeBuild` | variant | Classifies a failure at the edge build stage or component of `SourceRuntimeError`. | `src/session/extensions/source.rs:762` |
| sym-946f9e8948d0ab5c28f9 | `pocketstation::session::extensions::source::SourceRuntimeError::InvalidConfiguration` | variant | Reports that the supplied configuration is invalid. | `src/session/extensions/source.rs:758` |
| sym-297cef30f8ab3399e0a3 | `pocketstation::session::extensions::source::SourceRuntimeError::InvalidManifest` | variant | Reports that the supplied manifest is invalid. | `src/session/extensions/source.rs:756` |
| sym-a14eeed0cf81b7fbbfea | `pocketstation::session::extensions::source::SourceRuntimeError::MissingSessionLineage` | variant | Reports that the required session lineage is missing. | `src/session/extensions/source.rs:772` |
| sym-da3136d44a0f9546a22f | `pocketstation::session::extensions::source::SourceRuntimeError::NoRoutedOutputs` | variant | Reports that no routed outputs is available. | `src/session/extensions/source.rs:764` |
| sym-f5a2e9e0333ab5fb51f4 | `pocketstation::session::extensions::source::SourceRuntimeError::OutputContractMismatch` | variant | Reports that output contract does not match the expected contract. | `src/session/extensions/source.rs:770` |
| sym-0bb276d039ca208065a0 | `pocketstation::session::extensions::source::SourceRuntimeError::OutputIdentityMismatch` | variant | Reports that output identity does not match the expected contract. | `src/session/extensions/source.rs:774` |
| sym-7ef3195390dbc162444e | `pocketstation::session::extensions::source::SourceRuntimeError::PreparedStateConsumed` | variant | Classifies a failure at the prepared state consumed stage or component of `SourceRuntimeError`. | `src/session/extensions/source.rs:786` |
| sym-44c703cd1ab4683be3c6 | `pocketstation::session::extensions::source::SourceRuntimeError::Publish` | variant | Classifies a failure at the publish stage or component of `SourceRuntimeError`. | `src/session/extensions/source.rs:778` |
| sym-a6f113c988701269e1d9 | `pocketstation::session::extensions::source::SourceRuntimeError::Spawn` | variant | Classifies a failure at the spawn stage or component of `SourceRuntimeError`. | `src/session/extensions/source.rs:780` |
| sym-f3705f719ae69d9fcc35 | `pocketstation::session::extensions::source::SourceRuntimeError::UnknownOutput` | variant | Reports that the referenced output is not declared or registered. | `src/session/extensions/source.rs:766` |
| sym-db0d2ae7547305f52e58 | `pocketstation::session::extensions::source::SourceRuntimeError::UnregisteredSource` | variant | Reports that source has not been registered. | `src/session/extensions/source.rs:788` |
| sym-713daba94b569089c448 | `pocketstation::session::extensions::source::SourceRuntimeError::UnroutedOutput` | variant | Classifies a failure at the unrouted output stage or component of `SourceRuntimeError`. | `src/session/extensions/source.rs:768` |
| sym-0a146ce6fb30fc1de1cf | `pocketstation::session::extensions::source::SourceRuntimeError::WorkerPanicked` | variant | Reports that worker panicked while the operation was active. | `src/session/extensions/source.rs:782` |
| sym-78768732f5a0d6d45106 | `pocketstation::session::extensions::source::SourceTypeIdError::Empty` | variant | Represents an empty value or collection. | `src/session/extensions/source.rs:70` |
| sym-e3de8d5a179b4593280f | `pocketstation::session::extensions::source::SourceTypeIdError::InvalidContractSyntax` | variant | Reports that the supplied contract syntax is invalid. | `src/session/extensions/source.rs:81` |
| sym-71c54c00f67c32b65a1c | `pocketstation::session::extensions::source::SourceTypeIdError::MissingSourceCategory` | variant | Reports that the required source category is missing. | `src/session/extensions/source.rs:83` |
| sym-3a2eb85997f2802695ab | `pocketstation::session::extensions::source::SourceTypeIdError::NonAscii` | variant | Reports that no n ascii is available. | `src/session/extensions/source.rs:79` |
| sym-d74c350a7d31ff16f88a | `pocketstation::session::extensions::source::SourceTypeIdError::SurroundingWhitespace` | variant | Classifies a failure at the surrounding whitespace stage or component of `SourceTypeIdError`. | `src/session/extensions/source.rs:72` |
| sym-6813ff01534cc62abe44 | `pocketstation::session::extensions::source::SourceTypeIdError::TooLong` | variant | Classifies a failure at the too long stage or component of `SourceTypeIdError`. | `src/session/extensions/source.rs:74` |
| sym-4bbdbda13a2e2735e293 | `pocketstation::session::lifecycle::control::SessionStartError::Cancelled` | variant | Indicates that the operation was cancelled. | `src/session/lifecycle/control.rs:202` |
| sym-eb513b2d7bbed8eefffd | `pocketstation::session::lifecycle::control::SessionStartError::CaptureOpen` | variant | Classifies a failure at the capture open stage or component of `SessionStartError`. | `src/session/lifecycle/control.rs:173` |
| sym-793517bc56d163ce19d9 | `pocketstation::session::lifecycle::control::SessionStartError::CapturePrepare` | variant | Classifies a failure at the capture prepare stage or component of `SessionStartError`. | `src/session/lifecycle/control.rs:166` |
| sym-c89d4a5dfe7b37c98152 | `pocketstation::session::lifecycle::control::SessionStartError::EndpointPrepare` | variant | Classifies a failure at the endpoint prepare stage or component of `SessionStartError`. | `src/session/lifecycle/control.rs:160` |
| sym-fabd757dcf14c3600f64 | `pocketstation::session::lifecycle::control::SessionStartError::EndpointStart` | variant | Classifies a failure at the endpoint start stage or component of `SessionStartError`. | `src/session/lifecycle/control.rs:180` |
| sym-82b9e576d7cfd32a6b8b | `pocketstation::session::lifecycle::control::SessionStartError::ExternalAudioBridge` | variant | Classifies a failure at the external audio bridge stage or component of `SessionStartError`. | `src/session/lifecycle/control.rs:132` |
| sym-150d95d05a893238a6d4 | `pocketstation::session::lifecycle::control::SessionStartError::ExternalSourcePrepare` | variant | Classifies a failure at the external source prepare stage or component of `SessionStartError`. | `src/session/lifecycle/control.rs:127` |
| sym-0e0619a5018b9ac8263d | `pocketstation::session::lifecycle::control::SessionStartError::ExternalSourceStart` | variant | Classifies a failure at the external source start stage or component of `SessionStartError`. | `src/session/lifecycle/control.rs:142` |
| sym-fe072d1afd48a092ca54 | `pocketstation::session::lifecycle::control::SessionStartError::GeneratedAudioBridge` | variant | Classifies a failure at the generated audio bridge stage or component of `SessionStartError`. | `src/session/lifecycle/control.rs:137` |
| sym-a124b806781a684a82ae | `pocketstation::session::lifecycle::control::SessionStartError::InvalidOptions` | variant | Reports that the supplied options is invalid. | `src/session/lifecycle/control.rs:123` |
| sym-dc552e9743a31538fca0 | `pocketstation::session::lifecycle::control::SessionStartError::MissingEndpointDeclaration` | variant | Reports that the required endpoint declaration is missing. | `src/session/lifecycle/control.rs:158` |
| sym-80029918a592b2d9c43c | `pocketstation::session::lifecycle::control::SessionStartError::OperatorPrepare` | variant | Classifies a failure at the operator prepare stage or component of `SessionStartError`. | `src/session/lifecycle/control.rs:152` |
| sym-42ab0bffd89a99dab705 | `pocketstation::session::lifecycle::control::SessionStartError::OperatorRuntimeHost` | variant | Classifies a failure at the operator runtime host stage or component of `SessionStartError`. | `src/session/lifecycle/control.rs:147` |
| sym-bc9f360a29c2f2609e36 | `pocketstation::session::lifecycle::control::SessionStartError::RuntimeRunner` | variant | Classifies a failure at the runtime runner stage or component of `SessionStartError`. | `src/session/lifecycle/control.rs:186` |
| sym-7500b671839c35f99045 | `pocketstation::session::lifecycle::control::SessionStartError::RuntimeWorkerReady` | variant | Classifies a failure at the runtime worker ready stage or component of `SessionStartError`. | `src/session/lifecycle/control.rs:197` |
| sym-8748b617932b6bc67ef9 | `pocketstation::session::lifecycle::control::SessionStartError::RuntimeWorkerSpawn` | variant | Classifies a failure at the runtime worker spawn stage or component of `SessionStartError`. | `src/session/lifecycle/control.rs:192` |
| sym-fdcb603f4f16e4d91442 | `pocketstation::session::lifecycle::control::SessionStartError::UnsupportedSourceTopology` | variant | Reports that the requested source topology is unsupported. | `src/session/lifecycle/control.rs:125` |
| sym-8284e9acf0d6d1c8dfd6 | `pocketstation::session::lifecycle::engine::EndpointExtensionRegistrationError::ConflictingDefinition` | variant | Reports that definition conflicts with an existing registration or declaration. | `src/session/lifecycle/engine.rs:311` |
| sym-b778d1712410a44b96a2 | `pocketstation::session::lifecycle::engine::EndpointExtensionRegistrationError::Definition` | variant | Classifies a failure at the definition stage or component of `EndpointExtensionRegistrationError`. | `src/session/lifecycle/engine.rs:307` |
| sym-5bcc3a89c619bc27f4f5 | `pocketstation::session::lifecycle::engine::EndpointExtensionRegistrationError::Driver` | variant | Classifies a failure at the driver stage or component of `EndpointExtensionRegistrationError`. | `src/session/lifecycle/engine.rs:309` |
| sym-6c15c2307860aa234363 | `pocketstation::session::lifecycle::engine::SessionEngineBuildError::DuplicateSidecarId` | variant | Reports that sidecar identifier duplicates an existing declaration or record. | `src/session/lifecycle/engine.rs:301` |
| sym-c3683834c158382e7e54 | `pocketstation::session::lifecycle::engine::SessionEngineBuildError::InvalidConfiguration` | variant | Reports that the supplied configuration is invalid. | `src/session/lifecycle/engine.rs:299` |
| sym-b885f7d445a1cc4fd69e | `pocketstation::session::lifecycle::engine::SessionEngineBuildError::StructuralNodeRegistration` | variant | Classifies a failure at the structural node registration stage or component of `SessionEngineBuildError`. | `src/session/lifecycle/engine.rs:297` |
| sym-c346413be207643cd7d5 | `pocketstation::session::lifecycle::engine::SessionEngineStartError::Compile` | variant | Classifies a failure at the compile stage or component of `SessionEngineStartError`. | `src/session/lifecycle/engine.rs:319` |
| sym-d9f6cf1e6fda6b5d6100 | `pocketstation::session::lifecycle::engine::SessionEngineStartError::Freeze` | variant | Classifies a failure at the freeze stage or component of `SessionEngineStartError`. | `src/session/lifecycle/engine.rs:317` |
| sym-c3847170e5fa0f27f9ac | `pocketstation::session::lifecycle::engine::SessionEngineStartError::Prepare` | variant | Classifies a failure at the prepare stage or component of `SessionEngineStartError`. | `src/session/lifecycle/engine.rs:321` |
| sym-d425d55ec30b6b15af7d | `pocketstation::session::lifecycle::engine::SessionEngineStartError::Sidecar` | variant | Classifies a failure at the sidecar stage or component of `SessionEngineStartError`. | `src/session/lifecycle/engine.rs:325` |
| sym-9c0784824d76cbdc14f7 | `pocketstation::session::lifecycle::engine::SessionEngineStartError::Start` | variant | Classifies a failure at the start stage or component of `SessionEngineStartError`. | `src/session/lifecycle/engine.rs:323` |
| sym-3f212da27ed8890eb453 | `pocketstation::session::lifecycle::events::SessionComponentId::Endpoint` | variant | Represents the endpoint alternative defined by `SessionComponentId`. | `src/session/lifecycle/events.rs:55` |
| sym-c75e9935cf7789c13066 | `pocketstation::session::lifecycle::events::SessionComponentId::Operator` | variant | Represents the operator alternative defined by `SessionComponentId`. | `src/session/lifecycle/events.rs:59` |
| sym-d7dd2c8c5f0861192150 | `pocketstation::session::lifecycle::events::SessionComponentId::Runtime` | variant | Represents the runtime alternative defined by `SessionComponentId`. | `src/session/lifecycle/events.rs:65` |
| sym-7e1e0ff85d28e03af120 | `pocketstation::session::lifecycle::events::SessionComponentId::Sidecar` | variant | Represents the sidecar alternative defined by `SessionComponentId`. | `src/session/lifecycle/events.rs:62` |
| sym-862c7d9ec6151f862388 | `pocketstation::session::lifecycle::events::SessionComponentId::Source` | variant | Represents the source alternative defined by `SessionComponentId`. | `src/session/lifecycle/events.rs:52` |
| sym-2157b9a7e55204a5f946 | `pocketstation::session::lifecycle::events::SessionEventKind::Endpoint` | variant | Identifies the endpoint state or stage represented by `SessionEventKind`. | `src/session/lifecycle/events.rs:297` |
| sym-8ec9fc34f3dc072d0c89 | `pocketstation::session::lifecycle::events::SessionEventKind::Finalization` | variant | Identifies the finalization state or stage represented by `SessionEventKind`. | `src/session/lifecycle/events.rs:299` |
| sym-f3b85b33a1481c3d8f20 | `pocketstation::session::lifecycle::events::SessionEventKind::Lifecycle` | variant | Identifies the lifecycle state or stage represented by `SessionEventKind`. | `src/session/lifecycle/events.rs:295` |
| sym-967205dde1d1c5d51e65 | `pocketstation::session::lifecycle::events::SessionEventKind::Rollback` | variant | Identifies the rollback state or stage represented by `SessionEventKind`. | `src/session/lifecycle/events.rs:298` |
| sym-2e7b93f42d0bdf9eb66e | `pocketstation::session::lifecycle::events::SessionEventKind::Source` | variant | Identifies the source state or stage represented by `SessionEventKind`. | `src/session/lifecycle/events.rs:296` |
| sym-ea6caaceec70c3c75333 | `pocketstation::session::lifecycle::events::SessionEventKind::Terminal` | variant | Identifies the terminal state or stage represented by `SessionEventKind`. | `src/session/lifecycle/events.rs:300` |
| sym-b560417b875bade19519 | `pocketstation::session::lifecycle::events::SessionEventReceive::Closed` | variant | Reports that the underlying channel or resource is closed. | `src/session/lifecycle/events.rs:495` |
| sym-d15b2b5815e78a4c35f1 | `pocketstation::session::lifecycle::events::SessionEventReceive::Empty` | variant | Represents an empty value or collection. | `src/session/lifecycle/events.rs:494` |
| sym-d02dbc6c6096d8a33597 | `pocketstation::session::lifecycle::events::SessionEventReceive::Event` | variant | Identifies the event state or stage represented by `SessionEventReceive`. | `src/session/lifecycle/events.rs:493` |
| sym-9aa2a099e0cb8ba72e5e | `pocketstation::session::lifecycle::events::SessionFinalizationStage::DrainOperator` | variant | Identifies the drain operator state or stage represented by `SessionFinalizationStage`. | `src/session/lifecycle/events.rs:42` |
| sym-7fecb981c38a9cb7e0b9 | `pocketstation::session::lifecycle::events::SessionFinalizationStage::DrainRuntime` | variant | Identifies the drain runtime state or stage represented by `SessionFinalizationStage`. | `src/session/lifecycle/events.rs:41` |
| sym-6c53c5dbbb169b5c834e | `pocketstation::session::lifecycle::events::SessionFinalizationStage::DrainSidecar` | variant | Identifies the drain sidecar state or stage represented by `SessionFinalizationStage`. | `src/session/lifecycle/events.rs:46` |
| sym-27819dfd19c124fd8cf4 | `pocketstation::session::lifecycle::events::SessionFinalizationStage::FinalizeEndpoint` | variant | Identifies the finalize endpoint state or stage represented by `SessionFinalizationStage`. | `src/session/lifecycle/events.rs:45` |
| sym-2dcee70d5dd82e2fad57 | `pocketstation::session::lifecycle::events::SessionFinalizationStage::JoinEndpoint` | variant | Identifies the join endpoint state or stage represented by `SessionFinalizationStage`. | `src/session/lifecycle/events.rs:44` |
| sym-34792723c8783848efc1 | `pocketstation::session::lifecycle::events::SessionFinalizationStage::RequestEndpointStop` | variant | Identifies the request endpoint stop state or stage represented by `SessionFinalizationStage`. | `src/session/lifecycle/events.rs:43` |
| sym-f53872810cb737ef1d5d | `pocketstation::session::lifecycle::events::SessionFinalizationStage::StopCapture` | variant | Identifies the stop capture state or stage represented by `SessionFinalizationStage`. | `src/session/lifecycle/events.rs:40` |
| sym-d81fdbd86423755797d6 | `pocketstation::session::lifecycle::events::SessionLifecycleState::Failed` | variant | Identifies the failed state or stage represented by `SessionLifecycleState`. | `src/session/lifecycle/events.rs:24` |
| sym-e991ca677d715ed33ab9 | `pocketstation::session::lifecycle::events::SessionLifecycleState::Running` | variant | Identifies the running state or stage represented by `SessionLifecycleState`. | `src/session/lifecycle/events.rs:21` |
| sym-e5d8bbb2eb1acab017d2 | `pocketstation::session::lifecycle::events::SessionLifecycleState::Starting` | variant | Identifies the starting state or stage represented by `SessionLifecycleState`. | `src/session/lifecycle/events.rs:20` |
| sym-7c2560bdcf339cd169a0 | `pocketstation::session::lifecycle::events::SessionLifecycleState::Stopped` | variant | Indicates that the operation stopped normally. | `src/session/lifecycle/events.rs:23` |
| sym-e7411bb7d663d2d1bf5f | `pocketstation::session::lifecycle::events::SessionLifecycleState::Stopping` | variant | Identifies the stopping state or stage represented by `SessionLifecycleState`. | `src/session/lifecycle/events.rs:22` |
| sym-7c222f6fc240d1c7e2ad | `pocketstation::session::lifecycle::events::SessionRollbackStage::CancelEndpointPreparation` | variant | Identifies the cancel endpoint preparation state or stage represented by `SessionRollbackStage`. | `src/session/lifecycle/events.rs:31` |
| sym-08b5db33e6e433e2dbba | `pocketstation::session::lifecycle::events::SessionRollbackStage::CancelOperator` | variant | Identifies the cancel operator state or stage represented by `SessionRollbackStage`. | `src/session/lifecycle/events.rs:30` |
| sym-4e01d1dc631149ae384e | `pocketstation::session::lifecycle::events::SessionRollbackStage::DiscardRuntimeQueues` | variant | Identifies the discard runtime queues state or stage represented by `SessionRollbackStage`. | `src/session/lifecycle/events.rs:34` |
| sym-6b4274b35bb503dd7137 | `pocketstation::session::lifecycle::events::SessionRollbackStage::FinalizeStartedEndpoint` | variant | Identifies the finalize started endpoint state or stage represented by `SessionRollbackStage`. | `src/session/lifecycle/events.rs:32` |
| sym-de99d538eafaabc9cbb4 | `pocketstation::session::lifecycle::events::SessionRollbackStage::StopOpenedCapture` | variant | Identifies the stop opened capture state or stage represented by `SessionRollbackStage`. | `src/session/lifecycle/events.rs:33` |
| sym-c5138ddfe91f5221a438 | `pocketstation::session::lifecycle::events::SessionTerminalState::Failed` | variant | Identifies the failed state or stage represented by `SessionTerminalState`. | `src/session/lifecycle/events.rs:212` |
| sym-7baa3f582705e602fb58 | `pocketstation::session::lifecycle::events::SessionTerminalState::Stopped` | variant | Indicates that the operation stopped normally. | `src/session/lifecycle/events.rs:211` |
| sym-302f53ca07aaaa697c9b | `pocketstation::session::lifecycle::host::SessionEngineHostBuildError::EndpointExtensionRegistration` | variant | Classifies a failure at the endpoint extension registration stage or component of `SessionEngineHostBuildError`. | `src/session/lifecycle/host.rs:368` |
| sym-1699f293ce175219459a | `pocketstation::session::lifecycle::host::SessionEngineHostBuildError::EndpointRegistration` | variant | Classifies a failure at the endpoint registration stage or component of `SessionEngineHostBuildError`. | `src/session/lifecycle/host.rs:366` |
| sym-1f3703c075ff9821c791 | `pocketstation::session::lifecycle::host::SessionEngineHostBuildError::Engine` | variant | Classifies a failure at the engine stage or component of `SessionEngineHostBuildError`. | `src/session/lifecycle/host.rs:364` |
| sym-501f64b6448db4d1b192 | `pocketstation::session::lifecycle::host::SessionEngineHostBuildError::MissingApplicationBackend` | variant | Reports that the required application backend is missing. | `src/session/lifecycle/host.rs:374` |
| sym-6cb7c42ee02a5091b11c | `pocketstation::session::lifecycle::host::SessionEngineHostBuildError::MissingMicrophoneBackend` | variant | Reports that the required microphone backend is missing. | `src/session/lifecycle/host.rs:376` |
| sym-773e7ba23d3d54ee4f95 | `pocketstation::session::lifecycle::host::SessionEngineHostBuildError::OperatorRegistration` | variant | Classifies a failure at the operator registration stage or component of `SessionEngineHostBuildError`. | `src/session/lifecycle/host.rs:370` |
| sym-3d21f83f50b0e1d3cf2f | `pocketstation::session::lifecycle::host::SessionEngineHostBuildError::PolledAudioEndpoint` | variant | Classifies a failure at the polled audio endpoint stage or component of `SessionEngineHostBuildError`. | `src/session/lifecycle/host.rs:372` |
| sym-b89da10b47d65c57682e | `pocketstation::session::lifecycle::host::SessionEngineHostBuildError::UnsupportedPlatform` | variant | Reports that the requested platform is unsupported. | `src/session/lifecycle/host.rs:378` |
| sym-044b4b815fd84f897e19 | `pocketstation::session::lifecycle::observations::EndpointObservationStage::Finalized` | variant | Identifies the finalized state or stage represented by `EndpointObservationStage`. | `src/session/lifecycle/observations.rs:444` |
| sym-8584743165e80a176c8e | `pocketstation::session::lifecycle::observations::EndpointObservationStage::Live` | variant | Identifies the live state or stage represented by `EndpointObservationStage`. | `src/session/lifecycle/observations.rs:443` |
| sym-dafcfabe8eb0897fe63b | `pocketstation::session::lifecycle::observations::EndpointObservationStage::Unavailable` | variant | Reports that the requested resource is unavailable. | `src/session/lifecycle/observations.rs:442` |
| sym-d511ff5ef0df82201aec | `pocketstation::session::lifecycle::observations::SessionRouteLatencyBoundary::SourceMonotonicTimestampToRouteReceive` | variant | Represents the source monotonic timestamp to route receive alternative defined by `SessionRouteLatencyBoundary`. | `src/session/lifecycle/observations.rs:197` |
| sym-fee639d89aa0e3b2b550 | `pocketstation::session::lifecycle::observations::SessionRouteLatencyUnit::Nanoseconds` | variant | Represents the nanoseconds alternative defined by `SessionRouteLatencyUnit`. | `src/session/lifecycle/observations.rs:202` |
| sym-507483297aeefa70e20f | `pocketstation::session::lifecycle::observations::SessionRouteObservationInterval::RouteLifetimeToSnapshot` | variant | From route start through the instant of the Session snapshot. | `src/session/lifecycle/observations.rs:152` |
| sym-6a1e9e5b35a67a533519 | `pocketstation::session::lifecycle::trace::SessionTraceRecordKind::EndpointFailure` | variant | Tags a Session trace record as endpoint failure. | `src/session/lifecycle/trace.rs:34` |
| sym-356b7488d32266e20be6 | `pocketstation::session::lifecycle::trace::SessionTraceRecordKind::FinalizationFailure` | variant | Tags a Session trace record as finalization failure. | `src/session/lifecycle/trace.rs:42` |
| sym-e0028c913ea92b869217 | `pocketstation::session::lifecycle::trace::SessionTraceRecordKind::Lifecycle` | variant | Tags a Session trace record as lifecycle. | `src/session/lifecycle/trace.rs:28` |
| sym-3f538a1ec43e362c6249 | `pocketstation::session::lifecycle::trace::SessionTraceRecordKind::RollbackFailure` | variant | Tags a Session trace record as rollback failure. | `src/session/lifecycle/trace.rs:39` |
| sym-a3a6563e309787cc8aa6 | `pocketstation::session::lifecycle::trace::SessionTraceRecordKind::SourceFailure` | variant | Tags a Session trace record as source failure. | `src/session/lifecycle/trace.rs:31` |
| sym-db7245045edb1ce12b70 | `pocketstation::session::lifecycle::trace::SessionTraceRecordKind::Terminal` | variant | Tags a Session trace record as terminal. | `src/session/lifecycle/trace.rs:45` |
| sym-6ae7bc405070081e8037 | `pocketstation::session::lifecycle::trace::SessionTraceRecorderFinishError::ChannelClosed` | variant | Reports that channel closed before the operation completed. | `src/session/lifecycle/trace.rs:100` |
| sym-4db4aad9f658fd9b7bf7 | `pocketstation::session::lifecycle::trace::SessionTraceRecorderFinishError::Io` | variant | Reports an operating-system or filesystem I/O failure. | `src/session/lifecycle/trace.rs:104` |
| sym-6d3f9ab36cd46284df2d | `pocketstation::session::lifecycle::trace::SessionTraceRecorderFinishError::WorkerPanicked` | variant | Reports that worker panicked while the operation was active. | `src/session/lifecycle/trace.rs:102` |
| sym-05e45aca1fb7d5f89d7e | `pocketstation::session::lifecycle::trace::SessionTraceRecorderStartError::Io` | variant | Reports an operating-system or filesystem I/O failure. | `src/session/lifecycle/trace.rs:94` |
| sym-7e7bcf206910a985e75f | `pocketstation::session::lifecycle::trace::SessionTraceRecorderStartError::OutputExists` | variant | Reports that output already exists and would be overwritten. | `src/session/lifecycle/trace.rs:92` |
| sym-6a9a989311ccc616593b | `pocketstation::session::lifecycle::trace::SessionTraceRecorderStartError::ZeroCapacity` | variant | Reports that capacity must be greater than zero. | `src/session/lifecycle/trace.rs:90` |
| sym-644b14a406381fc1d4dc | `pocketstation::session::lifecycle::trace::SessionTraceValidationError::IncompleteTrace` | variant | Classifies a failure at the incomplete trace stage or component of `SessionTraceValidationError`. | `src/session/lifecycle/trace.rs:370` |
| sym-39a481c52cb7442a2afb | `pocketstation::session::lifecycle::trace::SessionTraceValidationError::InvalidChecksum` | variant | Reports that the supplied checksum is invalid. | `src/session/lifecycle/trace.rs:368` |
| sym-3ccc6b63621b9f6c1f80 | `pocketstation::session::lifecycle::trace::SessionTraceValidationError::InvalidLayout` | variant | Reports that the supplied layout is invalid. | `src/session/lifecycle/trace.rs:364` |
| sym-6f2941c3fde8ef917eb9 | `pocketstation::session::lifecycle::trace::SessionTraceValidationError::InvalidLifecycleTransition` | variant | Reports that the supplied lifecycle transition is invalid. | `src/session/lifecycle/trace.rs:378` |
| sym-8542540eeba2efd38400 | `pocketstation::session::lifecycle::trace::SessionTraceValidationError::InvalidMagic` | variant | Reports that the supplied magic is invalid. | `src/session/lifecycle/trace.rs:360` |
| sym-939118eb1e2afb6dbc1c | `pocketstation::session::lifecycle::trace::SessionTraceValidationError::Io` | variant | Reports an operating-system or filesystem I/O failure. | `src/session/lifecycle/trace.rs:358` |
| sym-ad6cdc3e9ba0139b798a | `pocketstation::session::lifecycle::trace::SessionTraceValidationError::MissingTerminal` | variant | Reports that the required terminal is missing. | `src/session/lifecycle/trace.rs:380` |
| sym-4ee10e6680520f7a3e8d | `pocketstation::session::lifecycle::trace::SessionTraceValidationError::RecordAfterTerminal` | variant | Classifies a failure at the record after terminal stage or component of `SessionTraceValidationError`. | `src/session/lifecycle/trace.rs:384` |
| sym-dbedb5a5eaed0679beb2 | `pocketstation::session::lifecycle::trace::SessionTraceValidationError::SequenceGap` | variant | Classifies a failure at the sequence gap stage or component of `SessionTraceValidationError`. | `src/session/lifecycle/trace.rs:372` |
| sym-59da27d2011f9123fec0 | `pocketstation::session::lifecycle::trace::SessionTraceValidationError::SessionMismatch` | variant | Reports that session does not match the expected contract. | `src/session/lifecycle/trace.rs:374` |
| sym-2c05f5e254e564d17ef7 | `pocketstation::session::lifecycle::trace::SessionTraceValidationError::TerminalMismatch` | variant | Reports that terminal does not match the expected contract. | `src/session/lifecycle/trace.rs:382` |
| sym-fec1455accfdbdfb3edd | `pocketstation::session::lifecycle::trace::SessionTraceValidationError::TimestampRegression` | variant | Reports that timestamp moved backward instead of remaining monotonic. | `src/session/lifecycle/trace.rs:376` |
| sym-c69c9d400755934ae8d8 | `pocketstation::session::lifecycle::trace::SessionTraceValidationError::Truncated` | variant | Reports that the encoded input ended before the complete record was available. | `src/session/lifecycle/trace.rs:366` |
| sym-c6028fd6a94588b75bdf | `pocketstation::session::lifecycle::trace::SessionTraceValidationError::UnknownRecordType` | variant | Reports that the referenced record type is not declared or registered. | `src/session/lifecycle/trace.rs:386` |
| sym-3b16f7723e87994f85f8 | `pocketstation::session::lifecycle::trace::SessionTraceValidationError::UnsupportedVersion` | variant | Reports that the requested version is unsupported. | `src/session/lifecycle/trace.rs:362` |
| sym-2c9c7e397369c6db7751 | `pocketstation::session::prepare::error::SessionPrepareError::DuplicateOperatorInput` | variant | Reports that operator input duplicates an existing declaration or record. | `src/session/prepare/error.rs:76` |
| sym-81240a09cfe3b38ebe4b | `pocketstation::session::prepare::error::SessionPrepareError::DuplicateSignalRoute` | variant | Reports that signal route duplicates an existing declaration or record. | `src/session/prepare/error.rs:80` |
| sym-b26ef32be8f4f3e1b7fb | `pocketstation::session::prepare::error::SessionPrepareError::DuplicateSourceNode` | variant | Reports that source node duplicates an existing declaration or record. | `src/session/prepare/error.rs:17` |
| sym-9d53da9dc4074c727538 | `pocketstation::session::prepare::error::SessionPrepareError::DuplicateWorkerRoute` | variant | Reports that worker route duplicates an existing declaration or record. | `src/session/prepare/error.rs:53` |
| sym-95f92a415be5252d20d5 | `pocketstation::session::prepare::error::SessionPrepareError::IncompatibleNodeBinding` | variant | Reports that node binding is incompatible with the required contract. | `src/session/prepare/error.rs:74` |
| sym-30a7a90ba8664829aaee | `pocketstation::session::prepare::error::SessionPrepareError::InvalidExternalAudioMedia` | variant | Reports that the supplied external audio media is invalid. | `src/session/prepare/error.rs:26` |
| sym-1b24fd3c46a90566cb18 | `pocketstation::session::prepare::error::SessionPrepareError::InvalidGeneratedAudioMedia` | variant | Reports that the supplied generated audio media is invalid. | `src/session/prepare/error.rs:33` |
| sym-d0ddd57a73e80fe44a80 | `pocketstation::session::prepare::error::SessionPrepareError::InvalidOperatorInputPort` | variant | Reports that the supplied operator input port is invalid. | `src/session/prepare/error.rs:49` |
| sym-567bd443785c26c6e875 | `pocketstation::session::prepare::error::SessionPrepareError::MissingAsyncOperatorFactory` | variant | Reports that the required async operator factory is missing. | `src/session/prepare/error.rs:68` |
| sym-4d5639be77ee16d58949 | `pocketstation::session::prepare::error::SessionPrepareError::MissingExternalAudioIngress` | variant | Reports that the required external audio ingress is missing. | `src/session/prepare/error.rs:19` |
| sym-cc66346c005b8e5e6e3d | `pocketstation::session::prepare::error::SessionPrepareError::MissingExternalSourceDefinition` | variant | Reports that the required external source definition is missing. | `src/session/prepare/error.rs:24` |
| sym-fe9651808af518442062 | `pocketstation::session::prepare::error::SessionPrepareError::MissingExternalSourceRouteEdge` | variant | Reports that the required external source route edge is missing. | `src/session/prepare/error.rs:37` |
| sym-3b7d227a0c422934219a | `pocketstation::session::prepare::error::SessionPrepareError::MissingGeneratedAudioBridge` | variant | Reports that the required generated audio bridge is missing. | `src/session/prepare/error.rs:35` |
| sym-3478830656644d554244 | `pocketstation::session::prepare::error::SessionPrepareError::MissingGeneratedAudioIngress` | variant | Reports that the required generated audio ingress is missing. | `src/session/prepare/error.rs:31` |
| sym-8871e3e7c13a87f7bb00 | `pocketstation::session::prepare::error::SessionPrepareError::MissingNodeBinding` | variant | Reports that the required node binding is missing. | `src/session/prepare/error.rs:72` |
| sym-6f40a8319f948227d6ce | `pocketstation::session::prepare::error::SessionPrepareError::MissingOperatorSignalInput` | variant | Reports that the required operator signal input is missing. | `src/session/prepare/error.rs:78` |
| sym-7baa16da380a9f79df7e | `pocketstation::session::prepare::error::SessionPrepareError::MissingSourceNode` | variant | Reports that the required source node is missing. | `src/session/prepare/error.rs:15` |
| sym-7aa619c8e9d6394ae6e0 | `pocketstation::session::prepare::error::SessionPrepareError::MissingTypedEdgePlan` | variant | Reports that the required typed edge plan is missing. | `src/session/prepare/error.rs:66` |
| sym-368d2e24a2eaeac6b47c | `pocketstation::session::prepare::error::SessionPrepareError::MissingWorkerCapacity` | variant | Reports that the required worker capacity is missing. | `src/session/prepare/error.rs:45` |
| sym-1c4b1bcd13214d3b759e | `pocketstation::session::prepare::error::SessionPrepareError::MissingWorkerEdge` | variant | Reports that the required worker edge is missing. | `src/session/prepare/error.rs:41` |
| sym-6e4b28d26378b5d7de69 | `pocketstation::session::prepare::error::SessionPrepareError::MissingWorkerEdgeContract` | variant | Reports that the required worker edge contract is missing. | `src/session/prepare/error.rs:43` |
| sym-b0f9760a1bb862ba985d | `pocketstation::session::prepare::error::SessionPrepareError::MissingWorkerSampleSpec` | variant | Reports that the required worker sample spec is missing. | `src/session/prepare/error.rs:47` |
| sym-47697da1484c69008169 | `pocketstation::session::prepare::error::SessionPrepareError::MissingWorkerTarget` | variant | Reports that the required worker target is missing. | `src/session/prepare/error.rs:39` |
| sym-7767f11db5aa1208a794 | `pocketstation::session::prepare::error::SessionPrepareError::OperatorDeclarationMismatch` | variant | Reports that operator declaration does not match the expected contract. | `src/session/prepare/error.rs:70` |
| sym-14238923b59dfa183969 | `pocketstation::session::prepare::error::SessionPrepareError::Runtime` | variant | Classifies a failure at the runtime stage or component of `SessionPrepareError`. | `src/session/prepare/error.rs:11` |
| sym-7f98c6a6d158e7739ead | `pocketstation::session::prepare::error::SessionPrepareError::SignalRouteMismatch` | variant | Reports that signal route does not match the expected contract. | `src/session/prepare/error.rs:82` |
| sym-e33f0ed369f4c896444f | `pocketstation::session::prepare::error::SessionPrepareError::SourceChannel` | variant | Classifies a failure at the source channel stage or component of `SessionPrepareError`. | `src/session/prepare/error.rs:13` |
| sym-435bc30089256441be8e | `pocketstation::session::prepare::error::SessionPrepareError::UnknownWorkerRoute` | variant | Reports that the referenced worker route is not declared or registered. | `src/session/prepare/error.rs:51` |
| sym-c1341c79cddc8f52ff0d | `pocketstation::session::prepare::error::SessionPrepareError::WorkerRouteMismatch` | variant | Reports that worker route does not match the expected contract. | `src/session/prepare/error.rs:57` |
| sym-eb869a5008bcffb02781 | `pocketstation::session::prepare::error::SessionPrepareError::WorkerTopologyMismatch` | variant | Reports that worker topology does not match the expected contract. | `src/session/prepare/error.rs:86` |
| sym-99b3c55887e45a315fed | `pocketstation::timing::domain::ClockDomainKind::ProcessMonotonic` | variant | Identifies timestamps as belonging to the process monotonic clock domain. | `src/timing/domain.rs:9` |
| sym-90786f825231e7b95285 | `pocketstation::timing::domain::ClockDomainKind::ProviderDefined` | variant | Identifies timestamps as belonging to the provider defined clock domain. | `src/timing/domain.rs:10` |
| sym-d9dd4700e264a084f4c8 | `pocketstation::timing::domain::ClockDomainKind::Unspecified` | variant | Identifies timestamps as belonging to the unspecified clock domain. | `src/timing/domain.rs:8` |
| sym-ea93bf173163a9225a06 | `pocketstation::timing::domain::ClockDomainOrigin::ProcessStart` | variant | Represents the process start alternative defined by `ClockDomainOrigin`. | `src/timing/domain.rs:17` |
| sym-4d8df7f26a599285aa84 | `pocketstation::timing::domain::ClockDomainOrigin::ProviderDefined` | variant | Represents the provider defined alternative defined by `ClockDomainOrigin`. | `src/timing/domain.rs:18` |
| sym-f9596f17e9b3277c3bec | `pocketstation::timing::domain::ClockDomainOrigin::Unspecified` | variant | Represents the unspecified alternative defined by `ClockDomainOrigin`. | `src/timing/domain.rs:16` |

## Interpretation

The **Rust API reference** inventory records compiler-visible or extracted evidence at the frozen snapshot. A field marked unknown or not declared remains outside the published guarantee; use the native signature, owning error type, and cited test before relying on panic, blocking, cancellation, ordering, limits, retry, or recovery behavior.

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

The claims on **Rust API reference** are anchored to Git snapshot `136e74888962558aa846d3143a19136a70936f45` and these primary files:

- `src/lib.rs:55-71` (`DIRECT`)
- `src/lib.rs:236-250` (`DIRECT`)
- `src/lib.rs:237-237` (`DIRECT`)
- `src/lib.rs:238-238` (`DIRECT`)
- `src/lib.rs:239-239` (`DIRECT`)
- `src/lib.rs:240-240` (`DIRECT`)
- `src/lib.rs:241-241` (`DIRECT`)
- `src/lib.rs:242-242` (`DIRECT`)
- `src/lib.rs:243-243` (`DIRECT`)
- `src/lib.rs:244-244` (`DIRECT`)
- `src/lib.rs:245-245` (`DIRECT`)
- `src/lib.rs:246-246` (`DIRECT`)
- `src/lib.rs:247-247` (`DIRECT`)
- `src/lib.rs:248-248` (`DIRECT`)
- `src/lib.rs:249-249` (`DIRECT`)
- `src/lib.rs:252-255` (`DIRECT`)
- `src/lib.rs:253-253` (`DIRECT`)
- `src/lib.rs:254-254` (`DIRECT`)
- `src/lib.rs:257-261` (`DIRECT`)
- `src/lib.rs:258-258` (`DIRECT`)
- `src/lib.rs:259-259` (`DIRECT`)
- `src/lib.rs:260-260` (`DIRECT`)
- `src/lib.rs:263-267` (`DIRECT`)
- `src/lib.rs:264-264` (`DIRECT`)

For **Rust API reference**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.
